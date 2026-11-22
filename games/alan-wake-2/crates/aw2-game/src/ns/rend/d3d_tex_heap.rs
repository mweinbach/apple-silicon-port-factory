use crate::exe::{self, rd, wr};

#[inline]
unsafe fn advance_swiss_iter(mut ctrl: usize, mut slot: usize) -> (usize, usize) {
    while rd::<i8>(ctrl) < -1 {
        let mut mask = 0u32;
        let mut i = 0usize;
        while i < 16 {
            if rd::<i8>(ctrl.wrapping_add(i)) < -1 {
                mask |= 1u32 << i;
            }
            i += 1;
        }
        let step = (mask.wrapping_add(1)).trailing_zeros() as usize;
        ctrl = ctrl.wrapping_add(step);
        slot = slot.wrapping_add(step << 6);
    }
    (ctrl, slot)
}

/// Original: `0x142081a30` (30 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_init_entry_60(this: usize) -> usize {
    let mut off = 0usize;
    while off < 0x60 {
        wr::<u64>(this.wrapping_add(off), 0);
        off += 8;
    }
    this
}

pub unsafe fn diff_d3d_tex_heap_init_entry_60(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0xAAu8; 0x70];
    let mut buf_n = [0xAAu8; 0x70];
    let po = buf_o.as_mut_ptr().add(4) as usize;
    let pn = buf_n.as_mut_ptr().add(4) as usize;
    let ro = exe::call_va!(orig, (po) -> usize);
    let rn = exe::call_va!(new, (pn) -> usize);
    crate::diff_eq!(ro.wrapping_sub(po), rn.wrapping_sub(pn), "ret offset");
    crate::diff_eq!(buf_o, buf_n, "buffer");
    Ok(1)
}

/// Original: `0x142081a60` (20 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_init_handle_ffff(this: usize) -> usize {
    wr::<u32>(this.wrapping_add(0x8), 0xffff);
    wr::<u64>(this, 0);
    wr::<u16>(this.wrapping_add(0xc), 0);
    this
}

pub unsafe fn diff_d3d_tex_heap_init_handle_ffff(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0x55u8; 0x20];
    let mut buf_n = [0x55u8; 0x20];
    let po = buf_o.as_mut_ptr().add(4) as usize;
    let pn = buf_n.as_mut_ptr().add(4) as usize;
    let ro = exe::call_va!(orig, (po) -> usize);
    let rn = exe::call_va!(new, (pn) -> usize);
    crate::diff_eq!(ro.wrapping_sub(po), rn.wrapping_sub(pn), "ret offset");
    crate::diff_eq!(buf_o, buf_n, "buffer");
    Ok(1)
}

/// Original: `0x142081a80` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_zero_1e(this: usize) -> usize {
    wr::<u64>(this, 0);
    wr::<u64>(this.wrapping_add(0x8), 0);
    wr::<u64>(this.wrapping_add(0x10), 0);
    wr::<u32>(this.wrapping_add(0x18), 0);
    wr::<u16>(this.wrapping_add(0x1c), 0);
    this
}

pub unsafe fn diff_d3d_tex_heap_zero_1e(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0x77u8; 0x30];
    let mut buf_n = [0x77u8; 0x30];
    let po = buf_o.as_mut_ptr().add(4) as usize;
    let pn = buf_n.as_mut_ptr().add(4) as usize;
    let ro = exe::call_va!(orig, (po) -> usize);
    let rn = exe::call_va!(new, (pn) -> usize);
    crate::diff_eq!(ro.wrapping_sub(po), rn.wrapping_sub(pn), "ret offset");
    crate::diff_eq!(buf_o, buf_n, "buffer");
    Ok(1)
}

/// Original: `0x142081aa0` (16 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_zero_18_head_u32(this: usize) -> usize {
    wr::<u32>(this, 0);
    wr::<u64>(this.wrapping_add(0x8), 0);
    wr::<u64>(this.wrapping_add(0x10), 0);
    this
}

pub unsafe fn diff_d3d_tex_heap_zero_18_head_u32(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0x99u8; 0x28];
    let mut buf_n = [0x99u8; 0x28];
    let po = buf_o.as_mut_ptr().add(4) as usize;
    let pn = buf_n.as_mut_ptr().add(4) as usize;
    let ro = exe::call_va!(orig, (po) -> usize);
    let rn = exe::call_va!(new, (pn) -> usize);
    crate::diff_eq!(ro.wrapping_sub(po), rn.wrapping_sub(pn), "ret offset");
    crate::diff_eq!(buf_o, buf_n, "buffer");
    Ok(1)
}

/// Original: `0x142081ea0` (18 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_tex_heap_alloc_array_60(_this: usize, count: usize) -> usize {
    let bytes = count.wrapping_mul(3) << 5;
    exe::call_va!(0x1_4241_c0a0, (bytes, 8usize) -> usize)
}

pub unsafe fn diff_d3d_tex_heap_alloc_array_60(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_alloc(bytes: usize, align: usize) -> usize {
        0xCAFE_0000usize.wrapping_add(bytes).wrapping_add(align)
    }
    let _m = crate::difftest::Mock::new(0x1_4241_c0a0, fake_alloc as *const ())?;
    for &c in &[0usize, 1, 7] {
        let ro = exe::call_va!(orig, (0usize, c) -> usize);
        let rn = exe::call_va!(new, (0usize, c) -> usize);
        crate::diff_eq!(ro, rn, "alloc_array_60 count={c}");
    }
    Ok(3)
}

/// Original: `0x142081ec0` (18 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_tex_heap_alloc_array_30(_this: usize, count: usize) -> usize {
    let bytes = count.wrapping_mul(3) << 4;
    exe::call_va!(0x1_4241_c0a0, (bytes, 8usize) -> usize)
}

pub unsafe fn diff_d3d_tex_heap_alloc_array_30(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_alloc(bytes: usize, align: usize) -> usize {
        0xBEEF_0000usize.wrapping_add(bytes).wrapping_add(align)
    }
    let _m = crate::difftest::Mock::new(0x1_4241_c0a0, fake_alloc as *const ())?;
    for &c in &[0usize, 1, 9] {
        let ro = exe::call_va!(orig, (0usize, c) -> usize);
        let rn = exe::call_va!(new, (0usize, c) -> usize);
        crate::diff_eq!(ro, rn, "alloc_array_30 count={c}");
    }
    Ok(3)
}

/// Original: `0x142081ee0` (14 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_tex_heap_alloc_array_38(_this: usize, count: usize) -> usize {
    let bytes = count.wrapping_mul(0x38);
    exe::call_va!(0x1_4241_c0a0, (bytes, 8usize) -> usize)
}

pub unsafe fn diff_d3d_tex_heap_alloc_array_38(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_alloc(bytes: usize, align: usize) -> usize {
        0xFACE_0000usize.wrapping_add(bytes).wrapping_add(align)
    }
    let _m = crate::difftest::Mock::new(0x1_4241_c0a0, fake_alloc as *const ())?;
    for &c in &[0usize, 1, 11] {
        let ro = exe::call_va!(orig, (0usize, c) -> usize);
        let rn = exe::call_va!(new, (0usize, c) -> usize);
        crate::diff_eq!(ro, rn, "alloc_array_38 count={c}");
    }
    Ok(3)
}

/// Original: `0x142081f80` (76 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_release_span_stride_18(this: usize) {
    let end = rd::<usize>(this.wrapping_add(0x8));
    let mut cur = rd::<usize>(this);
    while cur != end {
        let obj = rd::<usize>(cur.wrapping_add(0x10));
        if obj != 0 {
            let vtbl = rd::<usize>(obj);
            let release = rd::<usize>(vtbl.wrapping_add(0x10));
            exe::call_va!(release, (obj) -> ());
        }
        wr::<usize>(cur.wrapping_add(0x10), 0);
        cur = cur.wrapping_add(0x18);
    }
}

pub unsafe fn diff_d3d_tex_heap_release_span_stride_18(orig: usize, new: usize) -> Result<u32, String> {
    let mut data_o = [0u64; 6];
    let mut data_n = [0u64; 6];
    let mut span_o = [data_o.as_mut_ptr() as usize, data_o.as_mut_ptr().add(6) as usize];
    let mut span_n = [data_n.as_mut_ptr() as usize, data_n.as_mut_ptr().add(6) as usize];
    exe::call_va!(orig, (span_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (span_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(data_o, data_n, "data");
    Ok(1)
}

/// Original: `0x142082010` (74 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_release_span_stride_20(this: usize) {
    let end = rd::<usize>(this.wrapping_add(0x8));
    let mut cur = rd::<usize>(this);
    while cur != end {
        let obj = rd::<usize>(cur);
        if obj != 0 {
            let vtbl = rd::<usize>(obj);
            let release = rd::<usize>(vtbl.wrapping_add(0x10));
            exe::call_va!(release, (obj) -> ());
        }
        wr::<usize>(cur, 0);
        cur = cur.wrapping_add(0x20);
    }
}

pub unsafe fn diff_d3d_tex_heap_release_span_stride_20(orig: usize, new: usize) -> Result<u32, String> {
    let mut data_o = [0u64; 8];
    let mut data_n = [0u64; 8];
    let mut span_o = [data_o.as_mut_ptr() as usize, data_o.as_mut_ptr().add(8) as usize];
    let mut span_n = [data_n.as_mut_ptr() as usize, data_n.as_mut_ptr().add(8) as usize];
    exe::call_va!(orig, (span_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (span_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(data_o, data_n, "data");
    Ok(1)
}

/// Original: `0x142082070` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_move_assign_pair_10(this: usize, src: usize) -> usize {
    let old = rd::<usize>(this);
    if old != 0 {
        let vtbl = rd::<usize>(old);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (old) -> ());
    }
    wr::<usize>(this, 0);
    let p0 = rd::<u64>(src);
    wr::<u64>(src, 0);
    wr::<u64>(this, p0);
    let p1 = rd::<u64>(src.wrapping_add(0x8));
    wr::<u64>(this.wrapping_add(0x8), p1);
    this
}

pub unsafe fn diff_d3d_tex_heap_move_assign_pair_10(orig: usize, new: usize) -> Result<u32, String> {
    let mut dst_o = [0u64, 0x1111u64];
    let mut src_o = [0x2222u64, 0x3333u64];
    let mut dst_n = dst_o;
    let mut src_n = src_o;
    let ro = exe::call_va!(orig, (dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(ro == dst_o.as_ptr() as usize, rn == dst_n.as_ptr() as usize, "ret");
    crate::diff_eq!(dst_o, dst_n, "dst");
    crate::diff_eq!(src_o, src_n, "src");
    Ok(1)
}

/// Original: `0x142082160` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_iter_sub_assign_60(this: usize, out: usize) -> usize {
    let v = rd::<usize>(this).wrapping_sub(0x60);
    wr::<usize>(this, v);
    wr::<usize>(out, v);
    out
}

pub unsafe fn diff_d3d_tex_heap_iter_sub_assign_60(orig: usize, new: usize) -> Result<u32, String> {
    let mut this_o = 0x1000usize;
    let mut out_o = 0usize;
    let mut this_n = 0x1000usize;
    let mut out_n = 0usize;
    let _ = exe::call_va!(orig, (&mut this_o as *mut usize as usize, &mut out_o as *mut usize as usize) -> usize);
    let _ = exe::call_va!(new, (&mut this_n as *mut usize as usize, &mut out_n as *mut usize as usize) -> usize);
    crate::diff_eq!(this_o, this_n, "this");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x1420821a0` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_iter_sub_assign_38(this: usize, out: usize) -> usize {
    let v = rd::<usize>(this).wrapping_sub(0x38);
    wr::<usize>(this, v);
    wr::<usize>(out, v);
    out
}

pub unsafe fn diff_d3d_tex_heap_iter_sub_assign_38(orig: usize, new: usize) -> Result<u32, String> {
    let mut this_o = 0x1000usize;
    let mut out_o = 0usize;
    let mut this_n = 0x1000usize;
    let mut out_n = 0usize;
    let _ = exe::call_va!(orig, (&mut this_o as *mut usize as usize, &mut out_o as *mut usize as usize) -> usize);
    let _ = exe::call_va!(new, (&mut this_n as *mut usize as usize, &mut out_n as *mut usize as usize) -> usize);
    crate::diff_eq!(this_o, this_n, "this");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142082750` (8 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ptr_minus_60(this: usize) -> usize {
    rd::<usize>(this).wrapping_sub(0x60)
}

pub unsafe fn diff_d3d_tex_heap_ptr_minus_60(orig: usize, new: usize) -> Result<u32, String> {
    let val = 0x2000usize;
    let ro = exe::call_va!(orig, (&val as *const usize as usize) -> usize);
    let rn = exe::call_va!(new, (&val as *const usize as usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x142082770` (8 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ptr_minus_38(this: usize) -> usize {
    rd::<usize>(this).wrapping_sub(0x38)
}

pub unsafe fn diff_d3d_tex_heap_ptr_minus_38(orig: usize, new: usize) -> Result<u32, String> {
    let val = 0x2000usize;
    let ro = exe::call_va!(orig, (&val as *const usize as usize) -> usize);
    let rn = exe::call_va!(new, (&val as *const usize as usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x1420827c0` (57 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_dtor_lambda_notify(this: usize) {
    let sync = rd::<usize>(this.wrapping_add(0x8));
    let mtx = sync.wrapping_add(0x8);
    exe::call_iat!(0x1_4264_55a8, (mtx) -> ());
    wr::<u32>(sync.wrapping_add(0x10), rd::<u32>(sync.wrapping_add(0x10)).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (mtx) -> ());
    exe::call_iat!(0x1_4264_5590, (sync) -> ());
}

pub unsafe fn diff_d3d_tex_heap_dtor_lambda_notify(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x10));
    }
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x100));
    }
    unsafe extern "win64" fn fake_wake(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x1000));
    }
    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5590, fake_wake as *const ())?;
    let mut sync_o = [0u32; 8];
    let mut sync_n = [0u32; 8];
    sync_o[4] = 7;
    sync_n[4] = 7;
    let this_o = [0usize, sync_o.as_mut_ptr() as usize];
    let this_n = [0usize, sync_n.as_mut_ptr() as usize];
    exe::call_va!(orig, (this_o.as_ptr() as usize) -> ());
    exe::call_va!(new, (this_n.as_ptr() as usize) -> ());
    crate::diff_eq!(sync_o, sync_n, "sync");
    Ok(1)
}

/// Original: `0x142082840` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_move_triple_18(_this: usize, dst: usize, src: usize) {
    let v0 = rd::<u64>(src);
    wr::<u64>(src, 0);
    wr::<u64>(dst, v0);
    let v1 = rd::<u64>(src.wrapping_add(0x8));
    wr::<u64>(src.wrapping_add(0x8), 0);
    wr::<u64>(dst.wrapping_add(0x8), v1);
    let v2 = rd::<u64>(src.wrapping_add(0x10));
    wr::<u64>(dst.wrapping_add(0x10), v2);
}

pub unsafe fn diff_d3d_tex_heap_move_triple_18(orig: usize, new: usize) -> Result<u32, String> {
    let mut dst_o = [0u64; 3];
    let mut src_o = [11u64, 22u64, 33u64];
    let mut dst_n = dst_o;
    let mut src_n = src_o;
    exe::call_va!(orig, (0usize, dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (0usize, dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(dst_o, dst_n, "dst");
    crate::diff_eq!(src_o, src_n, "src");
    Ok(1)
}

/// Original: `0x142083420` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_back_18(this: usize) -> usize {
    let count_m1 = rd::<u32>(this.wrapping_add(0x10)).wrapping_sub(1) as u64;
    let head = rd::<u32>(this.wrapping_add(0x8)) as u64;
    let cap = rd::<u32>(this.wrapping_add(0x14)) as u64;
    let idx = (count_m1.wrapping_add(head)) % cap;
    let base = rd::<usize>(this);
    base.wrapping_add((idx as usize).wrapping_mul(0x18))
}

pub unsafe fn diff_d3d_tex_heap_ring_back_18(orig: usize, new: usize) -> Result<u32, String> {
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let ring = Ring {
        base: 0x4000,
        head: 5,
        tail: 2,
        count: 4,
        cap: 7,
    };
    let ro = exe::call_va!(orig, (&ring as *const Ring as usize) -> usize);
    let rn = exe::call_va!(new, (&ring as *const Ring as usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x142084190` (36 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_move_init_triple_18(dst: usize, src: usize) -> usize {
    let v0 = rd::<u64>(src);
    wr::<u64>(src, 0);
    wr::<u64>(dst, v0);
    let v1 = rd::<u64>(src.wrapping_add(0x8));
    wr::<u64>(src.wrapping_add(0x8), 0);
    wr::<u64>(dst.wrapping_add(0x8), v1);
    let v2 = rd::<u64>(src.wrapping_add(0x10));
    wr::<u64>(dst.wrapping_add(0x10), v2);
    dst
}

pub unsafe fn diff_d3d_tex_heap_move_init_triple_18(orig: usize, new: usize) -> Result<u32, String> {
    let mut dst_o = [0u64; 3];
    let mut src_o = [101u64, 202u64, 303u64];
    let mut dst_n = dst_o;
    let mut src_n = src_o;
    let _ = exe::call_va!(orig, (dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(dst_o, dst_n, "dst");
    crate::diff_eq!(src_o, src_n, "src");
    Ok(1)
}

/// Original: `0x1420841d0` (80 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_zero_tail_items_20(this: usize, _unused: usize, mut count: usize) -> usize {
    let vec_ptr = rd::<usize>(this);
    let len = rd::<u32>(vec_ptr.wrapping_add(0x8)) as usize;
    let mut cur = rd::<usize>(vec_ptr).wrapping_add(len << 5);
    while count != 0 {
        wr::<u64>(cur, 0);
        wr::<u64>(cur.wrapping_add(0x8), 0);
        wr::<u64>(cur.wrapping_add(0x10), 0);
        wr::<u64>(cur.wrapping_add(0x18), 0);
        cur = cur.wrapping_add(0x20);
        count -= 1;
    }
    cur
}

pub unsafe fn diff_d3d_tex_heap_zero_tail_items_20(orig: usize, new: usize) -> Result<u32, String> {
    let mut backing_o = [0xCCu8; 0x80];
    let mut backing_n = [0xCCu8; 0x80];
    #[repr(C)]
    struct VecHdr {
        data: usize,
        len: u32,
        cap: u32,
    }
    let hdr_o = VecHdr { data: backing_o.as_mut_ptr() as usize, len: 1, cap: 4 };
    let hdr_n = VecHdr { data: backing_n.as_mut_ptr() as usize, len: 1, cap: 4 };
    let ptr_o = &hdr_o as *const VecHdr as usize;
    let ptr_n = &hdr_n as *const VecHdr as usize;
    let ro = exe::call_va!(orig, (&ptr_o as *const usize as usize, 0usize, 2usize) -> usize);
    let rn = exe::call_va!(new, (&ptr_n as *const usize as usize, 0usize, 2usize) -> usize);
    crate::diff_eq!(ro.wrapping_sub(hdr_o.data), rn.wrapping_sub(hdr_n.data), "ret offset");
    crate::diff_eq!(backing_o, backing_n, "backing");
    Ok(1)
}

/// Original: `0x142084250` (54 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_zero_tail_items_18(this: usize, _unused: usize, mut count: usize) -> usize {
    let vec_ptr = rd::<usize>(this);
    let len = rd::<u32>(vec_ptr.wrapping_add(0x8)) as usize;
    let mut cur = rd::<usize>(vec_ptr).wrapping_add(len.wrapping_mul(0x18));
    while count != 0 {
        wr::<u64>(cur, 0);
        wr::<u64>(cur.wrapping_add(0x8), 0);
        wr::<u64>(cur.wrapping_add(0x10), 0);
        cur = cur.wrapping_add(0x18);
        count -= 1;
    }
    cur
}

pub unsafe fn diff_d3d_tex_heap_zero_tail_items_18(orig: usize, new: usize) -> Result<u32, String> {
    let mut backing_o = [0xDDu8; 0x60];
    let mut backing_n = [0xDDu8; 0x60];
    #[repr(C)]
    struct VecHdr {
        data: usize,
        len: u32,
        cap: u32,
    }
    let hdr_o = VecHdr { data: backing_o.as_mut_ptr() as usize, len: 1, cap: 4 };
    let hdr_n = VecHdr { data: backing_n.as_mut_ptr() as usize, len: 1, cap: 4 };
    let ptr_o = &hdr_o as *const VecHdr as usize;
    let ptr_n = &hdr_n as *const VecHdr as usize;
    let ro = exe::call_va!(orig, (&ptr_o as *const usize as usize, 0usize, 2usize) -> usize);
    let rn = exe::call_va!(new, (&ptr_n as *const usize as usize, 0usize, 2usize) -> usize);
    crate::diff_eq!(ro.wrapping_sub(hdr_o.data), rn.wrapping_sub(hdr_n.data), "ret offset");
    crate::diff_eq!(backing_o, backing_n, "backing");
    Ok(1)
}

/// Original: `0x142084530` (157 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_clear_18(this: usize) {
    let mut i = 0u64;
    if rd::<u32>(this.wrapping_add(0x10)) > 0 {
        loop {
            let base = rd::<usize>(this);
            let head = rd::<u32>(this.wrapping_add(0x8)) as u64;
            let cap = rd::<u32>(this.wrapping_add(0x14)) as u64;
            let idx = (head.wrapping_add(i)) % cap;
            let elem = base.wrapping_add((idx as usize).wrapping_mul(0x18));
            let p1 = rd::<usize>(elem.wrapping_add(0x8));
            if p1 != 0 {
                let vtbl = rd::<usize>(p1);
                let release = rd::<usize>(vtbl.wrapping_add(0x10));
                exe::call_va!(release, (p1) -> ());
            }
            wr::<usize>(elem.wrapping_add(0x8), 0);
            let p0 = rd::<usize>(elem);
            if p0 != 0 {
                let vtbl = rd::<usize>(p0);
                let release = rd::<usize>(vtbl.wrapping_add(0x10));
                exe::call_va!(release, (p0) -> ());
            }
            wr::<usize>(elem, 0);
            i += 1;
            if i >= rd::<u32>(this.wrapping_add(0x10)) as u64 {
                break;
            }
        }
    }
    wr::<u64>(this.wrapping_add(0x8), 0);
    wr::<u32>(this.wrapping_add(0x10), 0);
}

pub unsafe fn diff_d3d_tex_heap_ring_clear_18(orig: usize, new: usize) -> Result<u32, String> {
    let mut elems_o = [0u64; 9];
    let mut elems_n = [0u64; 9];
    #[repr(C)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let mut r_o = Ring { base: elems_o.as_mut_ptr() as usize, head: 1, tail: 0, count: 2, cap: 3 };
    let mut r_n = Ring { base: elems_n.as_mut_ptr() as usize, head: 1, tail: 0, count: 2, cap: 3 };
    exe::call_va!(orig, (&mut r_o as *mut Ring as usize) -> ());
    exe::call_va!(new, (&mut r_n as *mut Ring as usize) -> ());
    crate::diff_eq!(r_o.head, r_n.head, "head");
    crate::diff_eq!(r_o.tail, r_n.tail, "tail");
    crate::diff_eq!(r_o.count, r_n.count, "count");
    Ok(1)
}

/// Original: `0x142084730` (89 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swiss_iter_post_inc_a(this: usize, out: usize) -> usize {
    let c0 = rd::<usize>(this);
    let s0 = rd::<usize>(this.wrapping_add(0x8));
    let mut ctrl = c0.wrapping_add(1);
    let mut slot = s0.wrapping_add(0x40);
    wr::<usize>(this, ctrl);
    wr::<usize>(this.wrapping_add(0x8), slot);
    wr::<usize>(out, c0);
    wr::<usize>(out.wrapping_add(0x8), s0);
    if rd::<i8>(ctrl) < -1 {
        let (nc, ns) = advance_swiss_iter(ctrl, slot);
        ctrl = nc;
        slot = ns;
        wr::<usize>(this, ctrl);
        wr::<usize>(this.wrapping_add(0x8), slot);
    }
    out
}

pub unsafe fn diff_d3d_tex_heap_swiss_iter_post_inc_a(orig: usize, new: usize) -> Result<u32, String> {
    let ctrl_bytes: [u8; 32] = [0x01, 0x80, 0x80, 0xFE, 0x05, 0xFF, 0x80, 0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut it_o = [ctrl_bytes.as_ptr() as usize, 0x10000usize];
    let mut it_n = it_o;
    let mut out_o = [0usize; 2];
    let mut out_n = [0usize; 2];
    let _ = exe::call_va!(orig, (it_o.as_mut_ptr() as usize, out_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (it_n.as_mut_ptr() as usize, out_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(it_o, it_n, "iter");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x1420847c0` (23 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swiss_alloc_size(this: usize) -> usize {
    let a = (rd::<usize>(this).wrapping_add(7)) & !7usize;
    let b = rd::<usize>(this.wrapping_add(0x8)) << 6;
    a.wrapping_add(b)
}

pub unsafe fn diff_d3d_tex_heap_swiss_alloc_size(orig: usize, new: usize) -> Result<u32, String> {
    let pair = [19usize, 16usize];
    let ro = exe::call_va!(orig, (pair.as_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (pair.as_ptr() as usize) -> usize);
    crate::diff_eq!(ro, rn, "size");
    Ok(1)
}

/// Original: `0x1420847e0` (47 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_destroy_std_func_38(this: usize) -> usize {
    let fn_obj = rd::<usize>(this.wrapping_add(0x38));
    if fn_obj != 0 {
        let vtbl = rd::<usize>(fn_obj);
        let destroy = rd::<usize>(vtbl.wrapping_add(0x20));
        let is_heap = u8::from(fn_obj != this);
        exe::call_va!(destroy, (fn_obj, is_heap) -> ());
        wr::<usize>(this.wrapping_add(0x38), 0);
    }
    this
}

pub unsafe fn diff_d3d_tex_heap_destroy_std_func_38(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0u64; 8];
    let mut buf_n = [0u64; 8];
    let _ = exe::call_va!(orig, (buf_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (buf_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(buf_o, buf_n, "buf");
    Ok(1)
}

/// Original: `0x142084820` (68 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_release_tile_entry_ret(this: usize) -> usize {
    let alloc_handle = rd::<usize>(this.wrapping_add(0x18));
    if alloc_handle != 0 {
        let mgr = rd::<usize>(this.wrapping_add(0x30));
        let page_count = rd::<u16>(this.wrapping_add(0x28)) as u32;
        let vtbl = rd::<usize>(mgr);
        let free_fn = rd::<usize>(vtbl.wrapping_add(0x10));
        let arg2 = rd::<usize>(this.wrapping_add(0x20));
        exe::call_va!(free_fn, (mgr, alloc_handle, arg2, page_count) -> ());
    }
    let com_ptr = rd::<usize>(this);
    if com_ptr != 0 {
        let vtbl = rd::<usize>(com_ptr);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (com_ptr) -> ());
    }
    wr::<usize>(this, 0);
    this
}

pub unsafe fn diff_d3d_tex_heap_release_tile_entry_ret(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0u64; 7];
    let mut buf_n = [0u64; 7];
    let _ = exe::call_va!(orig, (buf_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (buf_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(buf_o, buf_n, "buf");
    Ok(1)
}

/// Original: `0x1420849c0` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_at_60(this: usize, index: usize) -> usize {
    let head = rd::<u32>(this.wrapping_add(0x8)) as u64;
    let cap = rd::<u32>(this.wrapping_add(0x14)) as u64;
    let slot = (head.wrapping_add(index as u64)) % cap;
    let off = (slot as usize).wrapping_mul(3) << 5;
    rd::<usize>(this).wrapping_add(off)
}

pub unsafe fn diff_d3d_tex_heap_ring_at_60(orig: usize, new: usize) -> Result<u32, String> {
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let r = Ring { base: 0x8000, head: 3, tail: 0, count: 5, cap: 6 };
    let ro = exe::call_va!(orig, (&r as *const Ring as usize, 4usize) -> usize);
    let rn = exe::call_va!(new, (&r as *const Ring as usize, 4usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x142084ac0` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_at_18(this: usize, index: usize) -> usize {
    let cap = rd::<u32>(this.wrapping_add(0x14)) as u64;
    let head = rd::<u32>(this.wrapping_add(0x8)) as u64;
    let slot = (head.wrapping_add(index as u64)) % cap;
    let base = rd::<usize>(this);
    base.wrapping_add((slot as usize).wrapping_mul(0x18))
}

pub unsafe fn diff_d3d_tex_heap_ring_at_18(orig: usize, new: usize) -> Result<u32, String> {
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let r = Ring { base: 0x8000, head: 3, tail: 0, count: 5, cap: 6 };
    let ro = exe::call_va!(orig, (&r as *const Ring as usize, 4usize) -> usize);
    let rn = exe::call_va!(new, (&r as *const Ring as usize, 4usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x142084e80` (25 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swiss_iter_offset(this: usize, out: usize, offset: usize) -> usize {
    let ctrl = rd::<usize>(this).wrapping_add(offset);
    let slot = rd::<usize>(this.wrapping_add(0x8)).wrapping_add(offset << 6);
    wr::<usize>(out.wrapping_add(0x8), slot);
    wr::<usize>(out, ctrl);
    out
}

pub unsafe fn diff_d3d_tex_heap_swiss_iter_offset(orig: usize, new: usize) -> Result<u32, String> {
    let base = [0x1000usize, 0x20000usize];
    let mut out_o = [0usize; 2];
    let mut out_n = [0usize; 2];
    let _ = exe::call_va!(orig, (base.as_ptr() as usize, out_o.as_mut_ptr() as usize, 5usize) -> usize);
    let _ = exe::call_va!(new, (base.as_ptr() as usize, out_n.as_mut_ptr() as usize, 5usize) -> usize);
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142084f50` (68 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swiss_iter_skip_empty(this: usize) {
    let ctrl = rd::<usize>(this);
    if rd::<i8>(ctrl) < -1 {
        let slot = rd::<usize>(this.wrapping_add(0x8));
        let (nc, ns) = advance_swiss_iter(ctrl, slot);
        wr::<usize>(this, nc);
        wr::<usize>(this.wrapping_add(0x8), ns);
    }
}

pub unsafe fn diff_d3d_tex_heap_swiss_iter_skip_empty(orig: usize, new: usize) -> Result<u32, String> {
    let ctrl_bytes: [u8; 32] = [0x80, 0xFE, 0x80, 0x02, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut it_o = [ctrl_bytes.as_ptr() as usize, 0x4000usize];
    let mut it_n = it_o;
    exe::call_va!(orig, (it_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (it_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(it_o, it_n, "iter");
    Ok(1)
}

/// Original: `0x142084fa0` (93 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swiss_iter_post_inc_b(this: usize, out: usize) -> usize {
    let c0 = rd::<usize>(this);
    let s0 = rd::<usize>(this.wrapping_add(0x8));
    let ctrl = c0.wrapping_add(1);
    let slot = s0.wrapping_add(0x40);
    wr::<usize>(this, ctrl);
    wr::<usize>(this.wrapping_add(0x8), slot);
    if rd::<i8>(ctrl) < -1 {
        let (nc, ns) = advance_swiss_iter(ctrl, slot);
        wr::<usize>(this, nc);
        wr::<usize>(this.wrapping_add(0x8), ns);
    }
    wr::<usize>(out, c0);
    wr::<usize>(out.wrapping_add(0x8), s0);
    out
}

pub unsafe fn diff_d3d_tex_heap_swiss_iter_post_inc_b(orig: usize, new: usize) -> Result<u32, String> {
    let ctrl_bytes: [u8; 32] = [0x01, 0x80, 0xFE, 0x07, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut it_o = [ctrl_bytes.as_ptr() as usize, 0x8000usize];
    let mut it_n = it_o;
    let mut out_o = [0usize; 2];
    let mut out_n = [0usize; 2];
    let _ = exe::call_va!(orig, (it_o.as_mut_ptr() as usize, out_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (it_n.as_mut_ptr() as usize, out_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(it_o, it_n, "iter");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142085050` (62 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_release_pair_ret(this: usize) -> usize {
    let p1 = rd::<usize>(this.wrapping_add(0x8));
    if p1 != 0 {
        let vtbl = rd::<usize>(p1);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (p1) -> ());
    }
    wr::<usize>(this.wrapping_add(0x8), 0);
    let p0 = rd::<usize>(this);
    if p0 != 0 {
        let vtbl = rd::<usize>(p0);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (p0) -> ());
    }
    wr::<usize>(this, 0);
    this
}

pub unsafe fn diff_d3d_tex_heap_release_pair_ret(orig: usize, new: usize) -> Result<u32, String> {
    let mut pair_o = [0usize; 2];
    let mut pair_n = [0usize; 2];
    let _ = exe::call_va!(orig, (pair_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (pair_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(pair_o, pair_n, "pair");
    Ok(1)
}

/// Original: `0x142085260` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_back_60(this: usize) -> usize {
    let count_m1 = rd::<u32>(this.wrapping_add(0x10)).wrapping_sub(1) as u64;
    let head = rd::<u32>(this.wrapping_add(0x8)) as u64;
    let cap = rd::<u32>(this.wrapping_add(0x14)) as u64;
    let slot = (count_m1.wrapping_add(head)) % cap;
    let off = (slot as usize).wrapping_mul(3) << 5;
    rd::<usize>(this).wrapping_add(off)
}

pub unsafe fn diff_d3d_tex_heap_ring_back_60(orig: usize, new: usize) -> Result<u32, String> {
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let r = Ring { base: 0x10000, head: 4, tail: 0, count: 3, cap: 5 };
    let ro = exe::call_va!(orig, (&r as *const Ring as usize) -> usize);
    let rn = exe::call_va!(new, (&r as *const Ring as usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x1420852e0` (104 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_clear_60(this: usize) {
    let mut i = 0u64;
    if rd::<u32>(this.wrapping_add(0x10)) > 0 {
        loop {
            let cap = rd::<u32>(this.wrapping_add(0x14)) as u64;
            let head = rd::<u32>(this.wrapping_add(0x8)) as u64;
            let slot = (head.wrapping_add(i)) % cap;
            let off = (slot as usize).wrapping_mul(3) << 5;
            let elem = rd::<usize>(this).wrapping_add(off);
            exe::call_va!(0x1_4208_50b0, (elem) -> ());
            i += 1;
            if i >= rd::<u32>(this.wrapping_add(0x10)) as u64 {
                break;
            }
        }
    }
    wr::<u64>(this.wrapping_add(0x8), 0);
    wr::<u32>(this.wrapping_add(0x10), 0);
}

pub unsafe fn diff_d3d_tex_heap_ring_clear_60(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_dtor(elem: usize) {
        wr::<u32>(elem, rd::<u32>(elem).wrapping_add(0x77));
    }
    let _m = crate::difftest::Mock::new(0x1_4208_50b0, fake_dtor as *const ())?;
    let mut buf_o = [0u8; 0x120];
    let mut buf_n = [0u8; 0x120];
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let mut r_o = Ring { base: buf_o.as_mut_ptr() as usize, head: 1, tail: 0, count: 2, cap: 3 };
    let mut r_n = Ring { base: buf_n.as_mut_ptr() as usize, head: 1, tail: 0, count: 2, cap: 3 };
    exe::call_va!(orig, (&mut r_o as *mut Ring as usize) -> ());
    exe::call_va!(new, (&mut r_n as *mut Ring as usize) -> ());
    crate::diff_eq!(buf_o, buf_n, "buf");
    crate::diff_eq!(r_o.head, r_n.head, "head");
    crate::diff_eq!(r_o.tail, r_n.tail, "tail");
    crate::diff_eq!(r_o.count, r_n.count, "count");
    Ok(1)
}

/// Original: `0x142085350` (85 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_push_60(this: usize, item: usize) {
    let count = rd::<u32>(this.wrapping_add(0x10));
    if count == rd::<u32>(this.wrapping_add(0x14)) {
        exe::call_va!(0x1_4208_3150, (this, count.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(this.wrapping_add(0xc)) as usize;
    let dst = rd::<usize>(this).wrapping_add(tail.wrapping_mul(3) << 5);
    exe::call_va!(0x1_4208_4000, (dst, item) -> ());
    let new_tail = rd::<u32>(this.wrapping_add(0xc)).wrapping_add(1);
    wr::<u32>(this.wrapping_add(0xc), new_tail);
    wr::<u32>(this.wrapping_add(0x10), rd::<u32>(this.wrapping_add(0x10)).wrapping_add(1));
    if rd::<u32>(this.wrapping_add(0xc)) == rd::<u32>(this.wrapping_add(0x14)) {
        wr::<u32>(this.wrapping_add(0xc), 0);
    }
}

pub unsafe fn diff_d3d_tex_heap_ring_push_60(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_grow(ring: usize, new_cap: u32) {
        wr::<u32>(ring.wrapping_add(0x14), new_cap);
    }
    unsafe extern "win64" fn fake_move(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u64>(src, 0);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4208_3150, fake_grow as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4208_4000, fake_move as *const ())?;
    let mut buf_o = [0u8; 0x120];
    let mut buf_n = [0u8; 0x120];
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let mut r_o = Ring { base: buf_o.as_mut_ptr() as usize, head: 0, tail: 1, count: 1, cap: 2 };
    let mut r_n = Ring { base: buf_n.as_mut_ptr() as usize, head: 0, tail: 1, count: 1, cap: 2 };
    let mut item_o = [0x1234_5678u64; 12];
    let mut item_n = item_o;
    exe::call_va!(orig, (&mut r_o as *mut Ring as usize, item_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (&mut r_n as *mut Ring as usize, item_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(buf_o, buf_n, "buf");
    crate::diff_eq!(item_o, item_n, "item");
    crate::diff_eq!(r_o.tail, r_n.tail, "tail");
    crate::diff_eq!(r_o.count, r_n.count, "count");
    Ok(1)
}

/// Original: `0x142085460` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_front_18(this: usize) -> usize {
    let cap = rd::<u32>(this.wrapping_add(0x14)) as u64;
    let head = rd::<u32>(this.wrapping_add(0x8)) as u64;
    let slot = head % cap;
    let base = rd::<usize>(this);
    base.wrapping_add((slot as usize).wrapping_mul(0x18))
}

pub unsafe fn diff_d3d_tex_heap_ring_front_18(orig: usize, new: usize) -> Result<u32, String> {
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let r = Ring { base: 0x9000, head: 9, tail: 0, count: 2, cap: 4 };
    let ro = exe::call_va!(orig, (&r as *const Ring as usize) -> usize);
    let rn = exe::call_va!(new, (&r as *const Ring as usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x142085490` (104 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_push_18(this: usize, item: usize) {
    let count = rd::<u32>(this.wrapping_add(0x10));
    if count == rd::<u32>(this.wrapping_add(0x14)) {
        exe::call_va!(0x1_4208_3450, (this, count.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(this.wrapping_add(0xc)) as usize;
    let dst = rd::<usize>(this).wrapping_add(tail.wrapping_mul(0x18));
    let v0 = rd::<u64>(item);
    wr::<u64>(item, 0);
    wr::<u64>(dst, v0);
    let v1 = rd::<u64>(item.wrapping_add(0x8));
    wr::<u64>(item.wrapping_add(0x8), 0);
    wr::<u64>(dst.wrapping_add(0x8), v1);
    let v2 = rd::<u64>(item.wrapping_add(0x10));
    wr::<u64>(dst.wrapping_add(0x10), v2);
    let next_tail = rd::<u32>(this.wrapping_add(0xc)).wrapping_add(1);
    wr::<u32>(this.wrapping_add(0xc), next_tail);
    wr::<u32>(this.wrapping_add(0x10), rd::<u32>(this.wrapping_add(0x10)).wrapping_add(1));
    if rd::<u32>(this.wrapping_add(0xc)) == rd::<u32>(this.wrapping_add(0x14)) {
        wr::<u32>(this.wrapping_add(0xc), 0);
    }
}

pub unsafe fn diff_d3d_tex_heap_ring_push_18(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0u64; 6];
    let mut buf_n = [0u64; 6];
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let mut r_o = Ring { base: buf_o.as_mut_ptr() as usize, head: 0, tail: 1, count: 1, cap: 2 };
    let mut r_n = Ring { base: buf_n.as_mut_ptr() as usize, head: 0, tail: 1, count: 1, cap: 2 };
    let mut item_o = [10u64, 20u64, 30u64];
    let mut item_n = item_o;
    exe::call_va!(orig, (&mut r_o as *mut Ring as usize, item_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (&mut r_n as *mut Ring as usize, item_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(buf_o, buf_n, "buf");
    crate::diff_eq!(item_o, item_n, "item");
    crate::diff_eq!(r_o.tail, r_n.tail, "tail");
    crate::diff_eq!(r_o.count, r_n.count, "count");
    Ok(1)
}

/// Original: `0x142085500` (109 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_pop_front_18(this: usize) {
    if rd::<u32>(this.wrapping_add(0x10)) != 0 {
        let head = rd::<u32>(this.wrapping_add(0x8)) as usize;
        let elem = rd::<usize>(this).wrapping_add(head.wrapping_mul(0x18));
        let p1 = rd::<usize>(elem.wrapping_add(0x8));
        if p1 != 0 {
            let vtbl = rd::<usize>(p1);
            let release = rd::<usize>(vtbl.wrapping_add(0x10));
            exe::call_va!(release, (p1) -> ());
        }
        wr::<usize>(elem.wrapping_add(0x8), 0);
        let p0 = rd::<usize>(elem);
        if p0 != 0 {
            let vtbl = rd::<usize>(p0);
            let release = rd::<usize>(vtbl.wrapping_add(0x10));
            exe::call_va!(release, (p0) -> ());
        }
        wr::<usize>(elem, 0);
        let next_head = rd::<u32>(this.wrapping_add(0x8)).wrapping_add(1);
        wr::<u32>(this.wrapping_add(0x8), next_head);
        if next_head == rd::<u32>(this.wrapping_add(0x14)) {
            wr::<u32>(this.wrapping_add(0x8), 0);
        }
        wr::<u32>(this.wrapping_add(0x10), rd::<u32>(this.wrapping_add(0x10)).wrapping_sub(1));
    }
}

pub unsafe fn diff_d3d_tex_heap_ring_pop_front_18(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0u64; 6];
    let mut buf_n = [0u64; 6];
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    let mut r_o = Ring { base: buf_o.as_mut_ptr() as usize, head: 1, tail: 0, count: 1, cap: 2 };
    let mut r_n = Ring { base: buf_n.as_mut_ptr() as usize, head: 1, tail: 0, count: 1, cap: 2 };
    exe::call_va!(orig, (&mut r_o as *mut Ring as usize) -> ());
    exe::call_va!(new, (&mut r_n as *mut Ring as usize) -> ());
    crate::diff_eq!(r_o.head, r_n.head, "head");
    crate::diff_eq!(r_o.count, r_n.count, "count");
    Ok(1)
}

/// Original: `0x1420856a0` (73 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swiss_begin(this: usize, out: usize) -> usize {
    let ctrl = rd::<usize>(this);
    let slot = rd::<usize>(this.wrapping_add(0x8));
    wr::<usize>(out, ctrl);
    wr::<usize>(out.wrapping_add(0x8), slot);
    if rd::<i8>(ctrl) < -1 {
        let (nc, ns) = advance_swiss_iter(ctrl, slot);
        wr::<usize>(out, nc);
        wr::<usize>(out.wrapping_add(0x8), ns);
    }
    out
}

pub unsafe fn diff_d3d_tex_heap_swiss_begin(orig: usize, new: usize) -> Result<u32, String> {
    let ctrl_bytes: [u8; 32] = [0x80, 0x80, 0x03, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let tbl = [ctrl_bytes.as_ptr() as usize, 0x5000usize];
    let mut out_o = [0usize; 2];
    let mut out_n = [0usize; 2];
    let _ = exe::call_va!(orig, (tbl.as_ptr() as usize, out_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (tbl.as_ptr() as usize, out_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142085720` (116 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swiss_erase_iter(this: usize, out: usize, iter: usize) -> usize {
    let c0 = rd::<usize>(iter);
    let s0 = rd::<usize>(iter.wrapping_add(0x8));
    let ctrl = c0.wrapping_add(1);
    let slot = s0.wrapping_add(0x40);
    wr::<usize>(out, ctrl);
    wr::<usize>(out.wrapping_add(0x8), slot);
    if rd::<i8>(ctrl) < -1 {
        let (nc, ns) = advance_swiss_iter(ctrl, slot);
        wr::<usize>(out, nc);
        wr::<usize>(out.wrapping_add(0x8), ns);
    }
    let tmp = [c0, s0];
    exe::call_va!(0x1_4208_4c50, (this, tmp.as_ptr() as usize) -> ());
    out
}

pub unsafe fn diff_d3d_tex_heap_swiss_erase_iter(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_erase(this: usize, it: usize) {
        wr::<usize>(this, rd::<usize>(it));
        wr::<usize>(this.wrapping_add(0x8), rd::<usize>(it.wrapping_add(0x8)));
    }
    let _m = crate::difftest::Mock::new(0x1_4208_4c50, fake_erase as *const ())?;
    let ctrl_bytes: [u8; 32] = [0x01, 0x80, 0xFE, 0x05, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let iter = [ctrl_bytes.as_ptr() as usize, 0x10000usize];
    let mut this_o = [0usize; 2];
    let mut this_n = [0usize; 2];
    let mut out_o = [0usize; 2];
    let mut out_n = [0usize; 2];
    let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize, out_o.as_mut_ptr() as usize, iter.as_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize, out_n.as_mut_ptr() as usize, iter.as_ptr() as usize) -> usize);
    crate::diff_eq!(ro == out_o.as_ptr() as usize, rn == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(this_o, this_n, "this");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142085910` (18 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swap_pair_to_out(this: usize, out: usize) -> usize {
    let a = rd::<u64>(this.wrapping_add(0x8));
    wr::<u64>(out, a);
    let b = rd::<u64>(this);
    wr::<u64>(out.wrapping_add(0x8), b);
    out
}

pub unsafe fn diff_d3d_tex_heap_swap_pair_to_out(orig: usize, new: usize) -> Result<u32, String> {
    let inp = [0x1111_2222u64, 0x3333_4444u64];
    let mut out_o = [0u64; 2];
    let mut out_n = [0u64; 2];
    let _ = exe::call_va!(orig, (inp.as_ptr() as usize, out_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (inp.as_ptr() as usize, out_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142085930` (9 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_iter_inc_18(this: usize) {
    wr::<u64>(this.wrapping_add(0x8), rd::<u64>(this.wrapping_add(0x8)).wrapping_add(1));
    wr::<u64>(this, rd::<u64>(this).wrapping_add(0x18));
}

pub unsafe fn diff_d3d_tex_heap_iter_inc_18(orig: usize, new: usize) -> Result<u32, String> {
    let mut it_o = [0x1000u64, 7u64];
    let mut it_n = it_o;
    exe::call_va!(orig, (it_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (it_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(it_o, it_n, "iter");
    Ok(1)
}

/// Original: `0x142085980` (34 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_ring_iter_deref_60(this: usize) -> usize {
    let ring = rd::<usize>(this);
    let idx = rd::<u32>(this.wrapping_add(0x8)) as u64;
    let head = rd::<u32>(ring.wrapping_add(0x8)) as u64;
    let cap = rd::<u32>(ring.wrapping_add(0x14)) as u64;
    let slot = (head.wrapping_add(idx)) % cap;
    let off = (slot as usize).wrapping_mul(3) << 5;
    rd::<usize>(ring).wrapping_add(off)
}

pub unsafe fn diff_d3d_tex_heap_ring_iter_deref_60(orig: usize, new: usize) -> Result<u32, String> {
    #[repr(C)]
    struct Ring {
        base: usize,
        head: u32,
        tail: u32,
        count: u32,
        cap: u32,
    }
    #[repr(C)]
    struct Iter {
        ring: usize,
        idx: u32,
        pad: u32,
    }
    let r = Ring { base: 0x20000, head: 2, tail: 0, count: 4, cap: 5 };
    let it = Iter { ring: &r as *const Ring as usize, idx: 4, pad: 0 };
    let ro = exe::call_va!(orig, (&it as *const Iter as usize) -> usize);
    let rn = exe::call_va!(new, (&it as *const Iter as usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x142085a20` (74 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_swiss_iter_pre_inc(this: usize) -> usize {
    let delta_addr = exe::ptr::<u64>(0x1_4342_7980) as usize;
    let dc = rd::<usize>(delta_addr);
    let ds = rd::<usize>(delta_addr.wrapping_add(0x8));
    let ctrl = rd::<usize>(this).wrapping_add(dc);
    let slot = rd::<usize>(this.wrapping_add(0x8)).wrapping_add(ds);
    wr::<usize>(this, ctrl);
    wr::<usize>(this.wrapping_add(0x8), slot);
    if rd::<i8>(ctrl) < -1 {
        let (nc, ns) = advance_swiss_iter(ctrl, slot);
        wr::<usize>(this, nc);
        wr::<usize>(this.wrapping_add(0x8), ns);
    }
    this
}

pub unsafe fn diff_d3d_tex_heap_swiss_iter_pre_inc(orig: usize, new: usize) -> Result<u32, String> {
    let ctrl_bytes: [u8; 32] = [0x01, 0x80, 0xFE, 0x04, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut it_o = [ctrl_bytes.as_ptr() as usize, 0x8000usize];
    let mut it_n = it_o;
    let _ = exe::call_va!(orig, (it_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (it_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(it_o, it_n, "iter");
    Ok(1)
}

/// Original: `0x142085b40` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_iter_add_assign_60(this: usize, out: usize) -> usize {
    let v = rd::<usize>(this).wrapping_add(0x60);
    wr::<usize>(this, v);
    wr::<usize>(out, v);
    out
}

pub unsafe fn diff_d3d_tex_heap_iter_add_assign_60(orig: usize, new: usize) -> Result<u32, String> {
    let mut this_o = 0x1000usize;
    let mut out_o = 0usize;
    let mut this_n = 0x1000usize;
    let mut out_n = 0usize;
    let _ = exe::call_va!(orig, (&mut this_o as *mut usize as usize, &mut out_o as *mut usize as usize) -> usize);
    let _ = exe::call_va!(new, (&mut this_n as *mut usize as usize, &mut out_n as *mut usize as usize) -> usize);
    crate::diff_eq!(this_o, this_n, "this");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142085c10` (31 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_clamp_range_max(this: usize, max_val: u64) {
    let min_val = rd::<u64>(this.wrapping_add(0x8));
    let cur = rd::<u64>(this);
    wr::<u64>(this.wrapping_add(0x10), max_val);
    if cur < min_val {
        wr::<u64>(this, min_val);
    } else {
        let clamped = if cur > max_val { max_val } else { cur };
        wr::<u64>(this, clamped);
    }
}

pub unsafe fn diff_d3d_tex_heap_clamp_range_max(orig: usize, new: usize) -> Result<u32, String> {
    for &cur in &[5u64, 15u64, 25u64] {
        let mut s_o = [cur, 10u64, 0u64];
        let mut s_n = s_o;
        exe::call_va!(orig, (s_o.as_mut_ptr() as usize, 20u64) -> ());
        exe::call_va!(new, (s_n.as_mut_ptr() as usize, 20u64) -> ());
        crate::diff_eq!(s_o, s_n, "clamped");
    }
    Ok(3)
}

/// Original: `0x1420860e0` (11 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_init_ptr_flag(this: usize, ptr_val: u64, flag: u8) -> usize {
    wr::<u64>(this, ptr_val);
    wr::<u8>(this.wrapping_add(0x8), flag);
    this
}

pub unsafe fn diff_d3d_tex_heap_init_ptr_flag(orig: usize, new: usize) -> Result<u32, String> {
    let mut b_o = [0xEEu8; 16];
    let mut b_n = [0xEEu8; 16];
    let _ = exe::call_va!(orig, (b_o.as_mut_ptr() as usize, 0x1234_5678u64, 3u8) -> usize);
    let _ = exe::call_va!(new, (b_n.as_mut_ptr() as usize, 0x1234_5678u64, 3u8) -> usize);
    crate::diff_eq!(b_o, b_n, "buf");
    Ok(1)
}

/// Original: `0x142086100` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_key_matches(this: usize, other: usize) -> u8 {
    if rd::<u64>(other.wrapping_add(0x8)) != rd::<u64>(this) {
        return 0;
    }
    let f = rd::<u8>(this.wrapping_add(0x8)) as u32;
    if rd::<u32>(other.wrapping_add(0x10)) != f {
        return 0;
    }
    1
}

pub unsafe fn diff_d3d_tex_heap_key_matches(orig: usize, new: usize) -> Result<u32, String> {
    #[repr(C)]
    struct Key {
        id: u64,
        flag: u8,
    }
    #[repr(C)]
    struct Entry {
        pad: u64,
        id: u64,
        flag: u32,
    }
    let k = Key { id: 42, flag: 7 };
    let e1 = Entry { pad: 0, id: 42, flag: 7 };
    let e2 = Entry { pad: 0, id: 42, flag: 8 };
    for e in [&e1, &e2] {
        let ro = exe::call_va!(orig, (&k as *const Key as usize, e as *const Entry as usize) -> u8);
        let rn = exe::call_va!(new, (&k as *const Key as usize, e as *const Entry as usize) -> u8);
        crate::diff_eq!(ro, rn, "match");
    }
    Ok(2)
}

/// Original: `0x142086ea0` (5 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_set_r9_field_8(_a: usize, _b: usize, val: u64, dst: usize) {
    wr::<u64>(dst.wrapping_add(0x8), val);
}

pub unsafe fn diff_d3d_tex_heap_set_r9_field_8(orig: usize, new: usize) -> Result<u32, String> {
    let mut o = [0u64; 2];
    let mut n = [0u64; 2];
    exe::call_va!(orig, (0usize, 0usize, 0xABCDu64, o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (0usize, 0usize, 0xABCDu64, n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(o, n, "dst");
    Ok(1)
}

/// Original: `0x1420878a0` (65 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_release_tile_entry(this: usize) {
    let alloc_handle = rd::<usize>(this.wrapping_add(0x18));
    if alloc_handle != 0 {
        let mgr = rd::<usize>(this.wrapping_add(0x30));
        let page_count = rd::<u16>(this.wrapping_add(0x28)) as u32;
        let vtbl = rd::<usize>(mgr);
        let free_fn = rd::<usize>(vtbl.wrapping_add(0x10));
        let arg2 = rd::<usize>(this.wrapping_add(0x20));
        exe::call_va!(free_fn, (mgr, alloc_handle, arg2, page_count) -> ());
    }
    let com_ptr = rd::<usize>(this);
    if com_ptr != 0 {
        let vtbl = rd::<usize>(com_ptr);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (com_ptr) -> ());
    }
    wr::<usize>(this, 0);
}

pub unsafe fn diff_d3d_tex_heap_release_tile_entry(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0u64; 7];
    let mut buf_n = [0u64; 7];
    exe::call_va!(orig, (buf_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (buf_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(buf_o, buf_n, "buf");
    Ok(1)
}

/// Original: `0x142088160` (90 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_move_assign_entry_1e(this: usize, src: usize) -> usize {
    let old = rd::<usize>(this);
    if old != 0 {
        let vtbl = rd::<usize>(old);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (old) -> ());
    }
    wr::<usize>(this, 0);
    let p0 = rd::<u64>(src);
    wr::<u64>(src, 0);
    wr::<u64>(this, p0);
    wr::<u64>(this.wrapping_add(0x8), rd::<u64>(src.wrapping_add(0x8)));
    wr::<u64>(this.wrapping_add(0x10), rd::<u64>(src.wrapping_add(0x10)));
    wr::<u32>(this.wrapping_add(0x18), rd::<u32>(src.wrapping_add(0x18)));
    wr::<u16>(this.wrapping_add(0x1c), rd::<u16>(src.wrapping_add(0x1c)));
    this
}

pub unsafe fn diff_d3d_tex_heap_move_assign_entry_1e(orig: usize, new: usize) -> Result<u32, String> {
    let mut dst_o = [0u8; 32];
    let mut src_o = [0u8; 32];
    for i in 0..30 {
        src_o[i] = (i as u8).wrapping_add(1);
    }
    let mut dst_n = dst_o;
    let mut src_n = src_o;
    let _ = exe::call_va!(orig, (dst_o.as_mut_ptr() as usize, src_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (dst_n.as_mut_ptr() as usize, src_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(dst_o, dst_n, "dst");
    crate::diff_eq!(src_o, src_n, "src");
    Ok(1)
}

/// Original: `0x142088210` (56 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_notify_completion(this: usize) {
    let sync = rd::<usize>(this);
    let mtx = sync.wrapping_add(0x8);
    exe::call_iat!(0x1_4264_55a8, (mtx) -> ());
    wr::<u32>(sync.wrapping_add(0x10), rd::<u32>(sync.wrapping_add(0x10)).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (mtx) -> ());
    exe::call_iat!(0x1_4264_5590, (sync) -> ());
}

pub unsafe fn diff_d3d_tex_heap_notify_completion(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x10));
    }
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x100));
    }
    unsafe extern "win64" fn fake_wake(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x1000));
    }
    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5590, fake_wake as *const ())?;
    let mut sync_o = [0u32; 8];
    let mut sync_n = [0u32; 8];
    sync_o[4] = 12;
    sync_n[4] = 12;
    let this_o = [sync_o.as_mut_ptr() as usize];
    let this_n = [sync_n.as_mut_ptr() as usize];
    exe::call_va!(orig, (this_o.as_ptr() as usize) -> ());
    exe::call_va!(new, (this_n.as_ptr() as usize) -> ());
    crate::diff_eq!(sync_o, sync_n, "sync");
    Ok(1)
}

/// Original: `0x1420888b0` (86 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_unlock_and_wake_global(this: usize) {
    let inner = rd::<usize>(this);
    exe::call_iat!(0x1_4264_5598, (inner.wrapping_add(0x8)) -> ());
    let glob = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    let mtx = glob.wrapping_add(0x250);
    exe::call_iat!(0x1_4264_55a8, (mtx) -> ());
    wr::<u32>(glob.wrapping_add(0x258), rd::<u32>(glob.wrapping_add(0x258)).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (mtx) -> ());
    exe::call_iat!(0x1_4264_5590, (glob.wrapping_add(0x248)) -> ());
}

pub unsafe fn diff_d3d_tex_heap_unlock_and_wake_global(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x10));
    }
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x100));
    }
    unsafe extern "win64" fn fake_wake(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x1000));
    }
    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5590, fake_wake as *const ())?;
    let g_addr = exe::ptr::<u8>(0x1_43a0_8298) as *mut u8;
    let snap = crate::difftest::Snapshot::take(g_addr, 8);
    let mut inner_o = [0u32; 8];
    let mut inner_n = [0u32; 8];
    let mut glob_o = [0u8; 0x270];
    let mut glob_n = [0u8; 0x270];
    let this_o = [inner_o.as_mut_ptr() as usize];
    let this_n = [inner_n.as_mut_ptr() as usize];
    wr::<usize>(g_addr as usize, glob_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (this_o.as_ptr() as usize) -> ());
    wr::<usize>(g_addr as usize, glob_n.as_mut_ptr() as usize);
    exe::call_va!(new, (this_n.as_ptr() as usize) -> ());
    snap.restore();
    crate::diff_eq!(inner_o, inner_n, "inner");
    crate::diff_eq!(glob_o, glob_n, "glob");
    Ok(1)
}

/// Original: `0x142088910` (31 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_lock_guard_init(this: usize, target: usize) -> usize {
    wr::<usize>(this, target);
    exe::call_iat!(0x1_4264_55a8, (target.wrapping_add(0x8)) -> ());
    this
}

pub unsafe fn diff_d3d_tex_heap_lock_guard_init(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(1));
    }
    let _m = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let mut t_o = [0u32; 8];
    let mut t_n = [0u32; 8];
    let mut g_o = [0usize; 2];
    let mut g_n = [0usize; 2];
    let ro = exe::call_va!(orig, (g_o.as_mut_ptr() as usize, t_o.as_mut_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (g_n.as_mut_ptr() as usize, t_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(ro == g_o.as_ptr() as usize, rn == g_n.as_ptr() as usize, "ret");
    crate::diff_eq!(g_o[0] == t_o.as_ptr() as usize, g_n[0] == t_n.as_ptr() as usize, "guard");
    crate::diff_eq!(t_o, t_n, "target");
    Ok(1)
}

/// Original: `0x142089130` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_pack_desc_14(_this: usize, out: usize, ptr_val: u64, desc: usize) -> usize {
    wr::<u32>(out.wrapping_add(0x8), rd::<u32>(desc.wrapping_add(0xc)));
    wr::<u32>(out.wrapping_add(0xc), rd::<u32>(desc));
    wr::<u32>(out.wrapping_add(0x10), rd::<u32>(desc.wrapping_add(0x4)));
    wr::<u64>(out, ptr_val);
    out
}

pub unsafe fn diff_d3d_tex_heap_pack_desc_14(orig: usize, new: usize) -> Result<u32, String> {
    let desc = [10u32, 20u32, 30u32, 40u32];
    let mut out_o = [0u8; 24];
    let mut out_n = [0u8; 24];
    let _ = exe::call_va!(orig, (0usize, out_o.as_mut_ptr() as usize, 0x9988_7766u64, desc.as_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (0usize, out_n.as_mut_ptr() as usize, 0x9988_7766u64, desc.as_ptr() as usize) -> usize);
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142089160` (62 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_flush_and_signal(this: usize, _unused: usize, fence_val: u64) -> u32 {
    exe::call_va!(0x1_4208_9c40, (this, fence_val) -> ());
    let queue = rd::<usize>(this.wrapping_add(0x48));
    let fence = rd::<usize>(this.wrapping_add(0xd0));
    wr::<u64>(this.wrapping_add(0xd8), fence_val);
    let vtbl = rd::<usize>(queue);
    let signal_fn = rd::<usize>(vtbl.wrapping_add(0x70));
    exe::call_va!(signal_fn, (queue, fence, fence_val) -> u32)
}

pub unsafe fn diff_d3d_tex_heap_flush_and_signal(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_submit(this: usize, fv: u64) {
        wr::<u64>(this.wrapping_add(0x50), fv);
    }
    unsafe extern "win64" fn fake_signal(queue: usize, fence: usize, fv: u64) -> u32 {
        wr::<usize>(queue.wrapping_add(0x8), fence);
        wr::<u64>(queue.wrapping_add(0x10), fv);
        0x1122_3344
    }
    let _m = crate::difftest::Mock::new(0x1_4208_9c40, fake_submit as *const ())?;
    let mut vtbl = [0usize; 16];
    vtbl[14] = fake_signal as *const () as usize;
    let mut q_o = [vtbl.as_ptr() as usize, 0, 0];
    let mut q_n = [vtbl.as_ptr() as usize, 0, 0];
    let mut this_o = [0u8; 0xe8];
    let mut this_n = [0u8; 0xe8];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x48, q_o.as_mut_ptr() as usize);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0xd0, 0x7777);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x48, q_n.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0xd0, 0x7777);
    let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize, 0usize, 99u64) -> u32);
    let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize, 0usize, 99u64) -> u32);
    crate::diff_eq!(ro, rn, "ret");
    crate::diff_eq!(q_o[1..], q_n[1..], "queue");
    crate::diff_eq!(rd::<u64>(this_o.as_ptr() as usize + 0x50), rd::<u64>(this_n.as_ptr() as usize + 0x50), "submit_fv");
    crate::diff_eq!(rd::<u64>(this_o.as_ptr() as usize + 0xd8), rd::<u64>(this_n.as_ptr() as usize + 0xd8), "d8");
    Ok(1)
}

/// Original: `0x1420891b0` (92 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_flush_signal_and_wait(this: usize, wait_pair: usize, fence_val: u64) -> u32 {
    exe::call_va!(0x1_4208_9c40, (this, fence_val) -> ());
    let queue = rd::<usize>(this.wrapping_add(0x48));
    let fence = rd::<usize>(this.wrapping_add(0xd0));
    wr::<u64>(this.wrapping_add(0xd8), fence_val);
    let vtbl = rd::<usize>(queue);
    let signal_fn = rd::<usize>(vtbl.wrapping_add(0x70));
    exe::call_va!(signal_fn, (queue, fence, fence_val) -> u32);
    let queue2 = rd::<usize>(this.wrapping_add(0x48));
    let wait_val = rd::<u64>(wait_pair.wrapping_add(0x8));
    let wait_fence = rd::<usize>(wait_pair);
    let vtbl2 = rd::<usize>(queue2);
    let wait_fn = rd::<usize>(vtbl2.wrapping_add(0x78));
    exe::call_va!(wait_fn, (queue2, wait_fence, wait_val) -> u32)
}

pub unsafe fn diff_d3d_tex_heap_flush_signal_and_wait(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_submit(this: usize, fv: u64) {
        wr::<u64>(this.wrapping_add(0x50), fv);
    }
    unsafe extern "win64" fn fake_signal(queue: usize, fence: usize, fv: u64) -> u32 {
        wr::<usize>(queue.wrapping_add(0x8), fence);
        wr::<u64>(queue.wrapping_add(0x10), fv);
        0
    }
    unsafe extern "win64" fn fake_wait(queue: usize, fence: usize, fv: u64) -> u32 {
        wr::<usize>(queue.wrapping_add(0x18), fence);
        wr::<u64>(queue.wrapping_add(0x20), fv);
        0x5566_7788
    }
    let _m = crate::difftest::Mock::new(0x1_4208_9c40, fake_submit as *const ())?;
    let mut vtbl = [0usize; 16];
    vtbl[14] = fake_signal as *const () as usize;
    vtbl[15] = fake_wait as *const () as usize;
    let mut q_o = [vtbl.as_ptr() as usize, 0, 0, 0, 0];
    let mut q_n = [vtbl.as_ptr() as usize, 0, 0, 0, 0];
    let mut this_o = [0u8; 0xe8];
    let mut this_n = [0u8; 0xe8];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x48, q_o.as_mut_ptr() as usize);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0xd0, 0x8888);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x48, q_n.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0xd0, 0x8888);
    let wait_pair = [0x9999usize, 123usize];
    let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize, wait_pair.as_ptr() as usize, 77u64) -> u32);
    let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize, wait_pair.as_ptr() as usize, 77u64) -> u32);
    crate::diff_eq!(ro, rn, "ret");
    crate::diff_eq!(q_o[1..], q_n[1..], "queue");
    crate::diff_eq!(rd::<u64>(this_o.as_ptr() as usize + 0xd8), rd::<u64>(this_n.as_ptr() as usize + 0xd8), "d8");
    Ok(1)
}

/// Original: `0x142089220` (83 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_mark_slot_state_3(this: usize, handle_ptr: usize, val: u64) {
    let table = rd::<usize>(this.wrapping_add(0xe0));
    let idx = rd::<u16>(handle_ptr) as usize;
    exe::call_iat!(0x1_4264_55a8, (table) -> ());
    let arr = rd::<usize>(table.wrapping_add(0x8));
    let elem = arr.wrapping_add(idx.wrapping_mul(0x18));
    wr::<u32>(elem, 3);
    wr::<u64>(elem.wrapping_add(0x8), val);
    exe::call_iat!(0x1_4264_5598, (table) -> ());
}

pub unsafe fn diff_d3d_tex_heap_mark_slot_state_3(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x10));
    }
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x100));
    }
    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    let mut arr_o = [0u8; 0x60];
    let mut arr_n = [0u8; 0x60];
    let mut tbl_o = [0usize, arr_o.as_mut_ptr() as usize];
    let mut tbl_n = [0usize, arr_n.as_mut_ptr() as usize];
    let mut this_o = [0u8; 0xf0];
    let mut this_n = [0u8; 0xf0];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0xe0, tbl_o.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0xe0, tbl_n.as_mut_ptr() as usize);
    let h = 2u16;
    exe::call_va!(orig, (this_o.as_mut_ptr() as usize, &h as *const u16 as usize, 0xABCD_1234u64) -> ());
    exe::call_va!(new, (this_n.as_mut_ptr() as usize, &h as *const u16 as usize, 0xABCD_1234u64) -> ());
    crate::diff_eq!(tbl_o[0], tbl_n[0], "lock_state");
    crate::diff_eq!(arr_o, arr_n, "arr");
    Ok(1)
}

/// Original: `0x142089280` (15 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_tex_heap_lookup_slot_by_handle(this: usize, handle_ptr: usize) -> usize {
    let idx = rd::<u16>(handle_ptr) as usize;
    let table = rd::<usize>(this.wrapping_add(0xe0));
    exe::call_va!(0x1_4208_b190, (table, idx) -> usize)
}

pub unsafe fn diff_d3d_tex_heap_lookup_slot_by_handle(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_lookup(table: usize, idx: usize) -> usize {
        table.wrapping_add(idx.wrapping_mul(0x18))
    }
    let _m = crate::difftest::Mock::new(0x1_4208_b190, fake_lookup as *const ())?;
    let mut this_buf = [0u8; 0xf0];
    wr::<usize>(this_buf.as_mut_ptr() as usize + 0xe0, 0x50000usize);
    let h = 7u16;
    let ro = exe::call_va!(orig, (this_buf.as_ptr() as usize, &h as *const u16 as usize) -> usize);
    let rn = exe::call_va!(new, (this_buf.as_ptr() as usize, &h as *const u16 as usize) -> usize);
    crate::diff_eq!(ro, rn, "ret");
    Ok(1)
}

/// Original: `0x142089ba0` (59 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_flush_and_signal_direct(this: usize, fence_val: u64) -> u32 {
    exe::call_va!(0x1_4208_9c40, (this, fence_val) -> ());
    let queue = rd::<usize>(this.wrapping_add(0x48));
    let fence = rd::<usize>(this.wrapping_add(0xd0));
    wr::<u64>(this.wrapping_add(0xd8), fence_val);
    let vtbl = rd::<usize>(queue);
    let signal_fn = rd::<usize>(vtbl.wrapping_add(0x70));
    exe::call_va!(signal_fn, (queue, fence, fence_val) -> u32)
}

pub unsafe fn diff_d3d_tex_heap_flush_and_signal_direct(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_submit(this: usize, fv: u64) {
        wr::<u64>(this.wrapping_add(0x50), fv);
    }
    unsafe extern "win64" fn fake_signal(queue: usize, fence: usize, fv: u64) -> u32 {
        wr::<usize>(queue.wrapping_add(0x8), fence);
        wr::<u64>(queue.wrapping_add(0x10), fv);
        0x8765_4321
    }
    let _m = crate::difftest::Mock::new(0x1_4208_9c40, fake_submit as *const ())?;
    let mut vtbl = [0usize; 16];
    vtbl[14] = fake_signal as *const () as usize;
    let mut q_o = [vtbl.as_ptr() as usize, 0, 0];
    let mut q_n = [vtbl.as_ptr() as usize, 0, 0];
    let mut this_o = [0u8; 0xe8];
    let mut this_n = [0u8; 0xe8];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0x48, q_o.as_mut_ptr() as usize);
    wr::<usize>(this_o.as_mut_ptr() as usize + 0xd0, 0x3333);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0x48, q_n.as_mut_ptr() as usize);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0xd0, 0x3333);
    let ro = exe::call_va!(orig, (this_o.as_mut_ptr() as usize, 55u64) -> u32);
    let rn = exe::call_va!(new, (this_n.as_mut_ptr() as usize, 55u64) -> u32);
    crate::diff_eq!(ro, rn, "ret");
    crate::diff_eq!(q_o[1..], q_n[1..], "queue");
    crate::diff_eq!(rd::<u64>(this_o.as_ptr() as usize + 0xd8), rd::<u64>(this_n.as_ptr() as usize + 0xd8), "d8");
    Ok(1)
}

/// Original: `0x142089bf0` (59 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_release_pair_void(this: usize) {
    let p1 = rd::<usize>(this.wrapping_add(0x8));
    if p1 != 0 {
        let vtbl = rd::<usize>(p1);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (p1) -> ());
    }
    wr::<usize>(this.wrapping_add(0x8), 0);
    let p0 = rd::<usize>(this);
    if p0 != 0 {
        let vtbl = rd::<usize>(p0);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (p0) -> ());
    }
    wr::<usize>(this, 0);
}

pub unsafe fn diff_d3d_tex_heap_release_pair_void(orig: usize, new: usize) -> Result<u32, String> {
    let mut pair_o = [0usize; 2];
    let mut pair_n = [0usize; 2];
    exe::call_va!(orig, (pair_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (pair_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(pair_o, pair_n, "pair");
    Ok(1)
}

/// Original: `0x142089c40` (200 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_submit_cmd_list(this: usize, fence_val: u64) {
    let cmd_list = rd::<usize>(this.wrapping_add(0x58));
    if cmd_list != 0 {
        let cl_vtbl = rd::<usize>(cmd_list);
        let close_fn = rd::<usize>(cl_vtbl.wrapping_add(0x48));
        exe::call_va!(close_fn, (cmd_list) -> ());
        let cl_copy = rd::<usize>(this.wrapping_add(0x58));
        let lists = [cl_copy];
        let queue = rd::<usize>(this.wrapping_add(0x48));
        let q_vtbl = rd::<usize>(queue);
        let exec_fn = rd::<usize>(q_vtbl.wrapping_add(0x50));
        exe::call_va!(exec_fn, (queue, 1u32, lists.as_ptr() as usize) -> ());
        let ring = this.wrapping_add(0x80);
        let p0 = rd::<u64>(this.wrapping_add(0x50));
        wr::<u64>(this.wrapping_add(0x50), 0);
        let p1 = rd::<u64>(this.wrapping_add(0x58));
        wr::<u64>(this.wrapping_add(0x58), 0);
        let count = rd::<u32>(ring.wrapping_add(0x10));
        if count == rd::<u32>(ring.wrapping_add(0x14)) {
            exe::call_va!(0x1_4208_3450, (ring, count.wrapping_add(1)) -> ());
        }
        let tail = rd::<u32>(ring.wrapping_add(0xc)) as usize;
        let dst = rd::<usize>(ring).wrapping_add(tail.wrapping_mul(0x18));
        wr::<u64>(dst, p0);
        wr::<u64>(dst.wrapping_add(0x8), p1);
        wr::<u64>(dst.wrapping_add(0x10), fence_val);
        let next_tail = rd::<u32>(ring.wrapping_add(0xc)).wrapping_add(1);
        wr::<u32>(ring.wrapping_add(0xc), next_tail);
        if next_tail == rd::<u32>(ring.wrapping_add(0x14)) {
            wr::<u32>(ring.wrapping_add(0xc), 0);
        }
        wr::<u32>(ring.wrapping_add(0x10), rd::<u32>(ring.wrapping_add(0x10)).wrapping_add(1));
    }
}

pub unsafe fn diff_d3d_tex_heap_submit_cmd_list(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_close(cl: usize) {
        wr::<u32>(cl.wrapping_add(0x8), rd::<u32>(cl.wrapping_add(0x8)).wrapping_add(1));
    }
    unsafe extern "win64" fn fake_exec(q: usize, cnt: u32, lists: usize) {
        wr::<u32>(q.wrapping_add(0x8), cnt);
        wr::<u32>(q.wrapping_add(0xc), u32::from(rd::<usize>(lists) != 0));
    }
    let mut cl_vtbl = [0usize; 12];
    cl_vtbl[9] = fake_close as *const () as usize;
    let mut q_vtbl = [0usize; 12];
    q_vtbl[10] = fake_exec as *const () as usize;
    let mut cl_o = [cl_vtbl.as_ptr() as usize, 0];
    let mut cl_n = [cl_vtbl.as_ptr() as usize, 0];
    let mut q_o = [q_vtbl.as_ptr() as usize, 0];
    let mut q_n = [q_vtbl.as_ptr() as usize, 0];
    let mut ring_buf_o = [0u64; 6];
    let mut ring_buf_n = [0u64; 6];
    let mut this_o = [0u8; 0xa0];
    let mut this_n = [0u8; 0xa0];
    let po = this_o.as_mut_ptr() as usize;
    let pn = this_n.as_mut_ptr() as usize;
    wr::<usize>(po + 0x48, q_o.as_mut_ptr() as usize);
    wr::<u64>(po + 0x50, 0x1111);
    wr::<usize>(po + 0x58, cl_o.as_mut_ptr() as usize);
    wr::<usize>(po + 0x80, ring_buf_o.as_mut_ptr() as usize);
    wr::<u32>(po + 0x8c, 1);
    wr::<u32>(po + 0x90, 1);
    wr::<u32>(po + 0x94, 2);
    wr::<usize>(pn + 0x48, q_n.as_mut_ptr() as usize);
    wr::<u64>(pn + 0x50, 0x1111);
    wr::<usize>(pn + 0x58, cl_n.as_mut_ptr() as usize);
    wr::<usize>(pn + 0x80, ring_buf_n.as_mut_ptr() as usize);
    wr::<u32>(pn + 0x8c, 1);
    wr::<u32>(pn + 0x90, 1);
    wr::<u32>(pn + 0x94, 2);
    exe::call_va!(orig, (po, 999u64) -> ());
    exe::call_va!(new, (pn, 999u64) -> ());
    crate::diff_eq!(cl_o[1], cl_n[1], "cl");
    crate::diff_eq!(q_o[1], q_n[1], "q");
    crate::diff_eq!(ring_buf_o[3], ring_buf_n[3], "ring_p0");
    crate::diff_eq!(ring_buf_o[4] == cl_o.as_ptr() as u64, ring_buf_n[4] == cl_n.as_ptr() as u64, "ring_p1");
    crate::diff_eq!(ring_buf_o[5], ring_buf_n[5], "ring_fv");
    crate::diff_eq!(rd::<u64>(po + 0x88), rd::<u64>(pn + 0x88), "ring_ht");
    crate::diff_eq!(rd::<u32>(po + 0x90), rd::<u32>(pn + 0x90), "ring_cnt");
    Ok(1)
}

/// Original: `0x14208a2b0` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_tex_heap_queue_wait_streamer(this: usize, queue: usize) -> u32 {
    let q_vtbl = rd::<usize>(queue);
    let streamer = rd::<usize>(this);
    let fence_val = rd::<u64>(this.wrapping_add(0x8));
    let fence = rd::<usize>(streamer.wrapping_add(0xd0));
    let wait_fn = rd::<usize>(q_vtbl.wrapping_add(0x78));
    exe::call_va!(wait_fn, (queue, fence, fence_val) -> u32)
}

pub unsafe fn diff_d3d_tex_heap_queue_wait_streamer(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_wait(queue: usize, fence: usize, fv: u64) -> u32 {
        wr::<usize>(queue.wrapping_add(0x8), fence);
        wr::<u64>(queue.wrapping_add(0x10), fv);
        0x42
    }
    let mut vtbl = [0usize; 16];
    vtbl[15] = fake_wait as *const () as usize;
    let mut q_o = [vtbl.as_ptr() as usize, 0, 0];
    let mut q_n = [vtbl.as_ptr() as usize, 0, 0];
    let mut streamer = [0u8; 0xe0];
    wr::<usize>(streamer.as_mut_ptr() as usize + 0xd0, 0xABCD);
    let this = [streamer.as_ptr() as usize, 0x1234_5678usize];
    let ro = exe::call_va!(orig, (this.as_ptr() as usize, q_o.as_mut_ptr() as usize) -> u32);
    let rn = exe::call_va!(new, (this.as_ptr() as usize, q_n.as_mut_ptr() as usize) -> u32);
    crate::diff_eq!(ro, rn, "ret");
    crate::diff_eq!(q_o[1..], q_n[1..], "queue");
    Ok(1)
}

/// Original: `0x14208a7a0` (111 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_async_streamer_has_pending_work(this: usize) -> u8 {
    let mtx1 = this.wrapping_add(0x8);
    exe::call_iat!(0x1_4264_55a8, (mtx1) -> ());
    if rd::<u32>(this.wrapping_add(0x20)) != 0 {
        exe::call_iat!(0x1_4264_5598, (mtx1) -> ());
        return 1;
    }
    exe::call_iat!(0x1_4264_5598, (mtx1) -> ());
    let mtx2 = this.wrapping_add(0x28);
    exe::call_iat!(0x1_4264_55a8, (mtx2) -> ());
    if rd::<u8>(this.wrapping_add(0x78)) != 0 || rd::<u64>(this.wrapping_add(0xa8)) != 0 {
        exe::call_iat!(0x1_4264_5598, (mtx2) -> ());
        return 1;
    }
    exe::call_iat!(0x1_4264_5598, (mtx2) -> ());
    0
}

pub unsafe fn diff_d3d_async_streamer_has_pending_work(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x10));
    }
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x100));
    }
    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    for &(c20, c78, ca8) in &[(1u32, 0u8, 0u64), (0, 1, 0), (0, 0, 5), (0, 0, 0)] {
        let mut s_o = [0u8; 0xc0];
        wr::<u32>(s_o.as_mut_ptr() as usize + 0x20, c20);
        wr::<u8>(s_o.as_mut_ptr() as usize + 0x78, c78);
        wr::<u64>(s_o.as_mut_ptr() as usize + 0xa8, ca8);
        let mut s_n = s_o;
        let ro = exe::call_va!(orig, (s_o.as_mut_ptr() as usize) -> u8);
        let rn = exe::call_va!(new, (s_n.as_mut_ptr() as usize) -> u8);
        crate::diff_eq!(ro, rn, "ret");
        crate::diff_eq!(s_o, s_n, "state");
    }
    Ok(4)
}

/// Original: `0x14208a820` (62 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_async_streamer_close_fence_event(this: usize) -> usize {
    let handle = rd::<usize>(this.wrapping_add(0x10));
    if handle != 0 {
        exe::call_iat!(0x1_4264_55b0, (handle) -> i32);
        wr::<usize>(this.wrapping_add(0x10), 0);
    }
    let fence = rd::<usize>(this);
    if fence != 0 {
        let vtbl = rd::<usize>(fence);
        let release = rd::<usize>(vtbl.wrapping_add(0x10));
        exe::call_va!(release, (fence) -> ());
    }
    wr::<usize>(this, 0);
    this
}

pub unsafe fn diff_d3d_async_streamer_close_fence_event(orig: usize, new: usize) -> Result<u32, String> {
    let mut s_o = [0usize; 3];
    let mut s_n = [0usize; 3];
    let _ = exe::call_va!(orig, (s_o.as_mut_ptr() as usize) -> usize);
    let _ = exe::call_va!(new, (s_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(s_o, s_n, "state");
    Ok(1)
}

/// Original: `0x14208aa30` (52 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_async_streamer_scalar_deleting_dtor(this: usize, flags: u32) -> usize {
    exe::call_va!(0x1_4208_a870, (this) -> ());
    if (flags & 1) != 0 {
        exe::call_va!(0x1_4183_4d30, (this, 0xf0usize) -> ());
    }
    this
}

pub unsafe fn diff_d3d_async_streamer_scalar_deleting_dtor(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_dtor(this: usize) {
        wr::<u32>(this, rd::<u32>(this).wrapping_add(1));
    }
    unsafe extern "win64" fn fake_free(this: usize, sz: usize) {
        wr::<u32>(this.wrapping_add(0x4), sz as u32);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4208_a870, fake_dtor as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4183_4d30, fake_free as *const ())?;
    for &flags in &[0u32, 1u32] {
        let mut s_o = [0u32; 4];
        let mut s_n = [0u32; 4];
        let ro = exe::call_va!(orig, (s_o.as_mut_ptr() as usize, flags) -> usize);
        let rn = exe::call_va!(new, (s_n.as_mut_ptr() as usize, flags) -> usize);
        crate::diff_eq!(ro == s_o.as_ptr() as usize, rn == s_n.as_ptr() as usize, "ret");
        crate::diff_eq!(s_o, s_n, "state");
    }
    Ok(2)
}

/// Original: `0x1420848e0` (216 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420848e0(rcx: usize) {
    let tag = rd::<i8>(rcx + 0x50) as isize;
    let rva = rd::<u32>(exe::ptr::<u32>((0x1_4208_4998isize + tag * 4) as u64) as usize);
    match rva {
        0x0208_4901 | 0x0208_491c => {
            let obj = rd::<usize>(rcx);
            if obj != 0 {
                let vtbl = rd::<usize>(obj);
                let release = rd::<usize>(vtbl + 0x10);
                exe::call_va!(release, (obj) -> ());
            }
            wr::<usize>(rcx, 0);
        }
        0x0208_4937 => {
            let rdx = rd::<usize>(rcx + 0x18);
            if rdx != 0 {
                let alloc = rd::<usize>(rcx + 0x30);
                let r9d = rd::<u16>(rcx + 0x28) as u32;
                let vtbl = rd::<usize>(alloc);
                let r8 = rd::<usize>(rcx + 0x20);
                let fn_ptr = rd::<usize>(vtbl + 0x10);
                exe::call_va!(fn_ptr, (alloc, rdx, r8, r9d) -> ());
            }
            let obj = rd::<usize>(rcx);
            if obj != 0 {
                let vtbl = rd::<usize>(obj);
                let release = rd::<usize>(vtbl + 0x10);
                exe::call_va!(release, (obj) -> ());
            }
            wr::<usize>(rcx, 0);
        }
        0x0208_496f => {
            let target = rd::<usize>(rcx + 0x38);
            if target != 0 {
                let vtbl = rd::<usize>(target);
                let fn_ptr = rd::<usize>(vtbl + 0x20);
                let is_heap = (target != rcx) as u8;
                exe::call_va!(fn_ptr, (target, is_heap) -> ());
                wr::<usize>(rcx + 0x38, 0);
            }
        }
        _ => {}
    }
}

pub unsafe fn diff_d3d_unknown_1420848e0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_rel(obj: usize) {
        wr::<u32>(obj.wrapping_add(0x8), rd::<u32>(obj.wrapping_add(0x8)).wrapping_add(1));
    }
    unsafe extern "win64" fn fake_destroy(obj: usize, is_heap: u8) {
        wr::<u32>(obj.wrapping_add(0x8), (is_heap as u32) + 10);
    }
    let mut vtbl = [0usize; 6];
    vtbl[2] = fake_rel as *const () as usize;
    vtbl[4] = fake_destroy as *const () as usize;
    for &tag in &[-1i8, 0, 2, 7] {
        let mut obj_o = [vtbl.as_ptr() as usize, 0];
        let mut obj_n = [vtbl.as_ptr() as usize, 0];
        let mut buf_o = [0u8; 0x60];
        let mut buf_n = [0u8; 0x60];
        wr::<usize>(buf_o.as_mut_ptr() as usize, obj_o.as_mut_ptr() as usize);
        wr::<usize>(buf_o.as_mut_ptr() as usize + 0x38, obj_o.as_mut_ptr() as usize);
        wr::<i8>(buf_o.as_mut_ptr() as usize + 0x50, tag);
        wr::<usize>(buf_n.as_mut_ptr() as usize, obj_n.as_mut_ptr() as usize);
        wr::<usize>(buf_n.as_mut_ptr() as usize + 0x38, obj_n.as_mut_ptr() as usize);
        wr::<i8>(buf_n.as_mut_ptr() as usize + 0x50, tag);
        exe::call_va!(orig, (buf_o.as_mut_ptr() as usize) -> ());
        exe::call_va!(new, (buf_n.as_mut_ptr() as usize) -> ());
        crate::diff_eq!(obj_o[1], obj_n[1], "obj");
        crate::diff_eq!(rd::<usize>(buf_o.as_ptr() as usize) == 0, rd::<usize>(buf_n.as_ptr() as usize) == 0, "p0");
        crate::diff_eq!(rd::<usize>(buf_o.as_ptr() as usize + 0x38) == 0, rd::<usize>(buf_n.as_ptr() as usize + 0x38) == 0, "p38");
    }
    Ok(4)
}

#[inline(always)]
unsafe fn free_arena_block(r9: usize) {
    let rcx = (r9.wrapping_sub(1)) & !0x1ff_ffffusize;
    let idx = r9.wrapping_sub(rcx) >> 16;
    let mut rdx = rcx.wrapping_add(0x108).wrapping_add(idx.wrapping_mul(80));
    let back = rd::<u32>(rdx + 0x4) as usize;
    rdx = rdx.wrapping_sub(back);
    let teb = exe::teb();
    if teb == rd::<usize>(rcx + 0x100) {
        if rd::<u8>(rdx + 0xe) == 0 {
            let next = rd::<usize>(rdx + 0x20);
            wr::<usize>(r9, next);
            wr::<usize>(rdx + 0x20, r9);
            let cnt = rd::<u32>(rdx + 0x18).wrapping_sub(1);
            wr::<u32>(rdx + 0x18, cnt);
            if cnt == 0 {
                exe::call_va!(0x1_4253_71a0, (rdx) -> ());
            }
        } else {
            exe::call_va!(0x1_4253_3200, (rcx, rdx, 1u8, r9) -> ());
        }
    } else {
        exe::call_va!(0x1_4253_3200, (rcx, rdx, 0u8, r9) -> ());
    }
}

/// Original: `0x1420849f0` (157 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420849f0(rcx: usize) {
    exe::call_va!(0x1_4208_52e0, (rcx) -> ());
    let r9 = rd::<usize>(rcx);
    if r9 != 0 {
        free_arena_block(r9);
    }
    wr::<usize>(rcx, 0);
    wr::<u32>(rcx + 0x14, 0);
}

pub unsafe fn diff_d3d_unknown_1420849f0(orig: usize, new: usize) -> Result<u32, String> {
    let mut r_o = [0u32; 6];
    r_o[2] = 3;
    r_o[3] = 4;
    r_o[5] = 8;
    let mut r_n = r_o;
    exe::call_va!(orig, (r_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (r_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(r_o, r_n, "ring");
    Ok(1)
}

#[inline(always)]
unsafe fn mask_empty_16(addr: usize) -> u32 {
    let mut mask = 0u32;
    let mut i = 0usize;
    while i < 16 {
        if rd::<u8>(addr.wrapping_add(i)) == 0x80 {
            mask |= 1u32 << i;
        }
        i += 1;
    }
    mask
}

/// Original: `0x142084c50` (218 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142084c50(rcx: usize, rdx: usize) {
    let rsi = rd::<usize>(rdx + 0x8);
    let obj = rd::<usize>(rsi + 0x18);
    if obj != 0 {
        let vtbl = rd::<usize>(obj);
        let release = rd::<usize>(vtbl + 0x10);
        exe::call_va!(release, (obj) -> ());
    }
    wr::<usize>(rsi + 0x18, 0);
    wr::<u64>(rcx + 0x10, rd::<u64>(rcx + 0x10).wrapping_sub(1));

    let r10 = rd::<usize>(rcx);
    let rax = rd::<usize>(rdx);
    let r9 = rax.wrapping_sub(r10);
    let mask_a = mask_empty_16(rax);
    let r11 = r9.wrapping_sub(0x10);
    let cap_mask = rd::<usize>(rcx + 0x18);
    let mask_c = mask_empty_16(r10.wrapping_add(r11 & cap_mask));

    let use_empty = if mask_c != 0 && mask_a != 0 {
        let tz = mask_a.trailing_zeros();
        let lz = (mask_c << 16).leading_zeros();
        lz.wrapping_add(tz) < 0x10
    } else {
        false
    };

    let ctrl_byte: u8 = if use_empty { 0x80 } else { 0xfe };
    wr::<u8>(r10.wrapping_add(r9), ctrl_byte);

    let cap_mask2 = rd::<usize>(rcx + 0x18);
    let clone_off = (cap_mask2 & r11).wrapping_add(cap_mask2 & 0xf);
    let base2 = rd::<usize>(rcx);
    wr::<u8>(base2.wrapping_add(clone_off).wrapping_add(1), ctrl_byte);
    wr::<u64>(
        rcx + 0x28,
        rd::<u64>(rcx + 0x28).wrapping_add(use_empty as u64),
    );
}

pub unsafe fn diff_d3d_unknown_142084c50(orig: usize, new: usize) -> Result<u32, String> {
    for &fill in &[0x80u8, 0x01u8] {
        let mut ctrl_o = [fill; 64];
        let mut ctrl_n = [fill; 64];
        let mut slot_o = [0u64; 8];
        let mut slot_n = [0u64; 8];
        let mut tbl_o = [0u64; 6];
        let mut tbl_n = [0u64; 6];
        tbl_o[0] = ctrl_o.as_mut_ptr() as u64;
        tbl_o[2] = 5;
        tbl_o[3] = 0x1f;
        tbl_o[5] = 10;
        tbl_n[0] = ctrl_n.as_mut_ptr() as u64;
        tbl_n[2] = 5;
        tbl_n[3] = 0x1f;
        tbl_n[5] = 10;
        let it_o = [ctrl_o.as_mut_ptr() as usize + 5, slot_o.as_mut_ptr() as usize];
        let it_n = [ctrl_n.as_mut_ptr() as usize + 5, slot_n.as_mut_ptr() as usize];
        exe::call_va!(orig, (tbl_o.as_mut_ptr() as usize, it_o.as_ptr() as usize) -> ());
        exe::call_va!(new, (tbl_n.as_mut_ptr() as usize, it_n.as_ptr() as usize) -> ());
        crate::diff_eq!(ctrl_o, ctrl_n, "ctrl");
        crate::diff_eq!(tbl_o[1..], tbl_n[1..], "tbl");
    }
    Ok(2)
}

/// Original: `0x1420850b0` (216 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420850b0(rcx: usize) {
    let tag = rd::<i8>(rcx + 0x50) as isize;
    let rva = rd::<u32>(exe::ptr::<u32>((0x1_4208_5168isize + tag * 4) as u64) as usize);
    match rva {
        0x0208_50d1 | 0x0208_50ec => {
            let obj = rd::<usize>(rcx);
            if obj != 0 {
                let vtbl = rd::<usize>(obj);
                let release = rd::<usize>(vtbl + 0x10);
                exe::call_va!(release, (obj) -> ());
            }
            wr::<usize>(rcx, 0);
        }
        0x0208_5107 => {
            let rdx = rd::<usize>(rcx + 0x18);
            if rdx != 0 {
                let alloc = rd::<usize>(rcx + 0x30);
                let r9d = rd::<u16>(rcx + 0x28) as u32;
                let vtbl = rd::<usize>(alloc);
                let r8 = rd::<usize>(rcx + 0x20);
                let fn_ptr = rd::<usize>(vtbl + 0x10);
                exe::call_va!(fn_ptr, (alloc, rdx, r8, r9d) -> ());
            }
            let obj = rd::<usize>(rcx);
            if obj != 0 {
                let vtbl = rd::<usize>(obj);
                let release = rd::<usize>(vtbl + 0x10);
                exe::call_va!(release, (obj) -> ());
            }
            wr::<usize>(rcx, 0);
        }
        0x0208_513f => {
            let target = rd::<usize>(rcx + 0x38);
            if target != 0 {
                let vtbl = rd::<usize>(target);
                let fn_ptr = rd::<usize>(vtbl + 0x20);
                let is_heap = (target != rcx) as u8;
                exe::call_va!(fn_ptr, (target, is_heap) -> ());
                wr::<usize>(rcx + 0x38, 0);
            }
        }
        _ => {}
    }
}

pub unsafe fn diff_d3d_unknown_1420850b0(orig: usize, new: usize) -> Result<u32, String> {
    diff_d3d_unknown_1420848e0(orig, new)
}

/// Original: `0x1420851b0` (167 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420851b0(rcx: usize) {
    exe::call_va!(0x1_4208_52e0, (rcx) -> ());
    let r9 = rd::<usize>(rcx);
    if r9 != 0 {
        free_arena_block(r9);
    }
    wr::<u64>(rcx, 0);
    wr::<u64>(rcx + 0x8, 0);
    wr::<u64>(rcx + 0x10, 0);
}

pub unsafe fn diff_d3d_unknown_1420851b0(orig: usize, new: usize) -> Result<u32, String> {
    let mut r_o = [0u32; 6];
    r_o[2] = 3;
    r_o[3] = 4;
    r_o[5] = 8;
    let mut r_n = r_o;
    exe::call_va!(orig, (r_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (r_n.as_mut_ptr() as usize) -> ());
    crate::diff_eq!(r_o, r_n, "ring");
    Ok(1)
}

#[inline]
unsafe fn skip_empty_or_deleted_ctrl(mut ctrl: usize, mut slot: usize) -> (usize, usize) {
    while rd::<i8>(ctrl) < -1 {
        let mut mask: u32 = 0;
        let mut i = 0usize;
        while i < 16 {
            if rd::<i8>(ctrl + i) < -1 {
                mask |= 1u32 << i;
            }
            i += 1;
        }
        let step = mask.wrapping_add(1).trailing_zeros() as usize;
        ctrl = ctrl.wrapping_add(step);
        slot = slot.wrapping_add(step << 6);
    }
    (ctrl, slot)
}

/// Original: `0x1420857a0` (199 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_swiss64_erase_range(
    rcx: usize,
    rdx: usize,
    r8: usize,
    r9: usize,
) -> usize {
    let end_ctrl = rd::<usize>(r9);
    if rd::<usize>(r8) != end_ctrl {
        let step_const = exe::ptr::<u64>(0x1_4342_7980) as usize;
        let step_c = rd::<u64>(step_const) as usize;
        let step_s = rd::<u64>(step_const + 0x8) as usize;
        loop {
            let cur_c = rd::<usize>(r8);
            let cur_s = rd::<usize>(r8 + 0x8);
            let next_c = cur_c.wrapping_add(step_c);
            let next_s = cur_s.wrapping_add(step_s);
            wr::<usize>(r8, next_c);
            wr::<usize>(r8 + 0x8, next_s);
            let mut c = next_c;
            if rd::<i8>(c) < -1 {
                let (nc, ns) = skip_empty_or_deleted_ctrl(c, next_s);
                c = nc;
                wr::<usize>(r8, nc);
                wr::<usize>(r8 + 0x8, ns);
            }
            let mut tmp = [cur_c, cur_s];
            exe::call_va!(0x1_4208_4c50, (rcx, tmp.as_mut_ptr() as usize) -> ());
            if c == end_ctrl {
                break;
            }
        }
    }
    wr::<u64>(rdx, rd::<u64>(r9));
    wr::<u64>(rdx + 0x8, rd::<u64>(r9 + 0x8));
    rdx
}

pub unsafe fn diff_d3d_swiss64_erase_range(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_erase(rcx: usize, it: usize) {
        wr::<u32>(rcx, rd::<u32>(rcx).wrapping_add(1));
        wr::<usize>(rcx.wrapping_add(0x8), rd::<usize>(it.wrapping_add(0x8)));
    }
    let _m = crate::difftest::Mock::new(0x1_4208_4c50, fake_erase as *const ())?;
    let ctrl_bytes: [u8; 32] = [0x01, 0x80, 0xFE, 0x02, 0x03, 0xFF, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let c_base = ctrl_bytes.as_ptr() as usize;
    let mut rcx_o = [0usize; 2];
    let mut rcx_n = [0usize; 2];
    let mut out_o = [0usize; 2];
    let mut out_n = [0usize; 2];
    let mut beg_o = [c_base, 0x1000usize];
    let mut beg_n = beg_o;
    let end = [c_base + 4, 0x1100usize];
    let ro = exe::call_va!(orig, (rcx_o.as_mut_ptr() as usize, out_o.as_mut_ptr() as usize, beg_o.as_mut_ptr() as usize, end.as_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (rcx_n.as_mut_ptr() as usize, out_n.as_mut_ptr() as usize, beg_n.as_mut_ptr() as usize, end.as_ptr() as usize) -> usize);
    crate::diff_eq!(ro == out_o.as_ptr() as usize, rn == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(rcx_o, rcx_n, "rcx");
    crate::diff_eq!(out_o, out_n, "out");
    crate::diff_eq!(beg_o, beg_n, "beg");
    Ok(1)
}

/// Original: `0x142085ff0` (226 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_queue_bind_tile_mappings(rcx: usize, rdx: usize) {
    exe::call_va!(0x1_4208_b990, (rcx, rdx) -> ());
    let queue_vtbl = rd::<usize>(rdx);
    let rsi = rd::<usize>(rcx);
    let update_fn = rd::<usize>(queue_vtbl + 0x40);
    let mgr_owner = rd::<usize>(rcx + 0x8);
    let heap_idx = rd::<u16>(rsi + 0x8) as usize;
    let heap_mgr = rd::<usize>(mgr_owner + 0x18);
    exe::call_iat!(0x1_4264_55a8, (heap_mgr + 0x8) -> ());
    let heaps_base = rd::<usize>(heap_mgr + 0x10);
    let heap_ptr = rd::<usize>(heaps_base + heap_idx * 24 + 0x10);
    exe::call_iat!(0x1_4264_5598, (heap_mgr + 0x8) -> ());
    let node = rd::<usize>(rcx);
    let res_ptr = rd::<usize>(node);
    exe::call_va!(
        update_fn,
        (
            rdx,
            res_ptr,
            1u32,
            node + 0xc,
            node + 0x1c,
            heap_ptr,
            1u32,
            0usize,
            rsi + 0x2c,
            rsi + 0x30,
            0u32
        ) -> ()
    );
    let flag_addr = exe::ptr::<u8>(0x1_4385_8078) as usize;
    if rd::<u8>(flag_addr) != 0 {
        let fence_holder = rd::<usize>(exe::ptr::<usize>(0x1_43a3_6228) as usize);
        let next_val = rd::<u64>(fence_holder + 0x8).wrapping_add(1);
        wr::<u64>(fence_holder + 0x8, next_val);
        let signal_fn = rd::<usize>(rd::<usize>(rdx) + 0x70);
        let fence_ptr = rd::<usize>(fence_holder);
        exe::call_va!(signal_fn, (rdx, fence_ptr, next_val) -> ());
    }
}

pub unsafe fn diff_d3d_queue_bind_tile_mappings(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_wait(rcx: usize, _rdx: usize) {
        wr::<u32>(rcx.wrapping_add(0x10), 0x55);
    }
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(1));
    }
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(10));
    }
    unsafe extern "win64" fn fake_update(
        q: usize,
        res: usize,
        n_reg: u32,
        _coord: usize,
        _size: usize,
        heap: usize,
        n_rng: u32,
        _flags: usize,
        _offs: usize,
        _cnts: usize,
        mflags: u32,
    ) {
        wr::<usize>(q.wrapping_add(0x8), res ^ heap ^ (n_reg as usize) ^ ((n_rng as usize) << 8) ^ ((mflags as usize) << 16));
    }
    unsafe extern "win64" fn fake_signal(q: usize, fence: usize, fv: u64) {
        wr::<u64>(q.wrapping_add(0x10), (fence as u64) ^ fv);
    }
    let _m1 = crate::difftest::Mock::new(0x1_4208_b990, fake_wait as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    let f_addr = exe::ptr::<u8>(0x1_4385_8078) as *mut u8;
    let h_addr = exe::ptr::<u8>(0x1_43a3_6228) as *mut u8;
    let snap_f = crate::difftest::Snapshot::take(f_addr, 1);
    let snap_h = crate::difftest::Snapshot::take(h_addr, 8);
    let mut vtbl = [0usize; 16];
    vtbl[8] = fake_update as *const () as usize;
    vtbl[14] = fake_signal as *const () as usize;
    let mut q_o = [vtbl.as_ptr() as usize, 0, 0];
    let mut q_n = [vtbl.as_ptr() as usize, 0, 0];
    let mut node = [0u8; 0x40];
    wr::<usize>(node.as_mut_ptr() as usize, 0x1111);
    wr::<u16>(node.as_mut_ptr() as usize + 0x8, 1);
    let mut heaps = [0usize; 6];
    heaps[5] = 0x2222;
    let mut hmgr_o = [0usize, 0, heaps.as_ptr() as usize];
    let mut hmgr_n = [0usize, 0, heaps.as_ptr() as usize];
    let owner_o = [0usize, 0, 0, hmgr_o.as_mut_ptr() as usize];
    let owner_n = [0usize, 0, 0, hmgr_n.as_mut_ptr() as usize];
    let mut rcx_o = [node.as_ptr() as usize, owner_o.as_ptr() as usize, 0];
    let mut rcx_n = [node.as_ptr() as usize, owner_n.as_ptr() as usize, 0];
    let mut fh_o = [0x3333u64, 10u64];
    let mut fh_n = [0x3333u64, 10u64];
    wr::<u8>(f_addr as usize, 1);
    wr::<usize>(h_addr as usize, fh_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (rcx_o.as_mut_ptr() as usize, q_o.as_mut_ptr() as usize) -> ());
    wr::<usize>(h_addr as usize, fh_n.as_mut_ptr() as usize);
    exe::call_va!(new, (rcx_n.as_mut_ptr() as usize, q_n.as_mut_ptr() as usize) -> ());
    snap_h.restore();
    snap_f.restore();
    crate::diff_eq!(q_o[1..], q_n[1..], "queue");
    crate::diff_eq!(fh_o, fh_n, "fence_holder");
    crate::diff_eq!(hmgr_o[1], hmgr_n[1], "hmgr_lock");
    crate::diff_eq!(rcx_o[2], rcx_n[2], "wait_called");
    Ok(1)
}

/// Original: `0x142086eb0` (156 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_push_tile_res_if_type2(
    rcx: usize,
    _rdx: usize,
    _r8: usize,
    r9: u16,
    arg5: u16,
    arg6: u16,
    arg7: usize,
) {
    if rd::<u8>(arg7 + 0x1f) == 2 {
        let vec = rd::<usize>(rcx + 0x18).wrapping_add(0x1b0);
        let len = rd::<u32>(vec + 0x8) as u64;
        let cap = rd::<u32>(vec + 0xc) as u64;
        if (len << 4) == (cap << 4) {
            exe::call_va!(0x1_411e_06a0, (vec, len.wrapping_add(1)) -> ());
        }
        let idx = rd::<u32>(vec + 0x8) as usize;
        let base = rd::<usize>(vec);
        wr::<u64>(base + idx * 16 + 0x8, 0xffff);
        wr::<u64>(base + idx * 16, 0);
        let new_len = (idx as u32).wrapping_add(1);
        wr::<u32>(vec + 0x8, new_len);
        let elem = rd::<usize>(vec) + (new_len as usize) * 16 - 0x10;
        wr::<usize>(elem, arg7);
        wr::<u16>(elem + 0x8, arg5);
        wr::<u16>(elem + 0xa, arg6);
        wr::<u16>(elem + 0xc, r9);
    }
}

pub unsafe fn diff_d3d_push_tile_res_if_type2(orig: usize, new: usize) -> Result<u32, String> {
    let mut owner_a = [0u8; 0x1d0];
    let mut owner_b = [0u8; 0x1d0];
    let mut arr_a = [0u64; 4];
    let mut arr_b = [0u64; 4];
    let mut ctx_a = [0usize; 4];
    let mut ctx_b = [0usize; 4];
    let mut item = [0u8; 0x24];
    item[0x1f] = 2;
    let va = owner_a.as_mut_ptr() as usize + 0x1b0;
    let vb = owner_b.as_mut_ptr() as usize + 0x1b0;
    wr::<usize>(va, arr_a.as_mut_ptr() as usize);
    wr::<u32>(va + 0x8, 0);
    wr::<u32>(va + 0xc, 2);
    wr::<usize>(vb, arr_b.as_mut_ptr() as usize);
    wr::<u32>(vb + 0x8, 0);
    wr::<u32>(vb + 0xc, 2);
    ctx_a[3] = owner_a.as_mut_ptr() as usize;
    ctx_b[3] = owner_b.as_mut_ptr() as usize;
    let ip = item.as_ptr() as usize;
    exe::call_va!(orig, (ctx_a.as_mut_ptr() as usize, 0usize, 0usize, 11u16, 22u16, 33u16, ip) -> ());
    exe::call_va!(new, (ctx_b.as_mut_ptr() as usize, 0usize, 0usize, 11u16, 22u16, 33u16, ip) -> ());
    crate::diff_eq!(arr_a, arr_b, "pushed_elem");
    crate::diff_eq!(rd::<u32>(va + 0x8), rd::<u32>(vb + 0x8), "vec_len");
    Ok(1)
}

/// Original: `0x1420887f0` (183 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_enqueue_cmd_type6(rcx: usize, rdx: usize) -> usize {
    let r8 = rd::<usize>(rcx);
    let rbp = rd::<u64>(r8 + 0xc8);
    wr::<u64>(r8 + 0xc8, rbp.wrapping_add(1));
    let rbx = rd::<usize>(rcx);
    let mut cmd = [0u8; 0x60];
    let cmd_ptr = cmd.as_mut_ptr() as usize;
    wr::<u8>(cmd_ptr + 0x50, 6);
    wr::<u64>(cmd_ptr + 0x58, rbp);
    let len = rd::<u32>(rbx + 0x20);
    if len == rd::<u32>(rbx + 0x24) {
        exe::call_va!(0x1_4208_3150, (rbx + 0x10, len.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(rbx + 0x1c) as usize;
    let dst = rd::<usize>(rbx + 0x10).wrapping_add(tail * 96);
    exe::call_va!(0x1_4208_4000, (dst, cmd_ptr) -> ());
    let next_tail = rd::<u32>(rbx + 0x1c).wrapping_add(1);
    wr::<u32>(rbx + 0x1c, next_tail);
    if next_tail == rd::<u32>(rbx + 0x24) {
        wr::<u32>(rbx + 0x1c, 0);
    }
    wr::<u32>(rbx + 0x20, rd::<u32>(rbx + 0x20).wrapping_add(1));
    exe::call_va!(0x1_4208_50b0, (cmd_ptr) -> ());
    wr::<u64>(rdx, rbp);
    let owner = rd::<usize>(rcx);
    wr::<u64>(rdx + 0x8, rd::<u64>(owner + 0xd0));
    rdx
}

pub unsafe fn diff_d3d_enqueue_cmd_type6(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_move(dst: usize, src: usize) {
        wr::<u8>(dst.wrapping_add(0x50), rd::<u8>(src.wrapping_add(0x50)));
        wr::<u64>(dst.wrapping_add(0x58), rd::<u64>(src.wrapping_add(0x58)));
        wr::<i8>(src.wrapping_add(0x50), -1);
    }
    unsafe extern "win64" fn fake_dtor(_cmd: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_4208_4000, fake_move as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4208_50b0, fake_dtor as *const ())?;
    let mut ring_o = [0u8; 0xc0];
    let mut ring_n = [0u8; 0xc0];
    let mut own_o = [0u8; 0xe0];
    let mut own_n = [0u8; 0xe0];
    let po = own_o.as_mut_ptr() as usize;
    let pn = own_n.as_mut_ptr() as usize;
    wr::<usize>(po + 0x10, ring_o.as_mut_ptr() as usize);
    wr::<u32>(po + 0x1c, 1);
    wr::<u32>(po + 0x20, 1);
    wr::<u32>(po + 0x24, 2);
    wr::<u64>(po + 0xc8, 42);
    wr::<u64>(po + 0xd0, 0x7788);
    wr::<usize>(pn + 0x10, ring_n.as_mut_ptr() as usize);
    wr::<u32>(pn + 0x1c, 1);
    wr::<u32>(pn + 0x20, 1);
    wr::<u32>(pn + 0x24, 2);
    wr::<u64>(pn + 0xc8, 42);
    wr::<u64>(pn + 0xd0, 0x7788);
    let rcx_o = [po];
    let rcx_n = [pn];
    let mut out_o = [0u64; 2];
    let mut out_n = [0u64; 2];
    let ro = exe::call_va!(orig, (rcx_o.as_ptr() as usize, out_o.as_mut_ptr() as usize) -> usize);
    let rn = exe::call_va!(new, (rcx_n.as_ptr() as usize, out_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(ro == out_o.as_ptr() as usize, rn == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(ring_o, ring_n, "ring");
    crate::diff_eq!(out_o, out_n, "out");
    crate::diff_eq!(rd::<u64>(po + 0xc8), rd::<u64>(pn + 0xc8), "seq");
    crate::diff_eq!(rd::<u64>(po + 0x1c), rd::<u64>(pn + 0x1c), "tail_len");
    Ok(1)
}

/// Original: `0x142089060` (190 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_insert_and_release_entry(
    rcx: usize,
    rdx: usize,
    r8: usize,
    r9: usize,
) {
    let mut frame = [0u8; 0xc0];
    let fp = frame.as_mut_ptr() as usize;
    let r10 = rd::<u64>(rdx);
    wr::<u64>(fp + 0x40, r10);
    wr::<u32>(fp + 0x48, rd::<u32>(r8 + 0xc));
    wr::<u32>(fp + 0x4c, rd::<u32>(r8));
    wr::<u32>(fp + 0x50, rd::<u32>(r8 + 0x4));
    wr::<u64>(rdx, 0);
    wr::<u64>(fp + 0x70, r10);
    wr::<u64>(fp + 0x78, rd::<u64>(r8));
    wr::<u64>(fp + 0x80, rd::<u64>(r8 + 0x8));
    wr::<u64>(fp + 0x88, rd::<u64>(r9));
    wr::<u64>(fp + 0x90, rd::<u64>(r9 + 0x8));
    wr::<usize>(fp + 0xb0, rcx + 0x98);
    wr::<usize>(fp + 0x30, fp + 0x40);
    wr::<usize>(fp + 0x38, fp + 0x70);
    exe::call_va!(
        0x1_411e_1780,
        (fp + 0xb0, fp + 0x58, fp + 0x40, r9, fp + 0x30, fp + 0x38) -> ()
    );
    let p_temp = rd::<usize>(fp + 0x70);
    if p_temp != 0 {
        let vfn = rd::<usize>(rd::<usize>(p_temp) + 0x10);
        exe::call_va!(vfn, (p_temp) -> ());
    }
    let p_out = rd::<usize>(rdx);
    if p_out != 0 {
        let vfn = rd::<usize>(rd::<usize>(p_out) + 0x10);
        exe::call_va!(vfn, (p_out) -> ());
    }
    wr::<usize>(rdx, 0);
}

pub unsafe fn diff_d3d_insert_and_release_entry(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_insert(a1: usize, _a2: usize, a3: usize, _a4: usize, a5: usize, a6: usize) {
        let dst = rd::<usize>(a1);
        let k = rd::<usize>(a5);
        let v = rd::<usize>(a6);
        wr::<u64>(dst, rd::<u64>(k));
        wr::<u32>(dst + 0x8, rd::<u32>(a3 + 0x8) ^ rd::<u32>(a3 + 0xc) ^ rd::<u32>(a3 + 0x10));
        wr::<u64>(dst + 0x10, rd::<u64>(v + 0x8) ^ rd::<u64>(v + 0x10) ^ rd::<u64>(v + 0x18) ^ rd::<u64>(v + 0x20));
        wr::<u64>(v, 0);
    }
    let _m = crate::difftest::Mock::new(0x1_411e_1780, fake_insert as *const ())?;
    let mut rcx_o = [0u8; 0xc0];
    let mut rcx_n = [0u8; 0xc0];
    let mut rdx_o = [0x9999u64];
    let mut rdx_n = [0x9999u64];
    let r8 = [10u32, 20, 30, 40];
    let r9 = [0x1111_2222u64, 0x3333_4444u64];
    exe::call_va!(orig, (rcx_o.as_mut_ptr() as usize, rdx_o.as_mut_ptr() as usize, r8.as_ptr() as usize, r9.as_ptr() as usize) -> ());
    exe::call_va!(new, (rcx_n.as_mut_ptr() as usize, rdx_n.as_mut_ptr() as usize, r8.as_ptr() as usize, r9.as_ptr() as usize) -> ());
    crate::diff_eq!(rcx_o, rcx_n, "rcx");
    crate::diff_eq!(rdx_o, rdx_n, "rdx");
    Ok(1)
}

/// Original: `0x142089fb0` (200 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_enqueue_cmd_type5_and_unlock(rcx: usize) {
    wr::<u8>(rcx + 0xec, 0);
    let mut pair = [0u64; 2];
    let pair_ptr = pair.as_mut_ptr() as usize;
    exe::call_va!(0x1_4207_34e0, (pair_ptr, 0u32) -> ());
    let mut cmd = [0u8; 0x60];
    let cmd_ptr = cmd.as_mut_ptr() as usize;
    wr::<u64>(cmd_ptr, rd::<u64>(pair_ptr + 0x8));
    wr::<u64>(cmd_ptr + 0x8, rd::<u64>(pair_ptr));
    wr::<u8>(cmd_ptr + 0x50, 5);
    let seq = rd::<u64>(rcx + 0xc8);
    wr::<u64>(cmd_ptr + 0x58, seq);
    wr::<u64>(rcx + 0xc8, seq.wrapping_add(1));
    let len = rd::<u32>(rcx + 0x20);
    if len == rd::<u32>(rcx + 0x24) {
        exe::call_va!(0x1_4208_3150, (rcx + 0x10, len.wrapping_add(1)) -> ());
    }
    let tail = rd::<u32>(rcx + 0x1c) as usize;
    let dst = rd::<usize>(rcx + 0x10).wrapping_add(tail * 96);
    exe::call_va!(0x1_4208_4000, (dst, cmd_ptr) -> ());
    let next_tail = rd::<u32>(rcx + 0x1c).wrapping_add(1);
    wr::<u32>(rcx + 0x1c, next_tail);
    if next_tail == rd::<u32>(rcx + 0x24) {
        wr::<u32>(rcx + 0x1c, 0);
    }
    wr::<u32>(rcx + 0x20, rd::<u32>(rcx + 0x20).wrapping_add(1));
    exe::call_va!(0x1_4208_50b0, (cmd_ptr) -> ());
    exe::call_iat!(0x1_4264_5598, (rcx + 0x8) -> ());
    exe::call_iat!(0x1_4264_5598, (rcx + 0x28) -> ());
}

pub unsafe fn diff_d3d_enqueue_cmd_type5_and_unlock(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_pair(out: usize, _flag: u32) {
        wr::<u64>(out, 0xAAAA_BBBB);
        wr::<u64>(out + 0x8, 0xCCCC_DDDD);
    }
    unsafe extern "win64" fn fake_move(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u64>(dst + 0x8, rd::<u64>(src + 0x8));
        wr::<u8>(dst + 0x50, rd::<u8>(src + 0x50));
        wr::<u64>(dst + 0x58, rd::<u64>(src + 0x58));
    }
    unsafe extern "win64" fn fake_dtor(_cmd: usize) {}
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(1));
    }
    let _m1 = crate::difftest::Mock::new(0x1_4207_34e0, fake_pair as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4208_4000, fake_move as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4208_50b0, fake_dtor as *const ())?;
    let _m4 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    let mut ring_o = [0u8; 0xc0];
    let mut ring_n = [0u8; 0xc0];
    let mut rcx_o = [0u8; 0xf0];
    let mut rcx_n = [0u8; 0xf0];
    let po = rcx_o.as_mut_ptr() as usize;
    let pn = rcx_n.as_mut_ptr() as usize;
    wr::<usize>(po + 0x10, ring_o.as_mut_ptr() as usize);
    wr::<u32>(po + 0x1c, 1);
    wr::<u32>(po + 0x20, 1);
    wr::<u32>(po + 0x24, 2);
    wr::<u64>(po + 0xc8, 77);
    wr::<u8>(po + 0xec, 1);
    wr::<usize>(pn + 0x10, ring_n.as_mut_ptr() as usize);
    wr::<u32>(pn + 0x1c, 1);
    wr::<u32>(pn + 0x20, 1);
    wr::<u32>(pn + 0x24, 2);
    wr::<u64>(pn + 0xc8, 77);
    wr::<u8>(pn + 0xec, 1);
    exe::call_va!(orig, (po) -> ());
    exe::call_va!(new, (pn) -> ());
    crate::diff_eq!(ring_o, ring_n, "ring");
    crate::diff_eq!(rcx_o[0..0x10], rcx_n[0..0x10], "rcx_head");
    crate::diff_eq!(rcx_o[0x18..], rcx_n[0x18..], "rcx_tail");
    Ok(1)
}

/// Original: `0x14208a080` (239 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_try_begin_idle_flush(rcx: usize) -> u8 {
    let lock_outer = rcx + 0x28;
    let acquired = exe::call_iat!(0x1_4264_5788, (lock_outer) -> u8);
    if acquired == 0 {
        return 0;
    }
    let lock_inner = rcx + 0x8;
    exe::call_iat!(0x1_4264_55a8, (lock_inner) -> ());
    if rd::<u32>(rcx + 0x20) != 0 {
        exe::call_iat!(0x1_4264_5598, (lock_inner) -> ());
        exe::call_iat!(0x1_4264_5598, (lock_outer) -> ());
        return 0;
    }
    let fence_val = rd::<u64>(rcx + 0xd8);
    let fence = rd::<usize>(rcx + 0xd0);
    if fence != 0 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        let tls_ctx = rd::<usize>(tls0 + 0x6328);
        exe::call_va!(0x1_4206_df80, (tls_ctx) -> ());
        exe::call_va!(0x1_4207_02c0, (fence, fence_val) -> ());
    }
    wr::<u8>(rcx + 0xec, 1);
    1
}

pub unsafe fn diff_d3d_try_begin_idle_flush(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_try_lock(p: usize) -> u8 {
        rd::<u8>(p)
    }
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x10));
    }
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x100));
    }
    unsafe extern "win64" fn fake_ctx_fn(ctx: usize) {
        wr::<u32>(ctx, rd::<u32>(ctx).wrapping_add(1));
    }
    unsafe extern "win64" fn fake_fence_wait(fence: usize, fv: u64) {
        wr::<u64>(fence, fv);
    }
    let _m1 = crate::difftest::IatMock::new(0x1_4264_5788, fake_try_lock as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4206_df80, fake_ctx_fn as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4207_02c0, fake_fence_wait as *const ())?;
    let tls_slot = exe::tls_pointer();
    let old_tls0 = rd::<usize>(tls_slot);
    let mut tls_buf = vec![0u8; 0x6400];
    tls_buf[0x70] = 1;
    let mut ctx_o = 0u32;
    let mut ctx_n = 0u32;
    let mut f_o = 0u64;
    let mut f_n = 0u64;
    let mut s_o = [0u8; 0xf0];
    let mut s_n = [0u8; 0xf0];
    wr::<u8>(s_o.as_mut_ptr() as usize + 0x28, 1);
    wr::<usize>(s_o.as_mut_ptr() as usize + 0xd0, &mut f_o as *mut u64 as usize);
    wr::<u64>(s_o.as_mut_ptr() as usize + 0xd8, 0x4455);
    wr::<u8>(s_n.as_mut_ptr() as usize + 0x28, 1);
    wr::<usize>(s_n.as_mut_ptr() as usize + 0xd0, &mut f_n as *mut u64 as usize);
    wr::<u64>(s_n.as_mut_ptr() as usize + 0xd8, 0x4455);
    wr::<usize>(tls_slot, tls_buf.as_mut_ptr() as usize);
    wr::<usize>(tls_buf.as_mut_ptr() as usize + 0x6328, &mut ctx_o as *mut u32 as usize);
    let ro = exe::call_va!(orig, (s_o.as_mut_ptr() as usize) -> u8);
    wr::<usize>(tls_buf.as_mut_ptr() as usize + 0x6328, &mut ctx_n as *mut u32 as usize);
    let rn = exe::call_va!(new, (s_n.as_mut_ptr() as usize) -> u8);
    wr::<usize>(tls_slot, old_tls0);
    crate::diff_eq!(ro, rn, "ret");
    crate::diff_eq!(ctx_o, ctx_n, "ctx");
    crate::diff_eq!(f_o, f_n, "fence");
    crate::diff_eq!(s_o[0..0xd0], s_n[0..0xd0], "state_lo");
    crate::diff_eq!(s_o[0xd8..], s_n[0xd8..], "state_hi");
    Ok(1)
}

/// Original: `0x14208a1b0` (214 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_poll_completed_fence_and_wake(rcx: usize, rdx: u64) {
    let fence = rd::<usize>(rcx + 0xd0);
    let get_completed = rd::<usize>(rd::<usize>(fence) + 0x40);
    let completed = exe::call_va!(get_completed, (fence) -> u64);
    if completed == u64::MAX {
        exe::call_va!(0x1_4205_d410, (rcx, 0x887a_0005u32) -> ());
    }
    let mgr = rd::<usize>(rcx + 0xe0);
    exe::call_va!(0x1_4208_ad10, (mgr, rdx >> 0x19, completed) -> ());
    let lock = rcx + 0x28;
    let acquired = exe::call_iat!(0x1_4264_5788, (lock) -> u8);
    if acquired != 0 {
        let begin = rd::<usize>(rcx + 0x68);
        let count = rd::<u32>(rcx + 0x70) as usize;
        let end = begin.wrapping_add(count * 16);
        let mut cur = begin;
        while cur != end {
            if rd::<u64>(cur + 0x8) > completed {
                break;
            }
            cur = cur.wrapping_add(16);
        }
        if cur != rd::<usize>(rcx + 0x68) {
            wr::<u8>(rcx + 0x78, 1);
            let glob = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
            exe::call_iat!(0x1_4264_55a8, (glob + 0x250) -> ());
            wr::<u32>(glob + 0x258, rd::<u32>(glob + 0x258).wrapping_add(1));
            exe::call_iat!(0x1_4264_5598, (glob + 0x250) -> ());
            exe::call_iat!(0x1_4264_5590, (glob + 0x248) -> ());
        }
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }
}

pub unsafe fn diff_d3d_poll_completed_fence_and_wake(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_get_completed(_f: usize) -> u64 {
        100
    }
    unsafe extern "win64" fn fake_ad10(mgr: usize, arg2: u64, comp: u64) {
        wr::<u64>(mgr, arg2 ^ comp);
    }
    unsafe extern "win64" fn fake_try_lock(_p: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn fake_lock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x10));
    }
    unsafe extern "win64" fn fake_unlock(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x100));
    }
    unsafe extern "win64" fn fake_wake(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x1000));
    }
    let _m1 = crate::difftest::Mock::new(0x1_4208_ad10, fake_ad10 as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5788, fake_try_lock as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_55a8, fake_lock as *const ())?;
    let _m4 = crate::difftest::IatMock::new(0x1_4264_5598, fake_unlock as *const ())?;
    let _m5 = crate::difftest::IatMock::new(0x1_4264_5590, fake_wake as *const ())?;
    let g_addr = exe::ptr::<u8>(0x1_43a0_8298) as *mut u8;
    let snap = crate::difftest::Snapshot::take(g_addr, 8);
    let mut vtbl = [0usize; 10];
    vtbl[8] = fake_get_completed as *const () as usize;
    let fence = [vtbl.as_ptr() as usize];
    let items = [0u64, 50u64, 0u64, 150u64];
    let mut mgr_o = 0u64;
    let mut mgr_n = 0u64;
    let mut glob_o = [0u8; 0x270];
    let mut glob_n = [0u8; 0x270];
    let mut rcx_o = [0u8; 0xf0];
    let mut rcx_n = [0u8; 0xf0];
    wr::<usize>(rcx_o.as_mut_ptr() as usize + 0x68, items.as_ptr() as usize);
    wr::<u32>(rcx_o.as_mut_ptr() as usize + 0x70, 2);
    wr::<usize>(rcx_o.as_mut_ptr() as usize + 0xd0, fence.as_ptr() as usize);
    wr::<usize>(rcx_o.as_mut_ptr() as usize + 0xe0, &mut mgr_o as *mut u64 as usize);
    wr::<usize>(rcx_n.as_mut_ptr() as usize + 0x68, items.as_ptr() as usize);
    wr::<u32>(rcx_n.as_mut_ptr() as usize + 0x70, 2);
    wr::<usize>(rcx_n.as_mut_ptr() as usize + 0xd0, fence.as_ptr() as usize);
    wr::<usize>(rcx_n.as_mut_ptr() as usize + 0xe0, &mut mgr_n as *mut u64 as usize);
    wr::<usize>(g_addr as usize, glob_o.as_mut_ptr() as usize);
    exe::call_va!(orig, (rcx_o.as_mut_ptr() as usize, 3u64 << 0x19) -> ());
    wr::<usize>(g_addr as usize, glob_n.as_mut_ptr() as usize);
    exe::call_va!(new, (rcx_n.as_mut_ptr() as usize, 3u64 << 0x19) -> ());
    snap.restore();
    crate::diff_eq!(mgr_o, mgr_n, "mgr");
    crate::diff_eq!(glob_o, glob_n, "glob");
    crate::diff_eq!(rcx_o[0..0xe0], rcx_n[0..0xe0], "rcx");
    Ok(1)
}

// REGISTRY: Replacement::verified_throws(0x1_4208_1ea0, "d3d::d3d_tex_heap_alloc_array_60_142081ea0", rend::d3d_tex_heap::d3d_tex_heap_alloc_array_60 as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_tex_heap_alloc_array_60),
// REGISTRY: Replacement::verified_throws(0x1_4208_1ec0, "d3d::d3d_tex_heap_alloc_array_30_142081ec0", rend::d3d_tex_heap::d3d_tex_heap_alloc_array_30 as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_tex_heap_alloc_array_30),
// REGISTRY: Replacement::verified_throws(0x1_4208_1ee0, "d3d::d3d_tex_heap_alloc_array_38_142081ee0", rend::d3d_tex_heap::d3d_tex_heap_alloc_array_38 as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_tex_heap_alloc_array_38),
// REGISTRY: Replacement::verified(0x1_4208_27c0, "d3d::d3d_tex_heap_dtor_lambda_notify_1420827c0", rend::d3d_tex_heap::d3d_tex_heap_dtor_lambda_notify as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_dtor_lambda_notify),
// REGISTRY: Replacement::verified(0x1_4208_48e0, "d3d::d3d_unknown_1420848e0", rend::d3d_tex_heap::d3d_unknown_1420848e0 as *const (), rend::d3d_tex_heap::diff_d3d_unknown_1420848e0),
// REGISTRY: Replacement::verified(0x1_4208_49f0, "d3d::d3d_unknown_1420849f0", rend::d3d_tex_heap::d3d_unknown_1420849f0 as *const (), rend::d3d_tex_heap::diff_d3d_unknown_1420849f0),
// REGISTRY: Replacement::verified(0x1_4208_4c50, "d3d::d3d_unknown_142084c50", rend::d3d_tex_heap::d3d_unknown_142084c50 as *const (), rend::d3d_tex_heap::diff_d3d_unknown_142084c50),
// REGISTRY: Replacement::verified(0x1_4208_50b0, "d3d::d3d_unknown_1420850b0", rend::d3d_tex_heap::d3d_unknown_1420850b0 as *const (), rend::d3d_tex_heap::diff_d3d_unknown_1420850b0),
// REGISTRY: Replacement::verified(0x1_4208_51b0, "d3d::d3d_unknown_1420851b0", rend::d3d_tex_heap::d3d_unknown_1420851b0 as *const (), rend::d3d_tex_heap::diff_d3d_unknown_1420851b0),
// REGISTRY: Replacement::verified(0x1_4208_52e0, "d3d::d3d_tex_heap_ring_clear_60_1420852e0", rend::d3d_tex_heap::d3d_tex_heap_ring_clear_60 as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_ring_clear_60),
// REGISTRY: Replacement::verified(0x1_4208_5350, "d3d::d3d_tex_heap_ring_push_60_142085350", rend::d3d_tex_heap::d3d_tex_heap_ring_push_60 as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_ring_push_60),
// REGISTRY: Replacement::verified(0x1_4208_5720, "d3d::d3d_tex_heap_swiss_erase_iter_142085720", rend::d3d_tex_heap::d3d_tex_heap_swiss_erase_iter as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_swiss_erase_iter),
// REGISTRY: Replacement::verified(0x1_4208_57a0, "d3d::d3d_swiss64_erase_range_1420857a0", rend::d3d_tex_heap::d3d_swiss64_erase_range as *const (), rend::d3d_tex_heap::diff_d3d_swiss64_erase_range),
// REGISTRY: Replacement::verified_throws(0x1_4208_5ff0, "d3d::d3d_queue_bind_tile_mappings_142085ff0", rend::d3d_tex_heap::d3d_queue_bind_tile_mappings as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_queue_bind_tile_mappings),
// REGISTRY: Replacement::verified(0x1_4208_8210, "d3d::d3d_tex_heap_notify_completion_142088210", rend::d3d_tex_heap::d3d_tex_heap_notify_completion as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_notify_completion),
// REGISTRY: Replacement::verified(0x1_4208_87f0, "d3d::d3d_enqueue_cmd_type6_1420887f0", rend::d3d_tex_heap::d3d_enqueue_cmd_type6 as *const (), rend::d3d_tex_heap::diff_d3d_enqueue_cmd_type6),
// REGISTRY: Replacement::verified(0x1_4208_88b0, "d3d::d3d_tex_heap_unlock_and_wake_global_1420888b0", rend::d3d_tex_heap::d3d_tex_heap_unlock_and_wake_global as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_unlock_and_wake_global),
// REGISTRY: Replacement::verified(0x1_4208_8910, "d3d::d3d_tex_heap_lock_guard_init_142088910", rend::d3d_tex_heap::d3d_tex_heap_lock_guard_init as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_lock_guard_init),
// REGISTRY: Replacement::verified(0x1_4208_9060, "d3d::d3d_insert_and_release_entry_142089060", rend::d3d_tex_heap::d3d_insert_and_release_entry as *const (), rend::d3d_tex_heap::diff_d3d_insert_and_release_entry),
// REGISTRY: Replacement::verified(0x1_4208_9160, "d3d::d3d_tex_heap_flush_and_signal_142089160", rend::d3d_tex_heap::d3d_tex_heap_flush_and_signal as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_flush_and_signal),
// REGISTRY: Replacement::verified(0x1_4208_91b0, "d3d::d3d_tex_heap_flush_signal_and_wait_1420891b0", rend::d3d_tex_heap::d3d_tex_heap_flush_signal_and_wait as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_flush_signal_and_wait),
// REGISTRY: Replacement::verified(0x1_4208_9220, "d3d::d3d_tex_heap_mark_slot_state_3_142089220", rend::d3d_tex_heap::d3d_tex_heap_mark_slot_state_3 as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_mark_slot_state_3),
// REGISTRY: Replacement::verified_throws(0x1_4208_9280, "d3d::d3d_tex_heap_lookup_slot_by_handle_142089280", rend::d3d_tex_heap::d3d_tex_heap_lookup_slot_by_handle as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_tex_heap_lookup_slot_by_handle),
// REGISTRY: Replacement::verified(0x1_4208_9ba0, "d3d::d3d_tex_heap_flush_and_signal_direct_142089ba0", rend::d3d_tex_heap::d3d_tex_heap_flush_and_signal_direct as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_flush_and_signal_direct),
// REGISTRY: Replacement::verified(0x1_4208_9c40, "d3d::d3d_tex_heap_submit_cmd_list_142089c40", rend::d3d_tex_heap::d3d_tex_heap_submit_cmd_list as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_submit_cmd_list),
// REGISTRY: Replacement::verified_throws(0x1_4208_9fb0, "d3d::d3d_enqueue_cmd_type5_and_unlock_142089fb0", rend::d3d_tex_heap::d3d_enqueue_cmd_type5_and_unlock as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_enqueue_cmd_type5_and_unlock),
// REGISTRY: Replacement::verified_throws(0x1_4208_a080, "d3d::d3d_try_begin_idle_flush_14208a080", rend::d3d_tex_heap::d3d_try_begin_idle_flush as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_try_begin_idle_flush),
// REGISTRY: Replacement::verified_throws(0x1_4208_a1b0, "d3d::d3d_poll_completed_fence_and_wake_14208a1b0", rend::d3d_tex_heap::d3d_poll_completed_fence_and_wake as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_poll_completed_fence_and_wake),
// REGISTRY: Replacement::verified(0x1_4208_a2b0, "d3d::d3d_tex_heap_queue_wait_streamer_14208a2b0", rend::d3d_tex_heap::d3d_tex_heap_queue_wait_streamer as *const (), rend::d3d_tex_heap::diff_d3d_tex_heap_queue_wait_streamer),
// REGISTRY: Replacement::verified(0x1_4208_a7a0, "d3d::d3d_async_streamer_has_pending_work_14208a7a0", rend::d3d_tex_heap::d3d_async_streamer_has_pending_work as *const (), rend::d3d_tex_heap::diff_d3d_async_streamer_has_pending_work),
// REGISTRY: Replacement::verified_throws(0x1_4208_aa30, "d3d::d3d_async_streamer_scalar_deleting_dtor_14208aa30", rend::d3d_tex_heap::d3d_async_streamer_scalar_deleting_dtor as *const (), Throws::Discard, rend::d3d_tex_heap::diff_d3d_async_streamer_scalar_deleting_dtor),

