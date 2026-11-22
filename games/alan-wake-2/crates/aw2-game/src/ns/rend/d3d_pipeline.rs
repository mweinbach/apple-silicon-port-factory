use crate::exe::{self, rd, wr};

/// Original: `0x14205e9c0` (133 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14205e9c0(this: usize) {
    let lock_addr = exe::ptr::<u8>(0x1_4385_7ed0) as usize;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let owner_addr = exe::ptr::<u32>(0x1_4385_7ed8) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_7edc) as usize;

    if rd::<u32>(owner_addr) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_addr) -> ());
        wr::<u32>(owner_addr, tid);
    }
    wr::<u32>(depth_addr, rd::<u32>(depth_addr).wrapping_add(1));

    let tls0 = rd::<usize>(exe::tls_pointer());
    let slot = tls0.wrapping_add(0x6360);
    exe::call_va!(0x1_411d_9eb0, (this, slot) -> ());
    wr::<u64>(slot, 0);

    let next_depth = rd::<u32>(depth_addr).wrapping_sub(1);
    wr::<u32>(depth_addr, next_depth);
    if next_depth == 0 {
        wr::<u32>(owner_addr, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (lock_addr) -> ());
    }
}

/// Original: `0x1420617a0` (198 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420617a0(
    _rcx: usize,
    out: usize,
    r8d: u32,
    r9d: u32,
    _arg5: usize,
    desc_ptr: usize,
) -> usize {
    let packed = rd::<u64>(desc_ptr);
    if r8d == 0 {
        wr::<u64>(out, 0);
        wr::<u64>(out + 0x8, 0);
        return out;
    }

    let mut frame = [0u8; 0x60];
    let base = frame.as_mut_ptr() as usize;
    let out_slot = base;
    let req_slot = base + 0x10;
    let desc_slot = base + 0x28;

    wr::<u64>(out_slot, 0);
    wr::<u64>(out_slot + 0x8, 0);

    wr::<u32>(req_slot, 0x1_0018);
    wr::<usize>(req_slot + 0x8, desc_slot);
    wr::<usize>(req_slot + 0x10, out_slot);

    wr::<u32>(desc_slot, r8d);
    wr::<u32>(desc_slot + 0x4, r9d);
    wr::<u32>(desc_slot + 0x8, 1);
    wr::<u32>(desc_slot + 0xc, 1);
    wr::<u64>(desc_slot + 0x10, packed);
    wr::<u64>(desc_slot + 0x18, 0);
    wr::<u64>(desc_slot + 0x20, 0);
    wr::<u64>(desc_slot + 0x28, 0);

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc90) as usize);
    exe::call_va!(0x1_4000_17a0, (dev, req_slot) -> ());

    wr::<u64>(out, rd::<u64>(out_slot));
    wr::<u64>(out + 0x8, rd::<u64>(out_slot + 0x8));
    out
}

/// Differential test for `d3d_1420617a0` (early-out path when `r8d == 0`).
pub unsafe fn diff_d3d_1420617a0(orig: usize, new: usize) -> Result<u32, String> {
    let desc_o = [0x1122_3344_5566_7788u64];
    let desc_n = desc_o;
    let mut out_o = [0xdead_beef_cafe_babeu64; 2];
    let mut out_n = out_o;

    let ret_o = exe::call_va!(
        orig,
        (
            0usize,
            out_o.as_mut_ptr() as usize,
            0u32,
            7u32,
            0usize,
            desc_o.as_ptr() as usize
        ) -> usize
    );
    let ret_n = exe::call_va!(
        new,
        (
            0usize,
            out_n.as_mut_ptr() as usize,
            0u32,
            7u32,
            0usize,
            desc_n.as_ptr() as usize
        ) -> usize
    );

    crate::diff_eq!(ret_o == out_o.as_ptr() as usize, ret_n == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142061870` (262 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142061870(
    _rcx: usize,
    out: usize,
    r8d: u32,
    r9d: u32,
    _arg5: usize,
    desc_ptr: usize,
) -> usize {
    let q0 = rd::<u64>(desc_ptr);
    let q1 = rd::<u64>(desc_ptr + 0x8);
    let q2 = rd::<u64>(desc_ptr + 0x10);
    let q3 = rd::<u64>(desc_ptr + 0x18);

    if r8d == 0 {
        wr::<u64>(out, 0);
        wr::<u64>(out + 0x8, 0);
        return out;
    }

    let mut frame = [0u8; 0x60];
    let base = frame.as_mut_ptr() as usize;
    let out_slot = base;
    let req_slot = base + 0x10;
    let desc_slot = base + 0x28;

    wr::<u64>(out_slot, 0);
    wr::<u64>(out_slot + 0x8, 0);

    let kind_byte = (q0 & 0xff) as u8;
    let mapped_kind = if kind_byte == 2 {
        6u32
    } else if kind_byte == 0xa {
        0xdu32
    } else {
        0u32
    };

    wr::<u32>(desc_slot, r8d);
    wr::<u32>(desc_slot + 0x4, r9d);
    wr::<u32>(desc_slot + 0x8, 4);
    wr::<u32>(desc_slot + 0xc, 1);
    wr::<u32>(desc_slot + 0x10, mapped_kind);
    wr::<u32>(desc_slot + 0x14, (q0 >> 32) as u32);
    wr::<u64>(desc_slot + 0x18, q1);
    wr::<u64>(desc_slot + 0x20, q2);
    wr::<u64>(desc_slot + 0x28, q3);

    wr::<u32>(req_slot, 0x1_0018);
    wr::<usize>(req_slot + 0x8, desc_slot);
    wr::<usize>(req_slot + 0x10, out_slot);

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc90) as usize);
    exe::call_va!(0x1_4000_17a0, (dev, req_slot) -> ());

    wr::<u64>(out, rd::<u64>(out_slot));
    wr::<u64>(out + 0x8, rd::<u64>(out_slot + 0x8));
    out
}

/// Differential test for `d3d_142061870` (early-out path when `r8d == 0`).
pub unsafe fn diff_d3d_142061870(orig: usize, new: usize) -> Result<u32, String> {
    let desc_o = [1u64, 2, 3, 4];
    let desc_n = desc_o;
    let mut out_o = [0xdead_beef_cafe_babeu64; 2];
    let mut out_n = out_o;

    let ret_o = exe::call_va!(
        orig,
        (
            0usize,
            out_o.as_mut_ptr() as usize,
            0u32,
            9u32,
            0usize,
            desc_o.as_ptr() as usize
        ) -> usize
    );
    let ret_n = exe::call_va!(
        new,
        (
            0usize,
            out_n.as_mut_ptr() as usize,
            0u32,
            9u32,
            0usize,
            desc_n.as_ptr() as usize
        ) -> usize
    );

    crate::diff_eq!(ret_o == out_o.as_ptr() as usize, ret_n == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142061980` (262 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142061980(
    _rcx: usize,
    out: usize,
    r8d: u32,
    r9d: u32,
    _arg5: usize,
    desc_ptr: usize,
) -> usize {
    let q0 = rd::<u64>(desc_ptr);
    let q1 = rd::<u64>(desc_ptr + 0x8);
    let q2 = rd::<u64>(desc_ptr + 0x10);
    let q3 = rd::<u64>(desc_ptr + 0x18);

    if r8d == 0 {
        wr::<u64>(out, 0);
        wr::<u64>(out + 0x8, 0);
        return out;
    }

    let mut frame = [0u8; 0x60];
    let base = frame.as_mut_ptr() as usize;
    let out_slot = base;
    let req_slot = base + 0x10;
    let desc_slot = base + 0x28;

    wr::<u64>(out_slot, 0);
    wr::<u64>(out_slot + 0x8, 0);

    let kind_byte = (q0 & 0xff) as u8;
    let mapped_kind = if kind_byte == 2 {
        6u32
    } else if kind_byte == 0xa {
        0xdu32
    } else {
        0u32
    };

    wr::<u32>(desc_slot, r8d);
    wr::<u32>(desc_slot + 0x4, r9d);
    wr::<u32>(desc_slot + 0x8, 3);
    wr::<u32>(desc_slot + 0xc, 1);
    wr::<u32>(desc_slot + 0x10, mapped_kind);
    wr::<u32>(desc_slot + 0x14, (q0 >> 32) as u32);
    wr::<u64>(desc_slot + 0x18, q1);
    wr::<u64>(desc_slot + 0x20, q2);
    wr::<u64>(desc_slot + 0x28, q3);

    wr::<u32>(req_slot, 0x1_0018);
    wr::<usize>(req_slot + 0x8, desc_slot);
    wr::<usize>(req_slot + 0x10, out_slot);

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc90) as usize);
    exe::call_va!(0x1_4000_17a0, (dev, req_slot) -> ());

    wr::<u64>(out, rd::<u64>(out_slot));
    wr::<u64>(out + 0x8, rd::<u64>(out_slot + 0x8));
    out
}

/// Differential test for `d3d_142061980` (early-out path when `r8d == 0`).
pub unsafe fn diff_d3d_142061980(orig: usize, new: usize) -> Result<u32, String> {
    let desc_o = [1u64, 2, 3, 4];
    let desc_n = desc_o;
    let mut out_o = [0xdead_beef_cafe_babeu64; 2];
    let mut out_n = out_o;

    let ret_o = exe::call_va!(
        orig,
        (
            0usize,
            out_o.as_mut_ptr() as usize,
            0u32,
            9u32,
            0usize,
            desc_o.as_ptr() as usize
        ) -> usize
    );
    let ret_n = exe::call_va!(
        new,
        (
            0usize,
            out_n.as_mut_ptr() as usize,
            0u32,
            9u32,
            0usize,
            desc_n.as_ptr() as usize
        ) -> usize
    );

    crate::diff_eq!(ret_o == out_o.as_ptr() as usize, ret_n == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142061b80` (174 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142061b80(
    out: usize,
    edx: u32,
    r8d: u32,
    r9d: u32,
    _arg5: usize,
    desc_ptr: usize,
) -> usize {
    if edx == 0 {
        wr::<u64>(out, 0);
        wr::<u64>(out + 0x8, 0);
        return out;
    }

    let mut frame = [0u8; 0x60];
    let base = frame.as_mut_ptr() as usize;
    let out_slot = base;
    let req_slot = base + 0x10;
    let desc_slot = base + 0x28;

    wr::<u64>(out_slot, 0);
    wr::<u64>(out_slot + 0x8, 0);

    wr::<u32>(desc_slot, edx);
    wr::<u32>(desc_slot + 0x4, r8d);
    wr::<u32>(desc_slot + 0x8, r9d);
    wr::<u32>(desc_slot + 0xc, 1);
    wr::<u64>(desc_slot + 0x10, 0);
    wr::<u64>(desc_slot + 0x18, 0);
    wr::<u64>(desc_slot + 0x20, 0);
    wr::<u64>(desc_slot + 0x28, 0);

    exe::call_va!(0x1_4206_1c40, (desc_slot, desc_ptr) -> ());

    wr::<u32>(req_slot, 0x1_0018);
    wr::<usize>(req_slot + 0x8, desc_slot);
    wr::<usize>(req_slot + 0x10, out_slot);

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc90) as usize);
    exe::call_va!(0x1_4000_17a0, (dev, req_slot) -> ());

    wr::<u64>(out, rd::<u64>(out_slot));
    wr::<u64>(out + 0x8, rd::<u64>(out_slot + 0x8));
    out
}

/// Differential test for `d3d_142061b80` (early-out path when `edx == 0`).
pub unsafe fn diff_d3d_142061b80(orig: usize, new: usize) -> Result<u32, String> {
    let mut out_o = [0xdead_beef_cafe_babeu64; 2];
    let mut out_n = out_o;

    let ret_o = exe::call_va!(
        orig,
        (out_o.as_mut_ptr() as usize, 0u32, 1u32, 2u32, 0usize, 0usize) -> usize
    );
    let ret_n = exe::call_va!(
        new,
        (out_n.as_mut_ptr() as usize, 0u32, 1u32, 2u32, 0usize, 0usize) -> usize
    );

    crate::diff_eq!(ret_o == out_o.as_ptr() as usize, ret_n == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142061ec0` (154 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142061ec0(
    _rcx: usize,
    dl: u8,
    r8: usize,
    _r9: usize,
    arg5: u32,
    arg6_ptr: usize,
    arg7_ptr: usize,
) {
    let mut frame = [0u8; 0x48];
    let base = frame.as_mut_ptr() as usize;
    let ctx_slot = base;
    let cmd_desc = base + 0x10;

    let r10 = rd::<u64>(arg6_ptr);
    wr::<u64>(cmd_desc, r10);
    wr::<u32>(cmd_desc + 0x8, 0);
    wr::<u32>(cmd_desc + 0xc, dl as u32);
    wr::<u32>(cmd_desc + 0x10, arg5);
    wr::<u32>(cmd_desc + 0x14, 0);
    wr::<u64>(cmd_desc + 0x18, rd::<u64>(r8));
    wr::<u64>(cmd_desc + 0x20, if (dl & 0x20) != 0 { r10 } else { 0 });
    wr::<u64>(cmd_desc + 0x28, rd::<u64>(arg7_ptr));

    let ctx_ret = exe::call_va!(0x1_411d_b050, (ctx_slot) -> usize);
    let cmd = rd::<usize>(ctx_ret + 0x8);
    let vtbl = rd::<usize>(cmd);
    let fn_ptr = rd::<usize>(vtbl + 0x240);
    exe::call_va!(fn_ptr, (cmd, cmd_desc, 0usize, 0usize) -> ());

    let lock_ptr = rd::<usize>(ctx_slot);
    if lock_ptr != 0 {
        let next = rd::<u32>(lock_ptr + 0xc).wrapping_sub(1);
        wr::<u32>(lock_ptr + 0xc, next);
        if next == 0 {
            wr::<u32>(lock_ptr + 0x8, 0xffff_ffff);
            exe::call_iat!(0x1_4264_5598, (lock_ptr) -> ());
        }
    }
}

/// Original: `0x142061f60` (264 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142061f60(
    _rcx: usize,
    out: usize,
    r8: usize,
    r9: usize,
    arg5: u64,
    _arg6: usize,
    arg7_ptr: usize,
    arg8: u64,
) -> usize {
    let mut frame = [0u8; 0x70];
    let base = frame.as_mut_ptr() as usize;
    let ctx_slot = base;
    let inner_desc = base + 0x10;
    let outer_desc = base + 0x38;

    wr::<u32>(inner_desc, rd::<u32>(r8 + 0x4));
    wr::<u32>(inner_desc + 0x4, 0);
    wr::<u64>(inner_desc + 0x8, rd::<u64>(r8 + 0x8));

    let p10 = rd::<usize>(r8 + 0x10);
    wr::<u64>(
        inner_desc + 0x10,
        rd::<u64>(p10).wrapping_add(rd::<u64>(r8 + 0x18)),
    );

    let p20 = rd::<usize>(r8 + 0x20);
    wr::<u64>(
        inner_desc + 0x18,
        rd::<u64>(p20).wrapping_add(rd::<u64>(r8 + 0x28)),
    );
    wr::<u64>(inner_desc + 0x20, 8);

    wr::<u64>(outer_desc, rd::<u64>(r9).wrapping_add(arg5));
    wr::<u32>(outer_desc + 0x8, 2);
    wr::<u32>(outer_desc + 0xc, rd::<u8>(r8) as u32);
    wr::<u64>(outer_desc + 0x10, 1);
    wr::<usize>(outer_desc + 0x18, inner_desc);
    wr::<u64>(outer_desc + 0x20, 0);
    wr::<u64>(outer_desc + 0x28, rd::<u64>(arg7_ptr).wrapping_add(arg8));

    let ctx_ret = exe::call_va!(0x1_411d_b050, (ctx_slot) -> usize);
    let cmd = rd::<usize>(ctx_ret + 0x8);
    let vtbl = rd::<usize>(cmd);
    let fn_ptr = rd::<usize>(vtbl + 0x240);
    exe::call_va!(fn_ptr, (cmd, outer_desc, 0usize, 0usize) -> ());

    let lock_ptr = rd::<usize>(ctx_slot);
    if lock_ptr != 0 {
        let next = rd::<u32>(lock_ptr + 0xc).wrapping_sub(1);
        wr::<u32>(lock_ptr + 0xc, next);
        if next == 0 {
            wr::<u32>(lock_ptr + 0x8, 0xffff_ffff);
            exe::call_iat!(0x1_4264_5598, (lock_ptr) -> ());
        }
    }

    wr::<u32>(out + 0x8, 0xffff_ffff);
    wr::<u32>(out + 0xc, 0);
    wr::<u64>(out, rd::<u64>(r9));
    out
}

/// Original: `0x142062080` (107 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142062080(
    _rcx: usize,
    rdx: usize,
    r8: u64,
    _r9: usize,
    arg5_ptr: usize,
    arg6_off: u64,
) {
    let dst = rd::<u64>(rdx).wrapping_add(r8);
    let src = rd::<u64>(arg5_ptr).wrapping_add(arg6_off);

    let mut ctx_slot = [0u64; 2];
    let ctx_ptr = ctx_slot.as_mut_ptr() as usize;
    let ctx_ret = exe::call_va!(0x1_411d_b050, (ctx_ptr) -> usize);
    let cmd = rd::<usize>(ctx_ret + 0x8);
    let vtbl = rd::<usize>(cmd);
    let fn_ptr = rd::<usize>(vtbl + 0x250);
    exe::call_va!(fn_ptr, (cmd, dst, src, 1u32) -> ());

    let lock_ptr = rd::<usize>(ctx_ptr);
    if lock_ptr != 0 {
        let next = rd::<u32>(lock_ptr + 0xc).wrapping_sub(1);
        wr::<u32>(lock_ptr + 0xc, next);
        if next == 0 {
            wr::<u32>(lock_ptr + 0x8, 0xffff_ffff);
            exe::call_iat!(0x1_4264_5598, (lock_ptr) -> ());
        }
    }
}

/// Original: `0x142062630` (123 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142062630(
    _rcx: usize,
    out: usize,
    r8b: u8,
    r9: u64,
) -> usize {
    if r9 == 0 {
        wr::<u64>(out, 0);
        wr::<u64>(out + 0x8, 0);
        wr::<u64>(out + 0x10, 0);
        return out;
    }

    let mut frame = [0u8; 0x30];
    let base = frame.as_mut_ptr() as usize;
    let req_slot = base;
    let out_slot = base + 0x18;

    wr::<u32>(req_slot, 0);
    wr::<u32>(req_slot + 0x4, r8b as u32);
    wr::<u32>(req_slot + 0x8, r9 as u32);
    wr::<u32>(req_slot + 0xc, 0);
    wr::<u64>(req_slot + 0x10, 0);

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc90) as usize);
    let vtbl = rd::<usize>(dev);
    let fn_ptr = rd::<usize>(vtbl + 0x1f8);
    exe::call_va!(fn_ptr, (dev, req_slot, out_slot) -> ());

    wr::<u64>(out, rd::<u64>(out_slot));
    wr::<u64>(out + 0x8, rd::<u64>(out_slot + 0x8));
    wr::<u64>(out + 0x10, rd::<u64>(out_slot + 0x10));
    out
}

/// Differential test for `d3d_142062630` (early-out path when `r9 == 0`).
pub unsafe fn diff_d3d_142062630(orig: usize, new: usize) -> Result<u32, String> {
    let mut out_o = [0xdead_beef_cafe_babeu64; 3];
    let mut out_n = out_o;

    let ret_o = exe::call_va!(orig, (0usize, out_o.as_mut_ptr() as usize, 3u8, 0u64) -> usize);
    let ret_n = exe::call_va!(new, (0usize, out_n.as_mut_ptr() as usize, 3u8, 0u64) -> usize);

    crate::diff_eq!(ret_o == out_o.as_ptr() as usize, ret_n == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x1420626c0` (185 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420626c0(_rcx: usize, out: usize, r8: usize) -> usize {
    let mut frame = [0u8; 0x58];
    let base = frame.as_mut_ptr() as usize;
    let outer_desc = base;
    let out_slot = base + 0x18;
    let inner_desc = base + 0x30;

    wr::<u32>(inner_desc, rd::<u32>(r8 + 0x4));
    wr::<u32>(inner_desc + 0x4, 0);
    wr::<u64>(inner_desc + 0x8, rd::<u64>(r8 + 0x8));

    let p10 = rd::<usize>(r8 + 0x10);
    wr::<u64>(
        inner_desc + 0x10,
        rd::<u64>(p10).wrapping_add(rd::<u64>(r8 + 0x18)),
    );

    let p20 = rd::<usize>(r8 + 0x20);
    wr::<u64>(
        inner_desc + 0x18,
        rd::<u64>(p20).wrapping_add(rd::<u64>(r8 + 0x28)),
    );
    wr::<u64>(inner_desc + 0x20, 8);

    wr::<u64>(out_slot, 0);
    wr::<u64>(out_slot + 0x8, 0);

    wr::<u32>(outer_desc, 2);
    wr::<u32>(outer_desc + 0x4, rd::<u8>(r8) as u32);
    wr::<u64>(outer_desc + 0x8, 1);
    wr::<usize>(outer_desc + 0x10, inner_desc);
    wr::<u64>(outer_desc + 0x28, 0);

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc90) as usize);
    let vtbl = rd::<usize>(dev);
    let fn_ptr = rd::<usize>(vtbl + 0x1f8);
    exe::call_va!(fn_ptr, (dev, outer_desc, out_slot) -> ());

    wr::<u64>(out, rd::<u64>(out_slot));
    wr::<u64>(out + 0x8, rd::<u64>(out_slot + 0x8));
    wr::<u64>(out + 0x10, 0);
    out
}

/// Original: `0x1420627a0` (406 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420627a0(
    _rcx: usize,
    out: usize,
    r8b: u8,
    r9: usize,
) -> usize {
    let mut frame = [0u8; 0x420];
    let base = frame.as_mut_ptr() as usize;
    let dyn_buf_slot = base;
    let req_slot = base + 0x10;
    let helper_b_slot = base + 0x28;
    let helper_a_slot = base + 0x38;
    let out_slot = base + 0x48;
    let tmp_entry_slot = base + 0x60;
    let inline_entries = base + 0xa0;

    wr::<u64>(dyn_buf_slot, 0);
    wr::<u64>(dyn_buf_slot + 0x8, 0);

    let count = rd::<u64>(r9) as usize;
    let mut entries_ptr = inline_entries;

    exe::call_va!(0x1_411d_b3a0, (helper_a_slot, count) -> ());
    exe::call_va!(0x1_411d_b4d0, (helper_b_slot, count) -> ());

    if count > 0x10 {
        exe::call_va!(0x1_411d_b2d0, (dyn_buf_slot, count) -> ());
        entries_ptr = rd::<usize>(dyn_buf_slot);
    }

    if count != 0 {
        let mut i: i32 = 0;
        let mut dst = entries_ptr;
        loop {
            let idx = i as isize as usize;
            let src_item = rd::<usize>(r9 + 0x8).wrapping_add(idx.wrapping_mul(0x88));
            let arg_a = rd::<usize>(helper_a_slot).wrapping_add(idx.wrapping_mul(0x30));
            let arg_b = rd::<usize>(helper_b_slot).wrapping_add(idx.wrapping_mul(0x20));
            let built = exe::call_va!(
                0x1_411d_ae50,
                (tmp_entry_slot, src_item, arg_a, arg_b) -> usize
            );
            for q in 0..7usize {
                wr::<u64>(dst + q * 8, rd::<u64>(built + q * 8));
            }
            i = i.wrapping_add(1);
            dst = dst.wrapping_add(0x38);
            if (i as isize as usize) >= count {
                break;
            }
        }
    }

    wr::<u32>(req_slot, 1);
    wr::<u32>(req_slot + 0x4, r8b as u32);
    wr::<u32>(req_slot + 0x8, rd::<u32>(r9));
    wr::<u32>(req_slot + 0xc, 0);
    wr::<usize>(req_slot + 0x10, if count == 0 { 0 } else { entries_ptr });

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc90) as usize);
    let vtbl = rd::<usize>(dev);
    let fn_ptr = rd::<usize>(vtbl + 0x1f8);
    exe::call_va!(fn_ptr, (dev, req_slot, out_slot) -> ());

    wr::<u64>(out, rd::<u64>(out_slot));
    wr::<u64>(out + 0x8, rd::<u64>(out_slot + 0x8));
    wr::<u64>(out + 0x10, rd::<u64>(out_slot + 0x10));

    exe::call_va!(0x1_401e_59a0, (helper_b_slot) -> ());
    exe::call_va!(0x1_401e_59a0, (helper_a_slot) -> ());
    exe::call_va!(0x1_401d_e830, (dyn_buf_slot) -> ());

    out
}

/// Original: `0x142063050` (295 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142063050(out: usize, desc: usize) -> usize {
    let b9 = rd::<u8>(desc + 0x9);
    let b8 = rd::<u8>(desc + 0x8);
    let d0 = rd::<u32>(desc);
    let d4 = rd::<u32>(desc + 0x4);

    let mut flags = if b9 != 0 { 4u32 } else { 0u32 };
    flags |= d0;
    flags <<= 2;
    flags |= d4;
    flags = flags.wrapping_add(flags);
    if b8 != 0 {
        flags |= 1;
    }
    wr::<u32>(out, flags);

    let f_c = rd::<f32>(desc + 0xc);
    wr::<f32>(out + 0x4, f_c);

    let u_10 = rd::<u32>(desc + 0x10);
    wr::<u32>(out + 0x8, u_10);
    let f_10 = f32::from_bits(u_10);

    let tbl0 = exe::ptr::<u32>(0x1_4297_ed78) as isize;
    let tbl1 = exe::ptr::<u32>(0x1_4297_ed68) as isize;
    let esi = rd::<u32>((tbl0 + (d0 as i32 as isize) * 4) as usize);
    let ebp = rd::<u32>((tbl1 + (d4 as i32 as isize) * 4) as usize);

    let base_val = rd::<f32>(exe::ptr::<f32>(0x1_4342_6abc) as usize);
    let exp_val = rd::<f32>(exe::ptr::<f32>(0x1_4342_6e20) as usize);
    let powf_fn: unsafe extern "win64" fn(f32, f32) -> f32 = exe::func(0x1_4183_ad46);
    let pow_val = powf_fn(base_val, exp_val);
    let scaled = (pow_val * f_c) as i32 as u32;

    let flag_34 = if b9 != 0 {
        let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
        if rd::<u8>(g + 0x9d) != 0 {
            1u32
        } else {
            0u32
        }
    } else {
        0u32
    };

    wr::<u32>(out + 0xc, esi);
    wr::<u32>(out + 0x10, ebp);
    wr::<u32>(out + 0x14, 0);
    wr::<u32>(out + 0x18, scaled);
    wr::<u32>(out + 0x1c, 0);
    wr::<f32>(out + 0x20, f_10);
    wr::<u32>(out + 0x24, 1);
    wr::<u64>(out + 0x28, 1);
    wr::<u32>(out + 0x30, 0);
    wr::<u32>(out + 0x34, flag_34);

    let hash = exe::call_va!(0x1_4205_5890, (desc) -> u64);
    wr::<u64>(out + 0x38, hash);
    out
}

/// Differential test for `d3d_142063050` (with `desc + 0x9 == 0` so no global pointer deref occurs).
pub unsafe fn diff_d3d_142063050(orig: usize, new: usize) -> Result<u32, String> {
    let mut desc_o = [0u8; 0x20];
    let desc_o_ptr = desc_o.as_mut_ptr() as usize;
    wr::<u32>(desc_o_ptr, 1);
    wr::<u32>(desc_o_ptr + 0x4, 2);
    wr::<u8>(desc_o_ptr + 0x8, 1);
    wr::<u8>(desc_o_ptr + 0x9, 0);
    wr::<f32>(desc_o_ptr + 0xc, 4.0);
    wr::<f32>(desc_o_ptr + 0x10, 0.5);
    let desc_n = desc_o;

    let mut out_o = [0u64; 8];
    let mut out_n = [0u64; 8];

    let ret_o = exe::call_va!(orig, (out_o.as_mut_ptr() as usize, desc_o.as_ptr() as usize) -> usize);
    let ret_n = exe::call_va!(new, (out_n.as_mut_ptr() as usize, desc_n.as_ptr() as usize) -> usize);

    crate::diff_eq!(ret_o == out_o.as_ptr() as usize, ret_n == out_n.as_ptr() as usize, "ret");
    crate::diff_eq!(out_o, out_n, "out");
    Ok(1)
}

/// Original: `0x142063650` (255 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142063650(this: usize) -> usize {
    exe::call_va!(0x1_4205_6c10, (this) -> ());

    wr::<usize>(this, exe::ptr::<u8>(0x1_433d_c950) as usize);
    wr::<u32>(this + 0x10, 0xffff_ffff);
    wr::<u32>(this + 0x14, 0);
    wr::<u64>(this + 0x8, 0);

    let ctor = exe::ptr::<u8>(0x1_411d_b850) as usize;
    let dtor = exe::ptr::<u8>(0x1_4034_96f0) as usize;
    exe::call_va!(0x1_4183_4fc0, (this + 0x18, 0x18usize, 2usize, ctor, dtor) -> ());
    exe::call_va!(0x1_4183_4fc0, (this + 0x48, 0x18usize, 2usize, ctor, dtor) -> ());
    exe::call_va!(0x1_4183_4fc0, (this + 0x78, 0x18usize, 2usize, ctor, dtor) -> ());

    let g = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let c38 = rd::<u64>(g + 0x38);
    let c40 = rd::<u64>(g + 0x40);

    let mut pair = [c40, c38];
    let mut idx = 0u32;
    let pair_ptr = pair.as_mut_ptr() as usize;
    let idx_ptr = &mut idx as *mut u32 as usize;

    let a8 = exe::call_va!(0x1_411d_b770, (c38 as usize, 0usize, idx_ptr, pair_ptr) -> u64);
    wr::<u64>(this + 0xa8, a8);

    wr::<u32>(idx_ptr, 1);
    let b0 = exe::call_va!(0x1_411d_b770, (c38 as usize, 0usize, idx_ptr, pair_ptr) -> u64);
    wr::<u64>(this + 0xb0, b0);

    this
}

pub unsafe fn diff_d3d_14205e9c0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_1411d9eb0(_rcx: usize, slot: usize) {
        wr::<u64>(slot + 8, rd::<u64>(slot).wrapping_add(0x2222_3333));
    }

    let _m = crate::difftest::Mock::new(0x1_411d_9eb0, fake_1411d9eb0 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let owner_addr = exe::ptr::<u32>(0x1_4385_7ed8) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_7edc) as usize;
    let s_owner = rd::<u32>(owner_addr);
    let s_depth = rd::<u32>(depth_addr);

    let tls_slot = exe::tls_pointer();
    let s_tls0 = rd::<usize>(tls_slot);

    let mut tls_o = vec![0u8; 0x6380];
    let mut tls_n = vec![0u8; 0x6380];
    wr::<u64>(tls_o.as_mut_ptr() as usize + 0x6360, 0x1234_5678_9abc_def0);
    wr::<u64>(tls_n.as_mut_ptr() as usize + 0x6360, 0x1234_5678_9abc_def0);

    wr::<u32>(owner_addr, tid);
    wr::<u32>(depth_addr, 1);

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0);
    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0);

    wr::<usize>(tls_slot, s_tls0);
    wr::<u32>(owner_addr, s_owner);
    wr::<u32>(depth_addr, s_depth);

    crate::diff_eq!(
        &tls_o[0x6360..0x6370],
        &tls_n[0x6360..0x6370],
        "d3d_14205e9c0 tls slot"
    );
    Ok(1)
}

unsafe extern "win64" fn fake_cmd_240_30(cmd: usize, desc: usize, r8: usize, r9: usize) {
    let mut h = (r8 as u64).wrapping_mul(31).wrapping_add(r9 as u64);
    for i in 0..6usize {
        h = h
            .wrapping_mul(1099511628211)
            .wrapping_add(rd::<u64>(desc + i * 8));
    }
    wr::<u64>(cmd + 8, h);
}

pub unsafe fn diff_d3d_142061ec0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8, usize, usize, u32, usize, usize);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 80];
    vtbl[0x240 / 8] = fake_cmd_240_30 as *const () as usize;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let tls_slot = exe::tls_pointer();
    let s_tls0 = rd::<usize>(tls_slot);

    let r8_val = [0x1111_2222_3333_4444u64];
    let arg6_val = [0x5555_6666_7777_8888u64];
    let arg7_val = [0x9999_aaaa_bbbb_ccccu64];
    let mut ran = 0u32;

    for dl in [0x05u8, 0x25u8] {
        let mut cmd_o = [vtbl.as_ptr() as u64, 0u64];
        let mut cmd_n = [vtbl.as_ptr() as u64, 0u64];
        let mut lock_o = [0u32, 0, tid, 1];
        let mut lock_n = [0u32, 0, tid, 1];
        let mut holder_o = [0u64; 8];
        let mut holder_n = [0u64; 8];
        holder_o[6] = cmd_o.as_mut_ptr() as u64;
        holder_n[6] = cmd_n.as_mut_ptr() as u64;

        let mut tls_o = vec![0u8; 0x6340];
        let mut tls_n = vec![0u8; 0x6340];
        tls_o[0x70] = 1;
        tls_n[0x70] = 1;
        wr::<usize>(tls_o.as_mut_ptr() as usize + 0x6300, holder_o.as_mut_ptr() as usize);
        wr::<usize>(tls_o.as_mut_ptr() as usize + 0x6318, lock_o.as_mut_ptr() as usize);
        wr::<usize>(tls_n.as_mut_ptr() as usize + 0x6300, holder_n.as_mut_ptr() as usize);
        wr::<usize>(tls_n.as_mut_ptr() as usize + 0x6318, lock_n.as_mut_ptr() as usize);

        wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
        orig_fn(
            0,
            dl,
            r8_val.as_ptr() as usize,
            0,
            0x42,
            arg6_val.as_ptr() as usize,
            arg7_val.as_ptr() as usize,
        );

        wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
        new_fn(
            0,
            dl,
            r8_val.as_ptr() as usize,
            0,
            0x42,
            arg6_val.as_ptr() as usize,
            arg7_val.as_ptr() as usize,
        );

        wr::<usize>(tls_slot, s_tls0);

        crate::diff_eq!(cmd_o[1], cmd_n[1], "d3d_142061ec0 cmd_desc hash");
        crate::diff_eq!(lock_o, lock_n, "d3d_142061ec0 lock state");
        crate::diff_eq!(holder_o[3], holder_n[3], "d3d_142061ec0 holder count");
        ran += 1;
    }

    Ok(ran)
}

unsafe extern "win64" fn fake_cmd_240_nested(cmd: usize, outer: usize, r8: usize, r9: usize) {
    let inner = rd::<usize>(outer + 0x18);
    let mut h = (r8 as u64).wrapping_mul(31).wrapping_add(r9 as u64);
    for &off in &[0x00usize, 0x08, 0x10, 0x20, 0x28] {
        h = h
            .wrapping_mul(1099511628211)
            .wrapping_add(rd::<u64>(outer + off));
    }
    for i in 0..5usize {
        h = h
            .wrapping_mul(1099511628211)
            .wrapping_add(rd::<u64>(inner + i * 8));
    }
    wr::<u64>(cmd + 8, h);
}

pub unsafe fn diff_d3d_142061f60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64, usize, usize, u64) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 80];
    vtbl[0x240 / 8] = fake_cmd_240_nested as *const () as usize;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let tls_slot = exe::tls_pointer();
    let s_tls0 = rd::<usize>(tls_slot);

    let p10_target = [0x1000_0000u64];
    let p20_target = [0x2000_0000u64];
    let mut r8_buf = [0u8; 0x30];
    let r8_ptr = r8_buf.as_mut_ptr() as usize;
    wr::<u8>(r8_ptr, 7);
    wr::<u32>(r8_ptr + 0x04, 0x1234);
    wr::<u64>(r8_ptr + 0x08, 0x5678_9abc);
    wr::<usize>(r8_ptr + 0x10, p10_target.as_ptr() as usize);
    wr::<u64>(r8_ptr + 0x18, 0x100);
    wr::<usize>(r8_ptr + 0x20, p20_target.as_ptr() as usize);
    wr::<u64>(r8_ptr + 0x28, 0x200);

    let r9_val = [0x3000_0000u64];
    let arg7_val = [0x4000_0000u64];

    let mut cmd_o = [vtbl.as_ptr() as u64, 0u64];
    let mut cmd_n = [vtbl.as_ptr() as u64, 0u64];
    let mut lock_o = [0u32, 0, tid, 1];
    let mut lock_n = [0u32, 0, tid, 1];
    let mut holder_o = [0u64; 8];
    let mut holder_n = [0u64; 8];
    holder_o[6] = cmd_o.as_mut_ptr() as u64;
    holder_n[6] = cmd_n.as_mut_ptr() as u64;

    let mut tls_o = vec![0u8; 0x6340];
    let mut tls_n = vec![0u8; 0x6340];
    tls_o[0x70] = 1;
    tls_n[0x70] = 1;
    wr::<usize>(tls_o.as_mut_ptr() as usize + 0x6300, holder_o.as_mut_ptr() as usize);
    wr::<usize>(tls_o.as_mut_ptr() as usize + 0x6318, lock_o.as_mut_ptr() as usize);
    wr::<usize>(tls_n.as_mut_ptr() as usize + 0x6300, holder_n.as_mut_ptr() as usize);
    wr::<usize>(tls_n.as_mut_ptr() as usize + 0x6318, lock_n.as_mut_ptr() as usize);

    let mut out_o = [0xaa_u8; 0x10];
    let mut out_n = [0xaa_u8; 0x10];

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    let ro = orig_fn(
        0,
        out_o.as_mut_ptr() as usize,
        r8_ptr,
        r9_val.as_ptr() as usize,
        0x40,
        0,
        arg7_val.as_ptr() as usize,
        0x80,
    );

    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    let rn = new_fn(
        0,
        out_n.as_mut_ptr() as usize,
        r8_ptr,
        r9_val.as_ptr() as usize,
        0x40,
        0,
        arg7_val.as_ptr() as usize,
        0x80,
    );

    wr::<usize>(tls_slot, s_tls0);

    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "d3d_142061f60 out");
    crate::diff_eq!(cmd_o[1], cmd_n[1], "d3d_142061f60 desc hash");
    crate::diff_eq!(lock_o, lock_n, "d3d_142061f60 lock state");
    Ok(1)
}

pub unsafe fn diff_d3d_142062080(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_cmd_250(cmd: usize, dst: u64, src: u64, count: u32) {
        wr::<u64>(cmd + 8, dst);
        wr::<u64>(cmd + 16, src);
        wr::<u32>(cmd + 24, count);
    }

    type F = unsafe extern "win64" fn(usize, usize, u64, usize, usize, u64);
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 80];
    vtbl[0x250 / 8] = fake_cmd_250 as *const () as usize;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let tls_slot = exe::tls_pointer();
    let s_tls0 = rd::<usize>(tls_slot);

    let rdx_val = [0x1000_0000u64];
    let arg5_val = [0x2000_0000u64];

    let mut cmd_o = [vtbl.as_ptr() as u64, 0, 0, 0];
    let mut cmd_n = [vtbl.as_ptr() as u64, 0, 0, 0];
    let mut lock_o = [0u32, 0, tid, 1];
    let mut lock_n = [0u32, 0, tid, 1];
    let mut holder_o = [0u64; 8];
    let mut holder_n = [0u64; 8];
    holder_o[6] = cmd_o.as_mut_ptr() as u64;
    holder_n[6] = cmd_n.as_mut_ptr() as u64;

    let mut tls_o = vec![0u8; 0x6340];
    let mut tls_n = vec![0u8; 0x6340];
    tls_o[0x70] = 1;
    tls_n[0x70] = 1;
    wr::<usize>(tls_o.as_mut_ptr() as usize + 0x6300, holder_o.as_mut_ptr() as usize);
    wr::<usize>(tls_o.as_mut_ptr() as usize + 0x6318, lock_o.as_mut_ptr() as usize);
    wr::<usize>(tls_n.as_mut_ptr() as usize + 0x6300, holder_n.as_mut_ptr() as usize);
    wr::<usize>(tls_n.as_mut_ptr() as usize + 0x6318, lock_n.as_mut_ptr() as usize);

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0, rdx_val.as_ptr() as usize, 0x120, 0, arg5_val.as_ptr() as usize, 0x340);

    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0, rdx_val.as_ptr() as usize, 0x120, 0, arg5_val.as_ptr() as usize, 0x340);

    wr::<usize>(tls_slot, s_tls0);

    crate::diff_eq!(cmd_o[1..], cmd_n[1..], "d3d_142062080 cmd args");
    crate::diff_eq!(lock_o, lock_n, "d3d_142062080 lock state");
    Ok(1)
}

pub unsafe fn diff_d3d_1420626c0(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_dev_1f8_nested(_dev: usize, outer: usize, out_slot: usize) {
        let inner = rd::<usize>(outer + 0x10);
        let mut h0 = rd::<u64>(outer).wrapping_mul(31).wrapping_add(rd::<u64>(outer + 8));
        let mut h1 = rd::<u64>(out_slot + 0x10);
        for i in 0..5usize {
            h0 = h0
                .wrapping_mul(1099511628211)
                .wrapping_add(rd::<u64>(inner + i * 8));
            h1 = h1.wrapping_add(rd::<u64>(inner + i * 8).rotate_left((i * 7) as u32));
        }
        wr::<u64>(out_slot, h0);
        wr::<u64>(out_slot + 8, h1);
    }

    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 68];
    vtbl[0x1f8 / 8] = fake_dev_1f8_nested as *const () as usize;
    let dev_obj = [vtbl.as_ptr() as u64];

    let p10_target = [0x1100_0000u64];
    let p20_target = [0x2200_0000u64];
    let mut r8_buf = [0u8; 0x30];
    let r8_ptr = r8_buf.as_mut_ptr() as usize;
    wr::<u8>(r8_ptr, 9);
    wr::<u32>(r8_ptr + 0x04, 0x77);
    wr::<u64>(r8_ptr + 0x08, 0x8888_9999);
    wr::<usize>(r8_ptr + 0x10, p10_target.as_ptr() as usize);
    wr::<u64>(r8_ptr + 0x18, 0x50);
    wr::<usize>(r8_ptr + 0x20, p20_target.as_ptr() as usize);
    wr::<u64>(r8_ptr + 0x28, 0x60);

    let g_dev = exe::ptr::<usize>(0x1_4397_fc90) as usize;
    let s_dev = rd::<usize>(g_dev);
    wr::<usize>(g_dev, dev_obj.as_ptr() as usize);

    let mut out_o = [0xbb_u8; 0x18];
    let mut out_n = [0xbb_u8; 0x18];
    let ro = orig_fn(0, out_o.as_mut_ptr() as usize, r8_ptr);
    let rn = new_fn(0, out_n.as_mut_ptr() as usize, r8_ptr);

    wr::<usize>(g_dev, s_dev);

    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "d3d_1420626c0 out");
    Ok(1)
}

pub unsafe fn diff_d3d_1420627a0(orig: usize, new: usize) -> Result<u32, String> {
    static mut SCRATCH_A: [u8; 0xc0] = [0; 0xc0];
    static mut SCRATCH_B: [u8; 0x80] = [0; 0x80];

    unsafe extern "win64" fn fake_alloc_a(slot: usize, count: usize) -> usize {
        let ptr = core::ptr::addr_of_mut!(SCRATCH_A) as *mut u8 as usize;
        core::ptr::write_bytes(ptr as *mut u8, 0, 0xc0);
        wr::<usize>(slot, if count == 0 { 0 } else { ptr });
        wr::<u32>(slot + 8, count as u32);
        wr::<u32>(slot + 12, count as u32);
        slot
    }
    unsafe extern "win64" fn fake_alloc_b(slot: usize, count: usize) -> usize {
        let ptr = core::ptr::addr_of_mut!(SCRATCH_B) as *mut u8 as usize;
        core::ptr::write_bytes(ptr as *mut u8, 0, 0x80);
        wr::<usize>(slot, if count == 0 { 0 } else { ptr });
        wr::<u32>(slot + 8, count as u32);
        wr::<u32>(slot + 12, count as u32);
        slot
    }
    unsafe extern "win64" fn fake_free_vec(slot: usize) {
        wr::<u64>(slot, 0);
        wr::<u64>(slot + 8, 0);
    }
    unsafe extern "win64" fn fake_dev_1f8_build(_dev: usize, req: usize, out_slot: usize) {
        let count = rd::<u32>(req + 8) as usize;
        let entries = rd::<usize>(req + 0x10);
        let mut h = rd::<u64>(req).wrapping_mul(31).wrapping_add(rd::<u64>(req + 8));
        for i in 0..count {
            let e = entries + i * 0x38;
            h = h
                .wrapping_mul(1099511628211)
                .wrapping_add(rd::<u64>(e))
                .wrapping_mul(1099511628211)
                .wrapping_add(rd::<u64>(e + 0x18));
        }
        wr::<u64>(out_slot, h);
        wr::<u64>(out_slot + 8, (count as u64) ^ 0x1234_5678);
        wr::<u64>(out_slot + 16, if entries == 0 { 0 } else { 1 });
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_b3a0, fake_alloc_a as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_b4d0, fake_alloc_b as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_59a0, fake_free_vec as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize, u8, usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 68];
    vtbl[0x1f8 / 8] = fake_dev_1f8_build as *const () as usize;
    let dev_obj = [vtbl.as_ptr() as u64];

    let g_dev = exe::ptr::<usize>(0x1_4397_fc90) as usize;
    let s_dev = rd::<usize>(g_dev);
    wr::<usize>(g_dev, dev_obj.as_ptr() as usize);

    let mut items = vec![0u8; 0x88 * 2];
    for i in 0..2usize {
        let base = items.as_mut_ptr() as usize + i * 0x88;
        wr::<u32>(base, 1); // kind = 1 (branch 0x1411db018)
        wr::<u32>(base + 0x04, (i as u32) + 10);
        wr::<u32>(base + 0x18, (i as u32) * 16 + 8);
        wr::<u32>(base + 0x1c, (i as u32) + 3);
    }

    let mut ran = 0u32;
    for count in [0u64, 2u64] {
        let r9_struct = [count, items.as_ptr() as u64];
        let mut out_o = [0u64; 3];
        let mut out_n = [0u64; 3];
        let ro = orig_fn(0, out_o.as_mut_ptr() as usize, 5, r9_struct.as_ptr() as usize);
        let rn = new_fn(0, out_n.as_mut_ptr() as usize, 5, r9_struct.as_ptr() as usize);
        crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret orig");
        crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret new");
        crate::diff_eq!(out_o, out_n, "d3d_1420627a0 out");
        ran += 1;
    }

    wr::<usize>(g_dev, s_dev);
    Ok(ran)
}

pub unsafe fn diff_d3d_142063650(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_142056c10(this: usize) {
        wr::<u64>(this, 0xaaaa_bbbb_cccc_dddd);
    }
    unsafe extern "win64" fn fake_1411db770(_rcx: usize, _rdx: usize, idx_ptr: usize, pair_ptr: usize) -> u64 {
        let idx = rd::<u32>(idx_ptr) as u64;
        let p0 = rd::<u64>(pair_ptr);
        let p1 = rd::<u64>(pair_ptr + 8);
        p0.wrapping_mul(31).wrapping_add(p1).wrapping_add(idx * 0x100)
    }

    let _m1 = crate::difftest::Mock::new(0x1_4205_6c10, fake_142056c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_b770, fake_1411db770 as *const ())?;

    type F = unsafe extern "win64" fn(usize) -> usize;
    let orig_fn: F = core::mem::transmute(orig);
    let new_fn: F = core::mem::transmute(new);

    let mut fake_g = [0u8; 0x50];
    wr::<u64>(fake_g.as_mut_ptr() as usize + 0x38, 0x1111_2222);
    wr::<u64>(fake_g.as_mut_ptr() as usize + 0x40, 0x3333_4444);

    let g_ptr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let s_g = rd::<usize>(g_ptr);
    wr::<usize>(g_ptr, fake_g.as_ptr() as usize);

    let mut obj_o = [0x5au8; 0xb8];
    let mut obj_n = [0x5au8; 0xb8];
    let ro = orig_fn(obj_o.as_mut_ptr() as usize);
    let rn = new_fn(obj_n.as_mut_ptr() as usize);

    wr::<usize>(g_ptr, s_g);

    crate::diff_eq!(ro, obj_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, obj_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(obj_o, obj_n, "d3d_142063650 constructed obj");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x14205e9c0, "d3d::d3d_14205e9c0_14205e9c0", crate::ns::rend::d3d_pipeline::d3d_14205e9c0 as *const (), crate::ns::rend::d3d_pipeline::diff_d3d_14205e9c0),
// Replacement::verified(0x142061ec0, "d3d::d3d_142061ec0_142061ec0", crate::ns::rend::d3d_pipeline::d3d_142061ec0 as *const (), crate::ns::rend::d3d_pipeline::diff_d3d_142061ec0),
// Replacement::verified(0x142061f60, "d3d::d3d_142061f60_142061f60", crate::ns::rend::d3d_pipeline::d3d_142061f60 as *const (), crate::ns::rend::d3d_pipeline::diff_d3d_142061f60),
// Replacement::verified(0x142062080, "d3d::d3d_142062080_142062080", crate::ns::rend::d3d_pipeline::d3d_142062080 as *const (), crate::ns::rend::d3d_pipeline::diff_d3d_142062080),
// Replacement::verified(0x1420626c0, "d3d::d3d_1420626c0_1420626c0", crate::ns::rend::d3d_pipeline::d3d_1420626c0 as *const (), crate::ns::rend::d3d_pipeline::diff_d3d_1420626c0),
// Replacement::verified(0x1420627a0, "d3d::d3d_1420627a0_1420627a0", crate::ns::rend::d3d_pipeline::d3d_1420627a0 as *const (), crate::ns::rend::d3d_pipeline::diff_d3d_1420627a0),
// Replacement::verified_throws(0x142063650, "d3d::d3d_142063650_142063650", crate::ns::rend::d3d_pipeline::d3d_142063650 as *const (), Throws::Discard, crate::ns::rend::d3d_pipeline::diff_d3d_142063650),
