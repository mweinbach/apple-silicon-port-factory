//! Decompiled hot functions shard Y8a (`0x142535770..0x14253a960`).

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn collect_page_thread_free(rbx: usize) {
    let mut rax = crate::simd_ops::rd_acq_u64(rbx + 0x28);
    if (rax & !3u64) != 0 {
        let rdx_head = loop {
            let rdx_ptr = (rax & !3u64) as usize;
            let rcx_low = rax & 3u64;
            let (ok, actual) =
                crate::simd_ops::atomic_cmpxchg_u64((rbx + 0x28) as u64, rax, rcx_low);
            if ok {
                break rdx_ptr;
            }
            rax = actual;
        };
        if rdx_head != 0 {
            let mut rax_cur = rd::<usize>(rdx_head);
            let mut ecx: u32 = 1;
            let r9d_cap = rd::<u16>(rbx + 0x0a) as u32;
            let mut r8_tail = rdx_head;
            let mut corrupted = false;
            while rax_cur != 0 {
                if ecx > r9d_cap {
                    corrupted = true;
                    break;
                }
                r8_tail = rax_cur;
                ecx = ecx.wrapping_add(1);
                rax_cur = rd::<usize>(rax_cur);
            }
            if corrupted || ecx > r9d_cap {
                let msg = exe::ptr::<u8>(0x1_4342_54d8) as usize;
                exe::call_va!(0x1_4253_86a0, (0xeu32, msg));
            } else {
                wr::<u64>(r8_tail, rd::<u64>(rbx + 0x20));
                wr::<u32>(rbx + 0x18, rd::<u32>(rbx + 0x18).wrapping_sub(ecx));
                wr::<usize>(rbx + 0x20, rdx_head);
            }
        }
    }
}

/// `unknown_142535770`
/// Original: 0x142535770 (530 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142535770(
    param_1: usize,
    param_2: usize,
    param_3: u8,
) -> usize {
    let mut rbx = rd::<usize>(param_2);
    while rbx != 0 {
        let rdi = rd::<usize>(rbx + 0x38);
        collect_page_thread_free(rbx);

        let local_free = rd::<u64>(rbx + 0x20);
        if local_free != 0 {
            if rd::<u64>(rbx + 0x10) != 0 {
                wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & 1u8);
                return rbx;
            }
            wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & !1u8);
            wr::<u64>(rbx + 0x10, local_free);
            wr::<u64>(rbx + 0x20, 0);
        }

        if rd::<u64>(rbx + 0x10) != 0 {
            wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & 1u8);
            return rbx;
        }

        if rd::<u16>(rbx + 0x0a) < rd::<u16>(rbx + 0x0c) {
            exe::call_va!(0x1_4253_5a00, (param_1, rbx));
            wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & 1u8);
            return rbx;
        }

        if (rd::<u8>(rbx + 0x0e) & 1) == 0 {
            let full_q = rd::<usize>(rbx + 0x30).wrapping_add(0xb00);
            exe::call_va!(0x1_4253_68c0, (full_q, param_2, rbx));
            collect_page_thread_free(rbx);
            let local_free2 = rd::<u64>(rbx + 0x20);
            if local_free2 != 0 && rd::<u64>(rbx + 0x10) == 0 {
                wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & !1u8);
                wr::<u64>(rbx + 0x10, local_free2);
                wr::<u64>(rbx + 0x20, 0);
            }
        }

        rbx = rdi;
    }

    exe::call_va!(0x1_4253_5f70, (param_1, 0u8));
    let bsize = rd::<u64>(param_2 + 0x10);
    let page = exe::call_va!(0x1_4253_64e0, (param_1, param_2, bsize, 0u64) -> usize);
    if page != 0 {
        return page;
    }
    if param_3 != 0 {
        return exe::call_va!(0x1_4253_5770, (param_1, param_2, 0u8) -> usize);
    }
    0
}

/// `unknown_142535ac0`
/// Original: 0x142535ac0 (197 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142535ac0(param_1: usize, param_2: u64, param_3: u64) {
    let rcx_seg = (param_1 as u64).wrapping_sub(1) & !0x01ff_ffffu64;
    let r10 = rd::<u32>(param_1 + 0x1c) as u64;
    let diff = (param_1 as u64)
        .wrapping_sub(rcx_seg)
        .wrapping_sub(0x108) as i64;
    let r9_idx = (diff / 0x50) as u64;
    let rax_pad = if r10 < 8 {
        0
    } else if r10 <= 0x40 {
        r10.wrapping_mul(3)
    } else if r10 <= 0x200 {
        r10
    } else {
        0
    };
    let r9_base = r9_idx
        .wrapping_shl(16)
        .wrapping_add(rax_pad)
        .wrapping_add(rcx_seg);
    let cap = rd::<u16>(param_1 + 0x0a) as u64;
    let r8_start = cap.wrapping_mul(param_2).wrapping_add(r9_base);
    let rdx_last = cap
        .wrapping_sub(1)
        .wrapping_add(param_3)
        .wrapping_mul(param_2)
        .wrapping_add(r9_base);
    let mut cur = r8_start;
    if r8_start <= rdx_last {
        loop {
            let next = cur.wrapping_add(param_2);
            wr::<u64>(cur as usize, next);
            cur = next;
            if cur > rdx_last {
                break;
            }
        }
    }
    wr::<u64>(rdx_last as usize, rd::<u64>(param_1 + 0x10));
    wr::<u64>(param_1 + 0x10, r8_start);
}

/// `unknown_142535f70`
/// Original: 0x142535f70 (240 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142535f70(param_1: usize, param_2: u8) {
    let mut rbx = rd::<u64>(param_1 + 0xbd8);
    let mut rbp: u64 = 0;
    let mut rdi: u64 = 0x4a;
    if rbx > rd::<u64>(param_1 + 0xbe0) {
        wr::<u64>(param_1 + 0xbd8, rdi);
        wr::<u64>(param_1 + 0xbe0, rbp);
        return;
    }

    loop {
        let r8 = param_1.wrapping_add((rbx as usize).wrapping_mul(0x18));
        let rcx_page = rd::<usize>(r8 + 0x410);
        if rcx_page != 0 {
            let al = rd::<u8>(rcx_page + 0x0f);
            if al >= 2 {
                let dl = al & 1;
                if rd::<u32>(rcx_page + 0x18) == 0 {
                    let new_al = (((al >> 1).wrapping_sub(1)) << 1) | dl;
                    wr::<u8>(rcx_page + 0x0f, new_al);
                    if param_2 == 0 && new_al >= 2 {
                        if rbx < rdi {
                            rdi = rbx;
                        }
                        if rbx > rbp {
                            rbp = rbx;
                        }
                    } else {
                        let page_arg = rd::<usize>(r8 + 0x410);
                        exe::call_va!(0x1_4253_6070, (page_arg, r8.wrapping_add(0x410)));
                    }
                } else {
                    wr::<u8>(rcx_page + 0x0f, dl);
                }
            }
        }
        rbx = rbx.wrapping_add(1);
        if rbx > rd::<u64>(param_1 + 0xbe0) {
            break;
        }
    }

    wr::<u64>(param_1 + 0xbd8, rdi);
    wr::<u64>(param_1 + 0xbe0, rbp);
}

/// `unknown_142536070`
/// Original: 0x142536070 (218 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142536070(param_1: usize, param_2: usize) {
    wr::<u8>(param_1 + 0x0e, rd::<u8>(param_1 + 0x0e) & !2u8);
    let r14 = rd::<usize>(param_1 + 0x30);
    let r8_prev = rd::<usize>(param_1 + 0x40);
    let r12 = rd::<usize>(r14);
    if r8_prev != 0 {
        wr::<u64>(r8_prev + 0x38, rd::<u64>(param_1 + 0x38));
    }
    let r11_next = rd::<usize>(param_1 + 0x38);
    if r11_next != 0 {
        wr::<u64>(r11_next + 0x40, rd::<u64>(param_1 + 0x40));
    }
    if param_1 == rd::<usize>(param_2 + 0x08) {
        wr::<u64>(param_2 + 0x08, rd::<u64>(param_1 + 0x40));
    }
    if param_1 == rd::<usize>(param_2) {
        wr::<u64>(param_2, rd::<u64>(param_1 + 0x38));
        exe::call_va!(0x1_4253_6aa0, (r14, param_2));
    }
    wr::<u64>(r14 + 0xbd0, rd::<u64>(r14 + 0xbd0).wrapping_sub(1));
    let r8_tld = r12.wrapping_add(0x20);
    wr::<u8>(param_1 + 0x0e, rd::<u8>(param_1 + 0x0e) & !1u8);
    wr::<u64>(param_1 + 0x38, 0);
    wr::<u64>(param_1 + 0x40, 0);
    wr::<u64>(param_1 + 0x30, 0);
    exe::call_va!(0x1_4253_e5c0, (param_1, param_2, r8_tld));
}

/// `unknown_142536370`
/// Original: 0x142536370 (156 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142536370(param_1: usize) -> u8 {
    let mut rbx = crate::simd_ops::rd_acq_usize(param_1 + 0xb18);
    loop {
        if rbx == 0 {
            return 1;
        }
        let (ok, actual) =
            crate::simd_ops::atomic_cmpxchg_u64((param_1 + 0xb18) as u64, rbx as u64, 0);
        if ok {
            break;
        }
        rbx = actual as usize;
    }

    let mut bpl: u8 = 1;
    loop {
        let rsi = rd::<usize>(rbx);
        let ok = exe::call_va!(0x1_4253_3040, (rbx) -> u8);
        if ok == 0 {
            let mut rax = crate::simd_ops::rd_acq_u64(param_1 + 0xb18);
            bpl = 0;
            loop {
                wr::<u64>(rbx, rax);
                let (cas_ok, actual) =
                    crate::simd_ops::atomic_cmpxchg_u64((param_1 + 0xb18) as u64, rax, rbx as u64);
                if cas_ok {
                    break;
                }
                rax = actual;
            }
        }
        rbx = rsi;
        if rbx == 0 {
            break;
        }
    }
    bpl
}

/// `unknown_1425364e0`
/// Original: 0x1425364e0 (343 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1425364e0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: u64,
) -> usize {
    let mut rbx = param_3;
    let r9_tld = rd::<usize>(param_1).wrapping_add(0x20);
    let rdi = if param_4 <= 0x0100_0000 {
        if rbx <= 0x4000 {
            exe::call_va!(0x1_4253_a960, (param_1, param_2 as u32, rbx, rbx, r9_tld) -> usize)
        } else if rbx <= 0x20000 {
            exe::call_va!(
                0x1_4253_a960,
                (param_1, param_2 as u32, 0x80000u64, rbx, r9_tld) -> usize
            )
        } else if rbx <= 0x0100_0000 {
            exe::call_va!(0x1_4253_a960, (param_1, param_2 as u32, rbx, rbx, r9_tld) -> usize)
        } else {
            let r8d = rd::<u32>(param_1 + 0xb28);
            exe::call_va!(0x1_4253_a840, (rbx, param_4, r8d, r9_tld) -> usize)
        }
    } else {
        let rdx_align = param_4.max(0x0200_0000);
        let r8d = rd::<u32>(param_1 + 0xb28);
        exe::call_va!(0x1_4253_a840, (rbx, rdx_align, r8d, r9_tld) -> usize)
    };

    if rdi != 0 {
        if param_2 == 0 || rd::<u64>(param_2 + 0x10) == 0x20008 {
            rbx = rd::<u32>(rdi + 0x1c) as u64;
            if rbx >= 0x8000_0000 {
                rbx = (rd::<u32>(rdi) as u64) << 16;
            }
        }
        let mut r8 = (rd::<u32>(rdi) as u64) << 16;
        wr::<usize>(rdi + 0x30, param_1);
        let rcx: u64 = if rbx < 0x8000_0000 {
            rbx as u32 as u64
        } else {
            0x8000_0000
        };
        wr::<u32>(rdi + 0x1c, rcx as u32);
        let rax_pad = if rcx < 8 {
            0
        } else if rcx <= 0x40 {
            rcx.wrapping_mul(3)
        } else if rcx <= 0x200 {
            rcx
        } else {
            0
        };
        let b08 = rd::<u8>(rdi + 0x08);
        r8 = r8.wrapping_sub(rax_pad);
        let b0f = rd::<u8>(rdi + 0x0f);
        let cl = ((b08 >> 1) ^ b0f) & 1;
        let quot = (r8 / rbx) as u16;
        wr::<u8>(rdi + 0x0f, b0f ^ cl);
        wr::<u16>(rdi + 0x0c, quot);
        exe::call_va!(0x1_4253_5a00, (cl as u32, rdi));
        if param_2 != 0 {
            exe::call_va!(0x1_4253_69a0, (param_1, param_2, rdi));
        }
    }
    rdi
}

/// `unknown_1425369a0`
/// Original: 0x1425369a0 (89 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1425369a0(param_1: usize, param_2: usize, param_3: usize) {
    let is_full = u8::from(rd::<u64>(param_2 + 0x10) == 0x20010);
    let b0e = (rd::<u8>(param_3 + 0x0e) & !1u8) | is_full;
    wr::<u8>(param_3 + 0x0e, b0e);
    let old_head = rd::<usize>(param_2);
    wr::<usize>(param_3 + 0x38, old_head);
    wr::<u64>(param_3 + 0x40, 0);
    let old_head2 = rd::<usize>(param_2);
    if old_head2 != 0 {
        wr::<usize>(old_head2 + 0x40, param_3);
    } else {
        wr::<usize>(param_2 + 0x08, param_3);
    }
    wr::<usize>(param_2, param_3);
    exe::call_va!(0x1_4253_6aa0, (param_1, param_2));
    wr::<u64>(param_1 + 0xbd0, rd::<u64>(param_1 + 0xbd0).wrapping_add(1));
}

#[inline(always)]
fn wsize_bin(w: u64) -> u8 {
    if w <= 1 {
        1
    } else if w <= 8 {
        ((w as u8).wrapping_add(1)) & !1u8
    } else if w > 0x4000 {
        0x49
    } else {
        let w_m1 = w - 1;
        let dl: u8 = if w_m1 == 0 {
            0x40
        } else {
            63 - (w_m1.leading_zeros() as u8)
        };
        let shift = (dl as u32).wrapping_sub(2) & 63;
        let low2 = ((w_m1 >> shift) as u8) & 3;
        dl.wrapping_shl(2).wrapping_add(low2).wrapping_sub(3)
    }
}

/// `unknown_142536aa0`
/// Original: 0x142536aa0 (375 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142536aa0(param_1: usize, param_2: usize) {
    let bsize = rd::<u64>(param_2 + 0x10);
    if bsize > 0x400 {
        return;
    }
    let page = rd::<usize>(param_2);
    let rsi = if page != 0 {
        page
    } else {
        exe::ptr::<u8>(0x1_4307_4980) as usize
    };
    let r14 = param_1.wrapping_add(0x08);
    let r10 = bsize.wrapping_add(7) >> 3;
    if rd::<usize>(r14.wrapping_add((r10 as usize) * 8)) == rsi {
        return;
    }

    let start_idx: u64 = if r10 <= 1 {
        0
    } else {
        let r9b = wsize_bin(r10);
        let mut r8_prev_pq = param_2.wrapping_sub(0x18);
        let rbx_prev_w = loop {
            let prev_w = rd::<u64>(r8_prev_pq + 0x10).wrapping_add(7) >> 3;
            let al = wsize_bin(prev_w);
            if r9b != al || r8_prev_pq <= param_1.wrapping_add(0x410) {
                break prev_w;
            }
            r8_prev_pq = r8_prev_pq.wrapping_sub(0x18);
        };
        let cand = rbx_prev_w.wrapping_add(1);
        if cand <= r10 {
            cand
        } else {
            r10
        }
    };

    if start_idx <= r10 {
        let mut idx = start_idx;
        while idx <= r10 {
            wr::<usize>(r14.wrapping_add((idx as usize) * 8), rsi);
            idx = idx.wrapping_add(1);
        }
    }
}

/// `unknown_14253a960`
/// Original: 0x14253a960 (251 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14253a960(
    param_1: usize,
    param_2: u32,
    param_3: u64,
    param_4: u64,
    param_5: usize,
) -> usize {
    let mut local_flag: u32 = param_2;
    let r13: u64 = if param_3 > 0x80000 { 0x80000 } else { 0x10000 };
    let r12: u64 = r13 - 1;
    let rdi: u64 = r12.wrapping_add(param_3);
    let r15: u64 = r12 & r13;

    loop {
        let aligned = if r15 == 0 {
            !r12 & rdi
        } else {
            rdi.wrapping_sub(rdi % r13)
        };
        let rbx_slices = aligned >> 16;
        let rsi = exe::call_va!(
            0x1_4253_bf60,
            (rbx_slices, rd::<u32>(param_1 + 0xb28), param_5) -> usize
        );
        if rsi != 0 {
            let seg = rsi.wrapping_sub(1) & !0x01ff_ffffusize;
            exe::call_va!(0x1_4253_c770, (seg, 0u8, rd::<u64>(param_5 + 0x380)));
            return rsi;
        }
        let rax_ab = exe::call_va!(
            0x1_4253_abc0,
            (
                param_1,
                rbx_slices,
                param_4,
                (&mut local_flag as *mut u32) as usize,
                param_5
            ) -> usize
        );
        if (local_flag as u8) != 0 {
            return 0;
        }
        if rax_ab != 0 {
            continue;
        }
        let rax_b9 = exe::call_va!(
            0x1_4253_b9c0,
            (
                0u64,
                0u64,
                rd::<u32>(param_1 + 0xb28),
                param_5,
                param_5,
                0usize
            ) -> usize
        );
        if rax_b9 == 0 {
            return 0;
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142535770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_86a0(code: u32, _msg: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(code as u64);
    }
    unsafe extern "win64" fn mock_5a00(_rcx: usize, page: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0x5a00);
        wr::<u64>(page + 0x10, 0x7777);
    }
    unsafe extern "win64" fn mock_68c0(full_q: usize, _pq: usize, page: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(full_q as u64 & 0xfff);
        wr::<u8>(page + 0x0e, rd::<u8>(page + 0x0e) | 1);
    }
    unsafe extern "win64" fn mock_5f70(_heap: usize, flag: u8) {
        LOG_SUM = LOG_SUM.wrapping_add(0x5f70 + flag as u64);
    }
    unsafe extern "win64" fn mock_64e0(
        _heap: usize,
        _pq: usize,
        bsize: u64,
        align: u64,
    ) -> usize {
        LOG_SUM = LOG_SUM.wrapping_add(bsize).wrapping_add(align);
        0x64e0_0000
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_86a0, mock_86a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_5a00, mock_5a00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_68c0, mock_68c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4253_5f70, mock_5f70 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4253_64e0, mock_64e0 as *const ())?;

    let run = |f: F, mode: u8| -> (usize, u32, u8, u64, u64) {
        LOG_SUM = 0;
        let mut page1 = [0u8; 0x48];
        let mut page2 = [0u8; 0x48];
        let p1 = page1.as_mut_ptr() as usize;
        let p2 = page2.as_mut_ptr() as usize;
        let mut tf_node: u64 = 0;
        let tf_ptr = (&mut tf_node as *mut u64) as usize;

        let mut pq: [u64; 3] = [0, 0, 64];
        let pq_ptr = pq.as_mut_ptr() as usize;

        match mode {
            0 => {
                // Empty queue -> calls 5f70 and 64e0
            }
            1 => {
                // Page 1 has thread-free node, moves into free list (+0x10)
                wr::<u16>(p1 + 0x0a, 4);
                wr::<u16>(p1 + 0x0c, 4);
                wr::<u8>(p1 + 0x0f, 3);
                wr::<u32>(p1 + 0x18, 2);
                wr::<u64>(p1 + 0x20, 0xaaaa);
                wr::<u64>(p1 + 0x28, (tf_ptr as u64) | 2);
                wr::<usize>(pq_ptr, p1);
            }
            2 => {
                // Page 1 has capacity < reserved -> calls 5a00
                wr::<u16>(p1 + 0x0a, 2);
                wr::<u16>(p1 + 0x0c, 8);
                wr::<u8>(p1 + 0x0f, 3);
                wr::<usize>(pq_ptr, p1);
            }
            _ => {
                // Page 1 is full -> moved via 68c0, advances to Page 2 which has free block
                wr::<u16>(p1 + 0x0a, 4);
                wr::<u16>(p1 + 0x0c, 4);
                wr::<u8>(p1 + 0x0e, 0);
                wr::<usize>(p1 + 0x30, 0x10_0000);
                wr::<usize>(p1 + 0x38, p2);

                wr::<u8>(p2 + 0x0f, 3);
                wr::<u64>(p2 + 0x10, 0xbbbb);
                wr::<usize>(pq_ptr, p1);
            }
        }

        let ret = f(0x10_0000, pq_ptr, 0);
        let norm_ret = if ret == p1 {
            1
        } else if ret == p2 {
            2
        } else {
            ret
        };
        let norm_p1_10 = if rd::<usize>(p1 + 0x10) == tf_ptr {
            0x1111
        } else {
            rd::<u64>(p1 + 0x10)
        };
        (
            norm_ret,
            rd::<u32>(p1 + 0x18),
            rd::<u8>(p1 + 0x0f),
            norm_p1_10,
            LOG_SUM,
        )
    };

    for mode in 0..4u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "142535770 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_142535ac0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, bsize_field: u32, stride: u64, cap: u16, extend: u64| -> (u64, [u64; 4]) {
        core::ptr::write_bytes((seg + 0x100) as *mut u8, 0, 0x200);
        core::ptr::write_bytes((seg + 0x1_0000) as *mut u8, 0, 0x2000);
        let page = seg + 0x108 + 0x50;
        wr::<u16>(page + 0x0a, cap);
        wr::<u64>(page + 0x10, 0xdead_beef);
        wr::<u32>(page + 0x1c, bsize_field);

        f(page, stride, extend);

        let head = rd::<u64>(page + 0x10);
        let mut chain = [0u64; 4];
        let mut cur = head as usize;
        for slot in &mut chain {
            if cur >= seg + 0x1_0000 && cur < seg + 0x1_2000 {
                let next = rd::<u64>(cur);
                *slot = (cur - seg) as u64;
                cur = next as usize;
            } else {
                *slot = cur as u64;
                break;
            }
        }
        (head.wrapping_sub(seg as u64), chain)
    };

    for (bf, st, cap, ext) in [
        (4u32, 16u64, 2u16, 3u64),
        (16, 16, 1, 2),
        (128, 128, 0, 3),
        (1024, 1024, 1, 2),
    ] {
        let o = run(f_o, bf, st, cap, ext);
        let n = run(f_n, bf, st, cap, ext);
        crate::diff_eq!(o, n, "142535ac0 bf={} cap={} ext={}", bf, cap, ext);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_142535f70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RETIRED_SUM: u64 = 0;
    static mut CUR_HP: usize = 0;

    unsafe extern "win64" fn mock_6070(page: usize, pq: usize) {
        RETIRED_SUM = RETIRED_SUM
            .wrapping_add(rd::<u32>(page + 0x1c) as u64)
            .wrapping_add(pq.wrapping_sub(CUR_HP) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_6070, mock_6070 as *const ())?;

    let run = |f: F, empty_range: bool, force: u8| -> (u64, u64, [u8; 3], u64) {
        RETIRED_SUM = 0;
        let mut heap = vec![0u8; 0xc00];
        let hp = heap.as_mut_ptr() as usize;
        CUR_HP = hp;

        let mut pages = [[0u8; 0x30]; 3];
        let p0 = pages[0].as_mut_ptr() as usize;
        let p1 = pages[1].as_mut_ptr() as usize;
        let p2 = pages[2].as_mut_ptr() as usize;

        // p0: used != 0 -> clears upper bits of +0x0f
        wr::<u8>(p0 + 0x0f, 0x07);
        wr::<u32>(p0 + 0x18, 2);
        wr::<u32>(p0 + 0x1c, 10);

        // p1: used == 0, b0f = 0x06 -> decrements to 0x04 (>= 2)
        wr::<u8>(p1 + 0x0f, 0x06);
        wr::<u32>(p1 + 0x18, 0);
        wr::<u32>(p1 + 0x1c, 20);

        // p2: used == 0, b0f = 0x03 -> decrements to 0x01 (< 2 -> calls 6070)
        wr::<u8>(p2 + 0x0f, 0x03);
        wr::<u32>(p2 + 0x18, 0);
        wr::<u32>(p2 + 0x1c, 30);

        wr::<usize>(hp + 0x410 + 2 * 0x18, p0);
        wr::<usize>(hp + 0x410 + 3 * 0x18, p1);
        wr::<usize>(hp + 0x410 + 4 * 0x18, p2);

        if empty_range {
            wr::<u64>(hp + 0xbd8, 10);
            wr::<u64>(hp + 0xbe0, 5);
        } else {
            wr::<u64>(hp + 0xbd8, 2);
            wr::<u64>(hp + 0xbe0, 4);
        }

        f(hp, force);
        (
            rd::<u64>(hp + 0xbd8),
            rd::<u64>(hp + 0xbe0),
            [rd::<u8>(p0 + 0x0f), rd::<u8>(p1 + 0x0f), rd::<u8>(p2 + 0x0f)],
            RETIRED_SUM,
        )
    };

    for (er, force) in [(true, 0u8), (false, 0), (false, 1)] {
        let o = run(f_o, er, force);
        let n = run(f_n, er, force);
        crate::diff_eq!(o, n, "142535f70 er={} force={}", er, force);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142536070(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_6AA0: u32 = 0;
    static mut LOG_E5C0: usize = 0;

    unsafe extern "win64" fn mock_6aa0(_heap: usize, _pq: usize) {
        LOG_6AA0 += 1;
    }
    unsafe extern "win64" fn mock_e5c0(_page: usize, _rdx: usize, r8_tld: usize) {
        LOG_E5C0 = r8_tld;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_6aa0, mock_6aa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_e5c0, mock_e5c0 as *const ())?;

    let run = |f: F, is_head_tail: bool| -> (u8, u64, usize, usize, u32, usize) {
        LOG_6AA0 = 0;
        LOG_E5C0 = 0;

        let mut heap = vec![0u8; 0xc00];
        let hp = heap.as_mut_ptr() as usize;
        wr::<usize>(hp, 0x5000);
        wr::<u64>(hp + 0xbd0, 7);

        let mut page = [0u8; 0x48];
        let mut prev_page = [0u8; 0x48];
        let mut next_page = [0u8; 0x48];
        let pp = page.as_mut_ptr() as usize;
        let prev_p = prev_page.as_mut_ptr() as usize;
        let next_p = next_page.as_mut_ptr() as usize;

        wr::<u8>(pp + 0x0e, 0x07);
        wr::<usize>(pp + 0x30, hp);
        wr::<usize>(pp + 0x38, next_p);
        wr::<usize>(pp + 0x40, prev_p);

        let mut pq: [usize; 2] = if is_head_tail {
            [pp, pp]
        } else {
            [prev_p, next_p]
        };

        f(pp, pq.as_mut_ptr() as usize);

        let head_changed = usize::from(pq[0] == next_p);
        let tail_changed = usize::from(pq[1] == prev_p);
        let links_ok = usize::from(
            rd::<usize>(prev_p + 0x38) == next_p
                && rd::<usize>(next_p + 0x40) == prev_p
                && rd::<usize>(pp + 0x30) == 0
                && rd::<usize>(pp + 0x38) == 0
                && rd::<usize>(pp + 0x40) == 0,
        );
        (
            rd::<u8>(pp + 0x0e),
            rd::<u64>(hp + 0xbd0),
            head_changed + tail_changed * 2,
            links_ok,
            LOG_6AA0,
            LOG_E5C0,
        )
    };

    for ht in [true, false] {
        let o = run(f_o, ht);
        let n = run(f_n, ht);
        crate::diff_eq!(o, n, "142536070 ht={}", ht);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142536370(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_3040(node: usize) -> u8 {
        rd::<u8>(node + 0x08)
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_3040, mock_3040 as *const ())?;

    let run = |f: F, mode: u8| -> (u8, usize) {
        let mut heap = vec![0u8; 0xb40];
        let hp = heap.as_mut_ptr() as usize;

        let mut n1: [u64; 2] = [0, 1];
        let mut n2: [u64; 2] = [0, 1];
        let p1 = n1.as_mut_ptr() as usize;
        let p2 = n2.as_mut_ptr() as usize;

        match mode {
            0 => {
                // Empty list
            }
            1 => {
                // Both succeed
                wr::<usize>(p1, p2);
                wr::<usize>(hp + 0xb18, p1);
            }
            _ => {
                // n2 fails -> pushed back to +0xb18
                wr::<u64>(p2 + 8, 0);
                wr::<usize>(p1, p2);
                wr::<usize>(hp + 0xb18, p1);
            }
        }

        let ret = f(hp);
        let head = rd::<usize>(hp + 0xb18);
        let norm_head = if head == p2 { 2 } else { head };
        (ret, norm_head)
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "142536370 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1425364e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut PAGE_MEM: [u8; 0x48] = [0; 0x48];
    static mut CALL_LOG: (u64, u64, usize, u32) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_a960(
        _heap: usize,
        _pq_low: u32,
        req_size: u64,
        bsize: u64,
        tld: usize,
    ) -> usize {
        CALL_LOG = (req_size, bsize, tld, 1);
        core::ptr::addr_of_mut!(PAGE_MEM) as usize
    }
    unsafe extern "win64" fn mock_a840(bsize: u64, align: u64, numa: u32, tld: usize) -> usize {
        CALL_LOG = (bsize, align, tld.wrapping_add(numa as usize), 2);
        core::ptr::addr_of_mut!(PAGE_MEM) as usize
    }
    unsafe extern "win64" fn mock_5a00(cl: u32, page: usize) {
        wr::<u8>(page + 0x09, cl as u8);
    }
    unsafe extern "win64" fn mock_69a0(_heap: usize, _pq: usize, page: usize) {
        wr::<u8>(page + 0x0a, 0x55);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_a960, mock_a960 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_a840, mock_a840 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_5a00, mock_5a00 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4253_69a0, mock_69a0 as *const ())?;

    let run = |f: F, bsize: u64, align: u64, has_pq: bool| -> (u32, u16, u8, u8, u8, (u64, u64, usize, u32)) {
        PAGE_MEM = [0; 0x48];
        CALL_LOG = (0, 0, 0, 0);
        let pp = core::ptr::addr_of_mut!(PAGE_MEM) as usize;
        wr::<u32>(pp, 2); // 2 slices = 0x20000 bytes
        wr::<u8>(pp + 0x08, 2); // bit 1 set
        wr::<u8>(pp + 0x0f, 0);
        wr::<u32>(pp + 0x1c, bsize as u32);

        let mut heap = vec![0u8; 0xb40];
        let hp = heap.as_mut_ptr() as usize;
        wr::<usize>(hp, 0x9000);
        wr::<u32>(hp + 0xb28, 7);

        let pq: [u64; 3] = [0, 0, bsize];
        let pq_ptr = if has_pq { pq.as_ptr() as usize } else { 0 };

        let ret = f(hp, pq_ptr, bsize, align);
        assert_eq!(ret, pp);
        (
            rd::<u32>(pp + 0x1c),
            rd::<u16>(pp + 0x0c),
            rd::<u8>(pp + 0x0f),
            rd::<u8>(pp + 0x09),
            rd::<u8>(pp + 0x0a),
            CALL_LOG,
        )
    };

    for (bs, al, hpq) in [
        (32u64, 0u64, true),
        (256, 0, true),
        (0x8000, 0, true),
        (0x40000, 0, false),
        (0x1000, 0x0200_0000, true),
    ] {
        let o = run(f_o, bs, al, hpq);
        let n = run(f_n, bs, al, hpq);
        crate::diff_eq!(o, n, "1425364e0 bs={:#x} al={:#x} hpq={}", bs, al, hpq);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1425369a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_6AA0: u32 = 0;

    unsafe extern "win64" fn mock_6aa0(_heap: usize, _pq: usize) {
        LOG_6AA0 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_6aa0, mock_6aa0 as *const ())?;

    let run = |f: F, empty_q: bool, pq_bsize: u64| -> (u8, usize, usize, u64, u32) {
        LOG_6AA0 = 0;
        let mut heap = vec![0u8; 0xc00];
        let hp = heap.as_mut_ptr() as usize;
        wr::<u64>(hp + 0xbd0, 3);

        let mut old_page = [0u8; 0x48];
        let mut new_page = [0u8; 0x48];
        let old_p = old_page.as_mut_ptr() as usize;
        let new_p = new_page.as_mut_ptr() as usize;
        wr::<u8>(new_p + 0x0e, 0x04);

        let mut pq: [u64; 3] = if empty_q {
            [0, 0, pq_bsize]
        } else {
            [old_p as u64, old_p as u64, pq_bsize]
        };

        f(hp, pq.as_mut_ptr() as usize, new_p);

        let links = usize::from(
            pq[0] == new_p as u64
                && pq[1] == (if empty_q { new_p } else { old_p }) as u64
                && rd::<usize>(new_p + 0x38) == (if empty_q { 0 } else { old_p })
                && rd::<usize>(new_p + 0x40) == 0,
        );
        let old_prev_ok = usize::from(empty_q || rd::<usize>(old_p + 0x40) == new_p);
        (
            rd::<u8>(new_p + 0x0e),
            links,
            old_prev_ok,
            rd::<u64>(hp + 0xbd0),
            LOG_6AA0,
        )
    };

    for (eq, bs) in [(true, 64u64), (false, 0x20010)] {
        let o = run(f_o, eq, bs);
        let n = run(f_n, eq, bs);
        crate::diff_eq!(o, n, "1425369a0 eq={} bs={:#x}", eq, bs);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142536aa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, q_idx: usize, bsize: u64, page_val: usize| -> Vec<usize> {
        let mut heap = vec![0u8; 0x900];
        let hp = heap.as_mut_ptr() as usize;

        // Initialize queues at hp + 0x410 + i * 0x18
        let q_sizes: [u64; 6] = [8, 16, 32, 48, 80, 96];
        for (i, &sz) in q_sizes.iter().enumerate() {
            let q = hp + 0x410 + i * 0x18;
            wr::<u64>(q + 0x10, sz);
        }
        let target_q = hp + 0x410 + q_idx * 0x18;
        wr::<usize>(target_q, page_val);
        wr::<u64>(target_q + 0x10, bsize);

        f(hp, target_q);

        let mut direct = Vec::with_capacity(16);
        for i in 0..16 {
            direct.push(rd::<usize>(hp + 0x08 + i * 8));
        }
        direct
    };

    for (qi, bs, pv) in [
        (0usize, 8u64, 0x1111usize),
        (0, 8, 0),
        (2, 32, 0x2222),
        (5, 96, 0x3333),
        (5, 0x800, 0x4444),
    ] {
        let o = run(f_o, qi, bs, pv);
        let n = run(f_n, qi, bs, pv);
        crate::diff_eq!(o, n, "142536aa0 qi={} bs={} pv={:#x}", qi, bs, pv);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_14253a960(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u64, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRY_STEP: u32 = 0;
    static mut MODE: u8 = 0;
    static mut LOG_C770: (usize, u8, u64) = (0, 0, 0);

    unsafe extern "win64" fn mock_bf60(slices: u64, numa: u32, _tld: usize) -> usize {
        TRY_STEP += 1;
        if MODE == 0 || (MODE == 1 && TRY_STEP >= 2) {
            0x0400_1000usize.wrapping_add((slices as usize) * 0x100 + numa as usize)
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_abc0(
        _heap: usize,
        _slices: u64,
        _bsize: u64,
        out_flag: usize,
        _tld: usize,
    ) -> usize {
        if MODE == 2 {
            wr::<u8>(out_flag, 1);
            0
        } else if MODE == 1 {
            wr::<u8>(out_flag, 0);
            1
        } else {
            wr::<u8>(out_flag, 0);
            0
        }
    }
    unsafe extern "win64" fn mock_b9c0(
        _a1: u64,
        _a2: u64,
        _numa: u32,
        _tld1: usize,
        _tld2: usize,
        _a6: usize,
    ) -> usize {
        0
    }
    unsafe extern "win64" fn mock_c770(seg: usize, force: u8, os_tld: u64) {
        LOG_C770 = (seg, force, os_tld);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_bf60, mock_bf60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_abc0, mock_abc0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_b9c0, mock_b9c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4253_c770, mock_c770 as *const ())?;

    let run = |f: F, mode: u8, req_size: u64| -> (usize, (usize, u8, u64), u32) {
        TRY_STEP = 0;
        MODE = mode;
        LOG_C770 = (0, 0, 0);

        let mut heap = vec![0u8; 0xb40];
        let hp = heap.as_mut_ptr() as usize;
        wr::<u32>(hp + 0xb28, 3);

        let mut tld = vec![0u8; 0x390];
        let tp = tld.as_mut_ptr() as usize;
        wr::<u64>(tp + 0x380, 0x7788_99aa);

        let ret = f(hp, 0, req_size, 64, tp);
        (ret, LOG_C770, TRY_STEP)
    };

    for (mode, req) in [(0u8, 0x4000u64), (0, 0x10_0000), (1, 0x8000), (2, 0x8000), (3, 0x8000)] {
        let o = run(f_o, mode, req);
        let n = run(f_n, mode, req);
        crate::diff_eq!(o, n, "14253a960 mode={} req={:#x}", mode, req);
    }
    Ok(5)
}

// REGISTRY:
// Replacement::verified(0x1_4253_5770, "unknown_142535770", rend::cauldron_hot_y8a::unknown_142535770 as *const (), rend::cauldron_hot_y8a::diff_unknown_142535770),
// Replacement::verified(0x1_4253_5ac0, "unknown_142535ac0", rend::cauldron_hot_y8a::unknown_142535ac0 as *const (), rend::cauldron_hot_y8a::diff_unknown_142535ac0),
// Replacement::verified(0x1_4253_5f70, "unknown_142535f70", rend::cauldron_hot_y8a::unknown_142535f70 as *const (), rend::cauldron_hot_y8a::diff_unknown_142535f70),
// Replacement::verified(0x1_4253_6070, "unknown_142536070", rend::cauldron_hot_y8a::unknown_142536070 as *const (), rend::cauldron_hot_y8a::diff_unknown_142536070),
// Replacement::verified(0x1_4253_6370, "unknown_142536370", rend::cauldron_hot_y8a::unknown_142536370 as *const (), rend::cauldron_hot_y8a::diff_unknown_142536370),
// Replacement::verified(0x1_4253_64e0, "unknown_1425364e0", rend::cauldron_hot_y8a::unknown_1425364e0 as *const (), rend::cauldron_hot_y8a::diff_unknown_1425364e0),
// Replacement::verified(0x1_4253_69a0, "unknown_1425369a0", rend::cauldron_hot_y8a::unknown_1425369a0 as *const (), rend::cauldron_hot_y8a::diff_unknown_1425369a0),
// Replacement::verified(0x1_4253_6aa0, "unknown_142536aa0", rend::cauldron_hot_y8a::unknown_142536aa0 as *const (), rend::cauldron_hot_y8a::diff_unknown_142536aa0),
// Replacement::verified(0x1_4253_a960, "unknown_14253a960", rend::cauldron_hot_y8a::unknown_14253a960 as *const (), rend::cauldron_hot_y8a::diff_unknown_14253a960),
