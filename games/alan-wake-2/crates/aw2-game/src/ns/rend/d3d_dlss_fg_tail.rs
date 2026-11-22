#![allow(dead_code)]

use crate::exe::{self, rd, wr};

#[inline]
fn map_d3d_format_code(code: u8) -> u8 {
    match code {
        1 => 0x07,
        2 => 0x0c,
        3 => 0x16,
        4 => 0x18,
        5 => 0x08,
        6 => 0x0d,
        7 => 0x1b,
        8 => 0x1a,
        9 => 0x28,
        10 => 0x2a,
        11 => 0x1b,
        12 => 0x1a,
        _ => 0x18,
    }
}

/// Original: `0x142055f20` (79 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142055f20(this: usize) {
    if rd::<i32>(this) == -1 {
        return;
    }
    let ctx = exe::call_va!(0x1_4205_5c40, () -> usize);
    let global_arg = exe::ptr::<u8>(0x1_43ce_6878) as usize;
    let slot = exe::call_va!(0x1_4205_5f80, (global_arg) -> i32);
    let table = rd::<usize>(this + 0x50);
    let idx_offset = ((slot as isize) as usize).wrapping_mul(8);
    wr::<usize>(table.wrapping_add(idx_offset), ctx);
    let inner = rd::<usize>(ctx);
    exe::call_va!(0x1_4206_32b0, (inner) -> ());
    let get_time: unsafe extern "win64" fn() -> f64 = exe::func(0x1_4240_69a0);
    let timestamp: f64 = get_time();
    wr::<f64>(ctx + 0x10, timestamp);
}

/// Original: `0x142055f80` (165 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142055f80(arg: usize) -> u32 {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let tls_slot = tls0 + 0x6350;
    if rd::<i32>(tls_slot) == -1 {
        let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
        let lock_addr = exe::ptr::<u8>(0x1_4385_79d0) as usize;
        let owner_addr = exe::ptr::<u32>(0x1_4385_79d8) as usize;
        let depth_addr = exe::ptr::<u32>(0x1_4385_79dc) as usize;
        if rd::<u32>(owner_addr) != tid {
            exe::call_iat!(0x1_4264_55a8, (lock_addr) -> ());
            wr::<u32>(owner_addr, tid);
        }
        let depth = rd::<u32>(depth_addr);
        if rd::<i32>(tls_slot) == -1 {
            let counter_addr = exe::ptr::<u32>(0x1_43a2_b038) as usize;
            let cur = rd::<u32>(counter_addr);
            wr::<u32>(tls_slot, cur);
            wr::<u32>(counter_addr, cur.wrapping_add(1));
        }
        wr::<u32>(depth_addr, depth);
        if depth == 0 {
            wr::<u32>(owner_addr, 0xffff_ffff);
            exe::call_iat!(0x1_4264_5598, (lock_addr) -> ());
        }
    }
    let idx = rd::<u32>(tls_slot);
    (idx << 6) | rd::<u32>(arg)
}

/// Original: `0x1420562e0` (84 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420562e0(stream: usize, buf: usize) -> usize {
    let vtable = rd::<usize>(stream);
    let read_fn = rd::<usize>(vtable + 0x70);
    exe::call_va!(read_fn, (stream, buf) -> ());
    let mut i: u8 = 0;
    if rd::<u8>(buf) > 0 {
        loop {
            let mut byte_val: u8 = 0;
            let vt = rd::<usize>(stream);
            let fn_ptr = rd::<usize>(vt + 0x70);
            let tmp_ptr = (&mut byte_val as *mut u8) as usize;
            exe::call_va!(fn_ptr, (stream, tmp_ptr) -> ());
            let offset = i as usize;
            i = i.wrapping_add(1);
            wr::<u8>(buf + 1 + offset, byte_val);
            if i >= rd::<u8>(buf) {
                break;
            }
        }
    }
    stream
}

/// Original: `0x142056340` (204 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142056340(src: usize, out_count: usize, out_arr: usize) {
    let raw_count = rd::<u8>(src);
    let clamped = if raw_count < 8 { raw_count } else { 8 };
    wr::<u32>(out_count, clamped as u32);
    if clamped != 0 {
        let mut i: u8 = 0;
        loop {
            let idx = i as usize;
            let code = rd::<u8>(src + 1 + idx);
            let mapped = map_d3d_format_code(code);
            i = i.wrapping_add(1);
            wr::<u8>(out_arr + idx, mapped);
            if (i as i32) >= rd::<i32>(out_count) {
                break;
            }
        }
    }
}

pub unsafe fn diff_d3d_142056340(orig: usize, new: usize) -> Result<u32, String> {
    let mut cases = 0u32;
    let mut buf_o = [0u8; 64];
    let mut buf_n = [0u8; 64];
    for len in [0u8, 1, 5, 8, 12] {
        buf_o.fill(0x5a);
        buf_n.fill(0x5a);
        let src_o = (buf_o.as_mut_ptr() as usize) + 4;
        let src_n = (buf_n.as_mut_ptr() as usize) + 4;
        let cnt_o = src_o + 20;
        let cnt_n = src_n + 20;
        let arr_o = src_o + 28;
        let arr_n = src_n + 28;
        wr::<u8>(src_o, len);
        wr::<u8>(src_n, len);
        for k in 0..15usize {
            let v = (k as u8).wrapping_add(len);
            wr::<u8>(src_o + 1 + k, v);
            wr::<u8>(src_n + 1 + k, v);
        }
        exe::call_va!(orig, (src_o, cnt_o, arr_o) -> ());
        exe::call_va!(new, (src_n, cnt_n, arr_n) -> ());
        crate::diff_eq!(rd::<u32>(cnt_o), rd::<u32>(cnt_n), "out_count");
        for k in 0..12usize {
            crate::diff_eq!(rd::<u8>(arr_o + k), rd::<u8>(arr_n + k), "out_arr byte");
        }
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142056420` (112 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142056420(code: u8) -> u8 {
    map_d3d_format_code(code)
}

pub unsafe fn diff_d3d_142056420(orig: usize, new: usize) -> Result<u32, String> {
    let mut cases = 0u32;
    for code in 0u8..=20u8 {
        let ro: u8 = exe::call_va!(orig, (code) -> u8);
        let rn: u8 = exe::call_va!(new, (code) -> u8);
        crate::diff_eq!(ro, rn, "mapped format code");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142056660` (235 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142056660() {
    let data_addr = exe::ptr::<usize>(0x1_43a6_31e8) as usize;
    let count_addr = exe::ptr::<u32>(0x1_43a6_31f0) as usize;
    let mut i: i32 = 0;
    if rd::<u32>(count_addr) == 0 {
        return;
    }
    loop {
        let base = rd::<usize>(data_addr);
        let elem_offset = ((i as isize) as usize).wrapping_mul(24);
        let target_val = rd::<u64>(base + elem_offset + 0x8);
        let fence = rd::<usize>(base + elem_offset + 0x10);
        let mut keep = false;
        if fence != 0 {
            let vtable = rd::<usize>(fence);
            let get_completed = rd::<usize>(vtable + 0x40);
            let completed: u64 = exe::call_va!(get_completed, (fence) -> u64);
            if completed == u64::MAX {
                exe::call_va!(0x1_4205_d410, (fence, 0x887a_0005u32) -> ());
            }
            if completed < target_val {
                keep = true;
            }
        }
        let new_count: u32;
        if keep {
            new_count = rd::<u32>(count_addr);
        } else {
            let item = rd::<usize>(base + elem_offset);
            exe::call_va!(0x1_4205_64b0, (item) -> ());
            let cur_base = rd::<usize>(data_addr);
            let cur_count = rd::<u32>(count_addr) as usize;
            let dst = cur_base + elem_offset;
            let src = dst + 0x18;
            let move_bytes = (cur_count * 24).wrapping_sub(elem_offset + 0x18);
            exe::call_va!(0x1_4183_acc8, (dst, src, move_bytes) -> usize);
            new_count = rd::<u32>(count_addr).wrapping_sub(1);
            wr::<u32>(count_addr, new_count);
            i = i.wrapping_sub(1);
        }
        i = i.wrapping_add(1);
        if ((i as isize) as usize) >= (new_count as usize) {
            break;
        }
    }
}

/// Original: `0x142056760` (178 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_142056760() {
    let val0 = rd::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize);
    let mut tmp = [0u64; 2];
    let tmp_ptr = tmp.as_mut_ptr() as usize;
    let ret_ptr = exe::call_va!(0x1_4207_34e0, (tmp_ptr, 0u32) -> usize);
    let f0 = rd::<u64>(ret_ptr);
    let f1 = rd::<u64>(ret_ptr + 0x8);

    let vec_addr = exe::ptr::<usize>(0x1_43a6_31e8) as usize;
    let count_addr = exe::ptr::<u32>(0x1_43a6_31f0) as usize;
    let cap_addr = exe::ptr::<u32>(0x1_43a6_31f4) as usize;

    let mut count = rd::<u32>(count_addr) as usize;
    let mut base = rd::<usize>(vec_addr);
    let cap = rd::<u32>(cap_addr) as usize;
    if base.wrapping_add(count * 24) == base.wrapping_add(cap * 24) {
        let new_len = count.wrapping_add(1);
        exe::call_va!(0x1_411d_5690, (vec_addr, new_len) -> ());
        count = rd::<u32>(count_addr) as usize;
        base = rd::<usize>(vec_addr);
    }
    let dst = base + count * 24;
    wr::<u64>(dst, val0);
    wr::<u64>(dst + 0x8, f0);
    wr::<u64>(dst + 0x10, f1);
    wr::<u32>(count_addr, (count as u32).wrapping_add(1));
}

/// Original: `0x142056ce0` (193 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_142056ce0() {
    let lock_addr = exe::ptr::<u8>(0x1_4385_7a20) as usize;
    let owner_addr = exe::ptr::<u32>(0x1_4385_7a28) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_7a2c) as usize;

    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(owner_addr) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_addr) -> ());
        wr::<u32>(owner_addr, tid);
    }
    let mut depth = rd::<u32>(depth_addr).wrapping_add(1);
    wr::<u32>(depth_addr, depth);

    let arg = rd::<usize>(exe::ptr::<usize>(0x1_43a3_5918) as usize);
    let mut cur = rd::<usize>(exe::ptr::<usize>(0x1_43a6_31f8) as usize);
    let count = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3200) as usize) as usize;
    let end = cur.wrapping_add(count * 8);
    if cur != end {
        while cur != end {
            let obj = rd::<usize>(cur);
            let vtable = rd::<usize>(obj);
            let cb = rd::<usize>(vtable + 0x8);
            exe::call_va!(cb, (obj, arg) -> ());
            cur = cur.wrapping_add(8);
        }
        depth = rd::<u32>(depth_addr);
    }
    let next_depth = depth.wrapping_sub(1);
    wr::<u32>(depth_addr, next_depth);
    if next_depth == 0 {
        wr::<u32>(owner_addr, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (lock_addr) -> ());
    }
}

/// Original: `0x142057f90` (71 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142057f90(src: usize) {
    let dst = exe::ptr::<u8>(0x1_4385_7c50) as usize;
    for offset in (0..0x88usize).step_by(8) {
        wr::<u64>(dst + offset, rd::<u64>(src + offset));
    }
}

/// Original: `0x142057fe0` (42 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142057fe0(src: usize) {
    let dst = exe::ptr::<u8>(0x1_4385_7bf0) as usize;
    for offset in (0..0x60usize).step_by(8) {
        wr::<u64>(dst + offset, rd::<u64>(src + offset));
    }
}

/// Original: `0x142058140` (43 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142058140() {
    exe::call_va!(0x1_4205_8180, () -> ());
    let dev_addr = exe::ptr::<usize>(0x1_43a3_59f8) as usize;
    let dev = rd::<usize>(dev_addr);
    if dev != 0 {
        exe::call_iat!(0x1_4264_5ca0, (dev) -> ());
        wr::<usize>(dev_addr, 0);
    }
}

/// Original: `0x142058180` (213 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142058180() {
    let ctx_addr = exe::ptr::<usize>(0x1_43a3_5a98) as usize;
    if rd::<usize>(ctx_addr) == 0 {
        return;
    }
    let mut fence_pair = [0u64; 2];
    let pair_ptr = fence_pair.as_mut_ptr() as usize;
    exe::call_va!(0x1_4207_34e0, (pair_ptr, 0u32) -> usize);
    let target_val = rd::<u64>(pair_ptr);
    let fence = rd::<usize>(pair_ptr + 0x8);
    if fence != 0 {
        loop {
            let vtable = rd::<usize>(fence);
            let get_completed = rd::<usize>(vtable + 0x40);
            let completed: u64 = exe::call_va!(get_completed, (fence) -> u64);
            if completed == u64::MAX {
                exe::call_va!(0x1_4205_d410, (fence, 0x887a_0005u32) -> ());
            }
            if completed >= target_val {
                break;
            }
            exe::call_iat!(0x1_4264_5868, () -> ());
        }
    }
    let mut slot_a = exe::ptr::<usize>(0x1_43c4_8a60) as usize;
    let mut slot_b = exe::ptr::<usize>(0x1_43c4_8e70) as usize;
    let destroy_fn_addr = exe::ptr::<usize>(0x1_43c4_8ba0) as usize;
    for _ in 0..14usize {
        let handle = rd::<usize>(slot_b);
        if handle != 0 {
            let destroy_fn = rd::<usize>(destroy_fn_addr);
            exe::call_va!(destroy_fn, (handle) -> ());
            wr::<usize>(slot_b, 0);
            wr::<usize>(slot_a, 0);
        }
        slot_b += 0x18;
        slot_a += 0x8;
    }
    let ctx = rd::<usize>(ctx_addr);
    let freed = exe::call_va!(0x1_4205_8640, (ctx) -> usize);
    exe::call_va!(0x1_411d_5f40, (freed, 0u32) -> ());
}

/// Original: `0x142058540` (250 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142058540() -> u8 {
    let dev_addr = exe::ptr::<usize>(0x1_43a3_59f8) as usize;
    if rd::<usize>(dev_addr) == 0 {
        let mut desc = [0u8; 0x60];
        let desc_ptr = desc.as_mut_ptr() as usize;
        let d3d_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        let queue_holder = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
        let queue = rd::<usize>(queue_holder + 0x10);
        wr::<usize>(desc_ptr, d3d_dev);
        wr::<usize>(desc_ptr + 0x8, queue);
        wr::<u8>(desc_ptr + 0x58, 0);
        exe::call_iat!(0x1_4264_5ca8, (desc_ptr, dev_addr) -> u32);

        let core_name = exe::ptr::<u8>(0x1_433d_c3e0) as usize;
        let core_out = exe::ptr::<u8>(0x1_43c4_8ad0) as usize;
        exe::call_iat!(0x1_4264_5cb0, (rd::<usize>(dev_addr), core_name, 0x350u32, core_out) -> u32);

        let helper_name = exe::ptr::<u8>(0x1_433d_c3b0) as usize;
        let helper_out = exe::ptr::<u8>(0x1_43c4_8e20) as usize;
        exe::call_iat!(0x1_4264_5cb0, (rd::<usize>(dev_addr), helper_name, 0x28u32, helper_out) -> u32);

        let wrapper_name = exe::ptr::<u8>(0x1_433d_c390) as usize;
        let wrapper_out = exe::ptr::<u8>(0x1_43c4_8e48) as usize;
        exe::call_iat!(0x1_4264_5cb0, (rd::<usize>(dev_addr), wrapper_name, 0x28u32, wrapper_out) -> u32);
    }
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    exe::call_va!(0x1_4207_2ec0, (tls0 + 0x467e0) -> ());
    1
}

/// Original: `0x142059280` (244 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142059280(
    this: usize,
    a2: usize,
    _a3: u32,
    cmd: usize,
    a5: usize,
) {
    let mut entries_ptr: usize = 0;
    let mut entries_count: u32 = 0;
    let handle = rd::<usize>(this + 0x110);
    let entries_ptr_addr = (&mut entries_ptr as *mut usize) as usize;
    let entries_count_addr = (&mut entries_count as *mut u32) as usize;
    exe::call_iat!(
        0x1_4264_5c80,
        (handle, a2, 1u32, entries_ptr_addr, entries_count_addr) -> ()
    );
    if rd::<u8>(this + 0x14d) == 0 {
        exe::call_va!(0x1_411d_5e10, (this + 0x18) -> ());
    }
    let idx = rd::<u32>(this + 0x144) as usize;
    let pool_arr = rd::<usize>(this + 0xb8);
    let pool_elem = rd::<usize>(pool_arr + idx * 8);
    let nri = rd::<usize>(this + 0xe8);
    let set_desc_pool = rd::<usize>(nri + 0x158);
    exe::call_va!(set_desc_pool, (cmd, pool_elem) -> ());
    let mut i: u32 = 0;
    if entries_count > 0 {
        loop {
            let elem_ptr = entries_ptr + (i as usize) * 0x30;
            let cur_nri = rd::<usize>(this + 0xe8);
            let begin_fn = rd::<usize>(cur_nri + 0x1f8);
            exe::call_va!(begin_fn, (cmd, rd::<usize>(elem_ptr)) -> ());
            exe::call_va!(0x1_4205_8950, (this, cmd, pool_elem, elem_ptr, a5) -> ());
            let end_nri = rd::<usize>(this + 0xe8);
            let end_fn = rd::<usize>(end_nri + 0x200);
            exe::call_va!(end_fn, (cmd) -> ());
            i = i.wrapping_add(1);
            if i >= entries_count {
                break;
            }
        }
    }
}

/// Original: `0x1420593e0` (205 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420593e0(this: usize) {
    let rem = rd::<u32>(this + 0x148) % rd::<u32>(this + 0x140);
    let nri = rd::<usize>(this + 0xe8);
    wr::<u32>(this + 0x144, rem);
    let arr_b8 = rd::<usize>(this + 0xb8);
    let elem = rd::<usize>(arr_b8 + (rem as usize) * 8);
    let reset_fn = rd::<usize>(nri + 0x290);
    exe::call_va!(reset_fn, (elem, rem, rem) -> ());

    let cur_idx = rd::<u32>(this + 0x144) as usize;
    let arr_d0 = rd::<usize>(this + 0xd0);
    wr::<u64>(arr_d0 + cur_idx * 8, 0);

    if rd::<u8>(this + 0x14d) == 0 {
        let idx0 = rd::<u32>(this + 0x144) as usize;
        let vec_base = rd::<usize>(this + 0x28) + idx0 * 24;
        let end = rd::<usize>(vec_base + 0x8);
        let mut it = rd::<usize>(vec_base);
        while it != end {
            let cur_nri = rd::<usize>(this + 0xe8);
            let destroy_fn = rd::<usize>(cur_nri + 0xd8);
            exe::call_va!(destroy_fn, (rd::<usize>(it)) -> ());
            it += 8;
        }
        let idx1 = rd::<u32>(this + 0x144) as usize;
        let vec_base2 = rd::<usize>(this + 0x28) + idx1 * 24;
        let start = rd::<usize>(vec_base2);
        if start != rd::<usize>(vec_base2 + 0x8) {
            wr::<usize>(vec_base2 + 0x8, start);
        }
    }
    wr::<u32>(this + 0x148, rd::<u32>(this + 0x148).wrapping_add(1));
}

/// Original: `0x14205a300` (176 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205a300(
    this: usize,
    a2: usize,
    a3: usize,
    _a4: usize,
    a5: usize,
) -> u8 {
    let get_desc_fn = rd::<usize>(exe::ptr::<usize>(0x1_43c4_8ad0) as usize);
    let desc = exe::call_va!(get_desc_fn, (a3) -> usize);
    if rd::<u16>(desc + 0xa2) != 1 || rd::<u16>(desc + 0xa4) != 0x63 {
        return 0;
    }
    let ver_info = exe::call_iat!(0x1_4264_5c70, () -> usize);
    if rd::<u8>(ver_info + 0x1c) != 4 || rd::<u8>(ver_info + 0x1d) != 3 {
        return 0;
    }
    let status: u32 = exe::call_iat!(0x1_4264_5c88, (a2, this + 0x110) -> u32);
    if status != 0 {
        return 0;
    }
    let core_iface = exe::ptr::<u8>(0x1_43c4_8ad0) as usize;
    wr::<usize>(this + 0xf8, a3);
    wr::<usize>(this + 0xe8, core_iface);
    wr::<usize>(this + 0xf0, a5);
    exe::call_va!(0x1_4205_9ea0, (this) -> ());
    exe::call_va!(0x1_4205_97a0, (this) -> ());
    1
}

/// Original: `0x14205a3c0` (48 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205a3c0(a1: u64, a2: u8, a3: u8, a4: u8) -> u64 {
    let low_a3 = (a3 as u64) & 0x7f;
    let hi_a4 = (a4 as u64) << 7;
    let combined = ((low_a3 | hi_a4) << 1) | (a2 as u64);
    let masked_a1 = a1 & 0x0001_ffff_ffff_ffff;
    (combined << 49) | masked_a1
}

pub unsafe fn diff_d3d_14205a3c0(orig: usize, new: usize) -> Result<u32, String> {
    let mut cases = 0u32;
    let samples = [
        (0u64, 0u8, 0u8, 0u8),
        (0xffff_ffff_ffff_ffffu64, 1u8, 0x7fu8, 0xffu8),
        (0x1234_5678_9abc_def0u64, 0u8, 0xaa, 0x55),
        (0x0001_ffff_ffff_ffffu64, 0xff, 0x80, 0x01),
    ];
    for (a1, a2, a3, a4) in samples {
        let ro: u64 = exe::call_va!(orig, (a1, a2, a3, a4) -> u64);
        let rn: u64 = exe::call_va!(new, (a1, a2, a3, a4) -> u64);
        crate::diff_eq!(ro, rn, "packed handle");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x14205a400` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205a400(idx: u32) -> u8 {
    let table = exe::ptr::<u8>(0x1_4297_e550) as usize;
    rd::<u8>(table + (idx as usize))
}

pub unsafe fn diff_d3d_14205a400(orig: usize, new: usize) -> Result<u32, String> {
    let mut cases = 0u32;
    for idx in 0u32..32u32 {
        let ro: u8 = exe::call_va!(orig, (idx) -> u8);
        let rn: u8 = exe::call_va!(new, (idx) -> u8);
        crate::diff_eq!(ro, rn, "lookup table byte");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x14205a420` (61 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205a420(
    out: usize,
    state: usize,
    a3: u16,
    a4: u8,
    a5: u16,
) -> usize {
    wr::<u16>(state + 0x8, a5);
    let prev_w = rd::<u16>(state + 0x12);
    wr::<u16>(state + 0x10, prev_w);
    let prev_b = rd::<u8>(state + 0x15);
    wr::<u8>(state + 0x14, prev_b);
    wr::<u16>(state + 0xa, 1);
    wr::<u16>(state + 0x12, a3);
    wr::<u8>(state + 0x15, a4);
    wr::<u64>(out, rd::<u64>(state));
    wr::<u64>(out + 0x8, rd::<u64>(state + 0x8));
    wr::<u64>(out + 0x10, rd::<u64>(state + 0x10));
    out
}

pub unsafe fn diff_d3d_14205a420(orig: usize, new: usize) -> Result<u32, String> {
    let mut cases = 0u32;
    let mut buf_o = [0u8; 80];
    let mut buf_n = [0u8; 80];
    for i in 0..80usize {
        buf_o[i] = (i as u8).wrapping_mul(7).wrapping_add(3);
        buf_n[i] = buf_o[i];
    }
    let out_o = (buf_o.as_mut_ptr() as usize) + 4;
    let state_o = out_o + 32;
    let out_n = (buf_n.as_mut_ptr() as usize) + 4;
    let state_n = out_n + 32;
    let ro: usize = exe::call_va!(orig, (out_o, state_o, 0x1234u16, 0x56u8, 0x789au16) -> usize);
    let rn: usize = exe::call_va!(new, (out_n, state_n, 0x1234u16, 0x56u8, 0x789au16) -> usize);
    crate::diff_eq!(ro - out_o, rn - out_n, "return offset");
    for k in 0..24usize {
        crate::diff_eq!(rd::<u8>(out_o + k), rd::<u8>(out_n + k), "out byte");
        crate::diff_eq!(rd::<u8>(state_o + k), rd::<u8>(state_n + k), "state byte");
    }
    cases += 1;
    Ok(cases)
}

/// Original: `0x14205a470` (42 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205a470(
    out: usize,
    ptr: u64,
    _a3: usize,
    _a4: usize,
    a5: u16,
) -> usize {
    wr::<u16>(out + 0x8, a5);
    wr::<u32>(out + 0x14, 0x808);
    wr::<u64>(out, ptr);
    wr::<u32>(out + 0x10, 0);
    wr::<u32>(out + 0xa, 1);
    wr::<u16>(out + 0xe, 0);
    out
}

pub unsafe fn diff_d3d_14205a470(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0xffu8; 40];
    let mut buf_n = [0xffu8; 40];
    let out_o = (buf_o.as_mut_ptr() as usize) + 4;
    let out_n = (buf_n.as_mut_ptr() as usize) + 4;
    let ro: usize = exe::call_va!(orig, (out_o, 0xdead_beef_cafe_babeu64, 0usize, 0usize, 0x4321u16) -> usize);
    let rn: usize = exe::call_va!(new, (out_n, 0xdead_beef_cafe_babeu64, 0usize, 0usize, 0x4321u16) -> usize);
    crate::diff_eq!(ro - out_o, rn - out_n, "ret");
    for k in 0..24usize {
        crate::diff_eq!(rd::<u8>(out_o + k), rd::<u8>(out_n + k), "struct byte");
    }
    Ok(1)
}

/// Original: `0x14205a4a0` (68 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205a4a0(
    out: usize,
    a2: u8,
    a3: u16,
    a4: u16,
    a5: u16,
    _a6: usize,
    a7: u16,
) -> usize {
    wr::<u64>(out, 0);
    wr::<u64>(out + 0x8, 0);
    wr::<u64>(out + 0x10, 0);
    wr::<u16>(out + 0x2, a7);
    wr::<u8>(out + 0x4, a2);
    wr::<u16>(out + 0xc, a5);
    wr::<u8>(out, 1);
    wr::<u16>(out + 0x6, a3);
    wr::<u16>(out + 0x8, a4);
    wr::<u16>(out + 0xa, 1);
    wr::<u16>(out + 0xe, 1);
    wr::<u8>(out + 0x10, 1);
    out
}

pub unsafe fn diff_d3d_14205a4a0(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0xffu8; 40];
    let mut buf_n = [0xffu8; 40];
    let out_o = (buf_o.as_mut_ptr() as usize) + 4;
    let out_n = (buf_n.as_mut_ptr() as usize) + 4;
    let ro: usize = exe::call_va!(
        orig,
        (out_o, 0x11u8, 0x2233u16, 0x4455u16, 0x6677u16, 0usize, 0x8899u16) -> usize
    );
    let rn: usize = exe::call_va!(
        new,
        (out_n, 0x11u8, 0x2233u16, 0x4455u16, 0x6677u16, 0usize, 0x8899u16) -> usize
    );
    crate::diff_eq!(ro - out_o, rn - out_n, "ret");
    for k in 0..24usize {
        crate::diff_eq!(rd::<u8>(out_o + k), rd::<u8>(out_n + k), "struct byte");
    }
    Ok(1)
}

/// Original: `0x14205a4f0` (199 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_14205a4f0() {
    let lock_addr = exe::ptr::<u8>(0x1_43a3_2d38) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_addr) -> ());

    let vec_addr = exe::ptr::<usize>(0x1_43a6_3248) as usize;
    let count = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3250) as usize) as usize;
    let base = rd::<usize>(vec_addr);
    if count != 0 {
        let mut first_expired: i32 = -1;
        let mut idx: usize = 0;
        let mut ptr = base + 0x8;
        let cur_frame = rd::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize);
        while idx < count {
            let expired = rd::<u8>(ptr + 0x19) != 0
                && cur_frame > rd::<u64>(ptr).wrapping_add(0x3e8);
            if first_expired == -1 {
                if expired {
                    first_expired = idx as i32;
                }
            } else if !expired {
                first_expired = -1;
            }
            idx += 1;
            ptr += 0x28;
        }
        if first_expired != -1 {
            let start_idx = (first_expired as isize) as usize;
            exe::call_va!(0x1_411d_6b80, (ptr, start_idx, idx, vec_addr) -> ());
        }
    }

    exe::call_iat!(0x1_4264_5598, (lock_addr) -> ());
}

/// Original: `0x14205a5c0` (49 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_14205a5c0() {
    let lock_addr = exe::ptr::<u8>(0x1_43a3_2d38) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_addr) -> ());
    let vec_addr = exe::ptr::<usize>(0x1_43a6_3248) as usize;
    exe::call_va!(0x1_411d_6a90, (vec_addr) -> ());
    exe::call_iat!(0x1_4264_5598, (lock_addr) -> ());
}

/// Original: `0x14205a600` (151 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205a600(key: u64, clear_timestamp: u8) -> u8 {
    let lock_addr = exe::ptr::<u8>(0x1_43a3_2d38) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_addr) -> ());

    let vec_addr = exe::ptr::<usize>(0x1_43a6_3248) as usize;
    let count_addr = exe::ptr::<u32>(0x1_43a6_3250) as usize;
    let mut found: u8 = 0;
    let mut idx: usize = 0;
    if rd::<u32>(count_addr) > 0 {
        let mut offset: usize = 0;
        loop {
            let base = rd::<usize>(vec_addr);
            if rd::<u64>(base + offset) == key {
                found = 1;
                wr::<u8>(base + offset + 0x21, 1);
                if clear_timestamp != 0 {
                    let base2 = rd::<usize>(vec_addr);
                    wr::<u64>(base2 + offset + 0x8, 0);
                }
            }
            let total = rd::<u32>(count_addr) as usize;
            idx += 1;
            offset += 0x28;
            if idx >= total {
                break;
            }
        }
    }

    exe::call_iat!(0x1_4264_5598, (lock_addr) -> ());
    found
}

/// Original: `0x14205aa70` (125 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205aa70(out: usize) -> usize {
    wr::<usize>(out + 0x08, exe::ptr::<u8>(0x1_433d_c4e8) as usize);
    wr::<usize>(out + 0x10, exe::ptr::<u8>(0x1_433d_c430) as usize);
    wr::<usize>(out + 0x18, exe::ptr::<u8>(0x1_433d_c670) as usize);
    wr::<usize>(out + 0x20, exe::ptr::<u8>(0x1_433d_c5b0) as usize);
    wr::<usize>(out + 0x28, exe::ptr::<u8>(0x1_433d_c748) as usize);
    wr::<usize>(out + 0x30, exe::ptr::<u8>(0x1_433d_c6c0) as usize);
    wr::<usize>(out + 0x38, exe::ptr::<u8>(0x1_433d_c548) as usize);
    wr::<usize>(out + 0x40, exe::ptr::<u8>(0x1_433d_c528) as usize);
    wr::<usize>(out + 0x48, exe::ptr::<u8>(0x1_433d_c578) as usize);
    wr::<usize>(out + 0x50, exe::ptr::<u8>(0x1_433d_c550) as usize);
    wr::<usize>(out + 0x58, exe::ptr::<u8>(0x1_433d_c770) as usize);
    out
}

pub unsafe fn diff_d3d_14205aa70(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0u8; 0x68];
    let mut buf_n = [0u8; 0x68];
    let out_o = (buf_o.as_mut_ptr() as usize) + 4;
    let out_n = (buf_n.as_mut_ptr() as usize) + 4;
    let ro: usize = exe::call_va!(orig, (out_o) -> usize);
    let rn: usize = exe::call_va!(new, (out_n) -> usize);
    crate::diff_eq!(ro - out_o, rn - out_n, "ret");
    for off in (0x08..=0x58usize).step_by(8) {
        crate::diff_eq!(rd::<usize>(out_o + off), rd::<usize>(out_n + off), "str ptr");
    }
    Ok(1)
}

/// Original: `0x14205ad20` (17 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205ad20(this: usize) -> u64 {
    let num = rd::<u64>(this + 0x10);
    if num == 0 {
        return 0;
    }
    num / rd::<u64>(this + 0x8)
}

pub unsafe fn diff_d3d_14205ad20(orig: usize, new: usize) -> Result<u32, String> {
    let mut cases = 0u32;
    let mut buf = [0u8; 32];
    let ptr = (buf.as_mut_ptr() as usize) + 4;
    for (denom, num) in [(10u64, 0u64), (7u64, 49u64), (3u64, 100u64)] {
        wr::<u64>(ptr + 0x8, denom);
        wr::<u64>(ptr + 0x10, num);
        let ro: u64 = exe::call_va!(orig, (ptr) -> u64);
        let rn: u64 = exe::call_va!(new, (ptr) -> u64);
        crate::diff_eq!(ro, rn, "quotient");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x14205ad40` (16 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205ad40(handle: u64, rdx_unused: usize) -> usize {
    let bucket = (handle >> 32) as i16;
    if bucket < 0 {
        return 0;
    }
    d3d_14205ad50(handle, rdx_unused, (bucket as isize) as usize)
}

/// Original: `0x14205ad50` (48 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205ad50(handle: u64, _rdx: usize, bucket: usize) -> usize {
    let table = exe::ptr::<usize>(0x1_43a6_3260) as usize;
    let idx = (handle as u32) as usize;
    let gen = (handle >> 48) as u16;
    let arr = rd::<usize>(table.wrapping_add(bucket.wrapping_mul(16)));
    let entry = arr.wrapping_add(idx.wrapping_mul(0xa0));
    if rd::<u16>(entry + 0x9c) == gen {
        entry
    } else {
        0
    }
}

/// Original: `0x14205ada0` (115 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205ada0(handle: u64) {
    let bucket = (handle >> 32) as i16;
    if bucket < 0 {
        return;
    }
    let lock_addr = exe::ptr::<u8>(0x1_43a3_2f28) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_addr) -> ());

    let table = exe::ptr::<usize>(0x1_43a6_3260) as usize;
    let idx = (handle as u32) as usize;
    let gen = (handle >> 48) as u16;
    let bucket_off = ((bucket as isize) as usize).wrapping_mul(16);
    let arr = rd::<usize>(table.wrapping_add(bucket_off));
    let entry = arr.wrapping_add(idx.wrapping_mul(0xa0));
    if rd::<u16>(entry + 0x9c) == gen {
        wr::<u32>(entry + 0x98, rd::<u32>(entry + 0x98).wrapping_add(1));
    }

    exe::call_iat!(0x1_4264_5598, (lock_addr) -> ());
}

/// Original: `0x14205afd0` (171 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205afd0(bucket: i32, target: usize) -> u64 {
    if target == 0 {
        return u64::MAX;
    }
    let lock_addr = exe::ptr::<u8>(0x1_43a3_2f28) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_addr) -> ());

    let table = exe::ptr::<u8>(0x1_43a6_3260) as usize;
    let slot = table.wrapping_add(((bucket as isize) as usize).wrapping_mul(16));
    let count = rd::<u32>(slot + 0x8) as usize;
    let mut result = u64::MAX;
    if count != 0 {
        let mut entry = rd::<usize>(slot);
        let mut idx: usize = 0;
        while idx < count {
            if rd::<usize>(entry + 0x90) == target {
                let gen = rd::<u16>(entry + 0x9c) as u64;
                wr::<u32>(entry + 0x98, rd::<u32>(entry + 0x98).wrapping_add(1));
                let high = (gen << 16) | ((bucket as u16) as u64);
                result = (high << 32) | ((idx as u32) as u64);
                break;
            }
            idx += 1;
            entry += 0xa0;
        }
    }

    exe::call_iat!(0x1_4264_5598, (lock_addr) -> ());
    result
}

// ============================================================================
// Differential tests for remaining functions
// ============================================================================

use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

struct Tls0Guard {
    tls_arr: usize,
    saved: usize,
}

impl Tls0Guard {
    unsafe fn new() -> Self {
        let tls_arr = exe::tls_pointer();
        let saved = rd::<usize>(tls_arr);
        Self { tls_arr, saved }
    }
    unsafe fn set(&self, val: usize) {
        wr::<usize>(self.tls_arr, val);
    }
}

impl Drop for Tls0Guard {
    fn drop(&mut self) {
        unsafe {
            wr::<usize>(self.tls_arr, self.saved);
        }
    }
}

struct SnapGuard(crate::difftest::Snapshot);

impl SnapGuard {
    unsafe fn take(at: usize, len: usize) -> Self {
        Self(crate::difftest::Snapshot::take(at as *mut u8, len))
    }
}

impl Drop for SnapGuard {
    fn drop(&mut self) {
        unsafe {
            self.0.restore();
        }
    }
}

unsafe extern "win64" fn fake_srw_noop(_lock: usize) {}

unsafe extern "win64" fn fake_get_tid() -> u32 {
    0x4242
}

static FAKE_55F20_CTX: AtomicUsize = AtomicUsize::new(0);

unsafe extern "win64" fn fake_55c40() -> usize {
    FAKE_55F20_CTX.load(Ordering::SeqCst)
}

unsafe extern "win64" fn fake_55f80(_arg: usize) -> i32 {
    2
}

unsafe extern "win64" fn fake_632b0(inner: usize) {
    wr::<u64>(inner, 0x1234_5678_9abc_def0);
}

unsafe extern "win64" fn fake_24069a0() -> f64 {
    123.5
}

pub unsafe fn diff_d3d_142055f20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m1 = crate::difftest::Mock::new(0x1_4205_5c40, fake_55c40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_5f80, fake_55f80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4206_32b0, fake_632b0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_69a0, fake_24069a0 as *const ())?;

    let mut this_buf = [0u8; 0x60];
    let mut table_buf = [0usize; 4];
    let mut ctx_buf = [0u8; 0x20];
    let mut inner_buf = [0u64; 2];

    let this = this_buf.as_mut_ptr() as usize;
    let table_ptr = table_buf.as_mut_ptr() as usize;
    let ctx_ptr = ctx_buf.as_mut_ptr() as usize;
    let inner_ptr = inner_buf.as_mut_ptr() as usize;
    FAKE_55F20_CTX.store(ctx_ptr, Ordering::SeqCst);

    for tag in [-1i32, 5i32] {
        let mut run_one = |f: F| {
            this_buf.fill(0);
            table_buf.fill(0);
            ctx_buf.fill(0);
            inner_buf.fill(0);
            wr::<i32>(this, tag);
            wr::<usize>(this + 0x50, table_ptr);
            wr::<usize>(ctx_ptr, inner_ptr);
            f(this);
            (this_buf, table_buf, ctx_buf, inner_buf)
        };

        let o_state = run_one(f_orig);
        let n_state = run_one(f_new);
        crate::diff_eq!(o_state, n_state, "d3d_142055f20");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_142055f80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let lock_base = exe::ptr::<u8>(0x1_4385_79d0) as usize;
    let counter_addr = exe::ptr::<u32>(0x1_43a2_b038) as usize;
    let _s1 = SnapGuard::take(lock_base, 16);
    let _s2 = SnapGuard::take(counter_addr, 4);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_5790, fake_get_tid as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_srw_noop as *const ())?;
    let _im3 = crate::difftest::IatMock::new(0x1_4264_5598, fake_srw_noop as *const ())?;

    let tls_guard = Tls0Guard::new();
    let mut tls_buf = vec![0u8; 0x6400];
    let tls0 = tls_buf.as_mut_ptr() as usize;
    tls_guard.set(tls0);

    let mut arg_val: u32 = 0x19;
    let arg_ptr = (&mut arg_val as *mut u32) as usize;

    for init_slot in [-1i32, 3i32] {
        let setup = || {
            wr::<u64>(lock_base, 0);
            wr::<u32>(lock_base + 8, u32::MAX);
            wr::<u32>(lock_base + 12, 0);
            wr::<u32>(counter_addr, 7);
            wr::<i32>(tls0 + 0x6350, init_slot);
        };

        setup();
        let ro = f_orig(arg_ptr);
        let o_state = (
            ro,
            rd::<i32>(tls0 + 0x6350),
            rd::<u32>(counter_addr),
            rd::<u32>(lock_base + 8),
            rd::<u32>(lock_base + 12),
        );

        setup();
        let rn = f_new(arg_ptr);
        let n_state = (
            rn,
            rd::<i32>(tls0 + 0x6350),
            rd::<u32>(counter_addr),
            rd::<u32>(lock_base + 8),
            rd::<u32>(lock_base + 12),
        );
        crate::diff_eq!(o_state, n_state, "d3d_142055f80");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_562e0_read_u8(stream: usize, out: usize) {
    let data = rd::<usize>(stream + 0x08);
    let cur = rd::<usize>(stream + 0x10);
    wr::<u8>(out, rd::<u8>(data + cur));
    wr::<usize>(stream + 0x10, cur + 1);
}

pub unsafe fn diff_d3d_1420562e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 16];
    vtbl[0x70 / 8] = fake_562e0_read_u8 as *const () as usize;

    let stream_bytes: [u8; 8] = [4, 0x11, 0x22, 0x33, 0x44, 0, 0, 0];
    let mut stream_obj = [0usize; 3];
    stream_obj[0] = vtbl.as_ptr() as usize;
    stream_obj[1] = stream_bytes.as_ptr() as usize;
    let stream_ptr = stream_obj.as_mut_ptr() as usize;

    let mut buf = [0xffu8; 16];
    let buf_ptr = buf.as_mut_ptr() as usize;

    for first_byte in [0u8, 4u8] {
        let mut cur_bytes = stream_bytes;
        cur_bytes[0] = first_byte;
        stream_obj[1] = cur_bytes.as_ptr() as usize;

        stream_obj[2] = 0;
        buf.fill(0xff);
        let ro = f_orig(stream_ptr, buf_ptr);
        let o_state = (ro, stream_obj[2], buf);

        stream_obj[2] = 0;
        buf.fill(0xff);
        let rn = f_new(stream_ptr, buf_ptr);
        let n_state = (rn, stream_obj[2], buf);

        crate::diff_eq!(o_state, n_state, "d3d_1420562e0");
    }
    Ok(2)
}

static RELEASED_56660: AtomicU64 = AtomicU64::new(0);

unsafe extern "win64" fn fake_564b0(item: usize) {
    RELEASED_56660.fetch_add(item as u64, Ordering::SeqCst);
}

unsafe extern "win64" fn fake_fence_get_completed(fence: usize) -> u64 {
    rd::<u64>(fence + 8)
}

pub unsafe fn diff_d3d_142056660(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let vec_global = exe::ptr::<usize>(0x1_43a6_31e8) as usize;
    let _s1 = SnapGuard::take(vec_global, 16);
    let _m1 = crate::difftest::Mock::new(0x1_4205_64b0, fake_564b0 as *const ())?;

    let mut fence_vtbl = [0usize; 10];
    fence_vtbl[0x40 / 8] = fake_fence_get_completed as *const () as usize;
    let fence_obj: [usize; 2] = [fence_vtbl.as_ptr() as usize, 15];
    let fence_ptr = fence_obj.as_ptr() as usize;

    let mut arr = [0u64; 9];
    let arr_ptr = arr.as_mut_ptr() as usize;

    let mut run_56660 = |f: F| {
        arr = [
            0x101, 10, fence_ptr as u64, // completed (15 >= 10) -> remove
            0x202, 20, fence_ptr as u64, // pending (15 < 20) -> keep
            0x303, 5, 0,                 // null fence -> remove
        ];
        wr::<usize>(vec_global, arr_ptr);
        wr::<u32>(vec_global + 8, 3);
        wr::<u32>(vec_global + 12, 4);
        RELEASED_56660.store(0, Ordering::SeqCst);
        f();
        (rd::<u32>(vec_global + 8), arr, RELEASED_56660.load(Ordering::SeqCst))
    };

    let o_state = run_56660(f_orig);
    let n_state = run_56660(f_new);
    crate::diff_eq!(o_state, n_state, "d3d_142056660");
    Ok(1)
}

unsafe extern "win64" fn fake_734e0(out: usize, _arg: u32) -> usize {
    wr::<u64>(out, 10);
    wr::<u64>(out + 8, 0);
    out
}

unsafe extern "win64" fn fake_11d5690(_rcx: usize, new_len: usize) {
    let cap_addr = exe::ptr::<u32>(0x1_43a6_31f4) as usize;
    wr::<u32>(cap_addr, (new_len as u32) + 2);
}

pub unsafe fn diff_d3d_142056760(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let val0_global = exe::ptr::<u64>(0x1_43a3_5918) as usize;
    let vec_global = exe::ptr::<usize>(0x1_43a6_31e8) as usize;
    let _s1 = SnapGuard::take(val0_global, 8);
    let _s2 = SnapGuard::take(vec_global, 16);
    let _m1 = crate::difftest::Mock::new(0x1_4207_34e0, fake_734e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_5690, fake_11d5690 as *const ())?;

    let mut arr = [0u64; 12];
    let arr_ptr = arr.as_mut_ptr() as usize;

    for (init_cnt, init_cap) in [(1u32, 3u32), (2u32, 2u32)] {
        let mut run_one = |f: F| {
            arr.fill(0);
            wr::<u64>(val0_global, 0x9988_7766);
            wr::<usize>(vec_global, arr_ptr);
            wr::<u32>(vec_global + 8, init_cnt);
            wr::<u32>(vec_global + 12, init_cap);
            f();
            (rd::<u32>(vec_global + 8), rd::<u32>(vec_global + 12), arr)
        };

        let o_state = run_one(f_orig);
        let n_state = run_one(f_new);
        crate::diff_eq!(o_state, n_state, "d3d_142056760");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_56ce0_cb(obj: usize, arg: usize) {
    wr::<usize>(obj + 8, arg ^ 0x55aa);
}

pub unsafe fn diff_d3d_142056ce0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let lock_base = exe::ptr::<u8>(0x1_4385_7a20) as usize;
    let arg_global = exe::ptr::<usize>(0x1_43a3_5918) as usize;
    let vec_global = exe::ptr::<usize>(0x1_43a6_31f8) as usize;
    let _s1 = SnapGuard::take(lock_base, 16);
    let _s2 = SnapGuard::take(arg_global, 8);
    let _s3 = SnapGuard::take(vec_global, 16);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_5790, fake_get_tid as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_srw_noop as *const ())?;
    let _im3 = crate::difftest::IatMock::new(0x1_4264_5598, fake_srw_noop as *const ())?;

    let vtbl: [usize; 2] = [0, fake_56ce0_cb as *const () as usize];
    let mut obj0: [usize; 2] = [vtbl.as_ptr() as usize, 0];
    let mut obj1: [usize; 2] = [vtbl.as_ptr() as usize, 0];
    let ptrs: [usize; 2] = [obj0.as_mut_ptr() as usize, obj1.as_mut_ptr() as usize];

    let mut run_56ce0 = |f: F| {
        wr::<u64>(lock_base, 0);
        wr::<u32>(lock_base + 8, u32::MAX);
        wr::<u32>(lock_base + 12, 0);
        wr::<usize>(arg_global, 0x1234_0000);
        wr::<usize>(vec_global, ptrs.as_ptr() as usize);
        wr::<u32>(vec_global + 8, 2);
        obj0[1] = 0;
        obj1[1] = 0;
        f();
        (obj0[1], obj1[1], rd::<u32>(lock_base + 8), rd::<u32>(lock_base + 12))
    };

    let o_state = run_56ce0(f_orig);
    let n_state = run_56ce0(f_new);
    crate::diff_eq!(o_state, n_state, "d3d_142056ce0");
    Ok(1)
}

pub unsafe fn diff_d3d_142057f90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let dst = exe::ptr::<u8>(0x1_4385_7c50) as usize;
    let _s1 = SnapGuard::take(dst, 0x88);

    let mut src_buf = [0u8; 0x90];
    for (i, b) in src_buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(11).wrapping_add(5);
    }
    let src = (src_buf.as_ptr() as usize) + 4;

    std::ptr::write_bytes(dst as *mut u8, 0, 0x88);
    f_orig(src);
    let o_bytes = std::slice::from_raw_parts(dst as *const u8, 0x88).to_vec();

    std::ptr::write_bytes(dst as *mut u8, 0, 0x88);
    f_new(src);
    let n_bytes = std::slice::from_raw_parts(dst as *const u8, 0x88).to_vec();

    crate::diff_eq!(o_bytes, n_bytes, "d3d_142057f90");
    Ok(1)
}

pub unsafe fn diff_d3d_142057fe0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let dst = exe::ptr::<u8>(0x1_4385_7bf0) as usize;
    let _s1 = SnapGuard::take(dst, 0x60);

    let mut src_buf = [0u8; 0x68];
    for (i, b) in src_buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(13).wrapping_add(7);
    }
    let src = (src_buf.as_ptr() as usize) + 4;

    std::ptr::write_bytes(dst as *mut u8, 0, 0x60);
    f_orig(src);
    let o_bytes = std::slice::from_raw_parts(dst as *const u8, 0x60).to_vec();

    std::ptr::write_bytes(dst as *mut u8, 0, 0x60);
    f_new(src);
    let n_bytes = std::slice::from_raw_parts(dst as *const u8, 0x60).to_vec();

    crate::diff_eq!(o_bytes, n_bytes, "d3d_142057fe0");
    Ok(1)
}

static STATE_58140: AtomicU64 = AtomicU64::new(0);

unsafe extern "win64" fn fake_58180() {
    STATE_58140.fetch_add(1, Ordering::SeqCst);
}

unsafe extern "win64" fn fake_nri_destroy_dev(dev: usize) {
    STATE_58140.fetch_add(dev as u64, Ordering::SeqCst);
}

pub unsafe fn diff_d3d_142058140(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let dev_addr = exe::ptr::<usize>(0x1_43a3_59f8) as usize;
    let _s1 = SnapGuard::take(dev_addr, 8);
    let _m1 = crate::difftest::Mock::new(0x1_4205_8180, fake_58180 as *const ())?;
    let _im1 = crate::difftest::IatMock::new(0x1_4264_5ca0, fake_nri_destroy_dev as *const ())?;

    for dev_val in [0usize, 0x1234_5000usize] {
        wr::<usize>(dev_addr, dev_val);
        STATE_58140.store(0, Ordering::SeqCst);
        f_orig();
        let o_state = (rd::<usize>(dev_addr), STATE_58140.load(Ordering::SeqCst));

        wr::<usize>(dev_addr, dev_val);
        STATE_58140.store(0, Ordering::SeqCst);
        f_new();
        let n_state = (rd::<usize>(dev_addr), STATE_58140.load(Ordering::SeqCst));

        crate::diff_eq!(o_state, n_state, "d3d_142058140");
    }
    Ok(2)
}

static STATE_58180: AtomicU64 = AtomicU64::new(0);
static FENCE_58180: AtomicUsize = AtomicUsize::new(0);

unsafe extern "win64" fn fake_734e0_with_fence(out: usize, _arg: u32) -> usize {
    wr::<u64>(out, 10);
    wr::<usize>(out + 8, FENCE_58180.load(Ordering::SeqCst));
    out
}

unsafe extern "win64" fn fake_destroy_handle_58180(handle: usize) {
    STATE_58180.fetch_add(handle as u64, Ordering::SeqCst);
}

unsafe extern "win64" fn fake_58640(ctx: usize) -> usize {
    STATE_58180.fetch_add(0x40, Ordering::SeqCst);
    ctx
}

unsafe extern "win64" fn fake_11d5f40(freed: usize, flag: u32) {
    STATE_58180.fetch_add((freed as u64) ^ (flag as u64), Ordering::SeqCst);
}

pub unsafe fn diff_d3d_142058180(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let ctx_addr = exe::ptr::<usize>(0x1_43a3_5a98) as usize;
    let slot_a_base = exe::ptr::<usize>(0x1_43c4_8a60) as usize;
    let destroy_fn_addr = exe::ptr::<usize>(0x1_43c4_8ba0) as usize;
    let slot_b_base = exe::ptr::<usize>(0x1_43c4_8e70) as usize;

    let _s1 = SnapGuard::take(ctx_addr, 8);
    let _s2 = SnapGuard::take(slot_a_base, 14 * 8);
    let _s3 = SnapGuard::take(destroy_fn_addr, 8);
    let _s4 = SnapGuard::take(slot_b_base, 14 * 0x18);
    let _m1 = crate::difftest::Mock::new(0x1_4207_34e0, fake_734e0_with_fence as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_8640, fake_58640 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411d_5f40, fake_11d5f40 as *const ())?;

    let mut fence_vtbl = [0usize; 10];
    fence_vtbl[0x40 / 8] = fake_fence_get_completed as *const () as usize;
    let fence_obj: [usize; 2] = [fence_vtbl.as_ptr() as usize, 15];
    FENCE_58180.store(fence_obj.as_ptr() as usize, Ordering::SeqCst);

    for ctx_val in [0usize, 0x8000_1000usize] {
        let setup = || {
            wr::<usize>(ctx_addr, ctx_val);
            wr::<usize>(destroy_fn_addr, fake_destroy_handle_58180 as *const () as usize);
            std::ptr::write_bytes(slot_a_base as *mut u8, 0, 14 * 8);
            std::ptr::write_bytes(slot_b_base as *mut u8, 0, 14 * 0x18);
            wr::<usize>(slot_a_base, 0xaa);
            wr::<usize>(slot_b_base, 0x111);
            wr::<usize>(slot_a_base + 3 * 8, 0xbb);
            wr::<usize>(slot_b_base + 3 * 0x18, 0x222);
            STATE_58180.store(0, Ordering::SeqCst);
        };

        setup();
        f_orig();
        let o_state = (
            STATE_58180.load(Ordering::SeqCst),
            std::slice::from_raw_parts(slot_a_base as *const u8, 14 * 8).to_vec(),
            std::slice::from_raw_parts(slot_b_base as *const u8, 14 * 0x18).to_vec(),
        );

        setup();
        f_new();
        let n_state = (
            STATE_58180.load(Ordering::SeqCst),
            std::slice::from_raw_parts(slot_a_base as *const u8, 14 * 8).to_vec(),
            std::slice::from_raw_parts(slot_b_base as *const u8, 14 * 0x18).to_vec(),
        );
        crate::diff_eq!(o_state, n_state, "d3d_142058180");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_nri_create_dev(desc: usize, out_dev: usize) -> u32 {
    let d3d_dev = rd::<usize>(desc);
    let queue = rd::<usize>(desc + 8);
    wr::<usize>(out_dev, d3d_dev ^ queue ^ 0x7000);
    0
}

unsafe extern "win64" fn fake_nri_get_iface(
    dev: usize,
    _name: usize,
    size: u32,
    out_iface: usize,
) -> u32 {
    wr::<usize>(out_iface, dev ^ (size as usize));
    0
}

unsafe extern "win64" fn fake_72ec0(ctx: usize) {
    wr::<u64>(ctx, 0xcafe_1234_5678);
}

pub unsafe fn diff_d3d_142058540(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let dev_addr = exe::ptr::<usize>(0x1_43a3_59f8) as usize;
    let d3d_dev_global = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let queue_holder_global = exe::ptr::<usize>(0x1_4397_fc60) as usize;
    let core_out = exe::ptr::<u8>(0x1_43c4_8ad0) as usize;

    let _s1 = SnapGuard::take(dev_addr, 8);
    let _s2 = SnapGuard::take(d3d_dev_global, 8);
    let _s3 = SnapGuard::take(queue_holder_global, 8);
    let _s4 = SnapGuard::take(core_out, 0x3a0);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_5ca8, fake_nri_create_dev as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_5cb0, fake_nri_get_iface as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4207_2ec0, fake_72ec0 as *const ())?;

    let tls_guard = Tls0Guard::new();
    let mut tls_buf = vec![0u8; 0x47000];
    let tls0 = tls_buf.as_mut_ptr() as usize;
    tls_guard.set(tls0);

    let queue_holder: [usize; 3] = [0, 0, 0x2000_0000];

    for init_dev in [0usize, 0x9000_0000usize] {
        let setup = || {
            wr::<usize>(dev_addr, init_dev);
            wr::<usize>(d3d_dev_global, 0x1000_0000);
            wr::<usize>(queue_holder_global, queue_holder.as_ptr() as usize);
            std::ptr::write_bytes(core_out as *mut u8, 0, 0x3a0);
            wr::<u8>(tls0 + 0x70, 1);
            wr::<u64>(tls0 + 0x467e0, 0);
        };

        setup();
        let ro = f_orig();
        let o_state = (
            ro,
            rd::<usize>(dev_addr),
            rd::<usize>(core_out),
            rd::<usize>(core_out + 0x350),
            rd::<usize>(core_out + 0x378),
            rd::<u64>(tls0 + 0x467e0),
        );

        setup();
        let rn = f_new();
        let n_state = (
            rn,
            rd::<usize>(dev_addr),
            rd::<usize>(core_out),
            rd::<usize>(core_out + 0x350),
            rd::<usize>(core_out + 0x378),
            rd::<u64>(tls0 + 0x467e0),
        );
        crate::diff_eq!(o_state, n_state, "d3d_142058540");
    }
    Ok(2)
}

static ENTRIES_59280: AtomicUsize = AtomicUsize::new(0);
static STATE_59280: AtomicU64 = AtomicU64::new(0);

unsafe extern "win64" fn fake_nrd_get_dispatches(
    handle: usize,
    a2: usize,
    flag: u32,
    out_ptr: usize,
    out_cnt: usize,
) {
    STATE_59280.fetch_add((handle ^ a2 ^ (flag as usize)) as u64, Ordering::SeqCst);
    wr::<usize>(out_ptr, ENTRIES_59280.load(Ordering::SeqCst));
    wr::<u32>(out_cnt, 2);
}

unsafe extern "win64" fn fake_11d5e10(p: usize) {
    wr::<u32>(p, 0x5e10);
}

unsafe extern "win64" fn fake_nri_set_desc_pool(cmd: usize, pool: usize) {
    wr::<usize>(cmd, pool);
}

unsafe extern "win64" fn fake_nri_begin_annotation(cmd: usize, name: usize) {
    wr::<usize>(cmd + 8, rd::<usize>(cmd + 8).wrapping_add(name));
}

unsafe extern "win64" fn fake_58950(
    this: usize,
    cmd: usize,
    pool: usize,
    elem: usize,
    a5: usize,
) {
    let v = (this ^ pool ^ rd::<usize>(elem) ^ a5) as u64;
    wr::<u64>(cmd + 16, rd::<u64>(cmd + 16).wrapping_add(v));
}

unsafe extern "win64" fn fake_nri_end_annotation(cmd: usize) {
    wr::<u32>(cmd + 24, rd::<u32>(cmd + 24).wrapping_add(1));
}

pub unsafe fn diff_d3d_142059280(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _im1 =
        crate::difftest::IatMock::new(0x1_4264_5c80, fake_nrd_get_dispatches as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_411d_5e10, fake_11d5e10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_8950, fake_58950 as *const ())?;

    let mut nri = [0usize; 0x42];
    nri[0x158 / 8] = fake_nri_set_desc_pool as *const () as usize;
    nri[0x1f8 / 8] = fake_nri_begin_annotation as *const () as usize;
    nri[0x200 / 8] = fake_nri_end_annotation as *const () as usize;

    let pools: [usize; 2] = [0xaaaa_0001, 0xbbbb_0002];
    let mut entries = [0u8; 2 * 0x30];
    wr::<usize>(entries.as_mut_ptr() as usize, 0x111);
    wr::<usize>((entries.as_mut_ptr() as usize) + 0x30, 0x222);
    ENTRIES_59280.store(entries.as_ptr() as usize, Ordering::SeqCst);

    let mut this_buf = [0u8; 0x160];
    let mut cmd_buf = [0u8; 0x20];
    let this = this_buf.as_mut_ptr() as usize;
    let cmd = cmd_buf.as_mut_ptr() as usize;

    for flag_14d in [0u8, 1u8] {
        let mut run_one = |f: F| {
            this_buf.fill(0);
            cmd_buf.fill(0);
            wr::<usize>(this + 0xb8, pools.as_ptr() as usize);
            wr::<usize>(this + 0xe8, nri.as_ptr() as usize);
            wr::<usize>(this + 0x110, 0x7777);
            wr::<u32>(this + 0x144, 1);
            wr::<u8>(this + 0x14d, flag_14d);
            STATE_59280.store(0, Ordering::SeqCst);
            f(this, 0x8888, 0, cmd, 0x9999);
            (this_buf, cmd_buf, STATE_59280.load(Ordering::SeqCst))
        };

        let o_state = run_one(f_orig);
        let n_state = run_one(f_new);
        crate::diff_eq!(o_state, n_state, "d3d_142059280");
    }
    Ok(2)
}

static STATE_593E0: AtomicU64 = AtomicU64::new(0);

unsafe extern "win64" fn fake_nri_reset_pool(elem: usize, rdx: u32, r8: u32) {
    STATE_593E0.fetch_add((elem as u64) ^ ((rdx as u64) << 8) ^ (r8 as u64), Ordering::SeqCst);
}

unsafe extern "win64" fn fake_nri_destroy_desc(desc: usize) {
    STATE_593E0.fetch_add(desc as u64, Ordering::SeqCst);
}

pub unsafe fn diff_d3d_1420593e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut nri = [0usize; 0x54];
    nri[0xd8 / 8] = fake_nri_destroy_desc as *const () as usize;
    nri[0x290 / 8] = fake_nri_reset_pool as *const () as usize;

    let pools: [usize; 3] = [0x1001, 0x1002, 0x1003];
    let mut d0_arr: [u64; 3] = [0, 0, 0];
    let items: [usize; 2] = [0x501, 0x502];
    let mut vec_headers = [0usize; 9]; // 3 * 24 bytes
    let mut this_buf = [0u8; 0x160];
    let this = this_buf.as_mut_ptr() as usize;

    for flag_14d in [0u8, 1u8] {
        let mut run_one = |f: F| {
            this_buf.fill(0);
            d0_arr = [0xaa, 0xbb, 0xcc];
            vec_headers.fill(0);
            let items_ptr = items.as_ptr() as usize;
            // rem = 5 % 3 = 2 -> index 2 in vec_headers (offset 2 * 3 = 6 u64s)
            vec_headers[6] = items_ptr;
            vec_headers[7] = items_ptr + 16;
            vec_headers[8] = items_ptr + 16;

            wr::<usize>(this + 0x28, vec_headers.as_mut_ptr() as usize);
            wr::<usize>(this + 0xb8, pools.as_ptr() as usize);
            wr::<usize>(this + 0xd0, d0_arr.as_mut_ptr() as usize);
            wr::<usize>(this + 0xe8, nri.as_ptr() as usize);
            wr::<u32>(this + 0x140, 3);
            wr::<u32>(this + 0x148, 5);
            wr::<u8>(this + 0x14d, flag_14d);
            STATE_593E0.store(0, Ordering::SeqCst);
            f(this);
            (this_buf, d0_arr, vec_headers, STATE_593E0.load(Ordering::SeqCst))
        };

        let o_state = run_one(f_orig);
        let n_state = run_one(f_new);
        crate::diff_eq!(o_state, n_state, "d3d_1420593e0");
    }
    Ok(2)
}

static DESC_5A300: AtomicUsize = AtomicUsize::new(0);
static VER_5A300: AtomicUsize = AtomicUsize::new(0);

unsafe extern "win64" fn fake_get_dev_desc(_a3: usize) -> usize {
    DESC_5A300.load(Ordering::SeqCst)
}

unsafe extern "win64" fn fake_nrd_get_lib_desc() -> usize {
    VER_5A300.load(Ordering::SeqCst)
}

unsafe extern "win64" fn fake_nrd_create_instance(a2: usize, out_inst: usize) -> u32 {
    wr::<usize>(out_inst, a2 ^ 0x1100);
    0
}

unsafe extern "win64" fn fake_59ea0(this: usize) {
    wr::<u32>(this + 0x120, 0x9ea0);
}

unsafe extern "win64" fn fake_597a0(this: usize) {
    wr::<u32>(this + 0x124, 0x97a0);
}

pub unsafe fn diff_d3d_14205a300(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let core_global = exe::ptr::<usize>(0x1_43c4_8ad0) as usize;
    let _s1 = SnapGuard::take(core_global, 8);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_5c70, fake_nrd_get_lib_desc as *const ())?;
    let _im2 =
        crate::difftest::IatMock::new(0x1_4264_5c88, fake_nrd_create_instance as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4205_9ea0, fake_59ea0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_97a0, fake_597a0 as *const ())?;

    wr::<usize>(core_global, fake_get_dev_desc as *const () as usize);

    let mut desc_buf = [0u8; 0xb0];
    let mut ver_buf = [0u8; 0x30];
    let mut this_buf = [0u8; 0x140];
    let desc_ptr = desc_buf.as_mut_ptr() as usize;
    let ver_ptr = ver_buf.as_mut_ptr() as usize;
    let this = this_buf.as_mut_ptr() as usize;
    DESC_5A300.store(desc_ptr, Ordering::SeqCst);
    VER_5A300.store(ver_ptr, Ordering::SeqCst);

    for valid in [false, true] {
        let mut run_one = |f: F| {
            desc_buf.fill(0);
            ver_buf.fill(0);
            this_buf.fill(0);
            if valid {
                wr::<u16>(desc_ptr + 0xa2, 1);
                wr::<u16>(desc_ptr + 0xa4, 0x63);
                wr::<u8>(ver_ptr + 0x1c, 4);
                wr::<u8>(ver_ptr + 0x1d, 3);
            }
            let r = f(this, 0x2222, 0x3333, 0, 0x5555);
            (r, this_buf)
        };

        let o_state = run_one(f_orig);
        let n_state = run_one(f_new);
        crate::diff_eq!(o_state, n_state, "d3d_14205a300");
    }
    Ok(2)
}

static STATE_5A4F0: AtomicU64 = AtomicU64::new(0);

unsafe extern "win64" fn fake_11d6b80(ptr: usize, start_idx: usize, end_idx: usize, vec_addr: usize) {
    let v = (ptr as u64)
        ^ ((start_idx as u64) << 16)
        ^ ((end_idx as u64) << 32)
        ^ (vec_addr as u64);
    STATE_5A4F0.store(v, Ordering::SeqCst);
}

pub unsafe fn diff_d3d_14205a4f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let lock_addr = exe::ptr::<u8>(0x1_43a3_2d38) as usize;
    let vec_global = exe::ptr::<usize>(0x1_43a6_3248) as usize;
    let frame_global = exe::ptr::<u64>(0x1_43a3_5918) as usize;
    let _s1 = SnapGuard::take(lock_addr, 8);
    let _s2 = SnapGuard::take(vec_global, 16);
    let _s3 = SnapGuard::take(frame_global, 8);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_srw_noop as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_srw_noop as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_411d_6b80, fake_11d6b80 as *const ())?;

    let mut entries = [0u8; 3 * 0x28];
    let base = entries.as_mut_ptr() as usize;

    let mut setup = || {
        entries.fill(0);
        // Entry 0: expired (2000 > 100 + 1000)
        wr::<u64>(base + 0x08, 100);
        wr::<u8>(base + 0x21, 1);
        // Entry 1: not expired (2000 <= 1500 + 1000) -> resets first_expired
        wr::<u64>(base + 0x28 + 0x08, 1500);
        wr::<u8>(base + 0x28 + 0x21, 1);
        // Entry 2: expired (2000 > 200 + 1000) -> first_expired = 2
        wr::<u64>(base + 0x50 + 0x08, 200);
        wr::<u8>(base + 0x50 + 0x21, 1);

        wr::<usize>(vec_global, base);
        wr::<u32>(vec_global + 8, 3);
        wr::<u64>(frame_global, 2000);
        STATE_5A4F0.store(0, Ordering::SeqCst);
    };

    setup();
    f_orig();
    let o_val = STATE_5A4F0.load(Ordering::SeqCst);

    setup();
    f_new();
    let n_val = STATE_5A4F0.load(Ordering::SeqCst);
    crate::diff_eq!(o_val, n_val, "d3d_14205a4f0");
    Ok(1)
}

unsafe extern "win64" fn fake_11d6a90(vec_addr: usize) {
    wr::<u32>(vec_addr + 8, 0x6a90);
}

pub unsafe fn diff_d3d_14205a5c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let lock_addr = exe::ptr::<u8>(0x1_43a3_2d38) as usize;
    let vec_global = exe::ptr::<usize>(0x1_43a6_3248) as usize;
    let _s1 = SnapGuard::take(lock_addr, 8);
    let _s2 = SnapGuard::take(vec_global, 16);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_srw_noop as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_srw_noop as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_411d_6a90, fake_11d6a90 as *const ())?;

    wr::<u32>(vec_global + 8, 0);
    f_orig();
    let o_val = rd::<u32>(vec_global + 8);

    wr::<u32>(vec_global + 8, 0);
    f_new();
    let n_val = rd::<u32>(vec_global + 8);
    crate::diff_eq!(o_val, n_val, "d3d_14205a5c0");
    Ok(1)
}

pub unsafe fn diff_d3d_14205a600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u8) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let lock_addr = exe::ptr::<u8>(0x1_43a3_2d38) as usize;
    let vec_global = exe::ptr::<usize>(0x1_43a6_3248) as usize;
    let _s1 = SnapGuard::take(lock_addr, 8);
    let _s2 = SnapGuard::take(vec_global, 16);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_srw_noop as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_srw_noop as *const ())?;

    let mut entries = [0u8; 2 * 0x28];
    let base = entries.as_mut_ptr() as usize;

    for (key, clear_ts) in [(0x1111u64, 0u8), (0x2222u64, 1u8), (0x9999u64, 1u8)] {
        let mut run_one = |f: F| {
            entries.fill(0);
            wr::<u64>(base, 0x1111);
            wr::<u64>(base + 0x08, 500);
            wr::<u64>(base + 0x28, 0x2222);
            wr::<u64>(base + 0x28 + 0x08, 600);
            wr::<usize>(vec_global, base);
            wr::<u32>(vec_global + 8, 2);
            let r = f(key, clear_ts);
            (r, entries)
        };

        let o_state = run_one(f_orig);
        let n_state = run_one(f_new);
        crate::diff_eq!(o_state, n_state, "d3d_14205a600");
    }
    Ok(3)
}

pub unsafe fn diff_d3d_14205ad40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let table = exe::ptr::<usize>(0x1_43a6_3260) as usize;
    let _s1 = SnapGuard::take(table, 32);

    let mut entries = [0u8; 2 * 0xa0];
    let base = entries.as_mut_ptr() as usize;
    wr::<u16>(base + 0xa0 + 0x9c, 0x1234);
    wr::<usize>(table, base);

    for handle in [
        0x1234_ffff_0000_0001u64,
        0x1234_0000_0000_0001u64,
        0x9999_0000_0000_0001u64,
    ] {
        let ro = f_orig(handle, 0);
        let rn = f_new(handle, 0);
        crate::diff_eq!(ro, rn, "d3d_14205ad40");
    }
    Ok(3)
}

pub unsafe fn diff_d3d_14205ad50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let table = exe::ptr::<usize>(0x1_43a6_3260) as usize;
    let _s1 = SnapGuard::take(table, 32);

    let mut entries = [0u8; 2 * 0xa0];
    let base = entries.as_mut_ptr() as usize;
    wr::<u16>(base + 0xa0 + 0x9c, 0x1234);
    wr::<usize>(table + 16, base);

    for handle in [0x1234_0001_0000_0001u64, 0x5678_0001_0000_0001u64] {
        let ro = f_orig(handle, 0, 1);
        let rn = f_new(handle, 0, 1);
        crate::diff_eq!(ro, rn, "d3d_14205ad50");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_14205ada0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let lock_addr = exe::ptr::<u8>(0x1_43a3_2f28) as usize;
    let table = exe::ptr::<usize>(0x1_43a6_3260) as usize;
    let _s1 = SnapGuard::take(lock_addr, 8);
    let _s2 = SnapGuard::take(table, 32);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_srw_noop as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_srw_noop as *const ())?;

    let mut entries = [0u8; 2 * 0xa0];
    let base = entries.as_mut_ptr() as usize;
    wr::<usize>(table, base);

    for handle in [
        0x1234_ffff_0000_0001u64,
        0x1234_0000_0000_0001u64,
        0x5678_0000_0000_0001u64,
    ] {
        let mut setup = || {
            entries.fill(0);
            wr::<u32>(base + 0xa0 + 0x98, 10);
            wr::<u16>(base + 0xa0 + 0x9c, 0x1234);
        };

        setup();
        f_orig(handle);
        let o_ref = rd::<u32>(base + 0xa0 + 0x98);

        setup();
        f_new(handle);
        let n_ref = rd::<u32>(base + 0xa0 + 0x98);
        crate::diff_eq!(o_ref, n_ref, "d3d_14205ada0");
    }
    Ok(3)
}

pub unsafe fn diff_d3d_14205afd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let lock_addr = exe::ptr::<u8>(0x1_43a3_2f28) as usize;
    let table = exe::ptr::<usize>(0x1_43a6_3260) as usize;
    let _s1 = SnapGuard::take(lock_addr, 8);
    let _s2 = SnapGuard::take(table, 32);
    let _im1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_srw_noop as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_srw_noop as *const ())?;

    let mut entries = [0u8; 2 * 0xa0];
    let base = entries.as_mut_ptr() as usize;
    wr::<usize>(table + 16, base);
    wr::<u32>(table + 24, 2);

    for target in [0usize, 0xcafe_babeusize, 0xdead_beefusize] {
        let mut setup = || {
            entries.fill(0);
            wr::<usize>(base + 0xa0 + 0x90, 0xcafe_babe);
            wr::<u32>(base + 0xa0 + 0x98, 7);
            wr::<u16>(base + 0xa0 + 0x9c, 0x4321);
        };

        setup();
        let ro = f_orig(1, target);
        let o_state = (ro, rd::<u32>(base + 0xa0 + 0x98));

        setup();
        let rn = f_new(1, target);
        let n_state = (rn, rd::<u32>(base + 0xa0 + 0x98));
        crate::diff_eq!(o_state, n_state, "d3d_14205afd0");
    }
    Ok(3)
}

// REGISTRY: Replacement::verified_throws(0x142055f20, "d3d::d3d_142055f20", crate::ns::rend::d3d_dlss_fg_tail::d3d_142055f20 as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142055f20),
// REGISTRY: Replacement::verified(0x142055f80, "d3d::d3d_142055f80", crate::ns::rend::d3d_dlss_fg_tail::d3d_142055f80 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142055f80),
// REGISTRY: Replacement::verified(0x1420562e0, "d3d::d3d_1420562e0", crate::ns::rend::d3d_dlss_fg_tail::d3d_1420562e0 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_1420562e0),
// REGISTRY: Replacement::verified_throws(0x142056660, "d3d::d3d_142056660", crate::ns::rend::d3d_dlss_fg_tail::d3d_142056660 as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142056660),
// REGISTRY: Replacement::verified_throws(0x142056760, "d3d::d3d_142056760", crate::ns::rend::d3d_dlss_fg_tail::d3d_142056760 as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142056760),
// REGISTRY: Replacement::verified(0x142056ce0, "d3d::d3d_142056ce0", crate::ns::rend::d3d_dlss_fg_tail::d3d_142056ce0 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142056ce0),
// REGISTRY: Replacement::verified(0x142057f90, "d3d::d3d_142057f90", crate::ns::rend::d3d_dlss_fg_tail::d3d_142057f90 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142057f90),
// REGISTRY: Replacement::verified(0x142057fe0, "d3d::d3d_142057fe0", crate::ns::rend::d3d_dlss_fg_tail::d3d_142057fe0 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142057fe0),
// REGISTRY: Replacement::verified_throws(0x142058140, "d3d::d3d_142058140", crate::ns::rend::d3d_dlss_fg_tail::d3d_142058140 as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142058140),
// REGISTRY: Replacement::verified_throws(0x142058180, "d3d::d3d_142058180", crate::ns::rend::d3d_dlss_fg_tail::d3d_142058180 as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142058180),
// REGISTRY: Replacement::verified(0x142058540, "d3d::d3d_142058540", crate::ns::rend::d3d_dlss_fg_tail::d3d_142058540 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142058540),
// REGISTRY: Replacement::verified_throws(0x142059280, "d3d::d3d_142059280", crate::ns::rend::d3d_dlss_fg_tail::d3d_142059280 as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_142059280),
// REGISTRY: Replacement::verified(0x1420593e0, "d3d::d3d_1420593e0", crate::ns::rend::d3d_dlss_fg_tail::d3d_1420593e0 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_1420593e0),
// REGISTRY: Replacement::verified_throws(0x14205a300, "d3d::d3d_14205a300", crate::ns::rend::d3d_dlss_fg_tail::d3d_14205a300 as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_14205a300),
// REGISTRY: Replacement::verified_throws(0x14205a4f0, "d3d::d3d_14205a4f0", crate::ns::rend::d3d_dlss_fg_tail::d3d_14205a4f0 as *const (), Throws::Caught, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_14205a4f0),
// REGISTRY: Replacement::verified_throws(0x14205a5c0, "d3d::d3d_14205a5c0", crate::ns::rend::d3d_dlss_fg_tail::d3d_14205a5c0 as *const (), Throws::Caught, crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_14205a5c0),
// REGISTRY: Replacement::verified(0x14205a600, "d3d::d3d_14205a600", crate::ns::rend::d3d_dlss_fg_tail::d3d_14205a600 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_14205a600),
// REGISTRY: Replacement::verified(0x14205ad40, "d3d::d3d_14205ad40", crate::ns::rend::d3d_dlss_fg_tail::d3d_14205ad40 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_14205ad40),
// REGISTRY: Replacement::verified(0x14205ad50, "d3d::d3d_14205ad50", crate::ns::rend::d3d_dlss_fg_tail::d3d_14205ad50 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_14205ad50),
// REGISTRY: Replacement::verified(0x14205ada0, "d3d::d3d_14205ada0", crate::ns::rend::d3d_dlss_fg_tail::d3d_14205ada0 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_14205ada0),
// REGISTRY: Replacement::verified(0x14205afd0, "d3d::d3d_14205afd0", crate::ns::rend::d3d_dlss_fg_tail::d3d_14205afd0 as *const (), crate::ns::rend::d3d_dlss_fg_tail::diff_d3d_14205afd0),



