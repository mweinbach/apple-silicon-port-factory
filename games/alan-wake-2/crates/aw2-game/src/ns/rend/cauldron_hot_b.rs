//! Decompiled `rend` Cauldron ECS / physics / transform hot-path functions
//! (`0x141e98490`, `0x141e98960`, `0x141e98c80`, `0x141eb1f70`, `0x141e22480`, `0x141eac4e0`).

use crate::exe::{self, rd, wr};

#[inline(always)]
fn vrsqrtps_128(a: u128) -> u128 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{__m128, _mm_rsqrt_ps};
        let v: __m128 = core::mem::transmute(a);
        core::mem::transmute(_mm_rsqrt_ps(v))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::simd_ops::vrsqrtps(a)
    }
}

#[inline(always)]
fn vfmadd231ps_128(acc: u128, a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for lane in 0..4 {
        let fa = f32::from_bits(((a >> (lane * 32)) & 0xffff_ffff) as u32);
        let fb = f32::from_bits(((b >> (lane * 32)) & 0xffff_ffff) as u32);
        let fc = f32::from_bits(((acc >> (lane * 32)) & 0xffff_ffff) as u32);
        let r = fa.mul_add(fb, fc);
        out |= (r.to_bits() as u128) << (lane * 32);
    }
    out
}

#[inline(always)]
fn vfnmadd231ps_128(acc: u128, a: u128, b: u128) -> u128 {
    let mut out = 0u128;
    for lane in 0..4 {
        let fa = f32::from_bits(((a >> (lane * 32)) & 0xffff_ffff) as u32);
        let fb = f32::from_bits(((b >> (lane * 32)) & 0xffff_ffff) as u32);
        let fc = f32::from_bits(((acc >> (lane * 32)) & 0xffff_ffff) as u32);
        let r = (-fa).mul_add(fb, fc);
        out |= (r.to_bits() as u128) << (lane * 32);
    }
    out
}

/// `unknown_141e98490`
/// Original: 0x141e98490 (1219 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141e98490(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let mut vecs = [0u64; 6];
    let world = rd::<usize>(param_1);
    let r11 = world.wrapping_add(0x40);
    let mut span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter = [0u64; 3];
    let _ = exe::call_va!(
        0x1_401e_7c10,
        (iter.as_mut_ptr() as usize, r11, span.as_mut_ptr() as usize) -> usize
    );

    let mut outer = [0u64; 11];
    outer[0] = r11 as u64;
    outer[1] = iter[0];
    outer[2] = iter[1];
    outer[3] = iter[2];
    outer[5] = 0;
    outer[6] = 0;
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>(r11.wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>(r11.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_4100_84e0, (outer.as_mut_ptr() as usize) -> ());

    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let r12 = outer[6] as usize;
            let r15 = (outer[7] as usize).wrapping_add(r12.wrapping_mul(0x20));
            let comp_b = outer[8] as usize;
            let comp_dw = outer[9] as usize;
            let rsi = (outer[10] as usize).wrapping_add(r12.wrapping_mul(0x10));

            let active: u8 = if rd::<u32>(comp_dw.wrapping_add(r12 * 4)) == 0
                && rd::<u8>(comp_b.wrapping_add(r12)) != 0
            {
                1
            } else {
                0
            };

            if active != rd::<u8>(r15 + 0x18) {
                let r13 = rd::<usize>(r15);
                if active != 0 {
                    let b114 = rd::<u8>(r13 + 0x114) | 1;
                    wr::<u8>(r13 + 0x114, b114);
                    if (b114 & 2) == 0 {
                        exe::call_va!(0x1_4215_e510, (r13, vecs.as_mut_ptr() as usize) -> ());
                    } else {
                        exe::call_va!(0x1_4215_e100, (r13, vecs.as_mut_ptr() as usize, 0u64) -> ());
                    }

                    let mut cur_sub = rd::<usize>(rsi);
                    let sub_cnt = rd::<u32>(rsi + 0x08) as usize;
                    let end_sub = cur_sub.wrapping_add(sub_cnt.wrapping_mul(0x40));
                    while cur_sub != end_sub {
                        let mut key = rd::<u64>(cur_sub);
                        let slot_idx: u64 = if rd::<u64>(param_3 + 0x38) == rd::<u64>(param_3 + 0x30)
                        {
                            rd::<u64>(param_3 + 0x40)
                        } else {
                            let mut found_idx: i64 = 0;
                            let _ = exe::call_va!(
                                0x1_401f_a730,
                                (
                                    param_3,
                                    (&mut found_idx as *mut i64) as usize,
                                    (&mut key as *mut u64) as usize
                                ) -> usize
                            );
                            if found_idx == -1 {
                                rd::<u64>(param_3 + 0x40)
                            } else {
                                found_idx as u64
                            }
                        };

                        let base_58 = rd::<usize>(param_3 + 0x58);
                        let entry_ptr = base_58.wrapping_add((slot_idx as usize).wrapping_mul(0x10));
                        let end_entry_ptr =
                            base_58.wrapping_add((rd::<usize>(param_3 + 0x40)).wrapping_mul(0x10));
                        if entry_ptr != end_entry_ptr {
                            let r8_val = rd::<u64>(entry_ptr + 0x08);
                            if (r8_val as u32) != 0xffff_ffff {
                                let ent_idx = (r8_val as u32) as usize;
                                let tbl_30 = rd::<usize>(world + 0x30);
                                let rec = rd::<u64>(tbl_30.wrapping_add(ent_idx * 8));
                                let dx = rec as u16;
                                if dx == 0xffff {
                                    let _ = exe::call_iat!(0x1_4264_67e8, () -> u64);
                                    unreachable!();
                                }

                                let mut first = rd::<usize>(param_1 + 0x10);
                                let len_u16 = rd::<usize>(param_1 + 0x08);
                                let last = first.wrapping_add(len_u16.wrapping_mul(2));
                                let mut rem = (last.wrapping_sub(first) as isize) >> 1;
                                while rem > 0 {
                                    let half = (rem as usize) >> 1;
                                    let mid = first.wrapping_add(half * 2);
                                    if rd::<u16>(mid) < dx {
                                        first = mid.wrapping_add(2);
                                        rem += -1 - (half as isize);
                                    } else {
                                        rem = half as isize;
                                    }
                                }

                                if first != last && dx >= rd::<u16>(first) {
                                    let tbl_30_2 = rd::<usize>(world + 0x30);
                                    let r15_rec = rd::<u64>(tbl_30_2.wrapping_add(ent_idx * 8));
                                    if (r15_rec as u16) == 0xffff {
                                        let _ = exe::call_iat!(0x1_4264_67e8, () -> u64);
                                        unreachable!();
                                    }
                                    let arch_idx = (r15_rec as u16) as usize;
                                    let r11_meta = world.wrapping_add(arch_idx * 0x20);
                                    let col_id =
                                        rd::<u16>(exe::ptr::<u16>(0x1_43c9_14bc) as usize) as u32;
                                    let col_pos = exe::call_va!(
                                        0x1_4248_9ee0,
                                        (
                                            rd::<u64>(r11_meta + 0x18098),
                                            rd::<u32>(r11_meta + 0x180ac),
                                            col_id
                                        ) -> i32
                                    );
                                    let col_offs = rd::<usize>(r11_meta + 0x180a0);
                                    let col_off = rd::<u32>(
                                        col_offs.wrapping_add(((col_pos as isize) * 4) as usize),
                                    ) as usize;
                                    let chunk_base =
                                        rd::<usize>(r11.wrapping_add(arch_idx * 8).wrapping_add(0x48));
                                    let row_off = ((r15_rec >> 32) as usize).wrapping_mul(0x20);
                                    let target_obj =
                                        rd::<usize>(chunk_base.wrapping_add(col_off).wrapping_add(row_off));
                                    let target_f20 = rd::<u64>(target_obj + 0x20);

                                    let p4_inner = rd::<usize>(rd::<usize>(param_4));
                                    let p4_vtbl = rd::<usize>(p4_inner);
                                    let arg3 = rd::<u32>(cur_sub + 0x08) as u64;
                                    let arg5 = rd::<u32>(cur_sub + 0x0c) as u64;
                                    let arg6 = cur_sub.wrapping_add(0x10);
                                    exe::call_va!(
                                        rd::<usize>(p4_vtbl + 0x10),
                                        (p4_inner, r13, arg3, target_f20, arg5, arg6) -> ()
                                    );

                                    if (rd::<u8>(target_obj + 0x114) & 4) != 0 {
                                        let p4_inner2 = rd::<usize>(rd::<usize>(param_4));
                                        let p4_vtbl2 = rd::<usize>(p4_inner2);
                                        exe::call_va!(
                                            rd::<usize>(p4_vtbl2 + 0x18),
                                            (p4_inner2, target_f20, target_obj) -> ()
                                        );
                                    }
                                }
                            }
                        }
                        cur_sub = cur_sub.wrapping_add(0x40);
                    }

                    let p4_inner = rd::<usize>(rd::<usize>(param_4));
                    let p4_vtbl = rd::<usize>(p4_inner);
                    exe::call_va!(
                        rd::<usize>(p4_vtbl + 0x18),
                        (p4_inner, rd::<u64>(r13 + 0x20), r13) -> ()
                    );
                } else {
                    exe::call_va!(0x1_4216_5bf0, (r13) -> ());
                    wr::<u8>(r13 + 0x114, rd::<u8>(r13 + 0x114) & !1);
                    exe::call_va!(0x1_4215_e100, (r13, vecs.as_mut_ptr() as usize, 0u64) -> ());
                }
                wr::<u8>(r15 + 0x18, active);
            }

            outer[6] = (r12 + 1) as u64;
        }

        outer[2] = outer[2].wrapping_add(2);
        if outer[2] != outer[3] {
            let base_e0 = outer[1] as usize;
            while outer[2] != outer[3] {
                let arch_id = rd::<u16>(outer[2] as usize) as usize;
                if rd::<u64>(base_e0.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(base_e0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                outer[2] = outer[2].wrapping_add(2);
            }
            if outer[2] != outer[3] {
                let base_e8 = outer[0] as usize;
                let arch_id = rd::<u16>(outer[2] as usize) as usize;
                let cp = rd::<u64>(base_e8.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_e8.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
            }
        }
        exe::call_va!(0x1_4100_84e0, (outer.as_mut_ptr() as usize) -> ());
    }

    let slot_idx = rd::<u16>(exe::ptr::<u16>(0x1_43c9_398c) as usize) as usize;
    let tbl_580e0 = rd::<usize>(param_2 + 0x580e0);
    let slot_entry = rd::<usize>(tbl_580e0.wrapping_add(slot_idx * 8));
    let mut p2_cell = param_2;
    let fn_obj = rd::<usize>(slot_entry + 0x38);
    if fn_obj == 0 {
        let _ = exe::call_iat!(0x1_4264_5878, () -> u64);
        unreachable!();
    }
    let fn_vtbl = rd::<usize>(fn_obj);
    exe::call_va!(
        rd::<usize>(fn_vtbl + 0x10),
        (fn_obj, (&mut p2_cell as *mut usize) as usize, vecs.as_mut_ptr() as usize) -> ()
    );

    if vecs[4] != 0 {
        exe::call_va!(0x1_4241_c190, (vecs[4] as usize) -> ());
    }
    vecs[4] = 0;
    vecs[5] = 0;
    if vecs[2] != 0 {
        exe::call_va!(0x1_4241_c190, (vecs[2] as usize) -> ());
    }
    vecs[2] = 0;
    vecs[3] = 0;
    if vecs[0] != 0 {
        exe::call_va!(0x1_4241_c190, (vecs[0] as usize) -> ());
    }
}

/// `unknown_141e98960`
/// Original: 0x141e98960 (775 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e98960(param_1: usize) {
    let count = rd::<u64>(param_1 + 0x18) as usize;
    if count == 0 {
        return;
    }
    let mut r12 = 0usize;
    for r15 in 0..count {
        let arr_20 = rd::<usize>(param_1 + 0x20);
        let arr_28 = rd::<usize>(param_1 + 0x28);
        let arr_30 = rd::<usize>(param_1 + 0x30);
        let mut local_20: [u64; 3] = [arr_20 as u64, arr_28 as u64, arr_30 as u64];
        let r14 = arr_20.wrapping_add(r12);

        let al: u8 = if rd::<u32>(arr_28.wrapping_add(r15 * 4)) == 1
            || rd::<u8>(arr_30.wrapping_add(r15)) != 0
        {
            1
        } else {
            0
        };

        if rd::<u8>(r14 + 0x16) != al {
            let rdi = rd::<usize>(r14);
            wr::<u8>(r14 + 0x16, al);
            if al != 0 {
                exe::call_va!(0x1_4216_89d0, (rdi) -> ());
            } else {
                let b114 = rd::<u8>(rdi + 0x114);
                if (b114 & 8) != 0 {
                    let r8_sub = rd::<usize>(rdi + 0x20);
                    wr::<u8>(rdi + 0x114, b114 & !8);
                    if rd::<u64>(r8_sub + 0x70) != 0 {
                        let mut p = rd::<usize>(r8_sub + 0x68);
                        let n_words = rd::<u64>(r8_sub + 0x60) as usize;
                        let end_p = p.wrapping_add(n_words.wrapping_sub(1).wrapping_mul(8));
                        while p != end_p {
                            wr::<u64>(p, 0);
                            p = p.wrapping_add(8);
                        }
                        let last_p = rd::<usize>(r8_sub + 0x68)
                            .wrapping_add((rd::<usize>(r8_sub + 0x60)).wrapping_mul(8))
                            .wrapping_sub(8);
                        let bit_cnt = rd::<u64>(r8_sub + 0x70);
                        let sh = (!(bit_cnt.wrapping_sub(1)) & 0x3f) as u32;
                        let mask = !(u64::MAX >> sh);
                        wr::<u64>(last_p, rd::<u64>(last_p) & mask);
                    }

                    let sub20 = rd::<usize>(rdi + 0x20);
                    if rd::<u16>(sub20 + 0x90) != 0 {
                        let mut rsi_idx = 0usize;
                        let mut rbp_off = 0usize;
                        loop {
                            let c0_arr = rd::<usize>(rdi + 0xc0);
                            let actor = rd::<usize>(c0_arr.wrapping_add(rsi_idx * 8));
                            let rbx_actor = if rd::<u16>(actor + 0x08) == 6 { actor } else { 0 };
                            if rbx_actor != 0 {
                                let vtbl = rd::<usize>(rbx_actor);
                                let ok = exe::call_va!(
                                    rd::<usize>(vtbl + 0x1b8),
                                    (rbx_actor, local_20.as_mut_ptr() as usize) -> u8
                                );
                                if ok != 0 {
                                    let vtbl2 = rd::<usize>(rbx_actor);
                                    exe::call_va!(
                                        rd::<usize>(vtbl2 + 0xa8),
                                        (rbx_actor, local_20.as_mut_ptr() as usize, 1u64) -> ()
                                    );
                                }
                                let b8_arr = rd::<usize>(rdi + 0xb8);
                                let c0_arr2 = rd::<usize>(rdi + 0xc0);
                                let bit_idx = (rsi_idx & 0x3f) as u64;
                                let word_idx = rsi_idx >> 6;
                                let r9_bit =
                                    ((rd::<u8>(b8_arr.wrapping_add(rbp_off + 0x10)) >> 2) & 1) as u64;
                                let sub20_2 = rd::<usize>(rdi + 0x20);
                                let actor_arg = rd::<usize>(c0_arr2.wrapping_add(rsi_idx * 8));
                                let words_68 = rd::<usize>(sub20_2 + 0x68);
                                let word_val = rd::<u64>(words_68.wrapping_add(word_idx * 8));
                                let r8_arg = ((word_idx as u64) & !0xff)
                                    | (((word_val >> bit_idx) & 1) as u64);
                                exe::call_va!(
                                    0x1_4216_0980,
                                    (bit_idx, actor_arg, r8_arg, r9_bit) -> ()
                                );
                            }
                            rsi_idx += 1;
                            rbp_off += 0x14;
                            if rsi_idx == rd::<u16>(rd::<usize>(rdi + 0x20) + 0x90) as usize {
                                break;
                            }
                        }
                    }
                }

                let flag_17 = rd::<u8>(r14 + 0x17);
                let count_90 = rd::<u16>(rd::<usize>(rdi + 0x20) + 0x90) as usize;
                if flag_17 == 0 {
                    if count_90 != 0 {
                        let mut rbx_idx = 0usize;
                        loop {
                            exe::call_va!(0x1_4215_eb60, (rdi, rbx_idx as u32 as u64) -> ());
                            rbx_idx += 1;
                            if rbx_idx == rd::<u16>(rd::<usize>(rdi + 0x20) + 0x90) as usize {
                                break;
                            }
                        }
                    }
                } else if count_90 != 0 {
                    let mut rsi_idx = 0usize;
                    loop {
                        let c0_arr = rd::<usize>(rdi + 0xc0);
                        let actor = rd::<usize>(c0_arr.wrapping_add(rsi_idx * 8));
                        let rbx_actor = if rd::<u16>(actor + 0x08) == 6 { actor } else { 0 };
                        if rbx_actor != 0 {
                            let vtbl = rd::<usize>(rbx_actor);
                            let has_scene =
                                exe::call_va!(rd::<usize>(vtbl + 0x30), (rbx_actor) -> usize);
                            if has_scene != 0 {
                                let p28 = rd::<usize>(rdi + 0x28);
                                let p28_vtbl = rd::<usize>(p28);
                                let scene_obj =
                                    exe::call_va!(rd::<usize>(p28_vtbl + 0x48), (p28) -> usize);
                                let scene_vtbl = rd::<usize>(scene_obj);
                                exe::call_va!(
                                    rd::<usize>(scene_vtbl + 0x1b8),
                                    (scene_obj, rbx_actor) -> ()
                                );
                                let mut flags_out = 0u64;
                                let vtbl2 = rd::<usize>(rbx_actor);
                                let flags_ptr = exe::call_va!(
                                    rd::<usize>(vtbl2 + 0x178),
                                    (rbx_actor, (&mut flags_out as *mut u64) as usize) -> usize
                                );
                                if (rd::<u8>(flags_ptr) & 1) == 0 {
                                    let vtbl3 = rd::<usize>(rbx_actor);
                                    exe::call_va!(rd::<usize>(vtbl3 + 0x248), (rbx_actor) -> ());
                                }
                            }
                        }
                        rsi_idx += 1;
                        if rsi_idx == rd::<u16>(rd::<usize>(rdi + 0x20) + 0x90) as usize {
                            break;
                        }
                    }
                }
            }
        }

        let b_val = rd::<u8>(arr_30.wrapping_add(r15));
        r12 = r12.wrapping_add(0x20);
        wr::<u8>(r14 + 0x17, (b_val >> 1) & 1);
    }
}

/// `unknown_141e98c80`
/// Original: 0x141e98c80 (364 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141e98c80(param_1: usize) {
    if rd::<u64>(param_1 + 0x18) == 0 {
        return;
    }
    let mut r12 = 0usize;
    let mut r13 = 0usize;
    loop {
        let arr_20 = rd::<usize>(param_1 + 0x20);
        let arr_28 = rd::<usize>(param_1 + 0x28);
        let arr_30 = rd::<usize>(param_1 + 0x30);
        let bp = rd::<u16>(arr_28.wrapping_add(r12 * 2));
        let r15 = arr_20.wrapping_add(r13);
        let r14d: u32 = if arr_30 != 0 {
            rd::<u32>(arr_30.wrapping_add(r12 * 4))
        } else {
            0
        };
        let rbx = rd::<usize>(r15);

        if rd::<u16>(r15 + 0x14) != bp {
            let sub20 = rd::<usize>(rbx + 0x20);
            if rd::<u16>(sub20 + 0x90) != 0 {
                let mut edi = 0u32;
                let mut rsi = 0usize;
                loop {
                    let b8_arr = rd::<usize>(rbx + 0xb8);
                    let slot_addr = b8_arr.wrapping_add(rsi + 0x0c);
                    if rd::<u16>(slot_addr) != bp {
                        wr::<u16>(slot_addr, bp);
                        exe::call_va!(0x1_4215_d9d0, (rbx, edi as u64) -> ());
                    }
                    edi = edi.wrapping_add(1);
                    rsi = rsi.wrapping_add(0x14);
                    if edi == rd::<u16>(rd::<usize>(rbx + 0x20) + 0x90) as u32 {
                        break;
                    }
                }
            }
            wr::<u16>(r15 + 0x14, bp);
        }

        if rd::<u32>(r15 + 0x10) != r14d {
            let sub20 = rd::<usize>(rbx + 0x20);
            if rd::<u16>(sub20 + 0x90) != 0 {
                let mut edi = 0u32;
                let mut rsi = 0usize;
                loop {
                    let b8_arr = rd::<usize>(rbx + 0xb8);
                    let slot_addr = b8_arr.wrapping_add(rsi + 0x08);
                    if rd::<u32>(slot_addr) != r14d {
                        wr::<u32>(slot_addr, r14d);
                        exe::call_va!(0x1_4215_d9d0, (rbx, edi as u64) -> ());
                    }
                    edi = edi.wrapping_add(1);
                    rsi = rsi.wrapping_add(0x14);
                    if edi == rd::<u16>(rd::<usize>(rbx + 0x20) + 0x90) as u32 {
                        break;
                    }
                }
            }
            wr::<u32>(r15 + 0x10, r14d);
        }

        r12 += 1;
        r13 = r13.wrapping_add(0x20);
        if (r12 as u64) >= rd::<u64>(param_1 + 0x18) {
            break;
        }
    }
}

/// `unknown_141eb1f70`
/// Original: 0x141eb1f70 (1494 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141eb1f70(param_1: usize, param_2: usize) {
    let t_bits = rd::<u32>(param_2);
    let count = rd::<u64>(param_1 + 0x18) as usize;
    let xmm15 = crate::simd_ops::vbroadcastss(t_bits);
    if count == 0 {
        return;
    }

    let mut rbx = 0usize;
    let mut rdi = 0usize;
    let mut r14 = 0usize;
    let mut r15 = 0usize;
    while rbx < count {
        let p20 = rd::<u64>(param_1 + 0x20);
        let p28 = rd::<usize>(param_1 + 0x28);
        let p30 = rd::<usize>(param_1 + 0x30);
        let p38 = rd::<usize>(param_1 + 0x38);
        let p40 = rd::<usize>(param_1 + 0x40);
        let p50 = rd::<usize>(param_1 + 0x50);
        let p58 = rd::<usize>(param_1 + 0x58);
        let p60 = rd::<usize>(param_1 + 0x60);
        let p68 = rd::<usize>(param_1 + 0x68);
        let p70 = rd::<usize>(param_1 + 0x70);

        let cur_lo = rd::<u128>(p60 + rdi);
        let cur_hi = rd::<u128>(p60 + rdi + 0x10);
        let mut ymm13_lo = rd::<u128>(p38 + rdi);
        let ymm13_hi = rd::<u128>(p38 + rdi + 0x10);
        let mut ymm11_lo = rd::<u128>(p40 + rdi);
        let mut ymm11_hi = rd::<u128>(p40 + rdi + 0x10);

        let eq1 = crate::simd_ops::vcmpps(cur_lo, ymm13_lo, 0)
            & crate::simd_ops::vcmpps(cur_hi, ymm13_hi, 0);
        let mut sil: bool = if crate::simd_ops::vmovmskps(eq1) == 0xf {
            let eq2 = crate::simd_ops::vcmpps(cur_lo, ymm11_lo, 0)
                & crate::simd_ops::vcmpps(cur_hi, ymm11_hi, 0);
            crate::simd_ops::vmovmskps(eq2) != 0xf
        } else {
            true
        };

        if p28 != 0 && rd::<u64>(p28 + r15) != 0 {
            sil = true;
        }

        let flag_al: u8 = if (sil && rd::<u8>(p50 + rbx) == 0 && rd::<u32>(p30 + rbx * 4) == 0)
            || p20 != 0
        {
            1
        } else {
            0
        };

        if flag_al != rd::<u8>(p58 + rbx) {
            let mut cmd_type: u32 = 2;
            let mut flag_cell: u8 = flag_al;
            exe::call_va!(
                0x1_40ff_f3d0,
                (
                    p70.wrapping_add(0x10).wrapping_add(r14),
                    (&mut cmd_type as *mut u32) as usize,
                    (&mut flag_cell as *mut u8) as usize
                ) -> ()
            );
            wr::<u8>(p58 + rbx, flag_cell);
        }
        wr::<u8>(p50 + rbx, 0);

        if sil {
            let eq_endpoints = crate::simd_ops::vcmpps(ymm13_lo, ymm11_lo, 0)
                & crate::simd_ops::vcmpps(ymm11_hi, ymm13_hi, 0);
            if crate::simd_ops::vmovmskps(eq_endpoints) != 0xf {
                let mut xmm3 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8840) as usize);
                let mut xmm6 = rd::<u128>(exe::ptr::<u8>(0x1_4342_81b0) as usize);
                let xmm1 = crate::simd_ops::vdpps(ymm13_lo, ymm11_lo, 0xff);
                let xmm8 = xmm1 & rd::<u128>(exe::ptr::<u8>(0x1_4342_8760) as usize);
                let mut xmm9 = crate::simd_ops::vbroadcastss(t_bits);
                let mut xmm4 =
                    crate::simd_ops::vsubps(xmm9, rd::<u128>(exe::ptr::<u8>(0x1_4342_8120) as usize));
                let xmm2 =
                    crate::simd_ops::vsubps(xmm9, rd::<u128>(exe::ptr::<u8>(0x1_4342_8240) as usize));
                let xmm5 = xmm8 ^ xmm1;
                xmm3 = vfmadd231ps_128(xmm3, xmm5, rd::<u128>(exe::ptr::<u8>(0x1_4342_80c0) as usize));
                let xmm1_mul = crate::simd_ops::vmulps(xmm4, xmm9);
                let xmm7 = crate::simd_ops::vmulps(xmm2, xmm1_mul);
                let mut xmm1_poly = rd::<u128>(exe::ptr::<u8>(0x1_4342_84c0) as usize);
                xmm1_poly = vfnmadd231ps_128(
                    xmm1_poly,
                    xmm5,
                    rd::<u128>(exe::ptr::<u8>(0x1_4342_82c0) as usize),
                );
                let mut xmm2_poly = rd::<u128>(exe::ptr::<u8>(0x1_4342_88d0) as usize);
                xmm2_poly = vfmadd231ps_128(xmm2_poly, xmm5, xmm1_poly);
                let xmm1_c460 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8460) as usize);
                xmm6 = vfmadd231ps_128(xmm6, xmm5, xmm3);
                let mut xmm3_poly = rd::<u128>(exe::ptr::<u8>(0x1_4342_8280) as usize);
                xmm3_poly = vfmadd231ps_128(xmm3_poly, xmm5, xmm2_poly);
                xmm4 = crate::simd_ops::vmulps(xmm4, xmm4);
                xmm6 = vfmadd231ps_128(xmm6, xmm3_poly, xmm4);
                xmm9 = vfmadd231ps_128(xmm9, xmm6, xmm7);
                let xmm0 = xmm9 ^ xmm8;
                ymm13_lo = vfnmadd231ps_128(ymm13_lo, xmm9, ymm13_lo);
                ymm13_lo = vfmadd231ps_128(ymm13_lo, xmm0, ymm11_lo);
                let xmm0_dp = crate::simd_ops::vdpps(ymm13_lo, ymm13_lo, 0xff);
                let xmm4_rsq = vrsqrtps_128(xmm0_dp);
                let xmm0_mul = crate::simd_ops::vmulps(xmm4_rsq, xmm0_dp);
                let xmm2_mul = crate::simd_ops::vmulps(xmm0_mul, xmm4_rsq);
                let xmm0_half = crate::simd_ops::vmulps(
                    xmm4_rsq,
                    rd::<u128>(exe::ptr::<u8>(0x1_4342_8120) as usize),
                );
                let xmm3_sub = crate::simd_ops::vsubps(xmm1_c460, xmm2_mul);
                let xmm2_scale = crate::simd_ops::vmulps(xmm3_sub, xmm0_half);
                let mut xmm12 = ymm13_hi;
                xmm12 = vfnmadd231ps_128(xmm12, xmm12, xmm15);
                let xmm1_q = crate::simd_ops::vmulps(xmm2_scale, ymm13_lo);
                xmm12 = vfmadd231ps_128(xmm12, ymm11_hi, xmm15);
                ymm11_lo = xmm1_q;
                ymm11_hi = xmm12;
            }

            let xmm1_c3b0 = rd::<u128>(exe::ptr::<u8>(0x1_4342_83b0) as usize);
            let xmm8 = rd::<u128>(exe::ptr::<u8>(0x1_4342_8240) as usize);
            let sx_bits = rd::<u32>(p68 + rdi + 0x10);
            let sy_bits = rd::<u32>(p68 + rdi + 0x14);
            let sz_bits = rd::<u32>(p68 + rdi + 0x18);
            let mut xmm0 = crate::simd_ops::vblendps(0, sx_bits as u128, 0x1);
            xmm0 = crate::simd_ops::vinsertps(xmm0, sy_bits as u128, 0x10);
            let xmm3 = crate::simd_ops::vmulps(ymm11_lo, xmm1_c3b0);
            xmm0 = crate::simd_ops::vinsertps(xmm0, sz_bits as u128, 0x20);
            xmm0 = crate::simd_ops::vcmpps(xmm0, xmm8, 4);
            let mask_eax = crate::simd_ops::vmovmskps(xmm0) as u32;
            let xmm5 = crate::simd_ops::vmulps(ymm11_lo, xmm3);
            let xmm0 = crate::simd_ops::vshufps(ymm11_lo, ymm11_lo, 0xe0);
            let xmm2 = crate::simd_ops::vmulps(xmm0, xmm1_c3b0);
            let xmm1 = crate::simd_ops::vshufps(ymm11_lo, ymm11_lo, 0xd9);
            let xmm6 = crate::simd_ops::vmulps(xmm2, xmm1);
            let xmm1 = crate::simd_ops::vpermilps(xmm5, 0xc1);
            let xmm2 = crate::simd_ops::vsubps(xmm8, xmm1);
            let xmm0 = crate::simd_ops::vshufps(ymm11_lo, ymm11_lo, 0xff);
            let xmm4 = crate::simd_ops::vmulps(xmm0, xmm3);
            let xmm0 = crate::simd_ops::vpermilps(xmm5, 0xda);
            let xmm7 = crate::simd_ops::vsubps(xmm2, xmm0);
            let xmm0 = crate::simd_ops::vpermilps(xmm6, 0xd8);
            let xmm1 = crate::simd_ops::vpermilps(xmm4, 0xd2);
            let xmm3 = crate::simd_ops::vaddps(xmm1, xmm0);
            let xmm0 = crate::simd_ops::vpermilps(xmm6, 0xe1);
            let xmm2 = crate::simd_ops::vpermilps(xmm4, 0xc9);
            let xmm5 = crate::simd_ops::vsubps(xmm0, xmm2);
            let xmm1_zero = 0u128;

            let (out_70, out_80, out_90) = if (mask_eax & 7) != 0 {
                let xmm2 = crate::simd_ops::vshufps(xmm7, xmm3, 0xee);
                let xmm6 = crate::simd_ops::vshufps(xmm7, xmm3, 0x44);
                let xmm3 = crate::simd_ops::vshufps(xmm5, xmm1_zero, 0x44);
                let xmm1 = crate::simd_ops::vshufps(xmm5, xmm1_zero, 0xee);
                let xmm0 = ymm11_hi;
                let xmm7 = crate::simd_ops::vblendps(xmm8, xmm0, 0x7);
                let xmm0 = crate::simd_ops::vshufps(xmm2, xmm1, 0x88);
                let xmm2 = crate::simd_ops::vpermilps(xmm0, 0xc9);
                let xmm0 = crate::simd_ops::vshufps(xmm6, xmm3, 0xdd);
                let xmm1 = crate::simd_ops::vbroadcastss(sz_bits);
                let xmm5 = crate::simd_ops::vmulps(xmm2, xmm1);
                let xmm2 = crate::simd_ops::vpermilps(xmm0, 0xd2);
                let xmm1 = crate::simd_ops::vbroadcastss(sy_bits);
                let xmm4 = crate::simd_ops::vmulps(xmm2, xmm1);
                let xmm0 = crate::simd_ops::vbroadcastss(sx_bits);
                let xmm3 = crate::simd_ops::vshufps(xmm6, xmm3, 0x88);
                let xmm1 = crate::simd_ops::vmulps(xmm3, xmm0);
                let xmm0 = crate::simd_ops::vshufps(xmm1, xmm4, 0x4e);
                let xmm1_70 = crate::simd_ops::vshufps(xmm1, xmm0, 0x84);
                let xmm0 = crate::simd_ops::vshufps(xmm5, xmm7, 0x4e);
                let xmm1_90 = crate::simd_ops::vshufps(xmm0, xmm7, 0x98);
                let xmm2_80 = crate::simd_ops::vshufps(xmm4, xmm5, 0x49);
                (xmm1_70, xmm2_80, xmm1_90)
            } else {
                let xmm0 = crate::simd_ops::vshufps(xmm5, xmm1_zero, 0x44);
                let xmm2 = crate::simd_ops::vshufps(xmm7, xmm3, 0x44);
                let xmm6 = crate::simd_ops::vshufps(xmm2, xmm0, 0x88);
                let xmm0 = crate::simd_ops::vshufps(xmm2, xmm0, 0xdd);
                let xmm2 = crate::simd_ops::vpermilps(xmm0, 0xd2);
                let xmm1 = crate::simd_ops::vshufps(xmm5, xmm1_zero, 0xee);
                let xmm0 = ymm11_hi;
                let xmm4 = crate::simd_ops::vshufps(xmm7, xmm3, 0xee);
                let xmm3 = crate::simd_ops::vblendps(xmm8, xmm0, 0x7);
                let xmm1 = crate::simd_ops::vshufps(xmm4, xmm1, 0x88);
                let xmm5 = crate::simd_ops::vpermilps(xmm1, 0xc9);
                let xmm0 = crate::simd_ops::vshufps(xmm6, xmm2, 0x4e);
                let xmm1_70 = crate::simd_ops::vshufps(xmm6, xmm0, 0x84);
                let xmm0 = crate::simd_ops::vshufps(xmm5, xmm3, 0x4e);
                let xmm1_90 = crate::simd_ops::vshufps(xmm0, xmm3, 0x98);
                let xmm2_80 = crate::simd_ops::vshufps(xmm2, xmm5, 0x49);
                (xmm1_70, xmm2_80, xmm1_90)
            };

            let mat_buf: [u128; 3] = [out_70, out_80, out_90];
            wr::<u128>(p60 + rdi, ymm11_lo);
            wr::<u128>(p60 + rdi + 0x10, ymm11_hi);
            exe::call_va!(
                0x1_4103_8eb0,
                (
                    p70.wrapping_add(0x10).wrapping_add(r14),
                    mat_buf.as_ptr() as usize
                ) -> ()
            );
        }

        rbx += 1;
        r14 = r14.wrapping_add(0x18);
        rdi = rdi.wrapping_add(0x20);
        r15 = r15.wrapping_add(0x10);
    }
}

/// `unknown_141e22480`
/// Original: 0x141e22480 (522 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e22480(param_1: usize) {
    let r15 = rd::<usize>(param_1);
    let mut span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter = [0u64; 3];
    let _ = exe::call_va!(
        0x1_401e_7c10,
        (iter.as_mut_ptr() as usize, r15.wrapping_add(0x40), span.as_mut_ptr() as usize) -> usize
    );

    let rbx_base = iter[0] as usize;
    let mut cur_u16 = iter[1] as usize;
    let end_u16 = iter[2] as usize;
    let mut rbp_idx = 0usize;
    let mut r12_cnt = 0usize;
    let mut r11_chunk = 0usize;

    if cur_u16 != end_u16 {
        let arch_id = rd::<u16>(cur_u16) as usize;
        r11_chunk = rd::<usize>(r15.wrapping_add(arch_id * 8).wrapping_add(0x88));
        r12_cnt = if r11_chunk != 0 {
            rd::<u32>(r15.wrapping_add(arch_id * 4).wrapping_add(0x10088)) as usize
        } else {
            0
        };
    }

    let mut rax_ptr = 0usize;
    if cur_u16 != end_u16 {
        let arch_id = rd::<u16>(cur_u16) as usize;
        let col_id = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d074) as usize) as u32;
        let meta = r15.wrapping_add(arch_id * 0x20);
        let pos = exe::call_va!(
            0x1_4248_9ee0,
            (rd::<u64>(meta + 0x18098), rd::<u32>(meta + 0x180ac), col_id) -> i32
        );
        if (pos as i64) != -1 {
            let offs_ptr = rd::<usize>(r15.wrapping_add((arch_id + 0xc03) * 0x20).wrapping_add(0x40));
            let off = rd::<u32>(offs_ptr.wrapping_add(((pos as isize) * 4) as usize)) as usize;
            if off != 0 {
                rax_ptr = off.wrapping_add(r11_chunk);
            }
        }
    }

    if r12_cnt != 0 {
        while rbp_idx < r12_cnt {
            while rbp_idx < r12_cnt {
                wr::<u8>(rax_ptr.wrapping_add(rbp_idx), 0);
                rbp_idx += 1;
            }
            loop {
                cur_u16 = cur_u16.wrapping_add(2);
                if cur_u16 == end_u16 {
                    break;
                }
                let arch_id = rd::<u16>(cur_u16) as usize;
                if rd::<u64>(rbx_base.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(rbx_base.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
            }
            if cur_u16 != end_u16 {
                let arch_id = rd::<u16>(cur_u16) as usize;
                let chunk_p = rd::<usize>(r15.wrapping_add(arch_id * 8).wrapping_add(0x88));
                r12_cnt = if chunk_p != 0 {
                    rd::<u32>(r15.wrapping_add(arch_id * 4).wrapping_add(0x10088)) as usize
                } else {
                    0
                };
                rbp_idx = 0;
                let col_id = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d074) as usize) as u32;
                let meta = r15.wrapping_add(arch_id * 0x20);
                let pos = exe::call_va!(
                    0x1_4248_9ee0,
                    (rd::<u64>(meta + 0x18098), rd::<u32>(meta + 0x180ac), col_id) -> i32
                );
                if (pos as i64) == -1 {
                    rax_ptr = 0;
                } else {
                    let offs_ptr =
                        rd::<usize>(r15.wrapping_add((arch_id + 0xc03) * 0x20).wrapping_add(0x40));
                    let off =
                        rd::<u32>(offs_ptr.wrapping_add(((pos as isize) * 4) as usize)) as usize;
                    rax_ptr = if off != 0 { off.wrapping_add(chunk_p) } else { 0 };
                }
            }
        }
    }
}

/// `unknown_141eac4e0`
/// Original: 0x141eac4e0 (336 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141eac4e0(param_1: usize) {
    let r11 = rd::<usize>(param_1).wrapping_add(0x40);
    let mut span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter = [0u64; 3];
    let _ = exe::call_va!(
        0x1_401e_7c10,
        (iter.as_mut_ptr() as usize, r11, span.as_mut_ptr() as usize) -> usize
    );

    let mut outer = [0u64; 9];
    outer[0] = r11 as u64;
    outer[1] = iter[0];
    outer[2] = iter[1];
    outer[3] = iter[2];
    outer[5] = 0;
    outer[6] = 0;
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>(r11.wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>(r11.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }
    exe::call_va!(0x1_4103_84a0, (outer.as_mut_ptr() as usize) -> ());

    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let idx = outer[6] as usize;
            let src_p = (outer[7] as usize).wrapping_add(idx);
            let dst_p = (outer[8] as usize).wrapping_add(idx);
            wr::<u8>(dst_p, rd::<u8>(dst_p) | rd::<u8>(src_p));
            outer[6] = (idx + 1) as u64;
        }
        loop {
            outer[2] = outer[2].wrapping_add(2);
            if outer[2] == outer[3] {
                break;
            }
            let arch_id = rd::<u16>(outer[2] as usize) as usize;
            let base_e0 = outer[1] as usize;
            if rd::<u64>(base_e0.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                && rd::<u32>(base_e0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
            {
                break;
            }
        }
        if outer[2] != outer[3] {
            let arch_id = rd::<u16>(outer[2] as usize) as usize;
            let base_e8 = outer[0] as usize;
            let cp = rd::<u64>(base_e8.wrapping_add(arch_id * 8).wrapping_add(0x48));
            outer[4] = cp;
            outer[5] = if cp != 0 {
                rd::<u32>(base_e8.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
            } else {
                0
            };
            outer[6] = 0;
        }
        exe::call_va!(0x1_4103_84a0, (outer.as_mut_ptr() as usize) -> ());
    }
}

pub unsafe fn diff_unknown_141e98490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: usize, span: usize) -> usize {
        let len = rd::<usize>(span);
        let ptr = rd::<usize>(span + 8);
        wr::<usize>(out, r11);
        wr::<usize>(out + 8, ptr);
        wr::<usize>(out + 16, ptr.wrapping_add(len * 2));
        out
    }
    unsafe extern "win64" fn mock_step_84e0(outer: usize) {
        if rd::<usize>(outer + 0x10) != rd::<usize>(outer + 0x18) {
            let cp = rd::<usize>(outer + 0x20);
            wr::<usize>(outer + 0x38, cp);
            wr::<usize>(outer + 0x40, cp + 0x80);
            wr::<usize>(outer + 0x48, cp + 0xa0);
            wr::<usize>(outer + 0x50, cp + 0xc0);
        }
    }
    unsafe extern "win64" fn mock_65bf0(r13: usize) {
        wr::<u32>(r13 + 0x30, rd::<u32>(r13 + 0x30).wrapping_add(0x11));
    }
    unsafe extern "win64" fn mock_5e100(_r13: usize, vecs: usize, flag: u64) {
        wr::<u64>(vecs, rd::<u64>(vecs).wrapping_add(flag + 5));
    }
    unsafe extern "win64" fn mock_5e510(_r13: usize, vecs: usize) {
        wr::<u64>(vecs + 0x10, rd::<u64>(vecs + 0x10).wrapping_add(9));
    }
    unsafe extern "win64" fn mock_fa730(_p3: usize, out_idx: usize, key_ptr: usize) -> usize {
        let k = rd::<u64>(key_ptr);
        wr::<i64>(out_idx, if k == 0x777 { 0 } else { -1 });
        out_idx
    }
    unsafe extern "win64" fn mock_find_col(_p: u64, _n: u32, _col: u32) -> i32 {
        0
    }
    unsafe extern "win64" fn mock_free(_p: usize) {}
    unsafe extern "win64" fn mock_p4_10(
        this: usize,
        _r13: usize,
        a3: u32,
        f20: u64,
        a5: u32,
        _a6: usize,
    ) {
        wr::<u64>(
            this + 0x08,
            rd::<u64>(this + 0x08).wrapping_add((a3 as u64) + f20 + (a5 as u64)),
        );
    }
    unsafe extern "win64" fn mock_p4_18(this: usize, f20: u64, _obj: usize) {
        wr::<u64>(this + 0x10, rd::<u64>(this + 0x10).wrapping_add(f20 + 1));
    }
    unsafe extern "win64" fn mock_fn_10(this: usize, _p2_cell: usize, vecs: usize) {
        wr::<u64>(this + 0x08, rd::<u64>(vecs) ^ (rd::<u64>(vecs + 0x10) << 8));
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4100_84e0, mock_step_84e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4216_5bf0, mock_65bf0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4215_e100, mock_5e100 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4215_e510, mock_5e510 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401f_a730, mock_fa730 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find_col as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_40f6_7380, mock_p4_10 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_40f6_7390, mock_p4_18 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_40f6_73a0, mock_fn_10 as *const ())?;

    let g_4bc = exe::ptr::<u16>(0x1_43c9_14bc) as usize;
    let g_98c = exe::ptr::<u16>(0x1_43c9_398c) as usize;
    let s_4bc = rd::<u16>(g_4bc);
    let s_98c = rd::<u16>(g_98c);
    wr::<u16>(g_4bc, 0);
    wr::<u16>(g_98c, 0);

    let run = |f: F| -> (u8, u8, u8, u8, u32, u64, u64, u64) {
        let mut r13_0 = [0u8; 0x120];
        let r13_0p = r13_0.as_mut_ptr() as usize;
        wr::<u64>(r13_0p + 0x20, 0x33);
        wr::<u8>(r13_0p + 0x114, 4);

        let mut r13_1 = [0u8; 0x120];
        let r13_1p = r13_1.as_mut_ptr() as usize;
        wr::<u64>(r13_1p + 0x20, 0x44);
        wr::<u8>(r13_1p + 0x114, 1);

        let mut sub_items = [0u8; 0x40];
        let sip = sub_items.as_mut_ptr() as usize;
        wr::<u64>(sip, 0x777);
        wr::<u32>(sip + 0x08, 10);
        wr::<u32>(sip + 0x0c, 20);

        let mut chunk = [0u8; 0x200];
        let cp = chunk.as_mut_ptr() as usize;
        wr::<usize>(cp, r13_0p);
        wr::<u8>(cp + 0x18, 0);
        wr::<usize>(cp + 0x20, r13_1p);
        wr::<u8>(cp + 0x38, 1);
        wr::<u8>(cp + 0x80, 1);
        wr::<u8>(cp + 0x81, 0);
        wr::<u32>(cp + 0xa0, 0);
        wr::<u32>(cp + 0xa4, 0);
        wr::<usize>(cp + 0xc0, sip);
        wr::<u32>(cp + 0xc8, 1);

        let ent_records = [0u64];
        let col_offsets = [0u32];
        let mut world = vec![0u8; 0x18200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x30, ent_records.as_ptr() as usize);
        wr::<usize>(wp + 0x88, cp);
        wr::<u32>(wp + 0x10088, 2);
        wr::<usize>(wp + 0x180a0, col_offsets.as_ptr() as usize);

        let arch_ids = [0u16];
        let param_1 = [wp, 1usize, arch_ids.as_ptr() as usize];

        let mut fn_vtbl = [0u64; 4];
        fn_vtbl[2] = exe::ptr::<u8>(0x1_40f6_73a0) as u64;
        let mut fn_obj = [0u64; 4];
        fn_obj[0] = fn_vtbl.as_ptr() as u64;
        let mut slot_entry = [0u8; 0x40];
        wr::<usize>(slot_entry.as_mut_ptr() as usize + 0x38, fn_obj.as_mut_ptr() as usize);
        let tbl_580e0 = [slot_entry.as_mut_ptr() as usize];
        let mut p2 = vec![0u8; 0x58100];
        wr::<usize>(p2.as_mut_ptr() as usize + 0x580e0, tbl_580e0.as_ptr() as usize);

        let nodes = [0u64, 0u64];
        let mut p3 = [0u8; 0x80];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<u64>(p3p + 0x30, 0);
        wr::<u64>(p3p + 0x38, 1);
        wr::<u64>(p3p + 0x40, 1);
        wr::<usize>(p3p + 0x58, nodes.as_ptr() as usize);

        let mut p4_vtbl = [0u64; 6];
        p4_vtbl[2] = exe::ptr::<u8>(0x1_40f6_7380) as u64;
        p4_vtbl[3] = exe::ptr::<u8>(0x1_40f6_7390) as u64;
        let mut p4_inner = [0u64; 4];
        p4_inner[0] = p4_vtbl.as_ptr() as u64;
        let p4_mid = p4_inner.as_mut_ptr() as usize;
        let p4_ptr = (&p4_mid as *const usize) as usize;
        let p4_top = (&p4_ptr as *const usize) as usize;

        f(
            param_1.as_ptr() as usize,
            p2.as_mut_ptr() as usize,
            p3p,
            p4_top,
        );
        (
            rd::<u8>(cp + 0x18),
            rd::<u8>(cp + 0x38),
            rd::<u8>(r13_0p + 0x114),
            rd::<u8>(r13_1p + 0x114),
            rd::<u32>(r13_1p + 0x30),
            p4_inner[1],
            p4_inner[2],
            fn_obj[1],
        )
    };
    let o = run(f_o);
    let n = run(f_n);

    wr::<u16>(g_4bc, s_4bc);
    wr::<u16>(g_98c, s_98c);

    crate::diff_eq!(o, n, "141e98490 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e98960(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_689d0(rdi: usize) {
        wr::<u32>(rdi + 0x30, 0xaa);
    }
    unsafe extern "win64" fn mock_60980(rcx: u64, rdx: usize, r8: u64, r9: u64) {
        wr::<u32>(
            rdx + 0x20,
            ((rcx as u32) << 16) | (((r8 & 1) as u32) << 8) | ((r9 & 1) as u32),
        );
    }
    unsafe extern "win64" fn mock_5eb60(rdi: usize, idx: u64) {
        wr::<u32>(rdi + 0x34, (idx as u32) + 0x50);
    }
    unsafe extern "win64" fn mock_actor_1b8(_actor: usize, out: usize) -> u8 {
        wr::<u64>(out, 0x1234);
        1
    }
    unsafe extern "win64" fn mock_actor_a8(actor: usize, inp: usize, flag: u64) {
        wr::<u32>(actor + 0x24, (rd::<u64>(inp) as u32) + (flag as u32));
    }
    unsafe extern "win64" fn mock_actor_30(_actor: usize) -> usize {
        1
    }
    unsafe extern "win64" fn mock_p28_48(p28: usize) -> usize {
        rd::<usize>(p28 + 0x08)
    }
    unsafe extern "win64" fn mock_scene_1b8(scene: usize, _actor: usize) {
        wr::<u32>(scene + 0x08, 0x77);
    }
    unsafe extern "win64" fn mock_actor_178(_actor: usize, out: usize) -> usize {
        wr::<u8>(out, 0);
        out
    }
    unsafe extern "win64" fn mock_actor_248(actor: usize) {
        wr::<u32>(actor + 0x28, 0x99);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4216_89d0, mock_689d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4216_0980, mock_60980 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4215_eb60, mock_5eb60 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40f6_7380, mock_actor_1b8 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40f6_7390, mock_actor_a8 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40f6_73a0, mock_actor_30 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_40f6_73b0, mock_p28_48 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_40f6_73c0, mock_scene_1b8 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_40f6_73d0, mock_actor_178 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_40f6_73e0, mock_actor_248 as *const ())?;

    let run = |f: F| -> ([u8; 6], [u32; 7], u64) {
        let mut actor_vtbl = [0u64; 80];
        actor_vtbl[0x30 / 8] = exe::ptr::<u8>(0x1_40f6_73a0) as u64;
        actor_vtbl[0xa8 / 8] = exe::ptr::<u8>(0x1_40f6_7390) as u64;
        actor_vtbl[0x178 / 8] = exe::ptr::<u8>(0x1_40f6_73d0) as u64;
        actor_vtbl[0x1b8 / 8] = exe::ptr::<u8>(0x1_40f6_7380) as u64;
        actor_vtbl[0x248 / 8] = exe::ptr::<u8>(0x1_40f6_73e0) as u64;

        let mut scene_vtbl = [0u64; 64];
        scene_vtbl[0x1b8 / 8] = exe::ptr::<u8>(0x1_40f6_73c0) as u64;
        let mut scene_obj = [0u64; 4];
        scene_obj[0] = scene_vtbl.as_ptr() as u64;

        let mut p28_vtbl = [0u64; 16];
        p28_vtbl[0x48 / 8] = exe::ptr::<u8>(0x1_40f6_73b0) as u64;
        let p28_obj = [p28_vtbl.as_ptr() as u64, scene_obj.as_mut_ptr() as u64];

        let mut actor1 = [0u8; 0x40];
        let a1p = actor1.as_mut_ptr() as usize;
        wr::<usize>(a1p, actor_vtbl.as_ptr() as usize);
        wr::<u16>(a1p + 0x08, 6);

        let mut actor2 = [0u8; 0x40];
        let a2p = actor2.as_mut_ptr() as usize;
        wr::<usize>(a2p, actor_vtbl.as_ptr() as usize);
        wr::<u16>(a2p + 0x08, 6);

        let mut words = [0xffff_ffff_ffff_ffffu64, 0xffff_ffff_ffff_ffffu64];
        let mut sub20_1 = [0u8; 0xa0];
        let s20_1p = sub20_1.as_mut_ptr() as usize;
        wr::<u64>(s20_1p + 0x60, 2);
        wr::<usize>(s20_1p + 0x68, words.as_mut_ptr() as usize);
        wr::<u64>(s20_1p + 0x70, 65);
        wr::<u16>(s20_1p + 0x90, 1);

        let mut sub20_2 = [0u8; 0xa0];
        let s20_2p = sub20_2.as_mut_ptr() as usize;
        wr::<u16>(s20_2p + 0x90, 1);

        let mut b8_1 = [0u8; 0x20];
        wr::<u8>(b8_1.as_mut_ptr() as usize + 0x10, 4);
        let c0_1 = [a1p];
        let c0_2 = [a2p];

        let mut rdi_0 = [0u8; 0x120];
        let mut rdi_1 = [0u8; 0x120];
        let r1p = rdi_1.as_mut_ptr() as usize;
        wr::<usize>(r1p + 0x20, s20_1p);
        wr::<usize>(r1p + 0xb8, b8_1.as_mut_ptr() as usize);
        wr::<usize>(r1p + 0xc0, c0_1.as_ptr() as usize);
        wr::<u8>(r1p + 0x114, 8);

        let mut rdi_2 = [0u8; 0x120];
        let r2p = rdi_2.as_mut_ptr() as usize;
        wr::<usize>(r2p + 0x20, s20_2p);
        wr::<usize>(r2p + 0x28, p28_obj.as_ptr() as usize);
        wr::<usize>(r2p + 0xc0, c0_2.as_ptr() as usize);

        let mut arr_20 = [0u8; 0x60];
        let a20p = arr_20.as_mut_ptr() as usize;
        wr::<usize>(a20p, rdi_0.as_mut_ptr() as usize);
        wr::<u8>(a20p + 0x16, 0);
        wr::<usize>(a20p + 0x20, r1p);
        wr::<u8>(a20p + 0x20 + 0x16, 1);
        wr::<u8>(a20p + 0x20 + 0x17, 0);
        wr::<usize>(a20p + 0x40, r2p);
        wr::<u8>(a20p + 0x40 + 0x16, 1);
        wr::<u8>(a20p + 0x40 + 0x17, 1);

        let arr_28 = [1u32, 0u32, 0u32];
        let arr_30 = [2u8, 0u8, 0u8];

        let mut p1 = [0u8; 0x40];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 3);
        wr::<usize>(p1p + 0x20, a20p);
        wr::<usize>(p1p + 0x28, arr_28.as_ptr() as usize);
        wr::<usize>(p1p + 0x30, arr_30.as_ptr() as usize);

        f(p1p);
        (
            [
                rd::<u8>(a20p + 0x16),
                rd::<u8>(a20p + 0x17),
                rd::<u8>(a20p + 0x20 + 0x16),
                rd::<u8>(a20p + 0x20 + 0x17),
                rd::<u8>(a20p + 0x40 + 0x16),
                rd::<u8>(r1p + 0x114),
            ],
            [
                rd::<u32>(rdi_0.as_ptr() as usize + 0x30),
                rd::<u32>(r1p + 0x34),
                rd::<u32>(a1p + 0x20),
                rd::<u32>(a1p + 0x24),
                rd::<u32>(scene_obj.as_ptr() as usize + 0x08),
                rd::<u32>(a2p + 0x28),
                words[0] as u32,
            ],
            words[1],
        )
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e98960 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e98c80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_5d9d0(rbx: usize, idx: u64) {
        wr::<u32>(
            rbx + 0x30,
            rd::<u32>(rbx + 0x30).wrapping_add((idx as u32) + 1),
        );
    }
    let _m1 = crate::difftest::Mock::new(0x1_4215_d9d0, mock_5d9d0 as *const ())?;

    let run = |f: F| -> (u16, u32, u16, u32, u32) {
        let mut sub20 = [0u8; 0xa0];
        wr::<u16>(sub20.as_mut_ptr() as usize + 0x90, 2);

        let mut b8_arr = [0u8; 0x30];
        let b8p = b8_arr.as_mut_ptr() as usize;

        let mut rbx = [0u8; 0xd0];
        let rbxp = rbx.as_mut_ptr() as usize;
        wr::<usize>(rbxp + 0x20, sub20.as_mut_ptr() as usize);
        wr::<usize>(rbxp + 0xb8, b8p);

        let mut arr_20 = [0u8; 0x20];
        let a20p = arr_20.as_mut_ptr() as usize;
        wr::<usize>(a20p, rbxp);
        wr::<u32>(a20p + 0x10, 1);
        wr::<u16>(a20p + 0x14, 2);

        let arr_28 = [7u16];
        let arr_30 = [42u32];

        let mut p1 = [0u8; 0x40];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 1);
        wr::<usize>(p1p + 0x20, a20p);
        wr::<usize>(p1p + 0x28, arr_28.as_ptr() as usize);
        wr::<usize>(p1p + 0x30, arr_30.as_ptr() as usize);

        f(p1p);
        (
            rd::<u16>(a20p + 0x14),
            rd::<u32>(a20p + 0x10),
            rd::<u16>(b8p + 0x14 + 0x0c),
            rd::<u32>(b8p + 0x14 + 0x08),
            rd::<u32>(rbxp + 0x30),
        )
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141e98c80 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141eb1f70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_f3d0(rcx: usize, rdx: usize, r8: usize) {
        wr::<u32>(rcx, (rd::<u32>(rdx) << 8) | (rd::<u8>(r8) as u32));
    }
    unsafe extern "win64" fn mock_8eb0(rcx: usize, rdx: usize) {
        let m0 = rd::<u128>(rdx);
        let m1 = rd::<u128>(rdx + 0x10);
        let m2 = rd::<u128>(rdx + 0x20);
        let fold = m0 ^ m1.rotate_left(13) ^ m2.rotate_left(29);
        wr::<u128>(rcx + 0x04, fold);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40ff_f3d0, mock_f3d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4103_8eb0, mock_8eb0 as *const ())?;

    let run = |f: F| -> ([u32; 16], [u8; 4], [u8; 0x30]) {
        let p38: [f32; 16] = [
            0.0, 0.0, 0.0, 1.0, 1.0, 2.0, 3.0, 0.0, // item 0: start rot + pos
            0.0, 0.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0, // item 1: identical rot + pos
        ];
        let p40: [f32; 16] = [
            0.0, 0.70710677, 0.0, 0.70710677, 5.0, 6.0, 7.0, 0.0, // item 0: end rot + pos
            0.0, 0.0, 0.0, 1.0, 4.0, 5.0, 6.0, 0.0,               // item 1: identical rot + pos
        ];
        let mut p60 = [0.0f32; 16];
        let p68: [f32; 16] = [
            0.0, 0.0, 0.0, 0.0, 2.0, 3.0, 4.0, 0.0, // item 0: non-unit scale (2, 3, 4)
            0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 0.0, // item 1: unit scale (1, 1, 1)
        ];
        let p30 = [0u32, 0u32];
        let mut p50 = [0u8, 0u8];
        let mut p58 = [0u8, 0u8];
        let mut p70 = [0u8; 0x40];

        let mut param_1 = [0u8; 0x80];
        let p1p = param_1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x18, 2);
        wr::<u64>(p1p + 0x20, 0);
        wr::<usize>(p1p + 0x28, 0);
        wr::<usize>(p1p + 0x30, p30.as_ptr() as usize);
        wr::<usize>(p1p + 0x38, p38.as_ptr() as usize);
        wr::<usize>(p1p + 0x40, p40.as_ptr() as usize);
        wr::<usize>(p1p + 0x50, p50.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x58, p58.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x60, p60.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x68, p68.as_ptr() as usize);
        wr::<usize>(p1p + 0x70, p70.as_mut_ptr() as usize);

        let dt: f32 = 0.35;
        f(p1p, (&dt as *const f32) as usize);

        let mut out_70 = [0u8; 0x30];
        out_70.copy_from_slice(&p70[0x10..0x40]);
        (p60.map(f32::to_bits), [p50[0], p50[1], p58[0], p58[1]], out_70)
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141eb1f70 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141e22480(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: usize, span: usize) -> usize {
        let len = rd::<usize>(span);
        let ptr = rd::<usize>(span + 8);
        wr::<usize>(out, r11);
        wr::<usize>(out + 8, ptr);
        wr::<usize>(out + 16, ptr.wrapping_add(len * 2));
        out
    }
    unsafe extern "win64" fn mock_find_col(_p: u64, _n: u32, _col: u32) -> i32 {
        0
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find_col as *const ())?;

    let g_074 = exe::ptr::<u16>(0x1_43c8_d074) as usize;
    let s_074 = rd::<u16>(g_074);
    wr::<u16>(g_074, 0);

    let run = |f: F| -> ([u8; 8], [u8; 8]) {
        let mut chunk0 = [0xffu8; 8];
        let mut chunk1 = [0xeeu8; 8];
        let col_offs = [2u32];

        let mut world = vec![0u8; 0x18200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88, chunk0.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x90, chunk1.as_mut_ptr() as usize);
        wr::<u32>(wp + 0x10088, 3);
        wr::<u32>(wp + 0x1008c, 4);
        wr::<usize>(wp + 0x180a0, col_offs.as_ptr() as usize);
        wr::<usize>(wp + 0x180c0, col_offs.as_ptr() as usize);

        let arch_ids = [0u16, 1u16];
        let param_1 = [wp, 2usize, arch_ids.as_ptr() as usize];
        f(param_1.as_ptr() as usize);
        (chunk0, chunk1)
    };
    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_074, s_074);
    crate::diff_eq!(o, n, "141e22480 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141eac4e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: usize, span: usize) -> usize {
        let len = rd::<usize>(span);
        let ptr = rd::<usize>(span + 8);
        wr::<usize>(out, r11);
        wr::<usize>(out + 8, ptr);
        wr::<usize>(out + 16, ptr.wrapping_add(len * 2));
        out
    }
    unsafe extern "win64" fn mock_step_84a0(outer: usize) {
        if rd::<usize>(outer + 0x10) != rd::<usize>(outer + 0x18) {
            let cp = rd::<usize>(outer + 0x20);
            wr::<usize>(outer + 0x38, cp);
            wr::<usize>(outer + 0x40, cp + 4);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4103_84a0, mock_step_84a0 as *const ())?;

    let run = |f: F| -> ([u8; 8], [u8; 8]) {
        let mut chunk0 = [0x01u8, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];
        let mut chunk1 = [0x0fu8, 0xf0, 0x33, 0xcc, 0x50, 0x05, 0xc0, 0x03];

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88, chunk0.as_mut_ptr() as usize);
        wr::<usize>(wp + 0x90, chunk1.as_mut_ptr() as usize);
        wr::<u32>(wp + 0x10088, 4);
        wr::<u32>(wp + 0x1008c, 4);

        let arch_ids = [0u16, 1u16];
        let param_1 = [wp, 2usize, arch_ids.as_ptr() as usize];
        f(param_1.as_ptr() as usize);
        (chunk0, chunk1)
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141eac4e0 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_41e2_2480, "unknown_141e22480", rend::cauldron_hot_b::unknown_141e22480 as *const (), rend::cauldron_hot_b::diff_unknown_141e22480),
// Replacement::verified_throws(0x1_41e9_8490, "unknown_141e98490", rend::cauldron_hot_b::unknown_141e98490 as *const (), Throws::Discard, rend::cauldron_hot_b::diff_unknown_141e98490),
// Replacement::verified(0x1_41e9_8960, "unknown_141e98960", rend::cauldron_hot_b::unknown_141e98960 as *const (), rend::cauldron_hot_b::diff_unknown_141e98960),
// Replacement::verified_throws(0x1_41e9_8c80, "unknown_141e98c80", rend::cauldron_hot_b::unknown_141e98c80 as *const (), Throws::Discard, rend::cauldron_hot_b::diff_unknown_141e98c80),
// Replacement::verified(0x1_41ea_c4e0, "unknown_141eac4e0", rend::cauldron_hot_b::unknown_141eac4e0 as *const (), rend::cauldron_hot_b::diff_unknown_141eac4e0),
// Replacement::verified(0x1_41eb_1f70, "unknown_141eb1f70", rend::cauldron_hot_b::unknown_141eb1f70 as *const (), rend::cauldron_hot_b::diff_unknown_141eb1f70),
