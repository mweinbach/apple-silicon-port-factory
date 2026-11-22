//! Decompiled hot functions shard V1 (`0x1403b3650..0x141092610`).

use crate::exe::{self, rd, wr};

/// `unknown_1403b3650`
/// Original: 0x1403b3650 (1140 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1403b3650(param_1: usize, param_2: usize) -> usize {
    let a = rd::<[f64; 16]>(param_1);
    let b = rd::<[f64; 16]>(param_2);

    let r1_0 = (((b[0] * a[4]) + (b[4] * a[5])) + (b[8] * a[6])) + (b[12] * a[7]);
    let r1_1 = (((b[5] * a[5]) + (a[4] * b[1])) + (a[6] * b[9])) + (a[7] * b[13]);
    let r1_2 = (((a[4] * b[2]) + (a[5] * b[6])) + (a[6] * b[10])) + (a[7] * b[14]);
    let r1_3 = (((a[4] * b[3]) + (a[5] * b[7])) + (a[6] * b[11])) + (a[7] * b[15]);

    let r2_0 = (((b[0] * a[8]) + (b[4] * a[9])) + (b[8] * a[10])) + (b[12] * a[11]);
    let r2_1 = (((b[5] * a[9]) + (a[8] * b[1])) + (a[10] * b[9])) + (a[11] * b[13]);
    let r2_2 = (((a[8] * b[2]) + (a[9] * b[6])) + (a[10] * b[10])) + (a[11] * b[14]);
    let r2_3 = (((a[8] * b[3]) + (a[9] * b[7])) + (a[10] * b[11])) + (a[11] * b[15]);

    let r3_0 = (((b[0] * a[12]) + (b[4] * a[13])) + (b[8] * a[14])) + (b[12] * a[15]);
    let r3_1 = (((b[5] * a[13]) + (a[12] * b[1])) + (a[14] * b[9])) + (a[15] * b[13]);
    let r3_2 = (((a[12] * b[2]) + (a[13] * b[6])) + (a[14] * b[10])) + (a[15] * b[14]);
    let r3_3 = (((a[12] * b[3]) + (a[13] * b[7])) + (a[14] * b[11])) + (a[15] * b[15]);

    let r0_0 = (((b[0] * a[0]) + (b[4] * a[1])) + (a[2] * b[8])) + (a[3] * b[12]);
    let r0_1 = (((a[0] * b[1]) + (b[5] * a[1])) + (a[2] * b[9])) + (a[3] * b[13]);
    let r0_2 = (((a[0] * b[2]) + (a[1] * b[6])) + (a[2] * b[10])) + (a[3] * b[14]);
    let r0_3 = (((a[0] * b[3]) + (a[1] * b[7])) + (a[2] * b[11])) + (a[3] * b[15]);

    wr::<[f64; 16]>(
        param_1,
        [
            r0_0, r0_1, r0_2, r0_3, r1_0, r1_1, r1_2, r1_3, r2_0, r2_1, r2_2, r2_3, r3_0, r3_1,
            r3_2, r3_3,
        ],
    );
    param_1
}

/// `std_14049c0a0`
/// Original: 0x14049c0a0 (274 bytes, Throws::No)
pub unsafe extern "win64" fn std_14049c0a0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let tree_ptr = exe::call_va!(0x1_401d_e2f0, (param_1) -> usize);
    let head = rd::<usize>(tree_ptr);
    wr::<usize>(param_2, rd::<usize>(head + 0x08));
    wr::<u32>(param_2 + 0x08, 0);
    wr::<usize>(param_2 + 0x10, head);

    let mut cur = rd::<usize>(param_2);
    while rd::<u8>(cur + 0x19) == 0 {
        wr::<usize>(param_2, cur);
        let cmp = exe::call_va!(0x1_4021_74f0, (cur.wrapping_add(0x20), param_3) -> u8);
        if cmp != 0 {
            wr::<u32>(param_2 + 0x08, 0);
            cur = rd::<usize>(cur + 0x10);
        } else {
            wr::<u32>(param_2 + 0x08, 1);
            wr::<usize>(param_2 + 0x10, cur);
            cur = rd::<usize>(cur);
        }
    }
    param_2
}

/// `unknown_14054b210`
/// Original: 0x14054b210 (543 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14054b210(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let world = rd::<usize>(param_1);

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
            let ent_count = if block != 0 {
                rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_2b4 = rd::<u16>(exe::ptr::<u16>(0x1_43cb_12b4) as usize) as u32;
                let arch_off = arch.wrapping_shl(5);
                let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac));
                let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
                let off_slot = world
                    .wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5))
                    .wrapping_add(0x40);

                let found1 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_2b4) -> i32);
                let mut off1 = if (found1 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found1 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_160 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d160) as usize) as u32;
                let found2 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_160) -> i32);
                let mut off2 = if (found2 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found2 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_ad0 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_5ad0) as usize) as u32;
                let found3 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_ad0) -> i32);
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
                    exe::call_va!(0x1_41a9_a450, (pkt.as_mut_ptr() as usize, world as u64));
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_140559f40`
/// Original: 0x140559f40 (120 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_140559f40(param_1: usize, param_2: usize) -> u64 {
    let r9 = rd::<u32>(exe::ptr::<u32>(0x1_4388_1f78) as usize);
    let r8 = rd::<u64>(exe::ptr::<u64>(0x1_4388_1f70) as usize);
    exe::call_va!(0x1_401e_9fa0, (param_2, param_1, r8, r9, 1u32, 1u32));
    if rd::<i32>(param_2 + 0x328) >= 0 {
        let stream = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(stream);
        exe::call_va!(rd::<usize>(vtbl), (stream, param_1));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u64>(param_1, 0);
    }
    exe::call_va!(0x1_401e_9e70, (param_2));
    0
}

/// `content_140933980`
/// Original: 0x140933980 (424 bytes, Throws::Caught)
pub unsafe extern "win64" fn content_140933980(param_1: usize, param_2: usize) -> u64 {
    let r9 = rd::<u32>(exe::ptr::<u32>(0x1_4387_dbf4) as usize);
    let r8 = rd::<u64>(exe::ptr::<u64>(0x1_4387_db78) as usize);
    exe::call_va!(0x1_401e_9fa0, (param_2, param_1, r8, r9, 1u32, 1u32));

    let r14 = rd::<usize>(exe::tls_pointer());
    let flags1 = rd::<u32>(r14 + 0x529c);
    let rbx1 = r14.wrapping_add(0x5388);
    if (flags1 & 1) == 0 {
        wr::<u32>(r14 + 0x529c, flags1 | 1);
        let v = (rd::<u32>(rbx1) & 0xff00_0001) | 1;
        wr::<u32>(rbx1, v);
        wr::<u8>(rbx1 + 0x03, 1);
        wr::<u32>(rbx1 + 0x04, 0);
        wr::<u8>(rbx1 + 0x08, 0);
        exe::call_va!(
            0x1_4183_4e48,
            (exe::ptr::<u8>(0x1_425f_eaa0) as usize)
        );
    }

    let s_empty = exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64;
    if rd::<i32>(param_2 + 0x328) >= 1 {
        let stream = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(stream);
        exe::call_va!(rd::<usize>(vtbl + 0x20), (stream, rbx1));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        let mut sv: [u64; 2] = [s_empty, 0];
        exe::call_va!(
            0x1_4240_9b50,
            (rbx1, sv.as_mut_ptr() as usize, 0u64, 0u64)
        );
    }

    exe::call_va!(0x1_4055_7390, (param_2, param_1, 1u32));

    let flags2 = rd::<u32>(r14 + 0x529c);
    let rbx2 = r14.wrapping_add(0x40c0);
    if (flags2 & 2) == 0 {
        wr::<u32>(r14 + 0x529c, flags2 | 2);
        let v = (rd::<u32>(rbx2) & 0xff00_0001) | 1;
        wr::<u32>(rbx2, v);
        wr::<u8>(rbx2 + 0x03, 1);
        wr::<u32>(rbx2 + 0x04, 0);
        wr::<u8>(rbx2 + 0x08, 0);
        exe::call_va!(
            0x1_4183_4e48,
            (exe::ptr::<u8>(0x1_425f_ea80) as usize)
        );
    }

    if rd::<i32>(param_2 + 0x328) >= 1 {
        let stream = rd::<usize>(param_2 + 0x338);
        let vtbl = rd::<usize>(stream);
        exe::call_va!(rd::<usize>(vtbl + 0x20), (stream, rbx2));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        let mut sv: [u64; 2] = [s_empty, 0];
        exe::call_va!(
            0x1_4240_9b50,
            (rbx2, sv.as_mut_ptr() as usize, 0u64, 0u64)
        );
    }

    exe::call_va!(0x1_401e_9e70, (param_2));
    0
}

/// `unknown_140f35ac0`
/// Original: 0x140f35ac0 (291 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f35ac0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let len = rd::<u64>(param_1 + 0x08);
    if len == 0 {
        wr::<u64>(param_2, 0);
        wr::<u64>(param_2 + 0x08, 0);
        return param_2;
    }

    let r9 = rd::<u64>(exe::ptr::<u64>(0x1_4307_9458) as usize);
    let ptr = rd::<u64>(param_1);
    let pos = exe::call_va!(0x1_401e_22e0, (ptr, len, param_3, r9, 2u64) -> u64);
    if pos == 0 {
        wr::<u64>(param_2, exe::ptr::<u8>(0x1_4307_8be8) as usize as u64);
        wr::<u64>(param_2 + 0x08, 1);
    } else if pos == u64::MAX {
        let ok = exe::call_va!(0x1_401d_fa30, (param_1) -> u8);
        if ok != 0 {
            let v = rd::<u128>(param_1);
            wr::<u128>(param_2, v);
        } else {
            wr::<u64>(param_2, exe::ptr::<u8>(0x1_4307_8be4) as usize as u64);
            wr::<u64>(param_2 + 0x08, 1);
        }
    } else {
        let sub_len = if len < pos { len } else { pos };
        let mut sub_sv: [u64; 2] = [ptr, sub_len];
        exe::call_va!(0x1_401d_f850, (param_2, sub_sv.as_mut_ptr() as usize));
    }
    param_2
}

/// `unknown_140fe9680`
/// Original: 0x140fe9680 (132 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fe9680(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let ent_count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if ent_count != 0 {
        let mut buf = [0u64; 12];
        exe::call_va!(
            0x1_40fe_9710,
            (
                buf.as_mut_ptr() as usize,
                param_2,
                block as u64,
                ent_count,
                world as u64
            )
        );
        let mut arg = buf;
        exe::call_va!(0x1_41e7_b330, (arg.as_mut_ptr() as usize));
    }
}

/// `unknown_140fff1e0`
/// Original: 0x140fff1e0 (103 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fff1e0(param_1: usize, param_2: u64) {
    let words = param_2.wrapping_add(0x3f) >> 6;
    exe::call_va!(0x1_408b_4ad0, (param_1, words));
    let old_bits = rd::<u64>(param_1 + 0x10);
    wr::<u64>(param_1 + 0x10, param_2);
    if old_bits < param_2 {
        let data_ptr = rd::<u64>(param_1);
        let mut view: [u64; 3] = [words, data_ptr, param_2];
        exe::call_va!(
            0x1_40ff_f0a0,
            (
                view.as_mut_ptr() as usize,
                old_bits,
                param_2.wrapping_sub(old_bits),
                0u32
            )
        );
    }
}

/// `unknown_1410470b0`
/// Original: 0x1410470b0 (124 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1410470b0(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let ent_count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if ent_count != 0 {
        let mut buf = [0u64; 7];
        exe::call_va!(
            0x1_4104_80c0,
            (
                buf.as_mut_ptr() as usize,
                param_2,
                block as u64,
                ent_count,
                world as u64
            )
        );
        let mut arg = buf;
        exe::call_va!(0x1_41ea_c8c0, (arg.as_mut_ptr() as usize));
    }
}

/// `r_14104a420`
/// Original: 0x14104a420 (497 bytes, Throws::No)
pub unsafe extern "win64" fn r_14104a420(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let id_f2c = rd::<u16>(exe::ptr::<u16>(0x1_43cc_ef2c) as usize) as usize;
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let world = rd::<usize>(param_1);
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_f2c = rd::<u64>(tbl_580e0.wrapping_add(id_f2c * 8));

        let mut pkt = [0u64; 6];
        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr);
            let block = rd::<usize>(world.wrapping_add((arch as usize) * 8).wrapping_add(0x88));
            let ent_count = if block != 0 {
                rd::<u32>(world.wrapping_add((arch as usize) * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_7a0 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f7a0) as usize) as u32;
                let arch_off = (arch as usize).wrapping_shl(5);
                let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac));
                let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
                let off_slot = world
                    .wrapping_add((arch as usize).wrapping_add(0xc03).wrapping_shl(5))
                    .wrapping_add(0x40);

                let found1 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_7a0) -> i32);
                let mut off1 = if (found1 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found1 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_928 = rd::<u16>(exe::ptr::<u16>(0x1_43ca_c928) as usize) as u32;
                let found2 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_928) -> i32);
                let mut off2 = if (found2 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found2 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                if off1 != 0 {
                    off1 = block.wrapping_add(off1);
                }
                if off2 != 0 {
                    off2 = block.wrapping_add(off2);
                }

                pkt[0] = (pkt[0] & !0xffffu64) | (arch as u64);
                pkt[1] = block as u64;
                pkt[2] = world as u64;
                pkt[3] = ent_count;
                pkt[4] = off1 as u64;
                pkt[5] = off2 as u64;
                exe::call_va!(0x1_41eb_0210, (pkt.as_mut_ptr() as usize, val_f2c));
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_141092610`
/// Original: 0x141092610 (352 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141092610(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let ent_count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088))
    } else {
        0
    };
    if ent_count != 0 {
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let id_904 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d904) as usize) as usize;
        let id_4a0 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_04a0) as usize) as usize;
        let val_4a0 = rd::<u64>(tbl_580e0.wrapping_add(id_4a0 * 8));
        let val_904 = rd::<u64>(tbl_580e0.wrapping_add(id_904 * 8));

        let mut buf = [0u64; 7];
        exe::call_va!(
            0x1_4109_36d0,
            (
                buf.as_mut_ptr() as usize,
                param_2,
                block as u64,
                ent_count,
                world as u64
            )
        );
        let loop_cnt = buf[3];
        if loop_cnt != 0 {
            let v_80 = buf[4];
            let v_90 = buf[6];
            let v_88 = buf[5];
            let v_68 = buf[1];
            for idx in 0..loop_cnt {
                buf[0] = v_80;
                buf[1] = v_88;
                buf[2] = v_90;
                buf[3] = v_68;
                buf[4] = idx;
                exe::call_va!(
                    0x1_41f2_b970,
                    (buf.as_mut_ptr() as usize, val_4a0, val_904)
                );
            }
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1403b3650(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, seed: f64| -> ([u64; 16], usize) {
        let mut a = [0.0f64; 16];
        let mut b = [0.0f64; 16];
        for i in 0..16 {
            a[i] = seed + (i as f64) * 1.25 - ((i & 3) as f64) * 0.375;
            b[i] = (seed * 0.5) - (i as f64) * 0.625 + ((i >> 2) as f64) * 2.125;
        }
        let ap = a.as_mut_ptr() as usize;
        let bp = b.as_ptr() as usize;
        let ret = f(ap, bp);
        (rd::<[u64; 16]>(ap), ret.wrapping_sub(ap))
    };

    for seed in [1.0f64, -3.5, 0.125] {
        let o = run(f_o, seed);
        let n = run(f_n, seed);
        crate::diff_eq!(o, n, "1403b3650 seed={}", seed);
    }
    Ok(3)
}

pub unsafe fn diff_std_14049c0a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_e2f0(rcx: usize) -> usize {
        rcx
    }
    unsafe extern "win64" fn mock_74f0(node_key_ptr: usize, search_key_ptr: usize) -> u8 {
        if rd::<u64>(node_key_ptr) < rd::<u64>(search_key_ptr) {
            1
        } else {
            0
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_e2f0, mock_e2f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4021_74f0, mock_74f0 as *const ())?;

    let run = |f: F, search_key: u64| -> (usize, u32, usize) {
        // 4 nodes: 0 = head (is_nil=1), 1 = root (key=20), 2 = left (key=10), 3 = right (key=30)
        let mut nodes = [0u8; 4 * 0x30];
        let base = nodes.as_mut_ptr() as usize;
        let n_head = base;
        let n_root = base + 0x30;
        let n_left = base + 0x60;
        let n_right = base + 0x90;

        wr::<usize>(n_head + 0x08, n_root);
        wr::<u8>(n_head + 0x19, 1);

        wr::<usize>(n_root, n_left);
        wr::<usize>(n_root + 0x10, n_right);
        wr::<u8>(n_root + 0x19, 0);
        wr::<u64>(n_root + 0x20, 20);

        wr::<usize>(n_left, n_head);
        wr::<usize>(n_left + 0x10, n_head);
        wr::<u8>(n_left + 0x19, 0);
        wr::<u64>(n_left + 0x20, 10);

        wr::<usize>(n_right, n_head);
        wr::<usize>(n_right + 0x10, n_head);
        wr::<u8>(n_right + 0x19, 0);
        wr::<u64>(n_right + 0x20, 30);

        let tree_holder: [usize; 1] = [n_head];
        let mut out = [0u64; 3];
        let op = out.as_mut_ptr() as usize;
        let ret = f(
            tree_holder.as_ptr() as usize,
            op,
            (&search_key as *const u64) as usize,
        );
        assert_eq!(ret, op);
        (
            rd::<usize>(op).wrapping_sub(base),
            rd::<u32>(op + 0x08),
            rd::<usize>(op + 0x10).wrapping_sub(base),
        )
    };

    for k in [5u64, 15, 20, 25, 35] {
        let o = run(f_o, k);
        let n = run(f_n, k);
        crate::diff_eq!(o, n, "14049c0a0 k={}", k);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_14054b210(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x5550 && n == 4 {
            match want {
                10 => 0,
                20 => 1,
                30 => 2,
                _ => -1,
            }
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_a450(pkt_ptr: usize, world: usize) {
        let pkt = rd::<[u64; 5]>(pkt_ptr);
        let idx = pkt[4] as usize;
        let dst = world + 0x200 + idx * (5 * 8);
        wr::<[u64; 5]>(dst, pkt);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41a9_a450, mock_a450 as *const ())?;

    let g_2b4 = exe::ptr::<u16>(0x1_43cb_12b4) as usize;
    let g_160 = exe::ptr::<u16>(0x1_43c8_d160) as usize;
    let g_ad0 = exe::ptr::<u16>(0x1_43c9_5ad0) as usize;
    let saved = (rd::<u16>(g_2b4), rd::<u16>(g_160), rd::<u16>(g_ad0));
    wr::<u16>(g_2b4, 10);
    wr::<u16>(g_160, 20);
    wr::<u16>(g_ad0, 99); // not found -> 0

    let run = |f: F| -> [u64; 10] {
        let off_tbl: [u32; 3] = [0x110, 0x220, 0x330];
        let mut world = vec![0u8; 0x30200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 1 * 8, 0x40_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);
        wr::<u64>(wp + 0x18098 + 0x20, 0x5550);
        wr::<u32>(wp + 0x180ac + 0x20, 4);
        wr::<usize>(wp + (1 + 0xc03) * 0x20 + 0x40, off_tbl.as_ptr() as usize);

        let arch_list: [u16; 2] = [0, 1];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 10]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_2b4, saved.0);
    wr::<u16>(g_160, saved.1);
    wr::<u16>(g_ad0, saved.2);

    crate::diff_eq!(o, n, "14054b210 out");
    Ok(1)
}

pub unsafe fn diff_unknown_140559f40(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static LOG_SUM: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_9fa0(
        rcx: usize,
        rdx: usize,
        r8: u64,
        r9: u32,
        a5: u32,
        a6: u32,
    ) {
        let off = rd::<u32>(rcx + 0x328) as u64;
        let prev = rd::<u64>(rdx);
        let add = off
            .wrapping_add(prev)
            .wrapping_add(r8 & 0xffff)
            .wrapping_add(r9 as u64)
            .wrapping_add(a5 as u64)
            .wrapping_add(a6 as u64);
        LOG_SUM.fetch_add(add, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_vfunc0(stream: usize, out_ptr: usize) {
        wr::<u64>(out_ptr, rd::<u64>(stream + 0x08));
    }
    unsafe extern "win64" fn mock_9e70(rcx: usize) {
        wr::<u32>(rcx + 0x32c, 0x7777);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9fa0, mock_9fa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_9e70, mock_9e70 as *const ())?;

    let run = |f: F, status: i32, clear_flag: u8| -> (u64, u64, u32, u64) {
        LOG_SUM.store(0, Ordering::Relaxed);
        let vtbl: [usize; 1] = [mock_vfunc0 as *const () as usize];
        let stream_obj: [u64; 2] = [vtbl.as_ptr() as u64, 0xcafe_babe_1234];

        let mut ctx = [0u8; 0x370];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<i32>(cp + 0x328, status);
        wr::<usize>(cp + 0x338, stream_obj.as_ptr() as usize);
        wr::<u8>(cp + 0x369, clear_flag);

        let mut val: u64 = 0xdead_beef_9999;
        let ret = f((&mut val as *mut u64) as usize, cp);
        (
            ret,
            val,
            rd::<u32>(cp + 0x32c),
            LOG_SUM.load(Ordering::Relaxed),
        )
    };

    for (status, flag) in [(0i32, 0u8), (-1, 1), (-1, 0)] {
        let o = run(f_o, status, flag);
        let n = run(f_n, status, flag);
        crate::diff_eq!(o, n, "140559f40 status={} flag={}", status, flag);
    }
    Ok(3)
}

pub unsafe fn diff_content_140933980(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static LOG_SUM: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_9fa0(
        _rcx: usize,
        rdx: usize,
        _r8: u64,
        r9: u32,
        a5: u32,
        a6: u32,
    ) {
        let add = rd::<u64>(rdx)
            .wrapping_add(r9 as u64)
            .wrapping_add(a5 as u64)
            .wrapping_add(a6 as u64);
        LOG_SUM.fetch_add(add, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_tlregdtor(fn_ptr: usize) -> i32 {
        LOG_SUM.fetch_add((fn_ptr & 0xffff) as u64, Ordering::Relaxed);
        0
    }
    unsafe extern "win64" fn mock_vfunc4(stream: usize, tls_field: usize) {
        let tag = rd::<u32>(tls_field) as u64;
        wr::<u32>(
            tls_field + 0x04,
            rd::<u32>(stream + 0x08).wrapping_add(tag as u32),
        );
    }
    unsafe extern "win64" fn mock_9b50(
        tls_field: usize,
        sv_ptr: usize,
        r8: u64,
        r9: u64,
    ) {
        let p = rd::<u64>(sv_ptr);
        let len = rd::<u64>(sv_ptr + 0x08);
        wr::<u32>(
            tls_field + 0x04,
            ((p & 0xffff) ^ len ^ r8 ^ r9) as u32,
        );
    }
    unsafe extern "win64" fn mock_7390(_ctx: usize, p1: usize, flag: u32) {
        wr::<u64>(p1, rd::<u64>(p1).wrapping_add(flag as u64));
    }
    unsafe extern "win64" fn mock_9e70(ctx: usize) {
        wr::<u32>(ctx + 0x32c, 0x5555);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9fa0, mock_9fa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4183_4e48, mock_tlregdtor as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4055_7390, mock_7390 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401e_9e70, mock_9e70 as *const ())?;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let run = |f: F,
               init_flags: u32,
               status: i32,
               flag_369: u8|
     -> (u64, u64, u32, [u8; 16], [u8; 16], u32, u64) {
        LOG_SUM.store(0, Ordering::Relaxed);
        let mut fake_tls = vec![0u8; 0x5400];
        let tp = fake_tls.as_mut_ptr() as usize;
        wr::<u32>(tp + 0x529c, init_flags);
        wr::<u32>(tp + 0x5388, 0x1234_5678);
        wr::<u32>(tp + 0x40c0, 0x9abc_def0);
        wr::<usize>(tls_slot, tp);

        let mut vtbl = [0usize; 5];
        vtbl[0x20 / 8] = mock_vfunc4 as *const () as usize;
        let stream_obj: [u64; 2] = [vtbl.as_ptr() as u64, 0x1000];

        let mut ctx = [0u8; 0x370];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<i32>(cp + 0x328, status);
        wr::<usize>(cp + 0x338, stream_obj.as_ptr() as usize);
        wr::<u8>(cp + 0x369, flag_369);

        let mut p1_val: u64 = 0x4200;
        let ret = f((&mut p1_val as *mut u64) as usize, cp);
        wr::<usize>(tls_slot, saved_tls0);

        (
            ret,
            p1_val,
            rd::<u32>(tp + 0x529c),
            rd::<[u8; 16]>(tp + 0x5388),
            rd::<[u8; 16]>(tp + 0x40c0),
            rd::<u32>(cp + 0x32c),
            LOG_SUM.load(Ordering::Relaxed),
        )
    };

    for (flags, status, f369) in [(0u32, 1i32, 0u8), (3, 0, 1), (1, 0, 0)] {
        let o = run(f_o, flags, status, f369);
        let n = run(f_n, flags, status, f369);
        crate::diff_eq!(
            o,
            n,
            "140933980 flags={} status={} f369={}",
            flags,
            status,
            f369
        );
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140f35ac0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU8, AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static MOCK_POS: AtomicU64 = AtomicU64::new(0);
    static MOCK_OK: AtomicU8 = AtomicU8::new(0);

    unsafe extern "win64" fn mock_22e0(
        _ptr: u64,
        _len: u64,
        _p3: u64,
        _r9: u64,
        _a5: u64,
    ) -> u64 {
        MOCK_POS.load(Ordering::Relaxed)
    }
    unsafe extern "win64" fn mock_fa30(_p1: usize) -> u8 {
        MOCK_OK.load(Ordering::Relaxed)
    }
    unsafe extern "win64" fn mock_f850(out_ptr: usize, sv_ptr: usize) {
        let p = rd::<u64>(sv_ptr);
        let l = rd::<u64>(sv_ptr + 0x08);
        wr::<u64>(out_ptr, p.wrapping_add(0x100));
        wr::<u64>(out_ptr + 0x08, l);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_22e0, mock_22e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_fa30, mock_fa30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_f850, mock_f850 as *const ())?;

    let run = |f: F, in_len: u64, pos: u64, ok: u8| -> [u64; 2] {
        MOCK_POS.store(pos, Ordering::Relaxed);
        MOCK_OK.store(ok, Ordering::Relaxed);
        let p1: [u64; 2] = [0x5000_0000, in_len];
        let mut out: [u64; 2] = [0xdead, 0xbeef];
        let op = out.as_mut_ptr() as usize;
        let ret = f(p1.as_ptr() as usize, op, 0);
        assert_eq!(ret, op);
        out
    };

    for (len, pos, ok) in [
        (0u64, 0u64, 0u8),
        (10, 0, 0),
        (10, u64::MAX, 1),
        (10, u64::MAX, 0),
        (10, 4, 0),
        (10, 20, 0),
    ] {
        let o = run(f_o, len, pos, ok);
        let n = run(f_n, len, pos, ok);
        crate::diff_eq!(o, n, "140f35ac0 len={} pos={} ok={}", len, pos, ok);
    }
    Ok(6)
}

pub unsafe fn diff_unknown_140fe9680(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_9710(
        out_ptr: usize,
        arch: u16,
        block: u64,
        count: u32,
        world: u64,
    ) -> usize {
        for i in 0..12usize {
            wr::<u64>(
                out_ptr + i * 8,
                block
                    .wrapping_add((arch as u64) << 16)
                    .wrapping_add((count as u64) << 8)
                    .wrapping_add(i as u64),
            );
        }
        wr::<u64>(out_ptr + 11 * 8, world);
        out_ptr
    }
    unsafe extern "win64" fn mock_b330(arg_ptr: usize) {
        let mut vals = rd::<[u64; 12]>(arg_ptr);
        let world = vals[11] as usize;
        vals[11] = 0;
        wr::<[u64; 12]>(world + 0x200, vals);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40fe_9710, mock_9710 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e7_b330, mock_b330 as *const ())?;

    let run = |f: F, arch: u16| -> [u64; 12] {
        let mut world = vec![0u8; 0x10400];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x90_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 7);

        let p1 = [wp];
        f(p1.as_ptr() as usize, arch);
        rd::<[u64; 12]>(wp + 0x200)
    };

    for arch in [1u16, 2] {
        let o = run(f_o, arch);
        let n = run(f_n, arch);
        crate::diff_eq!(o, n, "140fe9680 arch={}", arch);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140fff1e0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static RESIZE_ARG: AtomicU64 = AtomicU64::new(0);
    static FILL_REC0: AtomicU64 = AtomicU64::new(0);
    static FILL_REC1: AtomicU64 = AtomicU64::new(0);
    static FILL_REC2: AtomicU64 = AtomicU64::new(0);
    static FILL_REC3: AtomicU64 = AtomicU64::new(0);
    static FILL_REC4: AtomicU64 = AtomicU64::new(0);
    static FILL_REC5: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_4ad0(_p1: usize, words: u64) {
        RESIZE_ARG.store(words, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_f0a0(
        view_ptr: usize,
        old_bits: u64,
        count: u64,
        val: u8,
    ) {
        let v = rd::<[u64; 3]>(view_ptr);
        FILL_REC0.store(v[0], Ordering::Relaxed);
        FILL_REC1.store(v[1], Ordering::Relaxed);
        FILL_REC2.store(v[2], Ordering::Relaxed);
        FILL_REC3.store(old_bits, Ordering::Relaxed);
        FILL_REC4.store(count, Ordering::Relaxed);
        FILL_REC5.store(val as u64, Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_408b_4ad0, mock_4ad0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40ff_f0a0, mock_f0a0 as *const ())?;

    let run = |f: F, old_bits: u64, new_bits: u64| -> ([u64; 3], u64, [u64; 6]) {
        RESIZE_ARG.store(0, Ordering::Relaxed);
        FILL_REC0.store(0, Ordering::Relaxed);
        FILL_REC1.store(0, Ordering::Relaxed);
        FILL_REC2.store(0, Ordering::Relaxed);
        FILL_REC3.store(0, Ordering::Relaxed);
        FILL_REC4.store(0, Ordering::Relaxed);
        FILL_REC5.store(0, Ordering::Relaxed);
        let mut bv: [u64; 3] = [0xabcd_0000, 2, old_bits];
        f(bv.as_mut_ptr() as usize, new_bits);
        (
            bv,
            RESIZE_ARG.load(Ordering::Relaxed),
            [
                FILL_REC0.load(Ordering::Relaxed),
                FILL_REC1.load(Ordering::Relaxed),
                FILL_REC2.load(Ordering::Relaxed),
                FILL_REC3.load(Ordering::Relaxed),
                FILL_REC4.load(Ordering::Relaxed),
                FILL_REC5.load(Ordering::Relaxed),
            ],
        )
    };

    for (old_b, new_b) in [(10u64, 70u64), (70, 10), (64, 64)] {
        let o = run(f_o, old_b, new_b);
        let n = run(f_n, old_b, new_b);
        crate::diff_eq!(o, n, "140fff1e0 old={} new={}", old_b, new_b);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1410470b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_80c0(
        out_ptr: usize,
        arch: u16,
        block: u64,
        count: u32,
        world: u64,
    ) -> usize {
        for i in 0..6usize {
            wr::<u64>(
                out_ptr + i * 8,
                block
                    .wrapping_add((arch as u64) << 16)
                    .wrapping_add((count as u64) << 8)
                    .wrapping_add(i as u64),
            );
        }
        wr::<u64>(out_ptr + 6 * 8, world);
        out_ptr
    }
    unsafe extern "win64" fn mock_c8c0(arg_ptr: usize) {
        let mut vals = rd::<[u64; 7]>(arg_ptr);
        let world = vals[6] as usize;
        vals[6] = 0;
        wr::<[u64; 7]>(world + 0x200, vals);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4104_80c0, mock_80c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41ea_c8c0, mock_c8c0 as *const ())?;

    let run = |f: F, arch: u16| -> [u64; 7] {
        let mut world = vec![0u8; 0x10400];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 3 * 8, 0x80_0000);
        wr::<u32>(wp + 0x10088 + 3 * 4, 4);

        let p1 = [wp];
        f(p1.as_ptr() as usize, arch);
        rd::<[u64; 7]>(wp + 0x200)
    };

    for arch in [0u16, 3] {
        let o = run(f_o, arch);
        let n = run(f_n, arch);
        crate::diff_eq!(o, n, "1410470b0 arch={}", arch);
    }
    Ok(2)
}

pub unsafe fn diff_r_14104a420(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x7770 && n == 3 {
            match want {
                12 => 0,
                34 => 1,
                _ => -1,
            }
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_0210(pkt_ptr: usize, val_f2c: u64) {
        let mut pkt = rd::<[u64; 6]>(pkt_ptr);
        let world = pkt[2] as usize;
        pkt[0] &= 0xffff;
        pkt[2] = 0;
        let dst = world + 0x200;
        wr::<[u64; 6]>(dst, pkt);
        wr::<u64>(dst + 48, val_f2c);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41eb_0210, mock_0210 as *const ())?;

    let g_f2c = exe::ptr::<u16>(0x1_43cc_ef2c) as usize;
    let g_7a0 = exe::ptr::<u16>(0x1_43c8_f7a0) as usize;
    let g_928 = exe::ptr::<u16>(0x1_43ca_c928) as usize;
    let saved = (rd::<u16>(g_f2c), rd::<u16>(g_7a0), rd::<u16>(g_928));
    wr::<u16>(g_f2c, 1);
    wr::<u16>(g_7a0, 12);
    wr::<u16>(g_928, 34);

    let run = |f: F| -> [u64; 7] {
        let off_tbl: [u32; 2] = [0x120, 0x340];
        let tbl_580e0: [u64; 2] = [0, 0xdead_beef_cafe];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 1 * 8, 0x50_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 6);
        wr::<u64>(wp + 0x18098 + 0x20, 0x7770);
        wr::<u32>(wp + 0x180ac + 0x20, 3);
        wr::<usize>(wp + (1 + 0xc03) * 0x20 + 0x40, off_tbl.as_ptr() as usize);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let arch_list: [u16; 2] = [0, 1];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 7]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_f2c, saved.0);
    wr::<u16>(g_7a0, saved.1);
    wr::<u16>(g_928, saved.2);

    crate::diff_eq!(o, n, "14104a420 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141092610(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_36d0(
        out_ptr: usize,
        arch: u16,
        block: u64,
        ent_count: u32,
        world: u64,
    ) -> usize {
        wr::<u64>(out_ptr, 0);
        wr::<u64>(out_ptr + 0x08, world); // buf[1] -> r15
        wr::<u64>(out_ptr + 0x10, 0);
        wr::<u64>(out_ptr + 0x18, 2); // buf[3] -> loop_cnt = 2
        wr::<u64>(out_ptr + 0x20, block + (arch as u64)); // buf[4] -> rbx
        wr::<u64>(out_ptr + 0x28, block + 0x100); // buf[5] -> r14
        wr::<u64>(out_ptr + 0x30, block + (ent_count as u64) * 0x10); // buf[6] -> rbp
        out_ptr
    }
    unsafe extern "win64" fn mock_b970(pkt_ptr: usize, val_4a0: u64, val_904: u64) {
        let mut pkt = rd::<[u64; 5]>(pkt_ptr);
        let world = pkt[3] as usize;
        let idx = pkt[4] as usize;
        pkt[3] = 0;
        let dst = world + 0x200 + idx * (7 * 8);
        wr::<[u64; 5]>(dst, pkt);
        wr::<[u64; 2]>(dst + 40, [val_4a0, val_904]);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4109_36d0, mock_36d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f2_b970, mock_b970 as *const ())?;

    let g_904 = exe::ptr::<u16>(0x1_43c8_d904) as usize;
    let g_4a0 = exe::ptr::<u16>(0x1_43c9_04a0) as usize;
    let saved = (rd::<u16>(g_904), rd::<u16>(g_4a0));
    wr::<u16>(g_904, 0);
    wr::<u16>(g_4a0, 1);

    let run = |f: F| -> [u64; 14] {
        let tbl_580e0: [u64; 2] = [0x1111_2222, 0x3333_4444];
        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x60_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 5);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 2);
        rd::<[u64; 14]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_904, saved.0);
    wr::<u16>(g_4a0, saved.1);

    crate::diff_eq!(o, n, "141092610 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_403b_3650, "unknown_1403b3650", rend::cauldron_hot_v1::unknown_1403b3650 as *const (), rend::cauldron_hot_v1::diff_unknown_1403b3650),
// Replacement::verified(0x1_4049_c0a0, "std_14049c0a0", rend::cauldron_hot_v1::std_14049c0a0 as *const (), rend::cauldron_hot_v1::diff_std_14049c0a0),
// Replacement::verified(0x1_4054_b210, "unknown_14054b210", rend::cauldron_hot_v1::unknown_14054b210 as *const (), rend::cauldron_hot_v1::diff_unknown_14054b210),
// Replacement::verified_throws(0x1_4055_9f40, "unknown_140559f40", rend::cauldron_hot_v1::unknown_140559f40 as *const (), Throws::Discard, rend::cauldron_hot_v1::diff_unknown_140559f40),
// Replacement::verified_throws(0x1_4093_3980, "content_140933980", rend::cauldron_hot_v1::content_140933980 as *const (), Throws::Caught, rend::cauldron_hot_v1::diff_content_140933980),
// Replacement::verified(0x1_40f3_5ac0, "unknown_140f35ac0", rend::cauldron_hot_v1::unknown_140f35ac0 as *const (), rend::cauldron_hot_v1::diff_unknown_140f35ac0),
// Replacement::verified(0x1_40fe_9680, "unknown_140fe9680", rend::cauldron_hot_v1::unknown_140fe9680 as *const (), rend::cauldron_hot_v1::diff_unknown_140fe9680),
// Replacement::verified(0x1_40ff_f1e0, "unknown_140fff1e0", rend::cauldron_hot_v1::unknown_140fff1e0 as *const (), rend::cauldron_hot_v1::diff_unknown_140fff1e0),
// Replacement::verified(0x1_4104_70b0, "unknown_1410470b0", rend::cauldron_hot_v1::unknown_1410470b0 as *const (), rend::cauldron_hot_v1::diff_unknown_1410470b0),
// Replacement::verified(0x1_4104_a420, "r_14104a420", rend::cauldron_hot_v1::r_14104a420 as *const (), rend::cauldron_hot_v1::diff_r_14104a420),
// Replacement::verified(0x1_4109_2610, "unknown_141092610", rend::cauldron_hot_v1::unknown_141092610 as *const (), rend::cauldron_hot_v1::diff_unknown_141092610),
