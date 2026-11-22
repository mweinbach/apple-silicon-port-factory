//! Decompiled hot functions shard Y7 (`0x142443a50..0x142537280`).

use crate::exe::{self, rd, wr};

/// `unknown_142443a50`
/// Original: 0x142443a50 (135 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142443a50(param_1: usize) {
    if param_1 != 0 {
        let r10 = param_1.wrapping_sub(1) & !0x01ff_ffffusize;
        let page_idx = param_1.wrapping_sub(r10) >> 16;
        let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
        let rdx = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        if exe::teb() != rd::<usize>(r10 + 0x100) {
            exe::call_va!(0x1_4253_3200, (r10, rdx, 0u8, param_1));
            return;
        }
        if rd::<u8>(rdx + 0x0e) != 0 {
            exe::call_va!(0x1_4253_3200, (r10, rdx, 1u8, param_1));
            return;
        }
        wr::<u64>(param_1, rd::<u64>(rdx + 0x20));
        let new_cnt = rd::<u32>(rdx + 0x18).wrapping_sub(1);
        wr::<u32>(rdx + 0x18, new_cnt);
        wr::<usize>(rdx + 0x20, param_1);
        if new_cnt == 0 {
            exe::call_va!(0x1_4253_71a0, (rdx));
        }
    }
}

/// `r_1424788c0`
/// Original: 0x1424788c0 (904 bytes, Throws::No)
pub unsafe extern "win64" fn r_1424788c0(param_1: usize, param_2: usize) {
    let end_entries = param_2.wrapping_add(0x90);
    if param_2 == end_entries {
        return;
    }

    let mut local_39: usize = 0;
    let mut local_19: [u64; 2];
    let mut local_09: [u64; 4];
    let mut local_17 = [0u64; 4];

    let mut entry = param_2;
    while entry != end_entries {
        if rd::<u64>(entry) != 0 {
            let vtbl = rd::<usize>(param_1);
            local_19 = rd::<[u64; 2]>(entry);
            let rbx = exe::call_va!(
                rd::<usize>(vtbl + 0x168),
                (
                    param_1,
                    local_17.as_mut_ptr() as usize,
                    local_19.as_mut_ptr() as usize
                ) -> usize
            );

            local_09 = [0u64; 4];
            let l09_ptr = local_09.as_mut_ptr() as usize;
            wr::<u64>(l09_ptr, rd::<u64>(entry + 0x10));
            wr::<u32>(l09_ptr + 0x08, rd::<u32>(entry + 0x18));
            wr::<u32>(l09_ptr + 0x0c, rd::<u32>(entry + 0x1c));
            wr::<u64>(entry + 0x10, 0);
            wr::<u64>(entry + 0x18, 0);

            wr::<u64>(l09_ptr + 0x10, rd::<u64>(entry + 0x20));
            wr::<u32>(l09_ptr + 0x18, rd::<u32>(entry + 0x28));
            wr::<u32>(l09_ptr + 0x1c, rd::<u32>(entry + 0x2c));
            wr::<u64>(entry + 0x20, 0);
            wr::<u64>(entry + 0x28, 0);

            exe::call_va!(
                0x1_413b_bf10,
                (
                    rd::<u64>(rbx),
                    (&mut local_39 as *mut usize) as usize,
                    rd::<u32>(rbx + 0x08),
                    l09_ptr
                )
            );
            exe::call_va!(0x1_401e_5470, (rbx));

            let rdi_rc = local_39;
            if rdi_rc != 0 {
                let old = crate::simd_ops::atomic_xadd_u32(rdi_rc as u64, u32::MAX);
                if old == 1 {
                    let obj = rd::<usize>(rdi_rc + 0x08) & !3usize;
                    if obj != 0 {
                        let obj_vtbl = rd::<usize>(obj);
                        exe::call_va!(rd::<usize>(obj_vtbl + 0x08), (obj, 0u32));
                        exe::call_va!(0x1_4244_2de0, (obj));
                    }
                    exe::call_va!(0x1_4244_2de0, (rdi_rc));
                }
                local_39 = 0;
            }

            let cnt_70 = rd::<u32>(l09_ptr + 0x18) as usize;
            let arr_70 = rd::<usize>(l09_ptr + 0x10);
            let end_70 = arr_70.wrapping_add(cnt_70.wrapping_mul(0x70));
            let mut cur_70 = arr_70;
            while cur_70 != end_70 {
                let rdx_arg = rd::<u64>(cur_70 + 0x40);
                if rdx_arg != 0 {
                    let rcx_obj = rd::<usize>(cur_70 + 0x58);
                    let r9_arg = rd::<u16>(cur_70 + 0x50) as u32;
                    let r8_arg = rd::<u64>(cur_70 + 0x48);
                    let obj_vtbl = rd::<usize>(rcx_obj);
                    exe::call_va!(
                        rd::<usize>(obj_vtbl + 0x10),
                        (rcx_obj, rdx_arg, r8_arg, r9_arg)
                    );
                }
                exe::call_va!(0x1_4240_ab10, (cur_70 + 0x10));
                cur_70 = cur_70.wrapping_add(0x70);
            }
            if arr_70 != 0 {
                let r10 = arr_70.wrapping_sub(1) & !0x01ff_ffffusize;
                let same_thread = exe::teb() == rd::<usize>(r10 + 0x100);
                let r9_ptr = rd::<usize>(l09_ptr + 0x10);
                let page_idx = r9_ptr.wrapping_sub(r10) >> 16;
                let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
                let rcx_hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
                if same_thread {
                    if rd::<u8>(rcx_hdr + 0x0e) == 0 {
                        wr::<u64>(r9_ptr, rd::<u64>(rcx_hdr + 0x20));
                        wr::<usize>(rcx_hdr + 0x20, r9_ptr);
                        let new_cnt = rd::<u32>(rcx_hdr + 0x18).wrapping_sub(1);
                        wr::<u32>(rcx_hdr + 0x18, new_cnt);
                        if new_cnt == 0 {
                            exe::call_va!(0x1_4253_71a0, (rcx_hdr));
                        }
                    } else {
                        exe::call_va!(0x1_4253_3200, (r10, rcx_hdr, 1u8, r9_ptr));
                    }
                } else {
                    exe::call_va!(0x1_4253_3200, (r10, rcx_hdr, 0u8, r9_ptr));
                }
            }

            let cnt_40 = rd::<u32>(l09_ptr + 0x08) as usize;
            let arr_40 = rd::<usize>(l09_ptr);
            let end_40 = arr_40.wrapping_add(cnt_40.wrapping_mul(0x40));
            let mut cur_40 = arr_40;
            while cur_40 != end_40 {
                let rdx_arg = rd::<u64>(cur_40 + 0x10);
                if rdx_arg != 0 {
                    let rcx_obj = rd::<usize>(cur_40 + 0x28);
                    let r9_arg = rd::<u16>(cur_40 + 0x20) as u32;
                    let r8_arg = rd::<u64>(cur_40 + 0x18);
                    let obj_vtbl = rd::<usize>(rcx_obj);
                    exe::call_va!(
                        rd::<usize>(obj_vtbl + 0x10),
                        (rcx_obj, rdx_arg, r8_arg, r9_arg)
                    );
                }
                cur_40 = cur_40.wrapping_add(0x40);
            }
            if arr_40 != 0 {
                let r10 = arr_40.wrapping_sub(1) & !0x01ff_ffffusize;
                let same_thread = exe::teb() == rd::<usize>(r10 + 0x100);
                let r9_ptr = rd::<usize>(l09_ptr);
                let page_idx = r9_ptr.wrapping_sub(r10) >> 16;
                let hdr_raw = r10.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
                let rcx_hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
                if same_thread {
                    if rd::<u8>(rcx_hdr + 0x0e) == 0 {
                        wr::<u64>(r9_ptr, rd::<u64>(rcx_hdr + 0x20));
                        wr::<usize>(rcx_hdr + 0x20, r9_ptr);
                        let new_cnt = rd::<u32>(rcx_hdr + 0x18).wrapping_sub(1);
                        wr::<u32>(rcx_hdr + 0x18, new_cnt);
                        if new_cnt == 0 {
                            exe::call_va!(0x1_4253_71a0, (rcx_hdr));
                        }
                    } else {
                        exe::call_va!(0x1_4253_3200, (r10, rcx_hdr, 1u8, r9_ptr));
                    }
                } else {
                    exe::call_va!(0x1_4253_3200, (r10, rcx_hdr, 0u8, r9_ptr));
                }
            }

            exe::call_va!(0x1_401e_5470, (local_17.as_mut_ptr() as usize));
            wr::<u128>(entry, 0);

            let rem_40 = rd::<usize>(entry + 0x10);
            let rem_cnt_40 = rd::<u32>(entry + 0x18) as usize;
            let rem_end_40 = rem_40.wrapping_add(rem_cnt_40.wrapping_mul(0x40));
            let mut cur_rem_40 = rem_40;
            while cur_rem_40 != rem_end_40 {
                let rdx_arg = rd::<u64>(cur_rem_40 + 0x10);
                if rdx_arg != 0 {
                    let rcx_obj = rd::<usize>(cur_rem_40 + 0x28);
                    let r9_arg = rd::<u16>(cur_rem_40 + 0x20) as u32;
                    let r8_arg = rd::<u64>(cur_rem_40 + 0x18);
                    let obj_vtbl = rd::<usize>(rcx_obj);
                    exe::call_va!(
                        rd::<usize>(obj_vtbl + 0x10),
                        (rcx_obj, rdx_arg, r8_arg, r9_arg)
                    );
                }
                cur_rem_40 = cur_rem_40.wrapping_add(0x40);
            }
            wr::<u32>(entry + 0x18, 0);

            let rem_70 = rd::<usize>(entry + 0x20);
            let rem_cnt_70 = rd::<u32>(entry + 0x28) as usize;
            let rem_end_70 = rem_70.wrapping_add(rem_cnt_70.wrapping_mul(0x70));
            let mut cur_rem_70 = rem_70;
            while cur_rem_70 != rem_end_70 {
                let rdx_arg = rd::<u64>(cur_rem_70 + 0x40);
                if rdx_arg != 0 {
                    let rcx_obj = rd::<usize>(cur_rem_70 + 0x58);
                    let r9_arg = rd::<u16>(cur_rem_70 + 0x50) as u32;
                    let r8_arg = rd::<u64>(cur_rem_70 + 0x48);
                    let obj_vtbl = rd::<usize>(rcx_obj);
                    exe::call_va!(
                        rd::<usize>(obj_vtbl + 0x10),
                        (rcx_obj, rdx_arg, r8_arg, r9_arg)
                    );
                }
                exe::call_va!(0x1_4240_ab10, (cur_rem_70 + 0x10));
                cur_rem_70 = cur_rem_70.wrapping_add(0x70);
            }
            wr::<u32>(entry + 0x28, 0);
        }
        entry = entry.wrapping_add(0x30);
    }
}

/// `r_142479e50`
/// Original: 0x142479e50 (459 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_142479e50(param_1: usize, param_2: usize) -> u8 {
    if rd::<u8>(param_1) == 0 {
        return 0;
    }

    let s_ptr = rd::<usize>(param_2);
    let s_len = rd::<u64>(param_2 + 0x08);
    let mut view_30: [u64; 2] = [s_ptr as u64, s_len];
    let mut local_50 = [0u64; 4];
    let l50_ptr = local_50.as_mut_ptr() as usize;

    let is_rooted = if s_len == 0 {
        false
    } else {
        let c0 = rd::<u8>(s_ptr);
        c0 == b'/' || c0 == b'\\' || (s_len >= 2 && rd::<u8>(s_ptr + 1) == b':')
    };

    if is_rooted {
        let mut tmp_40 = [0u64; 2];
        let g_3f8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_83f8) as usize);
        let rax = exe::call_va!(
            0x1_401d_f980,
            (tmp_40.as_mut_ptr() as usize, g_3f8.wrapping_add(0xa70)) -> usize
        );
        let mut view_20 = rd::<[u64; 2]>(rax);
        exe::call_va!(
            0x1_413a_41b0,
            (
                l50_ptr,
                view_30.as_mut_ptr() as usize,
                view_20.as_mut_ptr() as usize
            )
        );
    } else {
        wr::<u32>(l50_ptr, 0x0200_0002);
        wr::<u32>(l50_ptr + 0x04, 0);
        wr::<u8>(l50_ptr + 0x08, 0);
        exe::call_va!(0x1_401e_01e0, (l50_ptr, view_30.as_mut_ptr() as usize));
    }

    exe::call_iat!(0x1_4264_55a8, (param_1 + 0x38));
    let rbp = rd::<u64>(param_1 + 0x08).wrapping_add(rd::<u64>(param_1 + 0x20));

    let b3 = rd::<u8>(l50_ptr + 0x03) as u32;
    let cap_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let len_bits = (rd::<u32>(l50_ptr) & 0x00ff_ffff) << 3;
    let mut cur_ch = if len_bits > cap_bits {
        rd::<usize>(l50_ptr + 0x08)
    } else {
        l50_ptr + 0x08
    };
    let end_ch = cur_ch.wrapping_add(rd::<u32>(l50_ptr + 0x04) as usize);

    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    while cur_ch != end_ch {
        let ch = (rd::<u8>(cur_ch) as i8 as i64 as u64) | 0x20;
        h = (h ^ ch).wrapping_mul(0x0100_0000_01b3);
        cur_ch = cur_ch.wrapping_add(1);
    }
    let prod = (h as u128).wrapping_mul(0xde5f_b9d2_6304_58e9u64 as u128);
    let hash_mix = ((prod >> 64) as u64).wrapping_add(prod as u64);

    let mut out_idx: u64 = 0;
    let found = exe::call_va!(
        0x1_413b_d290,
        (
            param_1 + 0x08,
            l50_ptr,
            hash_mix,
            (&mut out_idx as *mut u64) as usize
        ) -> u8
    );
    let mut rdi = if found != 0 {
        out_idx
    } else {
        rd::<u64>(param_1 + 0x20)
    };
    rdi = rdi.wrapping_add(rd::<u64>(param_1 + 0x08));

    exe::call_iat!(0x1_4264_5598, (param_1 + 0x38));
    exe::call_va!(0x1_4240_ab10, (l50_ptr));
    u8::from(rdi != rbp)
}

/// `unknown_1424b0060`
/// Original: 0x1424b0060 (139 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1424b0060(
    param_1: u64,
    param_2: usize,
    param_3: usize,
) -> u64 {
    let vtbl2 = rd::<usize>(param_2);
    let id2 = exe::call_va!(rd::<usize>(vtbl2 + 0x08), (param_2) -> u32);
    let vtbl3 = rd::<usize>(param_3);
    let id3 = exe::call_va!(rd::<usize>(vtbl3 + 0x08), (param_3) -> u32);
    if id2 == id3 {
        return param_1;
    }
    let ti2 = exe::call_va!(0x1_424b_14c0, (id2) -> usize);
    let ti3 = exe::call_va!(0x1_424b_14c0, (id3) -> usize);
    let d2 = rd::<i32>(ti2 + 0x30);
    let d3 = rd::<i32>(ti3 + 0x30);
    if d2 > d3 {
        exe::call_va!(0x1_424a_fd60, (param_1, ti2, ti3) -> u64)
    } else if d2 < d3 {
        exe::call_va!(0x1_424a_fcb0, (param_1, ti2, ti3) -> u64)
    } else {
        0
    }
}

/// `unknown_1424b0d90`
/// Original: 0x1424b0d90 (162 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1424b0d90(param_1: usize) -> u64 {
    let lock_base = exe::ptr::<u8>(0x1_4387_1780) as usize;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock_base + 0x08) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock_base));
        wr::<u32>(lock_base + 0x08, tid);
    }
    wr::<u32>(lock_base + 0x0c, rd::<u32>(lock_base + 0x0c).wrapping_add(1));

    let res = if param_1 != 0 {
        let vtbl = rd::<usize>(param_1);
        let ok = exe::call_va!(rd::<usize>(vtbl + 0x30), (param_1) -> u8);
        if ok != 0 {
            let vtbl2 = rd::<usize>(param_1);
            let fn_40 = rd::<usize>(vtbl2 + 0x40);
            let arg_rdx = exe::call_va!(rd::<usize>(vtbl2), (param_1) -> u64);
            exe::call_va!(fn_40, (param_1, arg_rdx) -> u64)
        } else {
            0
        }
    } else {
        0
    };

    let new_cnt = rd::<u32>(lock_base + 0x0c).wrapping_sub(1);
    wr::<u32>(lock_base + 0x0c, new_cnt);
    if new_cnt == 0 {
        let _ = crate::simd_ops::atomic_xchg_u32((lock_base + 0x08) as u64, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock_base));
    }
    res
}

/// `r_1424e5480`
/// Original: 0x1424e5480 (132 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1424e5480(param_1: usize) {
    let rbx = rd::<usize>(param_1 + 0x08);
    let rdi = rd::<u64>(param_1 + 0x10);
    exe::call_va!(0x1_4209_d670, (rbx));
    exe::call_va!(
        0x1_4207_5af0,
        (rdi.wrapping_mul(0x30), 0x30u32, 0u32, 3u32, 0u32, 0u64, rbx)
    );
    wr::<u32>(rbx + 0x50, 3);
}

/// `r_1424e5530`
/// Original: 0x1424e5530 (131 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1424e5530(param_1: usize) {
    let rbx = rd::<usize>(param_1 + 0x08);
    let rdi = rd::<u64>(param_1 + 0x10);
    exe::call_va!(0x1_4209_d670, (rbx));
    exe::call_va!(
        0x1_4207_5af0,
        (rdi << 4, 0x10u32, 0u32, 3u32, 0u32, 0u64, rbx)
    );
    wr::<u32>(rbx + 0x50, 3);
}

/// `r_1424e55e0`
/// Original: 0x1424e55e0 (132 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1424e55e0(param_1: usize) {
    let rbx = rd::<usize>(param_1 + 0x08);
    let rdi = rd::<u64>(param_1 + 0x10);
    exe::call_va!(0x1_4209_d670, (rbx));
    exe::call_va!(
        0x1_4207_5af0,
        (rdi.wrapping_mul(4), 4u32, 0u32, 5u32, 0u32, 0u64, rbx)
    );
    wr::<u32>(rbx + 0x50, 3);
}

/// `unknown_142533040`
/// Original: 0x142533040 (395 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142533040(param_1: usize) -> u8 {
    let seg = param_1.wrapping_sub(1) & !0x01ff_ffffusize;
    let page_idx = param_1.wrapping_sub(seg) >> 16;
    let hdr_raw = seg.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
    let rbx = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);

    let mut spins: u64 = 0;
    let mut rdx: u64 = loop {
        let rax = crate::simd_ops::rd_acq_u64(rbx + 0x28);
        let r9 = rax & !3u64;
        let ecx = (rax as u32) & 3;
        if ecx == 1 {
            if spins >= 4 {
                return 0;
            }
            spins = spins.wrapping_add(1);
            core::hint::spin_loop();
            continue;
        }
        if ecx == 0 || ecx == 3 {
            break rax;
        }
        let (ok, _) = crate::simd_ops::atomic_cmpxchg_u64((rbx + 0x28) as u64, rax, r9);
        if ok {
            break crate::simd_ops::rd_acq_u64(rbx + 0x28);
        }
    };

    if (rdx & !3u64) != 0 {
        let r8_head = loop {
            let r8_ptr = (rdx & !3u64) as usize;
            let rcx_low = rdx & 3u64;
            let (ok, actual) =
                crate::simd_ops::atomic_cmpxchg_u64((rbx + 0x28) as u64, rdx, rcx_low);
            if ok {
                break r8_ptr;
            }
            rdx = actual;
        };
        if r8_head != 0 {
            let mut rax_cur = rd::<usize>(r8_head);
            let mut ecx: u32 = 1;
            let edx_cap = rd::<u16>(rbx + 0x0a) as u32;
            let mut r9_tail = r8_head;
            let mut corrupted = false;
            while rax_cur != 0 {
                if ecx > edx_cap {
                    corrupted = true;
                    break;
                }
                r9_tail = rax_cur;
                ecx = ecx.wrapping_add(1);
                rax_cur = rd::<usize>(rax_cur);
            }
            if corrupted || ecx > edx_cap {
                let msg = exe::ptr::<u8>(0x1_4342_54d8) as usize;
                exe::call_va!(0x1_4253_86a0, (0xeu32, msg));
            } else {
                wr::<u64>(r9_tail, rd::<u64>(rbx + 0x20));
                wr::<u32>(rbx + 0x18, rd::<u32>(rbx + 0x18).wrapping_sub(ecx));
                wr::<usize>(rbx + 0x20, r8_head);
            }
        }
    }

    let mut rax_next = rd::<u64>(rbx + 0x20);
    if rax_next != 0 && rd::<u64>(rbx + 0x10) == 0 {
        wr::<u8>(rbx + 0x0f, rd::<u8>(rbx + 0x0f) & !1u8);
        wr::<u64>(rbx + 0x10, rax_next);
        rax_next = 0;
    }
    wr::<u64>(param_1, rax_next);
    let new_cnt = rd::<u32>(rbx + 0x18).wrapping_sub(1);
    wr::<u32>(rbx + 0x18, new_cnt);
    wr::<usize>(rbx + 0x20, param_1);
    if new_cnt == 0 {
        exe::call_va!(0x1_4253_71a0, (rbx));
        return 1;
    }

    let b0e = rd::<u8>(rbx + 0x0e);
    if (b0e & 1) != 0 {
        let ecx_bsize = rd::<u32>(rbx + 0x1c);
        let r9_heap = rd::<usize>(rbx + 0x30);
        wr::<u8>(rbx + 0x0e, b0e & !1u8);
        let bin = exe::call_va!(0x1_4253_6cb0, (ecx_bsize) -> u8) as usize;
        wr::<u8>(rbx + 0x0e, rd::<u8>(rbx + 0x0e) | 1u8);
        let rcx_q = r9_heap.wrapping_add(0x410).wrapping_add(bin * 0x18);
        let rdx_q = r9_heap.wrapping_add(0xb00);
        exe::call_va!(0x1_4253_68c0, (rcx_q, rdx_q, rbx));
    }
    1
}

/// `unknown_142533200`
/// Original: 0x142533200 (59 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142533200(
    param_1: usize,
    param_2: usize,
    param_3: u8,
    param_4: usize,
) {
    let mut ptr = param_4;
    if (rd::<u8>(param_2 + 0x0e) & 2) != 0 {
        ptr = exe::call_va!(0x1_4253_3250, (param_1, param_2, ptr) -> usize);
    }
    exe::call_va!(0x1_4253_3300, (param_2, param_3, ptr));
}

/// `unknown_142533380`
/// Original: 0x142533380 (288 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142533380(param_1: usize, param_2: usize) {
    let seg1 = param_1.wrapping_sub(1) & !0x01ff_ffffusize;
    if rd::<u32>(seg1 + 0xf0) == 1 && rd::<u8>(seg1 + 0x18) != 0 && param_2 != 0 {
        let seg2 = param_2.wrapping_sub(1) & !0x01ff_ffffusize;
        let page_idx = param_2.wrapping_sub(seg2) >> 16;
        let hdr_raw = seg2.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
        let rdx_hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        let sz: u64 = if (rd::<u8>(rdx_hdr + 0x0e) & 2) == 0 {
            let bsize = rd::<u32>(rdx_hdr + 0x1c) as u64;
            if bsize < 0x8000_0000 {
                bsize
            } else {
                (rd::<u32>(rdx_hdr) as u64) << 16
            }
        } else {
            exe::call_va!(0x1_4253_2ff0, (seg2, rdx_hdr, param_2) -> u64)
        };
        if sz > 8 {
            let g_5c0 = exe::ptr::<u8>(0x1_43a0_95c0) as usize;
            exe::call_va!(0x1_4253_91a0, (param_2 + 0x08, sz - 8, g_5c0));
        }
    }

    let mut rax = crate::simd_ops::rd_acq_u64(param_1 + 0x28);
    let r8d = loop {
        let r8_low = (rax as u32) & 3;
        let rcx_new = if r8_low == 0 {
            (rax & !2u64) | 1u64
        } else {
            wr::<u64>(param_2, rax & !3u64);
            (r8_low as u64) | (param_2 as u64)
        };
        let (ok, actual) =
            crate::simd_ops::atomic_cmpxchg_u64((param_1 + 0x28) as u64, rax, rcx_new);
        if ok {
            break r8_low;
        }
        rax = actual;
    };

    if r8d == 0 {
        let rcx_heap = rd::<usize>(param_1 + 0x30);
        if rcx_heap != 0 {
            let mut rax_h = crate::simd_ops::rd_acq_u64(rcx_heap + 0xb18);
            loop {
                wr::<u64>(param_2, rax_h);
                let (ok, actual) = crate::simd_ops::atomic_cmpxchg_u64(
                    (rcx_heap + 0xb18) as u64,
                    rax_h,
                    param_2 as u64,
                );
                if ok {
                    break;
                }
                rax_h = actual;
            }
        }
        let mut rax_p = crate::simd_ops::rd_acq_u64(param_1 + 0x28);
        loop {
            let rcx_new = (rax_p & !1u64) | 2u64;
            let (ok, actual) =
                crate::simd_ops::atomic_cmpxchg_u64((param_1 + 0x28) as u64, rax_p, rcx_new);
            if ok {
                break;
            }
            rax_p = actual;
        }
    }
}

/// `unknown_142535070`
/// Original: 0x142535070 (341 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142535070(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> usize {
    let rdi = param_3.wrapping_sub(1);
    if param_3 <= param_2 && param_2 <= 0x80 && (param_2 & rdi) == 0 {
        let idx = (param_2.wrapping_add(7) >> 3) as usize;
        let page = rd::<usize>(param_1 + 0x08 + idx * 8);
        return exe::call_va!(0x1_4148_4ec0, (param_1, page, param_2, 0u8) -> usize);
    }

    let rcx_ptr = if param_3 > 0x0100_0000 {
        let sz = param_2.max(0x401);
        exe::call_va!(0x1_4253_7280, (param_1, sz, 0u8, param_3) -> usize)
    } else {
        let oversize = param_2.wrapping_sub(1).wrapping_add(param_3);
        if oversize <= 0x400 {
            let idx = (oversize.wrapping_add(7) >> 3) as usize;
            let page = rd::<usize>(param_1 + 0x08 + idx * 8);
            exe::call_va!(0x1_4148_4ec0, (param_1, page, oversize, 0u8) -> usize)
        } else {
            exe::call_va!(0x1_4253_7280, (param_1, oversize, 0u8, 0u64) -> usize)
        }
    };

    if rcx_ptr == 0 {
        return 0;
    }

    let rem = (rcx_ptr as u64) & rdi;
    let pad = if rem != 0 {
        param_3.wrapping_sub(rem) as usize
    } else {
        0
    };
    let r8_aligned = rcx_ptr.wrapping_add(pad);
    if r8_aligned != rcx_ptr {
        let seg = rcx_ptr.wrapping_sub(1) & !0x01ff_ffffusize;
        let page_idx = rcx_ptr.wrapping_sub(seg) >> 16;
        let hdr_raw = seg.wrapping_add(0x108).wrapping_add(page_idx.wrapping_mul(0x50));
        let rdx_hdr = hdr_raw.wrapping_sub(rd::<u32>(hdr_raw + 0x04) as usize);
        wr::<u8>(rdx_hdr + 0x0e, rd::<u8>(rdx_hdr + 0x0e) | 2u8);
    }
    r8_aligned
}

/// `unknown_142535360`
/// Original: 0x142535360 (122 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142535360(param_1: u64, param_2: u64) -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let r10 = rd::<usize>(tls0 + 0x6348);
    if param_2 == 0 || (param_2 & param_2.wrapping_sub(1)) != 0 {
        return 0;
    }
    if (param_1 & param_1.wrapping_sub(1)) == 0 && param_1 >= param_2 && param_1 <= 0x400 {
        let idx = (param_1.wrapping_add(7) >> 3) as usize;
        let rdx_page = rd::<usize>(r10 + 0x08 + idx * 8);
        let rax_block = rd::<usize>(rdx_page + 0x10);
        if rax_block == 0 {
            return exe::call_va!(0x1_4253_7280, (r10, param_1, 0u8, 0u64) -> usize);
        }
        wr::<u32>(rdx_page + 0x18, rd::<u32>(rdx_page + 0x18).wrapping_add(1));
        wr::<u64>(rdx_page + 0x10, rd::<u64>(rax_block));
        return rax_block;
    }
    exe::call_va!(0x1_4253_4f80, (r10, param_1, param_2) -> usize)
}

/// `unknown_1425371a0`
/// Original: 0x1425371a0 (208 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1425371a0(param_1: usize) {
    let b0e = rd::<u8>(param_1 + 0x0e) & !2u8;
    wr::<u8>(param_1 + 0x0e, b0e);
    let bin: u8 = if (b0e & 1) != 0 {
        0x4a
    } else {
        exe::call_va!(0x1_4253_6cb0, (rd::<u32>(param_1 + 0x1c)) -> u8)
    };
    let r8_heap = rd::<usize>(param_1 + 0x30);
    let rdx_q = r8_heap.wrapping_add(0x410).wrapping_add((bin as usize) * 0x18);
    let bsize = rd::<u32>(param_1 + 0x1c);
    if bsize > 0x20000
        || rd::<u64>(rdx_q + 0x10) > 0x20000
        || rd::<usize>(rdx_q + 0x08) != param_1
        || rd::<usize>(rdx_q) != param_1
    {
        exe::call_va!(0x1_4253_6070, (param_1, rdx_q));
        return;
    }
    let cl_base: u8 = if bsize <= 0x4000 { 0x22 } else { 0x0a };
    let b0f = (rd::<u8>(param_1 + 0x0f) & 1) | cl_base;
    wr::<u8>(param_1 + 0x0f, b0f);

    let idx = bin as u64;
    if idx < rd::<u64>(r8_heap + 0xbd8) {
        wr::<u64>(r8_heap + 0xbd8, idx);
    }
    if idx > rd::<u64>(r8_heap + 0xbe0) {
        wr::<u64>(r8_heap + 0xbe0, idx);
    }
}

/// `unknown_142537280`
/// Original: 0x142537280 (347 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142537280(
    param_1: usize,
    param_2: u64,
    param_3: u8,
    param_4: u64,
) -> usize {
    let empty_heap = exe::ptr::<u8>(0x1_4307_3d80) as usize;
    let mut rdi = param_1;
    if rdi == empty_heap {
        rdi = exe::call_va!(0x1_4254_62a0, () -> usize);
        if rdi == empty_heap {
            return 0;
        }
    }

    let tld = rd::<usize>(rdi);
    wr::<u64>(tld, rd::<u64>(tld).wrapping_add(1));
    let g_cb = exe::ptr::<usize>(0x1_43a6_3258) as usize;
    if rd::<usize>(g_cb) != 0 {
        let tld2 = rd::<usize>(rdi);
        if rd::<u8>(tld2 + 0x08) == 0 {
            wr::<u8>(tld2 + 0x08, 1);
            let cb = rd::<usize>(g_cb);
            let cnt = rd::<u64>(rd::<usize>(rdi));
            exe::call_va!(cb, (0u32, cnt, 0u32));
            let tld3 = rd::<usize>(rdi);
            wr::<u8>(tld3 + 0x08, 0);
        }
    }

    exe::call_va!(0x1_4253_6370, (rdi));
    let mut rbx = exe::call_va!(0x1_4253_53e0, (rdi, param_2, param_4) -> usize);
    if rbx == 0 {
        exe::call_va!(0x1_4254_5b80, (rdi, 1u32));
        rbx = exe::call_va!(0x1_4253_53e0, (rdi, param_2, param_4) -> usize);
        if rbx == 0 {
            let msg = exe::ptr::<u8>(0x1_4342_5528) as usize;
            exe::call_va!(0x1_4253_86a0, (0xcu32, msg, param_2));
            return 0;
        }
    }

    if param_3 != 0 && rd::<u32>(rbx + 0x1c) == 0 {
        let rsi = exe::call_va!(0x1_4148_4ec0, (rdi, rbx, param_2, 0u8) -> usize);
        let bsize = rd::<u32>(rbx + 0x1c) as u64;
        let sz = if bsize < 0x8000_0000 {
            bsize
        } else {
            (rd::<u32>(rbx) as u64) << 16
        };
        let g_580 = exe::ptr::<u8>(0x1_43a0_9580) as usize;
        if rd::<u8>(g_580) != 0 {
            for i in 0..(sz as usize) {
                wr::<u8>(rsi + i, 0);
            }
        } else {
            let _ = exe::call_va!(0x1_4183_acc2, (rsi, 0u32, sz) -> usize);
        }
        rsi
    } else {
        exe::call_va!(0x1_4148_4ec0, (rdi, rbx, param_2, param_3) -> usize)
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_142443a50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_3200: (usize, usize, u8, usize) = (0, 0, 0, 0);
    static mut LOG_71A0: usize = 0;

    unsafe extern "win64" fn mock_3200(seg: usize, hdr: usize, flag: u8, ptr: usize) {
        LOG_3200 = (seg, hdr, flag, ptr);
    }
    unsafe extern "win64" fn mock_71a0(hdr: usize) {
        LOG_71A0 = hdr;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_71a0, mock_71a0 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let r10 = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, mode: u8| -> (u64, u32, usize, (usize, usize, u8, usize), usize) {
        LOG_3200 = (0, 0, 0, 0);
        LOG_71A0 = 0;
        core::ptr::write_bytes(r10 as *mut u8, 0, 0x1_1000);

        let block_ptr = r10 + 0x1_0000;
        let hdr_raw = r10 + 0x108 + 0x50;
        wr::<u32>(hdr_raw + 0x04, 0);
        let rdx = hdr_raw;
        wr::<u64>(rdx + 0x20, 0xfeed_cafe);
        wr::<u32>(rdx + 0x18, 1);

        match mode {
            0 => {
                f(0);
            }
            1 => {
                wr::<usize>(r10 + 0x100, 0);
                f(block_ptr);
            }
            2 => {
                wr::<usize>(r10 + 0x100, exe::teb());
                wr::<u8>(rdx + 0x0e, 1);
                f(block_ptr);
            }
            _ => {
                wr::<usize>(r10 + 0x100, exe::teb());
                wr::<u8>(rdx + 0x0e, 0);
                f(block_ptr);
            }
        }
        (
            rd::<u64>(block_ptr),
            rd::<u32>(rdx + 0x18),
            rd::<usize>(rdx + 0x20),
            LOG_3200,
            LOG_71A0,
        )
    };

    for mode in 0..4u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "142443a50 mode={}", mode);
    }
    Ok(4)
}

pub unsafe fn diff_r_1424788c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_SUM: u64 = 0;
    static mut RC_NODE_ADDR: usize = 0;

    unsafe extern "win64" fn mock_v168(_this: usize, sret: usize, arg_19: usize) -> usize {
        let k0 = rd::<u64>(arg_19);
        wr::<u64>(sret, k0.wrapping_add(0x100));
        wr::<u32>(sret + 0x08, 7);
        sret
    }
    unsafe extern "win64" fn mock_bf10(rcx: u64, out_rc: usize, r8d: u32, _l09: usize) {
        CALL_SUM = CALL_SUM.wrapping_add(rcx).wrapping_add(r8d as u64);
        wr::<usize>(out_rc, RC_NODE_ADDR);
    }
    unsafe extern "win64" fn mock_5470(ptr: usize) {
        CALL_SUM = CALL_SUM.wrapping_add(rd::<u64>(ptr));
        wr::<u64>(ptr, 0);
    }
    unsafe extern "win64" fn mock_v08(this: usize, flag: u32) {
        CALL_SUM = CALL_SUM
            .wrapping_add(rd::<u64>(this + 0x08))
            .wrapping_add(flag as u64);
    }
    unsafe extern "win64" fn mock_2de0(_ptr: usize) {
        CALL_SUM = CALL_SUM.wrapping_add(0x2de0);
    }
    unsafe extern "win64" fn mock_v10(_this: usize, rdx: u64, r8: u64, r9: u16) {
        CALL_SUM = CALL_SUM
            .wrapping_add(rdx)
            .wrapping_add(r8)
            .wrapping_add(r9 as u64);
    }
    unsafe extern "win64" fn mock_ab10(ptr: usize) {
        CALL_SUM = CALL_SUM.wrapping_add(rd::<u64>(ptr));
    }
    unsafe extern "win64" fn mock_71a0(_hdr: usize) {
        CALL_SUM = CALL_SUM.wrapping_add(0x71a0);
    }
    unsafe extern "win64" fn mock_3200(_seg: usize, _hdr: usize, flag: u8, _ptr: usize) {
        CALL_SUM = CALL_SUM.wrapping_add(0x3200 + flag as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_413b_bf10, mock_bf10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_5470, mock_5470 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4253_71a0, mock_71a0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let r10 = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F| -> (u64, [u64; 18]) {
        CALL_SUM = 0;
        core::ptr::write_bytes(r10 as *mut u8, 0, 0x1_2000);
        wr::<usize>(r10 + 0x100, exe::teb());
        let hdr_raw = r10 + 0x108 + 0x50;
        wr::<u32>(hdr_raw + 0x04, 0);
        wr::<u32>(hdr_raw + 0x18, 2);
        wr::<u64>(hdr_raw + 0x20, 0x9999);

        let mut p1_vtbl = [0usize; 48];
        p1_vtbl[0x168 / 8] = mock_v168 as *const () as usize;
        let p1: [usize; 1] = [p1_vtbl.as_ptr() as usize];

        let mut obj_vtbl = [0usize; 4];
        obj_vtbl[0x08 / 8] = mock_v08 as *const () as usize;
        obj_vtbl[0x10 / 8] = mock_v10 as *const () as usize;
        let sub_obj: [u64; 2] = [obj_vtbl.as_ptr() as u64, 0x5555];
        let mut rc_node: [u64; 2] = [1, (sub_obj.as_ptr() as u64) | 1];
        RC_NODE_ADDR = rc_node.as_mut_ptr() as usize;

        let arr_40 = r10 + 0x1_0000;
        wr::<u64>(arr_40 + 0x10, 0x111);
        wr::<u64>(arr_40 + 0x18, 0x222);
        wr::<u16>(arr_40 + 0x20, 3);
        wr::<usize>(arr_40 + 0x28, sub_obj.as_ptr() as usize);

        let arr_70 = r10 + 0x1_0100;
        wr::<u64>(arr_70 + 0x10, 0xabc);
        wr::<u64>(arr_70 + 0x40, 0x333);
        wr::<u64>(arr_70 + 0x48, 0x444);
        wr::<u16>(arr_70 + 0x50, 5);
        wr::<usize>(arr_70 + 0x58, sub_obj.as_ptr() as usize);

        let mut entries = [0u64; 18];
        let ep = entries.as_mut_ptr() as usize;
        wr::<u64>(ep, 0x42);
        wr::<u64>(ep + 0x08, 0x43);
        wr::<usize>(ep + 0x10, arr_40);
        wr::<u32>(ep + 0x18, 1);
        wr::<usize>(ep + 0x20, arr_70);
        wr::<u32>(ep + 0x28, 1);

        f(p1.as_ptr() as usize, ep);
        (CALL_SUM, entries)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1424788c0 out");
    Ok(1)
}

pub unsafe fn diff_r_142479e50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LAST_HASH: u64 = 0;
    static mut DTOR_CNT: u32 = 0;

    unsafe extern "win64" fn mock_f980(out: usize, _rdx: usize) -> usize {
        wr::<u64>(out, 0x1234);
        wr::<u64>(out + 0x08, 4);
        out
    }
    unsafe extern "win64" fn mock_41b0(dst_str: usize, src_view: usize, _base_view: usize) {
        let p = rd::<usize>(src_view);
        let len = rd::<u32>(src_view + 0x08);
        wr::<u32>(dst_str, (4u32 << 24) | 2);
        wr::<u32>(dst_str + 0x04, len);
        for i in 0..(len as usize) {
            wr::<u8>(dst_str + 0x08 + i, rd::<u8>(p + i));
        }
    }
    unsafe extern "win64" fn mock_01e0(dst_str: usize, src_view: usize) {
        let p = rd::<usize>(src_view);
        let len = rd::<u32>(src_view + 0x08);
        wr::<u32>(dst_str + 0x04, len);
        for i in 0..(len as usize) {
            wr::<u8>(dst_str + 0x08 + i, rd::<u8>(p + i));
        }
    }
    unsafe extern "win64" fn mock_d290(_map: usize, _key: usize, hash: u64, out_idx: usize) -> u8 {
        LAST_HASH = hash;
        if (hash & 1) != 0 {
            wr::<u64>(out_idx, 0x10);
            1
        } else {
            wr::<u64>(out_idx, 0x40);
            1
        }
    }
    unsafe extern "win64" fn mock_ab10(_str_ptr: usize) {
        DTOR_CNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_f980, mock_f980 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413a_41b0, mock_41b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_01e0, mock_01e0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_413b_d290, mock_d290 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F, active: u8, s: &[u8]| -> (u8, u64, u32) {
        LAST_HASH = 0;
        DTOR_CNT = 0;
        let mut p1 = [0u8; 0x48];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u8>(p1p, active);
        wr::<u64>(p1p + 0x08, 0x1000);
        wr::<u64>(p1p + 0x20, 0x40);

        let p2: [u64; 2] = [s.as_ptr() as u64, s.len() as u64];
        let ret = f(p1p, p2.as_ptr() as usize);
        (ret, LAST_HASH, DTOR_CNT)
    };

    for (active, s) in [
        (0u8, b"rel/path" as &[u8]),
        (1, b"Rel/Path"),
        (1, b"/Abs/Path"),
        (1, b"C:\\WinPath"),
    ] {
        let o = run(f_o, active, s);
        let n = run(f_n, active, s);
        crate::diff_eq!(o, n, "142479e50 active={}", active);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_1424b0060(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TI_A: [u8; 0x40] = [0; 0x40];
    static mut TI_B: [u8; 0x40] = [0; 0x40];

    unsafe extern "win64" fn mock_get_id(this: usize) -> u32 {
        rd::<u32>(this + 0x08)
    }
    unsafe extern "win64" fn mock_14c0(id: u32) -> usize {
        if id == 1 {
            core::ptr::addr_of_mut!(TI_A) as usize
        } else {
            core::ptr::addr_of_mut!(TI_B) as usize
        }
    }
    unsafe extern "win64" fn mock_fd60(p1: u64, _ti2: usize, _ti3: usize) -> u64 {
        p1.wrapping_add(0x100)
    }
    unsafe extern "win64" fn mock_fcb0(p1: u64, _ti2: usize, _ti3: usize) -> u64 {
        p1.wrapping_add(0x200)
    }

    let _m1 = crate::difftest::Mock::new(0x1_424b_14c0, mock_14c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_424a_fd60, mock_fd60 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_424a_fcb0, mock_fcb0 as *const ())?;

    let mut vtbl = [0usize; 2];
    vtbl[1] = mock_get_id as *const () as usize;

    for (id2, id3, d2, d3) in [(1u32, 1u32, 5i32, 5i32), (1, 2, 10, 3), (1, 2, 2, 8), (1, 2, 4, 4)]
    {
        wr::<i32>(core::ptr::addr_of_mut!(TI_A) as usize + 0x30, d2);
        wr::<i32>(core::ptr::addr_of_mut!(TI_B) as usize + 0x30, d3);
        let obj2: [u64; 2] = [vtbl.as_ptr() as u64, id2 as u64];
        let obj3: [u64; 2] = [vtbl.as_ptr() as u64, id3 as u64];
        let o = f_o(0x5000, obj2.as_ptr() as usize, obj3.as_ptr() as usize);
        let n = f_n(0x5000, obj2.as_ptr() as usize, obj3.as_ptr() as usize);
        crate::diff_eq!(o, n, "1424b0060 id2={} id3={} d2={} d3={}", id2, id3, d2, d3);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_1424b0d90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_v00(this: usize) -> u64 {
        rd::<u64>(this + 0x10)
    }
    unsafe extern "win64" fn mock_v30(this: usize) -> u8 {
        rd::<u8>(this + 0x08)
    }
    unsafe extern "win64" fn mock_v40(_this: usize, rdx: u64) -> u64 {
        rdx.wrapping_mul(3)
    }

    let lock_base = exe::ptr::<u8>(0x1_4387_1780) as usize;
    let saved_lock = rd::<[u64; 2]>(lock_base);

    let mut vtbl = [0usize; 9];
    vtbl[0] = mock_v00 as *const () as usize;
    vtbl[0x30 / 8] = mock_v30 as *const () as usize;
    vtbl[0x40 / 8] = mock_v40 as *const () as usize;

    let run = |f: F, mode: u8| -> (u64, u32, u32) {
        wr::<u64>(lock_base, 0);
        wr::<u32>(lock_base + 0x08, u32::MAX);
        wr::<u32>(lock_base + 0x0c, 0);

        let mut obj = [0u64; 3];
        let op = obj.as_mut_ptr() as usize;
        wr::<usize>(op, vtbl.as_ptr() as usize);
        wr::<u8>(op + 0x08, u8::from(mode == 2));
        wr::<u64>(op + 0x10, 0x1111);

        let ret = if mode == 0 { f(0) } else { f(op) };
        (ret, rd::<u32>(lock_base + 0x08), rd::<u32>(lock_base + 0x0c))
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        wr::<[u64; 2]>(lock_base, saved_lock);
        crate::diff_eq!(o, n, "1424b0d90 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_r_1424e5480(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_D670: usize = 0;
    static mut LOG_5AF0: (u64, u32, u32, u32, u32, usize) = (0, 0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_d670(rcx: usize) {
        LOG_D670 = rcx;
    }
    unsafe extern "win64" fn mock_5af0(
        rcx: u64,
        rdx: u32,
        r8: u32,
        r9: u32,
        a5: u32,
        _a6: u64,
        a7: usize,
    ) {
        LOG_5AF0 = (rcx, rdx, r8, r9, a5, a7);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_5af0, mock_5af0 as *const ())?;

    let mut rbx_buf = [0u8; 0x60];
    let rbx_ptr = rbx_buf.as_mut_ptr() as usize;

    let run = |f: F, count: u64| -> (u32, usize, (u64, u32, u32, u32, u32, usize)) {
        LOG_D670 = 0;
        LOG_5AF0 = (0, 0, 0, 0, 0, 0);
        wr::<u32>(rbx_ptr + 0x50, 0);
        let callable: [u64; 3] = [0, rbx_ptr as u64, count];
        f(callable.as_ptr() as usize);
        (rd::<u32>(rbx_ptr + 0x50), LOG_D670, LOG_5AF0)
    };

    for cnt in [5u64, 12] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1424e5480 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_r_1424e5530(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_D670: usize = 0;
    static mut LOG_5AF0: (u64, u32, u32, u32, u32, usize) = (0, 0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_d670(rcx: usize) {
        LOG_D670 = rcx;
    }
    unsafe extern "win64" fn mock_5af0(
        rcx: u64,
        rdx: u32,
        r8: u32,
        r9: u32,
        a5: u32,
        _a6: u64,
        a7: usize,
    ) {
        LOG_5AF0 = (rcx, rdx, r8, r9, a5, a7);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_5af0, mock_5af0 as *const ())?;

    let mut rbx_buf = [0u8; 0x60];
    let rbx_ptr = rbx_buf.as_mut_ptr() as usize;

    let run = |f: F, count: u64| -> (u32, usize, (u64, u32, u32, u32, u32, usize)) {
        LOG_D670 = 0;
        LOG_5AF0 = (0, 0, 0, 0, 0, 0);
        wr::<u32>(rbx_ptr + 0x50, 0);
        let callable: [u64; 3] = [0, rbx_ptr as u64, count];
        f(callable.as_ptr() as usize);
        (rd::<u32>(rbx_ptr + 0x50), LOG_D670, LOG_5AF0)
    };

    for cnt in [7u64, 16] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1424e5530 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_r_1424e55e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_D670: usize = 0;
    static mut LOG_5AF0: (u64, u32, u32, u32, u32, usize) = (0, 0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_d670(rcx: usize) {
        LOG_D670 = rcx;
    }
    unsafe extern "win64" fn mock_5af0(
        rcx: u64,
        rdx: u32,
        r8: u32,
        r9: u32,
        a5: u32,
        _a6: u64,
        a7: usize,
    ) {
        LOG_5AF0 = (rcx, rdx, r8, r9, a5, a7);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_5af0, mock_5af0 as *const ())?;

    let mut rbx_buf = [0u8; 0x60];
    let rbx_ptr = rbx_buf.as_mut_ptr() as usize;

    let run = |f: F, count: u64| -> (u32, usize, (u64, u32, u32, u32, u32, usize)) {
        LOG_D670 = 0;
        LOG_5AF0 = (0, 0, 0, 0, 0, 0);
        wr::<u32>(rbx_ptr + 0x50, 0);
        let callable: [u64; 3] = [0, rbx_ptr as u64, count];
        f(callable.as_ptr() as usize);
        (rd::<u32>(rbx_ptr + 0x50), LOG_D670, LOG_5AF0)
    };

    for cnt in [9u64, 24] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "1424e55e0 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142533040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_86a0(code: u32, _msg: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(code as u64);
    }
    unsafe extern "win64" fn mock_71a0(rbx: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u32>(rbx + 0x1c) as u64);
    }
    unsafe extern "win64" fn mock_6cb0(bsize: u32) -> u8 {
        (bsize >> 4) as u8
    }
    unsafe extern "win64" fn mock_68c0(rcx_q: usize, rdx_q: usize, rbx: usize) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rdx_q.wrapping_sub(rcx_q) as u64)
            .wrapping_add(rd::<u8>(rbx + 0x0e) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_86a0, mock_86a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_71a0, mock_71a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_6cb0, mock_6cb0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4253_68c0, mock_68c0 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, mode: u8| -> (u8, u64, u32, u64, u8, u64) {
        LOG_SUM = 0;
        core::ptr::write_bytes(seg as *mut u8, 0, 0x1_2000);
        let block_ptr = seg + 0x1_0000;
        let tf_node = seg + 0x1_0100;
        let heap_ptr = seg + 0x1_0200;
        let rbx = seg + 0x108 + 0x50;

        wr::<u16>(rbx + 0x0a, 4);
        wr::<u8>(rbx + 0x0f, 3);
        wr::<u64>(rbx + 0x20, 0xaaaa);
        wr::<u32>(rbx + 0x1c, 0x40);
        wr::<usize>(rbx + 0x30, heap_ptr);

        match mode {
            0 => {
                // Busy bit 1 -> spins 4 times and returns 0
                wr::<u64>(rbx + 0x28, 1);
                wr::<u32>(rbx + 0x18, 2);
            }
            1 => {
                // Thread-free list with 1 node, retires full flag (b0e & 1)
                wr::<u64>(tf_node, 0);
                wr::<u64>(rbx + 0x28, (tf_node as u64) | 2);
                wr::<u32>(rbx + 0x18, 3);
                wr::<u8>(rbx + 0x0e, 1);
            }
            _ => {
                // Count drops to 0 -> calls 0x1425371a0
                wr::<u64>(rbx + 0x28, 0);
                wr::<u32>(rbx + 0x18, 1);
            }
        }

        let ret = f(block_ptr);
        (
            ret,
            rd::<u64>(block_ptr),
            rd::<u32>(rbx + 0x18),
            rd::<u64>(rbx + 0x10),
            rd::<u8>(rbx + 0x0f),
            LOG_SUM,
        )
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "142533040 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142533200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_3300: (usize, u8, usize) = (0, 0, 0);

    unsafe extern "win64" fn mock_3250(_p1: usize, _p2: usize, p4: usize) -> usize {
        p4 & !0x3fusize
    }
    unsafe extern "win64" fn mock_3300(p2: usize, p3: u8, ptr: usize) {
        LOG_3300 = (p2, p3, ptr);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_3250, mock_3250 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_3300, mock_3300 as *const ())?;

    let mut page = [0u8; 0x20];
    let pp = page.as_mut_ptr() as usize;

    for (b0e, flag) in [(0u8, 0u8), (2, 1)] {
        LOG_3300 = (0, 0, 0);
        wr::<u8>(pp + 0x0e, b0e);
        f_o(0x1000, pp, flag, 0x2345);
        let o = LOG_3300;

        LOG_3300 = (0, 0, 0);
        wr::<u8>(pp + 0x0e, b0e);
        f_n(0x1000, pp, flag, 0x2345);
        let n = LOG_3300;

        crate::diff_eq!(o, n, "142533200 b0e={}", b0e);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142533380(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_91A0: (usize, u64) = (0, 0);

    unsafe extern "win64" fn mock_2ff0(_seg: usize, _hdr: usize, _p2: usize) -> u64 {
        0x40
    }
    unsafe extern "win64" fn mock_91a0(ptr: usize, sz: u64, _pat: usize) {
        LOG_91A0 = (ptr, sz);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_2ff0, mock_2ff0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_91a0, mock_91a0 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, init_28: u64, aligned_flag: u8| -> (u64, u64, u64, (usize, u64)) {
        LOG_91A0 = (0, 0);
        core::ptr::write_bytes(seg as *mut u8, 0, 0x1_2000);
        wr::<u32>(seg + 0xf0, 1);
        wr::<u8>(seg + 0x18, 1);

        let page_hdr = seg + 0x108 + 0x50;
        let block_ptr = seg + 0x1_0000;
        let heap_ptr = seg + 0x1_0200;

        wr::<u8>(page_hdr + 0x0e, aligned_flag);
        wr::<u32>(page_hdr + 0x1c, 0x30);
        wr::<u64>(page_hdr + 0x28, init_28);
        wr::<usize>(page_hdr + 0x30, heap_ptr);
        wr::<u64>(heap_ptr + 0xb18, 0x7777);

        f(page_hdr, block_ptr);
        (
            rd::<u64>(block_ptr),
            rd::<u64>(page_hdr + 0x28),
            rd::<u64>(heap_ptr + 0xb18),
            LOG_91A0,
        )
    };

    for (init_28, aflag) in [(0u64, 0u8), (0x8880 | 2, 2)] {
        let o = run(f_o, init_28, aflag);
        let n = run(f_n, init_28, aflag);
        crate::diff_eq!(o, n, "142533380 init_28={:#x}", init_28);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_142535070(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_BASE: usize = 0;
    static mut LOG_CALLS: u64 = 0;

    unsafe extern "win64" fn mock_4ec0(_heap: usize, page: usize, size: u64, zero: u8) -> usize {
        LOG_CALLS = LOG_CALLS
            .wrapping_add(page as u64)
            .wrapping_add(size)
            .wrapping_add(zero as u64);
        ALLOC_BASE
    }
    unsafe extern "win64" fn mock_7280(_heap: usize, size: u64, zero: u8, align: u64) -> usize {
        LOG_CALLS = LOG_CALLS
            .wrapping_add(size)
            .wrapping_add(zero as u64)
            .wrapping_add(align);
        ALLOC_BASE
    }

    let _m1 = crate::difftest::Mock::new(0x1_4148_4ec0, mock_4ec0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_7280, mock_7280 as *const ())?;

    let mut backing = vec![0u8; 0x0202_0000];
    let seg = (backing.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, size: u64, align: u64| -> (usize, u8, u64) {
        LOG_CALLS = 0;
        core::ptr::write_bytes(seg as *mut u8, 0, 0x1_1000);
        ALLOC_BASE = seg + 0x1_0008; // misaligned by 8 for 32-byte alignment
        let page_hdr = seg + 0x108 + 0x50;

        let mut heap = [0usize; 0x90];
        for i in 0..0x90 {
            heap[i] = 0x1000 + i;
        }
        let ret = f(heap.as_ptr() as usize, size, align);
        (ret, rd::<u8>(page_hdr + 0x0e), LOG_CALLS)
    };

    for (sz, al) in [(64u64, 16u64), (100, 32), (0x500, 64), (128, 0x0200_0000)] {
        let o = run(f_o, sz, al);
        let n = run(f_n, sz, al);
        crate::diff_eq!(o, n, "142535070 sz={} al={}", sz, al);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_142535360(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_CALL: (u64, u64) = (0, 0);

    unsafe extern "win64" fn mock_7280(_heap: usize, size: u64, _z: u8, align: u64) -> usize {
        LOG_CALL = (size, align);
        0x7280_0000
    }
    unsafe extern "win64" fn mock_4f80(_heap: usize, size: u64, align: u64) -> usize {
        LOG_CALL = (size, align);
        0x4f80_0000
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_7280, mock_7280 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_4f80, mock_4f80 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_tls_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, size: u64, align: u64, has_free: bool| -> (usize, u32, u64, (u64, u64)) {
        LOG_CALL = (0, 0);
        let mut free_node: u64 = 0xaaaa_bbbb;
        let mut page = [0u8; 0x20];
        let pp = page.as_mut_ptr() as usize;
        if has_free {
            wr::<usize>(pp + 0x10, (&mut free_node as *mut u64) as usize);
        }
        wr::<u32>(pp + 0x18, 5);

        let mut heap = [0usize; 0x90];
        for slot in &mut heap {
            *slot = pp;
        }
        wr::<usize>(tls0 + 0x6348, heap.as_ptr() as usize);

        let ret = f(size, align);
        let norm_ret = if has_free && ret == (&mut free_node as *mut u64) as usize {
            0x1111_0000
        } else {
            ret
        };
        (norm_ret, rd::<u32>(pp + 0x18), rd::<u64>(pp + 0x10), LOG_CALL)
    };

    for (sz, al, hf) in [
        (64u64, 0u64, true),
        (64, 6, true),
        (64, 16, true),
        (64, 16, false),
        (100, 16, true),
    ] {
        let o = run(f_o, sz, al, hf);
        let n = run(f_n, sz, al, hf);
        wr::<usize>(tls0 + 0x6348, saved_tls_heap);
        crate::diff_eq!(o, n, "142535360 sz={} al={} hf={}", sz, al, hf);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1425371a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLED_6070: u32 = 0;

    unsafe extern "win64" fn mock_6cb0(_bsize: u32) -> u8 {
        5
    }
    unsafe extern "win64" fn mock_6070(_page: usize, q: usize) {
        CALLED_6070 = (rd::<u64>(q + 0x10) as u32).wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_6cb0, mock_6cb0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_6070, mock_6070 as *const ())?;

    let run = |f: F, b0e: u8, bsize: u32, singleton: bool| -> (u8, u8, u64, u64, u32) {
        CALLED_6070 = 0;
        let mut heap = vec![0u8; 0xc00];
        let hp = heap.as_mut_ptr() as usize;
        wr::<u64>(hp + 0xbd8, 10);
        wr::<u64>(hp + 0xbe0, 2);

        let mut page = [0u8; 0x40];
        let pp = page.as_mut_ptr() as usize;
        wr::<u8>(pp + 0x0e, b0e);
        wr::<u8>(pp + 0x0f, 1);
        wr::<u32>(pp + 0x1c, bsize);
        wr::<usize>(pp + 0x30, hp);

        let bin = if (b0e & 1) != 0 { 0x4ausize } else { 5usize };
        let q = hp + 0x410 + bin * 0x18;
        if singleton {
            wr::<usize>(q, pp);
            wr::<usize>(q + 0x08, pp);
        }
        wr::<u64>(q + 0x10, 64);

        f(pp);
        (
            rd::<u8>(pp + 0x0e),
            rd::<u8>(pp + 0x0f),
            rd::<u64>(hp + 0xbd8),
            rd::<u64>(hp + 0xbe0),
            CALLED_6070,
        )
    };

    for (b0e, bsize, sing) in [(2u8, 0x100u32, true), (3, 0x8000, true), (2, 0x100, false)] {
        let o = run(f_o, b0e, bsize, sing);
        let n = run(f_n, b0e, bsize, sing);
        crate::diff_eq!(o, n, "1425371a0 b0e={} bsize={:#x} sing={}", b0e, bsize, sing);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142537280(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u8, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut PAGE_BUF: [u8; 0x30] = [0; 0x30];
    static mut OUT_MEM: [u8; 32] = [0; 32];
    static mut CALL_SUM: u64 = 0;
    static mut TRY_CNT: u32 = 0;

    unsafe extern "win64" fn mock_cb(_a: u32, cnt: u64, _b: u32) {
        CALL_SUM = CALL_SUM.wrapping_add(cnt * 10);
    }
    unsafe extern "win64" fn mock_6370(_heap: usize) {
        CALL_SUM = CALL_SUM.wrapping_add(1);
    }
    unsafe extern "win64" fn mock_53e0(_heap: usize, _sz: u64, _al: u64) -> usize {
        TRY_CNT += 1;
        if TRY_CNT == 1 {
            0
        } else {
            core::ptr::addr_of_mut!(PAGE_BUF) as usize
        }
    }
    unsafe extern "win64" fn mock_5b80(_heap: usize, arg: u32) {
        CALL_SUM = CALL_SUM.wrapping_add(arg as u64 * 100);
    }
    unsafe extern "win64" fn mock_4ec0(_heap: usize, page: usize, _sz: u64, zero: u8) -> usize {
        CALL_SUM = CALL_SUM.wrapping_add(zero as u64 * 1000);
        wr::<u32>(page + 0x1c, 16);
        core::ptr::addr_of_mut!(OUT_MEM) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_6370, mock_6370 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_53e0, mock_53e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4254_5b80, mock_5b80 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4148_4ec0, mock_4ec0 as *const ())?;

    let g_cb = exe::ptr::<usize>(0x1_43a6_3258) as usize;
    let g_580 = exe::ptr::<u8>(0x1_43a0_9580) as usize;
    let s_cb = rd::<usize>(g_cb);
    let s_580 = rd::<u8>(g_580);

    let run = |f: F, zflag: u8, rep_stosb: u8| -> (usize, u64, [u8; 32], u64) {
        PAGE_BUF = [0; 0x30];
        OUT_MEM = [0xaa; 32];
        CALL_SUM = 0;
        TRY_CNT = 0;
        wr::<usize>(g_cb, mock_cb as *const () as usize);
        wr::<u8>(g_580, rep_stosb);

        let mut tld = [5u64, 0];
        let heap: [usize; 1] = [tld.as_mut_ptr() as usize];
        let ret = f(heap.as_ptr() as usize, 16, zflag, 8);
        (ret, tld[0], OUT_MEM, CALL_SUM)
    };

    for (zf, rs) in [(0u8, 0u8), (1, 0), (1, 1)] {
        let o = run(f_o, zf, rs);
        let n = run(f_n, zf, rs);
        wr::<usize>(g_cb, s_cb);
        wr::<u8>(g_580, s_580);
        crate::diff_eq!(o, n, "142537280 zf={} rs={}", zf, rs);
    }
    Ok(3)
}

// REGISTRY:
// Replacement::verified(0x1_4244_3a50, "unknown_142443a50", rend::cauldron_hot_y7::unknown_142443a50 as *const (), rend::cauldron_hot_y7::diff_unknown_142443a50),
// Replacement::verified(0x1_4247_88c0, "r_1424788c0", rend::cauldron_hot_y7::r_1424788c0 as *const (), rend::cauldron_hot_y7::diff_r_1424788c0),
// Replacement::verified_throws(0x1_4247_9e50, "r_142479e50", rend::cauldron_hot_y7::r_142479e50 as *const (), Throws::Discard, rend::cauldron_hot_y7::diff_r_142479e50),
// Replacement::verified_throws(0x1_424b_0060, "unknown_1424b0060", rend::cauldron_hot_y7::unknown_1424b0060 as *const (), Throws::Discard, rend::cauldron_hot_y7::diff_unknown_1424b0060),
// Replacement::verified(0x1_424b_0d90, "unknown_1424b0d90", rend::cauldron_hot_y7::unknown_1424b0d90 as *const (), rend::cauldron_hot_y7::diff_unknown_1424b0d90),
// Replacement::verified_throws(0x1_424e_5480, "r_1424e5480", rend::cauldron_hot_y7::r_1424e5480 as *const (), Throws::Discard, rend::cauldron_hot_y7::diff_r_1424e5480),
// Replacement::verified_throws(0x1_424e_5530, "r_1424e5530", rend::cauldron_hot_y7::r_1424e5530 as *const (), Throws::Discard, rend::cauldron_hot_y7::diff_r_1424e5530),
// Replacement::verified_throws(0x1_424e_55e0, "r_1424e55e0", rend::cauldron_hot_y7::r_1424e55e0 as *const (), Throws::Discard, rend::cauldron_hot_y7::diff_r_1424e55e0),
// Replacement::verified(0x1_4253_3040, "unknown_142533040", rend::cauldron_hot_y7::unknown_142533040 as *const (), rend::cauldron_hot_y7::diff_unknown_142533040),
// Replacement::verified(0x1_4253_3200, "unknown_142533200", rend::cauldron_hot_y7::unknown_142533200 as *const (), rend::cauldron_hot_y7::diff_unknown_142533200),
// Replacement::verified(0x1_4253_3380, "unknown_142533380", rend::cauldron_hot_y7::unknown_142533380 as *const (), rend::cauldron_hot_y7::diff_unknown_142533380),
// Replacement::verified(0x1_4253_5070, "unknown_142535070", rend::cauldron_hot_y7::unknown_142535070 as *const (), rend::cauldron_hot_y7::diff_unknown_142535070),
// Replacement::verified(0x1_4253_5360, "unknown_142535360", rend::cauldron_hot_y7::unknown_142535360 as *const (), rend::cauldron_hot_y7::diff_unknown_142535360),
// Replacement::verified(0x1_4253_71a0, "unknown_1425371a0", rend::cauldron_hot_y7::unknown_1425371a0 as *const (), rend::cauldron_hot_y7::diff_unknown_1425371a0),
// Replacement::verified(0x1_4253_7280, "unknown_142537280", rend::cauldron_hot_y7::unknown_142537280 as *const (), rend::cauldron_hot_y7::diff_unknown_142537280),
