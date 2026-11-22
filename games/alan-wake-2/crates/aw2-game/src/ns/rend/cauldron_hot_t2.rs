//! Decompiled hot functions shard T2 (`0x1423970b0..0x1424b3910`).

use crate::exe::{self, rd, wr};

/// `unknown_1423970b0`
/// Original: `0x1423970b0` (367 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_1423970b0(param_1: usize, param_2: usize) -> usize {
    let mut local_38: [u64; 2] = [rd::<u64>(param_2), rd::<u64>(param_2 + 0x08)];
    let mut local_28: [u64; 2] = [0; 2];

    let ret_850 = exe::call_va!(
        0x1_401d_f850,
        (
            local_28.as_mut_ptr() as usize,
            local_38.as_mut_ptr() as usize
        ) -> usize
    );
    let s_ptr = rd::<usize>(ret_850);
    let s_len = rd::<u64>(ret_850 + 0x08);
    local_38[0] = s_ptr as u64;
    local_38[1] = s_len;

    let u_var5 = exe::call_va!(0x1_401d_fc80, (local_38.as_mut_ptr() as usize) -> u64);
    let span_len = if u_var5 != 0 && u_var5 <= s_len {
        u_var5
    } else {
        s_len
    };

    let mut cur = rd::<usize>(param_1);
    let count = rd::<u32>(param_1 + 0x08) as usize;
    let end = cur.wrapping_add(count * 8);
    if cur == end {
        return 0;
    }

    let char_tbl = exe::ptr::<u8>(0x1_426d_01e0) as usize;
    while cur != end {
        let item = rd::<usize>(cur);
        let top_byte = rd::<u8>(item + 0x7b) as u32;
        let mut str_data = item.wrapping_add(0x80);
        let cap_bits = if top_byte < 0x80 {
            top_byte * 8
        } else {
            (top_byte << 8).wrapping_sub(0x7c00)
        };
        let used_bits = (rd::<u32>(item + 0x78) & 0x00ff_ffff) << 3;
        if used_bits > cap_bits {
            str_data = rd::<usize>(str_data);
        }

        let item_len = rd::<u32>(item + 0x7c) as u64;
        if item_len == span_len {
            if item_len == 0 {
                return item;
            }
            let mut rem = item_len as usize;
            let mut idx = 0usize;
            let matched = loop {
                let b_item = rd::<u8>(str_data.wrapping_add(idx)) as usize;
                let b_span = rd::<u8>(s_ptr.wrapping_add(idx)) as usize;
                rem -= 1;
                if rd::<i8>(char_tbl.wrapping_add(b_item))
                    != rd::<i8>(char_tbl.wrapping_add(b_span))
                {
                    break false;
                }
                idx += 1;
                if rem == 0 {
                    break true;
                }
            };
            if matched {
                return item;
            }
        }
        cur = cur.wrapping_add(8);
    }
    0
}

/// `r::Streamer::vfunc_1` (`r_14240ce30`)
/// Original: `0x14240ce30` (700 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn r_14240ce30(param_1: usize) -> u64 {
    wr::<u8>(param_1 + 0x241, 1);
    let srw_3a8 = param_1.wrapping_add(0x3a8);

    loop {
        let ctx = exe::call_va!(0x1_4241_bbe0, () -> usize);
        if rd::<u64>(ctx + 0x10) != 0 {
            exe::call_va!(0x1_424b_3fa0, (ctx.wrapping_add(0x08)));
        }

        let mut has_work = false;
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(srw_3a8 + 0x08) != tid {
            exe::call_iat!(0x1_4264_55a8, (srw_3a8));
            wr::<u32>(srw_3a8 + 0x08, tid);
        }
        wr::<u32>(srw_3a8 + 0x0c, rd::<u32>(srw_3a8 + 0x0c).wrapping_add(1));

        let mut cur = rd::<usize>(param_1 + 0x3b8);
        let end0 = cur.wrapping_add((rd::<u32>(param_1 + 0x3c0) as usize) * 8);
        if cur != end0 {
            loop {
                let obj = rd::<usize>(cur);
                let vtbl = rd::<usize>(obj);
                let r = exe::call_va!(rd::<usize>(vtbl + 0x08), (obj) -> u8);
                if r != 0 {
                    has_work = true;
                }
                cur = cur.wrapping_add(8);
                let end = rd::<usize>(param_1 + 0x3b8)
                    .wrapping_add((rd::<u32>(param_1 + 0x3c0) as usize) * 8);
                if cur == end {
                    break;
                }
            }
        }

        let cnt_after_1 = rd::<u32>(srw_3a8 + 0x0c).wrapping_sub(1);
        wr::<u32>(srw_3a8 + 0x0c, cnt_after_1);
        if cnt_after_1 == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32((srw_3a8 + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (srw_3a8));
        }

        if !has_work {
            let cv_248 = param_1.wrapping_add(0x248);
            let srw_250 = cv_248.wrapping_add(0x08);
            exe::call_iat!(0x1_4264_55a8, (srw_250));

            let mut t0_buf: i64 = 0;
            let t0_ptr = exe::call_va!(
                0x1_401f_2f00,
                ((&mut t0_buf as *mut i64) as usize) -> usize
            );
            let start_ns = rd::<i64>(t0_ptr);
            let mut wake_cnt = rd::<i32>(cv_248 + 0x10);

            if wake_cnt <= 0 {
                loop {
                    let mut t1_buf: i64 = 0;
                    let t1_ptr = exe::call_va!(
                        0x1_401f_2f00,
                        ((&mut t1_buf as *mut i64) as usize) -> usize
                    );
                    let mut rem_ns = start_ns
                        .wrapping_add(250_000_000)
                        .wrapping_sub(rd::<i64>(t1_ptr));
                    if rem_ns < 0 {
                        rem_ns = 0;
                    }
                    let timeout_ms = (rem_ns / 1_000_000) as u32;
                    let slept = exe::call_iat!(
                        0x1_4264_55b8,
                        (cv_248, srw_250, timeout_ms, 0u32) -> i32
                    );
                    wake_cnt = rd::<i32>(cv_248 + 0x10);
                    if slept == 0 || wake_cnt > 0 {
                        break;
                    }
                }
            }
            if wake_cnt > 0 {
                wr::<i32>(cv_248 + 0x10, wake_cnt.wrapping_sub(1));
            }
            exe::call_iat!(0x1_4264_5598, (srw_250));
        }

        if rd::<u32>(param_1 + 0x288) != rd::<u32>(param_1 + 0x28c)
            && exe::call_va!(0x1_4240_cb50, (param_1) -> u8) == 0
        {
            let mut cb_cur = rd::<usize>(param_1 + 0x120);
            let cb_end = cb_cur.wrapping_add((rd::<u32>(param_1 + 0x128) as usize) << 4);
            while cb_cur != cb_end {
                let fn_ptr = rd::<usize>(cb_cur + 0x08);
                let arg = rd::<u64>(cb_cur);
                exe::call_va!(fn_ptr, (arg));
                cb_cur = cb_cur.wrapping_add(0x10);
            }
            wr::<u8>(param_1 + 0x241, 0);
            return 0;
        }

        let g_82f0 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f0) as usize);
        let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let ch_arg = rd::<u64>(g_82f0.wrapping_add(worker_idx * 8).wrapping_add(0x2d8));
        exe::call_va!(0x1_4247_5db0, (ch_arg, 0xffff_ffffu64));

        let tid2 = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(srw_3a8 + 0x08) != tid2 {
            exe::call_iat!(0x1_4264_55a8, (srw_3a8));
            wr::<u32>(srw_3a8 + 0x08, tid2);
        }
        wr::<u32>(srw_3a8 + 0x0c, rd::<u32>(srw_3a8 + 0x0c).wrapping_add(1));

        let mut cur2 = rd::<usize>(param_1 + 0x3b8);
        let end2_0 = cur2.wrapping_add((rd::<u32>(param_1 + 0x3c0) as usize) * 8);
        if cur2 != end2_0 {
            loop {
                let obj = rd::<usize>(cur2);
                let vtbl = rd::<usize>(obj);
                let r = exe::call_va!(rd::<usize>(vtbl + 0x08), (obj) -> u8);
                if r != 0 {
                    let obj_b = rd::<usize>(cur2);
                    let vtbl_b = rd::<usize>(obj_b);
                    exe::call_va!(rd::<usize>(vtbl_b + 0x10), (obj_b));
                }
                cur2 = cur2.wrapping_add(8);
                let end2 = rd::<usize>(param_1 + 0x3b8)
                    .wrapping_add((rd::<u32>(param_1 + 0x3c0) as usize) * 8);
                if cur2 == end2 {
                    break;
                }
            }
        }

        let cnt_after_2 = rd::<u32>(srw_3a8 + 0x0c).wrapping_sub(1);
        wr::<u32>(srw_3a8 + 0x0c, cnt_after_2);
        if cnt_after_2 == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32((srw_3a8 + 0x08) as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (srw_3a8));
        }
    }
}

/// `unknown_1424758f0`
/// Original: `0x1424758f0` (169 bytes, `Throws::No`)
pub unsafe extern "win64" fn unknown_1424758f0() {
    let idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let slot_ptr = rd::<usize>((exe::ptr::<usize>(0x1_43c4_aa00) as usize).wrapping_add(idx * 8));
    let slot = if slot_ptr != 0 {
        rd::<u32>(slot_ptr) as usize
    } else {
        0
    };
    let fn_ptr = rd::<usize>((exe::ptr::<usize>(0x1_43c4_acb0) as usize).wrapping_add(slot * 8));
    let g_ef4 = exe::ptr::<u8>(0x1_43a2_8ef4) as usize;
    if fn_ptr == 0 || rd::<u8>(g_ef4) != 0 {
        return;
    }

    let g_lock = exe::ptr::<u64>(0x1_4387_1568) as usize;
    let g_owner = exe::ptr::<u32>(0x1_4387_1570) as usize;
    let g_count = exe::ptr::<u32>(0x1_4387_1574) as usize;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    let acquired = if rd::<u32>(g_owner) == tid {
        true
    } else {
        exe::call_iat!(0x1_4264_5788, (g_lock) -> u8) != 0
    };

    if acquired {
        wr::<u32>(g_count, rd::<u32>(g_count).wrapping_add(1));
        wr::<u32>(g_owner, tid);
        wr::<u8>(g_ef4, 1);
        exe::call_va!(fn_ptr, ());
        let new_cnt = rd::<u32>(g_count).wrapping_sub(1);
        wr::<u32>(g_count, new_cnt);
        wr::<u8>(g_ef4, 0);
        if new_cnt == 0 {
            let _ = crate::simd_ops::atomic_xchg_u32(g_owner as u64, u32::MAX);
            exe::call_iat!(0x1_4264_5598, (g_lock));
        }
    }
}

/// `unknown_14248b1c0`
/// Original: `0x14248b1c0` (162 bytes, `Throws::Caught`)
pub unsafe extern "win64" fn unknown_14248b1c0(param_1: usize, param_2: usize, param_3: u64) {
    if rd::<u64>(param_2) != 0 {
        let lock_ptr = param_1.wrapping_add(0x60);
        exe::call_va!(0x1_4137_8fb0, (lock_ptr));
        if rd::<u64>(param_1 + 0x38) != rd::<u64>(param_1 + 0x30) {
            let mut local_18 = [0u64; 2];
            exe::call_va!(
                0x1_401f_a730,
                (param_1, local_18.as_mut_ptr() as usize, param_2)
            );
        }
        let slot = exe::call_va!(0x1_413c_fce0, (param_1, param_2) -> usize);
        wr::<u64>(slot + 0x08, param_3);
        let g_1b24 = exe::ptr::<u32>(0x1_4384_1b24) as u64;
        let _ = crate::simd_ops::atomic_add_u32(g_1b24, 1);
        if (rd::<u64>(lock_ptr) & 0xffff_ffff_0000_0000) == 0x0000_0001_0000_0000 {
            wr::<u64>(lock_ptr + 0x08, 0);
        }
        let _ = crate::simd_ops::atomic_add_u64(lock_ptr as u64, 0xffff_ffff_0000_0000);
    }
}

/// `unknown_1424b14c0`
/// Original: `0x1424b14c0` (275 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1424b14c0(param_1: u32) -> usize {
    let g_lock = exe::ptr::<u64>(0x1_4387_1780) as usize;
    let g_owner = exe::ptr::<u32>(0x1_4387_1788) as usize;
    let g_count = exe::ptr::<u32>(0x1_4387_178c) as usize;

    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(g_owner) != tid {
        exe::call_iat!(0x1_4264_55a8, (g_lock));
        wr::<u32>(g_owner, tid);
    }
    wr::<u32>(g_count, rd::<u32>(g_count).wrapping_add(1));

    exe::call_va!(0x1_424b_06b0, ());

    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    let b0 = (param_1 & 0xff) as u64;
    let b1 = ((param_1 >> 8) & 0xff) as u64;
    let b2 = ((param_1 >> 16) & 0xff) as u64;
    let b3 = ((param_1 >> 24) & 0xff) as u64;
    let h0 = (b0 ^ FNV_OFFSET).wrapping_mul(FNV_PRIME);
    let h1 = (h0 ^ b1).wrapping_mul(FNV_PRIME);
    let h2 = (h1 ^ b2).wrapping_mul(FNV_PRIME);
    let hash = (h2 ^ b3).wrapping_mul(FNV_PRIME);

    let map_ptr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_83f0) as usize);
    let mask = rd::<u64>(map_ptr + 0x30);
    let buckets = rd::<usize>(map_ptr + 0x18);
    let bucket = buckets.wrapping_add(((hash & mask) as usize) << 4);
    let mut node = rd::<usize>(bucket + 0x08);
    let head = rd::<usize>(map_ptr + 0x08);
    let mut result: usize = 0;

    if node == head {
        node = 0;
    } else {
        let bucket_first = rd::<usize>(bucket);
        while rd::<u32>(node + 0x10) != param_1 {
            if node == bucket_first {
                node = 0;
                break;
            }
            node = rd::<usize>(node + 0x08);
        }
    }
    if node == 0 {
        node = head;
    }
    if node != head {
        result = rd::<usize>(node + 0x18);
    }

    let new_cnt = rd::<u32>(g_count).wrapping_sub(1);
    wr::<u32>(g_count, new_cnt);
    if new_cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32(g_owner as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (g_lock));
    }

    result
}

/// `cityhash_1424b16a0`
/// Original: `0x1424b16a0` (1034 bytes, `Throws::No`)
pub unsafe extern "win64" fn cityhash_1424b16a0(param_1: usize, param_2: u64) -> u64 {
    const K1: u64 = 0xb492_b66f_be98_f273;
    const K2: u64 = 0x9ae1_6a3b_2f90_404f;
    const KMUL: u64 = 0x9ddf_ea08_eb38_2d69;

    if param_2 <= 0x20 {
        if param_2 <= 0x10 {
            return cityhash_1424b1d00(param_1, param_2);
        }
        let end = param_1.wrapping_add(param_2 as usize);
        let mul = K2.wrapping_add(param_2.wrapping_mul(2));
        let a = rd::<u64>(param_1).wrapping_mul(K1);
        let b = rd::<u64>(param_1 + 0x08);
        let c = rd::<u64>(end - 0x08).wrapping_mul(mul);
        let d = rd::<u64>(end - 0x10).wrapping_mul(K2);
        let r9 = b
            .wrapping_add(K2)
            .rotate_right(0x12)
            .wrapping_add(c)
            .wrapping_add(a);
        let r10 = (b
            .wrapping_add(a)
            .rotate_right(0x2b)
            .wrapping_add(c.rotate_right(0x1e))
            .wrapping_add(d)
            ^ r9)
            .wrapping_mul(mul);
        let rcx = ((r10 >> 0x2f) ^ r9 ^ r10).wrapping_mul(mul);
        return ((rcx >> 0x2f) ^ rcx).wrapping_mul(mul);
    }

    let end = param_1.wrapping_add(param_2 as usize);
    if param_2 <= 0x40 {
        let w3 = rd::<u64>(param_1 + 0x18);
        let r10 = rd::<u64>(end - 0x08);
        let rbp = rd::<u64>(end - 0x20);
        let rdi0 = rd::<u64>(param_1).wrapping_mul(K2).wrapping_add(r10);
        let rcx0 = w3.wrapping_mul(9);
        let r14 = K2.wrapping_add(param_2.wrapping_mul(2));
        let rdx = rd::<u64>(end - 0x18);
        let rbx = r14.wrapping_mul(rd::<u64>(end - 0x10));
        let r8 = rcx0.wrapping_add(1).wrapping_add(rdi0 ^ rbp);
        let rcx1 = rcx0.wrapping_add(rd::<u64>(param_1 + 0x10).wrapping_mul(K2));
        let r11 = rcx1.rotate_right(0x2a).wrapping_add(rdx);
        let r9 = rcx1.wrapping_add(rdx);
        let rcx2 = rd::<u64>(param_1 + 0x08).rotate_right(0x1e).wrapping_add(rdx);
        let rax1 = r8.wrapping_add(rcx2.wrapping_mul(9));
        let rdi1 = rdi0
            .rotate_right(0x2b)
            .wrapping_add(rax1)
            .wrapping_mul(r14)
            .swap_bytes();
        let rdi2 = rdi1
            .wrapping_add(r8)
            .wrapping_add(rbx)
            .wrapping_mul(r14)
            .swap_bytes();
        let rdi3 = rdi2
            .wrapping_add(r9)
            .wrapping_add(r11)
            .wrapping_add(r10)
            .wrapping_mul(r14)
            .swap_bytes();
        let rdi4 = rdi3
            .wrapping_add(r9)
            .wrapping_add(rd::<u64>(param_1 + 0x08))
            .wrapping_mul(r14)
            .wrapping_add(rbx)
            .wrapping_add(rbp);
        return ((rdi4 >> 0x2f) ^ rdi4).wrapping_mul(r14).wrapping_add(r11);
    }

    let mut r12 = rd::<u64>(end - 0x28);
    let m_30 = rd::<u64>(end - 0x30);
    let m_40 = rd::<u64>(end - 0x40);
    let m_18 = rd::<u64>(end - 0x18);
    let m_38 = rd::<u64>(end - 0x38);
    let m_10 = rd::<u64>(end - 0x10);
    let m_20 = rd::<u64>(end - 0x20);
    let m_08 = rd::<u64>(end - 0x08);

    let rax0 = (m_30.wrapping_add(param_2) ^ m_18).wrapping_mul(KMUL);
    let rsi0 = m_40.wrapping_add(param_2);
    let mut r14 = m_38.wrapping_add(m_10);
    let rbp0 = m_20.wrapping_add(K1).wrapping_add(r14);
    let rcx0 = (((rax0 >> 0x2f) ^ rax0) ^ m_18).wrapping_mul(KMUL);
    let r10_init = ((rcx0 >> 0x2f) ^ rcx0).wrapping_mul(KMUL);
    let rsi1 = rsi0.wrapping_add(m_30.wrapping_add(m_38));
    let mut r11 = r12.wrapping_add(rsi1);
    let rax1 = r12
        .wrapping_add(rsi0)
        .wrapping_add(r10_init)
        .rotate_right(0x15)
        .wrapping_add(rsi0);
    let mut rsi = rsi1.rotate_right(0x2c).wrapping_add(rax1);
    let mut r9 = r10_init;
    let rbp1 = rbp0.wrapping_add(m_10.wrapping_add(m_18));
    let mut r8 = m_08.wrapping_add(rbp1);
    let rax2 = m_08
        .wrapping_add(rbp0)
        .wrapping_add(r12)
        .rotate_right(0x15)
        .wrapping_add(rbp0);
    r12 = r12.wrapping_mul(K1).wrapping_add(rd::<u64>(param_1));
    let mut rbp = rbp1.rotate_right(0x2c).wrapping_add(rax2);

    let mut rem = param_2.wrapping_sub(1) & !0x3fu64;
    let mut p = param_1;
    let mut rdi_last;
    loop {
        let w0 = rd::<u64>(p);
        let w1 = rd::<u64>(p + 0x08);
        let w2 = rd::<u64>(p + 0x10);
        let w3 = rd::<u64>(p + 0x18);
        let w4 = rd::<u64>(p + 0x20);
        let w5 = rd::<u64>(p + 0x28);
        let w6 = rd::<u64>(p + 0x30);
        let w7 = rd::<u64>(p + 0x38);

        let rdi0 = w1.wrapping_add(r11).wrapping_add(r14).wrapping_add(r12);
        r14 = r14
            .wrapping_add(w6.wrapping_add(rsi))
            .rotate_right(0x2a)
            .wrapping_mul(K1)
            .wrapping_add(w5)
            .wrapping_add(r11);
        r12 = r8.wrapping_add(r9).rotate_right(0x21).wrapping_mul(K1);
        rsi = rsi.wrapping_mul(K1).wrapping_add(w0);
        let rdx_s = rsi;
        rdi_last = rdi0.rotate_right(0x25).wrapping_mul(K1) ^ rbp;
        let rcx_s = w3.wrapping_add(rsi);
        rsi = rsi.wrapping_add(w2.wrapping_add(w1));
        let rax_s = r8
            .wrapping_add(rcx_s)
            .wrapping_add(rdi_last)
            .rotate_right(0x15)
            .wrapping_add(rdx_s);
        let r10_next = w3.wrapping_add(rsi);
        rsi = rsi.rotate_right(0x2c).wrapping_add(rax_s);

        rbp = rbp.wrapping_add(w4).wrapping_add(r12);
        let rdx_b = rbp;
        let rcx_b = w2.wrapping_add(w7).wrapping_add(rbp);
        rbp = rbp.wrapping_add(w5.wrapping_add(w6));
        r8 = w7.wrapping_add(rbp);
        let rax_b = rcx_b
            .wrapping_add(r14)
            .rotate_right(0x15)
            .wrapping_add(rdx_b);
        rbp = rbp.rotate_right(0x2c).wrapping_add(rax_b);

        r9 = rdi_last;
        r11 = r10_next;
        p = p.wrapping_add(0x40);
        rem = rem.wrapping_sub(0x40);
        if rem == 0 {
            break;
        }
    }

    let f_rax0 = (rbp ^ rsi).wrapping_mul(KMUL);
    let f_rdx0 = ((f_rax0 >> 0x2f) ^ rbp ^ f_rax0).wrapping_mul(KMUL);
    let f_r8 = ((f_rdx0 >> 0x2f) ^ f_rdx0)
        .wrapping_mul(KMUL)
        .wrapping_add(r12);

    let f_rax1 = (r8 ^ r11).wrapping_mul(KMUL);
    let f_rcx1 = ((f_rax1 >> 0x2f) ^ f_rax1 ^ r8).wrapping_mul(KMUL);
    let f_rdx1 = ((f_rcx1 >> 0x2f) ^ f_rcx1).wrapping_mul(KMUL).wrapping_add(
        ((r14 >> 0x2f) ^ r14)
            .wrapping_mul(K1)
            .wrapping_add(rdi_last),
    );
    let f_rdx2 = (f_rdx1 ^ f_r8).wrapping_mul(KMUL);
    let f_rcx2 = ((f_rdx2 >> 0x2f) ^ f_r8 ^ f_rdx2).wrapping_mul(KMUL);
    ((f_rcx2 >> 0x2f) ^ f_rcx2).wrapping_mul(KMUL)
}

/// `cityhash_1424b1d00`
/// Original: `0x1424b1d00` (174 bytes, `Throws::No`)
pub unsafe extern "win64" fn cityhash_1424b1d00(param_1: usize, param_2: u64) -> u64 {
    const K0: u64 = 0xc3a5_c85c_97cb_3127;
    const K2: u64 = 0x9ae1_6a3b_2f90_404f;
    const K2_SHL8: u64 = 0xe16a_3b2f_9040_4f00;

    if param_2 >= 8 {
        let mul = K2.wrapping_add(param_2.wrapping_mul(2));
        let a = rd::<u64>(param_1).wrapping_add(K2);
        let b = rd::<u64>(param_1.wrapping_add((param_2 as usize).wrapping_sub(8)));
        let r8 = a.rotate_right(0x19).wrapping_add(b).wrapping_mul(mul);
        let rdx = (b.rotate_right(0x25).wrapping_mul(mul).wrapping_add(a) ^ r8).wrapping_mul(mul);
        let rcx = ((rdx >> 0x2f) ^ rdx ^ r8).wrapping_mul(mul);
        return ((rcx >> 0x2f) ^ rcx).wrapping_mul(mul);
    }
    if param_2 >= 4 {
        let mul = K2.wrapping_add(param_2.wrapping_mul(2));
        let a = rd::<u32>(param_1) as u64;
        let b = rd::<u32>(param_1.wrapping_add((param_2 as usize).wrapping_sub(4))) as u64;
        let rcx = (param_2.wrapping_add(a << 3) ^ b).wrapping_mul(mul);
        let r8 = ((rcx >> 0x2f) ^ rcx ^ b).wrapping_mul(mul);
        return ((r8 >> 0x2f) ^ r8).wrapping_mul(mul);
    }
    if param_2 != 0 {
        let b_mid = rd::<u8>(param_1.wrapping_add((param_2 >> 1) as usize)) as u64;
        let b_first = rd::<u8>(param_1) as u64;
        let b_last = rd::<u8>(param_1.wrapping_add((param_2 as usize).wrapping_sub(1))) as u32;
        let y = b_mid
            .wrapping_mul(K2_SHL8)
            .wrapping_add(b_first.wrapping_mul(K2));
        let z = ((param_2 as u32).wrapping_add(b_last << 2) as u64).wrapping_mul(K0);
        let v = y ^ z;
        return ((v >> 0x2f) ^ v).wrapping_mul(K2);
    }
    K2
}

/// `unknown_1424b3910`
/// Original: `0x1424b3910` (421 bytes, `Throws::Discard`)
pub unsafe extern "win64" fn unknown_1424b3910(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> usize {
    let mut rbx = rd::<usize>(param_1 + 0x08);
    if rbx != 0 {
        let cur_pos = rd::<u64>(rbx + 0x18);
        let base_pos = rd::<u64>(rbx);
        let cap = rd::<u64>(param_1 + 0x10);
        if cur_pos.wrapping_sub(base_pos) <= cap {
            let mask = param_3.wrapping_sub(1);
            let aligned = (!mask) & cur_pos.wrapping_sub(1).wrapping_add(param_3);
            let avail = cap.wrapping_sub(aligned).wrapping_add(base_pos);
            if avail >= param_2 {
                return rbx;
            }
        }
    }

    let rdi = rd::<usize>(param_1);
    let g_ce14 = exe::ptr::<u16>(0x1_43c8_ce14) as u64;
    let mut rsi = param_2;

    let popped = if param_2 <= rd::<u64>(rdi + 0x98) {
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32) as u64;
        if rd::<u64>(rdi + 0x10) == tid {
            let _ = crate::simd_ops::atomic_add_u64((rdi + 0x08) as u64, 1);
        } else {
            exe::call_va!(0x1_4137_9040, (rdi.wrapping_add(0x08), 0u64, 1u64));
        }
        let _ = crate::simd_ops::atomic_xadd_u16(g_ce14, 1);

        let mut head = rd::<u64>(rdi);
        let node = loop {
            let cand = (head & 0x0000_ffff_ffff_ffff) as usize;
            if cand == 0 {
                break 0usize;
            }
            let next = rd::<u64>(cand + 0x28);
            let (ok, prev) = crate::simd_ops::atomic_cmpxchg_u64(rdi as u64, head, next);
            if ok {
                break cand;
            }
            head = prev;
        };
        if node != 0 {
            wr::<u32>(rdi + 0xa0, rd::<u32>(rdi + 0xa0).wrapping_sub(1));
            let _ = crate::simd_ops::atomic_sub_u64((rdi + 0x08) as u64, 1);
            node
        } else {
            let _ = crate::simd_ops::atomic_sub_u64((rdi + 0x08) as u64, 1);
            rsi = rd::<u64>(rdi + 0x98);
            0
        }
    } else {
        0
    };

    if popped != 0 {
        rbx = popped;
    } else {
        let alloc_obj = rd::<usize>(rdi + 0x50);
        let mut alloc_sz: u64 = rsi.wrapping_add(0x40);
        rbx = 0;
        if alloc_obj == 0 {
            exe::call_iat!(0x1_4264_5878, ());
            return 0;
        }
        let vtbl = rd::<usize>(alloc_obj);
        let raw = exe::call_va!(
            rd::<usize>(vtbl + 0x10),
            (alloc_obj, (&mut alloc_sz as *mut u64) as usize) -> usize
        );
        if raw != 0 {
            let data_start = raw.wrapping_add(0x40) as u64;
            wr::<u64>(raw + 0x20, 0);
            wr::<u64>(raw, data_start);
            wr::<u64>(raw + 0x10, data_start.wrapping_add(rsi));
            wr::<u64>(raw + 0x08, data_start);
            wr::<u64>(raw + 0x18, data_start);
            wr::<u64>(raw + 0x28, 0);
            let tag = crate::simd_ops::atomic_xadd_u16(g_ce14, 1);
            wr::<u16>(raw + 0x2e, tag);
            let _ = crate::simd_ops::atomic_xchg_u64((raw + 0x30) as u64, 0x3ff0_0000_0000_0000);
            rbx = raw;
        }
        if rbx == 0 {
            return 0;
        }
    }

    wr::<usize>(rbx + 0x28, rd::<usize>(param_1 + 0x08));
    let tag2 = crate::simd_ops::atomic_xadd_u16(g_ce14, 1);
    wr::<u16>(rbx + 0x2e, tag2);
    wr::<usize>(param_1 + 0x08, rbx);
    let min_sz = rd::<u64>(param_1 + 0x10);
    let add_sz = if param_2 <= min_sz {
        min_sz as u32
    } else {
        param_2 as u32
    };
    wr::<u32>(param_1 + 0x24, rd::<u32>(param_1 + 0x24).wrapping_add(add_sz));
    wr::<u64>(rbx + 0x30, 0);
    let base = rd::<u64>(rbx);
    wr::<u64>(rbx + 0x18, base);
    wr::<u64>(rbx + 0x08, base);
    rbx
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1423970b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LIMIT_C80: u64 = 0;

    unsafe extern "win64" fn mock_850(out_view: usize, in_view: usize) -> usize {
        wr::<u64>(out_view, rd::<u64>(in_view));
        wr::<u64>(out_view + 0x08, rd::<u64>(in_view + 0x08));
        out_view
    }
    unsafe extern "win64" fn mock_c80(_view: usize) -> u64 {
        unsafe { LIMIT_C80 }
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_f850, mock_850 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_fc80, mock_c80 as *const ())?;

    let run = |f: F, limit: u64, query: &[u8]| -> usize {
        unsafe { LIMIT_C80 = limit };

        // Item 0: inline SSO string "alpha" (len 5)
        let mut item0 = vec![0u8; 0xa0];
        let i0 = item0.as_mut_ptr() as usize;
        wr::<u32>(i0 + 0x78, 0x1000_0005);
        wr::<u32>(i0 + 0x7c, 5);
        item0[0x80..0x85].copy_from_slice(b"alpha");

        // Item 1: heap-backed string "TeSt" (len 4)
        let heap_str = b"TeSt\0";
        let mut item1 = vec![0u8; 0xa0];
        let i1 = item1.as_mut_ptr() as usize;
        wr::<u32>(i1 + 0x78, 0x0100_0010); // used_bits (0x80) > cap_bits (0x08) -> heap pointer
        wr::<u32>(i1 + 0x7c, 4);
        wr::<usize>(i1 + 0x80, heap_str.as_ptr() as usize);

        let ptrs: [usize; 2] = [i0, i1];
        let mut container = [0u8; 0x10];
        let cp = container.as_mut_ptr() as usize;
        wr::<usize>(cp, ptrs.as_ptr() as usize);
        wr::<u32>(cp + 0x08, 2);

        let view: [u64; 2] = [query.as_ptr() as u64, query.len() as u64];
        let r = f(cp, view.as_ptr() as usize);
        if r == i0 {
            1
        } else if r == i1 {
            2
        } else {
            r
        }
    };

    for (lim, q) in [
        (0u64, &b"ALPHA"[..]),
        (4u64, &b"test_extra"[..]),
        (0u64, &b"nope"[..]),
    ] {
        let o = run(f_o, lim, q);
        let n = run(f_n, lim, q);
        crate::diff_eq!(o, n, "1423970b0 lim={lim}");
    }
    Ok(3)
}

pub unsafe fn diff_r_14240ce30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ITER_STEP: u32 = 0;
    static mut FAKE_TIME_NS: i64 = 0;
    static mut CTX_BUF: [u64; 3] = [0; 3];
    static mut TRACE_HASH: u64 = 0;

    #[inline(always)]
    unsafe fn rec(v: u64) {
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }

    unsafe extern "win64" fn mock_bbe0() -> usize {
        let step = unsafe { ITER_STEP };
        let ptr = core::ptr::addr_of_mut!(CTX_BUF) as usize;
        wr::<u64>(ptr + 0x10, if step == 0 { 1 } else { 0 });
        ptr
    }
    unsafe extern "win64" fn mock_3fa0(_rcx: usize) {
        unsafe { rec(0x3fa0) };
    }
    unsafe extern "win64" fn mock_2f00(out_ptr: usize) -> usize {
        let t = unsafe {
            FAKE_TIME_NS += 50_000_000;
            FAKE_TIME_NS
        };
        wr::<i64>(out_ptr, t);
        out_ptr
    }
    unsafe extern "win64" fn mock_sleep_cv(
        cv: usize,
        _srw: usize,
        timeout_ms: u32,
        flags: u32,
    ) -> i32 {
        unsafe { rec(0x55b8 ^ (timeout_ms as u64) ^ (flags as u64)) };
        wr::<i32>(cv + 0x10, 2);
        1
    }
    unsafe extern "win64" fn mock_cb50(_p1: usize) -> u8 {
        let step = unsafe { ITER_STEP };
        unsafe {
            ITER_STEP = step + 1;
            rec(0xcb50 ^ (step as u64));
        }
        if step == 0 { 1 } else { 0 }
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_5db0(rcx: u64, rdx: u64) {
        unsafe { rec(0x5db0 ^ rcx ^ (rdx & 0xffff_ffff)) };
    }
    unsafe extern "win64" fn mock_vfunc_08(this: usize) -> u8 {
        let mode = rd::<u8>(this + 0x08);
        let step = unsafe { ITER_STEP };
        unsafe { rec(0x0008 ^ (mode as u64) ^ (step as u64)) };
        if step == 0 { 0 } else { mode }
    }
    unsafe extern "win64" fn mock_vfunc_10(this: usize) {
        unsafe { rec(0x0010 ^ (rd::<u8>(this + 0x08) as u64)) };
    }
    unsafe extern "win64" fn mock_exit_cb(arg: u64) {
        unsafe { rec(0xecb0 ^ arg) };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_bbe0, mock_bbe0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424b_3fa0, mock_3fa0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401f_2f00, mock_2f00 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_cb50, mock_cb50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4247_5db0, mock_5db0 as *const ())?;
    let _im = crate::difftest::IatMock::new(0x1_4264_55b8, mock_sleep_cv as *const ())?;

    let g_82f0 = exe::ptr::<usize>(0x1_43a0_82f0) as usize;
    let s_82f0 = rd::<usize>(g_82f0);

    let run = |f: F| -> (u64, u64, u8, i32, u32, u32) {
        unsafe {
            ITER_STEP = 0;
            FAKE_TIME_NS = 1_000_000_000;
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
        }

        let mut g_obj = vec![0u8; 0x300];
        wr::<u64>(g_obj.as_mut_ptr() as usize + 0x2d8 + 8, 0x7777_8888);
        wr::<usize>(g_82f0, g_obj.as_mut_ptr() as usize);

        let vtbl: [usize; 3] = [
            0,
            mock_vfunc_08 as *const () as usize,
            mock_vfunc_10 as *const () as usize,
        ];
        let mut obj0 = [0u8; 0x10];
        let mut obj1 = [0u8; 0x10];
        wr::<usize>(obj0.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<u8>(obj0.as_mut_ptr() as usize + 0x08, 0);
        wr::<usize>(obj1.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<u8>(obj1.as_mut_ptr() as usize + 0x08, 1);
        let objs: [usize; 2] = [obj0.as_mut_ptr() as usize, obj1.as_mut_ptr() as usize];

        let cbs: [u64; 4] = [
            0xaa11,
            mock_exit_cb as *const () as usize as u64,
            0xbb22,
            mock_exit_cb as *const () as usize as u64,
        ];

        let mut streamer = vec![0u8; 0x3d0];
        let sp = streamer.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x120, cbs.as_ptr() as usize);
        wr::<u32>(sp + 0x128, 2);
        wr::<i32>(sp + 0x258, 0);
        wr::<u32>(sp + 0x288, 1);
        wr::<u32>(sp + 0x28c, 2);
        wr::<u32>(sp + 0x3b0, u32::MAX);
        wr::<u32>(sp + 0x3b4, 0);
        wr::<usize>(sp + 0x3b8, objs.as_ptr() as usize);
        wr::<u32>(sp + 0x3c0, 2);

        let r = f(sp);
        (
            r,
            unsafe { TRACE_HASH },
            rd::<u8>(sp + 0x241),
            rd::<i32>(sp + 0x258),
            rd::<u32>(sp + 0x3b0),
            rd::<u32>(sp + 0x3b4),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_82f0, s_82f0);

    crate::diff_eq!(o, n, "14240ce30 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1424758f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE_HASH: u64 = 0;

    unsafe extern "win64" fn mock_7c80() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_cb() {
        let g_ef4 = rd::<u8>(exe::ptr::<u8>(0x1_43a2_8ef4) as usize) as u64;
        let g_cnt = rd::<u32>(exe::ptr::<u32>(0x1_4387_1574) as usize) as u64;
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ 0x58f0 ^ (g_ef4 << 8) ^ g_cnt)
                .wrapping_mul(0x1000_0000_01b3);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;

    let g_aa08 = (exe::ptr::<usize>(0x1_43c4_aa00) as usize) + 8;
    let g_acb8 = (exe::ptr::<usize>(0x1_43c4_acb0) as usize) + 16;
    let g_ef4 = exe::ptr::<u8>(0x1_43a2_8ef4) as usize;
    let g_lock = exe::ptr::<u64>(0x1_4387_1568) as usize;
    let g_owner = exe::ptr::<u32>(0x1_4387_1570) as usize;
    let g_count = exe::ptr::<u32>(0x1_4387_1574) as usize;

    let s_aa08 = rd::<usize>(g_aa08);
    let s_acb8 = rd::<usize>(g_acb8);
    let s_ef4 = rd::<u8>(g_ef4);
    let s_lock = rd::<u64>(g_lock);
    let s_owner = rd::<u32>(g_owner);
    let s_count = rd::<u32>(g_count);

    let slot_val: u32 = 2;
    let run = |f: F, init_ef4: u8| -> (u64, u8, u32, u32) {
        unsafe { TRACE_HASH = 0xcbf2_9ce4_8422_2325 };
        wr::<usize>(g_aa08, (&slot_val as *const u32) as usize);
        wr::<usize>(g_acb8, mock_cb as *const () as usize);
        wr::<u8>(g_ef4, init_ef4);
        wr::<u64>(g_lock, 0);
        wr::<u32>(g_owner, u32::MAX);
        wr::<u32>(g_count, 0);

        f();
        (
            unsafe { TRACE_HASH },
            rd::<u8>(g_ef4),
            rd::<u32>(g_owner),
            rd::<u32>(g_count),
        )
    };

    for init_ef4 in [0u8, 1u8] {
        let o = run(f_o, init_ef4);
        let n = run(f_n, init_ef4);
        crate::diff_eq!(o, n, "1424758f0 ef4={init_ef4}");
    }

    wr::<usize>(g_aa08, s_aa08);
    wr::<usize>(g_acb8, s_acb8);
    wr::<u8>(g_ef4, s_ef4);
    wr::<u64>(g_lock, s_lock);
    wr::<u32>(g_owner, s_owner);
    wr::<u32>(g_count, s_count);

    Ok(2)
}

pub unsafe fn diff_unknown_14248b1c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOT_BUF: [u64; 2] = [0; 2];
    static mut TRACE_HASH: u64 = 0;

    unsafe extern "win64" fn mock_8fb0(lock_ptr: usize) {
        let v = rd::<u64>(lock_ptr);
        unsafe {
            TRACE_HASH = (TRACE_HASH ^ 0x8fb0 ^ v).wrapping_mul(0x1000_0000_01b3);
        }
    }
    unsafe extern "win64" fn mock_a730(_p1: usize, out_tmp: usize, key_ptr: usize) {
        wr::<u64>(out_tmp, rd::<u64>(key_ptr));
        unsafe {
            TRACE_HASH =
                (TRACE_HASH ^ 0xa730 ^ rd::<u64>(key_ptr)).wrapping_mul(0x1000_0000_01b3);
        }
    }
    unsafe extern "win64" fn mock_fce0(_p1: usize, key_ptr: usize) -> usize {
        let p = core::ptr::addr_of_mut!(SLOT_BUF) as usize;
        wr::<u64>(p, rd::<u64>(key_ptr));
        p
    }

    let _m1 = crate::difftest::Mock::new(0x1_4137_8fb0, mock_8fb0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_413c_fce0, mock_fce0 as *const ())?;

    let g_1b24 = exe::ptr::<u32>(0x1_4384_1b24) as usize;
    let s_1b24 = rd::<u32>(g_1b24);

    let run = |f: F, key: u64, init_lock: u64| -> (u64, [u64; 2], u64, u64, u32) {
        unsafe {
            TRACE_HASH = 0xcbf2_9ce4_8422_2325;
            SLOT_BUF = [0; 2];
        }
        wr::<u32>(g_1b24, 10);

        let mut p1 = [0u8; 0x78];
        let p1_p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1_p + 0x30, 1);
        wr::<u64>(p1_p + 0x38, 2);
        wr::<u64>(p1_p + 0x60, init_lock);
        wr::<u64>(p1_p + 0x68, 0x9999);

        let k = key;
        f(p1_p, (&k as *const u64) as usize, 0xabcd_ef01);
        (
            unsafe { TRACE_HASH },
            unsafe { SLOT_BUF },
            rd::<u64>(p1_p + 0x60),
            rd::<u64>(p1_p + 0x68),
            rd::<u32>(g_1b24),
        )
    };

    for (k, lck) in [
        (0u64, 0x0000_0001_0000_0005u64),
        (0x1234u64, 0x0000_0001_0000_0005u64),
        (0x5678u64, 0x0000_0002_0000_0005u64),
    ] {
        let o = run(f_o, k, lck);
        let n = run(f_n, k, lck);
        crate::diff_eq!(o, n, "14248b1c0 k={k}");
    }

    wr::<u32>(g_1b24, s_1b24);
    Ok(3)
}

pub unsafe fn diff_unknown_1424b14c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_06b0() {}
    let _m1 = crate::difftest::Mock::new(0x1_424b_06b0, mock_06b0 as *const ())?;

    let g_lock = exe::ptr::<u64>(0x1_4387_1780) as usize;
    let g_owner = exe::ptr::<u32>(0x1_4387_1788) as usize;
    let g_count = exe::ptr::<u32>(0x1_4387_178c) as usize;
    let g_map = exe::ptr::<usize>(0x1_43a0_83f0) as usize;

    let s_lock = rd::<u64>(g_lock);
    let s_owner = rd::<u32>(g_owner);
    let s_count = rd::<u32>(g_count);
    let s_map = rd::<usize>(g_map);

    let run = |f: F, query_crc: u32| -> (usize, u32, u32) {
        wr::<u64>(g_lock, 0);
        wr::<u32>(g_owner, u32::MAX);
        wr::<u32>(g_count, 0);

        // Single-bucket table (mask = 0) with 2 chained nodes: node_b -> node_a
        let mut head = [0u64; 4];
        let mut node_a = [0u64; 4];
        let mut node_b = [0u64; 4];
        let hp = head.as_mut_ptr() as usize;
        let na = node_a.as_mut_ptr() as usize;
        let nb = node_b.as_mut_ptr() as usize;

        wr::<usize>(nb + 0x08, na);
        wr::<u32>(nb + 0x10, 0x1111_2222);
        wr::<usize>(nb + 0x18, 0xaaaa_0001);

        wr::<usize>(na + 0x08, hp);
        wr::<u32>(na + 0x10, 0x3333_4444);
        wr::<usize>(na + 0x18, 0xbbbb_0002);

        let bucket: [usize; 2] = [na, nb];
        let mut map_obj = [0u8; 0x40];
        let mp = map_obj.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x08, hp);
        wr::<usize>(mp + 0x18, bucket.as_ptr() as usize);
        wr::<u64>(mp + 0x30, 0);

        wr::<usize>(g_map, mp);
        let res = f(query_crc);
        (res, rd::<u32>(g_owner), rd::<u32>(g_count))
    };

    for crc in [0x1111_2222u32, 0x3333_4444u32, 0x9999_0000u32] {
        let o = run(f_o, crc);
        let n = run(f_n, crc);
        crate::diff_eq!(o, n, "1424b14c0 crc={crc:#x}");
    }

    wr::<u64>(g_lock, s_lock);
    wr::<u32>(g_owner, s_owner);
    wr::<u32>(g_count, s_count);
    wr::<usize>(g_map, s_map);

    Ok(3)
}

pub unsafe fn diff_cityhash_1424b16a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut buf = [0u8; 256];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }
    let base = buf.as_ptr() as usize + 3; // unaligned pointer test

    for len in [
        0u64, 1, 3, 4, 7, 8, 12, 16, 17, 24, 32, 33, 48, 64, 65, 96, 128, 129, 192, 220,
    ] {
        let o = f_o(base, len);
        let n = f_n(base, len);
        crate::diff_eq!(o, n, "1424b16a0 len={len}");
    }
    Ok(20)
}

pub unsafe fn diff_cityhash_1424b1d00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut buf = [0u8; 32];
    for (i, b) in buf.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(53).wrapping_add(7);
    }
    let base = buf.as_ptr() as usize + 1; // unaligned pointer test

    for len in 0..=16u64 {
        let o = f_o(base, len);
        let n = f_n(base, len);
        crate::diff_eq!(o, n, "1424b1d00 len={len}");
    }
    Ok(17)
}

pub unsafe fn diff_unknown_1424b3910(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut POOL_BLK: [u8; 0x40] = [0; 0x40];
    static mut ALLOC_MEM: [u8; 0x200] = [0; 0x200];
    static mut LAST_ALLOC_REQ: u64 = 0;

    unsafe extern "win64" fn mock_9040(_lock: usize, _rdx: u64, _inc: u64) {}
    unsafe extern "win64" fn mock_alloc(_this: usize, sz_ptr: usize) -> usize {
        unsafe {
            LAST_ALLOC_REQ = rd::<u64>(sz_ptr);
            ALLOC_MEM = [0; 0x200];
            core::ptr::addr_of_mut!(ALLOC_MEM) as usize
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4137_9040, mock_9040 as *const ())?;

    let g_ce14 = exe::ptr::<u16>(0x1_43c8_ce14) as usize;
    let s_ce14 = rd::<u16>(g_ce14);

    let run = |f: F, mode: u32| -> (usize, u32, u16, u64, [u64; 7]) {
        wr::<u16>(g_ce14, 100);
        unsafe {
            POOL_BLK = [0; 0x40];
            LAST_ALLOC_REQ = 0;
        }

        let vtbl: [usize; 3] = [0, 0, mock_alloc as *const () as usize];
        let alloc_obj: [usize; 1] = [vtbl.as_ptr() as usize];

        let pbp = core::ptr::addr_of_mut!(POOL_BLK) as usize;
        wr::<u64>(pbp, 0x5000);
        wr::<u64>(pbp + 0x08, 0x5020);
        wr::<u64>(pbp + 0x10, 0x5100);
        wr::<u64>(pbp + 0x18, 0x5020);
        wr::<u64>(pbp + 0x28, 0);

        let mut rdi_buf = [0u8; 0xb0];
        let rdi = rdi_buf.as_mut_ptr() as usize;
        wr::<u64>(rdi, if mode == 1 { pbp as u64 } else { 0 });
        wr::<u64>(rdi + 0x08, 1);
        wr::<usize>(rdi + 0x50, alloc_obj.as_ptr() as usize);
        wr::<u64>(rdi + 0x98, 0x100);
        wr::<u32>(rdi + 0xa0, 5);

        let mut p1_buf = [0u8; 0x30];
        let p1 = p1_buf.as_mut_ptr() as usize;
        wr::<usize>(p1, rdi);
        wr::<usize>(p1 + 0x08, if mode == 0 { pbp } else { 0 });
        wr::<u64>(p1 + 0x10, 0x100);
        wr::<u32>(p1 + 0x24, 10);

        let req_sz = if mode == 2 { 0x120 } else { 0x40 };
        let r = f(p1, req_sz, 16);
        let norm_r = if r == pbp {
            1
        } else if r == core::ptr::addr_of!(ALLOC_MEM) as usize {
            2
        } else {
            r
        };
        let hdr = rd::<[u64; 7]>(r);
        let norm_hdr = [
            hdr[0].wrapping_sub(r as u64),
            hdr[1].wrapping_sub(r as u64),
            hdr[2].wrapping_sub(r as u64),
            hdr[3].wrapping_sub(r as u64),
            hdr[4],
            hdr[5],
            hdr[6],
        ];
        (
            norm_r,
            rd::<u32>(p1 + 0x24),
            rd::<u16>(g_ce14),
            unsafe { LAST_ALLOC_REQ },
            norm_hdr,
        )
    };

    for mode in 0..3 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1424b3910 mode={mode}");
    }

    wr::<u16>(g_ce14, s_ce14);
    Ok(3)
}

// REGISTRY:
// Replacement::verified(0x1_4239_70b0, "unknown_1423970b0", rend::cauldron_hot_t2::unknown_1423970b0 as *const (), rend::cauldron_hot_t2::diff_unknown_1423970b0),
// Replacement::verified_throws(0x1_4240_ce30, "r::Streamer::vfunc_1", rend::cauldron_hot_t2::r_14240ce30 as *const (), Throws::Discard, rend::cauldron_hot_t2::diff_r_14240ce30),
// Replacement::verified(0x1_4247_58f0, "unknown_1424758f0", rend::cauldron_hot_t2::unknown_1424758f0 as *const (), rend::cauldron_hot_t2::diff_unknown_1424758f0),
// Replacement::verified_throws(0x1_4248_b1c0, "unknown_14248b1c0", rend::cauldron_hot_t2::unknown_14248b1c0 as *const (), Throws::Caught, rend::cauldron_hot_t2::diff_unknown_14248b1c0),
// Replacement::verified_throws(0x1_424b_14c0, "unknown_1424b14c0", rend::cauldron_hot_t2::unknown_1424b14c0 as *const (), Throws::Discard, rend::cauldron_hot_t2::diff_unknown_1424b14c0),
// Replacement::verified(0x1_424b_16a0, "cityhash_1424b16a0", rend::cauldron_hot_t2::cityhash_1424b16a0 as *const (), rend::cauldron_hot_t2::diff_cityhash_1424b16a0),
// Replacement::verified(0x1_424b_1d00, "cityhash_1424b1d00", rend::cauldron_hot_t2::cityhash_1424b1d00 as *const (), rend::cauldron_hot_t2::diff_cityhash_1424b1d00),
// Replacement::verified_throws(0x1_424b_3910, "unknown_1424b3910", rend::cauldron_hot_t2::unknown_1424b3910 as *const (), Throws::Discard, rend::cauldron_hot_t2::diff_unknown_1424b3910),
