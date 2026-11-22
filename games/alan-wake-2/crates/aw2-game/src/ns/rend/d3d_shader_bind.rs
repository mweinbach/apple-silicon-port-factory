use crate::difftest::{Mock, Snapshot};
use crate::exe::{self, rd, wr};

struct TlsSlot0Guard(usize);

impl TlsSlot0Guard {
    unsafe fn install(fake_tls0: usize) -> Self {
        let slot_ptr = exe::tls_pointer();
        let old = rd::<usize>(slot_ptr);
        wr::<u8>(fake_tls0 + 0x70, 1);
        wr::<usize>(slot_ptr, fake_tls0);
        Self(old)
    }
}

impl Drop for TlsSlot0Guard {
    fn drop(&mut self) {
        unsafe {
            wr::<usize>(exe::tls_pointer(), self.0);
        }
    }
}

#[inline(always)]
unsafe fn tls_slot0() -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    tls0
}

/// Original: `0x14209ac20` (77 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_tls_shader_slot_empty(this: usize) -> u8 {
    let tls0 = tls_slot0();
    let idx = rd::<i32>(this + 0x10) as isize as usize;
    let base = tls0.wrapping_add(0x5b460).wrapping_add(idx.wrapping_mul(40));
    u8::from(rd::<u64>(base + 0x18) == 0)
}

pub unsafe fn diff_d3d_is_tls_shader_slot_empty(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(new);
    let mut tls_buf = vec![0u8; 0x5c000];
    let tls_ptr = tls_buf.as_mut_ptr() as usize;
    let _g = TlsSlot0Guard::install(tls_ptr);
    let mut obj = [0u8; 0x20];
    let this = obj.as_mut_ptr() as usize;
    for (idx, val) in [(0i32, 0u64), (0, 0x1234), (2, 0), (2, 0xdead_beef)] {
        wr::<i32>(this + 0x10, idx);
        wr::<u64>(tls_ptr + 0x5b460 + (idx as usize) * 40 + 0x18, val);
        crate::diff_eq!(f_o(this), f_n(this), "is_tls_shader_slot_empty");
    }
    Ok(4)
}

/// Original: `0x14209afe0` (58 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_dec_tls_shader_ref() {
    let tls0 = tls_slot0();
    let mgr = rd::<usize>(tls0 + 0x6360);
    let obj = exe::call_va!(0x1_411d_9920, (mgr) -> usize);
    let prev = rd::<u64>(obj + 0x28);
    wr::<u64>(obj + 0x28, prev.wrapping_sub(1));
}

pub unsafe fn diff_d3d_dec_tls_shader_ref(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_get_sub_mgr(mgr: usize) -> usize {
        mgr + 0x10
    }
    let _m = Mock::new(0x1_411d_9920, fake_get_sub_mgr as *const ())?;
    let f_o: unsafe extern "win64" fn() = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn() = core::mem::transmute(new);

    let mut tls_buf = vec![0u8; 0x6400];
    let tls_ptr = tls_buf.as_mut_ptr() as usize;
    let _g = TlsSlot0Guard::install(tls_ptr);

    let mut mgr_o = [0u8; 0x60];
    let mut mgr_n = [0u8; 0x60];
    let mo = mgr_o.as_mut_ptr() as usize;
    let mn = mgr_n.as_mut_ptr() as usize;
    wr::<u64>(mo + 0x38, 10);
    wr::<u64>(mn + 0x38, 10);

    wr::<usize>(tls_ptr + 0x6360, mo);
    f_o();
    wr::<usize>(tls_ptr + 0x6360, mn);
    f_n();
    crate::diff_eq!(mgr_o, mgr_n, "dec_tls_shader_ref");
    Ok(1)
}

/// Original: `0x14209b020` (141 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_fetch_tls_shader_slot(this: usize) -> u64 {
    let tls0 = tls_slot0();
    let slot_idx = rd::<i32>(this + 0x10) as isize as usize;
    let handle_id = rd::<i32>(this + 0x4) as isize as usize;
    let mgr = rd::<usize>(tls0 + 0x6360);
    let dst = tls0.wrapping_add(0x5b460).wrapping_add(slot_idx.wrapping_mul(40));
    let sub_mgr = exe::call_va!(0x1_411d_9920, (mgr) -> usize);
    let mut tmp = [0u8; 48];
    let tmp_ptr = tmp.as_mut_ptr() as usize;
    let src = exe::call_va!(0x1_411d_9690, (sub_mgr, tmp_ptr, handle_id) -> usize);
    for off in (0..0x28).step_by(8) {
        wr::<u64>(dst + off, rd::<u64>(src + off));
    }
    rd::<u64>(dst + 0x18)
}

pub unsafe fn diff_d3d_fetch_tls_shader_slot(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_get_sub_mgr(mgr: usize) -> usize {
        mgr ^ 0x100
    }
    unsafe extern "win64" fn fake_fetch_entry(sub_mgr: usize, tmp: usize, handle_id: usize) -> usize {
        for i in 0..5usize {
            wr::<u64>(
                tmp + i * 8,
                (sub_mgr as u64)
                    .wrapping_add((handle_id as u64).wrapping_mul(0x1000))
                    .wrapping_add((i as u64 + 1) * 0x11),
            );
        }
        tmp
    }
    let _m1 = Mock::new(0x1_411d_9920, fake_get_sub_mgr as *const ())?;
    let _m2 = Mock::new(0x1_411d_9690, fake_fetch_entry as *const ())?;
    let f_o: unsafe extern "win64" fn(usize) -> u64 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) -> u64 = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x5c000];
    let mut tls_n = vec![0u8; 0x5c000];
    let to = tls_o.as_mut_ptr() as usize;
    let tn = tls_n.as_mut_ptr() as usize;
    wr::<usize>(to + 0x6360, 0x5000);
    wr::<usize>(tn + 0x6360, 0x5000);

    let mut obj = [0u8; 0x20];
    let this = obj.as_mut_ptr() as usize;
    wr::<i32>(this + 0x4, 7);
    wr::<i32>(this + 0x10, 3);

    let _g = TlsSlot0Guard::install(to);
    let ro = f_o(this);
    wr::<u8>(tn + 0x70, 1);
    wr::<usize>(exe::tls_pointer(), tn);
    let rn = f_n(this);
    crate::diff_eq!(ro, rn, "fetch_tls_shader_slot_ret");
    crate::diff_eq!(
        &tls_o[0x5b460 + 3 * 40..0x5b460 + 4 * 40],
        &tls_n[0x5b460 + 3 * 40..0x5b460 + 4 * 40],
        "fetch_tls_shader_slot_dst"
    );
    Ok(1)
}

/// Original: `0x14209b340` (158 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_shader_has_binding_id(this: usize, id: u32) -> u8 {
    let group_count = rd::<u32>(this + 0x90) as usize;
    if group_count != 0 {
        let mut group_ptr = rd::<usize>(this + 0x88).wrapping_add(0x18);
        for _ in 0..group_count {
            let count = rd::<u32>(group_ptr + 0x8) as usize;
            if count != 0 {
                let mut item_ptr = rd::<usize>(group_ptr);
                for _ in 0..count {
                    if rd::<u32>(item_ptr) == id {
                        return 1;
                    }
                    item_ptr = item_ptr.wrapping_add(0x18);
                }
            }
            group_ptr = group_ptr.wrapping_add(0x28);
        }
    }
    let mut table_ptr = this.wrapping_add(0x8);
    for _ in 0..8usize {
        let count = rd::<u32>(table_ptr + 0x8) as usize;
        if count != 0 {
            let mut item_ptr = rd::<usize>(table_ptr);
            for _ in 0..count {
                if rd::<u32>(item_ptr) == id {
                    return 1;
                }
                item_ptr = item_ptr.wrapping_add(0x1c);
            }
        }
        table_ptr = table_ptr.wrapping_add(0x10);
    }
    0
}

pub unsafe fn diff_d3d_shader_has_binding_id(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, u32) -> u8 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u32) -> u8 = core::mem::transmute(new);

    let mut obj = [0u8; 0xb0];
    let mut group_items = [0u8; 0x30];
    let mut groups = [0u8; 0x50];
    let mut table_items = [0u8; 0x38];

    let g_items_ptr = group_items.as_mut_ptr() as usize;
    wr::<u32>(g_items_ptr, 100);
    wr::<u32>(g_items_ptr + 0x18, 200);

    let groups_ptr = groups.as_mut_ptr() as usize;
    wr::<usize>(groups_ptr + 0x18, g_items_ptr);
    wr::<u32>(groups_ptr + 0x20, 2);

    let t_items_ptr = table_items.as_mut_ptr() as usize;
    wr::<u32>(t_items_ptr, 300);
    wr::<u32>(t_items_ptr + 0x1c, 400);

    let base = obj.as_mut_ptr() as usize;
    wr::<usize>(base + 0x88, groups_ptr);
    wr::<u32>(base + 0x90, 1);
    wr::<usize>(base + 0x8 + 3 * 0x10, t_items_ptr);
    wr::<u32>(base + 0x8 + 3 * 0x10 + 0x8, 2);

    for target in [100u32, 200, 300, 400, 999, 0] {
        let ro = f_o(base, target);
        let rn = f_n(base, target);
        crate::diff_eq!(ro, rn, "shader_has_binding_id");
    }
    Ok(6)
}

/// Original: `0x14209b3f0` (158 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_deserialize_shader_refl(this: usize, stream: usize) -> u8 {
    let mut tmp = 0u32;
    let tmp_ptr = (&mut tmp as *mut u32) as usize;
    let vt = rd::<usize>(stream);
    let read_u32 = rd::<usize>(vt + 0x48);
    exe::call_va!(read_u32, (stream, tmp_ptr) -> ());
    wr::<u32>(this, tmp);

    let mut slot = this + 0x8;
    for _ in 0..8 {
        exe::call_va!(0x1_411e_3fe0, (stream, slot) -> ());
        slot += 0x10;
    }
    exe::call_va!(0x1_411e_4210, (stream, this + 0x88) -> ());
    exe::call_va!(0x1_411e_4210, (stream, this + 0x98) -> ());

    let vt2 = rd::<usize>(stream);
    let read_bool = rd::<usize>(vt2 + 0x80);
    exe::call_va!(read_bool, (stream, this + 0xa8) -> ());
    1
}

pub unsafe fn diff_d3d_deserialize_shader_refl(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_read_u32(stream: usize, out_ptr: usize) {
        let c = rd::<u32>(stream + 0x8).wrapping_add(1);
        wr::<u32>(stream + 0x8, c);
        wr::<u32>(out_ptr, 0x100 + c);
    }
    unsafe extern "win64" fn fake_read_bool(stream: usize, out_ptr: usize) {
        let c = rd::<u32>(stream + 0x8).wrapping_add(1);
        wr::<u32>(stream + 0x8, c);
        wr::<u8>(out_ptr, 1);
    }
    unsafe extern "win64" fn fake_3fe0(stream: usize, slot: usize) {
        let c = rd::<u32>(stream + 0x8).wrapping_add(1);
        wr::<u32>(stream + 0x8, c);
        wr::<u64>(slot, 0x2000 + c as u64);
        wr::<u64>(slot + 0x8, 0x3000 + c as u64);
    }
    unsafe extern "win64" fn fake_4210(stream: usize, slot: usize) {
        let c = rd::<u32>(stream + 0x8).wrapping_add(1);
        wr::<u32>(stream + 0x8, c);
        wr::<u64>(slot, 0x4000 + c as u64);
        wr::<u64>(slot + 0x8, 0x5000 + c as u64);
    }
    let _m1 = Mock::new(0x1_411e_3fe0, fake_3fe0 as *const ())?;
    let _m2 = Mock::new(0x1_411e_4210, fake_4210 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(new);

    let mut vtbl = [0u8; 0x90];
    let vt_ptr = vtbl.as_mut_ptr() as usize;
    wr::<usize>(vt_ptr + 0x48, fake_read_u32 as *const () as usize);
    wr::<usize>(vt_ptr + 0x80, fake_read_bool as *const () as usize);

    let mut this_o = [0u8; 0xb0];
    let mut this_n = [0u8; 0xb0];
    let mut stream_o = [0u8; 0x10];
    let mut stream_n = [0u8; 0x10];
    let so = stream_o.as_mut_ptr() as usize;
    let sn = stream_n.as_mut_ptr() as usize;
    wr::<usize>(so, vt_ptr);
    wr::<usize>(sn, vt_ptr);

    let ro = f_o(this_o.as_mut_ptr() as usize, so);
    let rn = f_n(this_n.as_mut_ptr() as usize, sn);
    crate::diff_eq!(ro, rn, "deserialize_shader_refl_ret");
    crate::diff_eq!(this_o, this_n, "deserialize_shader_refl_this");
    crate::diff_eq!(stream_o, stream_n, "deserialize_shader_refl_stream");
    Ok(1)
}

/// Original: `0x14209b6f0` (228 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_deserialize_shader_param(stream: usize, out: usize) -> usize {
    let mut sbuf = [0u8; 0xa0];
    let sbuf_ptr = sbuf.as_mut_ptr() as usize;
    wr::<u64>(sbuf_ptr, 0x1000_0010);
    wr::<u8>(sbuf_ptr + 0x8, 0);

    let vt0 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vt0 + 0x20), (stream, sbuf_ptr) -> ());

    let hash_fn = rd::<usize>(exe::ptr::<usize>(0x1_4397_fd20) as usize);
    let hdr = rd::<u32>(sbuf_ptr);
    let top = hdr >> 24;
    let cap_bits = if top < 0x80 {
        top << 3
    } else {
        (top << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (hdr & 0x00ff_ffff) << 3;
    let str_ptr = if len_bits > cap_bits {
        rd::<usize>(sbuf_ptr + 0x8)
    } else {
        sbuf_ptr + 0x8
    };
    let hash = exe::call_va!(hash_fn, (str_ptr) -> u32);
    wr::<u32>(out, hash);

    let mut tmp = 0u32;
    let tmp_ptr = (&mut tmp as *mut u32) as usize;
    let vt1 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vt1 + 0x48), (stream, tmp_ptr) -> ());
    wr::<u32>(out + 0x4, tmp);

    let vt2 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vt2 + 0x48), (stream, out + 0x8) -> ());
    let vt3 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vt3 + 0x48), (stream, out + 0xc) -> ());
    let vt4 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vt4 + 0x48), (stream, out + 0x10) -> ());
    let vt5 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vt5 + 0x80), (stream, out + 0x14) -> ());

    exe::call_va!(0x1_4240_ab10, (sbuf_ptr) -> ());
    stream
}

pub unsafe fn diff_d3d_deserialize_shader_param(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_read_str(stream: usize, sbuf: usize) {
        let mode = rd::<u32>(stream + 0xc);
        if mode == 0 {
            wr::<u32>(sbuf, 0x1000_0004);
            wr::<u32>(sbuf + 0x8, 0x6162_6364);
        } else {
            wr::<u32>(sbuf, 0x1000_0020);
            wr::<usize>(sbuf + 0x8, stream + 0x10);
        }
    }
    unsafe extern "win64" fn fake_hash(str_ptr: usize) -> u32 {
        rd::<u32>(str_ptr).wrapping_mul(0x9e37_79b1)
    }
    unsafe extern "win64" fn fake_read_u32(stream: usize, dst: usize) {
        let c = rd::<u32>(stream + 0x8).wrapping_add(1);
        wr::<u32>(stream + 0x8, c);
        wr::<u32>(dst, c.wrapping_mul(0x1111));
    }
    unsafe extern "win64" fn fake_read_bool(stream: usize, dst: usize) {
        let c = rd::<u32>(stream + 0x8).wrapping_add(1);
        wr::<u32>(stream + 0x8, c);
        wr::<u8>(dst, 1);
    }
    unsafe extern "win64" fn fake_dtor(_sbuf: usize) {}

    let _m = Mock::new(0x1_4240_ab10, fake_dtor as *const ())?;
    let snap = Snapshot::take(exe::ptr::<u8>(0x1_4397_fd20) as *mut u8, 8);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fd20) as usize, fake_hash as *const () as usize);

    let f_o: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(new);

    let mut vtbl = [0u8; 0x90];
    let vt_ptr = vtbl.as_mut_ptr() as usize;
    wr::<usize>(vt_ptr + 0x20, fake_read_str as *const () as usize);
    wr::<usize>(vt_ptr + 0x48, fake_read_u32 as *const () as usize);
    wr::<usize>(vt_ptr + 0x80, fake_read_bool as *const () as usize);

    let mut res = Ok(2);
    for mode in [0u32, 1u32] {
        let mut so_buf = [0u8; 0x20];
        let mut sn_buf = [0u8; 0x20];
        let mut out_o = [0u8; 0x20];
        let mut out_n = [0u8; 0x20];
        let so = so_buf.as_mut_ptr() as usize;
        let sn = sn_buf.as_mut_ptr() as usize;
        wr::<usize>(so, vt_ptr);
        wr::<u32>(so + 0xc, mode);
        wr::<u32>(so + 0x10, 0x5566_7788);
        wr::<usize>(sn, vt_ptr);
        wr::<u32>(sn + 0xc, mode);
        wr::<u32>(sn + 0x10, 0x5566_7788);

        let ro = f_o(so, out_o.as_mut_ptr() as usize);
        let rn = f_n(sn, out_n.as_mut_ptr() as usize);
        if ro - so != rn - sn || out_o != out_n || so_buf != sn_buf {
            res = Err(format!("mismatch in deserialize_shader_param mode={mode}"));
            break;
        }
    }
    snap.restore();
    res
}

/// Original: `0x14209b8f0` (76 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_shader_stage_slot_alignment(kind: u32) -> u32 {
    match kind {
        0 | 2 | 5 => 0x10,
        1 | 3 | 4 | 6 | 7 => 0x8,
        _ => 0,
    }
}

pub unsafe fn diff_d3d_shader_stage_slot_alignment(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(u32) -> u32 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(u32) -> u32 = core::mem::transmute(new);
    for k in 0..=12u32 {
        let ro = f_o(k);
        let rn = f_n(k);
        crate::diff_eq!(ro, rn, "shader_stage_slot_alignment");
    }
    Ok(13)
}

/// Original: `0x14209b980` (119 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_tls_shader_cbv_slice(this: usize, out: usize) -> usize {
    let tls0 = tls_slot0();
    let idx = rd::<i32>(this + 0x10) as isize as usize;
    wr::<u32>(out + 0xc, 0);
    let entry = tls0.wrapping_add(0x5b460).wrapping_add(idx.wrapping_mul(40));
    let offset_blocks = rd::<u32>(entry + 0x20) as u64;
    let base_ptr = rd::<u64>(entry + 0x8);
    wr::<u64>(out, base_ptr.wrapping_add(offset_blocks << 5));
    let size_blocks = rd::<u32>(entry + 0x24);
    wr::<u32>(out + 0x8, size_blocks << 4);
    out
}

pub unsafe fn diff_d3d_get_tls_shader_cbv_slice(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(new);
    let mut tls_buf = vec![0u8; 0x5c000];
    let tls_ptr = tls_buf.as_mut_ptr() as usize;
    let _g = TlsSlot0Guard::install(tls_ptr);

    let entry = tls_ptr + 0x5b460 + 2 * 40;
    wr::<u64>(entry + 0x8, 0x1000_0000);
    wr::<u32>(entry + 0x20, 5);
    wr::<u32>(entry + 0x24, 12);

    let mut obj = [0u8; 0x20];
    let this = obj.as_mut_ptr() as usize;
    wr::<i32>(this + 0x10, 2);

    let mut out_o = [0xffu8; 0x10];
    let mut out_n = [0xffu8; 0x10];
    let oo = out_o.as_mut_ptr() as usize;
    let on = out_n.as_mut_ptr() as usize;
    let ro = f_o(this, oo);
    let rn = f_n(this, on);
    crate::diff_eq!(ro - oo, rn - on, "cbv_slice_ret");
    crate::diff_eq!(out_o, out_n, "cbv_slice_out");
    Ok(1)
}

/// Original: `0x14209ba00` (188 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_dispatch_shader_refl_stage(this: usize) {
    let stage = rd::<i32>(this) as u32;
    match stage {
        0 => {
            exe::call_va!(0x1_411e_43c0, (this) -> ());
        }
        1 => {
            exe::call_va!(0x1_411e_4980, (this) -> ());
            exe::call_va!(0x1_411e_4f50, (this) -> ());
        }
        2 => {
            exe::call_va!(0x1_411e_5200, (this) -> ());
            exe::call_va!(0x1_411e_57c0, (this) -> ());
        }
        3 => {
            exe::call_va!(0x1_411e_43c0, (this) -> ());
            exe::call_va!(0x1_411e_5a70, (this) -> ());
        }
        4 => {
            exe::call_va!(0x1_411e_5d20, (this) -> ());
            exe::call_va!(0x1_411e_62f0, (this) -> ());
        }
        5 => {
            exe::call_va!(0x1_411e_65a0, (this) -> ());
            exe::call_va!(0x1_411e_57c0, (this) -> ());
        }
        _ => {}
    }
}

pub unsafe fn diff_d3d_dispatch_shader_refl_stage(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn tag<const K: u64>(this: usize) {
        wr::<u64>(this + 0x8, rd::<u64>(this + 0x8).wrapping_mul(131).wrapping_add(K));
    }
    let _m0 = Mock::new(0x1_411e_43c0, tag::<1> as *const ())?;
    let _m1 = Mock::new(0x1_411e_4980, tag::<2> as *const ())?;
    let _m2 = Mock::new(0x1_411e_4f50, tag::<3> as *const ())?;
    let _m3 = Mock::new(0x1_411e_5200, tag::<4> as *const ())?;
    let _m4 = Mock::new(0x1_411e_57c0, tag::<5> as *const ())?;
    let _m5 = Mock::new(0x1_411e_5a70, tag::<6> as *const ())?;
    let _m6 = Mock::new(0x1_411e_5d20, tag::<7> as *const ())?;
    let _m7 = Mock::new(0x1_411e_62f0, tag::<8> as *const ())?;
    let _m8 = Mock::new(0x1_411e_65a0, tag::<9> as *const ())?;

    let f_o: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    for s in [0i32, 1, 2, 3, 4, 5, 6, -1] {
        let mut bo = [0u8; 0x10];
        let mut bn = [0u8; 0x10];
        let po = bo.as_mut_ptr() as usize;
        let pn = bn.as_mut_ptr() as usize;
        wr::<i32>(po, s);
        wr::<i32>(pn, s);
        f_o(po);
        f_n(pn);
        crate::diff_eq!(bo, bn, "dispatch_shader_refl_stage");
    }
    Ok(8)
}

/// Original: `0x14209bad0` (60 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clear_tls_shader_slots() {
    let tls0 = tls_slot0();
    let dst = tls0.wrapping_add(0x5b460);
    exe::call_va!(0x1_4183_acc2, (dst, 0u32, 0x28000usize) -> usize);
}

pub unsafe fn diff_d3d_clear_tls_shader_slots(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn() = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn() = core::mem::transmute(new);
    let mut tls_o = vec![0xabu8; 0x83500];
    let mut tls_n = vec![0xabu8; 0x83500];
    let to = tls_o.as_mut_ptr() as usize;
    let tn = tls_n.as_mut_ptr() as usize;
    let _g = TlsSlot0Guard::install(to);
    f_o();
    wr::<u8>(tn + 0x70, 1);
    wr::<usize>(exe::tls_pointer(), tn);
    f_n();
    crate::diff_eq!(&tls_o[0x5b450..0x83470], &tls_n[0x5b450..0x83470], "clear_tls_shader_slots");
    Ok(1)
}

/// Original: `0x14209c0a0` (100 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_reinit_view_typed(
    this: usize,
    res: usize,
    a3: usize,
    a4: usize,
    fmt: u8,
) {
    exe::call_va!(0x1_4207_5970, (this) -> ());
    let mapped = exe::call_va!(0x1_4204_e8d0, (u32::from(fmt)) -> u64);
    exe::call_va!(0x1_4207_58f0, (this, res, a3, a4, mapped, u64::from(fmt)) -> ());
}

pub unsafe fn diff_d3d_reinit_view_typed(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5970(this: usize) {
        wr::<u64>(this, 0x1111);
    }
    unsafe extern "win64" fn fake_58f0(
        this: usize,
        res: usize,
        a3: usize,
        a4: usize,
        mapped: u64,
        fmt: u8,
    ) {
        wr::<u64>(this + 0x8, res as u64);
        wr::<u64>(this + 0x10, a3 as u64);
        wr::<u64>(this + 0x18, a4 as u64);
        wr::<u64>(this + 0x20, mapped);
        wr::<u8>(this + 0x28, fmt);
    }
    let _m1 = Mock::new(0x1_4207_5970, fake_5970 as *const ())?;
    let _m2 = Mock::new(0x1_4207_58f0, fake_58f0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, usize, usize, usize, u8) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize, usize, usize, u8) = core::mem::transmute(new);

    for fmt in [1u8, 10, 0x2a] {
        let mut bo = [0u8; 0x30];
        let mut bn = [0u8; 0x30];
        f_o(bo.as_mut_ptr() as usize, 0x2000, 0x300, 0x400, fmt);
        f_n(bn.as_mut_ptr() as usize, 0x2000, 0x300, 0x400, fmt);
        crate::diff_eq!(bo, bn, "reinit_view_typed");
    }
    Ok(3)
}

/// Original: `0x14209c110` (76 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_reinit_view_raw(
    this: usize,
    res: usize,
    _unused: usize,
    a4: usize,
) {
    exe::call_va!(0x1_4207_5970, (this) -> ());
    exe::call_va!(0x1_4207_58f0, (this, res, 0usize, a4, 0u64, 0u64) -> ());
}

pub unsafe fn diff_d3d_reinit_view_raw(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, usize, usize, usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize, usize, usize) = core::mem::transmute(new);
    let _m1 = Mock::new(0x1_4207_5970, {
        unsafe extern "win64" fn fake_5970(this: usize) {
            wr::<u64>(this, 0x2222);
        }
        fake_5970 as *const ()
    })?;
    let _m2 = Mock::new(0x1_4207_58f0, {
        unsafe extern "win64" fn fake_58f0(
            this: usize,
            res: usize,
            a3: usize,
            a4: usize,
            mapped: u64,
            fmt: u8,
        ) {
            wr::<u64>(this + 0x8, res as u64);
            wr::<u64>(this + 0x10, a3 as u64);
            wr::<u64>(this + 0x18, a4 as u64);
            wr::<u64>(this + 0x20, mapped);
            wr::<u8>(this + 0x28, fmt);
        }
        fake_58f0 as *const ()
    })?;
    let mut bo = [0u8; 0x30];
    let mut bn = [0u8; 0x30];
    f_o(bo.as_mut_ptr() as usize, 0x5000, 0x999, 0x777);
    f_n(bn.as_mut_ptr() as usize, 0x5000, 0x999, 0x777);
    crate::diff_eq!(bo, bn, "reinit_view_raw");
    Ok(1)
}

#[inline(always)]
unsafe fn resolve_buffer_residency_mode(mode: u32, byte_size: u64) -> u32 {
    if mode == 0 {
        4
    } else if mode == 2 {
        if byte_size > 0x40_0000 { 3 } else { 2 }
    } else {
        3
    }
}

unsafe extern "win64" fn mock_release_shader_buffer(this: usize) {
    wr::<u64>(this, 0xdead_0001);
}

unsafe extern "win64" fn mock_alloc_shader_buffer_5af0(
    byte_size: u64,
    stride: u64,
    fmt: u8,
    bind_flags: u32,
    heap_flags: u32,
    _unused: usize,
    out_this: usize,
) {
    wr::<u64>(out_this + 0x8, byte_size);
    wr::<u64>(out_this + 0x10, stride);
    wr::<u8>(out_this + 0x18, fmt);
    wr::<u32>(out_this + 0x1c, bind_flags);
    wr::<u32>(out_this + 0x20, heap_flags);
}

/// Original: `0x14209c940` (82 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_cbv_buffer(this: usize, byte_size: u64) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, 0u64, 0u8, 8u32, 0x10u32, 0usize, this) -> ());
    let state = if byte_size > 0x40_0000 { 3u32 } else { 2u32 };
    wr::<u32>(this + 0x50, state);
}

pub unsafe fn diff_d3d_init_cbv_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64) = core::mem::transmute(new);
    for sz in [0x100u64, 0x40_0000, 0x40_0001] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, sz);
        f_n(bn.as_mut_ptr() as usize, sz);
        crate::diff_eq!(bo, bn, "init_cbv_buffer");
    }
    Ok(3)
}

/// Original: `0x14209c9a0` (200 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_vertex_buffer(
    this: usize,
    count: u64,
    stride: u64,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    let total_bytes = count.wrapping_mul(stride);
    exe::call_va!(0x1_4207_5af0, (total_bytes, stride, 0u8, 5u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, total_bytes));
}

pub unsafe fn diff_d3d_init_vertex_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(new);
    for (cnt, stride, mode) in [(100u64, 32u64, 0u32), (100, 32, 1), (100, 32, 2), (0x20_0001, 2, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, cnt, stride, mode);
        f_n(bn.as_mut_ptr() as usize, cnt, stride, mode);
        crate::diff_eq!(bo, bn, "init_vertex_buffer");
    }
    Ok(4)
}

/// Original: `0x14209ca70` (200 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_index_buffer(
    this: usize,
    count: u64,
    stride: u64,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    let total_bytes = count.wrapping_mul(stride);
    exe::call_va!(0x1_4207_5af0, (total_bytes, stride, 0u8, 3u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, total_bytes));
}

pub unsafe fn diff_d3d_init_index_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(new);
    for (cnt, stride, mode) in [(64u64, 4u64, 0u32), (64, 4, 1), (64, 4, 2), (0x10_0001, 4, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, cnt, stride, mode);
        f_n(bn.as_mut_ptr() as usize, cnt, stride, mode);
        crate::diff_eq!(bo, bn, "init_index_buffer");
    }
    Ok(4)
}

/// Original: `0x14209cb40` (79 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_r32u_srv_buffer(this: usize, byte_size: u64) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    let elem_info = exe::call_va!(0x1_4204_e8d0, (0x2au32) -> u64);
    exe::call_va!(0x1_4207_5af0, (byte_size, elem_info, 0x2au8, 1u32, 0x10u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, 1);
}

pub unsafe fn diff_d3d_init_r32u_srv_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64) = core::mem::transmute(new);
    let mut bo = [0u8; 0x60];
    let mut bn = [0u8; 0x60];
    f_o(bo.as_mut_ptr() as usize, 0x1000);
    f_n(bn.as_mut_ptr() as usize, 0x1000);
    crate::diff_eq!(bo, bn, "init_r32u_srv_buffer");
    Ok(1)
}

/// Original: `0x14209cba0` (85 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_structured_srv_upload_buffer(
    this: usize,
    byte_size: u64,
    stride: u64,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, stride, 0u8, 1u32, 0x10u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, 1);
}

pub unsafe fn diff_d3d_init_structured_srv_upload_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64) = core::mem::transmute(new);
    let mut bo = [0u8; 0x60];
    let mut bn = [0u8; 0x60];
    f_o(bo.as_mut_ptr() as usize, 0x2000, 16);
    f_n(bn.as_mut_ptr() as usize, 0x2000, 16);
    crate::diff_eq!(bo, bn, "init_structured_srv_upload_buffer");
    Ok(1)
}

/// Original: `0x14209cc00` (196 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_rt_accel_buffer(
    this: usize,
    byte_size: u64,
    stride: u64,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, stride, 0u8, 0x51u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, byte_size));
}

pub unsafe fn diff_d3d_init_rt_accel_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(new);
    for (sz, mode) in [(0x1000u64, 0u32), (0x1000, 1), (0x1000, 2), (0x50_0000, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, sz, 64, mode);
        f_n(bn.as_mut_ptr() as usize, sz, 64, mode);
        crate::diff_eq!(bo, bn, "init_rt_accel_buffer");
    }
    Ok(4)
}

/// Original: `0x14209ccd0` (148 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_uav_only_buffer(
    this: usize,
    byte_size: u64,
    stride: u64,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, stride, 0u8, 0x10u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, byte_size));
}

pub unsafe fn diff_d3d_init_uav_only_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(new);
    for (sz, mode) in [(0x1000u64, 0u32), (0x1000, 1), (0x1000, 2), (0x50_0000, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, sz, 8, mode);
        f_n(bn.as_mut_ptr() as usize, sz, 8, mode);
        crate::diff_eq!(bo, bn, "init_uav_only_buffer");
    }
    Ok(4)
}

/// Original: `0x14209cd70` (131 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_indirect_uav_srv_buffer(
    this: usize,
    byte_size: u64,
    stride: u64,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, stride, 0u8, 0x31u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, 4);
}

pub unsafe fn diff_d3d_init_indirect_uav_srv_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64) = core::mem::transmute(new);
    let mut bo = [0u8; 0x60];
    let mut bn = [0u8; 0x60];
    f_o(bo.as_mut_ptr() as usize, 0x4000, 32);
    f_n(bn.as_mut_ptr() as usize, 0x4000, 32);
    crate::diff_eq!(bo, bn, "init_indirect_uav_srv_buffer");
    Ok(1)
}

/// Original: `0x14209ce00` (140 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_rgba32u_uav_srv_buffer(this: usize, byte_size: u64) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    let elem_info = exe::call_va!(0x1_4204_e8d0, (0xfu32) -> u64);
    exe::call_va!(0x1_4207_5af0, (byte_size, elem_info, 0xfu8, 0x11u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, 4);
}

pub unsafe fn diff_d3d_init_rgba32u_uav_srv_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64) = core::mem::transmute(new);
    let mut bo = [0u8; 0x60];
    let mut bn = [0u8; 0x60];
    f_o(bo.as_mut_ptr() as usize, 0x800);
    f_n(bn.as_mut_ptr() as usize, 0x800);
    crate::diff_eq!(bo, bn, "init_rgba32u_uav_srv_buffer");
    Ok(1)
}

/// Original: `0x14209cea0` (196 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_uav_srv_buffer(
    this: usize,
    byte_size: u64,
    stride: u64,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, stride, 0u8, 0x11u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, byte_size));
}

pub unsafe fn diff_d3d_init_uav_srv_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(new);
    for (sz, mode) in [(0x1000u64, 0u32), (0x1000, 1), (0x1000, 2), (0x50_0000, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, sz, 16, mode);
        f_n(bn.as_mut_ptr() as usize, sz, 16, mode);
        crate::diff_eq!(bo, bn, "init_uav_srv_buffer");
    }
    Ok(4)
}

/// Original: `0x14209cf70` (148 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_indirect_arg_buffer(
    this: usize,
    byte_size: u64,
    stride: u64,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, stride, 0u8, 0x20u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, byte_size));
}

pub unsafe fn diff_d3d_init_indirect_arg_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(new);
    for (sz, mode) in [(0x1000u64, 0u32), (0x1000, 1), (0x1000, 2), (0x50_0000, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, sz, 20, mode);
        f_n(bn.as_mut_ptr() as usize, sz, 20, mode);
        crate::diff_eq!(bo, bn, "init_indirect_arg_buffer");
    }
    Ok(4)
}

/// Original: `0x14209d010` (158 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_typed_srv_buffer(
    this: usize,
    byte_size: u64,
    fmt: u8,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    let elem_info = exe::call_va!(0x1_4204_e8d0, (u32::from(fmt)) -> u64);
    exe::call_va!(0x1_4207_5af0, (byte_size, elem_info, fmt, 1u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, byte_size));
}

pub unsafe fn diff_d3d_init_typed_srv_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u8, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u8, u32) = core::mem::transmute(new);
    for (sz, fmt, mode) in [(0x1000u64, 1u8, 0u32), (0x1000, 10, 1), (0x1000, 0x2a, 2), (0x50_0000, 1, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, sz, fmt, mode);
        f_n(bn.as_mut_ptr() as usize, sz, fmt, mode);
        crate::diff_eq!(bo, bn, "init_typed_srv_buffer");
    }
    Ok(4)
}

/// Original: `0x14209d0c0` (148 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_structured_srv_buffer(
    this: usize,
    byte_size: u64,
    stride: u64,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, stride, 0u8, 1u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, byte_size));
}

pub unsafe fn diff_d3d_init_structured_srv_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64, u32) = core::mem::transmute(new);
    for (sz, mode) in [(0x1000u64, 0u32), (0x1000, 1), (0x1000, 2), (0x50_0000, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, sz, 24, mode);
        f_n(bn.as_mut_ptr() as usize, sz, 24, mode);
        crate::diff_eq!(bo, bn, "init_structured_srv_buffer");
    }
    Ok(4)
}

/// Original: `0x14209d280` (149 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_move_construct_shader_buffer(this: usize, other: usize) -> usize {
    for off in (0..0x50).step_by(8) {
        wr::<u64>(this + off, rd::<u64>(other + off));
        wr::<u64>(other + off, 0);
    }
    let state = rd::<u32>(other + 0x50);
    wr::<u32>(other + 0x50, 0);
    wr::<u32>(this + 0x50, state);
    exe::call_va!(0x1_411e_8020, (this + 0x58, other + 0x58) -> ());
    this
}

pub unsafe fn diff_d3d_move_construct_shader_buffer(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_move_ext(dst: usize, src: usize) {
        let v = rd::<u64>(src);
        wr::<u64>(src, 0);
        wr::<u64>(dst, v);
    }
    let _m = Mock::new(0x1_411e_8020, fake_move_ext as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(new);

    let mut this_o = [0u8; 0x60];
    let mut this_n = [0u8; 0x60];
    let mut other_o = [0u8; 0x60];
    let mut other_n = [0u8; 0x60];
    for i in 0..0x60 {
        other_o[i] = (i as u8).wrapping_mul(3).wrapping_add(1);
        other_n[i] = other_o[i];
    }
    let to = this_o.as_mut_ptr() as usize;
    let tn = this_n.as_mut_ptr() as usize;
    let ro = f_o(to, other_o.as_mut_ptr() as usize);
    let rn = f_n(tn, other_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro - to, rn - tn, "move_construct_ret");
    crate::diff_eq!(&this_o[..0x54], &this_n[..0x54], "move_construct_this_head");
    crate::diff_eq!(&this_o[0x58..0x60], &this_n[0x58..0x60], "move_construct_this_ext");
    crate::diff_eq!(other_o, other_n, "move_construct_other");
    Ok(1)
}

/// Original: `0x14209d320` (144 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_ensure_staging_upload_slot(
    this: usize,
    slot_idx: usize,
    byte_size: u64,
) {
    let slot_off = slot_idx.wrapping_mul(0x50);
    let flag_addr = this + slot_idx + 0x18b;
    let slot_ptr = this + 0x50 + slot_off;
    if rd::<u8>(flag_addr) != 0 {
        if rd::<u64>(this + slot_off + 0x60) >= byte_size {
            return;
        }
        exe::call_va!(0x1_4207_5a20, (slot_ptr) -> ());
    }
    exe::call_va!(0x1_4207_5af0, (byte_size, 0u64, 0u8, 0u32, 0x10u32, 0usize, slot_ptr) -> ());
    wr::<u8>(flag_addr, 1);
}

pub unsafe fn diff_d3d_ensure_staging_upload_slot(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5a20(slot_ptr: usize) {
        wr::<u64>(slot_ptr, 0xfee1_dead);
    }
    let _m1 = Mock::new(0x1_4207_5a20, fake_5a20 as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, usize, u64) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize, u64) = core::mem::transmute(new);

    for (flag, cur_sz, req_sz) in [(0u8, 0u64, 0x1000u64), (1, 0x2000, 0x1000), (1, 0x800, 0x1000)] {
        let mut bo = [0u8; 0x1a0];
        let mut bn = [0u8; 0x1a0];
        let po = bo.as_mut_ptr() as usize;
        let pn = bn.as_mut_ptr() as usize;
        wr::<u8>(po + 1 + 0x18b, flag);
        wr::<u64>(po + 0x50 + 0x60, cur_sz);
        wr::<u8>(pn + 1 + 0x18b, flag);
        wr::<u64>(pn + 0x50 + 0x60, cur_sz);
        f_o(po, 1, req_sz);
        f_n(pn, 1, req_sz);
        crate::diff_eq!(bo, bn, "ensure_staging_upload_slot");
    }
    Ok(3)
}

/// Original: `0x14209d3c0` (83 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_ensure_readback_slot(this: usize, byte_size: u64) {
    if rd::<u8>(this + 0x18a) != 0 {
        if rd::<u64>(this + 0x10) >= byte_size {
            return;
        }
        exe::call_va!(0x1_4207_5a20, (this) -> ());
    }
    exe::call_va!(0x1_4207_5af0, (byte_size, 0u64, 0u8, 0u32, 0x20u32, 0usize, this) -> ());
    wr::<u8>(this + 0x18a, 1);
}

pub unsafe fn diff_d3d_ensure_readback_slot(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5a20(this: usize) {
        wr::<u64>(this, 0xfee1_dead);
    }
    let _m1 = Mock::new(0x1_4207_5a20, fake_5a20 as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64) = core::mem::transmute(new);

    for (flag, cur_sz, req_sz) in [(0u8, 0u64, 0x1000u64), (1, 0x2000, 0x1000), (1, 0x800, 0x1000)] {
        let mut bo = [0u8; 0x190];
        let mut bn = [0u8; 0x190];
        let po = bo.as_mut_ptr() as usize;
        let pn = bn.as_mut_ptr() as usize;
        wr::<u8>(po + 0x18a, flag);
        wr::<u64>(po + 0x10, cur_sz);
        wr::<u8>(pn + 0x18a, flag);
        wr::<u64>(pn + 0x10, cur_sz);
        f_o(po, req_sz);
        f_n(pn, req_sz);
        crate::diff_eq!(bo, bn, "ensure_readback_slot");
    }
    Ok(3)
}

/// Original: `0x14209d550` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_normalize_buffer_access_mode(mode: u32) -> u32 {
    if mode == 4 { 3 } else { mode }
}

pub unsafe fn diff_d3d_normalize_buffer_access_mode(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(u32) -> u32 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(u32) -> u32 = core::mem::transmute(new);
    for m in 0..=8u32 {
        let ro = f_o(m);
        let rn = f_n(m);
        crate::diff_eq!(ro, rn, "normalize_buffer_access_mode");
    }
    Ok(9)
}

/// Original: `0x14209d570` (61 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_compute_buffer_heap_class(
    usage: u32,
    flags: u32,
    byte_size: u64,
) -> u32 {
    if usage == 0 {
        if ((flags >> 5) & 1) != 0 {
            1
        } else if ((flags >> 4) & 1) == 0 {
            4
        } else {
            1
        }
    } else if usage == 2 && byte_size <= 0x40_0000 {
        2
    } else {
        3
    }
}

pub unsafe fn diff_d3d_compute_buffer_heap_class(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(u32, u32, u64) -> u32 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(u32, u32, u64) -> u32 = core::mem::transmute(new);
    let mut runs = 0;
    for usage in 0..=4u32 {
        for flags in [0u32, 0x10, 0x20, 0x30, 0xff] {
            for sz in [0u64, 0x1000, 0x40_0000, 0x40_0001, 0x100_0000] {
                let ro = f_o(usage, flags, sz);
                let rn = f_n(usage, flags, sz);
                crate::diff_eq!(ro, rn, "compute_buffer_heap_class");
                runs += 1;
            }
        }
    }
    Ok(runs)
}

/// Original: `0x14209d5c0` (167 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_typed_indirect_uav_srv_buffer(
    this: usize,
    byte_size: u64,
    fmt: u8,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    let elem_info = exe::call_va!(0x1_4204_e8d0, (u32::from(fmt)) -> u64);
    exe::call_va!(0x1_4207_5af0, (byte_size, elem_info, fmt, 0x31u32, 0u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, 4);
}

pub unsafe fn diff_d3d_init_typed_indirect_uav_srv_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u8) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u8) = core::mem::transmute(new);
    for fmt in [1u8, 10, 0x2a] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, 0x2000, fmt);
        f_n(bn.as_mut_ptr() as usize, 0x2000, fmt);
        crate::diff_eq!(bo, bn, "init_typed_indirect_uav_srv_buffer");
    }
    Ok(3)
}

/// Original: `0x14209d670` (115 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_release_shader_buffer(this: usize) {
    wr::<u32>(this + 0x50, 0);
    let ext = rd::<usize>(this + 0x58);
    wr::<usize>(this + 0x58, 0);
    if ext != 0 {
        if rd::<u8>(ext + 0x18a) != 0 {
            exe::call_va!(0x1_4207_5a20, (ext) -> ());
        }
        if rd::<u8>(ext + 0x18b) != 0 {
            exe::call_va!(0x1_4207_5a20, (ext + 0x50) -> ());
        }
        if rd::<u8>(ext + 0x18c) != 0 {
            exe::call_va!(0x1_4207_5a20, (ext + 0xa0) -> ());
        }
        exe::call_va!(0x1_4244_2de0, (ext) -> ());
    }
    exe::call_va!(0x1_4207_5a20, (this) -> ());
}

pub unsafe fn diff_d3d_release_shader_buffer(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5a20(ptr: usize) {
        wr::<u64>(ptr, rd::<u64>(ptr).wrapping_add(0x1111));
    }
    unsafe extern "win64" fn fake_2de0(ext: usize) {
        wr::<u64>(ext + 0x180, 0x9999);
    }
    let _m1 = Mock::new(0x1_4207_5a20, fake_5a20 as *const ())?;
    let _m2 = Mock::new(0x1_4244_2de0, fake_2de0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) = core::mem::transmute(new);

    for mask in [0u8, 1, 5, 7] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        let mut eo = [0u8; 0x190];
        let mut en = [0u8; 0x190];
        let po = bo.as_mut_ptr() as usize;
        let pn = bn.as_mut_ptr() as usize;
        wr::<u32>(po + 0x50, 4);
        wr::<u32>(pn + 0x50, 4);
        if mask != 0 {
            let epo = eo.as_mut_ptr() as usize;
            let epn = en.as_mut_ptr() as usize;
            wr::<u8>(epo + 0x18a, mask & 1);
            wr::<u8>(epo + 0x18b, (mask >> 1) & 1);
            wr::<u8>(epo + 0x18c, (mask >> 2) & 1);
            wr::<u8>(epn + 0x18a, mask & 1);
            wr::<u8>(epn + 0x18b, (mask >> 1) & 1);
            wr::<u8>(epn + 0x18c, (mask >> 2) & 1);
            wr::<usize>(po + 0x58, epo);
            wr::<usize>(pn + 0x58, epn);
        }
        f_o(po);
        f_n(pn);
        crate::diff_eq!(bo, bn, "release_shader_buffer_this");
        crate::diff_eq!(eo, en, "release_shader_buffer_ext");
    }
    Ok(4)
}

/// Original: `0x14209d930` (12 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_flush_full_shader_buffer(this: usize, flags: u32) {
    let size = rd::<u64>(this + 0x10);
    exe::call_va!(0x1_4209_c560, (this, flags as u64, size, flags) -> ());
}

pub unsafe fn diff_d3d_flush_full_shader_buffer(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_c560(this: usize, offset: u64, size: u64, flags: u32) {
        wr::<u64>(this + 0x20, offset);
        wr::<u64>(this + 0x28, size);
        wr::<u32>(this + 0x30, flags);
    }
    let _m = Mock::new(0x1_4209_c560, fake_c560 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u32) = core::mem::transmute(new);
    let mut bo = [0u8; 0x40];
    let mut bn = [0u8; 0x40];
    let po = bo.as_mut_ptr() as usize;
    let pn = bn.as_mut_ptr() as usize;
    wr::<u64>(po + 0x10, 0xabcd_0000);
    wr::<u64>(pn + 0x10, 0xabcd_0000);
    f_o(po, 7);
    f_n(pn, 7);
    crate::diff_eq!(bo, bn, "flush_full_shader_buffer");
    Ok(1)
}

/// Original: `0x14209d950` (148 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_raw_address_buffer(
    this: usize,
    byte_size: u64,
    allow_uav: u32,
    mode: u32,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    let is_one = u32::from(allow_uav == 1);
    exe::call_va!(0x1_4207_4d70, (byte_size, allow_uav, this, is_one) -> ());
    wr::<u32>(this + 0x50, resolve_buffer_residency_mode(mode, byte_size));
}

pub unsafe fn diff_d3d_init_raw_address_buffer(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_4d70(byte_size: u64, allow_uav: u32, this: usize, is_one: u32) {
        wr::<u64>(this + 0x8, byte_size);
        wr::<u32>(this + 0x10, allow_uav);
        wr::<u32>(this + 0x14, is_one);
    }
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_4d70, fake_4d70 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u32, u32) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u32, u32) = core::mem::transmute(new);
    for (sz, uav, mode) in [(0x1000u64, 0u32, 0u32), (0x1000, 1, 1), (0x1000, 2, 2), (0x50_0000, 1, 2)] {
        let mut bo = [0u8; 0x60];
        let mut bn = [0u8; 0x60];
        f_o(bo.as_mut_ptr() as usize, sz, uav, mode);
        f_n(bn.as_mut_ptr() as usize, sz, uav, mode);
        crate::diff_eq!(bo, bn, "init_raw_address_buffer");
    }
    Ok(4)
}

/// Original: `0x14209d9f0` (82 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_readback_shader_buffer(
    this: usize,
    byte_size: u64,
    stride: u64,
) {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_4207_5af0, (byte_size, stride, 0u8, 0u32, 0x20u32, 0usize, this) -> ());
    wr::<u32>(this + 0x50, 1);
}

pub unsafe fn diff_d3d_init_readback_shader_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = Mock::new(0x1_4209_d670, mock_release_shader_buffer as *const ())?;
    let _m2 = Mock::new(0x1_4207_5af0, mock_alloc_shader_buffer_5af0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u64, u64) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, u64) = core::mem::transmute(new);
    let mut bo = [0u8; 0x60];
    let mut bn = [0u8; 0x60];
    f_o(bo.as_mut_ptr() as usize, 0x4000, 4);
    f_n(bn.as_mut_ptr() as usize, 0x4000, 4);
    crate::diff_eq!(bo, bn, "init_readback_shader_buffer");
    Ok(1)
}

/// Original: `0x14209da80` (54 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_construct_empty_shader_buffer(this: usize) -> usize {
    for off in (0..0x50).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    wr::<u32>(this + 0x50, 0);
    wr::<u64>(this + 0x58, 0);
    this
}

pub unsafe fn diff_d3d_construct_empty_shader_buffer(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(new);
    let mut bo = [0xabu8; 0x60];
    let mut bn = [0xabu8; 0x60];
    let po = bo.as_mut_ptr() as usize;
    let pn = bn.as_mut_ptr() as usize;
    let ro = f_o(po);
    let rn = f_n(pn);
    crate::diff_eq!(ro - po, rn - pn, "ret_offset");
    crate::diff_eq!(&bo[..0x54], &bn[..0x54], "head");
    crate::diff_eq!(&bo[0x58..0x60], &bn[0x58..0x60], "tail");
    Ok(1)
}

/// Original: `0x14209dac0` (8 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_buddy_pool_max_order_size(this: usize) -> u16 {
    rd::<u16>(this + 0x20).wrapping_mul(2)
}

pub unsafe fn diff_d3d_buddy_pool_max_order_size(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize) -> u16 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) -> u16 = core::mem::transmute(new);
    let mut buf = [0u8; 0x30];
    let ptr = buf.as_mut_ptr() as usize;
    for v in [0u16, 1, 64, 512, 32768, 65535] {
        wr::<u16>(ptr + 0x20, v);
        let ro = f_o(ptr);
        let rn = f_n(ptr);
        crate::diff_eq!(ro, rn, "buddy_pool_max_order_size");
    }
    Ok(6)
}

/// Original: `0x14209dad0` (70 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_buddy_pool_return_node(this: usize, node: usize) {
    let mut cur = rd::<usize>(this + 0x40);
    let count = rd::<u32>(this + 0x48) as usize;
    let end = cur.wrapping_add(count.wrapping_mul(0x18));
    while cur != end {
        let block_start = rd::<usize>(cur);
        if node >= block_start {
            let block_count = rd::<u32>(cur + 0x8) as usize;
            let block_end = block_start.wrapping_add(block_count.wrapping_mul(0x28));
            if node < block_end {
                let prev_head = rd::<usize>(cur + 0x10);
                wr::<usize>(cur + 0x10, node);
                wr::<usize>(node + 0x20, prev_head);
                return;
            }
        }
        cur = cur.wrapping_add(0x18);
    }
}

pub unsafe fn diff_d3d_buddy_pool_return_node(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(new);

    let mut arena_o = [0u8; 0x200];
    let mut arena_n = [0u8; 0x200];
    let ao = arena_o.as_mut_ptr() as usize;
    let an = arena_n.as_mut_ptr() as usize;

    // Setup 2 blocks at +0x60, nodes at +0xa0
    wr::<usize>(ao + 0x40, ao + 0x60);
    wr::<u32>(ao + 0x48, 2);
    wr::<usize>(ao + 0x60, ao + 0xa0);
    wr::<u32>(ao + 0x68, 2);
    wr::<usize>(ao + 0x70, 0x1111);
    wr::<usize>(ao + 0x78, ao + 0x100);
    wr::<u32>(ao + 0x80, 2);
    wr::<usize>(ao + 0x88, 0x2222);

    wr::<usize>(an + 0x40, an + 0x60);
    wr::<u32>(an + 0x48, 2);
    wr::<usize>(an + 0x60, an + 0xa0);
    wr::<u32>(an + 0x68, 2);
    wr::<usize>(an + 0x70, 0x1111);
    wr::<usize>(an + 0x78, an + 0x100);
    wr::<u32>(an + 0x80, 2);
    wr::<usize>(an + 0x88, 0x2222);

    f_o(ao, ao + 0x128);
    f_n(an, an + 0x128);
    crate::diff_eq!(rd::<usize>(ao + 0x88) - ao, rd::<usize>(an + 0x88) - an, "head_rel");
    crate::diff_eq!(rd::<usize>(ao + 0x148), rd::<usize>(an + 0x148), "prev_head");
    Ok(2)
}

/// Original: `0x14209e050` (36 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_slist_find_midpoint(head: usize) -> usize {
    let mut slow = head;
    let mut fast = head;
    if fast != 0 {
        loop {
            fast = rd::<usize>(fast);
            slow = rd::<usize>(slow);
            if fast == 0 {
                break;
            }
            fast = rd::<usize>(fast);
            if fast == 0 {
                break;
            }
        }
    }
    slow
}

pub unsafe fn diff_d3d_slist_find_midpoint(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(new);
    let mut nodes = [0usize; 6];
    let base = nodes.as_mut_ptr() as usize;
    for i in 0..4 {
        wr::<usize>(base + i * 8, base + (i + 1) * 8);
    }
    wr::<usize>(base + 4 * 8, 0);
    for start in [0usize, base, base + 8, base + 16, base + 24, base + 32] {
        let ro = f_o(start);
        let rn = f_n(start);
        crate::diff_eq!(ro, rn, "slist_find_midpoint");
    }
    Ok(6)
}

/// Original: `0x14209e080` (43 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_slist_remove_node(head_ptr: usize, target: usize) {
    let mut prev_ptr = head_ptr;
    let mut cur = rd::<usize>(head_ptr);
    while cur != 0 {
        if cur == target {
            let next = rd::<usize>(target);
            wr::<usize>(prev_ptr, next);
            return;
        }
        let next = rd::<usize>(cur);
        prev_ptr = cur;
        cur = next;
    }
}

pub unsafe fn diff_d3d_slist_remove_node(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(new);
    let mut bo = [0usize; 4];
    let mut bn = [0usize; 4];
    let po = bo.as_mut_ptr() as usize;
    let pn = bn.as_mut_ptr() as usize;
    wr::<usize>(po, po + 8);
    wr::<usize>(po + 8, po + 16);
    wr::<usize>(po + 16, po + 24);
    wr::<usize>(po + 24, 0);

    wr::<usize>(pn, pn + 8);
    wr::<usize>(pn + 8, pn + 16);
    wr::<usize>(pn + 16, pn + 24);
    wr::<usize>(pn + 24, 0);

    f_o(po, po + 16);
    f_n(pn, pn + 16);
    crate::diff_eq!(rd::<usize>(po + 8) - po, rd::<usize>(pn + 8) - pn, "slist_remove_node");
    Ok(1)
}

/// Original: `0x14209e0c0` (63 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_slist_insert_sorted_buddy(head_ptr: usize, node: usize) {
    wr::<u8>(node + 0xe, 0);
    let mut prev_ptr = head_ptr;
    let mut cur = rd::<usize>(head_ptr);
    if cur != 0 {
        let node_k1 = rd::<u16>(node + 0xc);
        loop {
            let cur_k1 = rd::<u16>(cur + 0xc);
            if node_k1 < cur_k1 {
                break;
            }
            if node_k1 == cur_k1 && rd::<u16>(node + 0x8) < rd::<u16>(cur + 0x8) {
                break;
            }
            let next = rd::<usize>(cur);
            prev_ptr = cur;
            cur = next;
            if cur == 0 {
                break;
            }
        }
    }
    let next = rd::<usize>(prev_ptr);
    wr::<usize>(node, next);
    wr::<usize>(prev_ptr, node);
}

pub unsafe fn diff_d3d_slist_insert_sorted_buddy(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(new);
    let mut bo = [0u8; 0x80];
    let mut bn = [0u8; 0x80];
    let po = bo.as_mut_ptr() as usize;
    let pn = bn.as_mut_ptr() as usize;

    for (idx, (k1, k0)) in [(2u16, 10u16), (1, 20), (2, 5)].iter().enumerate() {
        let off = 0x10 + idx * 0x20;
        wr::<u16>(po + off + 0x8, *k0);
        wr::<u16>(po + off + 0xc, *k1);
        wr::<u8>(po + off + 0xe, 1);
        wr::<u16>(pn + off + 0x8, *k0);
        wr::<u16>(pn + off + 0xc, *k1);
        wr::<u8>(pn + off + 0xe, 1);
        f_o(po, po + off);
        f_n(pn, pn + off);
    }
    crate::diff_eq!(rd::<usize>(po) - po, rd::<usize>(pn) - pn, "head");
    crate::diff_eq!(rd::<u8>(po + 0x1e), rd::<u8>(pn + 0x1e), "flag");
    Ok(3)
}

/// Original: `0x14209e110` (28 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_buddy_node_less(a: usize, b: usize) -> u8 {
    let b_k1 = rd::<u16>(b + 0xc);
    let a_k1 = rd::<u16>(a + 0xc);
    if b_k1 < a_k1 {
        1
    } else if b_k1 > a_k1 {
        0
    } else {
        u8::from(rd::<u16>(b + 0x8) < rd::<u16>(a + 0x8))
    }
}

pub unsafe fn diff_d3d_buddy_node_less(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) -> u8 = core::mem::transmute(new);
    let mut buf = [0u8; 0x40];
    let a = buf.as_mut_ptr() as usize;
    let b = a + 0x20;
    for (ak1, ak0, bk1, bk0) in [
        (1u16, 1u16, 2u16, 1u16),
        (2, 1, 1, 1),
        (2, 5, 2, 3),
        (2, 3, 2, 5),
        (2, 5, 2, 5),
    ] {
        wr::<u16>(a + 0x8, ak0);
        wr::<u16>(a + 0xc, ak1);
        wr::<u16>(b + 0x8, bk0);
        wr::<u16>(b + 0xc, bk1);
        crate::diff_eq!(f_o(a, b), f_n(a, b), "buddy_node_less");
    }
    Ok(5)
}

/// Original: `0x14209e140` (20 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_slist_pop_buddy(head_ptr: usize) -> usize {
    let top = rd::<usize>(head_ptr);
    if top != 0 {
        let next = rd::<usize>(top);
        wr::<usize>(head_ptr, next);
        wr::<u8>(top + 0xe, 1);
    }
    top
}

pub unsafe fn diff_d3d_slist_pop_buddy(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) -> usize = core::mem::transmute(new);
    let mut bo = [0u8; 0x40];
    let mut bn = [0u8; 0x40];
    let po = bo.as_mut_ptr() as usize;
    let pn = bn.as_mut_ptr() as usize;
    wr::<usize>(po, po + 0x10);
    wr::<usize>(po + 0x10, 0x1234);
    wr::<usize>(pn, pn + 0x10);
    wr::<usize>(pn + 0x10, 0x1234);
    let ro = f_o(po);
    let rn = f_n(pn);
    crate::diff_eq!(ro - po, rn - pn, "pop_ret");
    crate::diff_eq!(&bo[..], &bn[..], "pop_state");
    Ok(2)
}

/// Original: `0x14209e160` (12 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_left_buddy(node: usize) -> u8 {
    let size = rd::<u16>(node + 0xa);
    let offset = rd::<u16>(node + 0x8);
    u8::from((offset & size) == 0)
}

pub unsafe fn diff_d3d_is_left_buddy(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) -> u8 = core::mem::transmute(new);
    let mut buf = [0u8; 0x20];
    let ptr = buf.as_mut_ptr() as usize;
    for (off, sz) in [(0u16, 4u16), (4, 4), (8, 4), (12, 4), (0, 8), (8, 8)] {
        wr::<u16>(ptr + 0x8, off);
        wr::<u16>(ptr + 0xa, sz);
        crate::diff_eq!(f_o(ptr), f_n(ptr), "is_left_buddy");
    }
    Ok(6)
}

/// Original: `0x14209e180` (29 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_ceil_log2_u16(val: u16) -> u8 {
    let v = u32::from(val);
    if v <= 1 {
        0
    } else {
        let m = v - 1;
        let bit = 31 - m.leading_zeros();
        (bit + 1) as u8
    }
}

pub unsafe fn diff_d3d_ceil_log2_u16(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(u16) -> u8 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(u16) -> u8 = core::mem::transmute(new);
    for v in [0u16, 1, 2, 3, 4, 5, 7, 8, 9, 15, 16, 17, 255, 256, 32768, 65535] {
        crate::diff_eq!(f_o(v), f_n(v), "ceil_log2_u16");
    }
    Ok(16)
}

/// Original: `0x14209e1b0` (9 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_pow2_u16(shift: u8) -> u16 {
    1u16.wrapping_shl(u32::from(shift))
}

pub unsafe fn diff_d3d_pow2_u16(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(u8) -> u16 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(u8) -> u16 = core::mem::transmute(new);
    for s in 0..16u8 {
        crate::diff_eq!(f_o(s), f_n(s), "pow2_u16");
    }
    Ok(16)
}

/// Original: `0x14209e410` (41 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_buddy_free_block(this: usize, node: usize) {
    let sz = rd::<u16>(node + 0xa) as u64;
    wr::<u64>(this + 0x50, rd::<u64>(this + 0x50).wrapping_sub(sz));
    wr::<u64>(this + 0x58, rd::<u64>(this + 0x58).wrapping_sub(1));
    let v = u32::from(rd::<u16>(node + 0xa));
    let order = if v <= 1 {
        0u8
    } else {
        let m = v - 1;
        let bit = 31 - m.leading_zeros();
        (bit + 1) as u8
    };
    exe::call_va!(0x1_4209_dc80, (this, node, order) -> ());
}

pub unsafe fn diff_d3d_buddy_free_block(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_dc80(this: usize, node: usize, order: u8) {
        wr::<u64>(this + 0x40, (node - this) as u64);
        wr::<u8>(this + 0x48, order);
    }
    let _m = Mock::new(0x1_4209_dc80, fake_dc80 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) = core::mem::transmute(new);
    for sz in [0u16, 1, 2, 3, 4, 5, 16] {
        let mut bo = [0u8; 0x80];
        let mut bn = [0u8; 0x80];
        let po = bo.as_mut_ptr() as usize;
        let pn = bn.as_mut_ptr() as usize;
        wr::<u64>(po + 0x50, 100);
        wr::<u64>(po + 0x58, 5);
        wr::<u16>(po + 0x60 + 0xa, sz);
        wr::<u64>(pn + 0x50, 100);
        wr::<u64>(pn + 0x58, 5);
        wr::<u16>(pn + 0x60 + 0xa, sz);
        f_o(po, po + 0x60);
        f_n(pn, pn + 0x60);
        crate::diff_eq!(bo, bn, "buddy_free_block");
    }
    Ok(7)
}

/// Original: `0x14209e440` (86 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_buddy_alloc_block(
    this: usize,
    req_size: u16,
    user_data: u64,
) -> usize {
    let v = u32::from(req_size);
    let order = if v <= 1 {
        0u8
    } else {
        let m = v - 1;
        let bit = 31 - m.leading_zeros();
        (bit + 1) as u8
    };
    let node = exe::call_va!(0x1_4209_dde0, (this, order) -> usize);
    if node != 0 {
        let alloc_sz = rd::<u16>(node + 0xa) as u64;
        wr::<u64>(node, user_data);
        wr::<u64>(this + 0x50, rd::<u64>(this + 0x50).wrapping_add(alloc_sz));
        wr::<u64>(this + 0x58, rd::<u64>(this + 0x58).wrapping_add(1));
    }
    node
}

pub unsafe fn diff_d3d_buddy_alloc_block(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_dde0(this: usize, order: u8) -> usize {
        wr::<u8>(this + 0x41, order);
        if rd::<u8>(this + 0x40) == 0 {
            0
        } else {
            let node = this + 0x60;
            wr::<u16>(node + 0xa, 1u16 << order);
            node
        }
    }
    let _m = Mock::new(0x1_4209_dde0, fake_dde0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, u16, u64) -> usize = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u16, u64) -> usize = core::mem::transmute(new);
    for (avail, req) in [(0u8, 4u16), (1, 1), (1, 2), (1, 3), (1, 8)] {
        let mut bo = [0u8; 0x80];
        let mut bn = [0u8; 0x80];
        let po = bo.as_mut_ptr() as usize;
        let pn = bn.as_mut_ptr() as usize;
        wr::<u8>(po + 0x40, avail);
        wr::<u8>(pn + 0x40, avail);
        let ro = f_o(po, req, 0x1234_5678);
        let rn = f_n(pn, req, 0x1234_5678);
        crate::diff_eq!(ro == 0, rn == 0, "buddy_alloc_null");
        if ro != 0 {
            crate::diff_eq!(ro - po, rn - pn, "buddy_alloc_ret");
        }
        crate::diff_eq!(bo, bn, "buddy_alloc_state");
    }
    Ok(5)
}

/// Original: `0x14209e4a0` (35 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_destroy_buddy_allocator(this: usize) {
    exe::call_va!(0x1_402c_30b0, (this + 0x40) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x30) -> ());
}

pub unsafe fn diff_d3d_destroy_buddy_allocator(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_30b0(p: usize) {
        wr::<u64>(p, 0xaaaa);
    }
    unsafe extern "win64" fn fake_e830(p: usize) {
        wr::<u64>(p, 0xbbbb);
    }
    let _m1 = Mock::new(0x1_402c_30b0, fake_30b0 as *const ())?;
    let _m2 = Mock::new(0x1_401d_e830, fake_e830 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) = core::mem::transmute(new);
    let mut bo = [0u8; 0x60];
    let mut bn = [0u8; 0x60];
    f_o(bo.as_mut_ptr() as usize);
    f_n(bn.as_mut_ptr() as usize);
    crate::diff_eq!(bo, bn, "destroy_buddy_allocator");
    Ok(1)
}

/// Original: `0x14209e4d0` (239 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_construct_buddy_allocator(this: usize, cfg: usize) -> usize {
    wr::<u64>(this, rd::<u64>(cfg));
    wr::<u64>(this + 0x8, rd::<u64>(cfg + 0x8));
    wr::<u64>(this + 0x10, rd::<u64>(cfg + 0x10));
    wr::<u64>(this + 0x18, rd::<u64>(cfg + 0x18));
    let max_units = rd::<u16>(cfg + 0x20);
    wr::<u16>(this + 0x20, max_units);
    wr::<u64>(this + 0x28, rd::<u64>(cfg + 0x28));
    wr::<u64>(this + 0x30, 0);
    wr::<u64>(this + 0x38, 0);
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0x50, 0);
    wr::<u64>(this + 0x58, 0);

    let doubled = (u32::from(max_units)).wrapping_mul(2);
    let num_orders = if doubled <= 1 {
        0u32
    } else {
        (31 - (doubled - 1).leading_zeros()) + 1
    };
    let req = num_orders as usize;
    let old_len = rd::<u32>(this + 0x38) as usize;
    let old_cap = rd::<u32>(this + 0x3c) as usize;
    if req > old_cap {
        let grow = old_cap + (old_cap >> 1);
        let new_cap = if grow < req { req } else { grow };
        exe::call_va!(0x1_4087_46b0, (this + 0x30, new_cap) -> ());
        let cur_len = rd::<u32>(this + 0x38) as usize;
        let data_ptr = rd::<usize>(this + 0x30) + cur_len * 8;
        let diff = req - old_len;
        if diff != 0 {
            exe::call_va!(0x1_4183_acc2, (data_ptr, 0u32, diff * 8) -> usize);
        }
        wr::<u32>(this + 0x38, num_orders);
    } else if req > old_len {
        let data_ptr = rd::<usize>(this + 0x30) + old_len * 8;
        let diff = req - old_len;
        exe::call_va!(0x1_4183_acc2, (data_ptr, 0u32, diff * 8) -> usize);
        wr::<u32>(this + 0x38, num_orders);
    } else if req < old_len {
        wr::<u32>(this + 0x38, num_orders);
    }
    this
}

pub unsafe fn diff_d3d_construct_buddy_allocator(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_realloc(vec_ptr: usize, new_cap: usize) {
        wr::<usize>(vec_ptr, vec_ptr + 0x50);
        wr::<u32>(vec_ptr + 0xc, new_cap as u32);
    }
    let _m = Mock::new(0x1_4087_46b0, fake_realloc as *const ())?;
    let f_o: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize) -> usize = core::mem::transmute(new);

    for max_units in [0u16, 1, 16] {
        let mut ao = [0xccu8; 0x100];
        let mut an = [0xccu8; 0x100];
        let mut cfg = [0u8; 0x30];
        let cp = cfg.as_mut_ptr() as usize;
        wr::<u64>(cp, 11);
        wr::<u64>(cp + 0x8, 22);
        wr::<u64>(cp + 0x10, 33);
        wr::<u64>(cp + 0x18, 44);
        wr::<u16>(cp + 0x20, max_units);
        wr::<u64>(cp + 0x28, 55);

        let po = ao.as_mut_ptr() as usize;
        let pn = an.as_mut_ptr() as usize;
        let ro = f_o(po, cp);
        let rn = f_n(pn, cp);
        crate::diff_eq!(ro - po, rn - pn, "construct_buddy_ret");
        crate::diff_eq!(&ao[..0x22], &an[..0x22], "construct_buddy_head");
        crate::diff_eq!(&ao[0x28..0x30], &an[0x28..0x30], "construct_buddy_28");
        crate::diff_eq!(&ao[0x38..0x100], &an[0x38..0x100], "construct_buddy_tail");
    }
    Ok(3)
}

/// Original: `0x14209e6f0` (98 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unbind_tls_shader_state_if_current(state_ptr: usize) {
    let tls0 = tls_slot0();
    let base = tls0.wrapping_add(0x467e0);
    let cur = rd::<usize>(base + 0xaf8);
    if cur == state_ptr && cur != 0 {
        let fallback = rd::<usize>(exe::ptr::<usize>(0x1_4397_fd30) as usize);
        wr::<usize>(base + 0xaf8, fallback);
        wr::<u8>(base + 0xa80, 0);
    }
}

pub unsafe fn diff_d3d_unbind_tls_shader_state_if_current(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let snap = Snapshot::take(exe::ptr::<u8>(0x1_4397_fd30) as *mut u8, 8);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fd30) as usize, 0x7777_8888);

    let f_o: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) = core::mem::transmute(new);

    let mut tls_o = vec![0u8; 0x47400];
    let mut tls_n = vec![0u8; 0x47400];
    let to = tls_o.as_mut_ptr() as usize;
    let tn = tls_n.as_mut_ptr() as usize;
    let _g = TlsSlot0Guard::install(to);

    let mut res = Ok(3);
    for (cur, arg) in [(0x1234usize, 0x1234usize), (0x1234, 0x5678), (0, 0)] {
        wr::<usize>(to + 0x467e0 + 0xaf8, cur);
        wr::<u8>(to + 0x467e0 + 0xa80, 1);
        wr::<usize>(tn + 0x467e0 + 0xaf8, cur);
        wr::<u8>(tn + 0x467e0 + 0xa80, 1);
        wr::<u8>(tn + 0x70, 1);

        wr::<usize>(exe::tls_pointer(), to);
        f_o(arg);
        wr::<usize>(exe::tls_pointer(), tn);
        f_n(arg);

        if tls_o[0x467e0 + 0xa80..0x467e0 + 0xb00] != tls_n[0x467e0 + 0xa80..0x467e0 + 0xb00] {
            res = Err("unbind_tls_shader_state_if_current mismatch".to_string());
            break;
        }
    }
    snap.restore();
    res
}

/// Original: `0x14209f630` (8 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_scale_shader_byte_count(_this: usize, count: u8) -> u16 {
    (u16::from(count)) << 3
}

pub unsafe fn diff_d3d_scale_shader_byte_count(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, u8) -> u16 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u8) -> u16 = core::mem::transmute(new);
    for c in [0u8, 1, 2, 15, 32, 128, 255] {
        crate::diff_eq!(f_o(0, c), f_n(0, c), "scale_shader_byte_count");
    }
    Ok(7)
}

/// Original: `0x14209f880` (48 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_accumulate_single_tex_size(
    _this: usize,
    accum: i64,
    slot_ptr: usize,
) -> i64 {
    let tex = rd::<usize>(slot_ptr);
    if tex != 0 {
        let b24 = u32::from(rd::<u8>(tex + 0x18));
        let d4 = rd::<u32>(tex + 0x4);
        let d0 = rd::<u32>(tex);
        let sz = exe::call_va!(0x1_4204_e7f0, (d0, d4, b24) -> i32) as i64;
        accum.wrapping_add(sz)
    } else {
        accum
    }
}

pub unsafe fn diff_d3d_accumulate_single_tex_size(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, i64, usize) -> i64 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, i64, usize) -> i64 = core::mem::transmute(new);
    let mut tex = [0u8; 0x20];
    let tp = tex.as_mut_ptr() as usize;
    wr::<u32>(tp, 64);
    wr::<u32>(tp + 0x4, 32);
    wr::<u8>(tp + 0x18, 1);

    let mut slot = 0usize;
    let sp = (&mut slot as *mut usize) as usize;
    crate::diff_eq!(f_o(0, 100, sp), f_n(0, 100, sp), "accum_null_tex");
    wr::<usize>(sp, tp);
    crate::diff_eq!(f_o(0, 100, sp), f_n(0, 100, sp), "accum_valid_tex");
    Ok(2)
}

/// Original: `0x14209f8c0` (68 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sum_three_tex_slot_sizes(
    _this: usize,
    _unused: usize,
    slots_base: usize,
) -> i64 {
    let mut total = 0i64;
    let mut cur = slots_base;
    let end = slots_base.wrapping_add(0x30);
    while cur != end {
        let tex = rd::<usize>(cur);
        if tex != 0 {
            let b24 = u32::from(rd::<u8>(tex + 0x18));
            let d4 = rd::<u32>(tex + 0x4);
            let d0 = rd::<u32>(tex);
            let sz = exe::call_va!(0x1_4204_e7f0, (d0, d4, b24) -> i32) as i64;
            total = total.wrapping_add(sz);
        }
        cur = cur.wrapping_add(0x10);
    }
    total
}

pub unsafe fn diff_d3d_sum_three_tex_slot_sizes(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, usize, usize) -> i64 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, usize, usize) -> i64 = core::mem::transmute(new);
    let mut tex1 = [0u8; 0x20];
    let mut tex2 = [0u8; 0x20];
    let t1 = tex1.as_mut_ptr() as usize;
    let t2 = tex2.as_mut_ptr() as usize;
    wr::<u32>(t1, 16);
    wr::<u32>(t1 + 0x4, 16);
    wr::<u8>(t1 + 0x18, 1);
    wr::<u32>(t2, 32);
    wr::<u32>(t2 + 0x4, 64);
    wr::<u8>(t2 + 0x18, 2);

    let mut slots = [0u8; 0x30];
    let sb = slots.as_mut_ptr() as usize;
    wr::<usize>(sb, t1);
    wr::<usize>(sb + 0x10, 0);
    wr::<usize>(sb + 0x20, t2);
    crate::diff_eq!(f_o(0, 0, sb), f_n(0, 0, sb), "sum_three_tex_slot_sizes");
    Ok(1)
}

/// Original: `0x14209f910` (19 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sum_shader_heap_footprints(
    _this: usize,
    base_acc: u64,
    entry: usize,
) -> u64 {
    rd::<u64>(entry + 0xd0)
        .wrapping_add(rd::<u64>(entry + 0x70))
        .wrapping_add(rd::<u64>(entry + 0x10))
        .wrapping_add(base_acc)
}

pub unsafe fn diff_d3d_sum_shader_heap_footprints(orig: usize, new: usize) -> Result<u32, String> {
    let f_o: unsafe extern "win64" fn(usize, u64, usize) -> u64 = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize, u64, usize) -> u64 = core::mem::transmute(new);
    let mut buf = [0u8; 0xe0];
    let ptr = buf.as_mut_ptr() as usize;
    wr::<u64>(ptr + 0x10, 1000);
    wr::<u64>(ptr + 0x70, 2500);
    wr::<u64>(ptr + 0xd0, 5000);
    for acc in [0u64, 42, 0x10_0000] {
        crate::diff_eq!(f_o(0, acc, ptr), f_n(0, acc, ptr), "sum_shader_heap_footprints");
    }
    Ok(3)
}

/// Original: `0x14209fa30` (68 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_update_active_frame_fence_pair(this: usize) {
    exe::call_va!(0x1_4207_82a0, (this) -> ());
    let mut pair = [0u64; 2];
    let pair_ptr = pair.as_mut_ptr() as usize;
    let src = exe::call_va!(0x1_4207_34e0, (pair_ptr, 1u32) -> usize);
    let lo = rd::<u64>(src);
    let hi = rd::<u64>(src + 0x8);
    let frame_idx = rd::<u8>(this + 0x25c) as usize;
    let dst = rd::<usize>(this + 0xc0).wrapping_add(frame_idx.wrapping_mul(0x2b0));
    wr::<u64>(dst + 0x2a0, lo);
    wr::<u64>(dst + 0x2a8, hi);
}

pub unsafe fn diff_d3d_update_active_frame_fence_pair(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_82a0(this: usize) {
        wr::<u64>(this + 0x10, 0x7777);
    }
    unsafe extern "win64" fn fake_34e0(pair_ptr: usize, kind: u32) -> usize {
        wr::<u64>(pair_ptr, 0x1122_3344_5566_7788);
        wr::<u64>(pair_ptr + 0x8, 0x99aa_bbcc_ddee_0000u64.wrapping_add(kind as u64));
        pair_ptr
    }
    let _m1 = Mock::new(0x1_4207_82a0, fake_82a0 as *const ())?;
    let _m2 = Mock::new(0x1_4207_34e0, fake_34e0 as *const ())?;
    let f_o: unsafe extern "win64" fn(usize) = core::mem::transmute(orig);
    let f_n: unsafe extern "win64" fn(usize) = core::mem::transmute(new);

    let mut this_o = [0u8; 0x260];
    let mut this_n = [0u8; 0x260];
    let mut frames_o = [0u8; 0x560];
    let mut frames_n = [0u8; 0x560];
    let po = this_o.as_mut_ptr() as usize;
    let pn = this_n.as_mut_ptr() as usize;
    wr::<usize>(po + 0xc0, frames_o.as_mut_ptr() as usize);
    wr::<u8>(po + 0x25c, 1);
    wr::<usize>(pn + 0xc0, frames_n.as_mut_ptr() as usize);
    wr::<u8>(pn + 0x25c, 1);

    f_o(po);
    f_n(pn);
    crate::diff_eq!(rd::<u64>(po + 0x10), rd::<u64>(pn + 0x10), "fence_pair_this");
    crate::diff_eq!(frames_o, frames_n, "fence_pair_frames");
    Ok(1)
}

// REGISTRY: Replacement::verified(0x1_4209_ac20, "d3d::d3d_is_tls_shader_slot_empty_14209ac20", rend::d3d_shader_bind::d3d_is_tls_shader_slot_empty as *const (), rend::d3d_shader_bind::diff_d3d_is_tls_shader_slot_empty),
// REGISTRY: Replacement::verified(0x1_4209_afe0, "d3d::d3d_dec_tls_shader_ref_14209afe0", rend::d3d_shader_bind::d3d_dec_tls_shader_ref as *const (), rend::d3d_shader_bind::diff_d3d_dec_tls_shader_ref),
// REGISTRY: Replacement::verified_throws(0x1_4209_b020, "d3d::d3d_fetch_tls_shader_slot_14209b020", rend::d3d_shader_bind::d3d_fetch_tls_shader_slot as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_fetch_tls_shader_slot),
// REGISTRY: Replacement::verified_throws(0x1_4209_b3f0, "d3d::d3d_deserialize_shader_refl_14209b3f0", rend::d3d_shader_bind::d3d_deserialize_shader_refl as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_deserialize_shader_refl),
// REGISTRY: Replacement::verified(0x1_4209_b6f0, "d3d::d3d_deserialize_shader_param_14209b6f0", rend::d3d_shader_bind::d3d_deserialize_shader_param as *const (), rend::d3d_shader_bind::diff_d3d_deserialize_shader_param),
// REGISTRY: Replacement::verified(0x1_4209_b980, "d3d::d3d_get_tls_shader_cbv_slice_14209b980", rend::d3d_shader_bind::d3d_get_tls_shader_cbv_slice as *const (), rend::d3d_shader_bind::diff_d3d_get_tls_shader_cbv_slice),
// REGISTRY: Replacement::verified_throws(0x1_4209_ba00, "d3d::d3d_dispatch_shader_refl_stage_14209ba00", rend::d3d_shader_bind::d3d_dispatch_shader_refl_stage as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_dispatch_shader_refl_stage),
// REGISTRY: Replacement::verified(0x1_4209_bad0, "d3d::d3d_clear_tls_shader_slots_14209bad0", rend::d3d_shader_bind::d3d_clear_tls_shader_slots as *const (), rend::d3d_shader_bind::diff_d3d_clear_tls_shader_slots),
// REGISTRY: Replacement::verified_throws(0x1_4209_c0a0, "d3d::d3d_reinit_view_typed_14209c0a0", rend::d3d_shader_bind::d3d_reinit_view_typed as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_reinit_view_typed),
// REGISTRY: Replacement::verified_throws(0x1_4209_c110, "d3d::d3d_reinit_view_raw_14209c110", rend::d3d_shader_bind::d3d_reinit_view_raw as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_reinit_view_raw),
// REGISTRY: Replacement::verified_throws(0x1_4209_c940, "d3d::d3d_init_cbv_buffer_14209c940", rend::d3d_shader_bind::d3d_init_cbv_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_cbv_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_c9a0, "d3d::d3d_init_vertex_buffer_14209c9a0", rend::d3d_shader_bind::d3d_init_vertex_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_vertex_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_ca70, "d3d::d3d_init_index_buffer_14209ca70", rend::d3d_shader_bind::d3d_init_index_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_index_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_cb40, "d3d::d3d_init_r32u_srv_buffer_14209cb40", rend::d3d_shader_bind::d3d_init_r32u_srv_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_r32u_srv_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_cba0, "d3d::d3d_init_structured_srv_upload_buffer_14209cba0", rend::d3d_shader_bind::d3d_init_structured_srv_upload_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_structured_srv_upload_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_cc00, "d3d::d3d_init_rt_accel_buffer_14209cc00", rend::d3d_shader_bind::d3d_init_rt_accel_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_rt_accel_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_ccd0, "d3d::d3d_init_uav_only_buffer_14209ccd0", rend::d3d_shader_bind::d3d_init_uav_only_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_uav_only_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_cd70, "d3d::d3d_init_indirect_uav_srv_buffer_14209cd70", rend::d3d_shader_bind::d3d_init_indirect_uav_srv_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_indirect_uav_srv_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_ce00, "d3d::d3d_init_rgba32u_uav_srv_buffer_14209ce00", rend::d3d_shader_bind::d3d_init_rgba32u_uav_srv_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_rgba32u_uav_srv_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_cea0, "d3d::d3d_init_uav_srv_buffer_14209cea0", rend::d3d_shader_bind::d3d_init_uav_srv_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_uav_srv_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_cf70, "d3d::d3d_init_indirect_arg_buffer_14209cf70", rend::d3d_shader_bind::d3d_init_indirect_arg_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_indirect_arg_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_d010, "d3d::d3d_init_typed_srv_buffer_14209d010", rend::d3d_shader_bind::d3d_init_typed_srv_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_typed_srv_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_d0c0, "d3d::d3d_init_structured_srv_buffer_14209d0c0", rend::d3d_shader_bind::d3d_init_structured_srv_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_structured_srv_buffer),
// REGISTRY: Replacement::verified(0x1_4209_d280, "d3d::d3d_move_construct_shader_buffer_14209d280", rend::d3d_shader_bind::d3d_move_construct_shader_buffer as *const (), rend::d3d_shader_bind::diff_d3d_move_construct_shader_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_d320, "d3d::d3d_ensure_staging_upload_slot_14209d320", rend::d3d_shader_bind::d3d_ensure_staging_upload_slot as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_ensure_staging_upload_slot),
// REGISTRY: Replacement::verified_throws(0x1_4209_d3c0, "d3d::d3d_ensure_readback_slot_14209d3c0", rend::d3d_shader_bind::d3d_ensure_readback_slot as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_ensure_readback_slot),
// REGISTRY: Replacement::verified_throws(0x1_4209_d5c0, "d3d::d3d_init_typed_indirect_uav_srv_buffer_14209d5c0", rend::d3d_shader_bind::d3d_init_typed_indirect_uav_srv_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_typed_indirect_uav_srv_buffer),
// REGISTRY: Replacement::verified(0x1_4209_d670, "d3d::d3d_release_shader_buffer_14209d670", rend::d3d_shader_bind::d3d_release_shader_buffer as *const (), rend::d3d_shader_bind::diff_d3d_release_shader_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_d930, "d3d::d3d_flush_full_shader_buffer_14209d930", rend::d3d_shader_bind::d3d_flush_full_shader_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_flush_full_shader_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_d950, "d3d::d3d_init_raw_address_buffer_14209d950", rend::d3d_shader_bind::d3d_init_raw_address_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_raw_address_buffer),
// REGISTRY: Replacement::verified_throws(0x1_4209_d9f0, "d3d::d3d_init_readback_shader_buffer_14209d9f0", rend::d3d_shader_bind::d3d_init_readback_shader_buffer as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_init_readback_shader_buffer),
// REGISTRY: Replacement::verified(0x1_4209_e410, "d3d::d3d_buddy_free_block_14209e410", rend::d3d_shader_bind::d3d_buddy_free_block as *const (), rend::d3d_shader_bind::diff_d3d_buddy_free_block),
// REGISTRY: Replacement::verified(0x1_4209_e440, "d3d::d3d_buddy_alloc_block_14209e440", rend::d3d_shader_bind::d3d_buddy_alloc_block as *const (), rend::d3d_shader_bind::diff_d3d_buddy_alloc_block),
// REGISTRY: Replacement::verified(0x1_4209_e4a0, "d3d::d3d_destroy_buddy_allocator_14209e4a0", rend::d3d_shader_bind::d3d_destroy_buddy_allocator as *const (), rend::d3d_shader_bind::diff_d3d_destroy_buddy_allocator),
// REGISTRY: Replacement::verified(0x1_4209_e4d0, "d3d::d3d_construct_buddy_allocator_14209e4d0", rend::d3d_shader_bind::d3d_construct_buddy_allocator as *const (), rend::d3d_shader_bind::diff_d3d_construct_buddy_allocator),
// REGISTRY: Replacement::verified(0x1_4209_e6f0, "d3d::d3d_unbind_tls_shader_state_if_current_14209e6f0", rend::d3d_shader_bind::d3d_unbind_tls_shader_state_if_current as *const (), rend::d3d_shader_bind::diff_d3d_unbind_tls_shader_state_if_current),
// REGISTRY: Replacement::verified(0x1_4209_f880, "d3d::d3d_accumulate_single_tex_size_14209f880", rend::d3d_shader_bind::d3d_accumulate_single_tex_size as *const (), rend::d3d_shader_bind::diff_d3d_accumulate_single_tex_size),
// REGISTRY: Replacement::verified(0x1_4209_f8c0, "d3d::d3d_sum_three_tex_slot_sizes_14209f8c0", rend::d3d_shader_bind::d3d_sum_three_tex_slot_sizes as *const (), rend::d3d_shader_bind::diff_d3d_sum_three_tex_slot_sizes),
// REGISTRY: Replacement::verified_throws(0x1_4209_fa30, "d3d::d3d_update_active_frame_fence_pair_14209fa30", rend::d3d_shader_bind::d3d_update_active_frame_fence_pair as *const (), Throws::Discard, rend::d3d_shader_bind::diff_d3d_update_active_frame_fence_pair),
