//! `d3d` render target setup, sampler cache, GPU profiler, NRD denoiser glue, staging texture pool,
//! shader table handles, and fatal GPU error reporting (`0x1420549b0..0x14205d410`).

use crate::exe::{self, rd, wr};

// ============================================================================
// 1. Render target setup & global state helpers (0x142054c80..0x142055890)
// ============================================================================

/// Zero-initialize a 280-byte (`0x118`) render-target / UAV binding setup struct at `rcx`.
///
/// Original: `0x142054c80` (198 bytes, leaf, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_render_target_setup(this: usize) {
    let mut off = 0x08usize;
    while off <= 0x100 {
        wr::<u64>(this + off, 0);
        off += 8;
    }
    wr::<u32>(this, 0);
    wr::<u64>(this + 0x108, 0);
    wr::<u64>(this + 0x110, 0);
}

/// Initialize a render-target setup struct at `this` (`rcx`) with `count` (`r8d`) RTV pointers
/// from `rtvs` (`rdx`) at `+0x08..+0x48` and depth-stencil pointer `dsv` (`r9`) at `+0x108`.
///
/// Original: `0x142054d60` (67 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_render_target_setup_rtvs(
    this: usize,
    rtvs: usize,
    count: i32,
    dsv: usize,
) -> usize {
    d3d_clear_render_target_setup(this);
    wr::<i32>(this, count);
    let mut i = 0i32;
    while i < rd::<i32>(this) {
        let val = rd::<u64>(rtvs + (i as usize) * 8);
        wr::<u64>(this + 0x08 + (i as usize) * 8, val);
        i += 1;
    }
    wr::<usize>(this + 0x108, dsv);
    this
}

/// Initialize a render-target setup struct at `this` (`rcx`) with a single pointer `target` (`rdx`)
/// at `+0x88`, count `1` at `+0x00`, and `0` at `+0x108`.
///
/// Original: `0x142054db0` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_render_target_setup_single_88(
    this: usize,
    target: usize,
) -> usize {
    d3d_clear_render_target_setup(this);
    wr::<u32>(this, 1);
    wr::<usize>(this + 0x88, target);
    wr::<u64>(this + 0x108, 0);
    this
}

/// Initialize a render-target setup struct at `this` (`rcx`) with `count` (`r8d`) pointers
/// from `views` (`rdx`) at `+0xc8..+0x108` and depth-stencil pointer `dsv` (`r9`) at `+0x108`.
///
/// Original: `0x142054e10` (70 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_render_target_setup_c8(
    this: usize,
    views: usize,
    count: i32,
    dsv: usize,
) -> usize {
    d3d_clear_render_target_setup(this);
    wr::<i32>(this, count);
    let mut i = 0i32;
    while i < rd::<i32>(this) {
        let val = rd::<u64>(views + (i as usize) * 8);
        wr::<u64>(this + 0xc8 + (i as usize) * 8, val);
        i += 1;
    }
    wr::<usize>(this + 0x108, dsv);
    this
}

/// Destroy the global D3D helper object at `0x14397fba0` if non-null and clear the pointer.
///
/// Original: `0x142054ea0` (112 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_destroy_global_397fba0() {
    let g_va = exe::ptr::<usize>(0x1439_7fba0) as usize;
    let rbx = rd::<usize>(g_va);
    if rbx != 0 {
        let dtor_fn = exe::ptr::<u8>(0x1411_db190) as usize;
        exe::call_va!(0x1418_34ee8, (rbx + 0x08, 8u32, 2u32, dtor_fn));
        let rdi = rd::<usize>(rbx);
        if rdi != 0 {
            exe::call_va!(0x1420_9d670, (rdi));
            exe::call_va!(0x1411_e7fc0, (rdi + 0x58));
            exe::call_va!(0x1424_42de0, (rdi));
        }
        exe::call_va!(0x1424_42de0, (rbx));
    }
    wr::<usize>(g_va, 0);
}

/// Allocate and initialize the global D3D helper object at `0x14397fba0`.
///
/// Original: `0x142054f20` (21 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_global_397fba0() {
    let ptr = exe::call_va!(0x1411_d4380, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1439_7fba0) as usize, ptr);
}

/// Acquire a recursive critical section `(cs_va, tid_va, count_va)`.
#[inline]
unsafe fn recursive_lock_acquire(cs_va: u64, tid_va: u64, count_va: u64) {
    let cs = exe::ptr::<u8>(cs_va) as usize;
    let tid_ptr = exe::ptr::<u32>(tid_va) as usize;
    let count_ptr = exe::ptr::<u32>(count_va) as usize;
    let cur_tid = exe::call_iat!(0x1426_45790, () -> u32);
    if rd::<u32>(tid_ptr) != cur_tid {
        exe::call_iat!(0x1426_455a8, (cs));
        wr::<u32>(tid_ptr, cur_tid);
    }
    wr::<u32>(count_ptr, rd::<u32>(count_ptr).wrapping_add(1));
}

/// Release a recursive critical section `(cs_va, tid_va, count_va)`.
#[inline]
unsafe fn recursive_lock_release(cs_va: u64, tid_va: u64, count_va: u64) {
    let cs = exe::ptr::<u8>(cs_va) as usize;
    let tid_ptr = exe::ptr::<u32>(tid_va) as usize;
    let count_ptr = exe::ptr::<u32>(count_va) as usize;
    let next = rd::<u32>(count_ptr).wrapping_sub(1);
    wr::<u32>(count_ptr, next);
    if next == 0 {
        wr::<u32>(tid_ptr, u32::MAX);
        exe::call_iat!(0x1426_45598, (cs));
    }
}

/// Destroy all cached sampler states in global array `0x143a631c8` under lock `0x1438579b0`.
///
/// Original: `0x142055320` (208 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_sampler_state_cache() {
    recursive_lock_acquire(0x1438_579b0, 0x1438_579b8, 0x1438_579bc);
    let arr_va = exe::ptr::<usize>(0x143a_631c8) as usize;
    let len_va = exe::ptr::<u32>(0x143a_631d0) as usize;
    let mut rdx = rd::<usize>(arr_va);
    let mut r8d = rd::<u32>(len_va);
    let mut rbx = rdx;
    if rdx != rdx.wrapping_add((r8d as usize) * 8) {
        loop {
            let rdi = rd::<usize>(rbx);
            if rdi != 0 {
                exe::call_va!(0x1411_d44d0, (rdi));
                exe::call_va!(0x1424_42de0, (rdi));
                r8d = rd::<u32>(len_va);
                rdx = rd::<usize>(arr_va);
            }
            rbx = rbx.wrapping_add(8);
            if rbx == rdx.wrapping_add((r8d as usize) * 8) {
                break;
            }
        }
    }
    exe::call_va!(0x1401_de830, (arr_va));
    recursive_lock_release(0x1438_579b0, 0x1438_579b8, 0x1438_579bc);
}

/// Reset length at `this + 0x8` to 0 and release the buffer at `this`.
///
/// Original: `0x142055410` (22 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_and_release_array_at_0(this: usize) {
    wr::<u32>(this + 0x08, 0);
    exe::call_va!(0x1401_de830, (this));
}

/// Create the 12 default global sampler state objects (`0x14397fba8..0x14397fc00`).
///
/// Original: `0x142055620` (522 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_default_sampler_states() {
    const TABLE: [(u32, u32, u16, u64); 12] = [
        (0, 0, 0x0000, 0x1439_7fbf8),
        (0, 1, 0x0000, 0x1439_7fbb0),
        (0, 2, 0x0000, 0x1439_7fbe8),
        (0, 0, 0x0001, 0x1439_7fbc0),
        (0, 1, 0x0001, 0x1439_7fbb8),
        (0, 2, 0x0001, 0x1439_7fbc8),
        (1, 0, 0x0000, 0x1439_7fba8),
        (1, 1, 0x0000, 0x1439_7fbd8),
        (1, 2, 0x0000, 0x1439_7fbd0),
        (0, 0, 0x0100, 0x1439_7fc00),
        (0, 1, 0x0100, 0x1439_7fbe0),
        (0, 2, 0x0100, 0x1439_7fbf0),
    ];
    let mut desc = [0u8; 24];
    let desc_ptr = desc.as_mut_ptr() as usize;
    for &(filter, addr_mode, flags, global_va) in &TABLE {
        wr::<u32>(desc_ptr, filter);
        wr::<u32>(desc_ptr + 0x04, addr_mode);
        wr::<u16>(desc_ptr + 0x08, flags);
        wr::<f32>(desc_ptr + 0x0c, 0.0);
        wr::<f32>(desc_ptr + 0x10, 0.0);
        let obj = exe::call_va!(0x1411_d4450, (0usize, 0usize, desc_ptr) -> usize);
        wr::<usize>(exe::ptr::<usize>(global_va) as usize, obj);
    }
}

/// Clear the TLS-cached bound object at `tls0 + 0x467e0 + 0xb10` if it matches `obj` (`rcx`) and is null.
///
/// Original: `0x142055830` (90 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_tls_bound_object_if_eq(obj: usize) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1418_349c0, ());
    }
    let ctx = tls0 + 0x467e0;
    let cur = rd::<usize>(ctx + 0xb10);
    if cur == obj && cur == 0 {
        wr::<usize>(ctx + 0xb10, 0);
        wr::<u8>(ctx + 0xa80, 0);
    }
}

/// Compute a 64-bit hash of the 20-byte sampler descriptor at `desc` (`rcx`).
///
/// Original: `0x142055890` (433 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_hash_sampler_desc(desc: usize) -> u64 {
    const K0: u64 = 0x9ae1_6a3b_2f90_4057; // -0x651e95c4d06fbfa9
    const K_MUL: u64 = 0x9ddf_ea08_eb38_2d69; // -0x622015f714c7d297
    const K_ADD: u64 = 0x651e_95c4_d06f_bfb1;

    let r8 = rd::<u32>(desc) as u64;
    let rcx_v = rd::<u32>(desc + 0x04) as u64;

    let rax0 = ((r8 << 3).wrapping_add(4) ^ r8).wrapping_mul(K0);
    let rdx0 = ((rax0 >> 47) ^ rax0 ^ r8).wrapping_mul(K0);
    let r8_h = ((rdx0 >> 47) ^ rdx0).wrapping_mul(K0);

    let rax1 = ((rcx_v << 3).wrapping_add(4) ^ rcx_v).wrapping_mul(K0);
    let rdx1 = ((rax1 >> 47) ^ rax1 ^ rcx_v).wrapping_mul(K0);
    let rcx1 = ((rdx1 >> 47) ^ rdx1).wrapping_mul(K0);

    let rcx2 = (rcx1.wrapping_add(K_ADD) ^ r8_h).wrapping_mul(K_MUL);
    let rax2 = ((rcx2 >> 47) ^ rcx2 ^ r8_h).wrapping_mul(K_MUL);
    let mut rbx = ((rax2 >> 47) ^ rax2).wrapping_mul(K_MUL);

    for (off, len) in [(0x08usize, 1u64), (0x09, 1), (0x0c, 4), (0x10, 4)] {
        let h = exe::call_va!(0x1424_b16a0, (desc + off, len) -> u64);
        let a = (h.wrapping_add(K_ADD) ^ rbx).wrapping_mul(K_MUL);
        let c = ((a >> 47) ^ a ^ rbx).wrapping_mul(K_MUL);
        rbx = ((c >> 47) ^ c).wrapping_mul(K_MUL);
    }
    rbx
}

// ============================================================================
// 2. GPU profiler & timer pool (0x142055a60..0x1420562d0)
// ============================================================================

/// Look up or register the `"Streamed Texture Defragment"` profiler event ID in `0x143a631d8`
/// and return the address of global `0x143ce6878`.
///
/// Original: `0x142055a60` (472 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_get_streamed_texture_defragment_id() -> usize {
    let out_id_va = exe::ptr::<u32>(0x143c_e6878) as usize;
    wr::<u32>(out_id_va, 0);
    recursive_lock_acquire(0x1438_579f0, 0x1438_579f8, 0x1438_579fc);

    let mut frame = [0u64; 56]; // 0x1c0 bytes (covers rbp + 0xa0 = rsp + 0x1a0)
    let rsp = frame.as_mut_ptr() as usize;
    let rbp = rsp + 0x100;

    let key_str = rbp - 0x20;
    wr::<u64>(key_str, 0x1000_0010);
    wr::<u8>(key_str + 0x08, 0);
    let name_lit = exe::ptr::<u8>(0x1433_dce38) as usize; // "Streamed Texture Defragment"
    wr::<usize>(rsp + 0x20, name_lit);
    wr::<u64>(rsp + 0x28, 0x1b);
    exe::call_va!(0x1424_09b50, (key_str, rsp + 0x20, 0u32, 0x1bu32));

    let tree_va = exe::ptr::<usize>(0x143a_631d8) as usize;
    let rsi = rd::<usize>(tree_va);
    let mut rbx = rd::<usize>(rsi + 0x08);
    let mut rdi = rsi;
    while rd::<u8>(rbx + 0x19) == 0 {
        let less = exe::call_va!(0x1402_174f0, (rbx + 0x1c, key_str) -> bool);
        if less {
            rbx = rd::<usize>(rbx + 0x10);
        } else {
            rdi = rbx;
            rbx = rd::<usize>(rbx);
        }
    }
    if rd::<u8>(rdi + 0x19) == 0 {
        let less = exe::call_va!(0x1402_174f0, (key_str, rdi + 0x1c) -> bool);
        if less {
            rdi = rsi;
        }
    }
    exe::call_va!(0x1424_0ab10, (key_str));

    if rdi == rd::<usize>(tree_va) {
        let new_id = rd::<u32>(exe::ptr::<u32>(0x143a_631e0) as usize).wrapping_add(1);
        wr::<u32>(out_id_va, new_id);
        let pair_str = rsp + 0x40;
        wr::<u64>(rbp + 0xa0, pair_str as u64);
        wr::<u32>(pair_str, 0x1000_0010);
        wr::<u32>(pair_str + 0x04, 0);
        wr::<u8>(pair_str + 0x08, 0);
        wr::<usize>(rsp + 0x30, name_lit);
        wr::<u64>(rsp + 0x38, 0x1b);
        exe::call_va!(0x1424_09b50, (pair_str, rsp + 0x30, 0u32, 0x1bu32));
        wr::<u32>(rbp - 0x38, rd::<u32>(out_id_va));
        exe::call_va!(0x1411_d4eb0, (tree_va, rbp - 0x30, pair_str));
        exe::call_va!(0x1424_0ab10, (pair_str));
    } else {
        wr::<u32>(out_id_va, rd::<u32>(rdi + 0xa4));
    }

    recursive_lock_release(0x1438_579f0, 0x1438_579f8, 0x1438_579fc);
    out_id_va
}

// ============================================================================
// Differential tests
// ============================================================================

pub unsafe fn diff_d3d_clear_render_target_setup(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut buf_o = [0x5au8; 0x118];
    let mut buf_n = [0x5au8; 0x118];
    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(&buf_o[..], &buf_n[..], "clear_rt_setup");
    Ok(1)
}

pub unsafe fn diff_d3d_init_render_target_setup_rtvs(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let rtvs: [u64; 3] = [0x1111, 0x2222, 0x3333];
    let mut buf_o = [0xffu8; 0x118];
    let mut buf_n = [0xffu8; 0x118];
    f_orig(buf_o.as_mut_ptr() as usize, rtvs.as_ptr() as usize, 3, 0x9999);
    f_new(buf_n.as_mut_ptr() as usize, rtvs.as_ptr() as usize, 3, 0x9999);
    crate::diff_eq!(&buf_o[..], &buf_n[..], "init_rt_setup_rtvs");
    Ok(1)
}

pub unsafe fn diff_d3d_init_render_target_setup_single_88(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut buf_o = [0xffu8; 0x118];
    let mut buf_n = [0xffu8; 0x118];
    f_orig(buf_o.as_mut_ptr() as usize, 0x123456);
    f_new(buf_n.as_mut_ptr() as usize, 0x123456);
    crate::diff_eq!(&buf_o[..], &buf_n[..], "init_rt_setup_88");
    Ok(1)
}

pub unsafe fn diff_d3d_init_render_target_setup_c8(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let views: [u64; 2] = [0xaaaa, 0xbbbb];
    let mut buf_o = [0xffu8; 0x118];
    let mut buf_n = [0xffu8; 0x118];
    f_orig(buf_o.as_mut_ptr() as usize, views.as_ptr() as usize, 2, 0x7777);
    f_new(buf_n.as_mut_ptr() as usize, views.as_ptr() as usize, 2, 0x7777);
    crate::diff_eq!(&buf_o[..], &buf_n[..], "init_rt_setup_c8");
    Ok(1)
}

pub unsafe fn diff_d3d_hash_sampler_desc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    let mut desc = [0u8; 24];
    let ptr = desc.as_mut_ptr() as usize;
    wr::<u32>(ptr, 1);
    wr::<u32>(ptr + 0x04, 2);
    wr::<u16>(ptr + 0x08, 0x0101);
    wr::<f32>(ptr + 0x0c, -0.5);
    wr::<f32>(ptr + 0x10, 16.0);
    let ro = f_orig(ptr);
    let rn = f_new(ptr);
    crate::diff_eq!(ro, rn, "hash_sampler_desc");
    Ok(1)
}

unsafe extern "win64" fn fake_eh_vec_dtor_34ee8(
    ptr: usize,
    size: u32,
    count: u32,
    dtor: usize,
) {
    wr::<u32>(ptr, size);
    wr::<u32>(ptr + 0x04, count);
    wr::<usize>(ptr + 0x08, dtor);
}

unsafe extern "win64" fn fake_dtor_9d670(rdi: usize) {
    wr::<u64>(rdi, 0xaaaa_1111);
}

unsafe extern "win64" fn fake_dtor_e7fc0(p58: usize) {
    wr::<u64>(p58, 0xbbbb_2222);
}

unsafe extern "win64" fn fake_free_42de0(p: usize) {
    wr::<u64>(p + 0x20, rd::<u64>(p + 0x20).wrapping_add(0x77));
}

pub unsafe fn diff_d3d_destroy_global_397fba0(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m1 = Mock::new(0x1418_34ee8, fake_eh_vec_dtor_34ee8 as *const ())?;
    let _m2 = Mock::new(0x1420_9d670, fake_dtor_9d670 as *const ())?;
    let _m3 = Mock::new(0x1411_e7fc0, fake_dtor_e7fc0 as *const ())?;
    let _m4 = Mock::new(0x1424_42de0, fake_free_42de0 as *const ())?;

    let g_va = exe::ptr::<usize>(0x1439_7fba0) as usize;
    let snap = Snapshot::take(g_va as *mut u8, 8);

    wr::<usize>(g_va, 0);
    exe::call_va!(orig, () -> ());
    let null_o = rd::<usize>(g_va);
    wr::<usize>(g_va, 0);
    exe::call_va!(new, () -> ());
    let null_n = rd::<usize>(g_va);

    let mut rdi_o = [0u64; 16];
    let mut rdi_n = [0u64; 16];
    let mut rbx_o = [0u64; 8];
    let mut rbx_n = [0u64; 8];
    rbx_o[0] = rdi_o.as_mut_ptr() as usize as u64;
    rbx_n[0] = rdi_n.as_mut_ptr() as usize as u64;

    wr::<usize>(g_va, rbx_o.as_mut_ptr() as usize);
    exe::call_va!(orig, () -> ());
    let after_o = rd::<usize>(g_va);

    wr::<usize>(g_va, rbx_n.as_mut_ptr() as usize);
    exe::call_va!(new, () -> ());
    let after_n = rd::<usize>(g_va);

    snap.restore();

    crate::diff_eq!(null_o, null_n, "destroy_global_397fba0 null");
    crate::diff_eq!(after_o, after_n, "destroy_global_397fba0 cleared");
    crate::diff_eq!(&rbx_o[1..], &rbx_n[1..], "destroy_global_397fba0 rbx");
    crate::diff_eq!(rdi_o, rdi_n, "destroy_global_397fba0 rdi");
    Ok(2)
}

unsafe extern "win64" fn fake_alloc_d4380() -> usize {
    0xdead_beef_1234_5678
}

pub unsafe fn diff_d3d_init_global_397fba0(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m = Mock::new(0x1411_d4380, fake_alloc_d4380 as *const ())?;
    let g_va = exe::ptr::<usize>(0x1439_7fba0) as usize;
    let snap = Snapshot::take(g_va as *mut u8, 8);

    wr::<usize>(g_va, 0);
    exe::call_va!(orig, () -> ());
    let v_o = rd::<usize>(g_va);

    wr::<usize>(g_va, 0);
    exe::call_va!(new, () -> ());
    let v_n = rd::<usize>(g_va);

    snap.restore();
    crate::diff_eq!(v_o, v_n, "init_global_397fba0");
    Ok(1)
}

unsafe extern "win64" fn fake_sampler_dtor_d44d0(rdi: usize) {
    wr::<u64>(rdi, rd::<u64>(rdi).wrapping_add(0x10));
}

unsafe extern "win64" fn fake_release_buf_de830(this: usize) {
    wr::<u32>(this + 0x0c, rd::<u32>(this + 0x08).wrapping_add(0x1234));
    wr::<usize>(this, 0);
}

pub unsafe fn diff_d3d_clear_sampler_state_cache(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m1 = Mock::new(0x1411_d44d0, fake_sampler_dtor_d44d0 as *const ())?;
    let _m2 = Mock::new(0x1424_42de0, fake_free_42de0 as *const ())?;
    let _m3 = Mock::new(0x1401_de830, fake_release_buf_de830 as *const ())?;

    let lock_va = exe::ptr::<u8>(0x1438_579b0) as usize;
    let arr_va = exe::ptr::<u8>(0x143a_631c8) as usize;
    let snap_lock = Snapshot::take(lock_va as *mut u8, 16);
    let snap_arr = Snapshot::take(arr_va as *mut u8, 16);

    let mut item0_o = [1u64; 6];
    let mut item1_o = [2u64; 6];
    let mut items_o = [
        item0_o.as_mut_ptr() as usize,
        0usize,
        item1_o.as_mut_ptr() as usize,
    ];
    wr::<u64>(lock_va, 0);
    wr::<u32>(lock_va + 0x08, u32::MAX);
    wr::<u32>(lock_va + 0x0c, 0);
    wr::<usize>(arr_va, items_o.as_mut_ptr() as usize);
    wr::<u32>(arr_va + 0x08, 3);
    wr::<u32>(arr_va + 0x0c, 3);
    exe::call_va!(orig, () -> ());
    let lock_o = snap_lock.current();
    let arr_hdr_o = snap_arr.current();

    let mut item0_n = [1u64; 6];
    let mut item1_n = [2u64; 6];
    let mut items_n = [
        item0_n.as_mut_ptr() as usize,
        0usize,
        item1_n.as_mut_ptr() as usize,
    ];
    wr::<u64>(lock_va, 0);
    wr::<u32>(lock_va + 0x08, u32::MAX);
    wr::<u32>(lock_va + 0x0c, 0);
    wr::<usize>(arr_va, items_n.as_mut_ptr() as usize);
    wr::<u32>(arr_va + 0x08, 3);
    wr::<u32>(arr_va + 0x0c, 3);
    exe::call_va!(new, () -> ());
    let lock_n = snap_lock.current();
    let arr_hdr_n = snap_arr.current();

    snap_arr.restore();
    snap_lock.restore();

    crate::diff_eq!(item0_o, item0_n, "clear_sampler_state_cache item0");
    crate::diff_eq!(item1_o, item1_n, "clear_sampler_state_cache item1");
    crate::diff_eq!(lock_o, lock_n, "clear_sampler_state_cache lock");
    crate::diff_eq!(arr_hdr_o, arr_hdr_n, "clear_sampler_state_cache arr_hdr");
    Ok(1)
}

pub unsafe fn diff_d3d_clear_and_release_array_at_0(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use crate::difftest::Mock;
    let _m = Mock::new(0x1401_de830, fake_release_buf_de830 as *const ())?;

    let mut buf_o = [0x1111_2222u64, 0x0000_0008_0000_0005u64];
    let mut buf_n = buf_o;
    exe::call_va!(orig, (buf_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (buf_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(buf_o, buf_n, "clear_and_release_array_at_0");
    Ok(1)
}

unsafe extern "win64" fn fake_create_sampler_d4450(_a: usize, _b: usize, desc: usize) -> usize {
    let f = rd::<u32>(desc) as usize;
    let m = rd::<u32>(desc + 0x04) as usize;
    let fl = rd::<u16>(desc + 0x08) as usize;
    let c0 = rd::<u32>(desc + 0x0c) as usize;
    let c1 = rd::<u32>(desc + 0x10) as usize;
    0x5000_0000usize
        .wrapping_add(f << 24)
        .wrapping_add(m << 16)
        .wrapping_add(fl)
        .wrapping_add(c0 ^ c1)
}

pub unsafe fn diff_d3d_init_default_sampler_states(orig: usize, new: usize) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m = Mock::new(0x1411_d4450, fake_create_sampler_d4450 as *const ())?;
    let base_va = exe::ptr::<u8>(0x1439_7fba8) as usize;
    let snap = Snapshot::take(base_va as *mut u8, 12 * 8);

    std::ptr::write_bytes(base_va as *mut u8, 0, 12 * 8);
    exe::call_va!(orig, () -> ());
    let out_o = snap.current();

    std::ptr::write_bytes(base_va as *mut u8, 0, 12 * 8);
    exe::call_va!(new, () -> ());
    let out_n = snap.current();

    snap.restore();
    crate::diff_eq!(out_o, out_n, "init_default_sampler_states");
    Ok(1)
}

pub unsafe fn diff_d3d_clear_tls_bound_object_if_eq(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut tls_o = vec![0u8; 0x47300];
    let mut tls_n = vec![0u8; 0x47300];
    let po = tls_o.as_mut_ptr() as usize;
    let pn = tls_n.as_mut_ptr() as usize;
    wr::<u8>(po + 0x70, 1);
    wr::<u8>(pn + 0x70, 1);

    for (cur_val, arg_obj) in [(0usize, 0usize), (0x1234, 0x1234), (0, 0x1234)] {
        wr::<u8>(po + 0x467e0 + 0xa80, 0x5a);
        wr::<usize>(po + 0x467e0 + 0xb10, cur_val);
        wr::<usize>(tls_slot, po);
        exe::call_va!(orig, (arg_obj) -> ());

        wr::<u8>(pn + 0x467e0 + 0xa80, 0x5a);
        wr::<usize>(pn + 0x467e0 + 0xb10, cur_val);
        wr::<usize>(tls_slot, pn);
        exe::call_va!(new, (arg_obj) -> ());

        if rd::<u8>(po + 0x467e0 + 0xa80) != rd::<u8>(pn + 0x467e0 + 0xa80)
            || rd::<usize>(po + 0x467e0 + 0xb10) != rd::<usize>(pn + 0x467e0 + 0xb10)
        {
            wr::<usize>(tls_slot, saved_tls0);
            return Err(format!(
                "clear_tls_bound_object_if_eq mismatch for cur={cur_val} arg={arg_obj}"
            ));
        }
    }

    wr::<usize>(tls_slot, saved_tls0);
    Ok(3)
}

unsafe extern "win64" fn fake_str_assign_09b50(s: usize, view: usize, _z: u32, len: u32) {
    wr::<u64>(s + 0x08, rd::<u64>(view));
    wr::<u32>(s + 0x10, len);
}

unsafe extern "win64" fn fake_str_less_174f0(_a: usize, _b: usize) -> u8 {
    0
}

unsafe extern "win64" fn fake_str_dtor_0ab10(s: usize) {
    wr::<u64>(s, 0);
}

unsafe extern "win64" fn fake_tree_insert_d4eb0(_tree: usize, _out: usize, pair_str: usize) {
    let id = rd::<u32>(pair_str + 0x88);
    wr::<u32>(exe::ptr::<u32>(0x143a_631e0) as usize, id);
}

pub unsafe fn diff_d3d_get_streamed_texture_defragment_id(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use crate::difftest::{Mock, Snapshot};
    let _m1 = Mock::new(0x1424_09b50, fake_str_assign_09b50 as *const ())?;
    let _m2 = Mock::new(0x1402_174f0, fake_str_less_174f0 as *const ())?;
    let _m3 = Mock::new(0x1424_0ab10, fake_str_dtor_0ab10 as *const ())?;
    let _m4 = Mock::new(0x1411_d4eb0, fake_tree_insert_d4eb0 as *const ())?;

    let out_va = exe::ptr::<u8>(0x143c_e6878) as usize;
    let lock_va = exe::ptr::<u8>(0x1438_579f0) as usize;
    let tree_va = exe::ptr::<u8>(0x143a_631d8) as usize;
    let snap_out = Snapshot::take(out_va as *mut u8, 4);
    let snap_lock = Snapshot::take(lock_va as *mut u8, 16);
    let snap_tree = Snapshot::take(tree_va as *mut u8, 16);

    // Case 1: empty tree (head.+0x08 == head, head.+0x19 == 1) -> allocates new ID
    let mut head = [0u8; 0xb0];
    let hp = head.as_mut_ptr() as usize;
    wr::<usize>(hp + 0x08, hp);
    wr::<u8>(hp + 0x19, 1);

    wr::<u64>(lock_va, 0);
    wr::<u32>(lock_va + 0x08, u32::MAX);
    wr::<u32>(lock_va + 0x0c, 0);
    wr::<usize>(tree_va, hp);
    wr::<u32>(tree_va + 0x08, 41);
    let ro1 = exe::call_va!(orig, () -> usize);
    let id_o1 = rd::<u32>(out_va);
    let cnt_o1 = rd::<u32>(tree_va + 0x08);

    wr::<u64>(lock_va, 0);
    wr::<u32>(lock_va + 0x08, u32::MAX);
    wr::<u32>(lock_va + 0x0c, 0);
    wr::<usize>(tree_va, hp);
    wr::<u32>(tree_va + 0x08, 41);
    let rn1 = exe::call_va!(new, () -> usize);
    let id_n1 = rd::<u32>(out_va);
    let cnt_n1 = rd::<u32>(tree_va + 0x08);

    // Case 2: 1-node tree where key matches node1 (+0xa4 == 99)
    let mut node1 = [0u8; 0xb0];
    let np = node1.as_mut_ptr() as usize;
    wr::<usize>(np, hp);
    wr::<usize>(np + 0x10, hp);
    wr::<u8>(np + 0x19, 0);
    wr::<u32>(np + 0xa4, 99);
    wr::<usize>(hp + 0x08, np);

    wr::<usize>(tree_va, hp);
    let ro2 = exe::call_va!(orig, () -> usize);
    let id_o2 = rd::<u32>(out_va);

    wr::<usize>(tree_va, hp);
    let rn2 = exe::call_va!(new, () -> usize);
    let id_n2 = rd::<u32>(out_va);

    snap_tree.restore();
    snap_lock.restore();
    snap_out.restore();

    crate::diff_eq!(ro1, rn1, "defrag_id ret1");
    crate::diff_eq!(id_o1, id_n1, "defrag_id val1");
    crate::diff_eq!(cnt_o1, cnt_n1, "defrag_id cnt1");
    crate::diff_eq!(ro2, rn2, "defrag_id ret2");
    crate::diff_eq!(id_o2, id_n2, "defrag_id val2");
    Ok(2)
}

// REGISTRY: Replacement::verified(0x142054ea0, "d3d::destroyGlobal397fba0_142054ea0", crate::ns::rend::d3d_dlss_fg::d3d_destroy_global_397fba0 as *const (), crate::ns::rend::d3d_dlss_fg::diff_d3d_destroy_global_397fba0)
// REGISTRY: Replacement::verified_throws(0x142054f20, "d3d::initGlobal397fba0_142054f20", crate::ns::rend::d3d_dlss_fg::d3d_init_global_397fba0 as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg::diff_d3d_init_global_397fba0)
// REGISTRY: Replacement::verified(0x142055320, "d3d::clearSamplerStateCache_142055320", crate::ns::rend::d3d_dlss_fg::d3d_clear_sampler_state_cache as *const (), crate::ns::rend::d3d_dlss_fg::diff_d3d_clear_sampler_state_cache)
// REGISTRY: Replacement::verified(0x142055410, "d3d::clearAndReleaseArrayAt0_142055410", crate::ns::rend::d3d_dlss_fg::d3d_clear_and_release_array_at_0 as *const (), crate::ns::rend::d3d_dlss_fg::diff_d3d_clear_and_release_array_at_0)
// REGISTRY: Replacement::verified(0x142055620, "d3d::initDefaultSamplerStates_142055620", crate::ns::rend::d3d_dlss_fg::d3d_init_default_sampler_states as *const (), crate::ns::rend::d3d_dlss_fg::diff_d3d_init_default_sampler_states)
// REGISTRY: Replacement::verified(0x142055830, "d3d::clearTlsBoundObjectIfEq_142055830", crate::ns::rend::d3d_dlss_fg::d3d_clear_tls_bound_object_if_eq as *const (), crate::ns::rend::d3d_dlss_fg::diff_d3d_clear_tls_bound_object_if_eq)
// REGISTRY: Replacement::verified_throws(0x142055a60, "d3d::getStreamedTextureDefragmentId_142055a60", crate::ns::rend::d3d_dlss_fg::d3d_get_streamed_texture_defragment_id as *const (), Throws::Discard, crate::ns::rend::d3d_dlss_fg::diff_d3d_get_streamed_texture_defragment_id)
