use crate::exe::{self, rd, wr};
use std::sync::atomic::{AtomicI32, Ordering};

/// Helper to release the command context reference at `[rsp + 0x20]` used by barrier functions.
#[inline]
unsafe fn release_cmd_ctx_ref(ctx_ref: usize) {
    if ctx_ref != 0 {
        let count_ptr = (ctx_ref + 0xc) as *const AtomicI32;
        if (*count_ptr).fetch_sub(1, Ordering::AcqRel) == 1 {
            let state_ptr = (ctx_ref + 0x8) as *const AtomicI32;
            (*state_ptr).swap(-1, Ordering::AcqRel);
            exe::call_iat!(0x1_4264_5598, (ctx_ref) -> ());
        }
    }
}

/// Original: `0x142072710` (14 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072710(this: usize, a2: i32, a3: i32) -> u64 {
    let idx = a3.wrapping_add(a2.wrapping_mul(3)) as isize;
    let addr = (this as isize)
        .wrapping_add(idx.wrapping_mul(8))
        .wrapping_add(0x28) as usize;
    rd::<u64>(addr)
}

pub unsafe fn diff_d3d_unknown_142072710(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, i32, i32) -> u64;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut buf = [0u8; 512];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }
    let base = buf.as_ptr() as usize + 64;

    let mut cases = 0u32;
    for a2 in [-2, -1, 0, 1, 2, 3] {
        for a3 in [0, 1, 2] {
            let o = orig_fn(base, a2, a3);
            let n = new_fn(base, a2, a3);
            crate::diff_eq!(o, n, "d3d_unknown_142072710");
            cases += 1;
        }
    }
    Ok(cases)
}

/// Original: `0x142072910` (37 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072910(a1: usize, a2: u8) -> u32 {
    let mut param = [0u32; 2];
    wr::<u32>(param.as_mut_ptr() as usize, 0x10008);
    wr::<u32>(param.as_mut_ptr() as usize + 4, (a2 as u32) << 2);
    exe::call_va!(0x1_4000_16d0, (a1, param.as_mut_ptr() as usize) -> u32)
}

/// Original: `0x142072940` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072940(a1: usize) -> u8 {
    let flag_addr = exe::ptr::<u8>(0x1_43a2_8c3d) as usize;
    if rd::<u8>(flag_addr) == 0 {
        return 0;
    }
    let mut val: u32 = 0;
    let val_ptr = (&mut val as *mut u32) as usize;
    let status = exe::call_va!(0x1_4000_15e0, (a1, 3u32, val_ptr, 4u32) -> i32);
    if status == 0 && (rd::<u8>(val_ptr) & 1) != 0 {
        1
    } else {
        0
    }
}

/// Original: `0x142072990` (93 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072990(_a1: usize) -> u8 {
    let flag_addr = exe::ptr::<u8>(0x1_43a2_8c3d) as usize;
    if rd::<u8>(flag_addr) == 0 {
        return 0;
    }
    let mut out_ver: u32 = 0;
    let mut scratch = [0u8; 64];
    let out_ver_ptr = (&mut out_ver as *mut u32) as usize;
    let status = exe::call_va!(0x1_4000_1430, (out_ver_ptr, scratch.as_mut_ptr() as usize) -> i32);
    let ver = if status == 0 { rd::<u32>(out_ver_ptr) } else { 0 };
    u8::from(ver >= 0xd538)
}

/// Original: `0x142072a30` (23 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072a30(a1: usize) {
    let status = exe::call_va!(0x1_4000_1940, (a1) -> i32);
    let flag_addr = exe::ptr::<u8>(0x1_43a2_8c3d) as usize;
    wr::<u8>(flag_addr, u8::from(status == 0));
}

/// Original: `0x142072a60` (6 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072a60(a1: u32, a2: u32) -> u32 {
    a2.wrapping_add(a1.wrapping_mul(3))
}

pub unsafe fn diff_d3d_unknown_142072a60(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(u32, u32) -> u32;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut cases = 0u32;
    for a1 in [0u32, 1, 2, 7, 100, 0x5555_5555, u32::MAX] {
        for a2 in [0u32, 1, 3, 42, 0x8000_0000, u32::MAX] {
            let o = orig_fn(a1, a2);
            let n = new_fn(a1, a2);
            crate::diff_eq!(o, n, "d3d_unknown_142072a60");
            cases += 1;
        }
    }
    Ok(cases)
}

/// Original: `0x142072a70` (86 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072a70(this: usize) {
    if rd::<usize>(this + 0x8) != 0 {
        let tls0 = rd::<usize>(exe::tls_pointer());
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        let tls_ctx = rd::<usize>(tls0 + 0x6328);
        exe::call_va!(0x1_4206_df80, (tls_ctx) -> ());
        let val = rd::<u64>(this);
        let obj = rd::<usize>(this + 0x8);
        exe::call_va!(0x1_4207_02c0, (obj, val) -> ());
    }
}

/// Original: `0x142072ad0` (163 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_142072ad0(_this: usize, out: usize, desc: usize) -> usize {
    let global_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let mut local_desc = [0u64; 2];
    local_desc[0] = rd::<u64>(desc);
    local_desc[1] = rd::<u64>(desc + 0x8);

    let mut created_obj: usize = 0;
    let created_obj_ptr = (&mut created_obj as *mut usize) as usize;
    let d3d_dev = rd::<usize>(global_dev);
    let vtbl = rd::<usize>(d3d_dev);
    let fn_ptr = rd::<usize>(vtbl + 0x178);
    let riid = exe::ptr::<u8>(0x1_433d_cb10) as usize;

    let hr = exe::call_va!(
        fn_ptr,
        (d3d_dev, local_desc.as_mut_ptr() as usize, riid, created_obj_ptr) -> i32
    );
    if hr >= 0 {
        wr::<usize>(out, rd::<usize>(created_obj_ptr));
        return out;
    }

    let hr_u = hr as u32;
    if hr_u == 0x8007_000e || hr_u.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr_u) -> ());
    }
    wr::<usize>(out, 0);
    let obj = rd::<usize>(created_obj_ptr);
    if obj != 0 {
        let obj_vtbl = rd::<usize>(obj);
        let release_fn = rd::<usize>(obj_vtbl + 0x10);
        exe::call_va!(release_fn, (obj) -> ());
    }
    out
}

/// Original: `0x142072b80` (138 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_142072b80(_this: usize, out: usize, desc: usize) -> usize {
    let global_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let mut local_desc = [0u64; 6];
    local_desc[0] = rd::<u64>(global_dev + 0x30);
    local_desc[1] = rd::<u64>(desc + 0x8);
    local_desc[2] = rd::<u64>(desc);

    wr::<usize>(out, 0);
    let d3d_dev = rd::<usize>(global_dev);
    let vtbl = rd::<usize>(d3d_dev);
    let fn_ptr = rd::<usize>(vtbl + 0x58);
    let riid = exe::ptr::<u8>(0x1_433d_cb10) as usize;

    let hr = exe::call_va!(
        fn_ptr,
        (d3d_dev, local_desc.as_mut_ptr() as usize, riid, out, 1u32) -> i32
    );
    let hr_u = hr as u32;
    if hr_u == 0x8007_000e || hr_u.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr_u) -> ());
    }
    out
}

/// Original: `0x142072c10` (26 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072c10(out: usize) -> usize {
    let global_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    wr::<u64>(out, rd::<u64>(global_dev + 0x40));
    wr::<u64>(out + 0x8, rd::<u64>(global_dev + 0x38));
    out
}

/// Original: `0x142072de0` (151 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072de0(
    this: usize,
    slot: i32,
    buf_info: usize,
    offset: u64,
    stride: u32,
) {
    let slot_isize = slot as isize;
    let entry = (this as isize).wrapping_add((slot_isize + 0xb3) << 4) as usize;
    if buf_info != 0 {
        let gpu_va = rd::<u64>(buf_info).wrapping_add(offset);
        let size = rd::<u32>(buf_info + 0x10).wrapping_sub(offset as u32);
        let count_addr = this + 0xb28;
        let cur_count = rd::<i32>(count_addr);
        if slot < cur_count
            && rd::<u64>(entry) == gpu_va
            && rd::<u32>(entry + 0x8) == size
            && rd::<u32>(entry + 0xc) == stride
        {
            return;
        }
        wr::<u8>(this + 0xb2c, 1);
        let min_count = slot.wrapping_add(1);
        let new_count = if cur_count >= min_count {
            cur_count
        } else {
            min_count
        };
        wr::<i32>(count_addr, new_count);
        wr::<u64>(entry, gpu_va);
        wr::<u32>(entry + 0x8, size);
        wr::<u32>(entry + 0xc, stride);
    } else if rd::<u64>(entry) != 0 {
        wr::<u8>(this + 0xb2c, 1);
        wr::<u64>(entry, 0);
        wr::<u64>(entry + 0x8, 0);
    }
}

pub unsafe fn diff_d3d_unknown_142072de0(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, i32, usize, u64, u32);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut state_o = vec![0u8; 0xd00];
    let mut state_n = vec![0u8; 0xd00];
    let mut buf_info = [0u8; 32];
    wr::<u64>(buf_info.as_mut_ptr() as usize, 0x1000_2000_3000_4000);
    wr::<u32>(buf_info.as_mut_ptr() as usize + 0x10, 0x8000);
    let info_ptr = buf_info.as_ptr() as usize;

    let mut cases = 0u32;

    // 1. Bind non-null buffer to slot 2
    orig_fn(state_o.as_mut_ptr() as usize, 2, info_ptr, 0x100, 16);
    new_fn(state_n.as_mut_ptr() as usize, 2, info_ptr, 0x100, 16);
    crate::diff_eq!(state_o, state_n, "d3d_unknown_142072de0 bind slot 2");
    cases += 1;

    // 2. Redundant bind (no-op after clearing dirty byte)
    state_o[0xb2c] = 0;
    state_n[0xb2c] = 0;
    orig_fn(state_o.as_mut_ptr() as usize, 2, info_ptr, 0x100, 16);
    new_fn(state_n.as_mut_ptr() as usize, 2, info_ptr, 0x100, 16);
    crate::diff_eq!(state_o, state_n, "d3d_unknown_142072de0 redundant bind");
    cases += 1;

    // 3. Unbind slot 2 (null buf_info)
    state_o[0xb2c] = 0;
    state_n[0xb2c] = 0;
    orig_fn(state_o.as_mut_ptr() as usize, 2, 0, 0, 0);
    new_fn(state_n.as_mut_ptr() as usize, 2, 0, 0, 0);
    crate::diff_eq!(state_o, state_n, "d3d_unknown_142072de0 unbind slot 2");
    cases += 1;

    // 4. Redundant unbind (already zero)
    state_o[0xb2c] = 0;
    state_n[0xb2c] = 0;
    orig_fn(state_o.as_mut_ptr() as usize, 2, 0, 0, 0);
    new_fn(state_n.as_mut_ptr() as usize, 2, 0, 0, 0);
    crate::diff_eq!(state_o, state_n, "d3d_unknown_142072de0 redundant unbind");
    cases += 1;

    Ok(cases)
}

/// Original: `0x142072e80` (53 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142072e80() -> u64 {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let tls_ctx = rd::<usize>(tls0 + 0x6328);
    exe::call_va!(0x1_4206_df80, (tls_ctx) -> u64)
}

/// Original: `0x142073120` (69 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142073120(this: usize, state: usize) {
    let cur = rd::<usize>(this + 0xb10);
    if cur != 0 {
        if state == 0 {
            return;
        }
        if rd::<u32>(cur) == rd::<u32>(state)
            && rd::<f32>(cur + 0x4) == rd::<f32>(state + 0x4)
            && rd::<f32>(cur + 0x8) == rd::<f32>(state + 0x8)
        {
            return;
        }
    }
    wr::<usize>(this + 0xb10, state);
    wr::<u8>(this + 0xa80, 0);
}

pub unsafe fn diff_d3d_unknown_142073120(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut ctx_o = vec![0u8; 0xb20];
    let mut ctx_n = vec![0u8; 0xb20];
    let mut s1 = [0u8; 16];
    let mut s2 = [0u8; 16];
    let mut s3 = [0u8; 16];
    wr::<u32>(s1.as_mut_ptr() as usize, 5);
    wr::<f32>(s1.as_mut_ptr() as usize + 4, 1.5);
    wr::<f32>(s1.as_mut_ptr() as usize + 8, 2.5);

    s2.copy_from_slice(&s1);
    s3.copy_from_slice(&s1);
    wr::<f32>(s3.as_mut_ptr() as usize + 8, 3.5);

    let mut cases = 0u32;

    // 1. cur == 0 -> sets state and clears [0xa80]
    ctx_o[0xa80] = 1;
    ctx_n[0xa80] = 1;
    orig_fn(ctx_o.as_mut_ptr() as usize, s1.as_ptr() as usize);
    new_fn(ctx_n.as_mut_ptr() as usize, s1.as_ptr() as usize);
    crate::diff_eq!(ctx_o, ctx_n, "d3d_unknown_142073120 initial set");
    cases += 1;

    // 2. cur != 0, equal values -> no-op
    ctx_o[0xa80] = 1;
    ctx_n[0xa80] = 1;
    orig_fn(ctx_o.as_mut_ptr() as usize, s2.as_ptr() as usize);
    new_fn(ctx_n.as_mut_ptr() as usize, s2.as_ptr() as usize);
    crate::diff_eq!(ctx_o, ctx_n, "d3d_unknown_142073120 equal state");
    cases += 1;

    // 3. cur != 0, state == 0 -> no-op
    orig_fn(ctx_o.as_mut_ptr() as usize, 0);
    new_fn(ctx_n.as_mut_ptr() as usize, 0);
    crate::diff_eq!(ctx_o, ctx_n, "d3d_unknown_142073120 null state when cur!=0");
    cases += 1;

    // 4. cur != 0, different float -> updates
    orig_fn(ctx_o.as_mut_ptr() as usize, s3.as_ptr() as usize);
    new_fn(ctx_n.as_mut_ptr() as usize, s3.as_ptr() as usize);
    crate::diff_eq!(ctx_o, ctx_n, "d3d_unknown_142073120 different state");
    cases += 1;

    Ok(cases)
}

/// Original: `0x142073490` (73 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142073490(this: usize) -> u8 {
    let fence = rd::<usize>(this + 0x8);
    if fence == 0 {
        return 0;
    }
    let vtbl = rd::<usize>(fence);
    let get_completed_fn = rd::<usize>(vtbl + 0x40);
    let completed = exe::call_va!(get_completed_fn, (fence) -> u64);
    if completed == u64::MAX {
        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32) -> ());
    }
    u8::from(completed < rd::<u64>(this))
}

pub unsafe fn diff_d3d_unknown_142073490(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_get_completed(fence: usize) -> u64 {
        rd::<u64>(fence + 8)
    }

    type FnT = unsafe extern "win64" fn(usize) -> u8;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut vtbl = [0usize; 10];
    vtbl[8] = fake_get_completed as *const () as usize; // offset 0x40
    let mut fence_obj = [0u64; 2];
    fence_obj[0] = vtbl.as_ptr() as u64;

    let mut holder = [0u64; 2];
    let mut cases = 0u32;

    // Null fence
    let o0 = orig_fn(holder.as_ptr() as usize);
    let n0 = new_fn(holder.as_ptr() as usize);
    crate::diff_eq!(o0, n0, "d3d_unknown_142073490 null fence");
    cases += 1;

    // Non-null fence with various completed vs target values
    holder[1] = fence_obj.as_ptr() as u64;
    for (target, completed) in [(10u64, 5u64), (10, 10), (10, 15), (0, 0)] {
        holder[0] = target;
        fence_obj[1] = completed;
        let o = orig_fn(holder.as_ptr() as usize);
        let n = new_fn(holder.as_ptr() as usize);
        crate::diff_eq!(o, n, "d3d_unknown_142073490 compare");
        cases += 1;
    }

    Ok(cases)
}

/// Original: `0x142073640` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142073640() -> u32 {
    let global_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    if rd::<u8>(global_dev + 0xa0) != 0 {
        0x1000
    } else {
        0
    }
}

/// Original: `0x142073660` (65 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142073660(this: usize, ib: usize) {
    let new_fmt = if ib != 0 {
        let fmt = rd::<u32>(ib).wrapping_add(2) as u64;
        if rd::<u64>(this + 0xb20) == fmt {
            return;
        }
        wr::<usize>(this + 0xb18, ib);
        rd::<u32>(ib).wrapping_add(2) as u64
    } else {
        wr::<usize>(this + 0xb18, 0);
        0
    };
    wr::<u64>(this + 0xb20, new_fmt);
    wr::<u64>(this + 0x48, 0);
    wr::<u8>(this + 0xa80, 0);
}

pub unsafe fn diff_d3d_unknown_142073660(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut ctx_o = vec![0u8; 0xb30];
    let mut ctx_n = vec![0u8; 0xb30];
    let mut ib = [0u32; 4];
    ib[0] = 2;

    let mut cases = 0u32;

    // 1. Set non-null ib
    ctx_o[0xa80] = 1;
    ctx_n[0xa80] = 1;
    wr::<u64>(ctx_o.as_mut_ptr() as usize + 0x48, 99);
    wr::<u64>(ctx_n.as_mut_ptr() as usize + 0x48, 99);
    orig_fn(ctx_o.as_mut_ptr() as usize, ib.as_ptr() as usize);
    new_fn(ctx_n.as_mut_ptr() as usize, ib.as_ptr() as usize);
    crate::diff_eq!(ctx_o, ctx_n, "d3d_unknown_142073660 non-null");
    cases += 1;

    // 2. Redundant set with matching [0xb20]
    ctx_o[0xa80] = 1;
    ctx_n[0xa80] = 1;
    orig_fn(ctx_o.as_mut_ptr() as usize, ib.as_ptr() as usize);
    new_fn(ctx_n.as_mut_ptr() as usize, ib.as_ptr() as usize);
    crate::diff_eq!(ctx_o, ctx_n, "d3d_unknown_142073660 redundant");
    cases += 1;

    // 3. Set null ib
    orig_fn(ctx_o.as_mut_ptr() as usize, 0);
    new_fn(ctx_n.as_mut_ptr() as usize, 0);
    crate::diff_eq!(ctx_o, ctx_n, "d3d_unknown_142073660 null");
    cases += 1;

    Ok(cases)
}

/// Original: `0x142073700` (56 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142073700(flags: u32) -> u32 {
    let mut out = (flags >> 1) & 1;
    if ((flags >> 2) & 1) != 0 {
        out |= 0x2;
    }
    if ((flags >> 5) & 1) != 0 {
        out |= 0x40;
    }
    if (flags & 1) != 0 {
        out |= 0xc0;
    }
    out
}

pub unsafe fn diff_d3d_unknown_142073700(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(u32) -> u32;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut cases = 0u32;
    for flags in 0u32..128 {
        let o = orig_fn(flags);
        let n = new_fn(flags);
        crate::diff_eq!(o, n, "d3d_unknown_142073700");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142073740` (106 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_142073740() {
    let mut barrier = [0u64; 4];
    barrier[0] = 2; // D3D12_RESOURCE_BARRIER_TYPE_UAV
    let mut ctx_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (ctx_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = ctx_pair[1];
    let vtbl = rd::<usize>(cmd_list);
    let resource_barrier_fn = rd::<usize>(vtbl + 0xd0);
    exe::call_va!(resource_barrier_fn, (cmd_list, 1u32, barrier.as_ptr() as usize) -> ());
    release_cmd_ctx_ref(ctx_pair[0]);
}

/// Original: `0x1420737b0` (249 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_1420737b0(mut items: usize, mut count: u64) {
    if count == 0 {
        return;
    }
    let mut barriers = [[0u64; 4]; 64];
    while count != 0 {
        let count_i32 = count as i32;
        let batch = if count_i32 < 0x40 { count_i32 } else { 0x40 };
        let batch_isize = batch as isize;
        if batch > 0 {
            for i in 0..(batch as usize) {
                let item = rd::<usize>(items + i * 8);
                let res = rd::<u64>(item + 0x40);
                barriers[i] = [2u64, res, 0u64, 0u64];
            }
        }
        let mut ctx_pair = [0usize; 2];
        exe::call_va!(0x1_411d_5b00, (ctx_pair.as_mut_ptr() as usize) -> ());
        let cmd_list = ctx_pair[1];
        let vtbl = rd::<usize>(cmd_list);
        let resource_barrier_fn = rd::<usize>(vtbl + 0xd0);
        exe::call_va!(
            resource_barrier_fn,
            (cmd_list, batch as u32, barriers.as_ptr() as usize) -> ()
        );
        release_cmd_ctx_ref(ctx_pair[0]);

        count = count.wrapping_sub(batch_isize as u64);
        items = (items as isize).wrapping_add(batch_isize * 8) as usize;
    }
}

/// Original: `0x142073c40` (134 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_142073c40(this: usize) {
    let res = rd::<u64>(this + 0x40);
    let mut barrier = [0u64; 4];
    barrier[0] = 0; // D3D12_RESOURCE_BARRIER_TYPE_TRANSITION
    barrier[1] = res;
    barrier[2] = 0xffff_ffffu64 | (0x200u64 << 32);
    barrier[3] = 0x8;

    let mut ctx_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (ctx_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = ctx_pair[1];
    let vtbl = rd::<usize>(cmd_list);
    let resource_barrier_fn = rd::<usize>(vtbl + 0xd0);
    exe::call_va!(resource_barrier_fn, (cmd_list, 1u32, barrier.as_ptr() as usize) -> ());
    release_cmd_ctx_ref(ctx_pair[0]);
    wr::<u32>(this + 0x48, 0x8);
}

/// Original: `0x142073f10` (193 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_142073f10(this: usize) {
    let flags = rd::<u32>(this + 0x4c);
    let mut state_before = (flags >> 1) & 1;
    if ((flags >> 2) & 1) != 0 {
        state_before |= 0x2;
    }
    if ((flags >> 5) & 1) != 0 {
        state_before |= 0x40;
    }
    let res = rd::<u64>(this + 0x40);
    if (rd::<u8>(this + 0x4c) & 1) != 0 {
        state_before |= 0xc0;
    }

    let mut barrier = [0u64; 4];
    barrier[0] = 0;
    barrier[1] = res;
    barrier[2] = 0xffff_ffffu64 | ((state_before as u64) << 32);
    barrier[3] = 0x200;

    let mut ctx_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (ctx_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = ctx_pair[1];
    let vtbl = rd::<usize>(cmd_list);
    let resource_barrier_fn = rd::<usize>(vtbl + 0xd0);
    exe::call_va!(resource_barrier_fn, (cmd_list, 1u32, barrier.as_ptr() as usize) -> ());
    release_cmd_ctx_ref(ctx_pair[0]);
    wr::<u32>(this + 0x48, 0x200);
}

/// Original: `0x1420741e0` (134 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_1420741e0(this: usize) {
    let res = rd::<u64>(this + 0x40);
    let mut barrier = [0u64; 4];
    barrier[0] = 0;
    barrier[1] = res;
    barrier[2] = 0xffff_ffffu64 | (0x8u64 << 32);
    barrier[3] = 0x200;

    let mut ctx_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (ctx_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = ctx_pair[1];
    let vtbl = rd::<usize>(cmd_list);
    let resource_barrier_fn = rd::<usize>(vtbl + 0xd0);
    exe::call_va!(resource_barrier_fn, (cmd_list, 1u32, barrier.as_ptr() as usize) -> ());
    release_cmd_ctx_ref(ctx_pair[0]);
    wr::<u32>(this + 0x48, 0x200);
}

/// Original: `0x142074270` (192 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_142074270(this: usize) {
    let flags = rd::<u32>(this + 0x4c);
    let mut state_after = (flags >> 1) & 1;
    if ((flags >> 2) & 1) != 0 {
        state_after |= 0x2;
    }
    if ((flags >> 5) & 1) != 0 {
        state_after |= 0x40;
    }
    if (rd::<u8>(this + 0x4c) & 1) != 0 {
        state_after |= 0xc0;
    }
    wr::<u32>(this + 0x48, state_after);

    let res = rd::<u64>(this + 0x40);
    let mut barrier = [0u64; 4];
    barrier[0] = 0;
    barrier[1] = res;
    barrier[2] = 0xffff_ffffu64 | (0x8u64 << 32);
    barrier[3] = state_after as u64;

    let mut ctx_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (ctx_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = ctx_pair[1];
    let vtbl = rd::<usize>(cmd_list);
    let resource_barrier_fn = rd::<usize>(vtbl + 0xd0);
    exe::call_va!(resource_barrier_fn, (cmd_list, 1u32, barrier.as_ptr() as usize) -> ());
    release_cmd_ctx_ref(ctx_pair[0]);
}

/// Original: `0x1420744a0` (193 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_1420744a0(this: usize) {
    let flags = rd::<u32>(this + 0x4c);
    let mut state_before = (flags >> 1) & 1;
    if ((flags >> 2) & 1) != 0 {
        state_before |= 0x2;
    }
    if ((flags >> 5) & 1) != 0 {
        state_before |= 0x40;
    }
    let res = rd::<u64>(this + 0x40);
    if (rd::<u8>(this + 0x4c) & 1) != 0 {
        state_before |= 0xc0;
    }

    let mut barrier = [0u64; 4];
    barrier[0] = 0;
    barrier[1] = res;
    barrier[2] = 0xffff_ffffu64 | ((state_before as u64) << 32);
    barrier[3] = 0x8;

    let mut ctx_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (ctx_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = ctx_pair[1];
    let vtbl = rd::<usize>(cmd_list);
    let resource_barrier_fn = rd::<usize>(vtbl + 0xd0);
    exe::call_va!(resource_barrier_fn, (cmd_list, 1u32, barrier.as_ptr() as usize) -> ());
    release_cmd_ctx_ref(ctx_pair[0]);
    wr::<u32>(this + 0x48, 0x8);
}

/// Original: `0x142074cd0` (43 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142074cd0(this: usize, range_ptr: usize) {
    let mut range = [0u64; 2];
    range[0] = rd::<u64>(range_ptr);
    let res = rd::<usize>(this + 0x40);
    range[1] = rd::<u64>(range_ptr + 0x8);
    let vtbl = rd::<usize>(res);
    let unmap_fn = rd::<usize>(vtbl + 0x48);
    exe::call_va!(unmap_fn, (res, 0u32, range.as_ptr() as usize) -> ());
}

/// Original: `0x142074d10` (87 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142074d10(this: usize, range_ptr: usize) -> usize {
    let mut range = [0u64; 2];
    range[0] = rd::<u64>(range_ptr);
    let res = rd::<usize>(this + 0x40);
    range[1] = rd::<u64>(range_ptr + 0x8);
    let mut mapped_ptr: usize = 0;
    let mapped_ptr_addr = (&mut mapped_ptr as *mut usize) as usize;
    let vtbl = rd::<usize>(res);
    let map_fn = rd::<usize>(vtbl + 0x40);
    let hr = exe::call_va!(
        map_fn,
        (res, 0u32, range.as_ptr() as usize, mapped_ptr_addr) -> i32
    );
    let hr_u = hr as u32;
    if hr_u == 0x8007_000e || hr_u.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr_u) -> ());
    }
    rd::<usize>(mapped_ptr_addr)
}

/// Original: `0x142075030` (72 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142075030(kind: u32) -> usize {
    match kind {
        0 => exe::ptr::<u8>(0x1_433d_cb20) as usize, // "RaytracingBLAS"
        1 => exe::ptr::<u8>(0x1_433d_cbe8) as usize, // "RaytracingTLAS"
        2 => exe::ptr::<u8>(0x1_433d_cbc0) as usize, // "RaytracingScratch"
        3 => exe::ptr::<u8>(0x1_433d_cb90) as usize, // "RaytracingShaderTable"
        4 => exe::ptr::<u8>(0x1_433d_cb60) as usize, // "RaytracingPostBuildInfo"
        _ => exe::ptr::<u8>(0x1_433d_cb40) as usize, // "RTUnknownBuffer"
    }
}

pub unsafe fn diff_d3d_unknown_142075030(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(u32) -> usize;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut cases = 0u32;
    for kind in [0u32, 1, 2, 3, 4, 5, 6, 100, u32::MAX] {
        let o = orig_fn(kind);
        let n = new_fn(kind);
        crate::diff_eq!(o, n, "d3d_unknown_142075030");
        cases += 1;
    }
    Ok(cases)
}

/// Original: `0x142075850` (123 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_unknown_142075850(this: usize) {
    let res = rd::<u64>(this + 0x40);
    let mut barrier = [0u64; 4];
    barrier[0] = 2; // D3D12_RESOURCE_BARRIER_TYPE_UAV
    barrier[1] = res;

    let mut ctx_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (ctx_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = ctx_pair[1];
    let vtbl = rd::<usize>(cmd_list);
    let resource_barrier_fn = rd::<usize>(vtbl + 0xd0);
    exe::call_va!(resource_barrier_fn, (cmd_list, 1u32, barrier.as_ptr() as usize) -> ());
    release_cmd_ctx_ref(ctx_pair[0]);
}

/// Original: `0x1420758f0` (122 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_1420758f0(
    this: usize,
    src: usize,
    offset: u64,
    size: u64,
    stride: u64,
    flag: u8,
) {
    let flags_4c = rd::<u32>(src + 0x4c);
    let res = rd::<u64>(src + 0x40);
    exe::call_va!(
        0x1_4207_5080,
        (this, res, flag as u32, stride, offset, size, flags_4c) -> ()
    );
    wr::<u64>(this + 0x10, size);
    wr::<u64>(this + 0x8, stride);
    let base_va = rd::<u64>(src);
    wr::<u64>(this, base_va.wrapping_add(offset));
}

/// Original: `0x142075970` (161 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142075970(this: usize) {
    let heap_mgr_addr = exe::ptr::<usize>(0x1_4397_fcb8) as usize;

    let h0 = rd::<u64>(this + 0x18);
    let mgr0 = rd::<usize>(heap_mgr_addr);
    exe::call_va!(0x1_4209_6c10, (mgr0, h0) -> ());

    let h1 = rd::<u64>(this + 0x20);
    let mgr1 = rd::<usize>(heap_mgr_addr);
    exe::call_va!(0x1_4209_6a90, (mgr1, h1) -> ());

    let h2 = rd::<u64>(this + 0x28);
    let mgr2 = rd::<usize>(heap_mgr_addr);
    exe::call_va!(0x1_4209_6a90, (mgr2, h2) -> ());

    let h3 = rd::<u64>(this + 0x30);
    let mgr3 = rd::<usize>(heap_mgr_addr);
    exe::call_va!(0x1_4209_6c10, (mgr3, h3) -> ());

    let h4 = rd::<u64>(this + 0x38);
    let mgr4 = rd::<usize>(heap_mgr_addr);
    exe::call_va!(0x1_4209_6c10, (mgr4, h4) -> ());

    for off in (0..0x40).step_by(8) {
        wr::<u64>(this + off, 0);
    }
}

/// Original: `0x142075a20` (200 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142075a20(this: usize) {
    let res = rd::<usize>(this + 0x40);
    if res != 0 {
        exe::call_va!(0x1_4209_a1e0, (res, 0u32) -> ());
        let heap_mgr_addr = exe::ptr::<usize>(0x1_4397_fcb8) as usize;

        let h0 = rd::<u64>(this + 0x18);
        let mgr0 = rd::<usize>(heap_mgr_addr);
        exe::call_va!(0x1_4209_6c10, (mgr0, h0) -> ());

        let h1 = rd::<u64>(this + 0x20);
        let mgr1 = rd::<usize>(heap_mgr_addr);
        exe::call_va!(0x1_4209_6a90, (mgr1, h1) -> ());

        let h2 = rd::<u64>(this + 0x28);
        let mgr2 = rd::<usize>(heap_mgr_addr);
        exe::call_va!(0x1_4209_6a90, (mgr2, h2) -> ());

        let h3 = rd::<u64>(this + 0x30);
        let mgr3 = rd::<usize>(heap_mgr_addr);
        exe::call_va!(0x1_4209_6c10, (mgr3, h3) -> ());

        let h4 = rd::<u64>(this + 0x38);
        let mgr4 = rd::<usize>(heap_mgr_addr);
        exe::call_va!(0x1_4209_6c10, (mgr4, h4) -> ());
    }

    for off in (0..0x50).step_by(8) {
        wr::<u64>(this + off, 0);
    }
}

/// Original: `0x142075af0` (185 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_unknown_142075af0(
    size: u64,
    stride: u64,
    flag: u8,
    usage_flags: u32,
    extra: u32,
    _unused6: usize,
    out_buf: usize,
) {
    let mut aligned_size = size.wrapping_add(3) & !3u64;
    if ((usage_flags >> 3) & 1) != 0 {
        aligned_size = aligned_size.wrapping_add(0xff) & !0xffu64;
    }
    let state = exe::call_va!(
        0x1_4207_56b0,
        (aligned_size, usage_flags, extra, out_buf + 0x40) -> u32
    );
    let res = rd::<usize>(out_buf + 0x40);
    wr::<u32>(out_buf + 0x48, state);
    let vtbl = rd::<usize>(res);
    let get_gpu_va_fn = rd::<usize>(vtbl + 0x58);
    let gpu_va = exe::call_va!(get_gpu_va_fn, (res) -> u64);

    let res_u64 = rd::<u64>(out_buf + 0x40);
    wr::<u64>(out_buf, gpu_va);
    wr::<u64>(out_buf + 0x8, stride);
    wr::<u64>(out_buf + 0x10, aligned_size);
    wr::<u32>(out_buf + 0x4c, usage_flags);
    exe::call_va!(
        0x1_4207_5080,
        (out_buf, res_u64, flag as u32, stride, 0u64, aligned_size, usage_flags) -> ()
    );
}

unsafe extern "win64" fn fake_cmd_list_resource_barrier(
    cmd_list: usize,
    count: u32,
    barriers: usize,
) {
    wr::<u64>(cmd_list + 0x08, count as u64);
    let n = (count as usize).min(2);
    for i in 0..(n * 4) {
        wr::<u64>(cmd_list + 0x10 + i * 8, rd::<u64>(barriers + i * 8));
    }
}

unsafe fn run_with_fake_cmd_list(f: impl FnOnce()) -> ([u64; 8], [u64; 12]) {
    let mut vtbl = [0usize; 32];
    vtbl[0xd0 / 8] = fake_cmd_list_resource_barrier as *const () as usize;
    let mut cmd_list = [0u64; 12];
    cmd_list[0] = vtbl.as_ptr() as u64;
    let mut cmd_ctx = [0u64; 8];
    wr::<usize>(
        cmd_ctx.as_mut_ptr() as usize + 0x28,
        cmd_list.as_mut_ptr() as usize,
    );
    let mut fake_tls = vec![0u8; 0x6400];
    fake_tls[0x70] = 1;
    wr::<usize>(
        fake_tls.as_mut_ptr() as usize + 0x6300,
        cmd_ctx.as_mut_ptr() as usize,
    );
    wr::<usize>(fake_tls.as_mut_ptr() as usize + 0x6318, 0);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);
    f();
    wr::<usize>(tls_slot, saved_tls0);
    (cmd_ctx, cmd_list)
}

pub unsafe fn diff_d3d_unknown_142072940(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize) -> u8;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let flag_addr = exe::ptr::<u8>(0x1_43a2_8c3d) as usize;
    let saved = rd::<u8>(flag_addr);
    wr::<u8>(flag_addr, 0);
    let o = orig_fn(0x1234);
    let n = new_fn(0x1234);
    wr::<u8>(flag_addr, saved);
    crate::diff_eq!(o, n, "d3d_unknown_142072940 flag==0");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142072990(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize) -> u8;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let flag_addr = exe::ptr::<u8>(0x1_43a2_8c3d) as usize;
    let saved = rd::<u8>(flag_addr);
    wr::<u8>(flag_addr, 0);
    let o = orig_fn(0x1234);
    let n = new_fn(0x1234);
    wr::<u8>(flag_addr, saved);
    crate::diff_eq!(o, n, "d3d_unknown_142072990 flag==0");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142072a70(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut obj_o = [0x42u64, 0u64];
    let mut obj_n = [0x42u64, 0u64];
    orig_fn(obj_o.as_mut_ptr() as usize);
    new_fn(obj_n.as_mut_ptr() as usize);
    crate::diff_eq!(obj_o, obj_n, "d3d_unknown_142072a70 null obj");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142072ad0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_release(obj: usize) -> u32 {
        wr::<u64>(obj + 0x08, rd::<u64>(obj + 0x08).wrapping_add(1));
        0
    }
    unsafe extern "win64" fn fake_create_pso(
        dev: usize,
        desc: usize,
        _riid: usize,
        out_ppv: usize,
    ) -> i32 {
        wr::<u64>(dev + 0x10, rd::<u64>(desc));
        wr::<u64>(dev + 0x18, rd::<u64>(desc + 0x08));
        let hr = rd::<i32>(dev + 0x08);
        let dummy_obj = rd::<usize>(dev + 0x20);
        wr::<usize>(out_ppv, dummy_obj);
        hr
    }

    type FnT = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut dev_vtbl = [0usize; 52];
    dev_vtbl[0x178 / 8] = fake_create_pso as *const () as usize;
    let mut obj_vtbl = [0usize; 4];
    obj_vtbl[0x10 / 8] = fake_release as *const () as usize;

    let global_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_global = rd::<usize>(global_addr);

    let desc = [0x1111_2222_3333_4444u64, 0x5555_6666_7777_8888u64];
    let mut cases = 0u32;

    for hr in [0i32, -1i32] {
        let mut dummy_o = [obj_vtbl.as_ptr() as u64, 0u64];
        let mut dummy_n = [obj_vtbl.as_ptr() as u64, 0u64];
        let mut dev_o = [
            dev_vtbl.as_ptr() as u64,
            hr as u32 as u64,
            0,
            0,
            dummy_o.as_mut_ptr() as u64,
        ];
        let mut dev_n = [
            dev_vtbl.as_ptr() as u64,
            hr as u32 as u64,
            0,
            0,
            dummy_n.as_mut_ptr() as u64,
        ];
        let ctx_o = [dev_o.as_mut_ptr() as u64];
        let ctx_n = [dev_n.as_mut_ptr() as u64];
        let mut out_o = 0usize;
        let mut out_n = 0usize;

        wr::<usize>(global_addr, ctx_o.as_ptr() as usize);
        let ro = orig_fn(
            0,
            core::ptr::addr_of_mut!(out_o) as usize,
            desc.as_ptr() as usize,
        );
        wr::<usize>(global_addr, ctx_n.as_ptr() as usize);
        let rn = new_fn(
            0,
            core::ptr::addr_of_mut!(out_n) as usize,
            desc.as_ptr() as usize,
        );
        wr::<usize>(global_addr, saved_global);

        crate::diff_eq!(ro, core::ptr::addr_of_mut!(out_o) as usize, "ret orig");
        crate::diff_eq!(rn, core::ptr::addr_of_mut!(out_n) as usize, "ret new");
        crate::diff_eq!(out_o != 0, out_n != 0, "out non-null");
        crate::diff_eq!(dev_o[2..4], dev_n[2..4], "copied desc");
        crate::diff_eq!(dummy_o[1], dummy_n[1], "release count");
        cases += 1;
    }

    Ok(cases)
}

pub unsafe fn diff_d3d_unknown_142072b80(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_create_cmd_sig(
        dev: usize,
        desc: usize,
        _riid: usize,
        out_ppv: usize,
        node_mask: u32,
    ) -> i32 {
        wr::<u64>(dev + 0x08, rd::<u64>(desc));
        wr::<u64>(dev + 0x10, rd::<u64>(desc + 0x08));
        wr::<u64>(dev + 0x18, rd::<u64>(desc + 0x10));
        wr::<u32>(dev + 0x20, node_mask);
        wr::<usize>(out_ppv, 0xcafe_babe);
        0
    }

    type FnT = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut dev_vtbl = [0usize; 16];
    dev_vtbl[0x58 / 8] = fake_create_cmd_sig as *const () as usize;

    let mut dev_o = [0u64; 6];
    let mut dev_n = [0u64; 6];
    dev_o[0] = dev_vtbl.as_ptr() as u64;
    dev_n[0] = dev_vtbl.as_ptr() as u64;

    let mut ctx_o = [0u64; 8];
    let mut ctx_n = [0u64; 8];
    ctx_o[0] = dev_o.as_mut_ptr() as u64;
    ctx_n[0] = dev_n.as_mut_ptr() as u64;
    ctx_o[6] = 0x9999_8888; // +0x30
    ctx_n[6] = 0x9999_8888;

    let desc = [0x1111_2222u64, 0x3333_4444u64];
    let mut out_o = 0usize;
    let mut out_n = 0usize;

    let global_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_global = rd::<usize>(global_addr);

    wr::<usize>(global_addr, ctx_o.as_mut_ptr() as usize);
    let ro = orig_fn(
        0,
        core::ptr::addr_of_mut!(out_o) as usize,
        desc.as_ptr() as usize,
    );
    wr::<usize>(global_addr, ctx_n.as_mut_ptr() as usize);
    let rn = new_fn(
        0,
        core::ptr::addr_of_mut!(out_n) as usize,
        desc.as_ptr() as usize,
    );
    wr::<usize>(global_addr, saved_global);

    crate::diff_eq!(ro, core::ptr::addr_of_mut!(out_o) as usize, "ret orig");
    crate::diff_eq!(rn, core::ptr::addr_of_mut!(out_n) as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "out_ppv");
    crate::diff_eq!(dev_o[1..5], dev_n[1..5], "recorded args");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142072c10(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut fake_ctx = [0u64; 10];
    fake_ctx[7] = 0xaaaa_bbbb_cccc_dddd; // +0x38
    fake_ctx[8] = 0x1111_2222_3333_4444; // +0x40

    let global_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_global = rd::<usize>(global_addr);
    wr::<usize>(global_addr, fake_ctx.as_ptr() as usize);

    let mut out_o = [0u64; 2];
    let mut out_n = [0u64; 2];
    let ro = orig_fn(out_o.as_mut_ptr() as usize);
    let rn = new_fn(out_n.as_mut_ptr() as usize);
    wr::<usize>(global_addr, saved_global);

    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "d3d_unknown_142072c10 out");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142073640(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn() -> u32;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut fake_ctx = [0u8; 0xb0];
    let global_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_global = rd::<usize>(global_addr);
    wr::<usize>(global_addr, fake_ctx.as_mut_ptr() as usize);

    let mut cases = 0u32;
    for flag in [0u8, 1u8, 0xffu8] {
        fake_ctx[0xa0] = flag;
        let o = orig_fn();
        let n = new_fn();
        if o != n {
            wr::<usize>(global_addr, saved_global);
            return Err(format!("d3d_unknown_142073640 flag={flag}: orig={o} new={n}"));
        }
        cases += 1;
    }

    wr::<usize>(global_addr, saved_global);
    Ok(cases)
}

pub unsafe fn diff_d3d_unknown_142073740(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn();
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let (ctx_o, list_o) = run_with_fake_cmd_list(|| orig_fn());
    let (ctx_n, list_n) = run_with_fake_cmd_list(|| new_fn());

    crate::diff_eq!(ctx_o[3], ctx_n[3], "cmd_ctx counter");
    crate::diff_eq!(list_o[1..6], list_n[1..6], "uav barrier");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_1420737b0(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize, u64);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    // 1. count == 0 early exit
    orig_fn(0, 0);
    new_fn(0, 0);

    // 2. count == 2 batch UAV barriers
    let mut item0 = [0u64; 10];
    let mut item1 = [0u64; 10];
    item0[8] = 0xaaaa_1111; // +0x40
    item1[8] = 0xbbbb_2222; // +0x40
    let items = [item0.as_ptr() as usize, item1.as_ptr() as usize];

    let (ctx_o, list_o) = run_with_fake_cmd_list(|| orig_fn(items.as_ptr() as usize, 2));
    let (ctx_n, list_n) = run_with_fake_cmd_list(|| new_fn(items.as_ptr() as usize, 2));

    crate::diff_eq!(ctx_o[3], ctx_n[3], "cmd_ctx counter");
    crate::diff_eq!(list_o[1..10], list_n[1..10], "batch uav barriers");
    Ok(2)
}

pub unsafe fn diff_d3d_unknown_142073c40(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut obj_o = [0u8; 0x50];
    let mut obj_n = [0u8; 0x50];
    wr::<u64>(obj_o.as_mut_ptr() as usize + 0x40, 0x1234_5678);
    wr::<u64>(obj_n.as_mut_ptr() as usize + 0x40, 0x1234_5678);

    let (_, list_o) = run_with_fake_cmd_list(|| orig_fn(obj_o.as_mut_ptr() as usize));
    let (_, list_n) = run_with_fake_cmd_list(|| new_fn(obj_n.as_mut_ptr() as usize));

    crate::diff_eq!(obj_o, obj_n, "this state");
    crate::diff_eq!(list_o[1..6], list_n[1..6], "transition barrier");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142073f10(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut cases = 0u32;
    for flags in [0u32, 1, 2, 4, 0x20, 0x27] {
        let mut obj_o = [0u8; 0x50];
        let mut obj_n = [0u8; 0x50];
        wr::<u64>(obj_o.as_mut_ptr() as usize + 0x40, 0xdead_beef);
        wr::<u64>(obj_n.as_mut_ptr() as usize + 0x40, 0xdead_beef);
        wr::<u32>(obj_o.as_mut_ptr() as usize + 0x4c, flags);
        wr::<u32>(obj_n.as_mut_ptr() as usize + 0x4c, flags);

        let (_, list_o) = run_with_fake_cmd_list(|| orig_fn(obj_o.as_mut_ptr() as usize));
        let (_, list_n) = run_with_fake_cmd_list(|| new_fn(obj_n.as_mut_ptr() as usize));

        crate::diff_eq!(obj_o, obj_n, "this state");
        crate::diff_eq!(list_o[1..6], list_n[1..6], "transition barrier");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_unknown_1420741e0(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut obj_o = [0u8; 0x50];
    let mut obj_n = [0u8; 0x50];
    wr::<u64>(obj_o.as_mut_ptr() as usize + 0x40, 0xfeed_face);
    wr::<u64>(obj_n.as_mut_ptr() as usize + 0x40, 0xfeed_face);

    let (_, list_o) = run_with_fake_cmd_list(|| orig_fn(obj_o.as_mut_ptr() as usize));
    let (_, list_n) = run_with_fake_cmd_list(|| new_fn(obj_n.as_mut_ptr() as usize));

    crate::diff_eq!(obj_o, obj_n, "this state");
    crate::diff_eq!(list_o[1..6], list_n[1..6], "transition barrier");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142074270(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut cases = 0u32;
    for flags in [0u32, 1, 2, 4, 0x20, 0x27] {
        let mut obj_o = [0u8; 0x50];
        let mut obj_n = [0u8; 0x50];
        wr::<u64>(obj_o.as_mut_ptr() as usize + 0x40, 0xabcd_ef01);
        wr::<u64>(obj_n.as_mut_ptr() as usize + 0x40, 0xabcd_ef01);
        wr::<u32>(obj_o.as_mut_ptr() as usize + 0x4c, flags);
        wr::<u32>(obj_n.as_mut_ptr() as usize + 0x4c, flags);

        let (_, list_o) = run_with_fake_cmd_list(|| orig_fn(obj_o.as_mut_ptr() as usize));
        let (_, list_n) = run_with_fake_cmd_list(|| new_fn(obj_n.as_mut_ptr() as usize));

        crate::diff_eq!(obj_o, obj_n, "this state");
        crate::diff_eq!(list_o[1..6], list_n[1..6], "transition barrier");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_unknown_1420744a0(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut cases = 0u32;
    for flags in [0u32, 1, 2, 4, 0x20, 0x27] {
        let mut obj_o = [0u8; 0x50];
        let mut obj_n = [0u8; 0x50];
        wr::<u64>(obj_o.as_mut_ptr() as usize + 0x40, 0x9876_5432);
        wr::<u64>(obj_n.as_mut_ptr() as usize + 0x40, 0x9876_5432);
        wr::<u32>(obj_o.as_mut_ptr() as usize + 0x4c, flags);
        wr::<u32>(obj_n.as_mut_ptr() as usize + 0x4c, flags);

        let (_, list_o) = run_with_fake_cmd_list(|| orig_fn(obj_o.as_mut_ptr() as usize));
        let (_, list_n) = run_with_fake_cmd_list(|| new_fn(obj_n.as_mut_ptr() as usize));

        crate::diff_eq!(obj_o, obj_n, "this state");
        crate::diff_eq!(list_o[1..6], list_n[1..6], "transition barrier");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_unknown_142074cd0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_unmap(res: usize, subres: u32, range: usize) {
        wr::<u64>(res + 0x08, subres as u64);
        wr::<u64>(res + 0x10, rd::<u64>(range));
        wr::<u64>(res + 0x18, rd::<u64>(range + 0x08));
    }

    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut vtbl = [0usize; 12];
    vtbl[0x48 / 8] = fake_unmap as *const () as usize;

    let mut res_o = [vtbl.as_ptr() as u64, 0, 0, 0];
    let mut res_n = [vtbl.as_ptr() as u64, 0, 0, 0];

    let mut obj_o = [0u8; 0x48];
    let mut obj_n = [0u8; 0x48];
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0x40, res_o.as_mut_ptr() as usize);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0x40, res_n.as_mut_ptr() as usize);

    let range = [0x100u64, 0x200u64];
    orig_fn(obj_o.as_mut_ptr() as usize, range.as_ptr() as usize);
    new_fn(obj_n.as_mut_ptr() as usize, range.as_ptr() as usize);

    crate::diff_eq!(res_o[1..4], res_n[1..4], "unmap args");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142074d10(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_map(
        res: usize,
        subres: u32,
        range: usize,
        out_data: usize,
    ) -> i32 {
        wr::<u64>(res + 0x08, subres as u64);
        wr::<u64>(res + 0x10, rd::<u64>(range));
        wr::<u64>(res + 0x18, rd::<u64>(range + 0x08));
        wr::<usize>(out_data, 0x7777_8888_9999);
        0
    }

    type FnT = unsafe extern "win64" fn(usize, usize) -> usize;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut vtbl = [0usize; 12];
    vtbl[0x40 / 8] = fake_map as *const () as usize;

    let mut res_o = [vtbl.as_ptr() as u64, 0, 0, 0];
    let mut res_n = [vtbl.as_ptr() as u64, 0, 0, 0];

    let mut obj_o = [0u8; 0x48];
    let mut obj_n = [0u8; 0x48];
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0x40, res_o.as_mut_ptr() as usize);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0x40, res_n.as_mut_ptr() as usize);

    let range = [0x300u64, 0x400u64];
    let po = orig_fn(obj_o.as_mut_ptr() as usize, range.as_ptr() as usize);
    let pn = new_fn(obj_n.as_mut_ptr() as usize, range.as_ptr() as usize);

    crate::diff_eq!(po, pn, "mapped ptr");
    crate::diff_eq!(res_o[1..4], res_n[1..4], "map args");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142075850(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut obj_o = [0u8; 0x50];
    let mut obj_n = [0u8; 0x50];
    wr::<u64>(obj_o.as_mut_ptr() as usize + 0x40, 0x5555_aaaa);
    wr::<u64>(obj_n.as_mut_ptr() as usize + 0x40, 0x5555_aaaa);

    let (_, list_o) = run_with_fake_cmd_list(|| orig_fn(obj_o.as_mut_ptr() as usize));
    let (_, list_n) = run_with_fake_cmd_list(|| new_fn(obj_n.as_mut_ptr() as usize));

    crate::diff_eq!(list_o[1..6], list_n[1..6], "uav barrier");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142075a20(orig: usize, new: usize) -> Result<u32, String> {
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut obj_o = [0x5au8; 0x58];
    let mut obj_n = [0x5au8; 0x58];
    wr::<usize>(obj_o.as_mut_ptr() as usize + 0x40, 0);
    wr::<usize>(obj_n.as_mut_ptr() as usize + 0x40, 0);

    orig_fn(obj_o.as_mut_ptr() as usize);
    new_fn(obj_n.as_mut_ptr() as usize);
    crate::diff_eq!(obj_o, obj_n, "d3d_unknown_142075a20 null res");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142072910(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_16d0(a1: usize, param: usize) -> u32 {
        let p0 = rd::<u32>(param);
        let p1 = rd::<u32>(param + 4);
        wr::<u32>(a1, p0);
        wr::<u32>(a1 + 4, p1);
        p0 ^ p1
    }

    let _m = crate::difftest::Mock::new(0x1_4000_16d0, fake_16d0 as *const ())?;
    type FnT = unsafe extern "win64" fn(usize, u8) -> u32;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut cases = 0u32;
    for a2 in [0u8, 1, 5, 0x3f, 0xff] {
        let mut rec_o = [0u32; 2];
        let mut rec_n = [0u32; 2];
        let ro = orig_fn(rec_o.as_mut_ptr() as usize, a2);
        let rn = new_fn(rec_n.as_mut_ptr() as usize, a2);
        crate::diff_eq!(ro, rn, "d3d_unknown_142072910 ret");
        crate::diff_eq!(rec_o, rec_n, "d3d_unknown_142072910 param");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_unknown_142072a30(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_1940(a1: usize) -> i32 {
        a1 as i32
    }

    let _m = crate::difftest::Mock::new(0x1_4000_1940, fake_1940 as *const ())?;
    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let flag_addr = exe::ptr::<u8>(0x1_43a2_8c3d) as usize;
    let saved = rd::<u8>(flag_addr);

    let mut cases = 0u32;
    for a1 in [0usize, 1usize, 0xffffusize] {
        wr::<u8>(flag_addr, 0xaa);
        orig_fn(a1);
        let fo = rd::<u8>(flag_addr);

        wr::<u8>(flag_addr, 0xaa);
        new_fn(a1);
        let fn_val = rd::<u8>(flag_addr);

        wr::<u8>(flag_addr, saved);
        crate::diff_eq!(fo, fn_val, "d3d_unknown_142072a30 flag");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_d3d_unknown_142072e80(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_df80(ctx: usize) -> u64 {
        (ctx as u64) ^ 0x55aa_1234
    }

    let _m = crate::difftest::Mock::new(0x1_4206_df80, fake_df80 as *const ())?;
    type FnT = unsafe extern "win64" fn() -> u64;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut fake_tls = vec![0u8; 0x6400];
    fake_tls[0x70] = 1;
    wr::<usize>(fake_tls.as_mut_ptr() as usize + 0x6328, 0x1234_5678);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);
    let ro = orig_fn();
    let rn = new_fn();
    wr::<usize>(tls_slot, saved_tls0);

    crate::diff_eq!(ro, rn, "d3d_unknown_142072e80");
    Ok(1)
}

unsafe extern "win64" fn fake_init_buf_views_5080(
    this: usize,
    res: u64,
    flag: u32,
    stride: u64,
    offset: u64,
    size: u64,
    flags_4c: u32,
) {
    wr::<u64>(this + 0x18, res);
    wr::<u64>(this + 0x20, (flag as u64) | ((flags_4c as u64) << 32));
    wr::<u64>(this + 0x28, stride);
    wr::<u64>(this + 0x30, offset);
    wr::<u64>(this + 0x38, size);
}

pub unsafe fn diff_d3d_unknown_1420758f0(orig: usize, new: usize) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4207_5080, fake_init_buf_views_5080 as *const ())?;
    type FnT = unsafe extern "win64" fn(usize, usize, u64, u64, u64, u8);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut src = [0u8; 0x50];
    wr::<u64>(src.as_mut_ptr() as usize, 0x8000_0000);
    wr::<u64>(src.as_mut_ptr() as usize + 0x40, 0xdead_beef_cafe);
    wr::<u32>(src.as_mut_ptr() as usize + 0x4c, 0x27);

    let mut this_o = [0u8; 0x48];
    let mut this_n = [0u8; 0x48];
    orig_fn(
        this_o.as_mut_ptr() as usize,
        src.as_ptr() as usize,
        0x200,
        0x1000,
        16,
        1,
    );
    new_fn(
        this_n.as_mut_ptr() as usize,
        src.as_ptr() as usize,
        0x200,
        0x1000,
        16,
        1,
    );

    crate::diff_eq!(this_o, this_n, "d3d_unknown_1420758f0");
    Ok(1)
}

pub unsafe fn diff_d3d_unknown_142075970(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_6c10(mgr: usize, h: u64) {
        let v = rd::<u64>(mgr).wrapping_mul(31).wrapping_add(h ^ 0x6c10);
        wr::<u64>(mgr, v);
    }
    unsafe extern "win64" fn fake_6a90(mgr: usize, h: u64) {
        let v = rd::<u64>(mgr).wrapping_mul(31).wrapping_add(h ^ 0x6a90);
        wr::<u64>(mgr, v);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_6c10, fake_6c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_6a90, fake_6a90 as *const ())?;

    type FnT = unsafe extern "win64" fn(usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let g_mgr = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let saved_mgr = rd::<usize>(g_mgr);

    let mut mgr_o = 0u64;
    let mut mgr_n = 0u64;
    let mut obj_o = [0u8; 0x48];
    let mut obj_n = [0u8; 0x48];
    for (i, b) in obj_o.iter_mut().enumerate() {
        *b = (i as u8).wrapping_add(1);
    }
    obj_n.copy_from_slice(&obj_o);

    wr::<usize>(g_mgr, core::ptr::addr_of_mut!(mgr_o) as usize);
    orig_fn(obj_o.as_mut_ptr() as usize);
    wr::<usize>(g_mgr, core::ptr::addr_of_mut!(mgr_n) as usize);
    new_fn(obj_n.as_mut_ptr() as usize);
    wr::<usize>(g_mgr, saved_mgr);

    crate::diff_eq!(obj_o, obj_n, "d3d_unknown_142075970 obj");
    crate::diff_eq!(mgr_o, mgr_n, "d3d_unknown_142075970 mgr");
    Ok(2)
}

pub unsafe fn diff_d3d_unknown_142075af0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_get_gpu_va(res: usize) -> u64 {
        rd::<u64>(res + 0x10)
    }
    unsafe extern "win64" fn fake_56b0(
        aligned_size: u64,
        usage_flags: u32,
        extra: u32,
        out_res_ptr: usize,
    ) -> u32 {
        let res = rd::<usize>(out_res_ptr);
        wr::<u64>(
            res + 0x08,
            aligned_size ^ (usage_flags as u64) ^ ((extra as u64) << 32),
        );
        0x80
    }

    let _m1 = crate::difftest::Mock::new(0x1_4207_56b0, fake_56b0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_5080, fake_init_buf_views_5080 as *const ())?;

    type FnT = unsafe extern "win64" fn(u64, u64, u8, u32, u32, usize, usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut vtbl = [0usize; 16];
    vtbl[0x58 / 8] = fake_get_gpu_va as *const () as usize;

    let mut cases = 0u32;
    for usage_flags in [0x04u32, 0x08u32] {
        let mut res_o = [vtbl.as_ptr() as u64, 0u64, 0x7000_1234_5678_0000u64];
        let mut res_n = [vtbl.as_ptr() as u64, 0u64, 0x7000_1234_5678_0000u64];
        let mut out_o = [0u8; 0x50];
        let mut out_n = [0u8; 0x50];
        wr::<usize>(out_o.as_mut_ptr() as usize + 0x40, res_o.as_mut_ptr() as usize);
        wr::<usize>(out_n.as_mut_ptr() as usize + 0x40, res_n.as_mut_ptr() as usize);

        orig_fn(300, 16, 1, usage_flags, 7, 0, out_o.as_mut_ptr() as usize);
        new_fn(300, 16, 1, usage_flags, 7, 0, out_n.as_mut_ptr() as usize);

        crate::diff_eq!(
            rd::<usize>(out_o.as_ptr() as usize + 0x18) - res_o.as_ptr() as usize,
            rd::<usize>(out_n.as_ptr() as usize + 0x18) - res_n.as_ptr() as usize,
            "d3d_unknown_142075af0 res ptr"
        );
        wr::<usize>(out_o.as_mut_ptr() as usize + 0x18, 0);
        wr::<usize>(out_n.as_mut_ptr() as usize + 0x18, 0);
        crate::diff_eq!(out_o[..0x40], out_n[..0x40], "d3d_unknown_142075af0 lo");
        crate::diff_eq!(out_o[0x48..], out_n[0x48..], "d3d_unknown_142075af0 hi");
        crate::diff_eq!(res_o[1], res_n[1], "d3d_unknown_142075af0 56b0 args");
        cases += 1;
    }
    Ok(cases)
}

// REGISTRY:
// Replacement::verified(0x142072710, "d3d::d3d_unknown_142072710", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072710 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072710),
// Replacement::verified(0x142072910, "d3d::d3d_unknown_142072910", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072910 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072910),
// Replacement::verified(0x142072940, "d3d::d3d_unknown_142072940", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072940 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072940),
// Replacement::verified(0x142072990, "d3d::d3d_unknown_142072990", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072990 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072990),
// Replacement::verified(0x142072a30, "d3d::d3d_unknown_142072a30", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072a30 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072a30),
// Replacement::verified(0x142072a60, "d3d::d3d_unknown_142072a60", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072a60 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072a60),
// Replacement::verified_throws(0x142072a70, "d3d::d3d_unknown_142072a70", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072a70 as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072a70),
// Replacement::verified_throws(0x142072ad0, "d3d::d3d_unknown_142072ad0", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072ad0 as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072ad0),
// Replacement::verified_throws(0x142072b80, "d3d::d3d_unknown_142072b80", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072b80 as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072b80),
// Replacement::verified(0x142072c10, "d3d::d3d_unknown_142072c10", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072c10 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072c10),
// Replacement::verified(0x142072de0, "d3d::d3d_unknown_142072de0", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072de0 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072de0),
// Replacement::verified_throws(0x142072e80, "d3d::d3d_unknown_142072e80", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142072e80 as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142072e80),
// Replacement::verified(0x142073120, "d3d::d3d_unknown_142073120", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142073120 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142073120),
// Replacement::verified(0x142073490, "d3d::d3d_unknown_142073490", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142073490 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142073490),
// Replacement::verified(0x142073640, "d3d::d3d_unknown_142073640", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142073640 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142073640),
// Replacement::verified(0x142073660, "d3d::d3d_unknown_142073660", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142073660 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142073660),
// Replacement::verified(0x142073700, "d3d::d3d_unknown_142073700", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142073700 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142073700),
// Replacement::verified(0x142073740, "d3d::d3d_unknown_142073740", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142073740 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142073740),
// Replacement::verified(0x1420737b0, "d3d::d3d_unknown_1420737b0", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_1420737b0 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_1420737b0),
// Replacement::verified(0x142073c40, "d3d::d3d_unknown_142073c40", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142073c40 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142073c40),
// Replacement::verified(0x142073f10, "d3d::d3d_unknown_142073f10", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142073f10 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142073f10),
// Replacement::verified(0x1420741e0, "d3d::d3d_unknown_1420741e0", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_1420741e0 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_1420741e0),
// Replacement::verified(0x142074270, "d3d::d3d_unknown_142074270", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142074270 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142074270),
// Replacement::verified(0x1420744a0, "d3d::d3d_unknown_1420744a0", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_1420744a0 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_1420744a0),
// Replacement::verified(0x142074cd0, "d3d::d3d_unknown_142074cd0", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142074cd0 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142074cd0),
// Replacement::verified_throws(0x142074d10, "d3d::d3d_unknown_142074d10", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142074d10 as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142074d10),
// Replacement::verified(0x142075030, "d3d::d3d_unknown_142075030", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142075030 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142075030),
// Replacement::verified(0x142075850, "d3d::d3d_unknown_142075850", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142075850 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142075850),
// Replacement::verified_throws(0x1420758f0, "d3d::d3d_unknown_1420758f0", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_1420758f0 as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_1420758f0),
// Replacement::verified(0x142075970, "d3d::d3d_unknown_142075970", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142075970 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142075970),
// Replacement::verified(0x142075a20, "d3d::d3d_unknown_142075a20", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142075a20 as *const (), crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142075a20),
// Replacement::verified_throws(0x142075af0, "d3d::d3d_unknown_142075af0", crate::ns::rend::d3d_cmd_ctx_tail::d3d_unknown_142075af0 as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx_tail::diff_d3d_unknown_142075af0),

