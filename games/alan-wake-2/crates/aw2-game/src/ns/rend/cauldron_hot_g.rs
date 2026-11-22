//! Decompiled hot functions shard G (`0x1401e8210..0x1410b0200`).

use crate::exe::{self, rd, wr};

/// `unknown_1401e8210`
/// Original: 0x1401e8210 (122 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1401e8210(param_1: usize, param_2: usize) -> usize {
    let entries = rd::<usize>(param_1 + 0x78);
    let cap = rd::<usize>(param_1 + 0x48);
    let entry_ptr = if rd::<u64>(param_1 + 0x40) == rd::<u64>(param_1 + 0x38) {
        entries.wrapping_add(cap.wrapping_mul(0x28))
    } else {
        let mut pair = [0u64; 2];
        let _ = exe::call_va!(
            0x1_401e_8680,
            (param_1, pair.as_mut_ptr() as usize, param_2) -> usize
        );
        if pair[0] == u64::MAX {
            return 0;
        }
        entries.wrapping_add((pair[0] as usize).wrapping_mul(0x28))
    };

    let end_ptr = rd::<usize>(param_1 + 0x78)
        .wrapping_add((rd::<usize>(param_1 + 0x48)).wrapping_mul(0x28));
    if entry_ptr == end_ptr {
        return 0;
    }

    let val_ptr = entry_ptr.wrapping_add(0x10);
    if rd::<u16>(val_ptr) != 6 {
        val_ptr
    } else {
        0
    }
}

/// `unknown_1401e9e70`
/// Original: 0x1401e9e70 (296 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1401e9e70(param_1: usize) {
    if rd::<u8>(param_1 + 0x368) == 0 {
        let depth = rd::<u32>(param_1 + 0x08) as usize;
        let rdi = rd::<usize>(param_1).wrapping_add(depth.wrapping_mul(0x30));

        let p338 = rd::<usize>(param_1 + 0x338);
        let vtbl_338 = rd::<usize>(p338);
        let mut sentinel: u32 = 0;
        exe::call_va!(
            rd::<usize>(vtbl_338 + 0x40),
            (p338, (&mut sentinel as *mut u32) as usize)
        );

        let p320 = rd::<usize>(param_1 + 0x320);
        let inner = rd::<usize>(p320 + 0x08);
        let vtbl_inner = rd::<usize>(inner);
        let pos = exe::call_va!(rd::<usize>(vtbl_inner + 0x10), (inner) -> u64);

        let diff = pos.wrapping_sub(rd::<u64>(rdi.wrapping_sub(0x18)));
        let expected = rd::<u64>(rdi.wrapping_sub(0x10));

        let mut exc_buf = [0u64; 2];
        if diff > expected {
            let prev_ctx = exe::call_va!(0x1_401e_7b40, (param_1) -> usize);
            let exc_ptr = exe::call_va!(
                0x1_401e_7590,
                (
                    exc_buf.as_mut_ptr() as usize,
                    rd::<u64>(rdi.wrapping_sub(0x28)),
                    rd::<u64>(prev_ctx + 0x08),
                    rd::<u64>(param_1 + 0x330)
                ) -> usize
            );
            let arr = rd::<usize>(param_1);
            let cur_depth = rd::<u32>(param_1 + 0x08) as usize;
            let prev = rd::<u32>(arr.wrapping_add(cur_depth.wrapping_mul(0x30)).wrapping_sub(0x08));
            wr::<u32>(param_1 + 0x08, (cur_depth as u32).wrapping_sub(1));
            wr::<u32>(param_1 + 0x328, prev);
            exe::call_va!(0x1_401e_af00, (exc_ptr));
            return;
        }
        if diff < expected {
            let prev_ctx = exe::call_va!(0x1_401e_7b40, (param_1) -> usize);
            let exc_ptr = exe::call_va!(
                0x1_401e_7610,
                (
                    exc_buf.as_mut_ptr() as usize,
                    rd::<u64>(rdi.wrapping_sub(0x28)),
                    rd::<u64>(prev_ctx + 0x08),
                    rd::<u64>(param_1 + 0x330)
                ) -> usize
            );
            let arr = rd::<usize>(param_1);
            let cur_depth = rd::<u32>(param_1 + 0x08) as usize;
            let prev = rd::<u32>(arr.wrapping_add(cur_depth.wrapping_mul(0x30)).wrapping_sub(0x08));
            wr::<u32>(param_1 + 0x08, (cur_depth as u32).wrapping_sub(1));
            wr::<u32>(param_1 + 0x328, prev);
            exe::call_va!(0x1_401e_af70, (exc_ptr));
            return;
        }
        if sentinel != 0xdead_beef && sentinel != 0xd34d_b33f {
            let fn_name = rd::<u64>(param_1 + 0x330);
            let prev_ctx = exe::call_va!(0x1_401e_7b40, (param_1) -> usize);
            let s_exiting = exe::ptr::<u8>(0x1_4307_aa80) as usize;
            let exc_ptr = exe::call_va!(
                0x1_401e_7430,
                (
                    exc_buf.as_mut_ptr() as usize,
                    rd::<u64>(rdi.wrapping_sub(0x28)),
                    rd::<u64>(prev_ctx + 0x08),
                    s_exiting,
                    fn_name
                ) -> usize
            );
            let arr = rd::<usize>(param_1);
            let cur_depth = rd::<u32>(param_1 + 0x08) as usize;
            let prev = rd::<u32>(arr.wrapping_add(cur_depth.wrapping_mul(0x30)).wrapping_sub(0x08));
            wr::<u32>(param_1 + 0x08, (cur_depth as u32).wrapping_sub(1));
            wr::<u32>(param_1 + 0x328, prev);
            exe::call_va!(0x1_401e_afe0, (exc_ptr));
            return;
        }
    }

    let arr = rd::<usize>(param_1);
    let cur_depth = rd::<u32>(param_1 + 0x08) as usize;
    let prev = rd::<u32>(arr.wrapping_add(cur_depth.wrapping_mul(0x30)).wrapping_sub(0x08));
    wr::<u32>(param_1 + 0x328, prev);
    wr::<u32>(param_1 + 0x08, (cur_depth as u32).wrapping_sub(1));
}

/// `unknown_1401e9fa0`
/// Original: 0x1401e9fa0 (720 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_1401e9fa0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u32,
    param_5: i32,
    param_6: i32,
) {
    exe::call_va!(
        0x1_4243_b0b0,
        (
            param_1,
            param_2,
            param_3,
            param_4,
            param_5 as u32,
            param_6 as u32
        )
    );

    let mut exc_buf = [0u64; 4];
    let p338 = rd::<usize>(param_1 + 0x338);
    let vtbl_338 = rd::<usize>(p338);

    if rd::<u8>(param_1 + 0x368) != 0 {
        let mut ver: i32 = 0;
        exe::call_va!(
            rd::<usize>(vtbl_338 + 0x48),
            (p338, (&mut ver as *mut i32) as usize)
        );
        if ver < param_6 {
            let exc_ptr = exe::call_va!(
                0x1_401e_7180,
                (
                    exc_buf.as_mut_ptr() as usize,
                    param_3,
                    ver as u32,
                    param_5 as u32,
                    param_6 as u32,
                    rd::<u64>(param_1 + 0x330)
                ) -> usize
            );
            exe::call_va!(0x1_401e_8800, (exc_ptr));
            return;
        }
        if ver > param_5 {
            let exc_ptr = exe::call_va!(
                0x1_401e_7380,
                (
                    exc_buf.as_mut_ptr() as usize,
                    param_3,
                    ver as u32,
                    param_5 as u32,
                    rd::<u64>(param_1 + 0x330)
                ) -> usize
            );
            exe::call_va!(0x1_401e_a620, (exc_ptr));
            return;
        }
        let arr = rd::<usize>(param_1);
        let depth = rd::<u32>(param_1 + 0x08) as usize;
        let prev = rd::<u32>(param_1 + 0x328);
        wr::<u32>(arr.wrapping_add(depth.wrapping_mul(0x30)).wrapping_sub(0x08), prev);
        wr::<i32>(param_1 + 0x328, ver);
        return;
    }

    let mut magic: u32 = 0;
    exe::call_va!(
        rd::<usize>(vtbl_338 + 0x40),
        (p338, (&mut magic as *mut u32) as usize)
    );
    if magic != 0xdead_beef && magic != 0xd34d_b33f {
        let fn_name = rd::<u64>(param_1 + 0x330);
        let prev_ctx = exe::call_va!(0x1_401e_7b40, (param_1) -> usize);
        let s_entering = exe::ptr::<u8>(0x1_4307_aa88) as usize;
        let exc_ptr = exe::call_va!(
            0x1_401e_7430,
            (
                exc_buf.as_mut_ptr() as usize,
                param_3,
                rd::<u64>(prev_ctx + 0x08),
                s_entering,
                fn_name
            ) -> usize
        );
        exe::call_va!(0x1_401e_afe0, (exc_ptr));
        return;
    }

    let mut flag_50: i32 = 0;
    if magic == 0xd34d_b33f {
        let s = rd::<usize>(param_1 + 0x338);
        exe::call_va!(
            rd::<usize>(rd::<usize>(s) + 0x48),
            (s, (&mut flag_50 as *mut i32) as usize)
        );
    }

    let mut size_val: u32 = 0;
    {
        let s = rd::<usize>(param_1 + 0x338);
        exe::call_va!(
            rd::<usize>(rd::<usize>(s) + 0x40),
            (s, (&mut size_val as *mut u32) as usize)
        );
    }
    let depth = rd::<u32>(param_1 + 0x08) as usize;
    let arr = rd::<usize>(param_1);
    wr::<u64>(
        arr.wrapping_add(depth.wrapping_mul(0x30)).wrapping_sub(0x10),
        size_val as u64,
    );

    let mut type_crc: u32 = 0;
    {
        let s = rd::<usize>(param_1 + 0x338);
        exe::call_va!(
            rd::<usize>(rd::<usize>(s) + 0x40),
            (s, (&mut type_crc as *mut u32) as usize)
        );
    }

    let mut ver: i32 = 0;
    let s_after = rd::<usize>(param_1 + 0x338);
    exe::call_va!(
        rd::<usize>(rd::<usize>(s_after) + 0x48),
        (s_after, (&mut ver as *mut i32) as usize)
    );

    if ver < param_6 {
        let exc_ptr = exe::call_va!(
            0x1_401e_7180,
            (
                exc_buf.as_mut_ptr() as usize,
                param_3,
                ver as u32,
                param_5 as u32,
                param_6 as u32,
                rd::<u64>(param_1 + 0x330)
            ) -> usize
        );
        exe::call_va!(0x1_401e_8800, (exc_ptr));
        return;
    }
    if ver > param_5 {
        let exc_ptr = exe::call_va!(
            0x1_401e_7380,
            (
                exc_buf.as_mut_ptr() as usize,
                param_3,
                ver as u32,
                param_5 as u32,
                rd::<u64>(param_1 + 0x330)
            ) -> usize
        );
        exe::call_va!(0x1_401e_a620, (exc_ptr));
        return;
    }

    let ok = exe::call_va!(0x1_4243_b000, (s_after, type_crc, param_4) -> u8);
    if ok != 0 {
        let arr2 = rd::<usize>(param_1);
        let depth2 = rd::<u32>(param_1 + 0x08) as usize;
        let prev = rd::<u32>(param_1 + 0x328);
        wr::<u32>(arr2.wrapping_add(depth2.wrapping_mul(0x30)).wrapping_sub(0x08), prev);
        wr::<i32>(param_1 + 0x328, ver);
        if flag_50 == 1 {
            let mut dummy: i32 = 0;
            let s_end = rd::<usize>(param_1 + 0x338);
            exe::call_va!(
                rd::<usize>(rd::<usize>(s_end) + 0x48),
                (s_end, (&mut dummy as *mut i32) as usize)
            );
        }
        return;
    }

    let type_info = exe::call_va!(0x1_424b_14c0, (type_crc) -> usize);
    let fn_name = rd::<u64>(param_1 + 0x330);
    if type_info == 0 {
        let prev_ctx = exe::call_va!(0x1_401e_7b40, (param_1) -> usize);
        let exc_ptr = exe::call_va!(
            0x1_401e_7510,
            (
                exc_buf.as_mut_ptr() as usize,
                param_3,
                type_crc,
                rd::<u64>(prev_ctx + 0x08),
                fn_name
            ) -> usize
        );
        exe::call_va!(0x1_401e_b050, (exc_ptr));
    } else {
        let prev_ctx = exe::call_va!(0x1_401e_7b40, (param_1) -> usize);
        let prev_name = rd::<u64>(prev_ctx + 0x08);
        let ti_vtbl = rd::<usize>(type_info);
        let ti_name = exe::call_va!(rd::<usize>(ti_vtbl), (type_info) -> u64);
        let exc_ptr = exe::call_va!(
            0x1_401e_74b0,
            (
                exc_buf.as_mut_ptr() as usize,
                param_3,
                ti_name,
                prev_name,
                fn_name
            ) -> usize
        );
        exe::call_va!(0x1_401e_b050, (exc_ptr));
    }
}

/// `unknown_1403ee490`
/// Original: 0x1403ee490 (323 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1403ee490(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if count != 0 {
        let v0 = rd::<usize>(world);
        let id1 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_c1a4) as usize) as usize;
        let want_id = rd::<u16>(exe::ptr::<u16>(0x1_43c9_bd60) as usize) as u64;
        let lut = rd::<usize>(v0 + 0x20);
        let idx1 = rd::<u16>(lut.wrapping_add(id1 * 2)) as usize;
        let arr_08 = rd::<usize>(world + 0x08);
        let len1 = rd::<u32>(arr_08.wrapping_add(idx1 * 16).wrapping_add(0x08)) as u64;
        let ptr1 = rd::<u64>(arr_08.wrapping_add(idx1 * 16));

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
            exe::call_va!(
                0x1_4195_a5c0,
                (arg1.as_mut_ptr() as usize, arg2.as_mut_ptr() as usize)
            );
        }
    }
}

/// `unknown_1404b76d0`
/// Original: 0x1404b76d0 (421 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1404b76d0(
    _param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let count = rd::<u64>(param_2 + 0x18);
    for idx in 0..count {
        let mut pkt = [0u64; 17];
        pkt[..15].copy_from_slice(&rd::<[u64; 15]>(param_2 + 0x20));
        pkt[15] = rd::<u64>(param_2 + 0x08);
        pkt[16] = idx;

        let mut arg3: [u64; 3] = [
            rd::<u64>(param_3 + 0x18),
            rd::<u64>(param_3 + 0x20),
            rd::<u64>(param_3 + 0x28),
        ];
        let mut arg4: [u64; 3] = [
            rd::<u64>(param_3),
            rd::<u64>(param_3 + 0x08),
            rd::<u64>(param_3 + 0x10),
        ];
        let arg2 = rd::<u64>(param_3 + 0x30);

        exe::call_va!(
            0x1_404b_5940,
            (
                pkt.as_mut_ptr() as usize,
                arg2,
                arg3.as_mut_ptr() as usize,
                arg4.as_mut_ptr() as usize
            )
        );
    }
}

/// `r_14059b710`
/// Original: 0x14059b710 (587 bytes, Throws::No)
pub unsafe extern "win64" fn r_14059b710(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let world = rd::<usize>(param_1);
        let id_118 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d118) as usize) as usize;
        let id_ed4 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ced4) as usize) as usize;
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let world_plus_40 = world.wrapping_add(0x40);
        let id_110 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d110) as usize) as usize;
        let val_118 = rd::<u64>(tbl_580e0.wrapping_add(id_118 * 8));
        let val_110 = rd::<u64>(tbl_580e0.wrapping_add(id_110 * 8));
        let id_0d4 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d0d4) as usize) as usize;
        let val_0d4 = rd::<u64>(tbl_580e0.wrapping_add(id_0d4 * 8));

        let v0 = rd::<usize>(world);
        let lut = rd::<usize>(v0 + 0x20);
        let idx_ed4 = rd::<u16>(lut.wrapping_add(id_ed4 * 2)) as usize;
        let entry_ed4 = rd::<usize>(world + 0x08).wrapping_add(idx_ed4 * 16);
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let len_ed4 = rd::<u32>(entry_ed4 + 0x08) as u64;
        let ptr_ed4 = rd::<u64>(entry_ed4);

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world_plus_40.wrapping_add(arch * 8).wrapping_add(0x48));
            let ent_count = if block != 0 {
                rd::<u32>(world_plus_40.wrapping_add(arch * 4).wrapping_add(0x10048)) as usize
            } else {
                0
            };
            if ent_count != 0 {
                let mut raw_offs = [0u64; 5];
                let _ = exe::call_va!(
                    0x1_4059_8a90,
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
                for idx in 0..ent_count {
                    let mut pkt: [u64; 7] = [
                        raw_offs[0],
                        raw_offs[1],
                        raw_offs[2],
                        raw_offs[3],
                        raw_offs[4],
                        block_u64,
                        idx as u64,
                    ];
                    let mut arg5: [u64; 3] = [world as u64, len_ed4, ptr_ed4];
                    exe::call_va!(
                        0x1_41ac_dfd0,
                        (
                            pkt.as_mut_ptr() as usize,
                            val_110,
                            val_118,
                            val_0d4,
                            arg5.as_mut_ptr() as usize
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `r_14059b990`
/// Original: 0x14059b990 (770 bytes, Throws::No)
pub unsafe extern "win64" fn r_14059b990(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let mut ctx9 = [0u64; 9];
        let _ = exe::call_va!(
            0x1_4059_5e10,
            (ctx9.as_mut_ptr() as usize, param_1) -> usize
        );
        let world_plus_40 = rd::<usize>(param_1).wrapping_add(0x40);
        let end_ptr = cur_ptr.wrapping_add(count * 2);

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world_plus_40.wrapping_add(arch * 8).wrapping_add(0x48));
            let ent_count = if block != 0 {
                rd::<u32>(world_plus_40.wrapping_add(arch * 4).wrapping_add(0x10048)) as usize
            } else {
                0
            };
            if ent_count != 0 {
                let mut raw_offs = [0u64; 12];
                let _ = exe::call_va!(
                    0x1_4059_87f0,
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
                for idx in 0..ent_count {
                    let mut pkt = [0u64; 14];
                    pkt[..12].copy_from_slice(&raw_offs);
                    pkt[12] = block_u64;
                    pkt[13] = idx as u64;
                    let mut arg2: [u64; 3] = [ctx9[6], ctx9[7], ctx9[8]];
                    let mut arg3: [u64; 3] = [ctx9[3], ctx9[4], ctx9[5]];
                    exe::call_va!(
                        0x1_41ac_c4e0,
                        (
                            pkt.as_mut_ptr() as usize,
                            arg2.as_mut_ptr() as usize,
                            arg3.as_mut_ptr() as usize,
                            ctx9[2],
                            ctx9[1],
                            ctx9[0]
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `r_14059c900`
/// Original: 0x14059c900 (593 bytes, Throws::No)
pub unsafe extern "win64" fn r_14059c900(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let mut ctx12 = [0u64; 12];
        let _ = exe::call_va!(
            0x1_4059_7660,
            (ctx12.as_mut_ptr() as usize, param_1) -> usize
        );
        let world_plus_40 = rd::<usize>(param_1).wrapping_add(0x40);
        let end_ptr = cur_ptr.wrapping_add(count * 2);

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world_plus_40.wrapping_add(arch * 8).wrapping_add(0x48));
            let ent_count = if block != 0 {
                rd::<u32>(world_plus_40.wrapping_add(arch * 4).wrapping_add(0x10048)) as usize
            } else {
                0
            };
            if ent_count != 0 {
                let mut raw_offs = [0u64; 6];
                let _ = exe::call_va!(
                    0x1_4059_8000,
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
                for idx in 0..ent_count {
                    let mut pkt = [0u64; 8];
                    pkt[..6].copy_from_slice(&raw_offs);
                    pkt[6] = block_u64;
                    pkt[7] = idx as u64;
                    let mut arg2: [u64; 3] = [ctx12[9], ctx12[10], ctx12[11]];
                    let mut arg3: [u64; 3] = [ctx12[6], ctx12[7], ctx12[8]];
                    let mut arg4: [u64; 3] = [ctx12[3], ctx12[4], ctx12[5]];
                    exe::call_va!(
                        0x1_41ac_d4d0,
                        (
                            pkt.as_mut_ptr() as usize,
                            arg2.as_mut_ptr() as usize,
                            arg3.as_mut_ptr() as usize,
                            arg4.as_mut_ptr() as usize,
                            ctx12[2],
                            ctx12[1],
                            ctx12[0]
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_141038eb0`
/// Original: 0x141038eb0 (425 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141038eb0(param_1: usize, param_2: usize) {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let ring = rd::<usize>(mgr + 0x168);
    exe::call_iat!(0x1_4264_55a8, (ring + 0x18));

    let mut rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
    let mut wr_ptr = crate::simd_ops::rd_acq_usize(ring + 0x28);
    let mut next_wr = wr_ptr.wrapping_add(0x50);
    let ring_end = rd::<usize>(ring + 0x10);
    if next_wr > ring_end {
        while wr_ptr <= rd_ptr
            && rd_ptr < next_wr
            && crate::simd_ops::rd_acq_u32(ring + 0x30) != crate::simd_ops::rd_acq_u32(ring + 0x34)
        {
            core::hint::spin_loop();
            rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        }
        if wr_ptr <= ring_end.wrapping_sub(8) {
            crate::simd_ops::wr_rel_u64(wr_ptr, 0x00ff_ffff_ffff);
        }
        wr_ptr = rd::<usize>(ring + 0x08);
        next_wr = wr_ptr.wrapping_add(0x50);
    }
    while wr_ptr <= rd_ptr
        && rd_ptr < next_wr
        && crate::simd_ops::rd_acq_u32(ring + 0x30) != crate::simd_ops::rd_acq_u32(ring + 0x34)
    {
        core::hint::spin_loop();
        rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
    }

    crate::simd_ops::wr_rel_u64(wr_ptr, 0x69fa_fafa_faff);
    crate::simd_ops::atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
    exe::call_iat!(0x1_4264_5598, (ring + 0x18));

    wr::<u32>(wr_ptr + 0x08, 0x252);
    wr::<u32>(wr_ptr + 0x0c, rd::<u32>(param_1));
    wr::<u64>(wr_ptr + 0x1c, rd::<u64>(param_2));
    wr::<u32>(wr_ptr + 0x24, rd::<u32>(param_2 + 0x08));
    wr::<u64>(wr_ptr + 0x28, rd::<u64>(param_2 + 0x0c));
    wr::<u32>(wr_ptr + 0x30, rd::<u32>(param_2 + 0x14));
    wr::<u64>(wr_ptr + 0x34, rd::<u64>(param_2 + 0x18));
    wr::<u32>(wr_ptr + 0x3c, rd::<u32>(param_2 + 0x20));
    wr::<u64>(wr_ptr + 0x40, rd::<u64>(param_2 + 0x24));
    wr::<u32>(wr_ptr + 0x48, rd::<u32>(param_2 + 0x2c));

    let mgr2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let hdr = (rd::<u32>(wr_ptr + 0x08) & 0x1ff) | 0x200;
    wr::<u32>(wr_ptr + 0x08, hdr);
    let ring2 = rd::<usize>(mgr2 + 0x168);
    crate::simd_ops::wr_rel_u64(wr_ptr, 0x44);
    let _ = crate::simd_ops::atomic_xadd_u32((ring2 + 0x30) as u64, 1);
    if rd::<u8>(mgr2 + 0x198) != 0 {
        let _ = exe::call_iat!(0x1_4264_5580, (rd::<u64>(mgr2 + 0x1a0), 1u32, 0usize) -> i32);
    }
}

/// `unknown_1410b0200`
/// Original: 0x1410b0200 (321 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1410b0200(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if count != 0 {
        let v0 = rd::<usize>(world);
        let id_0f4 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_a0f4) as usize) as usize;
        let lut = rd::<usize>(v0 + 0x20);
        let idx_0f4 = rd::<u16>(lut.wrapping_add(id_0f4 * 2)) as usize;
        let entry = rd::<usize>(world + 0x08).wrapping_add(idx_0f4 * 16);
        let len_0f4 = rd::<u32>(entry + 0x08) as u64;
        let ptr_0f4 = rd::<u64>(entry);

        let mut buf = [0u64; 6];
        let _ = exe::call_va!(
            0x1_410b_0f90,
            (
                buf.as_mut_ptr() as usize,
                param_2 as u64,
                block,
                count as u64,
                world
            ) -> usize
        );
        let total = buf[3];
        let b1 = buf[1];
        let b4 = buf[4];
        let b5 = buf[5];
        for idx in 0..total {
            let mut arg1: [u64; 4] = [b4, b5, b1, idx];
            let mut arg2: [u64; 3] = [world as u64, len_0f4, ptr_0f4];
            exe::call_va!(
                0x1_41f3_aa30,
                (arg1.as_mut_ptr() as usize, arg2.as_mut_ptr() as usize)
            );
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1401e8210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_probe(_tbl: usize, out_pair: usize, key: usize) -> usize {
        let k = rd::<u64>(key);
        let found = match k {
            100 => u64::MAX,
            200 => 0,
            300 => 1,
            _ => 2,
        };
        wr::<u64>(out_pair, found);
        wr::<u64>(out_pair + 0x08, u64::MAX);
        out_pair
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8680, mock_probe as *const ())?;

    let mut entries = [0u8; 0x28 * 3];
    let ep = entries.as_mut_ptr() as usize;
    wr::<u16>(ep + 0x10, 6);
    wr::<u16>(ep + 0x28 + 0x10, 42);

    let mut tbl = [0u8; 0x80];
    let tp = tbl.as_mut_ptr() as usize;
    wr::<usize>(tp + 0x78, ep);
    wr::<usize>(tp + 0x48, 2);

    // Case 1: [rcx + 0x40] == [rcx + 0x38] -> returns 0
    wr::<u64>(tp + 0x38, 5);
    wr::<u64>(tp + 0x40, 5);
    let k0: u64 = 300;
    let o0 = f_o(tp, (&k0 as *const u64) as usize);
    let n0 = f_n(tp, (&k0 as *const u64) as usize);
    crate::diff_eq!(o0, n0, "1401e8210 empty");

    // Case 2: [rcx + 0x40] != [rcx + 0x38]
    wr::<u64>(tp + 0x40, 6);
    for k in [100u64, 200, 300, 400] {
        let kp = (&k as *const u64) as usize;
        let o = f_o(tp, kp);
        let n = f_n(tp, kp);
        crate::diff_eq!(o, n, "1401e8210 probe");
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1401e9e70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_read_u32(this: usize, out_u32: usize) {
        let v = rd::<u32>(this + 0x08);
        wr::<u32>(out_u32, v);
        wr::<u32>(this + 0x10, rd::<u32>(this + 0x10).wrapping_add(1));
    }
    unsafe extern "win64" fn mock_get_pos(this: usize) -> u64 {
        rd::<u64>(this + 0x08)
    }

    let run = |f: F, fast_flag: u8, sentinel: u32| -> (u32, u32, u32) {
        let mut vtbl_338 = [0usize; 10];
        vtbl_338[0x40 / 8] = mock_read_u32 as *const () as usize;
        let mut obj_338 = [0u8; 0x20];
        let p338 = obj_338.as_mut_ptr() as usize;
        wr::<usize>(p338, vtbl_338.as_ptr() as usize);
        wr::<u32>(p338 + 0x08, sentinel);

        let mut vtbl_inner = [0usize; 4];
        vtbl_inner[0x10 / 8] = mock_get_pos as *const () as usize;
        let inner = [vtbl_inner.as_ptr() as u64, 0x150u64];
        let p320 = [0usize, inner.as_ptr() as usize];

        let mut stack_entries = [0u8; 0x90];
        let sep = stack_entries.as_mut_ptr() as usize;
        wr::<u64>(sep + 0x60 - 0x18, 0x100);
        wr::<u64>(sep + 0x60 - 0x10, 0x50);
        wr::<u32>(sep + 0x60 - 0x08, 77);

        let mut ctx = vec![0u8; 0x380];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp, sep);
        wr::<u32>(cp + 0x08, 2);
        wr::<usize>(cp + 0x320, p320.as_ptr() as usize);
        wr::<u32>(cp + 0x328, 99);
        wr::<usize>(cp + 0x338, p338);
        wr::<u8>(cp + 0x368, fast_flag);

        f(cp);
        (rd::<u32>(cp + 0x08), rd::<u32>(cp + 0x328), rd::<u32>(p338 + 0x10))
    };

    for (flag, sent) in [(1u8, 0u32), (0u8, 0xdead_beef), (0u8, 0xd34d_b33f)] {
        let o = run(f_o, flag, sent);
        let n = run(f_n, flag, sent);
        crate::diff_eq!(o, n, "1401e9e70 out");
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1401e9fa0(orig: usize, new: usize) -> Result<u32, String> {
    // 0x1401e9fa0 uses [rbp + 0x50] and [rbp + 0x58] (7th and 8th stack argument slots)
    // as scratch locals, so we pass 2 extra dummy u64 arguments to reserve those slots.
    type F = unsafe extern "win64" fn(usize, usize, usize, u32, i32, i32, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_b0b0(
        p1: usize,
        p2: usize,
        p3: usize,
        _p4: u32,
        p5: i32,
        p6: i32,
    ) {
        let idx = rd::<u32>(p1 + 0x08) as usize;
        let next = idx + 1;
        wr::<u32>(p1 + 0x08, next as u32);
        let slot = rd::<usize>(p1).wrapping_add(next * 0x30);
        wr::<usize>(slot - 0x30, p2);
        wr::<usize>(slot - 0x28, p3);
        wr::<i32>(slot - 0x20, p5);
        wr::<i32>(slot - 0x1c, p6);
        wr::<u64>(slot - 0x18, 0x200);
        wr::<u64>(slot - 0x10, 0);
        wr::<u32>(slot - 0x08, 0xffff_ffff);
    }
    unsafe extern "win64" fn mock_b000(_s: usize, crc: u32, want: u32) -> u8 {
        (crc == want) as u8
    }
    unsafe extern "win64" fn mock_read_40(this: usize, out_u32: usize) {
        let cur = rd::<usize>(this + 0x08);
        wr::<u32>(out_u32, rd::<u32>(cur));
        wr::<usize>(this + 0x08, cur + 4);
    }
    unsafe extern "win64" fn mock_read_48(this: usize, out_i32: usize) {
        let cur = rd::<usize>(this + 0x08);
        wr::<i32>(out_i32, rd::<i32>(cur));
        wr::<usize>(this + 0x08, cur + 4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4243_b0b0, mock_b0b0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4243_b000, mock_b000 as *const ())?;

    let run = |f: F, fast_flag: u8, words: &[u32]| -> (u32, i32, u64, u32, usize) {
        let mut vtbl_338 = [0usize; 12];
        vtbl_338[0x40 / 8] = mock_read_40 as *const () as usize;
        vtbl_338[0x48 / 8] = mock_read_48 as *const () as usize;

        let stream_base = words.as_ptr() as usize;
        let obj_338 = [vtbl_338.as_ptr() as usize, stream_base];

        let mut stack_entries = [0u8; 0xc0];
        let sep = stack_entries.as_mut_ptr() as usize;

        let mut ctx = vec![0u8; 0x380];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp, sep);
        wr::<u32>(cp + 0x08, 1);
        wr::<u32>(cp + 0x328, 12);
        wr::<usize>(cp + 0x338, obj_338.as_ptr() as usize);
        wr::<u8>(cp + 0x368, fast_flag);

        f(cp, 0x1111, 0x2222, 0xabcd_1234, 5, 1, 0, 0);

        let depth = rd::<u32>(cp + 0x08);
        let ver_after = rd::<i32>(cp + 0x328);
        let end_slot = sep + (depth as usize) * 0x30;
        let sz = rd::<u64>(end_slot - 0x10);
        let saved_prev = rd::<u32>(end_slot - 0x08);
        let consumed = rd::<usize>(obj_338.as_ptr() as usize + 0x08).wrapping_sub(stream_base);
        (depth, ver_after, sz, saved_prev, consumed)
    };

    let w_fast: [u32; 2] = [3, 0];
    let w_dead: [u32; 4] = [0xdead_beef, 0x80, 0xabcd_1234, 4];
    let w_d34d: [u32; 6] = [0xd34d_b33f, 1, 0xc0, 0xabcd_1234, 2, 99];

    for (flag, words) in [(1u8, &w_fast[..]), (0u8, &w_dead[..]), (0u8, &w_d34d[..])] {
        let o = run(f_o, flag, words);
        let n = run(f_n, flag, words);
        crate::diff_eq!(o, n, "1401e9fa0 out");
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1403ee490(orig: usize, new: usize) -> Result<u32, String> {
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
    unsafe extern "win64" fn mock_a5c0(a1_ptr: usize, a2_ptr: usize) {
        let a1 = rd::<[u64; 3]>(a1_ptr);
        let mut a2 = rd::<[u64; 3]>(a2_ptr);
        let world = a2[0] as usize;
        a2[0] = 0;
        let idx = a1[2] as usize;
        let dst = world + 0x200 + idx * 48;
        wr::<[u64; 3]>(dst, a1);
        wr::<[u64; 3]>(dst + 24, a2);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4195_a5c0, mock_a5c0 as *const ())?;

    let g_1a4 = exe::ptr::<u16>(0x1_43c9_c1a4) as usize;
    let g_d60 = exe::ptr::<u16>(0x1_43c9_bd60) as usize;
    let s_1a4 = rd::<u16>(g_1a4);
    let s_d60 = rd::<u16>(g_d60);
    wr::<u16>(g_1a4, 1);
    wr::<u16>(g_d60, 7);

    let run = |f: F| -> [u64; 12] {
        let lut: [u16; 4] = [0, 1, 2, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut r8_arr = [0u8; 0x40];
        let r8p = r8_arr.as_mut_ptr() as usize;
        wr::<u64>(r8p + 0x10, 0x1111_2222);
        wr::<u32>(r8p + 0x18, 4);

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
        rd::<[u64; 12]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_1a4, s_1a4);
    wr::<u16>(g_d60, s_d60);

    crate::diff_eq!(o, n, "1403ee490 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1404b76d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_5940(pkt_ptr: usize, a2: u64, a3_ptr: usize, a4_ptr: usize) {
        let pkt = rd::<[u64; 17]>(pkt_ptr);
        let a3 = rd::<[u64; 3]>(a3_ptr);
        let a4 = rd::<[u64; 3]>(a4_ptr);
        let sink = a2 as usize;
        let idx = pkt[16] as usize;
        let dst = sink + idx * 0xc0;
        wr::<[u64; 17]>(dst, pkt);
        wr::<[u64; 3]>(dst + 0x88, a3);
        wr::<[u64; 3]>(dst + 0xa0, a4);
    }

    let _m1 = crate::difftest::Mock::new(0x1_404b_5940, mock_5940 as *const ())?;

    let run = |f: F| -> [u64; 48] {
        let mut sink = [0u64; 48];
        let mut p2 = [0u64; 19];
        p2[1] = 0x9999_0001;
        p2[3] = 2;
        for i in 0..15usize {
            p2[4 + i] = 0x100 + i as u64;
        }
        let p3: [u64; 7] = [
            0xaa01,
            0xaa02,
            0xaa03,
            0xbb01,
            0xbb02,
            0xbb03,
            sink.as_mut_ptr() as u64,
        ];
        f(0, p2.as_ptr() as usize, p3.as_ptr() as usize);
        sink
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1404b76d0 out");
    Ok(1)
}

pub unsafe fn diff_r_14059b710(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_8a90(
        out: usize,
        _cnt: u64,
        _world40: usize,
        arch: u16,
    ) -> usize {
        wr::<u64>(out, 0);
        for i in 1..5usize {
            wr::<u64>(out + i * 8, (i as u64) * 0x100 + (arch as u64));
        }
        out
    }
    unsafe extern "win64" fn mock_dfd0(
        pkt_ptr: usize,
        v110: u64,
        v118: u64,
        v0d4: u64,
        arg5_ptr: usize,
    ) {
        let pkt = rd::<[u64; 7]>(pkt_ptr);
        let mut a5 = rd::<[u64; 3]>(arg5_ptr);
        let world = a5[0] as usize;
        a5[0] = 0;
        let idx = pkt[6] as usize;
        let dst = world + 0x200 + idx * 0x68;
        wr::<[u64; 7]>(dst, pkt);
        wr::<[u64; 3]>(dst + 0x38, [v110, v118, v0d4]);
        wr::<[u64; 3]>(dst + 0x50, a5);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4059_8a90, mock_8a90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41ac_dfd0, mock_dfd0 as *const ())?;

    let g_118 = exe::ptr::<u16>(0x1_43c8_d118) as usize;
    let g_110 = exe::ptr::<u16>(0x1_43c8_d110) as usize;
    let g_0d4 = exe::ptr::<u16>(0x1_43c8_d0d4) as usize;
    let g_ed4 = exe::ptr::<u16>(0x1_43c8_ced4) as usize;
    let s_118 = rd::<u16>(g_118);
    let s_110 = rd::<u16>(g_110);
    let s_0d4 = rd::<u16>(g_0d4);
    let s_ed4 = rd::<u16>(g_ed4);
    wr::<u16>(g_118, 0);
    wr::<u16>(g_110, 1);
    wr::<u16>(g_0d4, 2);
    wr::<u16>(g_ed4, 1);

    let run = |f: F| -> [u64; 26] {
        let lut: [u16; 4] = [0, 1, 0, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x30];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x10, 0x5555_6666);
        wr::<u32>(ap + 0x18, 9);

        let tbl_580e0: [u64; 3] = [0xaa11, 0xbb22, 0xcc33];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x88 + 2 * 8, 0x40_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 2);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let arch_list: [u16; 2] = [1, 2];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 26]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_118, s_118);
    wr::<u16>(g_110, s_110);
    wr::<u16>(g_0d4, s_0d4);
    wr::<u16>(g_ed4, s_ed4);

    crate::diff_eq!(o, n, "14059b710 out");
    Ok(1)
}

pub unsafe fn diff_r_14059b990(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_5e10(out: usize, p1: usize) -> usize {
        let world = rd::<usize>(p1) as u64;
        wr::<u64>(out, world);
        for i in 1..9usize {
            wr::<u64>(out + i * 8, 0x9000 + i as u64);
        }
        out
    }
    unsafe extern "win64" fn mock_87f0(
        out: usize,
        _cnt: u64,
        _world40: usize,
        arch: u16,
    ) -> usize {
        wr::<u64>(out, 0);
        for i in 1..12usize {
            wr::<u64>(out + i * 8, (i as u64) * 0x100 + (arch as u64));
        }
        out
    }
    unsafe extern "win64" fn mock_c4e0(
        pkt_ptr: usize,
        a2_ptr: usize,
        a3_ptr: usize,
        a4: u64,
        a5: u64,
        a6: u64,
    ) {
        let pkt = rd::<[u64; 14]>(pkt_ptr);
        let a2 = rd::<[u64; 3]>(a2_ptr);
        let a3 = rd::<[u64; 3]>(a3_ptr);
        let world = a6 as usize;
        let idx = pkt[13] as usize;
        let dst = world + 0x200 + idx * 0xb0;
        wr::<[u64; 14]>(dst, pkt);
        wr::<[u64; 3]>(dst + 0x70, a2);
        wr::<[u64; 3]>(dst + 0x88, a3);
        wr::<[u64; 2]>(dst + 0xa0, [a4, a5]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4059_5e10, mock_5e10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4059_87f0, mock_87f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41ac_c4e0, mock_c4e0 as *const ())?;

    let run = |f: F| -> [u64; 44] {
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
        rd::<[u64; 44]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14059b990 out");
    Ok(1)
}

pub unsafe fn diff_r_14059c900(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_7660(out: usize, p1: usize) -> usize {
        let world = rd::<usize>(p1) as u64;
        wr::<u64>(out, world);
        for i in 1..12usize {
            wr::<u64>(out + i * 8, 0x7000 + i as u64);
        }
        out
    }
    unsafe extern "win64" fn mock_8000(
        out: usize,
        _cnt: u64,
        _world40: usize,
        arch: u16,
    ) -> usize {
        wr::<u64>(out, 0);
        for i in 1..6usize {
            wr::<u64>(out + i * 8, (i as u64) * 0x100 + (arch as u64));
        }
        out
    }
    unsafe extern "win64" fn mock_d4d0(
        pkt_ptr: usize,
        a2_ptr: usize,
        a3_ptr: usize,
        a4_ptr: usize,
        a5: u64,
        a6: u64,
        a7: u64,
    ) {
        let pkt = rd::<[u64; 8]>(pkt_ptr);
        let a2 = rd::<[u64; 3]>(a2_ptr);
        let a3 = rd::<[u64; 3]>(a3_ptr);
        let a4 = rd::<[u64; 3]>(a4_ptr);
        let world = a7 as usize;
        let idx = pkt[7] as usize;
        let dst = world + 0x200 + idx * 0x98;
        wr::<[u64; 8]>(dst, pkt);
        wr::<[u64; 3]>(dst + 0x40, a2);
        wr::<[u64; 3]>(dst + 0x58, a3);
        wr::<[u64; 3]>(dst + 0x70, a4);
        wr::<[u64; 2]>(dst + 0x88, [a5, a6]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4059_7660, mock_7660 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4059_8000, mock_8000 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41ac_d4d0, mock_d4d0 as *const ())?;

    let run = |f: F| -> [u64; 38] {
        let mut world = vec![0u8; 0x10200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x20_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 2);

        let arch_list: [u16; 2] = [1, 2];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 38]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14059c900 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141038eb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_mgr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let s_mgr = rd::<usize>(g_mgr);

    let run = |f: F, wrap: bool| -> ([u32; 32], usize, u32) {
        let mut ring_mem = [0u32; 32];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let init_wr = if wrap { rmp + 0x58 } else { rmp };
        let mut ring = [0u8; 0x60];
        let rp = ring.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x08, rmp);
        wr::<usize>(rp + 0x10, rmp + 0x68);
        wr::<u64>(rp + 0x18, 0);
        wr::<usize>(rp + 0x20, rmp);
        wr::<usize>(rp + 0x28, init_wr);
        wr::<u32>(rp + 0x30, 3);
        wr::<u32>(rp + 0x34, 3);

        let mut mgr = [0u8; 0x200];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x168, rp);
        wr::<u8>(mp + 0x198, 0);
        wr::<usize>(g_mgr, mp);

        let p1: u32 = 0x1234_5678;
        let p2: [u32; 12] = [
            0x1111_0001,
            0x1111_0002,
            0x1111_0003,
            0x2222_0001,
            0x2222_0002,
            0x2222_0003,
            0x3333_0001,
            0x3333_0002,
            0x3333_0003,
            0x4444_0001,
            0x4444_0002,
            0x4444_0003,
        ];
        f((&p1 as *const u32) as usize, p2.as_ptr() as usize);
        (ring_mem, rd::<usize>(rp + 0x28).wrapping_sub(rmp), rd::<u32>(rp + 0x30))
    };

    for wrap in [false, true] {
        let o = run(f_o, wrap);
        let n = run(f_n, wrap);
        wr::<usize>(g_mgr, s_mgr);
        crate::diff_eq!(o, n, "141038eb0 out");
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1410b0200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_0f90(
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
        wr::<u64>(out + 0x20, block.wrapping_add(0x100) as u64);
        wr::<u64>(out + 0x28, block.wrapping_add(0x200) as u64);
        out
    }
    unsafe extern "win64" fn mock_aa30(a1_ptr: usize, a2_ptr: usize) {
        let a1 = rd::<[u64; 4]>(a1_ptr);
        let mut a2 = rd::<[u64; 3]>(a2_ptr);
        let world = a2[0] as usize;
        a2[0] = 0;
        let idx = a1[3] as usize;
        let dst = world + 0x200 + idx * 0x38;
        wr::<[u64; 4]>(dst, a1);
        wr::<[u64; 3]>(dst + 0x20, a2);
    }

    let _m1 = crate::difftest::Mock::new(0x1_410b_0f90, mock_0f90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f3_aa30, mock_aa30 as *const ())?;

    let g_0f4 = exe::ptr::<u16>(0x1_43cd_a0f4) as usize;
    let s_0f4 = rd::<u16>(g_0f4);
    wr::<u16>(g_0f4, 2);

    let run = |f: F| -> [u64; 14] {
        let lut: [u16; 4] = [0, 0, 1, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x30];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x10, 0x7777_8888);
        wr::<u32>(ap + 0x18, 6);

        let mut world = vec![0u8; 0x10200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x88 + 3 * 8, 0x80_0000);
        wr::<u32>(wp + 0x10088 + 3 * 4, 2);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 3);
        rd::<[u64; 14]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_0f4, s_0f4);

    crate::diff_eq!(o, n, "1410b0200 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_401e_8210, "unknown_1401e8210", rend::cauldron_hot_g::unknown_1401e8210 as *const (), rend::cauldron_hot_g::diff_unknown_1401e8210),
// Replacement::verified_throws(0x1_401e_9e70, "unknown_1401e9e70", rend::cauldron_hot_g::unknown_1401e9e70 as *const (), Throws::Caught, rend::cauldron_hot_g::diff_unknown_1401e9e70),
// Replacement::verified_throws(0x1_401e_9fa0, "unknown_1401e9fa0", rend::cauldron_hot_g::unknown_1401e9fa0 as *const (), Throws::Caught, rend::cauldron_hot_g::diff_unknown_1401e9fa0),
// Replacement::verified_throws(0x1_403e_e490, "unknown_1403ee490", rend::cauldron_hot_g::unknown_1403ee490 as *const (), Throws::Discard, rend::cauldron_hot_g::diff_unknown_1403ee490),
// Replacement::verified_throws(0x1_404b_76d0, "unknown_1404b76d0", rend::cauldron_hot_g::unknown_1404b76d0 as *const (), Throws::Discard, rend::cauldron_hot_g::diff_unknown_1404b76d0),
// Replacement::verified(0x1_4059_b710, "r_14059b710", rend::cauldron_hot_g::r_14059b710 as *const (), rend::cauldron_hot_g::diff_r_14059b710),
// Replacement::verified(0x1_4059_b990, "r_14059b990", rend::cauldron_hot_g::r_14059b990 as *const (), rend::cauldron_hot_g::diff_r_14059b990),
// Replacement::verified(0x1_4059_c900, "r_14059c900", rend::cauldron_hot_g::r_14059c900 as *const (), rend::cauldron_hot_g::diff_r_14059c900),
// Replacement::verified(0x1_4103_8eb0, "unknown_141038eb0", rend::cauldron_hot_g::unknown_141038eb0 as *const (), rend::cauldron_hot_g::diff_unknown_141038eb0),
// Replacement::verified(0x1_410b_0200, "unknown_1410b0200", rend::cauldron_hot_g::unknown_1410b0200 as *const (), rend::cauldron_hot_g::diff_unknown_1410b0200),
