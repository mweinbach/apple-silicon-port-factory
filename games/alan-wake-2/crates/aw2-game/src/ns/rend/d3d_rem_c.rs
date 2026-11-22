//! Decompiled `d3d` functions (`0x14208df00..0x1420a6b70`).

use crate::exe::{self, rd, wr};

#[inline]
unsafe fn read_teb() -> usize {
    exe::teb_self()
}

#[inline]
unsafe fn mimalloc_free_inline(ptr: usize) {
    let seg = (ptr.wrapping_sub(1)) & !0x1ff_ffffusize;
    let same_thread = read_teb() == rd::<usize>(seg + 0x100);
    let slice_idx = (ptr.wrapping_sub(seg)) >> 16;
    let slice_hdr = seg.wrapping_add(0x108).wrapping_add(slice_idx.wrapping_mul(0x50));
    let page = slice_hdr.wrapping_sub(rd::<u32>(slice_hdr + 0x4) as usize);
    if same_thread {
        if rd::<u8>(page + 0xe) == 0 {
            let next = rd::<usize>(page + 0x20);
            wr::<usize>(ptr, next);
            wr::<usize>(page + 0x20, ptr);
            let used = rd::<u32>(page + 0x18).wrapping_sub(1);
            wr::<u32>(page + 0x18, used);
            if used == 0 {
                exe::call_va!(0x1_4253_71a0, (page) -> ());
            }
            return;
        }
        exe::call_va!(0x1_4253_3200, (seg, page, 1u8, ptr) -> ());
    } else {
        exe::call_va!(0x1_4253_3200, (seg, page, 0u8, ptr) -> ());
    }
}

#[inline]
unsafe fn write_sl_resource_and_tag(
    this: usize,
    res: usize,
    tag_base_off: usize,
    tag_type: u32,
    lifecycle: u64,
) {
    let mut sub = rd::<usize>(res + 0x40);
    if sub == 0 {
        sub = res.wrapping_add(0x30);
    }
    let native_ptr = rd::<u64>(res + 0x70);
    let state = rd::<u32>(sub + 0x18);
    let idx = rd::<u32>(this) as usize;
    let res_slot = this.wrapping_add(0x8).wrapping_add(idx.wrapping_mul(0x70));

    wr::<u64>(res_slot, 0);
    wr::<[u8; 16]>(res_slot + 0x8, rd::<[u8; 16]>(exe::ptr::<u8>(0x1_433d_d280) as usize));
    wr::<u64>(res_slot + 0x18, 1);
    wr::<u8>(res_slot + 0x20, 0);
    wr::<u64>(res_slot + 0x28, native_ptr);
    wr::<[u8; 16]>(res_slot + 0x30, [0u8; 16]);
    wr::<u32>(res_slot + 0x40, state);
    wr::<u64>(res_slot + 0x44, 0);
    wr::<u64>(res_slot + 0x4c, 0);
    wr::<u32>(res_slot + 0x54, 0);
    wr::<u64>(res_slot + 0x58, 0);
    wr::<u64>(res_slot + 0x64, 0);

    let idx2 = rd::<u32>(this) as usize;
    let res_ptr = this.wrapping_add(0x8).wrapping_add(idx2.wrapping_mul(0x70));
    let tag_slot = this.wrapping_add(tag_base_off).wrapping_add(idx2 << 6);
    wr::<u64>(tag_slot, 0);
    wr::<[u8; 16]>(tag_slot + 0x8, rd::<[u8; 16]>(exe::ptr::<u8>(0x1_433d_d2a0) as usize));
    wr::<u64>(tag_slot + 0x18, 1);
    wr::<u64>(tag_slot + 0x20, res_ptr as u64);
    wr::<u32>(tag_slot + 0x28, tag_type);
    wr::<u64>(tag_slot + 0x2c, lifecycle);
    wr::<u64>(tag_slot + 0x34, 0);
    wr::<u32>(tag_slot + 0x3c, 0);

    wr::<u32>(this, (idx2 as u32).wrapping_add(1));
}

/// Original: `0x14208df00` (264 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14208df00(this: usize, res: usize) {
    write_sl_resource_and_tag(this, res, 0x78, 0x32, 1);
}

/// Original: `0x14208e030` (264 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14208e030(this: usize, res: usize, tag_type: u32) {
    write_sl_resource_and_tag(this, res, 0x468, tag_type, 2);
}

/// Original: `0x14208e190` (264 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14208e190(this: usize, res: usize, tag_type: u32) {
    write_sl_resource_and_tag(this, res, 0x158, tag_type, 1);
}

/// Original: `0x14208f090` (390 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_14208f090(
    slot: usize,
    name: u64,
    width: u32,
    height: u32,
    flag: u8,
) {
    let cur = rd::<usize>(slot);
    if cur != 0
        && rd::<u32>(cur + 0x8) == width
        && rd::<u32>(cur + 0xc) == height
        && rd::<u8>(cur + 0x14) == flag
        && rd::<u32>(cur + 0x10) == 1
    {
        return;
    }
    wr::<usize>(slot, 0);
    if cur != 0 {
        exe::call_va!(0x1_4204_f860, (cur) -> ());
        exe::call_va!(0x1_4244_2de0, (cur) -> ());
    }
    let mut desc = [0u8; 0x30];
    let desc_ptr = desc.as_mut_ptr() as usize;
    wr::<u32>(desc_ptr, width);
    wr::<u32>(desc_ptr + 0x4, height);
    wr::<u32>(desc_ptr + 0x8, 1);
    wr::<u32>(desc_ptr + 0xc, 1);
    wr::<u8>(desc_ptr + 0x10, flag);
    wr::<u64>(desc_ptr + 0x14, 1);
    wr::<[u8; 16]>(desc_ptr + 0x1c, [0u8; 16]);
    wr::<u32>(desc_ptr + 0x2c, 0xffff_ffff);

    let mut name_slot: u64 = name;
    let mut tmp_out: usize = 0;
    let tmp_out_ptr = (&mut tmp_out as *mut usize) as usize;
    let name_slot_ptr = (&mut name_slot as *mut u64) as usize;
    let ret = exe::call_va!(0x1_411e_2b80, (tmp_out_ptr, 0usize, desc_ptr, name_slot_ptr) -> usize);
    let new_obj = rd::<usize>(ret);
    wr::<usize>(ret, 0);
    let prev = rd::<usize>(slot);
    wr::<usize>(slot, new_obj);
    if prev != 0 {
        exe::call_va!(0x1_4204_f860, (prev) -> ());
        mimalloc_free_inline(prev);
    }
    let rem = rd::<usize>(tmp_out_ptr);
    if rem != 0 {
        exe::call_va!(0x1_4204_f860, (rem) -> ());
        exe::call_va!(0x1_4244_2de0, (rem) -> ());
    }
}

/// Original: `0x14208f220` (367 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_14208f220(slot: usize, src: usize, name: u64) {
    let cur = rd::<usize>(slot);
    if cur == 0 || rd::<u32>(cur + 0x8) != rd::<u32>(src) || rd::<u32>(cur + 0xc) != rd::<u32>(src + 0x4) {
        wr::<usize>(slot, 0);
        if cur != 0 {
            exe::call_va!(0x1_4204_f860, (cur) -> ());
            exe::call_va!(0x1_4244_2de0, (cur) -> ());
        }
        let mut desc = [0u8; 0x30];
        let desc_ptr = desc.as_mut_ptr() as usize;
        wr::<u32>(desc_ptr, rd::<u32>(src));
        wr::<u32>(desc_ptr + 0x4, rd::<u32>(src + 0x4));
        wr::<u32>(desc_ptr + 0x8, rd::<u32>(src + 0xc));
        wr::<u32>(desc_ptr + 0xc, 1);
        wr::<u8>(desc_ptr + 0x10, rd::<u8>(src + 0x18));
        wr::<u64>(desc_ptr + 0x14, 0);
        wr::<[u8; 16]>(desc_ptr + 0x1c, [0u8; 16]);
        wr::<u32>(desc_ptr + 0x2c, 0xffff_ffff);

        let mut name_slot: u64 = name;
        let mut tmp_out: usize = 0;
        let tmp_out_ptr = (&mut tmp_out as *mut usize) as usize;
        let name_slot_ptr = (&mut name_slot as *mut u64) as usize;
        let rdx_unused = rd::<u32>(src + 0xc) as usize;
        let ret = exe::call_va!(0x1_411e_2b80, (tmp_out_ptr, rdx_unused, desc_ptr, name_slot_ptr) -> usize);
        let new_obj = rd::<usize>(ret);
        wr::<usize>(ret, 0);
        let prev = rd::<usize>(slot);
        wr::<usize>(slot, new_obj);
        if prev != 0 {
            exe::call_va!(0x1_4204_f860, (prev) -> ());
            mimalloc_free_inline(prev);
        }
        let rem = rd::<usize>(tmp_out_ptr);
        if rem != 0 {
            exe::call_va!(0x1_4204_f860, (rem) -> ());
            exe::call_va!(0x1_4244_2de0, (rem) -> ());
        }
    }
    let holder = rd::<usize>(slot);
    let dst_res = rd::<usize>(holder);
    exe::call_va!(0x1_4206_70e0, (dst_res, src) -> ());
}

/// Original: `0x142091330` (309 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091330() -> usize {
    let c0 = rd::<[u8; 16]>(exe::ptr::<u8>(0x1_4342_75e0) as usize);
    let c1 = rd::<[u8; 16]>(exe::ptr::<u8>(0x1_433d_d290) as usize);
    let f0 = rd::<u32>(exe::ptr::<u32>(0x1_4342_6780) as usize);
    let base = exe::ptr::<u8>(0x1_4385_8320) as usize;

    wr::<u32>(base + 0x10, 0x0677_315f);
    wr::<[u8; 16]>(base + 0x20, c0);
    wr::<[u8; 16]>(base + 0x48, c1);
    wr::<[u8; 16]>(base + 0x70, [0u8; 16]);
    wr::<u32>(base + 0xd0, f0);
    wr::<[u8; 16]>(base + 0xf0, [0u8; 16]);
    wr::<u64>(base, 0);
    wr::<u64>(base + 0x8, 0);
    wr::<u32>(base + 0x14, 0x4492_a746);
    wr::<u32>(base + 0x18, 0x61cb_429f);
    wr::<u32>(base + 0x1c, 0xd4c3_c942);
    wr::<u32>(base + 0x30, 0);
    wr::<u64>(base + 0x38, 0);
    wr::<u64>(base + 0x40, 0);
    wr::<u64>(base + 0x58, 1);
    wr::<u32>(base + 0x60, 1);
    wr::<u64>(base + 0x68, 0);
    wr::<u64>(base + 0x80, 0);
    wr::<u64>(base + 0x88, 0);
    wr::<u64>(base + 0x90, 0);
    wr::<u64>(base + 0x98, 0);
    wr::<u64>(base + 0xa0, 0);
    wr::<u64>(base + 0xa8, 0);
    wr::<u8>(base + 0xb0, 0);
    wr::<u32>(base + 0xb4, 0);
    wr::<u8>(base + 0xb8, 0);
    wr::<u64>(base + 0xbc, 0);
    wr::<u64>(base + 0xc4, 0);
    wr::<u32>(base + 0xcc, 0);
    wr::<u64>(base + 0xd4, 0);
    wr::<u64>(base + 0xdc, 0);
    wr::<u32>(base + 0xe4, 0);
    wr::<u64>(base + 0xe8, 0);
    wr::<[u8; 16]>(base + 0x100, [0u8; 16]);
    wr::<u32>(base + 0x150, 0);
    base
}

/// Original: `0x142091970` (375 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142091970(
    width: u32,
    height: u32,
    mode: u32,
    out_w: usize,
    out_h: usize,
) -> u8 {
    let status = exe::call_iat!(0x1_4264_7340, (0u32, exe::ptr::<u8>(0x1_4385_8328) as usize) -> u32);
    if status != 0 || mode == 4 {
        return 0;
    }
    let mut settings = [0u8; 0x40];
    let mut opts = [0u8; 0x58];
    let settings_ptr = settings.as_mut_ptr() as usize;
    let opts_ptr = opts.as_mut_ptr() as usize;

    let f_const = rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize);
    wr::<u64>(settings_ptr, 0);
    wr::<u32>(settings_ptr + 0x8, 0xef1d_0957);
    wr::<u32>(settings_ptr + 0xc, 0x4df7_fd58);
    wr::<u32>(settings_ptr + 0x10, 0x698b_04b5);
    wr::<u32>(settings_ptr + 0x14, 0x766b_aad8);
    wr::<u64>(settings_ptr + 0x18, 1);

    let mapped_mode = exe::call_va!(0x1_4209_1210, (mode) -> u32);
    wr::<u64>(opts_ptr, 0);
    wr::<u32>(opts_ptr + 0x8, 0x6ac8_26e4);
    wr::<u32>(opts_ptr + 0xc, 0x4101_4c61);
    wr::<u32>(opts_ptr + 0x10, 0x8d63_2da9);
    wr::<u32>(opts_ptr + 0x14, 0xb857_1042);
    wr::<u64>(opts_ptr + 0x18, 3);
    wr::<u32>(opts_ptr + 0x20, mapped_mode);
    wr::<u32>(opts_ptr + 0x24, width);
    wr::<u32>(opts_ptr + 0x28, height);
    wr::<u32>(opts_ptr + 0x2c, 0);
    wr::<u32>(opts_ptr + 0x30, f_const);
    wr::<u32>(opts_ptr + 0x34, f_const);
    wr::<u8>(opts_ptr + 0x38, 1);

    let fn_slot = exe::ptr::<usize>(0x1_43c8_c5d8) as usize;
    let mut fn_ptr = rd::<usize>(fn_slot);
    let call_res = if fn_ptr != 0 {
        exe::call_va!(fn_ptr, (opts_ptr, settings_ptr) -> u32)
    } else {
        let name = exe::ptr::<u8>(0x1_433d_d1c8) as usize;
        let r = exe::call_iat!(0x1_4264_7308, (0u32, name, fn_slot) -> u32);
        if r == 0 {
            fn_ptr = rd::<usize>(fn_slot);
            exe::call_va!(fn_ptr, (opts_ptr, settings_ptr) -> u32)
        } else {
            r
        }
    };
    let mut ok = call_res == 0;
    let (w, h) = if mode == 5 {
        (rd::<u32>(settings_ptr + 0x34), rd::<u32>(settings_ptr + 0x38))
    } else {
        (rd::<u32>(settings_ptr + 0x20), rd::<u32>(settings_ptr + 0x24))
    };
    wr::<u32>(out_w, w);
    wr::<u32>(out_h, h);
    if rd::<u32>(out_w) == 0 || h == 0 {
        ok = false;
    }
    u8::from(ok)
}

/// Original: `0x142091f00` (226 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142091f00(
    out_budget: usize,
    out_usage: usize,
    check_hr: u8,
) -> u8 {
    let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let lock = ctx.wrapping_add(0x3bd0);
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let adapter = rd::<usize>(ctx + 0x3be0);
    if adapter != 0 {
        let vtbl = rd::<usize>(adapter);
        exe::call_va!(rd::<usize>(vtbl + 0x8), (adapter) -> ());
    }
    exe::call_iat!(0x1_4264_5598, (lock) -> ());

    let ok = if adapter != 0 {
        let mut info = [0u64; 4];
        let vtbl = rd::<usize>(adapter);
        let hr = exe::call_va!(
            rd::<usize>(vtbl + 0x70),
            (adapter, 0u32, 0u32, info.as_mut_ptr() as usize) -> u32
        );
        if check_hr != 0 && (hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2) {
            exe::call_va!(0x1_4205_d410, (adapter, hr) -> ());
        }
        if hr == 0 {
            wr::<u64>(out_budget, info[1]);
            wr::<u64>(out_usage, info[0]);
            1u8
        } else {
            wr::<u64>(out_budget, 0);
            wr::<u64>(out_usage, 0);
            0u8
        }
    } else {
        wr::<u64>(out_budget, 0);
        wr::<u64>(out_usage, 0);
        0u8
    };

    if adapter != 0 {
        let vtbl = rd::<usize>(adapter);
        exe::call_va!(rd::<usize>(vtbl + 0x10), (adapter) -> ());
    }
    ok
}

/// Original: `0x142092b00` (359 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142092b00(this: usize, expected: usize) {
    let global_lock = exe::ptr::<u8>(0x1_4385_8478) as usize;
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let mut acquired = true;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(owner_addr) != tid {
        let trylock = exe::call_iat!(0x1_4264_5788, (global_lock) -> u8);
        acquired = trylock != 0;
        if acquired {
            wr::<u32>(owner_addr, tid);
            wr::<u32>(depth_addr, rd::<u32>(depth_addr).wrapping_add(1));
        }
    } else {
        wr::<u32>(owner_addr, tid);
        wr::<u32>(depth_addr, rd::<u32>(depth_addr).wrapping_add(1));
    }

    if acquired {
        let inner = rd::<usize>(this);
        if rd::<usize>(inner + 0x68) == expected {
            let sub_lock = inner.wrapping_add(0x20);
            let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
            if rd::<u32>(sub_lock + 0x8) != tid2 {
                exe::call_iat!(0x1_4264_55a8, (sub_lock) -> ());
                wr::<u32>(sub_lock + 0x8, tid2);
            }
            wr::<u32>(sub_lock + 0xc, rd::<u32>(sub_lock + 0xc).wrapping_add(1));

            let mut page = rd::<usize>(exe::ptr::<usize>(0x1_4397_fca8) as usize);
            if page == 0 {
                page = exe::call_va!(0x1_4209_2770, (rd::<usize>(this)) -> usize);
            }
            let inner2 = rd::<usize>(this);
            wr::<usize>(inner2 + 0x68, page);
            let inner3 = rd::<usize>(this);
            let flag = rd::<u8>(inner3 + 0x1c);
            let inner4 = rd::<usize>(this);
            if flag == 0 {
                let cur_frame = rd::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize);
                if rd::<u64>(inner4 + 0x10) == cur_frame {
                    let p = rd::<usize>(inner4 + 0x68);
                    exe::call_va!(0x1_4209_2380, (inner4, p) -> ());
                }
            } else {
                let p = rd::<usize>(inner4 + 0x68);
                wr::<u64>(p + 0x30, rd::<u64>(inner4 + 0x8));
                let slot_68 = rd::<usize>(this).wrapping_add(0x68);
                let global_page = rd::<usize>(exe::ptr::<usize>(0x1_4397_fca8) as usize);
                if rd::<usize>(slot_68) != global_page {
                    exe::call_va!(0x1_411e_2d90, (0usize, slot_68) -> usize);
                }
            }

            let next_sub = rd::<u32>(sub_lock + 0xc).wrapping_sub(1);
            wr::<u32>(sub_lock + 0xc, next_sub);
            if next_sub == 0 {
                wr::<u32>(sub_lock + 0x8, 0xffff_ffff);
                exe::call_iat!(0x1_4264_5598, (sub_lock) -> ());
            }
        }

        let next_glob = rd::<u32>(depth_addr).wrapping_sub(1);
        wr::<u32>(depth_addr, next_glob);
        if next_glob == 0 {
            wr::<u32>(owner_addr, 0xffff_ffff);
            exe::call_iat!(0x1_4264_5598, (global_lock) -> ());
        }
    }
}

/// Original: `0x142092ca0` (270 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142092ca0(this: usize, out_page: usize) -> u8 {
    let rbx = rd::<usize>(this);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx + 0x28) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx.wrapping_add(0x20)) -> ());
        wr::<u32>(rbx + 0x28, tid);
    }
    wr::<u32>(rbx + 0x2c, rd::<u32>(rbx + 0x2c).wrapping_add(1));

    let rcx = rd::<usize>(this);
    let r9 = rd::<usize>(rcx + 0x68);
    wr::<usize>(out_page, r9);
    let mut ok = 0u8;
    if r9 != 0 {
        let cur_frame = rd::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize);
        if rd::<u64>(rcx + 0x10) != cur_frame || rd::<u64>(r9 + 0x30) == rd::<u64>(rcx + 0x8) {
            let req_ptr = rd::<usize>(this + 0x8);
            let used = rd::<u32>(r9 + 0x18);
            let total = (used as u64).wrapping_add(rd::<u64>(req_ptr));
            if total < 0xd0400 {
                let out_gpu_ptr = rd::<usize>(rd::<usize>(this + 0x10));
                let stride = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as i64;
                let off0 = ((used as i32 as i64).wrapping_mul(stride) as u64)
                    .wrapping_add(rd::<u64>(r9 + 0x10));
                wr::<u64>(out_gpu_ptr, off0);

                let out_cpu_ptr = rd::<usize>(rd::<usize>(this + 0x18));
                let used2 = rd::<i32>(r9 + 0x18) as i64;
                let off1 = (used2.wrapping_mul(stride) as u64).wrapping_add(rd::<u64>(r9 + 0x8));
                wr::<u64>(out_cpu_ptr, off1);

                let add_cnt = rd::<u32>(rd::<usize>(this + 0x8));
                wr::<u32>(r9 + 0x18, rd::<u32>(r9 + 0x18).wrapping_add(add_cnt));
                let g_ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
                wr::<u64>(r9 + 0x20, rd::<u64>(g_ctx + 0x40));
                let out_page_ref = rd::<usize>(rd::<usize>(this + 0x20));
                wr::<usize>(out_page_ref, r9);
                ok = 1;
            }
        }
    }

    let next_depth = rd::<u32>(rbx + 0x2c).wrapping_sub(1);
    wr::<u32>(rbx + 0x2c, next_depth);
    if next_depth == 0 {
        wr::<u32>(rbx + 0x28, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (rbx.wrapping_add(0x20)) -> ());
    }
    ok
}

/// Original: `0x142092e00` (267 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142092e00(
    _a1: usize,
    _a2: usize,
    out_heap: usize,
    out_cpu_desc: usize,
) {
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let global_lock = exe::ptr::<u8>(0x1_4385_8478) as usize;
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(owner_addr) != tid {
        exe::call_iat!(0x1_4264_55a8, (global_lock) -> ());
        wr::<u32>(owner_addr, tid);
    }
    wr::<u32>(depth_addr, rd::<u32>(depth_addr).wrapping_add(1));

    let sub_lock = rsi.wrapping_add(0x20);
    let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(sub_lock + 0x8) != tid2 {
        exe::call_iat!(0x1_4264_55a8, (sub_lock) -> ());
        wr::<u32>(sub_lock + 0x8, tid2);
    }
    wr::<u32>(sub_lock + 0xc, rd::<u32>(sub_lock + 0xc).wrapping_add(1));

    let page_slot = exe::ptr::<usize>(0x1_4397_fca8) as usize;
    let mut page = rd::<usize>(page_slot);
    if page == 0 {
        page = exe::call_va!(0x1_4209_2770, (rsi) -> usize);
        wr::<usize>(page_slot, page);
    }
    let stride = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let desc_val = rd::<u64>(page + 0x8).wrapping_add(stride.wrapping_mul(0x40400));
    wr::<u64>(out_cpu_desc, desc_val);
    wr::<u64>(out_heap, rd::<u64>(rsi + 0x2a0));

    let next_sub = rd::<u32>(sub_lock + 0xc).wrapping_sub(1);
    wr::<u32>(sub_lock + 0xc, next_sub);
    if next_sub == 0 {
        wr::<u32>(sub_lock + 0x8, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (sub_lock) -> ());
    }
    let next_glob = rd::<u32>(depth_addr).wrapping_sub(1);
    wr::<u32>(depth_addr, next_glob);
    if next_glob == 0 {
        wr::<u32>(owner_addr, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (global_lock) -> ());
    }
}

/// Original: `0x142092f40` (234 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142092f40(
    _a1: usize,
    _a2: usize,
    out_gpu: usize,
    out_cpu: usize,
) {
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let global_lock = exe::ptr::<u8>(0x1_4385_8478) as usize;
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(owner_addr) != tid {
        exe::call_iat!(0x1_4264_55a8, (global_lock) -> ());
        wr::<u32>(owner_addr, tid);
    }
    wr::<u32>(depth_addr, rd::<u32>(depth_addr).wrapping_add(1));

    let sub_lock = rsi.wrapping_add(0x20);
    let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(sub_lock + 0x8) != tid2 {
        exe::call_iat!(0x1_4264_55a8, (sub_lock) -> ());
        wr::<u32>(sub_lock + 0x8, tid2);
    }
    wr::<u32>(sub_lock + 0xc, rd::<u32>(sub_lock + 0xc).wrapping_add(1));

    let page = rd::<usize>(exe::ptr::<usize>(0x1_4397_fca8) as usize);
    wr::<u64>(out_cpu, rd::<u64>(page + 0x8));
    wr::<u64>(out_gpu, rd::<u64>(rsi + 0x280));
    let off = (rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64) << 18;
    wr::<u64>(out_cpu, rd::<u64>(out_cpu).wrapping_add(off));
    wr::<u64>(out_gpu, rd::<u64>(out_gpu).wrapping_add(off));

    let next_sub = rd::<u32>(sub_lock + 0xc).wrapping_sub(1);
    wr::<u32>(sub_lock + 0xc, next_sub);
    if next_sub == 0 {
        wr::<u32>(sub_lock + 0x8, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (sub_lock) -> ());
    }
    let next_glob = rd::<u32>(depth_addr).wrapping_sub(1);
    wr::<u32>(depth_addr, next_glob);
    if next_glob == 0 {
        wr::<u32>(owner_addr, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (global_lock) -> ());
    }
}

/// Original: `0x142093110` (221 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142093110(_this: usize) {
    let rsi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let global_lock = exe::ptr::<u8>(0x1_4385_8478) as usize;
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(owner_addr) != tid {
        exe::call_iat!(0x1_4264_55a8, (global_lock) -> ());
        wr::<u32>(owner_addr, tid);
    }
    wr::<u32>(depth_addr, rd::<u32>(depth_addr).wrapping_add(1));

    let sub_lock = rsi.wrapping_add(0x20);
    let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(sub_lock + 0x8) != tid2 {
        exe::call_iat!(0x1_4264_55a8, (sub_lock) -> ());
        wr::<u32>(sub_lock + 0x8, tid2);
    }
    wr::<u32>(sub_lock + 0xc, rd::<u32>(sub_lock + 0xc).wrapping_add(1));

    let page_slot = exe::ptr::<usize>(0x1_4397_fca8) as usize;
    if rd::<usize>(page_slot) == 0 {
        let page = exe::call_va!(0x1_4209_2770, (rsi) -> usize);
        wr::<usize>(page_slot, page);
    }

    let next_sub = rd::<u32>(sub_lock + 0xc).wrapping_sub(1);
    wr::<u32>(sub_lock + 0xc, next_sub);
    if next_sub == 0 {
        wr::<u32>(sub_lock + 0x8, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (sub_lock) -> ());
    }
    let next_glob = rd::<u32>(depth_addr).wrapping_sub(1);
    wr::<u32>(depth_addr, next_glob);
    if next_glob == 0 {
        wr::<u32>(owner_addr, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (global_lock) -> ());
    }
}

/// Original: `0x1420933f0` (267 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420933f0(this: usize) {
    let global_lock = exe::ptr::<u8>(0x1_4385_8478) as usize;
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(owner_addr) != tid {
        exe::call_iat!(0x1_4264_55a8, (global_lock) -> ());
        wr::<u32>(owner_addr, tid);
    }
    wr::<u32>(depth_addr, rd::<u32>(depth_addr).wrapping_add(1));

    if rd::<u8>(this + 0x1c) == 0 && rd::<u32>(this + 0x18) != 0 {
        let frame = rd::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize);
        let next_gen = rd::<u64>(this + 0x8).wrapping_add(1);
        wr::<u64>(this + 0x10, frame);
        let page = rd::<usize>(exe::ptr::<usize>(0x1_4397_fca8) as usize);
        wr::<u64>(this + 0x8, next_gen);
        if page != 0 {
            wr::<u64>(page + 0x30, next_gen);
        }
        let tls0 = rd::<usize>(exe::tls_pointer());
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        let tls_obj = rd::<usize>(tls0 + 0x6328);
        let tls_lock = tls_obj.wrapping_add(0x38);
        exe::call_iat!(0x1_4264_55a8, (tls_lock) -> ());
        let _ = crate::simd_ops::atomic_xchg_u8((tls_obj + 0x40) as u64, 1); // xchg byte [rsi+0x40]
        exe::call_iat!(0x1_4264_5598, (tls_lock) -> ());
        let _ = crate::simd_ops::atomic_xchg_u8((this + 0x1c) as u64, 1); // xchg byte [rbp+0x1c]
    }

    let next_glob = rd::<u32>(depth_addr).wrapping_sub(1);
    wr::<u32>(depth_addr, next_glob);
    if next_glob == 0 {
        wr::<u32>(owner_addr, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (global_lock) -> ());
    }
}

/// Original: `0x1420995c0` (253 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420995c0(idx: u32) {
    let rbx = if idx == 1 {
        exe::ptr::<u8>(0x1_4385_8500) as usize
    } else {
        exe::ptr::<u8>(0x1_4385_8490) as usize
    };
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(rbx + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (rbx) -> ());
        wr::<u32>(rbx + 0x8, tid);
    }
    wr::<u32>(rbx + 0xc, rd::<u32>(rbx + 0xc).wrapping_add(1));

    if rd::<usize>(rbx + 0x10) != 0 {
        let pair: [u64; 2] = if idx == 0 {
            let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
            [rd::<u64>(ctx + 0x40), rd::<u64>(ctx + 0x38)]
        } else {
            let mut tmp = [0u64; 2];
            let p = exe::call_va!(0x1_4207_34e0, (tmp.as_mut_ptr() as usize, 0u32) -> usize);
            [rd::<u64>(p), rd::<u64>(p + 0x8)]
        };
        let cur = rd::<usize>(rbx + 0x10);
        wr::<u64>(cur + 0x18, pair[0]);
        wr::<u64>(cur + 0x20, pair[1]);
        let count = rd::<u32>(rbx + 0x58);
        if count == rd::<u32>(rbx + 0x5c) {
            exe::call_va!(0x1_4209_84c0, (rbx.wrapping_add(0x48), count.wrapping_add(1)) -> ());
        }
        let tail = rd::<u32>(rbx + 0x54) as usize;
        let ring = rd::<usize>(rbx + 0x48);
        wr::<usize>(ring + tail * 8, rd::<usize>(rbx + 0x10));
        let next_tail = rd::<u32>(rbx + 0x54).wrapping_add(1);
        wr::<u32>(
            rbx + 0x54,
            if next_tail == rd::<u32>(rbx + 0x5c) { 0 } else { next_tail },
        );
        wr::<u32>(rbx + 0x58, rd::<u32>(rbx + 0x58).wrapping_add(1));
        wr::<usize>(rbx + 0x10, 0);
    }

    let next_obj = exe::call_va!(0x1_4209_8ff0, (rbx.wrapping_add(0x18)) -> usize);
    wr::<usize>(rbx + 0x10, next_obj);

    let next_depth = rd::<u32>(rbx + 0xc).wrapping_sub(1);
    wr::<u32>(rbx + 0xc, next_depth);
    if next_depth == 0 {
        wr::<u32>(rbx + 0x8, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (rbx) -> ());
    }
}

/// Original: `0x142099ae0` (299 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142099ae0() {
    for r14d in [0u32, 1u32] {
        let rbx = if r14d == 1 {
            exe::ptr::<u8>(0x1_4385_8518) as usize
        } else {
            exe::ptr::<u8>(0x1_4385_84a8) as usize
        };
        let ptr_slot = if r14d == 1 {
            exe::ptr::<usize>(0x1_4385_8510) as usize
        } else {
            exe::ptr::<usize>(0x1_4385_84a0) as usize
        };
        let rbp = rd::<usize>(ptr_slot);
        if rbp != 0 {
            let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
            if rd::<u32>(rbx + 0x8) != tid {
                exe::call_iat!(0x1_4264_55a8, (rbx) -> ());
                wr::<u32>(rbx + 0x8, tid);
            }
            wr::<u32>(rbx + 0xc, rd::<u32>(rbx + 0xc).wrapping_add(1));

            let count = rd::<u32>(rbx + 0x20);
            if count == rd::<u32>(rbx + 0x24) {
                exe::call_va!(0x1_4209_84c0, (rbx.wrapping_add(0x10), count.wrapping_add(1)) -> ());
            }
            let tail = rd::<u32>(rbx + 0x1c) as usize;
            let ring = rd::<usize>(rbx + 0x10);
            wr::<usize>(ring + tail * 8, rbp);
            let next_tail = rd::<u32>(rbx + 0x1c).wrapping_add(1);
            wr::<u32>(
                rbx + 0x1c,
                if next_tail == rd::<u32>(rbx + 0x24) { 0 } else { next_tail },
            );
            wr::<u32>(rbx + 0x20, rd::<u32>(rbx + 0x20).wrapping_add(1));

            let next_depth = rd::<u32>(rbx + 0xc).wrapping_sub(1);
            wr::<u32>(rbx + 0xc, next_depth);
            if next_depth == 0 {
                wr::<u32>(rbx + 0x8, 0xffff_ffff);
                exe::call_iat!(0x1_4264_5598, (rbx) -> ());
            }
        }
        wr::<usize>(ptr_slot, 0);
    }
}

/// Original: `0x14209a320` (251 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14209a320(key: i32, val: u64) {
    let map_addr = exe::ptr::<usize>(0x1_43a8_77c0) as usize;
    let head = rd::<usize>(map_addr);
    let mut cur = rd::<usize>(head + 0x8);
    let mut insert_parent = cur;
    let mut insert_left: u32 = 0;
    let mut lower_bound = head;

    while rd::<u8>(cur + 0x19) == 0 {
        insert_parent = cur;
        if rd::<i32>(cur + 0x20) < key {
            insert_left = 0;
            cur = rd::<usize>(cur + 0x10);
        } else {
            insert_left = 1;
            lower_bound = cur;
            cur = rd::<usize>(cur);
        }
    }

    if rd::<u8>(lower_bound + 0x19) != 0 || key < rd::<i32>(lower_bound + 0x20) {
        if rd::<u64>(map_addr + 0x8) == 0x0555_5555_5555_5555 {
            exe::call_va!(0x1_4020_0130, () -> ());
            return;
        }
        let node = exe::call_va!(0x1_4024_d8b0, (map_addr) -> usize);
        wr::<i32>(node + 0x20, key);
        wr::<u64>(node + 0x28, 0);
        wr::<usize>(node, head);
        wr::<usize>(node + 0x8, head);
        wr::<usize>(node + 0x10, head);
        wr::<u16>(node + 0x18, 0);
        let mut loc = [insert_parent as u64, insert_left as u64];
        lower_bound = exe::call_va!(
            0x1_4020_8500,
            (map_addr, loc.as_mut_ptr() as usize, node) -> usize
        );
    }
    wr::<u64>(lower_bound + 0x28, val);
}

#[inline]
unsafe fn rstring_cstr(str_ptr: usize) -> usize {
    let hdr = rd::<u32>(str_ptr);
    let top = hdr >> 24;
    let cap_bytes = if top < 0x80 {
        top << 3
    } else {
        (top << 8).wrapping_sub(0x7c00)
    };
    let len_bytes = (hdr & 0x00ff_ffff) << 3;
    if len_bytes > cap_bytes {
        rd::<usize>(str_ptr + 0x8)
    } else {
        str_ptr.wrapping_add(0x8)
    }
}

/// Original: `0x14209b4a0` (327 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14209b4a0(stream: usize, out: usize) -> usize {
    let vtbl0 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl0 + 0x48), (stream, out) -> ());
    let vtbl1 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl1 + 0x48), (stream, out.wrapping_add(0x4)) -> ());

    let mut str1 = [0u8; 0x90];
    let str1_ptr = str1.as_mut_ptr() as usize;
    wr::<u64>(str1_ptr, 0x1000_0010);
    wr::<u8>(str1_ptr + 0x8, 0);
    let vtbl2 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl2 + 0x20), (stream, str1_ptr) -> ());
    let cstr1 = rstring_cstr(str1_ptr);
    let v8 = exe::call_va!(0x1_4209_a520, (cstr1) -> u32);
    wr::<u32>(out + 0x8, v8);

    let mut str2 = [0u8; 0x90];
    let str2_ptr = str2.as_mut_ptr() as usize;
    wr::<u64>(str2_ptr, 0x1000_0010);
    wr::<u8>(str2_ptr + 0x8, 0);
    let vtbl3 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl3 + 0x20), (stream, str2_ptr) -> ());
    let v14 = if rd::<u32>(str2_ptr + 0x4) != 0 {
        let hash_fn = rd::<usize>(exe::ptr::<usize>(0x1_4397_fd20) as usize);
        let cstr2 = rstring_cstr(str2_ptr);
        exe::call_va!(hash_fn, (cstr2) -> u32)
    } else {
        0xffff_ffff
    };
    wr::<u32>(out + 0x14, v14);

    exe::call_va!(0x1_411e_3d90, (stream, out.wrapping_add(0x18)) -> ());
    let vtbl4 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl4 + 0x48), (stream, out.wrapping_add(0xc)) -> ());
    exe::call_va!(0x1_4240_ab10, (str2_ptr) -> ());
    exe::call_va!(0x1_4240_ab10, (str1_ptr) -> ());
    stream
}

/// Original: `0x14209b5f0` (241 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209b5f0(stream: usize, out: usize) -> usize {
    let mut str1 = [0u8; 0x90];
    let str1_ptr = str1.as_mut_ptr() as usize;
    wr::<u64>(str1_ptr, 0x1000_0010);
    wr::<u8>(str1_ptr + 0x8, 0);
    let vtbl0 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl0 + 0x20), (stream, str1_ptr) -> ());
    let hash_fn = rd::<usize>(exe::ptr::<usize>(0x1_4397_fd20) as usize);
    let cstr1 = rstring_cstr(str1_ptr);
    let h = exe::call_va!(hash_fn, (cstr1) -> u32);
    wr::<u32>(out, h);

    let vtbl1 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl1 + 0x48), (stream, out.wrapping_add(0x4)) -> ());
    let vtbl2 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl2 + 0x48), (stream, out.wrapping_add(0x8)) -> ());
    let vtbl3 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl3 + 0x48), (stream, out.wrapping_add(0xc)) -> ());
    let mut tmp: u32 = 0;
    let tmp_ptr = (&mut tmp as *mut u32) as usize;
    let vtbl4 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl4 + 0x48), (stream, tmp_ptr) -> ());
    wr::<u32>(out + 0x10, tmp);
    let vtbl5 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl5 + 0x48), (stream, out.wrapping_add(0x14)) -> ());
    let vtbl6 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl6 + 0x80), (stream, out.wrapping_add(0x18)) -> ());
    exe::call_va!(0x1_4240_ab10, (str1_ptr) -> ());
    stream
}

/// Original: `0x14209b7e0` (257 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14209b7e0(this: usize, stream: usize) -> usize {
    let vecs_base = this.wrapping_add(0x8);
    let ctor = exe::ptr::<u8>(0x1_4020_13d0) as usize;
    let dtor = exe::ptr::<u8>(0x1_401d_e400) as usize;
    exe::call_va!(0x1_4183_4fc0, (vecs_base, 0x10u32, 8u32, ctor, dtor) -> ());

    wr::<u64>(this + 0x88, 0);
    wr::<u64>(this + 0x90, 0);
    wr::<u64>(this + 0x98, 0);
    wr::<u64>(this + 0xa0, 0);
    wr::<u8>(this + 0xa8, 0);

    let mut hdr_val: u32 = 0;
    let hdr_ptr = (&mut hdr_val as *mut u32) as usize;
    let vtbl0 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl0 + 0x48), (stream, hdr_ptr) -> ());
    wr::<u32>(this, hdr_val);

    let mut cur_vec = vecs_base;
    for _ in 0..8 {
        exe::call_va!(0x1_411e_3fe0, (stream, cur_vec) -> usize);
        cur_vec = cur_vec.wrapping_add(0x10);
    }
    exe::call_va!(0x1_411e_4210, (stream, this.wrapping_add(0x88)) -> usize);
    exe::call_va!(0x1_411e_4210, (stream, this.wrapping_add(0x98)) -> usize);
    let vtbl1 = rd::<usize>(stream);
    exe::call_va!(rd::<usize>(vtbl1 + 0x80), (stream, this.wrapping_add(0xa8)) -> ());
    this
}

/// Original: `0x14209be60` (255 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209be60(out: usize, size: u64) -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);
    let ptr = if (size & size.wrapping_sub(1)) == 0 && size.wrapping_sub(0x100) <= 0x300 {
        let idx = (size.wrapping_add(7) >> 3) as usize;
        let page = rd::<usize>(heap + idx * 8 + 0x8);
        let block = rd::<usize>(page + 0x10);
        if block != 0 {
            wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
            wr::<usize>(page + 0x10, rd::<usize>(block));
            block
        } else {
            exe::call_va!(0x1_4253_7280, (heap, size, 0u32, 0u32) -> usize)
        }
    } else if size > 0x7fff_ffff_ffff_ffff {
        0
    } else {
        let mut fast_block = 0usize;
        if size.wrapping_sub(0x100) <= 0x300 {
            let idx = (size.wrapping_add(7) >> 3) as usize;
            let page = rd::<usize>(heap + idx * 8 + 0x8);
            let cand = rd::<usize>(page + 0x10);
            if cand != 0 && (cand as u8) == 0 {
                wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
                wr::<usize>(page + 0x10, rd::<usize>(cand));
                fast_block = cand;
            }
        }
        if fast_block != 0 {
            fast_block
        } else {
            exe::call_va!(0x1_4253_5070, (heap, size, 0x100u32) -> usize)
        }
    };

    if ptr == 0 {
        let err_ctx = exe::call_iat!(0x1_4264_56c8, () -> usize);
        exe::call_iat!(0x1_4264_53a0, (err_ctx, 0xdead_d00du32) -> ());
        wr::<usize>(out, 0);
        out
    } else {
        wr::<usize>(out, ptr);
        out
    }
}

/// Original: `0x14209bf70` (263 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209bf70(
    this: usize,
    handle_ptr: usize,
    a3: u64,
    a4: u32,
    a5: u32,
    name_ptr: usize,
    a7: u32,
) -> usize {
    let hdr = (rd::<u32>(this) & 0xff00_0008) | 0x8;
    wr::<u32>(this, hdr);
    wr::<u8>(this + 0x3, 0x8);
    wr::<u32>(this + 0x4, 0);
    wr::<u8>(this + 0x8, 0);
    exe::call_va!(0x1_4240_aa00, (this, name_ptr) -> usize);

    let h = rd::<u64>(handle_ptr);
    wr::<u64>(handle_ptr, 0);
    wr::<u64>(this + 0x48, h);
    wr::<u64>(this + 0x50, a3);
    wr::<u32>(this + 0x58, a4);
    wr::<u32>(this + 0x5c, a5);
    let v60 = exe::call_va!(0x1_424b_16a0, (h, a3) -> u64);
    wr::<u64>(this + 0x60, v60);
    wr::<u32>(this + 0x68, a7);

    let rem = rd::<usize>(handle_ptr);
    if rem != 0 {
        mimalloc_free_inline(rem);
    }
    this
}

/// Original: `0x14209d160` (274 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209d160(this: usize, other: usize) -> usize {
    exe::call_va!(0x1_4209_d670, (this) -> ());
    let saved = rd::<[u8; 0x50]>(other);
    wr::<[u8; 0x50]>(other, [0u8; 0x50]);
    wr::<[u8; 0x50]>(this, saved);

    let v50 = rd::<u32>(other + 0x50);
    wr::<u32>(other + 0x50, 0);
    wr::<u32>(this + 0x50, v50);

    let mut tmp_ptr: usize = 0;
    let tmp_addr = (&mut tmp_ptr as *mut usize) as usize;
    let rax = exe::call_va!(0x1_411e_8020, (tmp_addr, other.wrapping_add(0x58)) -> usize);
    exe::call_va!(0x1_411e_8110, (this.wrapping_add(0x58), rax) -> usize);

    let rbx = tmp_ptr;
    if rbx != 0 {
        if rd::<u8>(rbx + 0x18a) != 0 {
            exe::call_va!(0x1_4207_5a20, (rbx) -> ());
        }
        if rd::<u8>(rbx + 0x18b) != 0 {
            exe::call_va!(0x1_4207_5a20, (rbx.wrapping_add(0x50)) -> ());
        }
        if rd::<u8>(rbx + 0x18c) != 0 {
            exe::call_va!(0x1_4207_5a20, (rbx.wrapping_add(0xa0)) -> ());
        }
        exe::call_va!(0x1_4244_2de0, (rbx) -> ());
    }
    this
}

/// Original: `0x14209d420` (288 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209d420(this: usize) {
    if rd::<u8>(this + 0x18a) != 0 {
        exe::call_va!(0x1_4207_5a20, (this) -> ());
    }
    let mut rbx = this.wrapping_add(0x68);
    for i in 0..2usize {
        if rd::<u8>(this + 0x18b + i) != 0 {
            let p = rd::<usize>(rbx + 0x28);
            if p != 0 {
                exe::call_va!(0x1_4209_a1e0, (p, 0u32) -> ());
                let g = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
                exe::call_va!(0x1_4209_6c10, (rd::<usize>(g), rd::<u64>(rbx)) -> ());
                exe::call_va!(0x1_4209_6a90, (rd::<usize>(g), rd::<u64>(rbx + 0x8)) -> ());
                exe::call_va!(0x1_4209_6a90, (rd::<usize>(g), rd::<u64>(rbx + 0x10)) -> ());
                exe::call_va!(0x1_4209_6c10, (rd::<usize>(g), rd::<u64>(rbx + 0x18)) -> ());
                exe::call_va!(0x1_4209_6c10, (rd::<usize>(g), rd::<u64>(rbx + 0x20)) -> ());
            }
            wr::<[u8; 0x50]>(rbx.wrapping_sub(0x18), [0u8; 0x50]);
        }
        rbx = rbx.wrapping_add(0x50);
    }
}

/// Original: `0x14209db20` (343 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209db20(this: usize) -> usize {
    let base = rd::<usize>(this + 0x40);
    let mut cur = base;
    let len = rd::<u32>(this + 0x48) as usize;
    let byte_len = len.wrapping_mul(24);
    let end = base.wrapping_add(byte_len);
    while cur != end {
        let free_head = rd::<usize>(cur + 0x10);
        if free_head != 0 {
            wr::<usize>(cur + 0x10, rd::<usize>(free_head + 0x20));
            return free_head;
        }
        cur = cur.wrapping_add(0x18);
    }

    let cap = rd::<u32>(this + 0x4c) as usize;
    let cap_bytes = cap.wrapping_mul(24);
    if end == base.wrapping_add(cap_bytes) {
        let cur_cnt = ((byte_len as i64) / 24) as u64;
        let min_cap = cur_cnt.wrapping_add(1);
        let old_cnt = ((cap_bytes as i64) / 24) as u64;
        let grow = old_cnt.wrapping_add(old_cnt >> 1);
        let new_cap = if grow < min_cap { min_cap } else { grow };
        exe::call_va!(0x1_411e_8490, (this.wrapping_add(0x40), new_cap) -> ());
    }

    let idx = rd::<u32>(this + 0x48) as usize;
    let arr = rd::<usize>(this + 0x40);
    let slot = arr.wrapping_add(idx.wrapping_mul(24));
    wr::<u64>(slot, 0);
    wr::<u64>(slot + 0x8, 0);
    wr::<u64>(slot + 0x10, 0);
    wr::<u32>(this + 0x48, (idx as u32).wrapping_add(1));

    let new_len = rd::<u32>(this + 0x48) as usize;
    let arr2 = rd::<usize>(this + 0x40);
    let slab = arr2.wrapping_add((new_len - 1).wrapping_mul(24));
    let slab_cnt = rd::<u64>(this + 0x28);
    exe::call_va!(0x1_411e_8360, (slab, slab_cnt, 0usize, slab) -> ());
    let slab_buf = rd::<usize>(slab);
    wr::<usize>(slab + 0x10, slab_buf.wrapping_add(0x28));

    let mut i = 1usize;
    if (rd::<u32>(slab + 0x8) as usize).wrapping_sub(1) > 1 {
        let mut off = 0x28usize;
        loop {
            let buf = rd::<usize>(slab);
            let next_ptr = buf.wrapping_add(0x28).wrapping_add(off);
            wr::<usize>(buf.wrapping_add(off).wrapping_add(0x20), next_ptr);
            i += 1;
            off += 0x28;
            let lim = (rd::<u32>(slab + 0x8) as usize).wrapping_sub(1);
            if i >= lim {
                break;
            }
        }
    }
    rd::<usize>(slab)
}

/// Original: `0x14209dc80` (334 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209dc80(this: usize, node: usize, order: u8) {
    let size = rd::<u16>(node + 0xa);
    let offset = rd::<u16>(node + 0x8);
    let is_right_buddy = (size & offset) != 0;
    let pair_base = if is_right_buddy { node.wrapping_sub(0x10) } else { node };
    let buddy = if is_right_buddy {
        node.wrapping_sub(0x10)
    } else {
        node.wrapping_add(0x10)
    };
    let ord = order as usize;
    let head_slot = rd::<usize>(this + 0x30).wrapping_add(ord * 8);

    if rd::<u8>(buddy + 0xe) != 0 {
        wr::<u8>(node + 0xe, 0);
        let mut prev_slot = head_slot;
        let mut cur = rd::<usize>(head_slot);
        if cur != 0 {
            let node_page = rd::<u16>(node + 0xc);
            loop {
                let cur_page = rd::<u16>(cur + 0xc);
                if node_page < cur_page {
                    break;
                }
                if node_page == cur_page && rd::<u16>(node + 0x8) < rd::<u16>(cur + 0x8) {
                    break;
                }
                let next = rd::<usize>(cur);
                prev_slot = cur;
                cur = next;
                if cur == 0 {
                    break;
                }
            }
        }
        wr::<usize>(node, rd::<usize>(prev_slot));
        wr::<usize>(prev_slot, node);
        return;
    }

    let mut prev_slot = head_slot;
    let mut cur = rd::<usize>(head_slot);
    while cur != 0 {
        if cur == buddy {
            wr::<usize>(prev_slot, rd::<usize>(buddy));
            break;
        }
        prev_slot = cur;
        cur = rd::<usize>(cur);
    }
    wr::<u8>(node + 0xe, 0);
    let parent = rd::<usize>(pair_base + 0x20);
    if parent != 0 {
        wr::<u8>(parent + 0xe, 1);
        exe::call_va!(0x1_4209_dc80, (this, parent, order.wrapping_add(1)) -> ());
    } else {
        let page_id = rd::<u16>(node + 0xc);
        let cb_ctx = rd::<usize>(this + 0x10);
        let cb_fn = rd::<usize>(this + 0x18);
        exe::call_va!(cb_fn, (cb_ctx, page_id) -> ());
    }

    let slab_count = rd::<u32>(this + 0x48) as usize;
    let mut slab = rd::<usize>(this + 0x40);
    let slab_end = slab.wrapping_add(slab_count.wrapping_mul(24));
    while slab != slab_end {
        let buf_start = rd::<usize>(slab);
        if pair_base >= buf_start {
            let buf_cap = rd::<u32>(slab + 0x8) as usize;
            let buf_end = buf_start.wrapping_add(buf_cap.wrapping_mul(0x28));
            if pair_base < buf_end {
                wr::<usize>(pair_base + 0x20, rd::<usize>(slab + 0x10));
                wr::<usize>(slab + 0x10, pair_base);
                break;
            }
        }
        slab = slab.wrapping_add(0x18);
    }
}

/// Original: `0x14209dde0` (334 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209dde0(this: usize, order: u8) -> usize {
    let free_lists = rd::<usize>(this + 0x30);
    let ord = order as usize;
    let head = rd::<usize>(free_lists + ord * 8);
    if head != 0 {
        wr::<usize>(free_lists + ord * 8, rd::<usize>(head));
        wr::<u8>(head + 0xe, 1);
        return head;
    }

    let block_size: u16 = 1u16 << (order & 15);
    let max_units = rd::<u16>(this + 0x20);
    if block_size < max_units {
        let parent = exe::call_va!(0x1_4209_dde0, (this, order.wrapping_add(1)) -> usize);
        if parent == 0 {
            return 0;
        }
        let pair = exe::call_va!(0x1_4209_db20, (this) -> usize);
        wr::<u8>(parent + 0xe, 2);
        wr::<usize>(parent, pair);
        wr::<usize>(pair + 0x20, parent);
        let right = pair.wrapping_add(0x10);
        let page_id = rd::<u16>(parent + 0xc);
        let base_off = rd::<u16>(parent + 0x8);
        wr::<u16>(pair + 0xc, page_id);
        wr::<u16>(pair + 0x8, base_off);
        wr::<u16>(pair + 0xa, block_size);
        wr::<u8>(pair + 0xe, 1);
        wr::<u16>(pair + 0x1c, page_id);
        wr::<u16>(pair + 0x18, block_size.wrapping_add(base_off));
        wr::<u16>(pair + 0x1a, block_size);
        wr::<u8>(pair + 0x1e, 1);
        exe::call_va!(
            0x1_4209_e0c0,
            (rd::<usize>(this + 0x30).wrapping_add(ord * 8), right) -> ()
        );
        pair
    } else {
        let alloc_cb = rd::<usize>(this + 0x8);
        let alloc_ctx = rd::<usize>(this);
        let req_units = (max_units as u32).wrapping_mul(2);
        let page_id = exe::call_va!(alloc_cb, (alloc_ctx, req_units) -> u16);
        if page_id == 0xffff {
            return 0;
        }
        let pair = exe::call_va!(0x1_4209_db20, (this) -> usize);
        wr::<usize>(pair + 0x20, 0);
        let right = pair.wrapping_add(0x10);
        wr::<u16>(pair + 0xc, page_id);
        wr::<u16>(pair + 0x8, 0);
        wr::<u16>(pair + 0xa, block_size);
        wr::<u8>(pair + 0xe, 1);
        wr::<u16>(pair + 0x1c, page_id);
        wr::<u16>(pair + 0x18, block_size);
        wr::<u16>(pair + 0x1a, block_size);
        wr::<u8>(pair + 0x1e, 1);
        exe::call_va!(
            0x1_4209_e0c0,
            (rd::<usize>(this + 0x30).wrapping_add(ord * 8), right) -> ()
        );
        pair
    }
}

/// Original: `0x14209df40` (255 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209df40(
    cb_pair: usize,
    pair_base: usize,
    new_page: u16,
    base_off: u16,
    parent_size: u16,
    out_count: usize,
    out_bytes: usize,
) {
    let half_size = parent_size >> 1;
    let mut cur_off = base_off;
    let mut rbx = pair_base.wrapping_add(0xa);
    for _ in 0..2 {
        let state = rd::<u8>(rbx + 0x4);
        if state == 1 {
            let user_data = rd::<u64>(rbx - 0xa);
            let node_size = rd::<u16>(rbx);
            wr::<u64>(out_count, rd::<u64>(out_count).wrapping_add(1));
            wr::<u64>(out_bytes, rd::<u64>(out_bytes).wrapping_add(node_size as u64));
            let old_off = rd::<u16>(rbx - 0x2);
            let old_page = rd::<u16>(rbx + 0x2);
            let cb_ctx = rd::<usize>(cb_pair);
            let cb_fn = rd::<usize>(cb_pair + 0x8);
            exe::call_va!(
                cb_fn,
                (cb_ctx, old_page, old_off, node_size, new_page, cur_off, user_data) -> ()
            );
        } else if state == 2 {
            let mut sub_cb = [rd::<u64>(cb_pair), rd::<u64>(cb_pair + 0x8)];
            let child_pair = rd::<usize>(rbx - 0xa);
            let node_size = rd::<u16>(rbx);
            exe::call_va!(
                0x1_4209_df40,
                (
                    sub_cb.as_mut_ptr() as usize,
                    child_pair,
                    new_page,
                    cur_off,
                    node_size,
                    out_count,
                    out_bytes
                ) -> ()
            );
        }
        wr::<u16>(rbx - 0x2, cur_off);
        cur_off = cur_off.wrapping_add(half_size);
        wr::<u16>(rbx + 0x2, new_page);
        rbx = rbx.wrapping_add(0x10);
    }
}

/// Original: `0x14209f930` (244 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14209f930(this: usize) -> u64 {
    let count = rd::<u32>(this + 0xc8) as usize;
    let mut cur_elem = rd::<usize>(this + 0xc0);
    let end_elem = cur_elem.wrapping_add(count.wrapping_mul(0x2b0));
    let mut tex_sum: u64 = 0;

    let mut group = this.wrapping_add(0x1f0);
    let group_end = this.wrapping_add(0x250);
    while group != group_end {
        tex_sum = 0;
        let mut slot = group;
        let slot_end = group.wrapping_add(0x30);
        while slot != slot_end {
            let tex = rd::<usize>(slot);
            if tex != 0 {
                let fmt = rd::<u8>(tex + 0x18) as u32;
                let h = rd::<u32>(tex + 0x4);
                let w = rd::<u32>(tex);
                let bytes = exe::call_va!(0x1_4204_e7f0, (w, h, fmt) -> i32);
                tex_sum = tex_sum.wrapping_add(bytes as i64 as u64);
            }
            slot = slot.wrapping_add(0x10);
        }
        group = group.wrapping_add(0x30);
    }

    let mut elem_sum: u64 = 0;
    while cur_elem != end_elem {
        let v = rd::<u64>(cur_elem + 0xd0)
            .wrapping_add(rd::<u64>(cur_elem + 0x70))
            .wrapping_add(rd::<u64>(cur_elem + 0x10));
        cur_elem = cur_elem.wrapping_add(0x2b0);
        elem_sum = elem_sum.wrapping_add(v);
    }

    rd::<u64>(this + 0x340)
        .wrapping_add(rd::<u64>(this + 0xe0))
        .wrapping_add(rd::<u64>(this + 0x70))
        .wrapping_add(rd::<u64>(this + 0x10))
        .wrapping_add(elem_sum)
        .wrapping_add(tex_sum)
}

/// Original: `0x1420a0310` (264 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420a0310(this: usize) {
    exe::call_va!(0x1_4207_5970, (this.wrapping_add(0x390)) -> ());
    exe::call_va!(0x1_4209_d670, (this.wrapping_add(0x330)) -> ());
    exe::call_va!(0x1_411e_7fc0, (this.wrapping_add(0x388)) -> ());
    exe::call_va!(0x1_401d_e830, (this.wrapping_add(0x320)) -> ());
    exe::call_va!(0x1_401d_e830, (this.wrapping_add(0x310)) -> ());
    exe::call_va!(0x1_401d_e830, (this.wrapping_add(0x300)) -> ());

    let dtor_vec = exe::ptr::<u8>(0x1_401d_e400) as usize;
    exe::call_va!(
        0x1_4183_4ee8,
        (this.wrapping_add(0x2b0), 0x10u32, 5u32, dtor_vec) -> ()
    );
    let dtor_1f0 = exe::ptr::<u8>(0x1_411e_8b90) as usize;
    exe::call_va!(
        0x1_4183_4ee8,
        (this.wrapping_add(0x1f0), 0x30u32, 2u32, dtor_1f0) -> ()
    );
    let dtor_130 = exe::ptr::<u8>(0x1_411e_8b70) as usize;
    exe::call_va!(
        0x1_4183_4ee8,
        (this.wrapping_add(0x130), 0x40u32, 3u32, dtor_130) -> ()
    );

    exe::call_va!(0x1_4209_d670, (this.wrapping_add(0xd0)) -> ());
    exe::call_va!(0x1_411e_7fc0, (this.wrapping_add(0x128)) -> ());
    exe::call_va!(0x1_411e_8ed0, (this.wrapping_add(0xc0)) -> ());
    exe::call_va!(0x1_4209_d670, (this.wrapping_add(0x60)) -> ());
    exe::call_va!(0x1_411e_7fc0, (this.wrapping_add(0xb8)) -> ());
    exe::call_va!(0x1_4209_d670, (this) -> ());
    exe::call_va!(0x1_411e_7fc0, (this.wrapping_add(0x58)) -> ());
}

/// Original: `0x1420a0490` (266 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420a0490(
    this: usize,
    out_mapped: usize,
    out_size: usize,
) -> u8 {
    let count = rd::<u32>(this + 0x20) as usize;
    let mut best_val: u64 = 0;
    let mut has_best = false;
    if count != 0 {
        let mut off = 0usize;
        for _ in 0..count {
            let base = rd::<usize>(this + 0x18);
            let fence = rd::<usize>(base + off + 0x8);
            let target_fence = rd::<u64>(base + off);
            let mut still_pending = false;
            if fence != 0 {
                let vtbl = rd::<usize>(fence);
                let completed = exe::call_va!(rd::<usize>(vtbl + 0x40), (fence) -> u64);
                if completed == u64::MAX {
                    exe::call_va!(0x1_4205_d410, (fence, 0x887a_0005u32) -> ());
                }
                still_pending = completed < target_fence;
            }
            if !still_pending {
                let entry = rd::<usize>(this + 0x18).wrapping_add(off);
                let v = rd::<u64>(entry + 0x20);
                if v > best_val || !has_best {
                    has_best = true;
                    wr::<usize>(this, entry);
                    best_val = v;
                }
            }
            off = off.wrapping_add(0x28);
        }
    }

    let selected = rd::<usize>(this);
    if selected != 0 {
        let buf_obj = rd::<usize>(selected + 0x10);
        let sz = rd::<u64>(buf_obj + 0x10);
        let mapped = exe::call_va!(0x1_4209_c560, (buf_obj, out_mapped, sz, 0u32) -> u64);
        wr::<u64>(out_mapped, mapped);
        let sel2 = rd::<usize>(this);
        let buf2 = rd::<usize>(sel2 + 0x10);
        wr::<u64>(out_size, rd::<u64>(buf2 + 0x10));
        1
    } else {
        wr::<u64>(out_mapped, 0);
        wr::<u64>(out_size, 0);
        0
    }
}

#[inline]
unsafe fn sorted_u16_spans_intersect(span0: usize, span1: usize) -> bool {
    let len0 = rd::<u64>(span0) as usize;
    let ptr0 = rd::<usize>(span0 + 0x8);
    let len1 = rd::<u64>(span1) as usize;
    let ptr1 = rd::<usize>(span1 + 0x8);
    if len0 == 0 {
        return false;
    }
    let mut i0 = 0usize;
    let mut i1 = 0usize;
    while i1 != len1 {
        let v0 = rd::<u16>(ptr0 + i0 * 2);
        let v1 = rd::<u16>(ptr1 + i1 * 2);
        if v0 < v1 {
            i0 += 1;
        } else if v0 <= v1 {
            return true;
        } else {
            i1 += 1;
        }
        if i0 == len0 {
            break;
        }
    }
    false
}

/// Original: `0x1420a14d0` (299 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420a14d0(span_a: usize, span_b: usize, node: usize) -> u8 {
    if sorted_u16_spans_intersect(span_a, node.wrapping_add(0x18))
        || sorted_u16_spans_intersect(span_b, node.wrapping_add(0x8))
        || sorted_u16_spans_intersect(span_b, node.wrapping_add(0x18))
    {
        1
    } else {
        0
    }
}

/// Original: `0x1420a28e0` (319 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420a28e0(this: usize) {
    let r11 = rd::<usize>(this + 0x10);
    let mut cur = rd::<usize>(r11 + 0xf0);
    let count = rd::<u32>(r11 + 0xf8) as usize;
    let end = cur.wrapping_add(count.wrapping_mul(0x48));
    while cur != end {
        if rd::<u8>(cur + 0x2) != 0
            || sorted_u16_spans_intersect(cur.wrapping_add(0x8), r11.wrapping_add(0xc0))
        {
            wr::<u8>(r11 + 0x113, rd::<u8>(r11 + 0x113) | 1);
            break;
        }
        cur = cur.wrapping_add(0x48);
    }

    let has_60_or_58 = u8::from(rd::<u16>(r11 + 0x60) != 0xffff || rd::<u8>(r11 + 0x58) != 0);
    let rdx_28 = rd::<u64>(r11 + 0x28);
    wr::<u8>(r11 + 0x117, has_60_or_58);
    wr::<u8>(r11 + 0x118, u8::from(rdx_28 != 0));
    let f114 = u8::from(rd::<u8>(r11 + 0x113) != 0 || has_60_or_58 != 0 || rdx_28 != 0);
    wr::<u8>(r11 + 0x114, f114);
    let base_48 = rd::<u64>(r11 + 0x48);
    let len_50 = rd::<u32>(r11 + 0x50) as u64;
    let end_48 = base_48.wrapping_add(len_50.wrapping_mul(2));
    wr::<u8>(r11 + 0x116, u8::from(base_48 != end_48));
}

/// Original: `0x1420a4240` (396 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420a4240(this: usize, other: usize) -> usize {
    wr::<u32>(this, rd::<u32>(other));
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    let head = exe::call_va!(0x1_4241_c0a0, (0x18u32, 0x8u32) -> usize);
    wr::<usize>(head, head);
    wr::<usize>(head + 0x8, head);
    wr::<usize>(this + 0x8, head);

    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    wr::<u64>(this + 0x28, 0);
    let rsi = rd::<usize>(this + 0x8);
    let buckets = exe::call_va!(0x1_4241_c0a0, (0x80u32, 0x8u32) -> usize);
    let old_b = rd::<usize>(this + 0x18);
    let old_e = rd::<usize>(this + 0x28);
    if old_e.wrapping_sub(old_b) >= 8 && old_b != 0 {
        exe::call_va!(0x1_4241_c190, (old_b) -> ());
    }
    wr::<usize>(this + 0x18, buckets);
    let buckets_end = buckets.wrapping_add(0x80);
    wr::<usize>(this + 0x20, buckets_end);
    wr::<usize>(this + 0x28, buckets_end);
    let mut cur = buckets;
    while cur != buckets_end {
        wr::<usize>(cur, rsi);
        cur = cur.wrapping_add(8);
    }

    for off in [0x8usize, 0x10, 0x18, 0x20, 0x28] {
        let a = rd::<u64>(this + off);
        let b = rd::<u64>(other + off);
        wr::<u64>(this + off, b);
        wr::<u64>(other + off, a);
    }

    let v30 = rd::<u64>(other + 0x30);
    wr::<u64>(other + 0x30, 7);
    wr::<u64>(this + 0x30, v30);
    let v38 = rd::<u64>(other + 0x38);
    wr::<u64>(other + 0x38, 8);
    wr::<u64>(this + 0x38, v38);

    wr::<u64>(this + 0x40, rd::<u64>(other + 0x40));
    wr::<u32>(this + 0x48, rd::<u32>(other + 0x48));
    wr::<u32>(this + 0x4c, rd::<u32>(other + 0x4c));
    wr::<u64>(other + 0x40, 0);
    wr::<u64>(other + 0x48, 0);

    wr::<u64>(this + 0x50, rd::<u64>(other + 0x50));
    wr::<u32>(this + 0x58, rd::<u32>(other + 0x58));
    wr::<u32>(this + 0x5c, rd::<u32>(other + 0x5c));
    wr::<u64>(other + 0x50, 0);
    wr::<u64>(other + 0x58, 0);
    this
}

/// Original: `0x1420a4490` (345 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_1420a4490(
    this: usize,
    vec: usize,
    ctx: usize,
    a4: u64,
) -> usize {
    let count0 = rd::<u32>(vec + 0x8);
    exe::call_va!(0x1_420a_5150, (this, count0, ctx, a4, 0u32) -> ());
    let mut state = [vec as u64, this as u64, ctx as u64, a4];

    let mut rdi = rd::<usize>(vec);
    let mut end = rdi.wrapping_add((rd::<u32>(vec + 0x8) as usize) << 6);
    let mut rsi = this;
    if rdi != end {
        let sub_arr = rd::<usize>(ctx + 0x10);
        loop {
            rsi = rdi;
            let sub_idx = rd::<u16>(rdi + 0x22) as usize;
            let item = sub_arr.wrapping_add(sub_idx.wrapping_mul(0x30));
            if rd::<u8>(item + 0x20) != 0 && rd::<u8>(item + 0x21) == 0 {
                break;
            }
            rdi = rdi.wrapping_add(0x40);
            if rdi == end {
                break;
            }
        }
    }

    while rdi != end {
        let low16: u16 = if rd::<u8>(rsi + 0x38) != 0 {
            rd::<u16>(rsi + 0x36)
        } else {
            0
        };
        let high16: u16 = if rd::<u8>(rsi + 0x34) != 0 {
            rd::<u16>(rsi + 0x32)
        } else {
            rd::<u16>(vec + 0x8)
        };
        let packed: u32 = (low16 as u32) | ((high16 as u32) << 16);
        let node_id = rd::<u16>(rsi + 0x20);
        exe::call_va!(0x1_420a_4b10, (state.as_mut_ptr() as usize, node_id, packed) -> ());

        rdi = rdi.wrapping_add(0x40);
        end = rd::<usize>(vec).wrapping_add((rd::<u32>(vec + 0x8) as usize) << 6);
        if rdi == end {
            break;
        }
        let sub_arr = rd::<usize>(ctx + 0x10);
        loop {
            rsi = rdi;
            let sub_idx = rd::<u16>(rdi + 0x22) as usize;
            let item = sub_arr.wrapping_add(sub_idx.wrapping_mul(0x30));
            if rd::<u8>(item + 0x20) != 0 && rd::<u8>(item + 0x21) == 0 {
                break;
            }
            rdi = rdi.wrapping_add(0x40);
            if rdi == end {
                break;
            }
        }
    }
    this
}

/// Original: `0x1420a4d90` (310 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420a4d90(this: usize, id: u16) {
    let ctx = rd::<usize>(this);
    let table = rd::<usize>(ctx + 0x50);
    let packed = rd::<u32>(table + (id as usize) * 4);
    let start = packed as u16;
    let end_idx = (packed >> 16) as u16;
    if start == end_idx {
        return;
    }

    let count = end_idx.wrapping_sub(start) as usize;
    let arr = rd::<usize>(ctx + 0x40);
    let mut cur = arr.wrapping_add((start as usize) * 2);
    let end = cur.wrapping_add(count * 2);
    while cur != end {
        let target_id = rd::<u16>(cur) as usize;
        let visited = rd::<usize>(this + 0x8);
        if rd::<u8>(visited + target_id) == 0 {
            let len = rd::<u32>(this + 0x20) as usize;
            let cap = rd::<u32>(this + 0x24) as usize;
            let mut buf = rd::<usize>(this + 0x18);
            if len * 4 == cap * 4 {
                let min_cap = len.wrapping_add(1);
                let grow = cap.wrapping_add(cap >> 1);
                let new_cap = if grow < min_cap { min_cap } else { grow };
                let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 4, 2u32) -> usize);
                let cur_bytes = (rd::<u32>(this + 0x20) as usize) << 2;
                let old_buf = rd::<usize>(this + 0x18);
                exe::call_va!(0x1_4183_acc8, (new_buf, old_buf, cur_bytes) -> usize);
                let old_buf2 = rd::<usize>(this + 0x18);
                if old_buf2 != 0 {
                    exe::call_va!(0x1_4241_c190, (old_buf2) -> ());
                }
                wr::<usize>(this + 0x18, new_buf);
                wr::<u32>(this + 0x20, len as u32);
                wr::<u32>(this + 0x24, new_cap as u32);
                buf = new_buf;
            }
            wr::<u16>(buf + len * 4, target_id as u16);
            wr::<u8>(buf + len * 4 + 2, 0);
            wr::<u32>(this + 0x20, rd::<u32>(this + 0x20).wrapping_add(1));
            let visited2 = rd::<usize>(this + 0x8);
            wr::<u8>(visited2 + target_id, 1);
        }
        cur = cur.wrapping_add(2);
    }
}

/// Original: `0x1420a4ee0` (383 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420a4ee0(this: usize, key: u16) {
    let ctx = rd::<usize>(this);
    let fnv_offset: u64 = 0xcbf2_9ce4_8422_2325;
    let fnv_prime: u64 = 0x0000_0100_0000_01b3;
    let b0 = (key & 0xff) as u64;
    let b1 = (key >> 8) as u64;
    let h = ((b0 ^ fnv_offset).wrapping_mul(fnv_prime) ^ b1).wrapping_mul(fnv_prime);

    let mask = rd::<u64>(ctx + 0x30);
    let bucket_idx = ((h & mask) as usize) * 2;
    let buckets = rd::<usize>(ctx + 0x18);
    let mut rbx = rd::<usize>(buckets + bucket_idx * 8);
    let head = rd::<usize>(ctx + 0x8);
    if rbx == head {
        return;
    }
    let bucket_last = rd::<usize>(buckets + bucket_idx * 8 + 0x8);
    while rd::<u16>(rbx + 0x10) != key {
        if rbx == bucket_last {
            return;
        }
        rbx = rd::<usize>(rbx);
    }
    let mut rsi = rd::<usize>(rbx);
    if rbx != bucket_last {
        loop {
            let cur = rsi;
            if rd::<u16>(rsi + 0x10) != key {
                break;
            }
            rsi = rd::<usize>(rsi);
            if cur == bucket_last {
                break;
            }
        }
    }
    if rbx == rsi {
        return;
    }

    while rbx != rsi {
        let target_id = rd::<u16>(rbx + 0x12) as usize;
        let visited = rd::<usize>(this + 0x8);
        if rd::<u8>(visited + target_id) == 0 {
            let len = rd::<u32>(this + 0x20) as usize;
            let cap = rd::<u32>(this + 0x24) as usize;
            let mut buf = rd::<usize>(this + 0x18);
            if len * 4 == cap * 4 {
                let min_cap = len.wrapping_add(1);
                let grow = cap.wrapping_add(cap >> 1);
                let new_cap = if grow < min_cap { min_cap } else { grow };
                let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 4, 2u32) -> usize);
                let cur_bytes = (rd::<u32>(this + 0x20) as usize) << 2;
                let old_buf = rd::<usize>(this + 0x18);
                exe::call_va!(0x1_4183_acc8, (new_buf, old_buf, cur_bytes) -> usize);
                let old_buf2 = rd::<usize>(this + 0x18);
                if old_buf2 != 0 {
                    exe::call_va!(0x1_4241_c190, (old_buf2) -> ());
                }
                wr::<usize>(this + 0x18, new_buf);
                wr::<u32>(this + 0x20, len as u32);
                wr::<u32>(this + 0x24, new_cap as u32);
                buf = new_buf;
            }
            wr::<u16>(buf + len * 4, target_id as u16);
            wr::<u8>(buf + len * 4 + 2, 1);
            wr::<u32>(this + 0x20, rd::<u32>(this + 0x20).wrapping_add(1));
            let visited2 = rd::<usize>(this + 0x8);
            wr::<u8>(visited2 + target_id, 1);
        }
        rbx = rd::<usize>(rbx);
    }
}

/// Original: `0x1420a58b0` (286 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420a58b0(
    this: usize,
    range_pair: usize,
    a3: u16,
    a4: u16,
    tag: u16,
    a6: u32,
) {
    let rbx = rd::<usize>(this + 0x8);
    let len = rd::<u32>(rbx + 0x18) as usize;
    let cap = rd::<u32>(rbx + 0x1c) as usize;
    let mut buf = rd::<usize>(rbx + 0x10);
    if len * 8 == cap * 8 {
        let min_cap = len.wrapping_add(1);
        let grow = cap.wrapping_add(cap >> 1);
        let new_cap = if grow < min_cap { min_cap } else { grow };
        let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 8, 4u32) -> usize);
        let cur_bytes = (rd::<u32>(rbx + 0x18) as usize) << 3;
        let old_buf = rd::<usize>(rbx + 0x10);
        exe::call_va!(0x1_4183_acc8, (new_buf, old_buf, cur_bytes) -> usize);
        let old_buf2 = rd::<usize>(rbx + 0x10);
        if old_buf2 != 0 {
            exe::call_va!(0x1_4241_c190, (old_buf2) -> ());
        }
        wr::<usize>(rbx + 0x10, new_buf);
        wr::<u32>(rbx + 0x18, len as u32);
        wr::<u32>(rbx + 0x1c, new_cap as u32);
        buf = new_buf;
    }
    let packed = (a3 as u64) | ((a4 as u64) << 16) | ((a6 as u64) << 32);
    wr::<u64>(buf + len * 8, packed);
    wr::<u32>(rbx + 0x18, rd::<u32>(rbx + 0x18).wrapping_add(1));

    let len2 = rd::<u32>(this + 0x38) as usize;
    let cap2 = rd::<u32>(this + 0x3c) as usize;
    if len2 * 2 == cap2 * 2 {
        exe::call_va!(0x1_4039_c2d0, (this.wrapping_add(0x30), len2.wrapping_add(1)) -> ());
    }
    let idx2 = rd::<u32>(this + 0x38) as usize;
    let buf2 = rd::<usize>(this + 0x30);
    wr::<u16>(buf2 + idx2 * 2, tag);
    wr::<u32>(this + 0x38, (idx2 as u32).wrapping_add(1));

    let lo = rd::<u16>(range_pair);
    let hi = rd::<u16>(range_pair + 0x2).wrapping_add(1);
    wr::<u32>(range_pair, (lo as u32) | ((hi as u32) << 16));
}

/// Original: `0x1420a6040` (278 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420a6040(this: usize, out_range: usize) -> usize {
    let r8w = rd::<u16>(rd::<usize>(this + 0x8) + 0x8);
    let ctx0 = rd::<usize>(this);
    let rbp = rd::<u64>(this + 0x40);
    wr::<u16>(out_range, r8w);
    wr::<u16>(out_range + 0x2, r8w);

    let count = rd::<u32>(ctx0 + 0x18) as usize;
    let mut rbx = 0usize;
    let mut edi: u16 = 0;
    if count != 0 {
        let arr = rd::<usize>(this + 0x10);
        loop {
            edi = rbx as u16;
            if rd::<u8>(arr + (edi as usize) * 4 + 0x2) == 0 {
                break;
            }
            rbx += 1;
            if rbx == count {
                break;
            }
        }
    }

    while rbx != count {
        exe::call_va!(0x1_420a_5dc0, (this, edi, rbp.wrapping_add(1)) -> ());
        let lo = rd::<u16>(out_range);
        let hi = rd::<u16>(out_range + 0x2).wrapping_add(1);
        wr::<u32>(out_range, (lo as u32) | ((hi as u32) << 16));
        rbx += 1;
        if rbx == count {
            break;
        }
        let arr = rd::<usize>(this + 0x10);
        loop {
            edi = rbx as u16;
            if rd::<u8>(arr + (edi as usize) * 4 + 0x2) == 0 {
                break;
            }
            rbx += 1;
            if rbx == count {
                break;
            }
        }
    }
    out_range
}

/// Original: `0x1420a6160` (385 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420a6160(
    this: usize,
    out_range: usize,
    mask_pair: usize,
) -> usize {
    let mask_req = rd::<u64>(mask_pair);
    let mask_forbid = rd::<u64>(mask_pair + 0x8);
    let r9w = rd::<u16>(rd::<usize>(this + 0x8) + 0x8);
    let r10 = rd::<usize>(this);
    let rbp = rd::<u64>(this + 0x40);
    let count = rd::<u32>(r10 + 0x18) as usize;
    wr::<u16>(out_range, r9w);
    wr::<u16>(out_range + 0x2, r9w);

    let mut rbx = 0usize;
    let mut edi: u16 = 0;
    if count != 0 {
        let arr = rd::<usize>(r10 + 0x10);
        loop {
            edi = rbx as u16;
            let flags = rd::<u64>(arr + (edi as usize) * 0x30 + 0x10);
            if (flags & mask_req) == mask_req && (flags & mask_forbid) == 0 {
                break;
            }
            rbx += 1;
            if rbx == count {
                break;
            }
        }
    }

    while rbx != count {
        exe::call_va!(0x1_420a_5dc0, (this, edi, rbp.wrapping_add(1)) -> ());
        let lo = rd::<u16>(out_range);
        let hi = rd::<u16>(out_range + 0x2).wrapping_add(1);
        wr::<u32>(out_range, (lo as u32) | ((hi as u32) << 16));
        rbx += 1;
        if rbx == count {
            break;
        }
        let arr = rd::<usize>(r10 + 0x10);
        loop {
            edi = rbx as u16;
            let flags = rd::<u64>(arr + (edi as usize) * 0x30 + 0x10);
            if (flags & mask_req) == mask_req && (flags & mask_forbid) == 0 {
                break;
            }
            rbx += 1;
            if rbx == count {
                break;
            }
        }
    }
    out_range
}

/// Original: `0x1420a63f0` (269 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420a63f0(this: usize, out_id: usize, id: u16) -> usize {
    let r11 = rd::<usize>(this);
    let idx = id as usize;
    let r10 = rd::<usize>(r11 + 0x10).wrapping_add(idx * 0x30);
    let mask_req = rd::<u64>(this + 0x18);
    let mask_forbid = rd::<u64>(this + 0x20);
    let flags = rd::<u64>(r10 + 0x10);
    if (flags & mask_req) == mask_req && (flags & mask_forbid) == 0 {
        let map_vec = rd::<usize>(this + 0x8);
        let map_arr = rd::<usize>(map_vec);
        let rsi = map_arr.wrapping_add(idx * 2);
        if rd::<u16>(rsi) == 0xffff {
            let name_view = rd::<[u8; 16]>(r10);
            let arg5 = rd::<u16>(r10 + 0x22);
            let arg6 = rd::<u16>(r10 + 0x24);
            let r9 = rd::<u64>(this + 0x28);
            let mut tmp_id: u16 = 0;
            let tmp_ptr = (&mut tmp_id as *mut u16) as usize;
            let ret_ptr = exe::call_va!(
                0x1_420a_6ca0,
                (r11, tmp_ptr, name_view.as_ptr() as usize, r9, arg5, arg6) -> usize
            );
            wr::<u16>(rsi, rd::<u16>(ret_ptr));

            let list_vec = rd::<usize>(this + 0x10);
            let len = rd::<u32>(list_vec + 0x8) as usize;
            let cap = rd::<u32>(list_vec + 0xc) as usize;
            if len * 2 == cap * 2 {
                exe::call_va!(0x1_4039_c2d0, (list_vec, len.wrapping_add(1)) -> ());
            }
            let idx2 = rd::<u32>(list_vec + 0x8) as usize;
            let buf2 = rd::<usize>(list_vec);
            wr::<u16>(buf2 + idx2 * 2, rd::<u16>(rsi));
            wr::<u32>(list_vec + 0x8, (idx2 as u32).wrapping_add(1));
        }
        wr::<u16>(out_id, rd::<u16>(rsi));
        out_id
    } else {
        wr::<u16>(out_id, id);
        out_id
    }
}

/// Original: `0x1420a6b70` (254 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420a6b70(
    this: usize,
    src_id: u16,
    dst_id: u16,
    edge_type: u32,
) {
    let len = rd::<u32>(this + 0x8) as usize;
    let byte_len = len * 12;
    let cur_cnt = ((byte_len as i64) / 12) as u64;
    let cap = rd::<u32>(this + 0xc) as usize;
    if byte_len == cap * 12 {
        exe::call_va!(0x1_410c_0800, (this, cur_cnt.wrapping_add(1)) -> ());
    }

    let idx = rd::<u32>(this + 0x8) as usize;
    let buf = rd::<usize>(this);
    let slot = buf + idx * 12;
    wr::<u64>(slot, 0);
    wr::<u32>(slot + 0x8, 0);
    wr::<u16>(slot, 0xffff);
    wr::<u32>(this + 0x8, (idx as u32).wrapping_add(1));

    let new_len = rd::<u32>(this + 0x8) as usize;
    let buf2 = rd::<usize>(this);
    let elem = buf2 + new_len * 12 - 12;
    wr::<u16>(elem, dst_id);
    wr::<u32>(elem + 0x4, edge_type);

    let nodes = rd::<usize>(this + 0x10);
    let src_node = nodes + (src_id as usize) * 0x30;
    if rd::<u8>(src_node + 0x28) != 0 {
        wr::<u32>(elem + 0x8, rd::<u32>(src_node + 0x26));
        if rd::<u8>(src_node + 0x28) == 0 {
            wr::<u8>(src_node + 0x28, 1);
        }
    } else {
        wr::<u8>(src_node + 0x28, 1);
    }
    wr::<u16>(src_node + 0x26, cur_cnt as u16);
    let is_two = u8::from(edge_type == 2);
    wr::<u8>(src_node + 0x21, rd::<u8>(src_node + 0x21) | is_two);
    let dst_node = rd::<usize>(this + 0x10) + (dst_id as usize) * 0x30;
    wr::<u8>(dst_node + 0x20, rd::<u8>(dst_node + 0x20) | is_two);
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

unsafe fn verify_sl_res_and_tag(a: usize, b: usize, idx: usize, tag_base_off: usize) -> Result<(), String> {
    crate::diff_eq!(rd::<u32>(a), rd::<u32>(b), "sl batch count");
    let ra = a + 0x8 + idx * 0x70;
    let rb = b + 0x8 + idx * 0x70;
    for off in 0..0x21usize {
        crate::diff_eq!(rd::<u8>(ra + off), rd::<u8>(rb + off), "sl res 0..0x21");
    }
    for off in 0x28..0x60usize {
        crate::diff_eq!(rd::<u8>(ra + off), rd::<u8>(rb + off), "sl res 0x28..0x60");
    }
    for off in 0x64..0x6cusize {
        crate::diff_eq!(rd::<u8>(ra + off), rd::<u8>(rb + off), "sl res 0x64..0x6c");
    }
    let ta = a + tag_base_off + (idx << 6);
    let tb = b + tag_base_off + (idx << 6);
    for off in 0..0x20usize {
        crate::diff_eq!(rd::<u8>(ta + off), rd::<u8>(tb + off), "sl tag 0..0x20");
    }
    crate::diff_eq!(rd::<usize>(ta + 0x20) - a, rd::<usize>(tb + 0x20) - b, "sl tag res_ptr rel");
    for off in 0x28..0x40usize {
        crate::diff_eq!(rd::<u8>(ta + off), rd::<u8>(tb + off), "sl tag 0x28..0x40");
    }
    Ok(())
}

pub unsafe fn diff_d3d_14208df00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut batch_o = [0u8; 0x100];
    let mut batch_n = [0u8; 0x100];
    let mut res = [0u8; 0x80];
    let res_ptr = res.as_mut_ptr() as usize;
    wr::<u32>(res_ptr + 0x48, 0x1234);
    wr::<u64>(res_ptr + 0x70, 0xdead_beef_cafe_0001);

    f_orig(batch_o.as_mut_ptr() as usize, res_ptr);
    f_new(batch_n.as_mut_ptr() as usize, res_ptr);
    verify_sl_res_and_tag(batch_o.as_ptr() as usize, batch_n.as_ptr() as usize, 0, 0x78)?;
    Ok(1)
}

pub unsafe fn diff_d3d_14208e030(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut batch_o = [0u8; 0x500];
    let mut batch_n = [0u8; 0x500];
    let mut sub = [0u8; 0x20];
    wr::<u32>((sub.as_mut_ptr() as usize) + 0x18, 0x800);
    let mut res = [0u8; 0x80];
    let res_ptr = res.as_mut_ptr() as usize;
    wr::<usize>(res_ptr + 0x40, sub.as_ptr() as usize);
    wr::<u64>(res_ptr + 0x70, 0xdead_beef_cafe_0002);

    f_orig(batch_o.as_mut_ptr() as usize, res_ptr, 7);
    f_new(batch_n.as_mut_ptr() as usize, res_ptr, 7);
    verify_sl_res_and_tag(batch_o.as_ptr() as usize, batch_n.as_ptr() as usize, 0, 0x468)?;
    Ok(1)
}

pub unsafe fn diff_d3d_14208e190(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut batch_o = [0u8; 0x200];
    let mut batch_n = [0u8; 0x200];
    let mut res = [0u8; 0x80];
    let res_ptr = res.as_mut_ptr() as usize;
    wr::<u32>(res_ptr + 0x48, 0x400);
    wr::<u64>(res_ptr + 0x70, 0xdead_beef_cafe_0003);

    f_orig(batch_o.as_mut_ptr() as usize, res_ptr, 2);
    f_new(batch_n.as_mut_ptr() as usize, res_ptr, 2);
    verify_sl_res_and_tag(batch_o.as_ptr() as usize, batch_n.as_ptr() as usize, 0, 0x158)?;
    Ok(1)
}

pub unsafe fn diff_d3d_14208f090(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u32, u32, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut tex = [0u8; 0x20];
    let tex_ptr = tex.as_mut_ptr() as usize;
    wr::<u32>(tex_ptr + 0x8, 1920);
    wr::<u32>(tex_ptr + 0xc, 1080);
    wr::<u32>(tex_ptr + 0x10, 1);
    wr::<u8>(tex_ptr + 0x14, 7);

    let mut slot_o = tex_ptr;
    let mut slot_n = tex_ptr;
    f_orig((&mut slot_o as *mut usize) as usize, 0, 1920, 1080, 7);
    f_new((&mut slot_n as *mut usize) as usize, 0, 1920, 1080, 7);
    crate::diff_eq!(slot_o, slot_n, "cache hit slot preserved");
    Ok(1)
}

unsafe extern "win64" fn mock_copy_tex_1420670e0(dst_res: usize, src: usize) {
    wr::<u32>(dst_res, rd::<u32>(src).wrapping_add(0x100));
    wr::<u32>(dst_res + 0x4, rd::<u32>(src + 0x4).wrapping_add(0x200));
}

pub unsafe fn diff_d3d_14208f220(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m_copy = crate::difftest::Mock::new(0x1_4206_70e0, mock_copy_tex_1420670e0 as *const ())?;

    let mut src = [0u8; 0x20];
    let src_ptr = src.as_mut_ptr() as usize;
    wr::<u32>(src_ptr, 1280);
    wr::<u32>(src_ptr + 0x4, 720);
    wr::<u32>(src_ptr + 0xc, 1);
    wr::<u8>(src_ptr + 0x18, 5);

    let mut dst_res_o = [0u32; 4];
    let mut dst_res_n = [0u32; 4];
    let mut holder_o = [0u8; 0x20];
    let ho = holder_o.as_mut_ptr() as usize;
    wr::<usize>(ho, dst_res_o.as_mut_ptr() as usize);
    wr::<u32>(ho + 0x8, 1280);
    wr::<u32>(ho + 0xc, 720);

    let mut holder_n = [0u8; 0x20];
    let hn = holder_n.as_mut_ptr() as usize;
    wr::<usize>(hn, dst_res_n.as_mut_ptr() as usize);
    wr::<u32>(hn + 0x8, 1280);
    wr::<u32>(hn + 0xc, 720);

    let mut slot_o = ho;
    let mut slot_n = hn;
    f_orig((&mut slot_o as *mut usize) as usize, src_ptr, 0x77);
    f_new((&mut slot_n as *mut usize) as usize, src_ptr, 0x77);
    crate::diff_eq!(dst_res_o, dst_res_n, "copied into cached dst_res");
    Ok(1)
}

unsafe extern "win64" fn mock_sl_feature_ok(_feat: u32, _adapter: usize) -> u32 {
    0
}

unsafe extern "win64" fn mock_dlss_optimal_settings(opts: usize, settings: usize) -> u32 {
    let w = rd::<u32>(opts + 0x24);
    let h = rd::<u32>(opts + 0x28);
    let m = rd::<u32>(opts + 0x20);
    wr::<u32>(settings + 0x20, (w >> 1).wrapping_add(m));
    wr::<u32>(settings + 0x24, (h >> 1).wrapping_add(m));
    wr::<u32>(settings + 0x34, w.wrapping_sub(m));
    wr::<u32>(settings + 0x38, h.wrapping_sub(m));
    0
}

pub unsafe fn diff_d3d_142091970(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32, u32, u32, usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _im = crate::difftest::IatMock::new(0x1_4264_7340, mock_sl_feature_ok as *const ())?;
    let fn_slot = exe::ptr::<usize>(0x1_43c8_c5d8) as usize;
    let saved_fn = rd::<usize>(fn_slot);
    wr::<usize>(fn_slot, mock_dlss_optimal_settings as *const () as usize);

    for mode in [1u32, 4u32, 5u32] {
        let mut ow_o = 0xaaaa_bbbbu32;
        let mut oh_o = 0xcccc_ddddu32;
        let mut ow_n = 0xaaaa_bbbbu32;
        let mut oh_n = 0xcccc_ddddu32;
        let ro = f_orig(
            1920,
            1080,
            mode,
            (&mut ow_o as *mut u32) as usize,
            (&mut oh_o as *mut u32) as usize,
        );
        let rn = f_new(
            1920,
            1080,
            mode,
            (&mut ow_n as *mut u32) as usize,
            (&mut oh_n as *mut u32) as usize,
        );
        crate::diff_eq!(ro, rn, "dlss ret");
        crate::diff_eq!(ow_o, ow_n, "dlss out_w");
        crate::diff_eq!(oh_o, oh_n, "dlss out_h");
    }

    wr::<usize>(fn_slot, saved_fn);
    Ok(3)
}

unsafe extern "win64" fn mock_adapter_addref(this: usize) -> u32 {
    let c = rd::<u32>(this + 0x8).wrapping_add(1);
    wr::<u32>(this + 0x8, c);
    c
}

unsafe extern "win64" fn mock_adapter_release(this: usize) -> u32 {
    let c = rd::<u32>(this + 0xc).wrapping_add(1);
    wr::<u32>(this + 0xc, c);
    c
}

unsafe extern "win64" fn mock_adapter_query_video_mem(
    this: usize,
    _node: u32,
    _group: u32,
    info: usize,
) -> u32 {
    let hr = rd::<u32>(this + 0x10);
    if hr == 0 {
        wr::<u64>(info, 0x1111_2222_3333_4444);
        wr::<u64>(info + 0x8, 0x5555_6666_7777_8888);
    }
    hr
}

pub unsafe fn diff_d3d_142091f00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let ctx_slot = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_ctx = rd::<usize>(ctx_slot);

    let mut vtbl = [0usize; 16];
    vtbl[1] = mock_adapter_addref as *const () as usize;
    vtbl[2] = mock_adapter_release as *const () as usize;
    vtbl[14] = mock_adapter_query_video_mem as *const () as usize;

    for hr in [0u32, 0x8000_4005u32] {
        let mut ad_o = [0u64; 3];
        let mut ad_n = [0u64; 3];
        wr::<usize>(ad_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<u32>((ad_o.as_mut_ptr() as usize) + 0x10, hr);
        wr::<usize>(ad_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<u32>((ad_n.as_mut_ptr() as usize) + 0x10, hr);

        let mut ctx_o = vec![0u8; 0x3bf0];
        let mut ctx_n = vec![0u8; 0x3bf0];
        wr::<usize>((ctx_o.as_mut_ptr() as usize) + 0x3be0, ad_o.as_mut_ptr() as usize);
        wr::<usize>((ctx_n.as_mut_ptr() as usize) + 0x3be0, ad_n.as_mut_ptr() as usize);

        let mut bud_o = 99u64;
        let mut use_o = 99u64;
        let mut bud_n = 99u64;
        let mut use_n = 99u64;

        wr::<usize>(ctx_slot, ctx_o.as_mut_ptr() as usize);
        let ro = f_orig(
            (&mut bud_o as *mut u64) as usize,
            (&mut use_o as *mut u64) as usize,
            0,
        );
        wr::<usize>(ctx_slot, ctx_n.as_mut_ptr() as usize);
        let rn = f_new(
            (&mut bud_n as *mut u64) as usize,
            (&mut use_n as *mut u64) as usize,
            0,
        );
        crate::diff_eq!(ro, rn, "query video mem ret");
        crate::diff_eq!(bud_o, bud_n, "budget");
        crate::diff_eq!(use_o, use_n, "usage");
        crate::diff_eq!(ad_o, ad_n, "adapter refcounts");
    }

    wr::<usize>(ctx_slot, saved_ctx);
    Ok(2)
}

pub unsafe fn diff_d3d_142092b00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let page_slot = exe::ptr::<usize>(0x1_4397_fca8) as usize;
    let saved_owner = rd::<u32>(owner_addr);
    let saved_depth = rd::<u32>(depth_addr);
    let saved_page = rd::<usize>(page_slot);

    wr::<u32>(owner_addr, tid);
    wr::<u32>(depth_addr, 1);

    let mut page_o = [0u8; 0x40];
    let mut page_n = [0u8; 0x40];
    let mut inner_o = [0u8; 0x70];
    let io = inner_o.as_mut_ptr() as usize;
    wr::<u64>(io + 0x8, 0x7788_99aa);
    wr::<u8>(io + 0x1c, 1);
    wr::<u32>(io + 0x28, tid);
    wr::<u32>(io + 0x2c, 1);
    wr::<usize>(io + 0x68, 0x1234);
    let mut inner_n = inner_o;

    let this_o = [inner_o.as_mut_ptr() as usize];
    let this_n = [inner_n.as_mut_ptr() as usize];

    wr::<usize>(page_slot, page_o.as_mut_ptr() as usize);
    f_orig(this_o.as_ptr() as usize, 0x1234);

    wr::<usize>(page_slot, page_n.as_mut_ptr() as usize);
    f_new(this_n.as_ptr() as usize, 0x1234);

    wr::<u32>(owner_addr, saved_owner);
    wr::<u32>(depth_addr, saved_depth);
    wr::<usize>(page_slot, saved_page);

    crate::diff_eq!(page_o, page_n, "page gen updated");
    crate::diff_eq!(&inner_o[0..0x68], &inner_n[0..0x68], "inner head");
    crate::diff_eq!(
        rd::<usize>((inner_o.as_ptr() as usize) + 0x68) - (page_o.as_ptr() as usize),
        rd::<usize>((inner_n.as_ptr() as usize) + 0x68) - (page_n.as_ptr() as usize),
        "inner page ptr"
    );
    Ok(1)
}

pub unsafe fn diff_d3d_142091330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let base = exe::ptr::<u8>(0x1_4385_8320) as usize;
    let saved = rd::<[u8; 0x154]>(base);
    wr::<[u8; 0x154]>(base, [0x5au8; 0x154]);
    let r_o = f_orig();
    let snap_o = rd::<[u8; 0x154]>(base);

    wr::<[u8; 0x154]>(base, [0x5au8; 0x154]);
    let r_n = f_new();
    let snap_n = rd::<[u8; 0x154]>(base);
    wr::<[u8; 0x154]>(base, saved);

    crate::diff_eq!(r_o, r_n, "ret base");
    crate::diff_eq!(snap_o, snap_n, "sl global init bytes");
    Ok(1)
}

pub unsafe fn diff_d3d_142092ca0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let mut inner_o = [0u8; 0x70];
    wr::<u32>((inner_o.as_mut_ptr() as usize) + 0x28, tid);
    wr::<u32>((inner_o.as_mut_ptr() as usize) + 0x2c, 1);
    let mut inner_n = inner_o;

    let this_o = [inner_o.as_mut_ptr() as usize, 0, 0, 0, 0];
    let this_n = [inner_n.as_mut_ptr() as usize, 0, 0, 0, 0];
    let mut out_o = 0x1234usize;
    let mut out_n = 0x1234usize;
    let r_o = f_orig(this_o.as_ptr() as usize, (&mut out_o as *mut usize) as usize);
    let r_n = f_new(this_n.as_ptr() as usize, (&mut out_n as *mut usize) as usize);
    crate::diff_eq!(r_o, r_n, "null page ret");
    crate::diff_eq!(out_o, out_n, "null page out");
    crate::diff_eq!(inner_o, inner_n, "inner state");
    Ok(1)
}

unsafe extern "win64" fn mock_alloc_page_142092770(rsi: usize) -> usize {
    rd::<usize>(rsi + 0x10)
}

pub unsafe fn diff_d3d_142092e00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let page_slot = exe::ptr::<usize>(0x1_4397_fca8) as usize;
    let stride_slot = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let rsi_slot = exe::ptr::<usize>(0x1_4397_fcb8) as usize;

    let saved_owner = rd::<u32>(owner_addr);
    let saved_depth = rd::<u32>(depth_addr);
    let saved_page = rd::<usize>(page_slot);
    let saved_stride = rd::<u32>(stride_slot);
    let saved_rsi = rd::<usize>(rsi_slot);

    let mut page = [0u64; 4];
    page[1] = 0x1000_0000;
    let mut rsi_buf = vec![0u8; 0x2b0];
    let rp = rsi_buf.as_mut_ptr() as usize;
    wr::<u32>(rp + 0x28, tid);
    wr::<u32>(rp + 0x2c, 1);
    wr::<u64>(rp + 0x2a0, 0xdead_beef_cafe_0000);

    wr::<u32>(owner_addr, tid);
    wr::<u32>(depth_addr, 1);
    wr::<usize>(page_slot, page.as_ptr() as usize);
    wr::<u32>(stride_slot, 32);
    wr::<usize>(rsi_slot, rp);

    let mut heap_o = 0u64;
    let mut desc_o = 0u64;
    let mut heap_n = 0u64;
    let mut desc_n = 0u64;
    f_orig(
        0,
        0,
        (&mut heap_o as *mut u64) as usize,
        (&mut desc_o as *mut u64) as usize,
    );
    f_new(
        0,
        0,
        (&mut heap_n as *mut u64) as usize,
        (&mut desc_n as *mut u64) as usize,
    );

    wr::<u32>(owner_addr, saved_owner);
    wr::<u32>(depth_addr, saved_depth);
    wr::<usize>(page_slot, saved_page);
    wr::<u32>(stride_slot, saved_stride);
    wr::<usize>(rsi_slot, saved_rsi);

    crate::diff_eq!(heap_o, heap_n, "out_heap");
    crate::diff_eq!(desc_o, desc_n, "out_cpu_desc");
    Ok(1)
}

pub unsafe fn diff_d3d_142092f40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let page_slot = exe::ptr::<usize>(0x1_4397_fca8) as usize;
    let stride_slot = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let rsi_slot = exe::ptr::<usize>(0x1_4397_fcb8) as usize;

    let saved_owner = rd::<u32>(owner_addr);
    let saved_depth = rd::<u32>(depth_addr);
    let saved_page = rd::<usize>(page_slot);
    let saved_stride = rd::<u32>(stride_slot);
    let saved_rsi = rd::<usize>(rsi_slot);

    let mut page = [0u64; 4];
    page[1] = 0x2000_0000;
    let mut rsi_buf = vec![0u8; 0x290];
    let rp = rsi_buf.as_mut_ptr() as usize;
    wr::<u32>(rp + 0x28, tid);
    wr::<u32>(rp + 0x2c, 1);
    wr::<u64>(rp + 0x280, 0x4000_0000);

    wr::<u32>(owner_addr, tid);
    wr::<u32>(depth_addr, 1);
    wr::<usize>(page_slot, page.as_ptr() as usize);
    wr::<u32>(stride_slot, 16);
    wr::<usize>(rsi_slot, rp);

    let mut gpu_o = 0u64;
    let mut cpu_o = 0u64;
    let mut gpu_n = 0u64;
    let mut cpu_n = 0u64;
    f_orig(
        0,
        0,
        (&mut gpu_o as *mut u64) as usize,
        (&mut cpu_o as *mut u64) as usize,
    );
    f_new(
        0,
        0,
        (&mut gpu_n as *mut u64) as usize,
        (&mut cpu_n as *mut u64) as usize,
    );

    wr::<u32>(owner_addr, saved_owner);
    wr::<u32>(depth_addr, saved_depth);
    wr::<usize>(page_slot, saved_page);
    wr::<u32>(stride_slot, saved_stride);
    wr::<usize>(rsi_slot, saved_rsi);

    crate::diff_eq!(gpu_o, gpu_n, "out_gpu");
    crate::diff_eq!(cpu_o, cpu_n, "out_cpu");
    Ok(1)
}

pub unsafe fn diff_d3d_142093110(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m = crate::difftest::Mock::new(0x1_4209_2770, mock_alloc_page_142092770 as *const ())?;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let page_slot = exe::ptr::<usize>(0x1_4397_fca8) as usize;
    let rsi_slot = exe::ptr::<usize>(0x1_4397_fcb8) as usize;

    let saved_owner = rd::<u32>(owner_addr);
    let saved_depth = rd::<u32>(depth_addr);
    let saved_page = rd::<usize>(page_slot);
    let saved_rsi = rd::<usize>(rsi_slot);

    let mut rsi_buf = [0u8; 0x40];
    let rp = rsi_buf.as_mut_ptr() as usize;
    wr::<usize>(rp + 0x10, 0x8888_7777);
    wr::<u32>(rp + 0x28, tid);
    wr::<u32>(rp + 0x2c, 1);

    wr::<u32>(owner_addr, tid);
    wr::<u32>(depth_addr, 1);
    wr::<usize>(rsi_slot, rp);

    wr::<usize>(page_slot, 0);
    f_orig(0);
    let page_o = rd::<usize>(page_slot);

    wr::<usize>(page_slot, 0);
    f_new(0);
    let page_n = rd::<usize>(page_slot);

    wr::<u32>(owner_addr, saved_owner);
    wr::<u32>(depth_addr, saved_depth);
    wr::<usize>(page_slot, saved_page);
    wr::<usize>(rsi_slot, saved_rsi);

    crate::diff_eq!(page_o, page_n, "allocated page stored");
    Ok(1)
}

pub unsafe fn diff_d3d_1420933f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let owner_addr = exe::ptr::<u32>(0x1_4385_8480) as usize;
    let depth_addr = exe::ptr::<u32>(0x1_4385_8484) as usize;
    let saved_owner = rd::<u32>(owner_addr);
    let saved_depth = rd::<u32>(depth_addr);
    wr::<u32>(owner_addr, tid);
    wr::<u32>(depth_addr, 1);

    let mut obj_o = [0u8; 0x20];
    let mut obj_n = [0u8; 0x20];
    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);

    wr::<u32>(owner_addr, saved_owner);
    wr::<u32>(depth_addr, saved_depth);
    crate::diff_eq!(obj_o, obj_n, "inactive obj unchanged");
    Ok(1)
}

unsafe extern "win64" fn mock_next_obj_142098ff0(arg: usize) -> usize {
    rd::<usize>(arg).wrapping_add(0x5000)
}

pub unsafe fn diff_d3d_1420995c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m = crate::difftest::Mock::new(0x1_4209_8ff0, mock_next_obj_142098ff0 as *const ())?;

    let rbx = exe::ptr::<u8>(0x1_4385_8490) as usize;
    let ctx_slot = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let snap_rbx = crate::difftest::Snapshot::take(rbx as *mut u8, 0x70);
    let saved_ctx = rd::<usize>(ctx_slot);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);

    let mut ctx = [0u8; 0x50];
    wr::<u64>((ctx.as_mut_ptr() as usize) + 0x38, 0xaaaa_bbbb_cccc_dddd);
    wr::<u64>((ctx.as_mut_ptr() as usize) + 0x40, 0x1111_2222_3333_4444);
    wr::<usize>(ctx_slot, ctx.as_ptr() as usize);

    let mut cur_o = [0u64; 5];
    let mut cur_n = [0u64; 5];
    let mut ring_o = [0usize; 4];
    let mut ring_n = [0usize; 4];

    wr::<[u8; 0x70]>(rbx, [0u8; 0x70]);
    wr::<u32>(rbx + 0x8, tid);
    wr::<u32>(rbx + 0xc, 1);
    wr::<usize>(rbx + 0x10, cur_o.as_mut_ptr() as usize);
    wr::<usize>(rbx + 0x18, 0x2000);
    wr::<usize>(rbx + 0x48, ring_o.as_mut_ptr() as usize);
    wr::<u32>(rbx + 0x54, 1);
    wr::<u32>(rbx + 0x58, 1);
    wr::<u32>(rbx + 0x5c, 4);
    f_orig(0);
    let state_o = rd::<[u8; 0x70]>(rbx);

    wr::<[u8; 0x70]>(rbx, [0u8; 0x70]);
    wr::<u32>(rbx + 0x8, tid);
    wr::<u32>(rbx + 0xc, 1);
    wr::<usize>(rbx + 0x10, cur_n.as_mut_ptr() as usize);
    wr::<usize>(rbx + 0x18, 0x2000);
    wr::<usize>(rbx + 0x48, ring_n.as_mut_ptr() as usize);
    wr::<u32>(rbx + 0x54, 1);
    wr::<u32>(rbx + 0x58, 1);
    wr::<u32>(rbx + 0x5c, 4);
    f_new(0);
    let state_n = rd::<[u8; 0x70]>(rbx);

    snap_rbx.restore();
    wr::<usize>(ctx_slot, saved_ctx);

    crate::diff_eq!(cur_o, cur_n, "cur fence pair");
    crate::diff_eq!(
        ring_o[1] - (cur_o.as_ptr() as usize),
        ring_n[1] - (cur_n.as_ptr() as usize),
        "ring slot 1"
    );
    crate::diff_eq!(&state_o[0..0x48], &state_n[0..0x48], "rbx low");
    crate::diff_eq!(&state_o[0x50..0x70], &state_n[0x50..0x70], "rbx high");
    Ok(1)
}

pub unsafe fn diff_d3d_142099ae0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let base = exe::ptr::<u8>(0x1_4385_84a0) as usize;
    let saved = rd::<[u8; 0xa0]>(base);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);

    let mut ring_o = [0usize; 4];
    let mut ring_n = [0usize; 4];

    wr::<[u8; 0xa0]>(base, [0u8; 0xa0]);
    wr::<usize>(base, 0x1111_2222);
    wr::<u32>(base + 0x10, tid);
    wr::<u32>(base + 0x14, 1);
    wr::<usize>(base + 0x18, ring_o.as_mut_ptr() as usize);
    wr::<u32>(base + 0x2c, 4);
    f_orig();
    let snap_o = rd::<[u8; 0xa0]>(base);

    wr::<[u8; 0xa0]>(base, [0u8; 0xa0]);
    wr::<usize>(base, 0x1111_2222);
    wr::<u32>(base + 0x10, tid);
    wr::<u32>(base + 0x14, 1);
    wr::<usize>(base + 0x18, ring_n.as_mut_ptr() as usize);
    wr::<u32>(base + 0x2c, 4);
    f_new();
    let snap_n = rd::<[u8; 0xa0]>(base);

    wr::<[u8; 0xa0]>(base, saved);
    crate::diff_eq!(ring_o, ring_n, "ring contents");
    crate::diff_eq!(&snap_o[0..0x18], &snap_n[0..0x18], "state low");
    crate::diff_eq!(&snap_o[0x20..0xa0], &snap_n[0x20..0xa0], "state high");
    Ok(1)
}

pub unsafe fn diff_d3d_14209a320(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let map_addr = exe::ptr::<usize>(0x1_43a8_77c0) as usize;
    let saved_head = rd::<usize>(map_addr);

    let mut head = [0u8; 0x30];
    let head_ptr = head.as_mut_ptr() as usize;
    wr::<u8>(head_ptr + 0x19, 1);

    let mut root_o = [0u8; 0x30];
    let root_o_ptr = root_o.as_mut_ptr() as usize;
    wr::<usize>(root_o_ptr, head_ptr);
    wr::<usize>(root_o_ptr + 0x10, head_ptr);
    wr::<i32>(root_o_ptr + 0x20, 42);

    let mut root_n = root_o;
    let root_n_ptr = root_n.as_mut_ptr() as usize;

    wr::<usize>(head_ptr + 0x8, root_o_ptr);
    wr::<usize>(map_addr, head_ptr);
    f_orig(42, 0x1234_5678_9abc_def0);

    wr::<usize>(head_ptr + 0x8, root_n_ptr);
    f_new(42, 0x1234_5678_9abc_def0);

    wr::<usize>(map_addr, saved_head);
    crate::diff_eq!(root_o, root_n, "rb tree update");
    Ok(1)
}

unsafe extern "win64" fn mock_stream_read_str(_stream: usize, str_ptr: usize) {
    wr::<u32>(str_ptr + 0x4, 3);
    wr::<[u8; 4]>(str_ptr + 0x8, *b"abc\0");
}

unsafe extern "win64" fn mock_stream_read_u32(_stream: usize, out: usize) {
    wr::<u32>(out, 0);
}

unsafe extern "win64" fn mock_stream_read_u8(_stream: usize, out: usize) {
    wr::<u8>(out, 7);
}

unsafe extern "win64" fn mock_hash_cstr(cstr: usize) -> u32 {
    (rd::<u8>(cstr) as u32).wrapping_add(0x1000)
}

pub unsafe fn diff_d3d_14209b4a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m = crate::difftest::Mock::new(0x1_4209_a520, mock_hash_cstr as *const ())?;

    let mut vtbl = [0usize; 20];
    vtbl[0x20 / 8] = mock_stream_read_str as *const () as usize;
    vtbl[0x48 / 8] = mock_stream_read_u32 as *const () as usize;
    vtbl[0x80 / 8] = mock_stream_read_u8 as *const () as usize;
    let stream = [vtbl.as_ptr() as usize];

    let hash_slot = exe::ptr::<usize>(0x1_4397_fd20) as usize;
    let saved_hash = rd::<usize>(hash_slot);
    wr::<usize>(hash_slot, mock_hash_cstr as *const () as usize);

    let mut out_o = [0u8; 0x30];
    let mut out_n = [0u8; 0x30];
    let r_o = f_orig(stream.as_ptr() as usize, out_o.as_mut_ptr() as usize);
    let r_n = f_new(stream.as_ptr() as usize, out_n.as_mut_ptr() as usize);

    wr::<usize>(hash_slot, saved_hash);
    crate::diff_eq!(r_o, r_n, "stream ret");
    crate::diff_eq!(out_o, out_n, "deserialized struct");
    Ok(1)
}

pub unsafe fn diff_d3d_14209b5f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 20];
    vtbl[0x20 / 8] = mock_stream_read_str as *const () as usize;
    vtbl[0x48 / 8] = mock_stream_read_u32 as *const () as usize;
    vtbl[0x80 / 8] = mock_stream_read_u8 as *const () as usize;
    let stream = [vtbl.as_ptr() as usize];

    let hash_slot = exe::ptr::<usize>(0x1_4397_fd20) as usize;
    let saved_hash = rd::<usize>(hash_slot);
    wr::<usize>(hash_slot, mock_hash_cstr as *const () as usize);

    let mut out_o = [0x5au8; 0x20];
    let mut out_n = [0x5au8; 0x20];
    let r_o = f_orig(stream.as_ptr() as usize, out_o.as_mut_ptr() as usize);
    let r_n = f_new(stream.as_ptr() as usize, out_n.as_mut_ptr() as usize);

    wr::<usize>(hash_slot, saved_hash);
    crate::diff_eq!(r_o, r_n, "stream ret");
    crate::diff_eq!(out_o, out_n, "deserialized struct");
    Ok(1)
}

pub unsafe fn diff_d3d_14209b7e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 20];
    vtbl[0x48 / 8] = mock_stream_read_u32 as *const () as usize;
    vtbl[0x80 / 8] = mock_stream_read_u8 as *const () as usize;
    let stream = [vtbl.as_ptr() as usize];

    let mut out_o = [0x5au8; 0xb0];
    let mut out_n = [0x5au8; 0xb0];
    f_orig(out_o.as_mut_ptr() as usize, stream.as_ptr() as usize);
    f_new(out_n.as_mut_ptr() as usize, stream.as_ptr() as usize);
    crate::diff_eq!(&out_o[0..4], &out_n[0..4], "hdr");
    crate::diff_eq!(&out_o[0x8..0xa9], &out_n[0x8..0xa9], "vecs and tail");
    Ok(1)
}

pub unsafe fn diff_d3d_14209be60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let mut block = [0usize; 2];
    block[0] = 0xdead_beef;
    let mut page_o = [0u64; 4];
    page_o[2] = block.as_ptr() as u64;
    let mut page_n = page_o;

    let idx = ((0x100usize + 7) >> 3) + 1;
    let mut heap_o = [0usize; 40];
    heap_o[idx] = page_o.as_mut_ptr() as usize;
    let mut heap_n = [0usize; 40];
    heap_n[idx] = page_n.as_mut_ptr() as usize;

    let mut out_o = 0usize;
    let mut out_n = 0usize;
    wr::<usize>(tls0 + 0x6348, heap_o.as_mut_ptr() as usize);
    f_orig((&mut out_o as *mut usize) as usize, 0x100);
    wr::<usize>(tls0 + 0x6348, heap_n.as_mut_ptr() as usize);
    f_new((&mut out_n as *mut usize) as usize, 0x100);
    wr::<usize>(tls0 + 0x6348, saved_heap);

    crate::diff_eq!(out_o, out_n, "allocated block");
    crate::diff_eq!(page_o, page_n, "page state");
    Ok(1)
}

pub unsafe fn diff_d3d_14209bf70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u32, u32, usize, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut name = [0u8; 0x48];
    let name_ptr = name.as_mut_ptr() as usize;
    wr::<u32>(name_ptr, 0x0800_0008);
    wr::<u32>(name_ptr + 0x4, 5);
    wr::<[u8; 6]>(name_ptr + 0x8, *b"hello\0");

    let data = [1u8; 24];
    let mut h_o = data.as_ptr() as u64;
    let mut h_n = data.as_ptr() as u64;
    let mut this_o = [0u8; 0x70];
    let mut this_n = [0u8; 0x70];

    f_orig(
        this_o.as_mut_ptr() as usize,
        (&mut h_o as *mut u64) as usize,
        24,
        11,
        22,
        name_ptr,
        33,
    );
    f_new(
        this_n.as_mut_ptr() as usize,
        (&mut h_n as *mut u64) as usize,
        24,
        11,
        22,
        name_ptr,
        33,
    );
    crate::diff_eq!(h_o, h_n, "handle cleared");
    crate::diff_eq!(&this_o[0..0x6c], &this_n[0..0x6c], "constructed entry");
    Ok(1)
}

pub unsafe fn diff_d3d_14209d160(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut this_o = [0u8; 0x60];
    let mut this_n = [0u8; 0x60];
    let mut other_o = [0x33u8; 0x60];
    wr::<u64>((other_o.as_mut_ptr() as usize) + 0x58, 0x1234_5678);
    let mut other_n = other_o;

    f_orig(this_o.as_mut_ptr() as usize, other_o.as_mut_ptr() as usize);
    f_new(this_n.as_mut_ptr() as usize, other_n.as_mut_ptr() as usize);
    crate::diff_eq!(&this_o[0..0x54], &this_n[0..0x54], "moved data");
    crate::diff_eq!(&this_o[0x58..0x60], &this_n[0x58..0x60], "moved ptr");
    crate::diff_eq!(other_o, other_n, "other cleared");
    Ok(1)
}

pub unsafe fn diff_d3d_14209d420(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0x5au8; 0x190];
    let p_o = buf_o.as_mut_ptr() as usize;
    wr::<u64>(p_o + 0x40, 0);
    wr::<u64>(p_o + 0x90, 0);
    wr::<u64>(p_o + 0xe0, 0);
    wr::<u8>(p_o + 0x18a, 1);
    wr::<u8>(p_o + 0x18b, 0);
    wr::<u8>(p_o + 0x18c, 1);
    let mut buf_n = buf_o;

    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);
    crate::diff_eq!(buf_o, buf_n, "selective block clear");
    Ok(1)
}

pub unsafe fn diff_d3d_14209db20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut node = [0u64; 5];
    node[4] = 0x9999;
    let mut slabs_o = [0u64, 0, 0, 0, 0, node.as_ptr() as u64];
    let mut slabs_n = slabs_o;

    let mut alloc_o = [0u8; 0x50];
    wr::<usize>((alloc_o.as_mut_ptr() as usize) + 0x40, slabs_o.as_mut_ptr() as usize);
    wr::<u32>((alloc_o.as_mut_ptr() as usize) + 0x48, 2);
    let mut alloc_n = [0u8; 0x50];
    wr::<usize>((alloc_n.as_mut_ptr() as usize) + 0x40, slabs_n.as_mut_ptr() as usize);
    wr::<u32>((alloc_n.as_mut_ptr() as usize) + 0x48, 2);

    let r_o = f_orig(alloc_o.as_mut_ptr() as usize);
    let r_n = f_new(alloc_n.as_mut_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "popped node");
    crate::diff_eq!(slabs_o, slabs_n, "slab freelist updated");
    Ok(1)
}

pub unsafe fn diff_d3d_14209dc80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut pair_o = [0u8; 0x28];
    let po = pair_o.as_mut_ptr() as usize;
    wr::<u16>(po + 0x8, 0);
    wr::<u16>(po + 0xa, 4);
    wr::<u16>(po + 0xc, 2);
    wr::<u8>(po + 0xe, 1);
    wr::<u8>(po + 0x1e, 1);
    let mut pair_n = pair_o;
    let pn = pair_n.as_mut_ptr() as usize;

    let mut fl_o = [0usize; 4];
    let mut fl_n = [0usize; 4];
    let mut this_o = [0u8; 0x50];
    wr::<usize>((this_o.as_mut_ptr() as usize) + 0x30, fl_o.as_mut_ptr() as usize);
    let mut this_n = [0u8; 0x50];
    wr::<usize>((this_n.as_mut_ptr() as usize) + 0x30, fl_n.as_mut_ptr() as usize);

    f_orig(this_o.as_mut_ptr() as usize, po, 2);
    f_new(this_n.as_mut_ptr() as usize, pn, 2);
    crate::diff_eq!(fl_o[2] - po, fl_n[2] - pn, "freelist head rel");
    crate::diff_eq!(pair_o, pair_n, "pair state");
    Ok(1)
}

pub unsafe fn diff_d3d_14209dde0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut node_o = [0u8; 0x10];
    let mut node_n = [0u8; 0x10];
    let mut fl_o = [0usize, node_o.as_mut_ptr() as usize];
    let mut fl_n = [0usize, node_n.as_mut_ptr() as usize];
    let mut this_o = [0u8; 0x40];
    wr::<usize>((this_o.as_mut_ptr() as usize) + 0x30, fl_o.as_mut_ptr() as usize);
    let mut this_n = [0u8; 0x40];
    wr::<usize>((this_n.as_mut_ptr() as usize) + 0x30, fl_n.as_mut_ptr() as usize);

    let r_o = f_orig(this_o.as_mut_ptr() as usize, 1);
    let r_n = f_new(this_n.as_mut_ptr() as usize, 1);
    crate::diff_eq!(r_o - (node_o.as_ptr() as usize), r_n - (node_n.as_ptr() as usize), "popped head");
    crate::diff_eq!(fl_o[1], fl_n[1], "freelist cleared");
    crate::diff_eq!(node_o, node_n, "node marked allocated");
    Ok(1)
}

unsafe extern "win64" fn mock_reloc_cb(
    ctx: usize,
    old_page: u16,
    old_off: u16,
    sz: u16,
    new_page: u16,
    new_off: u16,
    user: u64,
) {
    let acc = (old_page as u64)
        .wrapping_add(old_off as u64)
        .wrapping_add(sz as u64)
        .wrapping_add(new_page as u64)
        .wrapping_add(new_off as u64)
        .wrapping_add(user);
    wr::<u64>(ctx, rd::<u64>(ctx).wrapping_add(acc));
}

pub unsafe fn diff_d3d_14209df40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16, u16, u16, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut pair_o = [0u8; 0x28];
    let po = pair_o.as_mut_ptr() as usize;
    wr::<u64>(po, 0x100);
    wr::<u16>(po + 0x8, 2);
    wr::<u16>(po + 0xa, 4);
    wr::<u16>(po + 0xc, 1);
    wr::<u8>(po + 0xe, 1);
    let mut pair_n = pair_o;

    let mut cb_sum_o = 0u64;
    let mut cb_sum_n = 0u64;
    let cb_o = [(&mut cb_sum_o as *mut u64) as usize, mock_reloc_cb as *const () as usize];
    let cb_n = [(&mut cb_sum_n as *mut u64) as usize, mock_reloc_cb as *const () as usize];
    let mut cnt_o = 0u64;
    let mut cnt_n = 0u64;
    let mut bytes_o = 0u64;
    let mut bytes_n = 0u64;

    f_orig(
        cb_o.as_ptr() as usize,
        pair_o.as_mut_ptr() as usize,
        9,
        16,
        8,
        (&mut cnt_o as *mut u64) as usize,
        (&mut bytes_o as *mut u64) as usize,
    );
    f_new(
        cb_n.as_ptr() as usize,
        pair_n.as_mut_ptr() as usize,
        9,
        16,
        8,
        (&mut cnt_n as *mut u64) as usize,
        (&mut bytes_n as *mut u64) as usize,
    );
    crate::diff_eq!(pair_o, pair_n, "pair updated");
    crate::diff_eq!(cb_sum_o, cb_sum_n, "cb sum");
    crate::diff_eq!(cnt_o, cnt_n, "cnt");
    crate::diff_eq!(bytes_o, bytes_n, "bytes");
    Ok(1)
}

pub unsafe fn diff_d3d_14209f930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut tex = [0u8; 0x20];
    let tp = tex.as_mut_ptr() as usize;
    wr::<u32>(tp, 16);
    wr::<u32>(tp + 0x4, 16);
    wr::<u8>(tp + 0x18, 1);

    let mut elem = [0u8; 0x2b0];
    let ep = elem.as_mut_ptr() as usize;
    wr::<u64>(ep + 0x10, 10);
    wr::<u64>(ep + 0x70, 20);
    wr::<u64>(ep + 0xd0, 30);

    let mut obj = [0u8; 0x350];
    let op = obj.as_mut_ptr() as usize;
    wr::<u64>(op + 0x10, 100);
    wr::<u64>(op + 0x70, 200);
    wr::<usize>(op + 0xc0, ep);
    wr::<u32>(op + 0xc8, 1);
    wr::<u64>(op + 0xe0, 300);
    wr::<usize>(op + 0x1f0, tp);
    wr::<usize>(op + 0x220, tp);
    wr::<u64>(op + 0x340, 400);

    let r_o = f_orig(op);
    let r_n = f_new(op);
    crate::diff_eq!(r_o, r_n, "total memory sum");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a0310(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut obj_o = [0u8; 0x3d0];
    wr::<u64>(obj_o.as_mut_ptr() as usize, 0x1234);
    wr::<u32>((obj_o.as_mut_ptr() as usize) + 0x50, 9);
    wr::<u64>((obj_o.as_mut_ptr() as usize) + 0x390, 0x5678);
    let mut obj_n = obj_o;

    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);
    crate::diff_eq!(obj_o, obj_n, "destroyed struct");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a0490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut state_o = [0u8; 0x190];
    wr::<u64>((state_o.as_mut_ptr() as usize) + 0x170, 0xcafe_babe);
    let mut state_n = state_o;

    let mut buf_o = [0u8; 0x60];
    wr::<u64>((buf_o.as_mut_ptr() as usize) + 0x10, 0x400);
    wr::<usize>((buf_o.as_mut_ptr() as usize) + 0x58, state_o.as_mut_ptr() as usize);
    let mut buf_n = [0u8; 0x60];
    wr::<u64>((buf_n.as_mut_ptr() as usize) + 0x10, 0x400);
    wr::<usize>((buf_n.as_mut_ptr() as usize) + 0x58, state_n.as_mut_ptr() as usize);

    let mut entries_o = [0u8; 0x28];
    wr::<usize>((entries_o.as_mut_ptr() as usize) + 0x10, buf_o.as_mut_ptr() as usize);
    wr::<u64>((entries_o.as_mut_ptr() as usize) + 0x20, 42);
    let mut entries_n = [0u8; 0x28];
    wr::<usize>((entries_n.as_mut_ptr() as usize) + 0x10, buf_n.as_mut_ptr() as usize);
    wr::<u64>((entries_n.as_mut_ptr() as usize) + 0x20, 42);

    let mut this_o = [0u8; 0x28];
    wr::<usize>((this_o.as_mut_ptr() as usize) + 0x18, entries_o.as_mut_ptr() as usize);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x20, 1);
    let mut this_n = [0u8; 0x28];
    wr::<usize>((this_n.as_mut_ptr() as usize) + 0x18, entries_n.as_mut_ptr() as usize);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x20, 1);

    let mut mapped_o = 0u64;
    let mut mapped_n = 0u64;
    let mut sz_o = 0u64;
    let mut sz_n = 0u64;

    let r_o = f_orig(
        this_o.as_mut_ptr() as usize,
        (&mut mapped_o as *mut u64) as usize,
        (&mut sz_o as *mut u64) as usize,
    );
    let r_n = f_new(
        this_n.as_mut_ptr() as usize,
        (&mut mapped_n as *mut u64) as usize,
        (&mut sz_n as *mut u64) as usize,
    );
    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(mapped_o, mapped_n, "mapped");
    crate::diff_eq!(sz_o, sz_n, "sz");
    crate::diff_eq!(state_o, state_n, "state");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a14d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let a_vals: [u16; 2] = [1, 5];
    let b_vals: [u16; 2] = [2, 8];
    let n0_vals: [u16; 2] = [3, 9];
    let n1_vals: [u16; 2] = [4, 8];

    let span_a = [2u64, a_vals.as_ptr() as u64];
    let span_b = [2u64, b_vals.as_ptr() as u64];
    let mut node = [0u8; 0x28];
    let np = node.as_mut_ptr() as usize;
    wr::<u64>(np + 0x8, 2);
    wr::<u64>(np + 0x10, n0_vals.as_ptr() as u64);
    wr::<u64>(np + 0x18, 2);
    wr::<u64>(np + 0x20, n1_vals.as_ptr() as u64);

    let r1 = f_orig(span_a.as_ptr() as usize, span_b.as_ptr() as usize, np);
    let r2 = f_new(span_a.as_ptr() as usize, span_b.as_ptr() as usize, np);
    crate::diff_eq!(r1, r2, "intersect hit");
    wr::<u64>(np + 0x18, 0);
    let r3 = f_orig(span_a.as_ptr() as usize, span_b.as_ptr() as usize, np);
    let r4 = f_new(span_a.as_ptr() as usize, span_b.as_ptr() as usize, np);
    crate::diff_eq!(r3, r4, "intersect miss");
    Ok(2)
}

pub unsafe fn diff_d3d_1420a28e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let vals_a: [u16; 2] = [3, 7];
    let vals_b: [u16; 2] = [5, 7];
    let mut item = [0u8; 0x48];
    wr::<u64>((item.as_mut_ptr() as usize) + 0x8, 2);
    wr::<u64>((item.as_mut_ptr() as usize) + 0x10, vals_a.as_ptr() as u64);

    let mut r11_o = [0u8; 0x120];
    let p_o = r11_o.as_mut_ptr() as usize;
    wr::<u64>(p_o + 0x28, 1);
    wr::<u32>(p_o + 0x50, 2);
    wr::<u16>(p_o + 0x60, 0xffff);
    wr::<u64>(p_o + 0xc0, 2);
    wr::<u64>(p_o + 0xc8, vals_b.as_ptr() as u64);
    wr::<usize>(p_o + 0xf0, item.as_ptr() as usize);
    wr::<u32>(p_o + 0xf8, 1);
    let mut r11_n = r11_o;

    let this_o = [0usize, 0, r11_o.as_mut_ptr() as usize];
    let this_n = [0usize, 0, r11_n.as_mut_ptr() as usize];
    f_orig(this_o.as_ptr() as usize);
    f_new(this_n.as_ptr() as usize);
    crate::diff_eq!(r11_o, r11_n, "flags updated");
    Ok(1)
}

static mut REM_C_SCRATCH_BUF: [u8; 2048] = [0; 2048];
static mut REM_C_SCRATCH_OFF: usize = 0;

unsafe fn reset_rem_c_scratch() {
    let base = core::ptr::addr_of_mut!(REM_C_SCRATCH_BUF) as *mut u8;
    core::ptr::write_bytes(base, 0, 2048);
    REM_C_SCRATCH_OFF = 0;
}

unsafe extern "win64" fn mock_scratch_alloc_14241c0a0(size: usize, align: usize) -> usize {
    let a = align.max(8);
    let base = core::ptr::addr_of_mut!(REM_C_SCRATCH_BUF) as *mut u8 as usize;
    let cur = (REM_C_SCRATCH_OFF + (a - 1)) & !(a - 1);
    REM_C_SCRATCH_OFF = cur + size;
    base + cur
}

unsafe extern "win64" fn mock_scratch_free_14241c190(_ptr: usize) {}

pub unsafe fn diff_d3d_1420a4240(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m_alloc =
        crate::difftest::Mock::new(0x1_4241_c0a0, mock_scratch_alloc_14241c0a0 as *const ())?;
    let _m_free =
        crate::difftest::Mock::new(0x1_4241_c190, mock_scratch_free_14241c190 as *const ())?;

    let mut other_init = [0u8; 0x60];
    for (i, b) in other_init.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(5).wrapping_add(1);
    }
    let mut other_o = other_init;
    let mut other_n = other_init;
    let mut this_o = [0x5au8; 0x60];
    let mut this_n = [0x5au8; 0x60];

    reset_rem_c_scratch();
    let ro = f_orig(this_o.as_mut_ptr() as usize, other_o.as_mut_ptr() as usize);
    let arena_o = rd::<[u8; 256]>(core::ptr::addr_of!(REM_C_SCRATCH_BUF) as usize);

    reset_rem_c_scratch();
    let rn = f_new(this_n.as_mut_ptr() as usize, other_n.as_mut_ptr() as usize);
    let arena_n = rd::<[u8; 256]>(core::ptr::addr_of!(REM_C_SCRATCH_BUF) as usize);

    crate::diff_eq!(
        ro - (this_o.as_ptr() as usize),
        rn - (this_n.as_ptr() as usize),
        "ret self"
    );
    crate::diff_eq!(this_o, this_n, "moved into this");
    crate::diff_eq!(other_o, other_n, "other reset");
    crate::diff_eq!(arena_o, arena_n, "scratch arena");
    Ok(1)
}

unsafe extern "win64" fn mock_visit_node_1420a4b10(
    state_ptr: usize,
    node_id: u16,
    packed: u32,
) {
    let this = rd::<usize>(state_ptr + 0x8);
    let a4 = rd::<u64>(state_ptr + 0x18);
    let mix = ((node_id as u64) | ((packed as u64) << 16)) ^ a4;
    wr::<u64>(this + 0x10, rd::<u64>(this + 0x10).wrapping_add(mix));
}

pub unsafe fn diff_d3d_1420a4490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _m_alloc =
        crate::difftest::Mock::new(0x1_4241_c0a0, mock_scratch_alloc_14241c0a0 as *const ())?;
    let _m_free =
        crate::difftest::Mock::new(0x1_4241_c190, mock_scratch_free_14241c190 as *const ())?;
    let _m_visit =
        crate::difftest::Mock::new(0x1_420a_4b10, mock_visit_node_1420a4b10 as *const ())?;

    let mut sub_items = [0u8; 0x60];
    let sp = sub_items.as_mut_ptr() as usize;
    wr::<u8>(sp + 0x20, 0);
    wr::<u8>(sp + 0x21, 0);
    wr::<u8>(sp + 0x30 + 0x20, 1);
    wr::<u8>(sp + 0x30 + 0x21, 0);

    let mut ctx = [0u8; 0x20];
    wr::<usize>((ctx.as_mut_ptr() as usize) + 0x10, sp);

    let mut elems = [0u8; 0x80];
    let ep = elems.as_mut_ptr() as usize;
    wr::<u16>(ep + 0x20, 10);
    wr::<u16>(ep + 0x22, 0);
    wr::<u16>(ep + 0x40 + 0x20, 25);
    wr::<u16>(ep + 0x40 + 0x22, 1);
    wr::<u16>(ep + 0x40 + 0x32, 9);
    wr::<u8>(ep + 0x40 + 0x34, 1);
    wr::<u16>(ep + 0x40 + 0x36, 3);
    wr::<u8>(ep + 0x40 + 0x38, 1);

    let mut vec_hdr = [0u8; 0x10];
    wr::<usize>(vec_hdr.as_mut_ptr() as usize, ep);
    wr::<u32>((vec_hdr.as_mut_ptr() as usize) + 0x8, 2);

    let mut this_o = [0u8; 0x60];
    let mut this_n = [0u8; 0x60];

    reset_rem_c_scratch();
    let ro = f_orig(
        this_o.as_mut_ptr() as usize,
        vec_hdr.as_ptr() as usize,
        ctx.as_ptr() as usize,
        0x1234_5678,
    );
    let arena_o = rd::<[u8; 256]>(core::ptr::addr_of!(REM_C_SCRATCH_BUF) as usize);

    reset_rem_c_scratch();
    let rn = f_new(
        this_n.as_mut_ptr() as usize,
        vec_hdr.as_ptr() as usize,
        ctx.as_ptr() as usize,
        0x1234_5678,
    );
    let arena_n = rd::<[u8; 256]>(core::ptr::addr_of!(REM_C_SCRATCH_BUF) as usize);

    crate::diff_eq!(
        ro - (this_o.as_ptr() as usize),
        rn - (this_n.as_ptr() as usize),
        "ret self"
    );
    crate::diff_eq!(this_o, this_n, "this state");
    crate::diff_eq!(arena_o, arena_n, "scratch arena");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a4d90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let table: [u32; 2] = [0, (2u32 << 16) | 0];
    let arr: [u16; 2] = [1, 3];
    let mut ctx = [0u8; 0x60];
    wr::<usize>((ctx.as_mut_ptr() as usize) + 0x40, arr.as_ptr() as usize);
    wr::<usize>((ctx.as_mut_ptr() as usize) + 0x50, table.as_ptr() as usize);

    let mut vis_o = [0u8, 0, 0, 1];
    let mut vis_n = vis_o;
    let mut buf_o = [0u8; 16];
    let mut buf_n = [0u8; 16];

    let mut this_o = [0u8; 0x28];
    let to = this_o.as_mut_ptr() as usize;
    wr::<usize>(to, ctx.as_ptr() as usize);
    wr::<usize>(to + 0x8, vis_o.as_mut_ptr() as usize);
    wr::<usize>(to + 0x18, buf_o.as_mut_ptr() as usize);
    wr::<u32>(to + 0x24, 4);

    let mut this_n = [0u8; 0x28];
    let tn = this_n.as_mut_ptr() as usize;
    wr::<usize>(tn, ctx.as_ptr() as usize);
    wr::<usize>(tn + 0x8, vis_n.as_mut_ptr() as usize);
    wr::<usize>(tn + 0x18, buf_n.as_mut_ptr() as usize);
    wr::<u32>(tn + 0x24, 4);

    f_orig(to, 1);
    f_new(tn, 1);
    crate::diff_eq!(vis_o, vis_n, "visited");
    crate::diff_eq!(rd::<u32>(to + 0x20), rd::<u32>(tn + 0x20), "len");
    crate::diff_eq!(&buf_o[0..3], &buf_n[0..3], "pushed item");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a4ee0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut head = [0usize; 3];
    let hp = head.as_mut_ptr() as usize;
    let mut node = [0u8; 0x18];
    let np = node.as_mut_ptr() as usize;
    wr::<usize>(np, hp);
    wr::<usize>(np + 0x8, hp);
    wr::<u16>(np + 0x10, 5);
    wr::<u16>(np + 0x12, 2);
    let buckets = [np, np];

    let mut ctx = [0u8; 0x40];
    let cp = ctx.as_mut_ptr() as usize;
    wr::<usize>(cp + 0x8, hp);
    wr::<usize>(cp + 0x18, buckets.as_ptr() as usize);
    wr::<u64>(cp + 0x30, 0);

    let mut vis_o = [0u8; 4];
    let mut vis_n = [0u8; 4];
    let mut buf_o = [0u8; 16];
    let mut buf_n = [0u8; 16];

    let mut this_o = [0u8; 0x28];
    let to = this_o.as_mut_ptr() as usize;
    wr::<usize>(to, cp);
    wr::<usize>(to + 0x8, vis_o.as_mut_ptr() as usize);
    wr::<usize>(to + 0x18, buf_o.as_mut_ptr() as usize);
    wr::<u32>(to + 0x24, 4);

    let mut this_n = [0u8; 0x28];
    let tn = this_n.as_mut_ptr() as usize;
    wr::<usize>(tn, cp);
    wr::<usize>(tn + 0x8, vis_n.as_mut_ptr() as usize);
    wr::<usize>(tn + 0x18, buf_n.as_mut_ptr() as usize);
    wr::<u32>(tn + 0x24, 4);

    f_orig(to, 5);
    f_new(tn, 5);
    crate::diff_eq!(vis_o, vis_n, "visited");
    crate::diff_eq!(rd::<u32>(to + 0x20), rd::<u32>(tn + 0x20), "len");
    crate::diff_eq!(&buf_o[0..3], &buf_n[0..3], "pushed item");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a58b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16, u16, u16, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf64_o = [0u64; 2];
    let mut buf64_n = [0u64; 2];
    let mut buf16_o = [0u16; 2];
    let mut buf16_n = [0u16; 2];

    let mut rbx_o = [0u8; 0x20];
    wr::<usize>((rbx_o.as_mut_ptr() as usize) + 0x10, buf64_o.as_mut_ptr() as usize);
    wr::<u32>((rbx_o.as_mut_ptr() as usize) + 0x1c, 2);
    let mut rbx_n = [0u8; 0x20];
    wr::<usize>((rbx_n.as_mut_ptr() as usize) + 0x10, buf64_n.as_mut_ptr() as usize);
    wr::<u32>((rbx_n.as_mut_ptr() as usize) + 0x1c, 2);

    let mut this_o = [0u8; 0x40];
    wr::<usize>((this_o.as_mut_ptr() as usize) + 0x8, rbx_o.as_mut_ptr() as usize);
    wr::<usize>((this_o.as_mut_ptr() as usize) + 0x30, buf16_o.as_mut_ptr() as usize);
    wr::<u32>((this_o.as_mut_ptr() as usize) + 0x3c, 2);
    let mut this_n = [0u8; 0x40];
    wr::<usize>((this_n.as_mut_ptr() as usize) + 0x8, rbx_n.as_mut_ptr() as usize);
    wr::<usize>((this_n.as_mut_ptr() as usize) + 0x30, buf16_n.as_mut_ptr() as usize);
    wr::<u32>((this_n.as_mut_ptr() as usize) + 0x3c, 2);

    let mut range_o: u32 = (5u32 << 16) | 2;
    let mut range_n: u32 = range_o;

    f_orig(
        this_o.as_mut_ptr() as usize,
        (&mut range_o as *mut u32) as usize,
        0x11,
        0x22,
        0x33,
        0x4444_5555,
    );
    f_new(
        this_n.as_mut_ptr() as usize,
        (&mut range_n as *mut u32) as usize,
        0x11,
        0x22,
        0x33,
        0x4444_5555,
    );
    crate::diff_eq!(buf64_o, buf64_n, "buf64");
    crate::diff_eq!(buf16_o, buf16_n, "buf16");
    crate::diff_eq!(range_o, range_n, "range");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a6040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut nodes = [0u8; 0x30];
    wr::<u64>((nodes.as_mut_ptr() as usize) + 0x18, 10);
    let mut ctx0 = [0u8; 0x20];
    wr::<usize>((ctx0.as_mut_ptr() as usize) + 0x10, nodes.as_ptr() as usize);
    wr::<u32>((ctx0.as_mut_ptr() as usize) + 0x18, 1);

    let flags = [0u8, 0, 0, 0];
    let mut out_nodes_o = [0u8; 0x40];
    let mut out_nodes_n = [0u8; 0x40];
    let mut r14_o = [out_nodes_o.as_mut_ptr() as u64, 2u64 << 32];
    let mut r14_n = [out_nodes_n.as_mut_ptr() as u64, 2u64 << 32];
    let mut cnts_o = [0u16; 2];
    let mut cnts_n = [0u16; 2];

    let mut this_o = [0u8; 0x48];
    let to = this_o.as_mut_ptr() as usize;
    wr::<usize>(to, ctx0.as_ptr() as usize);
    wr::<usize>(to + 0x8, r14_o.as_mut_ptr() as usize);
    wr::<usize>(to + 0x10, flags.as_ptr() as usize);
    wr::<usize>(to + 0x20, cnts_o.as_mut_ptr() as usize);
    wr::<u64>(to + 0x40, 5);

    let mut this_n = [0u8; 0x48];
    let tn = this_n.as_mut_ptr() as usize;
    wr::<usize>(tn, ctx0.as_ptr() as usize);
    wr::<usize>(tn + 0x8, r14_n.as_mut_ptr() as usize);
    wr::<usize>(tn + 0x10, flags.as_ptr() as usize);
    wr::<usize>(tn + 0x20, cnts_n.as_mut_ptr() as usize);
    wr::<u64>(tn + 0x40, 5);

    let mut range_o = 0u32;
    let mut range_n = 0u32;
    f_orig(to, (&mut range_o as *mut u32) as usize);
    f_new(tn, (&mut range_n as *mut u32) as usize);

    crate::diff_eq!(range_o, range_n, "range");
    crate::diff_eq!(cnts_o, cnts_n, "cnts");
    crate::diff_eq!(&out_nodes_o[0..0x31], &out_nodes_n[0..0x31], "node head");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a6160(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut nodes = [0u8; 0x30];
    wr::<u64>((nodes.as_mut_ptr() as usize) + 0x10, 0x3);
    wr::<u64>((nodes.as_mut_ptr() as usize) + 0x18, 10);
    let mut ctx0 = [0u8; 0x20];
    wr::<usize>((ctx0.as_mut_ptr() as usize) + 0x10, nodes.as_ptr() as usize);
    wr::<u32>((ctx0.as_mut_ptr() as usize) + 0x18, 1);

    let mut out_nodes_o = [0u8; 0x40];
    let mut out_nodes_n = [0u8; 0x40];
    let mut r14_o = [out_nodes_o.as_mut_ptr() as u64, 2u64 << 32];
    let mut r14_n = [out_nodes_n.as_mut_ptr() as u64, 2u64 << 32];
    let mut cnts_o = [0u16; 2];
    let mut cnts_n = [0u16; 2];

    let mut this_o = [0u8; 0x48];
    let to = this_o.as_mut_ptr() as usize;
    wr::<usize>(to, ctx0.as_ptr() as usize);
    wr::<usize>(to + 0x8, r14_o.as_mut_ptr() as usize);
    wr::<usize>(to + 0x20, cnts_o.as_mut_ptr() as usize);
    wr::<u64>(to + 0x40, 5);

    let mut this_n = [0u8; 0x48];
    let tn = this_n.as_mut_ptr() as usize;
    wr::<usize>(tn, ctx0.as_ptr() as usize);
    wr::<usize>(tn + 0x8, r14_n.as_mut_ptr() as usize);
    wr::<usize>(tn + 0x20, cnts_n.as_mut_ptr() as usize);
    wr::<u64>(tn + 0x40, 5);

    let masks: [u64; 2] = [0x1, 0x4];
    let mut range_o = 0u32;
    let mut range_n = 0u32;
    f_orig(to, (&mut range_o as *mut u32) as usize, masks.as_ptr() as usize);
    f_new(tn, (&mut range_n as *mut u32) as usize, masks.as_ptr() as usize);

    crate::diff_eq!(range_o, range_n, "range");
    crate::diff_eq!(cnts_o, cnts_n, "cnts");
    crate::diff_eq!(&out_nodes_o[0..0x31], &out_nodes_n[0..0x31], "node head");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a63f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut nodes_o = [0u8; 0x60];
    wr::<u64>((nodes_o.as_mut_ptr() as usize) + 0x10, 0x3);
    wr::<u16>((nodes_o.as_mut_ptr() as usize) + 0x22, 7);
    wr::<u16>((nodes_o.as_mut_ptr() as usize) + 0x24, 9);
    let mut nodes_n = nodes_o;

    let mut r11_o = [0u8; 0x20];
    wr::<usize>((r11_o.as_mut_ptr() as usize) + 0x10, nodes_o.as_mut_ptr() as usize);
    wr::<u32>((r11_o.as_mut_ptr() as usize) + 0x18, 1);
    wr::<u32>((r11_o.as_mut_ptr() as usize) + 0x1c, 2);
    let mut r11_n = [0u8; 0x20];
    wr::<usize>((r11_n.as_mut_ptr() as usize) + 0x10, nodes_n.as_mut_ptr() as usize);
    wr::<u32>((r11_n.as_mut_ptr() as usize) + 0x18, 1);
    wr::<u32>((r11_n.as_mut_ptr() as usize) + 0x1c, 2);

    let mut map_arr_o = [0xffffu16; 2];
    let mut map_arr_n = [0xffffu16; 2];
    let map_vec_o = [map_arr_o.as_mut_ptr() as usize];
    let map_vec_n = [map_arr_n.as_mut_ptr() as usize];

    let mut list_arr_o = [0u16; 2];
    let mut list_arr_n = [0u16; 2];
    let mut list_vec_o = [list_arr_o.as_mut_ptr() as u64, 2u64 << 32];
    let mut list_vec_n = [list_arr_n.as_mut_ptr() as u64, 2u64 << 32];

    let mut this_o = [0u8; 0x30];
    let to = this_o.as_mut_ptr() as usize;
    wr::<usize>(to, r11_o.as_mut_ptr() as usize);
    wr::<usize>(to + 0x8, map_vec_o.as_ptr() as usize);
    wr::<usize>(to + 0x10, list_vec_o.as_mut_ptr() as usize);
    wr::<u64>(to + 0x18, 1);
    wr::<u64>(to + 0x20, 4);
    wr::<u64>(to + 0x28, 0x77);

    let mut this_n = [0u8; 0x30];
    let tn = this_n.as_mut_ptr() as usize;
    wr::<usize>(tn, r11_n.as_mut_ptr() as usize);
    wr::<usize>(tn + 0x8, map_vec_n.as_ptr() as usize);
    wr::<usize>(tn + 0x10, list_vec_n.as_mut_ptr() as usize);
    wr::<u64>(tn + 0x18, 1);
    wr::<u64>(tn + 0x20, 4);
    wr::<u64>(tn + 0x28, 0x77);

    let mut out_o = 0u16;
    let mut out_n = 0u16;
    f_orig(to, (&mut out_o as *mut u16) as usize, 0);
    f_new(tn, (&mut out_n as *mut u16) as usize, 0);
    crate::diff_eq!(out_o, out_n, "out_id");
    crate::diff_eq!(map_arr_o, map_arr_n, "map_arr");
    crate::diff_eq!(list_arr_o, list_arr_n, "list_arr");
    crate::diff_eq!(nodes_o, nodes_n, "nodes");
    Ok(1)
}

pub unsafe fn diff_d3d_1420a6b70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u16, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut edges_o = [0u8; 24];
    let mut edges_n = [0u8; 24];
    let mut nodes_o = [0u8; 0x60];
    let mut nodes_n = [0u8; 0x60];

    let mut this_o = [0u8; 0x18];
    let to = this_o.as_mut_ptr() as usize;
    wr::<usize>(to, edges_o.as_mut_ptr() as usize);
    wr::<u32>(to + 0xc, 2);
    wr::<usize>(to + 0x10, nodes_o.as_mut_ptr() as usize);

    let mut this_n = [0u8; 0x18];
    let tn = this_n.as_mut_ptr() as usize;
    wr::<usize>(tn, edges_n.as_mut_ptr() as usize);
    wr::<u32>(tn + 0xc, 2);
    wr::<usize>(tn + 0x10, nodes_n.as_mut_ptr() as usize);

    f_orig(to, 0, 1, 2);
    f_new(tn, 0, 1, 2);
    f_orig(to, 0, 1, 1);
    f_new(tn, 0, 1, 1);

    crate::diff_eq!(edges_o, edges_n, "edges");
    crate::diff_eq!(nodes_o, nodes_n, "nodes");
    Ok(2)
}

// REGISTRY:
// FLOAT_RETURN: none (no function in this batch returns f32/f64 in xmm0).
//
// Replacement { va: 0x14208df00, name: "d3d::d3d_14208df00", ptr: rend::d3d_rem_c::d3d_14208df00 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14208df00) },
// Replacement { va: 0x14208e030, name: "d3d::d3d_14208e030", ptr: rend::d3d_rem_c::d3d_14208e030 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14208e030) },
// Replacement { va: 0x14208e190, name: "d3d::d3d_14208e190", ptr: rend::d3d_rem_c::d3d_14208e190 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14208e190) },
// Replacement { va: 0x14208f090, name: "d3d::d3d_14208f090", ptr: rend::d3d_rem_c::d3d_14208f090 as *const (), status: Status::Verified, throws: Throws::Caught, diff: Some(rend::d3d_rem_c::diff_d3d_14208f090) },
// Replacement { va: 0x14208f220, name: "d3d::d3d_14208f220", ptr: rend::d3d_rem_c::d3d_14208f220 as *const (), status: Status::Verified, throws: Throws::Caught, diff: Some(rend::d3d_rem_c::diff_d3d_14208f220) },
// Replacement { va: 0x142091330, name: "d3d::d3d_142091330", ptr: rend::d3d_rem_c::d3d_142091330 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_142091330) },
// Replacement { va: 0x142091970, name: "d3d::d3d_142091970", ptr: rend::d3d_rem_c::d3d_142091970 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_142091970) },
// Replacement { va: 0x142091f00, name: "d3d::d3d_142091f00", ptr: rend::d3d_rem_c::d3d_142091f00 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_142091f00) },
// Replacement { va: 0x142092b00, name: "d3d::d3d_142092b00", ptr: rend::d3d_rem_c::d3d_142092b00 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_142092b00) },
// Replacement { va: 0x142092ca0, name: "d3d::d3d_142092ca0", ptr: rend::d3d_rem_c::d3d_142092ca0 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_142092ca0) },
// Replacement { va: 0x142092e00, name: "d3d::d3d_142092e00", ptr: rend::d3d_rem_c::d3d_142092e00 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_142092e00) },
// Replacement { va: 0x142092f40, name: "d3d::d3d_142092f40", ptr: rend::d3d_rem_c::d3d_142092f40 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_142092f40) },
// Replacement { va: 0x142093110, name: "d3d::d3d_142093110", ptr: rend::d3d_rem_c::d3d_142093110 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_142093110) },
// Replacement { va: 0x1420933f0, name: "d3d::d3d_1420933f0", ptr: rend::d3d_rem_c::d3d_1420933f0 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_1420933f0) },
// Replacement { va: 0x1420995c0, name: "d3d::d3d_1420995c0", ptr: rend::d3d_rem_c::d3d_1420995c0 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420995c0) },
// Replacement { va: 0x142099ae0, name: "d3d::d3d_142099ae0", ptr: rend::d3d_rem_c::d3d_142099ae0 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_142099ae0) },
// Replacement { va: 0x14209a320, name: "d3d::d3d_14209a320", ptr: rend::d3d_rem_c::d3d_14209a320 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_14209a320) },
// Replacement { va: 0x14209b4a0, name: "d3d::d3d_14209b4a0", ptr: rend::d3d_rem_c::d3d_14209b4a0 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_14209b4a0) },
// Replacement { va: 0x14209b5f0, name: "d3d::d3d_14209b5f0", ptr: rend::d3d_rem_c::d3d_14209b5f0 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209b5f0) },
// Replacement { va: 0x14209b7e0, name: "d3d::d3d_14209b7e0", ptr: rend::d3d_rem_c::d3d_14209b7e0 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_14209b7e0) },
// Replacement { va: 0x14209be60, name: "d3d::d3d_14209be60", ptr: rend::d3d_rem_c::d3d_14209be60 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209be60) },
// Replacement { va: 0x14209bf70, name: "d3d::d3d_14209bf70", ptr: rend::d3d_rem_c::d3d_14209bf70 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209bf70) },
// Replacement { va: 0x14209d160, name: "d3d::d3d_14209d160", ptr: rend::d3d_rem_c::d3d_14209d160 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209d160) },
// Replacement { va: 0x14209d420, name: "d3d::d3d_14209d420", ptr: rend::d3d_rem_c::d3d_14209d420 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209d420) },
// Replacement { va: 0x14209db20, name: "d3d::d3d_14209db20", ptr: rend::d3d_rem_c::d3d_14209db20 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209db20) },
// Replacement { va: 0x14209dc80, name: "d3d::d3d_14209dc80", ptr: rend::d3d_rem_c::d3d_14209dc80 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209dc80) },
// Replacement { va: 0x14209dde0, name: "d3d::d3d_14209dde0", ptr: rend::d3d_rem_c::d3d_14209dde0 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209dde0) },
// Replacement { va: 0x14209df40, name: "d3d::d3d_14209df40", ptr: rend::d3d_rem_c::d3d_14209df40 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209df40) },
// Replacement { va: 0x14209f930, name: "d3d::d3d_14209f930", ptr: rend::d3d_rem_c::d3d_14209f930 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_14209f930) },
// Replacement { va: 0x1420a0310, name: "d3d::d3d_1420a0310", ptr: rend::d3d_rem_c::d3d_1420a0310 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_1420a0310) },
// Replacement { va: 0x1420a0490, name: "d3d::d3d_1420a0490", ptr: rend::d3d_rem_c::d3d_1420a0490 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420a0490) },
// Replacement { va: 0x1420a14d0, name: "d3d::d3d_1420a14d0", ptr: rend::d3d_rem_c::d3d_1420a14d0 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_1420a14d0) },
// Replacement { va: 0x1420a28e0, name: "d3d::d3d_1420a28e0", ptr: rend::d3d_rem_c::d3d_1420a28e0 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_1420a28e0) },
// Replacement { va: 0x1420a4240, name: "d3d::d3d_1420a4240", ptr: rend::d3d_rem_c::d3d_1420a4240 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420a4240) },
// Replacement { va: 0x1420a4490, name: "d3d::d3d_1420a4490", ptr: rend::d3d_rem_c::d3d_1420a4490 as *const (), status: Status::Verified, throws: Throws::Caught, diff: Some(rend::d3d_rem_c::diff_d3d_1420a4490) },
// Replacement { va: 0x1420a4d90, name: "d3d::d3d_1420a4d90", ptr: rend::d3d_rem_c::d3d_1420a4d90 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420a4d90) },
// Replacement { va: 0x1420a4ee0, name: "d3d::d3d_1420a4ee0", ptr: rend::d3d_rem_c::d3d_1420a4ee0 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420a4ee0) },
// Replacement { va: 0x1420a58b0, name: "d3d::d3d_1420a58b0", ptr: rend::d3d_rem_c::d3d_1420a58b0 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420a58b0) },
// Replacement { va: 0x1420a6040, name: "d3d::d3d_1420a6040", ptr: rend::d3d_rem_c::d3d_1420a6040 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420a6040) },
// Replacement { va: 0x1420a6160, name: "d3d::d3d_1420a6160", ptr: rend::d3d_rem_c::d3d_1420a6160 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420a6160) },
// Replacement { va: 0x1420a63f0, name: "d3d::d3d_1420a63f0", ptr: rend::d3d_rem_c::d3d_1420a63f0 as *const (), status: Status::Verified, throws: Throws::Discard, diff: Some(rend::d3d_rem_c::diff_d3d_1420a63f0) },
// Replacement { va: 0x1420a6b70, name: "d3d::d3d_1420a6b70", ptr: rend::d3d_rem_c::d3d_1420a6b70 as *const (), status: Status::Verified, throws: Throws::No, diff: Some(rend::d3d_rem_c::diff_d3d_1420a6b70) },
//
// REGISTRY: Replacement::verified_throws(0x14208f220, "unknown_14208f220", crate::ns::rend::d3d_rem_c::d3d_14208f220 as *const (), Throws::Caught, crate::ns::rend::d3d_rem_c::diff_d3d_14208f220),
// REGISTRY: Replacement::verified(0x142091970, "unknown_142091970", crate::ns::rend::d3d_rem_c::d3d_142091970 as *const (), crate::ns::rend::d3d_rem_c::diff_d3d_142091970),
// REGISTRY: Replacement::verified_throws(0x142091f00, "unknown_142091f00", crate::ns::rend::d3d_rem_c::d3d_142091f00 as *const (), Throws::Discard, crate::ns::rend::d3d_rem_c::diff_d3d_142091f00),
// REGISTRY: Replacement::verified_throws(0x142092b00, "unknown_142092b00", crate::ns::rend::d3d_rem_c::d3d_142092b00 as *const (), Throws::Discard, crate::ns::rend::d3d_rem_c::diff_d3d_142092b00),
// REGISTRY: Replacement::verified_throws(0x142092e00, "unknown_142092e00", crate::ns::rend::d3d_rem_c::d3d_142092e00 as *const (), Throws::Discard, crate::ns::rend::d3d_rem_c::diff_d3d_142092e00),
// REGISTRY: Replacement::verified(0x142092f40, "unknown_142092f40", crate::ns::rend::d3d_rem_c::d3d_142092f40 as *const (), crate::ns::rend::d3d_rem_c::diff_d3d_142092f40),
// REGISTRY: Replacement::verified_throws(0x142093110, "unknown_142093110", crate::ns::rend::d3d_rem_c::d3d_142093110 as *const (), Throws::Discard, crate::ns::rend::d3d_rem_c::diff_d3d_142093110),
// REGISTRY: Replacement::verified_throws(0x1420995c0, "unknown_1420995c0", crate::ns::rend::d3d_rem_c::d3d_1420995c0 as *const (), Throws::Discard, crate::ns::rend::d3d_rem_c::diff_d3d_1420995c0),
// REGISTRY: Replacement::verified_throws(0x14209b4a0, "unknown_14209b4a0", crate::ns::rend::d3d_rem_c::d3d_14209b4a0 as *const (), Throws::Discard, crate::ns::rend::d3d_rem_c::diff_d3d_14209b4a0),
// REGISTRY: Replacement::verified_throws(0x1420a4240, "unknown_1420a4240", crate::ns::rend::d3d_rem_c::d3d_1420a4240 as *const (), Throws::Discard, crate::ns::rend::d3d_rem_c::diff_d3d_1420a4240),
// REGISTRY: Replacement::verified_throws(0x1420a4490, "unknown_1420a4490", crate::ns::rend::d3d_rem_c::d3d_1420a4490 as *const (), Throws::Caught, crate::ns::rend::d3d_rem_c::diff_d3d_1420a4490),

