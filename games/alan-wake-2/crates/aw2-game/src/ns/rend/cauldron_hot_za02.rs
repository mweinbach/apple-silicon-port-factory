//! Decompiled hot functions shard ZA02 (`0x1403c79b0..0x14094baa0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn clear_inplace_string(s_ptr: usize) {
    let b3 = rd::<u8>(s_ptr + 0x03) as u32;
    let mut data_ptr = s_ptr.wrapping_add(0x08);
    wr::<u32>(s_ptr + 0x04, 0);
    let inline_bits = if b3 < 0x80 {
        b3.wrapping_mul(8)
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let cap_bits = (rd::<u32>(s_ptr) & 0x00ff_ffff) << 3;
    if cap_bits > inline_bits {
        data_ptr = rd::<usize>(data_ptr);
    }
    wr::<u8>(data_ptr, 0);
}

#[allow(clippy::too_many_arguments)]
#[inline(always)]
unsafe fn call_1419ceef0(
    a1: usize,
    a2: usize,
    a3: usize,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
    a8: u64,
    a9: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a13: u64,
    a14: u64,
    a15: u64,
) {
    #[cfg(target_arch = "x86_64")]
    {
        let addr = exe::lookup_native_va(0x1_419c_eef0)
            .unwrap_or_else(|| exe::to_mapped_addr(0x1_419c_eef0));
        let f: unsafe extern "win64" fn(
            usize,
            usize,
            usize,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
        ) -> u64 = core::mem::transmute(addr);
        let _ = f(
            a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15,
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let buf: [u64; 16] = [
            0, 0, 0, 0, 0, 0, 0, 0, a9, a10, a11, a12, a13, a14, a15, 0,
        ];
        let base = buf.as_ptr() as u64;
        crate::emu::set_outgoing_rsp(base, base + 128);
        let _ = crate::emu::call_va8(
            0x1_419c_eef0,
            a1 as u64,
            a2 as u64,
            a3 as u64,
            a4,
            a5,
            a6,
            a7,
            a8,
        );
    }
}

/// `unknown_1403c79b0`
/// Original: 0x1403c79b0 (124 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1403c79b0(param_1: usize, param_2: i32, param_3: i32) {
    if rd::<u32>(param_1 + 0x318).wrapping_sub(1) > 3 {
        let v328 = rd::<i32>(param_1 + 0x328);
        if v328 < param_3 && v328 >= param_2 {
            let rbx = rd::<usize>(param_1 + 0x320);
            if rbx != 0 {
                let mut str_buf = [0u64; 18];
                let sbp = str_buf.as_mut_ptr() as usize;
                wr::<u64>(sbp, 0x1000_0010);
                wr::<u8>(sbp + 0x08, 0);
                let mut local_res8: i32 = 0;
                let fn48 = rd::<u64>(rd::<usize>(rbx) + 0x48);
                exe::call_va!(fn48, (rbx, (&mut local_res8 as *mut i32) as usize));
                let rcx_sub = rd::<usize>(rbx + 0x08);
                let fn38 = rd::<u64>(rd::<usize>(rcx_sub) + 0x38);
                exe::call_va!(fn38, (rcx_sub, local_res8 as i64 as u64, 0u32));
                exe::call_va!(0x1_4240_ab10, (sbp));
            }
        }
    }
}

/// `unknown_1403d17d0`
/// Original: 0x1403d17d0 (229 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1403d17d0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
) -> usize {
    let r10 = rd::<usize>(param_3);
    let idx = (param_2 as u32) as usize;
    let cap_20 = rd::<u32>(r10 + 0x20) as usize;
    if idx < cap_20 {
        let arr_18 = rd::<usize>(r10 + 0x18);
        let entry = rd::<u64>(arr_18.wrapping_add(idx * 8));
        if entry == param_2 && (param_2 as u32) != u32::MAX {
            let arr_30 = rd::<usize>(r10 + 0x30);
            let dx = rd::<u64>(arr_30.wrapping_add(idx * 8)) as u16;
            if dx == 0xffff {
                exe::call_iat!(0x1_4264_67e8, ());
                return param_1;
            }
            let rcx_slice = rd::<usize>(param_3 + 0x10);
            let count_8 = rd::<u64>(param_3 + 0x08) as usize;
            let rbx_end = rcx_slice.wrapping_add(count_8.wrapping_mul(2));
            let mut cur = rcx_slice;
            let mut rem = (rbx_end.wrapping_sub(rcx_slice) as isize) >> 1;
            while rem > 0 {
                let half = ((rem as usize) >> 1) as isize;
                let mid = cur.wrapping_add((half as usize) * 2);
                if rd::<u16>(mid) < dx {
                    cur = mid.wrapping_add(2);
                    rem = rem - 1 - half;
                } else {
                    rem = half;
                }
            }
            if cur != rbx_end && dx >= rd::<u16>(cur) {
                wr::<u64>(param_1, param_2);
                return param_1;
            }
        }
    }
    wr::<u64>(param_1, u64::MAX);
    param_1
}

/// `content_1403f4a80`
/// Original: 0x1403f4a80 (315 bytes, Throws::Discard)
pub unsafe extern "win64" fn content_1403f4a80(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
) {
    let ty = exe::call_va!(0x1_4211_9670, (param_1, param_2, param_3) -> i32);
    if ty != 5 {
        let tn = exe::call_va!(0x1_4211_9760, (param_1, ty as u32) -> usize);
        let fmt = exe::ptr::<u8>(0x1_4310_6af8) as usize;
        exe::call_va!(0x1_4211_6b30, (param_1, fmt, param_3, tn));
        return;
    }

    let top = rd::<usize>(param_1 + 0x08);
    let slot = top.wrapping_sub(0x18);
    let mut ok_str = true;
    if rd::<i32>(slot + 0x10) != 5 {
        let b1 = rd::<u8>(param_1 + 0x01);
        if (b1 & 4) != 0 {
            let g = rd::<usize>(param_1 + 0x18);
            wr::<u8>(param_1 + 0x01, b1 & !4);
            wr::<u64>(param_1 + 0x68, rd::<u64>(g + 0x30));
            wr::<usize>(g + 0x30, param_1);
        }
        let conv = exe::call_va!(0x1_420e_cc90, (param_1, slot) -> i32);
        if conv == 0 {
            ok_str = false;
        } else {
            let g = rd::<usize>(param_1 + 0x18);
            if rd::<u64>(g + 0x48) >= rd::<u64>(g + 0x40) {
                exe::call_va!(0x1_4210_c770, (param_1));
            }
        }
    }

    if ok_str {
        let slot2 = rd::<usize>(param_1 + 0x08).wrapping_sub(0x18);
        let str_ptr = rd::<usize>(slot2).wrapping_add(0x18);
        if str_ptr != 0 {
            let mut len = 0usize;
            while rd::<u8>(str_ptr.wrapping_add(len)) != 0 {
                len += 1;
            }
            let mut view: [u64; 2] = [str_ptr as u64, len as u64];
            exe::call_va!(
                0x1_4240_9b50,
                (param_4, view.as_mut_ptr() as usize, 0u64, len as u64)
            );
            wr::<usize>(param_1 + 0x08, rd::<usize>(param_1 + 0x08).wrapping_sub(0x18));
            return;
        }
    }

    clear_inplace_string(param_4);
    wr::<usize>(param_1 + 0x08, rd::<usize>(param_1 + 0x08).wrapping_sub(0x18));
}

/// `unknown_1403fec40`
/// Original: 0x1403fec40 (120 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1403fec40(param_1: usize, param_2: u64) {
    let cap = rd::<u32>(param_1 + 0x0c) as u64;
    let saved_len = rd::<u32>(param_1 + 0x08);
    let mut new_cap = (cap >> 1).wrapping_add(cap);
    if new_cap < param_2 {
        new_cap = param_2;
    }
    let dst = exe::call_va!(0x1_4024_a9e0, (param_1, new_cap) -> usize);
    let cur_len = rd::<u32>(param_1 + 0x08) as u64;
    let src = rd::<usize>(param_1);
    let byte_len = cur_len.wrapping_mul(0x30);
    let _ = exe::call_va!(0x1_4183_acc8, (dst, src, byte_len) -> usize);
    exe::call_va!(0x1_401d_e830, (param_1));
    wr::<usize>(param_1, dst);
    wr::<u32>(param_1 + 0x08, saved_len);
    wr::<u32>(param_1 + 0x0c, new_cap as u32);
}

/// `unknown_140455cd0`
/// Original: 0x140455cd0 (469 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_140455cd0(param_1: usize) {
    let r12_0 = rd::<usize>(param_1);
    let r8_arr = rd::<usize>(r12_0 + 0x08);
    let rdx_tbl = rd::<usize>(rd::<usize>(r12_0) + 0x20);
    let r12_arr = rd::<usize>(r12_0 + 0x580e0);

    let read_pair = |va: u64| -> [u64; 3] {
        let u = rd::<u16>(exe::ptr::<u16>(va) as usize) as usize;
        let idx = rd::<u16>(rdx_tbl.wrapping_add(u * 2)) as usize;
        let off = idx.wrapping_mul(0x10);
        let len = rd::<u32>(r8_arr.wrapping_add(off + 0x08)) as u64;
        let ptr = rd::<u64>(r8_arr.wrapping_add(off));
        [r12_0 as u64, len, ptr]
    };

    let read_slot = |va: u64| -> u64 {
        let u = rd::<u16>(exe::ptr::<u16>(va) as usize) as usize;
        rd::<u64>(r12_arr.wrapping_add(u * 8))
    };

    let mut s_a = read_pair(0x1_43c8_ced4);
    let mut s_b = read_pair(0x1_43c9_05d4);
    let mut s_c = read_pair(0x1_43c9_3cb0);

    let a4 = read_slot(0x1_43c8_ced8);
    let a5 = read_slot(0x1_43ca_14a0);
    let a6 = read_slot(0x1_43c9_3214);
    let a7 = read_slot(0x1_43ca_14a8);
    let a8 = read_slot(0x1_43ca_14ac);
    let a9 = read_slot(0x1_43c9_e24c);
    let a10 = read_slot(0x1_43ca_249c);
    let a11 = read_slot(0x1_43c9_f408);
    let a12 = read_slot(0x1_43c9_4010);
    let a13 = read_slot(0x1_43c8_ce6c);
    let a14 = read_slot(0x1_43c8_ceec);
    let a15 = read_slot(0x1_43c8_cebc);

    call_1419ceef0(
        s_a.as_mut_ptr() as usize,
        s_b.as_mut_ptr() as usize,
        s_c.as_mut_ptr() as usize,
        a4,
        a5,
        a6,
        a7,
        a8,
        a9,
        a10,
        a11,
        a12,
        a13,
        a14,
        a15,
    );
}

/// `unknown_140588bd0`
/// Original: 0x140588bd0 (99 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140588bd0(param_1: usize) -> u8 {
    let neg_inf = rd::<f32>(exe::ptr::<f32>(0x1_4342_7178) as usize);
    if rd::<f32>(param_1) != neg_inf
        || rd::<f32>(param_1 + 0x04) != neg_inf
        || rd::<f32>(param_1 + 0x08) != neg_inf
    {
        return 0;
    }
    let pos_inf = rd::<f32>(exe::ptr::<f32>(0x1_4342_6fd4) as usize);
    if rd::<f32>(param_1 + 0x0c) != pos_inf
        || rd::<f32>(param_1 + 0x10) != pos_inf
        || rd::<f32>(param_1 + 0x14) != pos_inf
    {
        return 0;
    }
    1
}

/// `r_14059c600`
/// Original: 0x14059c600 (732 bytes, Throws::No)
pub unsafe extern "win64" fn r_14059c600(param_1: usize, param_2: usize, param_3: u16) {
    let base_p2 = rd::<usize>(param_2);
    let entry_off = (param_3 as usize).wrapping_mul(0x30);
    let count = rd::<u32>(base_p2.wrapping_add(entry_off + 0x08)) as usize;
    let mut ptr_r9 = rd::<usize>(base_p2.wrapping_add(entry_off));
    if count == 0 {
        return;
    }

    let mut ctx = [0u64; 11];
    exe::call_va!(0x1_4059_70d0, (ctx.as_mut_ptr() as usize, param_1));

    let r8_base = rd::<usize>(param_1).wrapping_add(0x40);
    let end_r9 = ptr_r9.wrapping_add(count * 2);
    while ptr_r9 != end_r9 {
        let idx = rd::<u16>(ptr_r9) as usize;
        let block_base = rd::<usize>(r8_base.wrapping_add(idx * 8 + 0x48));
        let sub_count = if block_base != 0 {
            rd::<u32>(r8_base.wrapping_add(idx * 4 + 0x1_0048)) as u64
        } else {
            0
        };
        if sub_count != 0 {
            let mut offs = [0u64; 12];
            exe::call_va!(
                0x1_4059_8290,
                (offs.as_mut_ptr() as usize, 0usize, r8_base, idx as u16)
            );
            for slot in offs.iter_mut().take(10) {
                if *slot != 0 {
                    *slot = (*slot).wrapping_add(block_base as u64);
                }
            }
            for sub_idx in 0..sub_count {
                let mut arg4_buf: [u64; 3] = [ctx[2], ctx[3], ctx[4]];
                let mut arg3_buf: [u64; 3] = [ctx[5], ctx[6], ctx[7]];
                let mut arg2_buf: [u64; 3] = [ctx[8], ctx[9], ctx[10]];
                let mut arg1_buf: [u64; 12] = [
                    offs[0],
                    offs[1],
                    offs[2],
                    offs[3],
                    offs[4],
                    offs[5],
                    offs[6],
                    offs[7],
                    offs[8],
                    offs[9],
                    block_base as u64,
                    sub_idx,
                ];
                exe::call_va!(
                    0x1_41ac_a620,
                    (
                        arg1_buf.as_mut_ptr() as usize,
                        arg2_buf.as_mut_ptr() as usize,
                        arg3_buf.as_mut_ptr() as usize,
                        arg4_buf.as_mut_ptr() as usize,
                        ctx[1],
                        ctx[0]
                    )
                );
            }
        }
        ptr_r9 = ptr_r9.wrapping_add(2);
    }
}

/// `unknown_1406da930`
/// Original: 0x1406da930 (197 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1406da930(param_1: usize, param_2: usize, param_3: i32) {
    if rd::<i32>(param_1 + 0x328) >= param_3 {
        let rbx = rd::<usize>(param_1 + 0x338);
        exe::call_va!(0x1_404b_9a10, (param_2));
        let fn0 = rd::<u64>(rd::<usize>(rbx));
        exe::call_va!(fn0, (rbx, param_2));
    } else if rd::<u8>(param_1 + 0x369) != 0 {
        exe::call_va!(0x1_404b_9a10, (param_2));
        wr::<u64>(param_2, 0);
        wr::<u32>(param_2 + 0x08, 0);
        wr::<u64>(param_2 + 0x10, 0);
    }
}

/// `unknown_1406e01b0`
/// Original: 0x1406e01b0 (242 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1406e01b0(
    param_1: usize,
    param_2: usize,
    _param_3: u64,
    param_4: u64,
) -> u64 {
    let name_ptr = rd::<usize>(exe::ptr::<usize>(0x1_4387_d9c8) as usize);
    exe::call_va!(
        0x1_4243_b0b0,
        (param_2, param_1, name_ptr, param_4, 5u32, 1u32)
    );

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(1) <= 2
    {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let rcx_sub = rd::<usize>(p320 + 0x08);
            let fn38 = rd::<u64>(rd::<usize>(rcx_sub) + 0x38);
            exe::call_va!(fn38, (rcx_sub, 8u32, 0u32));
        }
    }

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3 && rd::<u32>(param_2 + 0x328) == 2 {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let rcx_sub = rd::<usize>(p320 + 0x08);
            let fn38 = rd::<u64>(rd::<usize>(rcx_sub) + 0x38);
            exe::call_va!(fn38, (rcx_sub, 4u32, 0u32));
        }
    }

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(3) <= 1
    {
        let p320 = rd::<usize>(param_2 + 0x320);
        if p320 != 0 {
            let rcx_sub = rd::<usize>(p320 + 0x08);
            let fn38 = rd::<u64>(rd::<usize>(rcx_sub) + 0x38);
            exe::call_va!(fn38, (rcx_sub, 4u32, 0u32));
        }
    }

    let len = rd::<u32>(param_2 + 0x08);
    let arr = rd::<usize>(param_2);
    let prev_v = rd::<u32>(arr.wrapping_add((len as usize) * 0x30).wrapping_sub(0x08));
    wr::<u32>(param_2 + 0x08, len.wrapping_sub(1));
    wr::<u32>(param_2 + 0x328, prev_v);
    0
}

/// `unknown_1406ecae0`
/// Original: 0x1406ecae0 (260 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1406ecae0(
    param_1: usize,
    param_2: usize,
    param_3: i32,
    param_4: i32,
) {
    if param_3 <= 0 {
        return;
    }
    let mut rdi = param_2;
    for _ in 0..(param_3 as usize) {
        if rd::<i32>(param_1 + 0x328) >= param_4 {
            let rbx = rd::<usize>(param_1 + 0x338);
            exe::call_va!(0x1_404b_9a10, (rdi));
            let fn0 = rd::<u64>(rd::<usize>(rbx));
            exe::call_va!(fn0, (rbx, rdi));
        } else if rd::<u8>(param_1 + 0x369) != 0 {
            exe::call_va!(0x1_404b_9a10, (rdi));
            wr::<u64>(rdi, 0);
            wr::<u32>(rdi + 0x08, 0);
            wr::<u64>(rdi + 0x10, 0);
        }
        rdi = rdi.wrapping_add(0x20);
    }
}

/// `unknown_1408435b0`
/// Original: 0x1408435b0 (430 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1408435b0(param_1: usize, param_2: usize) -> u64 {
    let r8_name = rd::<usize>(exe::ptr::<usize>(0x1_4387_e8b0) as usize);
    let r9_id = rd::<u32>(exe::ptr::<u32>(0x1_4387_e820) as usize);
    exe::call_va!(
        0x1_401e_9fa0,
        (param_2, param_1, r8_name, r9_id, 2u32, 0u32)
    );

    if rd::<i32>(param_2 + 0x328) >= 0 {
        let rbx = rd::<usize>(param_2 + 0x338);
        let fn38_a = rd::<u64>(rd::<usize>(rbx) + 0x38);
        exe::call_va!(fn38_a, (rbx, param_1));
        let fn38_b = rd::<u64>(rd::<usize>(rbx) + 0x38);
        exe::call_va!(fn38_b, (rbx, param_1 + 0x04));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u64>(param_1, 0);
    }

    if rd::<i32>(param_2 + 0x328) >= 0 {
        let rbx = rd::<usize>(param_2 + 0x338);
        let fn38_a = rd::<u64>(rd::<usize>(rbx) + 0x38);
        exe::call_va!(fn38_a, (rbx, param_1 + 0x08));
        let fn38_b = rd::<u64>(rd::<usize>(rbx) + 0x38);
        exe::call_va!(fn38_b, (rbx, param_1 + 0x0c));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u64>(param_1 + 0x08, 0);
    }

    if rd::<i32>(param_2 + 0x328) >= 0 {
        let rbx = rd::<usize>(param_2 + 0x338);
        let fn38_a = rd::<u64>(rd::<usize>(rbx) + 0x38);
        exe::call_va!(fn38_a, (rbx, param_1 + 0x10));
        let fn38_b = rd::<u64>(rd::<usize>(rbx) + 0x38);
        exe::call_va!(fn38_b, (rbx, param_1 + 0x14));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u64>(param_1 + 0x10, 0);
    }

    if rd::<i32>(param_2 + 0x328) >= 0 {
        let rcx_s = rd::<usize>(param_2 + 0x338);
        let fn48 = rd::<u64>(rd::<usize>(rcx_s) + 0x48);
        exe::call_va!(fn48, (rcx_s, param_1 + 0x18));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x18, 0);
    }

    exe::call_va!(0x1_403c_79b0, (param_2, 1i32, 2i32));

    if rd::<i32>(param_2 + 0x328) >= 2 {
        let rcx_s = rd::<usize>(param_2 + 0x338);
        let fn48 = rd::<u64>(rd::<usize>(rcx_s) + 0x48);
        exe::call_va!(fn48, (rcx_s, param_1 + 0x1c));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x1c, 0);
    }

    exe::call_va!(0x1_401e_9e70, (param_2));
    0
}

/// `content_14086b290`
/// Original: 0x14086b290 (248 bytes, Throws::No)
pub unsafe extern "win64" fn content_14086b290(param_1: usize, param_2: u64) {
    let old_len = rd::<u32>(param_1 + 0x08) as u64;
    let cap = rd::<u32>(param_1 + 0x0c) as u64;
    if param_2 > cap {
        let mut new_cap = (cap >> 1).wrapping_add(cap);
        if new_cap < param_2 {
            new_cap = param_2;
        }
        exe::call_va!(0x1_4087_4640, (param_1, new_cap));
        let mut count = param_2.wrapping_sub(old_len);
        if count != 0 {
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let mut dst = rd::<usize>(param_1).wrapping_add(cur_len * 0x0c);
            while count != 0 {
                wr::<u64>(dst, 0);
                wr::<u32>(dst + 0x08, 0);
                dst = dst.wrapping_add(0x0c);
                count = count.wrapping_sub(1);
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 > old_len {
        let mut count = param_2.wrapping_sub(old_len);
        let mut dst = rd::<usize>(param_1).wrapping_add((old_len as usize) * 0x0c);
        while count != 0 {
            wr::<u64>(dst, 0);
            wr::<u32>(dst + 0x08, 0);
            dst = dst.wrapping_add(0x0c);
            count = count.wrapping_sub(1);
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 != old_len {
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    }
}

/// `content_140943c70`
/// Original: 0x140943c70 (307 bytes, Throws::No)
pub unsafe extern "win64" fn content_140943c70(param_1: usize, param_2: u64) {
    let old_len = rd::<u32>(param_1 + 0x08) as u64;
    let old_bytes = (old_len as usize) * 0x18;
    let cap = rd::<u32>(param_1 + 0x0c) as u64;
    if param_2 > cap {
        let mut new_cap = (cap >> 1).wrapping_add(cap);
        if new_cap < param_2 {
            new_cap = param_2;
        }
        exe::call_va!(0x1_4094_baa0, (param_1, new_cap));
        let mut count = param_2.wrapping_sub(old_len);
        if count != 0 {
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let mut dst = rd::<usize>(param_1).wrapping_add(cur_len * 0x18);
            while count != 0 {
                wr::<u64>(dst, 0);
                wr::<u64>(dst + 0x08, 0);
                wr::<u32>(dst + 0x10, 0);
                wr::<u8>(dst + 0x14, 0);
                dst = dst.wrapping_add(0x18);
                count = count.wrapping_sub(1);
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 > old_len {
        let mut count = param_2.wrapping_sub(old_len);
        let mut dst = rd::<usize>(param_1).wrapping_add(old_bytes);
        while count != 0 {
            wr::<u64>(dst, 0);
            wr::<u64>(dst + 0x08, 0);
            wr::<u32>(dst + 0x10, 0);
            wr::<u8>(dst + 0x14, 0);
            dst = dst.wrapping_add(0x18);
            count = count.wrapping_sub(1);
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 != old_len {
        let base = rd::<usize>(param_1);
        let end = base.wrapping_add(old_bytes);
        let mut cur = base.wrapping_add((param_2 as usize) * 0x18);
        while cur != end {
            let _ = crate::simd_ops::atomic_xchg_u64((cur + 0x08) as u64, 0);
            cur = cur.wrapping_add(0x18);
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    }
}

/// `content_1409469e0`
/// Original: 0x1409469e0 (173 bytes, Throws::Discard)
pub unsafe extern "win64" fn content_1409469e0(
    param_1: usize,
    param_2: usize,
    _param_3: u8,
) -> u64 {
    let r8_name = rd::<usize>(exe::ptr::<usize>(0x1_4388_2038) as usize);
    let r9_id = rd::<u32>(exe::ptr::<u32>(0x1_4388_2040) as usize);
    exe::call_va!(
        0x1_401e_9fa0,
        (param_2, param_1, r8_name, r9_id, 0u32, 0u32)
    );
    exe::call_va!(0x1_406d_a930, (param_2, param_1 + 0x08, 0i32));
    let mut count: i32 = 0;
    if rd::<i32>(param_2 + 0x328) >= 0 {
        let rcx_s = rd::<usize>(param_2 + 0x338);
        let fn48 = rd::<u64>(rd::<usize>(rcx_s) + 0x48);
        exe::call_va!(fn48, (rcx_s, (&mut count as *mut i32) as usize));
    }
    exe::call_va!(0x1_4094_b880, (param_1 + 0x28, count as i64 as u64));
    if count > 0 {
        let arr = rd::<usize>(param_1 + 0x28);
        exe::call_va!(0x1_406e_cae0, (param_2, arr, count, 0i32));
    }
    exe::call_va!(0x1_401e_9e70, (param_2));
    0
}

/// `content_14094b880`
/// Original: 0x14094b880 (114 bytes, Throws::No)
pub unsafe extern "win64" fn content_14094b880(param_1: usize, param_2: u64) {
    let old_len = rd::<u32>(param_1 + 0x08) as u64;
    if param_2 > old_len {
        exe::call_va!(0x1_4094_38d0, (param_1, param_2));
        let cur_len = rd::<u32>(param_1 + 0x08) as usize;
        let base = rd::<usize>(param_1);
        let end = base.wrapping_add((param_2 as usize) << 5);
        let mut cur = base.wrapping_add(cur_len << 5);
        while cur != end {
            wr::<u64>(cur, 0);
            wr::<u32>(cur + 0x08, 0);
            wr::<u64>(cur + 0x10, 0);
            wr::<u8>(cur + 0x18, 0);
            cur = cur.wrapping_add(0x20);
        }
    }
    wr::<u32>(param_1 + 0x08, param_2 as u32);
}

/// `content_14094baa0`
/// Original: 0x14094baa0 (376 bytes, Throws::No)
pub unsafe extern "win64" fn content_14094baa0(param_1: usize, param_2: u64) {
    let saved_len = rd::<u32>(param_1 + 0x08);
    let byte_size = param_2.wrapping_mul(0x18);
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);

    let mut new_buf: usize =
        if (byte_size.wrapping_sub(1) & byte_size) == 0 && byte_size.wrapping_sub(0x10) <= 0x3f0 {
            let page = rd::<usize>(heap.wrapping_add(((byte_size >> 3) as usize) * 8 + 0x08));
            let blk = rd::<usize>(page + 0x10);
            if blk == 0 {
                exe::call_va!(0x1_4253_7280, (heap, byte_size, 0u32, 0u32) -> usize)
            } else {
                wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
                wr::<u64>(page + 0x10, rd::<u64>(blk));
                blk
            }
        } else if byte_size <= 0x7fff_ffff_ffff_ffff {
            let mut got = 0usize;
            if byte_size.wrapping_sub(0x10) <= 0x3f0 {
                let page = rd::<usize>(heap.wrapping_add(((byte_size >> 3) as usize) * 8 + 0x08));
                let blk = rd::<usize>(page + 0x10);
                if blk != 0 && (blk & 0xf) == 0 {
                    wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
                    wr::<u64>(page + 0x10, rd::<u64>(blk));
                    got = blk;
                }
            }
            if got != 0 {
                got
            } else {
                exe::call_va!(0x1_4253_5070, (heap, byte_size, 0x10u32) -> usize)
            }
        } else {
            0
        };

    if new_buf == 0 {
        let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
        exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
        new_buf = 0;
    }

    let mut src = rd::<usize>(param_1);
    let cur_len = rd::<u32>(param_1 + 0x08) as usize;
    let end = src.wrapping_add(cur_len * 0x18);
    if src != end {
        let mut dst = new_buf;
        while src != end {
            wr::<u64>(dst, 0);
            wr::<u64>(dst + 0x08, 0);
            wr::<u32>(dst + 0x10, 0);
            wr::<u8>(dst + 0x14, 0);
            exe::call_va!(0x1_401e_84a0, (dst, src));
            src = src.wrapping_add(0x18);
            dst = dst.wrapping_add(0x18);
        }
    }
    exe::call_va!(0x1_4023_6fa0, (param_1));
    wr::<usize>(param_1, new_buf);
    wr::<u32>(param_1 + 0x08, saved_len);
    wr::<u32>(param_1 + 0x0c, param_2 as u32);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1403c79b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_38: Vec<(i64, u32)> = Vec::new();
    static mut DTOR_HDR: Vec<u64> = Vec::new();

    unsafe extern "win64" fn vfn_48(_this: usize, out_i32: usize) {
        wr::<i32>(out_i32, 42);
    }
    unsafe extern "win64" fn vfn_38(_this: usize, val: i64, flag: u32) {
        LOG_38.push((val, flag));
    }
    unsafe extern "win64" fn mock_ab10(sbp: usize) {
        DTOR_HDR.push(rd::<u64>(sbp));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let mut vtbl0 = [0usize; 12];
    vtbl0[0x48 / 8] = vfn_48 as *const () as usize;
    let mut vtbl1 = [0usize; 10];
    vtbl1[0x38 / 8] = vfn_38 as *const () as usize;

    let run = |f: F, m318: u32, v328: i32, has_320: bool| -> (Vec<(i64, u32)>, Vec<u64>) {
        LOG_38.clear();
        DTOR_HDR.clear();
        let mut p1 = vec![0u8; 0x380];
        let mut sub_obj: [usize; 1] = [vtbl1.as_ptr() as usize];
        let mut rbx_obj: [usize; 2] = [vtbl0.as_ptr() as usize, sub_obj.as_mut_ptr() as usize];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x318, m318);
        wr::<i32>(p1p + 0x328, v328);
        if has_320 {
            wr::<usize>(p1p + 0x320, rbx_obj.as_mut_ptr() as usize);
        }
        f(p1p, 2, 5);
        (LOG_38.clone(), DTOR_HDR.clone())
    };

    for (m318, v328, has_320) in [
        (2u32, 3i32, true),
        (5, 1, true),
        (5, 5, true),
        (5, 3, false),
        (5, 3, true),
    ] {
        let o = run(f_o, m318, v328, has_320);
        let n = run(f_n, m318, v328, has_320);
        crate::diff_eq!(o, n, "1403c79b0 m318={} v328={} h={}", m318, v328, has_320);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1403d17d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, key: u64| -> (bool, u64) {
        let mut r10_obj = [0u8; 0x40];
        let arr_18: [u64; 4] = [
            0x0001_0000_0000_0000,
            0x0002_0000_0000_0001,
            0x0003_0000_0000_0002,
            0x0004_0000_0000_0003,
        ];
        let arr_30: [u64; 4] = [10, 25, 40, 60];
        let sorted_u16: [u16; 5] = [5, 10, 30, 40, 100];

        let r10p = r10_obj.as_mut_ptr() as usize;
        wr::<usize>(r10p + 0x18, arr_18.as_ptr() as usize);
        wr::<u32>(r10p + 0x20, 4);
        wr::<usize>(r10p + 0x30, arr_30.as_ptr() as usize);

        let p3: [usize; 3] = [r10p, sorted_u16.len(), sorted_u16.as_ptr() as usize];
        let mut out_val: u64 = 0x1234;
        let out_ptr = (&mut out_val as *mut u64) as usize;
        let ret = f(out_ptr, key, p3.as_ptr() as usize);
        (ret == out_ptr, out_val)
    };

    for key in [
        0x0001_0000_0000_0000u64,
        0x0002_0000_0000_0001,
        0x0003_0000_0000_0002,
        0x0009_0000_0000_0002,
        0x0001_0000_0000_0009,
    ] {
        let o = run(f_o, key);
        let n = run(f_n, key);
        crate::diff_eq!(o, n, "1403d17d0 key={:#x}", key);
    }
    Ok(5)
}

pub unsafe fn diff_content_1403f4a80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut MODE_CC90: i32 = 1;
    static mut GC_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_9670(_l: usize, _idx: u64, _k: usize) -> i32 {
        5
    }
    unsafe extern "win64" fn mock_cc90(_l: usize) -> i32 {
        MODE_CC90
    }
    unsafe extern "win64" fn mock_c770(_l: usize) {
        GC_CALLED += 1;
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, view: usize, _pos: u64, count: u64) {
        let src = rd::<usize>(view);
        wr::<u32>(dst + 0x04, count as u32);
        for i in 0..(count as usize) {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(src + i));
        }
        wr::<u8>(dst + 0x08 + count as usize, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4211_9670, mock_9670 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_420e_cc90, mock_cc90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4210_c770, mock_c770 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;

    let run = |f: F, mode: u8| -> (usize, u8, u32, u8, u32) {
        GC_CALLED = 0;
        MODE_CC90 = i32::from(mode != 2);
        let mut l_state = [0u8; 0x80];
        let mut g_state = [0u8; 0x60];
        let mut stk = [0u8; 0x40];
        let mut tstr = [0u8; 0x30];
        let mut out_str = [0u8; 0x30];

        let lp = l_state.as_mut_ptr() as usize;
        let gp = g_state.as_mut_ptr() as usize;
        let sp = stk.as_mut_ptr() as usize;
        let tp = tstr.as_mut_ptr() as usize;
        let op = out_str.as_mut_ptr() as usize;

        for (i, &b) in b"field_val\0".iter().enumerate() {
            wr::<u8>(tp + 0x18 + i, b);
        }

        wr::<u32>(op, 0x0400_0004);
        wr::<u32>(op + 0x04, 3);
        wr::<u8>(op + 0x08, b'z');

        wr::<u8>(lp + 0x01, 4);
        wr::<usize>(lp + 0x08, sp + 0x30);
        wr::<usize>(lp + 0x18, gp);
        wr::<u64>(gp + 0x40, 100);
        wr::<u64>(gp + 0x48, 200);

        let slot = sp + 0x18;
        wr::<usize>(slot, tp);
        wr::<i32>(slot + 0x10, if mode == 0 { 5 } else { 3 });

        f(lp, 1, 0x1234, op);
        (
            rd::<usize>(lp + 0x08).wrapping_sub(sp),
            rd::<u8>(lp + 0x01),
            rd::<u32>(op + 0x04),
            rd::<u8>(op + 0x08),
            GC_CALLED,
        )
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1403f4a80 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1403fec40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DST_BUF: [u8; 0x180] = [0; 0x180];
    static mut FREED_CNT: u32 = 0;

    unsafe extern "win64" fn mock_a9e0(_p1: usize, _cap: u64) -> usize {
        core::ptr::addr_of_mut!(DST_BUF) as usize
    }
    unsafe extern "win64" fn mock_e830(_p1: usize) {
        FREED_CNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4024_a9e0, mock_a9e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F, req: u64| -> (bool, u32, u32, u32, Vec<u8>) {
        DST_BUF = [0u8; 0x180];
        FREED_CNT = 0;
        let mut src = vec![0u8; 0x90];
        for (i, b) in src.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }
        let mut vec_hdr = [0u64; 2];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, src.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, 4);

        f(vp, req);
        (
            rd::<usize>(vp) == core::ptr::addr_of!(DST_BUF) as usize,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            FREED_CNT,
            DST_BUF[..0x60].to_vec(),
        )
    };

    for req in [5u64, 10u64] {
        let o = run(f_o, req);
        let n = run(f_n, req);
        crate::diff_eq!(o, n, "1403fec40 req={}", req);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140455cd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: Vec<u64> = Vec::new();

    #[allow(clippy::too_many_arguments)]
    unsafe extern "win64" fn mock_eef0(
        a1: usize,
        a2: usize,
        a3: usize,
        a4: u64,
        a5: u64,
        a6: u64,
        a7: u64,
        a8: u64,
        a9: u64,
        a10: u64,
        a11: u64,
        a12: u64,
        a13: u64,
        a14: u64,
        a15: u64,
    ) {
        CAPTURED.extend_from_slice(&[
            rd::<u64>(a1 + 0x08),
            rd::<u64>(a1 + 0x10),
            rd::<u64>(a2 + 0x08),
            rd::<u64>(a2 + 0x10),
            rd::<u64>(a3 + 0x08),
            rd::<u64>(a3 + 0x10),
            a4,
            a5,
            a6,
            a7,
            a8,
            a9,
            a10,
            a11,
            a12,
            a13,
            a14,
            a15,
        ]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_419c_eef0, mock_eef0 as *const ())?;

    let run = |f: F| -> Vec<u64> {
        CAPTURED.clear();
        let mut r12_0 = vec![0u8; 0x58100];
        let mut vtbl = [0usize; 8];
        let mut rdx_tbl = vec![0u16; 0x10000];
        let mut r8_arr = vec![0u64; 32];
        let mut r12_arr = vec![0u64; 0x10000];

        for (i, slot) in rdx_tbl.iter_mut().enumerate() {
            *slot = (i & 7) as u16;
        }
        for i in 0..8 {
            r8_arr[i * 2] = 0x1000 + (i as u64) * 0x100;
            r8_arr[i * 2 + 1] = 10 + i as u64;
        }
        for (i, slot) in r12_arr.iter_mut().enumerate() {
            *slot = 0x9000_0000 + i as u64;
        }

        let r12p = r12_0.as_mut_ptr() as usize;
        vtbl[4] = rdx_tbl.as_mut_ptr() as usize;
        wr::<usize>(r12p, vtbl.as_mut_ptr() as usize);
        wr::<usize>(r12p + 0x08, r8_arr.as_mut_ptr() as usize);
        wr::<usize>(r12p + 0x580e0, r12_arr.as_mut_ptr() as usize);

        let p1: [usize; 1] = [r12p];
        f(p1.as_ptr() as usize);
        CAPTURED.clone()
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "140455cd0");
    Ok(1)
}

pub unsafe fn diff_unknown_140588bd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let ni = f32::NEG_INFINITY;
    let pi = f32::INFINITY;
    let cases: [[f32; 6]; 4] = [
        [ni, ni, ni, pi, pi, pi],
        [0.0, ni, ni, pi, pi, pi],
        [ni, ni, ni, pi, 1.0, pi],
        [ni, ni, f32::NAN, pi, pi, pi],
    ];

    for (idx, c) in cases.iter().enumerate() {
        let p = c.as_ptr() as usize;
        let o = f_o(p);
        let n = f_n(p);
        crate::diff_eq!(o, n, "140588bd0 idx={}", idx);
    }
    Ok(4)
}

pub unsafe fn diff_r_14059c600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_A620: Vec<([u64; 12], [u64; 3], [u64; 3], [u64; 3], u64, u64)> = Vec::new();

    unsafe extern "win64" fn mock_70d0(ctx: usize, _p1: usize) {
        for i in 0..11usize {
            wr::<u64>(ctx + i * 8, 0x100 + i as u64);
        }
    }
    unsafe extern "win64" fn mock_8290(offs: usize) {
        for i in 0..10usize {
            wr::<u64>(offs + i * 8, if i % 2 == 0 { (i as u64 + 1) * 0x10 } else { 0 });
        }
    }
    unsafe extern "win64" fn mock_a620(
        a1: usize,
        a2: usize,
        a3: usize,
        a4: usize,
        a5: u64,
        a6: u64,
    ) {
        LOG_A620.push((
            rd::<[u64; 12]>(a1),
            rd::<[u64; 3]>(a2),
            rd::<[u64; 3]>(a3),
            rd::<[u64; 3]>(a4),
            a5,
            a6,
        ));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4059_70d0, mock_70d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4059_8290, mock_8290 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41ac_a620, mock_a620 as *const ())?;

    let run = |f: F, idx_arg: u16| -> Vec<([u64; 12], [u64; 3], [u64; 3], [u64; 3], u64, u64)> {
        LOG_A620.clear();
        let mut p1_obj = vec![0u8; 0x10200];
        let p1_base = p1_obj.as_mut_ptr() as usize;
        wr::<usize>(p1_base + 0x88 + 2 * 8, 0x5000_0000);
        wr::<u32>(p1_base + 0x1_0088 + 2 * 4, 2);

        let indices: [u16; 2] = [1, 2];
        let mut p2_table = [0u8; 0x90];
        let p2_base = p2_table.as_mut_ptr() as usize;
        wr::<usize>(p2_base + 0x30, indices.as_ptr() as usize);
        wr::<u32>(p2_base + 0x38, 2);

        let p1_ptr: [usize; 1] = [p1_base];
        let p2_ptr: [usize; 1] = [p2_base];
        f(p1_ptr.as_ptr() as usize, p2_ptr.as_ptr() as usize, idx_arg);
        LOG_A620.clone()
    };

    for idx_arg in [0u16, 1u16] {
        let o = run(f_o, idx_arg);
        let n = run(f_n, idx_arg);
        crate::diff_eq!(o, n, "14059c600 idx={}", idx_arg);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1406da930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VFN0_CALLED: u32 = 0;
    static mut CLEAN_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_9a10(_p2: usize) {
        CLEAN_CALLED += 1;
    }
    unsafe extern "win64" fn vfn_0(_this: usize, p2: usize) {
        VFN0_CALLED += 1;
        wr::<u64>(p2, 0x7777);
    }

    let _m1 = crate::difftest::Mock::new(0x1_404b_9a10, mock_9a10 as *const ())?;
    let vtbl: [usize; 1] = [vfn_0 as *const () as usize];

    let run = |f: F, v328: i32, b369: u8| -> ([u64; 3], u32, u32) {
        VFN0_CALLED = 0;
        CLEAN_CALLED = 0;
        let mut p1 = vec![0u8; 0x380];
        let mut p2: [u64; 4] = [0xaaaa, 0xbbbb, 0xcccc, 0];
        let mut sub_obj: [usize; 1] = [vtbl.as_ptr() as usize];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<i32>(p1p + 0x328, v328);
        wr::<usize>(p1p + 0x338, sub_obj.as_mut_ptr() as usize);
        wr::<u8>(p1p + 0x369, b369);

        f(p1p, p2.as_mut_ptr() as usize, 2);
        ([p2[0], p2[1], p2[2]], VFN0_CALLED, CLEAN_CALLED)
    };

    for (v328, b369) in [(3i32, 0u8), (1, 0), (1, 1)] {
        let o = run(f_o, v328, b369);
        let n = run(f_n, v328, b369);
        crate::diff_eq!(o, n, "1406da930 v328={} b369={}", v328, b369);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1406e01b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_38: Vec<(u32, u32)> = Vec::new();

    unsafe extern "win64" fn mock_b0b0(
        _p2: usize,
        _p1: usize,
        _name: usize,
        _p4: u64,
        _a5: u32,
        _a6: u32,
    ) {
    }
    unsafe extern "win64" fn vfn_38(_this: usize, a2: u32, a3: u32) {
        LOG_38.push((a2, a3));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_b0b0, mock_b0b0 as *const ())?;
    let mut vtbl = [0usize; 8];
    vtbl[0x38 / 8] = vfn_38 as *const () as usize;

    let run = |f: F, v328: u32| -> (u64, u32, u32, Vec<(u32, u32)>) {
        LOG_38.clear();
        let mut p2 = vec![0u8; 0x380];
        let mut arr = vec![0u8; 0x60];
        let mut sub_obj: [usize; 1] = [vtbl.as_ptr() as usize];
        let mut p320_obj: [usize; 2] = [0, sub_obj.as_mut_ptr() as usize];

        let p2p = p2.as_mut_ptr() as usize;
        let ap = arr.as_mut_ptr() as usize;
        wr::<u32>(ap + 0x30 - 0x08, 99);
        wr::<usize>(p2p, ap);
        wr::<u32>(p2p + 0x08, 1);
        wr::<u32>(p2p + 0x318, 5);
        wr::<usize>(p2p + 0x320, p320_obj.as_mut_ptr() as usize);
        wr::<u32>(p2p + 0x328, v328);

        let ret = f(0x1111, p2p, 0, 0x2222);
        (
            ret,
            rd::<u32>(p2p + 0x08),
            rd::<u32>(p2p + 0x328),
            LOG_38.clone(),
        )
    };

    for v328 in [1u32, 2, 3, 4, 5] {
        let o = run(f_o, v328);
        let n = run(f_n, v328);
        crate::diff_eq!(o, n, "1406e01b0 v328={}", v328);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1406ecae0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32, i32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VFN0_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_9a10(_rdi: usize) {}
    unsafe extern "win64" fn vfn_0(_this: usize, rdi: usize) {
        VFN0_CALLED += 1;
        wr::<u64>(rdi, 0x5555);
    }

    let _m1 = crate::difftest::Mock::new(0x1_404b_9a10, mock_9a10 as *const ())?;
    let vtbl: [usize; 1] = [vfn_0 as *const () as usize];

    let run = |f: F, count: i32, v328: i32, b369: u8| -> ([u64; 8], u32) {
        VFN0_CALLED = 0;
        let mut p1 = vec![0u8; 0x380];
        let mut items: [u64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut sub_obj: [usize; 1] = [vtbl.as_ptr() as usize];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<i32>(p1p + 0x328, v328);
        wr::<usize>(p1p + 0x338, sub_obj.as_mut_ptr() as usize);
        wr::<u8>(p1p + 0x369, b369);

        f(p1p, items.as_mut_ptr() as usize, count, 2);
        (items, VFN0_CALLED)
    };

    for (count, v328, b369) in [(0i32, 3i32, 1u8), (2, 3, 0), (2, 1, 1)] {
        let o = run(f_o, count, v328, b369);
        let n = run(f_n, count, v328, b369);
        crate::diff_eq!(o, n, "1406ecae0 c={} v328={}", count, v328);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1408435b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SEQ: u32 = 0;

    unsafe extern "win64" fn mock_9fa0(
        _p2: usize,
        _p1: usize,
        _n: usize,
        _id: u32,
        _a5: u32,
        _a6: u32,
    ) {
    }
    unsafe extern "win64" fn mock_79b0(_p2: usize, _a2: i32, _a3: i32) {}
    unsafe extern "win64" fn mock_9e70(_p2: usize) {}
    unsafe extern "win64" fn vfn_38(_this: usize, dst: usize) {
        SEQ += 1;
        wr::<u32>(dst, 0x100 + SEQ);
    }
    unsafe extern "win64" fn vfn_48(_this: usize, dst: usize) {
        SEQ += 1;
        wr::<u32>(dst, 0x200 + SEQ);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9fa0, mock_9fa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_403c_79b0, mock_79b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_9e70, mock_9e70 as *const ())?;

    let mut vtbl = [0usize; 10];
    vtbl[0x38 / 8] = vfn_38 as *const () as usize;
    vtbl[0x48 / 8] = vfn_48 as *const () as usize;

    let run = |f: F, v328: i32, b369: u8| -> (u64, [u32; 8]) {
        SEQ = 0;
        let mut p1: [u32; 8] = [9; 8];
        let mut p2 = vec![0u8; 0x380];
        let mut sub_obj: [usize; 1] = [vtbl.as_ptr() as usize];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x328, v328);
        wr::<usize>(p2p + 0x338, sub_obj.as_mut_ptr() as usize);
        wr::<u8>(p2p + 0x369, b369);

        let ret = f(p1.as_mut_ptr() as usize, p2p);
        (ret, p1)
    };

    for (v328, b369) in [(-1i32, 1u8), (1, 1), (2, 0)] {
        let o = run(f_o, v328, b369);
        let n = run(f_n, v328, b369);
        crate::diff_eq!(o, n, "1408435b0 v328={} b369={}", v328, b369);
    }
    Ok(3)
}

pub unsafe fn diff_content_14086b290(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_4640(p1: usize, new_cap: u64) {
        wr::<u32>(p1 + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4087_4640, mock_4640 as *const ())?;

    let run = |f: F, new_len: u64| -> (u32, u32, Vec<u8>) {
        let mut buf = vec![0xffu8; 0x60];
        let mut hdr = [0u64; 2];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp, buf.as_mut_ptr() as usize);
        wr::<u32>(hp + 0x08, 2);
        wr::<u32>(hp + 0x0c, 4);
        f(hp, new_len);
        (rd::<u32>(hp + 0x08), rd::<u32>(hp + 0x0c), buf)
    };

    for new_len in [1u64, 2, 3, 6] {
        let o = run(f_o, new_len);
        let n = run(f_n, new_len);
        crate::diff_eq!(o, n, "14086b290 new_len={}", new_len);
    }
    Ok(4)
}

pub unsafe fn diff_content_140943c70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_baa0(p1: usize, new_cap: u64) {
        wr::<u32>(p1 + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4094_baa0, mock_baa0 as *const ())?;

    let run = |f: F, new_len: u64| -> (u32, u32, Vec<u8>) {
        let mut buf = vec![0xffu8; 0x90];
        let mut hdr = [0u64; 2];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp, buf.as_mut_ptr() as usize);
        wr::<u32>(hp + 0x08, 2);
        wr::<u32>(hp + 0x0c, 3);
        f(hp, new_len);
        (rd::<u32>(hp + 0x08), rd::<u32>(hp + 0x0c), buf)
    };

    for new_len in [1u64, 2, 3, 5] {
        let o = run(f_o, new_len);
        let n = run(f_n, new_len);
        crate::diff_eq!(o, n, "140943c70 new_len={}", new_len);
    }
    Ok(4)
}

pub unsafe fn diff_content_1409469e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_CALLS: Vec<(u32, i64)> = Vec::new();

    unsafe extern "win64" fn mock_9fa0(
        _p2: usize,
        _p1: usize,
        _n: usize,
        _id: u32,
        _a5: u32,
        _a6: u32,
    ) {
        LOG_CALLS.push((1, 0));
    }
    unsafe extern "win64" fn mock_a930(_p2: usize, _p1_8: usize, a3: i32) {
        LOG_CALLS.push((2, a3 as i64));
    }
    unsafe extern "win64" fn mock_b880(_vec: usize, len: i64) {
        LOG_CALLS.push((3, len));
    }
    unsafe extern "win64" fn mock_cae0(_p2: usize, _arr: usize, count: i32, a4: i32) {
        LOG_CALLS.push((4, (count * 10 + a4) as i64));
    }
    unsafe extern "win64" fn mock_9e70(_p2: usize) {
        LOG_CALLS.push((5, 0));
    }
    unsafe extern "win64" fn vfn_48(_this: usize, out_cnt: usize) {
        wr::<i32>(out_cnt, 3);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9fa0, mock_9fa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_406d_a930, mock_a930 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4094_b880, mock_b880 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_406e_cae0, mock_cae0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401e_9e70, mock_9e70 as *const ())?;

    let mut vtbl = [0usize; 10];
    vtbl[0x48 / 8] = vfn_48 as *const () as usize;

    let run = |f: F, v328: i32| -> (u64, Vec<(u32, i64)>) {
        LOG_CALLS.clear();
        let mut p1 = [0u8; 0x40];
        let mut p2 = vec![0u8; 0x380];
        let mut sub_obj: [usize; 1] = [vtbl.as_ptr() as usize];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x328, v328);
        wr::<usize>(p2p + 0x338, sub_obj.as_mut_ptr() as usize);
        let ret = f(p1.as_mut_ptr() as usize, p2p, 1);
        (ret, LOG_CALLS.clone())
    };

    for v328 in [-1i32, 0i32] {
        let o = run(f_o, v328);
        let n = run(f_n, v328);
        crate::diff_eq!(o, n, "1409469e0 v328={}", v328);
    }
    Ok(2)
}

pub unsafe fn diff_content_14094b880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RESERVED_CAP: u64 = 0;

    unsafe extern "win64" fn mock_38d0(_p1: usize, cap: u64) {
        RESERVED_CAP = cap;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4094_38d0, mock_38d0 as *const ())?;

    let run = |f: F, new_len: u64| -> (u32, u64, Vec<u8>) {
        RESERVED_CAP = 0;
        let mut buf = vec![0xffu8; 0x80];
        let mut hdr = [0u64; 2];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp, buf.as_mut_ptr() as usize);
        wr::<u32>(hp + 0x08, 1);
        wr::<u32>(hp + 0x0c, 4);
        f(hp, new_len);
        (rd::<u32>(hp + 0x08), RESERVED_CAP, buf)
    };

    for new_len in [1u64, 3u64] {
        let o = run(f_o, new_len);
        let n = run(f_n, new_len);
        crate::diff_eq!(o, n, "14094b880 new_len={}", new_len);
    }
    Ok(2)
}

pub unsafe fn diff_content_14094baa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    #[repr(align(16))]
    struct AlignedPool([u8; 0x80]);
    static mut POOL_BLK: AlignedPool = AlignedPool([0; 0x80]);
    static mut FALLBACK_BLK: AlignedPool = AlignedPool([0; 0x80]);
    static mut FREED_OLD: u32 = 0;

    unsafe extern "win64" fn mock_5070(_heap: usize, _sz: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(FALLBACK_BLK.0) as usize
    }
    unsafe extern "win64" fn mock_84a0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src).wrapping_add(0x100));
        wr::<u64>(dst + 0x08, rd::<u64>(src + 0x08).wrapping_add(0x200));
    }
    unsafe extern "win64" fn mock_6fa0(_p1: usize) {
        FREED_OLD += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4023_6fa0, mock_6fa0 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, has_free: bool| -> (bool, u32, u32, u32, u64, u32, [u64; 4]) {
        POOL_BLK.0 = [0u8; 0x80];
        FALLBACK_BLK.0 = [0u8; 0x80];
        FREED_OLD = 0;

        let pool_ptr = core::ptr::addr_of_mut!(POOL_BLK.0) as usize;
        wr::<u64>(pool_ptr, 0xdead_0000);

        let mut page = [0u8; 0x30];
        let pp = page.as_mut_ptr() as usize;
        if has_free {
            wr::<usize>(pp + 0x10, pool_ptr);
        }
        wr::<u32>(pp + 0x18, 7);

        let mut heap = [0usize; 0x90];
        for slot in &mut heap {
            *slot = pp;
        }
        wr::<usize>(tls0 + 0x6348, heap.as_mut_ptr() as usize);

        let mut old_items: [u64; 6] = [1, 2, 3, 4, 5, 6];
        let mut vec_hdr = [0u64; 2];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, old_items.as_mut_ptr() as usize);
        wr::<u32>(vp + 0x08, 2);
        wr::<u32>(vp + 0x0c, 2);

        f(vp, 3);
        let new_ptr = rd::<usize>(vp);
        let used_pool = new_ptr == pool_ptr;
        let copied = [
            rd::<u64>(new_ptr),
            rd::<u64>(new_ptr + 0x08),
            rd::<u64>(new_ptr + 0x18),
            rd::<u64>(new_ptr + 0x20),
        ];
        (
            used_pool,
            rd::<u32>(vp + 0x08),
            rd::<u32>(vp + 0x0c),
            rd::<u32>(pp + 0x18),
            rd::<u64>(pp + 0x10),
            FREED_OLD,
            copied,
        )
    };

    for has_free in [true, false] {
        let o = run(f_o, has_free);
        let n = run(f_n, has_free);
        wr::<usize>(tls0 + 0x6348, saved_heap);
        crate::diff_eq!(o, n, "14094baa0 has_free={}", has_free);
    }
    Ok(2)
}
