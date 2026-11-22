use crate::exe::{self, rd, wr};

/// Original: `0x142075e60` (39 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_find_format_entry_142075e60(key: u32) -> u8 {
    let base = exe::ptr::<u32>(0x1_4297_eda0) as usize;
    let mut i: u32 = 0;
    while i < 0x2e {
        let entry_addr = base + (i as usize) * 20;
        if rd::<u32>(entry_addr) == key {
            return i as u8;
        }
        i += 1;
    }
    0
}

/// Original: `0x142075f30` (39 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_release_pass_entry_142075f30(this: usize) {
    let handle = rd::<usize>(this + 0x18);
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    exe::call_va!(0x1_4209_6a90, (mgr, handle) -> ());
    wr::<usize>(this + 0x18, 0);
}

/// Original: `0x142076070` (207 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_save_tls_pass_state_142076070(
    _unused: usize,
    buf_obj: usize,
) -> usize {
    let mut extra_size = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb4) as usize) as usize;
    let cap = rd::<u32>(buf_obj + 0x8) as usize;
    let needed = extra_size + 0x400;
    if cap < needed {
        exe::call_va!(0x1_4028_e490, (buf_obj, needed) -> ());
        extra_size = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb4) as usize) as usize;
    }
    let tls0 = rd::<usize>(exe::tls_pointer());
    let dst = rd::<usize>(buf_obj);
    let src_hdr = tls0 + 0x463e0;
    let mut off = 0usize;
    while off < 0x400 {
        wr::<u64>(dst + off, rd::<u64>(src_hdr + off));
        off += 8;
    }
    let dst_tail = rd::<usize>(buf_obj) + 0x400;
    let src_tail = tls0 + 0x63d0;
    exe::call_va!(0x1_4183_acbc, (dst_tail, src_tail, extra_size) -> usize)
}

/// Original: `0x142076150` (40 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_pass_tail_142076150(this: usize) {
    let mut off = 0x9e0usize;
    while off < 0xa60 {
        wr::<u64>(this + off, 0);
        off += 8;
    }
}

/// Original: `0x142076180` (17 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_pass_body_142076180(this: usize) -> usize {
    let dst = this + 0x50;
    exe::call_va!(0x1_4183_acc2, (dst, 0u32, 0x990usize) -> usize)
}

/// Original: `0x142076300` (208 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_restore_tls_pass_state_142076300(
    this: usize,
    src_holder: usize,
) -> usize {
    exe::call_va!(0x1_4207_61a0, (this, src_holder) -> ());
    let tls0 = rd::<usize>(exe::tls_pointer());
    if src_holder != 0 {
        let src_buf = rd::<usize>(src_holder);
        exe::call_va!(0x1_4183_acc2, (tls0 + 0x463e0, 0u32, 0x400usize) -> usize);
        let extra_size = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb4) as usize) as usize;
        if src_buf != 0 {
            exe::call_va!(0x1_4183_acbc, (tls0 + 0x63d0, src_buf + 0x400, extra_size) -> usize);
        } else {
            exe::call_va!(0x1_4183_acc2, (tls0 + 0x63d0, 0u32, extra_size) -> usize);
        }
        exe::call_va!(0x1_4183_acc2, (tls0 + 0x47460, 0xffff_ffffu32, 0x4000usize) -> usize)
    } else {
        exe::call_va!(0x1_4183_acc2, (tls0 + 0x463e0, 0u32, 0x400usize) -> usize);
        let extra_size = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb4) as usize) as usize;
        exe::call_va!(0x1_4183_acc2, (tls0 + 0x63d0, 0u32, extra_size) -> usize);
        exe::call_va!(0x1_4183_acc2, (tls0 + 0x47460, 0xffff_ffffu32, 0x4000usize) -> usize)
    }
}

/// Original: `0x1420763e0` (48 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_tls_pass_ctx_1420763e0() -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    tls0 + 0x467e0
}

/// Original: `0x142076920` (52 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_mark_pass_flag_locked_142076920(this: usize) {
    let lock = this + 0x38;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let _ = crate::simd_ops::atomic_xchg_u8((this + 0x40) as u64, 1); // xchg byte [rdi+0x40]
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
}

/// Original: `0x142076ed0` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_pass_global_142076ed0(idx: u32) -> usize {
    match idx {
        0 => rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize),
        1 => rd::<usize>(exe::ptr::<usize>(0x1_4397_fc70) as usize),
        2 => rd::<usize>(exe::ptr::<usize>(0x1_4397_fc78) as usize),
        _ => 0,
    }
}

/// Original: `0x142077180` (61 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_release_pass_com_pair_142077180(this: usize) {
    let p10 = rd::<usize>(this + 0x10);
    let vt10 = rd::<usize>(p10);
    let release10 = rd::<usize>(vt10 + 0x10);
    exe::call_va!(release10, (p10) -> ());
    wr::<usize>(this + 0x8, 0);
    wr::<usize>(this + 0x10, 0);
    let p0 = rd::<usize>(this);
    if p0 != 0 {
        let vt0 = rd::<usize>(p0);
        let release0 = rd::<usize>(vt0 + 0x10);
        exe::call_va!(release0, (p0) -> ());
    }
    wr::<usize>(this, 0);
}

/// Original: `0x142077430` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_pass_item_142077430(
    this: usize,
    b: u32,
    a: u32,
) -> usize {
    wr::<u64>(this, 0);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    wr::<u32>(this + 0x18, a);
    wr::<u32>(this + 0x1c, b);
    this
}

/// Original: `0x142077820` (86 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_pass_triple_142077820(
    this: usize,
    p0: u64,
    p1: u64,
    s0: usize,
    s1: usize,
    s2: usize,
) -> usize {
    wr::<u64>(this, p0);
    wr::<u64>(this + 0x8, p1);
    let mut off = 0usize;
    while off < 0x28 {
        wr::<u64>(this + 0x10 + off, rd::<u64>(s0 + off));
        off += 8;
    }
    let mut off = 0usize;
    while off < 0x28 {
        wr::<u64>(this + 0x38 + off, rd::<u64>(s1 + off));
        off += 8;
    }
    let mut off = 0usize;
    while off < 0x28 {
        wr::<u64>(this + 0x60 + off, rd::<u64>(s2 + off));
        off += 8;
    }
    this
}

/// Original: `0x142077c30` (247 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_pass_half_a_142077c30(
    this: usize,
    mode: i32,
    _unused: usize,
    consts_ptr: usize,
) {
    let mode_idx = mode as isize;

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let state = rd::<usize>(this + 0x10);
    let vt = rd::<usize>(cmd);
    let arg_e8 = rd::<usize>(state + 0x48);
    exe::call_va!(rd::<usize>(vt + 0xe8), (cmd, arg_e8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let pso_off = if mode == 1 { 0xd8usize } else { 0xc8usize };
    let state = rd::<usize>(this + 0x10);
    let vt = rd::<usize>(cmd);
    let arg_c8 = rd::<usize>(state + pso_off);
    exe::call_va!(rd::<usize>(vt + 0xc8), (cmd, arg_c8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr18 = rd::<usize>(this + 0x18);
    let desc0 = rd::<usize>(ptr18);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 0u32, desc0) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr20 = rd::<usize>(this + 0x20);
    let desc1 = rd::<usize>((ptr20 as isize + mode_idx * 8) as usize);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 1u32, desc1) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0x118), (cmd, 2u32, 4u32, consts_ptr, 0u32) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let hdr = rd::<usize>(this);
    let vt = rd::<usize>(cmd);
    let y = rd::<u32>(hdr + 0x254).wrapping_add(1) >> 1;
    let x = rd::<u32>(hdr + 0x250).wrapping_add(1) >> 1;
    exe::call_va!(rd::<usize>(vt + 0x70), (cmd, x, y, 1u32) -> ());
}

/// Original: `0x142077d60` (237 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_pass_full_142077d60(
    this: usize,
    mode: i32,
    _unused: usize,
    consts_ptr: usize,
) {
    let mode_idx = mode as isize;

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let state = rd::<usize>(this + 0x10);
    let vt = rd::<usize>(cmd);
    let arg_e8 = rd::<usize>(state + 0x38);
    exe::call_va!(rd::<usize>(vt + 0xe8), (cmd, arg_e8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let pso_off = if mode == 1 { 0xb8usize } else { 0xa8usize };
    let state = rd::<usize>(this + 0x10);
    let vt = rd::<usize>(cmd);
    let arg_c8 = rd::<usize>(state + pso_off);
    exe::call_va!(rd::<usize>(vt + 0xc8), (cmd, arg_c8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr18 = rd::<usize>(this + 0x18);
    let desc0 = rd::<usize>(ptr18);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 0u32, desc0) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr20 = rd::<usize>(this + 0x20);
    let desc1 = rd::<usize>((ptr20 as isize + mode_idx * 8) as usize);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 1u32, desc1) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0x118), (cmd, 2u32, 4u32, consts_ptr, 0u32) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let hdr = rd::<usize>(this);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let y = rd::<u32>(hdr + 0x254);
    let x = rd::<u32>(hdr + 0x250);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0x70), (cmd, x, y, 1u32) -> ());
}

/// Original: `0x142077e90` (247 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_pass_half_b_142077e90(
    this: usize,
    mode: i32,
    _unused: usize,
    consts_ptr: usize,
) {
    let mode_idx = mode as isize;

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let state = rd::<usize>(this + 0x10);
    let vt = rd::<usize>(cmd);
    let arg_e8 = rd::<usize>(state + 0x48);
    exe::call_va!(rd::<usize>(vt + 0xe8), (cmd, arg_e8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let pso_off = if mode == 1 { 0xf8usize } else { 0xe8usize };
    let state = rd::<usize>(this + 0x10);
    let vt = rd::<usize>(cmd);
    let arg_c8 = rd::<usize>(state + pso_off);
    exe::call_va!(rd::<usize>(vt + 0xc8), (cmd, arg_c8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr18 = rd::<usize>(this + 0x18);
    let desc0 = rd::<usize>(ptr18);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 0u32, desc0) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr20 = rd::<usize>(this + 0x20);
    let desc1 = rd::<usize>((ptr20 as isize + mode_idx * 8) as usize);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 1u32, desc1) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0x118), (cmd, 2u32, 4u32, consts_ptr, 0u32) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let hdr = rd::<usize>(this);
    let vt = rd::<usize>(cmd);
    let y = rd::<u32>(hdr + 0x254).wrapping_add(1) >> 1;
    let x = rd::<u32>(hdr + 0x250).wrapping_add(1) >> 1;
    exe::call_va!(rd::<usize>(vt + 0x70), (cmd, x, y, 1u32) -> ());
}

/// Original: `0x142077fc0` (208 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_pass_mode_a_142077fc0(this: usize, mode: i32) {
    let mode_idx = mode as isize;
    if mode == 2 {
        let hdr = rd::<usize>(this);
        if rd::<u8>(hdr + 0x25e) == 0 {
            return;
        }
    }

    let ctx_wrap = rd::<usize>(this + 0x8);
    let state = rd::<usize>(this + 0x10);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let arg_e8 = rd::<usize>(state + 0x8);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xe8), (cmd, arg_e8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let pso_off = if mode == 1 { 0x78usize } else { 0x58usize };
    let state = rd::<usize>(this + 0x10);
    let vt = rd::<usize>(cmd);
    let arg_c8 = rd::<usize>(state + pso_off);
    exe::call_va!(rd::<usize>(vt + 0xc8), (cmd, arg_c8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr18 = rd::<usize>(this + 0x18);
    let desc0 = rd::<usize>(ptr18);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 0u32, desc0) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr20 = rd::<usize>(this + 0x20);
    let desc1 = rd::<usize>((ptr20 as isize + mode_idx * 8) as usize);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 1u32, desc1) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let hdr = rd::<usize>(this);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let y = rd::<u32>(hdr + 0x254);
    let x = rd::<u32>(hdr + 0x250);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0x70), (cmd, x, y, 1u32) -> ());
}

/// Original: `0x1420780c0` (126 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_pass_1d_1420780c0(this: usize, groups_x: u32) {
    let ctx_wrap = rd::<usize>(this);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let state = rd::<usize>(this + 0x8);
    let vt = rd::<usize>(cmd);
    let arg_e8 = rd::<usize>(state + 0x28);
    exe::call_va!(rd::<usize>(vt + 0xe8), (cmd, arg_e8) -> ());

    let ctx_wrap = rd::<usize>(this);
    let state = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let arg_c8 = rd::<usize>(state + 0x98);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xc8), (cmd, arg_c8) -> ());

    let ctx_wrap = rd::<usize>(this);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr10 = rd::<usize>(this + 0x10);
    let desc0 = rd::<usize>(ptr10);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 0u32, desc0) -> ());

    let ctx_wrap = rd::<usize>(this);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0x70), (cmd, groups_x, 1u32, 1u32) -> ());
}

/// Original: `0x142078180` (282 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dispatch_pass_mode_b_142078180(this: usize, mode: i32) {
    let mode_idx = mode as isize;
    if mode == 2 {
        let hdr = rd::<usize>(this);
        if rd::<u8>(hdr + 0x25e) == 0 {
            return;
        }
    }

    let ctx_wrap = rd::<usize>(this + 0x8);
    let state = rd::<usize>(this + 0x10);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let arg_e8 = rd::<usize>(state + 0x18);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xe8), (cmd, arg_e8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let pso_off = if mode == 1 { 0x88usize } else { 0x68usize };
    let state = rd::<usize>(this + 0x10);
    let vt = rd::<usize>(cmd);
    let arg_c8 = rd::<usize>(state + pso_off);
    exe::call_va!(rd::<usize>(vt + 0xc8), (cmd, arg_c8) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr18 = rd::<usize>(this + 0x18);
    let desc0 = rd::<usize>(ptr18);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 0u32, desc0) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let ptr20 = rd::<usize>(this + 0x20);
    let desc1 = rd::<usize>((ptr20 as isize + mode_idx * 8) as usize);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0xf8), (cmd, 1u32, desc1) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let hdr = rd::<usize>(this);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let vt = rd::<usize>(cmd);
    let fn_108 = rd::<usize>(vt + 0x108);
    let hdr_off = match mode {
        0 => 0x260usize,
        1 => 0x264usize,
        2 => 0x270usize,
        _ => 0x260usize,
    };
    let cval = rd::<u32>(hdr + hdr_off) >> 1;
    exe::call_va!(fn_108, (cmd, 2u32, cval, 0u32) -> ());

    let ctx_wrap = rd::<usize>(this + 0x8);
    let hdr = rd::<usize>(this);
    let cmd = rd::<usize>(ctx_wrap + 0x8);
    let y = rd::<u32>(hdr + 0x254);
    let x = rd::<u32>(hdr + 0x250);
    let vt = rd::<usize>(cmd);
    exe::call_va!(rd::<usize>(vt + 0x70), (cmd, x, y, 1u32) -> ());
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_d3d_find_format_entry_142075e60(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let base = exe::ptr::<u32>(0x1_4297_eda0) as usize;
    let mut checks = 0u32;
    for i in [0usize, 1, 7, 23, 45] {
        let key = rd::<u32>(base + i * 20);
        let o = exe::call_va!(orig, (key) -> u8);
        let n = exe::call_va!(new, (key) -> u8);
        crate::diff_eq!(o, n, "found entry");
        checks += 1;
    }
    for key in [0xffff_fffeu32, 0xdead_beefu32] {
        let o = exe::call_va!(orig, (key) -> u8);
        let n = exe::call_va!(new, (key) -> u8);
        crate::diff_eq!(o, n, "missing entry");
        checks += 1;
    }
    Ok(checks)
}

pub unsafe fn diff_d3d_release_pass_entry_142075f30(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_release_handle(mgr: usize, handle: usize) {
        wr::<usize>(mgr, handle);
    }

    let _m = crate::difftest::Mock::new(0x1_4209_6a90, fake_release_handle as *const ())?;
    let g_mgr = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let saved_mgr = rd::<usize>(g_mgr);

    let mut mgr_o = 0usize;
    let mut mgr_n = 0usize;
    let mut this_o = [0usize; 4];
    let mut this_n = [0usize; 4];
    this_o[3] = 0xdead_beef;
    this_n[3] = 0xdead_beef;

    wr::<usize>(g_mgr, core::ptr::addr_of_mut!(mgr_o) as usize);
    exe::call_va!(orig, (this_o.as_mut_ptr() as usize) -> ());
    wr::<usize>(g_mgr, core::ptr::addr_of_mut!(mgr_n) as usize);
    exe::call_va!(new, (this_n.as_mut_ptr() as usize) -> ());
    wr::<usize>(g_mgr, saved_mgr);

    crate::diff_eq!(this_o, this_n, "release_pass_entry this");
    crate::diff_eq!(mgr_o, mgr_n, "release_pass_entry mgr");
    Ok(2)
}

pub unsafe fn diff_d3d_save_tls_pass_state_142076070(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut fake_tls = vec![0u8; 0x46900];
    for i in 0..0x400 {
        fake_tls[0x463e0 + i] = (i as u8).wrapping_mul(17).wrapping_add(3);
    }
    for i in 0..0x80 {
        fake_tls[0x63d0 + i] = (i as u8).wrapping_mul(29).wrapping_add(11);
    }

    let mut dst_o = vec![0u8; 0x480];
    let mut dst_n = vec![0u8; 0x480];
    let mut buf_obj_o = [dst_o.as_mut_ptr() as u64, 0x500u64];
    let mut buf_obj_n = [dst_n.as_mut_ptr() as u64, 0x500u64];

    let g_extra = exe::ptr::<u32>(0x1_4397_fcb4) as usize;
    let saved_extra = rd::<u32>(g_extra);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    wr::<u32>(g_extra, 0x80);
    wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);

    let ro = exe::call_va!(orig, (0usize, buf_obj_o.as_mut_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (0usize, buf_obj_n.as_mut_ptr() as usize) -> usize);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<u32>(g_extra, saved_extra);

    crate::diff_eq!(
        ro - dst_o.as_ptr() as usize,
        rn - dst_n.as_ptr() as usize,
        "save_tls_pass_state ret offset"
    );
    crate::diff_eq!(dst_o, dst_n, "save_tls_pass_state dst");
    Ok(2)
}

pub unsafe fn diff_d3d_restore_tls_pass_state_142076300(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let g_extra = exe::ptr::<u32>(0x1_4397_fcb4) as usize;
    let saved_extra = rd::<u32>(g_extra);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut src_buf = vec![0u8; 0x480];
    for (i, b) in src_buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(31).wrapping_add(7);
    }
    let holder = [src_buf.as_ptr() as usize];

    let mut checks = 0u32;
    for holder_ptr in [0usize, holder.as_ptr() as usize] {
        let mut this_o = vec![0x5au8; 0xd00];
        let mut this_n = vec![0x5au8; 0xd00];
        wr::<i32>(this_o.as_mut_ptr() as usize + 0xb28, 2);
        wr::<i32>(this_n.as_mut_ptr() as usize + 0xb28, 2);

        let mut tls_o = vec![0x33u8; 0x4b500];
        let mut tls_n = vec![0x33u8; 0x4b500];

        wr::<u32>(g_extra, 0x80);
        wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
        let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize, holder_ptr) -> usize);

        wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
        let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize, holder_ptr) -> usize);

        wr::<usize>(tls_slot, saved_tls0);
        wr::<u32>(g_extra, saved_extra);

        crate::diff_eq!(
            ro - tls_o.as_ptr() as usize,
            rn - tls_n.as_ptr() as usize,
            "restore_tls_pass_state ret offset"
        );
        crate::diff_eq!(this_o, this_n, "restore_tls_pass_state this");
        crate::diff_eq!(tls_o, tls_n, "restore_tls_pass_state tls");
        checks += 1;
    }
    Ok(checks)
}

pub unsafe fn diff_d3d_get_tls_pass_ctx_1420763e0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut fake_tls = [0u8; 0x80];
    fake_tls[0x70] = 1;
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);
    let ro = exe::call_va!(orig, () -> usize);
    let rn = exe::call_va!(new, () -> usize);
    wr::<usize>(tls_slot, saved_tls0);

    crate::diff_eq!(ro, rn, "get_tls_pass_ctx");
    Ok(1)
}

pub unsafe fn diff_d3d_clear_pass_tail_142076150(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf_o = vec![0xA5u8; 0xb00];
    let mut buf_n = vec![0xA5u8; 0xb00];
    let ptr_o = buf_o.as_mut_ptr() as usize + 4;
    let ptr_n = buf_n.as_mut_ptr() as usize + 4;
    exe::call_va!(orig, (ptr_o) -> ());
    exe::call_va!(new, (ptr_n) -> ());
    crate::diff_eq!(buf_o, buf_n, "clear_pass_tail buffer");
    Ok(1)
}

pub unsafe fn diff_d3d_clear_pass_body_142076180(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf_o = vec![0x5Au8; 0xa00];
    let mut buf_n = vec![0x5Au8; 0xa00];
    let ptr_o = buf_o.as_mut_ptr() as usize + 4;
    let ptr_n = buf_n.as_mut_ptr() as usize + 4;
    let ro = exe::call_va!(orig, (ptr_o) -> usize);
    let rn = exe::call_va!(new, (ptr_n) -> usize);
    crate::diff_eq!(ro - ptr_o, rn - ptr_n, "clear_pass_body ret offset");
    crate::diff_eq!(buf_o, buf_n, "clear_pass_body buffer");
    Ok(2)
}

pub unsafe fn diff_d3d_mark_pass_flag_locked_142076920(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf_o = [0u8; 0x60];
    let mut buf_n = [0u8; 0x60];
    let ptr_o = buf_o.as_mut_ptr() as usize;
    let ptr_n = buf_n.as_mut_ptr() as usize;
    exe::call_va!(orig, (ptr_o) -> ());
    exe::call_va!(new, (ptr_n) -> ());
    crate::diff_eq!(buf_o, buf_n, "mark_pass_flag_locked buffer");
    Ok(1)
}

pub unsafe fn diff_d3d_get_pass_global_142076ed0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut checks = 0u32;
    for idx in 0..5u32 {
        let o = exe::call_va!(orig, (idx) -> usize);
        let n = exe::call_va!(new, (idx) -> usize);
        crate::diff_eq!(o, n, "get_pass_global");
        checks += 1;
    }
    Ok(checks)
}

unsafe extern "win64" fn mock_com_release(this: usize) {
    let count = rd::<u64>(this + 0x8);
    wr::<u64>(this + 0x8, count + 1);
}

pub unsafe fn diff_d3d_release_pass_com_pair_142077180(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vt = [0usize; 4];
    vt[2] = mock_com_release as *const () as usize;
    let vt_ptr = vt.as_ptr() as usize;

    let mut com0_o = [vt_ptr, 0usize];
    let mut com1_o = [vt_ptr, 0usize];
    let mut com0_n = [vt_ptr, 0usize];
    let mut com1_n = [vt_ptr, 0usize];

    let mut holder_o = [com0_o.as_mut_ptr() as usize, 0x1234usize, com1_o.as_mut_ptr() as usize];
    let mut holder_n = [com0_n.as_mut_ptr() as usize, 0x1234usize, com1_n.as_mut_ptr() as usize];

    exe::call_va!(orig, (holder_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (holder_n.as_mut_ptr() as usize) -> ());

    crate::diff_eq!(holder_o, holder_n, "holder cleared");
    crate::diff_eq!(com0_o[1], com0_n[1], "com0 release count");
    crate::diff_eq!(com1_o[1], com1_n[1], "com1 release count");
    Ok(3)
}

pub unsafe fn diff_d3d_init_pass_item_142077430(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf_o = [0xFFu8; 0x30];
    let mut buf_n = [0xFFu8; 0x30];
    let ptr_o = buf_o.as_mut_ptr() as usize + 4;
    let ptr_n = buf_n.as_mut_ptr() as usize + 4;
    let ro = exe::call_va!(orig, (ptr_o, 0x1122_3344u32, 0x5566_7788u32) -> usize);
    let rn = exe::call_va!(new, (ptr_n, 0x1122_3344u32, 0x5566_7788u32) -> usize);
    crate::diff_eq!(ro - ptr_o, rn - ptr_n, "init_pass_item ret");
    crate::diff_eq!(buf_o, buf_n, "init_pass_item bytes");
    Ok(2)
}

pub unsafe fn diff_d3d_init_pass_triple_142077820(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf_o = [0xCCu8; 0x98];
    let mut buf_n = [0xCCu8; 0x98];
    let ptr_o = buf_o.as_mut_ptr() as usize + 4;
    let ptr_n = buf_n.as_mut_ptr() as usize + 4;

    let mut s0 = [0u8; 0x28];
    let mut s1 = [0u8; 0x28];
    let mut s2 = [0u8; 0x28];
    for i in 0..0x28 {
        s0[i] = (i as u8).wrapping_add(1);
        s1[i] = (i as u8).wrapping_add(0x30);
        s2[i] = (i as u8).wrapping_add(0x60);
    }

    let ro = exe::call_va!(
        orig,
        (
            ptr_o,
            0x1111_2222_3333_4444u64,
            0x5555_6666_7777_8888u64,
            s0.as_ptr() as usize,
            s1.as_ptr() as usize,
            s2.as_ptr() as usize
        ) -> usize
    );
    let rn = exe::call_va!(
        new,
        (
            ptr_n,
            0x1111_2222_3333_4444u64,
            0x5555_6666_7777_8888u64,
            s0.as_ptr() as usize,
            s1.as_ptr() as usize,
            s2.as_ptr() as usize
        ) -> usize
    );
    crate::diff_eq!(ro - ptr_o, rn - ptr_n, "init_pass_triple ret");
    crate::diff_eq!(buf_o, buf_n, "init_pass_triple bytes");
    Ok(2)
}

unsafe fn mix_cmd_log(cmd: usize, tag: u64, a: u64, b: u64, c: u64, d: u64) {
    let h = rd::<u64>(cmd + 0x8);
    let h = h
        .wrapping_mul(0x1000_0000_01b3)
        .wrapping_add(tag)
        .wrapping_mul(0x1000_0000_01b3)
        .wrapping_add(a)
        .wrapping_mul(0x1000_0000_01b3)
        .wrapping_add(b)
        .wrapping_mul(0x1000_0000_01b3)
        .wrapping_add(c)
        .wrapping_mul(0x1000_0000_01b3)
        .wrapping_add(d);
    wr::<u64>(cmd + 0x8, h);
    let cnt = rd::<u64>(cmd + 0x10);
    wr::<u64>(cmd + 0x10, cnt + 1);
}

unsafe extern "win64" fn mock_cmd_70(cmd: usize, x: u32, y: u32, z: u32) {
    mix_cmd_log(cmd, 0x70, x as u64, y as u64, z as u64, 0);
}

unsafe extern "win64" fn mock_cmd_c8(cmd: usize, pso: usize) {
    mix_cmd_log(cmd, 0xc8, pso as u64, 0, 0, 0);
}

unsafe extern "win64" fn mock_cmd_e8(cmd: usize, sig: usize) {
    mix_cmd_log(cmd, 0xe8, sig as u64, 0, 0, 0);
}

unsafe extern "win64" fn mock_cmd_f8(cmd: usize, slot: u32, desc: usize) {
    mix_cmd_log(cmd, 0xf8, slot as u64, desc as u64, 0, 0);
}

unsafe extern "win64" fn mock_cmd_108(cmd: usize, slot: u32, val: u32, off: u32) {
    mix_cmd_log(cmd, 0x108, slot as u64, val as u64, off as u64, 0);
}

unsafe extern "win64" fn mock_cmd_118(
    cmd: usize,
    slot: u32,
    cnt: u32,
    src: usize,
    off: u32,
) {
    mix_cmd_log(cmd, 0x118, slot as u64, cnt as u64, src as u64, off as u64);
}

struct MockPassEnv {
    vt: [usize; 40],
    cmd: [u64; 3],
    ctx_wrap: [usize; 2],
    hdr: [u8; 0x280],
    state: [usize; 36],
    desc0: [usize; 2],
    desc1: [usize; 4],
    this_buf: [usize; 6],
}

impl MockPassEnv {
    unsafe fn new(flag_25e: u8) -> Box<Self> {
        let mut env = Box::new(Self {
            vt: [0usize; 40],
            cmd: [0u64; 3],
            ctx_wrap: [0usize; 2],
            hdr: [0u8; 0x280],
            state: [0usize; 36],
            desc0: [0x9001usize, 0x9002usize],
            desc1: [0xA000usize, 0xA001usize, 0xA002usize, 0xA003usize],
            this_buf: [0usize; 6],
        });
        env.vt[0x70 / 8] = mock_cmd_70 as *const () as usize;
        env.vt[0xc8 / 8] = mock_cmd_c8 as *const () as usize;
        env.vt[0xe8 / 8] = mock_cmd_e8 as *const () as usize;
        env.vt[0xf8 / 8] = mock_cmd_f8 as *const () as usize;
        env.vt[0x108 / 8] = mock_cmd_108 as *const () as usize;
        env.vt[0x118 / 8] = mock_cmd_118 as *const () as usize;

        env.cmd[0] = env.vt.as_ptr() as usize as u64;
        env.cmd[1] = 0xcbf2_9ce4_8422_2325;
        env.cmd[2] = 0;

        env.ctx_wrap[1] = env.cmd.as_mut_ptr() as usize;

        let hdr_ptr = env.hdr.as_mut_ptr() as usize;
        wr::<u32>(hdr_ptr + 0x250, 1921);
        wr::<u32>(hdr_ptr + 0x254, 1081);
        wr::<u8>(hdr_ptr + 0x25e, flag_25e);
        wr::<u32>(hdr_ptr + 0x260, 400);
        wr::<u32>(hdr_ptr + 0x264, 600);
        wr::<u32>(hdr_ptr + 0x270, 800);

        for i in 0..env.state.len() {
            env.state[i] = 0x5000 + i * 0x11;
        }

        env.this_buf[0] = hdr_ptr;
        env.this_buf[1] = env.ctx_wrap.as_mut_ptr() as usize;
        env.this_buf[2] = env.state.as_mut_ptr() as usize;
        env.this_buf[3] = env.desc0.as_mut_ptr() as usize;
        env.this_buf[4] = env.desc1.as_mut_ptr() as usize;
        env
    }
}

pub unsafe fn diff_d3d_dispatch_pass_half_a_142077c30(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut checks = 0u32;
    for mode in [0i32, 1i32] {
        let mut eo = MockPassEnv::new(1);
        let mut en = MockPassEnv::new(1);
        exe::call_va!(orig, (eo.this_buf.as_mut_ptr() as usize, mode, 0usize, 0x7777usize) -> ());
        exe::call_va!(new, (en.this_buf.as_mut_ptr() as usize, mode, 0usize, 0x7777usize) -> ());
        crate::diff_eq!(eo.cmd[1], en.cmd[1], "half_a hash");
        crate::diff_eq!(eo.cmd[2], en.cmd[2], "half_a call count");
        checks += 1;
    }
    Ok(checks)
}

pub unsafe fn diff_d3d_dispatch_pass_full_142077d60(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut checks = 0u32;
    for mode in [0i32, 1i32] {
        let mut eo = MockPassEnv::new(1);
        let mut en = MockPassEnv::new(1);
        exe::call_va!(orig, (eo.this_buf.as_mut_ptr() as usize, mode, 0usize, 0x8888usize) -> ());
        exe::call_va!(new, (en.this_buf.as_mut_ptr() as usize, mode, 0usize, 0x8888usize) -> ());
        crate::diff_eq!(eo.cmd[1], en.cmd[1], "full hash");
        crate::diff_eq!(eo.cmd[2], en.cmd[2], "full call count");
        checks += 1;
    }
    Ok(checks)
}

pub unsafe fn diff_d3d_dispatch_pass_half_b_142077e90(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut checks = 0u32;
    for mode in [0i32, 1i32] {
        let mut eo = MockPassEnv::new(1);
        let mut en = MockPassEnv::new(1);
        exe::call_va!(orig, (eo.this_buf.as_mut_ptr() as usize, mode, 0usize, 0x9999usize) -> ());
        exe::call_va!(new, (en.this_buf.as_mut_ptr() as usize, mode, 0usize, 0x9999usize) -> ());
        crate::diff_eq!(eo.cmd[1], en.cmd[1], "half_b hash");
        crate::diff_eq!(eo.cmd[2], en.cmd[2], "half_b call count");
        checks += 1;
    }
    Ok(checks)
}

pub unsafe fn diff_d3d_dispatch_pass_mode_a_142077fc0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut checks = 0u32;
    for (mode, flag) in [(0i32, 0u8), (1i32, 0u8), (2i32, 0u8), (2i32, 1u8)] {
        let mut eo = MockPassEnv::new(flag);
        let mut en = MockPassEnv::new(flag);
        exe::call_va!(orig, (eo.this_buf.as_mut_ptr() as usize, mode) -> ());
        exe::call_va!(new, (en.this_buf.as_mut_ptr() as usize, mode) -> ());
        crate::diff_eq!(eo.cmd[1], en.cmd[1], "mode_a hash");
        crate::diff_eq!(eo.cmd[2], en.cmd[2], "mode_a call count");
        checks += 1;
    }
    Ok(checks)
}

pub unsafe fn diff_d3d_dispatch_pass_1d_1420780c0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut eo = MockPassEnv::new(1);
    let mut en = MockPassEnv::new(1);
    let this_o = [
        eo.ctx_wrap.as_mut_ptr() as usize,
        eo.state.as_mut_ptr() as usize,
        eo.desc0.as_mut_ptr() as usize,
    ];
    let this_n = [
        en.ctx_wrap.as_mut_ptr() as usize,
        en.state.as_mut_ptr() as usize,
        en.desc0.as_mut_ptr() as usize,
    ];
    exe::call_va!(orig, (this_o.as_ptr() as usize, 64u32) -> ());
    exe::call_va!(new, (this_n.as_ptr() as usize, 64u32) -> ());
    crate::diff_eq!(eo.cmd[1], en.cmd[1], "pass_1d hash");
    crate::diff_eq!(eo.cmd[2], en.cmd[2], "pass_1d call count");
    Ok(1)
}

pub unsafe fn diff_d3d_dispatch_pass_mode_b_142078180(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut checks = 0u32;
    for (mode, flag) in [(0i32, 0u8), (1i32, 0u8), (2i32, 0u8), (2i32, 1u8)] {
        let mut eo = MockPassEnv::new(flag);
        let mut en = MockPassEnv::new(flag);
        exe::call_va!(orig, (eo.this_buf.as_mut_ptr() as usize, mode) -> ());
        exe::call_va!(new, (en.this_buf.as_mut_ptr() as usize, mode) -> ());
        crate::diff_eq!(eo.cmd[1], en.cmd[1], "mode_b hash");
        crate::diff_eq!(eo.cmd[2], en.cmd[2], "mode_b call count");
        checks += 1;
    }
    Ok(checks)
}

// REGISTRY: Replacement::verified(0x142075f30, "d3d::d3d_release_pass_entry_142075f30_142075f30", crate::ns::rend::d3d_render_pass::d3d_release_pass_entry_142075f30 as *const (), crate::ns::rend::d3d_render_pass::diff_d3d_release_pass_entry_142075f30),
// REGISTRY: Replacement::verified(0x142076070, "d3d::d3d_save_tls_pass_state_142076070_142076070", crate::ns::rend::d3d_render_pass::d3d_save_tls_pass_state_142076070 as *const (), crate::ns::rend::d3d_render_pass::diff_d3d_save_tls_pass_state_142076070),
// REGISTRY: Replacement::verified(0x142076300, "d3d::d3d_restore_tls_pass_state_142076300_142076300", crate::ns::rend::d3d_render_pass::d3d_restore_tls_pass_state_142076300 as *const (), crate::ns::rend::d3d_render_pass::diff_d3d_restore_tls_pass_state_142076300),
// REGISTRY: Replacement::verified(0x1420763e0, "d3d::d3d_get_tls_pass_ctx_1420763e0_1420763e0", crate::ns::rend::d3d_render_pass::d3d_get_tls_pass_ctx_1420763e0 as *const (), crate::ns::rend::d3d_render_pass::diff_d3d_get_tls_pass_ctx_1420763e0),

