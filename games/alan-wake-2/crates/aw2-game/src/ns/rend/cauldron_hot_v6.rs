//! Decompiled hot functions shard V6 (`0x14220ea10..0x14237fff0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
unsafe fn call_log2f(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    {
        let f: unsafe extern "win64" fn(f32) -> f32 = exe::func(0x1_4254_76d0);
        f(x)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let (_, x0) = crate::simd_ops::call_va8_xmm(
            0x1_4254_76d0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [x.to_bits() as u128, 0, 0, 0],
        );
        f32::from_bits(x0 as u32)
    }
}

/// `unknown_14220ea10`
/// Original: 0x14220ea10 (66 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14220ea10(param_1: usize) -> u32 {
    let mut len: usize = 0;
    while rd::<u8>(param_1.wrapping_add(len)) != 0 {
        len = len.wrapping_add(1);
    }
    let mut desc: [u64; 2] = [param_1 as u64, len as u64];
    let mut out: u64 = 0;
    exe::call_va!(
        0x1_4220_ea70,
        (
            (&mut out as *mut u64) as usize,
            desc.as_mut_ptr() as usize
        )
    );
    out as u32
}

/// `unknown_142229a00`
/// Original: 0x142229a00 (145 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142229a00(param_1: usize, param_2: u32) -> u32 {
    loop {
        let count = rd::<u32>(param_1 + 0x10) as usize;
        if count != 0 {
            let mut cur = rd::<usize>(param_1 + 0x08);
            for idx in 0..count {
                let avail = rd::<u32>(cur + 0x04);
                if avail >= param_2 {
                    let start = rd::<u32>(cur);
                    let rem = avail.wrapping_sub(param_2);
                    wr::<u32>(cur + 0x04, rem);
                    wr::<u32>(cur, start.wrapping_add(param_2));
                    if rem == 0 {
                        let base = rd::<usize>(param_1 + 0x08);
                        let last_idx = (rd::<u32>(param_1 + 0x10) as usize).wrapping_sub(1);
                        let dst = base.wrapping_add(idx * 8);
                        let src = base.wrapping_add(last_idx * 8);
                        if dst != src {
                            wr::<u64>(dst, rd::<u64>(src));
                        }
                        wr::<u32>(param_1 + 0x10, rd::<u32>(param_1 + 0x10).wrapping_sub(1));
                    }
                    return start;
                }
                cur = cur.wrapping_add(8);
            }
        }
        let refilled = exe::call_va!(0x1_4222_9ab0, (param_1) -> u8);
        if refilled == 0 {
            return 0xffff_ffff;
        }
    }
}

/// `unknown_14224d9d0`
/// Original: 0x14224d9d0 (582 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14224d9d0(param_1: usize, param_2: usize) {
    wr::<u16>(param_1 + 0x96, 0);
    let r13 = exe::ptr::<u8>(0x1_43a6_3260) as usize;
    let lock_addr = exe::ptr::<u8>(0x1_43a3_2f28) as usize;

    for i in 0..6usize {
        let old_handle = rd::<u64>(param_1 + i * 8);
        exe::call_va!(0x1_4205_ae20, (old_handle));

        let rbx = rd::<u64>(param_2 + i * 8);
        let handle = if rbx != 0 {
            exe::call_iat!(0x1_4264_55a8, (lock_addr));
            let count = rd::<u32>(r13 + i * 16 + 0x08) as usize;
            let mut found: Option<u64> = None;
            if count != 0 {
                let mut entry = rd::<usize>(r13 + i * 16);
                for j in 0..count {
                    if rd::<u64>(entry + 0x90) == rbx {
                        wr::<u32>(entry + 0x98, rd::<u32>(entry + 0x98).wrapping_add(1));
                        let gen = rd::<u16>(entry + 0x9c) as u64;
                        exe::call_iat!(0x1_4264_5598, (lock_addr));
                        let hi = (gen << 16) | ((i as u16) as u64);
                        found = Some((hi << 32) | (j as u32 as u64));
                        break;
                    }
                    entry = entry.wrapping_add(0xa0);
                }
            }
            if let Some(h) = found {
                h
            } else {
                exe::call_iat!(0x1_4264_5598, (lock_addr));
                u64::MAX
            }
        } else {
            u64::MAX
        };

        wr::<u64>(param_1 + i * 8, handle);
        let slot_type = (handle >> 32) as u16 as i16;
        let entry_ptr = if slot_type < 0 {
            0usize
        } else {
            let idx_low = (handle as u32) as usize;
            let gen_hi = (handle >> 48) as u16;
            let base_arr = rd::<usize>(r13.wrapping_add((slot_type as isize as usize).wrapping_mul(16)));
            let cand = base_arr.wrapping_add(idx_low.wrapping_mul(0xa0));
            if gen_hi == rd::<u16>(cand + 0x9c) {
                cand
            } else {
                0usize
            }
        };

        if entry_ptr != 0 {
            wr::<u64>(param_1 + 0x30 + i * 8, rd::<u64>(entry_ptr + 0x08));
            let p10 = rd::<usize>(entry_ptr + 0x10);
            wr::<usize>(param_1 + 0x60 + i * 8, p10);
            if p10 != 0 {
                let gt0 = if rd::<i32>(p10 + 0x20) > 0 { 1u8 } else { 0u8 };
                wr::<u8>(param_1 + 0x97, rd::<u8>(param_1 + 0x97) | gt0);
                let p10_cur = rd::<usize>(param_1 + 0x60 + i * 8);
                let b_a8 = rd::<u8>(p10_cur + 0xa8);
                wr::<u8>(param_1 + 0x96, rd::<u8>(param_1 + 0x96) | b_a8);
            }
        } else {
            wr::<u64>(param_1 + 0x30 + i * 8, 0);
            wr::<u64>(param_1 + 0x60 + i * 8, 0);
        }
    }

    const C_RDI: u64 = 0x9ae1_6a3b_2f90_404f;
    const C_RSI: u64 = 0x651e_95c4_d06f_bfb1;
    const C_RBX: u64 = 0x9ddf_ea08_eb38_2d69;
    const C_R11: u64 = 0x9ae1_6a3b_2f90_405f;

    let mut rbp: u64 = 0;
    for k in 0..6usize {
        let r8 = rd::<u64>(param_1 + 0x30 + k * 8);
        let rax = r8.wrapping_add(C_RDI);
        let rdx = rax.rotate_right(0x19).wrapping_add(r8).wrapping_mul(C_R11);
        let r8_2 = (r8.rotate_right(0x25).wrapping_mul(C_R11).wrapping_add(rax) ^ rdx)
            .wrapping_mul(C_R11);
        let rcx_1 = ((r8_2 >> 0x2f) ^ r8_2 ^ rdx).wrapping_mul(C_R11);
        let rcx_2 = (((rcx_1 ^ (rcx_1 >> 0x2f)).wrapping_mul(C_R11)).wrapping_add(C_RSI) ^ rbp)
            .wrapping_mul(C_RBX);
        let rax_2 = ((rcx_2 >> 0x2f) ^ rcx_2 ^ rbp).wrapping_mul(C_RBX);
        rbp = ((rax_2 >> 0x2f) ^ rax_2).wrapping_mul(C_RBX);
    }
    wr::<u16>(param_1 + 0x94, (rbp as u16) & 0x1fff);
}

/// `unknown_142302e60`
/// Original: 0x142302e60 (363 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142302e60(param_1: usize, param_2: usize) {
    let rbx = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
    let edi = rd::<u32>(param_1 + 0x48);
    let idx_50 = rd::<u32>(param_1 + 0x50) as usize;
    let r15 = rd::<usize>(rbx + 0x120).wrapping_add(idx_50.wrapping_mul(0xa0));

    let ok = exe::call_va!(0x1_4222_ec60, (r15, edi) -> u8);
    if ok != 0 {
        let ecx = rd::<u32>(r15 + 0x98) as usize;
        let rbp = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
        let r14d = rd::<u8>(rd::<usize>(rbp + 0x1a8).wrapping_add(edi as usize));
        let esi = rd::<u32>(rd::<usize>(rbx + 0x100).wrapping_add(ecx * 4)) as usize;

        wr::<u8>(rd::<usize>(rbp + 0x130).wrapping_add(esi * 8 + 4), 0);

        let rdi = rd::<usize>(param_2);
        let count = rd::<u32>(rdi + 0x08) as u64;
        let byte_len = count.wrapping_mul(0x30);
        let dst_idx = exe::call_va!(0x1_4222_f750, (r15, r14d as u32) -> u32) as usize;
        let dst_ptr = rd::<usize>(rbp + 0x5a8).wrapping_add(dst_idx.wrapping_mul(0x30));
        let src_ptr = rd::<usize>(rdi);
        let _ = exe::call_va!(0x1_4183_acbc, (dst_ptr, src_ptr, byte_len) -> usize);

        let word_off = (esi >> 6) * 8;
        let bit_mask = 1u64 << (esi & 0x3f);
        let p698 = rd::<usize>(rbp + 0x698).wrapping_add(word_off);
        wr::<u64>(p698, rd::<u64>(p698) | bit_mask);
        let p668 = rd::<usize>(rbp + 0x668).wrapping_add(word_off);
        wr::<u64>(p668, rd::<u64>(p668) | bit_mask);

        let mut cur = rd::<usize>(r15 + 0x80);
        let end = cur.wrapping_add((rd::<u32>(r15 + 0x88) as usize) * 4);
        while cur != end {
            let edx = rd::<u32>(cur);
            if edx != 0xffff_ffff {
                let b = rd::<u8>(rd::<usize>(rbp + 0x1a8).wrapping_add(edx as usize));
                if b == r14d {
                    let p38 = rd::<usize>(rbp + 0x38).wrapping_add(((edx as usize) >> 6) * 8);
                    wr::<u64>(p38, rd::<u64>(p38) | (1u64 << (edx & 0x3f)));
                }
            }
            cur = cur.wrapping_add(4);
        }
    }
}

#[inline(always)]
unsafe fn run_pass_1423076a0(
    param_1: usize,
    param_2: u64,
    buf: usize,
    global_va: u64,
    flag_mask: u32,
    flag_expect: u32,
    str_va: u64,
    pass_kind: u64,
) {
    let g_ptr = rd::<usize>(exe::ptr::<usize>(global_va) as usize);
    let bit_idx = rd::<u32>(g_ptr);
    let r13 = rd::<u64>(g_ptr + 0x08);
    let mut out_cur = buf;

    if param_2 != 0 {
        let mut item_ptr = param_1.wrapping_add(0x08);
        let str_ptr = exe::ptr::<u8>(str_va) as usize;
        for _ in 0..param_2 {
            if (rd::<u32>(item_ptr - 0x08) & flag_mask) == flag_expect {
                let obj = rd::<usize>(item_ptr);
                if rd::<i32>(obj + 0x138) > 0 {
                    let rot_bit = 1u32.rotate_left(bit_idx);
                    let mut i: i32 = 0;
                    let mut off: usize = 0;
                    loop {
                        let r8d = rd::<u32>(item_ptr - 0x04);
                        let res = exe::call_va!(
                            0x1_4232_fb50,
                            (obj, i as u32, r8d, str_ptr, pass_kind, r13, 0u64) -> u64
                        );
                        if res != 0 {
                            let sub_arr = rd::<usize>(item_ptr + 0x08);
                            wr::<u32>(out_cur, rd::<u32>(sub_arr + off + 0x04));
                            wr::<u64>(out_cur + 0x08, res);
                            out_cur = out_cur.wrapping_add(0x10);
                            let slot = sub_arr + off;
                            wr::<u32>(slot, rd::<u32>(slot) | rot_bit);
                        }
                        i += 1;
                        off = off.wrapping_add(8);
                        if i >= rd::<i32>(obj + 0x138) {
                            break;
                        }
                    }
                }
            }
            item_ptr = item_ptr.wrapping_add(0x18);
        }
    }

    let count_out = ((out_cur as isize).wrapping_sub(buf as isize) >> 4) as u64;
    exe::call_va!(0x1_4232_1130, (g_ptr, buf, count_out, pass_kind as u32));
}

/// `unknown_1423076a0`
/// Original: 0x1423076a0 (1764 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1423076a0(param_1: usize, param_2: u64, param_3: u64) {
    let buf = exe::call_va!(0x1_4241_c0a0, (param_3 << 4, 0x10u32) -> usize);

    run_pass_1423076a0(param_1, param_2, buf, 0x1_4397_ffa0, 0x201, 0, 0x1_4386_c260, 3);
    run_pass_1423076a0(param_1, param_2, buf, 0x1_4397_ff90, 0x201, 0, 0x1_4386_c278, 5);
    run_pass_1423076a0(param_1, param_2, buf, 0x1_4397_ffb8, 0x201, 0, 0x1_4386_6b48, 1);
    run_pass_1423076a0(param_1, param_2, buf, 0x1_4397_ff98, 0x201, 0, 0x1_4386_5268, 1);
    run_pass_1423076a0(param_1, param_2, buf, 0x1_4397_ffa8, 0x205, 4, 0x1_4386_c250, 2);
    run_pass_1423076a0(param_1, param_2, buf, 0x1_4397_ffd8, 0x101, 0x100, 0x1_4386_1fe8, 1);

    let s_5ff8 = exe::ptr::<u8>(0x1_4386_5ff8) as usize;
    let g_ffc0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ffc0) as usize);
    exe::call_va!(
        0x1_4230_7d90,
        (param_1, param_2, 0x801u32, 0x800u32, g_ffc0, s_5ff8, 1u64, 1u8, buf)
    );

    let g_ffc8 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ffc8) as usize);
    exe::call_va!(
        0x1_4230_7d90,
        (param_1, param_2, 0x401u32, 0x400u32, g_ffc8, s_5ff8, 1u64, 1u8, buf)
    );

    let s_3778 = exe::ptr::<u8>(0x1_4386_3778) as usize;
    let g_ffd0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ffd0) as usize);
    exe::call_va!(
        0x1_4230_7d90,
        (param_1, param_2, 0x401u32, 0x400u32, g_ffd0, s_3778, 1u64, 1u8, buf)
    );

    exe::call_va!(0x1_4241_c190, (buf));
}

/// `unknown_14230ae90`
/// Original: 0x14230ae90 (179 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14230ae90(param_1: usize) {
    let len = rd::<u32>(param_1 + 0x38) as u64;
    let src = rd::<usize>(param_1 + 0x30);
    let dst = rd::<usize>(param_1 + 0x10);
    let _ = exe::call_va!(0x1_4183_acbc, (dst, src, len) -> usize);

    let r10 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
    let edi = rd::<i32>(exe::ptr::<i32>(0x1_4386_c398) as usize);
    let count = rd::<u32>(r10 + 0x2b8) as usize;

    for i in 0..count {
        let arr_2b0 = rd::<usize>(r10 + 0x2b0);
        let edx = rd::<u32>(arr_2b0 + i * 4);
        if edx != 0xffff_ffff {
            let target_ptr = rd::<usize>(param_1 + 0x10).wrapping_add(edx as usize);
            let arr_2a0 = rd::<usize>(r10 + 0x2a0);
            let r8d = (rd::<u8>(arr_2a0 + i) as i32).wrapping_sub(edi);
            let max0 = if r8d >= 0 { r8d } else { 0 };
            let clamped = if r8d <= 0xff { max0 as u8 } else { 0xffu8 };
            let cur = rd::<u8>(target_ptr);
            let next_val = if clamped < cur { clamped } else { cur };
            wr::<u8>(target_ptr, next_val);
        }
    }
}

#[inline(always)]
unsafe fn insert_or_update_14232beb0(
    rsi_vec: usize,
    item_ptr: usize,
    r15d: u32,
    r14d: u32,
    f_val: f32,
) {
    if item_ptr == 0 || (rd::<u8>(item_ptr + 0xbb) <= 1 && rd::<u8>(item_ptr + 0xb8) == 0) {
        return;
    }

    let len = rd::<u32>(rsi_vec + 0x08) as usize;
    let base = rd::<usize>(rsi_vec);
    for idx in 0..len {
        let slot = base.wrapping_add(idx * 0x18);
        if rd::<usize>(slot) == item_ptr {
            wr::<u32>(slot + 0x08, rd::<u32>(slot + 0x08) | (1u32 << (r15d & 0x1f)));
            wr::<u32>(slot + 0x0c, rd::<u32>(slot + 0x0c) | (1u32 << (r14d & 0x1f)));
            let old_f = rd::<f32>(slot + 0x10);
            let new_f = if old_f > f_val { old_f } else { f_val };
            wr::<f32>(slot + 0x10, new_f);
            return;
        }
    }

    let cap = rd::<u32>(rsi_vec + 0x0c) as u64;
    let len_u64 = len as u64;
    if len_u64 * 0x18 == cap * 0x18 {
        exe::call_va!(0x1_411d_ba80, (rsi_vec, len_u64 + 1));
    }
    let cur_len = rd::<u32>(rsi_vec + 0x08) as usize;
    let cur_base = rd::<usize>(rsi_vec);
    let zero_slot = cur_base.wrapping_add(cur_len * 0x18);
    wr::<u128>(zero_slot, 0);
    wr::<u64>(zero_slot + 0x10, 0);

    let new_len = (cur_len as u32).wrapping_add(1);
    wr::<u32>(rsi_vec + 0x08, new_len);
    let write_slot = rd::<usize>(rsi_vec).wrapping_add((new_len as usize) * 0x18 - 0x18);
    wr::<usize>(write_slot, item_ptr);
    wr::<u32>(write_slot + 0x08, 1u32 << (r15d & 0x1f));
    wr::<u32>(write_slot + 0x0c, 1u32 << (r14d & 0x1f));
    wr::<f32>(write_slot + 0x10, f_val);
    wr::<u8>(write_slot + 0x14, 0);
}

/// `unknown_14232beb0`
/// Original: 0x14232beb0 (1479 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14232beb0(param_1: usize) {
    let count_138 = rd::<u32>(param_1 + 0x138);
    let num_passes = rd::<i32>(param_1 + 0x178).wrapping_add(1);
    let f_280 = rd::<f32>(param_1 + 0x280);

    if (count_138 as i32) > 0 {
        let mut rdi_off: usize = 0;
        let rsi_vec = param_1 + 0x448;
        for _ in 0..(count_138 as usize) {
            let rbx = rd::<usize>(rd::<usize>(param_1 + 0x130).wrapping_add(rdi_off));
            let r15d = rd::<u32>(rbx + 0xa8);
            let f19 = call_log2f(rd::<f32>(rbx + 0x100) / f_280);
            let f20_raw = call_log2f(rd::<f32>(rbx + 0xd4));
            let f20 = (f19 + f19) - f20_raw;

            if num_passes > 0 {
                for r14d in 0..(num_passes as u32) {
                    let base_arr = if r14d == 0 || rd::<u32>(param_1 + 0x178) == 0 {
                        rd::<usize>(param_1 + 0x160)
                    } else {
                        let arr_170 = rd::<usize>(param_1 + 0x170);
                        rd::<usize>(arr_170.wrapping_add(((r14d as usize) - 1) * 0x18 + 0x08))
                    };
                    let elem = rd::<usize>(base_arr.wrapping_add(rdi_off));
                    let rbp_node = rd::<usize>(elem + 0xc0);

                    let mut cur_30 = rd::<usize>(rbp_node + 0x30);
                    let end_30 = cur_30.wrapping_add((rd::<u32>(rbp_node + 0x38) as usize) * 8);
                    while cur_30 != end_30 {
                        let item_ptr = rd::<usize>(cur_30);
                        insert_or_update_14232beb0(rsi_vec, item_ptr, r15d, r14d, f20);
                        cur_30 = cur_30.wrapping_add(8);
                    }

                    let mut cur_20 = rd::<usize>(rbp_node + 0x20);
                    let end_20 = cur_20.wrapping_add((rd::<u32>(rbp_node + 0x28) as usize) * 8);
                    while cur_20 != end_20 {
                        let sub_elem = rd::<usize>(cur_20);
                        let sub_c0 = rd::<usize>(sub_elem + 0xc0);
                        let mut sub_cur = rd::<usize>(sub_c0 + 0x30);
                        let sub_end =
                            sub_cur.wrapping_add((rd::<u32>(sub_c0 + 0x38) as usize) * 8);
                        while sub_cur != sub_end {
                            let item_ptr = rd::<usize>(sub_cur);
                            insert_or_update_14232beb0(rsi_vec, item_ptr, r15d, r14d, f20);
                            sub_cur = sub_cur.wrapping_add(8);
                        }
                        cur_20 = cur_20.wrapping_add(8);
                    }
                }
            }

            rdi_off = rdi_off.wrapping_add(8);
        }
    }

    let edi = rd::<u32>(param_1 + 0x450);
    let rdi_u64 = edi as u64;
    exe::call_va!(0x1_4220_58a0, (param_1 + 0x428, edi));

    let rbx_438 = param_1 + 0x438;
    let esi_len = rd::<u32>(rbx_438 + 0x08) as u64;
    let cap_444 = rd::<u32>(rbx_438 + 0x0c) as u64;
    if rdi_u64 > cap_444 {
        let new_cap = ((cap_444 >> 1) + cap_444).max(rdi_u64);
        exe::call_va!(0x1_4087_46b0, (rbx_438, new_cap));
        let diff = rdi_u64.wrapping_sub(esi_len);
        if diff != 0 {
            let cur_len = rd::<u32>(rbx_438 + 0x08) as usize;
            let dst = rd::<usize>(rbx_438).wrapping_add(cur_len * 8);
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff * 8) -> usize);
        }
        wr::<u32>(rbx_438 + 0x08, edi);
    } else if rdi_u64 > esi_len {
        let diff = rdi_u64.wrapping_sub(esi_len);
        if diff != 0 {
            let dst = rd::<usize>(rbx_438).wrapping_add((esi_len as usize) * 8);
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff * 8) -> usize);
        }
        wr::<u32>(rbx_438 + 0x08, edi);
    } else if rdi_u64 < esi_len {
        wr::<u32>(rbx_438 + 0x08, edi);
    }

    for i in 0..(edi as usize) {
        let src_arr = rd::<usize>(param_1 + 0x448);
        let dst_arr = rd::<usize>(param_1 + 0x438);
        let v_08 = rd::<u32>(src_arr + i * 0x18 + 0x08);
        let v_10 = rd::<u32>(src_arr + i * 0x18 + 0x10);
        wr::<u32>(dst_arr + i * 8 + 0x04, v_08);
        wr::<u32>(dst_arr + i * 8, v_10);
    }

    wr::<u32>(param_1 + 0x458, 0);
    let idx_26c = rd::<u32>(param_1 + 0x26c);
    if idx_26c != 0xffff_ffff {
        let r9 = rd::<u64>(param_1 + 0x438);
        let g_f48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
        let rdx = rd::<usize>(g_f48 + 0x250);
        let off_432 = rd::<u16>(param_1 + 0x432) as usize;
        let p428 = rd::<usize>(param_1 + 0x428);
        let rcx = p428.wrapping_add(off_432 * 4) as u64;
        let entry = rdx.wrapping_add((idx_26c as usize) * 0x18);
        wr::<u64>(entry + 0x08, rcx);
        wr::<u64>(entry + 0x10, r9);
        wr::<u16>(entry, edi as u16);
    }
}

/// `rend_142337130`
/// Original: 0x142337130 (1080 bytes, Throws::No)
pub unsafe extern "win64" fn rend_142337130(_param_1: usize, param_2: usize, param_3: usize) {
    wr::<u32>(param_3 + 0x08, 0);
    wr::<u32>(param_3 + 0x18, 0);

    let mut count: i32 = 0;
    let vtbl0 = rd::<usize>(param_2);
    exe::call_va!(
        rd::<usize>(vtbl0 + 0x48),
        (param_2, (&mut count as *mut i32) as usize)
    );

    let rbx = count as i64 as u64;
    if rbx > (rd::<u32>(param_3 + 0x0c) as u64) {
        exe::call_va!(0x1_405a_f8d0, (param_3, rbx));
    }
    let p10 = param_3.wrapping_add(0x10);
    if rbx > (rd::<u32>(p10 + 0x0c) as u64) {
        exe::call_va!(0x1_4131_19e0, (p10, rbx));
    }

    if count > 0 {
        for _ in 0..count {
            let mut rstr = [0u64; 10];
            let rstr_ptr = rstr.as_mut_ptr() as usize;
            wr::<u64>(rstr_ptr, 0x0800_0008);
            wr::<u8>(rstr_ptr + 0x08, 0);

            let vtbl = rd::<usize>(param_2);
            exe::call_va!(rd::<usize>(vtbl + 0x20), (param_2, rstr_ptr));

            let hdr = rd::<u32>(rstr_ptr);
            let b3 = hdr >> 24;
            let cap_bits = if b3 < 0x80 {
                b3.wrapping_mul(8)
            } else {
                (b3 << 8).wrapping_sub(0x7c00)
            };
            let len_bits = (hdr & 0x00ff_ffff) << 3;
            let s_ptr = if len_bits > cap_bits {
                rd::<usize>(rstr_ptr + 0x08)
            } else {
                rstr_ptr.wrapping_add(0x08)
            };

            let mut s_len: usize = 0;
            while rd::<u8>(s_ptr.wrapping_add(s_len)) != 0 {
                s_len = s_len.wrapping_add(1);
            }

            let mut slice_desc: [u64; 2] = [s_ptr as u64, s_len as u64];
            let mut name_id: u64 = 0;
            exe::call_va!(
                0x1_4220_ea70,
                (
                    (&mut name_id as *mut u64) as usize,
                    slice_desc.as_mut_ptr() as usize
                )
            );
            let name_id_u32 = name_id as u32;

            let mut kind: i32 = 0;
            let vtbl_k = rd::<usize>(param_2);
            exe::call_va!(
                rd::<usize>(vtbl_k + 0x48),
                (param_2, (&mut kind as *mut i32) as usize)
            );

            match kind {
                0 => {
                    let mut v0: u32 = 0;
                    let vt = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt + 0x38), (param_2, (&mut v0 as *mut u32) as usize));
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 0);
                    wr::<u32>(slot + 0x08, v0);
                }
                1 => {
                    let mut v = [0u32; 2];
                    let vp = v.as_mut_ptr() as usize;
                    let vt = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt + 0x38), (param_2, vp));
                    let vt2 = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt2 + 0x38), (param_2, vp + 4));
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 1);
                    wr::<u32>(slot + 0x08, v[0]);
                    wr::<u32>(slot + 0x0c, v[1]);
                }
                2 => {
                    let mut v = [0u32; 3];
                    let vp = v.as_mut_ptr() as usize;
                    for j in 0..3usize {
                        let vt = rd::<usize>(param_2);
                        exe::call_va!(rd::<usize>(vt + 0x38), (param_2, vp + j * 4));
                    }
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 2);
                    wr::<u32>(slot + 0x08, v[0]);
                    wr::<u32>(slot + 0x0c, v[1]);
                    wr::<u32>(slot + 0x10, v[2]);
                }
                3 => {
                    let mut v = [0u32; 4];
                    let vp = v.as_mut_ptr() as usize;
                    for j in 0..4usize {
                        let vt = rd::<usize>(param_2);
                        exe::call_va!(rd::<usize>(vt + 0x38), (param_2, vp + j * 4));
                    }
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 3);
                    wr::<u32>(slot + 0x08, v[0]);
                    wr::<u32>(slot + 0x0c, v[1]);
                    wr::<u32>(slot + 0x10, v[2]);
                    wr::<u32>(slot + 0x14, v[3]);
                }
                4 => {
                    let mut v0: u32 = 0;
                    let vt = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt + 0x40), (param_2, (&mut v0 as *mut u32) as usize));
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 4);
                    wr::<u64>(slot + 0x08, v0 as u64);
                }
                5 => {
                    let mut v0: u32 = 0;
                    let vt = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt + 0x40), (param_2, (&mut v0 as *mut u32) as usize));
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 5);
                    wr::<u64>(slot + 0x08, v0 as u64);
                }
                6 => {
                    let mut v0: u64 = 0;
                    let vt = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt), (param_2, (&mut v0 as *mut u64) as usize));
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 6);
                    wr::<u64>(slot + 0x08, v0);
                }
                0x0c => {
                    let mut v0: u8 = 0;
                    let vt = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt + 0x80), (param_2, (&mut v0 as *mut u8) as usize));
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 0x0c);
                    wr::<u8>(slot + 0x08, v0);
                }
                0x10 => {
                    let mut v0: u32 = 0;
                    let vt = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt + 0x48), (param_2, (&mut v0 as *mut u32) as usize));
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 0x10);
                    wr::<u32>(slot + 0x08, v0);
                }
                0x11 => {
                    let mut v = [0u32; 2];
                    let vp = v.as_mut_ptr() as usize;
                    let vt = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt + 0x48), (param_2, vp));
                    let vt2 = rd::<usize>(param_2);
                    exe::call_va!(rd::<usize>(vt2 + 0x48), (param_2, vp + 4));
                    let slot = exe::call_va!(0x1_4131_1210, (param_3, name_id_u32) -> usize);
                    wr::<u32>(slot, 0x11);
                    wr::<u32>(slot + 0x08, v[0]);
                    wr::<u32>(slot + 0x0c, v[1]);
                }
                _ => {}
            }

            exe::call_va!(0x1_4240_ab10, (rstr_ptr));
        }
    }
}

/// `unknown_14237fff0`
/// Original: 0x14237fff0 (2332 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14237fff0(param_1: usize, param_2: usize, param_3: usize) {
    let free_base = rd::<usize>(param_1 + 0x528);
    let free_len = rd::<u32>(param_1 + 0x530) as usize;
    let free_end = free_base.wrapping_add(free_len * 4);
    let vec_538 = param_1 + 0x538;

    if free_base == free_end {
        let cur_len = rd::<u32>(param_1 + 0x540);
        wr::<u32>(param_2 + 0x48, cur_len);
        let len_u64 = rd::<u32>(vec_538 + 0x08) as u64;
        let cap_u64 = rd::<u32>(vec_538 + 0x0c) as u64;
        if (len_u64 << 4) == (cap_u64 << 4) {
            let new_cap = ((cap_u64 >> 1) + cap_u64).max(len_u64 + 1);
            exe::call_va!(0x1_4135_51f0, (vec_538, new_cap));
        }
        let idx = rd::<u32>(vec_538 + 0x08) as usize;
        let slot = rd::<usize>(vec_538).wrapping_add(idx * 16);
        wr::<u64>(slot, 0);
        wr::<u64>(slot + 0x08, 0);
        wr::<u32>(vec_538 + 0x08, (idx as u32).wrapping_add(1));
    } else {
        wr::<u32>(param_2 + 0x48, rd::<u32>(free_end - 4));
        wr::<u32>(param_1 + 0x530, (free_len as u32).wrapping_sub(1));
    }

    let local_68 = rd::<usize>(vec_538).wrapping_add((rd::<u32>(param_2 + 0x48) as usize) * 16);
    let b_2a8 = rd::<u8>(param_2 + 0x2a8) as u32;
    exe::call_va!(0x1_4135_4d40, (local_68, b_2a8, 0u64, local_68));
    wr::<u32>(param_1 + 0x520, rd::<u32>(param_1 + 0x520).wrapping_add(1));

    let mut esi_sum3: u32 = 0;
    let mut ebp_sum1: u32 = 0;
    let b_272 = rd::<u8>(param_2 + 0x272) as usize;
    for k in 0..b_272 {
        let c = rd::<u32>(param_2 + 0x1c8 + k * 4);
        esi_sum3 = esi_sum3.wrapping_add(c.wrapping_mul(3));
        ebp_sum1 = ebp_sum1.wrapping_add(c);
    }

    let u_2d0 = rd::<u32>(param_2 + 0x2d0);
    exe::call_va!(0x1_4237_c2c0, (param_1 + 0x608));
    let i_var6 = exe::call_va!(0x1_4135_2520, (param_1 + 0x608, u_2d0) -> u32);
    let len_600 = rd::<u32>(param_1 + 0x600) as u64;
    let req_608 = rd::<u32>(param_1 + 0x608) as u64;
    if req_608 > len_600 {
        let diff = req_608.wrapping_sub(len_600);
        let cap_604 = rd::<u32>(param_1 + 0x604) as u64;
        if diff > cap_604.wrapping_sub(len_600) {
            exe::call_va!(0x1_402d_7910, (param_1 + 0x5f8, req_608));
        }
        if diff != 0 {
            let dst = rd::<usize>(param_1 + 0x5f8).wrapping_add(rd::<u32>(param_1 + 0x600) as usize);
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff) -> usize);
        }
        wr::<u32>(param_1 + 0x600, rd::<u32>(param_1 + 0x600).wrapping_add(diff as u32));
    }

    exe::call_va!(0x1_4237_c2c0, (param_1 + 0x658));
    let i_var7 = exe::call_va!(0x1_4135_2520, (param_1 + 0x658, esi_sum3) -> u32);
    let len_650 = rd::<u32>(param_1 + 0x650) as u64;
    let req_658 = rd::<u32>(param_1 + 0x658) as u64;
    if req_658 > len_650 {
        let diff = req_658.wrapping_sub(len_650);
        let cap_654 = rd::<u32>(param_1 + 0x654) as u64;
        if diff > cap_654.wrapping_sub(len_650) {
            let new_cap = ((cap_654 >> 1) + cap_654).max(req_658);
            exe::call_va!(0x1_4062_ee30, (param_1 + 0x648, new_cap));
        }
        if diff != 0 {
            let cur_650 = rd::<u32>(param_1 + 0x650) as usize;
            let base_648 = rd::<usize>(param_1 + 0x648);
            for k in 0..(diff as usize) {
                let p = base_648.wrapping_add((cur_650 + k) * 16);
                wr::<u64>(p, 0);
                wr::<u64>(p + 0x08, 0);
            }
        }
        wr::<u32>(param_1 + 0x650, rd::<u32>(param_1 + 0x650).wrapping_add(diff as u32));
    }

    exe::call_va!(0x1_4237_c2c0, (param_1 + 0x630));
    let i_var8 = exe::call_va!(0x1_4135_2520, (param_1 + 0x630, ebp_sum1) -> u32);
    let len_628 = rd::<u32>(param_1 + 0x628) as u64;
    let req_630 = rd::<u32>(param_1 + 0x630) as u64;
    if req_630 > len_628 {
        let diff = req_630.wrapping_sub(len_628);
        let cap_62c = rd::<u32>(param_1 + 0x62c) as u64;
        if diff > cap_62c.wrapping_sub(len_628) {
            let new_cap = ((cap_62c >> 1) + cap_62c).max(req_630);
            exe::call_va!(0x1_40a5_b830, (param_1 + 0x620, new_cap));
        }
        if diff != 0 {
            let cur_628 = rd::<u32>(param_1 + 0x628) as usize;
            let base_620 = rd::<usize>(param_1 + 0x620);
            for k in 0..(diff as usize) {
                let p = base_620.wrapping_add((cur_628 + k) * 16);
                wr::<u64>(p, 0);
                wr::<u64>(p + 0x08, 0);
            }
        }
        wr::<u32>(param_1 + 0x628, rd::<u32>(param_1 + 0x628).wrapping_add(diff as u32));
    }

    let mut r9_off: u64 = 0;
    let mut rdx_acc: u64 = 0;
    let mut r10_acc: u64 = 0;
    let mut r15d: u32 = 0;

    if rd::<u8>(param_2 + 0x2a8) != 0 {
        let vec_5b8 = param_1 + 0x5b8;
        let vec_5c8 = param_1 + 0x5c8;
        let vec_5a8 = param_1 + 0x5a8;
        let vec_5e8 = param_1 + 0x5e8;

        loop {
            let r12_elem = rd::<usize>(param_2 + 0x2a0)
                .wrapping_add((r15d as i32 as isize as usize).wrapping_mul(0x70));
            let r14_slot = rd::<usize>(local_68).wrapping_add((r15d as usize).wrapping_mul(0x30));
            let esi_idx: u32 = if rd::<u8>(param_2 + 0x272) == 1 { 0 } else { r15d };

            let old_6dc = rd::<u32>(param_1 + 0x6dc);
            wr::<u32>(param_1 + 0x6dc, old_6dc.wrapping_add(1));
            wr::<u32>(r14_slot, old_6dc);
            wr::<u32>(r14_slot + 0x08, i_var6.wrapping_add(r9_off as u32));
            wr::<u32>(r14_slot + 0x0c, i_var7.wrapping_add(rdx_acc as u32));
            wr::<u32>(r14_slot + 0x10, (r10_acc as u32).wrapping_add(i_var8));
            wr::<u32>(r14_slot + 0x04, 0);

            let len_5d0 = rd::<u32>(vec_5c8 + 0x08) as u64;
            let cap_5d4 = rd::<u32>(vec_5c8 + 0x0c) as u64;
            if (len_5d0 << 3) == (cap_5d4 << 3) {
                exe::call_va!(0x1_4020_bcb0, (vec_5c8, len_5d0 + 1));
            }
            let cur_5d0 = rd::<u32>(vec_5c8 + 0x08) as usize;
            wr::<u64>(rd::<usize>(vec_5c8).wrapping_add(cur_5d0 * 8), 0);
            let ebp_5d0 = (cur_5d0 as u32).wrapping_add(1);
            wr::<u32>(vec_5c8 + 0x08, ebp_5d0);
            let rdi_5c8 = rd::<usize>(vec_5c8);
            wr::<u32>(
                rdi_5c8.wrapping_add((ebp_5d0 as usize) * 8 - 8),
                rd::<u32>(param_1 + 0x5b0),
            );
            let cnt_r12 = rd::<u32>(r12_elem);
            wr::<u32>(rdi_5c8.wrapping_add((ebp_5d0 as usize) * 8 - 4), cnt_r12);

            let rbx_cnt = cnt_r12 as u64;
            let len_5b0 = rd::<u32>(vec_5a8 + 0x08) as u64;
            let cap_5b4 = rd::<u32>(vec_5a8 + 0x0c) as u64;
            if rbx_cnt > cap_5b4.wrapping_sub(len_5b0) {
                exe::call_va!(0x1_4135_4b30, (vec_5a8, len_5b0.wrapping_add(rbx_cnt)));
            }
            if rbx_cnt != 0 {
                let cur_5b0 = rd::<u32>(vec_5a8 + 0x08) as usize;
                let base_5a8 = rd::<usize>(vec_5a8);
                for k in 0..(rbx_cnt as usize) {
                    let p = base_5a8.wrapping_add((cur_5b0 + k) * 0x34);
                    wr::<[u8; 0x34]>(p, [0u8; 0x34]);
                }
            }
            wr::<u32>(
                vec_5a8 + 0x08,
                rd::<u32>(vec_5a8 + 0x08).wrapping_add(cnt_r12),
            );

            exe::call_va!(0x1_40ff_f1e0, (r14_slot + 0x18, rd::<u64>(r12_elem + 0x58)));
            let bit_bytes = ((rd::<u64>(r12_elem + 0x58).wrapping_add(0x3f)) >> 6) * 8;
            let _ = exe::call_va!(
                0x1_4183_acbc,
                (
                    rd::<usize>(r14_slot + 0x18),
                    rd::<usize>(r12_elem + 0x48),
                    bit_bytes
                ) -> usize
            );

            let mut r12_off = rd::<u32>(vec_5b8 + 0x08);
            let rbx_68 = rd::<u32>(r12_elem + 0x68) as u64;
            let cap_5bc = rd::<u32>(vec_5b8 + 0x0c) as u64;
            if rbx_68 > cap_5bc.wrapping_sub(r12_off as u64) {
                let new_cap = ((cap_5bc >> 1) + cap_5bc).max((r12_off as u64) + rbx_68);
                exe::call_va!(0x1_401d_eb40, (vec_5b8, new_cap));
            }
            if rbx_68 != 0 {
                let dst_zero =
                    rd::<usize>(vec_5b8).wrapping_add((rd::<u32>(vec_5b8 + 0x08) as usize) * 4);
                let _ = exe::call_va!(0x1_4183_acc2, (dst_zero, 0u32, rbx_68 * 4) -> usize);
            }
            wr::<u32>(
                vec_5b8 + 0x08,
                rd::<u32>(vec_5b8 + 0x08).wrapping_add(rbx_68 as u32),
            );
            let dst_copy = rd::<usize>(vec_5b8).wrapping_add((r12_off as usize) * 4);
            let copy_len = (rd::<u32>(r12_elem + 0x68) as u64) << 2;
            let _ = exe::call_va!(
                0x1_4183_acbc,
                (dst_copy, rd::<usize>(r12_elem + 0x60), copy_len) -> usize
            );

            if rd::<u32>(r12_elem) > 0 {
                let mut r14_sub: u32 = 0;
                loop {
                    let base_idx = rd::<u32>(rdi_5c8.wrapping_add((ebp_5d0 as usize) * 8 - 8))
                        .wrapping_add(r14_sub) as usize;
                    let entry_5a8 = rd::<usize>(vec_5a8).wrapping_add(base_idx * 0x34);
                    wr::<u32>(entry_5a8 + 0x2c, r12_off);
                    let arr_38 = rd::<usize>(r12_elem + 0x38);
                    let ecx_cnt = rd::<u32>(arr_38.wrapping_add((r14_sub as usize) * 4));
                    wr::<u32>(entry_5a8 + 0x30, ecx_cnt);
                    r12_off = r12_off.wrapping_add(ecx_cnt);

                    let cur_max = rd::<u32>(r14_slot + 0x04);
                    if cur_max < ecx_cnt {
                        wr::<u32>(r14_slot + 0x04, ecx_cnt);
                    }

                    let arr_18 = rd::<usize>(r12_elem + 0x18);
                    let parent_idx = rd::<i32>(arr_18.wrapping_add((r14_sub as usize) * 4));
                    wr::<u32>(entry_5a8 + 0x1c, r14_sub);

                    let arr_2b0 = rd::<usize>(param_2 + 0x2b0);
                    let sub_2b0 =
                        rd::<usize>(arr_2b0.wrapping_add((esi_idx as i32 as isize as usize) * 0x20));
                    wr::<u32>(
                        entry_5a8 + 0x20,
                        rd::<u32>(sub_2b0.wrapping_add((r14_sub as usize) * 4)),
                    );

                    let rel_parent = if parent_idx == -1 {
                        0
                    } else {
                        parent_idx.wrapping_sub(r14_sub as i32)
                    };
                    wr::<i32>(entry_5a8 + 0x24, rel_parent);

                    let off_28 = r14_sub
                        .wrapping_mul(rd::<u32>(param_2 + 0x2c0))
                        .wrapping_add(rd::<u32>(r14_slot + 0x08));
                    wr::<u32>(entry_5a8 + 0x28, off_28);

                    let src_xform =
                        rd::<usize>(r12_elem + 0x08).wrapping_add((r14_sub as usize) * 0x30);
                    let mut quat = [0u32; 4];
                    exe::call_va!(0x1_4027_5010, (quat.as_mut_ptr() as usize, src_xform));
                    wr::<u64>(entry_5a8 + 0x10, rd::<u64>(src_xform + 0x24));
                    wr::<u32>(entry_5a8 + 0x18, rd::<u32>(src_xform + 0x2c));
                    wr::<u32>(entry_5a8, quat[1]);
                    wr::<u32>(entry_5a8 + 0x04, quat[2]);
                    wr::<u32>(entry_5a8 + 0x08, quat[3]);
                    wr::<u32>(entry_5a8 + 0x0c, quat[0]);

                    r14_sub = r14_sub.wrapping_add(1);
                    if r14_sub >= rd::<u32>(r12_elem) {
                        break;
                    }
                }
            }

            let len_5f0 = rd::<u32>(vec_5e8 + 0x08) as u64;
            let cap_5f4 = rd::<u32>(vec_5e8 + 0x0c) as u64;
            if (len_5f0 << 4) == (cap_5f4 << 4) {
                exe::call_va!(0x1_401f_c320, (vec_5e8, len_5f0 + 1));
            }
            let cur_5f0 = rd::<u32>(vec_5e8 + 0x08) as usize;
            wr::<u128>(rd::<usize>(vec_5e8).wrapping_add(cur_5f0 * 16), 0);
            let new_5f0 = (cur_5f0 as u32).wrapping_add(1);
            wr::<u32>(vec_5e8 + 0x08, new_5f0);
            let rbx_ent = rd::<usize>(vec_5e8).wrapping_add((new_5f0 as usize) * 16);
            wr::<u32>(rbx_ent - 0x10, rd::<u32>(param_1 + 0x5e0));
            wr::<u32>(rbx_ent - 0x0c, 0);
            wr::<u32>(rbx_ent - 0x08, rd::<u32>(r14_slot + 0x0c));
            wr::<u32>(rbx_ent - 0x04, rd::<u32>(r14_slot + 0x10));

            let rdi_2b0 = rd::<usize>(param_2 + 0x2b0)
                .wrapping_add((esi_idx as i32 as isize as usize) * 0x20);
            if rd::<u32>(rdi_2b0 + 0x18) > 0 {
                let p3_len = rd::<usize>(param_3);
                let mut s_idx: u32 = 0;
                loop {
                    let src_val_ptr = if p3_len == 0 || r15d != 0 {
                        rd::<usize>(rdi_2b0 + 0x10).wrapping_add((s_idx as usize) * 4)
                    } else {
                        let p3_arr = rd::<usize>(param_3 + 0x08);
                        let p3_end = p3_arr.wrapping_add(p3_len * 4);
                        let mut p_cur = p3_arr;
                        while p_cur != p3_end && rd::<u32>(p_cur) != s_idx {
                            p_cur = p_cur.wrapping_add(4);
                        }
                        if p_cur == p3_end {
                            rd::<usize>(rdi_2b0 + 0x10).wrapping_add((s_idx as usize) * 4)
                        } else {
                            exe::ptr::<u8>(0x1_42c2_7d50) as usize
                        }
                    };
                    exe::call_va!(0x1_401d_e8d0, (param_1 + 0x5d8, src_val_ptr));
                    wr::<u32>(rbx_ent - 0x0c, rd::<u32>(rbx_ent - 0x0c).wrapping_add(1));
                    s_idx = s_idx.wrapping_add(1);
                    if s_idx >= rd::<u32>(rdi_2b0 + 0x18) {
                        break;
                    }
                }
            }

            let stride_2c0 = rd::<i32>(param_2 + 0x2c0) as i64 as u64;
            let copy_size = stride_2c0.wrapping_mul(rd::<u32>(r12_elem) as u64);
            if copy_size != 0 {
                let dst_5f8 = rd::<usize>(param_1 + 0x5f8)
                    .wrapping_add(rd::<u32>(r14_slot + 0x08) as usize);
                let src_2c8 = rd::<usize>(param_2 + 0x2c8).wrapping_add(r9_off as usize);
                let _ = exe::call_va!(0x1_4183_acbc, (dst_5f8, src_2c8, copy_size) -> usize);
            }

            if r15d < (rd::<u8>(param_2 + 0x272) as u32) {
                let idx_i = r15d as i32 as isize as usize;
                let rbx_1c8 = rd::<u32>(param_2 + 0x1c8 + idx_i * 4) as usize;
                let edx_1e8 = rd::<u32>(param_2 + 0x1e8 + idx_i * 4) as usize;
                let r9_src = rd::<usize>(param_2 + 0x1a0).wrapping_add(edx_1e8 * 16);
                let rdi_1b0 = rd::<usize>(param_2 + 0x1b0).wrapping_add(edx_1e8 * 0x30);
                if rbx_1c8 != 0 {
                    let dst_620 = rd::<usize>(param_1 + 0x620)
                        .wrapping_add((rd::<u32>(r14_slot + 0x10) as usize) * 16);
                    let _ = exe::call_va!(
                        0x1_4183_acbc,
                        (dst_620, r9_src, (rbx_1c8 * 16) as u64) -> usize
                    );

                    let mut r8_src = rdi_1b0.wrapping_add(0x18);
                    let mut row_off: usize = 0;
                    for _ in 0..rbx_1c8 {
                        let rdx_row = (rd::<u32>(r14_slot + 0x0c) as usize).wrapping_add(row_off);
                        let row0: [u32; 4] = [
                            rd::<u32>(r8_src - 0x18),
                            rd::<u32>(r8_src - 0x0c),
                            rd::<u32>(r8_src),
                            rd::<u32>(r8_src + 0x0c),
                        ];
                        wr::<[u32; 4]>(
                            rd::<usize>(param_1 + 0x648).wrapping_add(rdx_row * 16),
                            row0,
                        );
                        let row1: [u32; 4] = [
                            rd::<u32>(r8_src - 0x14),
                            rd::<u32>(r8_src - 0x08),
                            rd::<u32>(r8_src + 0x04),
                            rd::<u32>(r8_src + 0x10),
                        ];
                        wr::<[u32; 4]>(
                            rd::<usize>(param_1 + 0x648).wrapping_add(rdx_row * 16 + 0x10),
                            row1,
                        );
                        let row2: [u32; 4] = [
                            rd::<u32>(r8_src - 0x10),
                            rd::<u32>(r8_src - 0x04),
                            rd::<u32>(r8_src + 0x08),
                            rd::<u32>(r8_src + 0x14),
                        ];
                        wr::<[u32; 4]>(
                            rd::<usize>(param_1 + 0x648).wrapping_add((rdx_row + 2) * 16),
                            row2,
                        );
                        r8_src = r8_src.wrapping_add(0x30);
                        row_off = row_off.wrapping_add(3);
                    }
                }
            }

            r9_off = r9_off.wrapping_add(copy_size);
            let next_r15 = r15d.wrapping_add(1);
            if next_r15 < (rd::<u8>(param_2 + 0x272) as u32) {
                let c = rd::<u32>(param_2 + 0x1c8 + (r15d as i32 as isize as usize) * 4) as u64;
                rdx_acc = rdx_acc.wrapping_add(c * 3);
                r10_acc = r10_acc.wrapping_add(c);
            }
            r15d = next_r15;
            if r15d >= (rd::<u8>(param_2 + 0x2a8) as u32) {
                break;
            }
        }
    }

    wr::<u8>(param_1 + 0x6e5, 1);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14220ea10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_ea70(out_ptr: usize, desc_ptr: usize) {
        let s_ptr = rd::<usize>(desc_ptr);
        let s_len = rd::<usize>(desc_ptr + 0x08);
        let mut h: u32 = 0x811c_9dc5 ^ (s_len as u32);
        for i in 0..s_len {
            h = (h ^ (rd::<u8>(s_ptr + i) as u32)).wrapping_mul(0x0100_0193);
        }
        wr::<u32>(out_ptr, h);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;

    for s in [b"\0".as_slice(), b"x\0".as_slice(), b"cauldron_shader_param\0".as_slice()] {
        let o = f_o(s.as_ptr() as usize);
        let n = f_n(s.as_ptr() as usize);
        crate::diff_eq!(o, n, "14220ea10 len={}", s.len());
    }
    Ok(3)
}

pub unsafe fn diff_unknown_142229a00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut REFILL_STEP: u32 = 0;

    unsafe extern "win64" fn mock_9ab0(p1: usize) -> u8 {
        if REFILL_STEP == 0 {
            REFILL_STEP = 1;
            let arr = rd::<usize>(p1 + 0x08);
            let cnt = rd::<u32>(p1 + 0x10) as usize;
            wr::<u32>(arr + cnt * 8, 500);
            wr::<u32>(arr + cnt * 8 + 4, 20);
            wr::<u32>(p1 + 0x10, (cnt as u32) + 1);
            1
        } else {
            0
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_9ab0, mock_9ab0 as *const ())?;

    let run = |f: F, req: u32| -> (u32, u32, [u32; 8]) {
        REFILL_STEP = 0;
        let mut ranges: [u32; 8] = [10, 4, 30, 8, 100, 12, 0, 0];
        let mut mgr = [0u64; 3];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x08, ranges.as_mut_ptr() as usize);
        wr::<u32>(mp + 0x10, 3);

        let ret = f(mp, req);
        (ret, rd::<u32>(mp + 0x10), ranges)
    };

    for req in [4u32, 6, 8, 12, 16, 25] {
        let o = run(f_o, req);
        let n = run(f_n, req);
        crate::diff_eq!(o, n, "142229a00 req={}", req);
    }
    Ok(6)
}

pub unsafe fn diff_unknown_14224d9d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RELEASED_SUM: u64 = 0;

    unsafe extern "win64" fn mock_ae20(h: u64) {
        RELEASED_SUM = RELEASED_SUM.rotate_left(7).wrapping_add(h);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4205_ae20, mock_ae20 as *const ())?;

    let g_table = exe::ptr::<u8>(0x1_43a6_3260) as *mut u8;
    let snap = crate::difftest::Snapshot::take(g_table, 6 * 16);

    let run = |f: F| -> ([u64; 18], [u8; 8], [u32; 6], u64) {
        RELEASED_SUM = 0;

        let mut sub_objs = [[0u8; 0xb0]; 6];
        for i in 0..6usize {
            let sp = sub_objs[i].as_mut_ptr() as usize;
            wr::<i32>(sp + 0x20, if i == 2 { 5 } else { 0 });
            wr::<u8>(sp + 0xa8, (1u8 << i) & 0x15);
        }

        let mut pools = [[0u8; 0xa0 * 2]; 6];
        for i in 0..6usize {
            let pp = pools[i].as_mut_ptr() as usize;
            // Entry 0: non-matching
            wr::<u64>(pp + 0x90, 0x9999_0000 + i as u64);
            wr::<u16>(pp + 0x9c, 1);
            // Entry 1: matching
            let e1 = pp + 0xa0;
            wr::<u64>(e1 + 0x08, 0x1111_2222_3333_0000 + (i as u64) * 0x101);
            wr::<usize>(e1 + 0x10, sub_objs[i].as_mut_ptr() as usize);
            wr::<u64>(e1 + 0x90, 0x5000 + i as u64);
            wr::<u32>(e1 + 0x98, 10 + i as u32);
            wr::<u16>(e1 + 0x9c, (0x20 + i) as u16);

            let slot = (g_table as usize) + i * 16;
            wr::<usize>(slot, pp);
            wr::<u32>(slot + 0x08, 2);
        }

        let mut p1 = [0u8; 0x98];
        let p1p = p1.as_mut_ptr() as usize;
        for i in 0..6usize {
            wr::<u64>(p1p + i * 8, 0xaaaa_0000 + i as u64);
        }
        wr::<u16>(p1p + 0x96, 0x7777);

        let p2: [u64; 6] = [
            0x5000,     // slot 0: matches entry 1
            0,          // slot 1: null
            0x5002,     // slot 2: matches entry 1 (has +0x20 > 0)
            0xdead_beef, // slot 3: non-zero, not found
            0x5004,     // slot 4: matches entry 1
            0x5005,     // slot 5: matches entry 1
        ];

        f(p1p, p2.as_ptr() as usize);

        let mut refcounts = [0u32; 6];
        for i in 0..6usize {
            let e1 = (pools[i].as_ptr() as usize) + 0xa0;
            refcounts[i] = rd::<u32>(e1 + 0x98);
            // Normalize pointer at p1 + 0x60 + i*8 to boolean presence
            let ptr_val = rd::<usize>(p1p + 0x60 + i * 8);
            wr::<u64>(p1p + 0x60 + i * 8, if ptr_val != 0 { 1 } else { 0 });
        }

        (
            rd::<[u64; 18]>(p1p),
            rd::<[u8; 8]>(p1p + 0x90),
            refcounts,
            RELEASED_SUM,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    snap.restore();

    crate::diff_eq!(o, n, "14224d9d0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142302e60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_ec60(_r15: usize, edi: u32) -> u8 {
        if edi == 2 { 1 } else { 0 }
    }
    unsafe extern "win64" fn mock_f750(_r15: usize, r14d: u32) -> u32 {
        if r14d == 7 { 1 } else { 0 }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4222_ec60, mock_ec60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4222_f750, mock_f750 as *const ())?;

    let g_ed0 = exe::ptr::<usize>(0x1_4397_fed0) as usize;
    let g_f48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let s_ed0 = rd::<usize>(g_ed0);
    let s_f48 = rd::<usize>(g_f48);

    let run = |f: F, edi_val: u32| -> ([u8; 0x90], [u8; 32], u64, u64, u64) {
        let mut dst_5a8 = [0u8; 0x90];
        let mut arr_130 = [0xffu8; 32];
        let arr_1a8: [u8; 8] = [1, 2, 7, 7, 5, 7, 0, 0];
        let mut bits_698: u64 = 0;
        let mut bits_668: u64 = 0;
        let mut bits_38: u64 = 0;
        let list_80: [u32; 4] = [0xffff_ffff, 1, 3, 5];

        let mut r15_arr = [0u8; 0xa0 * 2];
        let r15p = (r15_arr.as_mut_ptr() as usize) + 0xa0;
        wr::<usize>(r15p + 0x80, list_80.as_ptr() as usize);
        wr::<u32>(r15p + 0x88, 4);
        wr::<u32>(r15p + 0x98, 1);

        let map_100: [u32; 2] = [0, 2];
        let mut obj_ed0 = [0u8; 0x130];
        let ed0p = obj_ed0.as_mut_ptr() as usize;
        wr::<usize>(ed0p + 0x100, map_100.as_ptr() as usize);
        wr::<usize>(ed0p + 0x120, r15_arr.as_mut_ptr() as usize);
        wr::<usize>(g_ed0, ed0p);

        let mut obj_f48 = [0u8; 0x6a8];
        let f48p = obj_f48.as_mut_ptr() as usize;
        wr::<usize>(f48p + 0x38, (&mut bits_38 as *mut u64) as usize);
        wr::<usize>(f48p + 0x130, arr_130.as_mut_ptr() as usize);
        wr::<usize>(f48p + 0x1a8, arr_1a8.as_ptr() as usize);
        wr::<usize>(f48p + 0x5a8, dst_5a8.as_mut_ptr() as usize);
        wr::<usize>(f48p + 0x668, (&mut bits_668 as *mut u64) as usize);
        wr::<usize>(f48p + 0x698, (&mut bits_698 as *mut u64) as usize);
        wr::<usize>(g_f48, f48p);

        let mut src_data = [0u8; 0x60];
        for i in 0..0x60usize {
            src_data[i] = (i as u8).wrapping_add(0x10);
        }
        let mut rdi_desc = [0u64; 2];
        let rdip = rdi_desc.as_mut_ptr() as usize;
        wr::<usize>(rdip, src_data.as_ptr() as usize);
        wr::<u32>(rdip + 0x08, 2);
        let p2: [usize; 1] = [rdip];

        let mut p1 = [0u8; 0x60];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x48, edi_val);
        wr::<u32>(p1p + 0x50, 1);

        f(p1p, p2.as_ptr() as usize);
        (dst_5a8, arr_130, bits_698, bits_668, bits_38)
    };

    for edi_val in [2u32, 0u32] {
        let o = run(f_o, edi_val);
        let n = run(f_n, edi_val);
        wr::<usize>(g_ed0, s_ed0);
        wr::<usize>(g_f48, s_f48);
        crate::diff_eq!(o, n, "142302e60 edi={}", edi_val);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1423076a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCRATCH_MEM: [u64; 32] = [0; 32];
    static mut LOG_HASH: u64 = 0;

    unsafe extern "win64" fn mock_c0a0(bytes: u64, align: u32) -> usize {
        LOG_HASH = LOG_HASH.rotate_left(5) ^ bytes ^ (align as u64);
        core::ptr::addr_of_mut!(SCRATCH_MEM) as usize
    }
    unsafe extern "win64" fn mock_fb50(
        obj: usize,
        idx: u32,
        r8d: u32,
        _str_ptr: usize,
        kind: u64,
        r13: u64,
        zero: u64,
    ) -> u64 {
        let tag = rd::<u32>(obj) as u64;
        LOG_HASH = LOG_HASH
            .rotate_left(3)
            .wrapping_add(tag ^ (idx as u64) ^ (r8d as u64) ^ kind ^ r13 ^ zero);
        if (idx + (kind as u32)) & 1 == 1 {
            0x7000 + (idx as u64) * 0x10 + kind
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_1130(g_ptr: usize, buf: usize, count: u64, kind: u32) {
        let gid = rd::<u32>(g_ptr) as u64;
        LOG_HASH = LOG_HASH.rotate_left(7).wrapping_add(gid ^ count ^ (kind as u64));
        for i in 0..(count as usize) {
            let v0 = rd::<u32>(buf + i * 16) as u64;
            let v1 = rd::<u64>(buf + i * 16 + 8);
            LOG_HASH = LOG_HASH.rotate_left(11).wrapping_add(v0 ^ v1);
        }
    }
    unsafe extern "win64" fn mock_7d90(
        _p1: usize,
        p2: u64,
        r8d: u32,
        r9d: u32,
        g_ptr: usize,
        _str_ptr: usize,
        a7: u64,
        a8: u8,
        buf: usize,
    ) {
        let gid = rd::<u32>(g_ptr) as u64;
        let ok_buf = (buf == core::ptr::addr_of_mut!(SCRATCH_MEM) as usize) as u64;
        LOG_HASH = LOG_HASH.rotate_left(13).wrapping_add(
            p2 ^ (r8d as u64) ^ (r9d as u64) ^ gid ^ a7 ^ (a8 as u64) ^ ok_buf,
        );
    }
    unsafe extern "win64" fn mock_c190(buf: usize) {
        let ok_buf = (buf == core::ptr::addr_of_mut!(SCRATCH_MEM) as usize) as u64;
        LOG_HASH = LOG_HASH.rotate_left(17).wrapping_add(ok_buf + 0x55);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4232_fb50, mock_fb50 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4232_1130, mock_1130 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4230_7d90, mock_7d90 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let g_base = exe::ptr::<u8>(0x1_4397_ff90) as *mut u8;
    let snap = crate::difftest::Snapshot::take(g_base, 0x50);

    let run = |f: F| -> (u64, [u32; 12]) {
        LOG_HASH = 0;
        SCRATCH_MEM = [0; 32];

        let globals_vas: [u64; 9] = [
            0x1_4397_ff90,
            0x1_4397_ff98,
            0x1_4397_ffa0,
            0x1_4397_ffa8,
            0x1_4397_ffb8,
            0x1_4397_ffc0,
            0x1_4397_ffc8,
            0x1_4397_ffd0,
            0x1_4397_ffd8,
        ];
        let mut g_structs = [[0u64; 2]; 9];
        for (idx, &va) in globals_vas.iter().enumerate() {
            let gp = g_structs[idx].as_mut_ptr() as usize;
            wr::<u32>(gp, (idx as u32) + 1);
            wr::<u64>(gp + 0x08, 0x1000 + (idx as u64) * 0x10);
            wr::<usize>(exe::ptr::<usize>(va) as usize, gp);
        }

        let mut objs = [[0u8; 0x140]; 3];
        for i in 0..3usize {
            let op = objs[i].as_mut_ptr() as usize;
            wr::<u32>(op, 0x100 + i as u32);
            wr::<i32>(op + 0x138, 2);
        }

        let mut sub_arrs: [u32; 12] = [
            0, 101, 0, 102, // item 0
            0, 201, 0, 202, // item 1
            0, 301, 0, 302, // item 2
        ];

        let mut items = [0u8; 0x18 * 3];
        let ip = items.as_mut_ptr() as usize;
        let item_flags: [u32; 3] = [0x0, 0x4, 0x100];
        for i in 0..3usize {
            let cur = ip + i * 0x18;
            wr::<u32>(cur, item_flags[i]);
            wr::<u32>(cur + 0x04, 0x50 + i as u32);
            wr::<usize>(cur + 0x08, objs[i].as_mut_ptr() as usize);
            wr::<usize>(cur + 0x10, (sub_arrs.as_mut_ptr() as usize) + i * 16);
        }

        f(ip, 3, 8);
        (LOG_HASH, sub_arrs)
    };

    let o = run(f_o);
    let n = run(f_n);
    snap.restore();

    crate::diff_eq!(o, n, "1423076a0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14230ae90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_f48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let g_398 = exe::ptr::<i32>(0x1_4386_c398) as usize;
    let s_f48 = rd::<usize>(g_f48);
    let s_398 = rd::<i32>(g_398);

    let run = |f: F, bias: i32| -> [u8; 8] {
        wr::<i32>(g_398, bias);

        let arr_2a0: [u8; 5] = [10, 50, 200, 30, 90];
        let arr_2b0: [u32; 5] = [0, 0xffff_ffff, 2, 3, 4];
        let mut r10_obj = [0u8; 0x2c0];
        let r10p = r10_obj.as_mut_ptr() as usize;
        wr::<usize>(r10p + 0x2a0, arr_2a0.as_ptr() as usize);
        wr::<usize>(r10p + 0x2b0, arr_2b0.as_ptr() as usize);
        wr::<u32>(r10p + 0x2b8, 5);
        wr::<usize>(g_f48, r10p);

        let src: [u8; 8] = [100, 100, 100, 20, 250, 77, 88, 99];
        let mut dst = [0u8; 8];
        let mut p1 = [0u8; 0x40];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x10, dst.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x30, src.as_ptr() as usize);
        wr::<u32>(p1p + 0x38, 8);

        f(p1p);
        dst
    };

    for bias in [20i32, -100i32] {
        let o = run(f_o, bias);
        let n = run(f_n, bias);
        wr::<usize>(g_f48, s_f48);
        wr::<i32>(g_398, s_398);
        crate::diff_eq!(o, n, "14230ae90 bias={}", bias);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14232beb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: u64 = 0;

    unsafe extern "win64" fn mock_ba80(vec_ptr: usize, new_cap: u64) {
        CALL_TRACE = CALL_TRACE.rotate_left(5).wrapping_add(new_cap);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_58a0(_p428: usize, count: u32) {
        CALL_TRACE = CALL_TRACE.rotate_left(7).wrapping_add(count as u64);
    }
    unsafe extern "win64" fn mock_46b0(vec_ptr: usize, new_cap: u64) {
        CALL_TRACE = CALL_TRACE.rotate_left(11).wrapping_add(new_cap);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411d_ba80, mock_ba80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4220_58a0, mock_58a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4087_46b0, mock_46b0 as *const ())?;

    let g_f48 = exe::ptr::<usize>(0x1_4397_ff48) as usize;
    let s_f48 = rd::<usize>(g_f48);

    let run = |f: F| -> (u64, u32, u32, [u32; 8], [u64; 3]) {
        CALL_TRACE = 0;

        let mut leaf_a = [0u8; 0xc0];
        wr::<u8>(leaf_a.as_mut_ptr() as usize + 0xbb, 2);
        let mut leaf_b = [0u8; 0xc0];
        wr::<u8>(leaf_b.as_mut_ptr() as usize + 0xb8, 1);
        let mut leaf_skip = [0u8; 0xc0];
        wr::<u8>(leaf_skip.as_mut_ptr() as usize + 0xbb, 1);

        let list_main: [usize; 3] = [
            leaf_a.as_mut_ptr() as usize,
            leaf_skip.as_mut_ptr() as usize,
            leaf_b.as_mut_ptr() as usize,
        ];
        let list_sub: [usize; 2] = [leaf_a.as_mut_ptr() as usize, leaf_b.as_mut_ptr() as usize];

        let mut sub_c0 = [0u8; 0x40];
        wr::<usize>(sub_c0.as_mut_ptr() as usize + 0x30, list_sub.as_ptr() as usize);
        wr::<u32>(sub_c0.as_mut_ptr() as usize + 0x38, 2);

        let mut sub_elem = [0u8; 0xd0];
        wr::<usize>(sub_elem.as_mut_ptr() as usize + 0xc0, sub_c0.as_mut_ptr() as usize);
        let list_20: [usize; 1] = [sub_elem.as_mut_ptr() as usize];

        let mut main_c0 = [0u8; 0x40];
        let mc0p = main_c0.as_mut_ptr() as usize;
        wr::<usize>(mc0p + 0x20, list_20.as_ptr() as usize);
        wr::<u32>(mc0p + 0x28, 1);
        wr::<usize>(mc0p + 0x30, list_main.as_ptr() as usize);
        wr::<u32>(mc0p + 0x38, 3);

        let mut main_elem = [0u8; 0xd0];
        wr::<usize>(main_elem.as_mut_ptr() as usize + 0xc0, mc0p);
        let arr_160: [usize; 1] = [main_elem.as_mut_ptr() as usize];
        let mut arr_170 = [0u64; 3];
        arr_170[1] = arr_160.as_ptr() as u64;

        let mut rbx_obj = [0u8; 0x110];
        let rbxp = rbx_obj.as_mut_ptr() as usize;
        wr::<u32>(rbxp + 0xa8, 3);
        wr::<f32>(rbxp + 0xd4, 4.0);
        wr::<f32>(rbxp + 0x100, 16.0);
        let arr_130: [usize; 1] = [rbxp];

        let mut buf_448 = [0u64; 3 * 8];
        let mut buf_438 = [0u32; 8];
        let mut table_250 = [0u64; 6];
        let mut f48_obj = [0u8; 0x260];
        wr::<usize>(
            f48_obj.as_mut_ptr() as usize + 0x250,
            table_250.as_mut_ptr() as usize,
        );
        wr::<usize>(g_f48, f48_obj.as_mut_ptr() as usize);

        let mut p1 = [0u8; 0x460];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x130, arr_130.as_ptr() as usize);
        wr::<u32>(p1p + 0x138, 1);
        wr::<usize>(p1p + 0x160, arr_160.as_ptr() as usize);
        wr::<usize>(p1p + 0x170, arr_170.as_ptr() as usize);
        wr::<u32>(p1p + 0x178, 1);
        wr::<u32>(p1p + 0x26c, 1);
        wr::<f32>(p1p + 0x280, 2.0);
        wr::<usize>(p1p + 0x428, 0x8000_0000);
        wr::<u16>(p1p + 0x432, 5);
        wr::<usize>(p1p + 0x438, buf_438.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x440, 0);
        wr::<u32>(p1p + 0x444, 1); // force grow via 0x1408746b0
        wr::<usize>(p1p + 0x448, buf_448.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x450, 0);
        wr::<u32>(p1p + 0x454, 1); // force grow via 0x1411dba80
        wr::<u32>(p1p + 0x458, 99);

        f(p1p);

        let mut entry1: [u64; 3] = [table_250[3], table_250[4], 0];
        if table_250[5] == buf_438.as_ptr() as u64 {
            entry1[2] = 1;
        }
        (
            CALL_TRACE,
            rd::<u32>(p1p + 0x440),
            rd::<u32>(p1p + 0x450),
            buf_438,
            entry1,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_f48, s_f48);

    crate::diff_eq!(o, n, "14232beb0 out");
    Ok(1)
}

pub unsafe fn diff_rend_142337130(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut INT_SEQ: [i32; 16] = [0; 16];
    static mut INT_POS: usize = 0;
    static mut F32_POS: u32 = 0;
    static mut U32_POS: u32 = 0;
    static mut STR_POS: usize = 0;
    static mut SLOTS_BUF: [u8; 32 * 12] = [0; 32 * 12];
    static mut SLOT_IDS: [u32; 12] = [0; 12];
    static mut SLOT_CNT: usize = 0;
    static mut TRACE_CALLS: u64 = 0;

    unsafe extern "win64" fn mock_v00(_this: usize, out: usize) {
        wr::<u64>(out, 0x1122_3344_5566_7788);
    }
    unsafe extern "win64" fn mock_v20(_this: usize, rstr: usize) {
        let idx = STR_POS;
        STR_POS += 1;
        // Alternate inline and heap RString representation
        if idx & 1 == 0 {
            wr::<u32>(rstr, (0x08u32 << 24) | 3);
            wr::<[u8; 4]>(rstr + 8, [b'a' + (idx as u8), b'b', b'c', 0]);
        } else {
            static HEAP_STR: [u8; 8] = *b"longkey\0";
            wr::<u32>(rstr, (0x01u32 << 24) | 7);
            wr::<usize>(rstr + 8, HEAP_STR.as_ptr() as usize);
        }
    }
    unsafe extern "win64" fn mock_v38(_this: usize, out: usize) {
        F32_POS += 1;
        wr::<f32>(out, (F32_POS as f32) * 1.5);
    }
    unsafe extern "win64" fn mock_v40(_this: usize, out: usize) {
        U32_POS += 1;
        wr::<u32>(out, 0x4000 + U32_POS);
    }
    unsafe extern "win64" fn mock_v48(_this: usize, out: usize) {
        let v = INT_SEQ[INT_POS];
        INT_POS += 1;
        wr::<i32>(out, v);
    }
    unsafe extern "win64" fn mock_v80(_this: usize, out: usize) {
        wr::<u8>(out, 0x5a);
    }
    unsafe extern "win64" fn mock_f8d0(_p3: usize, cap: u64) {
        TRACE_CALLS = TRACE_CALLS.rotate_left(5) ^ cap;
    }
    unsafe extern "win64" fn mock_19e0(_p3_10: usize, cap: u64) {
        TRACE_CALLS = TRACE_CALLS.rotate_left(7) ^ (cap + 0x100);
    }
    unsafe extern "win64" fn mock_ea70(out_ptr: usize, desc_ptr: usize) {
        let s_ptr = rd::<usize>(desc_ptr);
        let s_len = rd::<usize>(desc_ptr + 8);
        let b0 = rd::<u8>(s_ptr) as u32;
        wr::<u32>(out_ptr, (b0 << 8) | (s_len as u32));
    }
    unsafe extern "win64" fn mock_1210(_p3: usize, name_id: u32) -> usize {
        let idx = SLOT_CNT;
        SLOT_CNT += 1;
        SLOT_IDS[idx] = name_id;
        (core::ptr::addr_of_mut!(SLOTS_BUF) as usize) + idx * 32
    }
    unsafe extern "win64" fn mock_ab10(_rstr: usize) {
        TRACE_CALLS = TRACE_CALLS.wrapping_add(1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_405a_f8d0, mock_f8d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4131_19e0, mock_19e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4220_ea70, mock_ea70 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4131_1210, mock_1210 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let run = |f: F| -> (u64, usize, [u32; 12], [u8; 32 * 12], [u32; 8]) {
        // Count = 10: kinds 0, 1, 2, 3, 4, 5, 6, 0x0c, 0x10 (1 int), 0x11 (2 ints)
        INT_SEQ = [10, 0, 1, 2, 3, 4, 5, 6, 0x0c, 0x10, 777, 0x11, 888, 999, 0, 0];
        INT_POS = 0;
        F32_POS = 0;
        U32_POS = 0;
        STR_POS = 0;
        SLOTS_BUF = [0; 32 * 12];
        SLOT_IDS = [0; 12];
        SLOT_CNT = 0;
        TRACE_CALLS = 0;

        let mut vtbl = [0usize; 20];
        vtbl[0x00 / 8] = mock_v00 as *const () as usize;
        vtbl[0x20 / 8] = mock_v20 as *const () as usize;
        vtbl[0x38 / 8] = mock_v38 as *const () as usize;
        vtbl[0x40 / 8] = mock_v40 as *const () as usize;
        vtbl[0x48 / 8] = mock_v48 as *const () as usize;
        vtbl[0x80 / 8] = mock_v80 as *const () as usize;

        let p2: [usize; 1] = [vtbl.as_ptr() as usize];
        let mut p3 = [0u32; 8];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u32>(p3p + 0x08, 99);
        wr::<u32>(p3p + 0x0c, 2);
        wr::<u32>(p3p + 0x18, 99);
        wr::<u32>(p3p + 0x1c, 2);

        f(0, p2.as_ptr() as usize, p3p);
        (TRACE_CALLS, SLOT_CNT, SLOT_IDS, SLOTS_BUF, p3)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142337130 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14237fff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;
    static mut BIT_BUF: [u64; 4] = [0; 4];
    static mut SLOTS_R14: [u8; 0x30 * 2] = [0; 0x30 * 2];

    unsafe extern "win64" fn mock_51f0(vec_ptr: usize, new_cap: u64) {
        TRACE = TRACE.rotate_left(3) ^ new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_4d40(local_68: usize, count: u32) {
        TRACE = TRACE.rotate_left(5) ^ (count as u64);
        wr::<usize>(local_68, core::ptr::addr_of_mut!(SLOTS_R14) as usize);
        wr::<u32>(local_68 + 0x08, count);
        wr::<u32>(local_68 + 0x0c, count);
    }
    unsafe extern "win64" fn mock_c2c0(alloc_ptr: usize) {
        TRACE = TRACE.rotate_left(7) ^ (rd::<u32>(alloc_ptr) as u64);
    }
    unsafe extern "win64" fn mock_2520(alloc_ptr: usize, req: u32) -> u32 {
        let cur = rd::<u32>(alloc_ptr);
        wr::<u32>(alloc_ptr, cur.wrapping_add(req));
        cur
    }
    unsafe extern "win64" fn mock_7910(vec_ptr: usize, new_cap: u64) {
        TRACE = TRACE.rotate_left(11) ^ new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_ee30(vec_ptr: usize, new_cap: u64) {
        TRACE = TRACE.rotate_left(13) ^ new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_b830(vec_ptr: usize, new_cap: u64) {
        TRACE = TRACE.rotate_left(17) ^ new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_bcb0(vec_ptr: usize, new_cap: u64) {
        TRACE = TRACE.rotate_left(19) ^ new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_4b30(vec_ptr: usize, new_cap: u64) {
        TRACE = TRACE.rotate_left(23) ^ new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_f1e0(slot_18: usize, bits: u64) {
        TRACE = TRACE.rotate_left(29) ^ bits;
        wr::<usize>(slot_18, core::ptr::addr_of_mut!(BIT_BUF) as usize);
    }
    unsafe extern "win64" fn mock_eb40(vec_ptr: usize, new_cap: u64) {
        TRACE = TRACE.rotate_left(31) ^ new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_5010(out_quat: usize, src_xform: usize) {
        let s = rd::<u32>(src_xform);
        wr::<[u32; 4]>(out_quat, [s + 1, s + 2, s + 3, s + 4]);
    }
    unsafe extern "win64" fn mock_c320(vec_ptr: usize, new_cap: u64) {
        TRACE = TRACE.rotate_left(37) ^ new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_e8d0(_vec_5d8: usize, val_ptr: usize) {
        let v = rd::<u32>(val_ptr) as u64;
        TRACE = TRACE.rotate_left(41).wrapping_add(v);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4135_51f0, mock_51f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4135_4d40, mock_4d40 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4237_c2c0, mock_c2c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4135_2520, mock_2520 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_402d_7910, mock_7910 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4062_ee30, mock_ee30 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_40a5_b830, mock_b830 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4020_bcb0, mock_bcb0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_4135_4b30, mock_4b30 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_f1e0 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_401d_eb40, mock_eb40 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4027_5010, mock_5010 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_401f_c320, mock_c320 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_401d_e8d0, mock_e8d0 as *const ())?;

    let run = |f: F, use_freelist: bool| -> (u64, u32, [u32; 26], [u32; 8], [u32; 24], [u32; 8], u8) {
        TRACE = 0;
        BIT_BUF = [0; 4];
        SLOTS_R14 = [0; 0x30 * 2];

        let mut free_arr: [u32; 2] = [0, 0];
        let mut buf_538 = [0u64; 4];
        buf_538[0] = core::ptr::addr_of_mut!(SLOTS_R14) as usize as u64;

        let mut buf_5a8 = [0u32; 26]; // 2 * 0x34 bytes
        let mut buf_5b8 = [0u32; 8];
        let mut buf_5c8 = [0u64; 4];
        let mut buf_5e8 = [0u32; 8];
        let mut buf_5f8 = [0u8; 64];
        let mut buf_620 = [0u32; 8];
        let mut buf_648 = [0u32; 24];

        let mut p1 = [0u8; 0x700];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x520, 10);
        wr::<usize>(p1p + 0x528, free_arr.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x530, if use_freelist { 1 } else { 0 });
        wr::<usize>(p1p + 0x538, buf_538.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x540, 0);
        wr::<u32>(p1p + 0x544, 0);
        wr::<usize>(p1p + 0x5a8, buf_5a8.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x5b8, buf_5b8.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x5c8, buf_5c8.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x5e0, 0x77);
        wr::<usize>(p1p + 0x5e8, buf_5e8.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x5f8, buf_5f8.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x608, 2);
        wr::<usize>(p1p + 0x620, buf_620.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x630, 1);
        wr::<usize>(p1p + 0x648, buf_648.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x658, 1);
        wr::<u32>(p1p + 0x6dc, 50);

        let xforms: [u32; 24] = [
            100, 0, 0, 0, 0, 0, 0, 0, 0, 101, 102, 103, // xform 0
            200, 0, 0, 0, 0, 0, 0, 0, 0, 201, 202, 203, // xform 1
        ];
        let parents: [i32; 2] = [-1, 0];
        let counts_38: [u32; 2] = [2, 1];
        let bits_src: [u64; 1] = [0xfeed_cafe_1234_5678];
        let data_60: [u32; 3] = [11, 22, 33];

        let mut r12_elem = [0u8; 0x70];
        let r12p = r12_elem.as_mut_ptr() as usize;
        wr::<u32>(r12p, 2);
        wr::<usize>(r12p + 0x08, xforms.as_ptr() as usize);
        wr::<usize>(r12p + 0x18, parents.as_ptr() as usize);
        wr::<usize>(r12p + 0x38, counts_38.as_ptr() as usize);
        wr::<usize>(r12p + 0x48, bits_src.as_ptr() as usize);
        wr::<u64>(r12p + 0x58, 40);
        wr::<usize>(r12p + 0x60, data_60.as_ptr() as usize);
        wr::<u32>(r12p + 0x68, 3);

        let sub_2b0_0: [u32; 2] = [0xaa, 0xbb];
        let sub_2b0_10: [u32; 2] = [0x111, 0x222];
        let mut elem_2b0 = [0u8; 0x20];
        let e2b0p = elem_2b0.as_mut_ptr() as usize;
        wr::<usize>(e2b0p, sub_2b0_0.as_ptr() as usize);
        wr::<usize>(e2b0p + 0x10, sub_2b0_10.as_ptr() as usize);
        wr::<u32>(e2b0p + 0x18, 2);

        let src_1a0: [u32; 4] = [0x10, 0x20, 0x30, 0x40];
        let src_1b0: [u32; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let src_2c8: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

        let mut p2 = [0u8; 0x2e0];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x1a0, src_1a0.as_ptr() as usize);
        wr::<usize>(p2p + 0x1b0, src_1b0.as_ptr() as usize);
        wr::<u32>(p2p + 0x1c8, 1);
        wr::<u32>(p2p + 0x1e8, 0);
        wr::<u8>(p2p + 0x272, 1);
        wr::<usize>(p2p + 0x2a0, r12p);
        wr::<u8>(p2p + 0x2a8, 1);
        wr::<usize>(p2p + 0x2b0, e2b0p);
        wr::<i32>(p2p + 0x2c0, 8);
        wr::<usize>(p2p + 0x2c8, src_2c8.as_ptr() as usize);
        wr::<u32>(p2p + 0x2d0, 16);

        let filter_idxs: [u32; 1] = [1];
        let p3: [usize; 2] = [1, filter_idxs.as_ptr() as usize];

        f(p1p, p2p, p3.as_ptr() as usize);
        (
            TRACE,
            rd::<u32>(p2p + 0x48),
            buf_5a8,
            buf_5b8,
            buf_648,
            buf_5e8,
            rd::<u8>(p1p + 0x6e5),
        )
    };

    for use_fl in [false, true] {
        let o = run(f_o, use_fl);
        let n = run(f_n, use_fl);
        crate::diff_eq!(o, n, "14237fff0 use_fl={}", use_fl);
    }
    Ok(2)
}

// REGISTRY:
// Replacement::verified(0x1_4220_ea10, "unknown_14220ea10", rend::cauldron_hot_v6::unknown_14220ea10 as *const (), rend::cauldron_hot_v6::diff_unknown_14220ea10),
// Replacement::verified(0x1_4222_9a00, "unknown_142229a00", rend::cauldron_hot_v6::unknown_142229a00 as *const (), rend::cauldron_hot_v6::diff_unknown_142229a00),
// Replacement::verified(0x1_4224_d9d0, "unknown_14224d9d0", rend::cauldron_hot_v6::unknown_14224d9d0 as *const (), rend::cauldron_hot_v6::diff_unknown_14224d9d0),
// Replacement::verified(0x1_4230_2e60, "unknown_142302e60", rend::cauldron_hot_v6::unknown_142302e60 as *const (), rend::cauldron_hot_v6::diff_unknown_142302e60),
// Replacement::verified_throws(0x1_4230_76a0, "unknown_1423076a0", rend::cauldron_hot_v6::unknown_1423076a0 as *const (), Throws::Discard, rend::cauldron_hot_v6::diff_unknown_1423076a0),
// Replacement::verified(0x1_4230_ae90, "unknown_14230ae90", rend::cauldron_hot_v6::unknown_14230ae90 as *const (), rend::cauldron_hot_v6::diff_unknown_14230ae90),
// Replacement::verified_throws(0x1_4232_beb0, "unknown_14232beb0", rend::cauldron_hot_v6::unknown_14232beb0 as *const (), Throws::Caught, rend::cauldron_hot_v6::diff_unknown_14232beb0),
// Replacement::verified(0x1_4233_7130, "rend_142337130", rend::cauldron_hot_v6::rend_142337130 as *const (), rend::cauldron_hot_v6::diff_rend_142337130),
// Replacement::verified(0x1_4237_fff0, "unknown_14237fff0", rend::cauldron_hot_v6::unknown_14237fff0 as *const (), rend::cauldron_hot_v6::diff_unknown_14237fff0),
