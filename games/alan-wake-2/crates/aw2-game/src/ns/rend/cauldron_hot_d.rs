//! Decompiled hot cauldron / ECS / streaming culling functions:
//! - `0x141e5a750` (`unknown_141e5a750`)
//! - `0x141e71db0` (`coregame_141e71db0`)
//! - `0x141eaf8a0` (`unknown_141eaf8a0`)

use crate::exe::{self, rd, wr};

unsafe fn count_archetype_entities(param: usize) -> u64 {
    let mut total = 0u64;
    let base = rd::<usize>(param);
    let r11 = base.wrapping_add(0x40);
    let mut span = rd::<[u64; 2]>(param + 0x08);
    let mut iter_buf = [0u64; 3];
    let iter_ptr = exe::call_va!(
        0x1_401e_7c10,
        (
            iter_buf.as_mut_ptr() as usize,
            r11,
            span.as_mut_ptr() as usize
        ) -> usize
    );
    let iter_base = rd::<usize>(iter_ptr);
    let mut cur_u16 = rd::<usize>(iter_ptr + 0x08);
    let end_u16 = rd::<usize>(iter_ptr + 0x10);
    while cur_u16 != end_u16 {
        let arch_id = rd::<u16>(cur_u16) as usize;
        let n = if rd::<u64>(base.wrapping_add(arch_id * 8).wrapping_add(0x88)) != 0 {
            rd::<u32>(base.wrapping_add(arch_id * 4).wrapping_add(0x10088)) as u64
        } else {
            0
        };
        total = total.wrapping_add(n);
        cur_u16 = cur_u16.wrapping_add(2);
        while cur_u16 != end_u16 {
            let next_id = rd::<u16>(cur_u16) as usize;
            if rd::<u64>(iter_base.wrapping_add(next_id * 8).wrapping_add(0x48)) != 0
                && rd::<u32>(iter_base.wrapping_add(next_id * 4).wrapping_add(0x10048)) != 0
            {
                break;
            }
            cur_u16 = cur_u16.wrapping_add(2);
        }
    }
    total
}

unsafe fn init_archetype_outer_9(param: usize, outer: &mut [u64; 9]) {
    let r11 = rd::<usize>(param).wrapping_add(0x40);
    let mut span = rd::<[u64; 2]>(param + 0x08);
    let mut init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            init.as_mut_ptr() as usize,
            r11,
            span.as_mut_ptr() as usize
        ) -> usize
    );
    outer[0] = r11 as u64;
    outer[1] = init[0];
    outer[2] = init[1];
    outer[3] = init[2];
    outer[4] = 0;
    outer[5] = 0;
    outer[6] = 0;
    outer[7] = 0;
    outer[8] = 0;
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
    exe::call_va!(0x1_40fb_7280, (outer.as_mut_ptr() as usize));
}

unsafe fn scan_archetype_for_handle(
    param: usize,
    want_handle: u64,
    vec_178: &mut [u64; 2],
    r15_len: &mut u32,
) {
    let mut outer = [0u64; 9];
    init_archetype_outer_9(param, &mut outer);
    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let idx = outer[6] as usize;
            let flags_ptr = outer[7] as usize;
            let ids_ptr = outer[8] as usize;
            let id_slot = ids_ptr.wrapping_add(idx * 8);
            if rd::<u64>(id_slot) == want_handle {
                wr::<u64>(id_slot, 0);
                if rd::<u8>(flags_ptr.wrapping_add(idx)) != 0 {
                    let chunk_ptr = outer[4] as usize;
                    let mut val = rd::<u64>(chunk_ptr.wrapping_add(idx * 8).wrapping_add(0x10));
                    exe::call_va!(
                        0x1_40fa_5ca0,
                        (
                            vec_178.as_mut_ptr() as usize,
                            (&mut val as *mut u64) as usize
                        )
                    );
                    *r15_len = vec_178[1] as u32;
                }
                let lim = rd::<i32>(exe::ptr::<i32>(0x1_4385_0be8) as usize) as i64 as u64;
                if (*r15_len as u64) >= lim {
                    return;
                }
            }
            outer[6] = (idx + 1) as u64;
        }
        let iter_sub = (outer.as_mut_ptr() as usize).wrapping_add(8);
        exe::call_va!(0x1_401e_7c70, (iter_sub) -> usize);
        if outer[2] != outer[3] {
            let base_0 = outer[0] as usize;
            let arch_id = rd::<u16>(outer[2] as usize) as usize;
            let cp = rd::<u64>(base_0.wrapping_add(arch_id * 8).wrapping_add(0x48));
            outer[4] = cp;
            outer[5] = if cp != 0 {
                rd::<u32>(base_0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
            } else {
                0
            };
            outer[6] = 0;
        }
        exe::call_va!(0x1_40fb_7280, (outer.as_mut_ptr() as usize));
    }
}

/// `unknown_141e5a750`
/// Original: 0x141e5a750 (2852 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141e5a750(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u64,
) {
    let mut p5 = param_5;
    let mut vec_a8 = [0u64; 2];
    let mut cur = rd::<usize>(param_4);
    let end = cur.wrapping_add((rd::<u32>(param_4 + 0x08) as usize) << 6);

    while cur != end {
        if rd::<u32>(cur + 0x20) == 3 {
            let p10 = rd::<usize>(cur + 0x10);
            if p10 == 0 || rd::<u32>(p10 + 0x08) == 1 {
                let item_cnt = rd::<u32>(cur + 0x08) as u64;
                let lim = rd::<i32>(exe::ptr::<i32>(0x1_4385_0be8) as usize) as i64 as u64;
                let mut handled_early = false;
                if item_cnt > lim {
                    let mut vec_178 = [0u64; 2];
                    let c1 = count_archetype_entities(param_1);
                    let c2 = count_archetype_entities(param_2);
                    let total_cap = c1.wrapping_add(c2);
                    if total_cap != 0 {
                        let buf = exe::call_va!(
                            0x1_4241_c0a0,
                            (total_cap.wrapping_mul(8), 4u64) -> u64
                        );
                        vec_178[0] = buf;
                        vec_178[1] = ((total_cap as u32) as u64) << 32;
                    }
                    let want_handle = rd::<u64>(cur + 0x18);
                    let mut r15_len = 0u32;
                    scan_archetype_for_handle(param_1, want_handle, &mut vec_178, &mut r15_len);
                    scan_archetype_for_handle(param_2, want_handle, &mut vec_178, &mut r15_len);

                    let buf_ptr = vec_178[0] as usize;
                    if r15_len != 0 {
                        let span = [r15_len as u64, buf_ptr as u64];
                        exe::call_va!(
                            0x1_40fb_7ed0,
                            ((&mut p5 as *mut u64) as usize, span.as_ptr() as usize)
                        );
                        if buf_ptr != 0 {
                            exe::call_va!(0x1_4241_c190, (buf_ptr));
                        }
                        handled_early = true;
                    } else if buf_ptr != 0 {
                        exe::call_va!(0x1_4241_c190, (buf_ptr));
                    }
                }
                if !handled_early {
                    wr::<u32>(cur + 0x20, 4);
                    exe::call_va!(
                        0x1_4046_6a90,
                        (vec_a8.as_mut_ptr() as usize, cur.wrapping_add(0x18))
                    );
                }
            }
        }
        cur = cur.wrapping_add(0x40);
    }

    let a8_ptr = vec_a8[0] as usize;
    let a8_len = (vec_a8[1] as u32) as usize;
    let a8_end = a8_ptr.wrapping_add(a8_len * 8);

    let count3 = count_archetype_entities(param_3) as usize;
    let mut vec1_ptr = 0usize;
    let mut vec1_len = 0usize;
    let mut vec1_cap = 0usize;
    let mut vec2_ptr = 0usize;
    let mut vec2_len = 0usize;
    let mut vec2_cap = 0usize;
    if count3 != 0 {
        vec1_ptr = exe::call_va!(0x1_4241_c0a0, ((count3 * 8) as u64, 4u64) -> usize);
        vec1_cap = count3;
        vec2_ptr = exe::call_va!(0x1_4241_c0a0, (count3 as u64, 1u64) -> usize);
        vec2_cap = count3;
    }

    let mut outer3 = [0u64; 9];
    init_archetype_outer_9(param_3, &mut outer3);
    while outer3[6] < outer3[5] {
        while outer3[6] < outer3[5] {
            let idx = outer3[6] as usize;
            let byte_ptr = (outer3[7] as usize).wrapping_add(idx);
            let qword_ptr = (outer3[8] as usize).wrapping_add(idx * 8);
            if a8_ptr != a8_end {
                let target_id = rd::<u64>(qword_ptr);
                let mut scan = a8_ptr;
                while scan != a8_end {
                    if rd::<u64>(scan) == target_id {
                        wr::<u64>(qword_ptr, 0);
                        let val = rd::<u64>((outer3[4] as usize).wrapping_add(idx * 8).wrapping_add(0x10));
                        if vec1_len == vec1_cap {
                            let new_cap = (vec1_cap + (vec1_cap >> 1)).max(vec1_len + 1);
                            let new_ptr = exe::call_va!(
                                0x1_4241_c0a0,
                                ((new_cap * 8) as u64, 4u64) -> usize
                            );
                            exe::call_va!(
                                0x1_4183_acc8,
                                (new_ptr, vec1_ptr, (vec1_len * 8) as u64)
                            );
                            if vec1_ptr != 0 {
                                exe::call_va!(0x1_4241_c190, (vec1_ptr));
                            }
                            vec1_ptr = new_ptr;
                            vec1_cap = new_cap;
                        }
                        wr::<u64>(vec1_ptr.wrapping_add(vec1_len * 8), val);
                        vec1_len += 1;

                        if vec2_len == vec2_cap {
                            let new_cap = (vec2_cap + (vec2_cap >> 1)).max(vec2_len + 1);
                            let new_ptr = exe::call_va!(
                                0x1_4241_c0a0,
                                (new_cap as u64, 1u64) -> usize
                            );
                            exe::call_va!(
                                0x1_4183_acc8,
                                (new_ptr, vec2_ptr, vec2_len as u64)
                            );
                            if vec2_ptr != 0 {
                                exe::call_va!(0x1_4241_c190, (vec2_ptr));
                            }
                            vec2_ptr = new_ptr;
                            vec2_cap = new_cap;
                        }
                        wr::<u8>(vec2_ptr.wrapping_add(vec2_len), rd::<u8>(byte_ptr));
                        vec2_len += 1;
                        break;
                    }
                    scan = scan.wrapping_add(8);
                }
            }
            outer3[6] = (idx + 1) as u64;
        }

        let mut cur_u16 = (outer3[2] as usize).wrapping_add(2);
        outer3[2] = cur_u16 as u64;
        let end_u16 = outer3[3] as usize;
        if cur_u16 != end_u16 {
            let base_1 = outer3[1] as usize;
            while cur_u16 != end_u16 {
                let next_id = rd::<u16>(cur_u16) as usize;
                if rd::<u64>(base_1.wrapping_add(next_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(base_1.wrapping_add(next_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_u16 = cur_u16.wrapping_add(2);
            }
            outer3[2] = cur_u16 as u64;
            if cur_u16 != end_u16 {
                let base_0 = outer3[0] as usize;
                let arch_id = rd::<u16>(cur_u16) as usize;
                let cp = rd::<u64>(base_0.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer3[4] = cp;
                outer3[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer3[6] = 0;
            }
        }
        exe::call_va!(0x1_40fb_7280, (outer3.as_mut_ptr() as usize));
    }

    let mut vec3_ptr = 0usize;
    let mut vec3_len = 0usize;
    let mut vec3_cap;
    if vec1_len != 0 {
        vec3_ptr = exe::call_va!(0x1_4241_c0a0, ((vec1_len * 8) as u64, 4u64) -> usize);
        vec3_cap = vec1_len;
        for i in 0..vec1_len {
            if rd::<u8>(vec2_ptr.wrapping_add(i)) != 0 {
                if vec3_len == vec3_cap {
                    let new_cap = (vec3_cap + (vec3_cap >> 1)).max(vec3_len + 1);
                    let new_ptr = exe::call_va!(
                        0x1_4241_c0a0,
                        ((new_cap * 8) as u64, 4u64) -> usize
                    );
                    exe::call_va!(
                        0x1_4183_acc8,
                        (new_ptr, vec3_ptr, (vec3_len * 8) as u64)
                    );
                    if vec3_ptr != 0 {
                        exe::call_va!(0x1_4241_c190, (vec3_ptr));
                    }
                    vec3_ptr = new_ptr;
                    vec3_cap = new_cap;
                }
                wr::<u64>(
                    vec3_ptr.wrapping_add(vec3_len * 8),
                    rd::<u64>(vec1_ptr.wrapping_add(i * 8)),
                );
                vec3_len += 1;
            }
        }
    }

    let final_span = [vec3_len as u64, vec3_ptr as u64];
    exe::call_va!(
        0x1_40fb_7ed0,
        ((&mut p5 as *mut u64) as usize, final_span.as_ptr() as usize)
    );

    if vec3_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec3_ptr));
    }
    if vec2_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec2_ptr));
    }
    if vec1_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec1_ptr));
    }
    if a8_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (a8_ptr));
    }
}

#[inline]
fn quat_rotate_vec4(q: [f32; 4], v: [f32; 4]) -> [f32; 4] {
    let v_c9 = [v[1], v[2], v[0], v[3]];
    let v_d2 = [v[2], v[0], v[1], v[3]];
    let q_d2 = [q[2], q[0], q[1], q[3]];
    let q_c9 = [q[1], q[2], q[0], q[3]];
    let m1 = [
        v_c9[0] * q_d2[0],
        v_c9[1] * q_d2[1],
        v_c9[2] * q_d2[2],
        v_c9[3] * q_d2[3],
    ];
    let m2 = [
        v_d2[0] * q_c9[0],
        v_d2[1] * q_c9[1],
        v_d2[2] * q_c9[2],
        v_d2[3] * q_c9[3],
    ];
    let cross1 = [
        m2[0] - m1[0],
        m2[1] - m1[1],
        m2[2] - m1[2],
        m2[3] - m1[3],
    ];
    let t = [
        cross1[0] * 2.0f32,
        cross1[1] * 2.0f32,
        cross1[2] * 2.0f32,
        cross1[3] * 0.0f32,
    ];
    let t_c9 = [t[1], t[2], t[0], t[3]];
    let t_d2 = [t[2], t[0], t[1], t[3]];
    let m3 = [
        t_c9[0] * q_d2[0],
        t_c9[1] * q_d2[1],
        t_c9[2] * q_d2[2],
        t_c9[3] * q_d2[3],
    ];
    let m4 = [
        t_d2[0] * q_c9[0],
        t_d2[1] * q_c9[1],
        t_d2[2] * q_c9[2],
        t_d2[3] * q_c9[3],
    ];
    let cross2 = [
        m4[0] - m3[0],
        m4[1] - m3[1],
        m4[2] - m3[2],
        m4[3] - m3[3],
    ];
    let w_term = [
        (q[3] * t[0]) + v[0],
        (q[3] * t[1]) + v[1],
        (q[3] * t[2]) + v[2],
        (q[3] * t[3]) + v[3],
    ];
    [
        w_term[0] + cross2[0],
        w_term[1] + cross2[1],
        w_term[2] + cross2[2],
        w_term[3] + cross2[3],
    ]
}

/// `coregame_141e71db0`
/// Original: 0x141e71db0 (2832 bytes, Throws::Discard)
pub unsafe extern "win64" fn coregame_141e71db0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u64,
) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4385_0ee0) as usize) != 0 {
        return;
    }
    let idx = rd::<usize>(param_1 + 0x30);
    let r15_entry = rd::<usize>(param_1).wrapping_add(idx.wrapping_mul(0xa0));
    let rsp_d0 = rd::<usize>(param_1 + 0x08).wrapping_add(idx.wrapping_mul(8));
    let rsp_c8 = rd::<usize>(param_1 + 0x10).wrapping_add(idx.wrapping_mul(0xc0));
    let r14_slot = rd::<usize>(param_1 + 0x18).wrapping_add(idx.wrapping_mul(0x20));
    if rd::<u8>(r14_slot + 0x18) == 0 {
        return;
    }

    let mut rsp_130 = [0u64; 2];
    let p1_4 = rd::<usize>(param_1 + 0x20);
    if p1_4 != 0 {
        let off = rd::<usize>(param_1 + 0x30).wrapping_mul(0x30);
        rsp_130[0] = rd::<u32>(p1_4.wrapping_add(off).wrapping_add(0x08)) as u64;
        rsp_130[1] = rd::<u64>(p1_4.wrapping_add(off));
    }

    let count_88 = rd::<u32>(r15_entry + 0x88) as u64;
    let init_cap = (count_88 as u32) & 0x3fff_ffff;
    let mut vec_16 = [0u64; 2];
    let mut vec_32 = [0u64; 2];
    if count_88 != 0 {
        let _ = exe::call_va!(0x1_4241_bbe0, () -> usize);
        let p16 = exe::call_va!(0x1_4241_c0a0, (count_88 << 4, 0x10u64) -> u64);
        vec_16[0] = p16;
        vec_16[1] = (init_cap as u64) << 32;
        let _ = exe::call_va!(0x1_4241_bbe0, () -> usize);
        let p32 = exe::call_va!(0x1_4241_c0a0, (count_88 << 5, 0x10u64) -> u64);
        vec_32[0] = p32;
        vec_32[1] = (init_cap as u64) << 32;
    }

    let f_max = f32::from_bits(0x7f7f_ffff);
    let mut min_dist_sq = f_max;

    if count_88 != 0 {
        for i in 0..count_88 as usize {
            let box_base = rd::<usize>(r15_entry + 0x80).wrapping_add(i * 0x18);
            let min_x = rd::<f32>(box_base);
            let min_y = rd::<f32>(box_base + 0x04);
            let min_z = rd::<f32>(box_base + 0x08);
            let max_x = rd::<f32>(box_base + 0x0c);
            let max_y = rd::<f32>(box_base + 0x10);
            let max_z = rd::<f32>(box_base + 0x14);

            let size_vec = if min_x <= max_x || min_y <= max_y || min_z <= max_z {
                [max_x - min_x, max_y - min_y, max_z - min_z, 0.0f32]
            } else {
                [0.0f32; 4]
            };

            let half_min_x = min_x * 0.5f32;
            let half_min_y = 0.5f32 * min_y;
            let half_min_z = 0.5f32 * min_z;

            let mat_ptr = rd::<usize>(r15_entry + 0x90).wrapping_add(i * 0x30);
            let mut quat_wxyz = [0.0f32; 4];
            exe::call_va!(
                0x1_4027_5010,
                (quat_wxyz.as_mut_ptr() as usize, mat_ptr)
            );
            let q = [
                f32::from_bits(quat_wxyz[1].to_bits() ^ 0x8000_0000),
                f32::from_bits(quat_wxyz[2].to_bits() ^ 0x8000_0000),
                f32::from_bits(quat_wxyz[3].to_bits() ^ 0x8000_0000),
                quat_wxyz[0],
            ];
            let neg_pos = [
                f32::from_bits(rd::<u32>(mat_ptr + 0x24) ^ 0x8000_0000),
                f32::from_bits(rd::<u32>(mat_ptr + 0x28) ^ 0x8000_0000),
                f32::from_bits(rd::<u32>(mat_ptr + 0x2c) ^ 0x8000_0000),
                f32::from_bits(0x8000_0000),
            ];
            let rot_pos = quat_rotate_vec4(q, neg_pos);
            let center = [
                (max_x * 0.5f32) + half_min_x,
                (max_y * 0.5f32) + half_min_y,
                (max_z * 0.5f32) + half_min_z,
                0.0f32,
            ];
            let trans = [
                center[0] + rot_pos[0],
                center[1] + rot_pos[1],
                center[2] + rot_pos[2],
                center[3] + rot_pos[3],
            ];
            let item_32 = [q[0], q[1], q[2], q[3], trans[0], trans[1], trans[2], trans[3]];

            let len_32 = vec_32[1] as u32;
            let cap_32 = ((vec_32[1] >> 32) as u32) & 0x3fff_ffff;
            if len_32 == cap_32 {
                let mut end_ptr = (vec_32[0] as usize).wrapping_add((len_32 as usize) << 5);
                let _ = exe::call_va!(
                    0x1_40fd_d2c0,
                    (
                        vec_32.as_mut_ptr() as usize,
                        (&mut end_ptr as *mut usize) as usize,
                        item_32.as_ptr() as usize
                    ) -> usize
                );
            } else {
                let dst = (vec_32[0] as usize).wrapping_add((len_32 as usize) << 5);
                wr::<[f32; 8]>(dst, item_32);
                vec_32[1] = (vec_32[1] & !0xffff_ffff) | ((len_32.wrapping_add(1)) as u64);
            }

            let len_16 = vec_16[1] as u32;
            let cap_16 = ((vec_16[1] >> 32) as u32) & 0x3fff_ffff;
            if len_16 == cap_16 {
                let mut end_ptr = (vec_16[0] as usize).wrapping_add((len_16 as usize) << 4);
                let _ = exe::call_va!(
                    0x1_4024_3c50,
                    (
                        vec_16.as_mut_ptr() as usize,
                        (&mut end_ptr as *mut usize) as usize,
                        size_vec.as_ptr() as usize
                    ) -> usize
                );
            } else {
                let dst = (vec_16[0] as usize).wrapping_add((len_16 as usize) << 4);
                wr::<[f32; 4]>(dst, size_vec);
                vec_16[1] = (vec_16[1] & !0xffff_ffff) | ((len_16.wrapping_add(1)) as u64);
            }
        }

        let pts_ptr = rd::<usize>(param_3 + 0x10);
        let pts_cnt = rd::<u32>(param_3 + 0x18) as usize;
        let pts_end = pts_ptr.wrapping_add(pts_cnt << 5);
        let ptr_32 = vec_32[0] as usize;
        let ptr_16 = vec_16[0] as usize;

        for j in 0..count_88 as usize {
            let q = rd::<[f32; 4]>(ptr_32.wrapping_add(j * 32));
            let trans = rd::<[f32; 4]>(ptr_32.wrapping_add(j * 32 + 16));
            let ext = rd::<[f32; 4]>(ptr_16.wrapping_add(j * 16));
            let half_ext = [
                0.5f32 * ext[0],
                0.5f32 * ext[1],
                0.5f32 * ext[2],
                0.5f32 * ext[3],
            ];
            let neg_half_ext = [
                f32::from_bits(half_ext[0].to_bits() ^ 0x8000_0000),
                f32::from_bits(half_ext[1].to_bits() ^ 0x8000_0000),
                f32::from_bits(half_ext[2].to_bits() ^ 0x8000_0000),
                f32::from_bits(half_ext[3].to_bits() ^ 0x8000_0000),
            ];

            let mut pt_cur = pts_ptr;
            while pt_cur != pts_end {
                let pt = rd::<[f32; 4]>(pt_cur);
                let rot_pt = quat_rotate_vec4(q, pt);
                let local_pt = [
                    rot_pt[0] + trans[0],
                    rot_pt[1] + trans[1],
                    rot_pt[2] + trans[2],
                    rot_pt[3] + trans[3],
                ];
                let mut delta = [0.0f32; 4];
                for k in 0..4 {
                    let c_lo = if local_pt[k] > neg_half_ext[k] {
                        local_pt[k]
                    } else {
                        neg_half_ext[k]
                    };
                    let c_hi = if c_lo < half_ext[k] { c_lo } else { half_ext[k] };
                    delta[k] = local_pt[k] - c_hi;
                }
                let d_sq = (delta[0] * delta[0] + delta[1] * delta[1])
                    + (delta[2] * delta[2] + 0.0f32);
                if !(min_dist_sq < d_sq) {
                    min_dist_sq = d_sq;
                }
                pt_cur = pt_cur.wrapping_add(0x20);
            }
        }
    }

    let thresh_cnt = rd::<u32>(r15_entry + 0x78) as usize;
    let thresh_ptr = rd::<usize>(r15_entry + 0x70);
    let mut all_exceeded = true;
    for k in 0..thresh_cnt {
        let t = rd::<f32>(thresh_ptr.wrapping_add(k * 4));
        let eff_t = if 0.0f32 <= t { t } else { f_max };
        if eff_t >= min_dist_sq {
            all_exceeded = false;
            break;
        }
    }

    if (((vec_32[1] >> 32) as u32) & 0x4000_0000) == 0 && vec_32[0] != 0 {
        exe::call_va!(0x1_4241_c190, (vec_32[0] as usize));
    }
    if (((vec_16[1] >> 32) as u32) & 0x4000_0000) == 0 && vec_16[0] != 0 {
        exe::call_va!(0x1_4241_c190, (vec_16[0] as usize));
    }

    if all_exceeded {
        if rd::<u64>(r14_slot + 0x10) > 0 {
            let cnt_18 = rd::<u32>(r15_entry + 0x18) as usize;
            for i in 0..cnt_18 {
                let h = rd::<u64>(rd::<usize>(r14_slot).wrapping_add(i * 8));
                let h_lo = h as u32;
                let h_hi = (h >> 32) as u32;
                if h_lo != 0xffff_ffff || h_hi != 0xffff_ffff {
                    let p2_obj = rd::<usize>(param_2);
                    let p2_cap = rd::<u32>(p2_obj + 0x20) as u64;
                    if (h_lo as u64) < p2_cap {
                        let p2_arr = rd::<usize>(p2_obj + 0x18);
                        let entry = rd::<u64>(p2_arr.wrapping_add((h_lo as usize) * 8));
                        if (entry as u32) == h_lo
                            && ((entry >> 32) as u32) == h_hi
                            && h_lo != 0xffff_ffff
                        {
                            exe::call_va!(0x1_4034_8930, (param_5, h));
                            wr::<u64>(rd::<usize>(r14_slot).wrapping_add(i * 8), u64::MAX);
                            wr::<u64>(r14_slot + 0x10, rd::<u64>(r14_slot + 0x10).wrapping_sub(1));
                        }
                    }
                }
            }
        }
    } else {
        let cnt_18 = rd::<u32>(r15_entry + 0x18) as u64;
        let mut lcg_rem = if cnt_18 != 0 { 0xf4243u64 % cnt_18 } else { 0 };
        let inv_cnt = 1.0f32 / (cnt_18 as f32);

        let mut settings_buf = [0u64; 24];
        let g_settings = rd::<usize>(exe::ptr::<usize>(0x1_4397_da50) as usize);
        let _ = exe::call_va!(
            0x1_41f5_2280,
            (g_settings, settings_buf.as_mut_ptr() as usize) -> usize
        );
        let quality = rd::<u32>((settings_buf.as_ptr() as usize) + 0x28);
        let thresh_ratio = match quality {
            0 => f32::from_bits(0x3f4c_cccd),
            1 => f32::from_bits(0x3f00_0000),
            2 => f32::from_bits(0x3e80_0000),
            _ => 0.0f32,
        };

        if cnt_18 != 0 {
            for i in 0..cnt_18 as usize {
                lcg_rem = (lcg_rem + 0xf4243) % cnt_18;
                let r0 = (lcg_rem as f32) * inv_cnt;
                let r8b = thresh_ratio >= r0;

                let mut pt_min_dist_sq = f_max;
                let item_base = rd::<usize>(r15_entry + 0x10).wrapping_add(i * 0x30);
                let item_pos = [
                    rd::<f32>(item_base + 0x24),
                    rd::<f32>(item_base + 0x28),
                    rd::<f32>(item_base + 0x2c),
                    0.0f32,
                ];
                let pts_ptr = rd::<usize>(param_3 + 0x10);
                let pts_cnt = rd::<u32>(param_3 + 0x18) as usize;
                let pts_end = pts_ptr.wrapping_add(pts_cnt << 5);
                let mut pt_cur = pts_ptr;
                while pt_cur != pts_end {
                    let dx = item_pos[0] - rd::<f32>(pt_cur);
                    let dy = item_pos[1] - rd::<f32>(pt_cur + 4);
                    let dz = item_pos[2] - rd::<f32>(pt_cur + 8);
                    let d_sq = (dx * dx + dy * dy) + (dz * dz + 0.0f32);
                    if !(pt_min_dist_sq < d_sq) {
                        pt_min_dist_sq = d_sq;
                    }
                    pt_cur = pt_cur.wrapping_add(0x20);
                }

                let tag_byte = rd::<u8>(rd::<usize>(r15_entry + 0x30).wrapping_add(i)) as usize;
                let raw_t = rd::<f32>(rd::<usize>(r15_entry + 0x70).wrapping_add(tag_byte * 4));
                let item_thresh = if 0.0f32 <= raw_t { raw_t } else { f_max };

                let h = rd::<u64>(rd::<usize>(r14_slot).wrapping_add(i * 8));
                let h_lo = h as u32;
                let h_hi = (h >> 32) as u32;
                if h_lo == 0xffff_ffff && h_hi == 0xffff_ffff {
                    if item_thresh >= pt_min_dist_sq && !r8b {
                        let arg3 = rsp_130;
                        let ok = exe::call_va!(
                            0x1_41e7_3020,
                            (
                                rsp_d0,
                                r15_entry,
                                arg3.as_ptr() as usize,
                                i as u64,
                                rsp_c8 as u64,
                                rd::<u64>(param_4),
                                param_5
                            ) -> u8
                        );
                        if ok != 0 {
                            wr::<u64>(rd::<usize>(r14_slot).wrapping_add(i * 8), u64::MAX);
                            wr::<u64>(r14_slot + 0x10, rd::<u64>(r14_slot + 0x10).wrapping_add(1));
                        }
                    }
                } else if pt_min_dist_sq > item_thresh || r8b {
                    let p2_obj = rd::<usize>(param_2);
                    let p2_cap = rd::<u32>(p2_obj + 0x20) as u64;
                    if (h_lo as u64) < p2_cap {
                        let p2_arr = rd::<usize>(p2_obj + 0x18);
                        let entry = rd::<u64>(p2_arr.wrapping_add((h_lo as usize) * 8));
                        if (entry as u32) == h_lo
                            && ((entry >> 32) as u32) == h_hi
                            && h_lo != 0xffff_ffff
                        {
                            exe::call_va!(0x1_4034_8930, (param_5, h));
                            wr::<u64>(rd::<usize>(r14_slot).wrapping_add(i * 8), u64::MAX);
                            wr::<u64>(r14_slot + 0x10, rd::<u64>(r14_slot + 0x10).wrapping_sub(1));
                        }
                    }
                }
            }
        }
    }
}

unsafe fn submit_ring_u32_batch(buf_ptr: usize, count: usize, opcode: u32) {
    if count == 0 {
        return;
    }
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let byte_len = (count * 4) as u64;
    let pkt_size = byte_len + 4;

    let (hdr_ptr, dst_ptr, commit_size) = if pkt_size < 0xffffc0 {
        let ring = rd::<usize>(mgr + 0x168);
        let mut hdr_p = 0usize;
        if pkt_size != 0 {
            let acq = rd::<usize>(0x1_4264_55a8);
            exe::call_va!(acq, (ring + 0x18));
            let aligned_sz = ((pkt_size + 7) & !7) as usize;
            let mut rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
            let mut wr_ptr = crate::simd_ops::rd_acq_usize(ring + 0x28);
            let mut next_wr = wr_ptr.wrapping_add(aligned_sz + 8);
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
                next_wr = wr_ptr.wrapping_add(aligned_sz + 8);
            }
            while wr_ptr <= rd_ptr
                && rd_ptr < next_wr
                && crate::simd_ops::rd_acq_u32(ring + 0x30) != crate::simd_ops::rd_acq_u32(ring + 0x34)
            {
                core::hint::spin_loop();
                rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
            }
            crate::simd_ops::wr_rel_u64(wr_ptr, 0x69fa_fafa_faff);
            hdr_p = wr_ptr.wrapping_add(8);
            crate::simd_ops::atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
            let rel = rd::<usize>(0x1_4264_5598);
            exe::call_va!(rel, (ring + 0x18));
        }
        wr::<u32>(hdr_p, ((count as u32) << 9) | opcode);
        (hdr_p, hdr_p.wrapping_add(4), pkt_size)
    } else {
        let ext_buf = exe::call_va!(0x1_4020_c080, (0u64, 0u64, 0u64, byte_len) -> usize);
        let ring = rd::<usize>(mgr + 0x168);
        let acq = rd::<usize>(0x1_4264_55a8);
        exe::call_va!(acq, (ring + 0x18));
        let mut rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        let mut wr_ptr = crate::simd_ops::rd_acq_usize(ring + 0x28);
        let mut next_wr = wr_ptr.wrapping_add(0x20);
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
            next_wr = wr_ptr.wrapping_add(0x20);
        }
        while wr_ptr <= rd_ptr
            && rd_ptr < next_wr
            && crate::simd_ops::rd_acq_u32(ring + 0x30) != crate::simd_ops::rd_acq_u32(ring + 0x34)
        {
            core::hint::spin_loop();
            rd_ptr = crate::simd_ops::rd_acq_usize(ring + 0x20);
        }
        crate::simd_ops::wr_rel_u64(wr_ptr, 0x69fa_fafa_faff);
        let hdr_p = wr_ptr.wrapping_add(8);
        crate::simd_ops::atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
        let rel = rd::<usize>(0x1_4264_5598);
        exe::call_va!(rel, (ring + 0x18));
        wr::<u32>(hdr_p, ((count as u32) << 9) | 1);
        wr::<u32>(hdr_p + 4, opcode);
        wr::<usize>(hdr_p + 0x0c, ext_buf);
        (hdr_p, ext_buf, 0x14u64)
    };

    for i in 0..count {
        wr::<u32>(
            dst_ptr.wrapping_add(i * 4),
            rd::<u32>(buf_ptr.wrapping_add(i * 4)),
        );
    }

    let mgr2 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff58) as usize);
    let old_hdr = rd::<u32>(hdr_ptr) & 0x1ff;
    wr::<u32>(hdr_ptr, old_hdr | ((count as u32) << 9));
    let ring2 = rd::<usize>(mgr2 + 0x168);
    crate::simd_ops::wr_rel_u64(hdr_ptr.wrapping_sub(8), commit_size);
    crate::simd_ops::atomic_add_u32((ring2 + 0x30) as u64, 1);
    if rd::<u8>(mgr2 + 0x198) != 0 {
        let rel_sem = rd::<usize>(0x1_4264_5580);
        let _ = exe::call_va!(rel_sem, (rd::<u64>(mgr2 + 0x1a0), 1u64, 0u64) -> i32);
    }
}

/// `unknown_141eaf8a0`
/// Original: 0x141eaf8a0 (2409 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141eaf8a0(param_1: usize) {
    let mut vec_on_ptr = 0usize;
    let mut vec_on_len = 0usize;
    let mut vec_on_cap = 0usize;
    let c1 = (exe::call_va!(0x1_4103_82d0, (param_1) -> u64) >> 1) as usize;
    if c1 != 0 {
        vec_on_ptr = exe::call_va!(0x1_4241_c0a0, ((c1 * 4) as u64, 4u64) -> usize);
        vec_on_cap = c1;
    }

    let mut vec_off_ptr = 0usize;
    let mut vec_off_len = 0usize;
    let mut vec_off_cap = 0usize;
    let c2 = (exe::call_va!(0x1_4103_82d0, (param_1) -> u64) >> 1) as usize;
    if c2 != 0 {
        vec_off_ptr = exe::call_va!(0x1_4241_c0a0, ((c2 * 4) as u64, 4u64) -> usize);
        vec_off_cap = c2;
    }

    let r11 = rd::<usize>(param_1).wrapping_add(0x40);
    let mut span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            init.as_mut_ptr() as usize,
            r11,
            span.as_mut_ptr() as usize
        ) -> usize
    );

    let mut outer = [0u64; 11];
    outer[0] = r11 as u64;
    outer[1] = init[0];
    outer[2] = init[1];
    outer[3] = init[2];
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
    exe::call_va!(0x1_4103_8d40, (outer.as_mut_ptr() as usize));

    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let idx = outer[6] as usize;
            let flag_ptr = (outer[9] as usize).wrapping_add(idx);
            let v0 = rd::<u32>((outer[7] as usize).wrapping_add(idx * 4));
            let v1 = rd::<u32>((outer[8] as usize).wrapping_add(idx * 4));
            let new_flag = u8::from(v0 != 0 || v1 != 0);
            if rd::<u8>(flag_ptr) != new_flag {
                let elem_ptr = (outer[10] as usize).wrapping_add(idx * 0x18);
                let id_val = rd::<u32>(elem_ptr + 0x10);
                if new_flag != 0 {
                    if vec_on_len == vec_on_cap {
                        let new_cap = (vec_on_cap + (vec_on_cap >> 1)).max(vec_on_len + 1);
                        let new_ptr = exe::call_va!(
                            0x1_4241_c0a0,
                            ((new_cap * 4) as u64, 4u64) -> usize
                        );
                        exe::call_va!(
                            0x1_4183_acc8,
                            (new_ptr, vec_on_ptr, (vec_on_len * 4) as u64)
                        );
                        if vec_on_ptr != 0 {
                            exe::call_va!(0x1_4241_c190, (vec_on_ptr));
                        }
                        vec_on_ptr = new_ptr;
                        vec_on_cap = new_cap;
                    }
                    wr::<u32>(vec_on_ptr.wrapping_add(vec_on_len * 4), id_val);
                    vec_on_len += 1;
                } else {
                    if vec_off_len == vec_off_cap {
                        let new_cap = (vec_off_cap + (vec_off_cap >> 1)).max(vec_off_len + 1);
                        let new_ptr = exe::call_va!(
                            0x1_4241_c0a0,
                            ((new_cap * 4) as u64, 4u64) -> usize
                        );
                        exe::call_va!(
                            0x1_4183_acc8,
                            (new_ptr, vec_off_ptr, (vec_off_len * 4) as u64)
                        );
                        if vec_off_ptr != 0 {
                            exe::call_va!(0x1_4241_c190, (vec_off_ptr));
                        }
                        vec_off_ptr = new_ptr;
                        vec_off_cap = new_cap;
                    }
                    wr::<u32>(vec_off_ptr.wrapping_add(vec_off_len * 4), id_val);
                    vec_off_len += 1;
                }
                wr::<u8>(flag_ptr, new_flag);
            }
            outer[6] = (idx + 1) as u64;
        }

        let mut cur_u16 = (outer[2] as usize).wrapping_add(2);
        outer[2] = cur_u16 as u64;
        let end_u16 = outer[3] as usize;
        if cur_u16 != end_u16 {
            let base_1 = outer[1] as usize;
            while cur_u16 != end_u16 {
                let next_id = rd::<u16>(cur_u16) as usize;
                if rd::<u64>(base_1.wrapping_add(next_id * 8).wrapping_add(0x48)) != 0
                    && rd::<u32>(base_1.wrapping_add(next_id * 4).wrapping_add(0x10048)) != 0
                {
                    break;
                }
                cur_u16 = cur_u16.wrapping_add(2);
            }
            outer[2] = cur_u16 as u64;
            if cur_u16 != end_u16 {
                let base_0 = outer[0] as usize;
                let arch_id = rd::<u16>(cur_u16) as usize;
                let cp = rd::<u64>(base_0.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
            }
        }
        exe::call_va!(0x1_4103_8d40, (outer.as_mut_ptr() as usize));
    }

    submit_ring_u32_batch(vec_on_ptr, vec_on_len, 0x50);
    submit_ring_u32_batch(vec_off_ptr, vec_off_len, 0x51);

    if vec_off_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec_off_ptr));
    }
    if vec_on_ptr != 0 {
        exe::call_va!(0x1_4241_c190, (vec_on_ptr));
    }
}

// ============================================================================
// Differential tests
// ============================================================================

pub unsafe fn diff_unknown_141e5a750(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCRATCH: [u8; 0x4000] = [0u8; 0x4000];
    static mut SCRATCH_OFF: usize = 0;

    unsafe extern "win64" fn mock_alloc(sz: u64, _align: u64) -> usize {
        let base = core::ptr::addr_of_mut!(SCRATCH) as usize;
        let off = (SCRATCH_OFF + 15) & !15;
        let next = off + (sz as usize).max(16);
        SCRATCH_OFF = next;
        base + off
    }
    unsafe extern "win64" fn mock_free(_p: usize) {}
    unsafe extern "win64" fn mock_iter_init(out: usize, r11: usize, span: usize) -> usize {
        wr::<usize>(out, r11);
        wr::<u64>(out + 8, rd::<u64>(span));
        wr::<u64>(out + 16, rd::<u64>(span + 8));
        out
    }
    unsafe extern "win64" fn mock_iter_next(iter: usize) -> usize {
        let next_p = rd::<usize>(iter + 8).wrapping_add(2);
        wr::<usize>(iter + 8, next_p);
        iter
    }
    unsafe extern "win64" fn mock_setup_cols(outer: usize) {
        if rd::<u64>(outer + 16) != rd::<u64>(outer + 24) {
            let chunk = rd::<u64>(outer + 32);
            wr::<u64>(outer + 56, chunk.wrapping_add(0x40));
            wr::<u64>(outer + 64, chunk.wrapping_add(0x50));
        }
    }
    unsafe extern "win64" fn mock_push_u64(vec: usize, val_ptr: usize) {
        let p = rd::<usize>(vec);
        let len = rd::<u32>(vec + 8) as usize;
        wr::<u64>(p + len * 8, rd::<u64>(val_ptr));
        wr::<u32>(vec + 8, (len + 1) as u32);
    }
    unsafe extern "win64" fn mock_vec_a8_push(vec: usize, val_ptr: usize) -> usize {
        if rd::<usize>(vec) == 0 {
            let p = mock_alloc(64, 8);
            wr::<usize>(vec, p);
            wr::<u32>(vec + 12, 8);
        }
        let p = rd::<usize>(vec);
        let len = rd::<u32>(vec + 8) as usize;
        let slot = p + len * 8;
        wr::<u64>(slot, rd::<u64>(val_ptr));
        wr::<u32>(vec + 8, (len + 1) as u32);
        slot
    }
    unsafe extern "win64" fn mock_submit(p5_ptr: usize, span_ptr: usize) {
        let sink = rd::<usize>(p5_ptr);
        let cnt = rd::<u64>(span_ptr) as usize;
        let arr = rd::<usize>(span_ptr + 8);
        wr::<u64>(sink, rd::<u64>(sink).wrapping_add(cnt as u64));
        let mut sum = rd::<u64>(sink + 8);
        for i in 0..cnt {
            sum = sum.wrapping_add(rd::<u64>(arr + i * 8));
        }
        wr::<u64>(sink + 8, sum);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_init as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_7c70, mock_iter_next as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_40fb_7280, mock_setup_cols as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_40fa_5ca0, mock_push_u64 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4046_6a90, mock_vec_a8_push as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_40fb_7ed0, mock_submit as *const ())?;

    let g_lim = exe::ptr::<i32>(0x1_4385_0be8) as usize;
    let s_lim = rd::<i32>(g_lim);
    wr::<i32>(g_lim, 5);

    let run = |f: F| -> (u32, u32, u64, u64, u64, u64) {
        SCRATCH_OFF = 0;

        let mut chunk1 = [0u8; 0x80];
        let c1p = chunk1.as_mut_ptr() as usize;
        wr::<u64>(c1p + 0x10, 0x1111_2222);
        wr::<u8>(c1p + 0x40, 1);
        wr::<u64>(c1p + 0x50, 0xaaaa_0001);

        let mut chunk2 = [0u8; 0x80];
        let c2p = chunk2.as_mut_ptr() as usize;
        wr::<u64>(c2p + 0x10, 0x3333_4444);
        wr::<u8>(c2p + 0x40, 1);
        wr::<u64>(c2p + 0x50, 0xbbbb_0002);

        let mut world1 = vec![0u8; 0x10200];
        let w1p = world1.as_mut_ptr() as usize;
        wr::<usize>(w1p + 0x88, c1p);
        wr::<u32>(w1p + 0x10088, 1);

        let mut world3 = vec![0u8; 0x10200];
        let w3p = world3.as_mut_ptr() as usize;
        wr::<usize>(w3p + 0x88, c2p);
        wr::<u32>(w3p + 0x10088, 1);

        let arch_ids = [0u16];
        let arch_start = arch_ids.as_ptr() as u64;
        let arch_end = arch_start + 2;

        let p1 = [w1p as u64, arch_start, arch_end];
        let p2 = [w1p as u64, arch_start, arch_start];
        let p3 = [w3p as u64, arch_start, arch_end];

        let mut items = [0u8; 0x80];
        let ip = items.as_mut_ptr() as usize;
        wr::<u32>(ip + 0x08, 10);
        wr::<u64>(ip + 0x10, 0);
        wr::<u64>(ip + 0x18, 0xaaaa_0001);
        wr::<u32>(ip + 0x20, 3);

        wr::<u32>(ip + 0x40 + 0x08, 1);
        wr::<u64>(ip + 0x40 + 0x10, 0);
        wr::<u64>(ip + 0x40 + 0x18, 0xbbbb_0002);
        wr::<u32>(ip + 0x40 + 0x20, 3);

        let mut p4 = [0u8; 0x10];
        let p4p = p4.as_mut_ptr() as usize;
        wr::<usize>(p4p, ip);
        wr::<u32>(p4p + 0x08, 2);

        let mut sink = [0u64; 2];
        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3.as_ptr() as usize,
            p4p,
            sink.as_mut_ptr() as u64,
        );
        (
            rd::<u32>(ip + 0x20),
            rd::<u32>(ip + 0x40 + 0x20),
            rd::<u64>(c1p + 0x50),
            rd::<u64>(c2p + 0x50),
            sink[0],
            sink[1],
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<i32>(g_lim, s_lim);
    crate::diff_eq!(o, n, "141e5a750 out");
    Ok(1)
}

pub unsafe fn diff_coregame_141e71db0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCRATCH: [u8; 0x2000] = [0u8; 0x2000];
    static mut SCRATCH_OFF: usize = 0;

    unsafe extern "win64" fn noop0() -> usize {
        0
    }
    unsafe extern "win64" fn mock_alloc(sz: u64, _align: u64) -> usize {
        let base = (core::ptr::addr_of_mut!(SCRATCH) as usize + 15) & !15;
        let off = (SCRATCH_OFF + 31) & !31;
        SCRATCH_OFF = off + (sz as usize).max(32);
        base + off
    }
    unsafe extern "win64" fn mock_free(_p: usize) {}
    unsafe extern "win64" fn mock_mat_to_quat(out_q: usize, _mat: usize) {
        wr::<[f32; 4]>(out_q, [1.0f32, 0.0, 0.0, 0.0]);
    }
    unsafe extern "win64" fn mock_settings(_mgr: usize, out_buf: usize) -> usize {
        wr::<u32>(out_buf + 0x28, 2);
        out_buf
    }
    unsafe extern "win64" fn mock_spawn(
        _a: usize,
        _b: usize,
        _c: usize,
        _d: u64,
        _e: u64,
        _f: u64,
        p5: u64,
    ) -> u8 {
        let sp = p5 as usize;
        wr::<u64>(sp, rd::<u64>(sp).wrapping_add(1));
        1
    }
    unsafe extern "win64" fn mock_despawn(p5: u64, handle: u64) {
        let sp = p5 as usize;
        wr::<u64>(sp + 8, handle);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_bbe0, noop0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4027_5010, mock_mat_to_quat as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41f5_2280, mock_settings as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41e7_3020, mock_spawn as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4034_8930, mock_despawn as *const ())?;

    let g_ee0 = exe::ptr::<u8>(0x1_4385_0ee0) as usize;
    let s_ee0 = rd::<u8>(g_ee0);
    wr::<u8>(g_ee0, 0);

    let run = |f: F| -> (u64, u64, u64, u64, u64) {
        SCRATCH_OFF = 0;

        let obb_bounds: [f32; 6] = [-2.0, -2.0, -2.0, 2.0, 2.0, 2.0];
        let mut obb_mat = [0u8; 0x30];
        wr::<f32>(obb_mat.as_mut_ptr() as usize + 0x24, 0.0);
        wr::<f32>(obb_mat.as_mut_ptr() as usize + 0x28, 0.0);
        wr::<f32>(obb_mat.as_mut_ptr() as usize + 0x2c, 0.0);

        let thresholds: [f32; 2] = [25.0, 1.0];
        let tags: [u8; 2] = [1, 0];
        let mut item_mats = [0u8; 0x60];
        let imp = item_mats.as_mut_ptr() as usize;
        wr::<f32>(imp + 0x24, 10.0);
        wr::<f32>(imp + 0x28, 0.0);
        wr::<f32>(imp + 0x2c, 0.0);
        wr::<f32>(imp + 0x30 + 0x24, 1.0);
        wr::<f32>(imp + 0x30 + 0x28, 0.0);
        wr::<f32>(imp + 0x30 + 0x2c, 0.0);

        let mut r15_entry = [0u8; 0xa0];
        let r15p = r15_entry.as_mut_ptr() as usize;
        wr::<usize>(r15p + 0x10, imp);
        wr::<u32>(r15p + 0x18, 2);
        wr::<usize>(r15p + 0x30, tags.as_ptr() as usize);
        wr::<usize>(r15p + 0x70, thresholds.as_ptr() as usize);
        wr::<u32>(r15p + 0x78, 1);
        wr::<usize>(r15p + 0x80, obb_bounds.as_ptr() as usize);
        wr::<u32>(r15p + 0x88, 1);
        wr::<usize>(r15p + 0x90, obb_mat.as_ptr() as usize);

        let mut handles: [u64; 2] = [0x0000_0007_0000_0000, u64::MAX];
        let mut r14_slot = [0u8; 0x20];
        let r14p = r14_slot.as_mut_ptr() as usize;
        wr::<usize>(r14p, handles.as_mut_ptr() as usize);
        wr::<u64>(r14p + 0x10, 1);
        wr::<u8>(r14p + 0x18, 1);

        let mut p1_4_buf = [0u8; 0x30];
        wr::<u64>(p1_4_buf.as_mut_ptr() as usize, 0x1234_5678);
        wr::<u32>(p1_4_buf.as_mut_ptr() as usize + 0x08, 9);

        let p1: [u64; 7] = [
            r15p as u64,
            0x1000,
            0x2000,
            r14p as u64,
            p1_4_buf.as_ptr() as u64,
            0,
            0,
        ];

        let p2_entries: [u64; 1] = [0x0000_0007_0000_0000];
        let mut p2_obj = [0u8; 0x30];
        let p2op = p2_obj.as_mut_ptr() as usize;
        wr::<usize>(p2op + 0x18, p2_entries.as_ptr() as usize);
        wr::<u32>(p2op + 0x20, 1);
        let p2 = [p2op as u64];

        let pts_buf: [f32; 8] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let mut p3 = [0u8; 0x20];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p + 0x10, pts_buf.as_ptr() as usize);
        wr::<u32>(p3p + 0x18, 1);

        let p4 = [0x5555u64];
        let mut p5_sink = [0u64; 2];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3p,
            p4.as_ptr() as usize,
            p5_sink.as_mut_ptr() as u64,
        );
        (
            handles[0],
            handles[1],
            rd::<u64>(r14p + 0x10),
            p5_sink[0],
            p5_sink[1],
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u8>(g_ee0, s_ee0);
    crate::diff_eq!(o, n, "141e71db0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141eaf8a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCRATCH: [u8; 0x2000] = [0u8; 0x2000];
    static mut SCRATCH_OFF: usize = 0;

    unsafe extern "win64" fn mock_count(_p: usize) -> u64 {
        4
    }
    unsafe extern "win64" fn mock_alloc(sz: u64, _align: u64) -> usize {
        let base = core::ptr::addr_of_mut!(SCRATCH) as usize;
        let off = (SCRATCH_OFF + 15) & !15;
        SCRATCH_OFF = off + (sz as usize).max(16);
        base + off
    }
    unsafe extern "win64" fn mock_free(_p: usize) {}
    unsafe extern "win64" fn mock_iter_init(out: usize, r11: usize, span: usize) -> usize {
        wr::<usize>(out, r11);
        wr::<u64>(out + 8, rd::<u64>(span));
        wr::<u64>(out + 16, rd::<u64>(span + 8));
        out
    }
    unsafe extern "win64" fn mock_setup_cols_11(outer: usize) {
        if rd::<u64>(outer + 16) != rd::<u64>(outer + 24) {
            let chunk = rd::<u64>(outer + 32);
            wr::<u64>(outer + 56, chunk.wrapping_add(0x10));
            wr::<u64>(outer + 64, chunk.wrapping_add(0x20));
            wr::<u64>(outer + 72, chunk.wrapping_add(0x30));
            wr::<u64>(outer + 80, chunk.wrapping_add(0x40));
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4103_82d0, mock_count as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_alloc as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_free as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_init as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4103_8d40, mock_setup_cols_11 as *const ())?;

    let g_mgr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let s_mgr = rd::<usize>(g_mgr);

    let run = |f: F| -> (u8, u8, u32, [u32; 8]) {
        SCRATCH_OFF = 0;

        let mut chunk = [0u8; 0x100];
        let cp = chunk.as_mut_ptr() as usize;
        wr::<u32>(cp + 0x10, 1);
        wr::<u32>(cp + 0x14, 0);
        wr::<u32>(cp + 0x20, 0);
        wr::<u32>(cp + 0x24, 0);
        wr::<u8>(cp + 0x30, 0);
        wr::<u8>(cp + 0x31, 1);
        wr::<u32>(cp + 0x40 + 0x10, 0x101);
        wr::<u32>(cp + 0x58 + 0x10, 0x202);

        let mut world = vec![0u8; 0x10200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88, cp);
        wr::<u32>(wp + 0x10088, 2);

        let arch_ids = [0u16];
        let arch_start = arch_ids.as_ptr() as u64;
        let arch_end = arch_start + 2;
        let p1 = [wp as u64, arch_start, arch_end];

        let mut ring_mem = [0u32; 16];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let mut ring = [0u8; 0x60];
        let rp = ring.as_mut_ptr() as usize;
        wr::<usize>(rp + 0x08, rmp);
        wr::<usize>(rp + 0x10, rmp + 64);
        wr::<u64>(rp + 0x18, 0);
        wr::<usize>(rp + 0x20, rmp);
        wr::<usize>(rp + 0x28, rmp);
        wr::<u32>(rp + 0x30, 0);
        wr::<u32>(rp + 0x34, 0);

        let mut mgr = [0u8; 0x200];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<usize>(mp + 0x168, rp);
        wr::<u8>(mp + 0x198, 0);
        wr::<usize>(g_mgr, mp);

        f(p1.as_ptr() as usize);

        let mut out_ring = [0u32; 8];
        out_ring.copy_from_slice(&ring_mem[..8]);
        (
            rd::<u8>(cp + 0x30),
            rd::<u8>(cp + 0x31),
            rd::<u32>(rp + 0x30),
            out_ring,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_mgr, s_mgr);
    crate::diff_eq!(o, n, "141eaf8a0 out");
    Ok(1)
}

// REGISTRY:
// Replacement::verified_throws(0x1_41e5_a750, "unknown_141e5a750", rend::cauldron_hot_d::unknown_141e5a750 as *const (), Throws::Discard, rend::cauldron_hot_d::diff_unknown_141e5a750),
// Replacement::verified_throws(0x1_41e7_1db0, "coregame_141e71db0", rend::cauldron_hot_d::coregame_141e71db0 as *const (), Throws::Discard, rend::cauldron_hot_d::diff_coregame_141e71db0),
// Replacement::verified_throws(0x1_41ea_f8a0, "unknown_141eaf8a0", rend::cauldron_hot_d::unknown_141eaf8a0 as *const (), Throws::Discard, rend::cauldron_hot_d::diff_unknown_141eaf8a0),
