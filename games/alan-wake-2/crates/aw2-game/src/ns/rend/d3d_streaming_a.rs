use crate::exe::{self, rd, wr};

/// Original: `0x142078b90` (193 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_streaming_buffer_ctx(rcx: usize, rdx: usize) {
    let rbx = rd::<u64>(rcx + 0xe0);
    exe::call_va!(0x1_4207_5970, (rcx + 0x390) -> ());
    let eax = rd::<u32>(rcx + 0x11c);
    let rdx_arg = rd::<u64>(rcx + 0x110);
    exe::call_va!(
        0x1_4207_5080,
        (rcx + 0x390, rdx_arg, 0usize, 0usize, 0u64, rbx, eax) -> ()
    );
    wr::<u64>(rcx + 0x3a0, rbx);
    wr::<u64>(rcx + 0x398, 0);
    let rax = rd::<u64>(rcx + 0xd0);
    wr::<u64>(rcx + 0x390, rax);
    if rd::<u8>(rdx + 0x100) == 0 || rd::<u8>(rdx + 0x101) == 0 {
        let count = ((rd::<u32>(rcx + 0x258) << 4) as u64) << 5;
        exe::call_va!(0x1_4209_d5c0, (rcx + 0x330, count, 0x29u8) -> ());
    }
}

/// Original: `0x142079bd0` (171 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_copy_descriptor_range_table(rcx: usize, rdx: usize) -> u64 {
    let rsi = rd::<u64>(rcx);
    let mut out0: u64 = 0;
    let mut out1: usize = 0;
    let mut out2: u64 = 0;
    exe::call_va!(
        0x1_4209_3d00,
        (
            rcx,
            rsi,
            (&mut out0) as *mut u64 as usize,
            (&mut out2) as *mut u64 as usize,
            (&mut out1) as *mut usize as usize
        ) -> ()
    );
    let step = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let first_val = rd::<u64>(out1);
    wr::<u64>(rdx, first_val);
    let mut dst_handle = out2;
    if rsi != 0 {
        let arr = rd::<usize>(rcx + 0x8);
        let mut i: u64 = 0;
        while i < rsi {
            let src_handle = rd::<u64>(arr + (i as usize) * 8);
            if src_handle != 0 {
                let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
                let vt = rd::<usize>(dev);
                let fn_ptr = rd::<usize>(vt + 0xc0);
                exe::call_va!(fn_ptr, (dev, 1u32, dst_handle, src_handle, 0u32) -> ());
            }
            dst_handle = dst_handle.wrapping_add(step);
            i += 1;
        }
    }
    out0
}

/// Original: `0x142079c90` (70 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_check_format_support_bit6(ecx: u32) -> u8 {
    #[repr(C)]
    struct FormatSupportData {
        format: u32,
        support1: u32,
        support2: u32,
    }
    let mut data = FormatSupportData {
        format: ecx,
        support1: 0,
        support2: 0,
    };
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let vt = rd::<usize>(dev);
    let fn_ptr = rd::<usize>(vt + 0x68);
    let hr = exe::call_va!(
        fn_ptr,
        (dev, 3u32, (&mut data) as *mut FormatSupportData as usize, 0xcu32) -> i32
    );
    if hr < 0 {
        0
    } else {
        ((data.support2 >> 6) & 1) as u8
    }
}

/// Original: `0x142079ce0` (139 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_create_heap_wrapper(
    rcx: usize,
    rdx: usize,
    r8: u64,
) -> usize {
    #[repr(C)]
    struct HeapDesc {
        size_in_bytes: u64,
        props: [u8; 16],
        tail: [u8; 32],
    }
    let mut desc = HeapDesc {
        size_in_bytes: r8,
        props: rd::<[u8; 16]>(rdx),
        tail: [0u8; 32],
    };
    wr::<u32>(rcx, 0);
    wr::<u64>(rcx + 0x8, 0);
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let vt = rd::<usize>(dev);
    let fn_ptr = rd::<usize>(vt + 0x58);
    let guid = exe::ptr::<u8>(0x1_433d_cb10) as usize;
    let hr = exe::call_va!(
        fn_ptr,
        (dev, (&mut desc) as *mut HeapDesc as usize, guid, rcx + 0x8, 1u32) -> i32
    );
    let hr_u = hr as u32;
    if hr_u == 0x8007_000e || hr_u.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (hr_u, hr_u) -> ());
    }
    rcx
}

/// Original: `0x142079ec0` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_init_heap_props_default(
    rcx: usize,
    _rdx: usize,
    _r8: usize,
    r9d: u32,
) -> usize {
    wr::<u32>(rcx, 1);
    wr::<u32>(rcx + 0x18, 0);
    wr::<u64>(rcx + 0x8, 1);
    wr::<u32>(rcx + 0x10, r9d);
    rcx
}

pub unsafe fn diff_d3d_init_heap_props_default(orig: usize, new: usize) -> Result<u32, String> {
    let mut b_o = [0xAAu8; 0x20];
    let mut b_n = [0xAAu8; 0x20];
    let r_o = exe::call_va!(orig, (b_o.as_mut_ptr() as usize, 0usize, 0usize, 0x1234_5678u32) -> usize);
    let r_n = exe::call_va!(new, (b_n.as_mut_ptr() as usize, 0usize, 0usize, 0x1234_5678u32) -> usize);
    crate::diff_eq!(r_o, b_o.as_ptr() as usize, "ret orig");
    crate::diff_eq!(r_n, b_n.as_ptr() as usize, "ret new");
    crate::diff_eq!(b_o, b_n, "heap props buf");
    Ok(1)
}

/// Original: `0x142079f80` (213 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_copy_construct_subresource_list(
    rcx: usize,
    rdx: usize,
) -> usize {
    wr::<u64>(rcx, rd::<u64>(rdx));
    wr::<u32>(rcx + 0x8, rd::<u32>(rdx + 0x8));
    wr::<u64>(rcx + 0x10, 0);
    wr::<u64>(rcx + 0x18, 0);
    let count = rd::<u32>(rdx + 0x18) as usize;
    let byte_len = count.wrapping_mul(0x1c);
    let mut src = rd::<usize>(rdx + 0x10);
    let end = src.wrapping_add(byte_len);
    let r14 = ((byte_len as isize) / 0x1c) as isize;
    let mut dst = 0usize;
    if r14 > 0 {
        let alloc = exe::call_va!(0x1_40a6_e7a0, (byte_len, r14 as usize) -> usize);
        wr::<usize>(rcx + 0x10, alloc);
        wr::<u32>(rcx + 0x18, 0);
        wr::<u32>(rcx + 0x1c, r14 as u32);
        dst = alloc;
    }
    while src != end {
        wr::<[u8; 16]>(dst, rd::<[u8; 16]>(src));
        wr::<u64>(dst + 0x10, rd::<u64>(src + 0x10));
        wr::<u32>(dst + 0x18, rd::<u32>(src + 0x18));
        dst = dst.wrapping_add(0x1c);
        src = src.wrapping_add(0x1c);
    }
    wr::<u32>(rcx + 0x18, r14 as u32);
    wr::<u64>(rcx + 0x20, rd::<u64>(rdx + 0x20));
    wr::<u32>(rcx + 0x28, rd::<u32>(rdx + 0x28));
    rcx
}

/// Original: `0x14207a060` (213 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clone_subresource_list_into(
    rcx: usize,
    rdx: usize,
) -> usize {
    wr::<u64>(rdx, rd::<u64>(rcx));
    wr::<u32>(rdx + 0x8, rd::<u32>(rcx + 0x8));
    wr::<u64>(rdx + 0x10, 0);
    wr::<u64>(rdx + 0x18, 0);
    let count = rd::<u32>(rcx + 0x18) as usize;
    let byte_len = count.wrapping_mul(0x1c);
    let mut src = rd::<usize>(rcx + 0x10);
    let end = src.wrapping_add(byte_len);
    let r14 = ((byte_len as isize) / 0x1c) as isize;
    let mut dst = 0usize;
    if r14 > 0 {
        let alloc = exe::call_va!(0x1_40a6_e7a0, (byte_len, r14 as usize) -> usize);
        wr::<usize>(rdx + 0x10, alloc);
        wr::<u32>(rdx + 0x18, 0);
        wr::<u32>(rdx + 0x1c, r14 as u32);
        dst = alloc;
    }
    while src != end {
        wr::<[u8; 16]>(dst, rd::<[u8; 16]>(src));
        wr::<u64>(dst + 0x10, rd::<u64>(src + 0x10));
        wr::<u32>(dst + 0x18, rd::<u32>(src + 0x18));
        dst = dst.wrapping_add(0x1c);
        src = src.wrapping_add(0x1c);
    }
    wr::<u32>(rdx + 0x18, r14 as u32);
    wr::<u64>(rdx + 0x20, rd::<u64>(rcx + 0x20));
    wr::<u32>(rdx + 0x28, rd::<u32>(rcx + 0x28));
    rdx
}

/// Original: `0x14207a180` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_should_use_streaming_mode_a(rcx: usize) -> u8 {
    if rd::<u8>(rcx + 0x66) != 0 {
        return 0;
    }
    let eax = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize);
    if eax != 0 {
        if eax == 2 {
            0
        } else {
            1
        }
    } else if rd::<u8>(rcx + 0x67) != 0 {
        1
    } else {
        0
    }
}

pub unsafe fn diff_d3d_should_use_streaming_mode_a(orig: usize, new: usize) -> Result<u32, String> {
    let mut ctx = [0u8; 0x70];
    for (b66, b67) in [(0u8, 0u8), (0, 1), (1, 0), (1, 1)] {
        ctx[0x66] = b66;
        ctx[0x67] = b67;
        let r_o = exe::call_va!(orig, (ctx.as_ptr() as usize,) -> u8);
        let r_n = exe::call_va!(new, (ctx.as_ptr() as usize,) -> u8);
        crate::diff_eq!(r_o, r_n, "mode_a");
    }
    Ok(4)
}

/// Original: `0x14207ab70` (149 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_sync_streaming_cvars(rcx: usize) {
    let mut unk_ptr: usize = 0;
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    exe::call_va!(0x1_4208_d1f0, (mgr, (&mut unk_ptr) as *mut usize as usize, 1u8) -> ());
    if unk_ptr != 0 {
        let vt = rd::<usize>(unk_ptr);
        let rel = rd::<usize>(vt + 0x10);
        exe::call_va!(rel, (unk_ptr,) -> ());
    }
    let g1_addr = exe::ptr::<u32>(0x1_43a2_b11c) as usize;
    let g2_addr = exe::ptr::<u32>(0x1_43a2_b08c) as usize;
    let g3_addr = exe::ptr::<f32>(0x1_4385_49ac) as usize;
    let old_g1 = rd::<u32>(g1_addr);
    let old_g2 = rd::<u32>(g2_addr);
    let old_g3 = rd::<f32>(g3_addr);
    let new_g1 = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3738) as usize);
    wr::<u32>(g1_addr, new_g1);
    let new_g2 = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3710) as usize);
    wr::<u32>(g2_addr, new_g2);
    let new_g3 = rd::<f32>(exe::ptr::<f32>(0x1_4385_7fe8) as usize);
    wr::<f32>(g3_addr, new_g3);
    if old_g1 != new_g1 || old_g2 != new_g2 || old_g3 != new_g3 {
        exe::call_va!(0x1_4207_a1e0, (rcx, 0usize, 1u8) -> ());
    }
}

/// Original: `0x14207ac10` (223 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_destroy_swapchain_context(rcx: usize) {
    let inner = rd::<usize>(rcx);
    let sc = rd::<usize>(inner + 0x8);
    let mut fullscreen: u32 = 0;
    let vt = rd::<usize>(sc);
    let hr = exe::call_va!(
        rd::<usize>(vt + 0x58),
        (sc, (&mut fullscreen) as *mut u32 as usize, 0usize) -> i32
    );
    let hr_u = hr as u32;
    if hr_u == 0x8007_000e || hr_u.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (hr_u, hr_u) -> ());
    }
    if fullscreen == 1 {
        let inner2 = rd::<usize>(rcx);
        let sc2 = rd::<usize>(inner2 + 0x8);
        let vt2 = rd::<usize>(sc2);
        exe::call_va!(rd::<usize>(vt2 + 0x50), (sc2, 0u32, 0usize) -> i32);
    }
    exe::call_va!(
        0x1_4183_4ee8,
        (
            rcx + 0x20,
            8usize,
            3usize,
            exe::ptr::<u8>(0x1_411d_f630) as usize
        ) -> ()
    );
    exe::call_va!(
        0x1_4183_4ee8,
        (
            rcx + 0x8,
            8usize,
            3usize,
            exe::ptr::<u8>(0x1_411d_2db0) as usize
        ) -> ()
    );
    let rbx = rd::<usize>(rcx);
    if rbx != 0 {
        exe::call_va!(0x1_401d_e830, (rbx + 0x30,) -> ());
        let p20 = rd::<usize>(rbx + 0x20);
        if p20 != 0 {
            let v18 = rd::<u32>(rbx + 0x18);
            exe::call_va!(0x1_4209_a1e0, (p20, v18) -> ());
        }
        let p08 = rd::<usize>(rbx + 0x8);
        if p08 != 0 {
            let vt08 = rd::<usize>(p08);
            exe::call_va!(rd::<usize>(vt08 + 0x10), (p08,) -> ());
        }
        wr::<usize>(rbx + 0x8, 0);
        exe::call_va!(0x1_4244_2de0, (rbx,) -> ());
    }
}

/// Original: `0x14207ad20` (144 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_recreate_swapchain_target(rcx: usize, rdx: usize) {
    let p0 = rd::<usize>(rcx);
    let rsi = rd::<usize>(p0);
    let vt_rsi = rd::<usize>(rsi);
    let fn_create = rd::<usize>(vt_rsi + 0x50);
    let p8 = rd::<usize>(rcx + 0x8);
    let rbx = rd::<usize>(p8);
    let old = rd::<usize>(rbx + 0x8);
    if old != 0 {
        let vt_old = rd::<usize>(old);
        exe::call_va!(rd::<usize>(vt_old + 0x10), (old,) -> ());
    }
    wr::<usize>(rbx + 0x8, 0);
    let r8 = rd::<usize>(rcx + 0x10);
    let hr = exe::call_va!(fn_create, (rsi, rdx, r8, rbx + 0x8) -> i32);
    let hr_u = hr as u32;
    if hr_u == 0x8007_000e || hr_u.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (hr_u, hr_u) -> ());
    }
}

/// Original: `0x14207ade0` (204 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_nvapi_init_swapchain_hdr(rcx: usize, rdx: u64) {
    #[repr(C)]
    struct SlCreatePacket {
        header: u64,
        zero: u64,
        out_ptr: usize,
        f18: u64,
        f20: usize,
        f28: u64,
    }
    #[repr(C)]
    struct SlQueryPacket {
        header: u64,
        zero: u64,
        out_ptr: usize,
    }
    let mut out_sc: usize = 0;
    let mut tmp20 = [0u8; 16];
    let p8 = rd::<usize>(rcx + 0x8);
    let mut pkt1 = SlCreatePacket {
        header: 0x30005,
        zero: 0,
        out_ptr: (&mut out_sc) as *mut usize as usize,
        f18: rd::<u64>(rcx + 0x18),
        f20: rd::<usize>(p8),
        f28: rdx,
    };
    exe::call_iat!(
        0x1_4264_6498,
        (rd::<usize>(rcx), (&mut pkt1) as *mut SlCreatePacket as usize, 0usize) -> i32
    );
    let vt20 = rd::<usize>(pkt1.f20);
    exe::call_va!(rd::<usize>(vt20 + 0x10), (pkt1.f20,) -> ());
    let mut pkt2 = SlQueryPacket {
        header: 0x30009,
        zero: 0,
        out_ptr: tmp20.as_mut_ptr() as usize,
    };
    exe::call_iat!(
        0x1_4264_6488,
        (rd::<usize>(rcx), (&mut pkt2) as *mut SlQueryPacket as usize) -> i32
    );
    let val = rd::<u64>(rd::<usize>(rcx));
    let dst_holder = rd::<usize>(rd::<usize>(rcx + 0x10));
    wr::<u64>(dst_holder + 0x10, val);
    let dst_holder2 = rd::<usize>(rd::<usize>(rcx + 0x10));
    let old = rd::<usize>(dst_holder2 + 0x8);
    if old != 0 {
        let vt_old = rd::<usize>(old);
        exe::call_va!(rd::<usize>(vt_old + 0x10), (old,) -> ());
    }
    wr::<usize>(dst_holder2 + 0x8, out_sc);
}

/// Original: `0x14207b3d0` (32 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_should_use_streaming_mode_b(rcx: usize) -> u8 {
    if rd::<u8>(rcx + 0x66) != 0 {
        return 1;
    }
    let eax = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize);
    let eq = if eax != 0 {
        eax == 2
    } else {
        rd::<u8>(rcx + 0x67) == 0
    };
    if eq { 1 } else { 0 }
}

pub unsafe fn diff_d3d_should_use_streaming_mode_b(orig: usize, new: usize) -> Result<u32, String> {
    let mut ctx = [0u8; 0x70];
    for (b66, b67) in [(0u8, 0u8), (0, 1), (1, 0), (1, 1)] {
        ctx[0x66] = b66;
        ctx[0x67] = b67;
        let r_o = exe::call_va!(orig, (ctx.as_ptr() as usize,) -> u8);
        let r_n = exe::call_va!(new, (ctx.as_ptr() as usize,) -> u8);
        crate::diff_eq!(r_o, r_n, "mode_b");
    }
    Ok(4)
}

/// Original: `0x14207b400` (29 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_get_streaming_mode_code(rcx: usize) -> u32 {
    if rd::<u8>(rcx + 0x66) != 0 {
        return 2;
    }
    let eax = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b11c) as usize);
    if eax != 0 {
        eax
    } else if rd::<u8>(rcx + 0x67) != 0 {
        1
    } else {
        0
    }
}

pub unsafe fn diff_d3d_get_streaming_mode_code(orig: usize, new: usize) -> Result<u32, String> {
    let mut ctx = [0u8; 0x70];
    for (b66, b67) in [(0u8, 0u8), (0, 1), (1, 0), (1, 1)] {
        ctx[0x66] = b66;
        ctx[0x67] = b67;
        let r_o = exe::call_va!(orig, (ctx.as_ptr() as usize,) -> u32);
        let r_n = exe::call_va!(new, (ctx.as_ptr() as usize,) -> u32);
        crate::diff_eq!(r_o, r_n, "mode_code");
    }
    Ok(4)
}

/// Original: `0x14207b620` (232 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_enable_os_hdr_display(rcx: usize) -> u8 {
    let edx = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b08c) as usize);
    if edx.wrapping_sub(1) <= 1 {
        return 0;
    }
    if edx == 3 {
        return 1;
    }
    let mut info_buf = [0u8; 0x50];
    if exe::call_va!(0x1_4207_b830, (rcx, info_buf.as_mut_ptr() as usize) -> u8) == 0 {
        return 0;
    }
    #[repr(C)]
    struct DisplayConfigGetColor {
        kind: u32,
        size: u32,
        luid: u64,
        id: u32,
        flags: u32,
        pad0: u32,
        pad1: u32,
    }
    #[repr(C)]
    struct DisplayConfigSetColor {
        kind: u32,
        size: u32,
        luid: u64,
        id: u32,
        enable: u32,
        pad0: u32,
    }
    let mut get_req = DisplayConfigGetColor {
        kind: 0x9,
        size: 0x20,
        luid: rd::<u64>(info_buf.as_ptr() as usize + 0x14),
        id: rd::<u32>(info_buf.as_ptr() as usize + 0x1c),
        flags: 0,
        pad0: 0,
        pad1: 0,
    };
    if exe::call_iat!(0x1_4264_6258, ((&mut get_req) as *mut DisplayConfigGetColor as usize,) -> i32) != 0 {
        return 0;
    }
    let ecx = get_req.flags;
    if (ecx & 1) != 0 && ((ecx >> 1) & 1) == 0 {
        let mut set_req = DisplayConfigSetColor {
            kind: 0xa,
            size: 0x18,
            luid: get_req.luid,
            id: get_req.id,
            enable: 1,
            pad0: 0,
        };
        let status = exe::call_iat!(
            0x1_4264_6260,
            ((&mut set_req) as *mut DisplayConfigSetColor as usize,) -> i32
        );
        return if status == 0 { 1 } else { 0 };
    }
    ((ecx >> 1) & 1) as u8
}

/// Original: `0x14207b710` (135 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_os_hdr_enabled(rcx: usize) -> u8 {
    let eax = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b08c) as usize);
    if eax == 3 {
        return 1;
    }
    if eax.wrapping_sub(1) <= 1 {
        return 0;
    }
    let mut info_buf = [0u8; 0x50];
    if exe::call_va!(0x1_4207_b830, (rcx, info_buf.as_mut_ptr() as usize) -> u8) == 0 {
        return 0;
    }
    #[repr(C)]
    struct DisplayConfigGetColor {
        kind: u32,
        size: u32,
        luid: u64,
        id: u32,
        flags: u32,
        pad0: u32,
        pad1: u32,
    }
    let mut get_req = DisplayConfigGetColor {
        kind: 0x9,
        size: 0x20,
        luid: rd::<u64>(info_buf.as_ptr() as usize + 0x14),
        id: rd::<u32>(info_buf.as_ptr() as usize + 0x1c),
        flags: 0,
        pad0: 0,
        pad1: 0,
    };
    if exe::call_iat!(0x1_4264_6258, ((&mut get_req) as *mut DisplayConfigGetColor as usize,) -> i32) != 0 {
        return 0;
    }
    ((get_req.flags >> 1) & 1) as u8
}

/// Original: `0x14207b7a0` (135 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_os_hdr_supported(rcx: usize) -> u8 {
    let edx = rd::<u32>(exe::ptr::<u32>(0x1_43a2_b08c) as usize);
    if edx.wrapping_sub(2) <= 1 {
        return 1;
    }
    if edx == 1 {
        return 0;
    }
    let mut info_buf = [0u8; 0x50];
    if exe::call_va!(0x1_4207_b830, (rcx, info_buf.as_mut_ptr() as usize) -> u8) == 0 {
        return 0;
    }
    #[repr(C)]
    struct DisplayConfigGetColor {
        kind: u32,
        size: u32,
        luid: u64,
        id: u32,
        flags: u32,
        pad0: u32,
        pad1: u32,
    }
    let mut get_req = DisplayConfigGetColor {
        kind: 0x9,
        size: 0x20,
        luid: rd::<u64>(info_buf.as_ptr() as usize + 0x14),
        id: rd::<u32>(info_buf.as_ptr() as usize + 0x1c),
        flags: 0,
        pad0: 0,
        pad1: 0,
    };
    if exe::call_iat!(0x1_4264_6258, ((&mut get_req) as *mut DisplayConfigGetColor as usize,) -> i32) != 0 {
        return 0;
    }
    (get_req.flags & 1) as u8
}

/// Original: `0x14207bd60` (64 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_is_swapchain_fullscreen(rcx: usize) -> u8 {
    let vt = rd::<usize>(rcx);
    let mut state: u32 = 0;
    let hr = exe::call_va!(
        rd::<usize>(vt + 0x58),
        (rcx, (&mut state) as *mut u32 as usize, 0usize) -> i32
    );
    let hr_u = hr as u32;
    if hr_u == 0x8007_000e || hr_u.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (hr_u, hr_u) -> ());
    }
    if state == 1 { 1 } else { 0 }
}

/// Original: `0x14207bdb0` (107 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_resize_swapchain_and_buffers(
    rcx: usize,
    rdx: u8,
    r8: usize,
    r9d: u32,
    arg5: u32,
    arg6: u32,
) -> u8 {
    exe::call_va!(0x1_4207_c130, (r8,) -> ());
    exe::call_va!(0x1_4207_bf60, () -> ());
    let ok = exe::call_va!(0x1_4207_c410, (rcx, r9d, arg5, arg6) -> u8);
    exe::call_va!(0x1_4207_c040, (rcx, rdx, r8) -> ());
    ok
}

/// Original: `0x14207bf60` (206 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_flush_gpu_and_wait_idle() {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    exe::call_va!(0x1_4207_61a0, (tls0 + 0x467e0,) -> ());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    exe::call_va!(0x1_4206_df80, (rd::<usize>(tls0 + 0x6328),) -> ());
    exe::call_va!(0x1_4209_95c0, (0u32,) -> ());
    exe::call_va!(0x1_4209_95c0, (1u32,) -> ());
    exe::call_va!(0x1_4209_9c20, (1u8, 1u32) -> ());
    exe::call_va!(0x1_4209_9c20, (1u8, 0u32) -> ());
    let mut fence_pair = [0usize; 2];
    exe::call_va!(0x1_4207_34e0, (fence_pair.as_mut_ptr() as usize, 0u32) -> ());
    let fence = fence_pair[1];
    let target = fence_pair[0] as u64;
    while fence != 0 {
        let vt = rd::<usize>(fence);
        let completed = exe::call_va!(rd::<usize>(vt + 0x40), (fence,) -> u64);
        if completed == u64::MAX {
            exe::call_va!(0x1_4205_d410, (0u32, 0x887a_0005u32) -> ());
        }
        if completed >= target {
            break;
        }
        exe::call_iat!(0x1_4264_5868, () -> ());
    }
}

/// Original: `0x14207c040` (231 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_rebuild_backbuffer_views(rcx: usize, rdx: u8, r8: usize) {
    let mut slot = r8;
    for edi in 0..2u32 {
        let mut tmp: usize = 0;
        let rax = exe::call_va!(
            0x1_4207_c1c0,
            ((&mut tmp) as *mut usize as usize, rcx, rdx, edi) -> usize
        );
        let new_tex = rd::<usize>(rax);
        wr::<usize>(rax, 0);
        let old_tex = rd::<usize>(slot);
        wr::<usize>(slot, new_tex);
        if old_tex != 0 {
            exe::call_va!(0x1_4206_b020, (old_tex,) -> ());
            exe::call_va!(0x1_4244_2de0, (old_tex,) -> ());
        }
        if tmp != 0 {
            exe::call_va!(0x1_4206_b020, (tmp,) -> ());
            exe::call_va!(0x1_4244_2de0, (tmp,) -> ());
        }
        let mut cur_tex = rd::<usize>(slot);
        let new_view = exe::call_va!(
            0x1_411d_f430,
            (0usize, 0usize, (&mut cur_tex) as *mut usize as usize) -> usize
        );
        let old_view = rd::<usize>(slot + 0x18);
        wr::<usize>(slot + 0x18, new_view);
        if old_view != 0 {
            exe::call_va!(0x1_4204_f860, (old_view,) -> ());
            exe::call_va!(0x1_4244_2de0, (old_view,) -> ());
        }
        slot += 8;
    }
    let vt = rd::<usize>(rcx);
    let cur_idx = exe::call_va!(rd::<usize>(vt + 0x120), (rcx,) -> u32);
    wr::<u32>(r8 + 0x30, cur_idx);
}

/// Original: `0x14207c130` (124 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_release_backbuffer_views(rcx: usize) {
    for i in 0..2usize {
        let tex = rd::<usize>(rcx + i * 8);
        wr::<usize>(rcx + i * 8, 0);
        if tex != 0 {
            exe::call_va!(0x1_4206_b020, (tex,) -> ());
            exe::call_va!(0x1_4244_2de0, (tex,) -> ());
        }
        let view = rd::<usize>(rcx + i * 8 + 0x18);
        wr::<usize>(rcx + i * 8 + 0x18, 0);
        if view != 0 {
            exe::call_va!(0x1_4204_f860, (view,) -> ());
            exe::call_va!(0x1_4244_2de0, (view,) -> ());
        }
    }
}

/// Original: `0x14207c410` (149 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_resize_swapchain_buffers(
    rcx: usize,
    edx: u32,
    r8d: u32,
    r9d: u32,
) -> u8 {
    let mut desc = [0u8; 0x50];
    let vt = rd::<usize>(rcx);
    exe::call_va!(rd::<usize>(vt + 0x60), (rcx, desc.as_mut_ptr() as usize) -> i32);
    let flags = rd::<u32>(desc.as_ptr() as usize + 0x40);
    let vt2 = rd::<usize>(rcx);
    let hr = exe::call_va!(
        rd::<usize>(vt2 + 0x68),
        (rcx, 0u32, edx, r8d, r9d, flags) -> i32
    );
    let hr_u = hr as u32;
    if hr_u == 0x8007_0005 {
        return 0;
    }
    if hr_u == 0x8007_000e || hr_u.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (hr_u, hr_u) -> ());
    }
    ((hr_u >> 31) ^ 1) as u8
}

/// Original: `0x14207c6b0` (24 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_clamp_texture_dimension(ecx: i32) -> i32 {
    if ecx < 1 {
        1
    } else if ecx > 0x4000 {
        0x4000
    } else {
        ecx
    }
}

pub unsafe fn diff_d3d_clamp_texture_dimension(orig: usize, new: usize) -> Result<u32, String> {
    for v in [-100i32, 0, 1, 256, 0x4000, 0x4001, i32::MAX] {
        let r_o = exe::call_va!(orig, (v,) -> i32);
        let r_n = exe::call_va!(new, (v,) -> i32);
        crate::diff_eq!(r_o, r_n, "clamp_dim");
    }
    Ok(7)
}

/// Original: `0x14207c710` (138 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_create_subresource_iter(
    rcx: usize,
    rdx: u64,
    r8d: u32,
) -> usize {
    #[repr(C)]
    struct SubresIter {
        f00: u64,
        f08: u32,
        pad0c: u32,
        f10: u64,
        f18: u32,
        f1c: u32,
        f20: u64,
        f28: u32,
        pad2c: u32,
    }
    let mut tmp = SubresIter {
        f00: rdx,
        f08: 0,
        pad0c: 0,
        f10: 0,
        f18: 0,
        f1c: 0,
        f20: 0,
        f28: r8d,
        pad2c: 0,
    };
    exe::call_va!(0x1_4207_c7a0, ((&mut tmp) as *mut SubresIter as usize,) -> ());
    wr::<u64>(rcx, tmp.f00);
    wr::<u32>(rcx + 0x8, tmp.f08);
    wr::<[u8; 16]>(rcx + 0x10, [0u8; 16]);
    wr::<u64>(rcx + 0x10, tmp.f10);
    wr::<u32>(rcx + 0x18, tmp.f18);
    wr::<u32>(rcx + 0x1c, tmp.f1c);
    wr::<u64>(rcx + 0x20, tmp.f20);
    wr::<u32>(rcx + 0x28, tmp.f28);
    rcx
}

/// Original: `0x14207cad0` (11 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_subresource_iter_has_next(rcx: usize) -> u8 {
    let count = rd::<u32>(rcx + 0x18) as u64;
    if rd::<u64>(rcx + 0x20) < count { 1 } else { 0 }
}

pub unsafe fn diff_d3d_subresource_iter_has_next(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf = [0u8; 0x30];
    for (cnt, idx) in [(4u32, 0u64), (4, 3), (4, 4), (0, 0)] {
        wr::<u32>(buf.as_mut_ptr() as usize + 0x18, cnt);
        wr::<u64>(buf.as_mut_ptr() as usize + 0x20, idx);
        let r_o = exe::call_va!(orig, (buf.as_ptr() as usize,) -> u8);
        let r_n = exe::call_va!(new, (buf.as_ptr() as usize,) -> u8);
        crate::diff_eq!(r_o, r_n, "has_next");
    }
    Ok(4)
}

/// Original: `0x14207caf0` (43 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_subresource_iter_current(rcx: usize, rdx: usize) -> usize {
    let base = rd::<usize>(rcx + 0x10);
    let idx = rd::<usize>(rcx + 0x20);
    let src = base.wrapping_add(idx.wrapping_mul(0x1c));
    let v0 = rd::<[u8; 16]>(src);
    let v1 = rd::<u64>(src + 0x10);
    let v2 = rd::<u32>(src + 0x18);
    wr::<[u8; 16]>(rdx, v0);
    wr::<u64>(rdx + 0x10, v1);
    wr::<u32>(rdx + 0x18, v2);
    rdx
}

pub unsafe fn diff_d3d_subresource_iter_current(orig: usize, new: usize) -> Result<u32, String> {
    let mut items = [0u8; 0x1c * 3];
    for (i, b) in items.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7).wrapping_add(3);
    }
    let mut iter_s = [0u8; 0x30];
    wr::<usize>(iter_s.as_mut_ptr() as usize + 0x10, items.as_ptr() as usize);
    wr::<u64>(iter_s.as_mut_ptr() as usize + 0x20, 1);
    let mut out_o = [0u8; 0x1c];
    let mut out_n = [0u8; 0x1c];
    let r_o = exe::call_va!(orig, (iter_s.as_ptr() as usize, out_o.as_mut_ptr() as usize) -> usize);
    let r_n = exe::call_va!(new, (iter_s.as_ptr() as usize, out_n.as_mut_ptr() as usize) -> usize);
    crate::diff_eq!(r_o, out_o.as_ptr() as usize, "ret orig");
    crate::diff_eq!(r_n, out_n.as_ptr() as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "current item");
    Ok(1)
}

/// Original: `0x14207cb30` (36 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_subresource_iter_advance(rcx: usize) -> usize {
    let next = rd::<u64>(rcx + 0x20).wrapping_add(1);
    wr::<u64>(rcx + 0x20, next);
    let count = rd::<u32>(rcx + 0x18) as u64;
    if next >= count {
        exe::call_va!(0x1_4207_c7a0, (rcx,) -> ());
    }
    rcx
}

/// Original: `0x14207cb60` (61 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_subresource_iter_init_in_place(
    rcx: usize,
    rdx: u64,
    r8d: u32,
) -> usize {
    wr::<u64>(rcx, rdx);
    wr::<u32>(rcx + 0x8, 0);
    wr::<[u8; 16]>(rcx + 0x10, [0u8; 16]);
    wr::<u64>(rcx + 0x10, 0);
    wr::<u64>(rcx + 0x18, 0);
    wr::<u64>(rcx + 0x20, 0);
    wr::<u32>(rcx + 0x28, r8d);
    exe::call_va!(0x1_4207_c7a0, (rcx,) -> ());
    rcx
}

/// Original: `0x14207cbb0` (101 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_check_dxgi_factory_tearing_support(rcx: usize) -> u8 {
    let mut factory5: usize = 0;
    let vt = rd::<usize>(rcx);
    let guid = exe::ptr::<u8>(0x1_433d_cd80) as usize;
    let hr = exe::call_va!(
        rd::<usize>(vt),
        (rcx, guid, (&mut factory5) as *mut usize as usize) -> i32
    );
    let mut allow_tearing: u32 = 0;
    let mut supported: u8 = 0;
    if hr >= 0 {
        let vt5 = rd::<usize>(factory5);
        let hr2 = exe::call_va!(
            rd::<usize>(vt5 + 0xe0),
            (factory5, 0u32, (&mut allow_tearing) as *mut u32 as usize, 4u32) -> i32
        );
        if hr2 >= 0 && allow_tearing != 0 {
            supported = 1;
        }
    }
    if factory5 != 0 {
        let vt5 = rd::<usize>(factory5);
        exe::call_va!(rd::<usize>(vt5 + 0x10), (factory5,) -> ());
    }
    supported
}

/// Original: `0x14207cc40` (49 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_extract_swapchain_pair(rcx: usize, rdx: usize) -> usize {
    let rax = rd::<usize>(rcx);
    if rax != 0 {
        let p8 = rd::<u64>(rax + 0x8);
        if p8 != 0 {
            let p10 = rd::<u64>(rax + 0x10);
            if p10 != 0 {
                wr::<u64>(rdx, p8);
                wr::<u64>(rdx + 0x8, p10);
                wr::<u8>(rdx + 0x10, 1);
                return rdx;
            }
        }
    }
    wr::<u8>(rdx + 0x10, 0);
    rdx
}

pub unsafe fn diff_d3d_extract_swapchain_pair(orig: usize, new: usize) -> Result<u32, String> {
    let mut inner = [0u64; 3];
    inner[1] = 0x1111_2222;
    inner[2] = 0x3333_4444;
    let holder_valid: usize = inner.as_ptr() as usize;
    let holder_null: usize = 0;
    for ptr in [(&holder_null) as *const usize as usize, (&holder_valid) as *const usize as usize] {
        let mut out_o = [0u8; 0x18];
        let mut out_n = [0u8; 0x18];
        let r_o = exe::call_va!(orig, (ptr, out_o.as_mut_ptr() as usize) -> usize);
        let r_n = exe::call_va!(new, (ptr, out_n.as_mut_ptr() as usize) -> usize);
        crate::diff_eq!(r_o, out_o.as_ptr() as usize, "ret orig");
        crate::diff_eq!(r_n, out_n.as_ptr() as usize, "ret new");
        crate::diff_eq!(out_o, out_n, "pair out");
    }
    Ok(2)
}

/// Original: `0x14207d1b0` (43 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_copy_streaming_request_desc(rcx: usize, rdx: usize) -> usize {
    wr::<u64>(rcx, rd::<u64>(rdx));
    wr::<u8>(rcx + 0x8, rd::<u8>(rdx + 0x8));
    wr::<u64>(rcx + 0x10, 0);
    wr::<u64>(rcx + 0x18, 0);
    wr::<u64>(rcx + 0x18, rd::<u64>(rdx + 0x18));
    wr::<u64>(rcx + 0x10, rd::<u64>(rdx + 0x10));
    rcx
}

pub unsafe fn diff_d3d_copy_streaming_request_desc(orig: usize, new: usize) -> Result<u32, String> {
    let mut src = [0u8; 0x20];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(13).wrapping_add(5);
    }
    let mut dst_o = [0x55u8; 0x20];
    let mut dst_n = [0x55u8; 0x20];
    let r_o = exe::call_va!(orig, (dst_o.as_mut_ptr() as usize, src.as_ptr() as usize) -> usize);
    let r_n = exe::call_va!(new, (dst_n.as_mut_ptr() as usize, src.as_ptr() as usize) -> usize);
    crate::diff_eq!(r_o, dst_o.as_ptr() as usize, "ret orig");
    crate::diff_eq!(r_n, dst_n.as_ptr() as usize, "ret new");
    crate::diff_eq!(dst_o, dst_n, "copied desc");
    Ok(1)
}

unsafe extern "win64" fn mock_dev_check_feature_support_68(
    this: usize,
    feature: u32,
    data: usize,
    size: u32,
) -> i32 {
    wr::<u32>(this + 0x10, feature);
    wr::<u32>(this + 0x14, size);
    wr::<u32>(this + 0x18, rd::<u32>(data));
    wr::<u32>(data + 0x8, rd::<u32>(this + 0x8)); // support2 from [this + 0x8]
    rd::<i32>(this + 0xc) // hr from [this + 0xc]
}

unsafe extern "win64" fn mock_dev_create_heap_58(
    this: usize,
    desc: usize,
    _riid: usize,
    out_ppv: usize,
) -> i32 {
    wr::<u64>(this + 0x8, rd::<u64>(desc)); // size_in_bytes
    wr::<u64>(this + 0x10, rd::<u64>(desc + 0x8)); // props lo
    wr::<u64>(this + 0x18, rd::<u64>(desc + 0x10)); // props hi
    wr::<usize>(out_ppv, 0xbeef_cafe);
    0
}

unsafe extern "win64" fn mock_com_release_streaming(this: usize) -> u32 {
    let c = rd::<u32>(this + 0x8).wrapping_add(1);
    wr::<u32>(this + 0x8, c);
    c
}

unsafe extern "win64" fn mock_swapchain_create_target_50(
    this: usize,
    arg1: usize,
    arg2: usize,
    out_ptr: usize,
) -> i32 {
    wr::<usize>(this + 0x8, arg1);
    wr::<usize>(this + 0x10, arg2);
    wr::<usize>(out_ptr, 0x4444_5555);
    0
}

unsafe extern "win64" fn mock_swapchain_get_fullscreen_58(
    this: usize,
    out_state: usize,
    _out_target: usize,
) -> i32 {
    wr::<u32>(out_state, rd::<u32>(this + 0x8));
    0
}

unsafe extern "win64" fn mock_swapchain_get_desc_60(this: usize, desc: usize) -> i32 {
    wr::<u32>(desc + 0x40, rd::<u32>(this + 0x8)); // flags at +0x40
    0
}

unsafe extern "win64" fn mock_swapchain_resize_buffers_68(
    this: usize,
    buf_count: u32,
    width: u32,
    height: u32,
    format: u32,
    flags: u32,
) -> i32 {
    wr::<u32>(this + 0x10, buf_count);
    wr::<u32>(this + 0x14, width);
    wr::<u32>(this + 0x18, height);
    wr::<u32>(this + 0x1c, format);
    wr::<u32>(this + 0x20, flags);
    rd::<i32>(this + 0xc) // hr from [this + 0xc]
}

unsafe extern "win64" fn mock_factory_query_interface_0(
    this: usize,
    _riid: usize,
    out_ppv: usize,
) -> i32 {
    let hr = rd::<i32>(this + 0xc);
    if hr >= 0 {
        wr::<usize>(out_ppv, this);
    } else {
        wr::<usize>(out_ppv, 0);
    }
    hr
}

unsafe extern "win64" fn mock_factory5_check_feature_e0(
    this: usize,
    feature: u32,
    out_allow: usize,
    size: u32,
) -> i32 {
    wr::<u32>(this + 0x14, feature);
    wr::<u32>(this + 0x18, size);
    wr::<u32>(out_allow, rd::<u32>(this + 0x10));
    0
}

pub unsafe fn diff_d3d_check_format_support_bit6(orig: usize, new: usize) -> Result<u32, String> {
    let dev_slot = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_dev = rd::<usize>(dev_slot);

    let mut vtbl_dev = [0usize; 16];
    vtbl_dev[13] = mock_dev_check_feature_support_68 as *const () as usize;

    for (support2, hr) in [(0u32, 0i32), (0x40u32, 0i32), (0x40u32, -1i32)] {
        let mut dev_o = [0u64; 4];
        let mut dev_n = [0u64; 4];
        let do_p = dev_o.as_mut_ptr() as usize;
        let dn_p = dev_n.as_mut_ptr() as usize;
        wr::<usize>(do_p, vtbl_dev.as_ptr() as usize);
        wr::<u32>(do_p + 0x8, support2);
        wr::<i32>(do_p + 0xc, hr);

        wr::<usize>(dn_p, vtbl_dev.as_ptr() as usize);
        wr::<u32>(dn_p + 0x8, support2);
        wr::<i32>(dn_p + 0xc, hr);

        wr::<usize>(dev_slot, do_p);
        let ro = exe::call_va!(orig, (28u32) -> u8);

        wr::<usize>(dev_slot, dn_p);
        let rn = exe::call_va!(new, (28u32) -> u8);

        crate::diff_eq!(ro, rn, "bit6_support");
        crate::diff_eq!(&dev_o[1..], &dev_n[1..], "dev_feature_call");
    }

    wr::<usize>(dev_slot, saved_dev);
    Ok(3)
}

pub unsafe fn diff_d3d_create_heap_wrapper(orig: usize, new: usize) -> Result<u32, String> {
    let dev_slot = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_dev = rd::<usize>(dev_slot);

    let mut vtbl_dev = [0usize; 14];
    vtbl_dev[11] = mock_dev_create_heap_58 as *const () as usize;

    let mut dev_o = [0u64; 4];
    let mut dev_n = [0u64; 4];
    dev_o[0] = vtbl_dev.as_ptr() as usize as u64;
    dev_n[0] = vtbl_dev.as_ptr() as usize as u64;

    let props = [0x1122_3344_5566_7788u64, 0x99aa_bbcc_ddee_ff00u64];
    let mut out_o = [0xaaaa_bbbbu64, 0xcccc_ddddu64];
    let mut out_n = [0xaaaa_bbbbu64, 0xcccc_ddddu64];

    wr::<usize>(dev_slot, dev_o.as_mut_ptr() as usize);
    let ro = exe::call_va!(
        orig,
        (out_o.as_mut_ptr() as usize, props.as_ptr() as usize, 0x40000u64) -> usize
    );

    wr::<usize>(dev_slot, dev_n.as_mut_ptr() as usize);
    let rn = exe::call_va!(
        new,
        (out_n.as_mut_ptr() as usize, props.as_ptr() as usize, 0x40000u64) -> usize
    );

    wr::<usize>(dev_slot, saved_dev);

    crate::diff_eq!(
        ro - out_o.as_ptr() as usize,
        rn - out_n.as_ptr() as usize,
        "ret_off"
    );
    crate::diff_eq!(out_o, out_n, "heap_wrapper_out");
    crate::diff_eq!(&dev_o[1..], &dev_n[1..], "dev_create_heap");
    Ok(1)
}

pub unsafe fn diff_d3d_copy_construct_subresource_list(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut src = [0u64; 6];
    src[0] = 0x1111_2222_3333_4444;
    src[1] = 0x5555_6666;
    src[2] = 0; // vec ptr = 0
    src[3] = 0; // count (+0x18) = 0 -> no heap alloc
    src[4] = 0x7777_8888_9999_aaaa;
    src[5] = 0xbbbb_cccc;

    let mut dst_o = [0xeeu8; 0x30];
    let mut dst_n = [0xeeu8; 0x30];

    let ro = exe::call_va!(
        orig,
        (dst_o.as_mut_ptr() as usize, src.as_ptr() as usize) -> usize
    );
    let rn = exe::call_va!(
        new,
        (dst_n.as_mut_ptr() as usize, src.as_ptr() as usize) -> usize
    );

    crate::diff_eq!(
        ro - dst_o.as_ptr() as usize,
        rn - dst_n.as_ptr() as usize,
        "ret_off"
    );
    crate::diff_eq!(&dst_o[..0x0c], &dst_n[..0x0c], "head");
    crate::diff_eq!(&dst_o[0x10..0x2c], &dst_n[0x10..0x2c], "tail");
    Ok(1)
}

pub unsafe fn diff_d3d_clone_subresource_list_into(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut src = [0u64; 6];
    src[0] = 0x1234_5678_9abc_def0;
    src[1] = 0x1357_2468;
    src[2] = 0;
    src[3] = 0; // count (+0x18) = 0
    src[4] = 0xfedc_ba98_7654_3210;
    src[5] = 0x2468_1357;

    let mut dst_o = [0xdd_u8; 0x30];
    let mut dst_n = [0xdd_u8; 0x30];

    let ro = exe::call_va!(
        orig,
        (src.as_ptr() as usize, dst_o.as_mut_ptr() as usize) -> usize
    );
    let rn = exe::call_va!(
        new,
        (src.as_ptr() as usize, dst_n.as_mut_ptr() as usize) -> usize
    );

    crate::diff_eq!(
        ro - dst_o.as_ptr() as usize,
        rn - dst_n.as_ptr() as usize,
        "ret_off"
    );
    crate::diff_eq!(&dst_o[..0x0c], &dst_n[..0x0c], "head");
    crate::diff_eq!(&dst_o[0x10..0x2c], &dst_n[0x10..0x2c], "tail");
    Ok(1)
}

pub unsafe fn diff_d3d_recreate_swapchain_target(orig: usize, new: usize) -> Result<u32, String> {
    let mut vtbl_rsi = [0usize; 12];
    vtbl_rsi[10] = mock_swapchain_create_target_50 as *const () as usize;
    let mut vtbl_old = [0usize; 4];
    vtbl_old[2] = mock_com_release_streaming as *const () as usize;

    let mut rsi_o = [vtbl_rsi.as_ptr() as usize, 0usize, 0usize];
    let mut rsi_n = [vtbl_rsi.as_ptr() as usize, 0usize, 0usize];
    let mut old_o = [vtbl_old.as_ptr() as usize, 0usize];
    let mut old_n = [vtbl_old.as_ptr() as usize, 0usize];

    let mut rbx_o = [0usize, old_o.as_mut_ptr() as usize];
    let mut rbx_n = [0usize, old_n.as_mut_ptr() as usize];

    let p0_o = [rsi_o.as_mut_ptr() as usize];
    let p0_n = [rsi_n.as_mut_ptr() as usize];
    let p8_o = [rbx_o.as_mut_ptr() as usize];
    let p8_n = [rbx_n.as_mut_ptr() as usize];

    let rcx_o = [p0_o.as_ptr() as usize, p8_o.as_ptr() as usize, 0x9999usize];
    let rcx_n = [p0_n.as_ptr() as usize, p8_n.as_ptr() as usize, 0x9999usize];

    exe::call_va!(orig, (rcx_o.as_ptr() as usize, 0x7777usize) -> ());
    exe::call_va!(new, (rcx_n.as_ptr() as usize, 0x7777usize) -> ());

    crate::diff_eq!(old_o[1], old_n[1], "old_released");
    crate::diff_eq!(rbx_o[1], rbx_n[1], "new_target_ptr");
    crate::diff_eq!(&rsi_o[1..], &rsi_n[1..], "rsi_create_args");
    Ok(1)
}

pub unsafe fn diff_d3d_enable_os_hdr_display(orig: usize, new: usize) -> Result<u32, String> {
    let slot = exe::ptr::<u32>(0x1_43a2_b08c) as usize;
    let saved = rd::<u32>(slot);

    for state in [1u32, 2, 3] {
        wr::<u32>(slot, state);
        let ro = exe::call_va!(orig, (1u8) -> u8);
        wr::<u32>(slot, state);
        let rn = exe::call_va!(new, (1u8) -> u8);
        crate::diff_eq!(ro, rn, "enable_os_hdr");
    }

    wr::<u32>(slot, saved);
    Ok(3)
}

pub unsafe fn diff_d3d_is_os_hdr_enabled(orig: usize, new: usize) -> Result<u32, String> {
    let slot = exe::ptr::<u32>(0x1_43a2_b08c) as usize;
    let saved = rd::<u32>(slot);

    for state in [1u32, 2, 3] {
        wr::<u32>(slot, state);
        let ro = exe::call_va!(orig, () -> u8);
        wr::<u32>(slot, state);
        let rn = exe::call_va!(new, () -> u8);
        crate::diff_eq!(ro, rn, "is_os_hdr_enabled");
    }

    wr::<u32>(slot, saved);
    Ok(3)
}

pub unsafe fn diff_d3d_is_os_hdr_supported(orig: usize, new: usize) -> Result<u32, String> {
    let slot = exe::ptr::<u32>(0x1_43a2_b08c) as usize;
    let saved = rd::<u32>(slot);

    for state in [1u32, 2, 3] {
        wr::<u32>(slot, state);
        let ro = exe::call_va!(orig, () -> u8);
        wr::<u32>(slot, state);
        let rn = exe::call_va!(new, () -> u8);
        crate::diff_eq!(ro, rn, "is_os_hdr_supported");
    }

    wr::<u32>(slot, saved);
    Ok(3)
}

pub unsafe fn diff_d3d_is_swapchain_fullscreen(orig: usize, new: usize) -> Result<u32, String> {
    let mut vtbl_sc = [0usize; 14];
    vtbl_sc[11] = mock_swapchain_get_fullscreen_58 as *const () as usize;

    for state in [0u32, 1, 2] {
        let mut sc_o = [vtbl_sc.as_ptr() as usize as u64, state as u64];
        let mut sc_n = [vtbl_sc.as_ptr() as usize as u64, state as u64];

        let ro = exe::call_va!(orig, (sc_o.as_mut_ptr() as usize) -> u8);
        let rn = exe::call_va!(new, (sc_n.as_mut_ptr() as usize) -> u8);
        crate::diff_eq!(ro, rn, "is_fullscreen");
    }
    Ok(3)
}

pub unsafe fn diff_d3d_release_backbuffer_views(orig: usize, new: usize) -> Result<u32, String> {
    let mut buf_o = [0u64; 6];
    let mut buf_n = [0u64; 6];
    // Non-zero sentinel at +0x10 and +0x28; null pointers at +0x0, +0x8, +0x18, +0x20
    buf_o[2] = 0xaaaa_bbbb;
    buf_o[5] = 0xcccc_dddd;
    buf_n[2] = 0xaaaa_bbbb;
    buf_n[5] = 0xcccc_dddd;

    exe::call_va!(orig, (buf_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (buf_n.as_mut_ptr() as usize) -> ());

    crate::diff_eq!(buf_o, buf_n, "backbuffer_views_cleared");
    Ok(1)
}

pub unsafe fn diff_d3d_resize_swapchain_buffers(orig: usize, new: usize) -> Result<u32, String> {
    let mut vtbl_sc = [0usize; 16];
    vtbl_sc[12] = mock_swapchain_get_desc_60 as *const () as usize;
    vtbl_sc[13] = mock_swapchain_resize_buffers_68 as *const () as usize;

    for hr in [0i32, -0x7ff8_fffbi32 /* 0x80070005 */, -0x7fff_bffbi32 /* 0x80004005 */] {
        let mut sc_o = [0u64; 6];
        let mut sc_n = [0u64; 6];
        let so = sc_o.as_mut_ptr() as usize;
        let sn = sc_n.as_mut_ptr() as usize;

        for base in [so, sn] {
            wr::<usize>(base, vtbl_sc.as_ptr() as usize);
            wr::<u32>(base + 0x8, 0x800); // desc flags
            wr::<i32>(base + 0xc, hr);
        }

        let ro = exe::call_va!(orig, (so, 1920u32, 1080u32, 28u32) -> u8);
        let rn = exe::call_va!(new, (sn, 1920u32, 1080u32, 28u32) -> u8);

        crate::diff_eq!(ro, rn, "resize_ok");
        crate::diff_eq!(&sc_o[2..], &sc_n[2..], "resize_recorded_args");
    }
    Ok(3)
}

pub unsafe fn diff_d3d_subresource_iter_advance(orig: usize, new: usize) -> Result<u32, String> {
    let mut it_o = [0u64; 6];
    let mut it_n = [0u64; 6];
    let io = it_o.as_mut_ptr() as usize;
    let in_p = it_n.as_mut_ptr() as usize;

    // Set count (+0x18) = 4, idx (+0x20) = 1 -> advances to 2 and 3 (< 4) without calling 0x14207c7a0
    wr::<u32>(io + 0x18, 4);
    wr::<u64>(io + 0x20, 1);
    wr::<u32>(in_p + 0x18, 4);
    wr::<u64>(in_p + 0x20, 1);

    for _ in 0..2 {
        let ro = exe::call_va!(orig, (io) -> usize);
        let rn = exe::call_va!(new, (in_p) -> usize);
        crate::diff_eq!(ro - io, rn - in_p, "ret_off");
        crate::diff_eq!(it_o, it_n, "iter_state");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_check_dxgi_factory_tearing_support(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let mut vtbl_fac = [0usize; 30];
    vtbl_fac[0] = mock_factory_query_interface_0 as *const () as usize;
    vtbl_fac[2] = mock_com_release_streaming as *const () as usize;
    vtbl_fac[28] = mock_factory5_check_feature_e0 as *const () as usize;

    for (qi_hr, allow_tearing) in [(-1i32, 1u32), (0i32, 0u32), (0i32, 1u32)] {
        let mut fac_o = [0u64; 4];
        let mut fac_n = [0u64; 4];
        let fo = fac_o.as_mut_ptr() as usize;
        let fn_p = fac_n.as_mut_ptr() as usize;

        for base in [fo, fn_p] {
            wr::<usize>(base, vtbl_fac.as_ptr() as usize);
            wr::<u32>(base + 0x8, 0); // release count
            wr::<i32>(base + 0xc, qi_hr);
            wr::<u32>(base + 0x10, allow_tearing);
        }

        let ro = exe::call_va!(orig, (fo) -> u8);
        let rn = exe::call_va!(new, (fn_p) -> u8);

        crate::diff_eq!(ro, rn, "tearing_supported");
        crate::diff_eq!(&fac_o[1..], &fac_n[1..], "fac_state");
    }
    Ok(3)
}

unsafe extern "win64" fn mock_stream_buf_reset_142075970(p: usize) {
    wr::<u64>(p + 0x18, 0x1111_2222);
}

unsafe extern "win64" fn mock_stream_buf_init_142075080(
    p: usize,
    rdx_arg: u64,
    r8: usize,
    r9: usize,
    a5: u64,
    rbx: u64,
    eax: u32,
) {
    let mix = rdx_arg
        ^ rbx
        ^ (eax as u64)
        ^ (r8 as u64)
        ^ (r9 as u64)
        ^ a5;
    wr::<u64>(p + 0x20, mix);
}

unsafe extern "win64" fn mock_stream_sub_init_14209d5c0(p330: usize, count: u64, flag: u8) {
    wr::<u64>(p330, count);
    wr::<u8>(p330 + 0x8, flag);
}

pub unsafe fn diff_d3d_init_streaming_buffer_ctx(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4207_5970, mock_stream_buf_reset_142075970 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_5080, mock_stream_buf_init_142075080 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_d5c0, mock_stream_sub_init_14209d5c0 as *const ())?;

    for f101 in [0u8, 1u8] {
        let mut rdx_buf = [0u8; 0x110];
        rdx_buf[0x100] = 1;
        rdx_buf[0x101] = f101;

        let mut rcx_o = vec![0u8; 0x3c0];
        let co = rcx_o.as_mut_ptr() as usize;
        wr::<u64>(co + 0xd0, 0xaaaa_0001);
        wr::<u64>(co + 0xe0, 0xbbbb_0002);
        wr::<u64>(co + 0x110, 0xcccc_0003);
        wr::<u32>(co + 0x11c, 7);
        wr::<u32>(co + 0x258, 3);
        let mut rcx_n = rcx_o.clone();
        let cn = rcx_n.as_mut_ptr() as usize;

        exe::call_va!(orig, (co, rdx_buf.as_ptr() as usize) -> ());
        exe::call_va!(new, (cn, rdx_buf.as_ptr() as usize) -> ());
        crate::diff_eq!(rcx_o, rcx_n, "streaming_buffer_ctx");
    }
    Ok(2)
}

unsafe extern "win64" fn mock_alloc_desc_range_142093d00(
    rcx: usize,
    rsi: u64,
    out0: usize,
    out2: usize,
    out1: usize,
) {
    wr::<u64>(out0, rsi.wrapping_add(0x9000));
    wr::<u64>(out2, 0x4000);
    wr::<usize>(out1, rcx + 0x10);
}

unsafe extern "win64" fn mock_dev_copy_descriptors_c0(
    this: usize,
    num: u32,
    dst: u64,
    src: u64,
    heap_type: u32,
) {
    let mix = (num as u64)
        .wrapping_add(dst)
        .wrapping_add(src)
        .wrapping_add(heap_type as u64);
    wr::<u64>(this + 0x8, rd::<u64>(this + 0x8).wrapping_add(mix));
}

pub unsafe fn diff_d3d_copy_descriptor_range_table(orig: usize, new: usize) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4209_3d00, mock_alloc_desc_range_142093d00 as *const ())?;

    let stride_slot = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let dev_slot = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_stride = rd::<u32>(stride_slot);
    let saved_dev = rd::<usize>(dev_slot);

    let mut vtbl_dev = [0usize; 26];
    vtbl_dev[24] = mock_dev_copy_descriptors_c0 as *const () as usize;
    let mut dev_o = [vtbl_dev.as_ptr() as usize as u64, 0u64];
    let mut dev_n = [vtbl_dev.as_ptr() as usize as u64, 0u64];

    let handles: [u64; 2] = [0x1234, 0];
    let rcx_buf = [2u64, handles.as_ptr() as u64, 0x7777_8888u64];
    let mut out_o = 0u64;
    let mut out_n = 0u64;

    wr::<u32>(stride_slot, 32);
    wr::<usize>(dev_slot, dev_o.as_mut_ptr() as usize);
    let ro = exe::call_va!(
        orig,
        (rcx_buf.as_ptr() as usize, (&mut out_o as *mut u64) as usize) -> u64
    );

    wr::<usize>(dev_slot, dev_n.as_mut_ptr() as usize);
    let rn = exe::call_va!(
        new,
        (rcx_buf.as_ptr() as usize, (&mut out_n as *mut u64) as usize) -> u64
    );

    wr::<u32>(stride_slot, saved_stride);
    wr::<usize>(dev_slot, saved_dev);

    crate::diff_eq!(ro, rn, "ret out0");
    crate::diff_eq!(out_o, out_n, "first_val");
    crate::diff_eq!(dev_o[1], dev_n[1], "copied descriptors");
    Ok(1)
}

unsafe extern "win64" fn mock_get_unk_14208d1f0(_mgr: usize, out_ptr: usize, _flag: u8) {
    wr::<usize>(out_ptr, 0);
}

unsafe extern "win64" fn mock_on_cvar_change_14207a1e0(rcx: usize, rdx: usize, r8: u8) {
    wr::<u64>(rcx, (rdx as u64) | ((r8 as u64) << 8) | 0x5500);
}

pub unsafe fn diff_d3d_sync_streaming_cvars(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4208_d1f0, mock_get_unk_14208d1f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_a1e0, mock_on_cvar_change_14207a1e0 as *const ())?;

    let g1 = exe::ptr::<u32>(0x1_43a2_b11c) as usize;
    let g2 = exe::ptr::<u32>(0x1_43a2_b08c) as usize;
    let g3 = exe::ptr::<u32>(0x1_4385_49ac) as usize;
    let s1 = exe::ptr::<u32>(0x1_43a6_3738) as usize;
    let s2 = exe::ptr::<u32>(0x1_43a6_3710) as usize;
    let s3 = exe::ptr::<u32>(0x1_4385_7fe8) as usize;

    let saved = [
        rd::<u32>(g1),
        rd::<u32>(g2),
        rd::<u32>(g3),
        rd::<u32>(s1),
        rd::<u32>(s2),
        rd::<u32>(s3),
    ];

    wr::<u32>(s1, 2);
    wr::<u32>(s2, 3);
    wr::<f32>(s3, 1.5);

    let mut rcx_o = 0u64;
    let mut rcx_n = 0u64;

    wr::<u32>(g1, 1);
    wr::<u32>(g2, 3);
    wr::<f32>(g3, 1.5);
    exe::call_va!(orig, ((&mut rcx_o as *mut u64) as usize) -> ());
    let after_o = [rd::<u32>(g1), rd::<u32>(g2), rd::<u32>(g3)];

    wr::<u32>(g1, 1);
    wr::<u32>(g2, 3);
    wr::<f32>(g3, 1.5);
    exe::call_va!(new, ((&mut rcx_n as *mut u64) as usize) -> ());
    let after_n = [rd::<u32>(g1), rd::<u32>(g2), rd::<u32>(g3)];

    wr::<u32>(g1, saved[0]);
    wr::<u32>(g2, saved[1]);
    wr::<u32>(g3, saved[2]);
    wr::<u32>(s1, saved[3]);
    wr::<u32>(s2, saved[4]);
    wr::<u32>(s3, saved[5]);

    crate::diff_eq!(rcx_o, rcx_n, "cvar notify");
    crate::diff_eq!(after_o, after_n, "cvars synced");
    Ok(1)
}

unsafe extern "win64" fn mock_sc_set_fullscreen_50(this: usize, state: u32, _target: usize) -> i32 {
    wr::<u32>(this + 0x10, state.wrapping_add(0x40));
    0
}

unsafe extern "win64" fn mock_free_sub_14209a1e0(p20: usize, v18: u32) {
    wr::<u32>(p20, v18.wrapping_add(0x100));
}

unsafe extern "win64" fn mock_free_obj_142442de0(rbx: usize) {
    wr::<u32>(rbx + 0x1c, 0xdead);
}

pub unsafe fn diff_d3d_destroy_swapchain_context(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4209_a1e0, mock_free_sub_14209a1e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_2de0, mock_free_obj_142442de0 as *const ())?;

    let mut vtbl_sc = [0usize; 14];
    vtbl_sc[2] = mock_com_release_streaming as *const () as usize;
    vtbl_sc[10] = mock_sc_set_fullscreen_50 as *const () as usize;
    vtbl_sc[11] = mock_swapchain_get_fullscreen_58 as *const () as usize;

    let mut sc_o = [0u64; 3];
    wr::<usize>(sc_o.as_mut_ptr() as usize, vtbl_sc.as_ptr() as usize);
    wr::<u32>((sc_o.as_mut_ptr() as usize) + 0x8, 1);
    let mut sc_n = sc_o;

    let mut p20_o = 0u32;
    let mut p20_n = 0u32;

    let mut inner_o = [0u8; 0x40];
    let io = inner_o.as_mut_ptr() as usize;
    wr::<usize>(io + 0x8, sc_o.as_mut_ptr() as usize);
    wr::<u32>(io + 0x18, 7);
    wr::<usize>(io + 0x20, (&mut p20_o as *mut u32) as usize);

    let mut inner_n = [0u8; 0x40];
    let in_p = inner_n.as_mut_ptr() as usize;
    wr::<usize>(in_p + 0x8, sc_n.as_mut_ptr() as usize);
    wr::<u32>(in_p + 0x18, 7);
    wr::<usize>(in_p + 0x20, (&mut p20_n as *mut u32) as usize);

    let mut ctx_o = [0u8; 0x40];
    wr::<usize>(ctx_o.as_mut_ptr() as usize, io);
    let mut ctx_n = [0u8; 0x40];
    wr::<usize>(ctx_n.as_mut_ptr() as usize, in_p);

    exe::call_va!(orig, (ctx_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (ctx_n.as_mut_ptr() as usize) -> ());

    crate::diff_eq!(sc_o, sc_n, "sc fullscreen + release");
    crate::diff_eq!(p20_o, p20_n, "p20 freed");
    wr::<usize>(io + 0x8, 0);
    wr::<usize>(io + 0x20, 0);
    wr::<usize>(in_p + 0x8, 0);
    wr::<usize>(in_p + 0x20, 0);
    crate::diff_eq!(inner_o, inner_n, "inner destroyed");
    Ok(1)
}

unsafe extern "win64" fn mock_sl_create_6498(dev: usize, pkt1: usize, _r8: usize) -> i32 {
    let hdr = rd::<u64>(pkt1);
    let f18 = rd::<u64>(pkt1 + 0x18);
    let f28 = rd::<u64>(pkt1 + 0x28);
    wr::<u64>(dev + 0x8, hdr ^ f18 ^ f28);
    let out_ptr = rd::<usize>(pkt1 + 0x10);
    wr::<usize>(out_ptr, 0x8888_9999);
    0
}

unsafe extern "win64" fn mock_sl_query_6488(dev: usize, pkt2: usize) -> i32 {
    wr::<u64>(dev + 0x10, rd::<u64>(pkt2));
    0
}

pub unsafe fn diff_d3d_nvapi_init_swapchain_hdr(orig: usize, new: usize) -> Result<u32, String> {
    let _im1 = crate::difftest::IatMock::new(0x1_4264_6498, mock_sl_create_6498 as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_6488, mock_sl_query_6488 as *const ())?;

    let mut vtbl_com = [0usize; 4];
    vtbl_com[2] = mock_com_release_streaming as *const () as usize;

    let mut f20_o = [vtbl_com.as_ptr() as usize as u64, 0u64];
    let mut f20_n = f20_o;
    let mut old_o = [vtbl_com.as_ptr() as usize as u64, 0u64];
    let mut old_n = old_o;

    let mut dev_o = [0x1234_5678u64, 0, 0];
    let mut dev_n = dev_o;
    let p8_o = [f20_o.as_mut_ptr() as usize];
    let p8_n = [f20_n.as_mut_ptr() as usize];

    let mut holder_o = [0usize, old_o.as_mut_ptr() as usize, 0usize];
    let mut holder_n = [0usize, old_n.as_mut_ptr() as usize, 0usize];
    let p10_o = [holder_o.as_mut_ptr() as usize];
    let p10_n = [holder_n.as_mut_ptr() as usize];

    let rcx_o = [
        dev_o.as_mut_ptr() as usize,
        p8_o.as_ptr() as usize,
        p10_o.as_ptr() as usize,
        0x5555usize,
    ];
    let rcx_n = [
        dev_n.as_mut_ptr() as usize,
        p8_n.as_ptr() as usize,
        p10_n.as_ptr() as usize,
        0x5555usize,
    ];

    exe::call_va!(orig, (rcx_o.as_ptr() as usize, 0x7777u64) -> ());
    exe::call_va!(new, (rcx_n.as_ptr() as usize, 0x7777u64) -> ());

    crate::diff_eq!(dev_o, dev_n, "sl dev packets");
    crate::diff_eq!(f20_o[1], f20_n[1], "f20 released");
    crate::diff_eq!(old_o[1], old_n[1], "old released");
    crate::diff_eq!(holder_o, holder_n, "holder updated");
    Ok(1)
}

unsafe extern "win64" fn mock_release_bb_14207c130(r8: usize) {
    wr::<u32>(r8, 0x1111);
}

unsafe extern "win64" fn mock_flush_idle_14207bf60() {}

unsafe extern "win64" fn mock_resize_sc_14207c410(
    rcx: usize,
    r9d: u32,
    arg5: u32,
    arg6: u32,
) -> u8 {
    wr::<u32>(rcx, r9d.wrapping_add(arg5).wrapping_add(arg6));
    1
}

unsafe extern "win64" fn mock_rebuild_bb_14207c040(_rcx: usize, rdx: u8, r8: usize) {
    wr::<u8>(r8 + 0x4, rdx);
}

pub unsafe fn diff_d3d_resize_swapchain_and_buffers(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4207_c130, mock_release_bb_14207c130 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_bf60, mock_flush_idle_14207bf60 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_c410, mock_resize_sc_14207c410 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_c040, mock_rebuild_bb_14207c040 as *const ())?;

    let mut sc_o = 0u32;
    let mut sc_n = 0u32;
    let mut bb_o = [0u8; 8];
    let mut bb_n = [0u8; 8];

    let ro = exe::call_va!(
        orig,
        ((&mut sc_o as *mut u32) as usize, 3u8, bb_o.as_mut_ptr() as usize, 1920u32, 1080u32, 28u32) -> u8
    );
    let rn = exe::call_va!(
        new,
        ((&mut sc_n as *mut u32) as usize, 3u8, bb_n.as_mut_ptr() as usize, 1920u32, 1080u32, 28u32) -> u8
    );

    crate::diff_eq!(ro, rn, "resize ret");
    crate::diff_eq!(sc_o, sc_n, "sc args");
    crate::diff_eq!(bb_o, bb_n, "bb state");
    Ok(1)
}

unsafe extern "win64" fn mock_tls_flush_1420761a0(p: usize) {
    wr::<u32>(p, 0xaaaa);
}

unsafe extern "win64" fn mock_tls_submit_14206df80(p: usize) {
    wr::<u32>(p, 0xbbbb);
}

unsafe extern "win64" fn mock_queue_step_1420995c0(_idx: u32) {}

unsafe extern "win64" fn mock_queue_wait_142099c20(_a: u8, _b: u32) {}

unsafe extern "win64" fn mock_get_fence_pair_1420734e0(out_pair: usize, _idx: u32) {
    wr::<u64>(out_pair, 10);
    wr::<usize>(out_pair + 0x8, 0);
}

pub unsafe fn diff_d3d_flush_gpu_and_wait_idle(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4207_61a0, mock_tls_flush_1420761a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_df80, mock_tls_submit_14206df80 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_95c0, mock_queue_step_1420995c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_9c20, mock_queue_wait_142099c20 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4207_34e0, mock_get_fence_pair_1420734e0 as *const ())?;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut sub_o = 0u32;
    let mut sub_n = 0u32;
    let mut tls_o = vec![0u8; 0x46800];
    let mut tls_n = vec![0u8; 0x46800];
    tls_o[0x70] = 1;
    tls_n[0x70] = 1;
    wr::<usize>((tls_o.as_mut_ptr() as usize) + 0x6328, (&mut sub_o as *mut u32) as usize);
    wr::<usize>((tls_n.as_mut_ptr() as usize) + 0x6328, (&mut sub_n as *mut u32) as usize);

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    exe::call_va!(orig, () -> ());

    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    exe::call_va!(new, () -> ());

    wr::<usize>(tls_slot, saved_tls0);

    crate::diff_eq!(
        rd::<u32>((tls_o.as_ptr() as usize) + 0x467e0),
        rd::<u32>((tls_n.as_ptr() as usize) + 0x467e0),
        "tls +0x467e0 flushed"
    );
    crate::diff_eq!(sub_o, sub_n, "tls +0x6328 submitted");
    Ok(1)
}

unsafe extern "win64" fn mock_get_bb_tex_14207c1c0(
    out_tmp: usize,
    _rcx: usize,
    rdx: u8,
    edi: u32,
) -> usize {
    wr::<usize>(out_tmp, 0x1000usize + ((rdx as usize) << 8) + (edi as usize));
    out_tmp
}

unsafe extern "win64" fn mock_create_bb_view_1411df430(
    _rcx: usize,
    _rdx: usize,
    tex_ptr: usize,
) -> usize {
    rd::<usize>(tex_ptr).wrapping_add(0x400)
}

unsafe extern "win64" fn mock_sc_get_current_bb_index_120(_this: usize) -> u32 {
    1
}

pub unsafe fn diff_d3d_rebuild_backbuffer_views(orig: usize, new: usize) -> Result<u32, String> {
    let _m1 = crate::difftest::Mock::new(0x1_4207_c1c0, mock_get_bb_tex_14207c1c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_f430, mock_create_bb_view_1411df430 as *const ())?;

    let mut vtbl_sc = [0usize; 38];
    vtbl_sc[36] = mock_sc_get_current_bb_index_120 as *const () as usize;
    let sc = [vtbl_sc.as_ptr() as usize];

    let mut bb_o = [0u8; 0x38];
    let mut bb_n = [0u8; 0x38];

    exe::call_va!(orig, (sc.as_ptr() as usize, 2u8, bb_o.as_mut_ptr() as usize) -> ());
    exe::call_va!(new, (sc.as_ptr() as usize, 2u8, bb_n.as_mut_ptr() as usize) -> ());

    crate::diff_eq!(bb_o, bb_n, "rebuilt backbuffer views");
    Ok(1)
}

unsafe extern "win64" fn mock_subres_step_14207c7a0(iter_ptr: usize) {
    let f00 = rd::<u64>(iter_ptr);
    let f28 = rd::<u32>(iter_ptr + 0x28);
    wr::<u32>(iter_ptr + 0x8, f28.wrapping_add(3));
    wr::<u64>(iter_ptr + 0x10, f00 ^ 0xdead_beef);
    wr::<u32>(iter_ptr + 0x18, 2);
    wr::<u32>(iter_ptr + 0x1c, 4);
    wr::<u64>(iter_ptr + 0x20, 1);
}

pub unsafe fn diff_d3d_create_subresource_iter(orig: usize, new: usize) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4207_c7a0, mock_subres_step_14207c7a0 as *const ())?;

    let mut out_o = [0x5au8; 0x30];
    let mut out_n = [0x5au8; 0x30];

    let ro = exe::call_va!(orig, (out_o.as_mut_ptr() as usize, 0x1234_5678u64, 7u32) -> usize);
    let rn = exe::call_va!(new, (out_n.as_mut_ptr() as usize, 0x1234_5678u64, 7u32) -> usize);

    crate::diff_eq!(ro - (out_o.as_ptr() as usize), rn - (out_n.as_ptr() as usize), "ret self");
    crate::diff_eq!(&out_o[0..0x0c], &out_n[0..0x0c], "head");
    crate::diff_eq!(&out_o[0x10..0x2c], &out_n[0x10..0x2c], "tail");
    Ok(1)
}

pub unsafe fn diff_d3d_subresource_iter_init_in_place(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4207_c7a0, mock_subres_step_14207c7a0 as *const ())?;

    let mut out_o = [0x5au8; 0x30];
    let mut out_n = [0x5au8; 0x30];

    let ro = exe::call_va!(orig, (out_o.as_mut_ptr() as usize, 0x8765_4321u64, 9u32) -> usize);
    let rn = exe::call_va!(new, (out_n.as_mut_ptr() as usize, 0x8765_4321u64, 9u32) -> usize);

    crate::diff_eq!(ro - (out_o.as_ptr() as usize), rn - (out_n.as_ptr() as usize), "ret self");
    crate::diff_eq!(&out_o[0..0x0c], &out_n[0..0x0c], "head");
    crate::diff_eq!(&out_o[0x10..0x2c], &out_n[0x10..0x2c], "tail");
    Ok(1)
}

// REGISTRY:
// (0x1_4207_8b90, d3d_init_streaming_buffer_ctx, diff_d3d_init_streaming_buffer_ctx, Verified)
// (0x1_4207_9bd0, d3d_copy_descriptor_range_table, diff_d3d_copy_descriptor_range_table, Verified)
// (0x1_4207_9c90, d3d_check_format_support_bit6, diff_d3d_check_format_support_bit6, Verified)
// (0x1_4207_9ce0, d3d_create_heap_wrapper, diff_d3d_create_heap_wrapper, Verified)
// (0x1_4207_9f80, d3d_copy_construct_subresource_list, diff_d3d_copy_construct_subresource_list, Verified)
// (0x1_4207_a060, d3d_clone_subresource_list_into, diff_d3d_clone_subresource_list_into, Verified)
// (0x1_4207_ab70, d3d_sync_streaming_cvars, diff_d3d_sync_streaming_cvars, Verified)
// (0x1_4207_ac10, d3d_destroy_swapchain_context, diff_d3d_destroy_swapchain_context, Verified)
// (0x1_4207_ad20, d3d_recreate_swapchain_target, diff_d3d_recreate_swapchain_target, Verified)
// (0x1_4207_ade0, d3d_nvapi_init_swapchain_hdr, diff_d3d_nvapi_init_swapchain_hdr, Verified)
// (0x1_4207_b620, d3d_enable_os_hdr_display, diff_d3d_enable_os_hdr_display, Verified)
// (0x1_4207_b710, d3d_is_os_hdr_enabled, diff_d3d_is_os_hdr_enabled, Verified)
// (0x1_4207_b7a0, d3d_is_os_hdr_supported, diff_d3d_is_os_hdr_supported, Verified)
// (0x1_4207_bd60, d3d_is_swapchain_fullscreen, diff_d3d_is_swapchain_fullscreen, Verified)
// (0x1_4207_bdb0, d3d_resize_swapchain_and_buffers, diff_d3d_resize_swapchain_and_buffers, Verified)
// (0x1_4207_bf60, d3d_flush_gpu_and_wait_idle, diff_d3d_flush_gpu_and_wait_idle, Verified)
// (0x1_4207_c040, d3d_rebuild_backbuffer_views, diff_d3d_rebuild_backbuffer_views, Verified)
// (0x1_4207_c130, d3d_release_backbuffer_views, diff_d3d_release_backbuffer_views, Verified)
// (0x1_4207_c410, d3d_resize_swapchain_buffers, diff_d3d_resize_swapchain_buffers, Verified)
// (0x1_4207_c710, d3d_create_subresource_iter, diff_d3d_create_subresource_iter, Verified)
// (0x1_4207_cb30, d3d_subresource_iter_advance, diff_d3d_subresource_iter_advance, Verified)
// (0x1_4207_cb60, d3d_subresource_iter_init_in_place, diff_d3d_subresource_iter_init_in_place, Verified)
// (0x1_4207_cbb0, d3d_check_dxgi_factory_tearing_support, diff_d3d_check_dxgi_factory_tearing_support, Verified)
//
// REGISTRY: Replacement::verified_throws(0x142078b90, "d3d::d3d_init_streaming_buffer_ctx_142078b90", crate::ns::rend::d3d_streaming_a::d3d_init_streaming_buffer_ctx as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_a::diff_d3d_init_streaming_buffer_ctx),
// REGISTRY: Replacement::verified_throws(0x142079bd0, "d3d::d3d_copy_descriptor_range_table_142079bd0", crate::ns::rend::d3d_streaming_a::d3d_copy_descriptor_range_table as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_a::diff_d3d_copy_descriptor_range_table),
// REGISTRY: Replacement::verified_throws(0x14207ab70, "d3d::d3d_sync_streaming_cvars_14207ab70", crate::ns::rend::d3d_streaming_a::d3d_sync_streaming_cvars as *const (), Throws::Caught, crate::ns::rend::d3d_streaming_a::diff_d3d_sync_streaming_cvars),
// REGISTRY: Replacement::verified_throws(0x14207ac10, "d3d::d3d_destroy_swapchain_context_14207ac10", crate::ns::rend::d3d_streaming_a::d3d_destroy_swapchain_context as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_a::diff_d3d_destroy_swapchain_context),
// REGISTRY: Replacement::verified(0x14207ade0, "d3d::d3d_nvapi_init_swapchain_hdr_14207ade0", crate::ns::rend::d3d_streaming_a::d3d_nvapi_init_swapchain_hdr as *const (), crate::ns::rend::d3d_streaming_a::diff_d3d_nvapi_init_swapchain_hdr),
// REGISTRY: Replacement::verified_throws(0x14207bdb0, "d3d::d3d_resize_swapchain_and_buffers_14207bdb0", crate::ns::rend::d3d_streaming_a::d3d_resize_swapchain_and_buffers as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_a::diff_d3d_resize_swapchain_and_buffers),
// REGISTRY: Replacement::verified_throws(0x14207bf60, "d3d::d3d_flush_gpu_and_wait_idle_14207bf60", crate::ns::rend::d3d_streaming_a::d3d_flush_gpu_and_wait_idle as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_a::diff_d3d_flush_gpu_and_wait_idle),
// REGISTRY: Replacement::verified_throws(0x14207c040, "d3d::d3d_rebuild_backbuffer_views_14207c040", crate::ns::rend::d3d_streaming_a::d3d_rebuild_backbuffer_views as *const (), Throws::Caught, crate::ns::rend::d3d_streaming_a::diff_d3d_rebuild_backbuffer_views),
// REGISTRY: Replacement::verified_throws(0x14207c710, "d3d::d3d_create_subresource_iter_14207c710", crate::ns::rend::d3d_streaming_a::d3d_create_subresource_iter as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_a::diff_d3d_create_subresource_iter),
// REGISTRY: Replacement::verified_throws(0x14207cb60, "d3d::d3d_subresource_iter_init_in_place_14207cb60", crate::ns::rend::d3d_streaming_a::d3d_subresource_iter_init_in_place as *const (), Throws::Discard, crate::ns::rend::d3d_streaming_a::diff_d3d_subresource_iter_init_in_place),

