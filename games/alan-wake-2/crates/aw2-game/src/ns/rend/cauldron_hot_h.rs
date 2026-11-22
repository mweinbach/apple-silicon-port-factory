//! Decompiled hot functions shard H (`0x14103a0c0..0x14183e880`).

use crate::exe::{self, rd, wr};

#[inline(always)]
fn build_prefix_sums(hist: &[i32; 256], offsets: &mut [i32; 256]) {
    offsets[0] = 0;
    let mut i = 0usize;
    while i < 255 {
        let mut s = offsets[i].wrapping_add(hist[i]);
        offsets[i + 1] = s;
        s = hist[i + 1].wrapping_add(s);
        offsets[i + 2] = s;
        s = hist[i + 2].wrapping_add(s);
        offsets[i + 3] = s;
        s = hist[i + 3].wrapping_add(s);
        offsets[i + 4] = s;
        offsets[i + 5] = s.wrapping_add(hist[i + 4]);
        i += 5;
    }
}

/// `unknown_14103a0c0`
/// Original: 0x14103a0c0 (1674 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14103a0c0(param_1: usize, param_2: u32) {
    let n_signed = (param_2 as i32) as isize;
    let mut tmp_buf: usize = 0;
    if param_2 != 0 {
        let byte_size = (n_signed as usize).wrapping_mul(4);
        tmp_buf = exe::call_va!(0x1_4241_c0a0, (byte_size as u64, 4u64) -> usize);
        exe::call_va!(0x1_4183_acc2, (tmp_buf, 0u64, byte_size as u64));
    }

    let mut hist0 = [0i32; 256];
    let mut hist1 = [0i32; 256];
    let mut hist2 = [0i32; 256];
    let mut hist3 = [0i32; 256];

    if n_signed > 0 {
        let count = n_signed as usize;
        for i in 0..count {
            let v = rd::<u32>(param_1.wrapping_add(i * 4));
            let b0 = (v & 0xff) as usize;
            let b1 = ((v >> 8) & 0xff) as usize;
            let b2 = ((v >> 16) & 0xff) as usize;
            let b3 = (v >> 24) as usize;
            hist0[b0] = hist0[b0].wrapping_add(1);
            hist1[b1] = hist1[b1].wrapping_add(1);
            hist2[b2] = hist2[b2].wrapping_add(1);
            hist3[b3] = hist3[b3].wrapping_add(1);
        }
    }

    let mut rem = param_2 & 0x8000_0003;
    if (rem as i32) < 0 {
        rem = (rem.wrapping_sub(1) | 0xffff_fffc).wrapping_add(1);
    }
    let aligned_i32 = (param_2 as i32).wrapping_sub(rem as i32);
    let aligned_n = aligned_i32 as isize;

    let mut offsets = [0i32; 256];

    // Pass 0: byte 0 (param_1 -> tmp_buf)
    build_prefix_sums(&hist0, &mut offsets);
    if aligned_i32 > 0 {
        let groups = (((aligned_n as usize).wrapping_sub(1)) >> 2).wrapping_add(1);
        let mut p = param_1.wrapping_add(8);
        for _ in 0..groups {
            let v0 = rd::<u32>(p.wrapping_sub(8));
            let b0 = (v0 & 0xff) as usize;
            let b1 = rd::<u8>(p.wrapping_sub(4)) as usize;
            let b2 = rd::<u8>(p) as usize;
            let b3 = rd::<u8>(p.wrapping_add(4)) as usize;

            let idx0 = offsets[b0];
            offsets[b0] = idx0.wrapping_add(1);
            wr::<u32>(tmp_buf.wrapping_add((idx0 as isize as usize).wrapping_mul(4)), v0);

            let idx1 = offsets[b1];
            offsets[b1] = idx1.wrapping_add(1);
            let v1 = rd::<u32>(p.wrapping_sub(4));
            wr::<u32>(tmp_buf.wrapping_add((idx1 as isize as usize).wrapping_mul(4)), v1);

            let idx2 = offsets[b2];
            offsets[b2] = idx2.wrapping_add(1);
            let v2 = rd::<u32>(p);
            wr::<u32>(tmp_buf.wrapping_add((idx2 as isize as usize).wrapping_mul(4)), v2);

            let idx3 = offsets[b3];
            offsets[b3] = idx3.wrapping_add(1);
            let v3 = rd::<u32>(p.wrapping_add(4));
            wr::<u32>(tmp_buf.wrapping_add((idx3 as isize as usize).wrapping_mul(4)), v3);

            p = p.wrapping_add(0x10);
        }
    }
    if aligned_i32 < (param_2 as i32) {
        let mut p = param_1.wrapping_add((aligned_n as usize).wrapping_mul(4));
        let mut left = rem as usize;
        while left != 0 {
            let v = rd::<u32>(p);
            let b = (v & 0xff) as usize;
            let idx = offsets[b];
            offsets[b] = idx.wrapping_add(1);
            wr::<u32>(tmp_buf.wrapping_add((idx as isize as usize).wrapping_mul(4)), v);
            p = p.wrapping_add(4);
            left -= 1;
        }
    }

    // Pass 1: byte 1 (tmp_buf -> param_1)
    build_prefix_sums(&hist1, &mut offsets);
    if aligned_n > 0 {
        let groups = (((aligned_n as usize).wrapping_sub(1)) >> 2).wrapping_add(1);
        let mut p = tmp_buf.wrapping_add(5);
        for _ in 0..groups {
            let b1 = rd::<u8>(p) as usize;
            let b2 = rd::<u8>(p.wrapping_add(4)) as usize;
            let b3 = rd::<u8>(p.wrapping_add(8)) as usize;
            let b0 = rd::<u8>(p.wrapping_sub(4)) as usize;

            let idx0 = offsets[b0];
            offsets[b0] = idx0.wrapping_add(1);
            let v0 = rd::<u32>(p.wrapping_sub(5));
            wr::<u32>(param_1.wrapping_add((idx0 as isize as usize).wrapping_mul(4)), v0);

            let idx1 = offsets[b1];
            offsets[b1] = idx1.wrapping_add(1);
            let v1 = rd::<u32>(p.wrapping_sub(1));
            wr::<u32>(param_1.wrapping_add((idx1 as isize as usize).wrapping_mul(4)), v1);

            let idx2 = offsets[b2];
            offsets[b2] = idx2.wrapping_add(1);
            let v2 = rd::<u32>(p.wrapping_add(3));
            wr::<u32>(param_1.wrapping_add((idx2 as isize as usize).wrapping_mul(4)), v2);

            let idx3 = offsets[b3];
            offsets[b3] = idx3.wrapping_add(1);
            let v3 = rd::<u32>(p.wrapping_add(7));
            wr::<u32>(param_1.wrapping_add((idx3 as isize as usize).wrapping_mul(4)), v3);

            p = p.wrapping_add(0x10);
        }
    }
    let mut r9 = aligned_n;
    while r9 < n_signed {
        let elem_addr = tmp_buf.wrapping_add((r9 as usize).wrapping_mul(4));
        let b = rd::<u8>(elem_addr.wrapping_add(1)) as usize;
        let idx = offsets[b];
        offsets[b] = idx.wrapping_add(1);
        let v = rd::<u32>(elem_addr);
        wr::<u32>(param_1.wrapping_add((idx as isize as usize).wrapping_mul(4)), v);
        r9 += 1;
    }

    // Pass 2: byte 2 (param_1 -> tmp_buf)
    build_prefix_sums(&hist2, &mut offsets);
    if aligned_n > 0 {
        let groups = (((aligned_n as usize).wrapping_sub(1)) >> 2).wrapping_add(1);
        let mut p = param_1.wrapping_add(6);
        for _ in 0..groups {
            let b1 = rd::<u8>(p) as usize;
            let b2 = rd::<u8>(p.wrapping_add(4)) as usize;
            let b3 = rd::<u8>(p.wrapping_add(8)) as usize;
            let b0 = rd::<u8>(p.wrapping_sub(4)) as usize;

            let idx0 = offsets[b0];
            offsets[b0] = idx0.wrapping_add(1);
            let v0 = rd::<u32>(p.wrapping_sub(6));
            wr::<u32>(tmp_buf.wrapping_add((idx0 as isize as usize).wrapping_mul(4)), v0);

            let idx1 = offsets[b1];
            offsets[b1] = idx1.wrapping_add(1);
            let v1 = rd::<u32>(p.wrapping_sub(2));
            wr::<u32>(tmp_buf.wrapping_add((idx1 as isize as usize).wrapping_mul(4)), v1);

            let idx2 = offsets[b2];
            offsets[b2] = idx2.wrapping_add(1);
            let v2 = rd::<u32>(p.wrapping_add(2));
            wr::<u32>(tmp_buf.wrapping_add((idx2 as isize as usize).wrapping_mul(4)), v2);

            let idx3 = offsets[b3];
            offsets[b3] = idx3.wrapping_add(1);
            let v3 = rd::<u32>(p.wrapping_add(6));
            wr::<u32>(tmp_buf.wrapping_add((idx3 as isize as usize).wrapping_mul(4)), v3);

            p = p.wrapping_add(0x10);
        }
    }
    let mut r9 = aligned_n;
    while r9 < n_signed {
        let elem_addr = param_1.wrapping_add((r9 as usize).wrapping_mul(4));
        let b = rd::<u8>(elem_addr.wrapping_add(2)) as usize;
        let idx = offsets[b];
        offsets[b] = idx.wrapping_add(1);
        let v = rd::<u32>(elem_addr);
        wr::<u32>(tmp_buf.wrapping_add((idx as isize as usize).wrapping_mul(4)), v);
        r9 += 1;
    }

    // Pass 3: byte 3 (tmp_buf -> param_1)
    build_prefix_sums(&hist3, &mut offsets);
    if aligned_n > 0 {
        let groups = (((aligned_n as usize).wrapping_sub(1)) >> 2).wrapping_add(1);
        let mut p = tmp_buf.wrapping_add(8);
        for _ in 0..groups {
            let v0 = rd::<u32>(p.wrapping_sub(8));
            let b0 = (v0 >> 24) as usize;
            let b1 = rd::<u8>(p.wrapping_sub(1)) as usize;
            let b2 = rd::<u8>(p.wrapping_add(3)) as usize;
            let b3 = rd::<u8>(p.wrapping_add(7)) as usize;

            let idx0 = offsets[b0];
            offsets[b0] = idx0.wrapping_add(1);
            wr::<u32>(param_1.wrapping_add((idx0 as isize as usize).wrapping_mul(4)), v0);

            let idx1 = offsets[b1];
            offsets[b1] = idx1.wrapping_add(1);
            let v1 = rd::<u32>(p.wrapping_sub(4));
            wr::<u32>(param_1.wrapping_add((idx1 as isize as usize).wrapping_mul(4)), v1);

            let idx2 = offsets[b2];
            offsets[b2] = idx2.wrapping_add(1);
            let v2 = rd::<u32>(p);
            wr::<u32>(param_1.wrapping_add((idx2 as isize as usize).wrapping_mul(4)), v2);

            let idx3 = offsets[b3];
            offsets[b3] = idx3.wrapping_add(1);
            let v3 = rd::<u32>(p.wrapping_add(4));
            wr::<u32>(param_1.wrapping_add((idx3 as isize as usize).wrapping_mul(4)), v3);

            p = p.wrapping_add(0x10);
        }
    }
    if aligned_i32 < (param_2 as i32) {
        let mut p = tmp_buf.wrapping_add((aligned_n as usize).wrapping_mul(4));
        let mut left = rem as usize;
        while left != 0 {
            let v = rd::<u32>(p);
            let b = (v >> 24) as usize;
            let idx = offsets[b];
            offsets[b] = idx.wrapping_add(1);
            wr::<u32>(param_1.wrapping_add((idx as isize as usize).wrapping_mul(4)), v);
            p = p.wrapping_add(4);
            left -= 1;
        }
    }

    if tmp_buf != 0 {
        exe::call_va!(0x1_4241_c190, (tmp_buf));
    }
}

/// `unknown_14105fff0`
/// Original: 0x14105fff0 (625 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14105fff0(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let world = rd::<usize>(param_1);
        let id_014 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_6014) as usize) as usize;
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let id_d94 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_5d94) as usize) as usize;
        let world_plus_40 = world.wrapping_add(0x40);
        let val_014 = rd::<u64>(tbl_580e0.wrapping_add(id_014 * 8));
        let val_d94 = rd::<u64>(tbl_580e0.wrapping_add(id_d94 * 8));

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world_plus_40.wrapping_add(arch * 8).wrapping_add(0x48));
            let ent_count = if block != 0 {
                rd::<u32>(world_plus_40.wrapping_add(arch * 4).wrapping_add(0x10048)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_ee0 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_5ee0) as usize) as u32;
                let arch_off = arch.wrapping_shl(5);
                let search_len =
                    rd::<u32>(world_plus_40.wrapping_add(arch_off).wrapping_add(0x1806c)) as u64;
                let search_ptr =
                    rd::<u64>(world_plus_40.wrapping_add(arch_off).wrapping_add(0x18058));

                let found1 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_ee0) -> i32);
                let off_slot = world_plus_40.wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5));
                let mut off1 = if (found1 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found1 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_87c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d87c) as usize) as u32;
                let found2 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_87c) -> i32);
                let mut off2 = if (found2 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found2 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_ff8 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_5ff8) as usize) as u32;
                let found3 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_ff8) -> i32);
                let mut off3 = if (found3 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found3 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                if off1 != 0 {
                    off1 = off1.wrapping_add(block);
                }
                if off2 != 0 {
                    off2 = off2.wrapping_add(block);
                }
                if off3 != 0 {
                    off3 = off3.wrapping_add(block);
                }

                for idx in 0..ent_count {
                    let mut pkt: [u64; 5] = [
                        off1 as u64,
                        off2 as u64,
                        off3 as u64,
                        block as u64,
                        idx,
                    ];
                    exe::call_va!(
                        0x1_41ec_6a90,
                        (pkt.as_mut_ptr() as usize, val_d94, val_014)
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `r_1410c60e0`
/// Original: 0x1410c60e0 (316 bytes, Throws::Discard)
pub unsafe extern "win64" fn r_1410c60e0(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088)) as u64
    } else {
        0
    };
    if count != 0 {
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let id_904 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d904) as usize) as usize;
        let want_930 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_f930) as usize) as u32;
        let id_4a0 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_04a0) as usize) as usize;
        let val_904 = rd::<u64>(tbl_580e0.wrapping_add(id_904 * 8));
        let val_4a0 = rd::<u64>(tbl_580e0.wrapping_add(id_4a0 * 8));

        let arch_off = arch.wrapping_shl(5);
        let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac)) as u64;
        let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
        let found = exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_930) -> i32);

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

        for idx in 0..count {
            let mut pkt: [u64; 3] = [comp_off as u64, block as u64, idx];
            exe::call_va!(
                0x1_41f4_5ff0,
                (pkt.as_mut_ptr() as usize, world as u64, val_4a0, val_904)
            );
        }
    }
}

/// `unknown_141171890`
/// Original: 0x141171890 (566 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141171890(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let mut triples = [0u64; 9];
        let _ = exe::call_va!(
            0x1_4117_1030,
            (triples.as_mut_ptr() as usize, param_1) -> usize
        );
        let world = rd::<usize>(param_1);
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let world_plus_40 = world.wrapping_add(0x40);

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world_plus_40.wrapping_add(arch * 8).wrapping_add(0x48));
            let ent_count = if block != 0 {
                rd::<u32>(world_plus_40.wrapping_add(arch * 4).wrapping_add(0x10048)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let mut raw_offs = [0u64; 6];
                let _ = exe::call_va!(
                    0x1_4117_1670,
                    (
                        raw_offs.as_mut_ptr() as usize,
                        ent_count,
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
                for idx in 0..ent_count {
                    let mut arg4: [u64; 3] = [triples[0], triples[1], triples[2]];
                    let mut arg3: [u64; 3] = [triples[3], triples[4], triples[5]];
                    let mut arg2: [u64; 3] = [triples[6], triples[7], triples[8]];
                    let mut arg1: [u64; 8] = [
                        raw_offs[0],
                        raw_offs[1],
                        raw_offs[2],
                        raw_offs[3],
                        raw_offs[4],
                        raw_offs[5],
                        block_u64,
                        idx,
                    ];
                    exe::call_va!(
                        0x1_41fd_6a00,
                        (
                            arg1.as_mut_ptr() as usize,
                            arg2.as_mut_ptr() as usize,
                            arg3.as_mut_ptr() as usize,
                            arg4.as_mut_ptr() as usize
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_141184850`
/// Original: 0x141184850 (241 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141184850(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if count != 0 {
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let id_63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
        let val_63c = rd::<u64>(tbl_580e0.wrapping_add(id_63c * 8));
        let id_910 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d910) as usize) as usize;
        let val_910 = rd::<u64>(tbl_580e0.wrapping_add(id_910 * 8));

        let mut buf = [0u64; 22];
        let _ = exe::call_va!(
            0x1_4118_5300,
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
            0x1_41fe_4c30,
            (buf_copy.as_mut_ptr() as usize, val_63c, val_910)
        );
    }
}

/// `unknown_141184a70`
/// Original: 0x141184a70 (169 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141184a70(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if count != 0 {
        let id_63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_63c = rd::<u64>(tbl_580e0.wrapping_add(id_63c * 8));

        let mut buf = [0u64; 10];
        let _ = exe::call_va!(
            0x1_4118_50e0,
            (
                buf.as_mut_ptr() as usize,
                param_2 as u64,
                block,
                count as u64,
                world
            ) -> usize
        );
        let mut buf_copy = buf;
        exe::call_va!(0x1_41fe_4aa0, (buf_copy.as_mut_ptr() as usize, val_63c));
    }
}

/// `unknown_1411f4330`
/// Original: 0x1411f4330 (63 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411f4330(param_1: usize, param_2: u16) -> u8 {
    let head = rd::<usize>(param_1 + 0x28);
    let mut best = head;
    let mut cur = rd::<usize>(head + 0x08);
    while rd::<u8>(cur + 0x19) == 0 {
        if rd::<u16>(cur + 0x1a) < param_2 {
            cur = rd::<usize>(cur + 0x10);
        } else {
            best = cur;
            cur = rd::<usize>(cur);
        }
    }
    if rd::<u8>(best + 0x19) != 0 || param_2 < rd::<u16>(best + 0x1a) {
        best = head;
    }
    if best != head {
        1
    } else {
        0
    }
}

/// `rend_1412f9000`
/// Original: 0x1412f9000 (1083 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412f9000(
    param_1: usize,
    param_2: usize,
    param_3: isize,
    param_4: u8,
) {
    let mut last = rd::<usize>(param_2);
    let mut first = rd::<usize>(param_1);
    let mut ideal = param_3;

    loop {
        let byte_diff = (last.wrapping_sub(first) & !3usize) as isize;
        if byte_diff <= 0x80 {
            if first != last {
                let mut rcx = first.wrapping_add(4);
                while rcx != last {
                    let r9d = rd::<u32>(rcx);
                    let mut rdx = rcx;
                    if r9d > rd::<u32>(first) {
                        if first != rcx {
                            let mut rax = rcx;
                            let mut r8 = rcx.wrapping_add(4);
                            loop {
                                let edx = rd::<u32>(rax.wrapping_sub(4));
                                r8 = r8.wrapping_sub(4);
                                rax = rax.wrapping_sub(4);
                                wr::<u32>(r8, edx);
                                if rax == first {
                                    break;
                                }
                            }
                        }
                        wr::<u32>(first, r9d);
                    } else {
                        let mut r8d = rd::<u32>(rcx.wrapping_sub(4));
                        let mut rax = rcx.wrapping_sub(4);
                        while r9d > r8d {
                            wr::<u32>(rdx, r8d);
                            rdx = rax;
                            r8d = rd::<u32>(rax.wrapping_sub(4));
                            rax = rax.wrapping_sub(4);
                        }
                        wr::<u32>(rdx, r9d);
                    }
                    rcx = rcx.wrapping_add(4);
                }
            }
            return;
        }

        let raw_diff = last.wrapping_sub(first) as isize;
        if ideal <= 0 {
            let r9 = raw_diff >> 2;
            let mut r8 = r9 >> 1;
            if r8 > 0 {
                let r11 = (r9 - 1) >> 1;
                while r8 > 0 {
                    let r10d = rd::<u32>(first.wrapping_add((r8 as usize - 1) * 4));
                    r8 -= 1;
                    let mut rdx = r8;
                    let mut rcx = r8;
                    while rcx < r11 {
                        rcx *= 2;
                        let right = rd::<u32>(first.wrapping_add((rcx as usize) * 4 + 8));
                        let left = rd::<u32>(first.wrapping_add((rcx as usize) * 4 + 4));
                        let step = if left < right { 1isize } else { 2isize };
                        rcx += step;
                        let child_val = rd::<u32>(first.wrapping_add((rcx as usize) * 4));
                        wr::<u32>(first.wrapping_add((rdx as usize) * 4), child_val);
                        rdx = rcx;
                    }
                    if rdx == r11 && (r9 & 1) == 0 {
                        let last_val = rd::<u32>(first.wrapping_add((r9 as usize - 1) * 4));
                        wr::<u32>(first.wrapping_add((rdx as usize) * 4), last_val);
                        rdx = r9 - 1;
                    }
                    let mut rax = (rdx - 1) >> 1;
                    while r8 < rdx {
                        let ecx = rd::<u32>(first.wrapping_add((rax as usize) * 4));
                        if ecx <= r10d {
                            break;
                        }
                        wr::<u32>(first.wrapping_add((rdx as usize) * 4), ecx);
                        rdx = rax;
                        rax = (rax - 1) >> 1;
                    }
                    wr::<u32>(first.wrapping_add((rdx as usize) * 4), r10d);
                }
            }

            let mut rcx = rd::<usize>(param_2);
            let rdi = rd::<usize>(param_1);
            while ((rcx.wrapping_sub(rdi) & !3usize) as isize) >= 8 {
                let rbx = rcx.wrapping_sub(4);
                if ((rcx.wrapping_sub(rdi) & !3usize) as isize) >= 8 {
                    let mut val_slot: u32 = rd::<u32>(rbx);
                    wr::<u32>(rbx, rd::<u32>(rdi));
                    let mut rdi_slot: usize = rdi;
                    let hole_count = (rbx.wrapping_sub(rdi) as isize) >> 2;
                    exe::call_va!(
                        0x1_412f_9540,
                        (
                            (&mut rdi_slot as *mut usize) as usize,
                            0u64,
                            hole_count as u64,
                            (&mut val_slot as *mut u32) as usize,
                            param_4 as u64
                        )
                    );
                }
                rcx = rbx;
            }
            return;
        }

        let half = raw_diff >> 3;
        let mut rbx = first.wrapping_add((half as usize).wrapping_mul(4));
        let mut arg_first = first;
        let mut arg_mid = rbx;
        let mut arg_last_m1 = last.wrapping_sub(4);
        exe::call_va!(
            0x1_412f_9600,
            (
                (&mut arg_first as *mut usize) as usize,
                (&mut arg_mid as *mut usize) as usize,
                (&mut arg_last_m1 as *mut usize) as usize,
                param_4 as u64
            )
        );

        let mut rdi = rbx.wrapping_add(4);
        if first < rbx {
            let mut piv = rd::<u32>(rbx);
            loop {
                let rdx = rbx.wrapping_sub(4);
                let prev = rd::<u32>(rdx);
                if prev != piv {
                    break;
                }
                rbx = rdx;
                piv = prev;
                if first >= rdx {
                    break;
                }
            }
        }
        if rdi < last {
            let piv = rd::<u32>(rbx);
            while rdi < last {
                if rd::<u32>(rdi) != piv {
                    break;
                }
                rdi = rdi.wrapping_add(4);
            }
        }

        let mut rax = rdi;
        let mut rcx = rbx;
        loop {
            while rax < last {
                let r8d = rd::<u32>(rax);
                let piv = rd::<u32>(rbx);
                if piv > r8d {
                    rax = rax.wrapping_add(4);
                    continue;
                }
                if piv < r8d {
                    break;
                }
                if rdi != rax {
                    let edx = rd::<u32>(rdi);
                    wr::<u32>(rdi, r8d);
                    wr::<u32>(rax, edx);
                }
                rdi = rdi.wrapping_add(4);
                rax = rax.wrapping_add(4);
            }

            while rcx > first {
                let rdx = rcx.wrapping_sub(4);
                let r9d = rd::<u32>(rdx);
                let piv = rd::<u32>(rbx);
                if r9d > piv {
                    rcx = rcx.wrapping_sub(4);
                    continue;
                }
                if r9d < piv {
                    break;
                }
                rbx = rbx.wrapping_sub(4);
                if rbx != rdx {
                    let r8d = rd::<u32>(rbx);
                    wr::<u32>(rbx, r9d);
                    wr::<u32>(rdx, r8d);
                }
                rcx = rcx.wrapping_sub(4);
            }

            if rcx == first {
                if rax == last {
                    break;
                }
                if rdi != rax {
                    let r8d = rd::<u32>(rbx);
                    let edx = rd::<u32>(rdi);
                    wr::<u32>(rbx, edx);
                    wr::<u32>(rdi, r8d);
                }
                let r8d = rd::<u32>(rbx);
                rdi = rdi.wrapping_add(4);
                let edx = rd::<u32>(rax);
                wr::<u32>(rbx, edx);
                rbx = rbx.wrapping_add(4);
                wr::<u32>(rax, r8d);
                rax = rax.wrapping_add(4);
            } else {
                rcx = rcx.wrapping_sub(4);
                if rax == last {
                    rbx = rbx.wrapping_sub(4);
                    if rcx != rbx {
                        let r8d = rd::<u32>(rcx);
                        let edx = rd::<u32>(rbx);
                        wr::<u32>(rcx, edx);
                        wr::<u32>(rbx, r8d);
                    }
                    let edx = rd::<u32>(rdi.wrapping_sub(4));
                    rdi = rdi.wrapping_sub(4);
                    let r8d = rd::<u32>(rbx);
                    wr::<u32>(rbx, edx);
                    wr::<u32>(rdi, r8d);
                } else {
                    let r8d = rd::<u32>(rax);
                    let edx = rd::<u32>(rcx);
                    wr::<u32>(rax, edx);
                    rax = rax.wrapping_add(4);
                    wr::<u32>(rcx, r8d);
                }
            }
        }

        let cur_last = rd::<usize>(param_2);
        let cur_first = rd::<usize>(param_1);
        ideal = (ideal >> 1).wrapping_add(ideal >> 2);
        let left_bytes = (rbx.wrapping_sub(cur_first) & !3usize) as isize;
        let right_bytes = (cur_last.wrapping_sub(rdi) & !3usize) as isize;
        if left_bytes < right_bytes {
            let mut sub_first = cur_first;
            let mut sub_last = rbx;
            exe::call_va!(
                0x1_412f_9000,
                (
                    (&mut sub_first as *mut usize) as usize,
                    (&mut sub_last as *mut usize) as usize,
                    ideal as u64,
                    param_4 as u64
                )
            );
            wr::<usize>(param_1, rdi);
        } else {
            let mut sub_first = rdi;
            let mut sub_last = cur_last;
            exe::call_va!(
                0x1_412f_9000,
                (
                    (&mut sub_first as *mut usize) as usize,
                    (&mut sub_last as *mut usize) as usize,
                    ideal as u64,
                    param_4 as u64
                )
            );
            wr::<usize>(param_2, rbx);
        }
        last = rd::<usize>(param_2);
        first = rd::<usize>(param_1);
    }
}

/// `unknown_1415f1fd0`
/// Original: 0x1415f1fd0 (286 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1415f1fd0(param_1: usize) -> usize {
    let flag: u8 = if rd::<u32>(exe::ptr::<u32>(0x1_4393_1b28) as usize) == 2
        && rd::<u8>(exe::ptr::<u8>(0x1_4381_5ae0) as usize) == 0
    {
        0
    } else {
        1
    };

    let perf_ctx = rd::<usize>(exe::ptr::<usize>(0x1_4393_0f68) as usize);
    if rd::<u32>(perf_ctx + 0x08) != 0 {
        exe::call_iat!(0x1_4264_56f8, (perf_ctx.wrapping_add(0x48)));
    }

    let w_2b4 = rd::<u16>(exe::ptr::<u16>(0x1_4381_52b4) as usize);
    exe::call_va!(0x1_4160_c570, (w_2b4 as u32));
    exe::call_va!(0x1_415e_e330, ());
    exe::call_va!(0x1_4159_9270, ());

    let mut node = rd::<usize>(exe::ptr::<usize>(0x1_4393_1b60) as usize);
    while node != 0 {
        exe::call_va!(0x1_415e_ac20, (node));
        node = rd::<usize>(node + 0x28);
    }

    let mut local_18 = [0u64; 2];
    let rbx = exe::call_va!(
        0x1_415f_1590,
        (local_18.as_mut_ptr() as usize, flag as u32) -> usize
    );

    let mut slot_60: u64 = u64::MAX;
    let mut slot_68: usize = 0;
    let str_e08 = exe::ptr::<u8>(0x1_4268_de08) as usize;
    exe::call_va!(
        0x1_415b_1e50,
        (
            (&mut slot_68 as *mut usize) as usize,
            (&mut slot_60 as *mut u64) as usize,
            0x10u64,
            0x8u64,
            str_e08,
            0u64,
            1u64
        )
    );
    if slot_68 != 0 {
        let vtbl_f28 = exe::ptr::<u8>(0x1_4268_df28) as usize;
        wr::<usize>(slot_68, vtbl_f28);
        wr::<u8>(slot_68 + 0x08, slot_60 as u8);
    }
    slot_68 = slot_60 as usize;
    slot_60 = u64::MAX;
    let _ = slot_60;
    let r9_val = rd::<u64>(rbx + 0x08);
    exe::call_va!(
        0x1_415b_1720,
        (param_1, (&mut slot_68 as *mut usize) as usize, 1u64, r9_val)
    );
    param_1
}

/// `unknown_14183e880`
/// Original: 0x14183e880 (292 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14183e880(param_1: usize) {
    let tbl = exe::ptr::<usize>(0x1_43c4_aa00) as usize;
    let idx0 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let p0 = rd::<usize>(tbl.wrapping_add(idx0 * 8));
    if p0 != 0 && rd::<i32>(p0) == 1 {
        exe::call_va!(0x1_4247_58f0, ());
        let rbx = rd::<usize>(param_1 + 0x48);
        let idx1 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let arg_rcx = rd::<u64>(rbx.wrapping_add(idx1 * 8).wrapping_add(0x2d8));
        exe::call_va!(0x1_4247_5db0, (arg_rcx, 0xffff_ffffu64));
        exe::call_va!(0x1_4247_58f0, ());
        let p78 = rd::<usize>(param_1 + 0x78);
        if p78 != 0 {
            exe::call_va!(0x1_4242_2290, (p78));
            exe::call_va!(0x1_4247_58f0, ());
            let p78_2 = rd::<usize>(param_1 + 0x78);
            exe::call_va!(0x1_4242_2da0, (p78_2));
            exe::call_va!(0x1_4247_58f0, ());
        }
        exe::call_va!(0x1_401e_30f0, ());
        let idx2 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let p2 = rd::<usize>(tbl.wrapping_add(idx2 * 8));
        if p2 != 0 && rd::<i32>(p2) == 1 {
            exe::call_va!(0x1_4247_58f0, ());
        }
        exe::call_iat!(0x1_4264_5868, ());
        return;
    }

    let idx3 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
    let p3 = rd::<usize>(tbl.wrapping_add(idx3 * 8));
    if p3 != 0 && rd::<i32>(p3) == 3 {
        exe::call_va!(0x1_4247_58f0, ());
        let rbx = rd::<usize>(param_1 + 0x48);
        let idx4 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
        let arg_rcx = rd::<u64>(rbx.wrapping_add(idx4 * 8).wrapping_add(0x2d8));
        exe::call_va!(0x1_4247_5db0, (arg_rcx, 0xffff_ffffu64));
        let g_698 = rd::<usize>(exe::ptr::<usize>(0x1_43a3_4698) as usize);
        if g_698 != 0 {
            let idx5 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let p5 = rd::<usize>(tbl.wrapping_add(idx5 * 8));
            if p5 != 0 && rd::<i32>(p5) == 3 {
                let sub_obj = rd::<usize>(g_698 + 0x18).wrapping_add(0x30);
                exe::call_va!(0x1_4208_a430, (sub_obj));
            }
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_14103a0c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RADIX_SCRATCH: [u32; 64] = [0; 64];
    static mut LAST_ALLOC_ALIGN: u64 = 0;
    static mut FREED_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_alloc(_sz: u64, align: u64) -> usize {
        unsafe { LAST_ALLOC_ALIGN = align };
        core::ptr::addr_of_mut!(RADIX_SCRATCH) as usize
    }
    unsafe extern "win64" fn mock_free(ptr: usize) {
        if ptr == core::ptr::addr_of_mut!(RADIX_SCRATCH) as usize {
            unsafe { FREED_COUNT += 1 };
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;

    let init_data: [u32; 11] = [
        0xff01_0203,
        0x1234_5678,
        0x0000_0001,
        0xdead_beef,
        0x1234_5600,
        0x8000_0000,
        0x00ff_00ff,
        0x0102_0304,
        0xffff_ffff,
        0x0000_0000,
        0x1234_5678,
    ];

    let run = |f: F| -> ([u32; 11], u64, u32) {
        unsafe {
            RADIX_SCRATCH = [0; 64];
            LAST_ALLOC_ALIGN = 0;
            FREED_COUNT = 0;
        }
        let mut arr = init_data;
        f(arr.as_mut_ptr() as usize, 0);
        f(arr.as_mut_ptr() as usize, arr.len() as u32);
        (arr, unsafe { LAST_ALLOC_ALIGN }, unsafe { FREED_COUNT })
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14103a0c0 radix sort");
    Ok(2)
}

pub unsafe fn diff_unknown_14105fff0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x5550 && n == 4 {
            match want {
                10 => 0,
                20 => 1,
                30 => -1,
                _ => -1,
            }
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_6a90(rcx_pkt: usize, rdx_d94: u64, r8_014: u64) {
        let pkt = rd::<[u64; 5]>(rcx_pkt);
        let idx = pkt[4] as usize;
        let dst = (rdx_d94 as usize).wrapping_add(idx * 0x38);
        wr::<[u64; 5]>(dst, pkt);
        wr::<u64>(dst + 0x28, 0x1111_0000);
        wr::<u64>(dst + 0x30, r8_014);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41ec_6a90, mock_6a90 as *const ())?;

    let g_014 = exe::ptr::<u16>(0x1_43cd_6014) as usize;
    let g_d94 = exe::ptr::<u16>(0x1_43cd_5d94) as usize;
    let g_ee0 = exe::ptr::<u16>(0x1_43cd_5ee0) as usize;
    let g_87c = exe::ptr::<u16>(0x1_43c8_d87c) as usize;
    let g_ff8 = exe::ptr::<u16>(0x1_43cd_5ff8) as usize;
    let s_014 = rd::<u16>(g_014);
    let s_d94 = rd::<u16>(g_d94);
    let s_ee0 = rd::<u16>(g_ee0);
    let s_87c = rd::<u16>(g_87c);
    let s_ff8 = rd::<u16>(g_ff8);
    wr::<u16>(g_014, 0);
    wr::<u16>(g_d94, 1);
    wr::<u16>(g_ee0, 10);
    wr::<u16>(g_87c, 20);
    wr::<u16>(g_ff8, 30);

    let run = |f: F| -> [u64; 14] {
        let mut out_sink = [0u64; 14];
        let tbl_580e0: [u64; 2] = [0x9999_8888, out_sink.as_mut_ptr() as u64];
        let off_tbl: [u32; 2] = [0x100, 0x240];

        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);
        wr::<usize>(wp + 0x88 + 1 * 8, 0x70_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);
        wr::<u64>(wp + 0x40 + 0x20 + 0x18058, 0x5550);
        wr::<u32>(wp + 0x40 + 0x20 + 0x1806c, 4);
        wr::<usize>(
            wp + 0x40 + (1 + 0xc03) * 0x20,
            off_tbl.as_ptr() as usize,
        );

        let arch_list: [u16; 2] = [0, 1];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        out_sink
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_014, s_014);
    wr::<u16>(g_d94, s_d94);
    wr::<u16>(g_ee0, s_ee0);
    wr::<u16>(g_87c, s_87c);
    wr::<u16>(g_ff8, s_ff8);

    crate::diff_eq!(o, n, "14105fff0 out");
    Ok(1)
}

pub unsafe fn diff_r_1410c60e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x7770 && n == 3 && want == 42 {
            1
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_5ff0(
        rcx_pkt: usize,
        rdx_world: usize,
        r8_4a0: u64,
        r9_904: u64,
    ) {
        let pkt = rd::<[u64; 3]>(rcx_pkt);
        let idx = pkt[2] as usize;
        let dst = rdx_world + 0x200 + idx * 0x28;
        wr::<[u64; 3]>(dst, pkt);
        wr::<u64>(dst + 0x18, r8_4a0);
        wr::<u64>(dst + 0x20, r9_904);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f4_5ff0, mock_5ff0 as *const ())?;

    let g_904 = exe::ptr::<u16>(0x1_43c8_d904) as usize;
    let g_930 = exe::ptr::<u16>(0x1_43c9_f930) as usize;
    let g_4a0 = exe::ptr::<u16>(0x1_43c9_04a0) as usize;
    let s_904 = rd::<u16>(g_904);
    let s_930 = rd::<u16>(g_930);
    let s_4a0 = rd::<u16>(g_4a0);
    wr::<u16>(g_904, 0);
    wr::<u16>(g_930, 42);
    wr::<u16>(g_4a0, 1);

    let run = |f: F| -> [u64; 10] {
        let tbl_580e0: [u64; 2] = [0xaaaa_bbbb, 0xcccc_dddd];
        let off_tbl: [u32; 2] = [0x20, 0x380];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);
        wr::<usize>(wp + 0x88 + 1 * 8, 0x90_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);
        wr::<u64>(wp + 0x18098 + 0x20, 0x7770);
        wr::<u32>(wp + 0x180ac + 0x20, 3);
        wr::<usize>(wp + (1 + 0xc03) * 0x20 + 0x40, off_tbl.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 1);
        rd::<[u64; 10]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_904, s_904);
    wr::<u16>(g_930, s_930);
    wr::<u16>(g_4a0, s_4a0);

    crate::diff_eq!(o, n, "1410c60e0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141171890(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_1030(out: usize, p1: usize) -> usize {
        let world = rd::<usize>(p1) as u64;
        for i in 0..9usize {
            wr::<u64>(out + i * 8, if i % 3 == 0 { world } else { 0x100 + i as u64 });
        }
        out
    }
    unsafe extern "win64" fn mock_1670(
        out: usize,
        cnt: u64,
        _world_plus_40: usize,
        arch: u16,
    ) -> usize {
        wr::<u64>(out, 0);
        for i in 1..6usize {
            wr::<u64>(out + i * 8, (i as u64) * 0x100 + (arch as u64) + cnt);
        }
        out
    }
    unsafe extern "win64" fn mock_6a00(
        a1_ptr: usize,
        a2_ptr: usize,
        a3_ptr: usize,
        a4_ptr: usize,
    ) {
        let a1 = rd::<[u64; 8]>(a1_ptr);
        let mut a2 = rd::<[u64; 3]>(a2_ptr);
        let mut a3 = rd::<[u64; 3]>(a3_ptr);
        let mut a4 = rd::<[u64; 3]>(a4_ptr);
        let world = a4[0] as usize;
        a2[0] = (a2[0] as usize).wrapping_sub(world) as u64;
        a3[0] = (a3[0] as usize).wrapping_sub(world) as u64;
        a4[0] = 0;
        let idx = a1[7] as usize;
        let dst = world + 0x200 + idx * (17 * 8);
        wr::<[u64; 8]>(dst, a1);
        wr::<[u64; 3]>(dst + 64, a2);
        wr::<[u64; 3]>(dst + 88, a3);
        wr::<[u64; 3]>(dst + 112, a4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4117_1030, mock_1030 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4117_1670, mock_1670 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41fd_6a00, mock_6a00 as *const ())?;

    let run = |f: F| -> [u64; 34] {
        let mut world = vec![0u8; 0x10200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x30_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 2);

        let arch_list: [u16; 2] = [1, 2];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 34]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141171890 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141184850(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_5300(
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
        for i in 4..22usize {
            wr::<u64>(out + i * 8, 0x3000 + i as u64);
        }
        out
    }
    unsafe extern "win64" fn mock_4c30(rcx_buf: usize, rdx_63c: u64, r8_910: u64) {
        let mut arr = rd::<[u64; 22]>(rcx_buf);
        let world = arr[2] as usize;
        arr[2] = 0;
        wr::<[u64; 22]>(world + 0x200, arr);
        wr::<u64>(world + 0x2b0, rdx_63c);
        wr::<u64>(world + 0x2b8, r8_910);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4118_5300, mock_5300 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41fe_4c30, mock_4c30 as *const ())?;

    let g_63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let g_910 = exe::ptr::<u16>(0x1_43c8_d910) as usize;
    let s_63c = rd::<u16>(g_63c);
    let s_910 = rd::<u16>(g_910);
    wr::<u16>(g_63c, 0);
    wr::<u16>(g_910, 1);

    let run = |f: F| -> ([u64; 22], u64, u64) {
        let tbl_580e0: [u64; 2] = [0x1122_3344, 0x5566_7788];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x70_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 6);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 2);
        (
            rd::<[u64; 22]>(wp + 0x200),
            rd::<u64>(wp + 0x2b0),
            rd::<u64>(wp + 0x2b8),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_63c, s_63c);
    wr::<u16>(g_910, s_910);

    crate::diff_eq!(o, n, "141184850 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141184a70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_50e0(
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
        for i in 4..10usize {
            wr::<u64>(out + i * 8, 0x4000 + i as u64);
        }
        out
    }
    unsafe extern "win64" fn mock_4aa0(rcx_buf: usize, rdx_63c: u64) {
        let mut arr = rd::<[u64; 10]>(rcx_buf);
        let world = arr[2] as usize;
        arr[2] = 0;
        wr::<[u64; 10]>(world + 0x200, arr);
        wr::<u64>(world + 0x250, rdx_63c);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4118_50e0, mock_50e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41fe_4aa0, mock_4aa0 as *const ())?;

    let g_63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let s_63c = rd::<u16>(g_63c);
    wr::<u16>(g_63c, 1);

    let run = |f: F| -> ([u64; 10], u64) {
        let tbl_580e0: [u64; 2] = [0, 0xcafe_babe_5678];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 3 * 8, 0x80_0000);
        wr::<u32>(wp + 0x10088 + 3 * 4, 4);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 3);
        (rd::<[u64; 10]>(wp + 0x200), rd::<u64>(wp + 0x250))
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_63c, s_63c);

    crate::diff_eq!(o, n, "141184a70 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1411f4330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    // Build a 4-node RB-tree (head + root(20) + left(10) + right(30)).
    let mut nodes = [[0u8; 0x20]; 4];
    let head = nodes[0].as_mut_ptr() as usize;
    let root = nodes[1].as_mut_ptr() as usize;
    let left = nodes[2].as_mut_ptr() as usize;
    let right = nodes[3].as_mut_ptr() as usize;

    // head (is_nil = 1)
    wr::<usize>(head, left);
    wr::<usize>(head + 0x08, root);
    wr::<usize>(head + 0x10, right);
    wr::<u8>(head + 0x19, 1);

    // root (key = 20)
    wr::<usize>(root, left);
    wr::<usize>(root + 0x08, head);
    wr::<usize>(root + 0x10, right);
    wr::<u8>(root + 0x19, 0);
    wr::<u16>(root + 0x1a, 20);

    // left (key = 10)
    wr::<usize>(left, head);
    wr::<usize>(left + 0x08, root);
    wr::<usize>(left + 0x10, head);
    wr::<u8>(left + 0x19, 0);
    wr::<u16>(left + 0x1a, 10);

    // right (key = 30)
    wr::<usize>(right, head);
    wr::<usize>(right + 0x08, root);
    wr::<usize>(right + 0x10, head);
    wr::<u8>(right + 0x19, 0);
    wr::<u16>(right + 0x1a, 30);

    let mut obj = [0u8; 0x30];
    let obj_ptr = obj.as_mut_ptr() as usize;
    wr::<usize>(obj_ptr + 0x28, head);

    let queries: [u16; 7] = [5, 10, 15, 20, 25, 30, 35];
    for &q in &queries {
        let ro = f_o(obj_ptr, q);
        let rn = f_n(obj_ptr, q);
        crate::diff_eq!(ro, rn, "1411f4330 key={}", q);
    }
    Ok(queries.len() as u32)
}

pub unsafe fn diff_rend_1412f9000(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, isize, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut rng = crate::difftest::Rng::new(0x1412_f900_0123_4567);
    let mut big_quicksort = [0u32; 64];
    for i in 0..64 {
        big_quicksort[i] = (rng.next_u32() % 25) * 7;
    }
    let mut big_heapsort = [0u32; 48];
    for i in 0..48 {
        big_heapsort[i] = rng.next_u32();
    }
    let small_insert: [u32; 12] = [15, 3, 99, 42, 42, 0, 1000, 7, 15, 88, 1, 50];

    let run = |f: F| -> ([u32; 12], [u32; 48], [u32; 64]) {
        let mut a1 = small_insert;
        let mut f1 = a1.as_mut_ptr() as usize;
        let mut l1 = unsafe { a1.as_mut_ptr().add(a1.len()) } as usize;
        f(
            (&mut f1 as *mut usize) as usize,
            (&mut l1 as *mut usize) as usize,
            16,
            0,
        );

        let mut a2 = big_heapsort;
        let mut f2 = a2.as_mut_ptr() as usize;
        let mut l2 = unsafe { a2.as_mut_ptr().add(a2.len()) } as usize;
        f(
            (&mut f2 as *mut usize) as usize,
            (&mut l2 as *mut usize) as usize,
            0,
            0,
        );

        let mut a3 = big_quicksort;
        let mut f3 = a3.as_mut_ptr() as usize;
        let mut l3 = unsafe { a3.as_mut_ptr().add(a3.len()) } as usize;
        f(
            (&mut f3 as *mut usize) as usize,
            (&mut l3 as *mut usize) as usize,
            64,
            0,
        );

        (a1, a2, a3)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1412f9000 sort");
    Ok(3)
}

pub unsafe fn diff_unknown_1415f1fd0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: [u64; 12] = [0; 12];
    static mut CMD_OBJ: [u8; 16] = [0; 16];
    static mut RET_1590: [u64; 2] = [0, 0x7788_99aa];

    unsafe extern "win64" fn mock_qpc(ptr: usize) -> i32 {
        wr::<u64>(ptr, 0x1234_5678_9abc);
        unsafe { TRACE[0] = 1 };
        1
    }
    unsafe extern "win64" fn mock_c570(w: u16) {
        unsafe { TRACE[1] = w as u64 };
    }
    unsafe extern "win64" fn mock_e330() {
        unsafe { TRACE[2] += 1 };
    }
    unsafe extern "win64" fn mock_9270() {
        unsafe { TRACE[3] += 1 };
    }
    unsafe extern "win64" fn mock_ac20(node: usize) {
        unsafe {
            TRACE[4] += 1;
            TRACE[5] ^= rd::<u64>(node);
        }
    }
    unsafe extern "win64" fn mock_1590(buf: usize, flag: u8) -> usize {
        wr::<u64>(buf, 0xaaaa);
        unsafe { TRACE[6] = flag as u64 };
        core::ptr::addr_of_mut!(RET_1590) as usize
    }
    unsafe extern "win64" fn mock_1e50(
        out_ptr: usize,
        out_id: usize,
        sz: u64,
        align: u64,
        _name: usize,
        z0: u64,
        one: u64,
    ) {
        wr::<usize>(out_ptr, core::ptr::addr_of_mut!(CMD_OBJ) as usize);
        wr::<u64>(out_id, 0x55);
        unsafe {
            TRACE[7] = sz;
            // The original writes args 6/7 with 32-bit `mov dword [rsp+0x28/0x30]`
            // stores, so the upper halves of their stack slots carry stale guest
            // stack residue. A Win64 callee reads 32-bit stack args by their low
            // dword only; mask before comparing against the Rust port's clean u64s.
            TRACE[8] = align | ((z0 & 0xFFFF_FFFF) << 16) | ((one & 0xFFFF_FFFF) << 32);
        }
    }
    unsafe extern "win64" fn mock_1720(p1: usize, id_ptr: usize, one: u64, r9_val: u64) {
        unsafe {
            TRACE[9] = p1 as u64;
            TRACE[10] = rd::<u64>(id_ptr) ^ (one << 16);
            TRACE[11] = r9_val;
        }
    }

    let _m_qpc = crate::difftest::IatMock::new(0x1_4264_56f8, mock_qpc as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4160_c570, mock_c570 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_415e_e330, mock_e330 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4159_9270, mock_9270 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_415e_ac20, mock_ac20 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_415f_1590, mock_1590 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_415b_1e50, mock_1e50 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_415b_1720, mock_1720 as *const ())?;

    let g_b28 = exe::ptr::<u32>(0x1_4393_1b28) as usize;
    let g_ae0 = exe::ptr::<u8>(0x1_4381_5ae0) as usize;
    let g_f68 = exe::ptr::<usize>(0x1_4393_0f68) as usize;
    let g_2b4 = exe::ptr::<u16>(0x1_4381_52b4) as usize;
    let g_b60 = exe::ptr::<usize>(0x1_4393_1b60) as usize;

    let s_b28 = rd::<u32>(g_b28);
    let s_ae0 = rd::<u8>(g_ae0);
    let s_f68 = rd::<usize>(g_f68);
    let s_2b4 = rd::<u16>(g_2b4);
    let s_b60 = rd::<usize>(g_b60);

    let run = |f: F| -> (usize, [u64; 12], [u8; 16], u64) {
        unsafe {
            TRACE = [0; 12];
            CMD_OBJ = [0; 16];
        }
        let mut perf = [0u8; 0x60];
        let pp = perf.as_mut_ptr() as usize;
        wr::<u32>(pp + 0x08, 1);

        let mut node2 = [0u8; 0x30];
        let n2p = node2.as_mut_ptr() as usize;
        wr::<u64>(n2p, 0x2222);
        wr::<usize>(n2p + 0x28, 0);

        let mut node1 = [0u8; 0x30];
        let n1p = node1.as_mut_ptr() as usize;
        wr::<u64>(n1p, 0x1111);
        wr::<usize>(n1p + 0x28, n2p);

        wr::<u32>(g_b28, 2);
        wr::<u8>(g_ae0, 0);
        wr::<usize>(g_f68, pp);
        wr::<u16>(g_2b4, 0x4242);
        wr::<usize>(g_b60, n1p);

        let ret = f(0xdead_0001);
        (ret, unsafe { TRACE }, unsafe { CMD_OBJ }, rd::<u64>(pp + 0x48))
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<u32>(g_b28, s_b28);
    wr::<u8>(g_ae0, s_ae0);
    wr::<usize>(g_f68, s_f68);
    wr::<u16>(g_2b4, s_2b4);
    wr::<usize>(g_b60, s_b60);

    crate::diff_eq!(o, n, "1415f1fd0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14183e880(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_worker_id() -> u32 {
        1
    }
    unsafe extern "win64" fn mock_58f0() {
        unsafe { TRACE[0] += 1 };
    }
    unsafe extern "win64" fn mock_5db0(rcx_val: u64, edx_val: u32) {
        unsafe {
            TRACE[1] = rcx_val;
            TRACE[2] = edx_val as u64;
        }
    }
    unsafe extern "win64" fn mock_2290(rcx_val: usize) {
        unsafe { TRACE[3] = rcx_val as u64 };
    }
    unsafe extern "win64" fn mock_2da0(rcx_val: usize) {
        unsafe { TRACE[4] = rcx_val as u64 };
    }
    unsafe extern "win64" fn mock_30f0() {
        unsafe { TRACE[5] += 1 };
    }
    unsafe extern "win64" fn mock_yield() {
        unsafe { TRACE[6] += 1 };
    }
    unsafe extern "win64" fn mock_a430(rcx_val: usize) {
        unsafe { TRACE[7] = rcx_val as u64 };
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, mock_worker_id as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4247_58f0, mock_58f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4247_5db0, mock_5db0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4242_2290, mock_2290 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4242_2da0, mock_2da0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401e_30f0, mock_30f0 as *const ())?;
    let _m7 = crate::difftest::IatMock::new(0x1_4264_5868, mock_yield as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4208_a430, mock_a430 as *const ())?;

    let tbl_slot = (exe::ptr::<usize>(0x1_43c4_aa00) as usize).wrapping_add(8);
    let g_698 = exe::ptr::<usize>(0x1_43a3_4698) as usize;
    let s_slot = rd::<usize>(tbl_slot);
    let s_698 = rd::<usize>(g_698);

    let run = |f: F| -> ([u64; 8], [u64; 8]) {
        let mut sub_48 = vec![0u8; 0x300];
        let s48p = sub_48.as_mut_ptr() as usize;
        wr::<u64>(s48p + 0x2d8 + 8, 0xabcd_ef01);

        let mut ctx = [0u8; 0x90];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp + 0x48, s48p);
        wr::<usize>(cp + 0x78, 0x5555_6666);

        // Case 1: state == 1
        unsafe { TRACE = [0; 8] };
        let mut state = [1i32];
        let state_ptr = state.as_mut_ptr() as usize;
        wr::<usize>(tbl_slot, state_ptr);
        f(cp);
        let t1 = unsafe { TRACE };

        // Case 2: state == 3
        unsafe { TRACE = [0; 8] };
        wr::<i32>(state_ptr, 3);
        let mut streamer_holder = [0u8; 0x30];
        let shp = streamer_holder.as_mut_ptr() as usize;
        wr::<usize>(shp + 0x18, 0x9000_0000);
        wr::<usize>(g_698, shp);
        f(cp);
        let t2 = unsafe { TRACE };

        (t1, t2)
    };

    let o = run(f_o);
    let n = run(f_n);

    wr::<usize>(tbl_slot, s_slot);
    wr::<usize>(g_698, s_698);

    crate::diff_eq!(o, n, "14183e880 out");
    Ok(2)
}

// REGISTRY:
// Replacement::verified_throws(0x1_4103_a0c0, "unknown_14103a0c0", rend::cauldron_hot_h::unknown_14103a0c0 as *const (), Throws::Discard, rend::cauldron_hot_h::diff_unknown_14103a0c0),
// Replacement::verified(0x1_4105_fff0, "unknown_14105fff0", rend::cauldron_hot_h::unknown_14105fff0 as *const (), rend::cauldron_hot_h::diff_unknown_14105fff0),
// Replacement::verified_throws(0x1_410c_60e0, "r_1410c60e0", rend::cauldron_hot_h::r_1410c60e0 as *const (), Throws::Discard, rend::cauldron_hot_h::diff_r_1410c60e0),
// Replacement::verified(0x1_4117_1890, "unknown_141171890", rend::cauldron_hot_h::unknown_141171890 as *const (), rend::cauldron_hot_h::diff_unknown_141171890),
// Replacement::verified(0x1_4118_4850, "unknown_141184850", rend::cauldron_hot_h::unknown_141184850 as *const (), rend::cauldron_hot_h::diff_unknown_141184850),
// Replacement::verified(0x1_4118_4a70, "unknown_141184a70", rend::cauldron_hot_h::unknown_141184a70 as *const (), rend::cauldron_hot_h::diff_unknown_141184a70),
// Replacement::verified(0x1_411f_4330, "unknown_1411f4330", rend::cauldron_hot_h::unknown_1411f4330 as *const (), rend::cauldron_hot_h::diff_unknown_1411f4330),
// Replacement::verified(0x1_412f_9000, "rend_1412f9000", rend::cauldron_hot_h::rend_1412f9000 as *const (), rend::cauldron_hot_h::diff_rend_1412f9000),
// Replacement::verified(0x1_415f_1fd0, "unknown_1415f1fd0", rend::cauldron_hot_h::unknown_1415f1fd0 as *const (), rend::cauldron_hot_h::diff_unknown_1415f1fd0),
// Replacement::verified_throws(0x1_4183_e880, "unknown_14183e880", rend::cauldron_hot_h::unknown_14183e880 as *const (), Throws::Discard, rend::cauldron_hot_h::diff_unknown_14183e880),
