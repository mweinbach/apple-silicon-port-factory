//! Decompiled `rend` hot functions shard Z9A (`0x14231e750..0x14232c7a0`).

use crate::exe::{self, rd, wr};

/// `rend_14231e750`
/// Original: 0x14231e750 (426 bytes, Throws::Discard)
pub unsafe extern "win64" fn rend_14231e750(
    param_1: usize,
    param_2: usize,
    param_3: u64,
    param_4: usize,
) {
    let vec_258 = param_1 + 0x258;
    let free_len = rd::<u32>(param_1 + 0x260) as u64;
    let reused_count = if free_len < param_3 {
        free_len
    } else {
        param_3
    };
    let new_free_len = free_len.wrapping_sub(reused_count);

    let mut out_ptr = param_4;
    if reused_count != 0 {
        let mut byte_off = new_free_len.wrapping_mul(4) as usize;
        for _ in 0..reused_count {
            let base = rd::<usize>(vec_258);
            wr::<u32>(out_ptr, rd::<u32>(base.wrapping_add(byte_off)));
            out_ptr = out_ptr.wrapping_add(4);
            byte_off = byte_off.wrapping_add(4);
        }
    }

    exe::call_va!(0x1_406d_cc50, (vec_258, new_free_len));

    let obj_278 = param_1 + 0x278;
    let vtbl_278 = rd::<usize>(obj_278);
    let old_cap = exe::call_va!(rd::<usize>(vtbl_278 + 0x38), (obj_278) -> u64);
    let new_cap = old_cap.wrapping_sub(reused_count).wrapping_add(param_3);
    let vtbl_278_b = rd::<usize>(obj_278);
    exe::call_va!(rd::<usize>(vtbl_278_b + 0x28), (obj_278, new_cap));

    let stride = rd::<u32>(param_1 + 0x20) as u64;
    let obj_348 = param_1 + 0x348;
    let vtbl_348 = rd::<usize>(obj_348);
    let tag_ptr = exe::ptr::<u8>(0x1_433f_11a8) as usize;
    exe::call_va!(
        rd::<usize>(vtbl_348 + 0x20),
        (obj_348, stride.wrapping_mul(new_cap), tag_ptr)
    );

    let mut cur_idx = old_cap;
    while cur_idx < new_cap {
        wr::<u32>(out_ptr, cur_idx as u32);
        out_ptr = out_ptr.wrapping_add(4);
        cur_idx = cur_idx.wrapping_add(1);
    }

    let mut max_id: u32 = 0;
    if param_3 != 0 {
        let mut in_ptr = param_2;
        for i in 0..(param_3 as usize) {
            let slot = rd::<u32>(param_4.wrapping_add(i * 4));
            let id = rd::<u32>(in_ptr);
            in_ptr = in_ptr.wrapping_add(0x10);

            let arr_310 = rd::<usize>(param_1 + 0x310);
            wr::<u32>(arr_310.wrapping_add((slot as usize) * 4), id);

            let bits_320 = rd::<usize>(param_1 + 0x320);
            let word_addr = bits_320.wrapping_add(((slot >> 6) as usize) * 8);
            wr::<u64>(word_addr, rd::<u64>(word_addr) | (1u64 << (slot & 0x3f)));

            if id > max_id {
                max_id = id;
            }
        }
    }

    if rd::<u32>(param_1 + 0x240) <= max_id {
        let req_len = max_id.wrapping_add(1) as u64;
        exe::call_va!(0x1_412e_b5b0, (param_1 + 0x238, req_len, tag_ptr));
    }

    if param_3 != 0 {
        let mut in_ptr = param_2;
        for i in 0..(param_3 as usize) {
            let id = rd::<u32>(in_ptr);
            in_ptr = in_ptr.wrapping_add(0x10);
            let slot = rd::<u32>(param_4.wrapping_add(i * 4));
            let arr_238 = rd::<usize>(param_1 + 0x238);
            wr::<u32>(arr_238.wrapping_add((id as usize) * 4), slot);
        }
    }
}

/// `unknown_14232c7a0`
/// Original: 0x14232c7a0 (5138 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_14232c7a0(param_1: usize, param_2: usize) {
    let mut mem_file = [0u64; 0x140 / 8];
    let mf_ptr = mem_file.as_mut_ptr() as usize;
    exe::call_va!(
        0x1_4244_36a0,
        (
            mf_ptr,
            rd::<u64>(param_2 + 0x08),
            rd::<u64>(param_2),
            0u32
        )
    );

    let vtbl_fs = exe::ptr::<u8>(0x1_4340_e9e0) as usize;
    let mut fs: [usize; 2] = [vtbl_fs, mf_ptr];
    let fs_ptr = fs.as_mut_ptr() as usize;

    let mut ver: u32 = 0;
    exe::call_va!(0x1_4249_5aa0, (fs_ptr, (&mut ver as *mut u32) as usize));
    if ver != 0x4e {
        let mut err_buf = [0u64; 16];
        let err_ptr = err_buf.as_mut_ptr() as usize;
        if ver > 0x4e {
            let name = exe::call_va!(0x1_4131_1e60, (param_1) -> usize);
            let exc = exe::call_va!(0x1_4133_bc30, (err_ptr, name) -> usize);
            exe::call_va!(0x1_4133_cd10, (exc));
        }
        let name = exe::call_va!(0x1_4131_1e60, (param_1) -> usize);
        let exc = exe::call_va!(0x1_4133_bbc0, (err_ptr, name) -> usize);
        exe::call_va!(0x1_4133_cca0, (exc));
        return;
    }

    let fs_vcall = |fs_p: usize, slot_off: usize, dst_p: usize| {
        let vtbl = rd::<usize>(fs_p);
        exe::call_va!(rd::<usize>(vtbl + slot_off), (fs_p, dst_p));
    };
    let mf_read = |fs_p: usize, dst_p: usize, byte_len: u64| {
        let inner = rd::<usize>(fs_p + 0x08);
        let vtbl = rd::<usize>(inner);
        exe::call_va!(rd::<usize>(vtbl + 0x28), (inner, dst_p, byte_len));
    };

    let mut local_res10: i32 = 0;
    let mut local_554: i32 = 0;
    let mut local_res20: i32 = 0;
    fs_vcall(fs_ptr, 0x48, (&mut local_res10 as *mut i32) as usize);
    fs_vcall(fs_ptr, 0x48, (&mut local_554 as *mut i32) as usize);
    fs_vcall(fs_ptr, 0x48, (&mut local_res20 as *mut i32) as usize);

    wr::<u8>(param_1 + 0x270, local_res10 as u8);
    wr::<u8>(param_1 + 0x272, local_res20 as u8);
    wr::<u8>(param_1 + 0x271, local_res10 as u8);

    if local_res20 > 0 {
        let count_bytes = ((local_res20 as u8) as u64) << 2;
        mf_read(fs_ptr, param_1 + 0x1c8, count_bytes);
        if local_res20 > 0 {
            let mut sum: u32 = 0;
            for i in 0..(local_res20 as u32 as usize) {
                wr::<u32>(param_1 + 0x1e8 + i * 4, sum);
                sum = sum.wrapping_add(rd::<u32>(param_1 + 0x1c8 + i * 4));
            }
            exe::call_va!(0x1_4232_9e30, (param_1, sum));
        }
    }

    let mut local_568: i32 = 0;
    if local_res20 > 0 {
        let one_bits = rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize);
        let c_660 = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_7660) as usize);
        let mut ptr_1e8 = param_1 + 0x1e8;

        while local_568 < local_res20 {
            let group_cnt = rd::<u32>(ptr_1e8 - 0x20) as u64;
            if group_cnt != 0 {
                let mut group_hash: u64 = 0;
                let base_idx = rd::<u32>(ptr_1e8) as usize;
                let mut str_dst = rd::<usize>(param_1 + 0x180).wrapping_add(base_idx * 0x40);
                let hash_arr = rd::<usize>(param_1 + 0x190).wrapping_add(base_idx * 4);
                let mut dst_1a0 = rd::<usize>(param_1 + 0x1a0).wrapping_add(base_idx * 0x10);
                let mut dst_1b0 = rd::<usize>(param_1 + 0x1b0).wrapping_add(base_idx * 0x30);
                let mut dst_1c0 = rd::<usize>(param_1 + 0x1c0).wrapping_add(base_idx * 0x20);

                let mut buf_150: [u32; 17] = [
                    one_bits, 0, 0, 0, c_660[0], c_660[1], c_660[2], c_660[3], c_660[0], c_660[1],
                    c_660[2], c_660[3], 0, 0, 0, 0, 0,
                ];
                let buf_150_ptr = buf_150.as_mut_ptr() as usize;

                for item_idx in 0..(group_cnt as usize) {
                    let mut slen: i32 = 0;
                    fs_vcall(fs_ptr, 0x48, (&mut slen as *mut i32) as usize);
                    mf_read(fs_ptr, str_dst, slen as i64 as u64);

                    let mut hash32: u32 = 0x811c_9dc5;
                    let mut nul_off = 0usize;
                    while rd::<u8>(str_dst.wrapping_add(nul_off)) != 0 {
                        nul_off += 1;
                    }
                    for k in 0..nul_off {
                        let ch = rd::<i8>(str_dst.wrapping_add(k)) as i32 as u32;
                        hash32 = ((ch | 0x20) ^ hash32).wrapping_mul(0x0100_0193);
                    }
                    wr::<u32>(hash_arr.wrapping_add(item_idx * 4), hash32);

                    let mut h64: u64 = 0xcbf2_9ce4_8422_2325;
                    for b in hash32.to_le_bytes() {
                        h64 = (h64 ^ (b as u64)).wrapping_mul(0x0000_0100_0000_01b3);
                    }
                    let mix = h64
                        .wrapping_add(group_hash << 6)
                        .wrapping_add(group_hash >> 2)
                        .wrapping_add(0x9e37_79b9);
                    group_hash ^= mix;

                    exe::call_va!(0x1_4244_34d0, (mf_ptr, buf_150_ptr, 0x44u64));
                    wr::<[u8; 0x30]>(dst_1b0, rd::<[u8; 0x30]>(buf_150_ptr));

                    let mut quat: [u32; 4] = [0; 4];
                    exe::call_va!(
                        0x1_4027_5010,
                        (quat.as_mut_ptr() as usize, buf_150_ptr)
                    );
                    wr::<u32>(dst_1c0, quat[1]);
                    wr::<u32>(dst_1c0 + 0x04, quat[2]);
                    wr::<u32>(dst_1c0 + 0x08, quat[3]);
                    wr::<u32>(dst_1c0 + 0x0c, quat[0]);
                    wr::<u32>(dst_1c0 + 0x10, rd::<u32>(buf_150_ptr + 0x24));
                    wr::<u32>(dst_1c0 + 0x14, rd::<u32>(buf_150_ptr + 0x28));
                    wr::<u32>(dst_1c0 + 0x18, rd::<u32>(buf_150_ptr + 0x2c));
                    wr::<u32>(dst_1c0 + 0x1c, 0);

                    wr::<[u8; 0x10]>(dst_1a0, rd::<[u8; 0x10]>(buf_150_ptr + 0x30));

                    str_dst = str_dst.wrapping_add(0x40);
                    dst_1b0 = dst_1b0.wrapping_add(0x30);
                    dst_1c0 = dst_1c0.wrapping_add(0x20);
                    dst_1a0 = dst_1a0.wrapping_add(0x10);
                }

                wr::<u32>(ptr_1e8 + 0x20, group_hash as u32);
            }
            local_568 += 1;
            ptr_1e8 = ptr_1e8.wrapping_add(4);
        }
    }

    if local_554 > 0 {
        let vec_2a0 = param_1 + 0x2a0;
        exe::call_va!(
            0x1_4133_d2e0,
            (
                vec_2a0,
                local_554 as i64 as u64,
                local_568 as u32,
                vec_2a0
            )
        );
        let mut idx_2a0: u32 = 0;
        while (idx_2a0 as i32 as u64) < (rd::<u32>(param_1 + 0x2a8) as u64) {
            let elem = rd::<usize>(vec_2a0).wrapping_add((idx_2a0 as i32 as usize) * 0x70);
            fs_vcall(fs_ptr, 0x40, elem);
            fs_vcall(fs_ptr, 0x40, elem + 0x04);

            exe::call_va!(0x1_40a0_2480, (elem + 0x08, rd::<u32>(elem) as u64));
            mf_read(
                fs_ptr,
                rd::<usize>(elem + 0x08),
                (rd::<u32>(elem + 0x10) as u64).wrapping_mul(0x30),
            );

            exe::call_va!(0x1_406d_cc50, (elem + 0x18, rd::<u32>(elem) as u64));
            mf_read(
                fs_ptr,
                rd::<usize>(elem + 0x18),
                (rd::<u32>(elem + 0x20) as u64) << 2,
            );

            exe::call_va!(0x1_406d_cc50, (elem + 0x28, rd::<u32>(elem) as u64));
            mf_read(
                fs_ptr,
                rd::<usize>(elem + 0x28),
                (rd::<u32>(elem + 0x30) as u64) << 2,
            );

            exe::call_va!(0x1_406d_cc50, (elem + 0x38, rd::<u32>(elem) as u64));
            mf_read(
                fs_ptr,
                rd::<usize>(elem + 0x38),
                (rd::<u32>(elem + 0x40) as u64) << 2,
            );

            exe::call_va!(0x1_40ff_f1e0, (elem + 0x48, rd::<u32>(elem) as u64));
            let bits_bytes = ((rd::<u64>(elem + 0x58).wrapping_add(0x3f) & !0x3fu64) >> 6) << 3;
            mf_read(fs_ptr, rd::<usize>(elem + 0x48), bits_bytes);

            let mut count_60: u64 = 0;
            fs_vcall(fs_ptr, 0x00, (&mut count_60 as *mut u64) as usize);
            exe::call_va!(0x1_406d_cc50, (elem + 0x60, count_60));
            mf_read(
                fs_ptr,
                rd::<usize>(elem + 0x60),
                (rd::<u32>(elem + 0x68) as u64) << 2,
            );

            idx_2a0 = idx_2a0.wrapping_add(1);
        }

        let vec_2b0 = param_1 + 0x2b0;
        let new_len_2b0 = local_res10 as i64 as u64;
        let old_len_2b0 = rd::<u32>(vec_2b0 + 0x08) as u64;
        let old_cap_2b0 = rd::<u32>(vec_2b0 + 0x0c) as u64;
        let old_data_2b0 = rd::<usize>(vec_2b0);
        let old_end_2b0 = old_data_2b0.wrapping_add((old_len_2b0 as usize) * 0x20);
        if new_len_2b0 > old_cap_2b0 {
            let grow = (old_cap_2b0 >> 1).wrapping_add(old_cap_2b0);
            let new_cap = if grow < new_len_2b0 {
                new_len_2b0
            } else {
                grow
            };
            exe::call_va!(0x1_4133_d760, (vec_2b0, new_cap));
            let diff = new_len_2b0.wrapping_sub(old_len_2b0);
            if diff != 0 {
                let cur_len = rd::<u32>(vec_2b0 + 0x08) as usize;
                let mut dst = rd::<usize>(vec_2b0).wrapping_add(cur_len * 0x20);
                for _ in 0..diff {
                    wr::<[u64; 4]>(dst, [0; 4]);
                    dst = dst.wrapping_add(0x20);
                }
            }
            wr::<u32>(vec_2b0 + 0x08, new_len_2b0 as u32);
        } else if new_len_2b0 > old_len_2b0 {
            let diff = new_len_2b0.wrapping_sub(old_len_2b0);
            let mut dst = old_end_2b0;
            for _ in 0..diff {
                wr::<[u64; 4]>(dst, [0; 4]);
                dst = dst.wrapping_add(0x20);
            }
            wr::<u32>(vec_2b0 + 0x08, new_len_2b0 as u32);
        } else if new_len_2b0 != old_len_2b0 {
            let new_end = old_data_2b0.wrapping_add((new_len_2b0 as usize) * 0x20);
            exe::call_va!(0x1_40d4_d710, (old_data_2b0, new_end, old_end_2b0));
            wr::<u32>(vec_2b0 + 0x08, new_len_2b0 as u32);
        }

        if local_res10 > 0 {
            for i in 0..(local_res10 as usize) {
                let elem = rd::<usize>(vec_2b0).wrapping_add(i * 0x20);
                let mut cnt_a: i32 = 0;
                fs_vcall(fs_ptr, 0x48, (&mut cnt_a as *mut i32) as usize);
                let mut fill_a: u32 = u32::MAX;
                exe::call_va!(
                    0x1_412e_b5b0,
                    (
                        elem + 0x10,
                        cnt_a as i64 as u64,
                        (&mut fill_a as *mut u32) as usize
                    )
                );
                if cnt_a > 0 {
                    mf_read(
                        fs_ptr,
                        rd::<usize>(elem + 0x10),
                        (rd::<u32>(elem + 0x18) as u64) << 2,
                    );
                }

                let mut cnt_b: i32 = 0;
                fs_vcall(fs_ptr, 0x48, (&mut cnt_b as *mut i32) as usize);
                let mut fill_b: u32 = u32::MAX;
                exe::call_va!(
                    0x1_412e_b5b0,
                    (
                        elem,
                        cnt_b as i64 as u64,
                        (&mut fill_b as *mut u32) as usize
                    )
                );
                if cnt_b > 0 {
                    mf_read(
                        fs_ptr,
                        rd::<usize>(elem),
                        (rd::<u32>(elem + 0x08) as u64) << 2,
                    );
                }
            }
        }

        let mut local_548: i32 = 0;
        fs_vcall(fs_ptr, 0x48, (&mut local_548 as *mut i32) as usize);
        if local_548 > 0 {
            fs_vcall(fs_ptr, 0x48, param_1 + 0x2c0);
            exe::call_va!(0x1_4028_e490, (param_1 + 0x2c8, local_548 as i64 as u64));
            mf_read(
                fs_ptr,
                rd::<usize>(param_1 + 0x2c8),
                rd::<u32>(param_1 + 0x2d0) as u64,
            );
        }
    }

    fs_vcall(fs_ptr, 0x38, param_1 + 0x244);

    let mut local_544: i32 = 0;
    fs_vcall(fs_ptr, 0x48, (&mut local_544 as *mut i32) as usize);
    wr::<u8>(param_1 + 0x273, local_544 as u8);
    if local_544 > 0 {
        for i in 0..(local_544 as usize) {
            fs_vcall(fs_ptr, 0x38, param_1 + (i + 0x93) * 4);
        }
    }

    fs_vcall(fs_ptr, 0x38, param_1 + 0x248);

    let read_vec3_into = |fs_p: usize, dst_p: usize| {
        let mut v: [u32; 3] = [0; 3];
        for item in &mut v {
            fs_vcall(fs_p, 0x38, (item as *mut u32) as usize);
        }
        wr::<u32>(dst_p, v[0]);
        wr::<u32>(dst_p + 0x04, v[1]);
        wr::<u32>(dst_p + 0x08, v[2]);
    };

    read_vec3_into(fs_ptr, param_1 + 0x274);
    let mut f_280: u32 = 0;
    fs_vcall(fs_ptr, 0x38, (&mut f_280 as *mut u32) as usize);
    wr::<u32>(param_1 + 0x280, f_280);

    let mut v_284: [u32; 3] = [0; 3];
    let mut v_290: [u32; 3] = [0; 3];
    for item in &mut v_284 {
        fs_vcall(fs_ptr, 0x38, (item as *mut u32) as usize);
    }
    for item in &mut v_290 {
        fs_vcall(fs_ptr, 0x38, (item as *mut u32) as usize);
    }
    wr::<[u32; 3]>(param_1 + 0x284, v_284);
    wr::<[u32; 3]>(param_1 + 0x290, v_290);

    if local_res10 > 0 {
        for i in 0..(local_res10 as usize) {
            let dst = param_1 + 0x300 + i * 0x10;
            read_vec3_into(fs_ptr, dst);
            let mut w: u32 = 0;
            fs_vcall(fs_ptr, 0x38, (&mut w as *mut u32) as usize);
            wr::<u32>(dst + 0x0c, w);
        }
    }

    if rd::<u32>(param_1 + 0x2a8) != 0
        && rd::<u64>(param_1 + 0x228).wrapping_sub(1) < 0xffff_ffff_ffff_fffe
    {
        read_vec3_into(fs_ptr, param_1 + 0x2d8);
        let mut f_2e4: u32 = 0;
        fs_vcall(fs_ptr, 0x38, (&mut f_2e4 as *mut u32) as usize);
        wr::<u32>(param_1 + 0x2e4, f_2e4);

        let mut v_2e8: [u32; 3] = [0; 3];
        let mut v_2f4: [u32; 3] = [0; 3];
        for item in &mut v_2e8 {
            fs_vcall(fs_ptr, 0x38, (item as *mut u32) as usize);
        }
        for item in &mut v_2f4 {
            fs_vcall(fs_ptr, 0x38, (item as *mut u32) as usize);
        }
        wr::<[u32; 3]>(param_1 + 0x2e8, v_2e8);
        wr::<[u32; 3]>(param_1 + 0x2f4, v_2f4);
    }

    let mut local_564: i32 = 0;
    fs_vcall(fs_ptr, 0x48, (&mut local_564 as *mut i32) as usize);
    let need_150 = local_564 as i64 as u64;
    let cap_150 = rd::<u32>(param_1 + 0x15c) as u64;
    if need_150 > cap_150 {
        let grow = (cap_150 >> 1).wrapping_add(cap_150);
        let new_cap = if grow < need_150 { need_150 } else { grow };
        exe::call_va!(0x1_401d_eba0, (param_1 + 0x150, new_cap));
    }

    let g_2b8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82b8) as usize);
    let type_900 = exe::ptr::<u8>(0x1_43bb_d900) as usize;
    if local_564 > 0 {
        for _ in 0..local_564 {
            let mut id_val: u64 = 0;
            fs_vcall(fs_ptr, 0x00, (&mut id_val as *mut u64) as usize);
            let mut res_obj = exe::call_va!(
                0x1_4242_1ed0,
                (g_2b8, type_900, id_val, 0u64, 1u8) -> usize
            );
            exe::call_va!(
                0x1_403d_8aa0,
                (param_1 + 0x150, (&mut res_obj as *mut usize) as usize)
            );
        }
    }

    let mut local_res18: i32 = 0;
    fs_vcall(fs_ptr, 0x48, (&mut local_res18 as *mut i32) as usize);
    let need_160 = local_res18 as i64 as u64;
    let cap_160 = rd::<u32>(param_1 + 0x16c) as u64;
    if need_160 > cap_160 {
        let grow = (cap_160 >> 1).wrapping_add(cap_160);
        let new_cap = if grow < need_160 { need_160 } else { grow };
        exe::call_va!(0x1_401d_eba0, (param_1 + 0x160, new_cap));
    }

    if local_res18 > 0 {
        for _ in 0..local_res18 {
            let mut idx_560: i32 = 0;
            fs_vcall(fs_ptr, 0x48, (&mut idx_560 as *mut i32) as usize);
            let src_elem = rd::<usize>(param_1 + 0x150)
                .wrapping_add(((idx_560 as isize) * 8) as usize);
            exe::call_va!(0x1_403d_8aa0, (param_1 + 0x160, src_elem));
        }
    }

    let mut cnt_170: i32 = 0;
    fs_vcall(fs_ptr, 0x48, (&mut cnt_170 as *mut i32) as usize);
    exe::call_va!(
        0x1_4133_d460,
        (
            param_1 + 0x170,
            cnt_170 as i64 as u64,
            0u64,
            param_1 + 0x170
        )
    );

    if cnt_170 > 0 {
        let mut name_buf = [0u8; 544];
        let name_ptr = name_buf.as_mut_ptr() as usize;
        for i in 0..(cnt_170 as usize) {
            let mut nlen: i32 = 0;
            fs_vcall(fs_ptr, 0x48, (&mut nlen as *mut i32) as usize);
            mf_read(fs_ptr, name_ptr, nlen as i64 as u64);

            let mut tmp_vec_a: [u64; 2] = [0; 2];
            let mut alloc_ptr: usize = 0;
            let mut vec_len_u32: u32 = 0;
            let cnt_u64 = local_res18 as i64 as u64;
            if local_res18 != 0 {
                alloc_ptr = exe::call_va!(0x1_401d_ec00, (cnt_u64, cnt_u64) -> usize);
                let _ = exe::call_va!(
                    0x1_4183_acc2,
                    (alloc_ptr, 0u32, cnt_u64.wrapping_mul(8)) -> usize
                );
                vec_len_u32 = local_res18 as u32;
            }

            if local_res18 > 0 {
                for j in 0..(local_res18 as usize) {
                    let mut idx_560: i32 = 0;
                    fs_vcall(fs_ptr, 0x48, (&mut idx_560 as *mut i32) as usize);
                    let val = rd::<u64>(
                        rd::<usize>(param_1 + 0x150)
                            .wrapping_add(((idx_560 as isize) * 8) as usize),
                    );
                    wr::<u64>(alloc_ptr.wrapping_add(j * 8), val);
                }
            }

            let mut nul_off = 0usize;
            while rd::<u8>(name_ptr.wrapping_add(nul_off)) != 0 {
                nul_off += 1;
            }
            let hash = if nul_off == 0 {
                0u32
            } else {
                let mut h: u32 = 0x811c_9dc5;
                for k in 0..nul_off {
                    let ch = rd::<i8>(name_ptr.wrapping_add(k)) as i32 as u32;
                    h = ((ch | 0x20) ^ h).wrapping_mul(0x0100_0193);
                }
                h
            };

            let mut tmp_vec_b: [u64; 2] = [0; 2];
            let tvb_ptr = tmp_vec_b.as_mut_ptr() as usize;
            let tva_ptr = tmp_vec_a.as_mut_ptr() as usize;
            wr::<usize>(tvb_ptr, alloc_ptr);
            wr::<u32>(tvb_ptr + 0x08, vec_len_u32);
            wr::<u32>(tvb_ptr + 0x0c, vec_len_u32);

            let slot = rd::<usize>(param_1 + 0x170).wrapping_add(i * 0x18);
            wr::<u32>(slot, hash);
            let dst_vec = slot + 0x08;
            if dst_vec != tvb_ptr {
                exe::call_va!(0x1_401d_e830, (dst_vec));
                wr::<usize>(dst_vec, alloc_ptr);
                wr::<u32>(dst_vec + 0x08, vec_len_u32);
                wr::<u32>(dst_vec + 0x0c, vec_len_u32);
                wr::<u64>(tvb_ptr, 0);
                wr::<u64>(tvb_ptr + 0x08, 0);
            }
            exe::call_va!(0x1_401d_e830, (tvb_ptr));
            exe::call_va!(0x1_401d_e830, (tva_ptr));
        }
    }

    let mut ctx_1d0: [usize; 2] = [fs_ptr, param_1];
    let mut local_55c: i32 = 0;
    fs_vcall(fs_ptr, 0x48, (&mut local_55c as *mut i32) as usize);

    let vec_130 = param_1 + 0x130;
    let need_130 = local_55c as i64 as u64;
    let cap_130 = rd::<u32>(vec_130 + 0x0c) as u64;
    if need_130 > cap_130 {
        let grow = (cap_130 >> 1).wrapping_add(cap_130);
        let new_cap = if grow < need_130 { need_130 } else { grow };
        exe::call_va!(0x1_4133_da90, (vec_130, new_cap));
    }

    if local_55c > 0 {
        for _ in 0..local_55c {
            let mut out_1c0: [u64; 2] = [0; 2];
            let ret_ptr = exe::call_va!(
                0x1_4232_a5c0,
                (ctx_1d0.as_mut_ptr() as usize, out_1c0.as_mut_ptr() as usize) -> usize
            );
            let cur_len = rd::<u32>(vec_130 + 0x08) as u64;
            let cur_cap = rd::<u32>(vec_130 + 0x0c) as u64;
            if cur_len.wrapping_mul(8) == cur_cap.wrapping_mul(8) {
                let need1 = cur_len.wrapping_add(1);
                let grow1 = (cur_cap >> 1).wrapping_add(cur_cap);
                let new_cap1 = if grow1 < need1 { need1 } else { grow1 };
                exe::call_va!(0x1_4133_da90, (vec_130, new_cap1));
            }
            let idx = rd::<u32>(vec_130 + 0x08) as usize;
            let arr = rd::<usize>(vec_130);
            let val = rd::<u64>(ret_ptr);
            wr::<u64>(ret_ptr, 0);
            wr::<u64>(arr.wrapping_add(idx * 8), val);
            wr::<u32>(vec_130 + 0x08, rd::<u32>(vec_130 + 0x08).wrapping_add(1));

            let leftover = out_1c0[0] as usize;
            if leftover != 0 {
                let id_d8 = rd::<u32>(leftover + 0xd8);
                out_1c0[1] = 0x64;
                let spin_ptr = exe::ptr::<u8>(0x1_43a2_8d37) as usize;
                let spin_atom = &*(spin_ptr as *const core::sync::atomic::AtomicU8);
                loop {
                    while spin_atom.load(core::sync::atomic::Ordering::Relaxed) != 0 {
                        core::hint::spin_loop();
                    }
                    if spin_atom.swap(1, core::sync::atomic::Ordering::Acquire) == 0 {
                        break;
                    }
                    exe::call_va!(0x1_4241_7470, ((&mut out_1c0[1] as *mut u64) as usize));
                }

                let ring_base = exe::ptr::<u8>(0x1_43bb_d448) as usize;
                let r_cnt = rd::<u32>(ring_base + 0x10);
                let r_cap = rd::<u32>(ring_base + 0x14);
                if r_cnt == r_cap {
                    exe::call_va!(0x1_412e_bcc0, (ring_base, r_cnt.wrapping_add(1)));
                }
                let tail = rd::<u32>(ring_base + 0x0c);
                let r_buf = rd::<usize>(ring_base);
                wr::<u32>(r_buf.wrapping_add((tail as usize) * 4), id_d8);
                let next_tail = tail.wrapping_add(1);
                wr::<u32>(
                    ring_base + 0x0c,
                    if next_tail == rd::<u32>(ring_base + 0x14) {
                        0
                    } else {
                        next_tail
                    },
                );
                wr::<u32>(ring_base + 0x10, rd::<u32>(ring_base + 0x10).wrapping_add(1));
                spin_atom.store(0, core::sync::atomic::Ordering::Release);

                exe::call_va!(0x1_4130_b160, (leftover + 0x150));
                exe::call_va!(0x1_4204_db80, (leftover + 0x74));
                exe::call_va!(0x1_4204_db80, (leftover + 0x70));
                exe::call_va!(0x1_4204_db80, (leftover + 0x6c));
                exe::call_va!(0x1_4204_db80, (leftover + 0x68));
                exe::call_va!(0x1_4244_2de0, (leftover));
            }
        }
    }

    let mut val_140: u32 = 0;
    fs_vcall(fs_ptr, 0x40, (&mut val_140 as *mut u32) as usize);
    wr::<u32>(param_1 + 0x140, val_140);

    let g_c58 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    if rd::<u8>(g_c58 + 0xac) != 0 && rd::<u8>(param_1 + 0x272) != 0 {
        let rbx_cnt = ((rd::<u32>(param_1 + 0x178) as u64).wrapping_add(1))
            .wrapping_mul(rd::<u8>(param_1 + 0x270) as u64);
        let vec_400 = param_1 + 0x400;
        let old_len = rd::<u32>(vec_400 + 0x08) as u64;
        let old_cap = rd::<u32>(vec_400 + 0x0c) as u64;
        if rbx_cnt > old_cap {
            let grow = (old_cap >> 1).wrapping_add(old_cap);
            let new_cap = if grow < rbx_cnt { rbx_cnt } else { grow };
            exe::call_va!(0x1_408c_e8a0, (vec_400, new_cap));
            let base_len = rd::<u32>(vec_400 + 0x08) as usize;
            let mut dst = rd::<usize>(vec_400).wrapping_add(base_len * 0x18);
            let diff = rbx_cnt.wrapping_sub(old_len);
            for _ in 0..diff {
                wr::<[u64; 3]>(dst, [0; 3]);
                dst = dst.wrapping_add(0x18);
            }
            wr::<u32>(vec_400 + 0x08, rbx_cnt as u32);
        } else if rbx_cnt > old_len {
            let mut dst = rd::<usize>(vec_400).wrapping_add((old_len as usize) * 0x18);
            let diff = rbx_cnt.wrapping_sub(old_len);
            for _ in 0..diff {
                wr::<[u64; 3]>(dst, [0; 3]);
                dst = dst.wrapping_add(0x18);
            }
            wr::<u32>(vec_400 + 0x08, rbx_cnt as u32);
        } else if rbx_cnt != old_len {
            wr::<u32>(vec_400 + 0x08, rbx_cnt as u32);
        }
        exe::call_va!(0x1_40ff_f1e0, (param_1 + 0x410, rbx_cnt));
    }

    exe::call_va!(0x1_4240_ab10, (mf_ptr + 0x0c));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_rend_14231e750(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static CALL_LOG: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_cc50(vec: usize, new_len: u64) {
        wr::<u32>(vec + 0x08, new_len as u32);
        CALL_LOG.fetch_add(new_len.wrapping_add(0xcc50), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_b5b0(vec: usize, new_len: u64, _tag: usize) {
        wr::<u32>(vec + 0x08, new_len as u32);
        CALL_LOG.fetch_add(new_len.wrapping_add(0xb5b0), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_278_get_cap(this: usize) -> u64 {
        rd::<u64>(this + 0x08)
    }
    unsafe extern "win64" fn mock_278_set_cap(this: usize, new_cap: u64) {
        wr::<u64>(this + 0x08, new_cap);
        CALL_LOG.fetch_add(new_cap.wrapping_add(0x2780), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_348_resize(this: usize, total_bytes: u64, _tag: usize) {
        wr::<u64>(this + 0x08, total_bytes);
        CALL_LOG.fetch_add(total_bytes.wrapping_add(0x3480), Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_406d_cc50, mock_cc50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_b5b0, mock_b5b0 as *const ())?;

    let run = |f: F, free_cnt: u32, req_cnt: u64| -> (Vec<u32>, Vec<u32>, Vec<u32>, u64, u64, u64) {
        CALL_LOG.store(0, Ordering::Relaxed);

        let mut vtbl_278 = [0usize; 8];
        vtbl_278[0x28 / 8] = mock_278_set_cap as *const () as usize;
        vtbl_278[0x38 / 8] = mock_278_get_cap as *const () as usize;

        let mut vtbl_348 = [0usize; 6];
        vtbl_348[0x20 / 8] = mock_348_resize as *const () as usize;

        let mut free_arr: [u32; 4] = [10, 11, 12, 13];
        let mut arr_238 = vec![0u32; 16];
        let mut arr_310 = vec![0u32; 32];
        let mut bits_320: [u64; 1] = [0];

        let mut p1 = vec![0u8; 0x360];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x20, 16);
        wr::<usize>(p1p + 0x238, arr_238.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x240, 4);
        wr::<usize>(p1p + 0x258, free_arr.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x260, free_cnt);
        wr::<usize>(p1p + 0x278, vtbl_278.as_ptr() as usize);
        wr::<u64>(p1p + 0x280, 20);
        wr::<usize>(p1p + 0x310, arr_310.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x320, bits_320.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x348, vtbl_348.as_ptr() as usize);

        let mut in_items = [0u32; 16]; // 4 items * 4 u32s (0x10 bytes each)
        in_items[0] = 2;
        in_items[4] = 5;
        in_items[8] = 7;

        let mut out_slots = vec![0u32; 4];
        f(
            p1p,
            in_items.as_ptr() as usize,
            req_cnt,
            out_slots.as_mut_ptr() as usize,
        );

        (
            out_slots,
            arr_238,
            arr_310,
            bits_320[0],
            rd::<u64>(p1p + 0x280),
            CALL_LOG.load(Ordering::Relaxed),
        )
    };

    for (free_cnt, req_cnt) in [(2u32, 3u64), (3, 2)] {
        let o = run(f_o, free_cnt, req_cnt);
        let n = run(f_n, free_cnt, req_cnt);
        crate::diff_eq!(o, n, "14231e750 free={} req={}", free_cnt, req_cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14232c7a0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static CALL_LOG: AtomicU64 = AtomicU64::new(0);
    static MF_VTBL_PTR: AtomicUsize = AtomicUsize::new(0);
    static SCRATCH_PTR: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "win64" fn mock_mf_read(mf: usize, dst: usize, len: usize) {
        let base = rd::<usize>(mf + 0x118);
        let pos = rd::<usize>(mf + 0x128);
        for i in 0..len {
            wr::<u8>(dst + i, rd::<u8>(base + pos + i));
        }
        wr::<usize>(mf + 0x128, pos + len);
    }
    unsafe extern "win64" fn mock_36a0(mf: usize, size: u64, data_ptr: u64, _flag: u32) {
        wr::<usize>(mf, MF_VTBL_PTR.load(Ordering::Relaxed));
        wr::<u32>(mf + 0x0c, (4u32 << 24) | 3);
        wr::<u64>(mf + 0x14, 0x6162_6300);
        wr::<u64>(mf + 0x118, data_ptr);
        wr::<u64>(mf + 0x120, size);
        wr::<u64>(mf + 0x128, 0);
    }
    unsafe extern "win64" fn mock_9e30(_p1: usize, total: u32) {
        CALL_LOG.fetch_add((total as u64).wrapping_add(0x9e30), Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_5010(quat_out: usize, mat_in: usize) {
        wr::<u32>(quat_out, rd::<u32>(mat_in).wrapping_add(1));
        wr::<u32>(quat_out + 0x04, rd::<u32>(mat_in + 0x04).wrapping_add(2));
        wr::<u32>(quat_out + 0x08, rd::<u32>(mat_in + 0x08).wrapping_add(3));
        wr::<u32>(quat_out + 0x0c, rd::<u32>(mat_in + 0x0c).wrapping_add(4));
    }
    unsafe extern "win64" fn mock_d2e0(vec: usize, cnt: u64, _r8: u32, _r9: usize) {
        wr::<u32>(vec + 0x08, cnt as u32);
    }
    unsafe extern "win64" fn mock_vec_init(vec: usize, cnt: u64) {
        wr::<usize>(vec, SCRATCH_PTR.load(Ordering::Relaxed));
        wr::<u32>(vec + 0x08, cnt as u32);
    }
    unsafe extern "win64" fn mock_bits_init(bits: usize, cnt: u64) {
        wr::<usize>(bits, SCRATCH_PTR.load(Ordering::Relaxed));
        wr::<u64>(bits + 0x10, cnt);
    }
    unsafe extern "win64" fn mock_b5b0(vec: usize, cnt: u64, _fill_ptr: usize) {
        wr::<usize>(vec, SCRATCH_PTR.load(Ordering::Relaxed));
        wr::<u32>(vec + 0x08, cnt as u32);
    }
    unsafe extern "win64" fn mock_e490(vec: usize, cnt: u64) {
        wr::<usize>(vec, SCRATCH_PTR.load(Ordering::Relaxed));
        wr::<u32>(vec + 0x08, cnt as u32);
    }
    unsafe extern "win64" fn mock_eba0(vec: usize, new_cap: u64) {
        wr::<u32>(vec + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_1ed0(
        _g: usize,
        _ty: usize,
        id: u64,
        _r9: u64,
        flag: u8,
    ) -> usize {
        (id as usize).wrapping_add(flag as usize)
    }
    unsafe extern "win64" fn mock_8aa0(vec: usize, item_ptr: usize) {
        let idx = rd::<u32>(vec + 0x08) as usize;
        let base = rd::<usize>(vec);
        wr::<u64>(base + idx * 8, rd::<u64>(item_ptr));
        wr::<u32>(vec + 0x08, (idx + 1) as u32);
    }
    unsafe extern "win64" fn mock_d460(vec: usize, cnt: u64, _r8: u64, _r9: usize) {
        wr::<u32>(vec + 0x08, cnt as u32);
    }
    unsafe extern "win64" fn mock_ec00(_rcx: u64, _rdx: u64) -> usize {
        SCRATCH_PTR.load(Ordering::Relaxed) + 0x200
    }
    unsafe extern "win64" fn mock_e830(vec: usize) {
        CALL_LOG.fetch_add(rd::<u32>(vec + 0x08) as u64, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_da90(vec: usize, new_cap: u64) {
        wr::<u32>(vec + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_a5c0(_ctx: usize, out_1c0: usize) -> usize {
        wr::<u64>(out_1c0, 0x7788_99aa);
        out_1c0
    }
    unsafe extern "win64" fn mock_ab10(_str_ptr: usize) {
        CALL_LOG.fetch_add(0xab10, Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4244_36a0, mock_36a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4244_34d0, mock_mf_read as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4232_9e30, mock_9e30 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4027_5010, mock_5010 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4133_d2e0, mock_d2e0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40a0_2480, mock_vec_init as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_406d_cc50, mock_vec_init as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_40ff_f1e0, mock_bits_init as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_412e_b5b0, mock_b5b0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4028_e490, mock_e490 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_401d_eba0, mock_eba0 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_4242_1ed0, mock_1ed0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_403d_8aa0, mock_8aa0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4133_d460, mock_d460 as *const ())?;
    let _m15 = crate::difftest::Mock::new(0x1_401d_ec00, mock_ec00 as *const ())?;
    let _m16 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;
    let _m17 = crate::difftest::Mock::new(0x1_4133_da90, mock_da90 as *const ())?;
    let _m18 = crate::difftest::Mock::new(0x1_4232_a5c0, mock_a5c0 as *const ())?;
    let _m19 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let mut stream = Vec::<u8>::new();
    let push_u32 = |s: &mut Vec<u8>, v: u32| s.extend_from_slice(&v.to_le_bytes());
    let push_u64 = |s: &mut Vec<u8>, v: u64| s.extend_from_slice(&v.to_le_bytes());

    push_u32(&mut stream, 0x4e); // ver
    push_u32(&mut stream, 1); // local_res10
    push_u32(&mut stream, 1); // local_554
    push_u32(&mut stream, 1); // local_res20
    push_u32(&mut stream, 1); // param_1 + 0x1c8[0] = 1 item in group 0

    // Group 0, item 0:
    push_u32(&mut stream, 4); // slen
    stream.extend_from_slice(b"Abc\0");
    for k in 0..17u32 {
        push_u32(&mut stream, 0x100 + k); // 0x44 bytes for buf_150
    }

    // local_554 = 1 element in param_1 + 0x2a0:
    push_u32(&mut stream, 1); // elem[0]
    push_u32(&mut stream, 2); // elem[1]
    stream.extend_from_slice(&[0x11u8; 0x30]); // elem + 0x08 (1 * 0x30)
    push_u32(&mut stream, 0x22); // elem + 0x18 (1 * 4)
    push_u32(&mut stream, 0x33); // elem + 0x28 (1 * 4)
    push_u32(&mut stream, 0x44); // elem + 0x38 (1 * 4)
    push_u64(&mut stream, 0x55); // elem + 0x48 (8 bytes)
    push_u64(&mut stream, 1); // count_60
    push_u32(&mut stream, 0x66); // elem + 0x60 (1 * 4)

    // local_res10 = 1 element in param_1 + 0x2b0:
    push_u32(&mut stream, 1); // cnt_a
    push_u32(&mut stream, 0x77);
    push_u32(&mut stream, 1); // cnt_b
    push_u32(&mut stream, 0x88);

    // local_548:
    push_u32(&mut stream, 4);
    push_u32(&mut stream, 9); // param_1 + 0x2c0
    push_u32(&mut stream, 0x99aa_bbcc);

    // floats & bounds:
    push_u32(&mut stream, 0x3f80_0000); // +0x244
    push_u32(&mut stream, 2); // local_544 = 2
    push_u32(&mut stream, 10);
    push_u32(&mut stream, 20);
    push_u32(&mut stream, 30); // +0x248
    for k in 0..(3 + 1 + 6 + 4 + 10) {
        push_u32(&mut stream, 0x200 + k);
    }

    // local_564 = 1:
    push_u32(&mut stream, 1);
    push_u64(&mut stream, 0x5000);

    // local_res18 = 1:
    push_u32(&mut stream, 1);
    push_u32(&mut stream, 0); // idx_560

    // cnt_170 = 1:
    push_u32(&mut stream, 1);
    push_u32(&mut stream, 4); // nlen
    stream.extend_from_slice(b"Xyz\0");
    push_u32(&mut stream, 0); // idx_560

    // local_55c = 1:
    push_u32(&mut stream, 1);
    // val_140:
    push_u32(&mut stream, 0x4242);

    let g_c58_slot = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_c58 = rd::<usize>(g_c58_slot);
    let mut fake_c58 = [0u8; 0xc0];
    fake_c58[0xac] = 1;
    wr::<usize>(g_c58_slot, fake_c58.as_mut_ptr() as usize);

    let run = |f: F| -> (Vec<u8>, u32, [u32; 8], u32, u64, u64) {
        CALL_LOG.store(0, Ordering::Relaxed);
        let mut mf_vtbl = [0usize; 8];
        mf_vtbl[0x28 / 8] = mock_mf_read as *const () as usize;
        MF_VTBL_PTR.store(mf_vtbl.as_ptr() as usize, Ordering::Relaxed);

        let mut scratch = vec![0u8; 0x400];
        SCRATCH_PTR.store(scratch.as_mut_ptr() as usize, Ordering::Relaxed);

        let mut arr_130 = [0u64; 4];
        let mut arr_150 = [0u64; 4];
        let mut arr_160 = [0u64; 4];
        let mut arr_170 = [0u64; 6];
        let mut arr_180 = [0u8; 0x80];
        let mut arr_190 = [0u32; 4];
        let mut arr_1a0 = [0u32; 8];
        let mut arr_1b0 = [0u32; 12];
        let mut arr_1c0 = [0u32; 8];
        let mut arr_2a0 = [0u8; 0x80];
        let mut arr_2b0 = [0u64; 8];
        let mut arr_400 = [0xffu64; 12];

        let mut p1 = vec![0u8; 0x440];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x130, arr_130.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x13c, 4);
        wr::<usize>(p1p + 0x150, arr_150.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x15c, 4);
        wr::<usize>(p1p + 0x160, arr_160.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x16c, 4);
        wr::<usize>(p1p + 0x170, arr_170.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x180, arr_180.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x190, arr_190.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x1a0, arr_1a0.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x1b0, arr_1b0.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x1c0, arr_1c0.as_mut_ptr() as usize);
        wr::<u64>(p1p + 0x228, 5);
        wr::<usize>(p1p + 0x2a0, arr_2a0.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x2b0, arr_2b0.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x2bc, 2);
        wr::<usize>(p1p + 0x400, arr_400.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0x40c, 4);

        let p2: [u64; 2] = [stream.as_ptr() as u64, stream.len() as u64];
        f(p1p, p2.as_ptr() as usize);

        // Zero pointer fields in p1 before byte comparison
        for off in [
            0x130usize, 0x150, 0x160, 0x170, 0x180, 0x190, 0x1a0, 0x1b0, 0x1c0, 0x2a0, 0x2b0,
            0x2c8, 0x400, 0x410,
        ] {
            wr::<usize>(p1p + off, 0);
        }

        (
            p1,
            arr_190[0],
            arr_1c0,
            rd::<u32>(arr_170.as_ptr() as usize),
            arr_130[0],
            CALL_LOG.load(Ordering::Relaxed),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_c58_slot, saved_c58);
    crate::diff_eq!(o, n, "14232c7a0 full");
    Ok(1)
}

// =============================================================================
// Registry lines (for reference)
// =============================================================================
// Replacement::verified_throws(0x1_4231_e750, "rend_14231e750", rend::cauldron_hot_z9a::rend_14231e750 as *const (), Throws::Discard, rend::cauldron_hot_z9a::diff_rend_14231e750),
// Replacement::verified_throws(0x1_4232_c7a0, "unknown_14232c7a0", rend::cauldron_hot_z9a::unknown_14232c7a0 as *const (), Throws::Caught, rend::cauldron_hot_z9a::diff_unknown_14232c7a0),
