//! Decompiled `d3d` renderer functions (`0x142076db0..0x14208de00`).

use crate::exe::{self, rd, wr};
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};

#[inline]
unsafe fn check_gpu_hr(hr: u32) {
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
    }
}

#[inline]
unsafe fn read_teb() -> usize {
    exe::teb_self()
}

/// Original: `0x142076db0` (112 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_flush_deferred_releases_locked_142076db0(this: usize) {
    let lock = this + 0x38;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    while rd::<u8>(this + 0x40) != 0 {
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
        exe::call_iat!(0x1_4264_5868, () -> ());
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    }
    exe::call_va!(0x1_4207_6960, (this) -> ());
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
}

/// Original: `0x142076f00` (78 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_get_queue_timestamp_frequency_142076f00(_rcx: usize) -> u64 {
    let mut freq: u64 = 0;
    let freq_ptr = (&mut freq as *mut u64) as usize;
    let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
    let queue = rd::<usize>(g + 0x10);
    let fn_ptr = rd::<usize>(rd::<usize>(queue) + 0x80);
    let hr = exe::call_va!(fn_ptr, (queue, freq_ptr) -> i32);
    check_gpu_hr(hr as u32);
    rd::<u64>(freq_ptr)
}

/// Original: `0x142076f60` (285 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_enqueue_deferred_releases_142076f60(
    this: usize,
    count: usize,
    items: usize,
) {
    let lock = this + 0x38;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    for i in 0..count {
        let item = rd::<usize>(items + i * 8);
        let len = rd::<u32>(this + 0x20) as u64;
        let cap = rd::<u32>(this + 0x24) as u64;
        if len.wrapping_mul(8) == cap.wrapping_mul(8) {
            let min_cap = len.wrapping_add(1);
            let grow = (cap >> 1).wrapping_add(cap);
            let new_cap = if grow < min_cap { min_cap } else { grow };
            exe::call_va!(0x1_401d_eba0, (this + 0x18, new_cap) -> ());
        }
        let cur_len = rd::<u32>(this + 0x20) as usize;
        let buf = rd::<usize>(this + 0x18);
        wr::<usize>(buf + cur_len * 8, item);
        wr::<u32>(this + 0x20, (cur_len as u32).wrapping_add(1));

        let sub = rd::<usize>(item + 0x18);
        if sub != 0 {
            let len2 = rd::<u32>(this + 0x30) as u64;
            let cap2 = rd::<u32>(this + 0x34) as u64;
            if len2.wrapping_mul(8) == cap2.wrapping_mul(8) {
                exe::call_va!(0x1_4020_bcb0, (this + 0x28, len2.wrapping_add(1)) -> ());
            }
            let cur_len2 = rd::<u32>(this + 0x30) as usize;
            let buf2 = rd::<usize>(this + 0x28);
            wr::<usize>(buf2 + cur_len2 * 8, rd::<usize>(item + 0x18));
            wr::<u32>(this + 0x30, (cur_len2 as u32).wrapping_add(1));
        }
    }
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
    let len2 = rd::<u32>(this + 0x30) as u64;
    let thresh = rd::<i32>(exe::ptr::<i32>(0x1_4385_7f50) as usize) as i64 as u64;
    if len2 >= thresh {
        exe::call_va!(0x1_4207_6db0, (this) -> ());
    }
}

/// Original: `0x1420770a0` (133 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_cmd_list_with_ext_interfaces_1420770a0(
    this: usize,
    cmd_type: u32,
) -> usize {
    exe::call_va!(0x1_411d_eb90, (this, cmd_type) -> usize);
    wr::<usize>(this + 0x30, 0);
    wr::<usize>(this + 0x38, 0);
    let p28 = rd::<usize>(this + 0x28);
    let qi0 = rd::<usize>(rd::<usize>(p28));
    wr::<usize>(this + 0x30, 0);
    let iid0 = exe::ptr::<u8>(0x1_433d_ccc8) as usize;
    exe::call_va!(qi0, (p28, iid0, this + 0x30) -> i32);
    let p28_2 = rd::<usize>(this + 0x28);
    let qi1 = rd::<usize>(rd::<usize>(p28_2));
    let old38 = rd::<usize>(this + 0x38);
    if old38 != 0 {
        let rel = rd::<usize>(rd::<usize>(old38) + 0x10);
        exe::call_va!(rel, (old38) -> ());
    }
    wr::<usize>(this + 0x38, 0);
    let iid1 = exe::ptr::<u8>(0x1_433d_cc98) as usize;
    exe::call_va!(qi1, (p28_2, iid1, this + 0x38) -> i32);
    this
}

/// Original: `0x142077130` (60 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_fence_is_completed_142077130(this: usize) -> u8 {
    let fence = rd::<usize>(this + 0x10);
    let get_completed = rd::<usize>(rd::<usize>(fence) + 0x40);
    let val = exe::call_va!(get_completed, (fence) -> u64);
    if val == u64::MAX {
        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32) -> ());
    }
    u8::from(val >= rd::<u64>(this + 0x8))
}

/// Original: `0x1420771d0` (221 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_cmd_alloc_and_fence_1420771d0(
    this: usize,
    cmd_type: u32,
) -> usize {
    wr::<u64>(this, 0);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u8>(this + 0x20, 0);
    let mut dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let create_alloc = rd::<usize>(rd::<usize>(dev) + 0x48);
    let old0 = rd::<usize>(this);
    if old0 != 0 {
        let rel = rd::<usize>(rd::<usize>(old0) + 0x10);
        exe::call_va!(rel, (old0) -> ());
        dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    }
    wr::<usize>(this, 0);
    let iid_alloc = exe::ptr::<u8>(0x1_433d_ccb8) as usize;
    let hr1 = exe::call_va!(create_alloc, (dev, cmd_type, iid_alloc, this) -> i32);
    check_gpu_hr(hr1 as u32);

    let mut fence_out: usize = 0;
    let fence_out_ptr = (&mut fence_out as *mut usize) as usize;
    let dev2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let create_fence = rd::<usize>(rd::<usize>(dev2) + 0x120);
    let iid_fence = exe::ptr::<u8>(0x1_433d_cab8) as usize;
    let hr2 = exe::call_va!(create_fence, (dev2, 0u64, 1u32, iid_fence, fence_out_ptr) -> i32);
    check_gpu_hr(hr2 as u32);
    wr::<u64>(this + 0x8, 0);
    wr::<usize>(this + 0x10, rd::<usize>(fence_out_ptr));
    this
}

/// Original: `0x1420772c0` (250 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_execute_command_lists_and_signal_1420772c0(
    queue: usize,
    span: usize,
) {
    let count = rd::<usize>(span);
    let bytes = count.wrapping_mul(8);
    let mut buf: usize = 0;
    if count != 0 {
        buf = exe::call_va!(0x1_4241_c0a0, (bytes, 8u32) -> usize);
        if count != 0 {
            exe::call_va!(0x1_4183_acc2, (buf, 0u32, bytes) -> usize);
        }
        let src_base = rd::<usize>(span + 0x8);
        for i in 0..count {
            let item = rd::<usize>(src_base + i * 8);
            wr::<u64>(buf + i * 8, rd::<u64>(item + 0x28));
        }
    }
    let exec_fn = rd::<usize>(rd::<usize>(queue) + 0x50);
    exe::call_va!(exec_fn, (queue, rd::<u32>(span), buf) -> ());
    let mut cur = rd::<usize>(span + 0x8);
    let end = cur.wrapping_add(bytes);
    while cur != end {
        let sig_fn = rd::<usize>(rd::<usize>(queue) + 0x70);
        let item = rd::<usize>(cur);
        exe::call_va!(sig_fn, (queue, rd::<usize>(item + 0x10), rd::<u64>(item + 0x8)) -> i32);
        cur = cur.wrapping_add(8);
    }
    if buf != 0 {
        exe::call_va!(0x1_4241_c190, (buf) -> ());
    }
}

/// Original: `0x1420773c0` (66 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_signal_global_fence_on_queue_1420773c0(
    _rcx: usize,
    queue_wrap: usize,
) {
    let queue = rd::<usize>(queue_wrap + 0x10);
    let fence_state = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc68) as usize);
    let sig_fn = rd::<usize>(rd::<usize>(queue) + 0x70);
    let hr = exe::call_va!(
        sig_fn,
        (queue, rd::<usize>(fence_state + 0x10), rd::<u64>(fence_state + 0x8)) -> i32
    );
    check_gpu_hr(hr as u32);
    wr::<u64>(
        fence_state + 0x8,
        rd::<u64>(fence_state + 0x8).wrapping_add(1),
    );
}

/// Original: `0x142077690` (399 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_build_blend_state_desc_142077690(
    out: usize,
    src: usize,
    alpha_to_coverage: u8,
) -> usize {
    for off in (0..0x148).step_by(8) {
        wr::<u64>(out + off, 0);
    }
    let tbl_factor = exe::ptr::<u32>(0x1_4297_f180) as usize;
    let tbl_op = exe::ptr::<u32>(0x1_4297_f1b0) as usize;
    wr::<u32>(out, alpha_to_coverage as u32);
    wr::<u32>(out + 0x08, rd::<u8>(src) as u32);
    wr::<u32>(
        out + 0x10,
        rd::<u32>(tbl_factor.wrapping_add((rd::<i32>(src + 0x04) as isize * 4) as usize)),
    );
    wr::<u32>(
        out + 0x14,
        rd::<u32>(tbl_factor.wrapping_add((rd::<i32>(src + 0x08) as isize * 4) as usize)),
    );
    wr::<u32>(
        out + 0x18,
        rd::<u32>(tbl_op.wrapping_add((rd::<i32>(src + 0x0c) as isize * 4) as usize)),
    );
    wr::<u32>(
        out + 0x1c,
        rd::<u32>(tbl_factor.wrapping_add((rd::<i32>(src + 0x18) as isize * 4) as usize)),
    );
    wr::<u32>(
        out + 0x20,
        rd::<u32>(tbl_factor.wrapping_add((rd::<i32>(src + 0x1c) as isize * 4) as usize)),
    );
    wr::<u32>(
        out + 0x24,
        rd::<u32>(tbl_op.wrapping_add((rd::<i32>(src + 0x20) as isize * 4) as usize)),
    );
    wr::<u8>(out + 0x2c, rd::<u8>(src + 0x10));
    let hash = exe::call_va!(0x1_4209_e760, (src, 0u64) -> u64);
    wr::<u64>(out + 0x148, hash);
    out
}

/// Original: `0x142079d80` (308 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_create_root_signature_from_spans_142079d80(
    out: usize,
    params_span: usize,
    samplers_span: usize,
) -> usize {
    // The original zeros the final qword, including D3D12_ROOT_SIGNATURE_DESC::Flags at +0x20.
    let mut desc = [0u64; 5];
    let desc_ptr = desc.as_mut_ptr() as usize;
    wr::<u32>(desc_ptr, rd::<u64>(params_span) as u32);
    wr::<u32>(desc_ptr + 0x04, 0);
    wr::<u64>(desc_ptr + 0x08, rd::<u64>(params_span + 0x8));
    wr::<u32>(desc_ptr + 0x10, rd::<u64>(samplers_span) as u32);
    wr::<u32>(desc_ptr + 0x14, 0);
    wr::<u64>(desc_ptr + 0x18, rd::<u64>(samplers_span + 0x8));

    let mut blob: usize = 0;
    let blob_ptr = (&mut blob as *mut usize) as usize;
    let hr1 = exe::call_iat!(0x1_4264_7338, (desc_ptr, 1u32, blob_ptr, 0usize) -> i32);
    check_gpu_hr(hr1 as u32);

    wr::<u32>(out, 0);
    wr::<usize>(out + 0x8, 0);
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let create_rs = rd::<usize>(rd::<usize>(dev) + 0x80);
    wr::<usize>(out + 0x8, 0);
    wr::<u32>(out, 0);

    let b = rd::<usize>(blob_ptr);
    let get_size = rd::<usize>(rd::<usize>(b) + 0x20);
    let sz = exe::call_va!(get_size, (b) -> usize);
    let b2 = rd::<usize>(blob_ptr);
    let get_ptr = rd::<usize>(rd::<usize>(b2) + 0x18);
    let ptr = exe::call_va!(get_ptr, (b2) -> usize);

    let dev2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let iid = exe::ptr::<u8>(0x1_433d_ccd8) as usize;
    let hr2 = exe::call_va!(create_rs, (dev2, 0u32, ptr, sz, iid, out + 0x8) -> i32);
    check_gpu_hr(hr2 as u32);

    let b_final = rd::<usize>(blob_ptr);
    if b_final != 0 {
        exe::call_va!(0x1_4209_a1e0, (b_final, 0u32) -> ());
    }
    out
}

/// Original: `0x14207a540` (308 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_advance_swapchain_frame_14207a540(this: usize) {
    let inner0 = rd::<usize>(this);
    let sc0 = rd::<usize>(inner0 + 0x8);
    let get_idx = rd::<usize>(rd::<usize>(sc0) + 0x120);
    let idx = exe::call_va!(get_idx, (sc0) -> u32);
    wr::<u32>(this + 0x38, idx);

    let inner1 = rd::<usize>(this);
    let fence_val = rd::<u64>(inner1 + 0x28);
    let fence = rd::<usize>(inner1 + 0x20);
    let q_wrap = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
    let queue = rd::<usize>(q_wrap + 0x10);
    let sig_fn = rd::<usize>(rd::<usize>(queue) + 0x70);
    let hr = exe::call_va!(sig_fn, (queue, fence, fence_val) -> i32);
    check_gpu_hr(hr as u32);

    if rd::<u8>(exe::ptr::<u8>(0x1_4385_7fd0) as usize) != 0 {
        loop {
            let inner = rd::<usize>(this);
            let f = rd::<usize>(inner + 0x20);
            let get_comp = rd::<usize>(rd::<usize>(f) + 0x40);
            let comp = exe::call_va!(get_comp, (f) -> u64);
            if comp == u64::MAX {
                exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32) -> ());
            }
            let inner2 = rd::<usize>(this);
            if comp.wrapping_add(1) >= rd::<u64>(inner2 + 0x28) {
                break;
            }
            exe::call_iat!(0x1_4264_5868, () -> ());
        }
    }

    let inner2 = rd::<usize>(this);
    wr::<u64>(inner2 + 0x28, rd::<u64>(inner2 + 0x28).wrapping_add(1));

    let inner3 = rd::<usize>(this);
    if rd::<u8>(inner3 + 0x41) != 0 {
        let sc = rd::<usize>(inner3 + 0x8);
        let mut fullscreen: u32 = 0;
        let fs_ptr = (&mut fullscreen as *mut u32) as usize;
        let get_fs = rd::<usize>(rd::<usize>(sc) + 0x58);
        let hr2 = exe::call_va!(get_fs, (sc, fs_ptr, 0usize) -> i32);
        check_gpu_hr(hr2 as u32);
        if rd::<u32>(fs_ptr) != 1 {
            let hwnd = rd::<usize>(rd::<usize>(this));
            let iconic = exe::call_iat!(0x1_4264_61c0, (hwnd) -> i32);
            if iconic == 0 {
                let inner4 = rd::<usize>(this);
                let sc2 = rd::<usize>(inner4 + 0x8);
                let arg2 = rd::<u8>(this + 0x66);
                let arg4 = rd::<u32>(this + 0x40);
                let arg5 = rd::<u32>(this + 0x44);
                exe::call_va!(0x1_4207_be30, (sc2, arg2, this + 0x8, arg4, arg5, 1u8) -> u8);
                let inner5 = rd::<usize>(this);
                let flag = exe::call_va!(0x1_4207_bd60, (rd::<usize>(inner5 + 0x8)) -> u8);
                let inner6 = rd::<usize>(this);
                wr::<u8>(inner6 + 0x42, flag);
            }
        }
    }
}

/// Original: `0x14207be30` (285 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_resize_swapchain_buffers_14207be30(
    swapchain: usize,
    arg2: u8,
    arg3: usize,
    width: u32,
    height: u32,
    arg6: u8,
) -> u8 {
    let mut w: u32 = width;
    let mut h: u32 = height;
    let w_ptr = (&mut w as *mut u32) as usize;
    let h_ptr = (&mut h as *mut u32) as usize;
    exe::call_va!(0x1_4207_c130, (arg3) -> ());
    exe::call_va!(0x1_4207_bf60, () -> ());
    let ok1 = exe::call_va!(0x1_4207_c4b0, (swapchain, w_ptr, h_ptr, arg6) -> u8);
    let (use_w, use_h) = if ok1 != 0 {
        (rd::<u32>(w_ptr), rd::<u32>(h_ptr))
    } else {
        let mut desc = [0u8; 0x50];
        let desc_ptr = desc.as_mut_ptr() as usize;
        let get_desc = rd::<usize>(rd::<usize>(swapchain) + 0x60);
        let hr = exe::call_va!(get_desc, (swapchain, desc_ptr) -> i32);
        let mut rect = [0i32; 4];
        let rect_ptr = rect.as_mut_ptr() as usize;
        let hwnd = rd::<usize>(desc_ptr + 0x30);
        if hr >= 0
            && exe::call_iat!(0x1_4264_6120, (hwnd, rect_ptr) -> i32) == 1
            && rd::<i32>(rect_ptr + 0x8).wrapping_sub(rd::<i32>(rect_ptr)) > 0
            && rd::<i32>(rect_ptr + 0xc).wrapping_sub(rd::<i32>(rect_ptr + 0x4)) > 0
        {
            (0u32, 0u32)
        } else {
            (width, height)
        }
    };
    let ok2 = exe::call_va!(0x1_4207_c410, (swapchain, use_w, use_h, 0u8) -> u8);
    exe::call_va!(0x1_4207_c040, (swapchain, arg2, arg3) -> ());
    u8::from(ok1 != 0 && ok2 != 0)
}

/// Original: `0x14207e500` (379 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_release_mip_level_slot_14207e500(this: usize, idx: u8) {
    let slot = this + (idx as usize) * 8;
    if rd::<usize>(slot + 0x10) == 0 {
        return;
    }
    let flag_ptr = (this + 0x50 + (idx as usize)) as *const AtomicU8;
    (*flag_ptr).swap(0, Ordering::SeqCst);

    let limit = rd::<u8>(this + 0x6c);
    if idx < limit {
        let mut first_active: u8 = 0;
        if limit != 0 {
            while first_active < rd::<u8>(this + 0x6c) {
                if rd::<u8>(this + 0x50 + (first_active as usize)) != 0 {
                    break;
                }
                first_active = first_active.wrapping_add(1);
            }
        }
        let base = rd::<usize>(this + 0x8);
        let count = rd::<u32>(this + 0x68) as usize;
        let mut use_count = count;
        if first_active < rd::<u8>(this + 0x6c) && count != 0 {
            for i in 0..count {
                if rd::<u8>(base + i * 0x20 + 0x1c) >= first_active {
                    if i != usize::MAX {
                        use_count = i;
                    }
                    break;
                }
            }
        }
        let mut span = [use_count as u64, base as u64];
        let g = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize);
        exe::call_va!(0x1_4208_c000, (g, span.as_mut_ptr() as usize) -> ());

        let mut total: u64 = 0;
        let mut cur = rd::<usize>(this + 0x8);
        let end = cur.wrapping_add((rd::<u32>(this + 0x68) as usize) * 0x20);
        while cur != end {
            if rd::<u8>(cur + 0x1f) == 2 {
                let prod = rd::<u16>(cur + 0x18)
                    .wrapping_mul(rd::<u16>(cur + 0x1a))
                    .wrapping_mul(rd::<u16>(cur + 0x16));
                total = total.wrapping_add(prod as u64);
            }
            cur = cur.wrapping_add(0x20);
        }
        wr::<u64>(this + 0x60, total << 16);
    } else {
        let sz = exe::call_va!(0x1_4206_9580, (rd::<usize>(slot + 0x10)) -> u64);
        wr::<u64>(this + 0x58, rd::<u64>(this + 0x58).wrapping_sub(sz));
        let g = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize);
        let stat_ptr = (rd::<usize>(g + 0x18) + 0x210) as *const AtomicU64;
        (*stat_ptr).fetch_sub(sz, Ordering::SeqCst);
    }

    let obj = rd::<usize>(slot + 0x10);
    wr::<usize>(slot + 0x10, 0);
    if obj != 0 {
        exe::call_va!(0x1_4206_b020, (obj) -> ());
        exe::call_va!(0x1_4244_2de0, (obj) -> ());
    }
}

/// Original: `0x14207fec0` (277 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_reinit_streaming_texture_14207fec0(this: usize, src: usize) {
    let mut local = [0u64; 20];
    let local_ptr = local.as_mut_ptr() as usize;
    let p = exe::call_va!(0x1_4207_f660, (local_ptr) -> usize);
    exe::call_va!(0x1_4207_e9d0, (this, p) -> ());
    let mut i: u8 = 0;
    while i < rd::<u8>(local_ptr + 0x83) {
        exe::call_va!(0x1_4207_e500, (local_ptr, i) -> ());
        i = i.wrapping_add(1);
    }
    exe::call_va!(
        0x1_4183_4ee8,
        (
            local_ptr + 0x10,
            8usize,
            8usize,
            exe::ptr::<u8>(0x1_411d_2db0) as usize
        ) -> ()
    );
    let p08 = rd::<usize>(local_ptr + 0x8);
    if p08 != 0 {
        exe::call_va!(0x1_4244_2de0, (p08) -> ());
    }
    let p00 = rd::<usize>(local_ptr);
    if p00 != 0 {
        exe::call_va!(0x1_4206_b020, (p00) -> ());
        exe::call_va!(0x1_4244_2de0, (p00) -> ());
    }
    let flag_ptr = (this + 0x6d) as *const AtomicU8;
    (*flag_ptr).swap(1, Ordering::SeqCst);
    wr::<u32>(this + 0x70, rd::<u32>(src));
    wr::<u32>(this + 0x74, rd::<u32>(src + 0x04));
    wr::<u32>(this + 0x78, rd::<u32>(src + 0x08));
    wr::<u32>(this + 0x7c, rd::<u32>(src + 0x0c));
    wr::<u8>(this + 0x80, rd::<u8>(src + 0x10));
    wr::<u8>(this + 0x81, rd::<u8>(src + 0x11));
    wr::<u8>(this + 0x82, rd::<u8>(src + 0x12));
    wr::<u8>(this + 0x83, rd::<u8>(src + 0x13));
    wr::<u64>(this + 0x90, rd::<u64>(src + 0x20));
    wr::<u64>(this + 0x88, rd::<u64>(src + 0x18));
    exe::call_va!(0x1_4207_ec20, (this) -> ());
}

/// Original: `0x1420804b0` (376 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_small_texture_mip_chain_1420804b0(desc: usize) -> u8 {
    if rd::<u8>(desc + 0x10) != 0 {
        return 0;
    }
    let fmt = rd::<i8>(desc + 0x11) as i32;
    let block_bytes = exe::call_va!(0x1_4204_e7f0, (4u32, 4u32, fmt as u8) -> u32);
    let shift: u32 = if block_bytes > 1 {
        let v = block_bytes - 1;
        if v == 0 {
            0
        } else {
            (31 - v.leading_zeros()) + 1
        }
    } else {
        0
    };
    let mut h = rd::<u32>(desc + 0x4);
    let rem = 0xcu32.wrapping_sub(shift);
    let half = rem >> 1;
    let shift_w = rem.wrapping_sub(half);
    let tile_w = 4u32.wrapping_shl(shift_w);
    let tile_h = 4u32.wrapping_shl(half);
    let mips = rd::<u8>(desc + 0x12);
    let mut w = rd::<u32>(desc);
    let mut total: u64 = 0;
    if mips != 0 {
        let is_bc = (fmt as u32).wrapping_sub(0x1c) <= 9;
        for _ in 0..mips {
            if is_bc {
                w = w.wrapping_add(3) & !3u32;
                h = h.wrapping_add(3) & !3u32;
            }
            let aligned_w = w.wrapping_sub(1).wrapping_add(tile_w) & tile_w.wrapping_neg();
            let aligned_h = h.wrapping_sub(1).wrapping_add(tile_h) & tile_h.wrapping_neg();
            let mip_bytes =
                exe::call_va!(0x1_4204_e7f0, (aligned_w, aligned_h, fmt as u8) -> i32);
            total = total.wrapping_add(mip_bytes as i64 as u64);
            w >>= 1;
            if w == 0 {
                w = 1;
            }
            h >>= 1;
            if h == 0 {
                h = 1;
            }
        }
    }
    u8::from(total <= 0x10000)
}

/// Original: `0x142082a40` (384 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_stream_cmd_move_construct_142082a40(dst: usize, src: usize) {
    match rd::<u8>(src + 0x50) {
        0 => {
            let v0 = rd::<u64>(src);
            wr::<u64>(src, 0);
            wr::<u64>(dst, v0);
            wr::<u16>(dst + 0x08, rd::<u16>(src + 0x08));
            wr::<[u8; 16]>(dst + 0x0c, rd::<[u8; 16]>(src + 0x0c));
            wr::<[u8; 16]>(dst + 0x1c, rd::<[u8; 16]>(src + 0x1c));
            wr::<u32>(dst + 0x2c, rd::<u32>(src + 0x2c));
            wr::<u32>(dst + 0x30, rd::<u32>(src + 0x30));
            wr::<u8>(dst + 0x50, 0);
        }
        1 => {
            let v0 = rd::<u64>(src);
            wr::<u64>(src, 0);
            wr::<u64>(dst, v0);
            wr::<[u8; 16]>(dst + 0x08, rd::<[u8; 16]>(src + 0x08));
            wr::<[u8; 16]>(dst + 0x18, rd::<[u8; 16]>(src + 0x18));
            wr::<u8>(dst + 0x50, 1);
        }
        2 => {
            for off in [0x00usize, 0x08, 0x10, 0x18, 0x20] {
                let v = rd::<u64>(src + off);
                wr::<u64>(src + off, 0);
                wr::<u64>(dst + off, v);
            }
            let v28 = rd::<u16>(src + 0x28);
            wr::<u16>(src + 0x28, 0x10);
            wr::<u16>(dst + 0x28, v28);
            let v30 = rd::<u64>(src + 0x30);
            wr::<u64>(src + 0x30, 0);
            wr::<u64>(dst + 0x30, v30);
            wr::<u8>(dst + 0x38, rd::<u8>(src + 0x38));
            wr::<u32>(dst + 0x3c, rd::<u32>(src + 0x3c));
            wr::<u32>(dst + 0x40, rd::<u32>(src + 0x40));
            wr::<u8>(dst + 0x44, rd::<u8>(src + 0x44));
            wr::<u8>(dst + 0x48, rd::<u8>(src + 0x48));
            wr::<u8>(dst + 0x50, 2);
        }
        3 => {
            wr::<u16>(dst, rd::<u16>(src));
            wr::<u8>(dst + 0x50, 3);
        }
        4 => {
            wr::<u16>(dst, rd::<u16>(src));
            wr::<u8>(dst + 0x50, 4);
        }
        5 => {
            wr::<[u8; 16]>(dst, rd::<[u8; 16]>(src));
            wr::<u8>(dst + 0x50, 5);
        }
        6 => {
            wr::<u8>(dst + 0x50, 6);
        }
        7 => {
            wr::<u64>(dst + 0x38, 0);
            exe::call_va!(0x1_401f_a4d0, (dst, src) -> ());
            wr::<u8>(dst + 0x50, 7);
        }
        _ => {}
    }
}

/// Original: `0x142084000` (388 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_stream_cmd_move_with_seq_142084000(
    dst: usize,
    src: usize,
) -> usize {
    wr::<u8>(dst + 0x50, 0xff);
    d3d_stream_cmd_move_construct_142082a40(dst, src);
    wr::<u64>(dst + 0x58, rd::<u64>(src + 0x58));
    dst
}

/// Original: `0x142084af0` (280 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_destroy_com_ring_buffer_142084af0(this: usize) {
    let mut i: u64 = 0;
    while i < rd::<u32>(this + 0x10) as u64 {
        let base = rd::<usize>(this);
        let head = rd::<u32>(this + 0x08) as u64;
        let cap = rd::<u32>(this + 0x14) as u64;
        let slot_idx = head.wrapping_add(i) % cap;
        let entry = base.wrapping_add((slot_idx as usize) * 0x18);
        let p1 = rd::<usize>(entry + 0x8);
        if p1 != 0 {
            let rel = rd::<usize>(rd::<usize>(p1) + 0x10);
            exe::call_va!(rel, (p1) -> ());
        }
        wr::<usize>(entry + 0x8, 0);
        let p0 = rd::<usize>(entry);
        if p0 != 0 {
            let rel = rd::<usize>(rd::<usize>(p0) + 0x10);
            exe::call_va!(rel, (p0) -> ());
        }
        wr::<usize>(entry, 0);
        i = i.wrapping_add(1);
    }
    wr::<u64>(this + 0x08, 0);
    wr::<u32>(this + 0x10, 0);
    let buf = rd::<usize>(this);
    if buf != 0 {
        let slab_base = (buf.wrapping_sub(1)) & !0x1ff_ffffusize;
        let chunk_idx = (buf.wrapping_sub(slab_base)) >> 16;
        let mut span_hdr = slab_base
            .wrapping_add(0x108)
            .wrapping_add(chunk_idx * 0x50);
        span_hdr = span_hdr.wrapping_sub(rd::<u32>(span_hdr + 0x4) as usize);
        if rd::<usize>(slab_base + 0x100) == read_teb() {
            if rd::<u8>(span_hdr + 0xe) == 0 {
                wr::<usize>(buf, rd::<usize>(span_hdr + 0x20));
                wr::<usize>(span_hdr + 0x20, buf);
                let new_cnt = rd::<i32>(span_hdr + 0x18).wrapping_sub(1);
                wr::<i32>(span_hdr + 0x18, new_cnt);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (span_hdr) -> ());
                }
            } else {
                exe::call_va!(0x1_4253_3200, (slab_base, span_hdr, 1u8, buf) -> ());
            }
        } else {
            exe::call_va!(0x1_4253_3200, (slab_base, span_hdr, 0u8, buf) -> ());
        }
    }
    wr::<usize>(this, 0);
    wr::<u32>(this + 0x14, 0);
}

/// Original: `0x142085c40` (365 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_free_tile_pool_id_142085c40(this: usize, id: u16) {
    let inner = rd::<usize>(this + 0x18);
    if rd::<u8>(inner + 0x1d0) != 0 {
        let vec = inner + 0x1c0;
        let len = rd::<u32>(vec + 0x8) as u64;
        let cap = rd::<u32>(vec + 0xc) as u64;
        if len.wrapping_mul(2) == cap.wrapping_mul(2) {
            exe::call_va!(0x1_40fa_5d10, (vec, len.wrapping_add(1)) -> ());
        }
        let idx = rd::<u32>(vec + 0x8) as usize;
        let data = rd::<usize>(vec);
        wr::<u16>(data + idx * 2, id);
        wr::<u32>(vec + 0x8, (idx as u32).wrapping_add(1));
    } else {
        let sub = inner + 0x30;
        exe::call_iat!(0x1_4264_55a8, (sub + 0x8) -> ());
        let mut cmd = [0u8; 0x60];
        let cmd_ptr = cmd.as_mut_ptr() as usize;
        wr::<u16>(cmd_ptr, id);
        wr::<u8>(cmd_ptr + 0x50, 4);
        let seq = rd::<u64>(sub + 0xc8);
        wr::<u64>(sub + 0xc8, seq.wrapping_add(1));
        wr::<u64>(cmd_ptr + 0x58, seq);

        let count = rd::<u32>(sub + 0x20);
        if count == rd::<u32>(sub + 0x24) {
            exe::call_va!(0x1_4208_3150, (sub + 0x10, count.wrapping_add(1)) -> ());
        }
        let tail = rd::<u32>(sub + 0x1c) as usize;
        let dst = rd::<usize>(sub + 0x10).wrapping_add(tail * 0x60);
        exe::call_va!(0x1_4208_4000, (dst, cmd_ptr) -> usize);
        let next_tail = rd::<u32>(sub + 0x1c).wrapping_add(1);
        wr::<u32>(
            sub + 0x1c,
            if next_tail == rd::<u32>(sub + 0x24) {
                0
            } else {
                next_tail
            },
        );
        wr::<u32>(sub + 0x20, rd::<u32>(sub + 0x20).wrapping_add(1));
        exe::call_va!(0x1_4208_50b0, (cmd_ptr) -> ());
        exe::call_iat!(0x1_4264_5598, (sub + 0x8) -> ());

        let g = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
        exe::call_iat!(0x1_4264_55a8, (g + 0x250) -> ());
        wr::<u32>(g + 0x258, rd::<u32>(g + 0x258).wrapping_add(1));
        exe::call_iat!(0x1_4264_5598, (g + 0x250) -> ());
        exe::call_iat!(0x1_4264_5590, (g + 0x248) -> ());

        let inner2 = rd::<usize>(this + 0x18);
        let vec = inner2 + 0x188;
        let len = rd::<u32>(vec + 0x8) as u64;
        let cap = rd::<u32>(vec + 0xc) as u64;
        if len.wrapping_mul(2) == cap.wrapping_mul(2) {
            exe::call_va!(0x1_40fa_5d10, (vec, len.wrapping_add(1)) -> ());
        }
        let idx = rd::<u32>(vec + 0x8) as usize;
        let data = rd::<usize>(vec);
        wr::<u16>(data + idx * 2, id);
        wr::<u32>(vec + 0x8, (idx as u32).wrapping_add(1));
    }
}

/// Original: `0x142085dc0` (367 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_alloc_tile_pool_id_142085dc0(this: usize) -> u16 {
    let inner = rd::<usize>(this + 0x18);
    let free_len = rd::<u32>(inner + 0x190) as u64;
    let total_ids = rd::<u16>(inner + 0x180) as u64;
    let used = total_ids.wrapping_sub(free_len);
    let base_mem = rd::<u64>(inner + 0x210);
    let needed = used.wrapping_add(1).wrapping_shl(0x19).wrapping_add(base_mem);
    if needed > rd::<u64>(this) {
        return 0xffff;
    }

    let inner2 = rd::<usize>(this + 0x18);
    let flen = rd::<u32>(inner2 + 0x190) as usize;
    let fptr = rd::<usize>(inner2 + 0x188);
    let fend = fptr.wrapping_add(flen * 2);
    let id: u16 = if fptr == fend {
        let cur = rd::<u16>(inner2 + 0x180);
        wr::<u16>(inner2 + 0x180, cur.wrapping_add(1));
        cur
    } else {
        let popped = rd::<u16>(fend - 2);
        wr::<u32>(inner2 + 0x190, (flen as u32).wrapping_sub(1));
        popped
    };

    let sub = rd::<usize>(this + 0x18) + 0x30;
    exe::call_iat!(0x1_4264_55a8, (sub + 0x8) -> ());
    let mut cmd = [0u8; 0x60];
    let cmd_ptr = cmd.as_mut_ptr() as usize;
    wr::<u16>(cmd_ptr, id);
    wr::<u8>(cmd_ptr + 0x50, 3);
    let seq = rd::<u64>(sub + 0xc8);
    wr::<u64>(sub + 0xc8, seq.wrapping_add(1));
    wr::<u64>(cmd_ptr + 0x58, seq);

    let count = rd::<u32>(sub + 0x20);
    if count == rd::<u32>(sub + 0x24) {
        exe::call_va!(0x1_4208_3150, (sub + 0x10, count.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(sub + 0x1c) as usize;
    let dst = rd::<usize>(sub + 0x10).wrapping_add(tail * 0x60);
    exe::call_va!(0x1_4208_4000, (dst, cmd_ptr) -> usize);
    let next_tail = rd::<u32>(sub + 0x1c).wrapping_add(1);
    wr::<u32>(
        sub + 0x1c,
        if next_tail == rd::<u32>(sub + 0x24) {
            0
        } else {
            next_tail
        },
    );
    wr::<u32>(sub + 0x20, rd::<u32>(sub + 0x20).wrapping_add(1));
    exe::call_va!(0x1_4208_50b0, (cmd_ptr) -> ());
    exe::call_iat!(0x1_4264_5598, (sub + 0x8) -> ());

    let g = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    exe::call_iat!(0x1_4264_55a8, (g + 0x250) -> ());
    wr::<u32>(g + 0x258, rd::<u32>(g + 0x258).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (g + 0x250) -> ());
    exe::call_iat!(0x1_4264_5590, (g + 0x248) -> ());
    id
}

/// Original: `0x142087760` (313 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_enqueue_stream_buffer_cmd_142087760(
    _rcx: usize,
    ctx_ptr: usize,
    com_obj: usize,
    req: usize,
) {
    let mut local = [0u8; 0x50];
    let local_ptr = local.as_mut_ptr() as usize;
    wr::<u16>(local_ptr + 0x28, 0x10);
    if com_obj != 0 {
        let add_ref = rd::<usize>(rd::<usize>(com_obj) + 0x8);
        exe::call_va!(add_ref, (com_obj) -> u32);
    }
    wr::<usize>(local_ptr, com_obj);
    wr::<u8>(local_ptr + 0x48, 0);

    for (src_off, dst_off) in [
        (0x00usize, 0x08usize),
        (0x08, 0x10),
        (0x10, 0x18),
        (0x18, 0x20),
    ] {
        let v = rd::<u64>(req + src_off);
        wr::<u64>(req + src_off, 0);
        wr::<u64>(local_ptr + dst_off, v);
    }
    let v20 = rd::<u16>(req + 0x20);
    wr::<u16>(req + 0x20, 0x10);
    wr::<u16>(local_ptr + 0x28, v20);

    let v28 = rd::<u64>(req + 0x28);
    wr::<u64>(req + 0x28, 0);
    wr::<u64>(local_ptr + 0x30, v28);

    wr::<u8>(local_ptr + 0x38, rd::<u8>(req + 0x30));
    wr::<u32>(local_ptr + 0x3c, rd::<u32>(req + 0x34));
    wr::<u32>(local_ptr + 0x40, rd::<u32>(req + 0x38));
    wr::<u8>(local_ptr + 0x44, rd::<u8>(req + 0x3c));

    exe::call_va!(0x1_411d_ff20, (ctx_ptr, local_ptr) -> ());

    let rem_buf = rd::<u64>(local_ptr + 0x18);
    if rem_buf != 0 {
        let alloc = rd::<usize>(local_ptr + 0x30);
        let free_fn = rd::<usize>(rd::<usize>(alloc) + 0x10);
        exe::call_va!(
            free_fn,
            (alloc, rem_buf, rd::<u64>(local_ptr + 0x20), rd::<u16>(local_ptr + 0x28)) -> ()
        );
    }
    let rem_com = rd::<usize>(local_ptr);
    if rem_com != 0 {
        let rel_fn = rd::<usize>(rd::<usize>(rem_com) + 0x10);
        exe::call_va!(rel_fn, (rem_com) -> ());
    }
}

/// Original: `0x1420886c0` (289 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_enqueue_stream_callback_cmd_1420886c0(
    ctx_ptr: usize,
    func_obj: usize,
) {
    let sub = rd::<usize>(ctx_ptr);
    let mut tmp_fn = [0u8; 0x40];
    let tmp_fn_ptr = tmp_fn.as_mut_ptr() as usize;
    let callable = rd::<usize>(func_obj + 0x38);
    if callable != 0 {
        if callable == func_obj {
            let clone_fn = rd::<usize>(rd::<usize>(callable) + 0x8);
            let cloned = exe::call_va!(clone_fn, (callable, tmp_fn_ptr) -> usize);
            wr::<usize>(tmp_fn_ptr + 0x38, cloned);
            let callable2 = rd::<usize>(func_obj + 0x38);
            if callable2 != 0 {
                let destroy_fn = rd::<usize>(rd::<usize>(callable2) + 0x20);
                exe::call_va!(destroy_fn, (callable2, u8::from(callable2 != func_obj)) -> ());
                wr::<usize>(func_obj + 0x38, 0);
            }
        } else {
            wr::<usize>(tmp_fn_ptr + 0x38, callable);
            wr::<usize>(func_obj + 0x38, 0);
        }
    }

    let mut cmd = [0u8; 0x60];
    let cmd_ptr = cmd.as_mut_ptr() as usize;
    wr::<usize>(cmd_ptr + 0x38, 0);
    exe::call_va!(0x1_401f_a4d0, (cmd_ptr, tmp_fn_ptr) -> ());
    wr::<u8>(cmd_ptr + 0x50, 7);

    let sub2 = rd::<usize>(ctx_ptr);
    let seq = rd::<u64>(sub2 + 0xc8);
    wr::<u64>(sub2 + 0xc8, seq.wrapping_add(1));
    wr::<u64>(cmd_ptr + 0x58, seq);

    let count = rd::<u32>(sub + 0x20);
    if count == rd::<u32>(sub + 0x24) {
        exe::call_va!(0x1_4208_3150, (sub + 0x10, count.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(sub + 0x1c) as usize;
    let dst = rd::<usize>(sub + 0x10).wrapping_add(tail * 0x60);
    exe::call_va!(0x1_4208_4000, (dst, cmd_ptr) -> usize);
    let next_tail = rd::<u32>(sub + 0x1c).wrapping_add(1);
    wr::<u32>(
        sub + 0x1c,
        if next_tail == rd::<u32>(sub + 0x24) {
            0
        } else {
            next_tail
        },
    );
    wr::<u32>(sub + 0x20, rd::<u32>(sub + 0x20).wrapping_add(1));
    exe::call_va!(0x1_4208_50b0, (cmd_ptr) -> ());

    let rem = rd::<usize>(tmp_fn_ptr + 0x38);
    if rem != 0 {
        let destroy_fn = rd::<usize>(rd::<usize>(rem) + 0x20);
        exe::call_va!(destroy_fn, (rem, u8::from(rem != tmp_fn_ptr)) -> ());
    }
}

/// Original: `0x142088af0` (284 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_update_tile_mappings_and_track_142088af0(
    this: usize,
    res_slot: usize,
    coord: usize,
    region: usize,
    fence_val: u64,
) {
    exe::call_va!(0x1_4208_b990, () -> ());
    let mut range_flag: u32 = 1;
    let range_flag_ptr = (&mut range_flag as *mut u32) as usize;
    let queue = rd::<usize>(this + 0x48);
    let update_fn = rd::<usize>(rd::<usize>(queue) + 0x40);
    exe::call_va!(
        update_fn,
        (
            queue,
            rd::<usize>(res_slot),
            1u32,
            coord,
            region,
            0usize,
            1u32,
            range_flag_ptr,
            0usize,
            0usize,
            0u32
        ) -> ()
    );

    let queue2 = rd::<usize>(this + 0x48);
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_8078) as usize) != 0 {
        let g = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6228) as usize);
        let next_v = rd::<u64>(g + 0x8).wrapping_add(1);
        wr::<u64>(g + 0x8, next_v);
        let sig_fn = rd::<usize>(rd::<usize>(queue2) + 0x70);
        exe::call_va!(sig_fn, (queue2, rd::<usize>(g), rd::<u64>(g + 0x8)) -> i32);
    }

    let taken = rd::<usize>(res_slot);
    wr::<usize>(res_slot, 0);
    let len = rd::<u32>(this + 0x70) as u64;
    let cap = rd::<u32>(this + 0x74) as u64;
    if (len << 4) == (cap << 4) {
        let min_cap = len.wrapping_add(1);
        let grow = (cap >> 1).wrapping_add(cap);
        let new_cap = if grow < min_cap { min_cap } else { grow };
        exe::call_va!(0x1_411e_1390, (this + 0x68, new_cap) -> ());
    }
    let idx = rd::<u32>(this + 0x70) as usize;
    let slot = rd::<usize>(this + 0x68).wrapping_add(idx * 16);
    wr::<usize>(slot, taken);
    wr::<u64>(slot + 0x8, fence_val);
    wr::<u32>(this + 0x70, (idx as u32).wrapping_add(1));

    let rem = rd::<usize>(res_slot);
    if rem != 0 {
        let rel = rd::<usize>(rd::<usize>(rem) + 0x10);
        exe::call_va!(rel, (rem) -> ());
    }
    wr::<usize>(res_slot, 0);
}

/// Original: `0x142089770` (320 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_dispatch_tile_mapping_update_142089770(
    this: usize,
    req: usize,
    fence_val: u64,
) {
    exe::call_va!(0x1_4208_9c40, (this, fence_val) -> ());
    let mode = rd::<u32>(this + 0xe8);
    if mode == 0 {
        let mut res = rd::<usize>(req);
        wr::<usize>(req, 0);
        exe::call_va!(
            0x1_4208_8af0,
            (this, (&mut res as *mut usize) as usize, req + 0x8, req + 0x18, fence_val) -> ()
        );
    } else if mode == 1 {
        let mut res = rd::<usize>(req);
        wr::<usize>(req, 0);
        exe::call_va!(
            0x1_4208_8940,
            (this, (&mut res as *mut usize) as usize, req + 0x8, req + 0x18, fence_val) -> ()
        );
    } else if mode == 3 {
        let res = rd::<usize>(req);
        wr::<usize>(req, 0);
        let mut extra_com: usize = 0;
        let extra_com_ptr = (&mut extra_com as *mut usize) as usize;

        let mut key_buf = [0u64; 4];
        let key_ptr = key_buf.as_mut_ptr() as usize;
        wr::<usize>(key_ptr, res);
        wr::<u32>(key_ptr + 0x08, rd::<u32>(req + 0x14));
        wr::<u32>(key_ptr + 0x0c, rd::<u32>(req + 0x08));
        wr::<u32>(key_ptr + 0x10, rd::<u32>(req + 0x0c));
        wr::<usize>(key_ptr + 0x18, extra_com_ptr);

        let mut val_buf = [0u8; 40];
        let val_ptr = val_buf.as_mut_ptr() as usize;
        wr::<usize>(val_ptr, res);
        wr::<[u8; 32]>(val_ptr + 0x08, rd::<[u8; 32]>(req + 0x08));

        let mut key_ref: usize = key_ptr;
        let mut val_ref: usize = val_ptr;
        let mut map_ref: usize = this + 0x98;
        let mut out_pair = [0u8; 24];
        exe::call_va!(
            0x1_411e_1780,
            (
                (&mut map_ref as *mut usize) as usize,
                out_pair.as_mut_ptr() as usize,
                key_ptr,
                0usize,
                (&mut key_ref as *mut usize) as usize,
                (&mut val_ref as *mut usize) as usize
            ) -> usize
        );

        let rem_val = rd::<usize>(val_ptr);
        if rem_val != 0 {
            let rel = rd::<usize>(rd::<usize>(rem_val) + 0x10);
            exe::call_va!(rel, (rem_val) -> ());
        }
        let rem_extra = rd::<usize>(extra_com_ptr);
        if rem_extra != 0 {
            let rel = rd::<usize>(rd::<usize>(rem_extra) + 0x10);
            exe::call_va!(rel, (rem_extra) -> ());
        }
    }
}

/// Original: `0x1420898f0` (325 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_map_tile_range_to_heap_1420898f0(
    this: usize,
    req: usize,
    fence_val: u64,
) {
    exe::call_va!(0x1_4208_8e90, (this, rd::<usize>(req), req + 0xc) -> ());
    exe::call_va!(0x1_4208_b990, () -> ());
    let lock_obj = rd::<usize>(this + 0xe0);
    let idx = rd::<u16>(req + 0x8) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_obj) -> ());
    let arr = rd::<usize>(lock_obj + 0x8);
    let heap = rd::<usize>(arr + idx * 0x18 + 0x10);
    exe::call_iat!(0x1_4264_5598, (lock_obj) -> ());

    let queue = rd::<usize>(this + 0x48);
    let update_fn = rd::<usize>(rd::<usize>(queue) + 0x40);
    exe::call_va!(
        update_fn,
        (
            queue,
            rd::<usize>(req),
            1u32,
            req + 0x0c,
            req + 0x1c,
            heap,
            1u32,
            0usize,
            req + 0x2c,
            req + 0x30,
            0u32
        ) -> ()
    );

    let queue2 = rd::<usize>(this + 0x48);
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_8078) as usize) != 0 {
        let g = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6228) as usize);
        let next_v = rd::<u64>(g + 0x8).wrapping_add(1);
        wr::<u64>(g + 0x8, next_v);
        let sig_fn = rd::<usize>(rd::<usize>(queue2) + 0x70);
        exe::call_va!(sig_fn, (queue2, rd::<usize>(g), rd::<u64>(g + 0x8)) -> i32);
    }

    let taken = rd::<usize>(req);
    wr::<usize>(req, 0);
    let len = rd::<u32>(this + 0x70) as u64;
    let cap = rd::<u32>(this + 0x74) as u64;
    if (len << 4) == (cap << 4) {
        let min_cap = len.wrapping_add(1);
        let grow = (cap >> 1).wrapping_add(cap);
        let new_cap = if grow < min_cap { min_cap } else { grow };
        exe::call_va!(0x1_411e_1390, (this + 0x68, new_cap) -> ());
    }
    let slot_idx = rd::<u32>(this + 0x70) as usize;
    let slot = rd::<usize>(this + 0x68).wrapping_add(slot_idx * 16);
    wr::<usize>(slot, taken);
    wr::<u64>(slot + 0x8, fence_val);
    wr::<u32>(this + 0x70, (slot_idx as u32).wrapping_add(1));
}

/// Original: `0x142089a70` (288 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_retire_completed_tile_cmds_142089a70(this: usize) {
    let fence = rd::<usize>(this + 0xd0);
    let get_comp = rd::<usize>(rd::<usize>(fence) + 0x40);
    let completed = exe::call_va!(get_comp, (fence) -> u64);
    if completed == u64::MAX {
        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32) -> ());
    }

    let base = rd::<usize>(this + 0x68);
    let len = rd::<u32>(this + 0x70) as usize;
    let end = base.wrapping_add(len * 16);
    let mut cur = base;
    while cur != end && rd::<u64>(cur + 0x8) <= completed {
        cur = cur.wrapping_add(16);
    }

    if cur != base {
        if cur == end {
            let mut p = base;
            while p != end {
                let obj = rd::<usize>(p);
                if obj != 0 {
                    let rel = rd::<usize>(rd::<usize>(obj) + 0x10);
                    exe::call_va!(rel, (obj) -> ());
                }
                wr::<usize>(p, 0);
                p = p.wrapping_add(16);
            }
            wr::<u32>(this + 0x70, 0);
        } else {
            let mut dst = base;
            let mut src = cur;
            while src != end {
                let old_obj = rd::<usize>(dst);
                if old_obj != 0 {
                    let rel = rd::<usize>(rd::<usize>(old_obj) + 0x10);
                    exe::call_va!(rel, (old_obj) -> ());
                }
                wr::<usize>(dst, 0);
                let new_obj = rd::<usize>(src);
                wr::<usize>(src, 0);
                wr::<usize>(dst, new_obj);
                wr::<u64>(dst + 0x8, rd::<u64>(src + 0x8));
                dst = dst.wrapping_add(16);
                src = src.wrapping_add(16);
            }
            let cur_end = rd::<usize>(this + 0x68).wrapping_add((rd::<u32>(this + 0x70) as usize) * 16);
            exe::call_va!(0x1_411e_0430, (dst.wrapping_sub(8), dst, cur_end) -> ());
            let removed = ((cur.wrapping_sub(base)) >> 4) as u32;
            wr::<u32>(this + 0x70, rd::<u32>(this + 0x70).wrapping_sub(removed));
        }
    }
    wr::<u8>(this + 0x78, 0);
}

/// Original: `0x14208a320` (266 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_push_stream_callback_and_store_14208a320(
    this: usize,
    fn_src: usize,
) {
    let vec = rd::<usize>(this);
    let len = rd::<u32>(vec + 0x8) as usize;
    let cap = rd::<u32>(vec + 0xc) as usize;
    let mut buf = rd::<usize>(vec);
    if (len << 6) == (cap << 6) {
        let min_cap = len.wrapping_add(1);
        let grow = (cap >> 1).wrapping_add(cap);
        let new_cap = if grow < min_cap { min_cap } else { grow };
        let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap << 6, 8u32) -> usize);
        let mut src_p = rd::<usize>(vec);
        let src_end = src_p.wrapping_add((rd::<u32>(vec + 0x8) as usize) << 6);
        let mut dst_p = new_buf;
        while src_p != src_end {
            wr::<u64>(dst_p + 0x38, 0);
            exe::call_va!(0x1_401f_a4d0, (dst_p, src_p) -> ());
            dst_p = dst_p.wrapping_add(0x40);
            src_p = src_p.wrapping_add(0x40);
        }
        exe::call_va!(0x1_411e_07e0, (vec) -> ());
        wr::<usize>(vec, new_buf);
        wr::<u32>(vec + 0x8, len as u32);
        wr::<u32>(vec + 0xc, new_cap as u32);
        buf = new_buf;
    }
    let slot = buf.wrapping_add(len << 6);
    wr::<u64>(slot + 0x38, 0);
    exe::call_va!(0x1_401f_a4d0, (slot, fn_src) -> ());
    wr::<u32>(vec + 0x8, rd::<u32>(vec + 0x8).wrapping_add(1));
    let out_ptr = rd::<usize>(this + 0x8);
    wr::<u64>(out_ptr, rd::<u64>(this + 0x10));
}

/// Original: `0x14208a870` (327 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_async_streamer_dtor_14208a870(this: usize) {
    wr::<usize>(this, exe::ptr::<u8>(0x1_433d_ce98) as usize);
    let g_sched = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    exe::call_va!(0x1_4240_d480, (g_sched, this) -> ());

    let fence = rd::<usize>(this + 0xd0);
    let get_comp = rd::<usize>(rd::<usize>(fence) + 0x40);
    let comp = exe::call_va!(get_comp, (fence) -> u64);
    if comp == u64::MAX {
        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32) -> ());
    }

    let g_sync_addr = exe::ptr::<usize>(0x1_43a3_6228) as usize;
    let g_sync = rd::<usize>(g_sync_addr);
    if g_sync != 0 {
        let h = rd::<usize>(g_sync + 0x10);
        if h != 0 {
            exe::call_iat!(0x1_4264_55b0, (h) -> i32);
            wr::<usize>(g_sync + 0x10, 0);
        }
        let f = rd::<usize>(g_sync);
        if f != 0 {
            let rel = rd::<usize>(rd::<usize>(f) + 0x10);
            exe::call_va!(rel, (f) -> ());
        }
        wr::<usize>(g_sync, 0);
        exe::call_va!(0x1_4244_2de0, (g_sync) -> ());
    }
    wr::<usize>(g_sync_addr, 0);

    let f_d0 = rd::<usize>(this + 0xd0);
    if f_d0 != 0 {
        let rel = rd::<usize>(rd::<usize>(f_d0) + 0x10);
        exe::call_va!(rel, (f_d0) -> ());
    }
    wr::<usize>(this + 0xd0, 0);

    exe::call_va!(0x1_4208_5580, (this + 0x98) -> ());
    exe::call_va!(0x1_4208_4af0, (this + 0x80) -> ());
    wr::<u64>(this + 0x80, 0);
    wr::<u64>(this + 0x88, 0);
    wr::<u64>(this + 0x90, 0);
    exe::call_va!(0x1_411e_0350, (this + 0x68) -> ());

    for off in [0x60usize, 0x58, 0x50, 0x48] {
        let p = rd::<usize>(this + off);
        if p != 0 {
            let rel = rd::<usize>(rd::<usize>(p) + 0x10);
            exe::call_va!(rel, (p) -> ());
        }
        wr::<usize>(this + off, 0);
    }

    exe::call_va!(0x1_4208_51b0, (this + 0x30) -> ());
    exe::call_va!(0x1_4208_51b0, (this + 0x10) -> ());
}

/// Original: `0x14208b670` (393 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_hash_tile_key_14208b670(_this: usize, key: usize) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    const GOLDEN: u64 = 0x9e37_79b9;

    let v0 = rd::<u64>(key);
    let mut h0 = FNV_OFFSET;
    for i in 0..8 {
        h0 = (h0 ^ ((v0 >> (i * 8)) & 0xff)).wrapping_mul(FNV_PRIME);
    }

    let mut h1 = FNV_OFFSET;
    for i in 0..4 {
        h1 = (h1 ^ (rd::<u8>(key + 0x08 + i) as u64)).wrapping_mul(FNV_PRIME);
    }
    let mut acc = h0 ^ h1.wrapping_add(GOLDEN).wrapping_add(h0 << 6).wrapping_add(h0 >> 2);

    let mut h2 = FNV_OFFSET;
    for i in 0..4 {
        h2 = (h2 ^ (rd::<u8>(key + 0x0c + i) as u64)).wrapping_mul(FNV_PRIME);
    }
    acc ^= h2.wrapping_add(GOLDEN).wrapping_add(acc << 6).wrapping_add(acc >> 2);

    let mut h3 = FNV_OFFSET;
    for i in 0..4 {
        h3 = (h3 ^ (rd::<u8>(key + 0x10 + i) as u64)).wrapping_mul(FNV_PRIME);
    }
    acc ^ h3.wrapping_add(GOLDEN).wrapping_add(acc << 6).wrapping_add(acc >> 2)
}

/// Original: `0x14208bc90` (314 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_submit_stream_buffer_and_signal_14208bc90(
    this: usize,
    com_obj: usize,
    req: usize,
) {
    let inner = rd::<usize>(this + 0x18);
    let mut sub_ptr: usize = inner + 0x30;
    exe::call_iat!(0x1_4264_55a8, (inner + 0x38) -> ());
    exe::call_va!(
        0x1_4208_7760,
        (0usize, (&mut sub_ptr as *mut usize) as usize, com_obj, req) -> ()
    );

    let seq = rd::<u64>(inner + 0xf8);
    wr::<u64>(inner + 0xf8, seq.wrapping_add(1));
    let mut cmd = [0u8; 0x60];
    let cmd_ptr = cmd.as_mut_ptr() as usize;
    wr::<u8>(cmd_ptr + 0x50, 6);
    wr::<u64>(cmd_ptr + 0x58, seq);

    let count = rd::<u32>(inner + 0x50);
    if count == rd::<u32>(inner + 0x54) {
        exe::call_va!(0x1_4208_3150, (inner + 0x40, count.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(inner + 0x4c) as usize;
    let dst = rd::<usize>(inner + 0x40).wrapping_add(tail * 0x60);
    exe::call_va!(0x1_4208_4000, (dst, cmd_ptr) -> usize);
    let next_tail = rd::<u32>(inner + 0x4c).wrapping_add(1);
    wr::<u32>(
        inner + 0x4c,
        if next_tail == rd::<u32>(inner + 0x54) {
            0
        } else {
            next_tail
        },
    );
    wr::<u32>(inner + 0x50, rd::<u32>(inner + 0x50).wrapping_add(1));
    exe::call_va!(0x1_4208_50b0, (cmd_ptr) -> ());

    let q_wrap = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
    let queue = rd::<usize>(q_wrap + 0x10);
    let wait_fn = rd::<usize>(rd::<usize>(queue) + 0x78);
    let hr = exe::call_va!(wait_fn, (queue, rd::<usize>(inner + 0x100), seq) -> i32);
    check_gpu_hr(hr as u32);

    exe::call_iat!(0x1_4264_5598, (inner + 0x38) -> ());

    let g = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    exe::call_iat!(0x1_4264_55a8, (g + 0x250) -> ());
    wr::<u32>(g + 0x258, rd::<u32>(g + 0x258).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (g + 0x250) -> ());
    exe::call_iat!(0x1_4264_5590, (g + 0x248) -> ());
}

/// Original: `0x14208bea0` (332 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_submit_stream_pair_and_signal_14208bea0(
    this: usize,
    pair_ptr: usize,
    req: usize,
) {
    let inner = rd::<usize>(this + 0x18);
    let mut sub_ptr: usize = inner + 0x30;
    exe::call_iat!(0x1_4264_55a8, (inner + 0x38) -> ());
    let mut pair_copy = rd::<[u8; 16]>(pair_ptr);
    exe::call_va!(
        0x1_4208_7920,
        (
            0usize,
            (&mut sub_ptr as *mut usize) as usize,
            pair_copy.as_mut_ptr() as usize,
            req
        ) -> ()
    );

    let seq = rd::<u64>(inner + 0xf8);
    wr::<u64>(inner + 0xf8, seq.wrapping_add(1));
    let mut cmd = [0u8; 0x60];
    let cmd_ptr = cmd.as_mut_ptr() as usize;
    wr::<u8>(cmd_ptr + 0x50, 6);
    wr::<u64>(cmd_ptr + 0x58, seq);

    let count = rd::<u32>(inner + 0x50);
    if count == rd::<u32>(inner + 0x54) {
        exe::call_va!(0x1_4208_3150, (inner + 0x40, count.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(inner + 0x4c) as usize;
    let dst = rd::<usize>(inner + 0x40).wrapping_add(tail * 0x60);
    exe::call_va!(0x1_4208_4000, (dst, cmd_ptr) -> usize);
    let next_tail = rd::<u32>(inner + 0x4c).wrapping_add(1);
    wr::<u32>(
        inner + 0x4c,
        if next_tail == rd::<u32>(inner + 0x54) {
            0
        } else {
            next_tail
        },
    );
    wr::<u32>(inner + 0x50, rd::<u32>(inner + 0x50).wrapping_add(1));
    exe::call_va!(0x1_4208_50b0, (cmd_ptr) -> ());

    let q_wrap = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
    let queue = rd::<usize>(q_wrap + 0x10);
    let wait_fn = rd::<usize>(rd::<usize>(queue) + 0x78);
    let hr = exe::call_va!(wait_fn, (queue, rd::<usize>(inner + 0x100), seq) -> i32);
    check_gpu_hr(hr as u32);

    exe::call_iat!(0x1_4264_5598, (inner + 0x38) -> ());

    let g = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    exe::call_iat!(0x1_4264_55a8, (g + 0x250) -> ());
    wr::<u32>(g + 0x258, rd::<u32>(g + 0x258).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (g + 0x250) -> ());
    exe::call_iat!(0x1_4264_5590, (g + 0x248) -> ());
}

/// Original: `0x14208ca50` (286 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unmap_and_release_upload_chunk_14208ca50(this: usize) {
    let chunk = rd::<usize>(this + 0x58);
    let size = rd::<u64>(chunk + 0x10);
    let offset = rd::<u64>(chunk + 0x8);
    let block = rd::<usize>(chunk);
    let mut range = [offset, offset.wrapping_add(size)];
    let res = rd::<usize>(block + 0x40);
    let unmap_fn = rd::<usize>(rd::<usize>(res) + 0x48);
    exe::call_va!(unmap_fn, (res, 0u32, range.as_mut_ptr() as usize) -> ());

    let chunk2 = rd::<usize>(this + 0x58);
    let sz2 = rd::<u64>(chunk2 + 0x10);
    let off2 = rd::<u64>(chunk2 + 0x8);
    let blk2 = rd::<usize>(chunk2);
    let dst_obj = rd::<usize>(this + 0x50);
    let dst_off = rd::<u64>(dst_obj + 0x8).wrapping_add(rd::<u64>(this + 0x40));
    exe::call_va!(0x1_4207_5bc0, (dst_obj + 0x10, dst_off, blk2, off2, sz2) -> ());

    let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_fb88) as usize);
    let chunk3 = rd::<usize>(this + 0x58);
    if rd::<u64>(chunk3 + 0x10) > 0x10_0000 {
        let blk3 = rd::<usize>(chunk3);
        exe::call_va!(0x1_4207_5a20, (blk3) -> ());
        if blk3 != 0 {
            exe::call_va!(0x1_4243_b3e0, (g + 0xf0, blk3) -> ());
        }
    } else {
        let lock = g + 0x130;
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
        let blk3 = rd::<usize>(chunk3);
        let new_refs = rd::<u64>(blk3 + 0x50).wrapping_sub(1);
        wr::<u64>(blk3 + 0x50, new_refs);
        if new_refs == 0 && blk3 != rd::<usize>(g + 0x138) {
            exe::call_va!(0x1_4207_5a20, (blk3) -> ());
            exe::call_va!(0x1_4243_b3e0, (g + 0xf0, blk3) -> ());
        }
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }

    exe::call_va!(0x1_4243_b3e0, (g + 0xb0, chunk3) -> ());
    wr::<usize>(this + 0x58, 0);
    let counter_ptr = rd::<usize>(this + 0x50) as *const AtomicU64;
    (*counter_ptr).fetch_add(1, Ordering::SeqCst);
}

/// Original: `0x14208cb80` (375 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_alloc_and_map_upload_chunk_14208cb80(this: usize) -> usize {
    let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_fb88) as usize);
    let req_size = rd::<u64>(this + 0x10);
    let raw_chunk = exe::call_va!(0x1_4243_b450, (g + 0xb0) -> usize);
    let chunk = if raw_chunk != 0 {
        wr::<u64>(raw_chunk, 0);
        wr::<u64>(raw_chunk + 0x08, 0);
        wr::<u64>(raw_chunk + 0x10, 0);
        raw_chunk
    } else {
        0
    };

    if req_size > 0x10_0000 {
        let blk = exe::call_va!(0x1_4205_0230, (g, req_size) -> usize);
        wr::<usize>(chunk, blk);
        wr::<u64>(chunk + 0x08, 0);
        wr::<u64>(chunk + 0x10, req_size);
    } else {
        let lock = g + 0x130;
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
        let mut blk = rd::<usize>(g + 0x138);
        if blk == 0 {
            blk = exe::call_va!(0x1_4205_0230, (g, 0x20_0000u64) -> usize);
            wr::<usize>(g + 0x138, blk);
        } else if req_size > rd::<u64>(blk + 0x60) {
            if rd::<u64>(blk + 0x50) == 0 {
                exe::call_va!(0x1_4207_5a20, (blk) -> ());
                exe::call_va!(0x1_4243_b3e0, (g + 0xf0, blk) -> ());
            }
            blk = exe::call_va!(0x1_4205_0230, (g, 0x20_0000u64) -> usize);
            wr::<usize>(g + 0x138, blk);
        }
        wr::<usize>(chunk, blk);
        let cur_blk = rd::<usize>(g + 0x138);
        wr::<u64>(chunk + 0x08, rd::<u64>(cur_blk + 0x58));
        wr::<u64>(chunk + 0x10, req_size);
        let cur_blk2 = rd::<usize>(g + 0x138);
        wr::<u64>(cur_blk2 + 0x50, rd::<u64>(cur_blk2 + 0x50).wrapping_add(1));
        let cur_blk3 = rd::<usize>(g + 0x138);
        wr::<u64>(
            cur_blk3 + 0x58,
            rd::<u64>(cur_blk3 + 0x58).wrapping_add(req_size),
        );
        let cur_blk4 = rd::<usize>(g + 0x138);
        wr::<u64>(
            cur_blk4 + 0x60,
            rd::<u64>(cur_blk4 + 0x60).wrapping_sub(req_size),
        );
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }

    wr::<usize>(this + 0x58, chunk);
    let mut read_range = [0u64; 2];
    let mut mapped_ptr: usize = 0;
    let mapped_ptr_addr = (&mut mapped_ptr as *mut usize) as usize;
    let blk_final = rd::<usize>(chunk);
    let res = rd::<usize>(blk_final + 0x40);
    let map_fn = rd::<usize>(rd::<usize>(res) + 0x40);
    let hr = exe::call_va!(
        map_fn,
        (res, 0u32, read_range.as_mut_ptr() as usize, mapped_ptr_addr) -> i32
    );
    check_gpu_hr(hr as u32);
    let chunk_final = rd::<usize>(this + 0x58);
    rd::<usize>(mapped_ptr_addr).wrapping_add(rd::<usize>(chunk_final + 0x8))
}

/// Original: `0x14208cd70` (266 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_find_adapter_by_luid_14208cd70(
    out: usize,
    factory: usize,
    target_desc: usize,
) -> usize {
    wr::<usize>(out, 0);
    let mut adapter: usize = 0;
    let adapter_ptr = (&mut adapter as *mut usize) as usize;
    let mut idx: u32 = 0;
    let mut desc = [0u8; 0x130];
    let desc_ptr = desc.as_mut_ptr() as usize;

    loop {
        let enum_fn = rd::<usize>(rd::<usize>(factory) + 0x38);
        let cur = rd::<usize>(adapter_ptr);
        if cur != 0 {
            let rel = rd::<usize>(rd::<usize>(cur) + 0x10);
            exe::call_va!(rel, (cur) -> ());
        }
        wr::<usize>(adapter_ptr, 0);
        let hr = exe::call_va!(enum_fn, (factory, idx, adapter_ptr) -> i32);
        if hr as u32 == 0x887a_0002 {
            break;
        }
        let ad = rd::<usize>(adapter_ptr);
        let get_desc = rd::<usize>(rd::<usize>(ad) + 0x40);
        exe::call_va!(get_desc, (ad, desc_ptr) -> i32);
        if rd::<u32>(desc_ptr + 0x12c) == rd::<u32>(target_desc + 0x12c)
            && rd::<u32>(desc_ptr + 0x128) == rd::<u32>(target_desc + 0x128)
        {
            let found = rd::<usize>(adapter_ptr);
            let old_out = rd::<usize>(out);
            if old_out != 0 {
                let rel = rd::<usize>(rd::<usize>(old_out) + 0x10);
                exe::call_va!(rel, (old_out) -> ());
            }
            wr::<usize>(out, 0);
            let qi = rd::<usize>(rd::<usize>(found));
            let iid = exe::ptr::<u8>(0x1_433d_d090) as usize;
            exe::call_va!(qi, (found, iid, out) -> i32);
            break;
        }
        idx = idx.wrapping_add(1);
    }

    let rem = rd::<usize>(adapter_ptr);
    if rem != 0 {
        let rel = rd::<usize>(rd::<usize>(rem) + 0x10);
        exe::call_va!(rel, (rem) -> ());
    }
    out
}

/// Original: `0x14208de00` (247 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_default_pass_entry_14208de00(this: usize) {
    let idx0 = rd::<u32>(this) as usize;
    let dst0 = this.wrapping_add(idx0 * 0x70);
    wr::<u64>(dst0 + 0x08, 0);
    wr::<[u8; 16]>(
        dst0 + 0x10,
        rd::<[u8; 16]>(exe::ptr::<u8>(0x1_433d_d280) as usize),
    );
    wr::<u64>(dst0 + 0x20, 1);
    wr::<u8>(dst0 + 0x28, 0);
    wr::<[u8; 16]>(dst0 + 0x30, [0u8; 16]);
    wr::<u64>(dst0 + 0x40, 0);
    wr::<u32>(dst0 + 0x48, 0xffff_ffff);
    wr::<u64>(dst0 + 0x4c, 0);
    wr::<u64>(dst0 + 0x54, 0);
    wr::<u32>(dst0 + 0x5c, 0);
    wr::<u64>(dst0 + 0x60, 0);
    wr::<u64>(dst0 + 0x6c, 0);

    let idx1 = rd::<u32>(this) as usize;
    let ptr_field = (this + 0x08).wrapping_add(idx1 * 0x70);
    let dst1 = this.wrapping_add(idx1 << 6);
    wr::<u64>(dst1 + 0x78, 0);
    wr::<[u8; 16]>(
        dst1 + 0x80,
        rd::<[u8; 16]>(exe::ptr::<u8>(0x1_433d_d2a0) as usize),
    );
    wr::<u64>(dst1 + 0x90, 1);
    wr::<usize>(dst1 + 0x98, ptr_field);
    wr::<u32>(dst1 + 0xa0, 0x32);
    wr::<u64>(dst1 + 0xa4, 2);
    wr::<u64>(dst1 + 0xac, 0);
    wr::<u32>(dst1 + 0xb4, 0);
    wr::<u32>(this, (idx1 as u32).wrapping_add(1));
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

unsafe extern "win64" fn mock_com_addref_or_release(this: usize) -> u32 {
    let v = rd::<u32>(this + 0x8).wrapping_add(1);
    wr::<u32>(this + 0x8, v);
    v
}

unsafe extern "win64" fn mock_fence_get_completed(this: usize) -> u64 {
    rd::<u64>(this + 0x8)
}

unsafe extern "win64" fn mock_queue_exec_lists(this: usize, count: u32, lists_ptr: usize) {
    wr::<u32>(this + 0x8, count);
    wr::<usize>(this + 0x10, lists_ptr);
}

unsafe extern "win64" fn mock_factory_enum_adapters(
    this: usize,
    idx: u32,
    out_adapter: usize,
) -> i32 {
    if idx == 0 {
        wr::<usize>(out_adapter, rd::<usize>(this + 0x8));
        0
    } else {
        0x887a_0002u32 as i32
    }
}

unsafe extern "win64" fn mock_adapter_get_desc(this: usize, desc: usize) -> i32 {
    wr::<u32>(desc + 0x128, rd::<u32>(this + 0x10));
    wr::<u32>(desc + 0x12c, rd::<u32>(this + 0x14));
    0
}

unsafe extern "win64" fn mock_adapter_qi(this: usize, _iid: usize, out: usize) -> i32 {
    wr::<usize>(out, rd::<usize>(this + 0x18));
    0
}

pub unsafe fn diff_d3d_flush_deferred_releases_locked_142076db0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut bo = [0u8; 0x50];
    let mut bn = [0u8; 0x50];
    exe::call_va!(orig, (bo.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (bn.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(bo, bn, "flush_deferred_empty");
    Ok(1)
}

pub unsafe fn diff_d3d_enqueue_deferred_releases_142076f60(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut item0 = [0u64; 4];
    let mut item1 = [0u64; 4];
    item1[3] = 0x1234_5678;
    let items = [item0.as_mut_ptr() as usize, item1.as_mut_ptr() as usize];

    let mut vec1_o = [0usize; 4];
    let mut vec2_o = [0usize; 4];
    let mut vec1_n = [0usize; 4];
    let mut vec2_n = [0usize; 4];

    let mut bo = [0u8; 0x50];
    let mut bn = [0u8; 0x50];
    let po = bo.as_mut_ptr() as usize;
    let pn = bn.as_mut_ptr() as usize;
    wr::<usize>(po + 0x18, vec1_o.as_mut_ptr() as usize);
    wr::<u32>(po + 0x24, 4);
    wr::<usize>(po + 0x28, vec2_o.as_mut_ptr() as usize);
    wr::<u32>(po + 0x34, 4);

    wr::<usize>(pn + 0x18, vec1_n.as_mut_ptr() as usize);
    wr::<u32>(pn + 0x24, 4);
    wr::<usize>(pn + 0x28, vec2_n.as_mut_ptr() as usize);
    wr::<u32>(pn + 0x34, 4);

    exe::call_va!(orig, (po, 2usize, items.as_ptr() as usize) -> ());
    exe::call_va!(new, (pn, 2usize, items.as_ptr() as usize) -> ());
    crate::diff_eq!(vec1_o, vec1_n, "enqueue_vec1");
    crate::diff_eq!(vec2_o, vec2_n, "enqueue_vec2");
    crate::diff_eq!(rd::<u32>(po + 0x20), rd::<u32>(pn + 0x20), "len1");
    crate::diff_eq!(rd::<u32>(po + 0x30), rd::<u32>(pn + 0x30), "len2");
    Ok(4)
}

pub unsafe fn diff_d3d_fence_is_completed_142077130(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt = [0usize; 9];
    vt[8] = mock_fence_get_completed as *const () as usize;
    let mut cases = 0u32;
    for (comp, target) in [(5u64, 10u64), (10, 10), (15, 10), (0, 0)] {
        let fence = [vt.as_ptr() as usize, comp as usize];
        let holder = [0u64, target, fence.as_ptr() as u64];
        let ro = exe::call_va!(orig, (holder.as_ptr() as usize) -> u8);
        let rn = exe::call_va!(new, (holder.as_ptr() as usize) -> u8);
        crate::diff_eq!(ro, rn, "fence_is_completed");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_execute_command_lists_and_signal_1420772c0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt = [0usize; 16];
    vt[10] = mock_queue_exec_lists as *const () as usize;
    let mut qo = [vt.as_ptr() as usize, 0x99usize, 0x99usize];
    let mut qn = qo;
    let span = [0usize, 0x1000usize];
    exe::call_va!(orig, (qo.as_mut_ptr() as usize, span.as_ptr() as usize) -> ());
    exe::call_va!(new, (qn.as_mut_ptr() as usize, span.as_ptr() as usize) -> ());
    crate::diff_eq!(qo, qn, "exec_empty_span");
    Ok(1)
}

pub unsafe fn diff_d3d_build_blend_state_desc_142077690(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut src = [0u8; 0x28];
    let sp = src.as_mut_ptr() as usize;
    wr::<u8>(sp, 1);
    wr::<i32>(sp + 0x04, 1);
    wr::<i32>(sp + 0x08, 2);
    wr::<i32>(sp + 0x0c, 0);
    wr::<u8>(sp + 0x10, 0x0f);
    wr::<i32>(sp + 0x18, 3);
    wr::<i32>(sp + 0x1c, 4);
    wr::<i32>(sp + 0x20, 1);

    let mut out_o = [0xAAu8; 0x150];
    let mut out_n = [0xAAu8; 0x150];
    let po = out_o.as_mut_ptr() as usize;
    let pn = out_n.as_mut_ptr() as usize;
    let ro = exe::call_va!(orig, (po, sp, 1u8) -> usize);
    let rn = exe::call_va!(new, (pn, sp, 1u8) -> usize);
    crate::diff_eq!(ro - po, rn - pn, "blend_ret");
    crate::diff_eq!(out_o, out_n, "blend_bytes");
    Ok(2)
}

pub unsafe fn diff_d3d_release_mip_level_slot_14207e500(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut bo = [0u8; 0x80];
    let mut bn = [0u8; 0x80];
    for idx in [0u8, 1, 3, 7] {
        exe::call_va!(orig, (bo.as_mut_ptr() as usize, idx) -> ());
        exe::call_va!(new, (bn.as_mut_ptr() as usize, idx) -> ());
        crate::diff_eq!(bo, bn, "release_mip_empty_slot");
    }
    Ok(4)
}

pub unsafe fn diff_d3d_is_small_texture_mip_chain_1420804b0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut cases = 0u32;
    for (w, h, flag10, fmt, mips) in [
        (64u32, 64u32, 1u8, 1u8, 4u8),
        (64, 64, 0, 1, 0),
        (32, 32, 0, 1, 4),
        (512, 512, 0, 1, 6),
        (64, 64, 0, 0x1c, 4),
        (1024, 1024, 0, 0x20, 8),
    ] {
        let mut desc = [0u8; 0x20];
        let dp = desc.as_mut_ptr() as usize;
        wr::<u32>(dp, w);
        wr::<u32>(dp + 0x04, h);
        wr::<u8>(dp + 0x10, flag10);
        wr::<u8>(dp + 0x11, fmt);
        wr::<u8>(dp + 0x12, mips);
        let ro = exe::call_va!(orig, (dp) -> u8);
        let rn = exe::call_va!(new, (dp) -> u8);
        crate::diff_eq!(ro, rn, "is_small_mip_chain");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_stream_cmd_move_construct_142082a40(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut cases = 0u32;
    for tag in 0u8..=7u8 {
        let mut src_o = [0u8; 0x60];
        for (i, b) in src_o.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(5).wrapping_add(1);
        }
        src_o[0x50] = tag;
        if tag == 7 {
            wr::<usize>(src_o.as_mut_ptr() as usize + 0x38, 0xdead_beef);
        }
        let mut src_n = src_o;
        let mut dst_o = [0u8; 0x60];
        let mut dst_n = [0u8; 0x60];
        exe::call_va!(orig, (dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize) -> ());
        exe::call_va!(new, (dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize) -> ());
        crate::diff_eq!(dst_o, dst_n, "move_construct_dst");
        crate::diff_eq!(src_o, src_n, "move_construct_src");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_stream_cmd_move_with_seq_142084000(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut cases = 0u32;
    for tag in 0u8..=7u8 {
        let mut src_o = [0u8; 0x60];
        for (i, b) in src_o.iter_mut().enumerate() {
            *b = (i as u8).wrapping_mul(7).wrapping_add(3);
        }
        src_o[0x50] = tag;
        if tag == 7 {
            wr::<usize>(src_o.as_mut_ptr() as usize + 0x38, 0xcafe_babe);
        }
        let mut src_n = src_o;
        let mut dst_o = [0u8; 0x60];
        let mut dst_n = [0u8; 0x60];
        let po = dst_o.as_mut_ptr() as usize;
        let pn = dst_n.as_mut_ptr() as usize;
        let ro = exe::call_va!(orig, (po, src_o.as_mut_ptr() as usize) -> usize);
        let rn = exe::call_va!(new, (pn, src_n.as_mut_ptr() as usize) -> usize);
        crate::diff_eq!(ro - po, rn - pn, "move_with_seq_ret");
        crate::diff_eq!(dst_o, dst_n, "move_with_seq_dst");
        crate::diff_eq!(src_o, src_n, "move_with_seq_src");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_destroy_com_ring_buffer_142084af0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut bo = [0usize, 0x1111_2222usize, 0x3333_0000_0000_0000usize];
    let mut bn = bo;
    exe::call_va!(orig, (bo.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (bn.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(bo, bn, "destroy_empty_ring");
    Ok(1)
}

pub unsafe fn diff_d3d_free_tile_pool_id_142085c40(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut ids_o = [0u16; 4];
    let mut ids_n = [0u16; 4];
    let mut inner_o = [0u8; 0x1e0];
    let mut inner_n = [0u8; 0x1e0];
    let io = inner_o.as_mut_ptr() as usize;
    let in_ = inner_n.as_mut_ptr() as usize;
    wr::<usize>(io + 0x1c0, ids_o.as_mut_ptr() as usize);
    wr::<u32>(io + 0x1cc, 4);
    wr::<u8>(io + 0x1d0, 1);

    wr::<usize>(in_ + 0x1c0, ids_n.as_mut_ptr() as usize);
    wr::<u32>(in_ + 0x1cc, 4);
    wr::<u8>(in_ + 0x1d0, 1);

    let outer_o = [0usize, 0, 0, io];
    let outer_n = [0usize, 0, 0, in_];
    exe::call_va!(orig, (outer_o.as_ptr() as usize, 0x42u16) -> ());
    exe::call_va!(new, (outer_n.as_ptr() as usize, 0x42u16) -> ());
    crate::diff_eq!(ids_o, ids_n, "free_tile_ids");
    crate::diff_eq!(rd::<u32>(io + 0x1c8), rd::<u32>(in_ + 0x1c8), "free_tile_len");
    Ok(2)
}

pub unsafe fn diff_d3d_alloc_tile_pool_id_142085dc0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut inner = [0u8; 0x220];
    let ip = inner.as_mut_ptr() as usize;
    wr::<u16>(ip + 0x180, 4);
    wr::<u32>(ip + 0x190, 1);
    wr::<u64>(ip + 0x210, 0x100_0000);
    let outer = [0x100_0000u64, 0, 0, ip as u64];
    let ro = exe::call_va!(orig, (outer.as_ptr() as usize) -> u16);
    let rn = exe::call_va!(new, (outer.as_ptr() as usize) -> u16);
    crate::diff_eq!(ro, rn, "alloc_tile_id_over_budget");
    Ok(1)
}

pub unsafe fn diff_d3d_enqueue_stream_buffer_cmd_142087760(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt = [0usize; 4];
    vt[1] = mock_com_addref_or_release as *const () as usize;
    vt[2] = mock_com_addref_or_release as *const () as usize;
    let mut com_o = [vt.as_ptr() as usize, 0usize];
    let mut com_n = com_o;

    let mut ring_o = [0u8; 0x100];
    let mut ring_n = [0u8; 0x100];
    let mut sub_o = [0u8; 0xe0];
    let mut sub_n = [0u8; 0xe0];
    let so = sub_o.as_mut_ptr() as usize;
    let sn = sub_n.as_mut_ptr() as usize;
    wr::<usize>(so + 0x10, ring_o.as_mut_ptr() as usize);
    wr::<u32>(so + 0x24, 2);
    wr::<u64>(so + 0xc8, 77);

    wr::<usize>(sn + 0x10, ring_n.as_mut_ptr() as usize);
    wr::<u32>(sn + 0x24, 2);
    wr::<u64>(sn + 0xc8, 77);

    let ctx_o = [so];
    let ctx_n = [sn];
    let mut req_o = [0u8; 0x40];
    wr::<u64>(req_o.as_mut_ptr() as usize, 0x1111);
    wr::<u16>(req_o.as_mut_ptr() as usize + 0x20, 8);
    let mut req_n = req_o;

    exe::call_va!(
        orig,
        (
            0usize,
            ctx_o.as_ptr() as usize,
            com_o.as_mut_ptr() as usize,
            req_o.as_mut_ptr() as usize
        ) -> ()
    );
    exe::call_va!(
        new,
        (
            0usize,
            ctx_n.as_ptr() as usize,
            com_n.as_mut_ptr() as usize,
            req_n.as_mut_ptr() as usize
        ) -> ()
    );
    crate::diff_eq!(com_o[1], com_n[1], "com_ref_count");
    crate::diff_eq!(req_o, req_n, "req_after");
    crate::diff_eq!(&ring_o[0x08..0x60], &ring_n[0x08..0x60], "ring_slot_payload");
    crate::diff_eq!(sub_o[0x1c..], sub_n[0x1c..], "sub_state");
    Ok(4)
}

pub unsafe fn diff_d3d_enqueue_stream_callback_cmd_1420886c0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut ring_o = [0u8; 0x100];
    let mut ring_n = [0u8; 0x100];
    let mut sub_o = [0u8; 0xe0];
    let mut sub_n = [0u8; 0xe0];
    let so = sub_o.as_mut_ptr() as usize;
    let sn = sub_n.as_mut_ptr() as usize;
    wr::<usize>(so + 0x10, ring_o.as_mut_ptr() as usize);
    wr::<u32>(so + 0x24, 2);
    wr::<u64>(so + 0xc8, 99);

    wr::<usize>(sn + 0x10, ring_n.as_mut_ptr() as usize);
    wr::<u32>(sn + 0x24, 2);
    wr::<u64>(sn + 0xc8, 99);

    let ctx_o = [so];
    let ctx_n = [sn];
    let mut fn_o = [0u64; 8];
    fn_o[7] = 0x5555_6666;
    let mut fn_n = fn_o;

    exe::call_va!(orig, (ctx_o.as_ptr() as usize, fn_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (ctx_n.as_ptr() as usize, fn_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(fn_o, fn_n, "fn_cleared");
    crate::diff_eq!(
        rd::<u64>(ring_o.as_ptr() as usize + 0x38),
        rd::<u64>(ring_n.as_ptr() as usize + 0x38),
        "ring_callable"
    );
    crate::diff_eq!(
        rd::<u8>(ring_o.as_ptr() as usize + 0x50),
        rd::<u8>(ring_n.as_ptr() as usize + 0x50),
        "ring_tag"
    );
    crate::diff_eq!(
        rd::<u64>(ring_o.as_ptr() as usize + 0x58),
        rd::<u64>(ring_n.as_ptr() as usize + 0x58),
        "ring_seq"
    );
    Ok(4)
}

pub unsafe fn diff_d3d_dispatch_tile_mapping_update_142089770(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut bo = [0u8; 0x100];
    wr::<u32>(bo.as_mut_ptr() as usize + 0xe8, 2);
    let mut bn = bo;
    let mut req_o = [0x1234u64; 5];
    let mut req_n = req_o;
    exe::call_va!(
        orig,
        (bo.as_mut_ptr() as usize, req_o.as_mut_ptr() as usize, 42u64) -> ()
    );
    exe::call_va!(
        new,
        (bn.as_mut_ptr() as usize, req_n.as_mut_ptr() as usize, 42u64) -> ()
    );
    crate::diff_eq!(bo, bn, "dispatch_mode_noop");
    crate::diff_eq!(req_o, req_n, "dispatch_req_untouched");
    Ok(2)
}

pub unsafe fn diff_d3d_retire_completed_tile_cmds_142089a70(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt_fence = [0usize; 9];
    vt_fence[8] = mock_fence_get_completed as *const () as usize;
    let fence = [vt_fence.as_ptr() as usize, 10usize];

    let mut vt_com = [0usize; 4];
    vt_com[2] = mock_com_addref_or_release as *const () as usize;
    let mut com0_o = [vt_com.as_ptr() as usize, 0usize];
    let mut com1_o = [vt_com.as_ptr() as usize, 0usize];
    let mut com2_o = [vt_com.as_ptr() as usize, 0usize];
    let mut com0_n = com0_o;
    let mut com1_n = com1_o;
    let mut com2_n = com2_o;

    let mut entries_o = [
        com0_o.as_mut_ptr() as u64,
        5,
        com1_o.as_mut_ptr() as u64,
        10,
        com2_o.as_mut_ptr() as u64,
        15,
    ];
    let mut entries_n = [
        com0_n.as_mut_ptr() as u64,
        5,
        com1_n.as_mut_ptr() as u64,
        10,
        com2_n.as_mut_ptr() as u64,
        15,
    ];

    let mut bo = [0u8; 0xe0];
    let mut bn = [0u8; 0xe0];
    let po = bo.as_mut_ptr() as usize;
    let pn = bn.as_mut_ptr() as usize;
    wr::<usize>(po + 0x68, entries_o.as_mut_ptr() as usize);
    wr::<u32>(po + 0x70, 3);
    wr::<u8>(po + 0x78, 1);
    wr::<usize>(po + 0xd0, fence.as_ptr() as usize);

    wr::<usize>(pn + 0x68, entries_n.as_mut_ptr() as usize);
    wr::<u32>(pn + 0x70, 3);
    wr::<u8>(pn + 0x78, 1);
    wr::<usize>(pn + 0xd0, fence.as_ptr() as usize);

    exe::call_va!(orig, (po) -> ());
    exe::call_va!(new, (pn) -> ());
    crate::diff_eq!(com0_o[1], com0_n[1], "com0_released");
    crate::diff_eq!(com1_o[1], com1_n[1], "com1_released");
    crate::diff_eq!(com2_o[1], com2_n[1], "com2_kept");
    crate::diff_eq!(rd::<u32>(po + 0x70), rd::<u32>(pn + 0x70), "remaining_len");
    crate::diff_eq!(rd::<u8>(po + 0x78), rd::<u8>(pn + 0x78), "flag_cleared");
    crate::diff_eq!(entries_o[1], entries_n[1], "shifted_fence_val");
    Ok(6)
}

pub unsafe fn diff_d3d_push_stream_callback_and_store_14208a320(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf_o = [0u8; 0x80];
    let mut buf_n = [0u8; 0x80];
    let mut vec_o = [buf_o.as_mut_ptr() as u64, 2u64 << 32];
    let mut vec_n = [buf_n.as_mut_ptr() as u64, 2u64 << 32];
    let mut out_o = 0u64;
    let mut out_n = 0u64;
    let this_o = [
        vec_o.as_mut_ptr() as u64,
        (&mut out_o as *mut u64) as u64,
        0x7788_99aau64,
    ];
    let this_n = [
        vec_n.as_mut_ptr() as u64,
        (&mut out_n as *mut u64) as u64,
        0x7788_99aau64,
    ];
    let mut fn_o = [0u64; 8];
    fn_o[7] = 0x1357_2468;
    let mut fn_n = fn_o;

    exe::call_va!(orig, (this_o.as_ptr() as usize, fn_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (this_n.as_ptr() as usize, fn_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(buf_o, buf_n, "pushed_slot");
    crate::diff_eq!(vec_o[1], vec_n[1], "vec_len_cap");
    crate::diff_eq!(out_o, out_n, "stored_val");
    Ok(3)
}

pub unsafe fn diff_d3d_hash_tile_key_14208b670(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut cases = 0u32;
    for seed in [0u8, 1, 7, 42, 0xff] {
        let mut key = [0u8; 24];
        for (i, b) in key.iter_mut().enumerate() {
            *b = seed.wrapping_add((i as u8).wrapping_mul(19));
        }
        let ro = exe::call_va!(orig, (0usize, key.as_ptr() as usize) -> u64);
        let rn = exe::call_va!(new, (0usize, key.as_ptr() as usize) -> u64);
        crate::diff_eq!(ro, rn, "hash_tile_key");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_find_adapter_by_luid_14208cd70(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt_ad = [0usize; 9];
    vt_ad[0] = mock_adapter_qi as *const () as usize;
    vt_ad[2] = mock_com_addref_or_release as *const () as usize;
    vt_ad[8] = mock_adapter_get_desc as *const () as usize;
    let mut ad_o = [
        vt_ad.as_ptr() as usize,
        0usize,
        (0x2222_3333u64 << 32 | 0x1111_4444u64) as usize,
        0xabcd_ef01usize,
    ];
    let mut ad_n = ad_o;

    let mut vt_fac = [0usize; 8];
    vt_fac[7] = mock_factory_enum_adapters as *const () as usize;
    let fac_o = [vt_fac.as_ptr() as usize, ad_o.as_mut_ptr() as usize];
    let fac_n = [vt_fac.as_ptr() as usize, ad_n.as_mut_ptr() as usize];

    let mut target = [0u8; 0x130];
    wr::<u32>(target.as_mut_ptr() as usize + 0x128, 0x1111_4444);
    wr::<u32>(target.as_mut_ptr() as usize + 0x12c, 0x2222_3333);

    let mut out_o = 0usize;
    let mut out_n = 0usize;
    exe::call_va!(
        orig,
        (
            (&mut out_o as *mut usize) as usize,
            fac_o.as_ptr() as usize,
            target.as_ptr() as usize
        ) -> usize
    );
    exe::call_va!(
        new,
        (
            (&mut out_n as *mut usize) as usize,
            fac_n.as_ptr() as usize,
            target.as_ptr() as usize
        ) -> usize
    );
    crate::diff_eq!(out_o, out_n, "found_adapter_qi");
    crate::diff_eq!(ad_o[1], ad_n[1], "adapter_released");
    Ok(2)
}

pub unsafe fn diff_d3d_init_default_pass_entry_14208de00(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut bo = [0x5Au8; 0x100];
    let mut bn = [0x5Au8; 0x100];
    let po = bo.as_mut_ptr() as usize;
    let pn = bn.as_mut_ptr() as usize;
    wr::<u32>(po, 0);
    wr::<u32>(pn, 0);
    exe::call_va!(orig, (po) -> ());
    exe::call_va!(new, (pn) -> ());
    crate::diff_eq!(rd::<u32>(po), rd::<u32>(pn), "pass_entry_count");
    crate::diff_eq!(&bo[0x08..0x29], &bn[0x08..0x29], "pass_entry_part1");
    crate::diff_eq!(&bo[0x30..0x68], &bn[0x30..0x68], "pass_entry_part2");
    crate::diff_eq!(&bo[0x6c..0x74], &bn[0x6c..0x74], "pass_entry_part3");
    crate::diff_eq!(&bo[0x78..0x98], &bn[0x78..0x98], "pass_entry_part4");
    crate::diff_eq!(
        rd::<usize>(po + 0x98) - po,
        rd::<usize>(pn + 0x98) - pn,
        "pass_entry_self_ptr"
    );
    crate::diff_eq!(&bo[0xa0..0xb8], &bn[0xa0..0xb8], "pass_entry_part5");
    Ok(7)
}

unsafe extern "win64" fn mock_queue_get_ts_freq(queue: usize, out_freq: usize) -> i32 {
    wr::<u64>(out_freq, rd::<u64>(queue + 0x8));
    0
}

pub unsafe fn diff_d3d_get_queue_timestamp_frequency_142076f00(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt = [0usize; 20];
    vt[0x80 / 8] = mock_queue_get_ts_freq as *const () as usize;
    let g_addr = exe::ptr::<usize>(0x1_4397_fc60) as usize;
    let saved_g = rd::<usize>(g_addr);

    for freq in [24_000_000u64, 1_000_000_000u64] {
        let queue = [vt.as_ptr() as u64, freq];
        let g = [0usize, 0, queue.as_ptr() as usize];
        wr::<usize>(g_addr, g.as_ptr() as usize);
        let ro = exe::call_va!(orig, (0usize) -> u64);
        let rn = exe::call_va!(new, (0usize) -> u64);
        crate::diff_eq!(ro, rn, "timestamp_frequency");
    }
    wr::<usize>(g_addr, saved_g);
    Ok(2)
}

unsafe extern "win64" fn mock_1411deb90(this: usize, cmd_type: u32) -> usize {
    wr::<u32>(this + 0x20, cmd_type);
    this
}

unsafe extern "win64" fn mock_cmd_ext_qi(this: usize, iid: usize, out: usize) -> i32 {
    if iid == exe::ptr::<u8>(0x1_433d_ccc8) as usize {
        wr::<usize>(out, rd::<usize>(this + 0x8));
    } else if iid == exe::ptr::<u8>(0x1_433d_cc98) as usize {
        wr::<usize>(out, rd::<usize>(this + 0x10));
    }
    0
}

pub unsafe fn diff_d3d_init_cmd_list_with_ext_interfaces_1420770a0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_411d_eb90, mock_1411deb90 as *const ())?;
    let mut vt = [0usize; 4];
    vt[0] = mock_cmd_ext_qi as *const () as usize;
    let cmd_obj = [vt.as_ptr() as usize, 0xaaaa_1111usize, 0xbbbb_2222usize];

    let mut this_o = [0u8; 0x48];
    let mut this_n = [0u8; 0x48];
    let po = this_o.as_mut_ptr() as usize;
    let pn = this_n.as_mut_ptr() as usize;
    wr::<usize>(po + 0x28, cmd_obj.as_ptr() as usize);
    wr::<usize>(pn + 0x28, cmd_obj.as_ptr() as usize);

    let ro = exe::call_va!(orig, (po, 3u32) -> usize);
    let rn = exe::call_va!(new, (pn, 3u32) -> usize);
    crate::diff_eq!(ro, po, "ret orig");
    crate::diff_eq!(rn, pn, "ret new");
    crate::diff_eq!(this_o, this_n, "cmd_list ext interfaces");
    Ok(1)
}

unsafe extern "win64" fn mock_dev_create_cmd_alloc(
    dev: usize,
    cmd_type: u32,
    _iid: usize,
    out: usize,
) -> i32 {
    wr::<u32>(dev + 0x8, cmd_type);
    wr::<usize>(out, 0x1111_2222);
    0
}

unsafe extern "win64" fn mock_dev_create_fence(
    dev: usize,
    init_val: u64,
    flags: u32,
    _iid: usize,
    out: usize,
) -> i32 {
    wr::<u64>(dev + 0x10, init_val);
    wr::<u32>(dev + 0x18, flags);
    wr::<usize>(out, 0x3333_4444);
    0
}

pub unsafe fn diff_d3d_init_cmd_alloc_and_fence_1420771d0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt = [0usize; 40];
    vt[0x48 / 8] = mock_dev_create_cmd_alloc as *const () as usize;
    vt[0x120 / 8] = mock_dev_create_fence as *const () as usize;

    let mut dev_o = [0u8; 0x20];
    let mut dev_n = [0u8; 0x20];
    wr::<usize>(dev_o.as_mut_ptr() as usize, vt.as_ptr() as usize);
    wr::<usize>(dev_n.as_mut_ptr() as usize, vt.as_ptr() as usize);

    let mut this_o = [0x5au8; 0x28];
    let mut this_n = [0x5au8; 0x28];
    let po = this_o.as_mut_ptr() as usize;
    let pn = this_n.as_mut_ptr() as usize;

    let dev_addr = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_dev = rd::<usize>(dev_addr);

    wr::<usize>(dev_addr, dev_o.as_mut_ptr() as usize);
    let ro = exe::call_va!(orig, (po, 2u32) -> usize);
    wr::<usize>(dev_addr, dev_n.as_mut_ptr() as usize);
    let rn = exe::call_va!(new, (pn, 2u32) -> usize);
    wr::<usize>(dev_addr, saved_dev);

    crate::diff_eq!(ro, po, "ret orig");
    crate::diff_eq!(rn, pn, "ret new");
    crate::diff_eq!(this_o, this_n, "alloc and fence state");
    crate::diff_eq!(&dev_o[0x8..], &dev_n[0x8..], "dev call args");
    Ok(1)
}

unsafe extern "win64" fn mock_queue_signal(queue: usize, fence: usize, val: u64) -> i32 {
    wr::<usize>(queue + 0x8, fence);
    wr::<u64>(queue + 0x10, val);
    0
}

pub unsafe fn diff_d3d_signal_global_fence_on_queue_1420773c0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt = [0usize; 16];
    vt[0x70 / 8] = mock_queue_signal as *const () as usize;
    let mut qo = [vt.as_ptr() as usize, 0, 0];
    let mut qn = qo;
    let wrap_o = [0usize, 0, qo.as_mut_ptr() as usize];
    let wrap_n = [0usize, 0, qn.as_mut_ptr() as usize];

    let mut fs_o = [0u64, 41u64, 0x7777_8888u64];
    let mut fs_n = fs_o;

    let g_addr = exe::ptr::<usize>(0x1_4397_fc68) as usize;
    let saved_g = rd::<usize>(g_addr);

    wr::<usize>(g_addr, fs_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (0usize, wrap_o.as_ptr() as usize) -> ());

    wr::<usize>(g_addr, fs_n.as_mut_ptr() as usize);
    exe::call_va!(new, (0usize, wrap_n.as_ptr() as usize) -> ());
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(&qo[1..], &qn[1..], "queue signal args");
    crate::diff_eq!(fs_o, fs_n, "fence_state incremented");
    Ok(1)
}

static mut FAKE_BLOB_9D80: [usize; 4] = [0usize; 4];

unsafe extern "win64" fn mock_blob_get_ptr(_blob: usize) -> usize {
    0xbeef_1000
}

unsafe extern "win64" fn mock_blob_get_size(_blob: usize) -> usize {
    0x140
}

unsafe extern "win64" fn mock_serialize_root_sig(
    desc_ptr: usize,
    version: u32,
    blob_out: usize,
    _err_out: usize,
) -> i32 {
    let b = core::ptr::addr_of_mut!(FAKE_BLOB_9D80) as usize;
    wr::<u32>(
        b + 0x10,
        rd::<u32>(desc_ptr) ^ rd::<u32>(desc_ptr + 0x10) ^ version,
    );
    wr::<u32>(b + 0x14, rd::<u32>(desc_ptr + 0x20));
    wr::<u64>(
        b + 0x18,
        rd::<u64>(desc_ptr + 0x8) ^ rd::<u64>(desc_ptr + 0x18),
    );
    wr::<usize>(blob_out, b);
    0
}

unsafe extern "win64" fn mock_release_tagged_14209a1e0(obj: usize, tag: u32) {
    let b = core::ptr::addr_of_mut!(FAKE_BLOB_9D80) as usize;
    if obj == b {
        wr::<u32>(b + 0x8, rd::<u32>(b + 0x8).wrapping_add(10 + tag));
    }
}

unsafe extern "win64" fn mock_dev_create_root_sig(
    dev: usize,
    mask: u32,
    ptr: usize,
    sz: usize,
    _iid: usize,
    out: usize,
) -> i32 {
    wr::<u32>(dev + 0x8, mask);
    wr::<usize>(dev + 0x10, ptr);
    wr::<usize>(dev + 0x18, sz);
    wr::<usize>(out, 0xcafe_babe);
    0
}

pub unsafe fn diff_d3d_create_root_signature_from_spans_142079d80(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _im = crate::difftest::IatMock::new(0x1_4264_7338, mock_serialize_root_sig as *const ())?;
    let _m = crate::difftest::Mock::new(0x1_4209_a1e0, mock_release_tagged_14209a1e0 as *const ())?;

    let mut blob_vt = [0usize; 6];
    blob_vt[0x18 / 8] = mock_blob_get_ptr as *const () as usize;
    blob_vt[0x20 / 8] = mock_blob_get_size as *const () as usize;

    let mut dev_vt = [0usize; 20];
    dev_vt[0x80 / 8] = mock_dev_create_root_sig as *const () as usize;
    let mut dev_o = [0u8; 0x20];
    let mut dev_n = [0u8; 0x20];
    wr::<usize>(dev_o.as_mut_ptr() as usize, dev_vt.as_ptr() as usize);
    wr::<usize>(dev_n.as_mut_ptr() as usize, dev_vt.as_ptr() as usize);

    let params_span = [3u64, 0x1111_0000u64];
    let samplers_span = [2u64, 0x2222_0000u64];
    let mut out_o = [0x5au8; 0x10];
    let mut out_n = [0x5au8; 0x10];
    let po = out_o.as_mut_ptr() as usize;
    let pn = out_n.as_mut_ptr() as usize;

    let dev_addr = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_dev = rd::<usize>(dev_addr);
    let b_base = core::ptr::addr_of_mut!(FAKE_BLOB_9D80) as usize;

    wr::<[usize; 4]>(b_base, [blob_vt.as_ptr() as usize, 0, 0, 0]);
    wr::<usize>(dev_addr, dev_o.as_mut_ptr() as usize);
    let ro = exe::call_va!(
        orig,
        (po, params_span.as_ptr() as usize, samplers_span.as_ptr() as usize) -> usize
    );
    let blob_o = rd::<[usize; 4]>(b_base);

    wr::<[usize; 4]>(b_base, [blob_vt.as_ptr() as usize, 0, 0, 0]);
    wr::<usize>(dev_addr, dev_n.as_mut_ptr() as usize);
    let rn = exe::call_va!(
        new,
        (pn, params_span.as_ptr() as usize, samplers_span.as_ptr() as usize) -> usize
    );
    let blob_n = rd::<[usize; 4]>(b_base);
    wr::<usize>(dev_addr, saved_dev);

    crate::diff_eq!(ro, po, "ret orig");
    crate::diff_eq!(rn, pn, "ret new");
    crate::diff_eq!(rd::<u32>(po), rd::<u32>(pn), "out tag");
    crate::diff_eq!(rd::<usize>(po + 0x8), rd::<usize>(pn + 0x8), "out ptr");
    crate::diff_eq!(&dev_o[0x8..], &dev_n[0x8..], "dev create_rs args");
    crate::diff_eq!(blob_o[2] >> 32, 0, "original root signature flags");
    crate::diff_eq!(blob_n[2] >> 32, blob_o[2] >> 32, "serialized root signature flags");
    crate::diff_eq!(blob_o, blob_n, "blob serialized and released");
    Ok(1)
}

unsafe extern "win64" fn mock_sc_get_fs(_sc: usize, fs_out: usize, _target: usize) -> i32 {
    wr::<u32>(fs_out, 0);
    0
}

unsafe extern "win64" fn mock_sc_get_idx(_sc: usize) -> u32 {
    2
}

unsafe extern "win64" fn mock_is_iconic(_hwnd: usize) -> i32 {
    0
}

unsafe extern "win64" fn mock_resize_sc_14207be30(
    sc: usize,
    arg2: u8,
    _arg3: usize,
    w: u32,
    h: u32,
    arg6: u8,
) -> u8 {
    wr::<u32>(sc + 0x8, w ^ h ^ (arg2 as u32) ^ (arg6 as u32));
    1
}

unsafe extern "win64" fn mock_is_fs_14207bd60(_sc: usize) -> u8 {
    1
}

pub unsafe fn diff_d3d_advance_swapchain_frame_14207a540(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _im = crate::difftest::IatMock::new(0x1_4264_61c0, mock_is_iconic as *const ())?;
    let _m0 = crate::difftest::Mock::new(0x1_4207_be30, mock_resize_sc_14207be30 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4207_bd60, mock_is_fs_14207bd60 as *const ())?;

    let mut sc_vt = [0usize; 40];
    sc_vt[0x58 / 8] = mock_sc_get_fs as *const () as usize;
    sc_vt[0x120 / 8] = mock_sc_get_idx as *const () as usize;

    let mut f_vt = [0usize; 10];
    f_vt[0x40 / 8] = mock_fence_get_completed as *const () as usize;
    let fence = [f_vt.as_ptr() as usize, 10usize];

    let mut q_vt = [0usize; 16];
    q_vt[0x70 / 8] = mock_queue_signal as *const () as usize;
    let mut qo = [q_vt.as_ptr() as usize, 0, 0];
    let mut qn = qo;
    let q_wrap_o = [0usize, 0, qo.as_mut_ptr() as usize];
    let q_wrap_n = [0usize, 0, qn.as_mut_ptr() as usize];

    let mut sc_o = [sc_vt.as_ptr() as usize, 0];
    let mut sc_n = sc_o;

    let mut inner_o = [0u8; 0x50];
    let mut inner_n = [0u8; 0x50];
    let mut this_o = [0u8; 0x70];
    let mut this_n = [0u8; 0x70];
    for (t, inner, sc) in [
        (&mut this_o, &mut inner_o, &mut sc_o),
        (&mut this_n, &mut inner_n, &mut sc_n),
    ] {
        let ib = inner.as_mut_ptr() as usize;
        let tb = t.as_mut_ptr() as usize;
        wr::<usize>(ib, 0x1234);
        wr::<usize>(ib + 0x8, sc.as_mut_ptr() as usize);
        wr::<usize>(ib + 0x20, fence.as_ptr() as usize);
        wr::<u64>(ib + 0x28, 10);
        wr::<u8>(ib + 0x41, 1);
        wr::<usize>(tb, ib);
        wr::<u32>(tb + 0x40, 1920);
        wr::<u32>(tb + 0x44, 1080);
        wr::<u8>(tb + 0x66, 3);
    }

    let q_addr = exe::ptr::<usize>(0x1_4397_fc60) as usize;
    let wait_flag_addr = exe::ptr::<u8>(0x1_4385_7fd0) as usize;
    let saved_q = rd::<usize>(q_addr);
    let saved_wait = rd::<u8>(wait_flag_addr);

    wr::<u8>(wait_flag_addr, 1);
    wr::<usize>(q_addr, q_wrap_o.as_ptr() as usize);
    exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> ());

    wr::<usize>(q_addr, q_wrap_n.as_ptr() as usize);
    exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> ());

    wr::<usize>(q_addr, saved_q);
    wr::<u8>(wait_flag_addr, saved_wait);

    crate::diff_eq!(&this_o[0x8..], &this_n[0x8..], "this frame idx");
    crate::diff_eq!(&inner_o[0x28..0x48], &inner_n[0x28..0x48], "inner fence and fs");
    crate::diff_eq!(sc_o[1], sc_n[1], "resize_sc called");
    crate::diff_eq!(&qo[1..], &qn[1..], "queue signal args");
    Ok(1)
}

unsafe extern "win64" fn mock_14207c130(arg3: usize) {
    wr::<u32>(arg3, rd::<u32>(arg3).wrapping_add(1));
}

unsafe extern "win64" fn mock_14207bf60() {}

unsafe extern "win64" fn mock_14207c4b0(sc: usize, w_ptr: usize, h_ptr: usize, arg6: u8) -> u8 {
    if rd::<u8>(sc + 0x8) != 0 {
        wr::<u32>(w_ptr, rd::<u32>(w_ptr) + 10);
        wr::<u32>(h_ptr, rd::<u32>(h_ptr) + 20);
        wr::<u8>(sc + 0x9, arg6);
        1
    } else {
        0
    }
}

unsafe extern "win64" fn mock_sc_get_desc(sc: usize, desc_ptr: usize) -> i32 {
    wr::<usize>(desc_ptr + 0x30, rd::<usize>(sc + 0x10));
    0
}

unsafe extern "win64" fn mock_get_client_rect(_hwnd: usize, rect_ptr: usize) -> i32 {
    wr::<i32>(rect_ptr, 0);
    wr::<i32>(rect_ptr + 0x4, 0);
    wr::<i32>(rect_ptr + 0x8, 1280);
    wr::<i32>(rect_ptr + 0xc, 720);
    1
}

unsafe extern "win64" fn mock_14207c410(sc: usize, w: u32, h: u32, flag: u8) -> u8 {
    wr::<u32>(sc + 0x18, w);
    wr::<u32>(sc + 0x1c, h);
    wr::<u8>(sc + 0x20, flag);
    1
}

unsafe extern "win64" fn mock_14207c040(sc: usize, arg2: u8, arg3: usize) {
    wr::<u8>(sc + 0x21, arg2);
    wr::<u32>(arg3 + 0x4, 0x99);
}

pub unsafe fn diff_d3d_resize_swapchain_buffers_14207be30(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4207_c130, mock_14207c130 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4207_bf60, mock_14207bf60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_c4b0, mock_14207c4b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_c410, mock_14207c410 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_c040, mock_14207c040 as *const ())?;
    let _im = crate::difftest::IatMock::new(0x1_4264_6120, mock_get_client_rect as *const ())?;

    let mut vt = [0usize; 16];
    vt[0x60 / 8] = mock_sc_get_desc as *const () as usize;

    for ok1_flag in [1u8, 0u8] {
        let mut sc_o = [0u8; 0x28];
        let mut sc_n = [0u8; 0x28];
        for sc in [&mut sc_o, &mut sc_n] {
            let p = sc.as_mut_ptr() as usize;
            wr::<usize>(p, vt.as_ptr() as usize);
            wr::<u8>(p + 0x8, ok1_flag);
            wr::<usize>(p + 0x10, 0x4444);
        }
        let mut arg3_o = [0u32; 2];
        let mut arg3_n = [0u32; 2];

        let ro = exe::call_va!(
            orig,
            (sc_o.as_mut_ptr() as usize, 5u8, arg3_o.as_mut_ptr() as usize, 800u32, 600u32, 1u8) -> u8
        );
        let rn = exe::call_va!(
            new,
            (sc_n.as_mut_ptr() as usize, 5u8, arg3_n.as_mut_ptr() as usize, 800u32, 600u32, 1u8) -> u8
        );
        crate::diff_eq!(ro, rn, "resize ret");
        crate::diff_eq!(&sc_o[0x8..], &sc_n[0x8..], "sc state");
        crate::diff_eq!(arg3_o, arg3_n, "arg3 state");
    }
    Ok(2)
}

static mut REINIT_MIP_ACC: u32 = 0;

unsafe extern "win64" fn mock_14207e9d0(this: usize, local_ptr: usize) {
    wr::<u32>(this, 0xaa);
    wr::<u8>(local_ptr + 0x83, 2);
}

unsafe extern "win64" fn mock_14207e500(_local_ptr: usize, idx: u8) {
    REINIT_MIP_ACC = REINIT_MIP_ACC.wrapping_add((idx as u32) + 1);
}

unsafe extern "win64" fn mock_14207ec20(this: usize) {
    wr::<u32>(this + 0x4, 0xbb);
}

pub unsafe fn diff_d3d_reinit_streaming_texture_14207fec0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4207_e9d0, mock_14207e9d0 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4207_e500, mock_14207e500 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_ec20, mock_14207ec20 as *const ())?;

    let mut src = [0u8; 0x28];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(3).wrapping_add(1);
    }

    let mut this_o = [0u8; 0xa0];
    let mut this_n = [0u8; 0xa0];

    REINIT_MIP_ACC = 0;
    exe::call_va!(orig, (this_o.as_mut_ptr() as usize, src.as_ptr() as usize) -> ());
    let acc_o = REINIT_MIP_ACC;

    REINIT_MIP_ACC = 0;
    exe::call_va!(new, (this_n.as_mut_ptr() as usize, src.as_ptr() as usize) -> ());
    let acc_n = REINIT_MIP_ACC;

    crate::diff_eq!(this_o, this_n, "reinit streaming texture state");
    crate::diff_eq!(acc_o, acc_n, "mip release loop count");
    Ok(1)
}

unsafe extern "win64" fn mock_14208b990() {}

#[allow(clippy::too_many_arguments)]
unsafe extern "win64" fn mock_queue_update_tile_mappings(
    queue: usize,
    res: usize,
    num_regions: u32,
    coord: usize,
    region: usize,
    heap: usize,
    num_ranges: u32,
    range_flags: usize,
    heap_offsets: usize,
    range_counts: usize,
    flags: u32,
) {
    wr::<usize>(queue + 0x18, res);
    wr::<u32>(queue + 0x20, num_regions);
    wr::<u32>(queue + 0x24, rd::<u32>(coord));
    wr::<u32>(queue + 0x28, rd::<u32>(region));
    wr::<usize>(queue + 0x30, heap);
    wr::<u32>(queue + 0x38, num_ranges);
    wr::<u32>(
        queue + 0x3c,
        if range_flags != 0 { rd::<u32>(range_flags) } else { 0 },
    );
    wr::<u32>(
        queue + 0x40,
        if heap_offsets != 0 { rd::<u32>(heap_offsets) } else { 0 },
    );
    wr::<u32>(
        queue + 0x44,
        if range_counts != 0 { rd::<u32>(range_counts) } else { 0 },
    );
    wr::<u32>(queue + 0x48, flags);
}

pub unsafe fn diff_d3d_update_tile_mappings_and_track_142088af0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4208_b990, mock_14208b990 as *const ())?;
    let mut vt = [0usize; 16];
    vt[0x40 / 8] = mock_queue_update_tile_mappings as *const () as usize;
    vt[0x70 / 8] = mock_queue_signal as *const () as usize;

    let mut qo = [0u8; 0x50];
    let mut qn = [0u8; 0x50];
    wr::<usize>(qo.as_mut_ptr() as usize, vt.as_ptr() as usize);
    wr::<usize>(qn.as_mut_ptr() as usize, vt.as_ptr() as usize);

    let mut entries_o = [0u64; 4];
    let mut entries_n = [0u64; 4];
    let mut this_o = [0u8; 0x80];
    let mut this_n = [0u8; 0x80];
    for (t, q, ent) in [
        (&mut this_o, &mut qo, &mut entries_o),
        (&mut this_n, &mut qn, &mut entries_n),
    ] {
        let tb = t.as_mut_ptr() as usize;
        wr::<usize>(tb + 0x48, q.as_mut_ptr() as usize);
        wr::<usize>(tb + 0x68, ent.as_mut_ptr() as usize);
        wr::<u32>(tb + 0x70, 0);
        wr::<u32>(tb + 0x74, 2);
    }

    let flag_addr = exe::ptr::<u8>(0x1_4385_8078) as usize;
    let g_sync_addr = exe::ptr::<usize>(0x1_43a3_6228) as usize;
    let saved_flag = rd::<u8>(flag_addr);
    let saved_sync = rd::<usize>(g_sync_addr);

    let mut sync_o = [0x9999usize, 10usize];
    let mut sync_n = sync_o;
    let mut res_o: usize = 0x5555_6666;
    let mut res_n: usize = 0x5555_6666;
    let coord = [11u32, 22, 33, 44];
    let region = [55u32, 66, 77, 88];

    wr::<u8>(flag_addr, 1);
    wr::<usize>(g_sync_addr, sync_o.as_mut_ptr() as usize);
    exe::call_va!(
        orig,
        (
            this_o.as_mut_ptr() as usize,
            (&mut res_o as *mut usize) as usize,
            coord.as_ptr() as usize,
            region.as_ptr() as usize,
            99u64
        ) -> ()
    );

    wr::<usize>(g_sync_addr, sync_n.as_mut_ptr() as usize);
    exe::call_va!(
        new,
        (
            this_n.as_mut_ptr() as usize,
            (&mut res_n as *mut usize) as usize,
            coord.as_ptr() as usize,
            region.as_ptr() as usize,
            99u64
        ) -> ()
    );

    wr::<u8>(flag_addr, saved_flag);
    wr::<usize>(g_sync_addr, saved_sync);

    crate::diff_eq!(res_o, res_n, "res_slot cleared");
    crate::diff_eq!(entries_o, entries_n, "tracked entry");
    crate::diff_eq!(sync_o, sync_n, "sync counter");
    crate::diff_eq!(&qo[0x8..], &qn[0x8..], "queue calls");
    crate::diff_eq!(&this_o[0x70..0x78], &this_n[0x70..0x78], "vec len");
    Ok(1)
}

unsafe extern "win64" fn mock_142088e90(this: usize, res: usize, coord: usize) {
    wr::<u64>(this, (res as u64) ^ (rd::<u32>(coord) as u64));
}

pub unsafe fn diff_d3d_map_tile_range_to_heap_1420898f0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4208_8e90, mock_142088e90 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4208_b990, mock_14208b990 as *const ())?;

    let mut vt = [0usize; 16];
    vt[0x40 / 8] = mock_queue_update_tile_mappings as *const () as usize;
    vt[0x70 / 8] = mock_queue_signal as *const () as usize;

    let mut qo = [0u8; 0x50];
    let mut qn = [0u8; 0x50];
    wr::<usize>(qo.as_mut_ptr() as usize, vt.as_ptr() as usize);
    wr::<usize>(qn.as_mut_ptr() as usize, vt.as_ptr() as usize);

    let heap_arr = [0usize, 0, 0x1111, 0, 0, 0x7777_8888];
    let mut lock_obj = [0usize, heap_arr.as_ptr() as usize];
    let mut entries_o = [0u64; 4];
    let mut entries_n = [0u64; 4];
    let mut this_o = [0u8; 0xf0];
    let mut this_n = [0u8; 0xf0];
    for (t, q, ent) in [
        (&mut this_o, &mut qo, &mut entries_o),
        (&mut this_n, &mut qn, &mut entries_n),
    ] {
        let tb = t.as_mut_ptr() as usize;
        wr::<usize>(tb + 0x48, q.as_mut_ptr() as usize);
        wr::<usize>(tb + 0x68, ent.as_mut_ptr() as usize);
        wr::<u32>(tb + 0x70, 0);
        wr::<u32>(tb + 0x74, 2);
        wr::<usize>(tb + 0xe0, lock_obj.as_mut_ptr() as usize);
    }

    let mut req_o = [0u8; 0x38];
    let rp = req_o.as_mut_ptr() as usize;
    wr::<usize>(rp, 0x3333_4444);
    wr::<u16>(rp + 0x8, 1);
    wr::<u32>(rp + 0xc, 12);
    wr::<u32>(rp + 0x1c, 34);
    wr::<u32>(rp + 0x2c, 56);
    wr::<u32>(rp + 0x30, 78);
    let mut req_n = req_o;

    let flag_addr = exe::ptr::<u8>(0x1_4385_8078) as usize;
    let saved_flag = rd::<u8>(flag_addr);
    wr::<u8>(flag_addr, 0);

    exe::call_va!(orig, (this_o.as_mut_ptr() as usize, req_o.as_mut_ptr() as usize, 77u64) -> ());
    exe::call_va!(new, (this_n.as_mut_ptr() as usize, req_n.as_mut_ptr() as usize, 77u64) -> ());
    wr::<u8>(flag_addr, saved_flag);

    crate::diff_eq!(req_o, req_n, "req resource moved");
    crate::diff_eq!(entries_o, entries_n, "tracked entry");
    crate::diff_eq!(&qo[0x8..], &qn[0x8..], "queue update args");
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x48, 0);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x48, 0);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x68, 0);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x68, 0);
    crate::diff_eq!(this_o, this_n, "this state");
    Ok(1)
}

unsafe extern "win64" fn mock_14240d480(sched: usize, this: usize) {
    wr::<u32>(sched, rd::<u32>(sched).wrapping_add(1));
    wr::<u32>(this + 0x8, 0x11);
}

unsafe extern "win64" fn mock_142085580(field: usize) {
    wr::<u32>(field, 0x22);
}

unsafe extern "win64" fn mock_1411e0350(field: usize) {
    wr::<u32>(field, 0x33);
}

unsafe extern "win64" fn mock_1420851b0(field: usize) {
    wr::<u32>(field, 0x44);
}

pub unsafe fn diff_d3d_async_streamer_dtor_14208a870(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4240_d480, mock_14240d480 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4208_5580, mock_142085580 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411e_0350, mock_1411e0350 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4208_51b0, mock_1420851b0 as *const ())?;

    let mut vt_fence = [0usize; 10];
    vt_fence[0x10 / 8] = mock_com_addref_or_release as *const () as usize;
    vt_fence[0x40 / 8] = mock_fence_get_completed as *const () as usize;

    let mut vt_com = [0usize; 4];
    vt_com[0x10 / 8] = mock_com_addref_or_release as *const () as usize;

    let mut fence_o = [vt_fence.as_ptr() as usize, 5usize];
    let mut fence_n = fence_o;
    let mut com_o = [[vt_com.as_ptr() as usize, 0usize]; 4];
    let mut com_n = com_o;

    let mut sched_o = [0u32; 4];
    let mut sched_n = [0u32; 4];
    let mut this_o = [0u8; 0xe0];
    let mut this_n = [0u8; 0xe0];
    for (t, f, coms) in [
        (&mut this_o, &mut fence_o, &mut com_o),
        (&mut this_n, &mut fence_n, &mut com_n),
    ] {
        let tb = t.as_mut_ptr() as usize;
        wr::<usize>(tb + 0x48, coms[0].as_mut_ptr() as usize);
        wr::<usize>(tb + 0x50, coms[1].as_mut_ptr() as usize);
        wr::<usize>(tb + 0x58, coms[2].as_mut_ptr() as usize);
        wr::<usize>(tb + 0x60, coms[3].as_mut_ptr() as usize);
        wr::<usize>(tb + 0xd0, f.as_mut_ptr() as usize);
    }

    let sched_addr = exe::ptr::<usize>(0x1_43a0_8298) as usize;
    let sync_addr = exe::ptr::<usize>(0x1_43a3_6228) as usize;
    let saved_sched = rd::<usize>(sched_addr);
    let saved_sync = rd::<usize>(sync_addr);

    wr::<usize>(sync_addr, 0);
    wr::<usize>(sched_addr, sched_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> ());

    wr::<usize>(sync_addr, 0);
    wr::<usize>(sched_addr, sched_n.as_mut_ptr() as usize);
    exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> ());

    wr::<usize>(sched_addr, saved_sched);
    wr::<usize>(sync_addr, saved_sync);

    crate::diff_eq!(this_o, this_n, "dtor state");
    crate::diff_eq!(fence_o[1], fence_n[1], "fence released");
    crate::diff_eq!(com_o, com_n, "com objects released");
    crate::diff_eq!(sched_o, sched_n, "sched unregistered");
    Ok(1)
}

unsafe extern "win64" fn mock_142087760(
    _rcx: usize,
    sub_ptr_addr: usize,
    com_obj: usize,
    req: usize,
) {
    let sub = rd::<usize>(sub_ptr_addr);
    wr::<usize>(sub, com_obj ^ rd::<usize>(req));
}

unsafe extern "win64" fn mock_142087920(
    _rcx: usize,
    sub_ptr_addr: usize,
    pair_ptr: usize,
    req: usize,
) {
    let sub = rd::<usize>(sub_ptr_addr);
    wr::<u64>(sub, rd::<u64>(pair_ptr) ^ rd::<u64>(pair_ptr + 8) ^ rd::<u64>(req));
}

unsafe extern "win64" fn mock_queue_wait(queue: usize, fence: usize, seq: u64) -> i32 {
    wr::<usize>(queue + 0x8, fence);
    wr::<u64>(queue + 0x10, seq);
    0
}

unsafe fn run_submit_stream_difftest(orig: usize, new: usize, is_pair: bool) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4208_7760, mock_142087760 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4208_7920, mock_142087920 as *const ())?;

    let mut q_vt = [0usize; 16];
    q_vt[0x78 / 8] = mock_queue_wait as *const () as usize;
    let mut qo = [q_vt.as_ptr() as usize, 0, 0];
    let mut qn = qo;
    let q_wrap_o = [0usize, 0, qo.as_mut_ptr() as usize];
    let q_wrap_n = [0usize, 0, qn.as_mut_ptr() as usize];

    let mut ring_o = [0u8; 0xc0];
    let mut ring_n = [0u8; 0xc0];
    let mut inner_o = [0u8; 0x110];
    let mut inner_n = [0u8; 0x110];
    for (inn, ring) in [(&mut inner_o, &mut ring_o), (&mut inner_n, &mut ring_n)] {
        let ib = inn.as_mut_ptr() as usize;
        wr::<usize>(ib + 0x40, ring.as_mut_ptr() as usize);
        wr::<u32>(ib + 0x54, 2);
        wr::<u64>(ib + 0xf8, 55);
        wr::<usize>(ib + 0x100, 0x8888);
    }
    let this_o = [0usize, 0, 0, inner_o.as_mut_ptr() as usize];
    let this_n = [0usize, 0, 0, inner_n.as_mut_ptr() as usize];

    let mut g_o = vec![0u8; 0x260];
    let mut g_n = vec![0u8; 0x260];
    let pair_val = [0x1111_2222u64, 0x3333_4444u64];
    let req_val = [0x5555_6666u64; 2];

    let q_addr = exe::ptr::<usize>(0x1_4397_fc60) as usize;
    let g_addr = exe::ptr::<usize>(0x1_43a0_8298) as usize;
    let saved_q = rd::<usize>(q_addr);
    let saved_g = rd::<usize>(g_addr);

    let arg2 = if is_pair {
        pair_val.as_ptr() as usize
    } else {
        0x1234_5678usize
    };

    wr::<usize>(q_addr, q_wrap_o.as_ptr() as usize);
    wr::<usize>(g_addr, g_o.as_mut_ptr() as usize);
    exe::call_va!(
        orig,
        (this_o.as_ptr() as usize, arg2, req_val.as_ptr() as usize) -> ()
    );

    wr::<usize>(q_addr, q_wrap_n.as_ptr() as usize);
    wr::<usize>(g_addr, g_n.as_mut_ptr() as usize);
    exe::call_va!(
        new,
        (this_n.as_ptr() as usize, arg2, req_val.as_ptr() as usize) -> ()
    );

    wr::<usize>(q_addr, saved_q);
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(
        rd::<u64>(inner_o.as_ptr() as usize + 0x30),
        rd::<u64>(inner_n.as_ptr() as usize + 0x30),
        "enqueue sub_ptr result"
    );
    crate::diff_eq!(&inner_o[0x4c..], &inner_n[0x4c..], "inner ring and seq");
    crate::diff_eq!(
        rd::<u8>(ring_o.as_ptr() as usize + 0x50),
        rd::<u8>(ring_n.as_ptr() as usize + 0x50),
        "ring cmd tag"
    );
    crate::diff_eq!(
        rd::<u64>(ring_o.as_ptr() as usize + 0x58),
        rd::<u64>(ring_n.as_ptr() as usize + 0x58),
        "ring cmd seq"
    );
    crate::diff_eq!(&qo[1..], &qn[1..], "queue wait args");
    crate::diff_eq!(
        rd::<u32>(g_o.as_ptr() as usize + 0x258),
        rd::<u32>(g_n.as_ptr() as usize + 0x258),
        "g wake counter"
    );
    Ok(1)
}

pub unsafe fn diff_d3d_submit_stream_buffer_and_signal_14208bc90(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    run_submit_stream_difftest(orig, new, false)
}

pub unsafe fn diff_d3d_submit_stream_pair_and_signal_14208bea0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    run_submit_stream_difftest(orig, new, true)
}

unsafe extern "win64" fn mock_res_unmap(res: usize, subres: u32, range_ptr: usize) {
    wr::<u32>(res + 0x8, subres);
    wr::<u64>(res + 0x10, rd::<u64>(range_ptr));
    wr::<u64>(res + 0x18, rd::<u64>(range_ptr + 0x8));
}

unsafe extern "win64" fn mock_copy_142075bc0(
    dst_res: usize,
    dst_off: u64,
    _blk: usize,
    off: u64,
    sz: u64,
) {
    wr::<u64>(dst_res, dst_off);
    wr::<u64>(dst_res + 0x8, off);
    wr::<u64>(dst_res + 0x10, sz);
}

unsafe extern "win64" fn mock_free_pool_14243b3e0(pool: usize, _node: usize) {
    wr::<u32>(pool, rd::<u32>(pool).wrapping_add(1));
}

pub unsafe fn diff_d3d_unmap_and_release_upload_chunk_14208ca50(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4207_5bc0, mock_copy_142075bc0 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4243_b3e0, mock_free_pool_14243b3e0 as *const ())?;

    let mut vt = [0usize; 12];
    vt[0x48 / 8] = mock_res_unmap as *const () as usize;
    let mut res_o = [0u64; 4];
    let mut res_n = [0u64; 4];
    wr::<usize>(res_o.as_mut_ptr() as usize, vt.as_ptr() as usize);
    wr::<usize>(res_n.as_mut_ptr() as usize, vt.as_ptr() as usize);

    let mut blk_o = [0u8; 0x60];
    let mut blk_n = [0u8; 0x60];
    wr::<usize>(blk_o.as_mut_ptr() as usize + 0x40, res_o.as_mut_ptr() as usize);
    wr::<u64>(blk_o.as_mut_ptr() as usize + 0x50, 2);
    wr::<usize>(blk_n.as_mut_ptr() as usize + 0x40, res_n.as_mut_ptr() as usize);
    wr::<u64>(blk_n.as_mut_ptr() as usize + 0x50, 2);

    let mut chunk_o = [blk_o.as_mut_ptr() as u64, 0x100, 0x400];
    let mut chunk_n = [blk_n.as_mut_ptr() as u64, 0x100, 0x400];
    let mut dst_o = [10u64, 0x200, 0, 0, 0];
    let mut dst_n = dst_o;
    let mut g_o = [0u8; 0x150];
    let mut g_n = [0u8; 0x150];
    let mut this_o = [0u8; 0x68];
    let mut this_n = [0u8; 0x68];
    for (t, dst, chunk) in [
        (&mut this_o, &mut dst_o, &mut chunk_o),
        (&mut this_n, &mut dst_n, &mut chunk_n),
    ] {
        let tb = t.as_mut_ptr() as usize;
        wr::<u64>(tb + 0x40, 0x50);
        wr::<usize>(tb + 0x50, dst.as_mut_ptr() as usize);
        wr::<usize>(tb + 0x58, chunk.as_mut_ptr() as usize);
    }

    let g_addr = exe::ptr::<usize>(0x1_4397_fb88) as usize;
    let saved_g = rd::<usize>(g_addr);

    wr::<usize>(g_addr, g_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> ());

    wr::<usize>(g_addr, g_n.as_mut_ptr() as usize);
    exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> ());
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(&res_o[1..], &res_n[1..], "unmap range");
    crate::diff_eq!(dst_o, dst_n, "dst copy and atomic inc");
    crate::diff_eq!(
        rd::<u64>(blk_o.as_ptr() as usize + 0x50),
        rd::<u64>(blk_n.as_ptr() as usize + 0x50),
        "blk refcount"
    );
    crate::diff_eq!(
        rd::<u32>(g_o.as_ptr() as usize + 0xb0),
        rd::<u32>(g_n.as_ptr() as usize + 0xb0),
        "chunk freed to pool"
    );
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x50, 0);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x50, 0);
    crate::diff_eq!(this_o, this_n, "this chunk ptr cleared");
    Ok(1)
}

unsafe extern "win64" fn mock_res_map(
    res: usize,
    subres: u32,
    range_ptr: usize,
    out_ptr: usize,
) -> i32 {
    wr::<u32>(res + 0x8, subres);
    wr::<u64>(res + 0x10, rd::<u64>(range_ptr) ^ rd::<u64>(range_ptr + 0x8));
    wr::<usize>(out_ptr, 0x5000_0000);
    0
}

pub unsafe fn diff_d3d_alloc_and_map_upload_chunk_14208cb80(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt = [0usize; 10];
    vt[0x40 / 8] = mock_res_map as *const () as usize;
    let mut res_o = [0u64; 3];
    let mut res_n = [0u64; 3];
    wr::<usize>(res_o.as_mut_ptr() as usize, vt.as_ptr() as usize);
    wr::<usize>(res_n.as_mut_ptr() as usize, vt.as_ptr() as usize);

    let mut blk_o = [0u8; 0x70];
    let mut blk_n = [0u8; 0x70];
    let mut chunk_o = [0u64; 3];
    let mut chunk_n = [0u64; 3];
    let mut g_o = [0u8; 0x150];
    let mut g_n = [0u8; 0x150];
    let mut this_o = [0u8; 0x68];
    let mut this_n = [0u8; 0x68];

    for (t, g, chunk, blk, res) in [
        (&mut this_o, &mut g_o, &mut chunk_o, &mut blk_o, &mut res_o),
        (&mut this_n, &mut g_n, &mut chunk_n, &mut blk_n, &mut res_n),
    ] {
        let tb = t.as_mut_ptr() as usize;
        let gb = g.as_mut_ptr() as usize;
        let bb = blk.as_mut_ptr() as usize;
        wr::<u64>(tb + 0x10, 0x400);
        wr::<usize>(gb + 0xb0 + 0x18, chunk.as_mut_ptr() as usize);
        wr::<usize>(bb + 0x40, res.as_mut_ptr() as usize);
        wr::<u64>(bb + 0x50, 1);
        wr::<u64>(bb + 0x58, 0x200);
        wr::<u64>(bb + 0x60, 0x10000);
        wr::<usize>(gb + 0x138, bb);
    }

    let g_addr = exe::ptr::<usize>(0x1_4397_fb88) as usize;
    let saved_g = rd::<usize>(g_addr);

    wr::<usize>(g_addr, g_o.as_mut_ptr() as usize);
    let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> usize);

    wr::<usize>(g_addr, g_n.as_mut_ptr() as usize);
    let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> usize);
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(ro, rn, "mapped ptr + offset");
    crate::diff_eq!(chunk_o[1..3], chunk_n[1..3], "chunk offset and size");
    crate::diff_eq!(&blk_o[0x50..0x68], &blk_n[0x50..0x68], "blk suballoc state");
    crate::diff_eq!(&res_o[1..], &res_n[1..], "res map args");
    Ok(1)
}

// REGISTRY:
// FLOAT_RETURN: none (no function in this module returns f32/f64 in xmm0).
//
// Replacement::verified_throws(0x1_4207_6db0, "d3d::d3d_flush_deferred_releases_locked_142076db0", rend::d3d_rem_b::d3d_flush_deferred_releases_locked_142076db0 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_flush_deferred_releases_locked_142076db0),
// Replacement::verified_throws(0x1_4207_6f00, "d3d::d3d_get_queue_timestamp_frequency_142076f00", rend::d3d_rem_b::d3d_get_queue_timestamp_frequency_142076f00 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_get_queue_timestamp_frequency_142076f00),
// Replacement::verified_throws(0x1_4207_6f60, "d3d::d3d_enqueue_deferred_releases_142076f60", rend::d3d_rem_b::d3d_enqueue_deferred_releases_142076f60 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_enqueue_deferred_releases_142076f60),
// Replacement::verified_throws(0x1_4207_70a0, "d3d::d3d_init_cmd_list_with_ext_interfaces_1420770a0", rend::d3d_rem_b::d3d_init_cmd_list_with_ext_interfaces_1420770a0 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_init_cmd_list_with_ext_interfaces_1420770a0),
// Replacement::verified_throws(0x1_4207_7130, "d3d::d3d_fence_is_completed_142077130", rend::d3d_rem_b::d3d_fence_is_completed_142077130 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_fence_is_completed_142077130),
// Replacement::verified_throws(0x1_4207_71d0, "d3d::d3d_init_cmd_alloc_and_fence_1420771d0", rend::d3d_rem_b::d3d_init_cmd_alloc_and_fence_1420771d0 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_init_cmd_alloc_and_fence_1420771d0),
// Replacement::verified_throws(0x1_4207_72c0, "d3d::d3d_execute_command_lists_and_signal_1420772c0", rend::d3d_rem_b::d3d_execute_command_lists_and_signal_1420772c0 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_execute_command_lists_and_signal_1420772c0),
// Replacement::verified_throws(0x1_4207_73c0, "d3d::d3d_signal_global_fence_on_queue_1420773c0", rend::d3d_rem_b::d3d_signal_global_fence_on_queue_1420773c0 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_signal_global_fence_on_queue_1420773c0),
// Replacement::verified(0x1_4207_7690, "d3d::d3d_build_blend_state_desc_142077690", rend::d3d_rem_b::d3d_build_blend_state_desc_142077690 as *const (), rend::d3d_rem_b::diff_d3d_build_blend_state_desc_142077690),
// Replacement::verified_throws(0x1_4207_9d80, "d3d::d3d_create_root_signature_from_spans_142079d80", rend::d3d_rem_b::d3d_create_root_signature_from_spans_142079d80 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_create_root_signature_from_spans_142079d80),
// Replacement::verified_throws(0x1_4207_a540, "d3d::d3d_advance_swapchain_frame_14207a540", rend::d3d_rem_b::d3d_advance_swapchain_frame_14207a540 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_advance_swapchain_frame_14207a540),
// Replacement::verified_throws(0x1_4207_be30, "d3d::d3d_resize_swapchain_buffers_14207be30", rend::d3d_rem_b::d3d_resize_swapchain_buffers_14207be30 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_resize_swapchain_buffers_14207be30),
// Replacement::verified_throws(0x1_4207_e500, "d3d::d3d_release_mip_level_slot_14207e500", rend::d3d_rem_b::d3d_release_mip_level_slot_14207e500 as *const (), Throws::Caught, rend::d3d_rem_b::diff_d3d_release_mip_level_slot_14207e500),
// Replacement::verified_throws(0x1_4207_fec0, "d3d::d3d_reinit_streaming_texture_14207fec0", rend::d3d_rem_b::d3d_reinit_streaming_texture_14207fec0 as *const (), Throws::Caught, rend::d3d_rem_b::diff_d3d_reinit_streaming_texture_14207fec0),
// Replacement::verified(0x1_4208_04b0, "d3d::d3d_is_small_texture_mip_chain_1420804b0", rend::d3d_rem_b::d3d_is_small_texture_mip_chain_1420804b0 as *const (), rend::d3d_rem_b::diff_d3d_is_small_texture_mip_chain_1420804b0),
// Replacement::verified(0x1_4208_2a40, "d3d::d3d_stream_cmd_move_construct_142082a40", rend::d3d_rem_b::d3d_stream_cmd_move_construct_142082a40 as *const (), rend::d3d_rem_b::diff_d3d_stream_cmd_move_construct_142082a40),
// Replacement::verified(0x1_4208_4000, "d3d::d3d_stream_cmd_move_with_seq_142084000", rend::d3d_rem_b::d3d_stream_cmd_move_with_seq_142084000 as *const (), rend::d3d_rem_b::diff_d3d_stream_cmd_move_with_seq_142084000),
// Replacement::verified(0x1_4208_4af0, "d3d::d3d_destroy_com_ring_buffer_142084af0", rend::d3d_rem_b::d3d_destroy_com_ring_buffer_142084af0 as *const (), rend::d3d_rem_b::diff_d3d_destroy_com_ring_buffer_142084af0),
// Replacement::verified(0x1_4208_5c40, "d3d::d3d_free_tile_pool_id_142085c40", rend::d3d_rem_b::d3d_free_tile_pool_id_142085c40 as *const (), rend::d3d_rem_b::diff_d3d_free_tile_pool_id_142085c40),
// Replacement::verified(0x1_4208_5dc0, "d3d::d3d_alloc_tile_pool_id_142085dc0", rend::d3d_rem_b::d3d_alloc_tile_pool_id_142085dc0 as *const (), rend::d3d_rem_b::diff_d3d_alloc_tile_pool_id_142085dc0),
// Replacement::verified(0x1_4208_7760, "d3d::d3d_enqueue_stream_buffer_cmd_142087760", rend::d3d_rem_b::d3d_enqueue_stream_buffer_cmd_142087760 as *const (), rend::d3d_rem_b::diff_d3d_enqueue_stream_buffer_cmd_142087760),
// Replacement::verified(0x1_4208_86c0, "d3d::d3d_enqueue_stream_callback_cmd_1420886c0", rend::d3d_rem_b::d3d_enqueue_stream_callback_cmd_1420886c0 as *const (), rend::d3d_rem_b::diff_d3d_enqueue_stream_callback_cmd_1420886c0),
// Replacement::verified_throws(0x1_4208_8af0, "d3d::d3d_update_tile_mappings_and_track_142088af0", rend::d3d_rem_b::d3d_update_tile_mappings_and_track_142088af0 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_update_tile_mappings_and_track_142088af0),
// Replacement::verified_throws(0x1_4208_9770, "d3d::d3d_dispatch_tile_mapping_update_142089770", rend::d3d_rem_b::d3d_dispatch_tile_mapping_update_142089770 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_dispatch_tile_mapping_update_142089770),
// Replacement::verified_throws(0x1_4208_98f0, "d3d::d3d_map_tile_range_to_heap_1420898f0", rend::d3d_rem_b::d3d_map_tile_range_to_heap_1420898f0 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_map_tile_range_to_heap_1420898f0),
// Replacement::verified_throws(0x1_4208_9a70, "d3d::d3d_retire_completed_tile_cmds_142089a70", rend::d3d_rem_b::d3d_retire_completed_tile_cmds_142089a70 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_retire_completed_tile_cmds_142089a70),
// Replacement::verified_throws(0x1_4208_a320, "d3d::d3d_push_stream_callback_and_store_14208a320", rend::d3d_rem_b::d3d_push_stream_callback_and_store_14208a320 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_push_stream_callback_and_store_14208a320),
// Replacement::verified_throws(0x1_4208_a870, "d3d::d3d_async_streamer_dtor_14208a870", rend::d3d_rem_b::d3d_async_streamer_dtor_14208a870 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_async_streamer_dtor_14208a870),
// Replacement::verified(0x1_4208_b670, "d3d::d3d_hash_tile_key_14208b670", rend::d3d_rem_b::d3d_hash_tile_key_14208b670 as *const (), rend::d3d_rem_b::diff_d3d_hash_tile_key_14208b670),
// Replacement::verified_throws(0x1_4208_bc90, "d3d::d3d_submit_stream_buffer_and_signal_14208bc90", rend::d3d_rem_b::d3d_submit_stream_buffer_and_signal_14208bc90 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_submit_stream_buffer_and_signal_14208bc90),
// Replacement::verified_throws(0x1_4208_bea0, "d3d::d3d_submit_stream_pair_and_signal_14208bea0", rend::d3d_rem_b::d3d_submit_stream_pair_and_signal_14208bea0 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_submit_stream_pair_and_signal_14208bea0),
// Replacement::verified(0x1_4208_ca50, "d3d::d3d_unmap_and_release_upload_chunk_14208ca50", rend::d3d_rem_b::d3d_unmap_and_release_upload_chunk_14208ca50 as *const (), rend::d3d_rem_b::diff_d3d_unmap_and_release_upload_chunk_14208ca50),
// Replacement::verified_throws(0x1_4208_cb80, "d3d::d3d_alloc_and_map_upload_chunk_14208cb80", rend::d3d_rem_b::d3d_alloc_and_map_upload_chunk_14208cb80 as *const (), Throws::Discard, rend::d3d_rem_b::diff_d3d_alloc_and_map_upload_chunk_14208cb80),
// Replacement::verified(0x1_4208_cd70, "d3d::d3d_find_adapter_by_luid_14208cd70", rend::d3d_rem_b::d3d_find_adapter_by_luid_14208cd70 as *const (), rend::d3d_rem_b::diff_d3d_find_adapter_by_luid_14208cd70),
// Replacement::verified(0x1_4208_de00, "d3d::d3d_init_default_pass_entry_14208de00", rend::d3d_rem_b::d3d_init_default_pass_entry_14208de00 as *const (), rend::d3d_rem_b::diff_d3d_init_default_pass_entry_14208de00),

