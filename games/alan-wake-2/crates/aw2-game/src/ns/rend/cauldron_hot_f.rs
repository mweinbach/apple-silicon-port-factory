//! Decompiled hot functions shard F (`0x1404b7580..0x14123e2b0`).

use crate::exe::{self, rd, wr};

/// `unknown_1404b7580`
/// Original: 0x1404b7580 (329 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1404b7580(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if count != 0 {
        let r10 = rd::<usize>(world + 0x08);
        let v0 = rd::<usize>(world);
        let r8_lut = rd::<usize>(v0 + 0x20);

        let id1 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_4b84) as usize) as usize;
        let idx1 = rd::<u16>(r8_lut.wrapping_add(id1 * 2)) as usize;
        let len1 = rd::<u32>(r10.wrapping_add(idx1 * 16).wrapping_add(0x08)) as u64;
        let ptr1 = rd::<u64>(r10.wrapping_add(idx1 * 16));

        let id2 = rd::<u16>(exe::ptr::<u16>(0x1_43ca_72b8) as usize) as usize;
        let idx2 = rd::<u16>(r8_lut.wrapping_add(id2 * 2)) as usize;
        let len2 = rd::<u32>(r10.wrapping_add(idx2 * 16).wrapping_add(0x08)) as u64;
        let ptr2 = rd::<u64>(r10.wrapping_add(idx2 * 16));

        let mut ctx: [u64; 7] = [
            world as u64,
            len2,
            ptr2,
            world as u64,
            len1,
            ptr1,
            world as u64,
        ];
        let mut buf = [0u64; 19];
        let _ = exe::call_va!(
            0x1_404b_7d30,
            (
                buf.as_mut_ptr() as usize,
                param_2 as u64,
                block,
                count as u64,
                world
            ) -> usize
        );
        let mut buf_copy = buf;
        exe::call_va!(
            0x1_404b_76d0,
            (
                0usize,
                buf_copy.as_mut_ptr() as usize,
                ctx.as_mut_ptr() as usize
            )
        );
    }
}

/// `anonymous_namespace_140f4eb40`
/// Original: 0x140f4eb40 (382 bytes, Throws::Discard)
pub unsafe extern "win64" fn anonymous_namespace_140f4eb40(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if count != 0 {
        let r8_arr = rd::<usize>(world + 0x08);
        let v0 = rd::<usize>(world);
        let rdx_lut = rd::<usize>(v0 + 0x20);

        let id1 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0804) as usize) as usize;
        let idx1 = rd::<u16>(rdx_lut.wrapping_add(id1 * 2)) as usize;
        let len1 = rd::<u32>(r8_arr.wrapping_add(idx1 * 16).wrapping_add(0x08)) as u64;
        let ptr1 = rd::<u64>(r8_arr.wrapping_add(idx1 * 16));

        let id2 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ceb4) as usize) as usize;
        let idx2 = rd::<u16>(rdx_lut.wrapping_add(id2 * 2)) as usize;
        let len2 = rd::<u32>(r8_arr.wrapping_add(idx2 * 16).wrapping_add(0x08)) as u64;
        let ptr2 = rd::<u64>(r8_arr.wrapping_add(idx2 * 16));

        let want_id = rd::<u16>(exe::ptr::<u16>(0x1_43c9_0338) as usize) as u64;
        let arch_off = arch.wrapping_shl(5);
        let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac)) as u64;
        let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
        let found = exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_id) -> i32);

        let mut comp_off = if (found as isize) != -1 {
            let off_tbl = rd::<usize>(
                world
                    .wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5))
                    .wrapping_add(0x40),
            );
            rd::<u32>(off_tbl.wrapping_add((found as isize as usize) * 4)) as usize
        } else {
            0
        };
        if comp_off != 0 {
            comp_off = block.wrapping_add(comp_off);
        }

        let total = count as usize;
        for idx in 0..total {
            let mut arg1: [u64; 3] = [comp_off as u64, block as u64, idx as u64];
            let mut arg2: [u64; 3] = [world as u64, len1, ptr1];
            let mut arg3: [u64; 3] = [world as u64, len2, ptr2];
            exe::call_va!(
                0x1_41df_b350,
                (
                    arg1.as_mut_ptr() as usize,
                    arg2.as_mut_ptr() as usize,
                    arg3.as_mut_ptr() as usize
                )
            );
        }
    }
}

/// `unknown_140fbac80`
/// Original: 0x140fbac80 (299 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_140fbac80(param_1: usize) {
    let world = rd::<usize>(param_1);
    let v0 = rd::<usize>(world);
    let r9_lut = rd::<usize>(v0 + 0x20);
    let r8_arr = rd::<usize>(world + 0x08);

    let id_c4c = rd::<u16>(exe::ptr::<u16>(0x1_43cc_bc4c) as usize) as usize;
    let idx_c4c = rd::<u16>(r9_lut.wrapping_add(id_c4c * 2)) as usize;
    let len_c4c = rd::<u32>(r8_arr.wrapping_add(idx_c4c * 16).wrapping_add(0x08)) as u64;
    let ptr_c4c = rd::<u64>(r8_arr.wrapping_add(idx_c4c * 16));

    let id_c48 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_bc48) as usize) as usize;
    let idx_c48 = rd::<u16>(r9_lut.wrapping_add(id_c48 * 2)) as usize;
    let len_c48 = rd::<u32>(r8_arr.wrapping_add(idx_c48 * 16).wrapping_add(0x08)) as u64;
    let ptr_c48 = rd::<u64>(r8_arr.wrapping_add(idx_c48 * 16));

    let id_c44 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_bc44) as usize) as usize;
    let idx_c44 = rd::<u16>(r9_lut.wrapping_add(id_c44 * 2)) as usize;
    let len_c44 = rd::<u32>(r8_arr.wrapping_add(idx_c44 * 16).wrapping_add(0x08)) as u64;
    let ptr_c44 = rd::<u64>(r8_arr.wrapping_add(idx_c44 * 16));

    let id_2d0 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_a2d0) as usize) as usize;
    let tbl_580e0 = rd::<usize>(world + 0x580e0);
    let rdi_val = rd::<u64>(tbl_580e0.wrapping_add(id_2d0 * 8));

    let worker_idx = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let rbx_slot = world
        .wrapping_add(0x580f0)
        .wrapping_add(worker_idx.wrapping_mul(0x188));
    let _ = exe::call_va!(0x1_4240_7c80, () -> u32);

    let mut arg1: [u64; 3] = [world as u64, len_c4c, ptr_c4c];
    let mut arg2: [u64; 3] = [world as u64, len_c48, ptr_c48];
    let mut arg3: [u64; 3] = [world as u64, len_c44, ptr_c44];
    exe::call_va!(
        0x1_41e5_a750,
        (
            arg1.as_mut_ptr() as usize,
            arg2.as_mut_ptr() as usize,
            arg3.as_mut_ptr() as usize,
            rdi_val,
            rbx_slot
        )
    );
}

/// `unknown_141046dc0`
/// Original: 0x141046dc0 (191 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141046dc0(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if count != 0 {
        let id_0d8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_00d8) as usize) as usize;
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let rbx_val = rd::<u64>(tbl_580e0.wrapping_add(id_0d8 * 8));

        let mut buf = [0u64; 11];
        let _ = exe::call_va!(
            0x1_4104_8660,
            (
                buf.as_mut_ptr() as usize,
                param_2 as u64,
                block,
                count as u64,
                world
            ) -> usize
        );
        let mut buf_copy = buf;
        exe::call_va!(0x1_41eb_1010, (buf_copy.as_mut_ptr() as usize, rbx_val));
    }
}

/// `r_14104aa80`
/// Original: 0x14104aa80 (574 bytes, Throws::No)
pub unsafe extern "win64" fn r_14104aa80(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let world = rd::<usize>(param_1);
        let id_f2c = rd::<u16>(exe::ptr::<u16>(0x1_43cc_ef2c) as usize) as usize;
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let world_plus_40 = world.wrapping_add(0x40);
        let id_0d8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_00d8) as usize) as usize;
        let val_f2c = rd::<u64>(tbl_580e0.wrapping_add(id_f2c * 8));
        let val_0d8 = rd::<u64>(tbl_580e0.wrapping_add(id_0d8 * 8));

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world_plus_40.wrapping_add(arch * 8).wrapping_add(0x48));
            let ent_count = if block != 0 {
                rd::<u32>(world_plus_40.wrapping_add(arch * 4).wrapping_add(0x10048))
            } else {
                0
            };
            if ent_count != 0 {
                let mut raw_offs = [0u64; 11];
                let _ = exe::call_va!(
                    0x1_4104_8a00,
                    (
                        raw_offs.as_mut_ptr() as usize,
                        ent_count as u64,
                        world_plus_40,
                        arch as u64
                    ) -> usize
                );
                let block_u64 = block as u64;
                for off in &mut raw_offs {
                    if *off != 0 {
                        *off = (*off).wrapping_add(block_u64);
                    }
                }
                let mut pkt = [0u64; 15];
                pkt[0] = arch as u64;
                pkt[1] = block_u64;
                pkt[2] = world as u64;
                pkt[3] = ent_count as u64;
                pkt[4..15].copy_from_slice(&raw_offs);
                exe::call_va!(
                    0x1_41eb_1f70,
                    (pkt.as_mut_ptr() as usize, val_0d8, val_f2c)
                );
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_141084400`
/// Original: 0x141084400 (580 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141084400(_param_1: usize, param_2: usize) -> u64 {
    const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    const GOLDEN: u64 = 0x9e37_79b9;

    #[inline(always)]
    fn canon_f32_bits(bits: u32) -> u32 {
        if f32::from_bits(bits) == 0.0f32 {
            0
        } else {
            bits
        }
    }

    #[inline(always)]
    fn fnv1a_u32(v: u32) -> u64 {
        let b = v.to_le_bytes();
        let mut h = FNV_OFFSET;
        let mut i = 0;
        while i < 4 {
            h ^= b[i] as u64;
            h = h.wrapping_mul(FNV_PRIME);
            i += 1;
        }
        h
    }

    #[inline(always)]
    fn fnv1a_u64(v: u64) -> u64 {
        let b = v.to_le_bytes();
        let mut h = FNV_OFFSET;
        let mut i = 0;
        while i < 8 {
            h ^= b[i] as u64;
            h = h.wrapping_mul(FNV_PRIME);
            i += 1;
        }
        h
    }

    let x0 = canon_f32_bits(rd::<u32>(param_2));
    let x1 = canon_f32_bits(rd::<u32>(param_2 + 4));
    let x2 = canon_f32_bits(rd::<u32>(param_2 + 8));

    let h0 = fnv1a_u32(x0);
    let h1 = fnv1a_u64(fnv1a_u32(x1));
    let seed1 = h0 ^ (h1
        .wrapping_add(GOLDEN)
        .wrapping_add(h0 << 6)
        .wrapping_add(h0 >> 2));

    let h2 = fnv1a_u64(fnv1a_u32(x2));
    seed1 ^ (h2
        .wrapping_add(GOLDEN)
        .wrapping_add(seed1 << 6)
        .wrapping_add(seed1 >> 2))
}

/// `physics_14123e2b0`
/// Original: 0x14123e2b0 (469 bytes, Throws::Discard)
pub unsafe extern "win64" fn physics_14123e2b0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
) {
    let count = param_4.wrapping_sub(param_3);
    if count == 0 {
        return;
    }
    let old_data = rd::<usize>(param_1);
    let is_push_back_one = count == 1
        && param_2 == old_data.wrapping_add(rd::<u32>(param_1 + 0x08) as usize);

    let old_len = rd::<u32>(param_1 + 0x08) as usize;
    let old_cap = rd::<u32>(param_1 + 0x0c) as usize;
    let avail = old_cap.wrapping_sub(old_len);
    let new_len = old_len.wrapping_add(count);

    if count > avail {
        let insert_off = param_2.wrapping_sub(old_data);
        let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
        if new_cap < new_len {
            new_cap = new_len;
        }
        let new_data = exe::call_va!(0x1_4241_c0a0, (new_cap as u64, 1u64) -> usize);
        if param_3 != param_4 {
            let dst_base = new_data.wrapping_sub(param_3).wrapping_add(insert_off);
            let mut src = param_3;
            while src != param_4 {
                wr::<u8>(dst_base.wrapping_add(src), rd::<u8>(src));
                src = src.wrapping_add(1);
            }
        }
        let cur_data = rd::<usize>(param_1);
        if is_push_back_one {
            let cur_len = rd::<u32>(param_1 + 0x08) as u64;
            let _ = exe::call_va!(0x1_4183_acc8, (new_data, cur_data, cur_len) -> usize);
        } else {
            let prefix_len = param_2.wrapping_sub(cur_data) as u64;
            let _ = exe::call_va!(0x1_4183_acc8, (new_data, cur_data, prefix_len) -> usize);
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let cur_data2 = rd::<usize>(param_1);
            let suffix_len = cur_len.wrapping_sub(param_2).wrapping_add(cur_data2) as u64;
            let suffix_dst = insert_off.wrapping_add(new_data).wrapping_add(count);
            let _ = exe::call_va!(0x1_4183_acc8, (suffix_dst, param_2, suffix_len) -> usize);
        }
        let to_free = rd::<usize>(param_1);
        if to_free != 0 {
            exe::call_va!(0x1_4241_c190, (to_free));
        }
        wr::<u32>(param_1 + 0x08, new_len as u32);
        wr::<u32>(param_1 + 0x0c, new_cap as u32);
        wr::<usize>(param_1, new_data);
    } else {
        let old_end = old_data.wrapping_add(old_len);
        let tail_len = old_end.wrapping_sub(param_2);
        if count < tail_len {
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (old_end, old_end.wrapping_sub(count), count as u64) -> usize
            );
            let rem_tail = tail_len.wrapping_sub(count);
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (old_end.wrapping_sub(rem_tail), param_2, rem_tail as u64) -> usize
            );
            if param_3 != param_4 {
                let delta = param_2.wrapping_sub(param_3);
                let mut src = param_3;
                while src != param_4 {
                    wr::<u8>(delta.wrapping_add(src), rd::<u8>(src));
                    src = src.wrapping_add(1);
                }
            }
        } else {
            let _ = exe::call_va!(
                0x1_4183_acc8,
                (param_2.wrapping_add(count), param_2, tail_len as u64) -> usize
            );
            if param_3 != param_4 {
                let delta = param_2.wrapping_sub(param_3);
                let mut src = param_3;
                while src != param_4 {
                    wr::<u8>(delta.wrapping_add(src), rd::<u8>(src));
                    src = src.wrapping_add(1);
                }
            }
        }
        wr::<u32>(param_1 + 0x08, new_len as u32);
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1404b7580(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_b7d30(
        out: usize,
        arch: u16,
        block: usize,
        cnt: u64,
        world: usize,
    ) -> usize {
        wr::<u64>(out, arch as u64);
        wr::<u64>(out + 0x08, block as u64);
        wr::<u64>(out + 0x10, world as u64);
        wr::<u64>(out + 0x18, cnt);
        for i in 4..19usize {
            wr::<u64>(out + i * 8, 0x1000 + i as u64);
        }
        out
    }
    unsafe extern "win64" fn mock_b76d0(_rcx: usize, rdx_buf: usize, r8_ctx: usize) {
        let world = rd::<usize>(r8_ctx);
        let rel_ctx: [u64; 7] = [
            0,
            rd::<u64>(r8_ctx + 0x08),
            rd::<u64>(r8_ctx + 0x10),
            (rd::<usize>(r8_ctx + 0x18).wrapping_sub(world)) as u64,
            rd::<u64>(r8_ctx + 0x20),
            rd::<u64>(r8_ctx + 0x28),
            (rd::<usize>(r8_ctx + 0x30).wrapping_sub(world)) as u64,
        ];
        let mut rel_buf = rd::<[u64; 19]>(rdx_buf);
        rel_buf[2] = (rel_buf[2] as usize).wrapping_sub(world) as u64;
        wr::<[u64; 19]>(world + 0x200, rel_buf);
        wr::<[u64; 7]>(world + 0x300, rel_ctx);
    }

    let _m1 = crate::difftest::Mock::new(0x1_404b_7d30, mock_b7d30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_404b_76d0, mock_b76d0 as *const ())?;

    let g_b84 = exe::ptr::<u16>(0x1_43c9_4b84) as usize;
    let g_2b8 = exe::ptr::<u16>(0x1_43ca_72b8) as usize;
    let s_b84 = rd::<u16>(g_b84);
    let s_2b8 = rd::<u16>(g_2b8);
    wr::<u16>(g_b84, 1);
    wr::<u16>(g_2b8, 2);

    let run = |f: F| -> ([u64; 19], [u64; 7]) {
        let lut: [u16; 4] = [0, 2, 1, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut r10_arr = [0u8; 0x40];
        let r10p = r10_arr.as_mut_ptr() as usize;
        wr::<u64>(r10p + 0x10, 0xaaaa_1111);
        wr::<u32>(r10p + 0x18, 7);
        wr::<u64>(r10p + 0x20, 0xbbbb_2222);
        wr::<u32>(r10p + 0x28, 13);

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, r10p);
        wr::<usize>(wp + 0x88 + 2 * 8, 0x7777_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 5);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 2);
        (rd::<[u64; 19]>(wp + 0x200), rd::<[u64; 7]>(wp + 0x300))
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_b84, s_b84);
    wr::<u16>(g_2b8, s_2b8);

    crate::diff_eq!(o, n, "1404b7580 out");
    Ok(1)
}

pub unsafe fn diff_anonymous_namespace_140f4eb40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0xabc0 && n == 3 && want == 7 {
            1
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_b350(a1_ptr: usize, a2_ptr: usize, a3_ptr: usize) {
        let a1 = rd::<[u64; 3]>(a1_ptr);
        let mut a2 = rd::<[u64; 3]>(a2_ptr);
        let mut a3 = rd::<[u64; 3]>(a3_ptr);
        let world = a2[0] as usize;
        a2[0] = 0;
        a3[0] = (a3[0] as usize).wrapping_sub(world) as u64;
        let idx = a1[2] as usize;
        let dst = world + 0x200 + idx * 72;
        wr::<[u64; 3]>(dst, a1);
        wr::<[u64; 3]>(dst + 24, a2);
        wr::<[u64; 3]>(dst + 48, a3);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41df_b350, mock_b350 as *const ())?;

    let g_804 = exe::ptr::<u16>(0x1_43c9_0804) as usize;
    let g_eb4 = exe::ptr::<u16>(0x1_43c8_ceb4) as usize;
    let g_338 = exe::ptr::<u16>(0x1_43c9_0338) as usize;
    let s_804 = rd::<u16>(g_804);
    let s_eb4 = rd::<u16>(g_eb4);
    let s_338 = rd::<u16>(g_338);
    wr::<u16>(g_804, 1);
    wr::<u16>(g_eb4, 2);
    wr::<u16>(g_338, 7);

    let run = |f: F| -> [u64; 18] {
        let lut: [u16; 4] = [0, 1, 2, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut r8_arr = [0u8; 0x40];
        let r8p = r8_arr.as_mut_ptr() as usize;
        wr::<u64>(r8p + 0x10, 0x1111_2222);
        wr::<u32>(r8p + 0x18, 4);
        wr::<u64>(r8p + 0x20, 0x3333_4444);
        wr::<u32>(r8p + 0x28, 9);

        let off_tbl: [u32; 2] = [0x10, 0x240];
        let mut world = vec![0u8; 0x18200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, r8p);
        wr::<usize>(wp + 0x88 + 8, 0x50000);
        wr::<u32>(wp + 0x10088 + 4, 2);
        wr::<u64>(wp + 0x18098 + 0x20, 0xabc0);
        wr::<u32>(wp + 0x180ac + 0x20, 3);
        wr::<usize>(wp + 0x180a0 + 0x20, off_tbl.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 1);
        rd::<[u64; 18]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_804, s_804);
    wr::<u16>(g_eb4, s_eb4);
    wr::<u16>(g_338, s_338);

    crate::diff_eq!(o, n, "140f4eb40 out");
    Ok(1)
}

pub unsafe fn diff_unknown_140fbac80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_worker_id() -> u32 {
        2
    }
    unsafe extern "win64" fn mock_a750(
        a1_ptr: usize,
        a2_ptr: usize,
        a3_ptr: usize,
        r9_val: u64,
        stack_a5: usize,
    ) {
        let mut a1 = rd::<[u64; 3]>(a1_ptr);
        let mut a2 = rd::<[u64; 3]>(a2_ptr);
        let mut a3 = rd::<[u64; 3]>(a3_ptr);
        let world = a1[0] as usize;
        a1[0] = 0;
        a2[0] = (a2[0] as usize).wrapping_sub(world) as u64;
        a3[0] = (a3[0] as usize).wrapping_sub(world) as u64;
        wr::<[u64; 3]>(world + 0x200, a1);
        wr::<[u64; 3]>(world + 0x218, a2);
        wr::<[u64; 3]>(world + 0x230, a3);
        wr::<u64>(world + 0x248, r9_val);
        wr::<u64>(world + 0x250, stack_a5.wrapping_sub(world) as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_worker_id as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e5_a750, mock_a750 as *const ())?;

    let g_c4c = exe::ptr::<u16>(0x1_43cc_bc4c) as usize;
    let g_c48 = exe::ptr::<u16>(0x1_43cc_bc48) as usize;
    let g_c44 = exe::ptr::<u16>(0x1_43cc_bc44) as usize;
    let g_2d0 = exe::ptr::<u16>(0x1_43c9_a2d0) as usize;
    let s_c4c = rd::<u16>(g_c4c);
    let s_c48 = rd::<u16>(g_c48);
    let s_c44 = rd::<u16>(g_c44);
    let s_2d0 = rd::<u16>(g_2d0);
    wr::<u16>(g_c4c, 0);
    wr::<u16>(g_c48, 1);
    wr::<u16>(g_c44, 2);
    wr::<u16>(g_2d0, 1);

    let run = |f: F| -> [u64; 11] {
        let lut: [u16; 4] = [2, 0, 1, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut r8_arr = [0u8; 0x40];
        let r8p = r8_arr.as_mut_ptr() as usize;
        wr::<u64>(r8p, 0x1010);
        wr::<u32>(r8p + 0x08, 3);
        wr::<u64>(r8p + 0x10, 0x2020);
        wr::<u32>(r8p + 0x18, 6);
        wr::<u64>(r8p + 0x20, 0x3030);
        wr::<u32>(r8p + 0x28, 9);

        let tbl_580e0: [u64; 2] = [0, 0x9988_7766];
        let mut world = vec![0u8; 0x58600];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, r8p);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize);
        rd::<[u64; 11]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_c4c, s_c4c);
    wr::<u16>(g_c48, s_c48);
    wr::<u16>(g_c44, s_c44);
    wr::<u16>(g_2d0, s_2d0);

    crate::diff_eq!(o, n, "140fbac80 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141046dc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_8660(
        out: usize,
        arch: u16,
        block: usize,
        cnt: u64,
        world: usize,
    ) -> usize {
        wr::<u64>(out, arch as u64);
        wr::<u64>(out + 0x08, block as u64);
        wr::<u64>(out + 0x10, world as u64);
        wr::<u64>(out + 0x18, cnt);
        for i in 4..11usize {
            wr::<u64>(out + i * 8, 0x2000 + i as u64);
        }
        out
    }
    unsafe extern "win64" fn mock_1010(rcx_buf: usize, rdx_val: u64) {
        let mut arr = rd::<[u64; 11]>(rcx_buf);
        let world = arr[2] as usize;
        arr[2] = 0;
        wr::<[u64; 11]>(world + 0x200, arr);
        wr::<u64>(world + 0x258, rdx_val);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4104_8660, mock_8660 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41eb_1010, mock_1010 as *const ())?;

    let g_0d8 = exe::ptr::<u16>(0x1_43c9_00d8) as usize;
    let s_0d8 = rd::<u16>(g_0d8);
    wr::<u16>(g_0d8, 1);

    let run = |f: F| -> ([u64; 11], u64) {
        let tbl_580e0: [u64; 2] = [0, 0xdead_beef_1234];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 3 * 8, 0x60000);
        wr::<u32>(wp + 0x10088 + 3 * 4, 8);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 3);
        (rd::<[u64; 11]>(wp + 0x200), rd::<u64>(wp + 0x258))
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_0d8, s_0d8);

    crate::diff_eq!(o, n, "141046dc0 out");
    Ok(1)
}

pub unsafe fn diff_r_14104aa80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_8a00(
        out: usize,
        _cnt: u64,
        _world_plus_40: usize,
        arch: u16,
    ) -> usize {
        wr::<u64>(out, 0);
        for i in 1..11usize {
            wr::<u64>(out + i * 8, (i as u64) * 0x100 + (arch as u64));
        }
        out
    }
    unsafe extern "win64" fn mock_1f70(rcx_buf: usize, rdx_0d8: u64, r8_f2c: u64) {
        let world = rd::<usize>(rcx_buf + 0x10);
        let arch = rd::<u16>(rcx_buf) as u64;
        let mut rest = rd::<[u64; 14]>(rcx_buf + 0x08);
        rest[1] = 0;
        wr::<u64>(world + 0x200, arch);
        wr::<[u64; 14]>(world + 0x208, rest);
        wr::<u64>(world + 0x278, rdx_0d8);
        wr::<u64>(world + 0x280, r8_f2c);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4104_8a00, mock_8a00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41eb_1f70, mock_1f70 as *const ())?;

    let g_f2c = exe::ptr::<u16>(0x1_43cc_ef2c) as usize;
    let g_0d8 = exe::ptr::<u16>(0x1_43c9_00d8) as usize;
    let s_f2c = rd::<u16>(g_f2c);
    let s_0d8 = rd::<u16>(g_0d8);
    wr::<u16>(g_f2c, 0);
    wr::<u16>(g_0d8, 1);

    let run = |f: F| -> [u64; 17] {
        let tbl_580e0: [u64; 2] = [0xaaaa_1111, 0xbbbb_2222];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x40_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 5);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let arch_list: [u16; 2] = [1, 2];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 17]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_f2c, s_f2c);
    wr::<u16>(g_0d8, s_0d8);

    crate::diff_eq!(o, n, "14104aa80 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141084400(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let cases: [[f32; 3]; 5] = [
        [0.0, -0.0, 1.0],
        [-0.0, 0.0, -0.0],
        [1.5, -2.25, 3.75],
        [12345.678, -0.000123, 42.0],
        [f32::INFINITY, f32::NEG_INFINITY, f32::from_bits(0x7fc0_1234)],
    ];
    for c in &cases {
        let ptr = c.as_ptr() as usize;
        let r_o = f_o(0, ptr);
        let r_n = f_n(0, ptr);
        crate::diff_eq!(r_o, r_n, "141084400 hash");
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_physics_14123e2b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut HEAP_BUF: [u8; 256] = [0; 256];
    static mut HEAP_OFF: usize = 0;
    static mut LAST_FREED: usize = 0;

    unsafe extern "win64" fn mock_alloc(sz: u64, _align: u64) -> usize {
        let base = core::ptr::addr_of_mut!(HEAP_BUF) as usize;
        let off = unsafe { HEAP_OFF };
        unsafe { HEAP_OFF = off + (sz as usize).max(16) };
        base + off
    }
    unsafe extern "win64" fn mock_free(ptr: usize) {
        unsafe { LAST_FREED = ptr };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;

    let run = |f: F| -> ([u8; 16], u32, u32, [u8; 8], u32, u32, bool) {
        // Case 1: in-place insert (count < tail_len and count >= tail_len)
        let mut storage = [0u8; 16];
        storage[..6].copy_from_slice(&[10, 20, 30, 40, 50, 60]);
        let sp = storage.as_mut_ptr() as usize;
        let mut vec_hdr = [0u8; 16];
        let vp = vec_hdr.as_mut_ptr() as usize;
        wr::<usize>(vp, sp);
        wr::<u32>(vp + 0x08, 6);
        wr::<u32>(vp + 0x0c, 16);

        let ins1: [u8; 2] = [91, 92];
        f(
            vp,
            sp + 1,
            ins1.as_ptr() as usize,
            ins1.as_ptr() as usize + 2,
        );
        let ins2: [u8; 4] = [81, 82, 83, 84];
        f(
            vp,
            sp + 6,
            ins2.as_ptr() as usize,
            ins2.as_ptr() as usize + 4,
        );
        let len1 = rd::<u32>(vp + 0x08);
        let cap1 = rd::<u32>(vp + 0x0c);

        // Case 2: reallocating insert in middle
        unsafe {
            HEAP_BUF = [0; 256];
            HEAP_OFF = 0;
            LAST_FREED = 0;
        }
        let mut init_buf: [u8; 4] = [1, 2, 3, 4];
        let ip = init_buf.as_mut_ptr() as usize;
        let mut vec2 = [0u8; 16];
        let v2p = vec2.as_mut_ptr() as usize;
        wr::<usize>(v2p, ip);
        wr::<u32>(v2p + 0x08, 4);
        wr::<u32>(v2p + 0x0c, 4);
        let ins3: [u8; 3] = [7, 8, 9];
        f(
            v2p,
            ip + 2,
            ins3.as_ptr() as usize,
            ins3.as_ptr() as usize + 3,
        );
        let new_ptr = rd::<usize>(v2p);
        let out2 = rd::<[u8; 8]>(new_ptr);
        let len2 = rd::<u32>(v2p + 0x08);
        let cap2 = rd::<u32>(v2p + 0x0c);
        let freed_ok = unsafe { LAST_FREED } == ip;

        (storage, len1, cap1, out2, len2, cap2, freed_ok)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14123e2b0 out");
    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_404b_7580, "unknown_1404b7580", rend::cauldron_hot_f::unknown_1404b7580 as *const (), Throws::Discard, rend::cauldron_hot_f::diff_unknown_1404b7580),
// Replacement::verified_throws(0x1_40f4_eb40, "anonymous_namespace_140f4eb40", rend::cauldron_hot_f::anonymous_namespace_140f4eb40 as *const (), Throws::Discard, rend::cauldron_hot_f::diff_anonymous_namespace_140f4eb40),
// Replacement::verified_throws(0x1_40fb_ac80, "unknown_140fbac80", rend::cauldron_hot_f::unknown_140fbac80 as *const (), Throws::Discard, rend::cauldron_hot_f::diff_unknown_140fbac80),
// Replacement::verified_throws(0x1_4104_6dc0, "unknown_141046dc0", rend::cauldron_hot_f::unknown_141046dc0 as *const (), Throws::Discard, rend::cauldron_hot_f::diff_unknown_141046dc0),
// Replacement::verified(0x1_4104_aa80, "r_14104aa80", rend::cauldron_hot_f::r_14104aa80 as *const (), rend::cauldron_hot_f::diff_r_14104aa80),
// Replacement::verified(0x1_4108_4400, "unknown_141084400", rend::cauldron_hot_f::unknown_141084400 as *const (), rend::cauldron_hot_f::diff_unknown_141084400),
// Replacement::verified_throws(0x1_4123_e2b0, "physics_14123e2b0", rend::cauldron_hot_f::physics_14123e2b0 as *const (), Throws::Discard, rend::cauldron_hot_f::diff_physics_14123e2b0),
