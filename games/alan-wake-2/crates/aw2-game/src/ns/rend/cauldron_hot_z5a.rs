//! Decompiled `rend` hot functions shard Z5A (`0x14205d410..0x1420be3c0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

/// `d3d_14205d410`
/// Original: 0x14205d410 (437 bytes, Throws::Caught)
pub unsafe extern "win64" fn d3d_14205d410(_param_1: usize, param_2: u32) {
    let mut title_buf = [0u64; 18];
    let mut msg_buf = [0u64; 18];
    let str_title = title_buf.as_mut_ptr() as usize;
    let str_msg = msg_buf.as_mut_ptr() as usize;

    wr::<u64>(str_msg, 0x1000_0010);
    wr::<u8>(str_msg + 0x08, 0);
    wr::<u64>(str_title, 0x1000_0010);
    wr::<u8>(str_title + 0x08, 0);

    let loc1 = exe::call_va!(0x1_420e_af00, () -> usize);
    exe::call_va!(
        0x1_420e_a540,
        (loc1, exe::ptr::<u8>(0x1_433d_c790) as usize, str_title)
    );
    let loc2 = exe::call_va!(0x1_420e_af00, () -> usize);
    exe::call_va!(
        0x1_420e_a540,
        (loc2, exe::ptr::<u8>(0x1_433d_c7a8) as usize, str_msg)
    );

    let c_str = |s: usize| -> usize {
        let hdr = rd::<u32>(s);
        let b3 = hdr >> 24;
        let cap = if b3 < 0x80 {
            b3 * 8
        } else {
            (b3 << 8).wrapping_sub(0x7c00)
        };
        let len_bits = (hdr & 0x00ff_ffff) << 3;
        if len_bits > cap {
            rd::<usize>(s + 0x08)
        } else {
            s + 0x08
        }
    };

    let (proc, exit_code) = if param_2 == 0x8007_000e {
        let title_ptr = c_str(str_title);
        let msg_ptr = c_str(str_msg);
        exe::call_va!(0x1_4205_ce20, (msg_ptr, title_ptr));
        let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
        (proc, 0xdead_d00du32)
    } else {
        let title_ptr = c_str(str_title);
        let msg_ptr = c_str(str_msg);
        if rd::<u8>(exe::ptr::<u8>(0x1_4384_4045) as usize) != 0 {
            exe::call_va!(0x1_423f_2b40, (msg_ptr, title_ptr, 0u32));
        }
        let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
        (proc, 0x0d15_ea5eu32)
    };

    exe::call_iat!(0x1_4264_53a0, (proc, exit_code));
    exe::call_va!(0x1_4240_ab10, (str_title));
    exe::call_va!(0x1_4240_ab10, (str_msg));
}

/// `d3d_142093aa0`
/// Original: 0x142093aa0 (466 bytes, Throws::Caught)
pub unsafe extern "win64" fn d3d_142093aa0(param_1: usize, param_2: u32) {
    let mut desc = [0u64; 2];
    let desc_ptr = desc.as_mut_ptr() as usize;
    wr::<u32>(desc_ptr, rd::<u32>(param_1 + 0x40));
    wr::<u32>(desc_ptr + 0x04, param_2);
    wr::<u64>(desc_ptr + 0x08, 0);

    let mut heap_ptr: usize = 0;
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let dev_vtbl = rd::<usize>(dev);
    let hr = exe::call_va!(
        rd::<usize>(dev_vtbl + 0x70),
        (
            dev,
            desc_ptr,
            exe::ptr::<u8>(0x1_433d_d4c8) as usize,
            (&mut heap_ptr as *mut usize) as usize
        ) -> u32
    );
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }

    let heap = heap_ptr;
    let heap_vtbl = rd::<usize>(heap);
    exe::call_va!(
        rd::<usize>(heap_vtbl + 0x30),
        (heap, exe::ptr::<u8>(0x1_433d_d330) as usize)
    );

    let len = rd::<u32>(param_1 + 0x08) as u64;
    let cap = rd::<u32>(param_1 + 0x0c) as u64;
    if len.wrapping_mul(8) == cap.wrapping_mul(8) {
        let need = len.wrapping_add(1);
        let grow = (cap >> 1).wrapping_add(cap);
        let new_cap = if grow < need { need } else { grow };
        exe::call_va!(0x1_411e_3040, (param_1, new_cap));
    }

    let idx = rd::<u32>(param_1 + 0x08) as usize;
    let arr = rd::<usize>(param_1);
    let slot = arr.wrapping_add(idx * 8);
    let rbx = heap_ptr;
    if rbx != 0 {
        let vtbl = rd::<usize>(rbx);
        exe::call_va!(rd::<usize>(vtbl + 0x08), (rbx));
    }
    wr::<usize>(slot, rbx);
    wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_add(1));

    let mut handle_out: u64 = 0;
    let heap2 = heap_ptr;
    let vtbl2 = rd::<usize>(heap2);
    exe::call_va!(
        rd::<usize>(vtbl2 + 0x48),
        (heap2, (&mut handle_out as *mut u64) as usize)
    );

    let dev2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let dev2_vtbl = rd::<usize>(dev2);
    let step = exe::call_va!(
        rd::<usize>(dev2_vtbl + 0x78),
        (dev2, rd::<u32>(param_1 + 0x40)) -> u32
    );

    let mut offset: u64 = 0;
    if (param_2 as i32) > 0 {
        for _ in 0..param_2 {
            let h = handle_out.wrapping_add(offset);
            let count = rd::<u32>(param_1 + 0x20);
            if count == rd::<u32>(param_1 + 0x24) {
                exe::call_va!(0x1_4034_97a0, (param_1 + 0x10, count.wrapping_add(1)));
            }
            let tail = rd::<u32>(param_1 + 0x1c) as usize;
            let ring_buf = rd::<usize>(param_1 + 0x10);
            wr::<u64>(ring_buf.wrapping_add(tail * 8), h);
            let next_tail = rd::<u32>(param_1 + 0x1c).wrapping_add(1);
            wr::<u32>(param_1 + 0x1c, next_tail);
            if next_tail == rd::<u32>(param_1 + 0x24) {
                wr::<u32>(param_1 + 0x1c, 0);
            }
            wr::<u32>(param_1 + 0x20, rd::<u32>(param_1 + 0x20).wrapping_add(1));
            offset = offset.wrapping_add(step as u64);
        }
    }

    let range_start = handle_out;
    let range_end = range_start.wrapping_add(step.wrapping_mul(param_2) as u64);
    let r_len = rd::<u32>(param_1 + 0x50) as u64;
    let r_cap = rd::<u32>(param_1 + 0x54) as u64;
    if (r_len << 4) == (r_cap << 4) {
        exe::call_va!(0x1_401f_c320, (param_1 + 0x48, r_len.wrapping_add(1)));
    }
    let r_idx = rd::<u32>(param_1 + 0x50) as usize;
    let r_arr = rd::<usize>(param_1 + 0x48);
    wr::<u64>(r_arr.wrapping_add(r_idx * 16), range_start);
    wr::<u64>(r_arr.wrapping_add(r_idx * 16 + 8), range_end);
    wr::<u32>(param_1 + 0x50, rd::<u32>(param_1 + 0x50).wrapping_add(1));

    let heap3 = heap_ptr;
    if heap3 != 0 {
        let vtbl3 = rd::<usize>(heap3);
        exe::call_va!(rd::<usize>(vtbl3 + 0x10), (heap3));
    }
}

unsafe fn lock_recursive(lock_addr: usize) {
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock_addr + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_addr));
        wr::<u32>(lock_addr + 0x08, tid);
    }
    wr::<u32>(lock_addr + 0x0c, rd::<u32>(lock_addr + 0x0c).wrapping_add(1));
}

unsafe fn unlock_recursive(lock_addr: usize) {
    let new_cnt = rd::<u32>(lock_addr + 0x0c).wrapping_sub(1);
    wr::<u32>(lock_addr + 0x0c, new_cnt);
    if new_cnt == 0 {
        wr::<u32>(lock_addr + 0x08, 0xffff_ffff);
        exe::call_iat!(0x1_4264_5598, (lock_addr));
    }
}

unsafe fn alloc_from_pool(lock_addr: usize, pool_addr: usize) -> u64 {
    lock_recursive(lock_addr);
    if rd::<u32>(pool_addr + 0x20) == 0 {
        let chunk = rd::<u32>(pool_addr + 0x44);
        exe::call_va!(0x1_4209_3aa0, (pool_addr, chunk));
    }
    let ring_buf = rd::<usize>(pool_addr + 0x10);
    let cap = rd::<u32>(pool_addr + 0x24) as u64;
    let head = rd::<u32>(pool_addr + 0x18) as u64;
    let idx = (head % cap) as usize;
    let handle = rd::<u64>(ring_buf.wrapping_add(idx * 8));
    let cnt = rd::<u32>(pool_addr + 0x20);
    if cnt != 0 {
        let next_head = (head as u32).wrapping_add(1);
        wr::<u32>(pool_addr + 0x18, next_head);
        if next_head as u64 == cap {
            wr::<u32>(pool_addr + 0x18, 0);
        }
        wr::<u32>(pool_addr + 0x20, cnt.wrapping_sub(1));
    }
    unlock_recursive(lock_addr);
    handle
}

/// `d3d_142094c40`
/// Original: 0x142094c40 (3744 bytes, Throws::Caught)
pub unsafe extern "win64" fn d3d_142094c40(param_1: usize) -> usize {
    exe::call_va!(0x1_4205_6c10, (param_1));
    wr::<usize>(param_1, exe::ptr::<u8>(0x1_433d_d4e0) as usize);
    wr::<u32>(param_1 + 0x18, 0);
    wr::<u8>(param_1 + 0x1c, 0);

    let lock1 = param_1 + 0x20;
    wr::<u32>(lock1 + 0x08, 0xffff_ffff);
    wr::<u32>(lock1 + 0x0c, 0);
    wr::<u64>(lock1, 0);

    let lock2 = param_1 + 0x30;
    wr::<u32>(lock2 + 0x08, 0xffff_ffff);
    wr::<u32>(lock2 + 0x0c, 0);
    wr::<u64>(lock2, 0);

    wr::<[u64; 4]>(param_1 + 0x40, [0; 4]);
    wr::<u64>(param_1 + 0x70, 0);
    wr::<u64>(param_1 + 0x78, 0);

    let init_pool = |base: usize, ty: u32, chunk: u32| {
        wr::<[u64; 8]>(base, [0; 8]);
        wr::<u32>(base + 0x40, ty);
        wr::<u32>(base + 0x44, chunk);
        wr::<u64>(base + 0x48, 0);
        wr::<u64>(base + 0x50, 0);
    };
    init_pool(param_1 + 0xc0, 0, 0x1000);
    init_pool(param_1 + 0x118, 1, 0x80);
    init_pool(param_1 + 0x170, 2, 0x400);
    init_pool(param_1 + 0x1c8, 0, 0x400);
    init_pool(param_1 + 0x220, 3, 0x80);

    wr::<u32>(param_1 + 0x278, 0x40400);
    wr::<u32>(param_1 + 0x288, 0);
    wr::<u64>(param_1 + 0x290, 0);
    wr::<u32>(param_1 + 0x298, 0x10000);
    wr::<u32>(param_1 + 0x2a8, 0);
    wr::<u64>(param_1 + 0x2b0, 0);
    wr::<u64>(param_1 + 0x4b8, 0);

    lock_recursive(lock1);
    lock_recursive(lock2);

    let dev_glob = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let step0_glob = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let step1_glob = exe::ptr::<u32>(0x1_4397_db7c) as usize;
    let iid_ptr = exe::ptr::<u8>(0x1_433d_d4c8) as usize;

    let dev = rd::<usize>(dev_glob);
    let step0 = exe::call_va!(rd::<usize>(rd::<usize>(dev) + 0x78), (dev, 0u32) -> u32);
    wr::<u32>(step0_glob, step0);

    let dev = rd::<usize>(dev_glob);
    let step1 = exe::call_va!(rd::<usize>(rd::<usize>(dev) + 0x78), (dev, 1u32) -> u32);
    wr::<u32>(step1_glob, step1);

    exe::call_va!(0x1_4209_3aa0, (param_1 + 0xc0, 0xc000u32));
    exe::call_va!(0x1_4209_3aa0, (param_1 + 0x118, 0x100u32));
    exe::call_va!(0x1_4209_3aa0, (param_1 + 0x170, 0x800u32));
    exe::call_va!(0x1_4209_3aa0, (param_1 + 0x1c8, 0x800u32));
    exe::call_va!(0x1_4209_3aa0, (param_1 + 0x220, 0x100u32));

    wr::<u64>(param_1 + 0x60, 0);
    wr::<u64>(param_1 + 0x68, 0);
    wr::<u64>(param_1 + 0x08, 0);

    let h80 = alloc_from_pool(lock2, param_1 + 0x118);
    wr::<u64>(param_1 + 0x80, h80);
    let mut sampler_desc = [0u32; 14];
    let sd_ptr = sampler_desc.as_mut_ptr() as usize;
    wr::<[u8; 16]>(
        sd_ptr,
        rd::<[u8; 16]>(exe::ptr::<u8>(0x1_4342_7c90) as usize),
    );
    wr::<u32>(sd_ptr + 0x10, 0);
    wr::<u32>(sd_ptr + 0x14, 0);
    wr::<u32>(sd_ptr + 0x18, 1);
    wr::<[u32; 6]>(sd_ptr + 0x1c, [0; 6]);
    let dev = rd::<usize>(dev_glob);
    exe::call_va!(rd::<usize>(rd::<usize>(dev) + 0xb0), (dev, sd_ptr, h80));

    let h98 = alloc_from_pool(lock2, param_1 + 0xc0);
    wr::<u64>(param_1 + 0x98, h98);
    let mut srv_desc = [0u32; 10];
    let srv_ptr = srv_desc.as_mut_ptr() as usize;
    wr::<u32>(srv_ptr, 0);
    wr::<u32>(srv_ptr + 0x04, 1);
    wr::<u64>(srv_ptr + 0x08, 0x1688);
    wr::<u64>(srv_ptr + 0x10, 0);
    wr::<u32>(srv_ptr + 0x18, 0);
    wr::<u32>(srv_ptr + 0x1c, 4);
    wr::<u64>(srv_ptr + 0x20, 0);
    let dev = rd::<usize>(dev_glob);
    exe::call_va!(
        rd::<usize>(rd::<usize>(dev) + 0x90),
        (dev, 0usize, srv_ptr, h98)
    );

    let h88 = alloc_from_pool(lock2, param_1 + 0xc0);
    wr::<u64>(param_1 + 0x88, h88);
    wr::<u32>(srv_ptr, 0x57);
    wr::<u32>(srv_ptr + 0x04, 4);
    wr::<u32>(srv_ptr + 0x08, 0x1688);
    wr::<u32>(srv_ptr + 0x10, 0);
    wr::<u32>(srv_ptr + 0x14, 1);
    wr::<u32>(srv_ptr + 0x18, 0);
    wr::<u32>(srv_ptr + 0x1c, 0);
    let dev = rd::<usize>(dev_glob);
    exe::call_va!(
        rd::<usize>(rd::<usize>(dev) + 0x90),
        (dev, 0usize, srv_ptr, h88)
    );

    let ha0 = alloc_from_pool(lock2, param_1 + 0x1c8);
    wr::<u64>(param_1 + 0xa0, ha0);
    let mut uav_desc = [0u32; 12];
    let uav_ptr = uav_desc.as_mut_ptr() as usize;
    wr::<u32>(uav_ptr, 0);
    wr::<u32>(uav_ptr + 0x04, 1);
    wr::<u64>(uav_ptr + 0x08, 0);
    wr::<u32>(uav_ptr + 0x10, 0);
    wr::<u32>(uav_ptr + 0x14, 4);
    wr::<u64>(uav_ptr + 0x18, 0);
    wr::<u32>(uav_ptr + 0x20, 0);
    wr::<u64>(uav_ptr + 0x24, 0);
    wr::<u32>(uav_ptr + 0x2c, 0);
    let dev = rd::<usize>(dev_glob);
    exe::call_va!(
        rd::<usize>(rd::<usize>(dev) + 0x98),
        (dev, 0usize, 0usize, uav_ptr, ha0)
    );

    let h90 = alloc_from_pool(lock2, param_1 + 0x1c8);
    wr::<u64>(param_1 + 0x90, h90);
    wr::<u32>(uav_ptr, 0x1c);
    wr::<u64>(uav_ptr + 0x04, 4);
    wr::<u32>(uav_ptr + 0x0c, 0);
    let dev = rd::<usize>(dev_glob);
    exe::call_va!(
        rd::<usize>(rd::<usize>(dev) + 0x98),
        (dev, 0usize, 0usize, uav_ptr, h90)
    );

    let ha8 = alloc_from_pool(lock2, param_1 + 0x1c8);
    wr::<u64>(param_1 + 0xa8, ha8);
    let mut cbv_desc = [0u64; 2];
    let dev = rd::<usize>(dev_glob);
    exe::call_va!(
        rd::<usize>(rd::<usize>(dev) + 0x88),
        (dev, cbv_desc.as_mut_ptr() as usize, ha8)
    );

    let mut heap_desc = [0u32; 4];
    let hd_ptr = heap_desc.as_mut_ptr() as usize;
    wr::<u32>(hd_ptr, 0);
    wr::<u64>(hd_ptr + 0x04, 0x4d);
    wr::<u32>(hd_ptr + 0x0c, 0);
    let dev = rd::<usize>(dev_glob);
    let create_fn = rd::<usize>(rd::<usize>(dev) + 0x70);
    let old_70 = rd::<usize>(param_1 + 0x70);
    if old_70 != 0 {
        exe::call_va!(rd::<usize>(rd::<usize>(old_70) + 0x10), (old_70));
    }
    wr::<usize>(param_1 + 0x70, 0);
    let dev = rd::<usize>(dev_glob);
    let hr = exe::call_va!(create_fn, (dev, hd_ptr, iid_ptr, param_1 + 0x70) -> u32);
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }
    let h_70 = rd::<usize>(param_1 + 0x70);
    exe::call_va!(
        rd::<usize>(rd::<usize>(h_70) + 0x30),
        (h_70, exe::ptr::<u8>(0x1_433d_d3f8) as usize)
    );
    let mut tmp_h: u64 = 0;
    let h_70 = rd::<usize>(param_1 + 0x70);
    let ret_h = exe::call_va!(
        rd::<usize>(rd::<usize>(h_70) + 0x48),
        (h_70, (&mut tmp_h as *mut u64) as usize) -> usize
    );
    wr::<u64>(param_1 + 0xb0, rd::<u64>(ret_h));

    wr::<u32>(hd_ptr, 1);
    wr::<u64>(hd_ptr + 0x04, 4);
    wr::<u32>(hd_ptr + 0x0c, 0);
    let dev = rd::<usize>(dev_glob);
    let create_fn = rd::<usize>(rd::<usize>(dev) + 0x70);
    let old_78 = rd::<usize>(param_1 + 0x78);
    if old_78 != 0 {
        exe::call_va!(rd::<usize>(rd::<usize>(old_78) + 0x10), (old_78));
    }
    wr::<usize>(param_1 + 0x78, 0);
    let dev = rd::<usize>(dev_glob);
    let hr = exe::call_va!(create_fn, (dev, hd_ptr, iid_ptr, param_1 + 0x78) -> u32);
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }
    let h_78 = rd::<usize>(param_1 + 0x78);
    exe::call_va!(
        rd::<usize>(rd::<usize>(h_78) + 0x30),
        (h_78, exe::ptr::<u8>(0x1_433d_d3d0) as usize)
    );
    let h_78 = rd::<usize>(param_1 + 0x78);
    let ret_h = exe::call_va!(
        rd::<usize>(rd::<usize>(h_78) + 0x48),
        (h_78, (&mut tmp_h as *mut u64) as usize) -> usize
    );
    wr::<u64>(param_1 + 0xb8, rd::<u64>(ret_h));

    let mut ebx: u32 = 0;
    while (ebx as i32) < 10 {
        let dev = rd::<usize>(dev_glob);
        let copy_fn = rd::<usize>(rd::<usize>(dev) + 0xc0);
        let step = rd::<u32>(step0_glob) as u64;
        let dst = rd::<u64>(param_1 + 0xb0).wrapping_add((ebx as u64).wrapping_mul(step));
        exe::call_va!(copy_fn, (dev, 1u32, dst, rd::<u64>(param_1 + 0xa8), 0u32));
        ebx = ebx.wrapping_add(1);
    }
    for _ in 0..0x33 {
        let dev = rd::<usize>(dev_glob);
        let copy_fn = rd::<usize>(rd::<usize>(dev) + 0xc0);
        let step = rd::<u32>(step0_glob) as u64;
        let dst = rd::<u64>(param_1 + 0xb0).wrapping_add((ebx as i32 as i64 as u64).wrapping_mul(step));
        exe::call_va!(copy_fn, (dev, 1u32, dst, rd::<u64>(param_1 + 0x88), 0u32));
        ebx = ebx.wrapping_add(1);
    }
    for _ in 0..0x10 {
        let dev = rd::<usize>(dev_glob);
        let copy_fn = rd::<usize>(rd::<usize>(dev) + 0xc0);
        let step = rd::<u32>(step0_glob) as u64;
        let dst = rd::<u64>(param_1 + 0xb0).wrapping_add((ebx as i32 as i64 as u64).wrapping_mul(step));
        exe::call_va!(copy_fn, (dev, 1u32, dst, rd::<u64>(param_1 + 0x90), 0u32));
        ebx = ebx.wrapping_add(1);
    }

    for mult in 0..4u64 {
        let dev = rd::<usize>(dev_glob);
        let copy_fn = rd::<usize>(rd::<usize>(dev) + 0xc0);
        let step = rd::<u32>(step1_glob) as u64;
        let dst = rd::<u64>(param_1 + 0xb8).wrapping_add(step.wrapping_mul(mult));
        exe::call_va!(copy_fn, (dev, 1u32, dst, rd::<u64>(param_1 + 0x80), 1u32));
    }

    wr::<u32>(hd_ptr, 0);
    wr::<u64>(hd_ptr + 0x04, 0x40400);
    wr::<u32>(hd_ptr + 0x0c, 0);
    let dev = rd::<usize>(dev_glob);
    let create_fn = rd::<usize>(rd::<usize>(dev) + 0x70);
    let old_tag = rd::<u32>(param_1 + 0x288);
    let old_290 = rd::<usize>(param_1 + 0x290);
    if old_290 != 0 {
        exe::call_va!(0x1_4209_a1e0, (old_290, old_tag));
    }
    wr::<usize>(param_1 + 0x290, 0);
    wr::<u32>(param_1 + 0x288, old_tag);
    let dev = rd::<usize>(dev_glob);
    let hr = exe::call_va!(create_fn, (dev, hd_ptr, iid_ptr, param_1 + 0x290) -> u32);
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }
    let h_290 = rd::<usize>(param_1 + 0x290);
    exe::call_va!(
        rd::<usize>(rd::<usize>(h_290) + 0x30),
        (h_290, exe::ptr::<u8>(0x1_433d_d458) as usize)
    );
    let h_290 = rd::<usize>(param_1 + 0x290);
    let ret_h = exe::call_va!(
        rd::<usize>(rd::<usize>(h_290) + 0x48),
        (h_290, (&mut tmp_h as *mut u64) as usize) -> usize
    );
    wr::<u64>(param_1 + 0x280, rd::<u64>(ret_h));

    for i in 0..0x40400u32 {
        let dev = rd::<usize>(dev_glob);
        let copy_fn = rd::<usize>(rd::<usize>(dev) + 0xc0);
        let step = rd::<u32>(step0_glob) as u64;
        let dst = rd::<u64>(param_1 + 0x280).wrapping_add((i as i32 as i64 as u64).wrapping_mul(step));
        exe::call_va!(copy_fn, (dev, 1u32, dst, rd::<u64>(param_1 + 0x88), 0u32));
    }

    wr::<u32>(hd_ptr, 0);
    wr::<u64>(hd_ptr + 0x04, 0x10000);
    wr::<u32>(hd_ptr + 0x0c, 0);
    let dev = rd::<usize>(dev_glob);
    let create_fn = rd::<usize>(rd::<usize>(dev) + 0x70);
    let old_tag2 = rd::<u32>(param_1 + 0x2a8);
    let old_2b0 = rd::<usize>(param_1 + 0x2b0);
    if old_2b0 != 0 {
        exe::call_va!(0x1_4209_a1e0, (old_2b0, old_tag2));
    }
    wr::<usize>(param_1 + 0x2b0, 0);
    wr::<u32>(param_1 + 0x2a8, old_tag2);
    let dev = rd::<usize>(dev_glob);
    let hr = exe::call_va!(create_fn, (dev, hd_ptr, iid_ptr, param_1 + 0x2b0) -> u32);
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }
    let h_2b0 = rd::<usize>(param_1 + 0x2b0);
    exe::call_va!(
        rd::<usize>(rd::<usize>(h_2b0) + 0x30),
        (h_2b0, exe::ptr::<u8>(0x1_433d_d420) as usize)
    );
    let h_2b0 = rd::<usize>(param_1 + 0x2b0);
    let ret_h = exe::call_va!(
        rd::<usize>(rd::<usize>(h_2b0) + 0x48),
        (h_2b0, (&mut tmp_h as *mut u64) as usize) -> usize
    );
    wr::<u64>(param_1 + 0x2a0, rd::<u64>(ret_h));

    for i in 0..0x10000u32 {
        let dev = rd::<usize>(dev_glob);
        let copy_fn = rd::<usize>(rd::<usize>(dev) + 0xc0);
        let step = rd::<u32>(step0_glob) as u64;
        let dst = rd::<u64>(param_1 + 0x2a0).wrapping_add((i as i32 as i64 as u64).wrapping_mul(step));
        exe::call_va!(copy_fn, (dev, 1u32, dst, rd::<u64>(param_1 + 0x98), 0u32));
    }

    let null_samp = rd::<u64>(param_1 + 0x80);
    for i in 0..64usize {
        wr::<u64>(param_1 + 0x2b8 + i * 8, null_samp);
    }

    unlock_recursive(lock2);
    unlock_recursive(lock1);
    param_1
}

/// `unknown_1420be3c0`
/// Original: 0x1420be3c0 (3640 bytes, Throws::Caught)
#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn unknown_1420be3c0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: usize,
    param_7: usize,
    param_8: usize,
    param_9: usize,
    param_10: usize,
    param_11: usize,
    param_12: usize,
    param_13: usize,
    param_14: usize,
    param_15: usize,
    param_16: usize,
    param_17: usize,
    param_18: usize,
) -> usize {
    let p14_30 = rd::<usize>(param_14 + 0x30);
    let mut arg5_slot: u64 = 8;

    let mut vec_keys = [0u64; 3];
    let vk_ptr = vec_keys.as_mut_ptr() as usize;
    wr::<usize>(vk_ptr, param_2);
    wr::<u32>(vk_ptr + 0x10, 0);

    let mut vec_groups = [0u64; 3];
    let vg_ptr = vec_groups.as_mut_ptr() as usize;
    wr::<usize>(vg_ptr, param_2);
    wr::<usize>(vg_ptr + 0x08, 0);
    wr::<u64>(vg_ptr + 0x10, 0);

    let mut k_buf = exe::call_va!(0x1_411f_12b0, (param_2, 0usize, 0u64, 0x80u64, 8u64) -> usize);
    if k_buf == 0 {
        k_buf = exe::call_va!(0x1_411f_11e0, (param_2, 0x80u64, 8u32) -> usize);
    }
    wr::<u32>(vk_ptr + 0x14, 0x10);
    wr::<usize>(vk_ptr + 0x08, k_buf);

    exe::call_va!(0x1_411f_2a80, (vg_ptr));
    wr::<u32>(vg_ptr + 0x14, 0x10);

    let mut keys_data = k_buf;
    let mut keys_len: u32 = 0;
    let mut groups_data = rd::<usize>(vg_ptr + 0x08);

    if rd::<u64>(param_3) != 0 {
        let arr3 = rd::<usize>(param_3 + 0x08);
        let mut i: usize = 0;
        loop {
            let elem = rd::<u64>(arr3 + i * 8);
            if ((elem >> 32) as u16) == 0xffff && (elem as u32) != u32::MAX {
                let key = rd::<u64>(rd::<usize>(param_4 + 0x08) + i * 16 + 0x08);
                let pos = exe::call_va!(
                    0x1_411f_1c40,
                    (keys_data, keys_len as u64, key) -> u64
                );
                let val = rd::<u32>(rd::<usize>(param_5 + 0x08) + i * 4);
                if pos < (keys_len as u64) && rd::<u64>(keys_data + (pos as usize) * 8) == key {
                    let grp = groups_data + (pos as usize) * 0x50;
                    let g_len = rd::<u32>(grp + 0x10);
                    let g_cap = rd::<u32>(grp + 0x14);
                    if g_len == g_cap {
                        let need = (g_len.wrapping_add(1)) as u64;
                        let grow = (g_cap as u64).wrapping_mul(3);
                        let new_cap = if grow < need { need } else { grow };
                        exe::call_va!(0x1_411f_2490, (grp, new_cap));
                    }
                    let idx = rd::<u32>(grp + 0x10);
                    wr::<u32>(grp + 0x10, idx.wrapping_add(1));
                    wr::<u32>(rd::<usize>(grp + 0x08) + (idx as usize) * 4, val);
                } else {
                    let mut key_local: u64 = key;
                    exe::call_va!(
                        0x1_411f_2180,
                        (vk_ptr, pos, (&mut key_local as *mut u64) as usize)
                    );
                    let mut new_grp = [0u64; 10];
                    let ng_ptr = new_grp.as_mut_ptr() as usize;
                    wr::<usize>(ng_ptr, param_2);
                    wr::<u32>(ng_ptr + 0x10, 1);
                    wr::<u32>(ng_ptr + 0x14, 1);
                    let sub_buf = exe::call_va!(0x1_411f_11e0, (param_2, 4u64, 4u32) -> usize);
                    wr::<usize>(ng_ptr + 0x08, sub_buf);
                    wr::<u32>(sub_buf, val);
                    wr::<usize>(ng_ptr + 0x18, param_2);
                    wr::<u64>(ng_ptr + 0x20, 0);
                    wr::<u64>(ng_ptr + 0x28, 0);
                    wr::<usize>(ng_ptr + 0x30, param_2);
                    wr::<u64>(ng_ptr + 0x38, 0);
                    wr::<u64>(ng_ptr + 0x40, 0);
                    wr::<u16>(ng_ptr + 0x48, (elem >> 48) as u16);
                    exe::call_va!(0x1_411f_1750, (vg_ptr, pos, ng_ptr));
                    keys_len = rd::<u32>(vk_ptr + 0x10);
                    keys_data = rd::<usize>(vk_ptr + 0x08);
                    groups_data = rd::<usize>(vg_ptr + 0x08);
                }
            }
            i += 1;
            if (i as u64) == rd::<u64>(param_3) {
                break;
            }
        }
    }

    let count12 = rd::<u32>(param_12 + 0x68);
    if count12 != 0 {
        let arr7 = rd::<usize>(param_7 + 0x08);
        for j in 0..count12 {
            if rd::<u16>(arr7 + (j as usize) * 2) == 0xffff {
                let key = rd::<u64>(rd::<usize>(param_6 + 0x08) + (j as usize) * 8);
                let pos = exe::call_va!(
                    0x1_411f_1c40,
                    (keys_data, keys_len as u64, key) -> u64
                );
                if pos < (keys_len as u64) && rd::<u64>(keys_data + (pos as usize) * 8) == key {
                    let grp = groups_data + (pos as usize) * 0x50;
                    let mut g2_len = rd::<u32>(grp + 0x28);
                    let g2_cap = rd::<u32>(grp + 0x2c);
                    if g2_len == g2_cap {
                        let need = (g2_len.wrapping_add(1)) as u64;
                        let grow = (g2_cap as u64).wrapping_mul(3);
                        let new_cap = if grow < need { need } else { grow };
                        arg5_slot = 4;
                        let mut new_buf = exe::call_va!(
                            0x1_411f_12b0,
                            (
                                rd::<usize>(grp + 0x18),
                                rd::<usize>(grp + 0x20),
                                (g2_cap as u64) * 4,
                                new_cap * 4,
                                4u64
                            ) -> usize
                        );
                        if new_buf == 0 {
                            new_buf = exe::call_va!(
                                0x1_411f_11e0,
                                (rd::<usize>(grp + 0x18), new_cap * 4, 4u32) -> usize
                            );
                            let cur_cnt = rd::<u32>(grp + 0x28);
                            let old_buf = rd::<usize>(grp + 0x20);
                            if cur_cnt != 0 {
                                for k in 0..(cur_cnt as usize) {
                                    wr::<u32>(new_buf + k * 4, rd::<u32>(old_buf + k * 4));
                                }
                                g2_len = rd::<u32>(grp + 0x28);
                            } else {
                                g2_len = 0;
                            }
                        } else {
                            g2_len = rd::<u32>(grp + 0x28);
                        }
                        wr::<usize>(grp + 0x20, new_buf);
                        wr::<u32>(grp + 0x2c, new_cap as u32);
                    }
                    wr::<u32>(grp + 0x28, g2_len.wrapping_add(1));
                    wr::<u32>(rd::<usize>(grp + 0x20) + (g2_len as usize) * 4, j);
                } else {
                    let mut key_local: u64 = key;
                    exe::call_va!(
                        0x1_411f_2180,
                        (vk_ptr, pos, (&mut key_local as *mut u64) as usize)
                    );
                    let mut new_grp = [0u64; 10];
                    let ng_ptr = new_grp.as_mut_ptr() as usize;
                    wr::<usize>(ng_ptr, param_2);
                    wr::<u64>(ng_ptr + 0x08, 0);
                    wr::<u64>(ng_ptr + 0x10, 0);
                    wr::<usize>(ng_ptr + 0x18, param_2);
                    wr::<u32>(ng_ptr + 0x28, 1);
                    wr::<u32>(ng_ptr + 0x2c, 1);
                    let sub_buf = exe::call_va!(0x1_411f_11e0, (param_2, 4u64, 4u32) -> usize);
                    wr::<usize>(ng_ptr + 0x20, sub_buf);
                    wr::<u32>(sub_buf, j);
                    wr::<usize>(ng_ptr + 0x30, param_2);
                    wr::<u64>(ng_ptr + 0x38, 0);
                    wr::<u64>(ng_ptr + 0x40, 0);
                    wr::<u16>(ng_ptr + 0x48, 0xffff);
                    exe::call_va!(0x1_411f_1750, (vg_ptr, pos, ng_ptr));
                    keys_len = rd::<u32>(vk_ptr + 0x10);
                    keys_data = rd::<usize>(vk_ptr + 0x08);
                    groups_data = rd::<usize>(vg_ptr + 0x08);
                }
            }
        }
    }

    exe::call_va!(0x1_411f_13d0, (param_10));
    let groups_len = rd::<u32>(vg_ptr + 0x10) as usize;
    let groups_end = groups_data + groups_len * 0x50;
    let mut grp_cur = groups_data;
    while grp_cur != groups_end {
        if rd::<u16>(grp_cur + 0x48) == 0xffff {
            let mut span8 = rd::<[u64; 2]>(param_8);
            let first_idx = rd::<u32>(rd::<usize>(grp_cur + 0x20));
            exe::call_va!(
                0x1_420c_0720,
                (first_idx, grp_cur, span8.as_mut_ptr() as usize)
            );
        } else {
            let mut span9 = rd::<[u64; 2]>(param_9);
            let mut span10: [u64; 2] = [
                rd::<u32>(param_10 + 0x10) as u64,
                rd::<u64>(param_10 + 0x08),
            ];
            let mut span11 = rd::<[u64; 2]>(param_11);
            let first_idx = rd::<u32>(rd::<usize>(grp_cur + 0x08));
            exe::call_va!(
                0x1_420c_0190,
                (
                    param_2,
                    first_idx,
                    param_13,
                    grp_cur,
                    arg5_slot,
                    span9.as_mut_ptr() as usize,
                    span10.as_mut_ptr() as usize,
                    span11.as_mut_ptr() as usize
                )
            );
        }
        grp_cur += 0x50;
    }

    let keys_cnt = keys_len as usize;
    let arr_u16_a = exe::call_va!(0x1_411f_11e0, (param_2, (keys_cnt * 2) as u64, 2u32) -> usize);
    for i in 0..keys_cnt {
        wr::<u16>(arr_u16_a + i * 2, 0xffff);
    }

    let mut vec_pairs = [0u64; 3];
    let vp_ptr = vec_pairs.as_mut_ptr() as usize;
    wr::<usize>(vp_ptr, param_2);
    wr::<usize>(vp_ptr + 0x08, 0);
    wr::<u32>(vp_ptr + 0x10, 0);
    wr::<u32>(vp_ptr + 0x14, 0);
    if groups_len != 0 {
        exe::call_va!(0x1_411f_2eb0, (vp_ptr, groups_len as u64));
    }
    wr::<u32>(vp_ptr + 0x14, groups_len as u32);

    let mut grp_cur2 = groups_data;
    while grp_cur2 != groups_end {
        let w0 = rd::<u32>(grp_cur2 + 0x40) as u64;
        let w1 = rd::<u64>(grp_cur2 + 0x38);
        let p_len = rd::<u32>(vp_ptr + 0x10);
        let p_cap = rd::<u32>(vp_ptr + 0x14);
        if p_len == p_cap {
            let need = (p_len.wrapping_add(1)) as u64;
            let grow = (p_cap as u64).wrapping_mul(3);
            let new_cap = if grow < need { need } else { grow };
            exe::call_va!(0x1_411f_2eb0, (vp_ptr, new_cap));
        }
        let idx = rd::<u32>(vp_ptr + 0x10) as usize;
        let p_data = rd::<usize>(vp_ptr + 0x08);
        wr::<u32>(vp_ptr + 0x10, (idx + 1) as u32);
        wr::<u64>(p_data + idx * 16, w0);
        wr::<u64>(p_data + idx * 16 + 8, w1);
        grp_cur2 += 0x50;
    }

    let mut vec_u16_b = [0u64; 3];
    let vb_ptr = vec_u16_b.as_mut_ptr() as usize;
    wr::<usize>(vb_ptr, param_2);
    wr::<u32>(vb_ptr + 0x10, keys_cnt as u32);
    wr::<u32>(vb_ptr + 0x14, keys_cnt as u32);
    let arr_u16_b = exe::call_va!(0x1_411f_11e0, (param_2, (keys_cnt * 2) as u64, 2u32) -> usize);
    wr::<usize>(vb_ptr + 0x08, arr_u16_b);
    for i in 0..keys_cnt {
        wr::<u16>(arr_u16_b + i * 2, 0xffff);
    }

    let srw_lock = param_13 + 0x10;
    exe::call_iat!(0x1_4264_55a8, (srw_lock));
    let init_p13_8 = rd::<u64>(param_13 + 0x08);
    let mut cur_p13_8 = init_p13_8;
    let mut new_cnt: u16 = 0;
    let mut keys_cnt_local = keys_cnt as u32;
    if keys_cnt != 0 {
        let pairs_data = rd::<usize>(vp_ptr + 0x08);
        for idx in 0..keys_cnt {
            let key = rd::<u64>(keys_data + idx * 8);
            let found_idx = exe::call_va!(
                0x1_4248_9da0,
                (rd::<usize>(param_13 + 0x18), rd::<u32>(param_13 + 0x20), key) -> i32
            );
            if found_idx != -1 {
                let existing =
                    rd::<u16>(rd::<usize>(param_13 + 0x28) + (found_idx as isize as usize) * 2);
                wr::<u16>(arr_u16_a + idx * 2, existing);
            } else {
                let mut pair_copy = rd::<[u64; 2]>(pairs_data + idx * 16);
                let out_u16_ptr = exe::call_va!(
                    0x1_420b_3040,
                    (
                        param_13,
                        (&mut keys_cnt_local as *mut u32) as usize,
                        pair_copy.as_mut_ptr() as usize,
                        key,
                        (&mut cur_p13_8 as *mut u64) as usize
                    ) -> usize
                );
                let val_u16 = rd::<u16>(out_u16_ptr);
                wr::<u16>(arr_u16_a + idx * 2, val_u16);
                wr::<u16>(
                    rd::<usize>(vb_ptr + 0x08) + (new_cnt as usize) * 2,
                    val_u16,
                );
                new_cnt = new_cnt.wrapping_add(1);
            }
        }
    }
    let delta_p13_8 = cur_p13_8.wrapping_sub(init_p13_8);
    let _ = crate::simd_ops::atomic_add_u64((param_13 + 0x08) as u64, delta_p13_8);
    exe::call_iat!(0x1_4264_5598, (srw_lock));

    exe::call_va!(0x1_411f_15a0, (vb_ptr, new_cnt as u32));
    let b_len = rd::<u32>(vb_ptr + 0x10) as usize;
    let b_beg = rd::<usize>(vb_ptr + 0x08);
    let b_end = b_beg + b_len * 2;
    let count_i64 = ((b_end.wrapping_sub(b_beg)) as i64) >> 1;
    let mut copy_ptr: usize = 0;
    let mut copy_cap: u32 = 0;
    if count_i64 > 0 {
        copy_ptr = exe::call_va!(0x1_4028_e1c0, (vb_ptr, count_i64 as u64) -> usize);
        copy_cap = count_i64 as u32;
    }
    let mut src_u16 = b_beg;
    let mut dst_u16 = copy_ptr;
    while src_u16 != b_end {
        wr::<u16>(dst_u16, rd::<u16>(src_u16));
        dst_u16 += 2;
        src_u16 += 2;
    }

    let mut job_arg = [0u64; 5];
    let ja_ptr = job_arg.as_mut_ptr() as usize;
    wr::<usize>(ja_ptr, param_15);
    wr::<usize>(ja_ptr + 0x08, param_14);
    wr::<usize>(ja_ptr + 0x10, copy_ptr);
    wr::<u32>(ja_ptr + 0x18, count_i64 as u32);
    wr::<u32>(ja_ptr + 0x1c, copy_cap);
    wr::<u64>(ja_ptr + 0x20, delta_p13_8);

    let mut job_handle = [0u64; 2];
    let job_mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    exe::call_va!(
        0x1_411f_3720,
        (
            job_mgr,
            job_handle.as_mut_ptr() as usize,
            copy_ptr,
            ja_ptr
        )
    );
    if rd::<u32>(job_handle[0] as usize) == 1 {
        let q = rd::<usize>(job_mgr + 0x10);
        exe::call_va!(0x1_4248_6460, (q, job_handle[0] as usize));
        let sem = rd::<usize>(rd::<usize>(q) + 0x1008);
        exe::call_iat!(0x1_4264_5580, (sem, 1u32, 0usize));
    }
    wr::<[u64; 2]>(param_18, job_handle);

    exe::call_va!(0x1_411f_1a70, (param_17, keys_cnt as u64));
    let mut total_items: u64 = 0;
    if keys_cnt != 0 {
        for idx in 0..keys_cnt {
            let grp = groups_data + idx * 0x50;
            let u16_val = rd::<u16>(arr_u16_a + idx * 2);
            let item_cnt = rd::<u32>(grp + 0x28).wrapping_add(rd::<u32>(grp + 0x10));
            total_items = total_items.wrapping_add(item_cnt as u64);
            let mut tmp_30 = [0u64; 6];
            let src_ptr = exe::call_va!(
                0x1_420c_2d60,
                (
                    tmp_30.as_mut_ptr() as usize,
                    param_2,
                    rd::<u64>(param_14),
                    param_14,
                    u16_val,
                    item_cnt
                ) -> usize
            );
            let dst_ptr = rd::<usize>(param_17 + 0x08) + idx * 0x30;
            wr::<u16>(dst_ptr, rd::<u16>(src_ptr));
            wr::<u64>(dst_ptr + 0x08, rd::<u64>(src_ptr + 0x08));
            if src_ptr + 0x10 != dst_ptr + 0x10 {
                wr::<u64>(dst_ptr + 0x10, rd::<u64>(src_ptr + 0x10));
                wr::<u64>(dst_ptr + 0x18, rd::<u64>(src_ptr + 0x18));
                wr::<u32>(dst_ptr + 0x20, rd::<u32>(src_ptr + 0x20));
                wr::<u32>(dst_ptr + 0x24, rd::<u32>(src_ptr + 0x24));
                wr::<u64>(src_ptr + 0x18, 0);
                wr::<u64>(src_ptr + 0x20, 0);
            }
            wr::<u32>(dst_ptr + 0x28, rd::<u32>(src_ptr + 0x28));
        }
    }

    let mut vec_packed = [0u64; 3];
    let vpk_ptr = vec_packed.as_mut_ptr() as usize;
    wr::<usize>(vpk_ptr, param_2);
    wr::<usize>(vpk_ptr + 0x08, 0);
    wr::<u32>(vpk_ptr + 0x10, 0);
    if total_items != 0 {
        let mut buf = exe::call_va!(0x1_411f_11e0, (param_2, total_items * 8, 8u32) -> usize);
        if buf == 0 {
            buf = exe::call_va!(0x1_411f_11e0, (param_2, total_items * 8, 8u32) -> usize);
        }
        wr::<usize>(vpk_ptr + 0x08, buf);
    }
    wr::<u32>(vpk_ptr + 0x14, total_items as u32);

    if keys_cnt != 0 {
        for grp_idx in 0..keys_cnt {
            let grp = groups_data + grp_idx * 0x50;
            let mut cur_u32 = rd::<usize>(grp + 0x08);
            let end_u32 = cur_u32 + (rd::<u32>(grp + 0x10) as usize) * 4;
            while cur_u32 != end_u32 {
                let idx_val = rd::<u32>(cur_u32) as usize;
                let entry = rd::<u64>(p14_30 + idx_val * 8);
                let packed = ((entry >> 32) & 0xffff_ffff)
                    | (((grp_idx as u64) & 0xffff) << 32)
                    | ((entry & 0xffff) << 48);
                let v_len = rd::<u32>(vpk_ptr + 0x10);
                let v_cap = rd::<u32>(vpk_ptr + 0x14);
                if v_len == v_cap {
                    let need = (v_len.wrapping_add(1)) as u64;
                    let grow = (v_cap as u64).wrapping_mul(3);
                    let new_cap = if grow < need { need } else { grow };
                    exe::call_va!(0x1_411f_2570, (vpk_ptr, new_cap));
                }
                let pos_idx = rd::<u32>(vpk_ptr + 0x10) as usize;
                let v_data = rd::<usize>(vpk_ptr + 0x08);
                wr::<u32>(vpk_ptr + 0x10, (pos_idx + 1) as u32);
                wr::<u64>(v_data + pos_idx * 8, packed);
                cur_u32 += 4;
            }
        }
    }

    let sorted_len = rd::<u32>(vpk_ptr + 0x10);
    let sorted_data = rd::<usize>(vpk_ptr + 0x08);
    exe::call_va!(0x1_40fa_e550, (sorted_data, sorted_len));

    let p17_data = rd::<usize>(param_17 + 0x08);
    let mut indices_buf: usize = 0;
    let mut indices_cap: u32 = 0;
    if sorted_len != 0 {
        indices_buf = exe::call_va!(0x1_411f_11e0, (param_2, (sorted_len as u64) * 4, 4u32) -> usize);
        if indices_buf == 0 {
            indices_buf =
                exe::call_va!(0x1_411f_11e0, (param_2, (sorted_len as u64) * 4, 4u32) -> usize);
        }
        indices_cap = sorted_len;
        for k in 0..(sorted_len as usize) {
            wr::<u32>(indices_buf + k * 4, 0);
        }
        for k in 0..(sorted_len as usize) {
            let packed = rd::<u64>(sorted_data + k * 8);
            let g_idx = (packed >> 32) as u16;
            let out_val = if g_idx == 0xffff {
                u32::MAX
            } else {
                let addr = p17_data + (g_idx as usize) * 0x30 + 0x28;
                let cur = rd::<u32>(addr);
                wr::<u32>(addr, cur.wrapping_add(1));
                cur
            };
            wr::<u32>(indices_buf + k * 4, out_val);
        }
    }

    let mut span_packed: [u64; 2] = [
        rd::<u32>(vpk_ptr + 0x10) as u64,
        rd::<usize>(vpk_ptr + 0x08) as u64,
    ];
    let mut span_p13: [u64; 2] = [rd::<u64>(param_13 + 0x08), (param_13 + 0x48) as u64];
    let mut span_indices: [u64; 2] = [sorted_len as u64, indices_buf as u64];
    let mut span_p17: [u64; 2] = [
        rd::<u32>(param_17 + 0x10) as u64,
        rd::<u64>(param_17 + 0x08),
    ];
    let mut tmp_c2600 = [0u64; 14];
    exe::call_va!(
        0x1_420c_2600,
        (
            tmp_c2600.as_mut_ptr() as usize,
            param_2,
            span_packed.as_mut_ptr() as usize,
            span_p13.as_mut_ptr() as usize,
            param_13,
            rd::<u64>(param_14),
            span_indices.as_mut_ptr() as usize,
            span_p17.as_mut_ptr() as usize
        )
    );
    exe::call_va!(0x1_420c_0e20, (param_16, tmp_c2600.as_mut_ptr() as usize));

    let mut span_groups: [u64; 2] = [
        rd::<u32>(vg_ptr + 0x10) as u64,
        rd::<usize>(vg_ptr + 0x08) as u64,
    ];
    let mut span_packed2: [u64; 2] = [span_packed[0], span_packed[1]];
    let mut span_indices2: [u64; 2] = [sorted_len as u64, indices_buf as u64];
    let mut span_p17_2: [u64; 2] = [
        rd::<u32>(param_17 + 0x10) as u64,
        rd::<u64>(param_17 + 0x08),
    ];
    let mut tmp_bf7c0 = [0u64; 3];
    exe::call_va!(
        0x1_420b_f7c0,
        (
            tmp_bf7c0.as_mut_ptr() as usize,
            param_2,
            span_groups.as_mut_ptr() as usize,
            span_packed2.as_mut_ptr() as usize,
            param_13,
            param_12,
            span_indices2.as_mut_ptr() as usize,
            span_p17_2.as_mut_ptr() as usize,
            rd::<u64>(param_14)
        )
    );

    wr::<usize>(param_1, rd::<usize>(vg_ptr));
    wr::<usize>(param_1 + 0x08, span_groups[1] as usize);
    wr::<u32>(param_1 + 0x10, span_groups[0] as u32);
    wr::<u32>(param_1 + 0x14, rd::<u32>(vg_ptr + 0x14));
    wr::<usize>(param_1 + 0x18, param_2);
    wr::<usize>(param_1 + 0x20, arr_u16_a);
    wr::<u32>(param_1 + 0x28, keys_cnt as u32);
    wr::<u32>(param_1 + 0x2c, keys_cnt as u32);
    wr::<usize>(param_1 + 0x30, rd::<usize>(vpk_ptr));
    wr::<usize>(param_1 + 0x38, span_packed[1] as usize);
    wr::<u32>(param_1 + 0x40, span_packed[0] as u32);
    wr::<u32>(param_1 + 0x44, rd::<u32>(vpk_ptr + 0x14));
    wr::<usize>(param_1 + 0x48, param_2);
    wr::<usize>(param_1 + 0x50, indices_buf);
    wr::<u32>(param_1 + 0x58, sorted_len);
    wr::<u32>(param_1 + 0x5c, indices_cap);
    wr::<[u64; 3]>(param_1 + 0x60, tmp_bf7c0);
    param_1
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_d3d_14205d410(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG: (u32, u32, u32, usize, u32, u32) = (0, 0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_af00() -> usize {
        0x4444_0000
    }
    unsafe extern "win64" fn mock_a540(_loc: usize, key: usize, out_str: usize) {
        let tag = rd::<u32>(key);
        wr::<u32>(out_str, 0x1000_0004);
        wr::<u32>(out_str + 0x08, tag);
    }
    unsafe extern "win64" fn mock_ce20(msg: usize, title: usize) {
        LOG.0 = rd::<u32>(msg);
        LOG.1 = rd::<u32>(title);
    }
    unsafe extern "win64" fn mock_2b40(msg: usize, title: usize, flags: u32) {
        LOG.0 = rd::<u32>(msg);
        LOG.1 = rd::<u32>(title);
        LOG.2 = flags.wrapping_add(1);
    }
    unsafe extern "win64" fn mock_ab10(s: usize) {
        LOG.5 = LOG.5.wrapping_add(rd::<u32>(s + 0x08));
    }
    unsafe extern "win64" fn mock_get_proc() -> usize {
        0x7777
    }
    unsafe extern "win64" fn mock_term_proc(proc: usize, code: u32) {
        LOG.3 = proc;
        LOG.4 = code;
    }

    let _m1 = crate::difftest::Mock::new(0x1_420e_af00, mock_af00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420e_a540, mock_a540 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4205_ce20, mock_ce20 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_423f_2b40, mock_2b40 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _i1 = crate::difftest::IatMock::new(0x1_4264_56c8, mock_get_proc as *const ())?;
    let _i2 = crate::difftest::IatMock::new(0x1_4264_53a0, mock_term_proc as *const ())?;

    let flag_addr = exe::ptr::<u8>(0x1_4384_4045) as usize;
    let saved_flag = rd::<u8>(flag_addr);

    let run = |f: F, hr: u32, show_dlg: u8| -> (u32, u32, u32, usize, u32, u32) {
        LOG = (0, 0, 0, 0, 0, 0);
        wr::<u8>(flag_addr, show_dlg);
        f(0, hr);
        wr::<u8>(flag_addr, saved_flag);
        LOG
    };

    for (hr, flag) in [(0x8007_000eu32, 0u8), (0x887a_0005, 1), (0x887a_0005, 0)] {
        let o = run(f_o, hr, flag);
        let n = run(f_n, hr, flag);
        crate::diff_eq!(o, n, "14205d410 hr={:#x} flag={}", hr, flag);
    }
    Ok(3)
}

pub unsafe fn diff_d3d_142093aa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HEAP_CALLS: (u32, u32, u32, u32) = (0, 0, 0, 0);
    static mut FAKE_HEAP_OBJ: usize = 0;

    unsafe extern "win64" fn mock_heap_addref(_this: usize) -> u32 {
        HEAP_CALLS.0 += 1;
        2
    }
    unsafe extern "win64" fn mock_heap_release(_this: usize) -> u32 {
        HEAP_CALLS.1 += 1;
        1
    }
    unsafe extern "win64" fn mock_heap_setname(_this: usize, name: usize) -> u32 {
        HEAP_CALLS.2 = rd::<u16>(name) as u32;
        0
    }
    unsafe extern "win64" fn mock_heap_getcpu(_this: usize, out: usize) -> usize {
        wr::<u64>(out, 0x5000_0000);
        out
    }
    unsafe extern "win64" fn mock_dev_create_heap(
        _dev: usize,
        desc: usize,
        _iid: usize,
        out_heap: usize,
    ) -> u32 {
        HEAP_CALLS.3 = rd::<u32>(desc) ^ (rd::<u32>(desc + 0x04) << 8);
        wr::<usize>(out_heap, FAKE_HEAP_OBJ);
        0
    }
    unsafe extern "win64" fn mock_dev_get_inc(_dev: usize, ty: u32) -> u32 {
        (ty + 1) * 32
    }
    unsafe extern "win64" fn mock_grow_heaps(_vec: usize, new_cap: u64) {
        wr::<u32>(_vec + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_grow_ring(ring: usize, new_cap: u32) {
        wr::<u32>(ring + 0x14, new_cap.max(8));
    }
    unsafe extern "win64" fn mock_grow_ranges(vec: usize, new_cap: u64) {
        wr::<u32>(vec + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411e_3040, mock_grow_heaps as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4034_97a0, mock_grow_ring as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401f_c320, mock_grow_ranges as *const ())?;

    let mut heap_vtbl = [0usize; 12];
    heap_vtbl[1] = mock_heap_addref as *const () as usize;
    heap_vtbl[2] = mock_heap_release as *const () as usize;
    heap_vtbl[6] = mock_heap_setname as *const () as usize;
    heap_vtbl[9] = mock_heap_getcpu as *const () as usize;
    let fake_heap: usize = heap_vtbl.as_mut_ptr() as usize;
    FAKE_HEAP_OBJ = (&fake_heap as *const usize) as usize;

    let mut dev_vtbl = [0usize; 20];
    dev_vtbl[14] = mock_dev_create_heap as *const () as usize;
    dev_vtbl[15] = mock_dev_get_inc as *const () as usize;
    let fake_dev: usize = dev_vtbl.as_mut_ptr() as usize;
    let fake_dev_obj = (&fake_dev as *const usize) as usize;

    let dev_glob = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_dev = rd::<usize>(dev_glob);

    let run = |f: F, count: u32| -> ([u64; 8], [u64; 4], [u64; 4], (u32, u32, u32, u32)) {
        HEAP_CALLS = (0, 0, 0, 0);
        let mut heaps_arr = [0u64; 4];
        let mut ring_arr = [0u64; 8];
        let mut ranges_arr = [0u64; 4];

        let mut pool = [0u64; 11];
        let pp = pool.as_mut_ptr() as usize;
        wr::<usize>(pp, heaps_arr.as_mut_ptr() as usize);
        wr::<u32>(pp + 0x08, 0);
        wr::<u32>(pp + 0x0c, 0);
        wr::<usize>(pp + 0x10, ring_arr.as_mut_ptr() as usize);
        wr::<u32>(pp + 0x18, 0);
        wr::<u32>(pp + 0x1c, 0);
        wr::<u32>(pp + 0x20, 0);
        wr::<u32>(pp + 0x24, 0);
        wr::<u32>(pp + 0x40, 1);
        wr::<usize>(pp + 0x48, ranges_arr.as_mut_ptr() as usize);
        wr::<u32>(pp + 0x50, 0);
        wr::<u32>(pp + 0x54, 0);

        wr::<usize>(dev_glob, fake_dev_obj);
        f(pp, count);
        wr::<usize>(dev_glob, saved_dev);

        let norm_heaps = [
            if heaps_arr[0] == FAKE_HEAP_OBJ as u64 {
                1
            } else {
                0
            },
            rd::<u32>(pp + 0x08) as u64,
            rd::<u32>(pp + 0x1c) as u64,
            rd::<u32>(pp + 0x20) as u64,
        ];
        (ring_arr, norm_heaps, ranges_arr, HEAP_CALLS)
    };

    for cnt in [3u32, 0] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "142093aa0 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_d3d_142094c40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RING_STORAGE: [[u64; 8]; 5] = [[0; 8]; 5];
    static mut POOL_IDX: usize = 0;
    static mut HEAP_SEQ: u64 = 0;
    static mut COPY_HASH: u64 = 0;
    static mut COPY_COUNT: u32 = 0;
    static mut VIEW_HASH: u64 = 0;
    static mut FAKE_HEAP_OBJ: usize = 0;

    unsafe extern "win64" fn mock_56c10(this: usize) {
        wr::<u64>(this + 0x10, 0x1234);
    }
    unsafe extern "win64" fn mock_93aa0(pool: usize, chunk: u32) {
        let idx = POOL_IDX % 5;
        POOL_IDX += 1;
        for j in 0..8 {
            RING_STORAGE[idx][j] = 0x1000_0000 + (idx as u64) * 0x1000 + (j as u64) * 0x20;
        }
        wr::<usize>(pool + 0x10, RING_STORAGE[idx].as_mut_ptr() as usize);
        wr::<u32>(pool + 0x18, 0);
        wr::<u32>(pool + 0x1c, 4);
        wr::<u32>(pool + 0x20, 4);
        wr::<u32>(pool + 0x24, 8);
        VIEW_HASH = VIEW_HASH.wrapping_mul(31).wrapping_add(chunk as u64);
    }
    unsafe extern "win64" fn mock_heap_release(_this: usize) -> u32 {
        0
    }
    unsafe extern "win64" fn mock_heap_setname(_this: usize, name: usize) -> u32 {
        VIEW_HASH = VIEW_HASH
            .wrapping_mul(31)
            .wrapping_add(rd::<u16>(name) as u64);
        0
    }
    unsafe extern "win64" fn mock_heap_getcpu(_this: usize, out: usize) -> usize {
        HEAP_SEQ += 1;
        wr::<u64>(out, HEAP_SEQ * 0x0100_0000);
        out
    }
    unsafe extern "win64" fn mock_dev_create_heap(
        _dev: usize,
        desc: usize,
        _iid: usize,
        out_heap: usize,
    ) -> u32 {
        VIEW_HASH = VIEW_HASH
            .wrapping_mul(31)
            .wrapping_add(rd::<u32>(desc + 0x04) as u64);
        wr::<usize>(out_heap, FAKE_HEAP_OBJ);
        0
    }
    unsafe extern "win64" fn mock_dev_get_inc(_dev: usize, ty: u32) -> u32 {
        if ty == 0 {
            32
        } else {
            16
        }
    }
    unsafe extern "win64" fn mock_dev_cbv(_dev: usize, desc: usize, dest: u64) {
        VIEW_HASH = VIEW_HASH
            .wrapping_mul(31)
            .wrapping_add(rd::<u64>(desc) ^ dest);
    }
    unsafe extern "win64" fn mock_dev_srv(_dev: usize, _res: usize, desc: usize, dest: u64) {
        VIEW_HASH = VIEW_HASH
            .wrapping_mul(31)
            .wrapping_add((rd::<u32>(desc) as u64) ^ dest);
    }
    unsafe extern "win64" fn mock_dev_uav(
        _dev: usize,
        _res: usize,
        _cnt: usize,
        desc: usize,
        dest: u64,
    ) {
        VIEW_HASH = VIEW_HASH
            .wrapping_mul(31)
            .wrapping_add((rd::<u32>(desc) as u64) ^ dest);
    }
    unsafe extern "win64" fn mock_dev_sampler(_dev: usize, desc: usize, dest: u64) {
        VIEW_HASH = VIEW_HASH
            .wrapping_mul(31)
            .wrapping_add((rd::<u32>(desc + 0x04) as u64) ^ dest);
    }
    unsafe extern "win64" fn mock_dev_copy(
        _dev: usize,
        num: u32,
        dst: u64,
        src: u64,
        ty: u32,
    ) {
        COPY_COUNT = COPY_COUNT.wrapping_add(1);
        COPY_HASH = COPY_HASH
            .wrapping_mul(0x1000_0000_01b3)
            .wrapping_add(dst ^ src ^ ((num as u64) << 32) ^ (ty as u64));
    }
    unsafe extern "win64" fn mock_get_tid() -> u32 {
        0x1234
    }
    unsafe extern "win64" fn mock_srw_acq(_lock: usize) {}
    unsafe extern "win64" fn mock_srw_rel(_lock: usize) {}

    let _m1 = crate::difftest::Mock::new(0x1_4205_6c10, mock_56c10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_3aa0, mock_93aa0 as *const ())?;
    let _i1 = crate::difftest::IatMock::new(0x1_4264_5790, mock_get_tid as *const ())?;
    let _i2 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_srw_acq as *const ())?;
    let _i3 = crate::difftest::IatMock::new(0x1_4264_5598, mock_srw_rel as *const ())?;

    let mut heap_vtbl = [0usize; 12];
    heap_vtbl[2] = mock_heap_release as *const () as usize;
    heap_vtbl[6] = mock_heap_setname as *const () as usize;
    heap_vtbl[9] = mock_heap_getcpu as *const () as usize;
    let fake_heap: usize = heap_vtbl.as_mut_ptr() as usize;
    FAKE_HEAP_OBJ = (&fake_heap as *const usize) as usize;

    let mut dev_vtbl = [0usize; 28];
    dev_vtbl[14] = mock_dev_create_heap as *const () as usize;
    dev_vtbl[15] = mock_dev_get_inc as *const () as usize;
    dev_vtbl[17] = mock_dev_cbv as *const () as usize;
    dev_vtbl[18] = mock_dev_srv as *const () as usize;
    dev_vtbl[19] = mock_dev_uav as *const () as usize;
    dev_vtbl[22] = mock_dev_sampler as *const () as usize;
    dev_vtbl[24] = mock_dev_copy as *const () as usize;
    let fake_dev: usize = dev_vtbl.as_mut_ptr() as usize;
    let fake_dev_obj = (&fake_dev as *const usize) as usize;

    let dev_glob = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let step0_glob = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let step1_glob = exe::ptr::<u32>(0x1_4397_db7c) as usize;
    let saved_dev = rd::<usize>(dev_glob);
    let saved_s0 = rd::<u32>(step0_glob);
    let saved_s1 = rd::<u32>(step1_glob);

    let run = |f: F| -> (Vec<u64>, u32, u64, u64) {
        POOL_IDX = 0;
        HEAP_SEQ = 0;
        COPY_HASH = 0;
        COPY_COUNT = 0;
        VIEW_HASH = 0;

        let mut obj = vec![0xaaaa_bbbb_cccc_ddddu64; 0x4c0 / 8];
        let op = obj.as_mut_ptr() as usize;
        wr::<usize>(dev_glob, fake_dev_obj);
        let ret = f(op);
        assert_eq!(ret, op);
        wr::<usize>(dev_glob, saved_dev);
        wr::<u32>(step0_glob, saved_s0);
        wr::<u32>(step1_glob, saved_s1);

        // Normalize ring_buf pointers inside pools so stack/static addresses match
        for off in [0xc0usize, 0x118, 0x170, 0x1c8, 0x220] {
            obj[(off + 0x10) / 8] = 0;
        }
        for off in [0x70usize, 0x78, 0x290, 0x2b0] {
            if obj[off / 8] == FAKE_HEAP_OBJ as u64 {
                obj[off / 8] = 0xfeed;
            }
        }
        (obj, COPY_COUNT, COPY_HASH, VIEW_HASH)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142094c40");
    Ok(1)
}

pub unsafe fn diff_unknown_1420be3c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    ) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA: [u8; 0x4000] = [0; 0x4000];
    static mut ARENA_OFF: usize = 0;
    static mut CALL_HASH: u64 = 0;
    static mut TMP_U16: u16 = 0;
    static mut FAKE_JOB_NODE: [u64; 2] = [1, 0];

    unsafe fn bump_alloc(sz: usize, align: usize) -> usize {
        let a = align.max(1);
        let aligned = (ARENA_OFF + a - 1) & !(a - 1);
        let res = (core::ptr::addr_of_mut!(ARENA) as usize) + aligned;
        ARENA_OFF = aligned + sz.max(8);
        res
    }

    unsafe extern "win64" fn mock_12b0(
        _a: usize,
        _old: usize,
        _old_sz: u64,
        _new_sz: u64,
        _al: u64,
    ) -> usize {
        0
    }
    unsafe extern "win64" fn mock_11e0(_a: usize, sz: u64, al: u32) -> usize {
        bump_alloc(sz as usize, al as usize)
    }
    unsafe extern "win64" fn mock_2a80(vg: usize) {
        let p = bump_alloc(0x10 * 0x50, 8);
        wr::<usize>(vg + 0x08, p);
    }
    unsafe extern "win64" fn mock_1c40(data: usize, len: u64, key: u64) -> u64 {
        for i in 0..(len as usize) {
            if rd::<u64>(data + i * 8) >= key {
                return i as u64;
            }
        }
        len
    }
    unsafe extern "win64" fn mock_2490(grp: usize, new_cap: u64) {
        let old_p = rd::<usize>(grp + 0x08);
        let old_len = rd::<u32>(grp + 0x10) as usize;
        let new_p = bump_alloc((new_cap as usize) * 4, 4);
        for i in 0..old_len {
            wr::<u32>(new_p + i * 4, rd::<u32>(old_p + i * 4));
        }
        wr::<usize>(grp + 0x08, new_p);
        wr::<u32>(grp + 0x14, new_cap as u32);
    }
    unsafe extern "win64" fn mock_2180(vk: usize, pos: u64, key_ptr: usize) {
        let data = rd::<usize>(vk + 0x08);
        let len = rd::<u32>(vk + 0x10) as usize;
        let p = pos as usize;
        for i in (p..len).rev() {
            wr::<u64>(data + (i + 1) * 8, rd::<u64>(data + i * 8));
        }
        wr::<u64>(data + p * 8, rd::<u64>(key_ptr));
        wr::<u32>(vk + 0x10, (len + 1) as u32);
    }
    unsafe extern "win64" fn mock_1750(vg: usize, pos: u64, grp_ptr: usize) {
        let data = rd::<usize>(vg + 0x08);
        let len = rd::<u32>(vg + 0x10) as usize;
        let p = pos as usize;
        for i in (p..len).rev() {
            let s = rd::<[u64; 10]>(data + i * 0x50);
            wr::<[u64; 10]>(data + (i + 1) * 0x50, s);
        }
        wr::<[u64; 10]>(data + p * 0x50, rd::<[u64; 10]>(grp_ptr));
        wr::<u32>(vg + 0x10, (len + 1) as u32);
    }
    unsafe extern "win64" fn mock_13d0(p10: usize) {
        CALL_HASH ^= rd::<u64>(p10 + 0x08);
    }
    unsafe extern "win64" fn mock_0720(first_idx: u32, grp: usize, span8: usize) {
        wr::<u64>(grp + 0x38, rd::<u64>(span8).wrapping_add(first_idx as u64));
        wr::<u32>(grp + 0x40, first_idx.wrapping_add(10));
    }
    unsafe extern "win64" fn mock_0190(
        _p2: usize,
        first_idx: u32,
        _p13: usize,
        grp: usize,
        _arg5: u64,
        span9: usize,
        _span10: usize,
        span11: usize,
    ) {
        wr::<u64>(
            grp + 0x38,
            rd::<u64>(span9) ^ rd::<u64>(span11) ^ (first_idx as u64),
        );
        wr::<u32>(grp + 0x40, first_idx.wrapping_add(20));
    }
    unsafe extern "win64" fn mock_2eb0(vp: usize, new_cap: u64) {
        let old_p = rd::<usize>(vp + 0x08);
        let old_len = rd::<u32>(vp + 0x10) as usize;
        let new_p = bump_alloc((new_cap as usize) * 16, 8);
        for i in 0..old_len {
            wr::<[u64; 2]>(new_p + i * 16, rd::<[u64; 2]>(old_p + i * 16));
        }
        wr::<usize>(vp + 0x08, new_p);
        wr::<u32>(vp + 0x14, new_cap as u32);
    }
    unsafe extern "win64" fn mock_9da0(_tbl: usize, _mask: u32, key: u64) -> i32 {
        if (key & 1) == 0 {
            0
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_3040(
        _p13: usize,
        _kcnt: usize,
        pair_ptr: usize,
        key: u64,
        cur_p13_8: usize,
    ) -> usize {
        wr::<u64>(cur_p13_8, rd::<u64>(cur_p13_8).wrapping_add(0x10));
        TMP_U16 = ((rd::<u64>(pair_ptr) ^ key) & 0x7fff) as u16;
        core::ptr::addr_of_mut!(TMP_U16) as usize
    }
    unsafe extern "win64" fn mock_15a0(vb: usize, new_len: u32) {
        wr::<u32>(vb + 0x10, new_len);
    }
    unsafe extern "win64" fn mock_e1c0(_vb: usize, cnt: u64) -> usize {
        bump_alloc((cnt as usize) * 2, 2)
    }
    unsafe extern "win64" fn mock_3720(_mgr: usize, out_h: usize, _copy_p: usize, ja: usize) {
        CALL_HASH ^= rd::<u64>(ja + 0x20) ^ (rd::<u32>(ja + 0x18) as u64);
        wr::<usize>(out_h, core::ptr::addr_of_mut!(FAKE_JOB_NODE) as usize);
        wr::<u64>(out_h + 0x08, 0x99);
    }
    unsafe extern "win64" fn mock_6460(_q: usize, _node: usize) {
        CALL_HASH = CALL_HASH.wrapping_add(0x777);
    }
    unsafe extern "win64" fn mock_1a70(p17: usize, cnt: u64) {
        let p = bump_alloc((cnt as usize) * 0x30, 8);
        wr::<usize>(p17 + 0x08, p);
        wr::<u32>(p17 + 0x10, cnt as u32);
        wr::<u32>(p17 + 0x14, cnt as u32);
    }
    unsafe extern "win64" fn mock_2d60(
        out_30: usize,
        _p2: usize,
        p14_0: u64,
        _p14: usize,
        u16_val: u16,
        item_cnt: u32,
    ) -> usize {
        wr::<u16>(out_30, u16_val);
        wr::<u64>(out_30 + 0x08, p14_0.wrapping_add(item_cnt as u64));
        wr::<u64>(out_30 + 0x10, 0x11);
        wr::<u64>(out_30 + 0x18, 0x22);
        wr::<u32>(out_30 + 0x20, item_cnt);
        wr::<u32>(out_30 + 0x24, item_cnt);
        wr::<u32>(out_30 + 0x28, 0);
        out_30
    }
    unsafe extern "win64" fn mock_2570(vpk: usize, new_cap: u64) {
        let old_p = rd::<usize>(vpk + 0x08);
        let old_len = rd::<u32>(vpk + 0x10) as usize;
        let new_p = bump_alloc((new_cap as usize) * 8, 8);
        for i in 0..old_len {
            wr::<u64>(new_p + i * 8, rd::<u64>(old_p + i * 8));
        }
        wr::<usize>(vpk + 0x08, new_p);
        wr::<u32>(vpk + 0x14, new_cap as u32);
    }
    unsafe extern "win64" fn mock_e550(data: usize, len: u32) {
        let n = len as usize;
        for i in 0..n {
            for j in (i + 1)..n {
                let a = rd::<u64>(data + i * 8);
                let b = rd::<u64>(data + j * 8);
                if b < a {
                    wr::<u64>(data + i * 8, b);
                    wr::<u64>(data + j * 8, a);
                }
            }
        }
    }
    unsafe extern "win64" fn mock_2600(
        out_70: usize,
        _p2: usize,
        sp_pk: usize,
        sp_p13: usize,
        _p13: usize,
        p14_0: u64,
        sp_idx: usize,
        sp_p17: usize,
    ) -> usize {
        for i in 0..14usize {
            wr::<u64>(out_70 + i * 8, (i as u64) + p14_0);
        }
        wr::<u64>(out_70, rd::<u64>(sp_pk) ^ rd::<u64>(sp_p13));
        wr::<u64>(out_70 + 8, rd::<u64>(sp_idx) ^ rd::<u64>(sp_p17));
        out_70
    }
    unsafe extern "win64" fn mock_0e20(p16: usize, out_70: usize) {
        wr::<[u64; 14]>(p16, rd::<[u64; 14]>(out_70));
    }
    unsafe extern "win64" fn mock_f7c0(
        out_18: usize,
        _p2: usize,
        sp_grp: usize,
        sp_pk: usize,
        _p13: usize,
        _p12: usize,
        sp_idx: usize,
        sp_p17: usize,
        p14_0: u64,
    ) -> usize {
        wr::<u64>(out_18, rd::<u64>(sp_grp) ^ p14_0);
        wr::<u64>(out_18 + 0x08, rd::<u64>(sp_pk));
        wr::<u64>(out_18 + 0x10, rd::<u64>(sp_idx) ^ rd::<u64>(sp_p17));
        out_18
    }
    unsafe extern "win64" fn mock_srw(_l: usize) {}
    unsafe extern "win64" fn mock_sem(_h: usize, _c: u32, _prev: usize) -> i32 {
        CALL_HASH = CALL_HASH.wrapping_add(0x888);
        1
    }

    let _m01 = crate::difftest::Mock::new(0x1_411f_12b0, mock_12b0 as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_411f_11e0, mock_11e0 as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_411f_2a80, mock_2a80 as *const ())?;
    let _m04 = crate::difftest::Mock::new(0x1_411f_1c40, mock_1c40 as *const ())?;
    let _m05 = crate::difftest::Mock::new(0x1_411f_2490, mock_2490 as *const ())?;
    let _m06 = crate::difftest::Mock::new(0x1_411f_2180, mock_2180 as *const ())?;
    let _m07 = crate::difftest::Mock::new(0x1_411f_1750, mock_1750 as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_411f_13d0, mock_13d0 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_420c_0720, mock_0720 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_420c_0190, mock_0190 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_411f_2eb0, mock_2eb0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4248_9da0, mock_9da0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_420b_3040, mock_3040 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_411f_15a0, mock_15a0 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_4028_e1c0, mock_e1c0 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_411f_3720, mock_3720 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_411f_1a70, mock_1a70 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_420c_2d60, mock_2d60 as *const ())?;
    let _m20 = crate::difftest::Mock::new(0x1_411f_2570, mock_2570 as *const ())?;
    let _m21 = crate::difftest::Mock::new(0x1_40fa_e550, mock_e550 as *const ())?;
    let _m22 = crate::difftest::Mock::new(0x1_420c_2600, mock_2600 as *const ())?;
    let _m23 = crate::difftest::Mock::new(0x1_420c_0e20, mock_0e20 as *const ())?;
    let _m24 = crate::difftest::Mock::new(0x1_420b_f7c0, mock_f7c0 as *const ())?;
    let _i1 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_srw as *const ())?;
    let _i2 = crate::difftest::IatMock::new(0x1_4264_5598, mock_srw as *const ())?;
    let _i3 = crate::difftest::IatMock::new(0x1_4264_5580, mock_sem as *const ())?;

    let job_glob = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let saved_job_glob = rd::<usize>(job_glob);

    let run = |f: F| -> ([u64; 15], [u64; 14], [u64; 2], usize, u64) {
        ARENA = [0; 0x4000];
        ARENA_OFF = 0;
        CALL_HASH = 0;
        FAKE_JOB_NODE = [1, 0];

        let mut q_inner = vec![0u8; 0x1018];
        wr::<usize>(q_inner.as_mut_ptr() as usize + 0x1008, 0x5555);
        let q_ptr: usize = q_inner.as_mut_ptr() as usize;
        let q_obj = (&q_ptr as *const usize) as usize;
        let mut mgr = [0usize; 4];
        mgr[2] = q_obj;
        wr::<usize>(job_glob, mgr.as_mut_ptr() as usize);

        let mut arr3_data: [u64; 2] = [0x0003_ffff_0000_0001, 0x0004_ffff_0000_0002];
        let mut p3: [u64; 2] = [2, arr3_data.as_mut_ptr() as usize as u64];
        let mut arr4_data: [u64; 4] = [0, 101, 0, 101];
        let mut p4: [u64; 2] = [0, arr4_data.as_mut_ptr() as usize as u64];
        let mut arr5_data: [u32; 2] = [0, 1];
        let mut p5: [u64; 2] = [0, arr5_data.as_mut_ptr() as usize as u64];
        let mut arr6_data: [u64; 2] = [202, 202];
        let mut p6: [u64; 2] = [0, arr6_data.as_mut_ptr() as usize as u64];
        let mut arr7_data: [u16; 2] = [0xffff, 0xffff];
        let mut p7: [u64; 2] = [0, arr7_data.as_mut_ptr() as usize as u64];
        let mut p8: [u64; 2] = [0x800, 0x801];
        let mut p9: [u64; 2] = [0x900, 0x901];
        let mut p10: [u64; 3] = [0, 0x1000, 3];
        let mut p11: [u64; 2] = [0xb00, 0xb01];
        let mut p12 = [0u8; 0x70];
        wr::<u32>(p12.as_mut_ptr() as usize + 0x68, 2);
        let mut p13_u16s: [u16; 4] = [77, 88, 99, 111];
        let mut p13 = [0u64; 12];
        let p13p = p13.as_mut_ptr() as usize;
        wr::<u64>(p13p + 0x08, 0x500);
        wr::<usize>(p13p + 0x28, p13_u16s.as_mut_ptr() as usize);
        let mut p14_entries: [u64; 4] = [0x0000_0020_0000_0005, 0x0000_0010_0000_0003, 0, 0];
        let mut p14 = [0u64; 8];
        p14[0] = 0x1400;
        p14[6] = p14_entries.as_mut_ptr() as usize as u64;

        let mut out_p1 = [0u64; 15];
        let mut out_p16 = [0u64; 14];
        let mut out_p17 = [0u64; 3];
        let mut out_p18 = [0u64; 2];

        let hi_buf: [u64; 20] = [
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            p9.as_mut_ptr() as u64,
            p10.as_mut_ptr() as u64,
            p11.as_mut_ptr() as u64,
            p12.as_mut_ptr() as u64,
            p13p as u64,
            p14.as_mut_ptr() as u64,
            0xfeed_0015,
            out_p16.as_mut_ptr() as u64,
            out_p17.as_mut_ptr() as u64,
            out_p18.as_mut_ptr() as u64,
            0,
            0,
        ];
        let hi_base = hi_buf.as_ptr() as u64;
        crate::emu::set_incoming_rsp(hi_base, hi_base + 160);
        let ret = f(
            out_p1.as_mut_ptr() as usize,
            0xfeed_0002,
            p3.as_mut_ptr() as usize,
            p4.as_mut_ptr() as usize,
            p5.as_mut_ptr() as usize,
            p6.as_mut_ptr() as usize,
            p7.as_mut_ptr() as usize,
            p8.as_mut_ptr() as usize,
            p9.as_mut_ptr() as usize,
            p10.as_mut_ptr() as usize,
            p11.as_mut_ptr() as usize,
            p12.as_mut_ptr() as usize,
            p13p,
            p14.as_mut_ptr() as usize,
            0xfeed_0015,
            out_p16.as_mut_ptr() as usize,
            out_p17.as_mut_ptr() as usize,
            out_p18.as_mut_ptr() as usize,
        );
        wr::<usize>(job_glob, saved_job_glob);
        assert_eq!(ret, out_p1.as_mut_ptr() as usize);

        let arena_base = core::ptr::addr_of!(ARENA) as usize as u64;
        for idx in [1usize, 4, 7, 10] {
            if out_p1[idx] >= arena_base {
                out_p1[idx] -= arena_base;
            }
        }
        (out_p1, out_p16, out_p18, ARENA_OFF, CALL_HASH)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420be3c0");
    Ok(1)
}
