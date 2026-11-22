use crate::exe::{self, rd, wr};
use std::sync::atomic::{AtomicU8, Ordering};

/// Original: `0x14207d340` (93 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_reserve_async_lambda_clone(rcx: usize) -> usize {
    let rbx = rcx;
    let rax = exe::call_va!(0x1_4183_4d64, (0x30usize) -> usize);
    let vtbl = exe::ptr::<u8>(0x1_433d_cd98) as usize;
    wr::<usize>(rax, vtbl);
    wr::<u64>(rax + 0x8, rd::<u64>(rbx + 0x8));
    wr::<u64>(rax + 0x10, 0);
    wr::<u64>(rax + 0x18, 0);
    let f18 = rd::<u64>(rbx + 0x18);
    wr::<u64>(rax + 0x18, f18);
    let f10 = rd::<u64>(rbx + 0x10);
    wr::<u64>(rax + 0x20, 0);
    wr::<u64>(rax + 0x28, 0);
    wr::<u64>(rax + 0x10, f10);
    let f28 = rd::<u64>(rbx + 0x28);
    wr::<u64>(rax + 0x28, f28);
    let f20 = rd::<u64>(rbx + 0x20);
    wr::<u64>(rax + 0x20, f20);
    rax
}

/// Original: `0x14207d3c0` (53 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_reserve_async_lambda_call(rcx: usize) -> usize {
    let rbx = rd::<usize>(rcx + 0x8);
    let rdi = rcx;
    let arg0 = rd::<usize>(rdi + 0x10);
    let fn0 = rd::<usize>(rdi + 0x18);
    exe::call_va!(fn0, (arg0) -> ());
    exe::call_va!(0x1_4207_ec20, (rbx) -> ());
    let arg1 = rd::<usize>(rdi + 0x20);
    let fn1 = rd::<usize>(rdi + 0x28);
    exe::call_va!(fn1, (arg1) -> usize)
}

/// Original: `0x14207d440` (83 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_try_commit_async_lambda_clone(rcx: usize) -> usize {
    let rbx = rcx;
    let rax = exe::call_va!(0x1_4183_4d64, (0x28usize) -> usize);
    let vtbl = exe::ptr::<u8>(0x1_433d_cdd0) as usize;
    wr::<usize>(rax, vtbl);
    wr::<u64>(rax + 0x8, rd::<u64>(rbx + 0x8));
    wr::<u8>(rax + 0x10, rd::<u8>(rbx + 0x10));
    let b11 = rd::<u8>(rbx + 0x11);
    wr::<u64>(rax + 0x18, 0);
    wr::<u64>(rax + 0x20, 0);
    wr::<u8>(rax + 0x11, b11);
    wr::<u64>(rax + 0x20, rd::<u64>(rbx + 0x20));
    wr::<u64>(rax + 0x18, rd::<u64>(rbx + 0x18));
    rax
}

/// Original: `0x14207d4b0` (59 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_try_commit_async_lambda_call(
    rcx: usize,
    rdx: usize,
) {
    let mut cb = [0u64; 2];
    cb[1] = rd::<u64>(rcx + 0x20);
    let r9 = rdx.wrapping_add(0x20);
    let r8d = rd::<u8>(rcx + 0x10) as u32;
    let edx = rd::<u8>(rcx + 0x11) as u32;
    cb[0] = rd::<u64>(rcx + 0x18);
    let target_rcx = rd::<usize>(rcx + 0x8);
    exe::call_va!(
        0x1_4207_da50,
        (target_rcx, edx, r8d, r9, cb.as_mut_ptr() as usize) -> ()
    );
}

/// Original: `0x14207d540` (76 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_commit_async_lambda_clone(rcx: usize) -> usize {
    let rbx = rcx;
    let rax = exe::call_va!(0x1_4183_4d64, (0x28usize) -> usize);
    let vtbl = exe::ptr::<u8>(0x1_433d_ce08) as usize;
    wr::<usize>(rax, vtbl);
    wr::<u64>(rax + 0x8, rd::<u64>(rbx + 0x8));
    let b10 = rd::<u8>(rbx + 0x10);
    wr::<u64>(rax + 0x18, 0);
    wr::<u64>(rax + 0x20, 0);
    wr::<u8>(rax + 0x10, b10);
    wr::<u64>(rax + 0x20, rd::<u64>(rbx + 0x20));
    wr::<u64>(rax + 0x18, rd::<u64>(rbx + 0x18));
    rax
}

/// Original: `0x14207d5b0` (36 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_commit_async_lambda_call(rcx: usize) -> usize {
    let idx = rd::<u8>(rcx + 0x10) as usize;
    let base = rd::<usize>(rcx + 0x8);
    let state_addr = base.wrapping_add(idx).wrapping_add(0x50);
    AtomicU8::from_ptr(state_addr as *mut u8).swap(2, Ordering::SeqCst);
    let edx = rd::<u8>(rcx + 0x10) as u32;
    let cb_ctx = rd::<usize>(rcx + 0x18);
    let cb_fn = rd::<usize>(rcx + 0x20);
    exe::call_va!(cb_fn, (cb_ctx, edx, 1u32) -> usize)
}

/// Original: `0x14207d620` (95 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_calc_committed_bytes(rcx: usize) {
    let mut rdx = rd::<usize>(rcx + 0x8);
    let mut r9 = 0u64;
    let count = rd::<u32>(rcx + 0x68) as usize;
    let end = rdx.wrapping_add(count << 5);
    while rdx != end {
        if rd::<u8>(rdx + 0x1f) == 2 {
            let w1 = rd::<u16>(rdx + 0x18);
            let w0 = rd::<u16>(rdx + 0x16);
            let p0 = w0.wrapping_mul(w1);
            let w2 = rd::<u16>(rdx + 0x1a);
            let p1 = p0.wrapping_mul(w2);
            r9 = r9.wrapping_add(p1 as u64);
        }
        rdx = rdx.wrapping_add(0x20);
    }
    wr::<u64>(rcx + 0x60, r9 << 16);
}

pub unsafe fn diff_d3d_streamed_tex_calc_committed_bytes(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut tiles = [0u8; 0x64];
    let tiles_base = tiles.as_mut_ptr().add(4) as usize;
    // Tile 0: state == 2, dims = (3, 4, 2) -> 24
    wr::<u16>(tiles_base + 0x16, 3);
    wr::<u16>(tiles_base + 0x18, 4);
    wr::<u16>(tiles_base + 0x1a, 2);
    wr::<u8>(tiles_base + 0x1f, 2);
    // Tile 1: state == 1 (ignored), dims = (5, 5, 5)
    wr::<u16>(tiles_base + 0x36, 5);
    wr::<u16>(tiles_base + 0x38, 5);
    wr::<u16>(tiles_base + 0x3a, 5);
    wr::<u8>(tiles_base + 0x3f, 1);
    // Tile 2: state == 2, dims = (2, 3, 5) -> 30
    wr::<u16>(tiles_base + 0x56, 2);
    wr::<u16>(tiles_base + 0x58, 3);
    wr::<u16>(tiles_base + 0x5a, 5);
    wr::<u8>(tiles_base + 0x5f, 2);

    let mut buf_a = [0u8; 0x78];
    let mut buf_b = [0u8; 0x78];
    let a = buf_a.as_mut_ptr().add(4) as usize;
    let b = buf_b.as_mut_ptr().add(4) as usize;
    wr::<usize>(a + 0x8, tiles_base);
    wr::<u32>(a + 0x68, 3);
    wr::<usize>(b + 0x8, tiles_base);
    wr::<u32>(b + 0x68, 3);

    exe::call_va!(orig, (a) -> ());
    exe::call_va!(new, (b) -> ());
    crate::diff_eq!(rd::<u64>(a + 0x60), rd::<u64>(b + 0x60), "committed_bytes");
    Ok(1)
}

/// Original: `0x14207d690` (30 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_accum_tile_volume(
    _rcx: usize,
    rdx: u32,
    r8: usize,
) -> u32 {
    let w1 = rd::<u16>(r8 + 0x18);
    let w2 = rd::<u16>(r8 + 0x1a);
    let p0 = w2.wrapping_mul(w1);
    let w0 = rd::<u16>(r8 + 0x16);
    let p1 = p0.wrapping_mul(w0);
    (p1 as u32).wrapping_add(rdx)
}

pub unsafe fn diff_d3d_streamed_tex_accum_tile_volume(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf = [0u8; 0x28];
    let ptr = buf.as_mut_ptr().add(4) as usize;
    wr::<u16>(ptr + 0x16, 7);
    wr::<u16>(ptr + 0x18, 11);
    wr::<u16>(ptr + 0x1a, 3);
    let ra = exe::call_va!(orig, (0usize, 100u32, ptr) -> u32);
    let rb = exe::call_va!(new, (0usize, 100u32, ptr) -> u32);
    crate::diff_eq!(ra, rb, "accum_tile_volume");
    Ok(1)
}

/// Original: `0x14207d9f0` (39 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_commit_cb_init(
    rcx: usize,
    rdx: u64,
    r8: usize,
    r9: usize,
) -> usize {
    wr::<u64>(rcx, rdx);
    let b = rd::<u8>(r8);
    wr::<u8>(rcx + 0x8, b);
    wr::<u64>(rcx + 0x10, 0);
    wr::<u64>(rcx + 0x18, 0);
    let f8 = rd::<u64>(r9 + 0x8);
    wr::<u64>(rcx + 0x18, f8);
    let f0 = rd::<u64>(r9);
    wr::<u64>(rcx + 0x10, f0);
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_commit_cb_init(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut out_a = [0u8; 0x28];
    let mut out_b = [0u8; 0x28];
    let a = out_a.as_mut_ptr().add(4) as usize;
    let b = out_b.as_mut_ptr().add(4) as usize;
    let byte_val: u8 = 5;
    let cb_src: [u64; 2] = [0x1122_3344_5566_7788, 0x99aa_bbcc_ddee_ff00];
    let ra = exe::call_va!(
        orig,
        (a, 0xdead_beef_cafe_babeu64, &byte_val as *const u8 as usize, cb_src.as_ptr() as usize) -> usize
    );
    let rb = exe::call_va!(
        new,
        (b, 0xdead_beef_cafe_babeu64, &byte_val as *const u8 as usize, cb_src.as_ptr() as usize) -> usize
    );
    crate::diff_eq!(ra, a, "ret_a");
    crate::diff_eq!(rb, b, "ret_b");
    crate::diff_eq!(out_a, out_b, "commit_cb_init");
    Ok(1)
}

/// Original: `0x14207da20` (35 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_commit_cb_invoke(rcx: usize) -> usize {
    let idx = rd::<u8>(rcx + 0x8) as usize;
    let base = rd::<usize>(rcx);
    let state_addr = base.wrapping_add(idx).wrapping_add(0x50);
    AtomicU8::from_ptr(state_addr as *mut u8).swap(2, Ordering::SeqCst);
    let edx = rd::<u8>(rcx + 0x8) as u32;
    let cb_ctx = rd::<usize>(rcx + 0x10);
    let cb_fn = rd::<usize>(rcx + 0x18);
    exe::call_va!(cb_fn, (cb_ctx, edx, 1u32) -> usize)
}

/// Original: `0x14207e3c0` (216 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_calc_mip_range_bytes(
    rcx: usize,
    rdx: usize,
    r8: u8,
    r9: usize,
    mip_count: u8,
) -> usize {
    let shift = (r8 as u32) & 31;
    let mut r10d = ((rd::<u32>(rcx + 0x70) >> shift) as i32).max(1) as u32;
    let mut r11d = ((rd::<u32>(rcx + 0x74) >> shift) as i32).max(1) as u32;
    let mut ebx = ((rd::<u32>(rcx + 0x78) >> shift) as i32).max(1) as u32;
    let mut rsi = 0u64;
    if mip_count != 0 {
        let fmt = rd::<u8>(rcx + 0x81) as u32;
        let mut rem = mip_count as u32;
        while rem != 0 {
            let slice_bytes = exe::call_va!(0x1_4204_e7f0, (r10d, r11d, fmt) -> u32);
            let vol = (slice_bytes as i32).wrapping_mul(ebx as i32) as i64 as u64;
            rsi = rsi.wrapping_add(vol);
            r10d = (r10d >> 1).max(1);
            r11d = (r11d >> 1).max(1);
            ebx = (ebx >> 1).max(1);
            rem -= 1;
        }
    }
    let base = rd::<u64>(r9);
    wr::<u64>(rdx, base);
    wr::<u64>(rdx + 0x8, base.wrapping_add(rsi));
    rdx
}

pub unsafe fn diff_d3d_streamed_tex_calc_mip_range_bytes(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut tex = [0u8; 0x90];
    let tex_ptr = tex.as_mut_ptr().add(4) as usize;
    wr::<u32>(tex_ptr + 0x70, 64);
    wr::<u32>(tex_ptr + 0x74, 64);
    wr::<u32>(tex_ptr + 0x78, 1);
    wr::<u8>(tex_ptr + 0x81, 1);

    let base_offset: u64 = 0x1000;
    let mut out_a = [0u8; 0x18];
    let mut out_b = [0u8; 0x18];
    let a = out_a.as_mut_ptr().add(4) as usize;
    let b = out_b.as_mut_ptr().add(4) as usize;

    exe::call_va!(
        orig,
        (tex_ptr, a, 1u8, &base_offset as *const u64 as usize, 3u8) -> usize
    );
    exe::call_va!(
        new,
        (tex_ptr, b, 1u8, &base_offset as *const u64 as usize, 3u8) -> usize
    );
    crate::diff_eq!(out_a, out_b, "mip_range_bytes");
    Ok(1)
}

/// Original: `0x14207e4a0` (85 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_remaining_mips(rcx: usize, rdx: u8) -> u8 {
    let count = rd::<u8>(rcx + 0x6c);
    if rdx < count && count != 0 {
        let mut al = 0u8;
        loop {
            let st = rd::<u8>(rcx + 0x50 + al as usize);
            if st == 2 {
                let sub = if al < rdx { al } else { rdx };
                return al.wrapping_sub(sub);
            }
            al = al.wrapping_add(1);
            if al >= rd::<u8>(rcx + 0x6c) {
                break;
            }
        }
    }
    rd::<u8>(rcx + 0x82).wrapping_sub(rdx)
}

pub unsafe fn diff_d3d_streamed_tex_remaining_mips(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf = [0u8; 0x90];
    let ptr = buf.as_mut_ptr().add(4) as usize;
    wr::<u8>(ptr + 0x6c, 4);
    wr::<u8>(ptr + 0x82, 10);
    wr::<u8>(ptr + 0x50, 1);
    wr::<u8>(ptr + 0x51, 0);
    wr::<u8>(ptr + 0x52, 2);
    wr::<u8>(ptr + 0x53, 2);

    for q in [0u8, 1, 2, 3, 4, 5] {
        let ra = exe::call_va!(orig, (ptr, q) -> u8);
        let rb = exe::call_va!(new, (ptr, q) -> u8);
        crate::diff_eq!(ra, rb, "remaining_mips");
    }
    Ok(6)
}

/// Original: `0x14207f2a0` (94 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_set_desc(rcx: usize, rdx: usize) {
    AtomicU8::from_ptr((rcx + 0x6d) as *mut u8).swap(1, Ordering::SeqCst);
    wr::<u32>(rcx + 0x70, rd::<u32>(rdx));
    wr::<u32>(rcx + 0x74, rd::<u32>(rdx + 0x4));
    wr::<u32>(rcx + 0x78, rd::<u32>(rdx + 0x8));
    wr::<u32>(rcx + 0x7c, rd::<u32>(rdx + 0xc));
    wr::<u8>(rcx + 0x80, rd::<u8>(rdx + 0x10));
    wr::<u8>(rcx + 0x81, rd::<u8>(rdx + 0x11));
    wr::<u8>(rcx + 0x82, rd::<u8>(rdx + 0x12));
    wr::<u8>(rcx + 0x83, rd::<u8>(rdx + 0x13));
    wr::<u64>(rcx + 0x90, rd::<u64>(rdx + 0x20));
    wr::<u64>(rcx + 0x88, rd::<u64>(rdx + 0x18));
}

pub unsafe fn diff_d3d_streamed_tex_set_desc(orig: usize, new: usize) -> Result<u32, String> {
    let mut src = [0u8; 0x30];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7).wrapping_add(3);
    }
    let src_ptr = src.as_ptr().add(4) as usize;
    let mut dst_a = [0u8; 0xa0];
    let mut dst_b = [0u8; 0xa0];
    let a = dst_a.as_mut_ptr().add(4) as usize;
    let b = dst_b.as_mut_ptr().add(4) as usize;

    exe::call_va!(orig, (a, src_ptr) -> ());
    exe::call_va!(new, (b, src_ptr) -> ());
    crate::diff_eq!(dst_a, dst_b, "set_desc");
    Ok(1)
}

/// Original: `0x14207f310` (128 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_impl_dtor(rcx: usize) {
    let rdi = rcx;
    let mut bl = 0u8;
    if rd::<u8>(rdi + 0x83) > 0 {
        loop {
            exe::call_va!(0x1_4207_e500, (rdi, bl as u32) -> ());
            bl = bl.wrapping_add(1);
            if bl >= rd::<u8>(rdi + 0x83) {
                break;
            }
        }
    }
    let dtor_fn = exe::ptr::<u8>(0x1_411d_2db0) as usize;
    exe::call_va!(0x1_4183_4ee8, (rdi + 0x10, 0x8usize, 0x8usize, dtor_fn) -> ());
    let ptr8 = rd::<usize>(rdi + 0x8);
    if ptr8 != 0 {
        exe::call_va!(0x1_4244_2de0, (ptr8) -> ());
    }
    let ptr0 = rd::<usize>(rdi);
    if ptr0 != 0 {
        exe::call_va!(0x1_4206_b020, (ptr0) -> ());
        exe::call_va!(0x1_4244_2de0, (ptr0) -> ());
    }
}

/// Original: `0x14207f660` (154 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_impl_ctor(rcx: usize) -> usize {
    let rbx = rcx;
    wr::<u64>(rbx, 0);
    wr::<u64>(rbx + 0x8, 0);
    let ctor_fn = exe::ptr::<u8>(0x1_4020_0580) as usize;
    let dtor_fn = exe::ptr::<u8>(0x1_411d_2db0) as usize;
    exe::call_va!(
        0x1_4183_4fc0,
        (rbx + 0x10, 0x8usize, 0x8usize, ctor_fn, dtor_fn) -> ()
    );
    wr::<u64>(rbx + 0x50, 0);
    wr::<u64>(rbx + 0x58, 0);
    wr::<u64>(rbx + 0x60, 0);
    wr::<u32>(rbx + 0x68, 0);
    wr::<u16>(rbx + 0x6c, 0);
    wr::<u64>(rbx + 0x70, 0);
    wr::<u32>(rbx + 0x78, 0);
    wr::<u32>(rbx + 0x7c, 0xffff_ffff);
    wr::<u32>(rbx + 0x80, 0xff);
    wr::<u64>(rbx + 0x88, 0);
    wr::<u64>(rbx + 0x90, 0);
    for off in 0x50..0x58 {
        AtomicU8::from_ptr((rbx + off) as *mut u8).swap(0, Ordering::SeqCst);
    }
    rbx
}

pub unsafe fn diff_d3d_streamed_tex_impl_ctor(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_a = [0xaa_u8; 0xa0];
    let mut buf_b = [0xaa_u8; 0xa0];
    let a = buf_a.as_mut_ptr().add(4) as usize;
    let b = buf_b.as_mut_ptr().add(4) as usize;
    let ra = exe::call_va!(orig, (a) -> usize);
    let rb = exe::call_va!(new, (b) -> usize);
    crate::diff_eq!(ra, a, "ctor_ret_a");
    crate::diff_eq!(rb, b, "ctor_ret_b");
    crate::diff_eq!(buf_a, buf_b, "ctor_state");
    Ok(1)
}

/// Original: `0x14207f710` (51 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_try_commit_cb_init(
    rcx: usize,
    rdx: u64,
    r8: usize,
    r9: usize,
    cb_pair: usize,
) -> usize {
    wr::<u64>(rcx, rdx);
    let b8 = rd::<u8>(r8);
    wr::<u8>(rcx + 0x8, b8);
    let b9 = rd::<u8>(r9);
    wr::<u8>(rcx + 0x9, b9);
    wr::<u64>(rcx + 0x10, 0);
    wr::<u64>(rcx + 0x18, 0);
    let f8 = rd::<u64>(cb_pair + 0x8);
    wr::<u64>(rcx + 0x18, f8);
    let f0 = rd::<u64>(cb_pair);
    wr::<u64>(rcx + 0x10, f0);
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_try_commit_cb_init(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut out_a = [0u8; 0x28];
    let mut out_b = [0u8; 0x28];
    let a = out_a.as_mut_ptr().add(4) as usize;
    let b = out_b.as_mut_ptr().add(4) as usize;
    let v8: u8 = 3;
    let v9: u8 = 7;
    let pair: [u64; 2] = [0x1234_5678_9abc_def0, 0x0fed_cba9_8765_4321];
    exe::call_va!(
        orig,
        (a, 0x5555_6666_7777_8888u64, &v8 as *const u8 as usize, &v9 as *const u8 as usize, pair.as_ptr() as usize) -> usize
    );
    exe::call_va!(
        new,
        (b, 0x5555_6666_7777_8888u64, &v8 as *const u8 as usize, &v9 as *const u8 as usize, pair.as_ptr() as usize) -> usize
    );
    crate::diff_eq!(out_a, out_b, "try_commit_cb_init");
    Ok(1)
}

/// Original: `0x14207f750` (58 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_try_commit_cb_invoke(rcx: usize, rdx: usize) {
    let mut cb = [0u64; 2];
    cb[1] = rd::<u64>(rcx + 0x18);
    let r9 = rdx.wrapping_add(0x20);
    let r8d = rd::<u8>(rcx + 0x8) as u32;
    let edx = rd::<u8>(rcx + 0x9) as u32;
    cb[0] = rd::<u64>(rcx + 0x10);
    let target_rcx = rd::<usize>(rcx);
    exe::call_va!(
        0x1_4207_da50,
        (target_rcx, edx, r8d, r9, cb.as_mut_ptr() as usize) -> ()
    );
}

/// Original: `0x14207fc10` (55 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_reserve_cb_init(
    rcx: usize,
    rdx: u64,
    r8: usize,
    r9: usize,
) -> usize {
    wr::<u64>(rcx, rdx);
    wr::<u64>(rcx + 0x8, 0);
    wr::<u64>(rcx + 0x10, 0);
    wr::<u64>(rcx + 0x10, rd::<u64>(r8 + 0x8));
    wr::<u64>(rcx + 0x8, rd::<u64>(r8));
    wr::<u64>(rcx + 0x18, 0);
    wr::<u64>(rcx + 0x20, 0);
    wr::<u64>(rcx + 0x20, rd::<u64>(r9 + 0x8));
    wr::<u64>(rcx + 0x18, rd::<u64>(r9));
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_reserve_cb_init(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut out_a = [0u8; 0x30];
    let mut out_b = [0u8; 0x30];
    let a = out_a.as_mut_ptr().add(4) as usize;
    let b = out_b.as_mut_ptr().add(4) as usize;
    let p1: [u64; 2] = [0x1111, 0x2222];
    let p2: [u64; 2] = [0x3333, 0x4444];
    exe::call_va!(
        orig,
        (a, 0x9999u64, p1.as_ptr() as usize, p2.as_ptr() as usize) -> usize
    );
    exe::call_va!(
        new,
        (b, 0x9999u64, p1.as_ptr() as usize, p2.as_ptr() as usize) -> usize
    );
    crate::diff_eq!(out_a, out_b, "reserve_cb_init");
    Ok(1)
}

/// Original: `0x14207fc50` (52 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_reserve_cb_invoke(rcx: usize) -> usize {
    let rbx = rd::<usize>(rcx);
    let rdi = rcx;
    let arg0 = rd::<usize>(rdi + 0x8);
    let fn0 = rd::<usize>(rdi + 0x10);
    exe::call_va!(fn0, (arg0) -> ());
    exe::call_va!(0x1_4207_ec20, (rbx) -> ());
    let arg1 = rd::<usize>(rdi + 0x18);
    let fn1 = rd::<usize>(rdi + 0x20);
    exe::call_va!(fn1, (arg1) -> usize)
}

/// Original: `0x142080030` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_span_prefix_below(
    rcx: usize,
    rdx: usize,
    r8: u8,
) -> usize {
    let mut r10 = rd::<u64>(rdx);
    let mut rax = 0u64;
    if r10 != 0 {
        let r11 = rd::<usize>(rdx + 0x8);
        let mut r9 = r11.wrapping_add(0x1c);
        loop {
            if rd::<u8>(r9) >= r8 {
                wr::<usize>(rcx + 0x8, r11);
                if rax != u64::MAX {
                    r10 = rax;
                }
                wr::<u64>(rcx, r10);
                return rcx;
            }
            rax = rax.wrapping_add(1);
            r9 = r9.wrapping_add(0x20);
            if rax >= r10 {
                break;
            }
        }
    }
    wr::<u128>(rcx, rd::<u128>(rdx));
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_span_prefix_below(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut items = [0u8; 0x64];
    let items_ptr = items.as_mut_ptr().add(4) as usize;
    wr::<u8>(items_ptr + 0x1c, 1);
    wr::<u8>(items_ptr + 0x3c, 3);
    wr::<u8>(items_ptr + 0x5c, 5);

    let mut span = [0u8; 0x18];
    let span_ptr = span.as_mut_ptr().add(4) as usize;
    wr::<u64>(span_ptr, 3);
    wr::<usize>(span_ptr + 0x8, items_ptr);

    for thresh in [0u8, 2, 3, 6] {
        let mut out_a = [0u8; 0x18];
        let mut out_b = [0u8; 0x18];
        let a = out_a.as_mut_ptr().add(4) as usize;
        let b = out_b.as_mut_ptr().add(4) as usize;
        exe::call_va!(orig, (a, span_ptr, thresh) -> usize);
        exe::call_va!(new, (b, span_ptr, thresh) -> usize);
        crate::diff_eq!(out_a, out_b, "span_prefix_below");
    }
    Ok(4)
}

/// Original: `0x142081110` (64 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_map_grow_or_rehash(rcx: usize) -> usize {
    let rdx = rd::<u64>(rcx + 0x18);
    if rdx == 0 {
        return exe::call_va!(0x1_4208_0b50, (rcx, 1u64) -> usize);
    }
    let threshold = (rdx - (rdx >> 3)) >> 1;
    if rd::<u64>(rcx + 0x10) <= threshold {
        exe::call_va!(0x1_4208_0e90, (rcx, rdx, rcx) -> usize)
    } else {
        exe::call_va!(0x1_4208_0b50, (rcx, rdx.wrapping_mul(2).wrapping_add(1)) -> usize)
    }
}

/// Original: `0x1420811e0` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_key_eq(_rcx: usize, rdx: usize, r8: usize) -> u8 {
    if rd::<u64>(rdx) == rd::<u64>(r8)
        && rd::<u32>(rdx + 0x8) == rd::<u32>(r8 + 0x8)
        && rd::<u32>(rdx + 0xc) == rd::<u32>(r8 + 0xc)
        && rd::<u32>(rdx + 0x10) == rd::<u32>(r8 + 0x10)
    {
        1
    } else {
        0
    }
}

pub unsafe fn diff_d3d_tex_key_eq(orig: usize, new: usize) -> Result<u32, String> {
    let mut ka = [0u8; 0x18];
    let mut kb = [0u8; 0x18];
    let a = ka.as_mut_ptr().add(4) as usize;
    let b = kb.as_mut_ptr().add(4) as usize;
    wr::<u64>(a, 0x1122_3344_5566_7788);
    wr::<u32>(a + 0x8, 10);
    wr::<u32>(a + 0xc, 20);
    wr::<u32>(a + 0x10, 30);
    kb.copy_from_slice(&ka);

    let r1 = exe::call_va!(orig, (0usize, a, b) -> u8);
    let r2 = exe::call_va!(new, (0usize, a, b) -> u8);
    crate::diff_eq!(r1, r2, "key_eq_same");

    wr::<u32>(b + 0x10, 31);
    let r3 = exe::call_va!(orig, (0usize, a, b) -> u8);
    let r4 = exe::call_va!(new, (0usize, a, b) -> u8);
    crate::diff_eq!(r3, r4, "key_eq_diff");
    Ok(2)
}

/// Original: `0x142081370` (123 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_entry_move_ctor(rcx: usize, rdx: usize) -> usize {
    let f0 = rd::<u64>(rdx);
    wr::<u64>(rdx, 0);
    wr::<u64>(rcx, f0);
    let f8 = rd::<u64>(rdx + 0x8);
    wr::<u64>(rdx + 0x8, 0);
    wr::<u64>(rcx + 0x8, f8);
    let f10 = rd::<u64>(rdx + 0x10);
    wr::<u64>(rdx + 0x10, 0);
    wr::<u64>(rcx + 0x10, f10);
    let f18 = rd::<u64>(rdx + 0x18);
    wr::<u64>(rdx + 0x18, 0);
    wr::<u64>(rcx + 0x18, f18);
    let f20 = rd::<u64>(rdx + 0x20);
    wr::<u64>(rdx + 0x20, 0);
    wr::<u64>(rcx + 0x20, f20);
    let f28 = rd::<u16>(rdx + 0x28);
    wr::<u16>(rdx + 0x28, 0x10);
    wr::<u16>(rcx + 0x28, f28);
    let f30 = rd::<u64>(rdx + 0x30);
    wr::<u64>(rdx + 0x30, 0);
    wr::<u64>(rcx + 0x30, f30);
    wr::<u8>(rcx + 0x38, rd::<u8>(rdx + 0x38));
    wr::<u32>(rcx + 0x3c, rd::<u32>(rdx + 0x3c));
    wr::<u32>(rcx + 0x40, rd::<u32>(rdx + 0x40));
    wr::<u8>(rcx + 0x44, rd::<u8>(rdx + 0x44));
    wr::<u8>(rcx + 0x48, rd::<u8>(rdx + 0x48));
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_entry_move_ctor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut src_a = [0u8; 0x58];
    for (i, v) in src_a.iter_mut().enumerate() {
        *v = (i as u8).wrapping_mul(3).wrapping_add(1);
    }
    let mut src_b = src_a;
    let mut dst_a = [0u8; 0x58];
    let mut dst_b = [0u8; 0x58];
    let sa = src_a.as_mut_ptr().add(4) as usize;
    let sb = src_b.as_mut_ptr().add(4) as usize;
    let da = dst_a.as_mut_ptr().add(4) as usize;
    let db = dst_b.as_mut_ptr().add(4) as usize;

    exe::call_va!(orig, (da, sa) -> usize);
    exe::call_va!(new, (db, sb) -> usize);
    crate::diff_eq!(src_a, src_b, "entry_move_src");
    crate::diff_eq!(dst_a, dst_b, "entry_move_dst");
    Ok(1)
}

/// Original: `0x142081470` (49 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_handle_move_ctor(rcx: usize, rdx: usize) -> usize {
    let f0 = rd::<u64>(rdx);
    wr::<u64>(rdx, 0);
    wr::<u64>(rcx, f0);
    wr::<u64>(rcx + 0x8, rd::<u64>(rdx + 0x8));
    wr::<u64>(rcx + 0x10, rd::<u64>(rdx + 0x10));
    wr::<u32>(rcx + 0x18, rd::<u32>(rdx + 0x18));
    wr::<u16>(rcx + 0x1c, rd::<u16>(rdx + 0x1c));
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_handle_move_ctor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut src_a = [0u8; 0x28];
    for (i, v) in src_a.iter_mut().enumerate() {
        *v = (i as u8).wrapping_mul(5).wrapping_add(9);
    }
    let mut src_b = src_a;
    let mut dst_a = [0u8; 0x28];
    let mut dst_b = [0u8; 0x28];
    let sa = src_a.as_mut_ptr().add(4) as usize;
    let sb = src_b.as_mut_ptr().add(4) as usize;
    let da = dst_a.as_mut_ptr().add(4) as usize;
    let db = dst_b.as_mut_ptr().add(4) as usize;

    exe::call_va!(orig, (da, sa) -> usize);
    exe::call_va!(new, (db, sb) -> usize);
    crate::diff_eq!(src_a, src_b, "handle_move_src");
    crate::diff_eq!(dst_a, dst_b, "handle_move_dst");
    Ok(1)
}

/// Original: `0x1420814b0` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_ref_move_ctor(rcx: usize, rdx: usize) -> usize {
    wr::<u32>(rcx, rd::<u32>(rdx));
    wr::<u64>(rcx + 0x8, rd::<u64>(rdx + 0x8));
    let f10 = rd::<u64>(rdx + 0x10);
    wr::<u64>(rdx + 0x10, 0);
    wr::<u64>(rcx + 0x10, f10);
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_ref_move_ctor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut src_a = [0u8; 0x20];
    for (i, v) in src_a.iter_mut().enumerate() {
        *v = (i as u8).wrapping_mul(11).wrapping_add(2);
    }
    let mut src_b = src_a;
    let mut dst_a = [0u8; 0x20];
    let mut dst_b = [0u8; 0x20];
    let sa = src_a.as_mut_ptr().add(4) as usize;
    let sb = src_b.as_mut_ptr().add(4) as usize;
    let da = dst_a.as_mut_ptr().add(4) as usize;
    let db = dst_b.as_mut_ptr().add(4) as usize;

    exe::call_va!(orig, (da, sa) -> usize);
    exe::call_va!(new, (db, sb) -> usize);
    crate::diff_eq!(src_a, src_b, "ref_move_src");
    crate::diff_eq!(dst_a, dst_b, "ref_move_dst");
    Ok(1)
}

/// Original: `0x1420815a0` (74 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_release_range(rcx: usize) {
    let rdi = rd::<usize>(rcx + 0x8);
    let mut rbx = rd::<usize>(rcx);
    while rbx != rdi {
        let obj = rd::<usize>(rbx);
        if obj != 0 {
            let vtbl = rd::<usize>(obj);
            let release_fn = rd::<usize>(vtbl + 0x10);
            exe::call_va!(release_fn, (obj) -> ());
        }
        wr::<usize>(rbx, 0);
        rbx = rbx.wrapping_add(0x10);
    }
}

/// Original: `0x142081600` (57 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_req_move_ctor(rcx: usize, rdx: usize) -> usize {
    let f0 = rd::<u64>(rdx);
    wr::<u64>(rdx, 0);
    wr::<u64>(rcx, f0);
    wr::<u16>(rcx + 0x8, rd::<u16>(rdx + 0x8));
    wr::<u128>(rcx + 0xc, rd::<u128>(rdx + 0xc));
    wr::<u128>(rcx + 0x1c, rd::<u128>(rdx + 0x1c));
    wr::<u32>(rcx + 0x2c, rd::<u32>(rdx + 0x2c));
    wr::<u32>(rcx + 0x30, rd::<u32>(rdx + 0x30));
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_req_move_ctor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut src_a = [0u8; 0x40];
    for (i, v) in src_a.iter_mut().enumerate() {
        *v = (i as u8).wrapping_mul(13).wrapping_add(7);
    }
    let mut src_b = src_a;
    let mut dst_a = [0u8; 0x40];
    let mut dst_b = [0u8; 0x40];
    let sa = src_a.as_mut_ptr().add(4) as usize;
    let sb = src_b.as_mut_ptr().add(4) as usize;
    let da = dst_a.as_mut_ptr().add(4) as usize;
    let db = dst_b.as_mut_ptr().add(4) as usize;

    exe::call_va!(orig, (da, sa) -> usize);
    exe::call_va!(new, (db, sb) -> usize);
    crate::diff_eq!(src_a, src_b, "req_move_src");
    crate::diff_eq!(dst_a, dst_b, "req_move_dst");
    Ok(1)
}

/// Original: `0x142081740` (38 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_ref_dtor(rcx: usize) {
    let obj = rd::<usize>(rcx + 0x10);
    if obj != 0 {
        let vtbl = rd::<usize>(obj);
        let release_fn = rd::<usize>(vtbl + 0x10);
        exe::call_va!(release_fn, (obj) -> ());
    }
    wr::<usize>(rcx + 0x10, 0);
}

/// Original: `0x1420817a0` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_ref_release_ret(rcx: usize) -> usize {
    let obj = rd::<usize>(rcx + 0x10);
    if obj != 0 {
        let vtbl = rd::<usize>(obj);
        let release_fn = rd::<usize>(vtbl + 0x10);
        exe::call_va!(release_fn, (obj) -> ());
    }
    wr::<usize>(rcx + 0x10, 0);
    rcx
}

/// Original: `0x1420817d0` (92 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_handle_copy_ctor(rcx: usize, rdx: usize) -> usize {
    let rdi = rd::<usize>(rdx);
    if rdi != 0 {
        let vtbl = rd::<usize>(rdi);
        let addref_fn = rd::<usize>(vtbl + 0x8);
        exe::call_va!(addref_fn, (rdi) -> ());
    }
    wr::<usize>(rcx, rdi);
    wr::<u64>(rcx + 0x8, rd::<u64>(rdx + 0x8));
    wr::<u64>(rcx + 0x10, rd::<u64>(rdx + 0x10));
    wr::<u32>(rcx + 0x18, rd::<u32>(rdx + 0x18));
    wr::<u16>(rcx + 0x1c, rd::<u16>(rdx + 0x1c));
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_handle_copy_ctor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut src = [0u8; 0x28];
    for (i, v) in src.iter_mut().enumerate() {
        *v = (i as u8).wrapping_mul(17).wrapping_add(5);
    }
    let s = src.as_mut_ptr().add(4) as usize;
    wr::<usize>(s, 0); // null COM pointer so copy ctor skips AddRef
    let mut dst_a = [0u8; 0x28];
    let mut dst_b = [0u8; 0x28];
    let da = dst_a.as_mut_ptr().add(4) as usize;
    let db = dst_b.as_mut_ptr().add(4) as usize;

    exe::call_va!(orig, (da, s) -> usize);
    exe::call_va!(new, (db, s) -> usize);
    crate::diff_eq!(dst_a, dst_b, "handle_copy_dst");
    Ok(1)
}

/// Original: `0x1420819e0` (17 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_slot_init(rcx: usize) -> usize {
    wr::<u64>(rcx, 0);
    wr::<u16>(rcx + 0x8, 0xffff);
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_slot_init(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_a = [0x55_u8; 0x18];
    let mut buf_b = [0x55_u8; 0x18];
    let a = buf_a.as_mut_ptr().add(4) as usize;
    let b = buf_b.as_mut_ptr().add(4) as usize;
    exe::call_va!(orig, (a) -> usize);
    exe::call_va!(new, (b) -> usize);
    crate::diff_eq!(buf_a, buf_b, "slot_init");
    Ok(1)
}

/// Original: `0x142081a00` (30 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_streamed_tex_state_zero_init(rcx: usize) -> usize {
    wr::<u64>(rcx, 0);
    wr::<u128>(rcx + 0x8, 0);
    wr::<u128>(rcx + 0x18, 0);
    wr::<u32>(rcx + 0x28, 0);
    rcx
}

pub unsafe fn diff_d3d_streamed_tex_state_zero_init(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut buf_a = [0x77_u8; 0x38];
    let mut buf_b = [0x77_u8; 0x38];
    let a = buf_a.as_mut_ptr().add(4) as usize;
    let b = buf_b.as_mut_ptr().add(4) as usize;
    exe::call_va!(orig, (a) -> usize);
    exe::call_va!(new, (b) -> usize);
    crate::diff_eq!(buf_a, buf_b, "state_zero_init");
    Ok(1)
}

static mut STREAM_B_ALLOC_BUF: [u8; 0x40] = [0; 0x40];

unsafe extern "win64" fn mock_operator_new_141834d64(_size: usize) -> usize {
    core::ptr::addr_of_mut!(STREAM_B_ALLOC_BUF) as *mut u8 as usize
}

pub unsafe fn diff_d3d_streamed_tex_reserve_async_lambda_clone(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4183_4d64, mock_operator_new_141834d64 as *const ())?;

    let mut src = [0u8; 0x30];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7).wrapping_add(1);
    }

    let base = core::ptr::addr_of_mut!(STREAM_B_ALLOC_BUF) as *mut u8;
    core::ptr::write_bytes(base, 0x5a, 0x40);
    let ro = exe::call_va!(orig, (src.as_ptr() as usize) -> usize);
    let snap_o = rd::<[u8; 0x30]>(base as usize);

    core::ptr::write_bytes(base, 0x5a, 0x40);
    let rn = exe::call_va!(new, (src.as_ptr() as usize) -> usize);
    let snap_n = rd::<[u8; 0x30]>(base as usize);

    crate::diff_eq!(ro, rn, "clone ret");
    crate::diff_eq!(snap_o, snap_n, "cloned lambda");
    Ok(1)
}

unsafe extern "win64" fn mock_reserve_fn0(arg0: usize) {
    wr::<u32>(arg0, 0x1111_2222);
}

unsafe extern "win64" fn mock_reserve_14207ec20(rbx: usize) {
    wr::<u32>(rbx, 0x3333_4444);
}

unsafe extern "win64" fn mock_reserve_fn1(arg1: usize) -> usize {
    wr::<u32>(arg1, 0x5555_6666);
    0x7777_8888
}

pub unsafe fn diff_d3d_streamed_tex_reserve_async_lambda_call(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4207_ec20, mock_reserve_14207ec20 as *const ())?;

    let mut s_o = [0u32; 3];
    let mut s_n = [0u32; 3];

    let rcx_o = [
        0usize,
        (&mut s_o[0] as *mut u32) as usize,
        (&mut s_o[1] as *mut u32) as usize,
        mock_reserve_fn0 as *const () as usize,
        (&mut s_o[2] as *mut u32) as usize,
        mock_reserve_fn1 as *const () as usize,
    ];
    let rcx_n = [
        0usize,
        (&mut s_n[0] as *mut u32) as usize,
        (&mut s_n[1] as *mut u32) as usize,
        mock_reserve_fn0 as *const () as usize,
        (&mut s_n[2] as *mut u32) as usize,
        mock_reserve_fn1 as *const () as usize,
    ];

    let ro = exe::call_va!(orig, (rcx_o.as_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (rcx_n.as_ptr() as usize) -> usize);

    crate::diff_eq!(ro, rn, "ret");
    crate::diff_eq!(s_o, s_n, "side effects");
    Ok(1)
}

pub unsafe fn diff_d3d_streamed_tex_try_commit_async_lambda_clone(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4183_4d64, mock_operator_new_141834d64 as *const ())?;

    let mut src = [0u8; 0x28];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(11).wrapping_add(3);
    }

    let base = core::ptr::addr_of_mut!(STREAM_B_ALLOC_BUF) as *mut u8;
    core::ptr::write_bytes(base, 0x5a, 0x40);
    let ro = exe::call_va!(orig, (src.as_ptr() as usize) -> usize);
    let snap_o = rd::<[u8; 0x28]>(base as usize);

    core::ptr::write_bytes(base, 0x5a, 0x40);
    let rn = exe::call_va!(new, (src.as_ptr() as usize) -> usize);
    let snap_n = rd::<[u8; 0x28]>(base as usize);

    crate::diff_eq!(ro, rn, "clone ret");
    crate::diff_eq!(snap_o, snap_n, "cloned try_commit lambda");
    Ok(1)
}

unsafe extern "win64" fn mock_try_commit_14207da50(
    target_rcx: usize,
    edx: u32,
    r8d: u32,
    r9: usize,
    cb_ptr: usize,
) {
    wr::<u32>(target_rcx, edx);
    wr::<u32>(target_rcx + 0x4, r8d);
    wr::<u64>(target_rcx + 0x8, rd::<u64>(r9));
    wr::<u64>(target_rcx + 0x10, rd::<u64>(cb_ptr));
    wr::<u64>(target_rcx + 0x18, rd::<u64>(cb_ptr + 0x8));
}

pub unsafe fn diff_d3d_streamed_tex_try_commit_async_lambda_call(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4207_da50, mock_try_commit_14207da50 as *const ())?;

    let mut target_o = [0u64; 4];
    let mut target_n = [0u64; 4];
    let mut rdx_buf = [0u8; 0x28];
    wr::<u64>((rdx_buf.as_mut_ptr() as usize) + 0x20, 0x9999_aaaa_bbbb_cccc);

    let mut rcx_o = [0u8; 0x28];
    let co = rcx_o.as_mut_ptr() as usize;
    wr::<usize>(co + 0x8, target_o.as_mut_ptr() as usize);
    wr::<u8>(co + 0x10, 4);
    wr::<u8>(co + 0x11, 7);
    wr::<u64>(co + 0x18, 0x1111_2222);
    wr::<u64>(co + 0x20, 0x3333_4444);

    let mut rcx_n = rcx_o;
    wr::<usize>((rcx_n.as_mut_ptr() as usize) + 0x8, target_n.as_mut_ptr() as usize);

    exe::call_va!(orig, (co, rdx_buf.as_ptr() as usize) -> ());
    exe::call_va!(new, (rcx_n.as_ptr() as usize, rdx_buf.as_ptr() as usize) -> ());

    crate::diff_eq!(target_o, target_n, "try_commit forwarded args");
    Ok(1)
}

pub unsafe fn diff_d3d_streamed_tex_commit_async_lambda_clone(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4183_4d64, mock_operator_new_141834d64 as *const ())?;

    let mut src = [0u8; 0x28];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(13).wrapping_add(5);
    }

    let base = core::ptr::addr_of_mut!(STREAM_B_ALLOC_BUF) as *mut u8;
    core::ptr::write_bytes(base, 0x5a, 0x40);
    let ro = exe::call_va!(orig, (src.as_ptr() as usize) -> usize);
    let snap_o = rd::<[u8; 0x28]>(base as usize);

    core::ptr::write_bytes(base, 0x5a, 0x40);
    let rn = exe::call_va!(new, (src.as_ptr() as usize) -> usize);
    let snap_n = rd::<[u8; 0x28]>(base as usize);

    crate::diff_eq!(ro, rn, "clone ret");
    crate::diff_eq!(snap_o, snap_n, "cloned commit lambda");
    Ok(1)
}

unsafe extern "win64" fn mock_commit_cb_fn(cb_ctx: usize, edx: u32, r8b: u8) -> usize {
    wr::<u32>(cb_ctx, (edx << 8) | (r8b as u32));
    0x4242
}

pub unsafe fn diff_d3d_streamed_tex_commit_async_lambda_call(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut base_o = [0u8; 0x60];
    let mut base_n = [0u8; 0x60];
    let mut ctx_o = 0u32;
    let mut ctx_n = 0u32;

    let mut rcx_o = [0u8; 0x28];
    let co = rcx_o.as_mut_ptr() as usize;
    wr::<usize>(co + 0x8, base_o.as_mut_ptr() as usize);
    wr::<u8>(co + 0x10, 3);
    wr::<usize>(co + 0x18, (&mut ctx_o as *mut u32) as usize);
    wr::<usize>(co + 0x20, mock_commit_cb_fn as *const () as usize);

    let mut rcx_n = [0u8; 0x28];
    let cn = rcx_n.as_mut_ptr() as usize;
    wr::<usize>(cn + 0x8, base_n.as_mut_ptr() as usize);
    wr::<u8>(cn + 0x10, 3);
    wr::<usize>(cn + 0x18, (&mut ctx_n as *mut u32) as usize);
    wr::<usize>(cn + 0x20, mock_commit_cb_fn as *const () as usize);

    let ro = exe::call_va!(orig, (co) -> usize);
    let rn = exe::call_va!(new, (cn) -> usize);

    crate::diff_eq!(ro, rn, "commit lambda ret");
    crate::diff_eq!(base_o, base_n, "state byte updated");
    crate::diff_eq!(ctx_o, ctx_n, "cb_ctx updated");
    Ok(1)
}

pub unsafe fn diff_d3d_streamed_tex_commit_cb_invoke(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut base_o = [0u8; 0x60];
    let mut base_n = [0u8; 0x60];
    let mut ctx_o = 0u32;
    let mut ctx_n = 0u32;

    let mut rcx_o = [0u8; 0x20];
    let co = rcx_o.as_mut_ptr() as usize;
    wr::<usize>(co, base_o.as_mut_ptr() as usize);
    wr::<u8>(co + 0x8, 5);
    wr::<usize>(co + 0x10, (&mut ctx_o as *mut u32) as usize);
    wr::<usize>(co + 0x18, mock_commit_cb_fn as *const () as usize);

    let mut rcx_n = [0u8; 0x20];
    let cn = rcx_n.as_mut_ptr() as usize;
    wr::<usize>(cn, base_n.as_mut_ptr() as usize);
    wr::<u8>(cn + 0x8, 5);
    wr::<usize>(cn + 0x10, (&mut ctx_n as *mut u32) as usize);
    wr::<usize>(cn + 0x18, mock_commit_cb_fn as *const () as usize);

    let ro = exe::call_va!(orig, (co) -> usize);
    let rn = exe::call_va!(new, (cn) -> usize);

    crate::diff_eq!(ro, rn, "commit cb ret");
    crate::diff_eq!(base_o, base_n, "state byte updated");
    crate::diff_eq!(ctx_o, ctx_n, "cb_ctx updated");
    Ok(1)
}

unsafe extern "win64" fn mock_evict_mip_14207e500(rdi: usize, mip: u32) {
    wr::<u32>(rdi + 0x50 + (mip as usize) * 4, mip.wrapping_add(0x10));
}

unsafe extern "win64" fn mock_destroy_res_14206b020(p: usize) {
    wr::<u32>(p, 0xaaaa_1111);
}

unsafe extern "win64" fn mock_free_mem_142442de0(p: usize) {
    wr::<u32>(p + 0x4, 0xbbbb_2222);
}

pub unsafe fn diff_d3d_streamed_tex_impl_dtor(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4207_e500, mock_evict_mip_14207e500 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_b020, mock_destroy_res_14206b020 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4244_2de0, mock_free_mem_142442de0 as *const ())?;

    let mut p0_o = [0u32; 2];
    let mut p0_n = [0u32; 2];
    let mut p8_o = [0u32; 2];
    let mut p8_n = [0u32; 2];

    let mut obj_o = [0u8; 0x90];
    let oo = obj_o.as_mut_ptr() as usize;
    wr::<usize>(oo, p0_o.as_mut_ptr() as usize);
    wr::<usize>(oo + 0x8, p8_o.as_mut_ptr() as usize);
    wr::<u8>(oo + 0x83, 2);

    let mut obj_n = [0u8; 0x90];
    let on = obj_n.as_mut_ptr() as usize;
    wr::<usize>(on, p0_n.as_mut_ptr() as usize);
    wr::<usize>(on + 0x8, p8_n.as_mut_ptr() as usize);
    wr::<u8>(on + 0x83, 2);

    exe::call_va!(orig, (oo) -> ());
    exe::call_va!(new, (on) -> ());

    crate::diff_eq!(p0_o, p0_n, "ptr0 destroyed + freed");
    crate::diff_eq!(p8_o, p8_n, "ptr8 freed");
    crate::diff_eq!(&obj_o[0x50..0x60], &obj_n[0x50..0x60], "mips evicted");
    Ok(1)
}

pub unsafe fn diff_d3d_streamed_tex_try_commit_cb_invoke(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4207_da50, mock_try_commit_14207da50 as *const ())?;

    let mut target_o = [0u64; 4];
    let mut target_n = [0u64; 4];
    let mut rdx_buf = [0u8; 0x28];
    wr::<u64>((rdx_buf.as_mut_ptr() as usize) + 0x20, 0xfeed_face_cafe_babe);

    let mut rcx_o = [0u8; 0x20];
    let co = rcx_o.as_mut_ptr() as usize;
    wr::<usize>(co, target_o.as_mut_ptr() as usize);
    wr::<u8>(co + 0x8, 2);
    wr::<u8>(co + 0x9, 6);
    wr::<u64>(co + 0x10, 0xaaaa_1111);
    wr::<u64>(co + 0x18, 0xbbbb_2222);

    let mut rcx_n = rcx_o;
    wr::<usize>(rcx_n.as_mut_ptr() as usize, target_n.as_mut_ptr() as usize);

    exe::call_va!(orig, (co, rdx_buf.as_ptr() as usize) -> ());
    exe::call_va!(new, (rcx_n.as_ptr() as usize, rdx_buf.as_ptr() as usize) -> ());

    crate::diff_eq!(target_o, target_n, "try_commit_cb forwarded args");
    Ok(1)
}

pub unsafe fn diff_d3d_streamed_tex_reserve_cb_invoke(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4207_ec20, mock_reserve_14207ec20 as *const ())?;

    let mut s_o = [0u32; 3];
    let mut s_n = [0u32; 3];

    let rcx_o = [
        (&mut s_o[0] as *mut u32) as usize,
        (&mut s_o[1] as *mut u32) as usize,
        mock_reserve_fn0 as *const () as usize,
        (&mut s_o[2] as *mut u32) as usize,
        mock_reserve_fn1 as *const () as usize,
    ];
    let rcx_n = [
        (&mut s_n[0] as *mut u32) as usize,
        (&mut s_n[1] as *mut u32) as usize,
        mock_reserve_fn0 as *const () as usize,
        (&mut s_n[2] as *mut u32) as usize,
        mock_reserve_fn1 as *const () as usize,
    ];

    let ro = exe::call_va!(orig, (rcx_o.as_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (rcx_n.as_ptr() as usize) -> usize);

    crate::diff_eq!(ro, rn, "reserve_cb ret");
    crate::diff_eq!(s_o, s_n, "reserve_cb side effects");
    Ok(1)
}

unsafe extern "win64" fn mock_tex_map_grow_142080b50(rcx: usize, new_cap: u64) -> usize {
    wr::<u64>(rcx, new_cap.wrapping_add(0x100));
    new_cap as usize
}

unsafe extern "win64" fn mock_tex_map_rehash_142080e90(rcx: usize, cap: u64, _self2: usize) -> usize {
    wr::<u64>(rcx, cap.wrapping_add(0x200));
    (cap as usize).wrapping_add(0x50)
}

pub unsafe fn diff_d3d_tex_map_grow_or_rehash(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4208_0b50, mock_tex_map_grow_142080b50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4208_0e90, mock_tex_map_rehash_142080e90 as *const ())?;

    for (len, cap) in [(0u64, 0u64), (2u64, 8u64), (6u64, 8u64)] {
        let mut map_o = [0u64, 0, len, cap];
        let mut map_n = [0u64, 0, len, cap];
        let ro = exe::call_va!(orig, (map_o.as_mut_ptr() as usize) -> usize);
        let rn = exe::call_va!(new, (map_n.as_mut_ptr() as usize) -> usize);
        crate::diff_eq!(ro, rn, "grow_or_rehash ret");
        crate::diff_eq!(map_o, map_n, "grow_or_rehash state");
    }
    Ok(3)
}

unsafe extern "win64" fn mock_com_release_b(this: usize) -> u32 {
    let c = rd::<u32>(this + 0x8).wrapping_add(1);
    wr::<u32>(this + 0x8, c);
    c
}

pub unsafe fn diff_d3d_streamed_tex_release_range(orig: usize, new: usize) -> Result<u32, String> {
    let mut vtbl = [0usize; 4];
    vtbl[2] = mock_com_release_b as *const () as usize;

    let mut com_o = [vtbl.as_ptr() as usize as u64, 0u64];
    let mut com_n = com_o;

    let mut items_o = [com_o.as_mut_ptr() as usize, 0x1111usize, 0usize, 0x2222usize];
    let mut items_n = [com_n.as_mut_ptr() as usize, 0x1111usize, 0usize, 0x2222usize];

    let range_o = [
        items_o.as_mut_ptr() as usize,
        unsafe { items_o.as_mut_ptr().add(4) } as usize,
    ];
    let range_n = [
        items_n.as_mut_ptr() as usize,
        unsafe { items_n.as_mut_ptr().add(4) } as usize,
    ];

    exe::call_va!(orig, (range_o.as_ptr() as usize) -> ());
    exe::call_va!(new, (range_n.as_ptr() as usize) -> ());

    crate::diff_eq!(com_o[1], com_n[1], "com released");
    crate::diff_eq!(items_o, items_n, "items cleared");
    Ok(1)
}

pub unsafe fn diff_d3d_streamed_tex_ref_dtor(orig: usize, new: usize) -> Result<u32, String> {
    let mut vtbl = [0usize; 4];
    vtbl[2] = mock_com_release_b as *const () as usize;

    let mut com_o = [vtbl.as_ptr() as usize as u64, 0u64];
    let mut com_n = com_o;

    let mut ref_o = [0x1111usize, 0x2222usize, com_o.as_mut_ptr() as usize];
    let mut ref_n = [0x1111usize, 0x2222usize, com_n.as_mut_ptr() as usize];

    exe::call_va!(orig, (ref_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (ref_n.as_mut_ptr() as usize) -> ());

    crate::diff_eq!(com_o[1], com_n[1], "com released");
    crate::diff_eq!(ref_o, ref_n, "ref +0x10 cleared");
    Ok(1)
}

pub unsafe fn diff_d3d_streamed_tex_ref_release_ret(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vtbl = [0usize; 4];
    vtbl[2] = mock_com_release_b as *const () as usize;

    let mut com_o = [vtbl.as_ptr() as usize as u64, 0u64];
    let mut com_n = com_o;

    let mut ref_o = [0x1111usize, 0x2222usize, com_o.as_mut_ptr() as usize];
    let mut ref_n = [0x1111usize, 0x2222usize, com_n.as_mut_ptr() as usize];

    let ro = exe::call_va!(orig, (ref_o.as_mut_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (ref_n.as_mut_ptr() as usize) -> usize);

    crate::diff_eq!(
        ro - (ref_o.as_ptr() as usize),
        rn - (ref_n.as_ptr() as usize),
        "ret self"
    );
    crate::diff_eq!(com_o[1], com_n[1], "com released");
    crate::diff_eq!(ref_o, ref_n, "ref +0x10 cleared");
    Ok(1)
}

// REGISTRY: Replacement::verified_throws(0x14207d340, "d3d::d3d_streamed_tex_reserve_async_lambda_clone_14207d340", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_reserve_async_lambda_clone as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_reserve_async_lambda_clone),
// REGISTRY: Replacement::verified_throws(0x14207d3c0, "d3d::d3d_streamed_tex_reserve_async_lambda_call_14207d3c0", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_reserve_async_lambda_call as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_reserve_async_lambda_call),
// REGISTRY: Replacement::verified_throws(0x14207d440, "d3d::d3d_streamed_tex_try_commit_async_lambda_clone_14207d440", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_try_commit_async_lambda_clone as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_try_commit_async_lambda_clone),
// REGISTRY: Replacement::verified_throws(0x14207d4b0, "d3d::d3d_streamed_tex_try_commit_async_lambda_call_14207d4b0", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_try_commit_async_lambda_call as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_try_commit_async_lambda_call),
// REGISTRY: Replacement::verified_throws(0x14207d540, "d3d::d3d_streamed_tex_commit_async_lambda_clone_14207d540", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_commit_async_lambda_clone as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_commit_async_lambda_clone),
// REGISTRY: Replacement::verified(0x14207d5b0, "d3d::d3d_streamed_tex_commit_async_lambda_call_14207d5b0", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_commit_async_lambda_call as *const (), crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_commit_async_lambda_call),
// REGISTRY: Replacement::verified(0x14207da20, "d3d::d3d_streamed_tex_commit_cb_invoke_14207da20", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_commit_cb_invoke as *const (), crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_commit_cb_invoke),
// REGISTRY: Replacement::verified_throws(0x14207f310, "d3d::d3d_streamed_tex_impl_dtor_14207f310", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_impl_dtor as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_impl_dtor),
// REGISTRY: Replacement::verified_throws(0x14207f750, "d3d::d3d_streamed_tex_try_commit_cb_invoke_14207f750", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_try_commit_cb_invoke as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_try_commit_cb_invoke),
// REGISTRY: Replacement::verified_throws(0x14207fc50, "d3d::d3d_streamed_tex_reserve_cb_invoke_14207fc50", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_reserve_cb_invoke as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_reserve_cb_invoke),
// REGISTRY: Replacement::verified(0x142081110, "d3d::d3d_tex_map_grow_or_rehash_142081110", crate::ns::rend::d3d_streaming_b::d3d_tex_map_grow_or_rehash as *const (), crate::ns::rend::d3d_streaming_b::diff_d3d_tex_map_grow_or_rehash),
// REGISTRY: Replacement::verified(0x1420815a0, "d3d::d3d_streamed_tex_release_range_1420815a0", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_release_range as *const (), crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_release_range),
// REGISTRY: Replacement::verified(0x142081740, "d3d::d3d_streamed_tex_ref_dtor_142081740", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_ref_dtor as *const (), crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_ref_dtor),
// REGISTRY: Replacement::verified(0x1420817a0, "d3d::d3d_streamed_tex_ref_release_ret_1420817a0", crate::ns::rend::d3d_streaming_b::d3d_streamed_tex_ref_release_ret as *const (), crate::ns::rend::d3d_streaming_b::diff_d3d_streamed_tex_ref_release_ret),
