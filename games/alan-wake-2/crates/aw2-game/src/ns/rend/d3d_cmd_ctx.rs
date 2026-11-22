//! `d3d` command-context, resource-state, barrier, and buffer management functions (`0x14206f9f0..0x142075bc0`).
#![allow(static_mut_refs, dead_code)]

use std::sync::atomic::{AtomicU32, Ordering};

use crate::exe::{self, rd, wr};

const IAT_RELEASE_SRW_EXCLUSIVE: u64 = 0x1_4264_5598;
const IAT_ACQUIRE_SRW_EXCLUSIVE: u64 = 0x1_4264_55a8;
const IAT_GET_CURRENT_THREAD_ID: u64 = 0x1_4264_5790;
const IAT_SWITCH_TO_THREAD: u64 = 0x1_4264_5868;
const IAT_WCSCPY_S: u64 = 0x1_4264_69d0;

#[inline]
unsafe fn check_hr_device_removed(obj: usize, hr: u32) {
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (obj, hr) -> ());
    }
}

#[inline]
unsafe fn lock_recursive_srw(lock: usize) {
    let tid = exe::call_iat!(IAT_GET_CURRENT_THREAD_ID, () -> u32);
    if rd::<u32>(lock + 0x08) != tid {
        let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock) -> ());
        wr::<u32>(lock + 0x08, tid);
    }
    wr::<u32>(lock + 0x0c, rd::<u32>(lock + 0x0c).wrapping_add(1));
}

#[inline]
unsafe fn unlock_recursive_srw(lock: usize) {
    if lock != 0 {
        let cnt = rd::<u32>(lock + 0x0c).wrapping_sub(1);
        wr::<u32>(lock + 0x0c, cnt);
        if cnt == 0 {
            AtomicU32::from_ptr((lock + 0x08) as *mut u32).swap(u32::MAX, Ordering::SeqCst);
            let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
        }
    }
}

#[inline]
unsafe fn ensure_tls0_init() -> usize {
    let tls_ptr = exe::tls_pointer();
    let tls0 = rd::<usize>(tls_ptr);
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    tls0
}

#[inline]
unsafe fn com_release_and_zero(slot: usize) {
    let p = rd::<usize>(slot);
    if p != 0 {
        let vtbl = rd::<usize>(p);
        let _ = exe::call_va!(rd::<usize>(vtbl + 0x10), (p) -> u32);
    }
    wr::<usize>(slot, 0);
}

#[inline]
unsafe fn f32_to_u64_cvtt(v: f32) -> u64 {
    let two63 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fc8) as usize);
    if v >= two63 {
        let sub = v - two63;
        if sub < two63 {
            return (sub as i64 as u64).wrapping_add(0x8000_0000_0000_0000);
        }
        return sub as i64 as u64;
    }
    v as i64 as u64
}

// ============================================================================
// 1. Feature check, device init/shutdown, and queue sync (0x14206f9f0..0x1420714a0)
// ============================================================================

/// Check `TypedUAVLoadAdditionalFormats` (`D3D12_FEATURE_D3D12_OPTIONS`) and `R11G11B10_FLOAT` typed UAV load support.
///
/// Original: `0x14206f9f0` (145 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_check_r11g11b10_uav_load_support() -> bool {
    let mut opts0 = [0u8; 0x3c];
    let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let dev = rd::<usize>(ctx);
    let vtbl = rd::<usize>(dev);
    let check_fn = rd::<usize>(vtbl + 0x68);
    let hr0 = exe::call_va!(check_fn, (dev, 0u32, opts0.as_mut_ptr() as usize, 0x3cu32) -> i32);
    if hr0 < 0 || rd::<u32>(opts0.as_ptr() as usize + 0x18) == 0 {
        return false;
    }
    let mut fmt_sup = [0x1au32, 0u32, 0u32];
    let ctx2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let dev2 = rd::<usize>(ctx2);
    let vtbl2 = rd::<usize>(dev2);
    let check_fn2 = rd::<usize>(vtbl2 + 0x68);
    let hr1 = exe::call_va!(check_fn2, (dev2, 3u32, fmt_sup.as_mut_ptr() as usize, 0x0cu32) -> i32);
    if hr1 < 0 {
        return false;
    }
    ((fmt_sup[2] >> 6) & 1) != 0
}

/// Shut down D3D12 device, queues, pools, raytracing state, and NVAPI.
///
/// Original: `0x14206fa90` (1454 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_shutdown_device_and_resources() {
    exe::call_va!(0x1_4207_14a0, () -> ());
    let mut cmd_ctx = exe::call_va!(0x1_4205_fad0, () -> usize);
    let rbx_ctx = cmd_ctx;
    exe::call_va!(0x1_4205_f870, (1u32, &mut cmd_ctx as *mut usize as usize) -> ());
    exe::call_va!(0x1_4207_6db0, (rd::<usize>(rbx_ctx + 0x10)) -> ());

    let q_wrapper = rd::<usize>(rbx_ctx + 0x10);
    let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let fence_val = rd::<u64>(ctx + 0x40);
    let fence = rd::<usize>(ctx + 0x38);
    let cmd_q = rd::<usize>(q_wrapper + 0x10);
    let q_vtbl = rd::<usize>(cmd_q);
    let hr = exe::call_va!(rd::<usize>(q_vtbl + 0x70), (cmd_q, fence, fence_val) -> u32);
    check_hr_device_removed(cmd_q, hr);

    let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let wait_target = rd::<u64>(ctx + 0x40);
    wr::<u64>(ctx + 0x40, u64::MAX);

    if rd::<u8>(ctx + 0xac) != 0 {
        let rt_mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fba0) as usize);
        if rt_mgr != 0 {
            exe::call_va!(
                0x1_4183_4ee8,
                (rt_mgr + 0x08, 8usize, 2usize, exe::ptr::<u8>(0x1_411d_b190) as usize) -> ()
            );
            let inner = rd::<usize>(rt_mgr);
            if inner != 0 {
                exe::call_va!(0x1_4209_d670, (inner) -> ());
                exe::call_va!(0x1_411e_7fc0, (inner + 0x58) -> ());
                exe::call_va!(0x1_4244_2de0, (inner) -> ());
            }
            exe::call_va!(0x1_4244_2de0, (rt_mgr) -> ());
        }
        wr::<usize>(exe::ptr::<usize>(0x1_4397_fba0) as usize, 0);
    }

    exe::call_va!(0x1_4209_e9e0, () -> ());
    exe::call_va!(0x1_4209_7470, () -> ());
    exe::call_va!(0x1_4205_5450, () -> ());
    exe::call_va!(0x1_4205_3480, () -> ());
    exe::call_va!(0x1_4205_e770, () -> ());
    exe::call_va!(0x1_4205_ec50, () -> ());
    exe::call_va!(0x1_4205_d600, () -> ());
    exe::call_va!(0x1_4205_e2a0, () -> ());
    exe::call_va!(0x1_4205_dc70, () -> ());

    let lock_32d38 = exe::ptr::<u8>(0x1_43a3_2d38) as usize;
    let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock_32d38) -> ());
    exe::call_va!(0x1_411d_6a90, (exe::ptr::<u8>(0x1_43a6_3248) as usize) -> ());
    let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock_32d38) -> ());
    wr::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize, 0);

    let mut rdi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let p68 = rd::<usize>(rdi + 0x68);
    wr::<usize>(rdi + 0x68, 0);
    if p68 != 0 {
        exe::call_va!(0x1_4208_8250, (p68) -> ());
        exe::call_va!(0x1_4244_2de0, (p68) -> ());
        rdi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    }

    let mut rbx = rd::<usize>(rdi + 0x138);
    let rsi_end = rbx.wrapping_add(rd::<usize>(rdi + 0x120) << 4);
    if rbx != rsi_end {
        let empty_key = rd::<usize>(rdi + 0x128);
        while rbx != rsi_end {
            let k = rd::<usize>(rbx);
            if k == empty_key || (rd::<usize>(rdi + 0x110) > 0 && rd::<usize>(rdi + 0x108) == k) {
                rbx += 0x10;
            } else {
                break;
            }
        }
        while rbx != rsi_end {
            let entry = rd::<usize>(rbx + 0x08);
            if entry != 0 {
                let com = rd::<usize>(entry);
                if com != 0 {
                    let vtbl = rd::<usize>(com);
                    let _ = exe::call_va!(rd::<usize>(vtbl + 0x10), (com) -> u32);
                }
                wr::<usize>(entry, 0);
                exe::call_va!(0x1_4244_2de0, (entry) -> ());
                wr::<usize>(rbx + 0x08, 0);
            }
            rbx += 0x10;
            let empty_key = rd::<usize>(rdi + 0x128);
            while rbx != rsi_end {
                let k = rd::<usize>(rbx);
                if k == empty_key || (rd::<usize>(rdi + 0x110) > 0 && rd::<usize>(rdi + 0x108) == k) {
                    rbx += 0x10;
                } else {
                    break;
                }
            }
        }
        rdi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    }

    if rd::<usize>(rdi + 0xb8) != 0 {
        let mut off = 0usize;
        while off < 0xc0 {
            let elem = rd::<usize>(rdi + 0xb8) + off;
            wr::<u32>(elem + 0x50, 0);
            let sub = rd::<usize>(elem + 0x58);
            wr::<usize>(elem + 0x58, 0);
            if sub != 0 {
                if rd::<u8>(sub + 0x18a) != 0 {
                    exe::call_va!(0x1_4207_5a20, (sub) -> ());
                }
                if rd::<u8>(sub + 0x18b) != 0 {
                    exe::call_va!(0x1_4207_5a20, (sub + 0x50) -> ());
                }
                if rd::<u8>(sub + 0x18c) != 0 {
                    exe::call_va!(0x1_4207_5a20, (sub + 0xa0) -> ());
                }
                exe::call_va!(0x1_4244_2de0, (sub) -> ());
            }
            exe::call_va!(0x1_4207_5a20, (elem) -> ());
            off += 0x60;
            if off < 0xc0 {
                rdi = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
            }
        }
    }

    exe::call_va!(0x1_4206_0060, () -> ());

    loop {
        let ctx_now = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
        let fence_obj = rd::<usize>(ctx_now + 0x38);
        let vtbl = rd::<usize>(fence_obj);
        let completed = exe::call_va!(rd::<usize>(vtbl + 0x40), (fence_obj) -> u64);
        if completed == u64::MAX {
            exe::call_va!(0x1_4205_d410, (fence_obj, 0x887a_0005u32) -> ());
        }
        if completed >= wait_target {
            break;
        }
        let _ = exe::call_iat!(IAT_SWITCH_TO_THREAD, () -> i32);
    }

    exe::call_va!(0x1_4209_9860, () -> ());
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fb88) as usize, 0);

    let mut ctx_now = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let p60 = rd::<usize>(ctx_now + 0x60);
    wr::<usize>(ctx_now + 0x60, 0);
    if p60 != 0 {
        exe::call_va!(0x1_411d_e510, (ctx_now, p60) -> ());
        ctx_now = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    }

    let p70 = rd::<usize>(ctx_now + 0x70);
    wr::<usize>(ctx_now + 0x70, 0);
    if p70 != 0 {
        exe::call_va!(0x1_4205_27c0, (p70) -> ());
        exe::call_va!(0x1_4244_2de0, (p70) -> ());
        ctx_now = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    }

    let p78 = rd::<usize>(ctx_now + 0x78);
    wr::<usize>(ctx_now + 0x78, 0);
    if p78 != 0 {
        exe::call_va!(0x1_4205_1db0, (p78) -> ());
        exe::call_va!(0x1_4244_2de0, (p78) -> ());
        ctx_now = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    }

    exe::call_va!(0x1_411d_da90, (ctx_now + 0x80, 0usize) -> ());

    let g50 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc50) as usize);
    if g50 != 0 {
        exe::call_va!(0x1_4206_5e20, (g50) -> ());
        exe::call_va!(0x1_4244_2de0, (g50) -> ());
    }
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc50) as usize, 0);

    for g_va in [0x1_4397_fcb8u64, 0x1_4397_fc48u64] {
        let p = rd::<usize>(exe::ptr::<usize>(g_va) as usize);
        if p != 0 {
            let vtbl = rd::<usize>(p);
            exe::call_va!(rd::<usize>(vtbl), (p, 0u32) -> ());
            exe::call_va!(0x1_4244_2de0, (p) -> ());
        }
        wr::<usize>(exe::ptr::<usize>(g_va) as usize, 0);
    }

    for g_va in [0x1_4397_fc78u64, 0x1_4397_fc60u64, 0x1_4397_fc70u64] {
        let p = rd::<usize>(exe::ptr::<usize>(g_va) as usize);
        if p != 0 {
            exe::call_va!(0x1_411d_d6c0, (p) -> ());
            exe::call_va!(0x1_4244_2de0, (p) -> ());
        }
        wr::<usize>(exe::ptr::<usize>(g_va) as usize, 0);
    }

    exe::call_va!(0x1_4209_9860, () -> ());

    for mode in [0u32, 1u32] {
        let lock_obj = if mode == 1 {
            exe::ptr::<u8>(0x1_4385_8518) as usize
        } else {
            exe::ptr::<u8>(0x1_4385_84a8) as usize
        };
        let slot_addr = if mode == 1 {
            exe::ptr::<usize>(0x1_4385_8510) as usize
        } else {
            exe::ptr::<usize>(0x1_4385_84a0) as usize
        };
        let item = rd::<usize>(slot_addr);
        if item != 0 {
            lock_recursive_srw(lock_obj);
            let len = rd::<u32>(lock_obj + 0x20);
            if len == rd::<u32>(lock_obj + 0x24) {
                exe::call_va!(0x1_4209_84c0, (lock_obj + 0x10, len.wrapping_add(1)) -> ());
            }
            let tail = rd::<u32>(lock_obj + 0x1c) as usize;
            let buf = rd::<usize>(lock_obj + 0x10);
            wr::<usize>(buf + tail * 8, item);
            let next_tail = (tail as u32).wrapping_add(1);
            wr::<u32>(
                lock_obj + 0x1c,
                if next_tail == rd::<u32>(lock_obj + 0x24) { 0 } else { next_tail },
            );
            wr::<u32>(lock_obj + 0x20, rd::<u32>(lock_obj + 0x20).wrapping_add(1));
            unlock_recursive_srw(lock_obj);
        }
        wr::<usize>(slot_addr, 0);
    }

    let nvapi_flag = exe::ptr::<u8>(0x1_43a2_8c3d) as usize;
    if rd::<u8>(nvapi_flag) != 0 {
        let _ = exe::call_va!(0x1_4000_1a60, () -> u32);
        wr::<u8>(nvapi_flag, 0);
    }

    let ctx_final = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    if ctx_final != 0 {
        exe::call_va!(0x1_4207_1da0, (ctx_final) -> ());
        exe::call_va!(0x1_4244_2de0, (ctx_final) -> ());
    }
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize, 0);
}

/// Initialize global D3D12 device, command queues, descriptor managers, and pools.
///
/// Original: `0x142070050` (608 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_init_device_and_resources(
    mode: u32,
    hwnd_or_desc: usize,
) -> usize {
    let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let table = exe::ptr::<usize>(0x1_43c4_aa00) as usize;
    let entry = rd::<usize>(table + worker_idx * 8);
    let val = if entry != 0 { rd::<u32>(entry) } else { 0 };
    wr::<u32>(exe::ptr::<u32>(0x1_4382_3bac) as usize, val);

    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fc38) as usize,
        exe::call_va!(0x1_411d_7e20, () -> usize),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fc30) as usize,
        exe::call_va!(0x1_411d_7e20, () -> usize),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_6078) as usize,
        exe::call_va!(0x1_40fd_31f0, () -> usize),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_6148) as usize,
        exe::call_va!(0x1_411d_7e20, () -> usize),
    );

    let tls0 = ensure_tls0_init();
    exe::call_va!(0x1_4207_61a0, (tls0 + 0x467e0) -> ());
    exe::call_va!(0x1_4209_95c0, (0u32) -> ());
    exe::call_va!(0x1_4209_95c0, (1u32) -> ());

    let ctx = exe::call_va!(0x1_411d_d660, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize, ctx);
    exe::call_va!(0x1_4207_19f0, (ctx, mode, hwnd_or_desc) -> ());

    exe::call_va!(0x1_4209_ecc0, () -> ());
    exe::call_va!(0x1_4209_7640, () -> ());
    exe::call_va!(0x1_4205_5620, () -> ());
    exe::call_va!(0x1_4205_38c0, () -> ());

    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fc28) as usize,
        exe::call_va!(0x1_411d_7e20, () -> usize),
    );
    exe::call_va!(0x1_411d_9ab0, () -> ());
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_5fe8) as usize,
        exe::call_va!(0x1_40fd_31f0, () -> usize),
    );

    let ring = exe::call_va!(0x1_411d_7e20, () -> usize);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc98) as usize, ring);
    lock_recursive_srw(ring);
    for _ in 0..0x20 {
        let item = exe::call_va!(0x1_411d_a620, () -> usize);
        exe::call_va!(0x1_411d_a6a0, (item) -> ());
        let len = rd::<u32>(ring + 0x20);
        if len == rd::<u32>(ring + 0x24) {
            exe::call_va!(0x1_4034_97a0, (ring + 0x10, len.wrapping_add(1)) -> ());
        }
        let tail = rd::<u32>(ring + 0x1c) as usize;
        let arr = rd::<usize>(ring + 0x10);
        wr::<usize>(arr + tail * 8, item);
        let next_tail = (tail as u32).wrapping_add(1);
        wr::<u32>(
            ring + 0x1c,
            if next_tail == rd::<u32>(ring + 0x24) { 0 } else { next_tail },
        );
        wr::<u32>(ring + 0x20, rd::<u32>(ring + 0x20).wrapping_add(1));
        wr::<u64>(ring + 0x28, rd::<u64>(ring + 0x28).wrapping_add(1));
    }
    unlock_recursive_srw(ring);

    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_5d58) as usize,
        exe::call_va!(0x1_411d_7da0, () -> usize),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_5e28) as usize,
        exe::call_va!(0x1_40fd_31f0, () -> usize),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fc10) as usize,
        exe::call_va!(0x1_411d_7e20, () -> usize),
    );
    exe::call_va!(0x1_411d_82e0, () -> ());

    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_5f98) as usize,
        exe::call_va!(0x1_40fd_31f0, () -> usize),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fc20) as usize,
        exe::call_va!(0x1_411d_7e20, () -> usize),
    );
    exe::call_va!(0x1_411d_93b0, () -> ());

    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_5e78) as usize,
        exe::call_va!(0x1_411d_7da0, () -> usize),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a3_5ec8) as usize,
        exe::call_va!(0x1_40fd_31f0, () -> usize),
    );
    wr::<usize>(
        exe::ptr::<usize>(0x1_4397_fc18) as usize,
        exe::call_va!(0x1_411d_7e20, () -> usize),
    );
    exe::call_va!(0x1_411d_8b90, () -> ());

    let ctx_now = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    if rd::<u8>(ctx_now + 0xac) != 0 {
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fba0) as usize,
            exe::call_va!(0x1_411d_4380, () -> usize),
        );
    }

    let mut init_desc = [
        0u64,
        0u64,
        rd::<u64>(exe::ptr::<u64>(0x1_4397_fc60) as usize),
    ];
    exe::call_va!(0x1_4205_fdc0, (init_desc.as_mut_ptr() as usize) -> ());
    rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize)
}

/// Flush global queues (`0x14397fc60`, `0x14397fc70`, `0x14397fc78`) and enqueue `ID3D12CommandQueue::Wait` on the TLS queue.
///
/// Original: `0x1420702c0` (166 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_flush_queues_and_wait_fence(fence: usize, fence_val: u64) {
    for g_va in [0x1_4397_fc60u64, 0x1_4397_fc70u64, 0x1_4397_fc78u64] {
        let q = rd::<usize>(exe::ptr::<usize>(g_va) as usize);
        if q != 0 {
            exe::call_va!(0x1_4207_6db0, (q) -> ());
        }
    }
    let tls0 = ensure_tls0_init();
    let tls_q_wrapper = rd::<usize>(tls0 + 0x6328);
    let cmd_q = rd::<usize>(tls_q_wrapper + 0x10);
    let vtbl = rd::<usize>(cmd_q);
    let hr = exe::call_va!(rd::<usize>(vtbl + 0x78), (cmd_q, fence, fence_val) -> u32);
    check_hr_device_removed(cmd_q, hr);
}

/// Resolve or create graphics `ID3D12PipelineState` for `state` (`rdx`) and bind via `SetPipelineState`.
///
/// Original: `0x142070fb0` (546 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_bind_graphics_pipeline_state(
    _unused: usize,
    state: usize,
) -> bool {
    let mut key = [0u8; 0x58];
    let key_ptr = key.as_mut_ptr() as usize;
    wr::<u64>(key_ptr, rd::<u64>(state + 0xc48));
    wr::<u64>(key_ptr + 0x08, rd::<u64>(state + 0xc60));
    wr::<u64>(key_ptr + 0x10, rd::<u64>(state + 0xc68));
    let ps_obj = rd::<usize>(state + 0xc50);
    wr::<usize>(key_ptr + 0x18, ps_obj);
    wr::<u64>(key_ptr + 0x20, rd::<u64>(state + 0x48));
    wr::<u32>(key_ptr + 0x28, rd::<u32>(state + 0xc40));
    wr::<u64>(key_ptr + 0x30, rd::<u64>(state + 0xaf8));
    wr::<u64>(key_ptr + 0x38, rd::<u64>(state + 0xb00));
    wr::<u64>(key_ptr + 0x40, rd::<u64>(state + 0xb10));

    let mut num_rts = rd::<i32>(state + 0xc44);
    if ps_obj != 0 {
        let ps_rts = rd::<i32>(ps_obj + 0x28);
        if ps_rts > 0 && num_rts > ps_rts {
            num_rts = ps_rts;
        }
    }
    wr::<i32>(key_ptr + 0x48, num_rts);

    if num_rts > 0 {
        for i in 0..(num_rts as usize) {
            let rt = rd::<usize>(state + 0xab0 + i * 8);
            wr::<u8>(key_ptr + 0x4c + i, rd::<u8>(rt + 0x18));
        }
    }
    let dsv = rd::<usize>(state + 0xaf0);
    wr::<u8>(key_ptr + 0x54, if dsv != 0 { rd::<u8>(dsv + 0x18) } else { 0 });

    let cache_mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc50) as usize);
    let hash = exe::call_va!(0x1_4206_4c70, (key_ptr) -> u64);
    let srw = cache_mgr + 0x68;
    let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (srw) -> ());

    let mut lookup_out = [0usize; 2];
    let _ = exe::call_va!(
        0x1_411d_bd00,
        (cache_mgr + 0x80, lookup_out.as_mut_ptr() as usize, &hash as *const u64 as usize) -> usize
    );
    let entry = lookup_out[1];
    if rd::<u8>(entry + 0x08) != 2 {
        let mut stream_blob = [0u64; 2];
        exe::call_va!(0x1_4206_4600, (key_ptr, stream_blob.as_mut_ptr() as usize) -> ());
        let blob_ptr = stream_blob[0] as usize;
        let blob_len = (stream_blob[1] as u32) as u64;
        let desc: [u64; 2] = [blob_len, blob_ptr as u64];
        let mut tmp_pso: usize = 0;
        let created = exe::call_va!(
            0x1_4207_2ad0,
            (
                0usize,
                &mut tmp_pso as *mut usize as usize,
                desc.as_ptr() as usize,
            ) -> usize
        );
        com_release_and_zero(entry + 0x10);
        let new_pso = rd::<usize>(created);
        wr::<usize>(created, 0);
        wr::<usize>(entry + 0x10, new_pso);
        if tmp_pso != 0 {
            let vtbl = rd::<usize>(tmp_pso);
            let _ = exe::call_va!(rd::<usize>(vtbl + 0x10), (tmp_pso) -> u32);
        }
        wr::<u8>(entry + 0x08, 2);
        if blob_ptr != 0 {
            exe::call_va!(0x1_4241_c190, (blob_ptr) -> ());
        }
    }
    let pso = rd::<usize>(entry + 0x10);
    let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (srw) -> ());

    if pso != 0 {
        let mut cmd_lock = [0usize; 2];
        let _ = exe::call_va!(0x1_411d_5b00, (cmd_lock.as_mut_ptr() as usize) -> usize);
        let cmd = cmd_lock[1];
        let vtbl = rd::<usize>(cmd);
        exe::call_va!(rd::<usize>(vtbl + 0xc8), (cmd, pso) -> ());
        unlock_recursive_srw(cmd_lock[0]);
        wr::<u8>(state + 0xa80, 1);
        return true;
    }
    false
}

/// Bind compute root signature (`+0x30`), descriptor heaps (`+0x50`, `+0x58`), and flush compute descriptor tables.
///
/// Original: `0x1420711e0` (305 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_bind_compute_root_sig_and_heaps(_unused: usize, state: usize) {
    let ctx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    let root_sig = rd::<usize>(ctx + 0x30);
    if rd::<usize>(state + 0xa68) != root_sig {
        let mut cmd_lock = [0usize; 2];
        let _ = exe::call_va!(0x1_411d_5b00, (cmd_lock.as_mut_ptr() as usize) -> usize);
        let cmd = cmd_lock[1];
        let vtbl = rd::<usize>(cmd);
        exe::call_va!(rd::<usize>(vtbl + 0xe8), (cmd, rd::<usize>(ctx + 0x30)) -> ());
        unlock_recursive_srw(cmd_lock[0]);
        wr::<usize>(state + 0xa68, rd::<usize>(ctx + 0x30));
        let dm = exe::call_va!(0x1_411d_cdf0, () -> usize);
        exe::call_va!(0x1_4209_5ba0, (dm) -> ());
    }
    let dm0 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    let h0 = rd::<usize>(rd::<usize>(dm0 + 0x50));
    let dm1 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    let h1 = rd::<usize>(rd::<usize>(dm1 + 0x58));
    let heaps = [h0, h1];
    if rd::<usize>(state + 0xa70) != h0 || rd::<usize>(state + 0xa78) != h1 {
        let mut cmd_lock = [0usize; 2];
        let _ = exe::call_va!(0x1_411d_5b00, (cmd_lock.as_mut_ptr() as usize) -> usize);
        let cmd = cmd_lock[1];
        let vtbl = rd::<usize>(cmd);
        exe::call_va!(rd::<usize>(vtbl + 0xe0), (cmd, 2u32, heaps.as_ptr() as usize) -> ());
        unlock_recursive_srw(cmd_lock[0]);
        wr::<usize>(state + 0xa70, h0);
        wr::<usize>(state + 0xa78, h1);
    }
    let dm2 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    exe::call_va!(0x1_4209_5e90, (dm2) -> ());
}

/// Bind graphics root signature (`rdx`), descriptor heaps (`+0x20`, `+0x28`), and flush graphics descriptor tables.
///
/// Original: `0x142071360` (306 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_bind_graphics_root_sig_and_heaps(state: usize, root_sig: usize) {
    if rd::<usize>(state + 0xa60) != root_sig {
        let mut cmd_lock = [0usize; 2];
        let _ = exe::call_va!(0x1_411d_5b00, (cmd_lock.as_mut_ptr() as usize) -> usize);
        let cmd = cmd_lock[1];
        let vtbl = rd::<usize>(cmd);
        exe::call_va!(rd::<usize>(vtbl + 0xf0), (cmd, root_sig) -> ());
        unlock_recursive_srw(cmd_lock[0]);
        wr::<usize>(state + 0xa60, root_sig);
        let dm = exe::call_va!(0x1_411d_cdf0, () -> usize);
        exe::call_va!(0x1_4209_5c30, (dm) -> ());
    }
    let dm0 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    let h0 = rd::<usize>(rd::<usize>(dm0 + 0x20));
    let dm1 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    let h1 = rd::<usize>(rd::<usize>(dm1 + 0x28));
    let heaps = [h0, h1];
    if rd::<usize>(state + 0xa70) != h0 || rd::<usize>(state + 0xa78) != h1 {
        let mut cmd_lock = [0usize; 2];
        let _ = exe::call_va!(0x1_411d_5b00, (cmd_lock.as_mut_ptr() as usize) -> usize);
        let cmd = cmd_lock[1];
        let vtbl = rd::<usize>(cmd);
        exe::call_va!(rd::<usize>(vtbl + 0xe0), (cmd, 2u32, heaps.as_ptr() as usize) -> ());
        unlock_recursive_srw(cmd_lock[0]);
        wr::<usize>(state + 0xa70, h0);
        wr::<usize>(state + 0xa78, h1);
    }
    let has_c68 = (rd::<usize>(state + 0xc68) != 0) as u8;
    let dm2 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    exe::call_va!(0x1_4209_60c0, (dm2, has_c68) -> ());
}

/// Reset thread-local command state (`tls0 + 0x467e0`) to default pipeline, vertex/index buffer, and render target state.
///
/// Original: `0x1420714a0` (408 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_reset_tls_command_state() {
    let tls0 = ensure_tls0_init();
    let state = tls0 + 0x467e0;
    if rd::<usize>(state + 0xaf8) != 0 {
        wr::<usize>(
            state + 0xaf8,
            rd::<usize>(exe::ptr::<usize>(0x1_4397_fd30) as usize),
        );
        wr::<u8>(state + 0xa80, 0);
    }
    let mut old_b00 = rd::<usize>(state + 0xb00);
    if old_b00 != 0 {
        wr::<usize>(state + 0xb00, 0);
        wr::<u8>(state + 0xa80, 0);
        old_b00 = 0;
    }
    if rd::<usize>(state + 0xb10) == 0 {
        wr::<usize>(state + 0xb10, 0);
        wr::<u8>(state + 0xa80, 0);
        if old_b00 != 0 {
            wr::<usize>(state + 0xb00, 0);
        }
    }
    wr::<usize>(state + 0xb18, 0);
    wr::<usize>(state + 0xb20, 0);
    wr::<usize>(state + 0x48, 0);
    wr::<u8>(state + 0xa80, 0);

    let zero_shaders = [0u64; 6];
    let zero_refs = [0u64; 6];
    exe::call_va!(
        0x1_4206_f680,
        (state, zero_shaders.as_ptr() as usize, zero_refs.as_ptr() as usize, 0u8) -> ()
    );

    if rd::<usize>(state + 0xb30) != 0 {
        wr::<u8>(state + 0xb2c, 1);
        wr::<usize>(state + 0xb30, 0);
        wr::<usize>(state + 0xb38, 0);
    }
    if rd::<usize>(state + 0xb40) != 0 {
        wr::<u8>(state + 0xb2c, 1);
        wr::<usize>(state + 0xb40, 0);
        wr::<usize>(state + 0xb48, 0);
    }
    if rd::<usize>(state + 0xc30) != 0 {
        let mut cmd_lock = [0usize; 2];
        let _ = exe::call_va!(0x1_411d_5b00, (cmd_lock.as_mut_ptr() as usize) -> usize);
        let cmd = cmd_lock[1];
        let vtbl = rd::<usize>(cmd);
        exe::call_va!(rd::<usize>(vtbl + 0x158), (cmd, 0usize) -> ());
        unlock_recursive_srw(cmd_lock[0]);
        wr::<usize>(state + 0xc30, 0);
        wr::<usize>(state + 0xc38, 0);
    }
    for i in 0..8 {
        wr::<u64>(state + i * 8, 0);
    }
}

pub unsafe fn diff_d3d_check_r11g11b10_uav_load_support(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_check_feature_support(
        dev: usize,
        feature: u32,
        data: usize,
        size: u32,
    ) -> i32 {
        let mode = rd::<u32>(dev + 0x08);
        if feature == 0 && size == 0x3c {
            if mode == 0 {
                return -1;
            }
            wr::<u32>(data + 0x18, if mode >= 2 { 1 } else { 0 });
            return 0;
        }
        if feature == 3 && size == 0x0c {
            if mode == 2 {
                return -1;
            }
            wr::<u32>(data + 0x08, if mode == 4 { 1 << 6 } else { 0 });
            return 0;
        }
        -1
    }

    type FnT = unsafe extern "win64" fn() -> bool;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut vtbl = [0usize; 16];
    vtbl[0x68 / 8] = fake_check_feature_support as *const () as usize;
    let mut fake_dev = [0u64; 4];
    fake_dev[0] = vtbl.as_ptr() as u64;
    let mut fake_ctx = [0u64; 4];
    fake_ctx[0] = fake_dev.as_mut_ptr() as u64;

    let global_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_global = rd::<usize>(global_addr);
    wr::<usize>(global_addr, fake_ctx.as_mut_ptr() as usize);

    let mut cases = 0u32;
    for mode in [0u32, 1, 2, 3, 4] {
        wr::<u32>(fake_dev.as_mut_ptr() as usize + 0x08, mode);
        let o = orig_fn();
        let n = new_fn();
        if o != n {
            wr::<usize>(global_addr, saved_global);
            return Err(format!(
                "d3d_check_r11g11b10_uav_load_support mode {mode}: orig={o} new={n}"
            ));
        }
        cases += 1;
    }

    wr::<usize>(global_addr, saved_global);
    Ok(cases)
}

pub unsafe fn diff_d3d_flush_queues_and_wait_fence(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_queue_wait(
        cmd_q: usize,
        fence: usize,
        fence_val: u64,
    ) -> u32 {
        wr::<usize>(cmd_q + 0x08, fence);
        wr::<u64>(cmd_q + 0x10, fence_val);
        0
    }

    type FnT = unsafe extern "win64" fn(usize, u64);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let mut vtbl = [0usize; 16];
    vtbl[0x78 / 8] = fake_queue_wait as *const () as usize;

    let mut q_o = [0u64; 4];
    let mut q_n = [0u64; 4];
    q_o[0] = vtbl.as_ptr() as u64;
    q_n[0] = vtbl.as_ptr() as u64;

    let mut wrapper_o = [0u64; 4];
    let mut wrapper_n = [0u64; 4];
    wr::<usize>(
        wrapper_o.as_mut_ptr() as usize + 0x10,
        q_o.as_mut_ptr() as usize,
    );
    wr::<usize>(
        wrapper_n.as_mut_ptr() as usize + 0x10,
        q_n.as_mut_ptr() as usize,
    );

    let mut tls_o = vec![0u8; 0x6400];
    let mut tls_n = vec![0u8; 0x6400];
    tls_o[0x70] = 1;
    tls_n[0x70] = 1;
    wr::<usize>(
        tls_o.as_mut_ptr() as usize + 0x6328,
        wrapper_o.as_mut_ptr() as usize,
    );
    wr::<usize>(
        tls_n.as_mut_ptr() as usize + 0x6328,
        wrapper_n.as_mut_ptr() as usize,
    );

    let g60 = exe::ptr::<usize>(0x1_4397_fc60) as usize;
    let g70 = exe::ptr::<usize>(0x1_4397_fc70) as usize;
    let g78 = exe::ptr::<usize>(0x1_4397_fc78) as usize;
    let saved_g60 = rd::<usize>(g60);
    let saved_g70 = rd::<usize>(g70);
    let saved_g78 = rd::<usize>(g78);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    wr::<usize>(g60, 0);
    wr::<usize>(g70, 0);
    wr::<usize>(g78, 0);

    wr::<usize>(tls_slot, tls_o.as_mut_ptr() as usize);
    orig_fn(0xdead_beef, 0x1234_5678_9abc_def0);

    wr::<usize>(tls_slot, tls_n.as_mut_ptr() as usize);
    new_fn(0xdead_beef, 0x1234_5678_9abc_def0);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<usize>(g60, saved_g60);
    wr::<usize>(g70, saved_g70);
    wr::<usize>(g78, saved_g78);

    crate::diff_eq!(q_o[1], q_n[1], "d3d_flush_queues_and_wait_fence fence");
    crate::diff_eq!(q_o[2], q_n[2], "d3d_flush_queues_and_wait_fence fence_val");
    Ok(1)
}

static mut CMD_CTX_LOG: u64 = 0;
static mut MOCK_PTR_RET: usize = 0;
static mut MOCK_RING_PTR: usize = 0;
static mut MOCK_COUNTER: usize = 0;

#[inline]
unsafe fn log_mix(tag: u64, val: u64) {
    CMD_CTX_LOG = CMD_CTX_LOG
        .wrapping_mul(0x1000_0000_01b3)
        .wrapping_add(tag)
        .wrapping_mul(0x1000_0000_01b3)
        .wrapping_add(val);
}

unsafe extern "win64" fn mock_noop_0() {
    log_mix(1, 0);
}
unsafe extern "win64" fn mock_ret_ptr() -> usize {
    log_mix(2, 0);
    MOCK_PTR_RET
}
unsafe extern "win64" fn mock_ret_ring() -> usize {
    log_mix(3, 0);
    MOCK_RING_PTR
}
unsafe extern "win64" fn mock_ret_u32_zero() -> u32 {
    log_mix(4, 0);
    0
}
unsafe extern "win64" fn mock_ret_counter() -> usize {
    MOCK_COUNTER += 1;
    log_mix(5, MOCK_COUNTER as u64);
    0x3000 + MOCK_COUNTER
}
unsafe extern "win64" fn mock_arg1_u64(a0: u64) {
    log_mix(6, a0);
}
unsafe extern "win64" fn mock_arg1_u32(a0: u32) {
    log_mix(7, a0 as u64);
}
unsafe extern "win64" fn mock_f870(a0: u32, a1: usize) {
    log_mix(8, (a0 as u64) ^ (rd::<u64>(a1) & 0xffff));
}
unsafe extern "win64" fn mock_dda90(_a0: usize, a1: usize) {
    log_mix(9, a1 as u64);
}
unsafe extern "win64" fn mock_719f0(_rcx: usize, mode: u32, hwnd: usize) {
    log_mix(10, ((mode as u64) << 32) ^ (hwnd as u64));
}
unsafe extern "win64" fn mock_5fdc0(desc: usize) {
    log_mix(11, rd::<u64>(desc + 0x10));
}

pub unsafe fn diff_d3d_shutdown_device_and_resources(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_signal(q: usize, fence: usize, val: u64) -> u32 {
        wr::<usize>(q + 0x08, fence);
        wr::<u64>(q + 0x10, val);
        0
    }
    unsafe extern "win64" fn fake_completed(_fence: usize) -> u64 {
        100
    }

    let _m01 = crate::difftest::Mock::new(0x1_4207_14a0, mock_noop_0 as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_4205_fad0, mock_ret_ptr as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_4205_f870, mock_f870 as *const ())?;
    let _m04 = crate::difftest::Mock::new(0x1_4207_6db0, mock_arg1_u64 as *const ())?;
    let _m05 = crate::difftest::Mock::new(0x1_4209_e9e0, mock_noop_0 as *const ())?;
    let _m06 = crate::difftest::Mock::new(0x1_4209_7470, mock_noop_0 as *const ())?;
    let _m07 = crate::difftest::Mock::new(0x1_4205_5450, mock_noop_0 as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_4205_3480, mock_noop_0 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_4205_e770, mock_noop_0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4205_ec50, mock_noop_0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4205_d600, mock_noop_0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4205_e2a0, mock_noop_0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4205_dc70, mock_noop_0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_411d_6a90, mock_arg1_u64 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_4206_0060, mock_noop_0 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_4209_9860, mock_noop_0 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_411d_da90, mock_dda90 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_4207_1da0, mock_noop_0 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_4244_2de0, mock_noop_0 as *const ())?;

    type FnT = unsafe extern "win64" fn();
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let snap_fc = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fb80) as *mut u8, 0x150);
    let snap_srw = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a3_2d38) as *mut u8, 8);
    let snap_4698 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a3_4698) as *mut u8, 8);
    let snap_84a0 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4385_84a0) as *mut u8, 0x80);
    let snap_nv = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a2_8c3d) as *mut u8, 1);

    let mut q_vtbl = [0usize; 16];
    q_vtbl[0x70 / 8] = fake_signal as *const () as usize;
    let mut f_vtbl = [0usize; 10];
    f_vtbl[0x40 / 8] = fake_completed as *const () as usize;
    let fence = [f_vtbl.as_ptr() as u64];

    let run_one = |f: FnT| -> (u64, [u64; 3], Vec<u8>) {
        std::ptr::write_bytes(exe::ptr::<u8>(0x1_4397_fb80) as *mut u8, 0, 0x150);
        wr::<usize>(exe::ptr::<usize>(0x1_43a3_2d38) as usize, 0);
        wr::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize, 0x7777);
        std::ptr::write_bytes(exe::ptr::<u8>(0x1_4385_84a0) as *mut u8, 0, 0x80);
        wr::<u8>(exe::ptr::<u8>(0x1_43a2_8c3d) as usize, 0);

        let mut cmd_q = [q_vtbl.as_ptr() as u64, 0, 0];
        let mut q_wrap = [0usize; 4];
        q_wrap[2] = cmd_q.as_mut_ptr() as usize;
        let mut cmd_ctx = [0usize; 4];
        cmd_ctx[2] = q_wrap.as_mut_ptr() as usize;

        let mut ctx = vec![0u8; 0x160];
        wr::<usize>(ctx.as_mut_ptr() as usize + 0x38, fence.as_ptr() as usize);
        wr::<u64>(ctx.as_mut_ptr() as usize + 0x40, 42);
        wr::<usize>(
            exe::ptr::<usize>(0x1_4397_fc58) as usize,
            ctx.as_mut_ptr() as usize,
        );

        MOCK_PTR_RET = cmd_ctx.as_mut_ptr() as usize;
        CMD_CTX_LOG = 0xcbf2_9ce4_8422_2325;
        f();
        (CMD_CTX_LOG, cmd_q, ctx)
    };

    let (log_o, q_o, ctx_o) = run_one(orig_fn);
    let (log_n, q_n, ctx_n) = run_one(new_fn);

    snap_fc.restore();
    snap_srw.restore();
    snap_4698.restore();
    snap_84a0.restore();
    snap_nv.restore();

    crate::diff_eq!(log_o, log_n, "shutdown log");
    crate::diff_eq!(q_o[1..3], q_n[1..3], "shutdown signal args");
    crate::diff_eq!(ctx_o, ctx_n, "shutdown ctx");
    Ok(1)
}

pub unsafe fn diff_d3d_init_device_and_resources(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    let _m01 = crate::difftest::Mock::new(0x1_4240_7c80, mock_ret_u32_zero as *const ())?;
    let _m02 = crate::difftest::Mock::new(0x1_411d_7e20, mock_ret_ring as *const ())?;
    let _m03 = crate::difftest::Mock::new(0x1_40fd_31f0, mock_ret_u32_zero as *const ())?;
    let _m04 = crate::difftest::Mock::new(0x1_4207_61a0, mock_noop_0 as *const ())?;
    let _m05 = crate::difftest::Mock::new(0x1_4209_95c0, mock_arg1_u32 as *const ())?;
    let _m06 = crate::difftest::Mock::new(0x1_411d_d660, mock_ret_ptr as *const ())?;
    let _m07 = crate::difftest::Mock::new(0x1_4207_19f0, mock_719f0 as *const ())?;
    let _m08 = crate::difftest::Mock::new(0x1_4209_ecc0, mock_noop_0 as *const ())?;
    let _m09 = crate::difftest::Mock::new(0x1_4209_7640, mock_noop_0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4205_5620, mock_noop_0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_4205_38c0, mock_noop_0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_411d_9ab0, mock_noop_0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_411d_a620, mock_ret_counter as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_411d_a6a0, mock_arg1_u64 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_411d_7da0, mock_ret_u32_zero as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_411d_82e0, mock_noop_0 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_411d_93b0, mock_noop_0 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_411d_8b90, mock_noop_0 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_4205_fdc0, mock_5fdc0 as *const ())?;

    type FnT = unsafe extern "win64" fn(u32, usize) -> usize;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let snap_fc = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_fb80) as *mut u8, 0x150);
    let snap_a3 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a3_5d50) as *mut u8, 0x410);
    let snap_bac = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4382_3bac) as *mut u8, 4);
    let snap_tbl = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c4_aa00) as *mut u8, 8);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut fake_tls = vec![0u8; 0x80];
    fake_tls[0x70] = 1;
    let worker_entry = 0x4242_1337u32;

    let mut run_one = |f: FnT| -> (u64, u32, Vec<u8>, [u64; 32]) {
        std::ptr::write_bytes(exe::ptr::<u8>(0x1_4397_fb80) as *mut u8, 0, 0x150);
        std::ptr::write_bytes(exe::ptr::<u8>(0x1_43a3_5d50) as *mut u8, 0, 0x410);
        wr::<usize>(
            exe::ptr::<usize>(0x1_43c4_aa00) as usize,
            core::ptr::addr_of!(worker_entry) as usize,
        );
        wr::<u64>(exe::ptr::<u64>(0x1_4397_fc60) as usize, 0x9988_7766);
        wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);

        let mut ctx = vec![0u8; 0xc0];
        let mut ring = vec![0u8; 0x40];
        let mut ring_items = [0u64; 32];
        wr::<u32>(ring.as_mut_ptr() as usize + 0x08, u32::MAX);
        wr::<usize>(
            ring.as_mut_ptr() as usize + 0x10,
            ring_items.as_mut_ptr() as usize,
        );
        wr::<u32>(ring.as_mut_ptr() as usize + 0x24, 32);

        MOCK_PTR_RET = ctx.as_mut_ptr() as usize;
        MOCK_RING_PTR = ring.as_mut_ptr() as usize;
        MOCK_COUNTER = 0;
        CMD_CTX_LOG = 0xcbf2_9ce4_8422_2325;

        let ret = f(2, 0xabcd_1234);
        let bac = rd::<u32>(exe::ptr::<u32>(0x1_4382_3bac) as usize) ^ ((ret == ctx.as_mut_ptr() as usize) as u32);
        wr::<usize>(ring.as_mut_ptr() as usize + 0x10, 0);
        (CMD_CTX_LOG, bac, ring, ring_items)
    };

    let (log_o, bac_o, ring_o, items_o) = run_one(orig_fn);
    let (log_n, bac_n, ring_n, items_n) = run_one(new_fn);

    wr::<usize>(tls_slot, saved_tls0);
    snap_fc.restore();
    snap_a3.restore();
    snap_bac.restore();
    snap_tbl.restore();

    crate::diff_eq!(log_o, log_n, "init log");
    crate::diff_eq!(bac_o, bac_n, "init worker val");
    crate::diff_eq!(ring_o, ring_n, "init ring header");
    crate::diff_eq!(items_o, items_n, "init ring items");
    Ok(1)
}

pub unsafe fn diff_d3d_bind_graphics_pipeline_state(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_hash_64c70(key: usize) -> u64 {
        let mut h = 0xcbf2_9ce4_8422_2325u64;
        for i in (0..0x2c).chain(0x30..0x55) {
            h = (h ^ (rd::<u8>(key + i) as u64)).wrapping_mul(0x1000_0000_01b3);
        }
        h
    }
    unsafe extern "win64" fn fake_lookup_bd00(
        mgr_map: usize,
        out: usize,
        hash_ptr: usize,
    ) -> usize {
        let entry = rd::<usize>(mgr_map);
        wr::<u64>(entry + 0x18, rd::<u64>(hash_ptr));
        wr::<usize>(out, 0);
        wr::<usize>(out + 8, entry);
        out
    }
    unsafe extern "win64" fn fake_set_pso(cmd: usize, pso: usize) {
        wr::<usize>(cmd + 0x08, pso);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_4c70, fake_hash_64c70 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_411d_bd00, fake_lookup_bd00 as *const ())?;

    type FnT = unsafe extern "win64" fn(usize, usize) -> bool;
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let g50 = exe::ptr::<usize>(0x1_4397_fc50) as usize;
    let saved_g50 = rd::<usize>(g50);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut vtbl = [0usize; 32];
    vtbl[0xc8 / 8] = fake_set_pso as *const () as usize;

    let run_one = |f: FnT| -> (bool, u64, usize, u8) {
        let mut cmd = [vtbl.as_ptr() as usize, 0usize];
        let mut cmd_ctx = [0usize; 8];
        cmd_ctx[5] = cmd.as_mut_ptr() as usize; // +0x28
        let mut fake_tls = vec![0u8; 0x6400];
        fake_tls[0x70] = 1;
        wr::<usize>(
            fake_tls.as_mut_ptr() as usize + 0x6300,
            cmd_ctx.as_mut_ptr() as usize,
        );

        let mut entry = [0u64; 4];
        wr::<u8>(entry.as_mut_ptr() as usize + 0x08, 2);
        wr::<usize>(entry.as_mut_ptr() as usize + 0x10, 0xfeed_cafe);

        let mut cache_mgr = vec![0u8; 0x90];
        wr::<usize>(
            cache_mgr.as_mut_ptr() as usize + 0x80,
            entry.as_mut_ptr() as usize,
        );

        let mut rt0 = [0u8; 0x20];
        rt0[0x18] = 0x0a;
        let mut dsv = [0u8; 0x20];
        dsv[0x18] = 0x28;
        let mut ps_obj = [0u8; 0x30];
        wr::<i32>(ps_obj.as_mut_ptr() as usize + 0x28, 1);

        let mut state = vec![0u8; 0xc80];
        let sp = state.as_mut_ptr() as usize;
        wr::<u64>(sp + 0x48, 0x1111);
        wr::<usize>(sp + 0xab0, rt0.as_ptr() as usize);
        wr::<usize>(sp + 0xaf0, dsv.as_ptr() as usize);
        wr::<u64>(sp + 0xaf8, 0x2222);
        wr::<u64>(sp + 0xb00, 0x3333);
        wr::<u64>(sp + 0xb10, 0x4444);
        wr::<u32>(sp + 0xc40, 4);
        wr::<i32>(sp + 0xc44, 2);
        wr::<u64>(sp + 0xc48, 0x5555);
        wr::<usize>(sp + 0xc50, ps_obj.as_ptr() as usize);
        wr::<u64>(sp + 0xc60, 0x6666);
        wr::<u64>(sp + 0xc68, 0x7777);

        wr::<usize>(g50, cache_mgr.as_mut_ptr() as usize);
        wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);
        let ok = f(0, sp);
        (ok, entry[3], cmd[1], state[0xa80])
    };

    let ro = run_one(orig_fn);
    let rn = run_one(new_fn);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<usize>(g50, saved_g50);

    crate::diff_eq!(ro, rn, "bind_graphics_pipeline_state");
    Ok(1)
}

unsafe extern "win64" fn fake_cmd_set_heaps(cmd: usize, count: u32, heaps: usize) {
    wr::<u64>(cmd + 0x10, count as u64);
    wr::<u64>(cmd + 0x18, rd::<u64>(heaps));
    wr::<u64>(cmd + 0x20, rd::<u64>(heaps + 8));
}

unsafe extern "win64" fn fake_cmd_set_root_sig(cmd: usize, sig: usize) {
    wr::<usize>(cmd + 0x08, sig);
}

pub unsafe fn diff_d3d_bind_compute_root_sig_and_heaps(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5ba0(dm: usize) {
        wr::<u64>(dm + 0x08, rd::<u64>(dm + 0x08) + 1);
    }
    unsafe extern "win64" fn fake_5e90(dm: usize) {
        wr::<u64>(dm + 0x10, rd::<u64>(dm + 0x10) + 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_5ba0, fake_5ba0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_5e90, fake_5e90 as *const ())?;

    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let g58 = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g58 = rd::<usize>(g58);
    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut vtbl = [0usize; 32];
    vtbl[0xe0 / 8] = fake_cmd_set_heaps as *const () as usize;
    vtbl[0xe8 / 8] = fake_cmd_set_root_sig as *const () as usize;

    let mut ctx = [0u64; 8];
    ctx[6] = 0x7777_8888; // +0x30
    let h0 = 0xaaaa_1111usize;
    let h1 = 0xbbbb_2222usize;

    let run_one = |f: FnT| -> ([u64; 5], [u64; 3], [usize; 3]) {
        let mut cmd = [vtbl.as_ptr() as u64, 0, 0, 0, 0];
        let mut cmd_ctx = [0usize; 8];
        cmd_ctx[5] = cmd.as_mut_ptr() as usize;
        let mut dm = [0u64; 16];
        wr::<usize>(dm.as_mut_ptr() as usize + 0x50, core::ptr::addr_of!(h0) as usize);
        wr::<usize>(dm.as_mut_ptr() as usize + 0x58, core::ptr::addr_of!(h1) as usize);

        let mut fake_tls = vec![0u8; 0x6400];
        fake_tls[0x70] = 1;
        wr::<usize>(
            fake_tls.as_mut_ptr() as usize + 0x6300,
            cmd_ctx.as_mut_ptr() as usize,
        );
        wr::<usize>(
            fake_tls.as_mut_ptr() as usize + 0x6340,
            dm.as_mut_ptr() as usize,
        );

        let mut state = vec![0u8; 0xa80];
        let sp = state.as_mut_ptr() as usize;
        wr::<usize>(g58, ctx.as_ptr() as usize);
        wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);

        f(0, sp);
        f(0, sp);

        (
            cmd,
            [dm[1], dm[2], cmd_ctx[3] as u64],
            [
                rd::<usize>(sp + 0xa68),
                rd::<usize>(sp + 0xa70),
                rd::<usize>(sp + 0xa78),
            ],
        )
    };

    let ro = run_one(orig_fn);
    let rn = run_one(new_fn);

    wr::<usize>(tls_slot, saved_tls0);
    wr::<usize>(g58, saved_g58);

    crate::diff_eq!(ro, rn, "bind_compute_root_sig_and_heaps");
    Ok(2)
}

pub unsafe fn diff_d3d_bind_graphics_root_sig_and_heaps(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_5c30(dm: usize) {
        wr::<u64>(dm + 0x08, rd::<u64>(dm + 0x08) + 1);
    }
    unsafe extern "win64" fn fake_60c0(dm: usize, has_c68: u8) {
        wr::<u64>(dm + 0x10, rd::<u64>(dm + 0x10) * 31 + (has_c68 as u64) + 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_5c30, fake_5c30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_60c0, fake_60c0 as *const ())?;

    type FnT = unsafe extern "win64" fn(usize, usize);
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut vtbl = [0usize; 32];
    vtbl[0xe0 / 8] = fake_cmd_set_heaps as *const () as usize;
    vtbl[0xf0 / 8] = fake_cmd_set_root_sig as *const () as usize;

    let h0 = 0xcccc_3333usize;
    let h1 = 0xdddd_4444usize;

    let run_one = |f: FnT| -> ([u64; 5], [u64; 3], [usize; 3]) {
        let mut cmd = [vtbl.as_ptr() as u64, 0, 0, 0, 0];
        let mut cmd_ctx = [0usize; 8];
        cmd_ctx[5] = cmd.as_mut_ptr() as usize;
        let mut dm = [0u64; 16];
        wr::<usize>(dm.as_mut_ptr() as usize + 0x20, core::ptr::addr_of!(h0) as usize);
        wr::<usize>(dm.as_mut_ptr() as usize + 0x28, core::ptr::addr_of!(h1) as usize);

        let mut fake_tls = vec![0u8; 0x6400];
        fake_tls[0x70] = 1;
        wr::<usize>(
            fake_tls.as_mut_ptr() as usize + 0x6300,
            cmd_ctx.as_mut_ptr() as usize,
        );
        wr::<usize>(
            fake_tls.as_mut_ptr() as usize + 0x6340,
            dm.as_mut_ptr() as usize,
        );

        let mut state = vec![0u8; 0xc80];
        let sp = state.as_mut_ptr() as usize;
        wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);

        f(sp, 0x5555_6666);
        wr::<usize>(sp + 0xc68, 0x9999);
        f(sp, 0x5555_6666);

        (
            cmd,
            [dm[1], dm[2], cmd_ctx[3] as u64],
            [
                rd::<usize>(sp + 0xa60),
                rd::<usize>(sp + 0xa70),
                rd::<usize>(sp + 0xa78),
            ],
        )
    };

    let ro = run_one(orig_fn);
    let rn = run_one(new_fn);

    wr::<usize>(tls_slot, saved_tls0);

    crate::diff_eq!(ro, rn, "bind_graphics_root_sig_and_heaps");
    Ok(2)
}

pub unsafe fn diff_d3d_reset_tls_command_state(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    unsafe extern "win64" fn fake_f680(
        state: usize,
        shaders: usize,
        refs: usize,
        flag: u8,
    ) {
        let mut s = flag as u64;
        for i in 0..6 {
            s |= rd::<u64>(shaders + i * 8) | rd::<u64>(refs + i * 8);
        }
        wr::<u64>(state + 0x50, 0x1234_5678 ^ s);
    }
    unsafe extern "win64" fn fake_cmd_158(cmd: usize, arg: usize) {
        wr::<usize>(cmd + 0x08, arg ^ 0x4242);
    }

    let _m = crate::difftest::Mock::new(0x1_4206_f680, fake_f680 as *const ())?;

    type FnT = unsafe extern "win64" fn();
    let orig_fn: FnT = std::mem::transmute(orig);
    let new_fn: FnT = std::mem::transmute(new);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let mut vtbl = [0usize; 48];
    vtbl[0x158 / 8] = fake_cmd_158 as *const () as usize;

    let run_one = |f: FnT| -> (Vec<u8>, usize) {
        let mut cmd = [vtbl.as_ptr() as usize, 0usize];
        let mut cmd_ctx = [0usize; 8];
        cmd_ctx[5] = cmd.as_mut_ptr() as usize;

        let mut fake_tls = vec![0u8; 0x47500];
        fake_tls[0x70] = 1;
        wr::<usize>(
            fake_tls.as_mut_ptr() as usize + 0x6300,
            cmd_ctx.as_mut_ptr() as usize,
        );

        let sp = fake_tls.as_mut_ptr() as usize + 0x467e0;
        for i in 0..8 {
            wr::<u64>(sp + i * 8, 0xaaaa_0000 + i as u64);
        }
        wr::<u64>(sp + 0x48, 0x1111);
        wr::<u8>(sp + 0xa80, 1);
        wr::<usize>(sp + 0xaf8, 0x2222);
        wr::<usize>(sp + 0xb00, 0x3333);
        wr::<usize>(sp + 0xb10, 0);
        wr::<usize>(sp + 0xb18, 0x4444);
        wr::<usize>(sp + 0xb20, 0x5555);
        wr::<usize>(sp + 0xb30, 0x6666);
        wr::<usize>(sp + 0xb38, 0x7777);
        wr::<usize>(sp + 0xb40, 0x8888);
        wr::<usize>(sp + 0xb48, 0x9999);
        wr::<usize>(sp + 0xc30, 0xaaaa);
        wr::<usize>(sp + 0xc38, 0xbbbb);

        wr::<usize>(tls_slot, fake_tls.as_mut_ptr() as usize);
        f();
        (fake_tls[0x467e0..0x47460].to_vec(), cmd[1])
    };

    let (state_o, cmd_o) = run_one(orig_fn);
    let (state_n, cmd_n) = run_one(new_fn);

    wr::<usize>(tls_slot, saved_tls0);

    crate::diff_eq!(state_o, state_n, "reset_tls_command_state state");
    crate::diff_eq!(cmd_o, cmd_n, "reset_tls_command_state cmd");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x14206f9f0, "d3d::checkR11G11B10UavLoadSupport_14206f9f0", crate::ns::rend::d3d_cmd_ctx::d3d_check_r11g11b10_uav_load_support as *const (), crate::ns::rend::d3d_cmd_ctx::diff_d3d_check_r11g11b10_uav_load_support),
// Replacement::verified_throws(0x14206fa90, "d3d::shutdownDeviceAndResources_14206fa90", crate::ns::rend::d3d_cmd_ctx::d3d_shutdown_device_and_resources as *const (), Throws::Caught, crate::ns::rend::d3d_cmd_ctx::diff_d3d_shutdown_device_and_resources),
// Replacement::verified_throws(0x142070050, "d3d::initDeviceAndResources_142070050", crate::ns::rend::d3d_cmd_ctx::d3d_init_device_and_resources as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx::diff_d3d_init_device_and_resources),
// Replacement::verified_throws(0x1420702c0, "d3d::flushQueuesAndWaitFence_1420702c0", crate::ns::rend::d3d_cmd_ctx::d3d_flush_queues_and_wait_fence as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx::diff_d3d_flush_queues_and_wait_fence),
// Replacement::verified_throws(0x142070fb0, "d3d::bindGraphicsPipelineState_142070fb0", crate::ns::rend::d3d_cmd_ctx::d3d_bind_graphics_pipeline_state as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx::diff_d3d_bind_graphics_pipeline_state),
// Replacement::verified(0x1420711e0, "d3d::bindComputeRootSigAndHeaps_1420711e0", crate::ns::rend::d3d_cmd_ctx::d3d_bind_compute_root_sig_and_heaps as *const (), crate::ns::rend::d3d_cmd_ctx::diff_d3d_bind_compute_root_sig_and_heaps),
// Replacement::verified(0x142071360, "d3d::bindGraphicsRootSigAndHeaps_142071360", crate::ns::rend::d3d_cmd_ctx::d3d_bind_graphics_root_sig_and_heaps as *const (), crate::ns::rend::d3d_cmd_ctx::diff_d3d_bind_graphics_root_sig_and_heaps),
// Replacement::verified_throws(0x1420714a0, "d3d::resetTlsCommandState_1420714a0", crate::ns::rend::d3d_cmd_ctx::d3d_reset_tls_command_state as *const (), Throws::Discard, crate::ns::rend::d3d_cmd_ctx::diff_d3d_reset_tls_command_state),

