//! Decompiled hot functions shard Y2 (`0x14206e380..0x1420b9480`).

use crate::exe::{self, rd, wr};

/// `unknown_14206e380`
/// Original: 0x14206e380 (710 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14206e380() {
    exe::call_va!(0x1_4206_e7f0, ());
    let flag = rd::<u8>(exe::ptr::<u8>(0x1_4385_7f28) as usize);
    wr::<u8>(exe::ptr::<u8>(0x1_4384_3fcd) as usize, flag);

    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, ());
    }
    exe::call_va!(0x1_4207_61a0, (tls0.wrapping_add(0x467e0)));
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, ());
    }
    exe::call_va!(0x1_4206_df80, (rd::<u64>(tls0.wrapping_add(0x6328))));

    let g_c60 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
    exe::call_va!(0x1_4207_6db0, (g_c60));

    let g_c58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let fence_val = rd::<u64>(g_c58 + 0x40);
    let fence_obj = rd::<u64>(g_c58 + 0x38);
    let g_c60_2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc60) as usize);
    let queue = rd::<usize>(g_c60_2 + 0x10);
    let hr = exe::call_va!(
        rd::<usize>(rd::<usize>(queue) + 0x70),
        (queue, fence_obj, fence_val) -> i32
    ) as u32;
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }

    let g_c58_2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    wr::<u64>(g_c58_2 + 0x40, rd::<u64>(g_c58_2 + 0x40).wrapping_add(1));

    exe::call_va!(0x1_4209_95c0, (0u32));
    exe::call_va!(0x1_4209_95c0, (1u32));
    exe::call_va!(0x1_4209_9c20, (0u8, 0u32));
    exe::call_va!(0x1_4209_9c20, (0u8, 1u32));
    exe::call_va!(0x1_4205_6940, ());

    let lock_d38 = exe::ptr::<u8>(0x1_43a3_2d38) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_d38));

    let count = rd::<u32>(exe::ptr::<u32>(0x1_43a6_3250) as usize) as u64;
    let base_248 = rd::<usize>(exe::ptr::<usize>(0x1_43a6_3248) as usize);
    if count != 0 {
        let mut start_idx: i32 = -1;
        let mut rcx = base_248.wrapping_add(0x08);
        let thresh = rd::<u64>(exe::ptr::<u64>(0x1_43a3_5918) as usize);
        let mut r8: u64 = 0;
        while r8 < count {
            let expired = rd::<u8>(rcx + 0x19) != 0 && thresh > rd::<u64>(rcx).wrapping_add(1000);
            if start_idx == -1 {
                if expired {
                    start_idx = r8 as i32;
                } else {
                    start_idx = -1;
                }
            } else if !expired {
                start_idx = -1;
            }
            r8 += 1;
            rcx = rcx.wrapping_add(0x28);
        }
        if start_idx != -1 {
            let vec_addr = exe::ptr::<u8>(0x1_43a6_3248) as usize;
            exe::call_va!(
                0x1_411d_6b80,
                (rcx, start_idx as i64 as u64, r8, vec_addr)
            );
        }
    }
    exe::call_iat!(0x1_4264_5598, (lock_d38));

    let g_c58_3 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let rsi = rd::<usize>(g_c58_3 + 0x68);
    let saved_lock = rd::<usize>(rsi + 0x18);
    exe::call_iat!(0x1_4264_55a8, (saved_lock));
    exe::call_va!(0x1_4208_7430, (rsi));
    exe::call_va!(0x1_4208_7d80, (rsi));

    let rdi = rd::<usize>(rsi + 0x18);
    let lim = rd::<u64>(rdi + 0x210);
    let cur_val = rd::<u64>(rsi);
    let min_val = if lim >= cur_val { cur_val } else { lim };
    let diff = cur_val.wrapping_sub(min_val);

    let fence_100 = rd::<usize>(rdi + 0x100);
    let completed = exe::call_va!(rd::<usize>(rd::<usize>(fence_100) + 0x40), (fence_100) -> u64);
    if completed == u64::MAX {
        exe::call_va!(0x1_4205_d410, (0usize, 0x887a_0005u32));
    }
    exe::call_va!(
        0x1_4208_ad10,
        (rd::<usize>(rdi + 0x110), diff >> 0x19, completed)
    );

    let lock_58 = rdi + 0x58;
    let acquired = exe::call_iat!(0x1_4264_5788, (lock_58) -> u8);
    if acquired != 0 {
        let arr_98 = rd::<usize>(rdi + 0x98);
        let end_98 = arr_98.wrapping_add((rd::<u32>(rdi + 0xa0) as usize) * 16);
        let mut p = arr_98;
        if arr_98 != end_98 {
            while p != end_98 {
                if rd::<u64>(p + 0x08) > completed {
                    break;
                }
                p = p.wrapping_add(16);
            }
            if p != arr_98 {
                wr::<u8>(rdi + 0xa8, 1);
                let g_298 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
                exe::call_iat!(0x1_4264_55a8, (g_298 + 0x250));
                wr::<u32>(g_298 + 0x258, rd::<u32>(g_298 + 0x258).wrapping_add(1));
                exe::call_iat!(0x1_4264_5598, (g_298 + 0x250));
                exe::call_iat!(0x1_4264_5590, (g_298 + 0x248));
            }
        }
        exe::call_iat!(0x1_4264_5598, (lock_58));
    }

    exe::call_va!(0x1_4208_6f60, (rsi));
    let rdi_after = rd::<usize>(rsi + 0x18);
    wr::<u8>(rdi_after + 0x218, 0);
    exe::call_iat!(0x1_4264_5598, (saved_lock));
}

#[inline]
unsafe fn pop_desc_handle(pool_offset: usize) -> u64 {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let lock_ptr = mgr.wrapping_add(0x30);
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock_ptr + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_ptr));
        wr::<u32>(lock_ptr + 0x08, tid);
    }
    wr::<u32>(lock_ptr + 0x0c, rd::<u32>(lock_ptr + 0x0c).wrapping_add(1));

    let pool = mgr.wrapping_add(pool_offset);
    if rd::<u32>(pool + 0x20) == 0 {
        exe::call_va!(0x1_4209_3aa0, (pool, rd::<u32>(pool + 0x44)));
    }
    let ring_buf = rd::<usize>(pool + 0x10);
    let cap = rd::<u32>(pool + 0x24) as u64;
    let head = rd::<u32>(pool + 0x18) as u64;
    let idx = (head % cap) as usize;
    let handle = rd::<u64>(ring_buf.wrapping_add(idx * 8));
    let avail = rd::<u32>(pool + 0x20);
    if avail != 0 {
        let next_head = (head as u32).wrapping_add(1);
        wr::<u32>(pool + 0x18, next_head);
        if next_head == (cap as u32) {
            wr::<u32>(pool + 0x18, 0);
        }
        wr::<u32>(pool + 0x20, avail.wrapping_sub(1));
    }

    let new_lock_cnt = rd::<u32>(lock_ptr + 0x0c).wrapping_sub(1);
    wr::<u32>(lock_ptr + 0x0c, new_lock_cnt);
    if new_lock_cnt == 0 {
        crate::simd_ops::atomic_xchg_u32((lock_ptr + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock_ptr));
    }
    handle
}

/// `unknown_142075080`
/// Original: 0x142075080 (1572 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142075080(
    param_1: usize,
    param_2: usize,
    param_3: i8,
    param_4: u64,
    param_5: u64,
    param_6: u64,
    param_7: u32,
) {
    let is_structured = param_3 == 0 && param_4 != 0;
    let is_raw = param_3 == 0 && param_4 == 0;
    let stride_u64 = if is_raw
        || param_4 == 0
        || ((param_7 >> 2) & 1) != 0
        || ((param_7 >> 1) & 1) != 0
    {
        4u64
    } else {
        (param_4 as u32) as u64
    };

    let first_elem = (param_5 / stride_u64) as u32;
    let num_elems = (param_6.wrapping_sub(1).wrapping_add(stride_u64) / stride_u64) as u32;
    let first_raw = (param_5 >> 2) as u32;
    let num_raw = (param_6.wrapping_add(3) >> 2) as u32;

    let dxgi_fmt: u32 = if is_raw {
        0x27
    } else if param_3 != 0 {
        let lut = exe::ptr::<u8>(0x1_4297_eda0) as usize;
        rd::<u32>(lut.wrapping_add(((param_3 as isize) * 0x14) as usize))
    } else {
        0
    };

    if (param_7 & 1) != 0 || ((param_7 >> 5) & 1) != 0 {
        let mut srv_desc = [0u32; 10];
        let sp = srv_desc.as_mut_ptr() as usize;
        wr::<u32>(sp, dxgi_fmt);
        wr::<u32>(sp + 0x04, 1);
        wr::<u64>(sp + 0x08, 0x1688);
        wr::<u64>(sp + 0x10, first_elem as u64);
        wr::<u32>(sp + 0x18, num_elems);
        wr::<u32>(
            sp + 0x1c,
            if is_structured {
                stride_u64 as u32
            } else {
                0
            },
        );
        wr::<u32>(sp + 0x20, is_raw as u32);
        wr::<u32>(sp + 0x24, 0);

        let h_srv1 = pop_desc_handle(0xc0);
        wr::<u64>(param_1 + 0x20, h_srv1);
        let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        exe::call_va!(
            rd::<usize>(rd::<usize>(dev) + 0x90),
            (dev, param_2, sp, h_srv1)
        );

        if (param_5 & 0xf) == 0 {
            let mut srv_raw = [0u32; 10];
            let srp = srv_raw.as_mut_ptr() as usize;
            wr::<u32>(srp, 0x27);
            wr::<u32>(srp + 0x04, 1);
            wr::<u64>(srp + 0x08, 0x1688);
            wr::<u64>(srp + 0x10, first_raw as u64);
            wr::<u32>(srp + 0x18, num_raw);
            wr::<u32>(srp + 0x1c, 0);
            wr::<u64>(srp + 0x20, 1);

            let h_srv2 = pop_desc_handle(0xc0);
            wr::<u64>(param_1 + 0x28, h_srv2);
            let dev2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
            exe::call_va!(
                rd::<usize>(rd::<usize>(dev2) + 0x90),
                (dev2, param_2, srp, h_srv2)
            );
        }
    }

    if ((param_7 >> 3) & 1) != 0 {
        let gpu_va = exe::call_va!(rd::<usize>(rd::<usize>(param_2) + 0x58), (param_2) -> u64);
        let mut cbv_desc = [0u64; 2];
        let cp = cbv_desc.as_mut_ptr() as usize;
        wr::<u64>(cp, gpu_va.wrapping_add(param_5));
        wr::<u32>(cp + 0x08, param_6 as u32);
        wr::<u32>(cp + 0x0c, 0);

        let h_cbv = pop_desc_handle(0x1c8);
        wr::<u64>(param_1 + 0x18, h_cbv);
        let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        exe::call_va!(rd::<usize>(rd::<usize>(dev) + 0x88), (dev, cp, h_cbv));
    }

    if ((param_7 >> 4) & 1) != 0 {
        let mut uav_desc = [0u32; 12];
        let up = uav_desc.as_mut_ptr() as usize;
        wr::<u32>(up, dxgi_fmt);
        wr::<u32>(up + 0x04, 1);
        wr::<u64>(up + 0x08, first_elem as u64);
        wr::<u32>(up + 0x10, num_elems);
        wr::<u32>(
            up + 0x14,
            if is_structured {
                stride_u64 as u32
            } else {
                0
            },
        );
        wr::<u64>(up + 0x18, 0);
        wr::<u32>(up + 0x20, is_raw as u32);
        wr::<u64>(up + 0x24, 0);
        wr::<u32>(up + 0x2c, 0);

        let h_uav1 = pop_desc_handle(0x1c8);
        wr::<u64>(param_1 + 0x30, h_uav1);
        let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        exe::call_va!(
            rd::<usize>(rd::<usize>(dev) + 0x98),
            (dev, param_2, 0usize, up, h_uav1)
        );

        let mut uav_raw = [0u32; 12];
        let urp = uav_raw.as_mut_ptr() as usize;
        wr::<u32>(urp, 0x27);
        wr::<u32>(urp + 0x04, 1);
        wr::<u64>(urp + 0x08, first_raw as u64);
        wr::<u32>(urp + 0x10, num_raw);
        wr::<u32>(urp + 0x14, 0);
        wr::<u64>(urp + 0x18, 0);
        wr::<u32>(urp + 0x20, 1);
        wr::<u64>(urp + 0x24, 0);
        wr::<u32>(urp + 0x2c, 0);

        let h_uav2 = pop_desc_handle(0x1c8);
        wr::<u64>(param_1 + 0x38, h_uav2);
        let dev2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
        exe::call_va!(
            rd::<usize>(rd::<usize>(dev2) + 0x98),
            (dev2, param_2, 0usize, urp, h_uav2)
        );
    }
}

/// `unknown_1420756b0`
/// Original: 0x1420756b0 (409 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1420756b0(
    param_1: u64,
    param_2: u32,
    param_3: u32,
    param_4: usize,
) -> u32 {
    let mut res_flags: u32 = if ((param_2 >> 4) & 1) != 0 { 4 } else { 0 };
    if (param_2 & 1) == 0 && ((param_2 >> 5) & 1) == 0 {
        res_flags = (((param_2 >> 4) & 1) << 2) | 8;
    }

    let mut state: u32 = (param_2 >> 1) & 1;
    if ((param_2 >> 2) & 1) != 0 {
        state |= 2;
    }
    if ((param_2 >> 5) & 1) != 0 {
        state |= 0x40;
    }
    if (param_2 & 1) != 0 {
        state |= 0xc0;
    }

    let mut heap_type: u32 = 1;
    if ((param_2 >> 3) & 1) == 0 {
        if ((param_3 >> 4) & 1) != 0 {
            heap_type = 2;
            state = 0xac3;
        } else if ((param_3 >> 5) & 1) != 0 {
            heap_type = 3;
            state = 0x400;
        }
    }

    let heap_props: [u32; 5] = [heap_type, 0, 0, 1, 1];
    let mut res_desc = [0u8; 0x38];
    let rdp = res_desc.as_mut_ptr() as usize;
    wr::<u32>(rdp, 1);
    wr::<u64>(rdp + 0x08, 0);
    wr::<u64>(rdp + 0x10, param_1);
    wr::<u32>(rdp + 0x18, 1);
    wr::<u32>(rdp + 0x1c, 0x0001_0001);
    wr::<u32>(rdp + 0x20, 0);
    wr::<u32>(rdp + 0x24, 1);
    wr::<u32>(rdp + 0x28, 0);
    wr::<u32>(rdp + 0x2c, 1);
    wr::<u32>(rdp + 0x30, res_flags);

    let g_c58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let heap_flags: u32 = if rd::<u8>(g_c58 + 0xa0) != 0 {
        0x1000
    } else {
        0
    };
    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let iid = exe::ptr::<u8>(0x1_433d_c7e8) as usize;
    let hr = exe::call_va!(
        rd::<usize>(rd::<usize>(dev) + 0xd8),
        (
            dev,
            heap_props.as_ptr() as usize,
            heap_flags,
            rdp,
            state,
            0usize,
            iid,
            param_4
        ) -> i32
    ) as u32;
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr));
    }
    state
}

/// `unknown_142075bc0`
/// Original: 0x142075bc0 (659 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142075bc0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: u64,
    param_5: u64,
) {
    let mut cmd_ctx = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_ctx.as_mut_ptr() as usize));
    let cmd_list = cmd_ctx[1];
    let list_type = exe::call_va!(rd::<usize>(rd::<usize>(cmd_list) + 0x40), (cmd_list) -> i32);

    if list_type == 3 {
        exe::call_va!(
            rd::<usize>(rd::<usize>(cmd_list) + 0x78),
            (
                cmd_list,
                rd::<u64>(param_1 + 0x40),
                param_2,
                rd::<u64>(param_3 + 0x40),
                param_4,
                param_5
            )
        );
    } else {
        let src_state = rd::<u32>(param_3 + 0x48);
        let need_src = src_state != 0x800 && src_state != 0xac3;
        let dst_state = rd::<u32>(param_1 + 0x48);

        let mut barriers_pre = [[0u64; 4]; 2];
        if need_src {
            barriers_pre[0] = [
                0,
                rd::<u64>(param_3 + 0x40),
                0xffff_ffffu64 | ((src_state as u64) << 32),
                0x800,
            ];
        }
        if dst_state != 0x400 {
            barriers_pre[1] = [
                0,
                rd::<u64>(param_1 + 0x40),
                0xffff_ffffu64 | ((dst_state as u64) << 32),
                0x400,
            ];
        }

        if need_src {
            let cnt: u32 = if dst_state == 0x400 { 1 } else { 2 };
            exe::call_va!(
                rd::<usize>(rd::<usize>(cmd_list) + 0xd0),
                (cmd_list, cnt, barriers_pre[0].as_ptr() as usize)
            );
        } else if dst_state != 0x400 {
            exe::call_va!(
                rd::<usize>(rd::<usize>(cmd_list) + 0xd0),
                (cmd_list, 1u32, barriers_pre[1].as_ptr() as usize)
            );
        }

        exe::call_va!(
            rd::<usize>(rd::<usize>(cmd_list) + 0x78),
            (
                cmd_list,
                rd::<u64>(param_1 + 0x40),
                param_2,
                rd::<u64>(param_3 + 0x40),
                param_4,
                param_5
            )
        );

        let mut barriers_post = [[0u64; 4]; 2];
        if need_src {
            let s_state = rd::<u32>(param_3 + 0x48);
            let s_res = rd::<u64>(param_3 + 0x40);
            barriers_post[0] = [
                0,
                s_res,
                0xffff_ffffu64 | (0x800u64 << 32),
                s_state as u64,
            ];
        }
        if dst_state != 0x400 {
            let d_state = rd::<u32>(param_1 + 0x48);
            let d_res = rd::<u64>(param_1 + 0x40);
            barriers_post[1] = [
                0,
                d_res,
                0xffff_ffffu64 | (0x400u64 << 32),
                d_state as u64,
            ];
        }

        if need_src {
            let cnt: u32 = if dst_state == 0x400 { 1 } else { 2 };
            exe::call_va!(
                rd::<usize>(rd::<usize>(cmd_list) + 0xd0),
                (cmd_list, cnt, barriers_post[0].as_ptr() as usize)
            );
        } else if dst_state != 0x400 {
            exe::call_va!(
                rd::<usize>(rd::<usize>(cmd_list) + 0xd0),
                (cmd_list, 1u32, barriers_post[1].as_ptr() as usize)
            );
        }
    }

    let lock = cmd_ctx[0];
    if lock != 0 {
        let new_cnt = rd::<u32>(lock + 0x0c).wrapping_sub(1);
        wr::<u32>(lock + 0x0c, new_cnt);
        if new_cnt == 0 {
            crate::simd_ops::atomic_xchg_u32((lock + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (lock));
        }
    }
}

/// `unknown_142076960`
/// Original: 0x142076960 (624 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_142076960(param_1: usize) {
    let count_30 = rd::<u32>(param_1 + 0x30) as usize;
    let arr_28 = rd::<usize>(param_1 + 0x28);
    if arr_28 != arr_28.wrapping_add(count_30 * 8) {
        let queue = rd::<usize>(param_1 + 0x10);
        let mut buf: usize = 0;
        if count_30 != 0 {
            let byte_len = (count_30 * 8) as u64;
            buf = exe::call_va!(0x1_4241_c0a0, (byte_len, 8u32) -> usize);
            let _ = exe::call_va!(0x1_4183_acc2, (buf, 0u32, byte_len) -> usize);
            for i in 0..count_30 {
                let item = rd::<usize>(arr_28.wrapping_add(i * 8));
                wr::<u64>(buf.wrapping_add(i * 8), rd::<u64>(item + 0x28));
            }
        }
        exe::call_va!(
            rd::<usize>(rd::<usize>(queue) + 0x50),
            (queue, count_30 as u32, buf)
        );

        let mut cur = arr_28;
        let end = arr_28.wrapping_add(count_30 * 8);
        while cur != end {
            let item = rd::<usize>(cur);
            exe::call_va!(
                rd::<usize>(rd::<usize>(queue) + 0x70),
                (queue, rd::<u64>(item + 0x10), rd::<u64>(item + 0x08))
            );
            cur = cur.wrapping_add(8);
        }

        if buf != 0 {
            exe::call_va!(0x1_4241_c190, (buf));
        }

        let lock_obj = rd::<usize>(param_1 + 0x08);
        let count2 = rd::<u32>(param_1 + 0x30) as usize;
        let mut cur2 = rd::<usize>(param_1 + 0x28);
        exe::call_iat!(0x1_4264_55a8, (lock_obj));
        let end2 = cur2.wrapping_add(count2 * 8);
        while cur2 != end2 {
            let mut p = rd::<usize>(lock_obj + 0x08);
            let p_end = p.wrapping_add((rd::<u32>(lock_obj + 0x10) as usize) * 16);
            if p != p_end {
                let want = rd::<u64>(cur2);
                while p != p_end {
                    if rd::<u64>(p) == want {
                        break;
                    }
                    p = p.wrapping_add(16);
                }
            }
            wr::<u8>(p + 0x08, 0);
            cur2 = cur2.wrapping_add(8);
        }
        exe::call_iat!(0x1_4264_5598, (lock_obj));
    }

    let mut cur_18 = rd::<usize>(param_1 + 0x18);
    let end_18 = cur_18.wrapping_add((rd::<u32>(param_1 + 0x20) as usize) * 8);
    while cur_18 != end_18 {
        let r15 = rd::<usize>(cur_18);
        wr::<u64>(r15 + 0x18, 0);
        let mut sub_cur = rd::<usize>(r15);
        let sub_end = sub_cur.wrapping_add((rd::<u32>(r15 + 0x08) as usize) * 24);
        while sub_cur != sub_end {
            let obj = rd::<usize>(sub_cur);
            let vtbl = rd::<usize>(obj);
            exe::call_va!(
                rd::<usize>(vtbl),
                (obj, rd::<u64>(sub_cur + 0x08), rd::<u64>(sub_cur + 0x10))
            );
            sub_cur = sub_cur.wrapping_add(24);
        }
        wr::<u32>(r15 + 0x08, 0);

        let rbx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc30) as usize);
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(rbx + 0x08) != tid {
            exe::call_iat!(0x1_4264_55a8, (rbx));
            wr::<u32>(rbx + 0x08, tid);
        }
        wr::<u32>(rbx + 0x0c, rd::<u32>(rbx + 0x0c).wrapping_add(1));

        let used = rd::<u32>(rbx + 0x20);
        if used == rd::<u32>(rbx + 0x24) {
            exe::call_va!(0x1_4034_97a0, (rbx + 0x10, used.wrapping_add(1)));
        }
        let tail = rd::<u32>(rbx + 0x1c);
        let ring_buf = rd::<usize>(rbx + 0x10);
        wr::<usize>(ring_buf.wrapping_add((tail as usize) * 8), r15);
        let next_tail = tail.wrapping_add(1);
        wr::<u32>(rbx + 0x1c, next_tail);
        if next_tail == rd::<u32>(rbx + 0x24) {
            wr::<u32>(rbx + 0x1c, 0);
        }
        wr::<u32>(rbx + 0x20, rd::<u32>(rbx + 0x20).wrapping_add(1));

        let new_lock_cnt = rd::<u32>(rbx + 0x0c).wrapping_sub(1);
        wr::<u32>(rbx + 0x0c, new_lock_cnt);
        if new_lock_cnt == 0 {
            crate::simd_ops::atomic_xchg_u32((rbx + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (rbx));
        }

        cur_18 = cur_18.wrapping_add(8);
    }

    wr::<u32>(param_1 + 0x20, 0);
    wr::<u32>(param_1 + 0x30, 0);
}

/// `r_1420ad980`
/// Original: 0x1420ad980 (1866 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_1420ad980(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    wr::<[u64; 6]>(param_1, [0; 6]);
    let num_buckets = rd::<u32>(param_2 + 0x18);
    let num_buckets_u64 = num_buckets as u64;
    let mut temp_buckets: usize = 0;
    if num_buckets_u64 != 0 {
        temp_buckets = exe::call_va!(0x1_4241_c0a0, (num_buckets_u64 << 5, 8u32) -> usize);
        for i in 0..(num_buckets as usize) {
            wr::<[u64; 4]>(temp_buckets.wrapping_add(i * 0x20), [0; 4]);
        }
    }

    let mut total_a8: u64 = 0;
    let mut total_pairs: u64 = 0;
    let mut scratch_ptr: usize = 0;
    let mut scratch_cap: u64 = 0;

    let item_count = rd::<u32>(param_3 + 0x10) as usize;
    let mut rsi = rd::<usize>(param_3 + 0x08);
    let items_end = rsi.wrapping_add(item_count * 0x130);

    while rsi != items_end {
        let mut scratch_len: usize = 0;
        let w_a8 = rd::<u16>(rsi + 0xa8);
        if w_a8 != 0xffff {
            exe::call_va!(
                0x1_411e_9ea0,
                (temp_buckets.wrapping_add((w_a8 as usize) << 5), rsi)
            );
            total_a8 = total_a8.wrapping_add(1);
            let w_a8_re = rd::<u16>(rsi + 0xa8);
            let cap_u32 = scratch_cap as u32 as u64;
            if cap_u32 == 0 {
                let mut new_cap = (cap_u32 >> 1).wrapping_add(cap_u32);
                if new_cap < 1 {
                    new_cap = 1;
                }
                scratch_cap = new_cap;
                let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 8, 4u32) -> usize);
                if scratch_ptr != 0 {
                    exe::call_va!(0x1_4241_c190, (scratch_ptr));
                }
                scratch_ptr = new_buf;
            }
            wr::<u64>(scratch_ptr, (w_a8_re as u64) | (1u64 << 32));
            scratch_len = 1;
        }

        let w_60 = rd::<u16>(rsi + 0x60);
        if w_60 != 0xffff {
            let mut found_slot: Option<usize> = None;
            for i in 0..scratch_len {
                let p = scratch_ptr.wrapping_add(i * 8);
                if rd::<u16>(p) == w_60 {
                    found_slot = Some(p);
                    break;
                }
            }
            let target_slot = if let Some(p) = found_slot {
                p
            } else {
                let cap_u32 = scratch_cap as u32 as u64;
                if (scratch_len as u64) == cap_u32 {
                    let need = (scratch_len as u64).wrapping_add(1);
                    let mut new_cap = (cap_u32 >> 1).wrapping_add(cap_u32);
                    if new_cap < need {
                        new_cap = need;
                    }
                    let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 8, 4u32) -> usize);
                    let _ = exe::call_va!(
                        0x1_4183_acc8,
                        (new_buf, scratch_ptr, (scratch_len as u64) * 8) -> usize
                    );
                    if scratch_ptr != 0 {
                        exe::call_va!(0x1_4241_c190, (scratch_ptr));
                    }
                    scratch_ptr = new_buf;
                    scratch_cap = new_cap as u32 as u64;
                }
                let p = scratch_ptr.wrapping_add(scratch_len * 8);
                wr::<u64>(p, 0);
                wr::<u16>(p, w_60);
                scratch_len += 1;
                p
            };
            wr::<u32>(target_slot + 0x04, rd::<u32>(target_slot + 0x04) | 4);
        }

        let sub_count = rd::<u32>(rsi + 0xf8) as usize;
        let mut sub_cur = rd::<usize>(rsi + 0xf0);
        let sub_end = sub_cur.wrapping_add(sub_count * 0x48);
        while sub_cur != sub_end {
            let w_sub = rd::<u16>(sub_cur);
            let mut found_slot: Option<usize> = None;
            for i in 0..scratch_len {
                let p = scratch_ptr.wrapping_add(i * 8);
                if rd::<u16>(p) == w_sub {
                    found_slot = Some(p);
                    break;
                }
            }
            let target_slot = if let Some(p) = found_slot {
                p
            } else {
                let cap_u32 = scratch_cap as u32 as u64;
                if (scratch_len as u64) == cap_u32 {
                    let need = (scratch_len as u64).wrapping_add(1);
                    let mut new_cap = (cap_u32 >> 1).wrapping_add(cap_u32);
                    if new_cap < need {
                        new_cap = need;
                    }
                    let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 8, 4u32) -> usize);
                    let _ = exe::call_va!(
                        0x1_4183_acc8,
                        (new_buf, scratch_ptr, (scratch_len as u64) * 8) -> usize
                    );
                    if scratch_ptr != 0 {
                        exe::call_va!(0x1_4241_c190, (scratch_ptr));
                    }
                    scratch_ptr = new_buf;
                    scratch_cap = new_cap;
                }
                let p = scratch_ptr.wrapping_add(scratch_len * 8);
                wr::<u64>(p, 0);
                wr::<u16>(p, w_sub);
                scratch_len += 1;
                p
            };
            wr::<u32>(target_slot + 0x04, rd::<u32>(target_slot + 0x04) | 2);
            sub_cur = sub_cur.wrapping_add(0x48);
        }

        for i in 0..scratch_len {
            let p = scratch_ptr.wrapping_add(i * 8);
            let bkt_idx = rd::<u16>(p) as usize;
            let bkt = temp_buckets.wrapping_add(bkt_idx << 5);
            let item_id = rd::<u16>(rsi);
            let flags = rd::<u32>(p + 0x04);
            let cur_len = rd::<u32>(bkt + 0x18) as u64;
            let cur_cap = rd::<u32>(bkt + 0x1c) as u64;
            let mut dst_buf = rd::<usize>(bkt + 0x10);
            if cur_len * 8 == cur_cap * 8 {
                let need = cur_len.wrapping_add(1);
                let mut new_cap = (cur_cap >> 1).wrapping_add(cur_cap);
                if new_cap < need {
                    new_cap = need;
                }
                dst_buf = exe::call_va!(0x1_4241_c0a0, (new_cap * 8, 4u32) -> usize);
                let copy_bytes = (rd::<u32>(bkt + 0x18) as u64) << 3;
                let _ = exe::call_va!(
                    0x1_4183_acc8,
                    (dst_buf, rd::<usize>(bkt + 0x10), copy_bytes) -> usize
                );
                let old_buf = rd::<usize>(bkt + 0x10);
                if old_buf != 0 {
                    exe::call_va!(0x1_4241_c190, (old_buf));
                }
                wr::<usize>(bkt + 0x10, dst_buf);
                wr::<u32>(bkt + 0x18, cur_len as u32);
                wr::<u32>(bkt + 0x1c, new_cap as u32);
            }
            let packed = (item_id as u64) | ((flags as u64) << 32);
            wr::<u64>(dst_buf.wrapping_add((cur_len as usize) * 8), packed);
            wr::<u32>(bkt + 0x18, rd::<u32>(bkt + 0x18).wrapping_add(1));
        }

        total_pairs = total_pairs.wrapping_add(scratch_len as u64);
        rsi = rsi.wrapping_add(0x130);
    }

    if total_a8 > (rd::<u32>(param_1 + 0x0c) as u64) {
        exe::call_va!(0x1_40fa_5d10, (param_1, total_a8));
    }
    if total_pairs > (rd::<u32>(param_1 + 0x1c) as u64) {
        exe::call_va!(0x1_4020_bcb0, (param_1 + 0x10, total_pairs));
    }

    let vec_20 = param_1 + 0x20;
    let old_len_20 = rd::<u32>(vec_20 + 0x08) as u64;
    let cap_20 = rd::<u32>(vec_20 + 0x0c) as u64;
    if num_buckets_u64 > cap_20 {
        let mut new_cap = (cap_20 >> 1).wrapping_add(cap_20);
        if new_cap < num_buckets_u64 {
            new_cap = num_buckets_u64;
        }
        exe::call_va!(0x1_40fb_1b60, (vec_20, new_cap));
        let cur_len = rd::<u32>(vec_20 + 0x08) as usize;
        let rem = num_buckets_u64.wrapping_sub(old_len_20) as usize;
        if rem != 0 {
            let base = rd::<usize>(vec_20).wrapping_add(cur_len * 0x20);
            for i in 0..rem {
                wr::<[u64; 4]>(base.wrapping_add(i * 0x20), [0; 4]);
            }
        }
        wr::<u32>(vec_20 + 0x08, num_buckets);
    } else if num_buckets_u64 > old_len_20 {
        let rem = (num_buckets_u64 - old_len_20) as usize;
        let base = rd::<usize>(vec_20).wrapping_add((old_len_20 as usize) * 0x20);
        for i in 0..rem {
            wr::<[u64; 4]>(base.wrapping_add(i * 0x20), [0; 4]);
        }
        wr::<u32>(vec_20 + 0x08, num_buckets);
    } else if num_buckets_u64 != old_len_20 {
        wr::<u32>(vec_20 + 0x08, num_buckets);
    }

    if num_buckets != 0 {
        for b_idx in 0..(num_buckets as usize) {
            let bkt = temp_buckets.wrapping_add(b_idx * 0x20);
            let start_w_idx = rd::<u32>(param_1 + 0x08) as usize;
            let src_w_ptr = rd::<usize>(bkt);
            let src_w_len = rd::<u32>(bkt + 0x08) as usize;
            let mut end_w_idx = start_w_idx;
            for j in 0..src_w_len {
                let cur_len = end_w_idx as u64;
                let cur_cap = rd::<u32>(param_1 + 0x0c) as u64;
                if cur_len * 2 == cur_cap * 2 {
                    exe::call_va!(0x1_40fa_5d10, (param_1, cur_len.wrapping_add(1)));
                }
                let pos = rd::<u32>(param_1 + 0x08) as usize;
                let dst = rd::<usize>(param_1).wrapping_add(pos * 2);
                wr::<u16>(dst, rd::<u16>(src_w_ptr.wrapping_add(j * 2)));
                let next_pos = (pos as u32).wrapping_add(1);
                wr::<u32>(param_1 + 0x08, next_pos);
                end_w_idx = next_pos as usize;
            }

            let start_q_idx = rd::<u32>(param_1 + 0x18) as usize;
            let src_q_ptr = rd::<usize>(bkt + 0x10);
            let src_q_len = rd::<u32>(bkt + 0x18) as usize;
            let mut cur_q_idx = start_q_idx;
            for j in 0..src_q_len {
                let cur_len = cur_q_idx as u64;
                let cur_cap = rd::<u32>(param_1 + 0x1c) as u64;
                if cur_len * 8 == cur_cap * 8 {
                    exe::call_va!(0x1_4020_bcb0, (param_1 + 0x10, cur_len.wrapping_add(1)));
                }
                let pos = rd::<u32>(param_1 + 0x18) as usize;
                let dst = rd::<usize>(param_1 + 0x10).wrapping_add(pos * 8);
                wr::<u64>(dst, rd::<u64>(src_q_ptr.wrapping_add(j * 8)));
                let next_pos = (pos as u32).wrapping_add(1);
                wr::<u32>(param_1 + 0x18, next_pos);
                cur_q_idx = next_pos as usize;
            }

            let base_w = rd::<usize>(param_1);
            let w_begin = base_w.wrapping_add(start_w_idx * 2);
            let w_end = base_w.wrapping_add(end_w_idx * 2);
            let w_cnt = ((w_end as isize - w_begin as isize) >> 1) as u64;

            let base_q = rd::<usize>(param_1 + 0x10);
            let end_q_idx = rd::<u32>(param_1 + 0x18) as usize;
            let q_begin = base_q.wrapping_add(start_q_idx * 8);
            let q_end = base_q.wrapping_add(end_q_idx * 8);
            let q_cnt = ((q_end as isize - q_begin as isize) >> 3) as u64;

            let out_slot = rd::<usize>(param_1 + 0x20).wrapping_add(b_idx * 0x20);
            wr::<u64>(out_slot, w_cnt);
            wr::<usize>(out_slot + 0x08, w_begin);
            wr::<u64>(out_slot + 0x10, q_cnt);
            wr::<usize>(out_slot + 0x18, q_begin);
        }
    }

    if scratch_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (scratch_ptr));
    }

    if num_buckets != 0 {
        for b_idx in 0..(num_buckets as usize) {
            let bkt = temp_buckets.wrapping_add(b_idx * 0x20);
            let p10 = rd::<usize>(bkt + 0x10);
            if p10 != 0 {
                exe::call_va!(0x1_4241_c190, (p10));
            }
            wr::<u64>(bkt + 0x10, 0);
            wr::<u64>(bkt + 0x18, 0);
            let p00 = rd::<usize>(bkt);
            if p00 != 0 {
                exe::call_va!(0x1_4241_c190, (p00));
            }
            wr::<u64>(bkt, 0);
            wr::<u64>(bkt + 0x08, 0);
        }
    }

    if temp_buckets != 0 {
        exe::call_va!(0x1_4241_c190, (temp_buckets));
    }

    param_1
}

#[inline]
unsafe fn slab_free_ptr(ptr: usize) {
    if ptr != 0 {
        let seg = ptr.wrapping_sub(1) & !0x01ff_ffffusize;
        let page_idx = ptr.wrapping_sub(seg) >> 16;
        let hdr_raw = seg.wrapping_add(0x108).wrapping_add(page_idx * 0x50);
        let hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        if exe::teb() == rd::<usize>(seg + 0x100) {
            if rd::<u8>(hdr + 0x0e) == 0 {
                wr::<u64>(ptr, rd::<u64>(hdr + 0x20));
                let new_cnt = rd::<u32>(hdr + 0x18).wrapping_sub(1);
                wr::<u32>(hdr + 0x18, new_cnt);
                wr::<usize>(hdr + 0x20, ptr);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (hdr));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (seg, hdr, 1u8, ptr));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (seg, hdr, 0u8, ptr));
        }
    }
}

/// `r_1420b0520`
/// Original: 0x1420b0520 (511 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420b0520(param_1: usize) {
    let mut cur0 = rd::<usize>(param_1);
    let end0 = cur0.wrapping_add((rd::<u32>(param_1 + 0x08) as usize) * 16);
    while cur0 != end0 {
        slab_free_ptr(rd::<usize>(cur0));
        cur0 = cur0.wrapping_add(16);
    }
    wr::<u32>(param_1 + 0x08, 0);

    let arr_20 = rd::<usize>(param_1 + 0x20);
    let cnt_28 = rd::<u32>(param_1 + 0x28) as usize;
    if arr_20 != arr_20.wrapping_add(cnt_28 * 16) {
        let mut rcx = rd::<u64>(arr_20);
        let mut rbp = rd::<u64>(arr_20 + 0x08).wrapping_add(rcx);
        let mut rbx: usize = 0;
        if rd::<u32>(param_1 + 0x38) != 0 {
            let mut rdi: usize = 0;
            loop {
                let fn_arr = rd::<usize>(param_1 + 0x30);
                let cb = rd::<usize>(fn_arr.wrapping_add(rbx * 8));
                if cb == 0 {
                    let base = rd::<usize>(param_1 + 0x20);
                    rdi = rdi.wrapping_add(0x10);
                    rcx = rd::<u64>(base.wrapping_add(rdi));
                    rbp = rd::<u64>(base.wrapping_add(rdi + 0x08)).wrapping_add(rcx);
                    rbx += 1;
                } else {
                    let next_rcx = exe::call_va!(cb, (rcx, rbp) -> u64);
                    rcx = next_rcx;
                    if next_rcx != 0 {
                        rbx += 1;
                    } else {
                        let base = rd::<usize>(param_1 + 0x20);
                        rdi = rdi.wrapping_add(0x10);
                        rcx = rd::<u64>(base.wrapping_add(rdi));
                        rbp = rd::<u64>(base.wrapping_add(rdi + 0x08)).wrapping_add(rcx);
                    }
                }
                if rbx >= (rd::<u32>(param_1 + 0x38) as usize) {
                    break;
                }
            }
        }

        let mut cur2 = rd::<usize>(param_1 + 0x20);
        let end2 = cur2.wrapping_add((rd::<u32>(param_1 + 0x28) as usize) * 16);
        while cur2 != end2 {
            slab_free_ptr(rd::<usize>(cur2));
            cur2 = cur2.wrapping_add(16);
        }
        wr::<u32>(param_1 + 0x28, 0);
        wr::<u32>(param_1 + 0x38, 0);
    }
}

/// `r_1420b1080`
/// Original: 0x1420b1080 (66 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420b1080(param_1: usize) {
    let mut a1 = rd::<[u64; 2]>(param_1 + 0x08);
    let mut a2 = rd::<[u64; 2]>(param_1 + 0x18);
    let a3 = rd::<u64>(param_1 + 0x28);
    let mut a4 = rd::<[u64; 2]>(param_1 + 0x30);
    exe::call_va!(
        0x1_420b_2b50,
        (
            a1.as_mut_ptr() as usize,
            a2.as_mut_ptr() as usize,
            a3,
            a4.as_mut_ptr() as usize
        )
    );
}

/// `r_1420b3040`
/// Original: 0x1420b3040 (536 bytes, Throws::No)
pub unsafe extern "win64" fn r_1420b3040(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u64,
    param_5: usize,
) -> usize {
    let head = rd::<u32>(param_1 + 0x18048);
    let idx_u16: u16 = if head == u32::MAX {
        let v = rd::<u16>(param_5);
        wr::<u64>(param_5, rd::<u64>(param_5).wrapping_add(1));
        v
    } else {
        let next = rd::<u32>(param_1.wrapping_add((head as usize) * 4 + 0x10048));
        wr::<u32>(param_1 + 0x18048, next);
        wr::<u32>(param_1.wrapping_add(((head as u16) as usize) * 4 + 0x10048), 0);
        head as u16
    };

    let span_len = rd::<u64>(param_3) as usize;
    let idx = idx_u16 as usize;
    wr::<u32>(param_1.wrapping_add(idx * 4 + 0x10048), 0);

    let span_ptr = rd::<usize>(param_3 + 0x08);
    let mut zero_cnt: u64 = 0;
    if span_len != 0 {
        let tbl_base = rd::<usize>(rd::<usize>(param_1));
        for i in 0..span_len {
            let w = rd::<u16>(span_ptr.wrapping_add(i * 2)) as usize;
            if rd::<u32>(tbl_base.wrapping_add(w * 0x50 + 0x24)) == 0 {
                zero_cnt += 1;
            }
        }
    }

    let alloc_bytes = zero_cnt
        .wrapping_mul(6)
        .wrapping_add((span_len as u64).wrapping_mul(2));
    let buf_ptr = exe::call_va!(0x1_4248_ca80, (param_1 + 0x58050, alloc_bytes) -> usize);
    let len_lo = rd::<u32>(param_3);
    let len_hi = zero_cnt as u32;

    let src1 = rd::<usize>(param_3 + 0x08);
    let mut dst1 = buf_ptr;
    for i in 0..span_len {
        wr::<u16>(dst1, rd::<u16>(src1.wrapping_add(i * 2)));
        dst1 = dst1.wrapping_add(2);
    }

    let src2 = rd::<usize>(param_3 + 0x08);
    let mut dst2 = dst1;
    for i in 0..span_len {
        let w = rd::<u16>(src2.wrapping_add(i * 2));
        let tbl_base = rd::<usize>(rd::<usize>(param_1));
        if rd::<u32>(tbl_base.wrapping_add((w as usize) * 0x50 + 0x24)) == 0 {
            wr::<u16>(dst2, w);
            dst2 = dst2.wrapping_add(2);
        }
    }

    let slot_ptr = param_1.wrapping_add(idx * 0x20 + 0x18050);
    wr::<usize>(slot_ptr, buf_ptr);
    wr::<usize>(slot_ptr + 0x08, dst1);
    wr::<usize>(slot_ptr + 0x10, dst2);
    wr::<u64>(
        slot_ptr + 0x18,
        (len_lo as u64) | ((len_hi as u64) << 32),
    );
    wr::<u16>(param_2, idx_u16);

    let mut rem = rd::<u32>(param_1 + 0x20) as usize;
    let arr_18 = rd::<usize>(param_1 + 0x18);
    let mut pos: usize = 0;
    while rem != 0 {
        let half = rem >> 1;
        let mid = pos.wrapping_add(half);
        if param_4 > rd::<u64>(arr_18.wrapping_add(mid * 8)) {
            pos = mid + 1;
            rem = rem - half - 1;
        } else {
            rem = half;
        }
    }

    let mut iter_out: usize = param_2;
    let mut p4_slot: u64 = param_4;
    exe::call_va!(
        0x1_411f_0610,
        (
            param_1 + 0x18,
            (&mut iter_out as *mut usize) as usize,
            arr_18.wrapping_add(pos * 8),
            (&mut p4_slot as *mut u64) as usize
        )
    );
    let arr_28 = rd::<usize>(param_1 + 0x28);
    exe::call_va!(
        0x1_411f_00d0,
        (
            param_1 + 0x28,
            (&mut iter_out as *mut usize) as usize,
            arr_28.wrapping_add(pos * 2),
            param_2
        )
    );
    exe::call_va!(0x1_408b_4ad0, (param_1 + 0x38, rd::<u64>(param_5)));
    let arr_38 = rd::<usize>(param_1 + 0x38);
    wr::<u64>(arr_38.wrapping_add(idx * 8), param_4);
    param_2
}

/// `r_1420b5520`
/// Original: 0x1420b5520 (69 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1420b5520(param_1: usize) {
    let mut pair_10 = rd::<[u64; 2]>(param_1 + 0x10);
    exe::call_va!(
        0x1_420c_43d0,
        (
            rd::<u64>(param_1 + 0x08),
            pair_10.as_mut_ptr() as usize,
            rd::<u64>(param_1 + 0x20),
            rd::<u64>(param_1 + 0x28),
            rd::<u64>(param_1 + 0x30),
            rd::<u64>(param_1 + 0x38),
            rd::<u64>(param_1 + 0x40)
        )
    );
}

/// `r_1420b5c00`
/// Original: 0x1420b5c00 (73 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1420b5c00(param_1: usize) {
    let mut pair_08 = rd::<[u64; 2]>(param_1 + 0x08);
    let mut pair_20 = rd::<[u64; 2]>(param_1 + 0x20);
    exe::call_va!(
        0x1_420b_9480,
        (
            pair_08.as_mut_ptr() as usize,
            rd::<u64>(param_1 + 0x18),
            pair_20.as_mut_ptr() as usize,
            rd::<u64>(param_1 + 0x30)
        )
    );
}

/// `unknown_1420b9480`
/// Original: 0x1420b9480 (391 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1420b9480(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let len1 = rd::<usize>(param_1);
    let mut cur1 = rd::<usize>(param_1 + 0x08);
    let end1 = cur1.wrapping_add(len1 * 8);
    let mut out_cnt: u32 = 0;

    while cur1 != end1 {
        let v = rd::<u64>(cur1);
        let lo32 = v as u32;
        if lo32 != u32::MAX {
            let packed = (lo32 as u64) | (((v >> 48) & 0xffff) << 32);
            let dst = rd::<usize>(param_4 + 0x08);
            wr::<u64>(dst.wrapping_add((out_cnt as usize) * 8), packed);
            out_cnt = out_cnt.wrapping_add(1);
        }
        cur1 = cur1.wrapping_add(8);
    }

    let mut node = rd::<usize>(param_2 + 0x78);
    let mut idx: u64 = 0;
    let tail_node = rd::<usize>(param_2 + 0x80);
    let tail_idx: u64 = if tail_node != 0 {
        rd::<u64>(tail_node + 0x08)
    } else {
        0
    };

    while node != tail_node || idx != tail_idx {
        let tbl_idx = rd::<u32>(node.wrapping_add((idx as usize) * 4 + 0x10)) as usize;
        idx = idx.wrapping_add(1);
        let src_arr = rd::<usize>(param_3 + 0x08);
        let v = rd::<u64>(src_arr.wrapping_add(tbl_idx * 8));
        let packed = (v >> 32) | ((v & 0xffff) << 32);
        let dst = rd::<usize>(param_4 + 0x08);
        wr::<u64>(dst.wrapping_add((out_cnt as usize) * 8), packed);
        out_cnt = out_cnt.wrapping_add(1);
        if idx == rd::<u64>(node + 0x08) {
            let next = rd::<usize>(node);
            if next != 0 {
                node = next;
                idx = 0;
            }
        }
    }

    exe::call_va!(0x1_411f_2040, (param_4, out_cnt as u64));
    exe::call_va!(
        0x1_40fa_e550,
        (rd::<usize>(param_4 + 0x08), rd::<u32>(param_4 + 0x10))
    );

    let begin = rd::<usize>(param_4 + 0x08);
    let len = rd::<u32>(param_4 + 0x10) as usize;
    let mut end = begin.wrapping_add(len * 8);
    if begin != end && begin.wrapping_add(8) != end {
        let mut prev = begin;
        let mut cur = begin.wrapping_add(8);
        let mut prev_val = rd::<u64>(begin);
        loop {
            let cur_val = rd::<u64>(cur);
            let next = cur.wrapping_add(8);
            if prev_val == cur_val {
                let mut read_ptr = next;
                while read_ptr != end {
                    let v = rd::<u64>(read_ptr);
                    if rd::<u64>(prev) != v {
                        prev = prev.wrapping_add(8);
                        wr::<u64>(prev, v);
                    }
                    read_ptr = read_ptr.wrapping_add(8);
                }
                end = prev.wrapping_add(8);
                break;
            }
            prev = cur;
            cur = next;
            prev_val = cur_val;
            if next == end {
                break;
            }
        }
    }

    let new_len = ((end as isize - rd::<usize>(param_4 + 0x08) as isize) >> 3) as u64;
    exe::call_va!(0x1_411f_2040, (param_4, new_len));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14206e380(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_LOG: [u64; 16] = [0; 16];

    unsafe extern "win64" fn mock_e7f0() {
        TRACE_LOG[0] += 1;
    }
    unsafe extern "win64" fn mock_61a0(p: usize) {
        wr::<u32>(p, rd::<u32>(p).wrapping_add(0x11));
    }
    unsafe extern "win64" fn mock_df80(val: u64) {
        TRACE_LOG[1] = val;
    }
    unsafe extern "win64" fn mock_6db0(p: usize) {
        TRACE_LOG[2] = rd::<u64>(p);
    }
    unsafe extern "win64" fn mock_queue_signal(_q: usize, fence: u64, val: u64) -> i32 {
        TRACE_LOG[3] = fence.wrapping_add(val);
        0
    }
    unsafe extern "win64" fn mock_95c0(idx: u32) {
        TRACE_LOG[4 + idx as usize] += 1;
    }
    unsafe extern "win64" fn mock_9c20(a: u8, b: u32) {
        TRACE_LOG[6 + b as usize] = (a as u64) + 10;
    }
    unsafe extern "win64" fn mock_6940() {
        TRACE_LOG[8] += 1;
    }
    unsafe extern "win64" fn mock_6b80(end_ptr: usize, start_idx: u64, total: u64, vec_addr: usize) {
        let base = rd::<usize>(vec_addr);
        TRACE_LOG[9] = (end_ptr.wrapping_sub(base) as u64)
            .wrapping_add(start_idx << 16)
            .wrapping_add(total << 32);
    }
    unsafe extern "win64" fn mock_7430(rsi: usize) {
        TRACE_LOG[10] = rd::<u64>(rsi);
    }
    unsafe extern "win64" fn mock_7d80(rsi: usize) {
        TRACE_LOG[11] = rd::<u64>(rsi + 0x08);
    }
    unsafe extern "win64" fn mock_fence_get_completed(fence: usize) -> u64 {
        rd::<u64>(fence + 0x08)
    }
    unsafe extern "win64" fn mock_ad10(rcx: usize, shift_diff: u64, completed: u64) {
        TRACE_LOG[12] = (rcx as u64) ^ (shift_diff << 16) ^ completed;
    }
    unsafe extern "win64" fn mock_6f60(rsi: usize) {
        TRACE_LOG[13] = rd::<u64>(rsi);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_e7f0, mock_e7f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_61a0, mock_61a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4206_df80, mock_df80 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_6db0, mock_6db0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4209_95c0, mock_95c0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4209_9c20, mock_9c20 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4205_6940, mock_6940 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_411d_6b80, mock_6b80 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4208_7430, mock_7430 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4208_7d80, mock_7d80 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4208_ad10, mock_ad10 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4208_6f60, mock_6f60 as *const ())?;

    let g_f28 = exe::ptr::<u8>(0x1_4385_7f28) as usize;
    let g_fcd = exe::ptr::<u8>(0x1_4384_3fcd) as usize;
    let tls_slot = exe::tls_pointer();
    let g_c60 = exe::ptr::<usize>(0x1_4397_fc60) as usize;
    let g_c58 = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let g_250 = exe::ptr::<u32>(0x1_43a6_3250) as usize;
    let g_248 = exe::ptr::<usize>(0x1_43a6_3248) as usize;
    let g_918 = exe::ptr::<u64>(0x1_43a3_5918) as usize;
    let g_298 = exe::ptr::<usize>(0x1_43a0_8298) as usize;

    let saved = (
        rd::<u8>(g_f28),
        rd::<u8>(g_fcd),
        rd::<usize>(tls_slot),
        rd::<usize>(g_c60),
        rd::<usize>(g_c58),
        rd::<u32>(g_250),
        rd::<usize>(g_248),
        rd::<u64>(g_918),
        rd::<usize>(g_298),
    );

    let run = |f: F| -> ([u64; 16], u8, u32, u64, u8, u8, u32) {
        TRACE_LOG = [0; 16];
        wr::<u8>(g_f28, 7);
        wr::<u8>(g_fcd, 0);

        let mut tls_buf = vec![0u8; 0x46800];
        let tp = tls_buf.as_mut_ptr() as usize;
        wr::<u8>(tp + 0x70, 1);
        wr::<u32>(tp + 0x467e0, 0x100);
        wr::<u64>(tp + 0x6328, 0xdead_beef);
        wr::<usize>(tls_slot, tp);

        let mut q_vtbl = [0usize; 16];
        q_vtbl[0x70 / 8] = mock_queue_signal as *const () as usize;
        let q_obj: [usize; 1] = [q_vtbl.as_ptr() as usize];

        let mut c60_obj = [0u64; 4];
        c60_obj[0] = 0x5555;
        c60_obj[2] = q_obj.as_ptr() as usize as u64;
        wr::<usize>(g_c60, c60_obj.as_mut_ptr() as usize);

        let mut f_vtbl = [0usize; 10];
        f_vtbl[0x40 / 8] = mock_fence_get_completed as *const () as usize;
        let fence_100: [u64; 2] = [f_vtbl.as_ptr() as usize as u64, 50];

        let arr_98: [u64; 4] = [1, 40, 2, 60];
        let mut rdi_obj = [0u8; 0x220];
        let rdip = rdi_obj.as_mut_ptr() as usize;
        wr::<usize>(rdip + 0x98, arr_98.as_ptr() as usize);
        wr::<u32>(rdip + 0xa0, 2);
        wr::<u8>(rdip + 0xa8, 0);
        wr::<usize>(rdip + 0x100, fence_100.as_ptr() as usize);
        wr::<usize>(rdip + 0x110, 0x9abc);
        wr::<u64>(rdip + 0x210, 0x0200_0000);
        wr::<u8>(rdip + 0x218, 1);

        let mut rsi_obj = [0u64; 4];
        rsi_obj[0] = 0x0600_0000;
        rsi_obj[1] = 0x7777;
        rsi_obj[3] = rdip as u64;

        let mut c58_obj = [0u8; 0x70];
        let c58p = c58_obj.as_mut_ptr() as usize;
        wr::<u64>(c58p + 0x38, 0x1000);
        wr::<u64>(c58p + 0x40, 25);
        wr::<usize>(c58p + 0x68, rsi_obj.as_mut_ptr() as usize);
        wr::<usize>(g_c58, c58p);

        // 2 entries of 0x28 bytes; entry 0 not expired, entry 1 expired
        let mut arr_248 = [0u8; 0x50];
        let a248p = arr_248.as_mut_ptr() as usize;
        wr::<u64>(a248p + 0x08, 5000);
        wr::<u8>(a248p + 0x08 + 0x19, 1);
        wr::<u64>(a248p + 0x28 + 0x08, 100);
        wr::<u8>(a248p + 0x28 + 0x08 + 0x19, 1);
        wr::<u32>(g_250, 2);
        wr::<usize>(g_248, a248p);
        wr::<u64>(g_918, 2000);

        let mut g298_obj = [0u8; 0x260];
        let g298p = g298_obj.as_mut_ptr() as usize;
        wr::<u32>(g298p + 0x258, 10);
        wr::<usize>(g_298, g298p);

        f();

        (
            TRACE_LOG,
            rd::<u8>(g_fcd),
            rd::<u32>(tp + 0x467e0),
            rd::<u64>(c58p + 0x40),
            rd::<u8>(rdip + 0xa8),
            rd::<u8>(rdip + 0x218),
            rd::<u32>(g298p + 0x258),
        )
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<u8>(g_f28, saved.0);
    wr::<u8>(g_fcd, saved.1);
    wr::<usize>(tls_slot, saved.2);
    wr::<usize>(g_c60, saved.3);
    wr::<usize>(g_c58, saved.4);
    wr::<u32>(g_250, saved.5);
    wr::<usize>(g_248, saved.6);
    wr::<u64>(g_918, saved.7);
    wr::<usize>(g_298, saved.8);

    crate::diff_eq!(o, n, "14206e380 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142075080(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i8, u64, u64, u64, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SRV_LOG: [[u32; 10]; 2] = [[0; 10]; 2];
    static mut SRV_CNT: usize = 0;
    static mut CBV_LOG: [u64; 2] = [0; 2];
    static mut UAV_LOG: [[u32; 12]; 2] = [[0; 12]; 2];
    static mut UAV_CNT: usize = 0;
    static mut GROW_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_3aa0(pool: usize, chunk_sz: u32) {
        GROW_CALLS += chunk_sz;
        wr::<u32>(pool + 0x20, 4);
    }
    unsafe extern "win64" fn mock_get_gpu_va(res: usize) -> u64 {
        rd::<u64>(res + 0x08)
    }
    unsafe extern "win64" fn mock_create_cbv(_dev: usize, desc: usize, _handle: u64) {
        CBV_LOG = rd::<[u64; 2]>(desc);
    }
    unsafe extern "win64" fn mock_create_srv(_dev: usize, _res: usize, desc: usize, _handle: u64) {
        if SRV_CNT < 2 {
            SRV_LOG[SRV_CNT] = rd::<[u32; 10]>(desc);
            SRV_CNT += 1;
        }
    }
    unsafe extern "win64" fn mock_create_uav(
        _dev: usize,
        _res: usize,
        _counter: usize,
        desc: usize,
        _handle: u64,
    ) {
        if UAV_CNT < 2 {
            UAV_LOG[UAV_CNT] = rd::<[u32; 12]>(desc);
            UAV_CNT += 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_3aa0, mock_3aa0 as *const ())?;

    let g_cb8 = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let g_c80 = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let s_cb8 = rd::<usize>(g_cb8);
    let s_c80 = rd::<usize>(g_c80);

    let run = |f: F, p3: i8, p4: u64| -> (
        [u64; 8],
        [[u32; 10]; 2],
        [u64; 2],
        [[u32; 12]; 2],
        u32,
        u32,
        u32,
    ) {
        SRV_LOG = [[0; 10]; 2];
        SRV_CNT = 0;
        CBV_LOG = [0; 2];
        UAV_LOG = [[0; 12]; 2];
        UAV_CNT = 0;
        GROW_CALLS = 0;

        let srv_ring: [u64; 4] = [0x1001, 0x1002, 0x1003, 0x1004];
        let uav_ring: [u64; 4] = [0x2001, 0x2002, 0x2003, 0x2004];
        let mut mgr = [0u8; 0x220];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<u32>(mp + 0x38, u32::MAX);
        // Pool at +0xc0: start with avail = 0 to trigger mock_3aa0
        wr::<usize>(mp + 0xc0 + 0x10, srv_ring.as_ptr() as usize);
        wr::<u32>(mp + 0xc0 + 0x18, 3);
        wr::<u32>(mp + 0xc0 + 0x20, 0);
        wr::<u32>(mp + 0xc0 + 0x24, 4);
        wr::<u32>(mp + 0xc0 + 0x44, 16);
        // Pool at +0x1c8: avail = 3
        wr::<usize>(mp + 0x1c8 + 0x10, uav_ring.as_ptr() as usize);
        wr::<u32>(mp + 0x1c8 + 0x18, 0);
        wr::<u32>(mp + 0x1c8 + 0x20, 3);
        wr::<u32>(mp + 0x1c8 + 0x24, 4);
        wr::<u32>(mp + 0x1c8 + 0x44, 32);
        wr::<usize>(g_cb8, mp);

        let mut dev_vtbl = [0usize; 24];
        dev_vtbl[0x88 / 8] = mock_create_cbv as *const () as usize;
        dev_vtbl[0x90 / 8] = mock_create_srv as *const () as usize;
        dev_vtbl[0x98 / 8] = mock_create_uav as *const () as usize;
        let dev_obj: [usize; 1] = [dev_vtbl.as_ptr() as usize];
        wr::<usize>(g_c80, dev_obj.as_ptr() as usize);

        let mut res_vtbl = [0usize; 16];
        res_vtbl[0x58 / 8] = mock_get_gpu_va as *const () as usize;
        let res_obj: [u64; 2] = [res_vtbl.as_ptr() as usize as u64, 0x8000_0000];

        let mut out_p1 = [0u64; 8];
        f(
            out_p1.as_mut_ptr() as usize,
            res_obj.as_ptr() as usize,
            p3,
            p4,
            0x20,
            0x40,
            0x19, // bits 0 (SRV), 3 (CBV), 4 (UAV)
        );
        (
            out_p1,
            SRV_LOG,
            CBV_LOG,
            UAV_LOG,
            GROW_CALLS,
            rd::<u32>(mp + 0xc0 + 0x18),
            rd::<u32>(mp + 0x1c8 + 0x18),
        )
    };

    for (p3, p4) in [(0i8, 16u64), (0, 0), (1, 0)] {
        let o = run(f_o, p3, p4);
        let n = run(f_n, p3, p4);
        wr::<usize>(g_cb8, s_cb8);
        wr::<usize>(g_c80, s_c80);
        crate::diff_eq!(o, n, "142075080 p3={} p4={}", p3, p4);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1420756b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u32, u32, usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LAST_HEAP_PROPS: [u32; 5] = [0; 5];
    static mut LAST_HEAP_FLAGS: u32 = 0;
    static mut LAST_RES_DESC: [u64; 6] = [0; 6];
    static mut LAST_RES_FLAGS: u32 = 0;
    static mut LAST_STATE: u32 = 0;

    unsafe extern "win64" fn mock_create_committed(
        _dev: usize,
        heap_props: usize,
        heap_flags: u32,
        res_desc: usize,
        initial_state: u32,
        _clear_val: usize,
        _iid: usize,
        out_res: usize,
    ) -> i32 {
        LAST_HEAP_PROPS = rd::<[u32; 5]>(heap_props);
        LAST_HEAP_FLAGS = heap_flags;
        // Zero padding at +0x04 before reading 48 bytes
        let mut d = rd::<[u64; 6]>(res_desc);
        d[0] &= 0xffff_ffff;
        LAST_RES_DESC = d;
        LAST_RES_FLAGS = rd::<u32>(res_desc + 0x30);
        LAST_STATE = initial_state;
        wr::<u64>(out_res, 0x1234_5678);
        0
    }

    let g_c80 = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let g_c58 = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let s_c80 = rd::<usize>(g_c80);
    let s_c58 = rd::<usize>(g_c58);

    let run = |f: F, p2: u32, p3: u32, a0_flag: u8| -> (u32, [u32; 5], u32, [u64; 6], u32, u32, u64)
    {
        LAST_HEAP_PROPS = [0; 5];
        LAST_HEAP_FLAGS = 0;
        LAST_RES_DESC = [0; 6];
        LAST_RES_FLAGS = 0;
        LAST_STATE = 0;

        let mut dev_vtbl = [0usize; 30];
        dev_vtbl[0xd8 / 8] = mock_create_committed as *const () as usize;
        let dev_obj: [usize; 1] = [dev_vtbl.as_ptr() as usize];
        wr::<usize>(g_c80, dev_obj.as_ptr() as usize);

        let mut c58_obj = [0u8; 0xb0];
        c58_obj[0xa0] = a0_flag;
        wr::<usize>(g_c58, c58_obj.as_ptr() as usize);

        let mut out_res: u64 = 0;
        let ret = f(0x4000, p2, p3, (&mut out_res as *mut u64) as usize);
        (
            ret,
            LAST_HEAP_PROPS,
            LAST_HEAP_FLAGS,
            LAST_RES_DESC,
            LAST_RES_FLAGS,
            LAST_STATE,
            out_res,
        )
    };

    for (p2, p3, a0) in [(0x37u32, 0u32, 1u8), (0x10, 0x10, 0), (0x00, 0x20, 1)] {
        let o = run(f_o, p2, p3, a0);
        let n = run(f_n, p2, p3, a0);
        wr::<usize>(g_c80, s_c80);
        wr::<usize>(g_c58, s_c58);
        crate::diff_eq!(o, n, "1420756b0 p2={:#x} p3={:#x}", p2, p3);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142075bc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CMD_CTX_LOCK: usize = 0;
    static mut CMD_CTX_LIST: usize = 0;
    static mut LIST_TYPE: i32 = 0;
    static mut BARRIER_CNT: usize = 0;
    static mut BARRIER_COUNTS: [u32; 4] = [0; 4];
    static mut BARRIER_DATA: [[u64; 8]; 4] = [[0; 8]; 4];
    static mut COPY_ARGS: [u64; 5] = [0; 5];

    unsafe extern "win64" fn mock_5b00(out_ctx: usize) {
        wr::<usize>(out_ctx, CMD_CTX_LOCK);
        wr::<usize>(out_ctx + 0x08, CMD_CTX_LIST);
    }
    unsafe extern "win64" fn mock_get_type(_list: usize) -> i32 {
        LIST_TYPE
    }
    unsafe extern "win64" fn mock_copy_buf(
        _list: usize,
        dst_res: u64,
        dst_off: u64,
        src_res: u64,
        src_off: u64,
        num_bytes: u64,
    ) {
        COPY_ARGS = [dst_res, dst_off, src_res, src_off, num_bytes];
    }
    unsafe extern "win64" fn mock_barrier(_list: usize, count: u32, barriers_ptr: usize) {
        let mut data = [0u64; 8];
        for i in 0..(count as usize) {
            let b = rd::<[u64; 4]>(barriers_ptr + i * 32);
            data[i * 4..i * 4 + 4].copy_from_slice(&b);
        }
        let idx = BARRIER_CNT;
        if idx < 4 {
            BARRIER_COUNTS[idx] = count;
            BARRIER_DATA[idx] = data;
            BARRIER_CNT = idx + 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_5b00, mock_5b00 as *const ())?;

    let run = |f: F, ltype: i32, src_st: u32, dst_st: u32| -> (
        usize,
        [u32; 4],
        [[u64; 8]; 4],
        [u64; 5],
        u32,
        u32,
    ) {
        BARRIER_CNT = 0;
        BARRIER_COUNTS = [0; 4];
        BARRIER_DATA = [[0; 8]; 4];
        COPY_ARGS = [0; 5];
        LIST_TYPE = ltype;

        let mut lock_obj = [0u8; 0x10];
        let lp = lock_obj.as_mut_ptr() as usize;
        wr::<u32>(lp + 0x08, 0x1234);
        wr::<u32>(lp + 0x0c, 1);
        exe::call_iat!(0x1_4264_55a8, (lp));
        CMD_CTX_LOCK = lp;

        let mut list_vtbl = [0usize; 30];
        list_vtbl[0x40 / 8] = mock_get_type as *const () as usize;
        list_vtbl[0x78 / 8] = mock_copy_buf as *const () as usize;
        list_vtbl[0xd0 / 8] = mock_barrier as *const () as usize;
        let list_obj: [usize; 1] = [list_vtbl.as_ptr() as usize];
        CMD_CTX_LIST = list_obj.as_ptr() as usize;

        let mut dst_buf = [0u8; 0x50];
        let dp = dst_buf.as_mut_ptr() as usize;
        wr::<u64>(dp + 0x40, 0xaaaa_0001);
        wr::<u32>(dp + 0x48, dst_st);

        let mut src_buf = [0u8; 0x50];
        let sp = src_buf.as_mut_ptr() as usize;
        wr::<u64>(sp + 0x40, 0xbbbb_0002);
        wr::<u32>(sp + 0x48, src_st);

        f(dp, 0x100, sp, 0x200, 0x400);
        (
            BARRIER_CNT,
            BARRIER_COUNTS,
            BARRIER_DATA,
            COPY_ARGS,
            rd::<u32>(lp + 0x08),
            rd::<u32>(lp + 0x0c),
        )
    };

    for (lt, sst, dst) in [
        (3i32, 0x40u32, 0x80u32),
        (0, 0x40, 0x80),
        (0, 0x800, 0x80),
        (0, 0x40, 0x400),
    ] {
        let o = run(f_o, lt, sst, dst);
        let n = run(f_n, lt, sst, dst);
        crate::diff_eq!(o, n, "142075bc0 lt={} sst={:#x} dst={:#x}", lt, sst, dst);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_142076960(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCRATCH_ALLOC: [u64; 8] = [0; 8];
    static mut FREED_COUNT: u32 = 0;
    static mut EXEC_LOG: [u64; 4] = [0; 4];
    static mut EXEC_CNT: usize = 0;
    static mut SIGNAL_LOG: [(u64, u64); 4] = [(0, 0); 4];
    static mut SIGNAL_CNT: usize = 0;
    static mut CB_LOG: [(u64, u64); 4] = [(0, 0); 4];
    static mut CB_CNT: usize = 0;

    unsafe extern "win64" fn mock_alloc(_sz: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(SCRATCH_ALLOC) as usize
    }
    unsafe extern "win64" fn mock_free(_ptr: usize) {
        FREED_COUNT += 1;
    }
    unsafe extern "win64" fn mock_exec_lists(_q: usize, count: u32, lists_ptr: usize) {
        for i in 0..(count as usize) {
            if EXEC_CNT < 4 {
                EXEC_LOG[EXEC_CNT] = rd::<u64>(lists_ptr + i * 8);
                EXEC_CNT += 1;
            }
        }
    }
    unsafe extern "win64" fn mock_signal(_q: usize, fence: u64, val: u64) {
        if SIGNAL_CNT < 4 {
            SIGNAL_LOG[SIGNAL_CNT] = (fence, val);
            SIGNAL_CNT += 1;
        }
    }
    unsafe extern "win64" fn mock_cb_vfunc0(_obj: usize, a: u64, b: u64) {
        if CB_CNT < 4 {
            CB_LOG[CB_CNT] = (a, b);
            CB_CNT += 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;

    let g_c30 = exe::ptr::<usize>(0x1_4397_fc30) as usize;
    let s_c30 = rd::<usize>(g_c30);

    let run = |f: F| -> (
        [u64; 4],
        [(u64, u64); 4],
        [(u64, u64); 4],
        u32,
        [u8; 2],
        u32,
        u32,
        u32,
        u32,
    ) {
        SCRATCH_ALLOC = [0; 8];
        FREED_COUNT = 0;
        EXEC_LOG = [0; 4];
        EXEC_CNT = 0;
        SIGNAL_LOG = [(0, 0); 4];
        SIGNAL_CNT = 0;
        CB_LOG = [(0, 0); 4];
        CB_CNT = 0;

        let mut q_vtbl = [0usize; 16];
        q_vtbl[0x50 / 8] = mock_exec_lists as *const () as usize;
        q_vtbl[0x70 / 8] = mock_signal as *const () as usize;
        let q_obj: [usize; 1] = [q_vtbl.as_ptr() as usize];

        let mut item0 = [0u8; 0x30];
        let i0p = item0.as_mut_ptr() as usize;
        wr::<u64>(i0p + 0x08, 101);
        wr::<u64>(i0p + 0x10, 0xf001);
        wr::<u64>(i0p + 0x28, 0xc001);

        let mut item1 = [0u8; 0x30];
        let i1p = item1.as_mut_ptr() as usize;
        wr::<u64>(i1p + 0x08, 102);
        wr::<u64>(i1p + 0x10, 0xf002);
        wr::<u64>(i1p + 0x28, 0xc002);

        let arr_28: [usize; 2] = [i0p, i1p];

        let mut lock_entries = [0u8; 32];
        let lep = lock_entries.as_mut_ptr() as usize;
        wr::<usize>(lep, i0p);
        wr::<u8>(lep + 0x08, 1);
        wr::<usize>(lep + 0x10, i1p);
        wr::<u8>(lep + 0x18, 1);

        let mut lock_obj = [0u8; 0x20];
        let lop = lock_obj.as_mut_ptr() as usize;
        wr::<usize>(lop + 0x08, lep);
        wr::<u32>(lop + 0x10, 2);

        let cb_vtbl: [usize; 1] = [mock_cb_vfunc0 as *const () as usize];
        let cb_obj: [usize; 1] = [cb_vtbl.as_ptr() as usize];
        let sub_items: [u64; 3] = [cb_obj.as_ptr() as usize as u64, 0xaa, 0xbb];

        let mut r15_obj = [0u8; 0x20];
        let r15p = r15_obj.as_mut_ptr() as usize;
        wr::<usize>(r15p, sub_items.as_ptr() as usize);
        wr::<u32>(r15p + 0x08, 1);
        wr::<u64>(r15p + 0x18, 0x9999);
        let arr_18: [usize; 1] = [r15p];

        let mut ring_buf = [0usize; 4];
        let mut c30_obj = [0u8; 0x30];
        let c30p = c30_obj.as_mut_ptr() as usize;
        wr::<u32>(c30p + 0x08, u32::MAX);
        wr::<usize>(c30p + 0x10, ring_buf.as_mut_ptr() as usize);
        wr::<u32>(c30p + 0x1c, 1);
        wr::<u32>(c30p + 0x20, 1);
        wr::<u32>(c30p + 0x24, 2);
        wr::<usize>(g_c30, c30p);

        let mut p1 = [0u8; 0x38];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, lop);
        wr::<usize>(p1p + 0x10, q_obj.as_ptr() as usize);
        wr::<usize>(p1p + 0x18, arr_18.as_ptr() as usize);
        wr::<u32>(p1p + 0x20, 1);
        wr::<usize>(p1p + 0x28, arr_28.as_ptr() as usize);
        wr::<u32>(p1p + 0x30, 2);

        f(p1p);

        (
            EXEC_LOG,
            SIGNAL_LOG,
            CB_LOG,
            FREED_COUNT,
            [rd::<u8>(lep + 0x08), rd::<u8>(lep + 0x18)],
            rd::<u32>(c30p + 0x1c),
            rd::<u32>(c30p + 0x20),
            rd::<u32>(p1p + 0x20),
            rd::<u32>(p1p + 0x30),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_c30, s_c30);
    crate::diff_eq!(o, n, "142076960 out");
    Ok(1)
}

pub unsafe fn diff_r_1420ad980(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA: [u8; 4096] = [0; 4096];
    static mut ARENA_OFF: usize = 0;
    static mut FREE_COUNT: u32 = 0;

    unsafe fn arena_alloc(bytes: usize) -> usize {
        let aligned = (ARENA_OFF + 15) & !15;
        let ptr = (core::ptr::addr_of_mut!(ARENA) as usize) + aligned;
        ARENA_OFF = aligned + bytes.max(16);
        ptr
    }

    unsafe extern "win64" fn mock_c0a0(sz: u64, _align: u32) -> usize {
        arena_alloc(sz as usize)
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREE_COUNT += 1;
    }
    unsafe extern "win64" fn mock_9ea0(bkt_vec: usize, item_ptr: usize) {
        let len = rd::<u32>(bkt_vec + 0x08) as usize;
        let cap = rd::<u32>(bkt_vec + 0x0c) as usize;
        if len == cap {
            let new_cap = (cap + 4).max(4);
            let new_buf = arena_alloc(new_cap * 2);
            let old_buf = rd::<usize>(bkt_vec);
            if old_buf != 0 && len != 0 {
                core::ptr::copy_nonoverlapping(old_buf as *const u8, new_buf as *mut u8, len * 2);
            }
            wr::<usize>(bkt_vec, new_buf);
            wr::<u32>(bkt_vec + 0x0c, new_cap as u32);
        }
        let buf = rd::<usize>(bkt_vec);
        wr::<u16>(buf + len * 2, rd::<u16>(item_ptr));
        wr::<u32>(bkt_vec + 0x08, (len + 1) as u32);
    }
    unsafe extern "win64" fn mock_5d10(vec_u16: usize, min_cap: u64) {
        let len = rd::<u32>(vec_u16 + 0x08) as usize;
        let new_cap = (min_cap as usize).max(8);
        let new_buf = arena_alloc(new_cap * 2);
        let old_buf = rd::<usize>(vec_u16);
        if old_buf != 0 && len != 0 {
            core::ptr::copy_nonoverlapping(old_buf as *const u8, new_buf as *mut u8, len * 2);
        }
        wr::<usize>(vec_u16, new_buf);
        wr::<u32>(vec_u16 + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_bcb0(vec_u64: usize, min_cap: u64) {
        let len = rd::<u32>(vec_u64 + 0x08) as usize;
        let new_cap = (min_cap as usize).max(8);
        let new_buf = arena_alloc(new_cap * 8);
        let old_buf = rd::<usize>(vec_u64);
        if old_buf != 0 && len != 0 {
            core::ptr::copy_nonoverlapping(old_buf as *const u8, new_buf as *mut u8, len * 8);
        }
        wr::<usize>(vec_u64, new_buf);
        wr::<u32>(vec_u64 + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_1b60(vec_0x20: usize, min_cap: u64) {
        let len = rd::<u32>(vec_0x20 + 0x08) as usize;
        let new_cap = (min_cap as usize).max(4);
        let new_buf = arena_alloc(new_cap * 0x20);
        let old_buf = rd::<usize>(vec_0x20);
        if old_buf != 0 && len != 0 {
            core::ptr::copy_nonoverlapping(old_buf as *const u8, new_buf as *mut u8, len * 0x20);
        }
        wr::<usize>(vec_0x20, new_buf);
        wr::<u32>(vec_0x20 + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411e_9ea0, mock_9ea0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40fa_5d10, mock_5d10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4020_bcb0, mock_bcb0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40fb_1b60, mock_1b60 as *const ())?;

    let run = |f: F| -> (Vec<u16>, Vec<u64>, Vec<[u64; 4]>, u32) {
        ARENA = [0; 4096];
        ARENA_OFF = 0;
        FREE_COUNT = 0;

        let mut sub_items = [0u8; 0x48 * 2];
        let sip = sub_items.as_mut_ptr() as usize;
        wr::<u16>(sip, 1);
        wr::<u16>(sip + 0x48, 0);

        let mut items = [0u8; 0x130 * 2];
        let ip = items.as_mut_ptr() as usize;
        // Item 0: id=10, w_a8=0, w_60=1, sub=[1, 0]
        wr::<u16>(ip, 10);
        wr::<u16>(ip + 0x60, 1);
        wr::<u16>(ip + 0xa8, 0);
        wr::<usize>(ip + 0xf0, sip);
        wr::<u32>(ip + 0xf8, 2);

        // Item 1: id=20, w_a8=1, w_60=0xffff, sub=[]
        let i1 = ip + 0x130;
        wr::<u16>(i1, 20);
        wr::<u16>(i1 + 0x60, 0xffff);
        wr::<u16>(i1 + 0xa8, 1);
        wr::<usize>(i1 + 0xf0, 0);
        wr::<u32>(i1 + 0xf8, 0);

        let mut p1 = [0u64; 6];
        let p1p = p1.as_mut_ptr() as usize;
        let mut p2 = [0u8; 0x20];
        wr::<u32>(p2.as_mut_ptr() as usize + 0x18, 2); // 2 buckets
        let mut p3 = [0u8; 0x18];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x08, ip);
        wr::<u32>(p3p + 0x10, 2);

        let ret = f(p1p, p2.as_ptr() as usize, p3p);
        assert_eq!(ret, p1p);

        let w_len = rd::<u32>(p1p + 0x08) as usize;
        let w_ptr = rd::<usize>(p1p);
        let mut w_out = Vec::with_capacity(w_len);
        for i in 0..w_len {
            w_out.push(rd::<u16>(w_ptr + i * 2));
        }

        let q_len = rd::<u32>(p1p + 0x18) as usize;
        let q_ptr = rd::<usize>(p1p + 0x10);
        let mut q_out = Vec::with_capacity(q_len);
        for i in 0..q_len {
            // Mask out the 2 uninitialized C++ struct padding bytes at bits 16..32
            q_out.push(rd::<u64>(q_ptr + i * 8) & 0xffff_ffff_0000_ffff);
        }

        let b_len = rd::<u32>(p1p + 0x28) as usize;
        let b_ptr = rd::<usize>(p1p + 0x20);
        let mut b_out = Vec::with_capacity(b_len);
        for i in 0..b_len {
            let mut row = rd::<[u64; 4]>(b_ptr + i * 0x20);
            row[1] = (row[1] as usize).wrapping_sub(w_ptr) as u64;
            row[3] = (row[3] as usize).wrapping_sub(q_ptr) as u64;
            b_out.push(row);
        }

        (w_out, q_out, b_out, FREE_COUNT)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420ad980 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b0520(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CB_CALLS: [(u64, u64); 8] = [(0, 0); 8];
    static mut CB_CNT: usize = 0;
    static mut LOG_71A0: u32 = 0;

    unsafe extern "win64" fn mock_71a0(_hdr: usize) {
        LOG_71A0 += 1;
    }
    unsafe extern "win64" fn mock_cb_step(cur: u64, end: u64) -> u64 {
        if CB_CNT < 8 {
            CB_CALLS[CB_CNT] = (cur, end);
            CB_CNT += 1;
        }
        if cur + 8 < end {
            cur + 8
        } else {
            0
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_71a0, mock_71a0 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F| -> ([(u64, u64); 8], usize, u32, u32, u32, u32, u32) {
        CB_CALLS = [(0, 0); 8];
        CB_CNT = 0;
        LOG_71A0 = 0;
        core::ptr::write_bytes(seg as *mut u8, 0, 0x1_1000);

        wr::<usize>(seg + 0x100, exe::teb());
        let hdr_raw = seg + 0x108 + 0x50;
        wr::<u32>(hdr_raw + 0x04, 8);
        let hdr = hdr_raw - 8;
        wr::<u8>(hdr + 0x0e, 0);
        wr::<u32>(hdr + 0x18, 2);
        wr::<u64>(hdr + 0x20, 0xfeed_0000);

        let blk1 = seg + 0x1_0000;
        let blk2 = seg + 0x1_0100;

        let arr0: [u64; 2] = [blk1 as u64, 16];
        let arr20: [u64; 4] = [blk2 as u64, 16, (blk2 + 0x20) as u64, 8];
        let fn_arr: [usize; 3] = [
            mock_cb_step as *const () as usize,
            0,
            mock_cb_step as *const () as usize,
        ];

        let mut p1 = [0u64; 8];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, arr0.as_ptr() as usize);
        wr::<u32>(p1p + 0x08, 1);
        wr::<usize>(p1p + 0x20, arr20.as_ptr() as usize);
        wr::<u32>(p1p + 0x28, 2);
        wr::<usize>(p1p + 0x30, fn_arr.as_ptr() as usize);
        wr::<u32>(p1p + 0x38, 3);

        f(p1p);

        let mut rel_calls = [(0u64, 0u64); 8];
        let cnt = CB_CNT;
        let raw_calls = CB_CALLS;
        for i in 0..cnt {
            rel_calls[i] = (
                raw_calls[i].0 - blk2 as u64,
                raw_calls[i].1 - blk2 as u64,
            );
        }
        (
            rel_calls,
            cnt,
            LOG_71A0,
            rd::<u32>(hdr + 0x18),
            rd::<u32>(p1p + 0x08),
            rd::<u32>(p1p + 0x28),
            rd::<u32>(p1p + 0x38),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b0520 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b1080(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: [u64; 7] = [0; 7];

    unsafe extern "win64" fn mock_2b50(a1: usize, a2: usize, a3: u64, a4: usize) {
        let p1 = rd::<[u64; 2]>(a1);
        let p2 = rd::<[u64; 2]>(a2);
        let p4 = rd::<[u64; 2]>(a4);
        CAPTURED = [p1[0], p1[1], p2[0], p2[1], a3, p4[0], p4[1]];
    }

    let _m1 = crate::difftest::Mock::new(0x1_420b_2b50, mock_2b50 as *const ())?;

    let run = |f: F| -> [u64; 7] {
        CAPTURED = [0; 7];
        let mut p1 = [0u64; 8];
        for i in 0..8usize {
            p1[i] = 0x100 + (i as u64) * 0x11;
        }
        f(p1.as_ptr() as usize);
        CAPTURED
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b1080 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b3040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_BUF: [u16; 32] = [0; 32];
    static mut LAST_ALLOC_BYTES: u64 = 0;
    static mut INSERT_610_OFF: usize = 0;
    static mut INSERT_610_VAL: u64 = 0;
    static mut INSERT_0D0_OFF: usize = 0;
    static mut INSERT_0D0_VAL: u16 = 0;
    static mut GROW_4AD0_ARG: u64 = 0;

    unsafe extern "win64" fn mock_ca80(_arena: usize, bytes: u64) -> usize {
        LAST_ALLOC_BYTES = bytes;
        core::ptr::addr_of_mut!(ALLOC_BUF) as usize
    }
    unsafe extern "win64" fn mock_0610(vec_ptr: usize, out_it: usize, pos_ptr: usize, val_ptr: usize) {
        let base = rd::<usize>(vec_ptr);
        INSERT_610_OFF = pos_ptr.wrapping_sub(base);
        INSERT_610_VAL = rd::<u64>(val_ptr);
        wr::<usize>(out_it, pos_ptr);
    }
    unsafe extern "win64" fn mock_00d0(vec_ptr: usize, out_it: usize, pos_ptr: usize, val_ptr: usize) {
        let base = rd::<usize>(vec_ptr);
        INSERT_0D0_OFF = pos_ptr.wrapping_sub(base);
        INSERT_0D0_VAL = rd::<u16>(val_ptr);
        wr::<usize>(out_it, pos_ptr);
    }
    unsafe extern "win64" fn mock_4ad0(_vec_ptr: usize, new_len: u64) {
        GROW_4AD0_ARG = new_len;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_ca80, mock_ca80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411f_0610, mock_0610 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_411f_00d0, mock_00d0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_408b_4ad0, mock_4ad0 as *const ())?;

    let run = |f: F, free_head: u32| -> (
        u16,
        u64,
        [u16; 8],
        u64,
        [u64; 4],
        usize,
        u64,
        usize,
        u16,
        u64,
        u64,
    ) {
        ALLOC_BUF = [0; 32];
        LAST_ALLOC_BYTES = 0;
        INSERT_610_OFF = 0;
        INSERT_610_VAL = 0;
        INSERT_0D0_OFF = 0;
        INSERT_0D0_VAL = 0;
        GROW_4AD0_ARG = 0;

        let mut meta_tbl = [0u8; 0x50 * 4];
        let mp = meta_tbl.as_mut_ptr() as usize;
        wr::<u32>(mp + 1 * 0x50 + 0x24, 0);
        wr::<u32>(mp + 2 * 0x50 + 0x24, 5); // non-zero
        wr::<u32>(mp + 3 * 0x50 + 0x24, 0);
        let root_ptr: [usize; 1] = [mp];

        let arr_18: [u64; 4] = [10, 20, 30, 40];
        let arr_28: [u16; 4] = [0, 1, 2, 3];
        let mut arr_38 = [0u64; 4];

        let mut p1 = vec![0u8; 0x58100];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p, root_ptr.as_ptr() as usize);
        wr::<usize>(p1p + 0x18, arr_18.as_ptr() as usize);
        wr::<u32>(p1p + 0x20, 4);
        wr::<usize>(p1p + 0x28, arr_28.as_ptr() as usize);
        wr::<usize>(p1p + 0x38, arr_38.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x10048 + 2 * 4, 99);
        wr::<u32>(p1p + 0x18048, free_head);

        let span_items: [u16; 3] = [1, 2, 3];
        let span: [u64; 2] = [3, span_items.as_ptr() as usize as u64];
        let mut out_idx: u16 = 0;
        let mut p5_counter: u64 = 1;

        let ret = f(
            p1p,
            (&mut out_idx as *mut u16) as usize,
            span.as_ptr() as usize,
            25,
            (&mut p5_counter as *mut u64) as usize,
        );
        assert_eq!(ret, (&mut out_idx as *mut u16) as usize);

        let base_alloc = core::ptr::addr_of!(ALLOC_BUF) as usize;
        let mut slot = rd::<[u64; 4]>(p1p + (out_idx as usize) * 0x20 + 0x18050);
        slot[0] = (slot[0] as usize).wrapping_sub(base_alloc) as u64;
        slot[1] = (slot[1] as usize).wrapping_sub(base_alloc) as u64;
        slot[2] = (slot[2] as usize).wrapping_sub(base_alloc) as u64;

        let short_buf = rd::<[u16; 8]>(base_alloc);

        (
            out_idx,
            p5_counter,
            short_buf,
            LAST_ALLOC_BYTES,
            slot,
            INSERT_610_OFF,
            INSERT_610_VAL,
            INSERT_0D0_OFF,
            INSERT_0D0_VAL,
            GROW_4AD0_ARG,
            arr_38[out_idx as usize],
        )
    };

    for head in [u32::MAX, 2] {
        let o = run(f_o, head);
        let n = run(f_n, head);
        crate::diff_eq!(o, n, "1420b3040 head={:#x}", head);
    }
    Ok(2)
}

pub unsafe fn diff_r_1420b5520(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_43d0(
        a1: u64,
        a2_ptr: usize,
        a3: u64,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
    ) {
        let p2 = rd::<[u64; 2]>(a2_ptr);
        CAPTURED = [a1, p2[0], p2[1], a3, a4, a5, a6, a7];
    }

    let _m1 = crate::difftest::Mock::new(0x1_420c_43d0, mock_43d0 as *const ())?;

    let run = |f: F| -> [u64; 8] {
        CAPTURED = [0; 8];
        let mut p1 = [0u64; 9];
        for i in 0..9usize {
            p1[i] = 0x200 + (i as u64) * 0x22;
        }
        f(p1.as_ptr() as usize);
        CAPTURED
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b5520 out");
    Ok(1)
}

pub unsafe fn diff_r_1420b5c00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: [u64; 6] = [0; 6];

    unsafe extern "win64" fn mock_9480(a1_ptr: usize, a2: u64, a3_ptr: usize, a4: u64) {
        let p1 = rd::<[u64; 2]>(a1_ptr);
        let p3 = rd::<[u64; 2]>(a3_ptr);
        CAPTURED = [p1[0], p1[1], a2, p3[0], p3[1], a4];
    }

    let _m1 = crate::difftest::Mock::new(0x1_420b_9480, mock_9480 as *const ())?;

    let run = |f: F| -> [u64; 6] {
        CAPTURED = [0; 6];
        let mut p1 = [0u64; 7];
        for i in 0..7usize {
            p1[i] = 0x300 + (i as u64) * 0x33;
        }
        f(p1.as_ptr() as usize);
        CAPTURED
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b5c00 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1420b9480(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_2040(vec_ptr: usize, new_len: u64) {
        wr::<u32>(vec_ptr + 0x10, new_len as u32);
    }
    unsafe extern "win64" fn mock_e550(arr_ptr: usize, len: u32) {
        let n = len as usize;
        let mut tmp = Vec::with_capacity(n);
        for i in 0..n {
            tmp.push(rd::<u64>(arr_ptr + i * 8));
        }
        tmp.sort_unstable();
        for (i, &v) in tmp.iter().enumerate() {
            wr::<u64>(arr_ptr + i * 8, v);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_411f_2040, mock_2040 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40fa_e550, mock_e550 as *const ())?;

    let run = |f: F| -> (u32, [u64; 8]) {
        let p1_items: [u64; 3] = [
            (0x0011u64 << 48) | 0x0000_0005,
            0xffff_ffff_ffff_ffff, // skipped (lo32 == -1)
            (0x0022u64 << 48) | 0x0000_0002,
        ];
        let p1: [u64; 2] = [3, p1_items.as_ptr() as usize as u64];

        let mut node2 = [0u8; 0x20];
        let n2p = node2.as_mut_ptr() as usize;
        wr::<usize>(n2p, 0);
        wr::<u64>(n2p + 0x08, 1);
        wr::<u32>(n2p + 0x10, 1);

        let mut node1 = [0u8; 0x20];
        let n1p = node1.as_mut_ptr() as usize;
        wr::<usize>(n1p, n2p);
        wr::<u64>(n1p + 0x08, 1);
        wr::<u32>(n1p + 0x10, 0);

        let mut p2 = [0u8; 0x88];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x78, n1p);
        wr::<usize>(p2p + 0x80, n2p);

        // Entry 0: produces duplicate of (0x0011 << 32) | 5; Entry 1: produces (0x0033 << 32) | 1
        let p3_items: [u64; 2] = [(5u64 << 32) | 0x0011, (1u64 << 32) | 0x0033];
        let p3: [u64; 2] = [2, p3_items.as_ptr() as usize as u64];

        let mut out_arr = [0u64; 8];
        let mut p4 = [0u8; 0x20];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p + 0x08, out_arr.as_mut_ptr() as usize);
        wr::<u32>(p4p + 0x10, 0);

        f(p1.as_ptr() as usize, p2p, p3.as_ptr() as usize, p4p);
        (rd::<u32>(p4p + 0x10), out_arr)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1420b9480 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4206_e380, "unknown_14206e380", rend::cauldron_hot_y2::unknown_14206e380 as *const (), Throws::Caught, rend::cauldron_hot_y2::diff_unknown_14206e380),
// Replacement::verified_throws(0x1_4207_5080, "unknown_142075080", rend::cauldron_hot_y2::unknown_142075080 as *const (), Throws::Caught, rend::cauldron_hot_y2::diff_unknown_142075080),
// Replacement::verified_throws(0x1_4207_56b0, "unknown_1420756b0", rend::cauldron_hot_y2::unknown_1420756b0 as *const (), Throws::Discard, rend::cauldron_hot_y2::diff_unknown_1420756b0),
// Replacement::verified(0x1_4207_5bc0, "unknown_142075bc0", rend::cauldron_hot_y2::unknown_142075bc0 as *const (), rend::cauldron_hot_y2::diff_unknown_142075bc0),
// Replacement::verified_throws(0x1_4207_6960, "unknown_142076960", rend::cauldron_hot_y2::unknown_142076960 as *const (), Throws::Caught, rend::cauldron_hot_y2::diff_unknown_142076960),
// Replacement::verified_throws(0x1_420a_d980, "r_1420ad980", rend::cauldron_hot_y2::r_1420ad980 as *const (), Throws::Caught, rend::cauldron_hot_y2::diff_r_1420ad980),
// Replacement::verified(0x1_420b_0520, "r_1420b0520", rend::cauldron_hot_y2::r_1420b0520 as *const (), rend::cauldron_hot_y2::diff_r_1420b0520),
// Replacement::verified(0x1_420b_1080, "r_1420b1080", rend::cauldron_hot_y2::r_1420b1080 as *const (), rend::cauldron_hot_y2::diff_r_1420b1080),
// Replacement::verified(0x1_420b_3040, "r_1420b3040", rend::cauldron_hot_y2::r_1420b3040 as *const (), rend::cauldron_hot_y2::diff_r_1420b3040),
// Replacement::verified_throws(0x1_420b_5520, "r_1420b5520", rend::cauldron_hot_y2::r_1420b5520 as *const (), Throws::Discard, rend::cauldron_hot_y2::diff_r_1420b5520),
// Replacement::verified_throws(0x1_420b_5c00, "r_1420b5c00", rend::cauldron_hot_y2::r_1420b5c00 as *const (), Throws::Discard, rend::cauldron_hot_y2::diff_r_1420b5c00),
// Replacement::verified_throws(0x1_420b_9480, "unknown_1420b9480", rend::cauldron_hot_y2::unknown_1420b9480 as *const (), Throws::Discard, rend::cauldron_hot_y2::diff_unknown_1420b9480),
