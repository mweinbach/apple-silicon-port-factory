//! Differential tests (`group14b`) for the second 30 `Status::Draft` functions in `fsr2_glue.rs`.
#![allow(static_mut_refs)]

use crate::difftest::{IatMock, Mock, Snapshot};
use crate::exe::{self, rd, wr};

// ============================================================================
// 1..15: ComputeShader, ComHolder, ShaderReflection, Sampler, DispatchRays,
//        RtCompactionPool, and Query::tryGetResult
// ============================================================================

unsafe extern "win64" fn fake_free_2de0(ptr: usize) {
    if ptr != 0 {
        wr::<u64>(ptr, 0xfee1_dead_2de0_0001);
    }
}

unsafe extern "win64" fn fake_cache_dtor_18c0(cache: usize) {
    wr::<u64>(cache, 0xdead_18c0_0000_0001);
}

pub unsafe fn diff_d3d_compute_shader_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_free = Mock::new(0x1_4244_2de0, fake_free_2de0 as *const ())?;
    let _m_cache = Mock::new(0x1_40ec_18c0, fake_cache_dtor_18c0 as *const ())?;

    let mut inner_a = [0x1111_2222_3333_4444u64; 2];
    let mut inner_b = [0x1111_2222_3333_4444u64; 2];
    let mut node0_a = [inner_a.as_mut_ptr() as usize as u64, 0x55];
    let mut node0_b = [inner_b.as_mut_ptr() as usize as u64, 0x55];
    let mut node1_a = [0u64, 0x66];
    let mut node1_b = [0u64, 0x66];

    let mut buckets_a = [0u64; 6];
    let mut buckets_b = [0u64; 6];
    buckets_a[1] = node0_a.as_mut_ptr() as usize as u64;
    buckets_b[1] = node0_b.as_mut_ptr() as usize as u64;
    buckets_a[3] = 0;
    buckets_b[3] = 0;
    buckets_a[5] = node1_a.as_mut_ptr() as usize as u64;
    buckets_b[5] = node1_b.as_mut_ptr() as usize as u64;

    let mut cs_a = [0u8; 0x60];
    let mut cs_b = [0u8; 0x60];
    wr::<usize>(cs_a.as_mut_ptr() as usize + 0x28, buckets_a.as_mut_ptr() as usize);
    wr::<usize>(cs_b.as_mut_ptr() as usize + 0x28, buckets_b.as_mut_ptr() as usize);
    wr::<u32>(cs_a.as_mut_ptr() as usize + 0x30, 3);
    wr::<u32>(cs_b.as_mut_ptr() as usize + 0x30, 3);

    o(cs_a.as_mut_ptr() as usize);
    n(cs_b.as_mut_ptr() as usize);

    crate::diff_eq!(cs_a, cs_b, "cs state");
    crate::diff_eq!(inner_a, inner_b, "inner0 freed");
    crate::diff_eq!(node0_a, node0_b, "node0 freed");
    crate::diff_eq!(node1_a, node1_b, "node1 freed");
    Ok(1)
}

unsafe extern "win64" fn fake_com_release_inc(this: usize) -> u32 {
    let c = rd::<u32>(this + 0x08).wrapping_add(1);
    wr::<u32>(this + 0x08, c);
    c
}

unsafe extern "win64" fn fake_sized_delete_4d30(ptr: usize, size: u32) {
    wr::<u32>(ptr + 0x10, size ^ 0x4d30_0000);
}

pub unsafe fn diff_d3d_com_holder_b0_deleting_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_del = Mock::new(0x1_4183_4d30, fake_sized_delete_4d30 as *const ())?;

    let mut vtbl = [0usize; 4];
    vtbl[2] = fake_com_release_inc as *const () as usize; // +0x10

    let mut cases = 0;
    for (has_com, flags) in [(false, 0u32), (true, 0), (true, 1)] {
        let mut com_a = [vtbl.as_ptr() as usize, 10];
        let mut com_b = [vtbl.as_ptr() as usize, 10];
        let mut obj_a = [0u8; 0x20];
        let mut obj_b = [0u8; 0x20];
        if has_com {
            wr::<usize>(obj_a.as_mut_ptr() as usize + 0x08, com_a.as_mut_ptr() as usize);
            wr::<usize>(obj_b.as_mut_ptr() as usize + 0x08, com_b.as_mut_ptr() as usize);
        }
        let ra = o(obj_a.as_mut_ptr() as usize, flags) - obj_a.as_ptr() as usize;
        let rb = n(obj_b.as_mut_ptr() as usize, flags) - obj_b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret has_com={has_com} flags={flags}");
        crate::diff_eq!(obj_a, obj_b, "obj has_com={has_com} flags={flags}");
        crate::diff_eq!(com_a[1], com_b[1], "com release count");
        cases += 1;
    }
    Ok(cases)
}

unsafe extern "win64" fn fake_refl_get_tg_size(
    this: usize,
    out_x: usize,
    out_y: usize,
    out_z: usize,
) -> u32 {
    let x = rd::<u32>(this + 0x08);
    let y = rd::<u32>(this + 0x0c);
    let z = rd::<u32>(this + 0x10);
    wr::<u32>(out_x, x);
    wr::<u32>(out_y, y);
    wr::<u32>(out_z, z);
    x.wrapping_mul(y).wrapping_mul(z)
}

pub unsafe fn diff_d3d_shader_reflection_get_thread_group_size(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut vtbl = [0usize; 24];
    vtbl[0xa0 / 8] = fake_refl_get_tg_size as *const () as usize;

    let mut refl = [0u32; 6];
    wr::<usize>(refl.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    refl[2] = 8;
    refl[3] = 8;
    refl[4] = 1;
    let holder: [usize; 2] = [0, refl.as_ptr() as usize];

    let mut out_a = [0xdead_u32; 3];
    let mut out_b = [0xdead_u32; 3];
    let ra = o(holder.as_ptr() as usize, out_a.as_mut_ptr() as usize) - out_a.as_ptr() as usize;
    let rb = n(holder.as_ptr() as usize, out_b.as_mut_ptr() as usize) - out_b.as_ptr() as usize;
    crate::diff_eq!(ra, rb, "ret");
    crate::diff_eq!(out_a, out_b, "xyz");
    Ok(1)
}

unsafe extern "win64" fn fake_refl_get_input_param(
    _this: usize,
    param_idx: u32,
    out_desc: usize,
) -> u32 {
    for i in 0..8usize {
        wr::<u32>(out_desc + i * 4, 0x1000 + param_idx * 0x10 + i as u32);
    }
    0
}

pub unsafe fn diff_d3d_shader_reflection_get_input_param_desc(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut vtbl = [0usize; 10];
    vtbl[0x30 / 8] = fake_refl_get_input_param as *const () as usize;

    let refl: [usize; 2] = [vtbl.as_ptr() as usize, 0];
    let holder: [usize; 2] = [0, refl.as_ptr() as usize];

    for idx in [0u32, 2, 5] {
        let mut out_a = [0xaa_u8; 32];
        let mut out_b = [0xaa_u8; 32];
        let ra =
            o(holder.as_ptr() as usize, out_a.as_mut_ptr() as usize, idx) - out_a.as_ptr() as usize;
        let rb =
            n(holder.as_ptr() as usize, out_b.as_mut_ptr() as usize, idx) - out_b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret idx={idx}");
        crate::diff_eq!(out_a, out_b, "desc idx={idx}");
    }
    Ok(3)
}

unsafe extern "win64" fn fake_vec_free_e830(vec_ptr: usize) {
    wr::<u64>(vec_ptr, 0);
    wr::<u32>(vec_ptr + 0x08, rd::<u32>(vec_ptr + 0x08).wrapping_add(0xe830));
}

pub unsafe fn diff_d3d_shader_reflection_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_vec = Mock::new(0x1_401d_e830, fake_vec_free_e830 as *const ())?;

    let mut vtbl = [0usize; 4];
    vtbl[2] = fake_com_release_inc as *const () as usize;

    for has_com in [false, true] {
        let mut com_a = [vtbl.as_ptr() as usize, 3];
        let mut com_b = [vtbl.as_ptr() as usize, 3];
        let mut obj_a = [0u64; 6];
        let mut obj_b = [0u64; 6];
        if has_com {
            obj_a[1] = com_a.as_mut_ptr() as usize as u64;
            obj_b[1] = com_b.as_mut_ptr() as usize as u64;
        }
        obj_a[2] = 0x1111;
        obj_b[2] = 0x1111;
        obj_a[4] = 0x2222;
        obj_b[4] = 0x2222;

        o(obj_a.as_mut_ptr() as usize);
        n(obj_b.as_mut_ptr() as usize);
        crate::diff_eq!(obj_a, obj_b, "obj has_com={has_com}");
        crate::diff_eq!(com_a[1], com_b[1], "com release has_com={has_com}");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_sized_delete_30_4d30(ptr: usize, size: u32) {
    wr::<u32>(ptr, size ^ 0x4d30_0000);
}

pub unsafe fn diff_d3d_shader_reflection_deleting_dtor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_vec = Mock::new(0x1_401d_e830, fake_vec_free_e830 as *const ())?;
    let _m_del = Mock::new(0x1_4183_4d30, fake_sized_delete_30_4d30 as *const ())?;

    let mut vtbl = [0usize; 4];
    vtbl[2] = fake_com_release_inc as *const () as usize;

    for flags in [0u32, 1] {
        let mut com_a = [vtbl.as_ptr() as usize, 7];
        let mut com_b = [vtbl.as_ptr() as usize, 7];
        let mut obj_a = [0u64; 6];
        let mut obj_b = [0u64; 6];
        obj_a[1] = com_a.as_mut_ptr() as usize as u64;
        obj_b[1] = com_b.as_mut_ptr() as usize as u64;

        let ra = o(obj_a.as_mut_ptr() as usize, flags) - obj_a.as_ptr() as usize;
        let rb = n(obj_b.as_mut_ptr() as usize, flags) - obj_b.as_ptr() as usize;
        crate::diff_eq!(ra, rb, "ret flags={flags}");
        crate::diff_eq!(obj_a, obj_b, "obj flags={flags}");
        crate::diff_eq!(com_a[1], com_b[1], "com flags={flags}");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_dev_create_sampler(dev: usize, desc_ptr: usize, cpu_handle: u64) {
    wr::<u64>(dev + 0x08, cpu_handle);
    for i in 0..13usize {
        wr::<u32>(dev + 0x10 + i * 4, rd::<u32>(desc_ptr + i * 4));
    }
}

pub unsafe fn diff_d3d_recreate_sampler_descriptor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    #[cfg(not(target_arch = "aarch64"))]
    type F = unsafe extern "win64" fn(usize, f32);
    #[cfg(target_arch = "aarch64")]
    type F = unsafe extern "win64" fn(usize, f32, f32);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut vtbl = [0usize; 24];
    vtbl[0xb0 / 8] = fake_dev_create_sampler as *const () as usize;

    let mut dev_a = [0u8; 0x50];
    let mut dev_b = [0u8; 0x50];
    wr::<usize>(dev_a.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<usize>(dev_b.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

    let handle_val: u64 = 0xfeed_cafe_1234_5678;
    let mut sampler = [0u32; 16];
    wr::<usize>(sampler.as_mut_ptr() as usize, &handle_val as *const u64 as usize);
    for i in 0..13usize {
        sampler[2 + i] = 0x100 + i as u32;
    }

    let dev_va = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_dev = rd::<usize>(dev_va);

    wr::<usize>(dev_va, dev_a.as_mut_ptr() as usize);
    #[cfg(not(target_arch = "aarch64"))]
    o(sampler.as_ptr() as usize, -0.75f32);
    #[cfg(target_arch = "aarch64")]
    o(sampler.as_ptr() as usize, 0.0, -0.75f32);

    wr::<usize>(dev_va, dev_b.as_mut_ptr() as usize);
    #[cfg(not(target_arch = "aarch64"))]
    n(sampler.as_ptr() as usize, -0.75f32);
    #[cfg(target_arch = "aarch64")]
    n(sampler.as_ptr() as usize, 0.0, -0.75f32);

    wr::<usize>(dev_va, saved_dev);

    crate::diff_eq!(&dev_a[0x08..0x44], &dev_b[0x08..0x44], "sampler desc & handle");
    Ok(1)
}

static mut RAY_CTX_PTR: usize = 0;

unsafe extern "win64" fn fake_get_ray_ctx_b050(lock_guard_out: usize) -> usize {
    let ctx = RAY_CTX_PTR;
    wr::<usize>(lock_guard_out, rd::<usize>(ctx));
    wr::<usize>(lock_guard_out + 0x08, ctx);
    ctx
}

unsafe extern "win64" fn fake_submit_ray_cmd_1870(dst_ring: usize, cmd_ptr: usize) {
    wr::<u32>(dst_ring, rd::<u32>(cmd_ptr));
    wr::<u32>(dst_ring + 0x04, rd::<u32>(cmd_ptr + 0x40));
    for i in 0..9usize {
        wr::<u64>(dst_ring + 0x08 + i * 8, rd::<u64>(cmd_ptr + 0x48 + i * 8));
    }
    let geom_ptr = rd::<usize>(cmd_ptr + 0x08);
    for i in 0..6usize {
        wr::<u32>(dst_ring + 0x50 + i * 4, rd::<u32>(geom_ptr + i * 4));
    }
    let mode = rd::<u32>(geom_ptr + 0x08);
    if (2..=4).contains(&mode) {
        for i in 6..12usize {
            wr::<u32>(dst_ring + 0x50 + i * 4, rd::<u32>(geom_ptr + i * 4));
        }
    }
}

unsafe fn run_dispatch_rays_variant_1(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u32, usize, u64, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_ctx = Mock::new(0x1_411d_b050, fake_get_ray_ctx_b050 as *const ())?;
    let _m_sub = Mock::new(0x1_4000_1870, fake_submit_ray_cmd_1870 as *const ())?;

    let mut state_obj = [0u64; 9];
    for (i, w) in state_obj.iter_mut().enumerate() {
        *w = 0x9000_0000_0000_0000 + i as u64;
    }
    let mut ring_a = [0u8; 0x80];
    let mut ring_b = [0u8; 0x80];
    let mut lock_a = [0u32, 0, 0x1234, 2];
    let mut lock_b = [0u32, 0, 0x1234, 2];
    let ctx_a: [usize; 2] = [lock_a.as_mut_ptr() as usize, ring_a.as_mut_ptr() as usize];
    let ctx_b: [usize; 2] = [lock_b.as_mut_ptr() as usize, ring_b.as_mut_ptr() as usize];

    RAY_CTX_PTR = ctx_a.as_ptr() as usize;
    o(0, 1920, 1080, 0, 0x0000_0004_0000_0008, state_obj.as_ptr() as usize);

    RAY_CTX_PTR = ctx_b.as_ptr() as usize;
    n(0, 1920, 1080, 0, 0x0000_0004_0000_0008, state_obj.as_ptr() as usize);

    crate::diff_eq!(ring_a, ring_b, "ray cmd packet");
    crate::diff_eq!(lock_a, lock_b, "ray lock state");
    Ok(1)
}

unsafe fn run_dispatch_rays_variant_32b(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u32, usize, usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_ctx = Mock::new(0x1_411d_b050, fake_get_ray_ctx_b050 as *const ())?;
    let _m_sub = Mock::new(0x1_4000_1870, fake_submit_ray_cmd_1870 as *const ())?;

    let mut state_obj = [0u64; 9];
    for (i, w) in state_obj.iter_mut().enumerate() {
        *w = 0xa000_0000_0000_0000 + i as u64;
    }
    let mut payload = [0u32; 8];
    for (i, w) in payload.iter_mut().enumerate() {
        *w = 0x300 + i as u32;
    }
    wr::<u8>(payload.as_mut_ptr() as usize, 2);

    let mut ring_a = [0u8; 0x80];
    let mut ring_b = [0u8; 0x80];
    let ctx_a: [usize; 2] = [0, ring_a.as_mut_ptr() as usize];
    let ctx_b: [usize; 2] = [0, ring_b.as_mut_ptr() as usize];

    RAY_CTX_PTR = ctx_a.as_ptr() as usize;
    o(0, 1280, 720, 0, payload.as_ptr() as usize, state_obj.as_ptr() as usize);

    RAY_CTX_PTR = ctx_b.as_ptr() as usize;
    n(0, 1280, 720, 0, payload.as_ptr() as usize, state_obj.as_ptr() as usize);

    crate::diff_eq!(ring_a, ring_b, "ray cmd packet 32b");
    Ok(1)
}

pub unsafe fn diff_d3d_dispatch_rays_1(orig: usize, new: usize) -> Result<u32, String> {
    run_dispatch_rays_variant_1(orig, new)
}

pub unsafe fn diff_d3d_dispatch_rays_4(orig: usize, new: usize) -> Result<u32, String> {
    run_dispatch_rays_variant_32b(orig, new)
}

pub unsafe fn diff_d3d_dispatch_rays_3(orig: usize, new: usize) -> Result<u32, String> {
    run_dispatch_rays_variant_32b(orig, new)
}

pub unsafe fn diff_d3d_dispatch_rays_common(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, u32, u32, usize, usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_ctx = Mock::new(0x1_411d_b050, fake_get_ray_ctx_b050 as *const ())?;
    let _m_sub = Mock::new(0x1_4000_1870, fake_submit_ray_cmd_1870 as *const ())?;

    let mut state_obj = [0u64; 9];
    for (i, w) in state_obj.iter_mut().enumerate() {
        *w = 0xb000_0000_0000_0000 + i as u64;
    }
    let mut geom_src = [0u32; 8];
    for (i, w) in geom_src.iter_mut().enumerate() {
        *w = 0x500 + i as u32;
    }
    wr::<u8>(geom_src.as_mut_ptr() as usize, 10);

    for mode in [1u32, 3, 4] {
        let mut ring_a = [0u8; 0x80];
        let mut ring_b = [0u8; 0x80];
        let mut lock_a = [0u32, 0, 0x1234, 2];
        let mut lock_b = [0u32, 0, 0x1234, 2];
        let ctx_a: [usize; 2] = [lock_a.as_mut_ptr() as usize, ring_a.as_mut_ptr() as usize];
        let ctx_b: [usize; 2] = [lock_b.as_mut_ptr() as usize, ring_b.as_mut_ptr() as usize];

        RAY_CTX_PTR = ctx_a.as_ptr() as usize;
        o(640, 360, mode, 0, geom_src.as_ptr() as usize, state_obj.as_ptr() as usize);

        RAY_CTX_PTR = ctx_b.as_ptr() as usize;
        n(640, 360, mode, 0, geom_src.as_ptr() as usize, state_obj.as_ptr() as usize);

        crate::diff_eq!(ring_a, ring_b, "ray common mode={mode}");
        crate::diff_eq!(lock_a, lock_b, "ray common lock mode={mode}");
    }
    Ok(3)
}

unsafe extern "win64" fn fake_rt_unmap_d6f0(buf: usize, subres: u32) {
    wr::<u64>(buf + 0x08, 0xd6f0_0000_0000_0000 | (subres as u64));
}

pub unsafe fn diff_d3d_rt_compaction_pool_unmap(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(0x1_4209_d6f0, fake_rt_unmap_d6f0 as *const ())?;

    for frame in [0u32, 1] {
        let mut buf0_a = [0u64; 4];
        let mut buf1_a = [0u64; 4];
        let mut buf0_b = [0u64; 4];
        let mut buf1_b = [0u64; 4];
        let mut pool_a = [0u64; 6];
        let mut pool_b = [0u64; 6];
        pool_a[1] = buf0_a.as_mut_ptr() as usize as u64;
        pool_a[2] = buf1_a.as_mut_ptr() as usize as u64;
        pool_a[3] = 0xdead_beef;
        pool_a[4] = frame as u64;

        pool_b[1] = buf0_b.as_mut_ptr() as usize as u64;
        pool_b[2] = buf1_b.as_mut_ptr() as usize as u64;
        pool_b[3] = 0xdead_beef;
        pool_b[4] = frame as u64;

        o(pool_a.as_mut_ptr() as usize);
        n(pool_b.as_mut_ptr() as usize);

        crate::diff_eq!(pool_a[3], pool_b[3], "mapped ptr cleared frame={frame}");
        crate::diff_eq!(buf0_a, buf0_b, "buf0 frame={frame}");
        crate::diff_eq!(buf1_a, buf1_b, "buf1 frame={frame}");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_rt_map_c560(
    buf: usize,
    frame_idx: u32,
    byte_len: u64,
    flags: u32,
) -> usize {
    wr::<u32>(buf + 0x18, frame_idx);
    wr::<u32>(buf + 0x1c, flags);
    wr::<u64>(buf + 0x20, byte_len);
    0x7777_0000usize + (byte_len as usize) + (frame_idx as usize)
}

pub unsafe fn diff_d3d_rt_compaction_pool_map(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(0x1_4209_c560, fake_rt_map_c560 as *const ())?;

    for frame in [4u32, 5] {
        let mut buf0_a = [0u64, 0, 0x200, 0, 0];
        let mut buf1_a = [0u64, 0, 0x400, 0, 0];
        let mut buf0_b = [0u64, 0, 0x200, 0, 0];
        let mut buf1_b = [0u64, 0, 0x400, 0, 0];
        let mut pool_a = [0u64; 6];
        let mut pool_b = [0u64; 6];
        pool_a[1] = buf0_a.as_mut_ptr() as usize as u64;
        pool_a[2] = buf1_a.as_mut_ptr() as usize as u64;
        pool_b[1] = buf0_b.as_mut_ptr() as usize as u64;
        pool_b[2] = buf1_b.as_mut_ptr() as usize as u64;

        o(pool_a.as_mut_ptr() as usize, frame);
        n(pool_b.as_mut_ptr() as usize, frame);

        crate::diff_eq!(pool_a[3], pool_b[3], "mapped ptr frame={frame}");
        crate::diff_eq!(pool_a[4], pool_b[4], "frame_idx stored frame={frame}");
        crate::diff_eq!(buf0_a, buf0_b, "buf0 frame={frame}");
        crate::diff_eq!(buf1_a, buf1_b, "buf1 frame={frame}");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_rt_barrier_5850(src_buf: usize) {
    wr::<u64>(src_buf + 0x08, 0x5850_cafe);
}

unsafe extern "win64" fn fake_rt_copy_5bc0(
    dst_buf: usize,
    dst_off: u64,
    src_buf: usize,
    src_off: u64,
    byte_size: u64,
) {
    wr::<u64>(dst_buf + 0x08, dst_off ^ 0x1111);
    wr::<u64>(dst_buf + 0x10, src_off ^ 0x2222);
    wr::<u64>(dst_buf + 0x18, byte_size);
    wr::<u64>(dst_buf + 0x20, rd::<u64>(src_buf));
}

pub unsafe fn diff_d3d_rt_compaction_pool_resolve(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_bar = Mock::new(0x1_4207_5850, fake_rt_barrier_5850 as *const ())?;
    let _m_cpy = Mock::new(0x1_4207_5bc0, fake_rt_copy_5bc0 as *const ())?;

    for (count, gen) in [(0u32, 2u32), (6, 3)] {
        let mut src_a = [0x9999_u64, 0];
        let mut src_b = [0x9999_u64, 0];
        let mut dst0_a = [0u64; 5];
        let mut dst1_a = [0u64; 5];
        let mut dst0_b = [0u64; 5];
        let mut dst1_b = [0u64; 5];

        let mut pool_a = [0u64; 7];
        let mut pool_b = [0u64; 7];
        pool_a[0] = src_a.as_mut_ptr() as usize as u64;
        pool_a[1] = dst0_a.as_mut_ptr() as usize as u64;
        pool_a[2] = dst1_a.as_mut_ptr() as usize as u64;
        wr::<u32>(pool_a.as_mut_ptr() as usize + 0x28, count);
        wr::<u32>(pool_a.as_mut_ptr() as usize + 0x30, gen);

        pool_b[0] = src_b.as_mut_ptr() as usize as u64;
        pool_b[1] = dst0_b.as_mut_ptr() as usize as u64;
        pool_b[2] = dst1_b.as_mut_ptr() as usize as u64;
        wr::<u32>(pool_b.as_mut_ptr() as usize + 0x28, count);
        wr::<u32>(pool_b.as_mut_ptr() as usize + 0x30, gen);

        o(pool_a.as_mut_ptr() as usize);
        n(pool_b.as_mut_ptr() as usize);

        crate::diff_eq!(pool_a[5..7], pool_b[5..7], "pool counters count={count}");
        crate::diff_eq!(src_a, src_b, "src count={count}");
        crate::diff_eq!(dst0_a, dst0_b, "dst0 count={count}");
        crate::diff_eq!(dst1_a, dst1_b, "dst1 count={count}");
    }
    Ok(2)
}

unsafe extern "win64" fn fake_fence_get_completed(fence: usize) -> u64 {
    rd::<u64>(fence + 0x08)
}

unsafe extern "win64" fn fake_query_res_map(
    res: usize,
    subres: u32,
    range_ptr: usize,
    out_mapped: usize,
) -> i32 {
    wr::<u32>(res + 0x0c, subres);
    wr::<u64>(res + 0x18, rd::<u64>(range_ptr));
    wr::<u64>(res + 0x20, rd::<u64>(range_ptr + 0x08));
    let hr = rd::<i32>(res + 0x08);
    if hr >= 0 {
        wr::<usize>(out_mapped, rd::<usize>(res + 0x10));
    }
    hr
}

unsafe extern "win64" fn fake_query_res_unmap(res: usize, _subres: u32, range_ptr: usize) {
    wr::<u64>(res + 0x28, rd::<u64>(range_ptr) ^ 0x5555);
    wr::<u64>(res + 0x30, rd::<u64>(range_ptr + 0x08) ^ 0x6666);
}

pub unsafe fn diff_d3d_query_try_get_result(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut fence_vtbl = [0usize; 10];
    fence_vtbl[0x40 / 8] = fake_fence_get_completed as *const () as usize;

    let mut res_vtbl = [0usize; 12];
    res_vtbl[0x40 / 8] = fake_query_res_map as *const () as usize;
    res_vtbl[0x48 / 8] = fake_query_res_unmap as *const () as usize;

    let slots_data: [u64; 4] = [0x1010, 0x2020, 0x3030_4040_5050_6060, 0x4040];

    // Case 0: null block -> 2
    {
        let q = [0usize, 0];
        let (mut oa, mut ob) = (0xdead_u64, 0xdead_u64);
        let ra = o(q.as_ptr() as usize, &mut oa as *mut u64 as usize);
        let rb = n(q.as_ptr() as usize, &mut ob as *mut u64 as usize);
        crate::diff_eq!(ra, rb, "null block ret");
        crate::diff_eq!(oa, ob, "null block out");
    }

    let mut cases = 1;
    for &(q_gen, b_gen, fence_target, fence_done, map_hr) in &[
        (1i32, 2i32, 10u64, 15u64, 0i32),  // stale -> 2
        (2, 2, 10, 5, 0),                  // fence pending -> 1
        (2, 2, 10, 10, 0),                 // ready -> 0
        (2, 2, 10, 12, -0x7fff_fffe),      // map failed -> 2
    ] {
        let fence = [fence_vtbl.as_ptr() as usize as u64, fence_done];
        let mut res_a = [0u64; 8];
        let mut res_b = [0u64; 8];
        for r in [&mut res_a, &mut res_b] {
            let rp = r.as_mut_ptr() as usize;
            wr::<usize>(rp, res_vtbl.as_ptr() as usize);
            wr::<i32>(rp + 0x08, map_hr);
            wr::<usize>(rp + 0x10, slots_data.as_ptr() as usize);
        }

        let mut block_a = [0u8; 0x38];
        let mut block_b = [0u8; 0x38];
        for (blk, res) in [(&mut block_a, &res_a), (&mut block_b, &res_b)] {
            let bp = blk.as_mut_ptr() as usize;
            wr::<u64>(bp, fence_target);
            wr::<usize>(bp + 0x10, fence.as_ptr() as usize);
            wr::<usize>(bp + 0x28, res.as_ptr() as usize);
            wr::<i32>(bp + 0x30, b_gen);
        }

        let mut q_a = [0u8; 0x18];
        let mut q_b = [0u8; 0x18];
        for (q, blk) in [(&mut q_a, &block_a), (&mut q_b, &block_b)] {
            let qp = q.as_mut_ptr() as usize;
            wr::<usize>(qp, blk.as_ptr() as usize);
            wr::<i32>(qp + 0x08, 2);
            wr::<i32>(qp + 0x0c, q_gen);
        }

        let (mut oa, mut ob) = (0xdead_beef_u64, 0xdead_beef_u64);
        let ra = o(q_a.as_ptr() as usize, &mut oa as *mut u64 as usize);
        let rb = n(q_b.as_ptr() as usize, &mut ob as *mut u64 as usize);
        crate::diff_eq!(ra, rb, "ret case={cases}");
        crate::diff_eq!(oa, ob, "out_val case={cases}");
        crate::diff_eq!(&res_a[2..], &res_b[2..], "res map/unmap state case={cases}");
        cases += 1;
    }
    Ok(cases)
}

// ============================================================================
// 16..25: Query::begin, Query::end, Query::ctor, QueryHeapManager::vfunc_1,
//         QueryBlock::resolve, QueryBlock::readSlot, QueryBlock::dtor,
//         QueryHeapManager::allocSlot, Query::readSlot, Query::isFenceComplete
// ============================================================================

static mut CMD_GUARD_LOCK_PTR: usize = 0;
static mut CMD_GUARD_LIST_PTR: usize = 0;

unsafe extern "win64" fn fake_get_cmd_guard_5b00(out_guard: usize) -> usize {
    wr::<usize>(out_guard, CMD_GUARD_LOCK_PTR);
    wr::<usize>(out_guard + 0x08, CMD_GUARD_LIST_PTR);
    out_guard
}

unsafe extern "win64" fn fake_cmd_begin_query(
    cmd_list: usize,
    heap: usize,
    d3d_qtype: u32,
    idx: u32,
) {
    wr::<usize>(cmd_list + 0x08, heap);
    wr::<u32>(cmd_list + 0x10, d3d_qtype);
    wr::<u32>(cmd_list + 0x14, idx);
}

unsafe extern "win64" fn fake_cmd_end_query(
    cmd_list: usize,
    heap: usize,
    d3d_qtype: u32,
    idx: u32,
) {
    wr::<usize>(cmd_list + 0x18, heap);
    wr::<u32>(cmd_list + 0x20, d3d_qtype);
    wr::<u32>(cmd_list + 0x24, idx);
}

unsafe extern "win64" fn fake_cmd_resolve_query_data(
    cmd_list: usize,
    heap: usize,
    qtype: u32,
    start_idx: u32,
    count: u32,
    dst_buf: usize,
    aligned_dst_off: u64,
) {
    wr::<usize>(cmd_list + 0x08, heap);
    wr::<u32>(cmd_list + 0x10, qtype);
    wr::<u32>(cmd_list + 0x14, start_idx);
    wr::<u32>(cmd_list + 0x18, count);
    wr::<usize>(cmd_list + 0x20, dst_buf);
    wr::<u64>(cmd_list + 0x28, aligned_dst_off ^ 0xabcd_ef01);
}

unsafe extern "win64" fn fake_get_tid_fixed() -> u32 {
    0x4242
}

pub unsafe fn diff_d3d_query_begin(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_cmd = Mock::new(0x1_411d_5b00, fake_get_cmd_guard_5b00 as *const ())?;
    let _im_tid = IatMock::new(0x1_4264_5790, fake_get_tid_fixed as *const ())?;

    let mut cmd_vtbl = [0usize; 60];
    cmd_vtbl[0x1a0 / 8] = fake_cmd_begin_query as *const () as usize;

    let mgr_va = exe::ptr::<usize>(0x1_4397_fc48) as usize;
    let saved_mgr = rd::<usize>(mgr_va);

    for qtype in [1u32, 0u32] {
        let mut blk_a = [0u8; 0x38];
        let mut blk_b = [0u8; 0x38];
        for blk in [&mut blk_a, &mut blk_b] {
            let bp = blk.as_mut_ptr() as usize;
            wr::<usize>(bp + 0x18, 0x8888_0018);
            wr::<u32>(bp + 0x30, 7);
            wr::<u32>(bp + 0x34, 3);
        }

        let mut mgr_a = [0u8; 0xc0];
        let mut mgr_b = [0u8; 0xc0];
        for (m, blk) in [(&mut mgr_a, &blk_a), (&mut mgr_b, &blk_b)] {
            let mp = m.as_mut_ptr() as usize;
            wr::<u32>(mp + 0x10, 0x4242);
            wr::<u32>(mp + 0x14, 1);
            wr::<usize>(mp + 0xa8, blk.as_ptr() as usize);
        }

        let mut cmd_a = [0u64; 6];
        let mut cmd_b = [0u64; 6];
        cmd_a[0] = cmd_vtbl.as_ptr() as usize as u64;
        cmd_b[0] = cmd_vtbl.as_ptr() as usize as u64;
        let mut lock_a = [0u32, 0, 0x4242, 2];
        let mut lock_b = [0u32, 0, 0x4242, 2];

        let mut q_a = [0u8; 0x18];
        let mut q_b = [0u8; 0x18];
        wr::<u32>(q_a.as_mut_ptr() as usize + 0x10, qtype);
        wr::<u32>(q_b.as_mut_ptr() as usize + 0x10, qtype);

        wr::<usize>(mgr_va, mgr_a.as_mut_ptr() as usize);
        CMD_GUARD_LOCK_PTR = lock_a.as_mut_ptr() as usize;
        CMD_GUARD_LIST_PTR = cmd_a.as_mut_ptr() as usize;
        o(q_a.as_mut_ptr() as usize);

        wr::<usize>(mgr_va, mgr_b.as_mut_ptr() as usize);
        CMD_GUARD_LOCK_PTR = lock_b.as_mut_ptr() as usize;
        CMD_GUARD_LIST_PTR = cmd_b.as_mut_ptr() as usize;
        n(q_b.as_mut_ptr() as usize);

        if qtype == 0 {
            wr::<usize>(
                q_a.as_mut_ptr() as usize,
                rd::<usize>(q_a.as_ptr() as usize).wrapping_sub(blk_a.as_ptr() as usize),
            );
            wr::<usize>(
                q_b.as_mut_ptr() as usize,
                rd::<usize>(q_b.as_ptr() as usize).wrapping_sub(blk_b.as_ptr() as usize),
            );
        }
        crate::diff_eq!(q_a, q_b, "q state qtype={qtype}");
        crate::diff_eq!(blk_a, blk_b, "blk state qtype={qtype}");
        crate::diff_eq!(&cmd_a[1..], &cmd_b[1..], "cmd_list state qtype={qtype}");
        crate::diff_eq!(lock_a, lock_b, "lock state qtype={qtype}");
    }

    wr::<usize>(mgr_va, saved_mgr);
    Ok(2)
}

pub unsafe fn diff_d3d_query_end(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_cmd = Mock::new(0x1_411d_5b00, fake_get_cmd_guard_5b00 as *const ())?;
    let _im_tid = IatMock::new(0x1_4264_5790, fake_get_tid_fixed as *const ())?;

    let mut cmd_vtbl = [0usize; 60];
    cmd_vtbl[0x1a8 / 8] = fake_cmd_end_query as *const () as usize;

    let mgr_va = exe::ptr::<usize>(0x1_4397_fc48) as usize;
    let saved_mgr = rd::<usize>(mgr_va);

    for qtype in [0u32, 1u32] {
        let mut blk_a = [0u8; 0x38];
        let mut blk_b = [0u8; 0x38];
        for blk in [&mut blk_a, &mut blk_b] {
            let bp = blk.as_mut_ptr() as usize;
            wr::<usize>(bp + 0x18, 0x9999_0018);
            wr::<u32>(bp + 0x30, 9);
            wr::<u32>(bp + 0x34, 4);
        }

        let mut mgr_a = [0u8; 0xc0];
        let mut mgr_b = [0u8; 0xc0];
        for (m, blk) in [(&mut mgr_a, &blk_a), (&mut mgr_b, &blk_b)] {
            let mp = m.as_mut_ptr() as usize;
            wr::<u32>(mp + 0x10, 0x4242);
            wr::<u32>(mp + 0x14, 1);
            wr::<usize>(mp + 0xa8 + (qtype as usize) * 8, blk.as_ptr() as usize);
        }

        let mut cmd_a = [0u64; 6];
        let mut cmd_b = [0u64; 6];
        cmd_a[0] = cmd_vtbl.as_ptr() as usize as u64;
        cmd_b[0] = cmd_vtbl.as_ptr() as usize as u64;
        let mut lock_a = [0u32, 0, 0x4242, 2];
        let mut lock_b = [0u32, 0, 0x4242, 2];

        let mut q_a = [0u8; 0x18];
        let mut q_b = [0u8; 0x18];
        for (q, blk) in [(&mut q_a, &blk_a), (&mut q_b, &blk_b)] {
            let qp = q.as_mut_ptr() as usize;
            wr::<usize>(qp, blk.as_ptr() as usize);
            wr::<u32>(qp + 0x08, 2);
            wr::<u32>(qp + 0x10, qtype);
        }

        wr::<usize>(mgr_va, mgr_a.as_mut_ptr() as usize);
        CMD_GUARD_LOCK_PTR = lock_a.as_mut_ptr() as usize;
        CMD_GUARD_LIST_PTR = cmd_a.as_mut_ptr() as usize;
        o(q_a.as_mut_ptr() as usize);

        wr::<usize>(mgr_va, mgr_b.as_mut_ptr() as usize);
        CMD_GUARD_LOCK_PTR = lock_b.as_mut_ptr() as usize;
        CMD_GUARD_LIST_PTR = cmd_b.as_mut_ptr() as usize;
        n(q_b.as_mut_ptr() as usize);

        wr::<usize>(
            q_a.as_mut_ptr() as usize,
            rd::<usize>(q_a.as_ptr() as usize).wrapping_sub(blk_a.as_ptr() as usize),
        );
        wr::<usize>(
            q_b.as_mut_ptr() as usize,
            rd::<usize>(q_b.as_ptr() as usize).wrapping_sub(blk_b.as_ptr() as usize),
        );
        crate::diff_eq!(q_a, q_b, "q state qtype={qtype}");
        crate::diff_eq!(blk_a, blk_b, "blk state qtype={qtype}");
        crate::diff_eq!(&cmd_a[1..], &cmd_b[1..], "cmd_list state qtype={qtype}");
        crate::diff_eq!(lock_a, lock_b, "lock state qtype={qtype}");
    }

    wr::<usize>(mgr_va, saved_mgr);
    Ok(2)
}

unsafe extern "win64" fn fake_queue_get_ts_freq(_queue: usize, out_freq: usize) -> u32 {
    wr::<u64>(out_freq, 100_000_000);
    0
}

pub unsafe fn diff_d3d_query_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut q_vtbl = [0usize; 20];
    q_vtbl[0x80 / 8] = fake_queue_get_ts_freq as *const () as usize;
    let fake_queue: [usize; 2] = [q_vtbl.as_ptr() as usize, 0];
    let queue_holder: [usize; 3] = [0, 0, fake_queue.as_ptr() as usize];

    let freq_va = exe::ptr::<u64>(0x1_4397_fc40) as usize;
    let qholder_va = exe::ptr::<usize>(0x1_4397_fc60) as usize;
    let saved_freq = rd::<u64>(freq_va);
    let saved_qholder = rd::<usize>(qholder_va);
    wr::<usize>(qholder_va, queue_holder.as_ptr() as usize);

    for (init_freq, qtype) in [(50_000_000u64, 0u32), (0u64, 1u32)] {
        let mut qa = [0xaa_u8; 0x18];
        let mut qb = [0xaa_u8; 0x18];

        wr::<u64>(freq_va, init_freq);
        let ra = o(qa.as_mut_ptr() as usize, qtype) - qa.as_ptr() as usize;
        let freq_a = rd::<u64>(freq_va);

        wr::<u64>(freq_va, init_freq);
        let rb = n(qb.as_mut_ptr() as usize, qtype) - qb.as_ptr() as usize;
        let freq_b = rd::<u64>(freq_va);

        crate::diff_eq!(ra, rb, "ret init_freq={init_freq}");
        crate::diff_eq!(qa[..0x14], qb[..0x14], "query mem init_freq={init_freq}");
        crate::diff_eq!(freq_a, freq_b, "freq global init_freq={init_freq}");
    }

    wr::<u64>(freq_va, saved_freq);
    wr::<usize>(qholder_va, saved_qholder);
    Ok(2)
}

unsafe extern "win64" fn fake_flush_query_pool_3760(this: usize, pool_idx: u32) {
    wr::<u32>(this + (pool_idx as usize) * 4, 0x3760_0000 + pool_idx);
}

pub unsafe fn diff_d3d_query_heap_mgr_vfunc_1(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(0x1_4206_3760, fake_flush_query_pool_3760 as *const ())?;

    let mut mgr_a = [0u32; 4];
    let mut mgr_b = [0u32; 4];
    o(mgr_a.as_mut_ptr() as usize);
    n(mgr_b.as_mut_ptr() as usize);
    crate::diff_eq!(mgr_a, mgr_b, "flushed pools 0 and 1");
    Ok(1)
}

pub unsafe fn diff_d3d_query_block_resolve(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_cmd = Mock::new(0x1_411d_5b00, fake_get_cmd_guard_5b00 as *const ())?;

    let mut cmd_vtbl = [0usize; 60];
    cmd_vtbl[0x1b0 / 8] = fake_cmd_resolve_query_data as *const () as usize;

    let mut cmd_a = [0u64; 8];
    let mut cmd_b = [0u64; 8];
    cmd_a[0] = cmd_vtbl.as_ptr() as usize as u64;
    cmd_b[0] = cmd_vtbl.as_ptr() as usize as u64;
    let mut lock_a = [0u32, 0, 0x4242, 2];
    let mut lock_b = [0u32, 0, 0x4242, 2];

    let mut blk_a = [0u8; 0x38];
    let mut blk_b = [0u8; 0x38];
    for blk in [&mut blk_a, &mut blk_b] {
        let bp = blk.as_mut_ptr() as usize;
        wr::<usize>(bp + 0x18, 0x1111_0018);
        wr::<u32>(bp + 0x20, 2);
        wr::<usize>(bp + 0x28, 0x2222_0028);
        wr::<u32>(bp + 0x34, 17);
    }

    CMD_GUARD_LOCK_PTR = lock_a.as_mut_ptr() as usize;
    CMD_GUARD_LIST_PTR = cmd_a.as_mut_ptr() as usize;
    o(blk_a.as_mut_ptr() as usize, 0x7777_8888_9999_aaaa);

    CMD_GUARD_LOCK_PTR = lock_b.as_mut_ptr() as usize;
    CMD_GUARD_LIST_PTR = cmd_b.as_mut_ptr() as usize;
    n(blk_b.as_mut_ptr() as usize, 0x7777_8888_9999_aaaa);

    crate::diff_eq!(blk_a, blk_b, "block fence_val updated");
    crate::diff_eq!(&cmd_a[1..], &cmd_b[1..], "ResolveQueryData args");
    crate::diff_eq!(lock_a, lock_b, "lock unlocked");
    Ok(1)
}

pub unsafe fn diff_d3d_query_block_read_slot(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32) -> i32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut res_vtbl = [0usize; 12];
    res_vtbl[0x40 / 8] = fake_query_res_map as *const () as usize;
    res_vtbl[0x48 / 8] = fake_query_res_unmap as *const () as usize;
    let slots_data: [u64; 4] = [0xaaaa_0000, 0xbbbb_1111, 0xcccc_2222, 0xdddd_3333];

    for (map_hr, slot) in [(0i32, 2i32), (-1i32, 1i32)] {
        let mut res_a = [0u64; 8];
        let mut res_b = [0u64; 8];
        for r in [&mut res_a, &mut res_b] {
            let rp = r.as_mut_ptr() as usize;
            wr::<usize>(rp, res_vtbl.as_ptr() as usize);
            wr::<i32>(rp + 0x08, map_hr);
            wr::<usize>(rp + 0x10, slots_data.as_ptr() as usize);
        }
        let mut blk_a = [0u8; 0x38];
        let mut blk_b = [0u8; 0x38];
        wr::<usize>(blk_a.as_mut_ptr() as usize + 0x28, res_a.as_mut_ptr() as usize);
        wr::<usize>(blk_b.as_mut_ptr() as usize + 0x28, res_b.as_mut_ptr() as usize);

        let (mut oa, mut ob) = (0xdead_u64, 0xdead_u64);
        let ra = o(blk_a.as_ptr() as usize, &mut oa as *mut u64 as usize, slot);
        let rb = n(blk_b.as_ptr() as usize, &mut ob as *mut u64 as usize, slot);
        crate::diff_eq!(ra, rb, "hr map_hr={map_hr}");
        crate::diff_eq!(oa, ob, "out_val map_hr={map_hr}");
        crate::diff_eq!(&res_a[2..], &res_b[2..], "res state map_hr={map_hr}");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_query_block_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut vtbl = [0usize; 4];
    vtbl[2] = fake_com_release_inc as *const () as usize;

    for has_objs in [false, true] {
        let mut res_a = [vtbl.as_ptr() as usize, 1];
        let mut res_b = [vtbl.as_ptr() as usize, 1];
        let mut heap_a = [vtbl.as_ptr() as usize, 5];
        let mut heap_b = [vtbl.as_ptr() as usize, 5];

        let mut blk_a = [0u8; 0x38];
        let mut blk_b = [0u8; 0x38];
        if has_objs {
            wr::<usize>(blk_a.as_mut_ptr() as usize + 0x18, heap_a.as_mut_ptr() as usize);
            wr::<usize>(blk_a.as_mut_ptr() as usize + 0x28, res_a.as_mut_ptr() as usize);
            wr::<usize>(blk_b.as_mut_ptr() as usize + 0x18, heap_b.as_mut_ptr() as usize);
            wr::<usize>(blk_b.as_mut_ptr() as usize + 0x28, res_b.as_mut_ptr() as usize);
        }

        o(blk_a.as_mut_ptr() as usize);
        n(blk_b.as_mut_ptr() as usize);
        crate::diff_eq!(blk_a, blk_b, "blk has_objs={has_objs}");
        crate::diff_eq!(res_a[1], res_b[1], "res release has_objs={has_objs}");
        crate::diff_eq!(heap_a[1], heap_b[1], "heap release has_objs={has_objs}");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_query_heap_mgr_alloc_slot(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _im_tid = IatMock::new(0x1_4264_5790, fake_get_tid_fixed as *const ())?;

    let mgr_va = exe::ptr::<usize>(0x1_4397_fc48) as usize;
    let saved_mgr = rd::<usize>(mgr_va);

    // Case 0: current block has room (`count < 0x80`), Case 1: current block full (`count == 0x80`), pop from free_q
    for init_cnt in [5u32, 0x80u32] {
        let mut blk1_a = [0u8; 0x38];
        let mut blk1_b = [0u8; 0x38];
        let mut blk2_a = [0u8; 0x38];
        let mut blk2_b = [0u8; 0x38];
        for (b1, b2) in [(&mut blk1_a, &mut blk2_a), (&mut blk1_b, &mut blk2_b)] {
            wr::<u32>(b1.as_mut_ptr() as usize + 0x30, 3);
            wr::<u32>(b1.as_mut_ptr() as usize + 0x34, init_cnt);
            wr::<u32>(b2.as_mut_ptr() as usize + 0x30, 4);
            wr::<u32>(b2.as_mut_ptr() as usize + 0x34, 0);
        }

        let mut active_arr_a = [0usize; 4];
        let mut active_arr_b = [0usize; 4];
        let free_arr_a = [blk2_a.as_mut_ptr() as usize; 2];
        let free_arr_b = [blk2_b.as_mut_ptr() as usize; 2];

        let mut mgr_a = [0u8; 0xc0];
        let mut mgr_b = [0u8; 0xc0];
        for (m, b1, act_arr, fr_arr) in [
            (&mut mgr_a, &blk1_a, &mut active_arr_a, &free_arr_a),
            (&mut mgr_b, &blk1_b, &mut active_arr_b, &free_arr_b),
        ] {
            let mp = m.as_mut_ptr() as usize;
            wr::<u32>(mp + 0x10, 0x4242);
            wr::<u32>(mp + 0x14, 1);
            // free_q for qtype=0 (`mgr + 0`): arr at `+0x18`, head at `+0x20`, count at `+0x28`, cap at `+0x2c`
            wr::<usize>(mp + 0x18, fr_arr.as_ptr() as usize);
            wr::<u32>(mp + 0x20, 0);
            wr::<u32>(mp + 0x28, 1);
            wr::<u32>(mp + 0x2c, 2);
            // active_q for qtype=0 (`mgr + 3 * 24 = mgr + 0x48`): arr at `+0x00`, tail at `+0x0c`, count at `+0x10`, cap at `+0x14`
            let aq = mp + 0x48;
            wr::<usize>(aq, act_arr.as_mut_ptr() as usize);
            wr::<u32>(aq + 0x0c, 0);
            wr::<u32>(aq + 0x10, 0);
            wr::<u32>(aq + 0x14, 4);
            // cur_slot for qtype=0 (`mgr + 0xa8`)
            wr::<usize>(mp + 0xa8, b1.as_ptr() as usize);
        }

        let mut out_a = [0u8; 16];
        let mut out_b = [0u8; 16];

        wr::<usize>(mgr_va, mgr_a.as_mut_ptr() as usize);
        let ra = o(0, out_a.as_mut_ptr() as usize, 0) - out_a.as_ptr() as usize;

        wr::<usize>(mgr_va, mgr_b.as_mut_ptr() as usize);
        let rb = n(0, out_b.as_mut_ptr() as usize, 0) - out_b.as_ptr() as usize;

        let expected_blk_a = if init_cnt == 0x80 {
            blk2_a.as_ptr() as usize
        } else {
            blk1_a.as_ptr() as usize
        };
        let expected_blk_b = if init_cnt == 0x80 {
            blk2_b.as_ptr() as usize
        } else {
            blk1_b.as_ptr() as usize
        };
        wr::<usize>(
            out_a.as_mut_ptr() as usize,
            rd::<usize>(out_a.as_ptr() as usize).wrapping_sub(expected_blk_a),
        );
        wr::<usize>(
            out_b.as_mut_ptr() as usize,
            rd::<usize>(out_b.as_ptr() as usize).wrapping_sub(expected_blk_b),
        );

        crate::diff_eq!(ra, rb, "ret init_cnt={init_cnt}");
        crate::diff_eq!(out_a, out_b, "out_slot init_cnt={init_cnt}");
        crate::diff_eq!(blk1_a, blk1_b, "blk1 init_cnt={init_cnt}");
        crate::diff_eq!(blk2_a, blk2_b, "blk2 init_cnt={init_cnt}");
    }

    wr::<usize>(mgr_va, saved_mgr);
    Ok(2)
}

pub unsafe fn diff_d3d_query_read_slot(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> i32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut res_vtbl = [0usize; 12];
    res_vtbl[0x40 / 8] = fake_query_res_map as *const () as usize;
    res_vtbl[0x48 / 8] = fake_query_res_unmap as *const () as usize;
    let slots_data: [u64; 4] = [0x11, 0x22, 0x33, 0x4444_5555_6666_7777];

    let mut res_a = [0u64; 8];
    let mut res_b = [0u64; 8];
    for r in [&mut res_a, &mut res_b] {
        let rp = r.as_mut_ptr() as usize;
        wr::<usize>(rp, res_vtbl.as_ptr() as usize);
        wr::<i32>(rp + 0x08, 0);
        wr::<usize>(rp + 0x10, slots_data.as_ptr() as usize);
    }
    let mut blk_a = [0u8; 0x38];
    let mut blk_b = [0u8; 0x38];
    wr::<usize>(blk_a.as_mut_ptr() as usize + 0x28, res_a.as_mut_ptr() as usize);
    wr::<usize>(blk_b.as_mut_ptr() as usize + 0x28, res_b.as_mut_ptr() as usize);

    let mut q_a = [0u8; 0x18];
    let mut q_b = [0u8; 0x18];
    wr::<usize>(q_a.as_mut_ptr() as usize, blk_a.as_ptr() as usize);
    wr::<i32>(q_a.as_mut_ptr() as usize + 0x08, 3);
    wr::<usize>(q_b.as_mut_ptr() as usize, blk_b.as_ptr() as usize);
    wr::<i32>(q_b.as_mut_ptr() as usize + 0x08, 3);

    let (mut oa, mut ob) = (0u64, 0u64);
    let ra = o(q_a.as_ptr() as usize, &mut oa as *mut u64 as usize);
    let rb = n(q_b.as_ptr() as usize, &mut ob as *mut u64 as usize);
    crate::diff_eq!(ra, rb, "hr");
    crate::diff_eq!(oa, ob, "out_val");
    crate::diff_eq!(&res_a[2..], &res_b[2..], "res state");
    Ok(1)
}

pub unsafe fn diff_d3d_query_is_fence_complete(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> bool;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut fence_vtbl = [0usize; 10];
    fence_vtbl[0x40 / 8] = fake_fence_get_completed as *const () as usize;

    for (target_val, completed_val) in [(10u64, 9u64), (10, 10), (10, 15)] {
        let fence = [fence_vtbl.as_ptr() as usize as u64, completed_val];
        let mut blk = [0u8; 0x20];
        wr::<u64>(blk.as_mut_ptr() as usize, target_val);
        wr::<usize>(blk.as_mut_ptr() as usize + 0x10, fence.as_ptr() as usize);
        let q: [usize; 2] = [blk.as_ptr() as usize, 0];

        let ra = o(q.as_ptr() as usize);
        let rb = n(q.as_ptr() as usize);
        crate::diff_eq!(ra, rb, "target={target_val} completed={completed_val}");
    }
    Ok(3)
}

// ============================================================================
// 26..30: FSR2Glue upscale/lambda/exec/FG wrappers & FrameRateHistory::init
// ============================================================================

static mut FFX_CAPTURE_BUF: [u8; 0x200] = [0; 0x200];
static mut FFX_CFG_CAPTURE: [u8; 0x90] = [0; 0x90];
static mut DEPTH_SLOT_PTR: usize = 0;

unsafe extern "win64" fn fake_barrier_93a0(res: usize) {
    wr::<u32>(res + 0x08, rd::<u32>(res + 0x08).wrapping_add(0x93a0));
}

unsafe extern "win64" fn fake_barrier_94a0(res: usize) {
    wr::<u32>(res + 0x0c, rd::<u32>(res + 0x0c).wrapping_add(0x94a0));
}

unsafe extern "win64" fn fake_ts_mark_2ec0(p: usize) {
    wr::<u32>(p, 0x2ec0_cafe);
}

unsafe extern "win64" fn fake_ts_read_34e0(out16: usize, flag: u32) -> usize {
    wr::<u64>(out16, 0x34e0_1111_2222_3333);
    wr::<u64>(out16 + 0x08, flag as u64);
    out16
}

unsafe extern "win64" fn fake_ffx_dispatch(_ctx: usize, disp_ptr: usize) -> u32 {
    let ty = rd::<u64>(disp_ptr);
    let len = if ty == 0x10001 { 0x1b0usize } else { 0x140usize };
    for i in 0..len {
        FFX_CAPTURE_BUF[i] = rd::<u8>(disp_ptr + i);
    }
    if ty == 0x10001 {
        // Normalize struct padding bytes in ffxDispatchDescUpscale
        for b in &mut FFX_CAPTURE_BUF[0x189..0x18c] {
            *b = 0;
        }
        for b in &mut FFX_CAPTURE_BUF[0x199..0x19c] {
            *b = 0;
        }
        for b in &mut FFX_CAPTURE_BUF[0x1a8..0x1ac] {
            *b = 0;
        }
    }
    0
}

unsafe extern "win64" fn fake_ffx_configure(_ctx: usize, cfg_ptr: usize) -> u32 {
    for i in 0..0x90usize {
        FFX_CFG_CAPTURE[i] = rd::<u8>(cfg_ptr + i);
    }
    // Clear stack pointer at +0x08 (ui_desc pointer) after verifying non-null bit
    let has_ui = rd::<usize>(cfg_ptr + 0x08) != 0;
    wr::<usize>((FFX_CFG_CAPTURE.as_mut_ptr() as usize) + 0x08, usize::from(has_ui));
    0
}

unsafe extern "win64" fn fake_ffx_query_fg(_ctx: usize, q_ptr: usize) -> u32 {
    let ty = rd::<u64>(q_ptr);
    let out_ptr = rd::<usize>(q_ptr + 0x10);
    if ty == 0x30003 {
        wr::<u64>(out_ptr, 0xcccc_3003);
    } else if ty == 0x30004 {
        for i in 0..6usize {
            wr::<u64>(out_ptr + i * 8, 0xdddd_3004_0000_0000 + i as u64);
        }
    }
    0
}

unsafe extern "win64" fn fake_get_depth_slot_f1b0(_pass_data: usize) -> usize {
    DEPTH_SLOT_PTR
}

pub unsafe fn diff_fsr2_glue_dispatch_upscale_wrapper(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
    );
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_93a0 = Mock::new(0x1_4206_93a0, fake_barrier_93a0 as *const ())?;
    let _m_94a0 = Mock::new(0x1_4206_94a0, fake_barrier_94a0 as *const ())?;
    let _m_5b00 = Mock::new(0x1_411d_5b00, fake_get_cmd_guard_5b00 as *const ())?;
    let _m_2ec0 = Mock::new(0x1_4207_2ec0, fake_ts_mark_2ec0 as *const ())?;
    let _m_34e0 = Mock::new(0x1_4207_34e0, fake_ts_read_34e0 as *const ())?;
    let _im_disp = IatMock::new(0x1_4264_6490, fake_ffx_dispatch as *const ())?;

    let snap_ts = Snapshot::take(exe::ptr::<u8>(0x1_43c4_8fc0) as *mut u8, 16);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut tls_buf = vec![0u8; 0x46800];
    tls_buf[0x70] = 1;
    wr::<usize>(tls_slot, tls_buf.as_mut_ptr() as usize);

    let mut res_a = [[0u8; 0x80]; 7];
    let mut res_b = [[0u8; 0x80]; 7];
    for set in [&mut res_a, &mut res_b] {
        wr::<u32>(set[1].as_mut_ptr() as usize, 1280);
        wr::<u32>(set[1].as_mut_ptr() as usize + 0x04, 720);
    }

    let mut lock_a = [0u32, 0, 0x4242, 2];
    let mut lock_b = [0u32, 0, 0x4242, 2];

    CMD_GUARD_LOCK_PTR = lock_a.as_mut_ptr() as usize;
    CMD_GUARD_LIST_PTR = 0x7777_8888;
    FFX_CAPTURE_BUF = [0; 0x200];
    o(
        res_a[0].as_mut_ptr() as usize,
        res_a[1].as_mut_ptr() as usize,
        res_a[2].as_mut_ptr() as usize,
        res_a[3].as_mut_ptr() as usize,
        res_a[4].as_mut_ptr() as usize,
        res_a[5].as_mut_ptr() as usize,
        res_a[6].as_mut_ptr() as usize,
        0x3f00_0000_bf00_0000,
        0.016f32.to_bits() as u64,
        0.1f32.to_bits() as u64,
        1000.0f32.to_bits() as u64,
        1.0f32.to_bits() as u64,
        1,
    );
    let cap_a = FFX_CAPTURE_BUF;

    CMD_GUARD_LOCK_PTR = lock_b.as_mut_ptr() as usize;
    CMD_GUARD_LIST_PTR = 0x7777_8888;
    FFX_CAPTURE_BUF = [0; 0x200];
    n(
        res_b[0].as_mut_ptr() as usize,
        res_b[1].as_mut_ptr() as usize,
        res_b[2].as_mut_ptr() as usize,
        res_b[3].as_mut_ptr() as usize,
        res_b[4].as_mut_ptr() as usize,
        res_b[5].as_mut_ptr() as usize,
        res_b[6].as_mut_ptr() as usize,
        0x3f00_0000_bf00_0000,
        0.016f32.to_bits() as u64,
        0.1f32.to_bits() as u64,
        1000.0f32.to_bits() as u64,
        1.0f32.to_bits() as u64,
        1,
    );
    let cap_b = FFX_CAPTURE_BUF;

    wr::<usize>(tls_slot, saved_tls0);
    snap_ts.restore();

    crate::diff_eq!(&cap_a[..0x1b0], &cap_b[..0x1b0], "upscale disp struct");
    crate::diff_eq!(res_a, res_b, "resource barrier counts");
    crate::diff_eq!(lock_a, lock_b, "cmd lock");
    Ok(1)
}

fn test_fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= (b as i8 as i64) as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

pub unsafe fn diff_fsr2_glue_lambda_call(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_93a0 = Mock::new(0x1_4206_93a0, fake_barrier_93a0 as *const ())?;
    let _m_94a0 = Mock::new(0x1_4206_94a0, fake_barrier_94a0 as *const ())?;
    let _m_5b00 = Mock::new(0x1_411d_5b00, fake_get_cmd_guard_5b00 as *const ())?;
    let _m_2ec0 = Mock::new(0x1_4207_2ec0, fake_ts_mark_2ec0 as *const ())?;
    let _m_34e0 = Mock::new(0x1_4207_34e0, fake_ts_read_34e0 as *const ())?;
    let _m_f1b0 = Mock::new(0x1_412f_f1b0, fake_get_depth_slot_f1b0 as *const ())?;
    let _im_disp = IatMock::new(0x1_4264_6490, fake_ffx_dispatch as *const ())?;

    let snap_ts = Snapshot::take(exe::ptr::<u8>(0x1_43c4_8fc0) as *mut u8, 16);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut tls_buf = vec![0u8; 0x46800];
    tls_buf[0x70] = 1;
    wr::<usize>(tls_slot, tls_buf.as_mut_ptr() as usize);

    let mut res_a = [[0u8; 0x80]; 7];
    let mut res_b = [[0u8; 0x80]; 7];
    for set in [&mut res_a, &mut res_b] {
        wr::<u32>(set[1].as_mut_ptr() as usize, 1920);
        wr::<u32>(set[1].as_mut_ptr() as usize + 0x04, 1080);
    }

    let names: [&[u8]; 5] = [
        b"fsr2_compmask",
        b"FSR2_InputReactiveMap",
        b"velocity_geometry_buffer",
        b"colorbufferIN",
        b"colorbufferOUT",
    ];

    let mut slots_a = [
        res_a[5].as_mut_ptr() as usize,
        res_a[4].as_mut_ptr() as usize,
        res_a[2].as_mut_ptr() as usize,
        res_a[1].as_mut_ptr() as usize,
        res_a[0].as_mut_ptr() as usize,
        res_a[3].as_mut_ptr() as usize,
    ];
    let mut slots_b = [
        res_b[5].as_mut_ptr() as usize,
        res_b[4].as_mut_ptr() as usize,
        res_b[2].as_mut_ptr() as usize,
        res_b[1].as_mut_ptr() as usize,
        res_b[0].as_mut_ptr() as usize,
        res_b[3].as_mut_ptr() as usize,
    ];

    let mut pass_a = [0u8; 0x410];
    let mut pass_b = [0u8; 0x410];
    for (pass, slots) in [(&mut pass_a, &mut slots_a), (&mut pass_b, &mut slots_b)] {
        let pp = pass.as_mut_ptr() as usize;
        for (i, name) in names.iter().enumerate() {
            wr::<usize>(pp + 0x08 + i * 8, &mut slots[i] as *mut usize as usize);
            wr::<u64>(pp + 0x208 + i * 8, test_fnv1a64(name));
        }
    }

    let mut ae_a = [0usize; 6];
    let mut ae_b = [0usize; 6];
    ae_a[4] = res_a[6].as_mut_ptr() as usize;
    ae_a[5] = res_a[6].as_mut_ptr() as usize;
    ae_b[4] = res_b[6].as_mut_ptr() as usize;
    ae_b[5] = res_b[6].as_mut_ptr() as usize;

    let mut lambda_a = [0u8; 0x18];
    let mut lambda_b = [0u8; 0x18];
    for (lam, ae) in [(&mut lambda_a, &ae_a), (&mut lambda_b, &ae_b)] {
        let lp = lam.as_mut_ptr() as usize;
        wr::<u8>(lp, 1);
        wr::<u8>(lp + 0x01, 0);
        wr::<u64>(lp + 0x04, 0x3e80_0000_be80_0000);
        wr::<usize>(lp + 0x10, ae.as_ptr() as usize);
    }

    let mut lock_a = [0u32, 0, 0x4242, 2];
    let mut lock_b = [0u32, 0, 0x4242, 2];

    DEPTH_SLOT_PTR = &mut slots_a[5] as *mut usize as usize;
    CMD_GUARD_LOCK_PTR = lock_a.as_mut_ptr() as usize;
    CMD_GUARD_LIST_PTR = 0x5555_6666;
    FFX_CAPTURE_BUF = [0; 0x200];
    o(lambda_a.as_ptr() as usize, pass_a.as_ptr() as usize);
    let cap_a = FFX_CAPTURE_BUF;

    DEPTH_SLOT_PTR = &mut slots_b[5] as *mut usize as usize;
    CMD_GUARD_LOCK_PTR = lock_b.as_mut_ptr() as usize;
    CMD_GUARD_LIST_PTR = 0x5555_6666;
    FFX_CAPTURE_BUF = [0; 0x200];
    n(lambda_b.as_ptr() as usize, pass_b.as_ptr() as usize);
    let cap_b = FFX_CAPTURE_BUF;

    wr::<usize>(tls_slot, saved_tls0);
    snap_ts.restore();

    crate::diff_eq!(&cap_a[..0x1b0], &cap_b[..0x1b0], "lambda disp struct");
    crate::diff_eq!(res_a, res_b, "lambda barrier counts");
    crate::diff_eq!(lock_a, lock_b, "lambda cmd lock");
    Ok(1)
}

unsafe extern "win64" fn fake_enqueue_pass_cb_cba0(pass_target: usize, f_ptr: usize) {
    wr::<usize>(pass_target, rd::<usize>(f_ptr));
    wr::<u8>(pass_target + 0x08, rd::<u8>(f_ptr + 0x08));
    wr::<u8>(pass_target + 0x09, rd::<u8>(f_ptr + 0x09));
    wr::<u64>(pass_target + 0x10, rd::<u64>(f_ptr + 0x0c));
    wr::<u64>(pass_target + 0x18, rd::<u64>(f_ptr + 0x18));
    wr::<u8>(
        pass_target + 0x20,
        u8::from(rd::<usize>(f_ptr + 0x38) == f_ptr),
    );
    wr::<usize>(f_ptr + 0x38, 0);
}

pub unsafe fn diff_fsr2_glue_exec_wrapper(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_enq = Mock::new(0x1_4128_cba0, fake_enqueue_pass_cb_cba0 as *const ())?;

    let idx_va = exe::ptr::<i32>(0x1_43bb_d168) as usize;
    let base_va = exe::ptr::<usize>(0x1_43bb_d180) as usize;
    let saved_idx = rd::<i32>(idx_va);
    let saved_base = rd::<usize>(base_va);

    let mut passes_a = [0u8; 0x300];
    let mut passes_b = [0u8; 0x300];
    wr::<i32>(idx_va, 0);

    wr::<usize>(base_va, passes_a.as_mut_ptr() as usize);
    o(0, 0x1111_2222_3333_4444, 0x5555_6666_7777_8888, 1);

    wr::<usize>(base_va, passes_b.as_mut_ptr() as usize);
    n(0, 0x1111_2222_3333_4444, 0x5555_6666_7777_8888, 1);

    wr::<i32>(idx_va, saved_idx);
    wr::<usize>(base_va, saved_base);

    crate::diff_eq!(&passes_a[0x10..0x38], &passes_b[0x10..0x38], "enqueued pass callback");
    Ok(1)
}

pub unsafe fn diff_fsr2_glue_dispatch_fg(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, usize, usize, u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let _m_2ec0 = Mock::new(0x1_4207_2ec0, fake_ts_mark_2ec0 as *const ())?;
    let _m_34e0 = Mock::new(0x1_4207_34e0, fake_ts_read_34e0 as *const ())?;
    let _im_cfg = IatMock::new(0x1_4264_64a8, fake_ffx_configure as *const ())?;
    let _im_qry = IatMock::new(0x1_4264_6488, fake_ffx_query_fg as *const ())?;
    let _im_disp = IatMock::new(0x1_4264_6490, fake_ffx_dispatch as *const ())?;

    let snap_cfg = Snapshot::take(exe::ptr::<u8>(0x1_4385_7d10) as *mut u8, 0xc0);
    let snap_ts = Snapshot::take(exe::ptr::<u8>(0x1_43c4_8fc0) as *mut u8, 16);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let mut tls_buf = vec![0u8; 0x46800];
    tls_buf[0x70] = 1;
    wr::<usize>(tls_slot, tls_buf.as_mut_ptr() as usize);

    let sc_inner: [usize; 3] = [0, 0x1234_0008, 0x5678_0010];
    let bb_res = [0u8; 0x80];
    let hudless_res = [0u8; 0x80];
    let ui_res = [0u8; 0x80];

    let mut sc = [0u8; 0x70];
    let scp = sc.as_mut_ptr() as usize;
    wr::<usize>(scp, sc_inner.as_ptr() as usize);
    wr::<usize>(scp + 0x08, bb_res.as_ptr() as usize);
    wr::<i32>(scp + 0x38, 0);
    wr::<u8>(scp + 0x4c, 1);
    wr::<f32>(scp + 0x50, 80.0);
    wr::<u8>(scp + 0x66, 1);

    let fg_en_va = exe::ptr::<u8>(0x1_4385_7dc8) as usize;
    for fg_en in [0u8, 1u8] {
        wr::<u8>(fg_en_va, fg_en);
        FFX_CAPTURE_BUF = [0; 0x200];
        FFX_CFG_CAPTURE = [0; 0x90];
        o(scp, 42, hudless_res.as_ptr() as usize, ui_res.as_ptr() as usize, 1);
        let disp_a = FFX_CAPTURE_BUF;
        let cfg_a = FFX_CFG_CAPTURE;

        FFX_CAPTURE_BUF = [0; 0x200];
        FFX_CFG_CAPTURE = [0; 0x90];
        n(scp, 42, hudless_res.as_ptr() as usize, ui_res.as_ptr() as usize, 1);
        let disp_b = FFX_CAPTURE_BUF;
        let cfg_b = FFX_CFG_CAPTURE;

        crate::diff_eq!(&disp_a[..0x140], &disp_b[..0x140], "fg disp fg_en={fg_en}");
        crate::diff_eq!(cfg_a, cfg_b, "fg cfg fg_en={fg_en}");
    }

    wr::<usize>(tls_slot, saved_tls0);
    snap_ts.restore();
    snap_cfg.restore();
    Ok(2)
}

unsafe extern "win64" fn fake_alloc_f64_16_ec00(this: usize, count: usize) -> usize {
    wr::<usize>(this + 0x28, count);
    rd::<usize>(this + 0x20)
}

pub unsafe fn diff_frame_rate_history_init(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(0x1_401d_ec00, fake_alloc_f64_16_ec00 as *const ())?;

    let mut buf_a = [0xaa_u8; 16 * 8];
    let mut buf_b = [0xaa_u8; 16 * 8];
    let mut hist_a = [0xff_u8; 0x30];
    let mut hist_b = [0xff_u8; 0x30];
    wr::<usize>(hist_a.as_mut_ptr() as usize + 0x20, buf_a.as_mut_ptr() as usize);
    wr::<usize>(hist_b.as_mut_ptr() as usize + 0x20, buf_b.as_mut_ptr() as usize);

    let ra = o(hist_a.as_mut_ptr() as usize) - hist_a.as_ptr() as usize;
    let rb = n(hist_b.as_mut_ptr() as usize) - hist_b.as_ptr() as usize;

    wr::<usize>(
        hist_a.as_mut_ptr() as usize + 0x10,
        rd::<usize>(hist_a.as_ptr() as usize + 0x10).wrapping_sub(buf_a.as_ptr() as usize),
    );
    wr::<usize>(
        hist_b.as_mut_ptr() as usize + 0x10,
        rd::<usize>(hist_b.as_ptr() as usize + 0x10).wrapping_sub(buf_b.as_ptr() as usize),
    );

    crate::diff_eq!(ra, rb, "ret");
    crate::diff_eq!(hist_a[..0x20], hist_b[..0x20], "hist struct");
    crate::diff_eq!(rd::<usize>(hist_a.as_ptr() as usize + 0x28), rd::<usize>(hist_b.as_ptr() as usize + 0x28), "alloc count");
    crate::diff_eq!(buf_a, buf_b, "zeroed 16 f64 slots");
    Ok(1)
}

// REGISTRY: Replacement::verified(0x142060790, "d3d::ComputeShader::dtor_142060790", rend::fsr2_glue::d3d_compute_shader_dtor as *const (), rend::fsr2_glue::diff_d3d_compute_shader_dtor),
// REGISTRY: Replacement::verified(0x1420610d0, "d3d::comHolderB0DeletingDtor_1420610d0", rend::fsr2_glue::d3d_com_holder_b0_deleting_dtor as *const (), rend::fsr2_glue::diff_d3d_com_holder_b0_deleting_dtor),
// REGISTRY: Replacement::verified(0x142061120, "d3d::ShaderReflection::getThreadGroupSize_142061120", rend::fsr2_glue::d3d_shader_reflection_get_thread_group_size as *const (), rend::fsr2_glue::diff_d3d_shader_reflection_get_thread_group_size),
// REGISTRY: Replacement::verified_throws(0x142061180, "d3d::ShaderReflection::getInputParameterDesc_142061180", rend::fsr2_glue::d3d_shader_reflection_get_input_param_desc as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_shader_reflection_get_input_param_desc),
// REGISTRY: Replacement::verified(0x142061270, "d3d::ShaderReflection::dtor_142061270", rend::fsr2_glue::d3d_shader_reflection_dtor as *const (), rend::fsr2_glue::diff_d3d_shader_reflection_dtor),
// REGISTRY: Replacement::verified(0x1420612b0, "d3d::ShaderReflection::deletingDtor_1420612b0", rend::fsr2_glue::d3d_shader_reflection_deleting_dtor as *const (), rend::fsr2_glue::diff_d3d_shader_reflection_deleting_dtor),
// REGISTRY: Replacement::verified(0x1420613d0, "d3d::recreateSamplerDescriptor_1420613d0", rend::fsr2_glue::d3d_recreate_sampler_descriptor as *const (), rend::fsr2_glue::diff_d3d_recreate_sampler_descriptor),
// REGISTRY: Replacement::verified(0x142061690, "d3d::dispatchRays1_142061690", rend::fsr2_glue::d3d_dispatch_rays_1 as *const (), rend::fsr2_glue::diff_d3d_dispatch_rays_1),
// REGISTRY: Replacement::verified(0x1420616e0, "d3d::dispatchRays4_1420616e0", rend::fsr2_glue::d3d_dispatch_rays_4 as *const (), rend::fsr2_glue::diff_d3d_dispatch_rays_4),
// REGISTRY: Replacement::verified(0x142061740, "d3d::dispatchRays3_142061740", rend::fsr2_glue::d3d_dispatch_rays_3 as *const (), rend::fsr2_glue::diff_d3d_dispatch_rays_3),
// REGISTRY: Replacement::verified(0x142061a90, "d3d::dispatchRaysCommon_142061a90", rend::fsr2_glue::d3d_dispatch_rays_common as *const (), rend::fsr2_glue::diff_d3d_dispatch_rays_common),
// REGISTRY: Replacement::verified_throws(0x142061df0, "d3d::RtCompactionPool::unmap_142061df0", rend::fsr2_glue::d3d_rt_compaction_pool_unmap as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_rt_compaction_pool_unmap),
// REGISTRY: Replacement::verified_throws(0x142061e20, "d3d::RtCompactionPool::map_142061e20", rend::fsr2_glue::d3d_rt_compaction_pool_map as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_rt_compaction_pool_map),
// REGISTRY: Replacement::verified(0x142061e60, "d3d::RtCompactionPool::resolve_142061e60", rend::fsr2_glue::d3d_rt_compaction_pool_resolve as *const (), rend::fsr2_glue::diff_d3d_rt_compaction_pool_resolve),
// REGISTRY: Replacement::verified_throws(0x142063180, "d3d::Query::tryGetResult_142063180", rend::fsr2_glue::d3d_query_try_get_result as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_query_try_get_result),
// REGISTRY: Replacement::verified_throws(0x142063200, "d3d::Query::begin_142063200", rend::fsr2_glue::d3d_query_begin as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_query_begin),
// REGISTRY: Replacement::verified_throws(0x1420632b0, "d3d::Query::end_1420632b0", rend::fsr2_glue::d3d_query_end as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_query_end),
// REGISTRY: Replacement::verified_throws(0x142063360, "d3d::Query::ctor_142063360", rend::fsr2_glue::d3d_query_ctor as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_query_ctor),
// REGISTRY: Replacement::verified_throws(0x142063410, "d3d::QueryHeapManager::vfunc_1", rend::fsr2_glue::d3d_query_heap_mgr_vfunc_1 as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_query_heap_mgr_vfunc_1),
// REGISTRY: Replacement::verified(0x142063ab0, "d3d::QueryBlock::resolve_142063ab0", rend::fsr2_glue::d3d_query_block_resolve as *const (), rend::fsr2_glue::diff_d3d_query_block_resolve),
// REGISTRY: Replacement::verified(0x142063b40, "d3d::QueryBlock::readSlot_142063b40", rend::fsr2_glue::d3d_query_block_read_slot as *const (), rend::fsr2_glue::diff_d3d_query_block_read_slot),
// REGISTRY: Replacement::verified(0x142063bf0, "d3d::QueryBlock::dtor_142063bf0", rend::fsr2_glue::d3d_query_block_dtor as *const (), rend::fsr2_glue::diff_d3d_query_block_dtor),
// REGISTRY: Replacement::verified_throws(0x142063e60, "d3d::QueryHeapManager::allocSlot_142063e60", rend::fsr2_glue::d3d_query_heap_mgr_alloc_slot as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_query_heap_mgr_alloc_slot),
// REGISTRY: Replacement::verified(0x142063fe0, "d3d::Query::readSlot_142063fe0", rend::fsr2_glue::d3d_query_read_slot as *const (), rend::fsr2_glue::diff_d3d_query_read_slot),
// REGISTRY: Replacement::verified_throws(0x142064090, "d3d::Query::isFenceComplete_142064090", rend::fsr2_glue::d3d_query_is_fence_complete as *const (), Throws::Discard, rend::fsr2_glue::diff_d3d_query_is_fence_complete),
// REGISTRY: Replacement::verified_throws(0x14238ce70, "rend::FSR2Glue::dispatchUpscaleWrapper_14238ce70", rend::fsr2_glue::fsr2_glue_dispatch_upscale_wrapper as *const (), Throws::Discard, rend::fsr2_glue::diff_fsr2_glue_dispatch_upscale_wrapper),
// REGISTRY: Replacement::verified_throws(0x14238cf90, "rend::FSR2Glue::execWrapper::<lambda_1>::operator()_14238cf90", rend::fsr2_glue::fsr2_glue_lambda_call as *const (), Throws::Discard, rend::fsr2_glue::diff_fsr2_glue_lambda_call),
// REGISTRY: Replacement::verified(0x14238d330, "rend::FSR2Glue::execWrapper_14238d330", rend::fsr2_glue::fsr2_glue_exec_wrapper as *const (), rend::fsr2_glue::diff_fsr2_glue_exec_wrapper),
// REGISTRY: Replacement::verified_throws(0x14238d400, "rend::FSR2Glue::dispatchFrameGen_14238d400", rend::fsr2_glue::fsr2_glue_dispatch_fg as *const (), Throws::Discard, rend::fsr2_glue::diff_fsr2_glue_dispatch_fg),
// REGISTRY: Replacement::verified(0x14238d830, "rend::FrameRateHistory::init_14238d830", rend::fsr2_glue::frame_rate_history_init as *const (), rend::fsr2_glue::diff_frame_rate_history_init),


