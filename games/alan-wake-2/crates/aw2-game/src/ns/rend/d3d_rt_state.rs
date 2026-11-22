use crate::exe::{self, rd, wr};

#[inline]
unsafe fn release_cmd_guard(guard: usize) {
    if guard != 0 {
        let next = rd::<i32>(guard + 0xc).wrapping_sub(1);
        wr::<i32>(guard + 0xc, next);
        if next == 0 {
            wr::<u32>(guard + 0x8, 0xffff_ffff);
            let _ = exe::call_iat!(0x1_4264_5598, (guard) -> usize);
        }
    }
}

/// Original: `0x14206b5f0` (74 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_rt_preset_by_ptr(rcx: usize) -> usize {
    let idx = rd::<i32>(rcx) as isize as usize;
    let table = exe::ptr::<u8>(0x1_43a6_32f0) as usize;
    let entry = table.wrapping_add(idx.wrapping_shl(6));
    let arg6 = rd::<usize>(entry + 0x30);
    let r9d = rd::<u32>(entry + 0x20);
    let r8 = rd::<usize>(entry + 0x18);
    let arg5 = rd::<u32>(entry + 0x28) as usize;
    let arg4 = rd::<u32>(entry + 0x24) as usize;
    let rdx = rd::<usize>(entry + 0x10);
    exe::call_va!(0x1_4206_c8b0, (idx, rdx, r8, r9d, arg4, arg5, arg6) -> usize)
}

/// Original: `0x14206be00` (28 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_rt_pool_entry_28(rcx: i32, rdx: i32) -> usize {
    let eax = rdx.wrapping_add(rcx.wrapping_mul(2));
    let ecx = rcx.wrapping_add(eax);
    let idx = ecx as isize as usize;
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let table = rd::<usize>(mgr + 0xd8);
    rd::<usize>(table.wrapping_add(idx.wrapping_mul(8)).wrapping_add(0x28))
}

/// Original: `0x14206be30` (23 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_rt_pool_entry_10(rcx: i32) -> usize {
    let idx = rcx as isize as usize;
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let table = rd::<usize>(mgr + 0xd8);
    rd::<usize>(table.wrapping_add(idx.wrapping_mul(8)).wrapping_add(0x10))
}

/// Original: `0x14206be50` (19 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_rt_pool_head() -> usize {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let table = rd::<usize>(mgr + 0xd8);
    rd::<usize>(table + 0x8)
}

/// Original: `0x14206be70` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_reset_rt_config_default() -> usize {
    exe::call_va!(0x1_4206_bea0, (0usize, 0usize, 0usize, 0usize, 0usize) -> usize)
}

/// Original: `0x14206c810` (71 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_rt_preset_by_idx(rcx: i32) -> usize {
    let table = exe::ptr::<u8>(0x1_43a6_32f0) as usize;
    let idx = rcx as isize as usize;
    let entry = table.wrapping_add(idx.wrapping_shl(6));
    let arg6 = rd::<usize>(entry + 0x30);
    let r9d = rd::<u32>(entry + 0x20);
    let r8 = rd::<usize>(entry + 0x18);
    let arg5 = rd::<u32>(entry + 0x28) as usize;
    let arg4 = rd::<u32>(entry + 0x24) as usize;
    let rdx = rd::<usize>(entry + 0x10);
    exe::call_va!(0x1_4206_c8b0, (rcx as u32 as usize, rdx, r8, r9d, arg4, arg5, arg6) -> usize)
}

/// Original: `0x14206c860` (37 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_splat_rt_desc_3x(rcx: usize, rdx: usize) -> usize {
    let v = rd::<[u8; 32]>(rdx);
    wr::<[u8; 32]>(rcx + 0x20, v);
    wr::<[u8; 32]>(rcx + 0x60, v);
    wr::<[u8; 32]>(rcx + 0xa0, v);
    rcx
}

/// Original: `0x14206c890` (16 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_copy_rt_desc_slot0(rcx: usize, rdx: usize) -> usize {
    let v = rd::<[u8; 32]>(rdx);
    wr::<[u8; 32]>(rcx + 0x20, v);
    rcx
}

/// Original: `0x14206d140` (33 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_extract_rt_handle_pair(rcx: usize, rdx: usize) -> usize {
    wr::<u64>(rcx, 0);
    wr::<u64>(rcx + 0x8, 0);
    if rdx != 0 {
        let a = rd::<u64>(rdx + 0x8);
        wr::<u64>(rcx + 0x8, a);
        let b = rd::<u64>(rdx + 0x10);
        wr::<u64>(rcx, b);
    }
    rcx
}

/// Original: `0x14206d2e0` (162 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_rt_slot_debug_name(rcx: i32, rdx: usize) {
    let idx = rcx as isize as usize;
    let empty_str = exe::ptr::<u8>(0x1_426c_dcb0) as usize;
    let src = if rdx != 0 { rdx } else { empty_str };
    let mut len: usize = 0;
    while rd::<u8>(src + len) != 0 {
        len += 1;
    }
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let r14 = mgr.wrapping_add(idx.wrapping_mul(9).wrapping_shl(5));
    let cur_len = rd::<u32>(r14 + 0x1000);
    let len_u32 = len as u32;
    let req_len = if (len_u32 as usize) >= (cur_len as usize) {
        len_u32
    } else {
        cur_len
    };
    exe::call_va!(0x1_423e_f2b0, (r14 + 0xff0, req_len) -> ());
    let cap = rd::<u32>(r14 + 0xffc) as usize;
    let dst = rd::<usize>(r14 + 0xff0);
    let mut out_status: u64 = 0;
    let status_ptr = core::ptr::addr_of_mut!(out_status) as usize;
    let _ = exe::call_iat!(0x1_4264_6508, (status_ptr, dst, cap, src, len_u32 as usize) -> usize);
    wr::<u32>(r14 + 0x1000, len_u32);
}

/// Original: `0x14206d390` (147 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_mgr_debug_name(rcx: usize) {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let empty_str = exe::ptr::<u8>(0x1_426c_dcb0) as usize;
    let src = if rcx != 0 { rcx } else { empty_str };
    let mut len: usize = 0;
    while rd::<u8>(src + len) != 0 {
        len += 1;
    }
    let cur_len = rd::<u32>(mgr + 0xee0);
    let len_u32 = len as u32;
    let req_len = if (len_u32 as usize) >= (cur_len as usize) {
        len_u32
    } else {
        cur_len
    };
    exe::call_va!(0x1_423e_f2b0, (mgr + 0xed0, req_len) -> ());
    let cap = rd::<u32>(mgr + 0xedc) as usize;
    let dst = rd::<usize>(mgr + 0xed0);
    let mut out_status: u64 = 0;
    let status_ptr = core::ptr::addr_of_mut!(out_status) as usize;
    let _ = exe::call_iat!(0x1_4264_6508, (status_ptr, dst, cap, src, len_u32 as usize) -> usize);
    wr::<u32>(mgr + 0xee0, len_u32);
}

/// Original: `0x14206d430` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_bind_rt_and_notify(rcx: usize, rdx: usize, r8b: u8) -> usize {
    let ctx = exe::call_va!(0x1_411d_cdf0, (rcx) -> usize);
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let flag = rd::<u8>(mgr + 0x9c) as u32;
    exe::call_va!(0x1_4209_64f0, (ctx, flag, r8b as u32) -> ());
    exe::call_va!(0x1_4209_ba00, (rdx) -> usize)
}

/// Original: `0x14206d480` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_mgr_active_ctx(rcx: usize) {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    wr::<usize>(mgr + 0xd0, rcx);
}

/// Original: `0x14206d4a0` (108 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_active_ctx_and_bind(rcx: usize, rdx: usize, r8b: u8) -> usize {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    wr::<usize>(mgr + 0xd0, rcx);
    let ctx = exe::call_va!(0x1_411d_cdf0, (rcx) -> usize);
    let mgr2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let flag = rd::<u8>(mgr2 + 0x9c) as u32;
    exe::call_va!(0x1_4209_64f0, (ctx, flag, r8b as u32) -> ());
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, (tls0, 0x70usize) -> ());
    }
    exe::call_va!(0x1_4209_ba00, (rdx) -> usize)
}

/// Original: `0x14206e210` (169 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_apply_predication_and_heaps(rcx: usize) {
    let mut cmd_pair = [0usize; 2];
    let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
    exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
    let cmd_list = rd::<usize>(pair_ptr + 0x8);
    let pred = rd::<usize>(rcx + 0xa68);
    if pred != 0 {
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0xe8);
        exe::call_va!(fn_ptr, (cmd_list, pred) -> ());
        let ctx = exe::call_va!(0x1_411d_cdf0, (cmd_list) -> usize);
        exe::call_va!(0x1_4209_5ba0, (ctx) -> ());
    }
    let h0 = rd::<usize>(rcx + 0xa70);
    if h0 != 0 {
        let h1 = rd::<usize>(rcx + 0xa78);
        if h1 != 0 {
            let mut heaps = [0usize; 2];
            let heaps_ptr = core::ptr::addr_of_mut!(heaps) as usize;
            wr::<usize>(heaps_ptr, h0);
            wr::<usize>(heaps_ptr + 0x8, h1);
            let vtbl = rd::<usize>(cmd_list);
            let fn_ptr = rd::<usize>(vtbl + 0xe0);
            exe::call_va!(fn_ptr, (cmd_list, 2u32, heaps_ptr) -> ());
        }
    }
    release_cmd_guard(rd::<usize>(pair_ptr));
}

/// Original: `0x14206e2c0` (180 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_invalidate_bound_ia_state(rcx: usize, rdx: usize) {
    let mut eax = rd::<u32>(rdx);
    if eax == 0 {
        return;
    }
    let mut has_ib = false;
    let mut has_vb = false;
    let mut idx: usize = 0;
    loop {
        let diff = eax.wrapping_sub(5);
        if diff == 0 {
            has_vb = true;
        } else if diff == 1 {
            has_ib = true;
        }
        idx += 1;
        eax = rd::<u32>(rdx + idx * 16);
        if eax == 0 {
            break;
        }
    }
    if has_ib && rd::<usize>(rcx + 0xc30) != 0 {
        let mut cmd_pair = [0usize; 2];
        let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
        exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
        let cmd_list = rd::<usize>(pair_ptr + 0x8);
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0x158);
        exe::call_va!(fn_ptr, (cmd_list, rcx + 0xc30) -> ());
        release_cmd_guard(rd::<usize>(pair_ptr));
    }
    if has_vb {
        wr::<u8>(rcx + 0xb2c, 1);
    }
}

/// Original: `0x14206e670` (83 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_rt_view_with_opt(
    rcx: usize,
    rdx: usize,
    r8: usize,
    r9: usize,
    arg5: usize,
) -> usize {
    exe::call_va!(
        0x1_4206_6c10,
        (r9, 0usize, 0usize, arg5, rdx, 0usize, 0usize, r8, 0usize) -> ()
    );
    wr::<u64>(rcx, 0);
    wr::<u64>(rcx + 0x8, 0);
    rcx
}

/// Original: `0x14206e6d0` (101 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_rt_view_default(
    rcx: usize,
    rdx: usize,
    r8: usize,
) -> usize {
    let mut opt_a = [0u32; 3];
    let mut opt_b = [0u32; 3];
    let ptr_a = core::ptr::addr_of_mut!(opt_a) as usize;
    let ptr_b = core::ptr::addr_of_mut!(opt_b) as usize;
    exe::call_va!(
        0x1_4206_6c10,
        (r8, 0usize, 0usize, ptr_b, rdx, 0usize, 0usize, ptr_a, 0usize) -> ()
    );
    wr::<u64>(rcx, 0);
    wr::<u64>(rcx + 0x8, 0);
    rcx
}

/// Original: `0x14206e9d0` (34 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_rt_binding_state(rcx: usize) -> usize {
    wr::<u64>(rcx, 0);
    wr::<u32>(rcx + 0x8, 0);
    wr::<u8>(rcx + 0xc, 0);
    wr::<u64>(rcx + 0x10, 0);
    wr::<u64>(rcx + 0x18, 0);
    wr::<u64>(rcx + 0x20, 0);
    wr::<u32>(rcx + 0x28, 0);
    wr::<u64>(rcx + 0x38, 0);
    rcx
}

/// Original: `0x14206ecc0` (221 bytes, `Throws::No`).
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn d3d_clear_depth_stencil_target(
    _rcx: usize,
    rdx: usize,
    depth: f32,
    stencil: u8,
    clear_mask: u32,
) {
    d3d_clear_depth_stencil_target_impl(_rcx, rdx, depth, stencil, clear_mask);
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn d3d_clear_depth_stencil_target(
    _rcx: usize,
    rdx: usize,
    _r8: usize,
    stencil: u8,
    clear_mask: u32,
    _xmm0: f32,
    _xmm1: f32,
    depth: f32,
) {
    d3d_clear_depth_stencil_target_impl(_rcx, rdx, depth, stencil, clear_mask);
}

#[inline(always)]
unsafe fn d3d_clear_depth_stencil_target_impl(
    _rcx: usize,
    rdx: usize,
    depth: f32,
    stencil: u8,
    clear_mask: u32,
) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, (tls0, 0x70usize) -> ());
    }
    if rd::<usize>(tls0 + 0x6300) == 0 {
        exe::call_va!(0x1_4206_7c90, (rdx) -> ());
    }
    if clear_mask != 0 {
        let base_flag = (clear_mask >> 1) & 1;
        let d3d_flags = if (clear_mask & 4) != 0 {
            base_flag | 2
        } else {
            base_flag
        };
        let mut cmd_pair = [0usize; 2];
        let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
        exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
        let cmd_list = rd::<usize>(pair_ptr + 0x8);
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0x178);
        let dsv_handle = rd::<usize>(rdx + 0x88);
        let _ = crate::simd_ops::call_va8_xmm(
            fn_ptr as u64,
            cmd_list as u64,
            dsv_handle as u64,
            d3d_flags as u64,
            0,
            stencil as u64,
            0,
            0,
            0,
            [0, 0, 0, depth.to_bits() as u128],
        );
        release_cmd_guard(rd::<usize>(pair_ptr));
    }
}

/// Original: `0x14206edb0` (182 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_render_target(
    _rcx: usize,
    rdx: usize,
    color_rgba: usize,
) {
    if rd::<usize>(rdx + 0x80) == 0 {
        return;
    }
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, (tls0, 0x70usize) -> ());
    }
    if rd::<usize>(tls0 + 0x6300) == 0 {
        exe::call_va!(0x1_4206_7c90, (rdx) -> ());
    }
    let mut cmd_pair = [0usize; 2];
    let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
    exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
    let cmd_list = rd::<usize>(pair_ptr + 0x8);
    let vtbl = rd::<usize>(cmd_list);
    let fn_ptr = rd::<usize>(vtbl + 0x180);
    let rtv_handle = rd::<usize>(rdx + 0x80);
    exe::call_va!(fn_ptr, (cmd_list, rtv_handle, color_rgba, 0u32, 0usize) -> ());
    release_cmd_guard(rd::<usize>(pair_ptr));
}

/// Original: `0x14206f290` (172 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_flush_vertex_buffers(rcx: usize) {
    if rd::<u8>(rcx + 0xb2c) == 0 {
        return;
    }
    let count = rd::<i32>(rcx + 0xb28);
    if count > 0 {
        let mut cmd_pair = [0usize; 2];
        let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
        exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
        let cmd_list = rd::<usize>(pair_ptr + 0x8);
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0x160);
        let cur_count = rd::<u32>(rcx + 0xb28);
        exe::call_va!(fn_ptr, (cmd_list, 0u32, cur_count, rcx + 0xb30) -> ());
        release_cmd_guard(rd::<usize>(pair_ptr));
    }
    let mut eax = rd::<i32>(rcx + 0xb28);
    while eax > 0 {
        let slot_off = ((eax as u32 as usize) + 0xb2) * 16;
        if rd::<usize>(rcx + slot_off) != 0 {
            break;
        }
        eax -= 1;
        wr::<i32>(rcx + 0xb28, eax);
    }
    wr::<u8>(rcx + 0xb2c, 0);
}

/// Original: `0x14206f350` (64 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_ensure_rt_query_slot(rcx: usize) -> u8 {
    let mut rax = rd::<usize>(rcx + 0x48);
    if rax == 0 {
        let sub_obj = rd::<usize>(rcx + 0xc48);
        if sub_obj != 0 {
            let p_idx = rd::<usize>(rcx + 0xb18);
            let edx = rd::<u32>(p_idx).wrapping_add(2);
            rax = exe::call_va!(0x1_411d_ce60, (sub_obj, edx) -> usize);
            wr::<usize>(rcx + 0x48, rax);
        }
    }
    u8::from(rax != 0)
}

/// Original: `0x14206f3a0` (110 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_primitive_topology(rcx: usize, rdx: u32) {
    if rd::<u32>(rcx + 0xc40) != rdx {
        let mut cmd_pair = [0usize; 2];
        let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
        exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
        let cmd_list = rd::<usize>(pair_ptr + 0x8);
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0xa0);
        exe::call_va!(fn_ptr, (cmd_list, rdx) -> ());
        release_cmd_guard(rd::<usize>(pair_ptr));
        wr::<u32>(rcx + 0xc40, rdx);
        wr::<u8>(rcx + 0xa80, 0);
    }
}

/// Original: `0x14206f420` (108 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_viewport_from_ints(
    rcx: usize,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    min_z_bits: u32,
    max_z_bits: u32,
) {
    wr::<f32>(rcx + 0xa84, x as f32);
    wr::<f32>(rcx + 0xa88, y as f32);
    wr::<f32>(rcx + 0xa8c, w as f32);
    wr::<f32>(rcx + 0xa90, h as f32);
    wr::<u32>(rcx + 0xa94, min_z_bits);
    wr::<u32>(rcx + 0xa98, max_z_bits);
    exe::call_va!(0x1_4206_f8c0, (rcx, rcx + 0xa84) -> ());
}

/// Original: `0x14206f4a0` (171 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_scissor_rects_8x(
    rcx: usize,
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
) {
    let rect = [left, top, right, bottom];
    let mut rects = [[0u32; 4]; 8];
    let rects_ptr = core::ptr::addr_of_mut!(rects) as usize;
    let mut i = 0usize;
    while i < 8 {
        wr::<[u32; 4]>(rects_ptr + i * 16, rect);
        i += 1;
    }
    let mut cmd_pair = [0usize; 2];
    let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
    exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
    let cmd_list = rd::<usize>(pair_ptr + 0x8);
    let vtbl = rd::<usize>(cmd_list);
    let fn_ptr = rd::<usize>(vtbl + 0xb0);
    exe::call_va!(fn_ptr, (cmd_list, 8u32, rects_ptr) -> ());
    release_cmd_guard(rd::<usize>(pair_ptr));
    wr::<[u32; 4]>(rcx + 0xa9c, rect);
}

/// Original: `0x14206f560` (273 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_index_buffer(rcx: usize, rdx: usize, r8: usize) {
    if rdx != 0 {
        let gpu_va = rd::<u64>(rdx).wrapping_add(r8 as u64);
        let size_in_bytes = rd::<u32>(rdx + 0x10).wrapping_sub(r8 as u32);
        let format: u32 = if rd::<u64>(rdx + 0x8) == 2 { 0x39 } else { 0x2a };
        if gpu_va == rd::<u64>(rcx + 0xc30)
            && size_in_bytes == rd::<u32>(rcx + 0xc38)
            && format == rd::<u32>(rcx + 0xc3c)
        {
            return;
        }
        #[repr(C)]
        struct IbView {
            gpu_va: u64,
            size_in_bytes: u32,
            format: u32,
        }
        let mut view = IbView {
            gpu_va,
            size_in_bytes,
            format,
        };
        let view_ptr = core::ptr::addr_of_mut!(view) as usize;
        let mut cmd_pair = [0usize; 2];
        let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
        exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
        let cmd_list = rd::<usize>(pair_ptr + 0x8);
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0x158);
        exe::call_va!(fn_ptr, (cmd_list, view_ptr) -> ());
        release_cmd_guard(rd::<usize>(pair_ptr));
        let bytes = rd::<[u8; 16]>(view_ptr);
        wr::<[u8; 16]>(rcx + 0xc30, bytes);
    } else if rd::<u64>(rcx + 0xc30) != 0 {
        let mut cmd_pair = [0usize; 2];
        let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
        exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
        let cmd_list = rd::<usize>(pair_ptr + 0x8);
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0x158);
        exe::call_va!(fn_ptr, (cmd_list, 0usize) -> ());
        release_cmd_guard(rd::<usize>(pair_ptr));
        wr::<u64>(rcx + 0xc30, 0);
        wr::<u64>(rcx + 0xc38, 0);
    }
}

/// Original: `0x14206f800` (103 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_stencil_ref(rcx: usize, rdx: u32) {
    if rd::<u32>(rcx + 0xb08) != rdx {
        let mut cmd_pair = [0usize; 2];
        let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
        exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
        let cmd_list = rd::<usize>(pair_ptr + 0x8);
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0xc0);
        exe::call_va!(fn_ptr, (cmd_list, rdx) -> ());
        release_cmd_guard(rd::<usize>(pair_ptr));
        wr::<u32>(rcx + 0xb08, rdx);
    }
}

/// Original: `0x14206f870` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_pipeline_state_obj(rcx: usize, rdx: usize) {
    if rd::<usize>(rcx + 0xb00) != rdx {
        wr::<usize>(rcx + 0xb00, rdx);
        wr::<u8>(rcx + 0xa80, 0);
    }
}

/// Original: `0x14206f890` (38 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_set_blend_depth_state(rcx: usize, rdx: usize) {
    if rd::<usize>(rcx + 0xaf8) != rdx {
        let default_state = rd::<usize>(exe::ptr::<usize>(0x1_4397_fd30) as usize);
        wr::<u8>(rcx + 0xa80, 0);
        let next = if rdx != 0 { rdx } else { default_state };
        wr::<usize>(rcx + 0xaf8, next);
    }
}

/// Original: `0x14206f8c0` (293 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_apply_viewports_8x(rcx: usize, rdx: usize) {
    let vp_lo = rd::<[u8; 16]>(rdx);
    let vp_hi = rd::<u64>(rdx + 0x10);
    let mut vps = [0u8; 24 * 8];
    let vps_ptr = core::ptr::addr_of_mut!(vps) as usize;
    let mut i = 0usize;
    while i < 8 {
        wr::<[u8; 16]>(vps_ptr + i * 24, vp_lo);
        wr::<u64>(vps_ptr + i * 24 + 16, vp_hi);
        i += 1;
    }
    let mut count: u32 = 0;
    let mut slot = 0usize;
    while slot < 8 {
        if rd::<u64>(rcx + slot * 8) != 0 {
            count += 1;
        }
        slot += 1;
    }
    if count != 0 || rd::<u64>(rcx + 0x40) != 0 {
        let mut cmd_pair = [0usize; 2];
        let pair_ptr = core::ptr::addr_of_mut!(cmd_pair) as usize;
        exe::call_va!(0x1_411d_5b00, (pair_ptr) -> ());
        let cmd_list = rd::<usize>(pair_ptr + 0x8);
        let vtbl = rd::<usize>(cmd_list);
        let fn_ptr = rd::<usize>(vtbl + 0xa8);
        exe::call_va!(fn_ptr, (cmd_list, 8u32, vps_ptr) -> ());
        release_cmd_guard(rd::<usize>(pair_ptr));
    }
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_d3d_dispatch_rt_preset_by_ptr(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    unsafe extern "win64" fn m_c8b0(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: u32,
        a4: u32,
        a5: u32,
        a6: usize,
    ) -> usize {
        a0 ^ a1.rotate_left(7)
            ^ a2.rotate_left(13)
            ^ (a3 as usize).rotate_left(19)
            ^ (a4 as usize).rotate_left(23)
            ^ (a5 as usize).rotate_left(29)
            ^ a6.rotate_left(37)
    }
    let _m = crate::difftest::Mock::new(0x1_4206_c8b0, m_c8b0 as *const ())?;

    let table = exe::ptr::<u8>(0x1_43a6_32f0) as *mut u8;
    let snap = crate::difftest::Snapshot::take(table, 128);
    for i in 0..16usize {
        wr::<u64>(table as usize + i * 8, 0x1000_0000_0000_0000 | ((i as u64) * 0x1234_5678));
    }

    for idx in [0i32, 1i32] {
        let p = (&idx as *const i32) as usize;
        let ro = orig_fn(p);
        let rn = new_fn(p);
        if ro != rn {
            snap.restore();
            return Err(format!("d3d_dispatch_rt_preset_by_ptr mismatch idx={idx}: {ro:#x} != {rn:#x}"));
        }
    }
    snap.restore();
    Ok(2)
}

pub unsafe fn diff_d3d_reset_rt_config_default(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn() -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    unsafe extern "win64" fn m_bea0(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: usize,
        a4: u32,
    ) -> usize {
        0x55aa_1234usize ^ a0 ^ a1 ^ a2 ^ a3 ^ (a4 as usize)
    }
    let _m = crate::difftest::Mock::new(0x1_4206_bea0, m_bea0 as *const ())?;

    let ro = orig_fn();
    let rn = new_fn();
    crate::diff_eq!(ro, rn, "d3d_reset_rt_config_default");
    Ok(1)
}

pub unsafe fn diff_d3d_dispatch_rt_preset_by_idx(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(i32) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    unsafe extern "win64" fn m_c8b0(
        a0: usize,
        a1: usize,
        a2: usize,
        a3: u32,
        a4: u32,
        a5: u32,
        a6: usize,
    ) -> usize {
        a0 ^ a1.rotate_left(5)
            ^ a2.rotate_left(11)
            ^ (a3 as usize).rotate_left(17)
            ^ (a4 as usize).rotate_left(23)
            ^ (a5 as usize).rotate_left(31)
            ^ a6.rotate_left(41)
    }
    let _m = crate::difftest::Mock::new(0x1_4206_c8b0, m_c8b0 as *const ())?;

    let table = exe::ptr::<u8>(0x1_43a6_32f0) as *mut u8;
    let snap = crate::difftest::Snapshot::take(table, 128);
    for i in 0..16usize {
        wr::<u64>(table as usize + i * 8, 0x2000_0000_0000_0000 | ((i as u64) * 0x8765_4321));
    }

    for idx in [0i32, 1i32] {
        let ro = orig_fn(idx);
        let rn = new_fn(idx);
        if ro != rn {
            snap.restore();
            return Err(format!("d3d_dispatch_rt_preset_by_idx mismatch idx={idx}: {ro:#x} != {rn:#x}"));
        }
    }
    snap.restore();
    Ok(2)
}

pub unsafe fn diff_d3d_get_rt_pool_entry_28(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(i32, i32) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);

    let mut fake_table = [0usize; 32];
    for (i, slot) in fake_table.iter_mut().enumerate() {
        *slot = 0xdead_0000_0000_0000usize | (i * 0x1111);
    }
    let mut fake_mgr = [0u8; 0x100];
    let mgr_ptr = fake_mgr.as_mut_ptr() as usize;
    wr::<usize>(mgr_ptr + 0xd8, fake_table.as_ptr() as usize);
    wr::<usize>(g_addr, mgr_ptr);

    let res = (|| -> Result<u32, String> {
        let mut cases = 0u32;
        for (a, b) in [(0i32, 0i32), (1, 2), (2, 1), (3, 4), (5, 0)] {
            let o = orig_fn(a, b);
            let n = new_fn(a, b);
            crate::diff_eq!(o, n, "d3d_get_rt_pool_entry_28 result");
            cases += 1;
        }
        Ok(cases)
    })();

    wr::<usize>(g_addr, saved_g);
    res
}

pub unsafe fn diff_d3d_get_rt_pool_entry_10(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(i32) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);

    let mut fake_table = [0usize; 16];
    for (i, slot) in fake_table.iter_mut().enumerate() {
        *slot = 0xbeef_0000_0000_0000usize | (i * 0x2222);
    }
    let mut fake_mgr = [0u8; 0x100];
    let mgr_ptr = fake_mgr.as_mut_ptr() as usize;
    wr::<usize>(mgr_ptr + 0xd8, fake_table.as_ptr() as usize);
    wr::<usize>(g_addr, mgr_ptr);

    let res = (|| -> Result<u32, String> {
        let mut cases = 0u32;
        for idx in [0i32, 1, 3, 7, 11] {
            let o = orig_fn(idx);
            let n = new_fn(idx);
            crate::diff_eq!(o, n, "d3d_get_rt_pool_entry_10 result");
            cases += 1;
        }
        Ok(cases)
    })();

    wr::<usize>(g_addr, saved_g);
    res
}

pub unsafe fn diff_d3d_get_rt_pool_head(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn() -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);

    let fake_table = [0x1111usize, 0x1234_5678_9abc_def0usize, 0x2222usize];
    let mut fake_mgr = [0u8; 0x100];
    let mgr_ptr = fake_mgr.as_mut_ptr() as usize;
    wr::<usize>(mgr_ptr + 0xd8, fake_table.as_ptr() as usize);
    wr::<usize>(g_addr, mgr_ptr);

    let o = orig_fn();
    let n = new_fn();
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(o, n, "d3d_get_rt_pool_head result");
    Ok(1)
}

pub unsafe fn diff_d3d_splat_rt_desc_3x(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let mut src = [0u8; 32];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7).wrapping_add(3);
    }
    let mut dst_o = [0xa5u8; 0xd0];
    let mut dst_n = [0xa5u8; 0xd0];

    let ro = orig_fn(dst_o.as_mut_ptr() as usize, src.as_ptr() as usize);
    let rn = new_fn(dst_n.as_mut_ptr() as usize, src.as_ptr() as usize);

    crate::diff_eq!(ro, dst_o.as_ptr() as usize, "d3d_splat_rt_desc_3x ret orig");
    crate::diff_eq!(rn, dst_n.as_ptr() as usize, "d3d_splat_rt_desc_3x ret new");
    crate::diff_eq!(dst_o, dst_n, "d3d_splat_rt_desc_3x buffer");
    Ok(1)
}

pub unsafe fn diff_d3d_copy_rt_desc_slot0(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let mut src = [0u8; 32];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(13).wrapping_add(9);
    }
    let mut dst_o = [0x5au8; 0x60];
    let mut dst_n = [0x5au8; 0x60];

    let ro = orig_fn(dst_o.as_mut_ptr() as usize, src.as_ptr() as usize);
    let rn = new_fn(dst_n.as_mut_ptr() as usize, src.as_ptr() as usize);

    crate::diff_eq!(ro, dst_o.as_ptr() as usize, "d3d_copy_rt_desc_slot0 ret orig");
    crate::diff_eq!(rn, dst_n.as_ptr() as usize, "d3d_copy_rt_desc_slot0 ret new");
    crate::diff_eq!(dst_o, dst_n, "d3d_copy_rt_desc_slot0 buffer");
    Ok(1)
}

pub unsafe fn diff_d3d_extract_rt_handle_pair(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let mut out_o = [0xffff_ffff_ffff_ffffu64; 2];
    let mut out_n = [0xffff_ffff_ffff_ffffu64; 2];
    orig_fn(out_o.as_mut_ptr() as usize, 0);
    new_fn(out_n.as_mut_ptr() as usize, 0);
    crate::diff_eq!(out_o, out_n, "d3d_extract_rt_handle_pair null rdx");

    let src = [0x1111_2222_3333_4444u64, 0xaaaa_bbbb_cccc_ddddu64, 0x5555_6666_7777_8888u64];
    orig_fn(out_o.as_mut_ptr() as usize, src.as_ptr() as usize);
    new_fn(out_n.as_mut_ptr() as usize, src.as_ptr() as usize);
    crate::diff_eq!(out_o, out_n, "d3d_extract_rt_handle_pair non-null rdx");
    Ok(2)
}

pub unsafe fn diff_d3d_set_rt_slot_debug_name(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(i32, usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut REQ_LEN: u32 = 0;
    unsafe extern "win64" fn m_f2b0(_wstr: usize, req: u32) {
        unsafe {
            REQ_LEN = req;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_423e_f2b0, m_f2b0 as *const ())?;

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);

    let name = b"rt_color_0\0";
    for src_ptr in [name.as_ptr() as usize, 0usize] {
        let mut wbuf_o = [0u16; 32];
        let mut wbuf_n = [0u16; 32];
        let mut mgr_o = vec![0u8; 0x1200];
        let mut mgr_n = vec![0u8; 0x1200];
        let off = (1usize * 9) << 5;
        wr::<usize>(mgr_o.as_mut_ptr() as usize + off + 0xff0, wbuf_o.as_mut_ptr() as usize);
        wr::<u32>(mgr_o.as_mut_ptr() as usize + off + 0xffc, 32);
        wr::<u32>(mgr_o.as_mut_ptr() as usize + off + 0x1000, 4);

        wr::<usize>(mgr_n.as_mut_ptr() as usize + off + 0xff0, wbuf_n.as_mut_ptr() as usize);
        wr::<u32>(mgr_n.as_mut_ptr() as usize + off + 0xffc, 32);
        wr::<u32>(mgr_n.as_mut_ptr() as usize + off + 0x1000, 4);

        wr::<usize>(g_addr, mgr_o.as_mut_ptr() as usize);
        REQ_LEN = 0;
        orig_fn(1, src_ptr);
        let req_o = REQ_LEN;

        wr::<usize>(g_addr, mgr_n.as_mut_ptr() as usize);
        REQ_LEN = 0;
        new_fn(1, src_ptr);
        let req_n = REQ_LEN;

        wr::<usize>(g_addr, saved_g);

        crate::diff_eq!(req_o, req_n, "d3d_set_rt_slot_debug_name req_len");
        crate::diff_eq!(wbuf_o, wbuf_n, "d3d_set_rt_slot_debug_name wbuf");
        crate::diff_eq!(
            rd::<u32>(mgr_o.as_ptr() as usize + off + 0x1000),
            rd::<u32>(mgr_n.as_ptr() as usize + off + 0x1000),
            "d3d_set_rt_slot_debug_name new_len"
        );
    }
    Ok(2)
}

pub unsafe fn diff_d3d_set_mgr_debug_name(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut REQ_LEN: u32 = 0;
    unsafe extern "win64" fn m_f2b0(_wstr: usize, req: u32) {
        unsafe {
            REQ_LEN = req;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_423e_f2b0, m_f2b0 as *const ())?;

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);

    let name = b"main_rt_mgr\0";
    for src_ptr in [name.as_ptr() as usize, 0usize] {
        let mut wbuf_o = [0u16; 32];
        let mut wbuf_n = [0u16; 32];
        let mut mgr_o = vec![0u8; 0xf00];
        let mut mgr_n = vec![0u8; 0xf00];
        wr::<usize>(mgr_o.as_mut_ptr() as usize + 0xed0, wbuf_o.as_mut_ptr() as usize);
        wr::<u32>(mgr_o.as_mut_ptr() as usize + 0xedc, 32);
        wr::<u32>(mgr_o.as_mut_ptr() as usize + 0xee0, 4);

        wr::<usize>(mgr_n.as_mut_ptr() as usize + 0xed0, wbuf_n.as_mut_ptr() as usize);
        wr::<u32>(mgr_n.as_mut_ptr() as usize + 0xedc, 32);
        wr::<u32>(mgr_n.as_mut_ptr() as usize + 0xee0, 4);

        wr::<usize>(g_addr, mgr_o.as_mut_ptr() as usize);
        REQ_LEN = 0;
        orig_fn(src_ptr);
        let req_o = REQ_LEN;

        wr::<usize>(g_addr, mgr_n.as_mut_ptr() as usize);
        REQ_LEN = 0;
        new_fn(src_ptr);
        let req_n = REQ_LEN;

        wr::<usize>(g_addr, saved_g);

        crate::diff_eq!(req_o, req_n, "d3d_set_mgr_debug_name req_len");
        crate::diff_eq!(wbuf_o, wbuf_n, "d3d_set_mgr_debug_name wbuf");
        crate::diff_eq!(
            rd::<u32>(mgr_o.as_ptr() as usize + 0xee0),
            rd::<u32>(mgr_n.as_ptr() as usize + 0xee0),
            "d3d_set_mgr_debug_name new_len"
        );
    }
    Ok(2)
}

pub unsafe fn diff_d3d_bind_rt_and_notify(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize, u8) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_cdf0() -> usize {
        0x4444
    }
    unsafe extern "win64" fn m_64f0(ctx: usize, flag: u32, r8d: u32) {
        unsafe {
            TRACE = (ctx as u64) ^ ((flag as u64) << 16) ^ ((r8d as u64) << 24);
        }
    }
    unsafe extern "win64" fn m_ba00(rdx: usize) -> usize {
        rdx.wrapping_add(0x99)
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_cdf0, m_cdf0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_64f0, m_64f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_ba00, m_ba00 as *const ())?;

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);

    let mut mgr = [0u8; 0xb0];
    mgr[0x9c] = 7;
    wr::<usize>(g_addr, mgr.as_ptr() as usize);

    TRACE = 0;
    let ro = orig_fn(0x1111, 0x2222, 3);
    let tr_o = TRACE;

    TRACE = 0;
    let rn = new_fn(0x1111, 0x2222, 3);
    let tr_n = TRACE;

    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(ro, rn, "d3d_bind_rt_and_notify ret");
    crate::diff_eq!(tr_o, tr_n, "d3d_bind_rt_and_notify trace");
    Ok(1)
}

pub unsafe fn diff_d3d_set_active_ctx_and_bind(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize, u8) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_cdf0() -> usize {
        0x5555
    }
    unsafe extern "win64" fn m_64f0(ctx: usize, flag: u32, r8d: u32) {
        unsafe {
            TRACE = (ctx as u64) ^ ((flag as u64) << 16) ^ ((r8d as u64) << 24);
        }
    }
    unsafe extern "win64" fn m_ba00(rdx: usize) -> usize {
        rdx.wrapping_add(0xaa)
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_cdf0, m_cdf0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_64f0, m_64f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_ba00, m_ba00 as *const ())?;

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut tls_buf = [0u8; 0x80];
    tls_buf[0x70] = 1;
    wr::<usize>(tls_slot, tls_buf.as_mut_ptr() as usize);

    let mut mgr_o = [0u8; 0xe0];
    mgr_o[0x9c] = 5;
    let mut mgr_n = mgr_o;

    wr::<usize>(g_addr, mgr_o.as_mut_ptr() as usize);
    TRACE = 0;
    let ro = orig_fn(0x1234, 0x5678, 9);
    let tr_o = TRACE;

    wr::<usize>(g_addr, mgr_n.as_mut_ptr() as usize);
    TRACE = 0;
    let rn = new_fn(0x1234, 0x5678, 9);
    let tr_n = TRACE;

    wr::<usize>(tls_slot, saved_tls0);
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(ro, rn, "d3d_set_active_ctx_and_bind ret");
    crate::diff_eq!(tr_o, tr_n, "d3d_set_active_ctx_and_bind trace");
    crate::diff_eq!(mgr_o, mgr_n, "d3d_set_active_ctx_and_bind mgr");
    Ok(1)
}

pub unsafe fn diff_d3d_set_mgr_active_ctx(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);

    let mut mgr_o = [0u8; 0x100];
    let mut mgr_n = [0u8; 0x100];

    wr::<usize>(g_addr, mgr_o.as_mut_ptr() as usize);
    orig_fn(0x1357_9bdf_2468_ace0usize);

    wr::<usize>(g_addr, mgr_n.as_mut_ptr() as usize);
    new_fn(0x1357_9bdf_2468_ace0usize);

    wr::<usize>(g_addr, saved_g);
    crate::diff_eq!(mgr_o, mgr_n, "d3d_set_mgr_active_ctx state");
    Ok(1)
}

pub unsafe fn diff_d3d_clear_rt_binding_state(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let mut buf_o = [0xffu8; 0x40];
    let mut buf_n = [0xffu8; 0x40];

    orig_fn(buf_o.as_mut_ptr() as usize);
    new_fn(buf_n.as_mut_ptr() as usize);

    crate::diff_eq!(buf_o, buf_n, "d3d_clear_rt_binding_state bytes");
    Ok(1)
}

pub unsafe fn diff_d3d_apply_predication_and_heaps(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut TRACE: u64 = 0;
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(v);
        }
    }
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_vfunc_e8(_cmd_list: usize, pred: usize) {
        unsafe { rec(0x10 ^ (pred as u64)) };
    }
    unsafe extern "win64" fn m_cdf0() -> usize {
        0x7777
    }
    unsafe extern "win64" fn m_5ba0(ctx: usize) {
        unsafe { rec(0x20 ^ (ctx as u64)) };
    }
    unsafe extern "win64" fn m_vfunc_e0(_cmd_list: usize, count: u32, heaps_ptr: usize) {
        let h0 = rd::<u64>(heaps_ptr);
        let h1 = rd::<u64>(heaps_ptr + 0x8);
        unsafe { rec(0x30 ^ (count as u64) ^ (h0 << 8) ^ (h1 << 16)) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_cdf0, m_cdf0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_5ba0, m_5ba0 as *const ())?;

    let mut vtbl = [0usize; 0xf0 / 8];
    vtbl[0xe0 / 8] = m_vfunc_e0 as *const () as usize;
    vtbl[0xe8 / 8] = m_vfunc_e8 as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    for active in [false, true] {
        let mut st = [0u8; 0xa90];
        if active {
            wr::<usize>(st.as_mut_ptr() as usize + 0xa68, 0x1111);
            wr::<usize>(st.as_mut_ptr() as usize + 0xa70, 0x2222);
            wr::<usize>(st.as_mut_ptr() as usize + 0xa78, 0x3333);
        }

        TRACE = 1;
        orig_fn(st.as_ptr() as usize);
        let tr_o = TRACE;

        TRACE = 1;
        new_fn(st.as_ptr() as usize);
        let tr_n = TRACE;

        crate::diff_eq!(tr_o, tr_n, "d3d_apply_predication_and_heaps active={active}");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_invalidate_bound_ia_state(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut BOUND_IB_GPU_VA: u64 = 0;
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_vfunc_158(_cmd_list: usize, ib_view: usize) {
        unsafe {
            BOUND_IB_GPU_VA = rd::<u64>(ib_view);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0x160 / 8];
    vtbl[0x158 / 8] = m_vfunc_158 as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let mut entries = [0u32; 12];
    entries[0] = 5; // VB
    entries[4] = 6; // IB
    entries[8] = 0; // terminator

    let mut st_o = [0u8; 0xc50];
    wr::<u64>(st_o.as_mut_ptr() as usize + 0xc30, 0xdead_beef_cafe_0000);
    let mut st_n = st_o;

    BOUND_IB_GPU_VA = 0;
    orig_fn(st_o.as_mut_ptr() as usize, entries.as_ptr() as usize);
    let ib_o = BOUND_IB_GPU_VA;

    BOUND_IB_GPU_VA = 0;
    new_fn(st_n.as_mut_ptr() as usize, entries.as_ptr() as usize);
    let ib_n = BOUND_IB_GPU_VA;

    crate::diff_eq!(ib_o, ib_n, "d3d_invalidate_bound_ia_state ib_gpu_va");
    crate::diff_eq!(st_o[0xb2c], st_n[0xb2c], "d3d_invalidate_bound_ia_state vb_dirty");
    Ok(1)
}

pub unsafe fn diff_d3d_init_rt_view_with_opt(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize, usize, usize, usize) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_6c10(
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: usize,
        a6: u32,
        a7: u32,
        a8: usize,
        a9: usize,
    ) {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for v in [a1, a2, a3, a4, a5, a6 as usize, a7 as usize, a8, a9] {
            h = (h ^ (v as u64)).wrapping_mul(0x1000_0000_01b3);
        }
        unsafe {
            TRACE = h;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4206_6c10, m_6c10 as *const ())?;

    let mut out_o = [0xffff_ffff_ffff_ffffu64; 2];
    let mut out_n = [0xffff_ffff_ffff_ffffu64; 2];

    TRACE = 0;
    let ro = orig_fn(out_o.as_mut_ptr() as usize, 0x11, 0x22, 0x33, 0x44);
    let tr_o = TRACE;

    TRACE = 0;
    let rn = new_fn(out_n.as_mut_ptr() as usize, 0x11, 0x22, 0x33, 0x44);
    let tr_n = TRACE;

    crate::diff_eq!(ro == out_o.as_ptr() as usize, rn == out_n.as_ptr() as usize, "d3d_init_rt_view_with_opt ret");
    crate::diff_eq!(tr_o, tr_n, "d3d_init_rt_view_with_opt trace");
    crate::diff_eq!(out_o, out_n, "d3d_init_rt_view_with_opt out");
    Ok(1)
}

pub unsafe fn diff_d3d_init_rt_view_default(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_6c10(
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: u32,
        a6: u32,
        a7: u32,
        a8: usize,
        a9: usize,
    ) {
        let opt_b = rd::<[u32; 3]>(a4);
        let opt_a = rd::<[u32; 3]>(a8);
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for v in [
            a1 as u64,
            a2 as u64,
            a3 as u64,
            opt_b[0] as u64,
            opt_b[1] as u64,
            opt_b[2] as u64,
            a5 as u64,
            a6 as u64,
            a7 as u64,
            opt_a[0] as u64,
            opt_a[1] as u64,
            opt_a[2] as u64,
            a9 as u64,
        ] {
            h = (h ^ v).wrapping_mul(0x1000_0000_01b3);
        }
        unsafe {
            TRACE = h;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4206_6c10, m_6c10 as *const ())?;

    let mut out_o = [0xffff_ffff_ffff_ffffu64; 2];
    let mut out_n = [0xffff_ffff_ffff_ffffu64; 2];

    TRACE = 0;
    let ro = orig_fn(out_o.as_mut_ptr() as usize, 0x55, 0x66);
    let tr_o = TRACE;

    TRACE = 0;
    let rn = new_fn(out_n.as_mut_ptr() as usize, 0x55, 0x66);
    let tr_n = TRACE;

    crate::diff_eq!(ro == out_o.as_ptr() as usize, rn == out_n.as_ptr() as usize, "d3d_init_rt_view_default ret");
    crate::diff_eq!(tr_o, tr_n, "d3d_init_rt_view_default trace");
    crate::diff_eq!(out_o, out_n, "d3d_init_rt_view_default out");
    Ok(1)
}

pub unsafe fn diff_d3d_ensure_rt_query_slot(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize) -> u8;
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let mut state_o = [0u8; 0xc60];
    let mut state_n = [0u8; 0xc60];

    let ro0 = orig_fn(state_o.as_mut_ptr() as usize);
    let rn0 = new_fn(state_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro0, rn0, "d3d_ensure_rt_query_slot both null");

    wr::<usize>(state_o.as_mut_ptr() as usize + 0x48, 0x9999usize);
    wr::<usize>(state_n.as_mut_ptr() as usize + 0x48, 0x9999usize);
    let ro1 = orig_fn(state_o.as_mut_ptr() as usize);
    let rn1 = new_fn(state_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro1, rn1, "d3d_ensure_rt_query_slot already present");
    Ok(2)
}

pub unsafe fn diff_d3d_set_viewport_from_ints(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, i32, i32, i32, i32, u32, u32);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    // With [rcx + 0x00..=0x40] == 0, d3d_apply_viewports_8x skips command list submission.
    let mut state_o = [0u8; 0xac0];
    let mut state_n = [0u8; 0xac0];

    orig_fn(state_o.as_mut_ptr() as usize, 10, 20, 1920, 1080, 0.0f32.to_bits(), 1.0f32.to_bits());
    new_fn(state_n.as_mut_ptr() as usize, 10, 20, 1920, 1080, 0.0f32.to_bits(), 1.0f32.to_bits());

    crate::diff_eq!(
        &state_o[0xa84..0xa9c],
        &state_n[0xa84..0xa9c],
        "d3d_set_viewport_from_ints viewport floats"
    );
    Ok(1)
}

pub unsafe fn diff_d3d_set_pipeline_state_obj(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let mut state_o = [0u8; 0xb10];
    let mut state_n = [0u8; 0xb10];
    state_o[0xa80] = 1;
    state_n[0xa80] = 1;
    wr::<usize>(state_o.as_mut_ptr() as usize + 0xb00, 0x1111usize);
    wr::<usize>(state_n.as_mut_ptr() as usize + 0xb00, 0x1111usize);

    orig_fn(state_o.as_mut_ptr() as usize, 0x1111usize);
    new_fn(state_n.as_mut_ptr() as usize, 0x1111usize);
    crate::diff_eq!(state_o[0xa80], state_n[0xa80], "d3d_set_pipeline_state_obj unchanged dirty");

    orig_fn(state_o.as_mut_ptr() as usize, 0x2222usize);
    new_fn(state_n.as_mut_ptr() as usize, 0x2222usize);
    crate::diff_eq!(state_o[0xa80], state_n[0xa80], "d3d_set_pipeline_state_obj changed dirty");
    crate::diff_eq!(
        rd::<usize>(state_o.as_ptr() as usize + 0xb00),
        rd::<usize>(state_n.as_ptr() as usize + 0xb00),
        "d3d_set_pipeline_state_obj value"
    );
    Ok(2)
}

pub unsafe fn diff_d3d_set_blend_depth_state(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    let g_addr = exe::ptr::<usize>(0x1_4397_fd30) as usize;
    let saved_default = rd::<usize>(g_addr);
    wr::<usize>(g_addr, 0xdef0_1234_5678_9abcusize);

    let res = (|| -> Result<u32, String> {
        let mut state_o = [0u8; 0xb10];
        let mut state_n = [0u8; 0xb10];
        state_o[0xa80] = 1;
        state_n[0xa80] = 1;

        orig_fn(state_o.as_mut_ptr() as usize, 0x4444usize);
        new_fn(state_n.as_mut_ptr() as usize, 0x4444usize);
        crate::diff_eq!(state_o[0xa80], state_n[0xa80], "d3d_set_blend_depth_state non-null dirty");
        crate::diff_eq!(
            rd::<usize>(state_o.as_ptr() as usize + 0xaf8),
            rd::<usize>(state_n.as_ptr() as usize + 0xaf8),
            "d3d_set_blend_depth_state non-null val"
        );

        state_o[0xa80] = 1;
        state_n[0xa80] = 1;
        orig_fn(state_o.as_mut_ptr() as usize, 0usize);
        new_fn(state_n.as_mut_ptr() as usize, 0usize);
        crate::diff_eq!(state_o[0xa80], state_n[0xa80], "d3d_set_blend_depth_state fallback dirty");
        crate::diff_eq!(
            rd::<usize>(state_o.as_ptr() as usize + 0xaf8),
            rd::<usize>(state_n.as_ptr() as usize + 0xaf8),
            "d3d_set_blend_depth_state fallback val"
        );
        Ok(2)
    })();

    wr::<usize>(g_addr, saved_default);
    res
}

pub unsafe fn diff_d3d_clear_depth_stencil_target(orig: usize, new: usize) -> Result<u32, String> {
    static mut CMD_LIST_PTR: usize = 0;
    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_7c90(rdx: usize) {
        unsafe {
            TRACE ^= (rdx as u64).rotate_left(7);
        }
    }
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    #[cfg(not(target_arch = "aarch64"))]
    unsafe extern "win64" fn m_clear_dsv(
        _cmd_list: usize,
        dsv: usize,
        flags: u32,
        depth: f32,
        stencil: u8,
        num_rects: u32,
        rects_ptr: usize,
    ) {
        unsafe {
            TRACE ^= (dsv as u64)
                ^ ((flags as u64) << 16)
                ^ ((depth.to_bits() as u64) << 24)
                ^ ((stencil as u64) << 56)
                ^ (num_rects as u64)
                ^ (rects_ptr as u64);
        }
    }
    #[cfg(target_arch = "aarch64")]
    unsafe extern "win64" fn m_clear_dsv(
        _cmd_list: usize,
        dsv: usize,
        flags: u32,
        _r9: usize,
        stencil: u8,
        num_rects: u32,
        rects_ptr: usize,
        _xmm0: f32,
        _xmm1: f32,
        _xmm2: f32,
        depth: f32,
    ) {
        unsafe {
            TRACE ^= (dsv as u64)
                ^ ((flags as u64) << 16)
                ^ ((depth.to_bits() as u64) << 24)
                ^ ((stencil as u64) << 56)
                ^ (num_rects as u64)
                ^ (rects_ptr as u64);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_7c90, m_7c90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0x180 / 8];
    vtbl[0x178 / 8] = m_clear_dsv as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut tls_buf = vec![0u8; 0x6310];
    tls_buf[0x70] = 1;
    wr::<usize>(tls_slot, tls_buf.as_mut_ptr() as usize);

    let mut target = [0u8; 0x90];
    wr::<usize>(target.as_mut_ptr() as usize + 0x88, 0xabcd_ef01);

    for mask in [0u32, 2u32, 4u32, 6u32] {
        TRACE = 1;
        let _ = crate::simd_ops::call_va8_xmm(
            orig as u64,
            0,
            target.as_ptr() as usize as u64,
            0,
            0x2a,
            mask as u64,
            0,
            0,
            0,
            [0, 0, 0.75f32.to_bits() as u128, 0],
        );
        let tr_o = TRACE;

        TRACE = 1;
        let _ = crate::simd_ops::call_va8_xmm(
            new as u64,
            0,
            target.as_ptr() as usize as u64,
            0,
            0x2a,
            mask as u64,
            0,
            0,
            0,
            [0, 0, 0.75f32.to_bits() as u128, 0],
        );
        let tr_n = TRACE;

        if tr_o != tr_n {
            wr::<usize>(tls_slot, saved_tls0);
            return Err(format!("d3d_clear_depth_stencil_target mismatch mask={mask}"));
        }
    }
    wr::<usize>(tls_slot, saved_tls0);
    Ok(4)
}

pub unsafe fn diff_d3d_clear_render_target(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize, usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_7c90(_rdx: usize) {
        unsafe {
            TRACE ^= 0x7c90;
        }
    }
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_clear_rtv(
        _cmd_list: usize,
        rtv: usize,
        color_ptr: usize,
        num_rects: u32,
        rects_ptr: usize,
    ) {
        let c0 = rd::<u64>(color_ptr);
        unsafe {
            TRACE ^= (rtv as u64) ^ c0 ^ (num_rects as u64) ^ (rects_ptr as u64);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_7c90, m_7c90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0x188 / 8];
    vtbl[0x180 / 8] = m_clear_rtv as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut tls_buf = vec![0u8; 0x6310];
    tls_buf[0x70] = 1;
    wr::<usize>(tls_slot, tls_buf.as_mut_ptr() as usize);

    let color = [0.25f32.to_bits(), 0.5f32.to_bits(), 0.75f32.to_bits(), 1.0f32.to_bits()];
    for rtv in [0usize, 0x5566_7788usize] {
        let mut target = [0u8; 0x90];
        wr::<usize>(target.as_mut_ptr() as usize + 0x80, rtv);

        TRACE = 1;
        orig_fn(0, target.as_ptr() as usize, color.as_ptr() as usize);
        let tr_o = TRACE;

        TRACE = 1;
        new_fn(0, target.as_ptr() as usize, color.as_ptr() as usize);
        let tr_n = TRACE;

        if tr_o != tr_n {
            wr::<usize>(tls_slot, saved_tls0);
            return Err(format!("d3d_clear_render_target mismatch rtv={rtv:#x}"));
        }
    }
    wr::<usize>(tls_slot, saved_tls0);
    Ok(2)
}

pub unsafe fn diff_d3d_flush_vertex_buffers(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_set_vb(
        _cmd_list: usize,
        start_slot: u32,
        num_views: u32,
        views_ptr: usize,
    ) {
        unsafe {
            TRACE = ((start_slot as u64) << 32) | ((num_views as u64) << 16) | rd::<u64>(views_ptr);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0x168 / 8];
    vtbl[0x160 / 8] = m_set_vb as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let mut st_o = [0u8; 0xb60];
    let po = st_o.as_mut_ptr() as usize;
    wr::<i32>(po + 0xb28, 2);
    wr::<u8>(po + 0xb2c, 1);
    wr::<u64>(po + 0xb30, 0x1234);
    wr::<u64>(po + 0xb40, 0);
    let mut st_n = st_o;

    TRACE = 0;
    orig_fn(st_o.as_mut_ptr() as usize);
    let tr_o = TRACE;

    TRACE = 0;
    new_fn(st_n.as_mut_ptr() as usize);
    let tr_n = TRACE;

    crate::diff_eq!(tr_o, tr_n, "d3d_flush_vertex_buffers trace");
    crate::diff_eq!(st_o, st_n, "d3d_flush_vertex_buffers state");
    Ok(1)
}

pub unsafe fn diff_d3d_set_primitive_topology(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, u32);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut LAST_TOPO: u32 = 0;
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_set_topo(_cmd_list: usize, topo: u32) {
        unsafe {
            LAST_TOPO = topo;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0xa8 / 8];
    vtbl[0xa0 / 8] = m_set_topo as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let mut st_o = [0u8; 0xc50];
    st_o[0xa80] = 1;
    wr::<u32>(st_o.as_mut_ptr() as usize + 0xc40, 4);
    let mut st_n = st_o;

    LAST_TOPO = 0;
    orig_fn(st_o.as_mut_ptr() as usize, 5);
    let t_o = LAST_TOPO;

    LAST_TOPO = 0;
    new_fn(st_n.as_mut_ptr() as usize, 5);
    let t_n = LAST_TOPO;

    crate::diff_eq!(t_o, t_n, "d3d_set_primitive_topology topo");
    crate::diff_eq!(st_o, st_n, "d3d_set_primitive_topology state");
    Ok(1)
}

pub unsafe fn diff_d3d_set_scissor_rects_8x(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, u32, u32, u32, u32);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut RECTS_OUT: [u8; 128] = [0u8; 128];
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_set_scissors(_cmd_list: usize, count: u32, rects_ptr: usize) {
        if count == 8 {
            unsafe {
                RECTS_OUT = rd::<[u8; 128]>(rects_ptr);
            }
        }
    }
    let _m = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0xb8 / 8];
    vtbl[0xb0 / 8] = m_set_scissors as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let mut st_o = [0u8; 0xab0];
    let mut st_n = [0u8; 0xab0];

    RECTS_OUT = [0u8; 128];
    orig_fn(st_o.as_mut_ptr() as usize, 10, 20, 1920, 1080);
    let r_o = RECTS_OUT;

    RECTS_OUT = [0u8; 128];
    new_fn(st_n.as_mut_ptr() as usize, 10, 20, 1920, 1080);
    let r_n = RECTS_OUT;

    crate::diff_eq!(r_o, r_n, "d3d_set_scissor_rects_8x rects");
    crate::diff_eq!(st_o, st_n, "d3d_set_scissor_rects_8x state");
    Ok(1)
}

pub unsafe fn diff_d3d_set_index_buffer(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize, usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut TRACE: u64 = 0;
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_set_ib(_cmd_list: usize, view_ptr: usize) {
        let v = if view_ptr != 0 {
            rd::<u64>(view_ptr) ^ (rd::<u64>(view_ptr + 0x8) << 16)
        } else {
            0xffff_ffff_ffff_ffffu64
        };
        unsafe {
            TRACE = TRACE.wrapping_mul(0x1000_0000_01b3).wrapping_add(v);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0x160 / 8];
    vtbl[0x158 / 8] = m_set_ib as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let mut ib_desc = [0u8; 0x18];
    wr::<u64>(ib_desc.as_mut_ptr() as usize, 0x8000_0000);
    wr::<u64>(ib_desc.as_mut_ptr() as usize + 0x8, 2);
    wr::<u32>(ib_desc.as_mut_ptr() as usize + 0x10, 0x1000);

    let mut st_o = [0u8; 0xc50];
    let mut st_n = [0u8; 0xc50];

    TRACE = 1;
    orig_fn(st_o.as_mut_ptr() as usize, ib_desc.as_ptr() as usize, 0x100);
    orig_fn(st_o.as_mut_ptr() as usize, ib_desc.as_ptr() as usize, 0x100);
    orig_fn(st_o.as_mut_ptr() as usize, 0, 0);
    let tr_o = TRACE;

    TRACE = 1;
    new_fn(st_n.as_mut_ptr() as usize, ib_desc.as_ptr() as usize, 0x100);
    new_fn(st_n.as_mut_ptr() as usize, ib_desc.as_ptr() as usize, 0x100);
    new_fn(st_n.as_mut_ptr() as usize, 0, 0);
    let tr_n = TRACE;

    crate::diff_eq!(tr_o, tr_n, "d3d_set_index_buffer trace");
    crate::diff_eq!(st_o, st_n, "d3d_set_index_buffer state");
    Ok(3)
}

pub unsafe fn diff_d3d_set_stencil_ref(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, u32);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut LAST_REF: u32 = 0;
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_set_stencil(_cmd_list: usize, sref: u32) {
        unsafe {
            LAST_REF = sref;
        }
    }
    let _m = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0xc8 / 8];
    vtbl[0xc0 / 8] = m_set_stencil as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let mut st_o = [0u8; 0xb20];
    let mut st_n = [0u8; 0xb20];

    LAST_REF = 0;
    orig_fn(st_o.as_mut_ptr() as usize, 0x7f);
    let r_o = LAST_REF;

    LAST_REF = 0;
    new_fn(st_n.as_mut_ptr() as usize, 0x7f);
    let r_n = LAST_REF;

    crate::diff_eq!(r_o, r_n, "d3d_set_stencil_ref ref");
    crate::diff_eq!(st_o, st_n, "d3d_set_stencil_ref state");
    Ok(1)
}

pub unsafe fn diff_d3d_apply_viewports_8x(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = core::mem::transmute(orig);
    let new_fn: FnT = core::mem::transmute(new);

    static mut CMD_LIST_PTR: usize = 0;
    static mut VPS_OUT: [u8; 192] = [0u8; 192];
    unsafe extern "win64" fn m_5b00(pair_out: usize) {
        wr::<usize>(pair_out, 0);
        wr::<usize>(pair_out + 0x8, unsafe { CMD_LIST_PTR });
    }
    unsafe extern "win64" fn m_set_vps(_cmd_list: usize, count: u32, vps_ptr: usize) {
        if count == 8 {
            unsafe {
                VPS_OUT = rd::<[u8; 192]>(vps_ptr);
            }
        }
    }
    let _m = crate::difftest::Mock::new(0x1_411d_5b00, m_5b00 as *const ())?;

    let mut vtbl = [0usize; 0xb0 / 8];
    vtbl[0xa8 / 8] = m_set_vps as *const () as usize;
    let cmd_list = [vtbl.as_ptr() as usize];
    CMD_LIST_PTR = cmd_list.as_ptr() as usize;

    let vp = [
        0.0f32.to_bits(),
        0.0f32.to_bits(),
        1920.0f32.to_bits(),
        1080.0f32.to_bits(),
        0.0f32.to_bits(),
        1.0f32.to_bits(),
    ];

    let mut st = [0u8; 0x60];
    wr::<u64>(st.as_mut_ptr() as usize + 0x10, 1);

    VPS_OUT = [0u8; 192];
    orig_fn(st.as_ptr() as usize, vp.as_ptr() as usize);
    let v_o = VPS_OUT;

    VPS_OUT = [0u8; 192];
    new_fn(st.as_ptr() as usize, vp.as_ptr() as usize);
    let v_n = VPS_OUT;

    crate::diff_eq!(v_o, v_n, "d3d_apply_viewports_8x vps");
    Ok(1)
}

// REGISTRY:
// FLOAT_RETURN: none
//
// Replacement::verified_throws(0x1_4206_b5f0, "d3d::d3d_dispatch_rt_preset_by_ptr_14206b5f0", rend::d3d_rt_state::d3d_dispatch_rt_preset_by_ptr as *const (), Throws::Discard, rend::d3d_rt_state::diff_d3d_dispatch_rt_preset_by_ptr),
// Replacement::verified(0x1_4206_be00, "d3d::d3d_get_rt_pool_entry_28_14206be00", rend::d3d_rt_state::d3d_get_rt_pool_entry_28 as *const (), rend::d3d_rt_state::diff_d3d_get_rt_pool_entry_28),
// Replacement::verified(0x1_4206_be30, "d3d::d3d_get_rt_pool_entry_10_14206be30", rend::d3d_rt_state::d3d_get_rt_pool_entry_10 as *const (), rend::d3d_rt_state::diff_d3d_get_rt_pool_entry_10),
// Replacement::verified(0x1_4206_be50, "d3d::d3d_get_rt_pool_head_14206be50", rend::d3d_rt_state::d3d_get_rt_pool_head as *const (), rend::d3d_rt_state::diff_d3d_get_rt_pool_head),
// Replacement::verified_throws(0x1_4206_be70, "d3d::d3d_reset_rt_config_default_14206be70", rend::d3d_rt_state::d3d_reset_rt_config_default as *const (), Throws::Discard, rend::d3d_rt_state::diff_d3d_reset_rt_config_default),
// Replacement::verified_throws(0x1_4206_c810, "d3d::d3d_dispatch_rt_preset_by_idx_14206c810", rend::d3d_rt_state::d3d_dispatch_rt_preset_by_idx as *const (), Throws::Discard, rend::d3d_rt_state::diff_d3d_dispatch_rt_preset_by_idx),
// Replacement::verified(0x1_4206_c860, "d3d::d3d_splat_rt_desc_3x_14206c860", rend::d3d_rt_state::d3d_splat_rt_desc_3x as *const (), rend::d3d_rt_state::diff_d3d_splat_rt_desc_3x),
// Replacement::verified(0x1_4206_c890, "d3d::d3d_copy_rt_desc_slot0_14206c890", rend::d3d_rt_state::d3d_copy_rt_desc_slot0 as *const (), rend::d3d_rt_state::diff_d3d_copy_rt_desc_slot0),
// Replacement::verified(0x1_4206_d140, "d3d::d3d_extract_rt_handle_pair_14206d140", rend::d3d_rt_state::d3d_extract_rt_handle_pair as *const (), rend::d3d_rt_state::diff_d3d_extract_rt_handle_pair),
// Replacement::verified(0x1_4206_d2e0, "d3d::d3d_set_rt_slot_debug_name_14206d2e0", rend::d3d_rt_state::d3d_set_rt_slot_debug_name as *const (), rend::d3d_rt_state::diff_d3d_set_rt_slot_debug_name),
// Replacement::verified(0x1_4206_d390, "d3d::d3d_set_mgr_debug_name_14206d390", rend::d3d_rt_state::d3d_set_mgr_debug_name as *const (), rend::d3d_rt_state::diff_d3d_set_mgr_debug_name),
// Replacement::verified_throws(0x1_4206_d430, "d3d::d3d_bind_rt_and_notify_14206d430", rend::d3d_rt_state::d3d_bind_rt_and_notify as *const (), Throws::Discard, rend::d3d_rt_state::diff_d3d_bind_rt_and_notify),
// Replacement::verified(0x1_4206_d480, "d3d::d3d_set_mgr_active_ctx_14206d480", rend::d3d_rt_state::d3d_set_mgr_active_ctx as *const (), rend::d3d_rt_state::diff_d3d_set_mgr_active_ctx),
// Replacement::verified_throws(0x1_4206_d4a0, "d3d::d3d_set_active_ctx_and_bind_14206d4a0", rend::d3d_rt_state::d3d_set_active_ctx_and_bind as *const (), Throws::Discard, rend::d3d_rt_state::diff_d3d_set_active_ctx_and_bind),
// Replacement::verified(0x1_4206_e210, "d3d::d3d_apply_predication_and_heaps_14206e210", rend::d3d_rt_state::d3d_apply_predication_and_heaps as *const (), rend::d3d_rt_state::diff_d3d_apply_predication_and_heaps),
// Replacement::verified(0x1_4206_e2c0, "d3d::d3d_invalidate_bound_ia_state_14206e2c0", rend::d3d_rt_state::d3d_invalidate_bound_ia_state as *const (), rend::d3d_rt_state::diff_d3d_invalidate_bound_ia_state),
// Replacement::verified(0x1_4206_e670, "d3d::d3d_init_rt_view_with_opt_14206e670", rend::d3d_rt_state::d3d_init_rt_view_with_opt as *const (), rend::d3d_rt_state::diff_d3d_init_rt_view_with_opt),
// Replacement::verified(0x1_4206_e6d0, "d3d::d3d_init_rt_view_default_14206e6d0", rend::d3d_rt_state::d3d_init_rt_view_default as *const (), rend::d3d_rt_state::diff_d3d_init_rt_view_default),
// Replacement::verified(0x1_4206_e9d0, "d3d::d3d_clear_rt_binding_state_14206e9d0", rend::d3d_rt_state::d3d_clear_rt_binding_state as *const (), rend::d3d_rt_state::diff_d3d_clear_rt_binding_state),
// Replacement::verified(0x1_4206_ecc0, "d3d::d3d_clear_depth_stencil_target_14206ecc0", rend::d3d_rt_state::d3d_clear_depth_stencil_target as *const (), rend::d3d_rt_state::diff_d3d_clear_depth_stencil_target),
// Replacement::verified(0x1_4206_edb0, "d3d::d3d_clear_render_target_14206edb0", rend::d3d_rt_state::d3d_clear_render_target as *const (), rend::d3d_rt_state::diff_d3d_clear_render_target),
// Replacement::verified(0x1_4206_f290, "d3d::d3d_flush_vertex_buffers_14206f290", rend::d3d_rt_state::d3d_flush_vertex_buffers as *const (), rend::d3d_rt_state::diff_d3d_flush_vertex_buffers),
// Replacement::verified(0x1_4206_f350, "d3d::d3d_ensure_rt_query_slot_14206f350", rend::d3d_rt_state::d3d_ensure_rt_query_slot as *const (), rend::d3d_rt_state::diff_d3d_ensure_rt_query_slot),
// Replacement::verified(0x1_4206_f3a0, "d3d::d3d_set_primitive_topology_14206f3a0", rend::d3d_rt_state::d3d_set_primitive_topology as *const (), rend::d3d_rt_state::diff_d3d_set_primitive_topology),
// Replacement::verified(0x1_4206_f420, "d3d::d3d_set_viewport_from_ints_14206f420", rend::d3d_rt_state::d3d_set_viewport_from_ints as *const (), rend::d3d_rt_state::diff_d3d_set_viewport_from_ints),
// Replacement::verified(0x1_4206_f4a0, "d3d::d3d_set_scissor_rects_8x_14206f4a0", rend::d3d_rt_state::d3d_set_scissor_rects_8x as *const (), rend::d3d_rt_state::diff_d3d_set_scissor_rects_8x),
// Replacement::verified(0x1_4206_f560, "d3d::d3d_set_index_buffer_14206f560", rend::d3d_rt_state::d3d_set_index_buffer as *const (), rend::d3d_rt_state::diff_d3d_set_index_buffer),
// Replacement::verified(0x1_4206_f800, "d3d::d3d_set_stencil_ref_14206f800", rend::d3d_rt_state::d3d_set_stencil_ref as *const (), rend::d3d_rt_state::diff_d3d_set_stencil_ref),
// Replacement::verified(0x1_4206_f870, "d3d::d3d_set_pipeline_state_obj_14206f870", rend::d3d_rt_state::d3d_set_pipeline_state_obj as *const (), rend::d3d_rt_state::diff_d3d_set_pipeline_state_obj),
// Replacement::verified(0x1_4206_f890, "d3d::d3d_set_blend_depth_state_14206f890", rend::d3d_rt_state::d3d_set_blend_depth_state as *const (), rend::d3d_rt_state::diff_d3d_set_blend_depth_state),
// Replacement::verified(0x1_4206_f8c0, "d3d::d3d_apply_viewports_8x_14206f8c0", rend::d3d_rt_state::d3d_apply_viewports_8x as *const (), rend::d3d_rt_state::diff_d3d_apply_viewports_8x),
