//! Decompiled hot functions shard Y5 (`0x1423ef6c0..0x142420c10`).

use crate::exe::{self, rd, wr};

/// `unknown_1423ef6c0`
/// Original: 0x1423ef6c0 (89 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1423ef6c0(param_1: usize, param_2: u64) -> usize {
    let old_len = rd::<u32>(param_1 + 0x10) as u64;
    exe::call_va!(0x1_423e_f2b0, (param_1, param_2));
    let mut idx = old_len;
    while idx < param_2 {
        let buf = rd::<usize>(param_1);
        wr::<u16>(buf.wrapping_add((idx as usize).wrapping_mul(2)), 0);
        idx = idx.wrapping_add(1);
    }
    let buf = rd::<usize>(param_1);
    wr::<u16>(buf.wrapping_add((param_2 as usize).wrapping_mul(2)), 0);
    wr::<u32>(param_1 + 0x10, param_2 as u32);
    buf
}

/// `unknown_1423f04c0`
/// Original: 0x1423f04c0 (680 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1423f04c0(param_1: usize) -> u32 {
    let mut saved_mxcsr: u32 = 0;
    let _ = exe::call_va!(
        0x1_4248_bd60,
        ((&mut saved_mxcsr as *mut u32) as usize) -> usize
    );
    exe::call_va!(0x1_4240_8090, (param_1 + 0x08));
    wr::<u32>(param_1 + 0x114, 2);
    wr::<u8>(param_1 + 0x118, 1);
    let vtbl = rd::<usize>(param_1);
    let ret = exe::call_va!(rd::<usize>(vtbl + 0x08), (param_1) -> u32);
    wr::<u8>(param_1 + 0x118, 0);
    wr::<u32>(param_1 + 0x114, 3);
    if rd::<u32>(param_1 + 0x110) == 0 {
        let _ = exe::call_iat!(0x1_4264_5790, () -> u32);
    }

    let tls_arr = exe::tls_pointer();
    let tls_slot0 = rd::<usize>(tls_arr);
    let tls_entry = rd::<usize>(tls_slot0 + 0x6368);
    if tls_entry != 0 {
        let lock_addr = exe::ptr::<u8>(0x1_43a2_8df7) as usize;
        let mut wait: u64 = 100;
        loop {
            while rd::<u8>(lock_addr) != 0 {
                core::hint::spin_loop();
            }
            if crate::simd_ops::atomic_xchg_u8(lock_addr as u64, 1) == 0 {
                break;
            }
            exe::call_va!(0x1_4241_7470, ((&mut wait as *mut u64) as usize));
        }

        let g_arr = exe::ptr::<usize>(0x1_43c1_6448) as usize;
        let g_len = exe::ptr::<u32>(0x1_43c1_6450) as usize;
        let mut arr_base = rd::<usize>(g_arr);
        let mut cur = arr_base;
        let mut len = rd::<u32>(g_len) as usize;
        let end = arr_base.wrapping_add(len * 8);
        if arr_base != end {
            loop {
                let cur_end = arr_base.wrapping_add(len * 8);
                if rd::<usize>(cur) == tls_entry {
                    let last_slot = arr_base.wrapping_add(len * 8).wrapping_sub(8);
                    if cur != last_slot {
                        let last_val = rd::<usize>(last_slot);
                        wr::<usize>(last_slot, 0);
                        let old_val = rd::<usize>(cur);
                        wr::<usize>(cur, last_val);
                        if old_val != 0 {
                            exe::call_va!(0x1_424b_3fa0, (old_val + 0x08));
                            exe::call_va!(0x1_4244_2de0, (old_val));
                        }
                        arr_base = rd::<usize>(g_arr);
                        len = rd::<u32>(g_len) as usize;
                    }
                    let tail_slot = arr_base.wrapping_add(len * 8).wrapping_sub(8);
                    let tail_val = rd::<usize>(tail_slot);
                    if tail_val != 0 {
                        exe::call_va!(0x1_424b_3fa0, (tail_val + 0x08));
                        exe::call_va!(0x1_4244_2de0, (tail_val));
                        len = rd::<u32>(g_len) as usize;
                    }
                    wr::<u32>(g_len, (len as u32).wrapping_sub(1));
                    break;
                }
                cur = cur.wrapping_add(8);
                if cur == cur_end {
                    break;
                }
            }
        }

        crate::simd_ops::wu8((lock_addr) as u64, 0); // spinlock release (x86 mov = release)
        let tls_arr2 = exe::tls_pointer();
        let tls_slot0_2 = rd::<usize>(tls_arr2);
        wr::<usize>(tls_slot0_2 + 0x6368, 0);
    }

    #[cfg(target_arch = "x86_64")]
    {
        let mut mxcsr: u32 = 0;
        core::arch::asm!(
            "vstmxcsr dword ptr [{p}]",
            p = in(reg) &mut mxcsr,
            options(nostack, preserves_flags),
        );
        mxcsr = (mxcsr & 0xffff_007f) | (saved_mxcsr & 0xff80);
        core::arch::asm!(
            "vldmxcsr dword ptr [{p}]",
            p = in(reg) &mxcsr,
            options(nostack, preserves_flags),
        );
    }

    ret
}

unsafe fn complete_async_io_request(param_1: usize, req_obj: usize) {
    wr::<u8>(req_obj + 0x09, 1);
    let inner_sync = rd::<usize>(req_obj);
    let old_state = crate::simd_ops::atomic_xchg_u64((inner_sync + 0x08) as u64, 1);
    if ((old_state as u8) >> 1) & 1 != 0 {
        exe::call_iat!(0x1_4264_55a8, (inner_sync + 0x18));
        wr::<u32>(inner_sync + 0x20, rd::<u32>(inner_sync + 0x20).wrapping_add(1));
        exe::call_iat!(0x1_4264_5598, (inner_sync + 0x18));
        exe::call_iat!(0x1_4264_55c0, (inner_sync + 0x10));
    }
    let cb_ptr = (old_state & !3u64) as usize;
    if cb_ptr != 0 {
        let _ = crate::simd_ops::atomic_xadd_u32(inner_sync as u64, 1);
        let cb_vtbl = rd::<usize>(cb_ptr);
        exe::call_va!(rd::<usize>(cb_vtbl), (cb_ptr));
        let old_ref = crate::simd_ops::atomic_xadd_u32(inner_sync as u64, 0xffff_ffff);
        if old_ref == 1 && inner_sync != 0 {
            let cb2 = (rd::<u64>(inner_sync + 0x08) & !3u64) as usize;
            if cb2 != 0 {
                let cb2_vtbl = rd::<usize>(cb2);
                exe::call_va!(rd::<usize>(cb2_vtbl + 0x08), (cb2, 0u32));
                exe::call_va!(0x1_4244_2de0, (cb2));
            }
            exe::call_va!(0x1_4244_2de0, (inner_sync));
        }
        let cb_vtbl2 = rd::<usize>(cb_ptr);
        exe::call_va!(rd::<usize>(cb_vtbl2 + 0x08), (cb_ptr, 0u32));
        exe::call_va!(0x1_4244_2de0, (cb_ptr));
    }
    let _ = param_1;
}

unsafe fn pop_async_io_queue(param_1: usize, out_req: &mut [u8; 0x40]) -> bool {
    let mut q_idx = 0usize;
    while q_idx < 3 {
        let q_base = param_1.wrapping_add(0x8f0 + q_idx * 0x18);
        let count = rd::<u32>(q_base + 0x10);
        if count != 0 {
            let buf_ptr = rd::<usize>(q_base);
            let cap = rd::<u32>(q_base + 0x14) as u64;
            let head = rd::<u32>(q_base + 0x08) as u64;
            let off = ((head % cap) as usize) * 0x40;
            *out_req = rd::<[u8; 0x40]>(buf_ptr.wrapping_add(off));
            let next_head = (head as u32).wrapping_add(1);
            wr::<u32>(q_base + 0x08, if next_head == cap as u32 { 0 } else { next_head });
            wr::<u32>(q_base + 0x10, count.wrapping_sub(1));
            let stat_addr = param_1.wrapping_add(0x938 + q_idx * 4);
            wr::<u32>(stat_addr, rd::<u32>(stat_addr).wrapping_add(1));
            return true;
        }
        q_idx += 1;
    }
    false
}

/// `r_1423f9f80` (`r::detail::win::AsyncIOManager::vfunc_1`)
/// Original: 0x1423f9f80 (1695 bytes, Throws::No)
pub unsafe extern "win64" fn r_1423f9f80(param_1: usize) -> u64 {
    let mut buf_540 = [0u8; 0x400];
    let mut buf_140 = [0u8; 0x400];
    let mut buf_40 = [0u8; 0x100];
    let buf_540_ptr = buf_540.as_mut_ptr() as usize;
    let buf_140_ptr = buf_140.as_mut_ptr() as usize;
    let buf_40_ptr = buf_40.as_mut_ptr() as usize;

    loop {
        let mut wait_res = exe::call_iat!(
            0x1_4264_5540,
            (0x11u32, param_1 + 0x860, 0u32, 0xffff_ffffu32) -> u32
        );
        if wait_res == 0 {
            return 0;
        }
        let mut slot_idx = wait_res.wrapping_sub(1);
        let mut ready_mask = 1u32 << (slot_idx & 31);
        while wait_res < 0x10 {
            let rem_handles = 0x0fu32.wrapping_sub(slot_idx);
            let handles_ptr = param_1
                .wrapping_add(0x870)
                .wrapping_add((slot_idx as usize) * 8);
            let poll_res = exe::call_iat!(
                0x1_4264_5540,
                (rem_handles, handles_ptr, 0u32, 0u32) -> u32
            );
            if poll_res == 0x102 {
                break;
            }
            slot_idx = slot_idx.wrapping_add(poll_res.wrapping_add(1));
            ready_mask |= 1u32 << (slot_idx & 31);
            wait_res = slot_idx.wrapping_add(1);
        }

        let _ = exe::call_va!(0x1_4183_acc2, (buf_540_ptr, 0u32, 0x400u64) -> usize);
        exe::call_va!(0x1_4137_2bf0, (buf_540_ptr));
        let _ = exe::call_va!(0x1_4183_acc2, (buf_40_ptr, 0u32, 0x100u64) -> usize);

        for slot in 0..16usize {
            let bit = 1u32.rotate_left(slot as u32);
            let rsi = param_1.wrapping_add(0x180 + slot * 0x70);
            let r14 = buf_540_ptr.wrapping_add(slot * 0x40);
            let r15 = buf_40_ptr.wrapping_add(slot * 0x10);
            if (ready_mask & bit) != 0 {
                let mut bytes_transferred: u32 = 0;
                let h_file = rd::<u64>(rsi + 0x30);
                let lp_overlapped = rsi.wrapping_sub(0x20);
                let _ = exe::call_iat!(
                    0x1_4264_5558,
                    (
                        h_file,
                        lp_overlapped,
                        (&mut bytes_transferred as *mut u32) as usize,
                        0u32
                    ) -> i32
                );
                wr::<[u8; 0x40]>(r14, rd::<[u8; 0x40]>(rsi));
                wr::<[u8; 0x10]>(r15, rd::<[u8; 0x10]>(rsi + 0x40));
                let h_ev = rd::<u64>(param_1.wrapping_add(0x860 + (slot + 1) * 8));
                let _ = exe::call_iat!(0x1_4264_5550, (h_ev) -> i32);
            }
        }

        let _ = exe::call_va!(0x1_4183_acc2, (buf_140_ptr, 0u32, 0x400u64) -> usize);
        exe::call_va!(0x1_4137_2bf0, (buf_140_ptr));
        exe::call_iat!(0x1_4264_55a8, (param_1 + 0x8e8));

        for slot in 0..16u32 {
            let bit = 1u32.rotate_left(slot);
            let dst_entry = buf_140_ptr.wrapping_add((slot as usize) * 0x40);
            if (ready_mask & bit) != 0 || (rd::<u32>(param_1 + 0x948) & bit) != 0 {
                let mut req = [0u8; 0x40];
                if !pop_async_io_queue(param_1, &mut req) {
                    wr::<u32>(param_1 + 0x948, rd::<u32>(param_1 + 0x948) | (1u32 << slot));
                } else {
                    loop {
                        if req[0x3a] == 0 {
                            wr::<[u8; 0x40]>(dst_entry, req);
                            wr::<u32>(
                                param_1 + 0x948,
                                rd::<u32>(param_1 + 0x948) & !(1u32 << slot),
                            );
                            break;
                        }
                        let mut req_bytes = [0u8; 8];
                        req_bytes.copy_from_slice(&req[0x20..0x28]);
                        let req_obj = u64::from_le_bytes(req_bytes) as usize;
                        let rem = rd::<u32>(req_obj + 0x10).wrapping_sub(1);
                        wr::<u32>(req_obj + 0x10, rem);
                        if rem == 0 {
                            complete_async_io_request(param_1, req_obj);
                            exe::call_va!(0x1_401f_3d00, (req_obj));
                            exe::call_va!(0x1_4243_b3e0, (param_1 + 0x120, req_obj));
                        }
                        if !pop_async_io_queue(param_1, &mut req) {
                            wr::<u32>(
                                param_1 + 0x948,
                                rd::<u32>(param_1 + 0x948) | (1u32 << slot),
                            );
                            break;
                        }
                    }
                }
            }
        }

        exe::call_iat!(0x1_4264_5598, (param_1 + 0x8e8));

        for slot in 0..16u32 {
            let entry_ptr = buf_140_ptr.wrapping_add((slot as usize) * 0x40);
            if rd::<i64>(entry_ptr + 0x30) != -1 {
                exe::call_va!(0x1_423f_9e70, (param_1, entry_ptr, slot));
            }
        }

        for slot in 0..16usize {
            let bit = 1u32.rotate_left(slot as u32);
            let pair_ptr = buf_40_ptr.wrapping_add(slot * 0x10);
            let entry_540 = buf_540_ptr.wrapping_add(slot * 0x40);
            if (ready_mask & bit) != 0 {
                if rd::<u8>(entry_540 + 0x39) != 0 {
                    let io_buf = rd::<usize>(pair_ptr);
                    let src = io_buf.wrapping_add((rd::<u32>(entry_540) & 0xfff) as usize);
                    let mode = rd::<u8>(entry_540 + 0x38);
                    if mode == 0 {
                        let dst = rd::<usize>(entry_540 + 0x18);
                        let len = rd::<u64>(entry_540 + 0x08);
                        let _ = exe::call_va!(0x1_4183_acbc, (dst, src, len) -> usize);
                    } else if mode == 1 {
                        let dst = rd::<usize>(entry_540 + 0x18);
                        let comp_len = rd::<u32>(entry_540 + 0x08);
                        let decomp_cap = rd::<u32>(entry_540 + 0x10);
                        let _ = exe::call_va!(
                            0x1_4246_b9e0,
                            (src, dst, comp_len, decomp_cap) -> i32
                        );
                    }
                    if io_buf != 0 {
                        let alloc_this = param_1.wrapping_add(0x950);
                        let alloc_vtbl = rd::<usize>(alloc_this);
                        let alloc_size = rd::<u64>(pair_ptr + 0x08);
                        exe::call_va!(
                            rd::<usize>(alloc_vtbl + 0x10),
                            (alloc_this, io_buf, alloc_size, 0x1000u32)
                        );
                    }
                }
                let req_obj = rd::<usize>(entry_540 + 0x20);
                let rem = rd::<u32>(req_obj + 0x10).wrapping_sub(1);
                wr::<u32>(req_obj + 0x10, rem);
                if rem == 0 {
                    complete_async_io_request(param_1, req_obj);
                    let req_obj2 = rd::<usize>(entry_540 + 0x20);
                    if req_obj2 != 0 {
                        exe::call_va!(0x1_401f_3d00, (req_obj2));
                        exe::call_va!(0x1_4243_b3e0, (param_1 + 0x120, req_obj2));
                    }
                }
            }
        }
    }
}

/// `r_1423fe080`
/// Original: 0x1423fe080 (523 bytes, Throws::No)
pub unsafe extern "win64" fn r_1423fe080(param_1: usize, param_2: usize) {
    let cur_buf = rd::<usize>(param_1);
    wr::<u32>(param_1 + 0x10, 0);
    if cur_buf != 0 {
        wr::<u16>(cur_buf, 0);
    }
    let byte_len = rd::<usize>(param_2 + 0x08);
    if byte_len == 0 {
        return;
    }

    let mut ptr = rd::<usize>(param_2);
    let end = ptr.wrapping_add(byte_len);
    let lut = exe::ptr::<u8>(0x1_42c6_1100) as usize;
    let mut u16_len: u64 = 0;
    let mut bad = false;

    while ptr < end {
        let b0 = rd::<u8>(ptr) as usize;
        u16_len = u16_len.wrapping_add(1);
        ptr = ptr.wrapping_add(1);
        let cls = rd::<u8>(lut + b0);
        if cls == 0 {
            continue;
        }
        if cls > 3 || ptr.wrapping_add(cls as usize) > end {
            bad = true;
            break;
        }
        let b1 = rd::<u8>(ptr) as usize;
        ptr = ptr.wrapping_add(1);
        if rd::<u8>(lut + b1) != 8 {
            return;
        }
        if cls == 1 {
            continue;
        }
        let b2 = rd::<u8>(ptr) as usize;
        ptr = ptr.wrapping_add(1);
        if rd::<u8>(lut + b2) != 8 {
            return;
        }
        if cls == 2 {
            continue;
        }
        let b3 = rd::<u8>(ptr) as usize;
        u16_len = u16_len.wrapping_add(1);
        ptr = ptr.wrapping_add(1);
        if rd::<u8>(lut + b3) != 8 {
            return;
        }
    }

    if bad {
        u16_len = 0;
    }
    if u16_len == 0 {
        return;
    }

    exe::call_va!(0x1_423e_f6c0, (param_1, u16_len));
    let mut src = rd::<usize>(param_2);
    let mut dst = rd::<usize>(param_1);
    let src_end = src.wrapping_add(byte_len);
    while src < src_end {
        let b0 = rd::<u8>(src) as u32;
        src = src.wrapping_add(1);
        let cls = rd::<u8>(lut + (b0 as usize));
        if cls == 0 {
            wr::<u16>(dst, b0 as u16);
            dst = dst.wrapping_add(2);
        } else if cls == 1 {
            let b1 = rd::<u8>(src) as u16;
            src = src.wrapping_add(1);
            let ch = ((b0 as u16) << 6).wrapping_add(b1).wrapping_sub(0x3080);
            wr::<u16>(dst, ch);
            dst = dst.wrapping_add(2);
        } else if cls == 2 {
            let b1 = rd::<u8>(src) as u16;
            let b2 = rd::<u8>(src + 1) as u16;
            src = src.wrapping_add(2);
            let ch = ((((b0 as u16) << 6).wrapping_add(b1)) << 6)
                .wrapping_add(b2)
                .wrapping_sub(0x2080);
            wr::<u16>(dst, ch);
            dst = dst.wrapping_add(2);
        } else if cls == 3 {
            let b2 = rd::<u8>(src + 1) as u32;
            let b1 = rd::<u8>(src) as u32;
            let b3 = rd::<u8>(src + 2) as u32;
            src = src.wrapping_add(3);
            let cp = b3
                .wrapping_add(0xfc36_df80)
                .wrapping_add(((b1 << 6).wrapping_add(b2)) << 6)
                .wrapping_add(b0 << 18);
            let hi = ((cp >> 10) as u16).wrapping_sub(0x2800);
            let lo = ((cp as u16) & 0x3ff).wrapping_sub(0x2400);
            wr::<u16>(dst, hi);
            wr::<u16>(dst + 2, lo);
            dst = dst.wrapping_add(4);
        }
    }
}

/// `r_14240cb50`
/// Original: 0x14240cb50 (727 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_14240cb50(param_1: usize) -> u32 {
    let _ = crate::simd_ops::atomic_xchg_u8((param_1 + 0x298) as u64, 1);
    let mut bpl: u8 = 1;
    let diff = rd::<u32>(param_1 + 0x288).wrapping_sub(rd::<u32>(param_1 + 0x28c)) as i32;
    let mut rem_outer: i64 = diff as i64;
    if rem_outer != 0 {
        loop {
            let mut wait1: u64 = 100;
            loop {
                while rd::<u8>(param_1 + 0x299) != 0 {
                    core::hint::spin_loop();
                }
                if crate::simd_ops::atomic_xchg_u8((param_1 + 0x299) as u64, 1) == 0 {
                    break;
                }
                exe::call_va!(0x1_4241_7470, ((&mut wait1 as *mut u64) as usize));
            }

            let mut rdi = rd::<usize>(param_1 + 0x278);
            if rd::<u32>(param_1 + 0x288) == rd::<u32>(param_1 + 0x28c) {
                rdi = 0;
            } else {
                let edx = rd::<u32>(param_1 + 0x290);
                wr::<u32>(param_1 + 0x290, edx.wrapping_add(2));
                let limit = rd::<usize>(param_1 + 0x270).wrapping_sub(8);
                let hdr = if rdi <= limit { rd::<i64>(rdi) } else { -1 };
                wr::<u32>(param_1 + 0x290, edx);
                if hdr == -1 {
                    rdi = rd::<usize>(param_1 + 0x268).wrapping_add(8);
                } else if hdr == -2 {
                    rdi = 0;
                } else {
                    rdi = rdi.wrapping_add(8);
                }
            }
            crate::simd_ops::wu8((param_1 + 0x299) as u64, 0); // spinlock release (x86 mov = release)

            while rdi != 0 {
                let ctx = exe::call_va!(0x1_4241_bbe0, () -> usize);
                if rd::<u64>(ctx + 0x10) != 0 {
                    exe::call_va!(0x1_424b_3fa0, (ctx + 0x08));
                }
                let vtbl1 = rd::<usize>(rdi);
                let ok = exe::call_va!(rd::<usize>(vtbl1 + 0x10), (rdi) -> u8);
                bpl = if ok != 0 { bpl } else { 0 };
                let vtbl2 = rd::<usize>(rdi);
                exe::call_va!(rd::<usize>(vtbl2), (rdi, 0u32));

                let mut wait2: u64 = 100;
                loop {
                    while rd::<u8>(param_1 + 0x299) != 0 {
                        core::hint::spin_loop();
                    }
                    if crate::simd_ops::atomic_xchg_u8((param_1 + 0x299) as u64, 1) == 0 {
                        break;
                    }
                    exe::call_va!(0x1_4241_7470, ((&mut wait2 as *mut u64) as usize));
                }

                wr::<u32>(param_1 + 0x290, rd::<u32>(param_1 + 0x290).wrapping_add(2));
                let mut rdx = rd::<usize>(param_1 + 0x278);
                if rd::<u32>(param_1 + 0x288) != rd::<u32>(param_1 + 0x28c) {
                    let r9d = rd::<u32>(param_1 + 0x290);
                    wr::<u32>(param_1 + 0x290, r9d.wrapping_add(2));
                    let limit = rd::<usize>(param_1 + 0x270).wrapping_sub(8);
                    let mut rcx = if rdx <= limit { rd::<i64>(rdx) } else { -1 };
                    let proceed = if rcx == -1 {
                        rdx = rd::<usize>(param_1 + 0x268);
                        rcx = rd::<i64>(rdx);
                        wr::<u32>(param_1 + 0x290, r9d);
                        true
                    } else {
                        wr::<u32>(param_1 + 0x290, r9d);
                        rcx != -2
                    };
                    if proceed {
                        let rax = rdx.wrapping_add(8);
                        if rax != 0 {
                            wr::<i64>(rax - 8, -2);
                            let next_ptr = rax.wrapping_add(rcx as usize) as u64;
                            let _ =
                                crate::simd_ops::atomic_xchg_u64((param_1 + 0x278) as u64, next_ptr);
                            let _ = crate::simd_ops::atomic_xadd_u32((param_1 + 0x28c) as u64, 1);
                        }
                    }
                }
                wr::<u32>(param_1 + 0x290, rd::<u32>(param_1 + 0x290).wrapping_sub(2));

                rdi = rd::<usize>(param_1 + 0x278);
                if rd::<u32>(param_1 + 0x288) == rd::<u32>(param_1 + 0x28c) {
                    rdi = 0;
                } else {
                    let edx = rd::<u32>(param_1 + 0x290);
                    wr::<u32>(param_1 + 0x290, edx.wrapping_add(2));
                    let limit = rd::<usize>(param_1 + 0x270).wrapping_sub(8);
                    let hdr = if rdi <= limit { rd::<i64>(rdi) } else { -1 };
                    wr::<u32>(param_1 + 0x290, edx);
                    if hdr == -1 {
                        rdi = rd::<usize>(param_1 + 0x268).wrapping_add(8);
                    } else if hdr == -2 {
                        rdi = 0;
                    } else {
                        rdi = rdi.wrapping_add(8);
                    }
                }
                crate::simd_ops::wu8((param_1 + 0x299) as u64, 0); // spinlock release (x86 mov = release)
            }

            rem_outer -= 1;
            if rem_outer == 0 {
                break;
            }
        }
    }
    let _ = crate::simd_ops::atomic_xchg_u8((param_1 + 0x298) as u64, 0);
    bpl as u32
}

/// `r_14240d430`
/// Original: 0x14240d430 (60 bytes, Throws::No)
pub unsafe extern "win64" fn r_14240d430(param_1: usize) {
    let rdi = param_1.wrapping_add(0x248);
    exe::call_iat!(0x1_4264_55a8, (rdi + 0x08));
    wr::<u32>(rdi + 0x10, rd::<u32>(rdi + 0x10).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (rdi + 0x08));
    exe::call_iat!(0x1_4264_5590, (rdi));
}

/// `r_14241c490` (`r::RingBufferMemoryResource::vfunc_2`)
/// Original: 0x14241c490 (647 bytes, Throws::No)
pub unsafe extern "win64" fn r_14241c490(
    param_1: usize,
    param_2: u64,
    param_3: u64,
    param_4: u64,
) {
    if param_3 > rd::<u64>(param_1 + 0x20) || param_4 > rd::<u64>(param_1 + 0x28) {
        if param_2 != 0 {
            let fallback = rd::<usize>(param_1 + 0x30);
            let vtbl = rd::<usize>(fallback);
            exe::call_va!(
                rd::<usize>(vtbl + 0x10),
                (fallback, param_2, param_3, param_4)
            );
        }
        return;
    }

    let lock_addr = param_1.wrapping_add(0x40);
    exe::call_iat!(0x1_4264_55a8, (lock_addr));

    let ring_cnt = rd::<u32>(param_1 + 0x80);
    let mut r9d: u32 = 0;
    while r9d != ring_cnt {
        let ring_buf = rd::<usize>(param_1 + 0x70);
        let head = rd::<u32>(param_1 + 0x78) as u64;
        let cap = rd::<u32>(param_1 + 0x84) as u64;
        let slot_idx = ((r9d as u64).wrapping_add(head) % cap) as usize;
        let chunk_base = rd::<u64>(ring_buf.wrapping_add(slot_idx * 0x18));
        if param_2 >= chunk_base && param_2 < chunk_base.wrapping_add(rd::<u64>(param_1 + 0x10)) {
            break;
        }
        r9d = r9d.wrapping_add(1);
    }

    if r9d == ring_cnt {
        let active_base = rd::<u64>(param_1 + 0x48);
        if param_2 >= active_base && param_2 < active_base.wrapping_add(rd::<u64>(param_1 + 0x10)) {
            let new_ref = rd::<u32>(param_1 + 0x58).wrapping_sub(1);
            wr::<u32>(param_1 + 0x58, new_ref);
            if new_ref == 0 {
                wr::<u64>(param_1 + 0x50, 0);
            }
        }
    } else {
        let ring_buf = rd::<usize>(param_1 + 0x70);
        let head = rd::<u32>(param_1 + 0x78) as u64;
        let cap = rd::<u32>(param_1 + 0x84) as u64;
        let slot_off = (((r9d as u64).wrapping_add(head) % cap) as usize) * 0x18;
        let new_ref = rd::<u32>(ring_buf.wrapping_add(slot_off + 0x10)).wrapping_sub(1);
        wr::<u32>(ring_buf.wrapping_add(slot_off + 0x10), new_ref);
        if new_ref == 0 {
            let ring_buf2 = rd::<usize>(param_1 + 0x70);
            let head2 = rd::<u32>(param_1 + 0x78) as u64;
            let cap2 = rd::<u32>(param_1 + 0x84) as u64;
            let slot_off2 = (((r9d as u64).wrapping_add(head2) % cap2) as usize) * 0x18;
            wr::<u64>(ring_buf2.wrapping_add(slot_off2 + 0x08), 0);

            while rd::<u32>(param_1 + 0x80) != 0 {
                let cur_head = rd::<u32>(param_1 + 0x78) as u64;
                let cur_cap = rd::<u32>(param_1 + 0x84) as u64;
                let r15 = rd::<usize>(param_1 + 0x70)
                    .wrapping_add(((cur_head % cur_cap) as usize) * 0x18);
                if rd::<u32>(r15 + 0x10) != 0 {
                    break;
                }
                let free_len = rd::<u32>(param_1 + 0x68) as u64;
                let mut r12_free_buf = rd::<usize>(param_1 + 0x60);
                let r8_bytes = free_len * 0x18;
                let rbp_count = free_len;
                let chunk_size = rd::<u64>(param_1 + 0x10);
                if rbp_count.wrapping_mul(chunk_size) < rd::<u64>(param_1 + 0x18) {
                    let free_cap = rd::<u32>(param_1 + 0x6c) as u64;
                    let mut r9_idx = free_len as u32;
                    if r8_bytes == free_cap * 0x18 {
                        let mut new_cap = (free_cap >> 1).wrapping_add(free_cap);
                        if new_cap < rbp_count.wrapping_add(1) {
                            new_cap = rbp_count.wrapping_add(1);
                        }
                        r12_free_buf = exe::call_va!(0x1_401e_2180, (free_cap, new_cap) -> usize);
                        let cur_bytes = (rd::<u32>(param_1 + 0x68) as u64) * 0x18;
                        let _ = exe::call_va!(
                            0x1_4183_acc8,
                            (r12_free_buf, rd::<usize>(param_1 + 0x60), cur_bytes) -> usize
                        );
                        exe::call_va!(0x1_401d_e830, (param_1 + 0x60));
                        wr::<usize>(param_1 + 0x60, r12_free_buf);
                        wr::<u32>(param_1 + 0x68, rbp_count as u32);
                        wr::<u32>(param_1 + 0x6c, new_cap as u32);
                        r9_idx = rbp_count as u32;
                    }
                    let dst = r12_free_buf.wrapping_add((r9_idx as usize) * 0x18);
                    wr::<u128>(dst, rd::<u128>(r15));
                    wr::<u64>(dst + 0x10, rd::<u64>(r15 + 0x10));
                    wr::<u32>(param_1 + 0x68, rd::<u32>(param_1 + 0x68).wrapping_add(1));
                } else {
                    let upstream = rd::<usize>(param_1 + 0x38);
                    let chunk_ptr = rd::<u64>(r15);
                    if chunk_ptr != 0 {
                        let vtbl = rd::<usize>(upstream);
                        exe::call_va!(
                            rd::<usize>(vtbl + 0x10),
                            (upstream, chunk_ptr, chunk_size, rd::<u64>(param_1 + 0x08))
                        );
                    }
                }
                let cur_ring_cnt = rd::<u32>(param_1 + 0x80);
                if cur_ring_cnt == 0 {
                    break;
                }
                let next_head = rd::<u32>(param_1 + 0x78).wrapping_add(1);
                wr::<u32>(
                    param_1 + 0x78,
                    if next_head == rd::<u32>(param_1 + 0x84) {
                        0
                    } else {
                        next_head
                    },
                );
                wr::<u32>(param_1 + 0x80, cur_ring_cnt.wrapping_sub(1));
            }
        }
    }

    exe::call_iat!(0x1_4264_5598, (lock_addr));
}

/// `r_14241c720` (`r::RingBufferMemoryResource::vfunc_1`)
/// Original: 0x14241c720 (384 bytes, Throws::No)
pub unsafe extern "win64" fn r_14241c720(param_1: usize, param_2: u64, param_3: u64) -> u64 {
    if param_2 > rd::<u64>(param_1 + 0x20) || param_3 > rd::<u64>(param_1 + 0x28) {
        let fallback = rd::<usize>(param_1 + 0x30);
        let vtbl = rd::<usize>(fallback);
        return exe::call_va!(rd::<usize>(vtbl + 0x08), (fallback, param_2, param_3) -> u64);
    }

    let lock_addr = param_1.wrapping_add(0x40);
    exe::call_iat!(0x1_4264_55a8, (lock_addr));

    let active_base = rd::<u64>(param_1 + 0x48);
    let align_mask = !param_3.wrapping_sub(1);
    let need_new_chunk = if active_base != 0 {
        let cur_off = rd::<u64>(param_1 + 0x50);
        let aligned_ptr =
            (cur_off.wrapping_sub(1).wrapping_add(active_base).wrapping_add(param_3)) & align_mask;
        let end_ptr = aligned_ptr.wrapping_add(param_2);
        let chunk_end = rd::<u64>(param_1 + 0x10).wrapping_add(active_base);
        if end_ptr <= chunk_end {
            false
        } else {
            let ring_cnt = rd::<u32>(param_1 + 0x80);
            if ring_cnt == rd::<u32>(param_1 + 0x84) {
                exe::call_va!(0x1_4139_e0a0, (param_1 + 0x70, ring_cnt.wrapping_add(1)));
            }
            let tail = rd::<u32>(param_1 + 0x7c) as usize;
            let ring_buf = rd::<usize>(param_1 + 0x70);
            let dst = ring_buf.wrapping_add(tail * 0x18);
            wr::<u128>(dst, rd::<u128>(param_1 + 0x48));
            wr::<u64>(dst + 0x10, rd::<u64>(param_1 + 0x58));
            let next_tail = rd::<u32>(param_1 + 0x7c).wrapping_add(1);
            wr::<u32>(
                param_1 + 0x7c,
                if next_tail == rd::<u32>(param_1 + 0x84) {
                    0
                } else {
                    next_tail
                },
            );
            wr::<u32>(param_1 + 0x80, rd::<u32>(param_1 + 0x80).wrapping_add(1));
            true
        }
    } else {
        true
    };

    if need_new_chunk {
        let free_buf = rd::<usize>(param_1 + 0x60);
        let free_len = rd::<u32>(param_1 + 0x68) as usize;
        let free_end = free_buf.wrapping_add(free_len * 0x18);
        if free_buf != free_end {
            wr::<u128>(param_1 + 0x48, rd::<u128>(free_end - 0x18));
            wr::<u64>(param_1 + 0x58, rd::<u64>(free_end - 0x08));
            wr::<u32>(param_1 + 0x68, rd::<u32>(param_1 + 0x68).wrapping_sub(1));
        } else {
            let upstream = rd::<usize>(param_1 + 0x38);
            let vtbl = rd::<usize>(upstream);
            let new_chunk = exe::call_va!(
                rd::<usize>(vtbl + 0x08),
                (upstream, rd::<u64>(param_1 + 0x10), rd::<u64>(param_1 + 0x08)) -> u64
            );
            wr::<u64>(param_1 + 0x48, new_chunk);
            wr::<u64>(param_1 + 0x50, 0);
            wr::<u32>(param_1 + 0x58, 0);
        }
    }

    let base_48 = rd::<u64>(param_1 + 0x48);
    let rdi = (rd::<u64>(param_1 + 0x50)
        .wrapping_sub(1)
        .wrapping_add(base_48)
        .wrapping_add(param_3))
        & align_mask;
    wr::<u64>(param_1 + 0x50, rdi.wrapping_sub(base_48).wrapping_add(param_2));
    wr::<u32>(param_1 + 0x58, rd::<u32>(param_1 + 0x58).wrapping_add(1));
    exe::call_iat!(0x1_4264_5598, (lock_addr));
    rdi
}

/// `r_14241d970`
/// Original: 0x14241d970 (514 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_14241d970(param_1: usize, param_2: usize, param_3: i32) {
    wr::<usize>(param_1 + 0x160, param_2);
    let rbx_vtbl = rd::<usize>(param_2);
    let rdi = param_3 as i64 as u64;
    let v0 = exe::call_va!(rd::<usize>(rbx_vtbl), (param_2) -> u64);
    let v9 = exe::call_va!(rd::<usize>(rbx_vtbl + 0x48), (param_2, param_3 as u32, v0) -> u64);
    wr::<u64>(param_1 + 0xc0, v9);
    let arr_c8 = exe::call_va!(0x1_4241_d850, (param_2, param_3 as u32, v0, rdi) -> usize);
    wr::<usize>(param_1 + 0xc8, arr_c8);
    let _ = exe::call_va!(0x1_4183_acc2, (arr_c8, 0u32, rdi.wrapping_mul(8)) -> usize);
    wr::<i32>(param_1 + 0xd0, param_3);

    let mut cur_count = param_3;
    if cur_count > 0 {
        let mut esi: i32 = 0;
        let mut r14_off: usize = 0;
        let r15 = param_1.wrapping_add(0x80);
        let g_538 = exe::ptr::<u8>(0x1_43c1_6538) as usize;
        loop {
            let p2_vtbl = rd::<usize>(param_2);
            let rbx_c0 = rd::<u64>(param_1 + 0xc0);
            let stride = exe::call_va!(rd::<usize>(p2_vtbl + 0x10), (param_2) -> u64);
            let elem_addr = stride.wrapping_mul(esi as i64 as u64).wrapping_add(rbx_c0);
            let created = exe::call_va!(0x1_424b_0060, (elem_addr, param_2, g_538) -> u64);
            let c8_base = rd::<usize>(param_1 + 0xc8);
            wr::<u64>(c8_base.wrapping_add(r14_off), created);
            let c8_base2 = rd::<usize>(param_1 + 0xc8);
            let elem_ptr = rd::<usize>(c8_base2.wrapping_add(r14_off));
            wr::<i32>(elem_ptr + 0x20, esi);

            let c8_base3 = rd::<usize>(param_1 + 0xc8);
            let rcx_98 = rd::<usize>(r15 + 0x18);
            let rdx_node = rd::<usize>(c8_base3.wrapping_add(r14_off)).wrapping_add(0x40);
            let r8_target = if rcx_98 != 0 && rcx_98 != r15 {
                exe::call_va!(0x1_4139_e2e0, (rcx_98, rdx_node) -> usize)
            } else {
                rd::<usize>(r15 + 0x08)
            };

            if r8_target != rdx_node {
                let r9_owner = rd::<usize>(rdx_node + 0x18);
                if r9_owner != 0 && r9_owner != rdx_node {
                    let prev = rd::<usize>(rdx_node + 0x08);
                    let next = rd::<usize>(rdx_node + 0x10);
                    wr::<usize>(prev + 0x10, next);
                    let next2 = rd::<usize>(rdx_node + 0x10);
                    let prev2 = rd::<usize>(rdx_node + 0x08);
                    wr::<usize>(next2 + 0x08, prev2);
                    wr::<usize>(rdx_node + 0x08, rdx_node);
                    wr::<usize>(rdx_node + 0x10, rdx_node);
                    wr::<usize>(rdx_node + 0x18, 0);
                    wr::<u32>(r9_owner + 0x28, rd::<u32>(r9_owner + 0x28).wrapping_sub(1));
                }
                let rax_owner = rd::<usize>(r8_target + 0x18);
                if (rax_owner != 0 && rax_owner != r8_target) || rax_owner == r8_target {
                    wr::<usize>(rdx_node + 0x18, rax_owner);
                    wr::<usize>(rdx_node + 0x08, r8_target);
                    let old_next = rd::<usize>(r8_target + 0x10);
                    wr::<usize>(rdx_node + 0x10, old_next);
                    wr::<usize>(r8_target + 0x10, rdx_node);
                    let new_next = rd::<usize>(rdx_node + 0x10);
                    wr::<usize>(new_next + 0x08, rdx_node);
                    let owner2 = rd::<usize>(rdx_node + 0x18);
                    wr::<u32>(owner2 + 0x28, rd::<u32>(owner2 + 0x28).wrapping_add(1));
                }
            }

            cur_count = rd::<i32>(param_1 + 0xd0);
            esi = esi.wrapping_add(1);
            r14_off = r14_off.wrapping_add(8);
            if esi >= cur_count {
                break;
            }
        }
    }

    if cur_count > 0 {
        let first_elem = rd::<usize>(rd::<usize>(param_1 + 0xc8));
        let first_vtbl = rd::<usize>(first_elem);
        let v150 = exe::call_va!(rd::<usize>(first_vtbl + 0x40), (first_elem) -> u32);
        wr::<u32>(param_1 + 0x150, v150);
    }

    if rd::<u8>(param_1 + 0xf0) != 0 || rdi == 0 {
        exe::call_va!(0x1_4139_e560, (param_1 + 0xd8));
    }

    let cap_110 = rd::<u64>(param_1 + 0x110);
    if rdi > cap_110 {
        exe::call_va!(0x1_4139_e3e0, (param_1 + 0xd8, rdi.wrapping_sub(cap_110)));
    }
}

/// `r_14241f450`
/// Original: 0x14241f450 (579 bytes, Throws::No)
pub unsafe extern "win64" fn r_14241f450(param_1: usize, param_2: usize) {
    let r15 = param_1.wrapping_add(0x1c0);
    let r12 = param_1.wrapping_add(0x1b0);
    let r13 = param_1.wrapping_add(0x1a8);
    let mut ebx: i32 = 0;

    loop {
        let mut edi: i32 = ebx;
        if ebx == 0 {
            exe::call_iat!(0x1_4264_55a8, (r13));
        } else if ebx == 1 {
            exe::call_iat!(0x1_4264_55a8, (r12));
        } else if ebx == 2 {
            exe::call_iat!(0x1_4264_55a8, (r15));
        }
        let ebp = ebx;
        let eax1 = exe::call_va!(0x1_413a_0af0, (0u32, ebx as u32, r13, r12, r15) -> i32);
        if eax1 == -1 {
            let eax2 =
                exe::call_va!(0x1_413a_0af0, ((ebx + 1) as u32, 3u32, r13, r12, r15) -> i32);
            ebx = eax2;
            if eax2 == -1 {
                break;
            }
            edi = edi.wrapping_add(1);
            if edi > 0 {
                exe::call_iat!(0x1_4264_5598, (r13));
            }
            if edi > 1 {
                exe::call_iat!(0x1_4264_5598, (r12));
            }
            if edi > 2 {
                exe::call_iat!(0x1_4264_5598, (r15));
            }
        } else {
            edi = ebx.wrapping_add(1);
            ebx = eax1;
            if ebp <= 0 {
                if edi > 0 {
                    exe::call_iat!(0x1_4264_5598, (r13));
                }
                if edi > 1 {
                    exe::call_iat!(0x1_4264_5598, (r12));
                }
                if edi > 2 {
                    exe::call_iat!(0x1_4264_5598, (r15));
                }
            } else if ebp <= 1 {
                if edi > 1 {
                    exe::call_iat!(0x1_4264_5598, (r12));
                }
                if edi > 2 {
                    exe::call_iat!(0x1_4264_5598, (r15));
                }
            } else if ebp <= 2 {
                if edi > 2 {
                    exe::call_iat!(0x1_4264_5598, (r15));
                }
            }
        }
        exe::call_iat!(0x1_4264_5868, ());
        if ebx == -1 {
            break;
        }
    }

    let mut edi: i32 = 0;
    if rd::<i32>(param_2 + 0x04) > 0 {
        loop {
            let arr_10 = rd::<usize>(param_2 + 0x10);
            let rbp_item = rd::<usize>(arr_10.wrapping_add(((edi as isize as usize) << 4) + 0x08));
            if rd::<u32>(rbp_item + 0x80) == 1 {
                exe::call_va!(0x1_4242_74c0, (rbp_item, rd::<u32>(param_2 + 0x0c)));
                let vec_178 = param_1.wrapping_add(0x178);
                let len = rd::<u32>(vec_178 + 0x08) as u64;
                let cap = rd::<u32>(vec_178 + 0x0c) as u64;
                if len * 8 == cap * 8 {
                    exe::call_va!(0x1_401e_6d50, (vec_178, len.wrapping_add(1)));
                }
                let idx = rd::<u32>(vec_178 + 0x08) as usize;
                let data = rd::<usize>(vec_178);
                wr::<usize>(data.wrapping_add(idx * 8), rbp_item);
                wr::<u32>(vec_178 + 0x08, (idx as u32).wrapping_add(1));
                wr::<u8>(param_1 + 0x170, 1);
            }
            edi = edi.wrapping_add(1);
            if edi >= rd::<i32>(param_2 + 0x04) {
                break;
            }
        }
    }

    let mut r14d: i32 = 0;
    if rd::<i32>(param_2) > 0 {
        loop {
            let arr_10 = rd::<usize>(param_2 + 0x10);
            let rcx_item = rd::<usize>(arr_10.wrapping_add((r14d as isize as usize) << 4));
            if rd::<u32>(rcx_item + 0x80) == 2 {
                let edx_val = rd::<i32>(param_2 + 0x0c);
                if rd::<i32>(rcx_item + 0x38) < edx_val {
                    exe::call_va!(0x1_4242_74c0, (rcx_item, edx_val as u32));
                }
            }
            r14d = r14d.wrapping_add(1);
            if r14d >= rd::<i32>(param_2) {
                break;
            }
        }
    }

    let vec_198 = param_1.wrapping_add(0x198);
    let len = rd::<u32>(vec_198 + 0x08) as u64;
    let cap = rd::<u32>(vec_198 + 0x0c) as u64;
    if len * 8 == cap * 8 {
        exe::call_va!(0x1_401e_6d50, (vec_198, len.wrapping_add(1)));
    }
    let idx = rd::<u32>(vec_198 + 0x08) as usize;
    let data = rd::<usize>(vec_198);
    wr::<usize>(data.wrapping_add(idx * 8), param_2);
    wr::<u32>(vec_198 + 0x08, (idx as u32).wrapping_add(1));
    wr::<u8>(param_1 + 0x172, 1);

    exe::call_iat!(0x1_4264_5598, (r13));
    exe::call_iat!(0x1_4264_5598, (r12));
    exe::call_iat!(0x1_4264_5598, (r15));
}

unsafe fn eval_14241fab0_delegate(entry: usize, param_2: usize) -> usize {
    let ctx = rd::<usize>(entry + 0x168);
    let fn_ptr = rd::<usize>(entry + 0x170);
    if ctx == 0 && fn_ptr == 0 {
        rd::<usize>(entry + 0x158)
    } else {
        exe::call_va!(fn_ptr, (ctx, param_2) -> usize)
    }
}

/// `r_14241fab0`
/// Original: 0x14241fab0 (390 bytes, Throws::No)
pub unsafe extern "win64" fn r_14241fab0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: u64,
    param_5: u8,
) -> usize {
    let rdi_map = param_1.wrapping_add(0x60);
    let mut key: u64 =
        (param_4 & 0xffff_ffff_0000_0000) | (rd::<u32>(param_2 + 0x3c) as u64);
    let mut iter_out: [usize; 2] = [0; 2];
    exe::call_va!(
        0x1_4139_f5c0,
        (
            rdi_map,
            iter_out.as_mut_ptr() as usize,
            (&mut key as *mut u64) as usize
        )
    );
    let end_ctrl = rd::<usize>(rdi_map + 0x18).wrapping_add(rd::<usize>(rdi_map));
    if iter_out[0] == end_ctrl {
        return 0;
    }

    let slot_ptr = iter_out[1];
    let p18 = rd::<usize>(param_2 + 0x18);
    let mut rbx = if rd::<u8>(p18 + 0x08) != 0 {
        exe::call_va!(0x1_4241_d5c0, (rd::<usize>(slot_ptr + 0x08), param_3) -> usize)
    } else {
        eval_14241fab0_delegate(rd::<usize>(slot_ptr + 0x08), param_2)
    };

    if rbx == 0 {
        rbx = eval_14241fab0_delegate(rd::<usize>(slot_ptr + 0x08), param_2);
        if rbx == 0 {
            return 0;
        }
    } else if param_5 == 0 && rd::<u32>(rbx + 0x80) != 5 && rd::<u32>(rbx + 0x80) != 3 {
        let alt = eval_14241fab0_delegate(rd::<usize>(iter_out[1] + 0x08), param_2);
        if alt != 0 {
            rbx = alt;
        }
    }

    let old_ref = crate::simd_ops::atomic_xadd_u32((rbx + 0x24) as u64, 1);
    if old_ref.wrapping_add(1) == 1 {
        let owner = rd::<usize>(rbx + 0x28);
        if owner != 0 {
            exe::call_va!(0x1_4241_d6f0, (owner, rbx));
        }
    }
    rbx
}

/// `r_142420c10`
/// Original: 0x142420c10 (101 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_142420c10(param_1: usize) -> u64 {
    if rd::<u8>(param_1 + 0x08) != 0 {
        return rd::<u64>(param_1 + 0x18);
    }
    let lock_addr = param_1.wrapping_add(0x30);
    exe::call_iat!(0x1_4264_55a8, (lock_addr));
    let _ = exe::call_va!(
        0x1_4139_f3b0,
        (rd::<usize>(param_1 + 0x10), param_1 + 0x20, param_1) -> u8
    );
    wr::<u8>(param_1 + 0x08, 1);
    exe::call_iat!(0x1_4264_5598, (lock_addr));
    rd::<u64>(param_1 + 0x18)
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1423ef6c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RESERVE_ARG: u64 = 0;

    unsafe extern "win64" fn mock_f2b0(wstr: usize, new_len: u64) {
        RESERVE_ARG = new_len;
        wr::<u32>(wstr + 0x0c, (new_len as u32) + 8);
    }

    let _m1 = crate::difftest::Mock::new(0x1_423e_f2b0, mock_f2b0 as *const ())?;

    let run = |f: F, old_len: u32, new_len: u64| -> (u64, u32, u32, [u16; 12]) {
        RESERVE_ARG = 0;
        let mut buf = [0xabcd_u16; 12];
        let mut wstr = [0u64; 4];
        let wp = wstr.as_mut_ptr() as usize;
        wr::<usize>(wp, buf.as_mut_ptr() as usize);
        wr::<u32>(wp + 0x08, 12);
        wr::<u32>(wp + 0x0c, 12);
        wr::<u32>(wp + 0x10, old_len);
        let _ = f(wp, new_len);
        (RESERVE_ARG, rd::<u32>(wp + 0x0c), rd::<u32>(wp + 0x10), buf)
    };

    for (old_len, new_len) in [(2u32, 6u64), (6, 2), (4, 4)] {
        let o = run(f_o, old_len, new_len);
        let n = run(f_n, old_len, new_len);
        crate::diff_eq!(o, n, "1423ef6c0 old={} new={}", old_len, new_len);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1423f04c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_bd60(out_mxcsr: usize) -> usize {
        wr::<u32>(out_mxcsr, 0x1f80);
        out_mxcsr
    }
    unsafe extern "win64" fn mock_8090(arg: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(arg));
    }
    unsafe extern "win64" fn mock_vfunc1(this: usize) -> u32 {
        let st_114 = rd::<u32>(this + 0x114);
        let st_118 = rd::<u8>(this + 0x118) as u32;
        0x1000 + st_114 * 0x10 + st_118
    }
    unsafe extern "win64" fn mock_3fa0(rcx: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(rcx));
    }
    unsafe extern "win64" fn mock_2de0(rcx: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(rcx));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_bd60, mock_bd60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_8090, mock_8090 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_424b_3fa0, mock_3fa0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;

    let tls_slot0 = rd::<usize>(exe::tls_pointer());
    let s_tls = rd::<usize>(tls_slot0 + 0x6368);
    let g_lock = exe::ptr::<u8>(0x1_43a2_8df7) as usize;
    let g_arr = exe::ptr::<usize>(0x1_43c1_6448) as usize;
    let g_len = exe::ptr::<u32>(0x1_43c1_6450) as usize;
    let s_lock = rd::<u8>(g_lock);
    let s_arr = rd::<usize>(g_arr);
    let s_len = rd::<u32>(g_len);

    let run = |f: F, match_first: bool| -> (u32, u32, u8, u32, usize, u64) {
        LOG_SUM = 0;
        let vtbl: [usize; 2] = [0, mock_vfunc1 as *const () as usize];
        let mut obj = [0u8; 0x120];
        let op = obj.as_mut_ptr() as usize;
        wr::<usize>(op, vtbl.as_ptr() as usize);
        wr::<u64>(op + 0x08, 0x55);
        wr::<u32>(op + 0x110, 0);

        let item0: [u64; 2] = [0x111, 0x222];
        let item1: [u64; 2] = [0x333, 0x444];
        let p0 = item0.as_ptr() as usize;
        let p1 = item1.as_ptr() as usize;
        let mut arr: [usize; 2] = [p0, p1];
        wr::<u8>(g_lock, 0);
        wr::<usize>(g_arr, arr.as_mut_ptr() as usize);
        wr::<u32>(g_len, 2);
        wr::<usize>(tls_slot0 + 0x6368, if match_first { p0 } else { p1 });

        let ret = f(op);
        (
            ret,
            rd::<u32>(op + 0x114),
            rd::<u8>(op + 0x118),
            rd::<u32>(g_len),
            rd::<usize>(tls_slot0 + 0x6368),
            LOG_SUM,
        )
    };

    for match_first in [true, false] {
        let o = run(f_o, match_first);
        let n = run(f_n, match_first);
        wr::<usize>(tls_slot0 + 0x6368, s_tls);
        wr::<u8>(g_lock, s_lock);
        wr::<usize>(g_arr, s_arr);
        wr::<u32>(g_len, s_len);
        crate::diff_eq!(o, n, "1423f04c0 match_first={}", match_first);
    }
    Ok(2)
}

pub unsafe fn diff_r_1423f9f80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut WAIT_STEP: u32 = 0;
    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_wait(
        count: u32,
        _handles: usize,
        _wait_all: u32,
        timeout: u32,
    ) -> u32 {
        let step = WAIT_STEP;
        WAIT_STEP += 1;
        if step == 0 {
            // First outer wait (count == 17, timeout == INFINITE): signal slot 0 (return 1)
            assert_eq!(count, 0x11);
            assert_eq!(timeout, 0xffff_ffff);
            1
        } else if step == 1 {
            // Inner poll wait (timeout == 0): return WAIT_TIMEOUT (0x102)
            0x102
        } else {
            // Second outer wait: return 0 (stop event) to exit loop
            0
        }
    }
    unsafe extern "win64" fn mock_overlapped(
        h_file: u64,
        _ovl: usize,
        out_bytes: usize,
        _wait: u32,
    ) -> i32 {
        LOG_SUM = LOG_SUM.wrapping_add(h_file);
        wr::<u32>(out_bytes, 64);
        1
    }
    unsafe extern "win64" fn mock_reset_ev(h_ev: u64) -> i32 {
        LOG_SUM = LOG_SUM.wrapping_add(h_ev);
        1
    }
    unsafe extern "win64" fn mock_9e70(_mgr: usize, entry_ptr: usize, slot: u32) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rd::<u64>(entry_ptr))
            .wrapping_add(slot as u64);
    }
    unsafe extern "win64" fn mock_3d00(req_obj: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u32>(req_obj + 0x08) as u64);
    }
    unsafe extern "win64" fn mock_b3e0(_pool: usize, req_obj: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u32>(req_obj + 0x08) as u64);
    }
    unsafe extern "win64" fn mock_alloc_free(
        _this: usize,
        _ptr: usize,
        size: u64,
        align: u32,
    ) {
        LOG_SUM = LOG_SUM.wrapping_add(size).wrapping_add(align as u64);
    }

    let _im1 = crate::difftest::IatMock::new(0x1_4264_5540, mock_wait as *const ())?;
    let _im2 = crate::difftest::IatMock::new(0x1_4264_5558, mock_overlapped as *const ())?;
    let _im3 = crate::difftest::IatMock::new(0x1_4264_5550, mock_reset_ev as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_423f_9e70, mock_9e70 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401f_3d00, mock_3d00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4243_b3e0, mock_b3e0 as *const ())?;

    let run = |f: F| -> (u64, [u8; 16], u32, u32, u64) {
        WAIT_STEP = 0;
        LOG_SUM = 0;

        let mut src_io_buf = [0u8; 32];
        for i in 0..16 {
            src_io_buf[4 + i] = (i as u8) + 10;
        }
        let mut dst_io_buf = [0u8; 16];

        let mut inner_sync = [0u8; 0x30];
        let isp = inner_sync.as_mut_ptr() as usize;
        wr::<u64>(isp + 0x08, 1);

        let mut req_obj = [0u8; 0x20];
        let rop = req_obj.as_mut_ptr() as usize;
        wr::<usize>(rop, isp);
        wr::<u32>(rop + 0x08, 0x77);
        wr::<u32>(rop + 0x10, 1);

        let mut q_item = [0u8; 0x40];
        let qp = q_item.as_mut_ptr() as usize;
        wr::<u64>(qp, 0x1234);
        wr::<i64>(qp + 0x30, 5); // != -1 so 0x1423f9e70 is called
        wr::<u8>(qp + 0x3a, 0); // not cancelled

        let alloc_vtbl: [usize; 3] = [0, 0, mock_alloc_free as *const () as usize];

        let mut mgr = vec![0u8; 0x970];
        let mp = mgr.as_mut_ptr() as usize;
        // Slot 0 active entry at +0x180
        let s0 = mp + 0x180;
        wr::<u32>(s0, 4); // offset & 0xfff = 4
        wr::<u64>(s0 + 0x08, 16); // copy len = 16
        wr::<usize>(s0 + 0x18, dst_io_buf.as_mut_ptr() as usize);
        wr::<usize>(s0 + 0x20, rop);
        wr::<u64>(s0 + 0x30, 0xaaaa); // hFile
        wr::<u8>(s0 + 0x38, 0); // memcpy mode
        wr::<u8>(s0 + 0x39, 1); // active copy flag
        wr::<usize>(s0 + 0x40, src_io_buf.as_ptr() as usize);
        wr::<u64>(s0 + 0x48, 0x2000); // alloc_size

        wr::<u64>(mp + 0x868, 0xbbbb); // event handle for slot 0
        // Queue 0 at +0x8f0
        wr::<usize>(mp + 0x8f0, qp);
        wr::<u32>(mp + 0x8f8, 0); // head
        wr::<u32>(mp + 0x900, 1); // count = 1
        wr::<u32>(mp + 0x904, 4); // cap = 4
        wr::<usize>(mp + 0x950, alloc_vtbl.as_ptr() as usize);

        let ret = f(mp);
        (
            ret,
            dst_io_buf,
            rd::<u32>(mp + 0x938),
            rd::<u32>(mp + 0x948),
            LOG_SUM,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1423f9f80 out");
    Ok(1)
}

pub unsafe fn diff_r_1423fe080(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_f6c0(wstr: usize, new_len: u64) {
        let buf = rd::<usize>(wstr);
        wr::<u16>(buf.wrapping_add((new_len as usize) * 2), 0);
        wr::<u32>(wstr + 0x10, new_len as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_423e_f6c0, mock_f6c0 as *const ())?;

    let run = |f: F, input: &[u8]| -> (u32, [u16; 16]) {
        let mut buf = [0x7777u16; 16];
        let mut wstr = [0u64; 4];
        let wp = wstr.as_mut_ptr() as usize;
        wr::<usize>(wp, buf.as_mut_ptr() as usize);
        wr::<u32>(wp + 0x08, 16);
        wr::<u32>(wp + 0x0c, 16);
        wr::<u32>(wp + 0x10, 3);

        let slice: [u64; 2] = [input.as_ptr() as u64, input.len() as u64];
        f(wp, slice.as_ptr() as usize);
        (rd::<u32>(wp + 0x10), buf)
    };

    for s in [
        "Aé€𝄞!".as_bytes(),
        b"",
        &[0xff, 0xfe],
        &[0xc3, 0x28],
    ] {
        let o = run(f_o, s);
        let n = run(f_n, s);
        crate::diff_eq!(o, n, "1423fe080 input={:?}", s);
    }
    Ok(4)
}

pub unsafe fn diff_r_14240cb50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TLS_CTX: [u64; 4] = [0; 4];
    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_bbe0() -> usize {
        core::ptr::addr_of_mut!(TLS_CTX) as usize
    }
    unsafe extern "win64" fn mock_3fa0(arg: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(arg));
    }
    unsafe extern "win64" fn mock_item_dtor(this: usize, flag: u32) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rd::<u64>(this + 0x08))
            .wrapping_add(flag as u64);
    }
    unsafe extern "win64" fn mock_item_exec(this: usize) -> u8 {
        (rd::<u64>(this + 0x08) & 1) as u8
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_bbe0, mock_bbe0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424b_3fa0, mock_3fa0 as *const ())?;

    let run = |f: F, item_flag: u64| -> (u32, u32, u8, i64, u64) {
        TLS_CTX = [0, 0x77, 1, 0];
        LOG_SUM = 0;

        let item_vtbl: [usize; 3] = [
            mock_item_dtor as *const () as usize,
            0,
            mock_item_exec as *const () as usize,
        ];
        // Ring buffer layout: [hdr (i64 = 0x10), vtbl_ptr, payload, next_hdr (-2)]
        let mut ring = [0u64; 8];
        let rp = ring.as_mut_ptr() as usize;
        wr::<i64>(rp, 0x10);
        wr::<usize>(rp + 0x08, item_vtbl.as_ptr() as usize);
        wr::<u64>(rp + 0x10, 0x100 + item_flag);
        wr::<i64>(rp + 0x18, -2);

        let mut obj = vec![0u8; 0x2a0];
        let op = obj.as_mut_ptr() as usize;
        wr::<usize>(op + 0x268, rp);
        wr::<usize>(op + 0x270, rp + 0x40);
        wr::<usize>(op + 0x278, rp);
        wr::<u32>(op + 0x288, 1);
        wr::<u32>(op + 0x28c, 0);

        let ret = f(op);
        (
            ret,
            rd::<u32>(op + 0x28c),
            rd::<u8>(op + 0x298),
            rd::<i64>(rp),
            LOG_SUM,
        )
    };

    for flag in [1u64, 0] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        crate::diff_eq!(o, n, "14240cb50 flag={}", flag);
    }
    Ok(2)
}

pub unsafe fn diff_r_14240d430(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, init: u32| -> u32 {
        let mut obj = vec![0u8; 0x260];
        let op = obj.as_mut_ptr() as usize;
        wr::<u32>(op + 0x258, init);
        f(op);
        rd::<u32>(op + 0x258)
    };

    for init in [0u32, 41] {
        let o = run(f_o, init);
        let n = run(f_n, init);
        crate::diff_eq!(o, n, "14240d430 init={}", init);
    }
    Ok(2)
}

pub unsafe fn diff_r_14241c490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FALLBACK_SUM: u64 = 0;

    unsafe extern "win64" fn mock_fallback_dealloc(
        _this: usize,
        ptr: u64,
        size: u64,
        align: u64,
    ) {
        FALLBACK_SUM = FALLBACK_SUM
            .wrapping_add(ptr)
            .wrapping_add(size)
            .wrapping_add(align);
    }

    let run = |f: F, ptr: u64, size: u64| -> (u64, u64, u32, u32, u32, [u64; 3]) {
        FALLBACK_SUM = 0;
        let fb_vtbl: [usize; 3] = [0, 0, mock_fallback_dealloc as *const () as usize];
        let fb_obj: [usize; 1] = [fb_vtbl.as_ptr() as usize];

        let mut ring_entries: [u64; 6] = [0x10_0000, 0x80, 1, 0x20_0000, 0x40, 2];
        let mut free_entries: [u64; 3] = [0; 3];

        let mut res = [0u8; 0x90];
        let rp = res.as_mut_ptr() as usize;
        wr::<u64>(rp + 0x08, 64); // chunk align
        wr::<u64>(rp + 0x10, 0x1000); // chunk size
        wr::<u64>(rp + 0x18, 0x8000); // max pooled bytes
        wr::<u64>(rp + 0x20, 0x400); // max alloc size
        wr::<u64>(rp + 0x28, 64); // max alloc align
        wr::<usize>(rp + 0x30, fb_obj.as_ptr() as usize);
        wr::<u64>(rp + 0x48, 0x30_0000); // active base
        wr::<u64>(rp + 0x50, 0x200); // active offset
        wr::<u32>(rp + 0x58, 1); // active refcount
        wr::<usize>(rp + 0x60, free_entries.as_mut_ptr() as usize);
        wr::<u32>(rp + 0x68, 0); // free_len
        wr::<u32>(rp + 0x6c, 1); // free_cap
        wr::<usize>(rp + 0x70, ring_entries.as_mut_ptr() as usize);
        wr::<u32>(rp + 0x78, 0); // head
        wr::<u32>(rp + 0x80, 2); // ring_cnt
        wr::<u32>(rp + 0x84, 2); // ring_cap

        f(rp, ptr, size, 16);
        (
            FALLBACK_SUM,
            rd::<u64>(rp + 0x50),
            rd::<u32>(rp + 0x58),
            rd::<u32>(rp + 0x68),
            rd::<u32>(rp + 0x80),
            free_entries,
        )
    };

    for (ptr, size) in [
        (0x90_0000u64, 0x800u64), // fallback path
        (0x30_0100, 0x80),        // active chunk path
        (0x10_0200, 0x80),        // retired chunk 0 -> moves to free list
    ] {
        let o = run(f_o, ptr, size);
        let n = run(f_n, ptr, size);
        crate::diff_eq!(o, n, "14241c490 ptr={:#x} size={:#x}", ptr, size);
    }
    Ok(3)
}

pub unsafe fn diff_r_14241c720(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_fallback_alloc(_this: usize, size: u64, align: u64) -> u64 {
        0x80_0000 + size + align
    }
    unsafe extern "win64" fn mock_upstream_alloc(_this: usize, size: u64, align: u64) -> u64 {
        0x50_0000 + size + align
    }

    let run = |f: F, active_off: u64, use_free_pool: bool, size: u64| -> (u64, u64, u64, u32, u32, u32, [u64; 3]) {
        let fb_vtbl: [usize; 2] = [0, mock_fallback_alloc as *const () as usize];
        let fb_obj: [usize; 1] = [fb_vtbl.as_ptr() as usize];
        let up_vtbl: [usize; 2] = [0, mock_upstream_alloc as *const () as usize];
        let up_obj: [usize; 1] = [up_vtbl.as_ptr() as usize];

        let mut free_entries: [u64; 3] = [0x40_0000, 0, 0];
        let mut ring_entries: [u64; 3] = [0; 3];

        let mut res = [0u8; 0x90];
        let rp = res.as_mut_ptr() as usize;
        wr::<u64>(rp + 0x08, 64);
        wr::<u64>(rp + 0x10, 0x1000);
        wr::<u64>(rp + 0x20, 0x400);
        wr::<u64>(rp + 0x28, 64);
        wr::<usize>(rp + 0x30, fb_obj.as_ptr() as usize);
        wr::<usize>(rp + 0x38, up_obj.as_ptr() as usize);
        wr::<u64>(rp + 0x48, 0x10_0000);
        wr::<u64>(rp + 0x50, active_off);
        wr::<u32>(rp + 0x58, 1);
        wr::<usize>(rp + 0x60, free_entries.as_mut_ptr() as usize);
        wr::<u32>(rp + 0x68, if use_free_pool { 1 } else { 0 });
        wr::<usize>(rp + 0x70, ring_entries.as_mut_ptr() as usize);
        wr::<u32>(rp + 0x7c, 0);
        wr::<u32>(rp + 0x80, 0);
        wr::<u32>(rp + 0x84, 2);

        let ret = f(rp, size, 16);
        (
            ret,
            rd::<u64>(rp + 0x48),
            rd::<u64>(rp + 0x50),
            rd::<u32>(rp + 0x58),
            rd::<u32>(rp + 0x68),
            rd::<u32>(rp + 0x80),
            ring_entries,
        )
    };

    for (active_off, use_free, size) in [
        (0x100u64, false, 0x800u64), // fallback
        (0x100, false, 0x80),        // fits in active chunk
        (0xff0, true, 0x80),         // retires active chunk, pops from free pool
        (0xff0, false, 0x80),        // retires active chunk, allocates from upstream
    ] {
        let o = run(f_o, active_off, use_free, size);
        let n = run(f_n, active_off, use_free, size);
        crate::diff_eq!(
            o,
            n,
            "14241c720 off={:#x} free={} size={:#x}",
            active_off,
            use_free,
            size
        );
    }
    Ok(4)
}

pub unsafe fn diff_r_14241d970(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut C8_ARR: [usize; 4] = [0; 4];
    static mut ELEM0: [u8; 0x70] = [0; 0x70];
    static mut ELEM1: [u8; 0x70] = [0; 0x70];
    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_p2_vf0(_this: usize) -> u64 {
        0xabc
    }
    unsafe extern "win64" fn mock_p2_vf2(_this: usize) -> u64 {
        0x100
    }
    unsafe extern "win64" fn mock_p2_vf9(_this: usize, cnt: u32, v0: u64) -> u64 {
        0x5000 + (cnt as u64) * 0x10 + v0
    }
    unsafe extern "win64" fn mock_d850(_a1: usize, _a2: u32, _a3: u64, r9_cnt: u64) -> usize {
        LOG_SUM = LOG_SUM.wrapping_add(r9_cnt);
        core::ptr::addr_of_mut!(C8_ARR) as usize
    }
    unsafe extern "win64" fn mock_elem_vf8(_this: usize) -> u32 {
        0x4242
    }
    unsafe extern "win64" fn mock_0060(addr: u64, _p2: usize, _g: usize) -> u64 {
        LOG_SUM = LOG_SUM.wrapping_add(addr);
        if (addr & 0x100) == 0 {
            core::ptr::addr_of_mut!(ELEM0) as u64
        } else {
            core::ptr::addr_of_mut!(ELEM1) as u64
        }
    }
    unsafe extern "win64" fn mock_e560(_rcx: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0x1000);
    }
    unsafe extern "win64" fn mock_e3e0(_rcx: usize, delta: u64) {
        LOG_SUM = LOG_SUM.wrapping_add(delta);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_d850, mock_d850 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424b_0060, mock_0060 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4139_e560, mock_e560 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4139_e3e0, mock_e3e0 as *const ())?;

    let run = |f: F| -> (u64, i32, u32, i32, i32, u32, u64) {
        C8_ARR = [0; 4];
        ELEM0 = [0; 0x70];
        ELEM1 = [0; 0x70];
        LOG_SUM = 0;

        let mut elem_vtbl = [0usize; 10];
        elem_vtbl[0x40 / 8] = mock_elem_vf8 as *const () as usize;
        wr::<usize>(
            core::ptr::addr_of_mut!(ELEM0) as usize,
            elem_vtbl.as_ptr() as usize,
        );
        wr::<usize>(
            core::ptr::addr_of_mut!(ELEM1) as usize,
            elem_vtbl.as_ptr() as usize,
        );

        let mut p2_vtbl = [0usize; 10];
        p2_vtbl[0] = mock_p2_vf0 as *const () as usize;
        p2_vtbl[0x10 / 8] = mock_p2_vf2 as *const () as usize;
        p2_vtbl[0x48 / 8] = mock_p2_vf9 as *const () as usize;
        let p2_obj: [usize; 1] = [p2_vtbl.as_ptr() as usize];

        let mut owner_node = [0u8; 0x30];
        let onp = owner_node.as_mut_ptr() as usize;
        let mut head_node = [0u8; 0x30];
        let hnp = head_node.as_mut_ptr() as usize;
        wr::<usize>(hnp + 0x08, hnp);
        wr::<usize>(hnp + 0x10, hnp);
        wr::<usize>(hnp + 0x18, onp);

        let mut p1 = vec![0u8; 0x170];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x88, hnp);
        wr::<usize>(p1p + 0x98, 0);
        wr::<u8>(p1p + 0xf0, 1);
        wr::<u64>(p1p + 0x110, 1);

        f(p1p, p2_obj.as_ptr() as usize, 2);
        let e0 = core::ptr::addr_of!(ELEM0) as usize;
        let e1 = core::ptr::addr_of!(ELEM1) as usize;
        (
            rd::<u64>(p1p + 0xc0),
            rd::<i32>(p1p + 0xd0),
            rd::<u32>(p1p + 0x150),
            rd::<i32>(e0 + 0x20),
            rd::<i32>(e1 + 0x20),
            rd::<u32>(onp + 0x28),
            LOG_SUM,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14241d970 out");
    Ok(1)
}

pub unsafe fn diff_r_14241f450(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOCK_STEP: u32 = 0;
    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_0af0(
        start: u32,
        end: u32,
        _l0: usize,
        _l1: usize,
        _l2: usize,
    ) -> i32 {
        let s = LOCK_STEP;
        LOCK_STEP += 1;
        // On step 0 (start=0, end=0): return -1; on step 1 (start=1, end=3): return -1 (all locked!)
        let _ = (start, end, s);
        -1
    }
    unsafe extern "win64" fn mock_74c0(item: usize, val: u32) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rd::<u32>(item + 0x80) as u64)
            .wrapping_add(val as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_0af0, mock_0af0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4242_74c0, mock_74c0 as *const ())?;

    let run = |f: F| -> (u32, u32, u8, u8, u64) {
        LOCK_STEP = 0;
        LOG_SUM = 0;

        let mut item_a = [0u8; 0x90];
        let iap = item_a.as_mut_ptr() as usize;
        wr::<i32>(iap + 0x38, 5);
        wr::<u32>(iap + 0x80, 2);

        let mut item_b = [0u8; 0x90];
        let ibp = item_b.as_mut_ptr() as usize;
        wr::<u32>(ibp + 0x80, 1);

        let pairs: [usize; 2] = [iap, ibp];
        let mut p2 = [0u8; 0x20];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p, 1);
        wr::<i32>(p2p + 0x04, 1);
        wr::<i32>(p2p + 0x0c, 10);
        wr::<usize>(p2p + 0x10, pairs.as_ptr() as usize);

        let mut buf_178 = [0usize; 4];
        let mut buf_198 = [0usize; 4];
        let mut p1 = vec![0u8; 0x1d0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x178, buf_178.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x180, 0);
        wr::<u32>(p1p + 0x184, 4);
        wr::<usize>(p1p + 0x198, buf_198.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x1a0, 0);
        wr::<u32>(p1p + 0x1a4, 4);

        f(p1p, p2p);
        assert_eq!(buf_178[0], ibp);
        assert_eq!(buf_198[0], p2p);
        (
            rd::<u32>(p1p + 0x180),
            rd::<u32>(p1p + 0x1a0),
            rd::<u8>(p1p + 0x170),
            rd::<u8>(p1p + 0x172),
            LOG_SUM,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14241f450 out");
    Ok(1)
}

pub unsafe fn diff_r_14241fab0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOT_NODE: [usize; 2] = [0; 2];
    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_f5c0(map: usize, out_iter: usize, key_ptr: usize) {
        let k = rd::<u32>(key_ptr);
        if k == 999 {
            let end = rd::<usize>(map + 0x18).wrapping_add(rd::<usize>(map));
            wr::<usize>(out_iter, end);
            wr::<usize>(out_iter + 0x08, 0);
        } else {
            wr::<usize>(out_iter, rd::<usize>(map));
            wr::<usize>(out_iter + 0x08, core::ptr::addr_of_mut!(SLOT_NODE) as usize);
        }
    }
    unsafe extern "win64" fn mock_d5c0(entry: usize, arg3: u64) -> usize {
        LOG_SUM = LOG_SUM.wrapping_add(arg3);
        rd::<usize>(entry + 0x150)
    }
    unsafe extern "win64" fn mock_d6f0(owner: usize, item: usize) {
        LOG_SUM = LOG_SUM
            .wrapping_add(owner as u64)
            .wrapping_add(rd::<u32>(item + 0x80) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4139_f5c0, mock_f5c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_d5c0, mock_d5c0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;

    let run = |f: F, key_id: u32, use_d5c0: u8, p5: u8| -> (bool, u32, u32, u64) {
        LOG_SUM = 0;

        let mut res_primary = [0u8; 0x90];
        let rpp = res_primary.as_mut_ptr() as usize;
        wr::<u32>(rpp + 0x24, 0);
        wr::<usize>(rpp + 0x28, 0x1234);
        wr::<u32>(rpp + 0x80, 1); // != 3 and != 5

        let mut res_fallback = [0u8; 0x90];
        let rfp = res_fallback.as_mut_ptr() as usize;
        wr::<u32>(rfp + 0x24, 0);
        wr::<usize>(rfp + 0x28, 0x5678);
        wr::<u32>(rfp + 0x80, 5);

        let mut entry = vec![0u8; 0x180];
        let ep = entry.as_mut_ptr() as usize;
        wr::<usize>(ep + 0x150, rpp);
        wr::<usize>(ep + 0x158, rfp);
        SLOT_NODE = [0, ep];

        let mut p18 = [0u8; 0x10];
        p18[0x08] = use_d5c0;

        let mut p2 = [0u8; 0x40];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x18, p18.as_ptr() as usize);
        wr::<u32>(p2p + 0x3c, key_id);

        let mut p1 = [0u8; 0x90];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x60, 0x1000);
        wr::<usize>(p1p + 0x78, 0x20);

        let ret = f(p1p, p2p, 0x77, 0x8800_0000_0000, p5);
        (
            ret != 0,
            rd::<u32>(rpp + 0x24),
            rd::<u32>(rfp + 0x24),
            LOG_SUM,
        )
    };

    for (key_id, use_d5c0, p5) in [(999u32, 0u8, 0u8), (1, 1, 0), (1, 1, 1), (1, 0, 0)] {
        let o = run(f_o, key_id, use_d5c0, p5);
        let n = run(f_n, key_id, use_d5c0, p5);
        crate::diff_eq!(
            o,
            n,
            "14241fab0 key={} d5c0={} p5={}",
            key_id,
            use_d5c0,
            p5
        );
    }
    Ok(4)
}

pub unsafe fn diff_r_142420c10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLS: u32 = 0;

    unsafe extern "win64" fn mock_f3b0(a1: usize, a2: usize, a3: usize) -> u8 {
        assert_eq!(a2, a3 + 0x20);
        CALLS += 1;
        wr::<u64>(a3 + 0x18, (a1 as u64).wrapping_add(0x999));
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_4139_f3b0, mock_f3b0 as *const ())?;

    let run = |f: F, init_done: u8| -> (u64, u8, u32) {
        CALLS = 0;
        let mut obj = [0u8; 0x40];
        let op = obj.as_mut_ptr() as usize;
        wr::<u8>(op + 0x08, init_done);
        wr::<usize>(op + 0x10, 0x1000);
        wr::<u64>(op + 0x18, 0x7777);
        let ret = f(op);
        (ret, rd::<u8>(op + 0x08), CALLS)
    };

    for done in [1u8, 0] {
        let o = run(f_o, done);
        let n = run(f_n, done);
        crate::diff_eq!(o, n, "142420c10 done={}", done);
    }
    Ok(2)
}

// REGISTRY:
// Replacement::verified(0x1_423e_f6c0, "unknown_1423ef6c0", rend::cauldron_hot_y5::unknown_1423ef6c0 as *const (), rend::cauldron_hot_y5::diff_unknown_1423ef6c0),
// Replacement::verified_throws(0x1_423f_04c0, "unknown_1423f04c0", rend::cauldron_hot_y5::unknown_1423f04c0 as *const (), Throws::Caught, rend::cauldron_hot_y5::diff_unknown_1423f04c0),
// Replacement::verified(0x1_423f_9f80, "r_1423f9f80", rend::cauldron_hot_y5::r_1423f9f80 as *const (), rend::cauldron_hot_y5::diff_r_1423f9f80),
// Replacement::verified(0x1_423f_e080, "r_1423fe080", rend::cauldron_hot_y5::r_1423fe080 as *const (), rend::cauldron_hot_y5::diff_r_1423fe080),
// Replacement::verified_throws(0x1_4240_cb50, "r_14240cb50", rend::cauldron_hot_y5::r_14240cb50 as *const (), Throws::Caught, rend::cauldron_hot_y5::diff_r_14240cb50),
// Replacement::verified(0x1_4240_d430, "r_14240d430", rend::cauldron_hot_y5::r_14240d430 as *const (), rend::cauldron_hot_y5::diff_r_14240d430),
// Replacement::verified(0x1_4241_c490, "r_14241c490", rend::cauldron_hot_y5::r_14241c490 as *const (), rend::cauldron_hot_y5::diff_r_14241c490),
// Replacement::verified(0x1_4241_c720, "r_14241c720", rend::cauldron_hot_y5::r_14241c720 as *const (), rend::cauldron_hot_y5::diff_r_14241c720),
// Replacement::verified_throws(0x1_4241_d970, "r_14241d970", rend::cauldron_hot_y5::r_14241d970 as *const (), Throws::Discard, rend::cauldron_hot_y5::diff_r_14241d970),
// Replacement::verified(0x1_4241_f450, "r_14241f450", rend::cauldron_hot_y5::r_14241f450 as *const (), rend::cauldron_hot_y5::diff_r_14241f450),
// Replacement::verified(0x1_4241_fab0, "r_14241fab0", rend::cauldron_hot_y5::r_14241fab0 as *const (), rend::cauldron_hot_y5::diff_r_14241fab0),
// Replacement::verified_throws(0x1_4242_0c10, "r_142420c10", rend::cauldron_hot_y5::r_142420c10 as *const (), Throws::Caught, rend::cauldron_hot_y5::diff_r_142420c10),
