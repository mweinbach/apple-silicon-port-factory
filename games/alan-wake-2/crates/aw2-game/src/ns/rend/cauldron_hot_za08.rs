//! Decompiled `rend` hot functions shard ZA08 (`0x141df8580..0x141e20fe0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
fn vpermilps(v: [f32; 4], imm: u8) -> [f32; 4] {
    [
        v[(imm & 3) as usize],
        v[((imm >> 2) & 3) as usize],
        v[((imm >> 4) & 3) as usize],
        v[((imm >> 6) & 3) as usize],
    ]
}

#[inline(always)]
fn vmulps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] * b[0], a[1] * b[1], a[2] * b[2], a[3] * b[3]]
}

#[inline(always)]
fn vaddps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]]
}

#[inline(always)]
fn vsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]]
}

#[inline(always)]
fn vhsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - a[1], a[2] - a[3], b[0] - b[1], b[2] - b[3]]
}

#[inline(always)]
fn vhaddps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] + a[1], a[2] + a[3], b[0] + b[1], b[2] + b[3]]
}

#[inline(always)]
fn vshufps(a: [f32; 4], b: [f32; 4], imm: u8) -> [f32; 4] {
    [
        a[(imm & 3) as usize],
        a[((imm >> 2) & 3) as usize],
        b[((imm >> 4) & 3) as usize],
        b[((imm >> 6) & 3) as usize],
    ]
}

#[inline(always)]
fn vblendps(a: [f32; 4], b: [f32; 4], mask: u8) -> [f32; 4] {
    [
        if (mask & 1) != 0 { b[0] } else { a[0] },
        if (mask & 2) != 0 { b[1] } else { a[1] },
        if (mask & 4) != 0 { b[2] } else { a[2] },
        if (mask & 8) != 0 { b[3] } else { a[3] },
    ]
}

#[inline(always)]
fn vaddsubps(a: [f32; 4], b: [f32; 4]) -> [f32; 4] {
    [a[0] - b[0], a[1] + b[1], a[2] - b[2], a[3] + b[3]]
}

#[inline(always)]
unsafe fn compose_parent_child(par: [f32; 8], child: [f32; 8]) -> [f32; 8] {
    let par_q = [par[0], par[1], par[2], par[3]];
    let par_p = [par[4], par[5], par[6], par[7]];
    let child_q = [child[0], child[1], child[2], child[3]];
    let child_p = [child[4], child[5], child[6], child[7]];

    let par_q_d2 = vpermilps(par_q, 0xd2);
    let par_q_c9 = vpermilps(par_q, 0xc9);
    let child_p_d2 = vpermilps(child_p, 0xd2);
    let m1 = vmulps(child_p_d2, par_q_c9);
    let child_p_c9 = vpermilps(child_p, 0xc9);
    let m2 = vmulps(child_p_c9, par_q_d2);
    let cross1 = vsubps(m1, m2);
    let scale = rd::<[f32; 4]>(exe::ptr::<f32>(0x1_4342_7ab0) as usize);
    let t = vmulps(cross1, scale);

    let cq_11 = vpermilps(child_q, 0x11);
    let qm1 = vmulps(par_q, cq_11);
    let cq_bb = vpermilps(child_q, 0xbb);
    let pq_1b = vpermilps(par_q, 0x1b);
    let qm2 = vmulps(cq_bb, pq_1b);
    let q_hsub = vhsubps(qm1, qm2);
    let qm3 = vmulps(par_q, cq_bb);
    let qm4 = vmulps(cq_11, pq_1b);
    let q_hadd = vhaddps(qm3, qm4);
    let q_shuf = vshufps(q_hsub, q_hadd, 0xb1);
    let q_blend = vblendps(q_hadd, q_hsub, 0x0c);
    let q_addsub = vaddsubps(q_blend, q_shuf);
    let out_q = vpermilps(q_addsub, 0x9c);

    let t_d2 = vpermilps(t, 0xd2);
    let pm1 = vmulps(t_d2, par_q_c9);
    let t_c9 = vpermilps(t, 0xc9);
    let par_w = vpermilps(par_q, 0xff);
    let pm2 = vmulps(t_c9, par_q_d2);
    let w_t = vmulps(par_w, t);
    let cross2 = vsubps(pm1, pm2);
    let sum1 = vaddps(w_t, child_p);
    let sum2 = vaddps(cross2, sum1);
    let out_p = vaddps(sum2, par_p);

    [
        out_q[0], out_q[1], out_q[2], out_q[3], out_p[0], out_p[1], out_p[2], out_p[3],
    ]
}

#[inline(always)]
unsafe fn inplace_cstr_call(s: usize) -> usize {
    let mut p = s.wrapping_add(0x08);
    if exe::call_va!(0x1_401d_f6c0, (s) -> u8) == 0 {
        p = rd::<usize>(p);
    }
    p
}

#[inline(always)]
unsafe fn inplace_cstr_inline(s: usize) -> usize {
    let hdr = rd::<u32>(s);
    let b3 = hdr >> 24;
    let cap_bits = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let alloc_bits = (hdr & 0x00ff_ffff) << 3;
    if alloc_bits > cap_bits {
        rd::<usize>(s.wrapping_add(0x08))
    } else {
        s.wrapping_add(0x08)
    }
}

#[inline(always)]
unsafe fn cstr_len(p: usize) -> u64 {
    let mut len = 0usize;
    while rd::<u8>(p.wrapping_add(len)) != 0 {
        len = len.wrapping_add(1);
    }
    len as u64
}

#[inline(always)]
unsafe fn upsert_table_string(rsi: usize, hash: u64, cstr: usize) {
    let r13_38 = rd::<u64>(rsi + 0x38);
    let cap_48 = rd::<u64>(rsi + 0x48);
    let buckets = rd::<usize>(rsi + 0x78);
    let end_ptr = buckets.wrapping_add((cap_48 as usize).wrapping_mul(0x28));

    let found_ptr = if rd::<u64>(rsi + 0x40) == r13_38 {
        end_ptr
    } else {
        let mask = cap_48.wrapping_sub(1);
        let mut step: u64 = 0;
        let mut idx = hash & mask;
        let mut tomb: u64 = u64::MAX;
        let empty0 = rd::<u64>(rsi + 0x50);
        let empty1 = rd::<u64>(rsi + 0x58);
        let del0 = rd::<u64>(rsi + 0x28);
        let del1 = rd::<u64>(rsi + 0x30);
        let matched_idx = loop {
            let ent = buckets.wrapping_add((idx as usize).wrapping_mul(0x28));
            let k0 = rd::<u64>(ent);
            let k1 = rd::<u64>(ent + 0x08);
            if empty0 == k0 && empty1 == k1 {
                break cap_48;
            }
            if r13_38 != 0 && del0 == k0 && del1 == k1 {
                if tomb == u64::MAX {
                    tomb = idx;
                }
            } else if k0 == 0 && k1 == hash {
                break if idx == u64::MAX { cap_48 } else { idx };
            }
            step = step.wrapping_add(1);
            idx = idx.wrapping_add(step) & mask;
        };
        buckets.wrapping_add((matched_idx as usize).wrapping_mul(0x28))
    };

    if found_ptr != end_ptr {
        let len = cstr_len(cstr);
        exe::call_va!(0x1_41f7_32a0, (found_ptr + 0x10, rsi, cstr, len));
    } else {
        let free_cnt = rd::<u32>(rsi + 0x108);
        let free_base = rd::<usize>(rsi + 0x100);
        let free_end = free_base.wrapping_add((free_cnt as usize) * 4);
        let id_val = if free_base != free_end {
            let v = rd::<u32>(free_end - 4);
            wr::<u32>(rsi + 0x108, free_cnt.wrapping_sub(1));
            v
        } else {
            let v = rd::<u32>(rsi + 0x110);
            wr::<u32>(rsi + 0x110, v.wrapping_add(1));
            v
        };
        let mut val_buf = [0u64; 3];
        let vbp = val_buf.as_mut_ptr() as usize;
        wr::<u32>(vbp + 0x04, id_val);
        let len = cstr_len(cstr);
        exe::call_va!(0x1_41f7_32a0, (vbp, rsi, cstr, len));
        wr::<u8>(vbp + 0x02, 0);
        let mut key: [u64; 2] = [0, hash];
        let slot = exe::call_va!(0x1_401e_b860, (rsi, key.as_mut_ptr() as usize) -> usize);
        wr::<[u64; 3]>(slot + 0x10, rd::<[u64; 3]>(vbp));
    }
}

/// `unknown_141df8580`
/// Original: 0x141df8580 (2453 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141df8580(param_1: usize, param_2: usize) {
    let idx_e88 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce88) as usize) as usize;
    let rsi = rd::<usize>(rd::<usize>(param_1 + 0x580e0).wrapping_add(idx_e88 * 8));

    let mut iter_buf = [0u64; 8];
    let iter_ptr = iter_buf.as_mut_ptr() as usize;
    exe::call_va!(0x1_401e_f430, (param_2, iter_ptr));

    let mut r12_idx = rd::<u64>(iter_ptr + 0x30);
    let mut ent_count = rd::<u64>(iter_ptr + 0x28);
    if r12_idx >= ent_count {
        return;
    }

    let mut rdi_comp_base = rd::<usize>(iter_ptr + 0x38);
    let mut cur_arch = rd::<usize>(iter_ptr + 0x10);
    let mut str_50 = [0u64; 18];
    let str_50_ptr = str_50.as_mut_ptr() as usize;

    loop {
        while r12_idx < ent_count {
            let r15_comp = rdi_comp_base.wrapping_add((r12_idx as usize).wrapping_mul(0x140));
            let rax_str = exe::call_va!(0x1_41df_ae90, (str_50_ptr, r15_comp) -> usize);
            let r14_cstr = inplace_cstr_call(rax_str);
            let r8_key_cstr = inplace_cstr_call(r15_comp + 0x28);
            let key_hash =
                exe::call_va!(0x1_424b_16a0, (r8_key_cstr, cstr_len(r8_key_cstr)) -> u64);

            upsert_table_string(rsi, key_hash, r14_cstr);
            exe::call_va!(0x1_4240_ab10, (str_50_ptr));

            let mut val_280: u64 = 0;
            let val_280_ptr = (&mut val_280 as *mut u64) as usize;

            let active = exe::call_va!(0x1_41df_aaa0, (r15_comp, param_2) -> u8);
            wr::<u8>(val_280_ptr, active);
            let cstr_78 = inplace_cstr_call(r15_comp + 0x78);
            let hash_78 = exe::call_va!(0x1_424b_16a0, (cstr_78, cstr_len(cstr_78)) -> u64);
            let mut key_60: [u64; 2] = [0, hash_78];
            exe::call_va!(
                0x1_401e_acd0,
                (
                    rsi,
                    key_60.as_mut_ptr() as usize,
                    cstr_78,
                    val_280_ptr,
                    0u32
                )
            );

            wr::<u32>(val_280_ptr, rd::<u32>(r15_comp + 0x128));
            let cstr_50 = inplace_cstr_call(r15_comp + 0x50);
            let hash_50 = exe::call_va!(0x1_424b_16a0, (cstr_50, cstr_len(cstr_50)) -> u64);
            let mut key_50: [u64; 2] = [0, hash_50];
            exe::call_va!(
                0x1_401f_1a80,
                (
                    rsi,
                    key_50.as_mut_ptr() as usize,
                    cstr_50,
                    val_280_ptr,
                    0u32
                )
            );

            let cstr_a0 = inplace_cstr_call(r15_comp + 0xa0);
            let hash_a0 = exe::call_va!(0x1_424b_16a0, (cstr_a0, cstr_len(cstr_a0)) -> u64);
            let mut key_40: [u64; 2] = [0, hash_a0];
            exe::call_va!(
                0x1_401f_1a80,
                (
                    rsi,
                    key_40.as_mut_ptr() as usize,
                    cstr_a0,
                    r15_comp + 0x11c,
                    0u32
                )
            );

            let cnt_f64 = (rd::<u32>(r15_comp + 0x128) as f64).to_bits();
            wr::<u64>(val_280_ptr, cnt_f64);
            let cstr_f0 = inplace_cstr_call(r15_comp + 0xf0);
            let hash_f0 = exe::call_va!(0x1_424b_16a0, (cstr_f0, cstr_len(cstr_f0)) -> u64);
            let mut key_30: [u64; 2] = [0, hash_f0];
            exe::call_va!(
                0x1_401f_3a20,
                (
                    rsi,
                    key_30.as_mut_ptr() as usize,
                    cstr_f0,
                    val_280_ptr,
                    0u32
                )
            );

            if rd::<u32>(r15_comp + 0x128) > 0 {
                let mut i_idx: u64 = 0;
                let mut fmt_iter = [0u64; 56];
                let ibp = fmt_iter.as_mut_ptr() as usize;
                let mut s68 = [0u64; 8];
                let s68_ptr = s68.as_mut_ptr() as usize;
                let lut = exe::ptr::<u8>(0x1_426d_01e0) as usize;

                while i_idx < rd::<u32>(r15_comp + 0x128) as u64 {
                    let r14_buf = exe::call_va!(0x1_423f_dfc0, () -> usize);

                    wr::<usize>(str_50_ptr, r15_comp + 0xc8);
                    wr::<usize>(
                        str_50_ptr + 0x08,
                        exe::ptr::<u8>(0x1_401f_ebb0) as usize,
                    );
                    wr::<u64>(str_50_ptr + 0x10, i_idx);
                    wr::<u64>(str_50_ptr + 0x18, 0);

                    let mut fmt_args: [u64; 2] = [0x4f, str_50_ptr as u64];
                    let mut fmt_str: [u64; 2] =
                        [exe::ptr::<u8>(0x1_430b_9b80) as usize as u64, 5];

                    wr::<usize>(ibp, exe::ptr::<u8>(0x1_4307_9330) as usize);
                    wr::<usize>(ibp + 0x08, r14_buf);
                    wr::<u64>(ibp + 0x10, 0);
                    wr::<u64>(ibp + 0x18, 0x7ff);
                    wr::<u64>(ibp + 0x20, 0);
                    wr::<u64>(ibp + 0x28, 0x7ff);
                    wr::<usize>(ibp + 0x30, r14_buf);

                    exe::call_va!(
                        0x1_4146_1fb0,
                        (
                            ibp,
                            fmt_str.as_mut_ptr() as usize,
                            fmt_args.as_mut_ptr() as usize,
                            0u64
                        )
                    );

                    let cap_e8 = rd::<u64>(ibp + 0x28);
                    let used_e0 = rd::<u64>(ibp + 0x20);
                    let mut rem = if cap_e8 > used_e0 {
                        cap_e8.wrapping_sub(used_e0)
                    } else {
                        0
                    };
                    let d0 = rd::<u64>(ibp + 0x10);
                    if d0 < rem {
                        rem = d0;
                    }
                    let mut out_end = rd::<usize>(ibp + 0x30);
                    if rd::<usize>(ibp + 0x08) == out_end {
                        out_end = out_end.wrapping_add(rem as usize);
                    }
                    wr::<u8>(out_end, 0);

                    let elem_str = rd::<usize>(r15_comp + 0x120)
                        .wrapping_add((i_idx as usize).wrapping_mul(0x28));
                    wr::<u64>(s68_ptr, 0x0400_0004);
                    wr::<u8>(s68_ptr + 0x08, 0);
                    if s68_ptr != elem_str {
                        let src_len = rd::<u32>(elem_str + 0x04) as u64;
                        let src_ptr = inplace_cstr_inline(elem_str);
                        let mut view: [u64; 2] = [src_ptr as u64, src_len];
                        exe::call_va!(
                            0x1_4240_9b50,
                            (s68_ptr, view.as_mut_ptr() as usize, 0u64, src_len)
                        );
                    }

                    let cur_len = rd::<u32>(s68_ptr + 0x04) as usize;
                    let mut data_p = inplace_cstr_inline(s68_ptr);
                    for k in 0..cur_len {
                        let ch = rd::<u8>(data_p.wrapping_add(k)) as usize;
                        wr::<u8>(data_p.wrapping_add(k), rd::<u8>(lut.wrapping_add(ch)));
                    }
                    data_p = inplace_cstr_inline(s68_ptr);

                    let item_hash =
                        exe::call_va!(0x1_424b_16a0, (r14_buf, cstr_len(r14_buf)) -> u64);
                    upsert_table_string(rsi, item_hash, data_p);
                    exe::call_va!(0x1_4240_ab10, (s68_ptr));

                    i_idx = i_idx.wrapping_add(1);
                }
            }

            r12_idx = r12_idx.wrapping_add(1);
        }

        cur_arch = cur_arch.wrapping_add(2);
        let end_arch = rd::<usize>(iter_ptr + 0x18);
        if cur_arch != end_arch {
            let world1 = rd::<usize>(iter_ptr + 0x08);
            while cur_arch != end_arch {
                let arch = rd::<u16>(cur_arch) as usize;
                if rd::<usize>(world1.wrapping_add(arch * 8 + 0x48)) != 0
                    && rd::<u32>(world1.wrapping_add(arch * 4 + 0x10048)) != 0
                {
                    break;
                }
                cur_arch = cur_arch.wrapping_add(2);
            }
            if cur_arch != end_arch {
                let r11 = rd::<u16>(cur_arch) as usize;
                let world0 = rd::<usize>(iter_ptr);
                let block = rd::<usize>(world0.wrapping_add(r11 * 8 + 0x48));
                ent_count = if block != 0 {
                    rd::<u32>(world0.wrapping_add(r11 * 4 + 0x10048)) as u64
                } else {
                    0
                };
                r12_idx = 0;
                let off32 = r11 << 5;
                let s_len = rd::<u32>(world0.wrapping_add(off32 + 0x1806c));
                let want = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize) as u32;
                let s_ptr = rd::<u64>(world0.wrapping_add(off32 + 0x18058));
                let found = exe::call_va!(0x1_4248_9ee0, (s_ptr, s_len, want) -> i32);
                if (found as isize) == -1 {
                    rdi_comp_base = 0;
                } else {
                    let off_tbl = rd::<usize>(world0.wrapping_add((r11 + 0xc03) << 5));
                    let off = rd::<u32>(off_tbl.wrapping_add((found as isize as usize) * 4)) as usize;
                    rdi_comp_base = if off != 0 {
                        off.wrapping_add(block)
                    } else {
                        0
                    };
                }
            }
        }

        if r12_idx >= ent_count {
            return;
        }
    }
}

/// `unknown_141dfae90`
/// Original: 0x141dfae90 (160 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141dfae90(param_1: usize, param_2: usize) -> usize {
    let idx = rd::<i32>(param_2 + 0x11c) as isize;
    if idx > -1 {
        let arr = rd::<usize>(param_2 + 0x120);
        let count = rd::<u32>(param_2 + 0x128) as usize;
        if (idx as usize) < count {
            let src = arr.wrapping_add((idx as usize).wrapping_mul(0x28));
            wr::<u32>(param_1, (rd::<u32>(param_1) & 0xff00_0004) | 0x4);
            wr::<u8>(param_1 + 0x03, 0x4);
            wr::<u32>(param_1 + 0x04, 0);
            wr::<u8>(param_1 + 0x08, 0);
            exe::call_va!(0x1_4240_aa00, (param_1, src));
            return param_1;
        }
    }
    wr::<u32>(param_1, (rd::<u32>(param_1) & 0xff00_0004) | 0x4);
    wr::<u8>(param_1 + 0x03, 0x4);
    wr::<u32>(param_1 + 0x04, 0);
    wr::<u8>(param_1 + 0x08, 0);
    let mut view: [u64; 2] = [exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64, 0];
    exe::call_va!(
        0x1_4240_9b50,
        (param_1, view.as_mut_ptr() as usize, 0u64, 0u64)
    );
    param_1
}

/// `coregame_141e111f0`
/// Original: 0x141e111f0 (460 bytes, Throws::Discard)
pub unsafe extern "win64" fn coregame_141e111f0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
    param_6: u32,
) {
    let p3_str = inplace_cstr_inline(param_3);
    let p3_len = rd::<u32>(param_3 + 0x04) as usize;
    let ebx_hash: u32 = if p3_len == 0 {
        0
    } else {
        let mut h: u32 = 0x811c_9dc5;
        for i in 0..p3_len {
            let ch = (rd::<i8>(p3_str.wrapping_add(i)) as i32 as u32) | 0x20;
            h = (ch ^ h).wrapping_mul(0x0100_0193);
        }
        h
    };

    if rd::<usize>(param_1 + 0x30) == 0 {
        return;
    }

    let p2_str = inplace_cstr_inline(param_2);
    let p2_hash = exe::call_va!(0x1_424b_16a0, (p2_str, cstr_len(p2_str)) -> u64);
    let mut key_30: [u64; 2] = [0, p2_hash];

    let mut cur = rd::<usize>(param_1);
    let end = cur.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(0xd0));
    while cur != end {
        if rd::<u32>(cur + 0x58) == ebx_hash {
            return;
        }
        cur = cur.wrapping_add(0xd0);
    }

    let mut p6_slot: u32 = param_6;
    let rbx_elem = exe::call_va!(
        0x1_40f6_5220,
        (
            param_1,
            key_30.as_mut_ptr() as usize,
            param_2,
            param_3,
            param_4,
            (&mut p6_slot as *mut u32) as usize
        ) -> usize
    );
    let mut elem_80: usize = rbx_elem.wrapping_add(0x80);
    let kind = (rd::<i8>(param_5 + 0x48) as i64).wrapping_add(1);
    exe::call_va!(
        0x1_40f6_9070,
        (kind, (&mut elem_80 as *mut usize) as usize, param_5)
    );

    let rdi_str = inplace_cstr_inline(param_3);
    let rcx_30 = rd::<usize>(param_1 + 0x30);
    exe::call_va!(0x1_40f6_4cf0, (rcx_30, rdi_str, rbx_elem));
}

/// `unknown_141e205a0`
/// Original: 0x141e205a0 (1618 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e205a0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: usize,
) {
    wr::<u32>(param_4 + 0x08, 0);
    let lock_ptr = exe::ptr::<u8>(0x1_43a3_26f8) as usize;
    exe::call_iat!(0x1_4264_55a8, (lock_ptr));

    let g_vec = exe::ptr::<u8>(0x1_43a4_5290) as usize;
    let g_ptr0 = rd::<usize>(g_vec);
    let g_len0 = rd::<u32>(g_vec + 0x08) as usize;
    let mut it_beg: usize = g_ptr0;
    let mut it_end: usize = g_ptr0.wrapping_add(g_len0 * 8);
    let diff0 = ((it_end.wrapping_sub(it_beg)) as isize) >> 3;
    exe::call_va!(
        0x1_40f8_17e0,
        (
            (&mut it_beg as *mut usize) as usize,
            (&mut it_end as *mut usize) as usize,
            diff0,
            0u32
        )
    );

    let g_ptr1 = rd::<usize>(g_vec);
    let g_len1 = rd::<u32>(g_vec + 0x08) as usize;
    it_beg = g_ptr1;
    it_end = g_ptr1.wrapping_add(g_len1 * 8);
    let mut new_end: usize = 0;
    exe::call_va!(
        0x1_40f7_d7e0,
        (
            (&mut new_end as *mut usize) as usize,
            (&mut it_beg as *mut usize) as usize,
            (&mut it_end as *mut usize) as usize
        )
    );
    let unique_len = ((new_end.wrapping_sub(rd::<usize>(g_vec))) as isize) >> 3;
    exe::call_va!(0x1_4028_a2a0, (g_vec, unique_len));

    let mut rdi_it = rd::<usize>(g_vec);
    let loop_end = rdi_it.wrapping_add((rd::<u32>(g_vec + 0x08) as usize) * 8);
    while rdi_it != loop_end {
        let handle = rd::<u64>(rdi_it);
        let idx = (handle as u32) as usize;
        let p1_ctx = rd::<usize>(param_1);
        let cap = rd::<u32>(p1_ctx + 0x20) as usize;
        if idx < cap {
            let handles_18 = rd::<usize>(p1_ctx + 0x18);
            let ent = rd::<u64>(handles_18.wrapping_add(idx * 8));
            if (ent as u32) == (handle as u32)
                && ((ent >> 32) as u32) == ((handle >> 32) as u32)
                && (handle as u32) != u32::MAX
            {
                let mut ctx1_copy = rd::<[u64; 3]>(param_1);
                let mut info = [0u64; 10];
                let info_ptr = info.as_mut_ptr() as usize;
                exe::call_va!(
                    0x1_40f7_dc00,
                    (info_ptr, handle, ctx1_copy.as_mut_ptr() as usize)
                );
                if rd::<u8>(info_ptr + 0x48) != 0 {
                    let row = rd::<usize>(info_ptr + 0x40);
                    let off32 = row << 5;
                    let ltf0_base = rd::<usize>(info_ptr + 0x18);
                    let ltf1_base = rd::<usize>(info_ptr + 0x20);
                    let children = rd::<usize>(info_ptr + 0x28).wrapping_add(row << 4);
                    let mut wtf0 = rd::<[f32; 8]>(ltf0_base.wrapping_add(off32));
                    let mut wtf1 = rd::<[f32; 8]>(ltf1_base.wrapping_add(off32));
                    let flags_base = rd::<usize>(info_ptr);
                    let mut bl_flag = rd::<u8>(flags_base.wrapping_add(row));
                    let par_arr = rd::<usize>(info_ptr + 0x30);
                    let raw_par = rd::<u64>(par_arr.wrapping_add(row * 8));
                    let par_h = if (raw_par as u32) != u32::MAX {
                        raw_par
                    } else {
                        u64::MAX
                    };

                    let mut ctx2_copy = rd::<[u64; 3]>(param_2);
                    let mut res_h: u64 = 0;
                    let check_ptr = exe::call_va!(
                        0x1_4020_d4a0,
                        (
                            (&mut res_h as *mut u64) as usize,
                            par_h,
                            ctx2_copy.as_mut_ptr() as usize
                        ) -> usize
                    );
                    if rd::<i32>(check_ptr) != -1 {
                        let mut ctx2_copy2 = rd::<[u64; 3]>(param_2);
                        let r8_par = rd::<u64>(par_arr.wrapping_add(row * 8));
                        let mut par_buf = [0u64; 10];
                        let rax_par = exe::call_va!(
                            0x1_41e2_0fe0,
                            (
                                par_buf.as_mut_ptr() as usize,
                                ctx2_copy2.as_mut_ptr() as usize,
                                r8_par
                            ) -> usize
                        );
                        let par0 = rd::<[f32; 8]>(rax_par);
                        let par1 = rd::<[f32; 8]>(rax_par + 0x20);
                        if rd::<u8>(rax_par + 0x40) != 0 || bl_flag != 0 {
                            bl_flag = 1;
                        }
                        wtf0 = compose_parent_child(par0, wtf0);
                        wtf1 = compose_parent_child(par1, wtf1);
                    }

                    let wtf0_base = rd::<usize>(info_ptr + 0x08);
                    wr::<[f32; 8]>(wtf0_base.wrapping_add(off32), wtf0);
                    let wtf1_base = rd::<usize>(info_ptr + 0x10);
                    wr::<[f32; 8]>(wtf1_base.wrapping_add(off32), wtf1);
                    wr::<u8>(flags_base.wrapping_add(row), bl_flag);

                    let mut child_it = rd::<usize>(children);
                    let child_end = child_it.wrapping_add((rd::<u32>(children + 0x08) as usize) * 8);
                    if child_it != child_end {
                        let mut child_arg = [0u64; 10];
                        let cap_ptr = child_arg.as_mut_ptr() as usize;
                        while child_it != child_end {
                            let mut ctx3_copy = rd::<[u64; 3]>(param_3);
                            wr::<[f32; 8]>(cap_ptr, wtf0);
                            wr::<[f32; 8]>(cap_ptr + 0x20, wtf1);
                            wr::<u8>(cap_ptr + 0x40, bl_flag);
                            let child_h = rd::<u64>(child_it);
                            exe::call_va!(
                                0x1_41e2_0c10,
                                (
                                    cap_ptr,
                                    child_h,
                                    ctx3_copy.as_mut_ptr() as usize,
                                    param_4
                                )
                            );
                            child_it = child_it.wrapping_add(8);
                        }
                    }
                }
            }
        }
        rdi_it = rdi_it.wrapping_add(8);
    }

    let g_ptr2 = rd::<usize>(g_vec);
    let g_len2 = rd::<u32>(g_vec + 0x08) as usize;
    it_beg = g_ptr2;
    it_end = g_ptr2.wrapping_add(g_len2 * 8);
    let p4_end = rd::<usize>(param_4).wrapping_add((rd::<u32>(param_4 + 0x08) as usize) * 8);
    exe::call_va!(
        0x1_40f7_d880,
        (
            param_4,
            p4_end,
            (&mut it_beg as *mut usize) as usize,
            (&mut it_end as *mut usize) as usize
        )
    );

    let p4_len1 = rd::<u32>(param_4 + 0x08) as usize;
    let p4_ptr1 = rd::<usize>(param_4);
    it_beg = p4_ptr1;
    it_end = p4_ptr1.wrapping_add(p4_len1 * 8);
    exe::call_va!(
        0x1_40f8_17e0,
        (
            (&mut it_beg as *mut usize) as usize,
            (&mut it_end as *mut usize) as usize,
            p4_len1 as u64,
            0u32
        )
    );

    let p4_ptr2 = rd::<usize>(param_4);
    let p4_len2 = rd::<u32>(param_4 + 0x08) as usize;
    it_beg = p4_ptr2;
    it_end = p4_ptr2.wrapping_add(p4_len2 * 8);
    let mut p4_new_end: usize = 0;
    exe::call_va!(
        0x1_40f7_d7e0,
        (
            (&mut p4_new_end as *mut usize) as usize,
            (&mut it_beg as *mut usize) as usize,
            (&mut it_end as *mut usize) as usize
        )
    );
    let p4_unique = ((p4_new_end.wrapping_sub(rd::<usize>(param_4))) as isize) >> 3;
    exe::call_va!(0x1_4028_a2a0, (param_4, p4_unique));

    let p4_ptr3 = rd::<usize>(param_4);
    let p4_len3 = rd::<u32>(param_4 + 0x08) as usize;
    it_beg = p4_ptr3;
    it_end = p4_ptr3.wrapping_add(p4_len3 * 8);
    let p5_end = rd::<usize>(param_5).wrapping_add((rd::<u32>(param_5 + 0x08) as usize) * 8);
    exe::call_va!(
        0x1_40f7_d880,
        (
            param_5,
            p5_end,
            (&mut it_beg as *mut usize) as usize,
            (&mut it_end as *mut usize) as usize
        )
    );

    exe::call_iat!(0x1_4264_5598, (lock_ptr));
}

/// `unknown_141e20c10`
/// Original: 0x141e20c10 (945 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e20c10(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
) {
    let ctx = rd::<usize>(param_3);
    let idx = (param_2 as u32) as usize;
    let cap = rd::<u32>(ctx + 0x20) as usize;
    if idx >= cap {
        return;
    }
    let handles_18 = rd::<usize>(ctx + 0x18);
    let ent = rd::<u64>(handles_18.wrapping_add(idx * 8));
    if (ent as u32) != (param_2 as u32)
        || ((ent >> 32) as u32) != ((param_2 >> 32) as u32)
        || (param_2 as u32) == u32::MAX
    {
        return;
    }

    let mut ctx_copy = rd::<[u64; 3]>(param_3);
    let mut info = [0u64; 9];
    let info_ptr = info.as_mut_ptr() as usize;
    exe::call_va!(
        0x1_40f7_da90,
        (info_ptr, param_2, ctx_copy.as_mut_ptr() as usize)
    );
    if rd::<u8>(info_ptr + 0x40) == 0 {
        return;
    }

    let row = rd::<usize>(info_ptr + 0x38);
    let off32 = row << 5;
    let flags_base = rd::<usize>(info_ptr);
    let wtf0_base = rd::<usize>(info_ptr + 0x08);
    let wtf1_base = rd::<usize>(info_ptr + 0x10);
    let ltf0_base = rd::<usize>(info_ptr + 0x18);
    let ltf1_base = rd::<usize>(info_ptr + 0x20);
    let children = rd::<usize>(info_ptr + 0x28).wrapping_add(row << 4);

    let par0 = rd::<[f32; 8]>(param_1);
    let child0 = rd::<[f32; 8]>(ltf0_base.wrapping_add(off32));
    let out0 = compose_parent_child(par0, child0);

    let par1 = rd::<[f32; 8]>(param_1 + 0x20);
    let child1 = rd::<[f32; 8]>(ltf1_base.wrapping_add(off32));
    let out1 = compose_parent_child(par1, child1);

    let al: u8 = if rd::<u8>(param_1 + 0x40) != 0 || rd::<u8>(flags_base.wrapping_add(row)) != 0 {
        1
    } else {
        0
    };

    wr::<[f32; 8]>(wtf0_base.wrapping_add(off32), out0);
    wr::<[f32; 8]>(wtf1_base.wrapping_add(off32), out1);
    wr::<u8>(flags_base.wrapping_add(row), al);

    let mut child_it = rd::<usize>(children);
    let child_end = child_it.wrapping_add((rd::<u32>(children + 0x08) as usize) * 8);
    if child_it != child_end {
        let mut child_arg = [0u64; 10];
        let cap_ptr = child_arg.as_mut_ptr() as usize;
        while child_it != child_end {
            let mut ctx_copy2 = rd::<[u64; 3]>(param_3);
            wr::<[f32; 8]>(cap_ptr, out0);
            wr::<[f32; 8]>(cap_ptr + 0x20, out1);
            wr::<u8>(cap_ptr + 0x40, al);
            let child_h = rd::<u64>(child_it);
            exe::call_va!(
                0x1_41e2_0c10,
                (
                    cap_ptr,
                    child_h,
                    ctx_copy2.as_mut_ptr() as usize,
                    param_4
                )
            );
            child_it = child_it.wrapping_add(8);
        }
    }

    let ent_val = rd::<u64>(rd::<usize>(info_ptr + 0x30).wrapping_add(row * 8 + 0x10));
    let len = rd::<u32>(param_4 + 0x08) as u64;
    let cap_v = rd::<u32>(param_4 + 0x0c) as u64;
    if len * 8 == cap_v * 8 {
        let min_cap = len.wrapping_add(1);
        let mut new_cap = (cap_v >> 1).wrapping_add(cap_v);
        if new_cap < min_cap {
            new_cap = min_cap;
        }
        exe::call_va!(0x1_401d_eba0, (param_4, new_cap));
    }
    let cur_len = rd::<u32>(param_4 + 0x08) as usize;
    let data_ptr = rd::<usize>(param_4);
    wr::<u64>(data_ptr.wrapping_add(cur_len * 8), ent_val);
    wr::<u32>(param_4 + 0x08, (cur_len as u32).wrapping_add(1));
}

/// `unknown_141e20fe0`
/// Original: 0x141e20fe0 (785 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141e20fe0(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let mut buf_40 = [0u64; 10];
    let b40p = buf_40.as_mut_ptr() as usize;
    exe::call_va!(0x1_40f7_bd90, (param_2, b40p, param_3));

    let rdi_row = rd::<usize>(b40p + 0x28);
    let rsi_off32 = rdi_row << 5;
    let r15_flags = rd::<usize>(b40p);
    let r12_tf0 = rd::<usize>(b40p + 0x08);
    let r13_tf1 = rd::<usize>(b40p + 0x10);
    let par_arr = rd::<usize>(b40p + 0x18);

    let raw_par = rd::<u64>(par_arr.wrapping_add(rdi_row * 8));
    let par_h = if (raw_par as u32) == u32::MAX {
        u64::MAX
    } else {
        raw_par
    };
    let mut ctx_copy = rd::<[u64; 3]>(param_2);
    let mut res_h: u64 = u64::MAX;
    let check_ptr = exe::call_va!(
        0x1_4020_d4a0,
        (
            (&mut res_h as *mut u64) as usize,
            par_h,
            ctx_copy.as_mut_ptr() as usize
        ) -> usize
    );

    if rd::<i32>(check_ptr) == -1 {
        wr::<[u64; 4]>(param_1, rd::<[u64; 4]>(r12_tf0.wrapping_add(rsi_off32)));
        wr::<[u64; 4]>(
            param_1 + 0x20,
            rd::<[u64; 4]>(r13_tf1.wrapping_add(rsi_off32)),
        );
        wr::<u8>(param_1 + 0x40, rd::<u8>(r15_flags.wrapping_add(rdi_row)));
        return param_1;
    }

    let mut ctx_copy2 = rd::<[u64; 3]>(param_2);
    let r8_par = rd::<u64>(par_arr.wrapping_add(rdi_row * 8));
    let rax_par = exe::call_va!(
        0x1_41e2_0fe0,
        (b40p, ctx_copy2.as_mut_ptr() as usize, r8_par) -> usize
    );

    let child0 = rd::<[f32; 8]>(r12_tf0.wrapping_add(rsi_off32));
    let par0 = rd::<[f32; 8]>(rax_par);
    let par1 = rd::<[f32; 8]>(rax_par + 0x20);
    let par_flag = rd::<u8>(rax_par + 0x40);

    let out0 = compose_parent_child(par0, child0);
    wr::<[f32; 8]>(param_1, out0);

    let child1 = rd::<[f32; 8]>(r13_tf1.wrapping_add(rsi_off32));
    let out1 = compose_parent_child(par1, child1);
    wr::<[f32; 8]>(param_1 + 0x20, out1);

    let al: u8 = if par_flag != 0 || rd::<u8>(r15_flags.wrapping_add(rdi_row)) != 0 {
        1
    } else {
        0
    };
    wr::<u8>(param_1 + 0x40, al);
    param_1
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_141df8580(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARCH_LIST: [u16; 2] = [0, 1];
    static mut COMP_MEM: [u8; 0x160] = [0; 0x160];
    static mut SUB_STR: [u8; 0x30] = [0; 0x30];
    static mut FMT_BUF: [u8; 64] = [0; 64];
    static mut BUCKET_SLOT: [u64; 6] = [0; 6];
    static mut LOG_OPS: Vec<(u64, u64)> = Vec::new();
    static mut EMPTY_ITER: bool = false;

    unsafe extern "win64" fn mock_f430(_p2: usize, iter_ptr: usize) {
        let arch_p = core::ptr::addr_of!(ARCH_LIST) as usize;
        wr::<usize>(iter_ptr, 0);
        wr::<usize>(iter_ptr + 0x08, 0);
        wr::<usize>(iter_ptr + 0x10, arch_p + 2);
        wr::<usize>(iter_ptr + 0x18, arch_p + 4);
        wr::<u64>(iter_ptr + 0x28, if EMPTY_ITER { 0 } else { 1 });
        wr::<u64>(iter_ptr + 0x30, 0);
        wr::<usize>(iter_ptr + 0x38, core::ptr::addr_of_mut!(COMP_MEM) as usize);
    }
    unsafe extern "win64" fn mock_ae90(out_str: usize, _comp: usize) -> usize {
        wr::<u32>(out_str, 0x0400_0004);
        wr::<u32>(out_str + 0x04, 3);
        for (i, &b) in b"ae9\0".iter().enumerate() {
            wr::<u8>(out_str + 0x08 + i, b);
        }
        out_str
    }
    unsafe extern "win64" fn mock_f6c0(_s: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_16a0(ptr: usize, len: u64) -> u64 {
        let mut h = len;
        for i in 0..(len as usize) {
            h = h.wrapping_mul(31).wrapping_add(rd::<u8>(ptr + i) as u64);
        }
        h
    }
    unsafe extern "win64" fn mock_32a0(dst: usize, _rsi: usize, cstr: usize, len: u64) {
        wr::<u32>(dst, rd::<u8>(cstr) as u32);
        wr::<u64>(dst + 0x08, len);
        wr::<u64>(dst + 0x10, 0x77);
    }
    unsafe extern "win64" fn mock_b860(_rsi: usize, key_ptr: usize) -> usize {
        LOG_OPS.push((0xb860, rd::<u64>(key_ptr + 0x08)));
        core::ptr::addr_of_mut!(BUCKET_SLOT) as usize
    }
    unsafe extern "win64" fn mock_ab10(_s: usize) {}
    unsafe extern "win64" fn mock_aaa0(_comp: usize, _p2: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_acd0(
        _rsi: usize,
        key_ptr: usize,
        _cstr: usize,
        val_ptr: usize,
        _z: u32,
    ) {
        LOG_OPS.push((rd::<u64>(key_ptr + 0x08), rd::<u8>(val_ptr) as u64));
    }
    unsafe extern "win64" fn mock_1a80(
        _rsi: usize,
        key_ptr: usize,
        _cstr: usize,
        val_ptr: usize,
        _z: u32,
    ) {
        LOG_OPS.push((rd::<u64>(key_ptr + 0x08), rd::<u32>(val_ptr) as u64));
    }
    unsafe extern "win64" fn mock_3a20(
        _rsi: usize,
        key_ptr: usize,
        _cstr: usize,
        val_ptr: usize,
        _z: u32,
    ) {
        LOG_OPS.push((rd::<u64>(key_ptr + 0x08), rd::<u64>(val_ptr)));
    }
    unsafe extern "win64" fn mock_dfc0() -> usize {
        core::ptr::addr_of_mut!(FMT_BUF) as usize
    }
    unsafe extern "win64" fn mock_1fb0(ibp: usize, _fstr: usize, _fargs: usize, _z: u64) {
        let out = rd::<usize>(ibp + 0x08);
        wr::<u8>(out, b'f');
        wr::<u8>(out + 1, b'0');
        wr::<u64>(ibp + 0x10, 2);
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, view: usize, _pos: u64, count: u64) {
        let src = rd::<usize>(view);
        wr::<u32>(dst + 0x04, count as u32);
        for i in 0..(count as usize) {
            wr::<u8>(dst + 0x08 + i, rd::<u8>(src + i));
        }
        wr::<u8>(dst + 0x08 + count as usize, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_f430, mock_f430 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41df_ae90, mock_ae90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401d_f6c0, mock_f6c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_424b_16a0, mock_16a0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_41f7_32a0, mock_32a0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401e_b860, mock_b860 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_41df_aaa0, mock_aaa0 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_401e_acd0, mock_acd0 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_401f_1a80, mock_1a80 as *const ())?;
    let _m11 = crate::difftest::Mock::new(0x1_401f_3a20, mock_3a20 as *const ())?;
    let _m12 = crate::difftest::Mock::new(0x1_423f_dfc0, mock_dfc0 as *const ())?;
    let _m13 = crate::difftest::Mock::new(0x1_4146_1fb0, mock_1fb0 as *const ())?;
    let _m14 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;

    let g_e88 = exe::ptr::<u16>(0x1_43c8_ce88) as usize;
    let saved_e88 = rd::<u16>(g_e88);

    let run = |f: F, empty: bool| -> (Vec<(u64, u64)>, u32, [u64; 6]) {
        EMPTY_ITER = empty;
        LOG_OPS.clear();
        COMP_MEM = [0u8; 0x160];
        SUB_STR = [0u8; 0x30];
        FMT_BUF = [0u8; 64];
        BUCKET_SLOT = [0u64; 6];

        wr::<u16>(g_e88, 0);
        let cp = core::ptr::addr_of_mut!(COMP_MEM) as usize;
        let sp = core::ptr::addr_of_mut!(SUB_STR) as usize;

        for &(off, tag) in &[
            (0x28usize, b"k1\0"),
            (0x50, b"k2\0"),
            (0x78, b"k3\0"),
            (0xa0, b"k4\0"),
            (0xf0, b"k5\0"),
        ] {
            wr::<u32>(cp + off, 0x0400_0004);
            wr::<u32>(cp + off + 0x04, 2);
            for (i, &b) in tag.iter().enumerate() {
                wr::<u8>(cp + off + 0x08 + i, b);
            }
        }
        wr::<u32>(cp + 0x11c, 7);
        wr::<usize>(cp + 0x120, sp);
        wr::<u32>(cp + 0x128, 1);

        wr::<u32>(sp, 0x0400_0004);
        wr::<u32>(sp + 0x04, 2);
        wr::<u8>(sp + 0x08, b'A');
        wr::<u8>(sp + 0x09, b'B');
        wr::<u8>(sp + 0x0a, 0);

        let mut rsi_buf = [0u8; 0x120];
        let rsip = rsi_buf.as_mut_ptr() as usize;
        wr::<u64>(rsip + 0x38, 0);
        wr::<u64>(rsip + 0x40, 0);
        wr::<u64>(rsip + 0x48, 2);
        wr::<usize>(rsip + 0x78, 0x1000);
        wr::<usize>(rsip + 0x100, 0x2000);
        wr::<u32>(rsip + 0x108, 0);
        wr::<u32>(rsip + 0x110, 10);

        let tbl: [usize; 1] = [rsip];
        let mut p1 = vec![0u8; 0x580f0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x580e0, tbl.as_ptr() as usize);

        f(p1p, 0x1234);
        wr::<u16>(g_e88, saved_e88);

        (LOG_OPS.clone(), rd::<u32>(rsip + 0x110), BUCKET_SLOT)
    };

    for empty in [true, false] {
        let o = run(f_o, empty);
        let n = run(f_n, empty);
        crate::diff_eq!(o, n, "141df8580 empty={}", empty);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141dfae90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TAG: u64 = 0;

    unsafe extern "win64" fn mock_aa00(_dst: usize, src: usize) {
        CALL_TAG = rd::<u64>(src);
    }
    unsafe extern "win64" fn mock_9b50(_dst: usize, _view: usize, pos: u64, cnt: u64) {
        CALL_TAG = 0x9b50_0000 ^ pos ^ cnt;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_aa00, mock_aa00 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;

    let run = |f: F, idx: i32| -> (bool, u32, u32, u8, u64) {
        CALL_TAG = 0;
        let mut out_s = [0xffu8; 48];
        let mut arr = [0u8; 0x50];
        let mut comp = [0u8; 0x130];

        let osp = out_s.as_mut_ptr() as usize;
        let ap = arr.as_mut_ptr() as usize;
        let cp = comp.as_mut_ptr() as usize;

        wr::<u64>(ap, 0x1111_2222);
        wr::<u64>(ap + 0x28, 0x3333_4444);
        wr::<i32>(cp + 0x11c, idx);
        wr::<usize>(cp + 0x120, ap);
        wr::<u32>(cp + 0x128, 2);

        let ret = f(osp, cp);
        (
            ret == osp,
            rd::<u32>(osp),
            rd::<u32>(osp + 0x04),
            rd::<u8>(osp + 0x08),
            CALL_TAG,
        )
    };

    for idx in [-1i32, 0, 1, 2] {
        let o = run(f_o, idx);
        let n = run(f_n, idx);
        crate::diff_eq!(o, n, "141dfae90 idx={}", idx);
    }
    Ok(4)
}

pub unsafe fn diff_coregame_141e111f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ELEM_BUF: [u8; 0xd0] = [0; 0xd0];
    static mut CALL_LOG: (u64, u32, i64, u8, bool) = (0, 0, 0, 0, false);

    unsafe extern "win64" fn mock_16a0(ptr: usize, len: u64) -> u64 {
        (rd::<u8>(ptr) as u64) ^ (len << 8)
    }
    unsafe extern "win64" fn mock_5220(
        _p1: usize,
        key: usize,
        _p2: usize,
        _p3: usize,
        _p4: usize,
        p6_ptr: usize,
    ) -> usize {
        CALL_LOG.0 = rd::<u64>(key + 0x08);
        CALL_LOG.1 = rd::<u32>(p6_ptr);
        core::ptr::addr_of_mut!(ELEM_BUF) as usize
    }
    unsafe extern "win64" fn mock_9070(kind: i64, elem_80_ptr: usize, _p5: usize) {
        let expected = (core::ptr::addr_of!(ELEM_BUF) as usize).wrapping_add(0x80);
        CALL_LOG.2 = if rd::<usize>(elem_80_ptr) == expected {
            kind
        } else {
            -999
        };
    }
    unsafe extern "win64" fn mock_4cf0(_p1_30: usize, rdi_str: usize, elem: usize) {
        CALL_LOG.3 = rd::<u8>(rdi_str);
        CALL_LOG.4 = elem == (core::ptr::addr_of!(ELEM_BUF) as usize);
    }

    let _m1 = crate::difftest::Mock::new(0x1_424b_16a0, mock_16a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40f6_5220, mock_5220 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_40f6_9070, mock_9070 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40f6_4cf0, mock_4cf0 as *const ())?;

    let run = |f: F, mode: u8| -> (u64, u32, i64, u8, bool) {
        CALL_LOG = (0, 0, 0, 0, false);
        ELEM_BUF = [0u8; 0xd0];

        let mut arr = [0u8; 0xd0];
        let mut p1 = [0u8; 0x40];
        let mut s2 = [0u8; 40];
        let mut s3 = [0u8; 40];
        let mut p5 = [0u8; 0x50];

        let ap = arr.as_mut_ptr() as usize;
        let p1p = p1.as_mut_ptr() as usize;
        let s2p = s2.as_mut_ptr() as usize;
        let s3p = s3.as_mut_ptr() as usize;
        let p5p = p5.as_mut_ptr() as usize;

        wr::<u32>(s2p, 0x0400_0004);
        wr::<u32>(s2p + 0x04, 2);
        wr::<u8>(s2p + 0x08, b'k');
        wr::<u8>(s2p + 0x09, b'y');
        wr::<u8>(s2p + 0x0a, 0);

        wr::<u32>(s3p, 0x0400_0004);
        wr::<u32>(s3p + 0x04, 1);
        wr::<u8>(s3p + 0x08, b'A');
        wr::<u8>(s3p + 0x09, 0);

        let a_hash = ((b'a' as u32) ^ 0x811c_9dc5).wrapping_mul(0x0100_0193);
        wr::<u32>(ap + 0x58, if mode == 1 { a_hash } else { 0x1234 });

        wr::<usize>(p1p, ap);
        wr::<u32>(p1p + 0x08, 1);
        wr::<usize>(p1p + 0x30, if mode == 0 { 0 } else { 0x5555 });
        wr::<i8>(p5p + 0x48, 3);

        f(p1p, s2p, s3p, 0x4444, p5p, 0x7788_99aa);
        CALL_LOG
    };

    for mode in 0..3u8 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141e111f0 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141e205a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FLAGS_MEM: [u8; 2] = [0; 2];
    static mut WTF0_MEM: [f32; 8] = [0.0; 8];
    static mut WTF1_MEM: [f32; 8] = [0.0; 8];
    static mut LTF0_MEM: [f32; 8] = [0.0, 0.70710677, 0.0, 0.70710677, 1.0, 2.0, 3.0, 0.0];
    static mut LTF1_MEM: [f32; 8] = [0.0, 0.0, 0.70710677, 0.70710677, 4.0, 5.0, 6.0, 0.0];
    static mut CHILD_ARR: [u64; 1] = [0x0001_0000_0002];
    static mut CHILDREN_SPAN: [u64; 2] = [0, 1];
    static mut PAR_ARR: [u64; 1] = [0x0001_0000_0001];
    static mut HAS_PARENT: bool = false;
    static mut CHILD_LOG: u64 = 0;
    static mut APPEND_CNT: u32 = 0;

    unsafe extern "win64" fn mock_17e0(_b: usize, _e: usize, _cnt: isize, _z: u32) {}
    unsafe extern "win64" fn mock_d7e0(out_end: usize, _b: usize, end_ptr: usize) -> usize {
        wr::<usize>(out_end, rd::<usize>(end_ptr));
        out_end
    }
    unsafe extern "win64" fn mock_a2a0(vec_ptr: usize, new_len: isize) {
        wr::<u32>(vec_ptr + 0x08, new_len as u32);
    }
    unsafe extern "win64" fn mock_dc00(info_ptr: usize, _h: u64, _ctx: usize) -> usize {
        wr::<usize>(info_ptr, core::ptr::addr_of_mut!(FLAGS_MEM) as usize);
        wr::<usize>(info_ptr + 0x08, core::ptr::addr_of_mut!(WTF0_MEM) as usize);
        wr::<usize>(info_ptr + 0x10, core::ptr::addr_of_mut!(WTF1_MEM) as usize);
        wr::<usize>(info_ptr + 0x18, core::ptr::addr_of_mut!(LTF0_MEM) as usize);
        wr::<usize>(info_ptr + 0x20, core::ptr::addr_of_mut!(LTF1_MEM) as usize);
        wr::<usize>(
            info_ptr + 0x28,
            core::ptr::addr_of_mut!(CHILDREN_SPAN) as usize,
        );
        wr::<usize>(info_ptr + 0x30, core::ptr::addr_of_mut!(PAR_ARR) as usize);
        wr::<usize>(info_ptr + 0x40, 0);
        wr::<u8>(info_ptr + 0x48, 1);
        info_ptr
    }
    unsafe extern "win64" fn mock_d4a0(out_ptr: usize, _h: u64, _ctx: usize) -> usize {
        wr::<i32>(out_ptr, if HAS_PARENT { 1 } else { -1 });
        out_ptr
    }
    unsafe extern "win64" fn mock_0fe0(out_ptr: usize, _ctx: usize, _h: u64) -> usize {
        wr::<[f32; 8]>(
            out_ptr,
            [0.0, 0.0, 0.70710677, 0.70710677, 10.0, 20.0, 30.0, 0.0],
        );
        wr::<[f32; 8]>(
            out_ptr + 0x20,
            [0.70710677, 0.0, 0.0, 0.70710677, -1.0, -2.0, -3.0, 0.0],
        );
        wr::<u8>(out_ptr + 0x40, 1);
        out_ptr
    }
    unsafe extern "win64" fn mock_0c10(tf_ptr: usize, child_h: u64, _ctx: usize, _p4: usize) {
        CHILD_LOG = child_h ^ (rd::<u32>(tf_ptr + 0x10) as u64) ^ (rd::<u8>(tf_ptr + 0x40) as u64);
    }
    unsafe extern "win64" fn mock_d880(_vec: usize, _pos: usize, b: usize, e: usize) {
        let cnt = ((rd::<usize>(e).wrapping_sub(rd::<usize>(b))) >> 3) as u32;
        APPEND_CNT = APPEND_CNT.wrapping_add(cnt);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f8_17e0, mock_17e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40f7_d7e0, mock_d7e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4028_a2a0, mock_a2a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40f7_dc00, mock_dc00 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4020_d4a0, mock_d4a0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_41e2_0fe0, mock_0fe0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_41e2_0c10, mock_0c10 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_40f7_d880, mock_d880 as *const ())?;

    let g_vec = exe::ptr::<u8>(0x1_43a4_5290) as usize;
    let saved_g = rd::<[u64; 2]>(g_vec);

    let run = |f: F, has_par: bool| -> ([u32; 8], [u32; 8], u8, u64, u32) {
        HAS_PARENT = has_par;
        FLAGS_MEM = [0, 0];
        WTF0_MEM = [0.0; 8];
        WTF1_MEM = [0.0; 8];
        CHILDREN_SPAN[0] = core::ptr::addr_of!(CHILD_ARR) as usize as u64;
        CHILDREN_SPAN[1] = 1;
        CHILD_LOG = 0;
        APPEND_CNT = 0;

        let mut g_items: [u64; 1] = [0x0001_0000_0000];
        wr::<usize>(g_vec, g_items.as_mut_ptr() as usize);
        wr::<u32>(g_vec + 0x08, 1);
        wr::<u32>(g_vec + 0x0c, 1);

        let ent_table: [u64; 1] = [0x0001_0000_0000];
        let mut scene1 = [0u8; 0x30];
        let s1p = scene1.as_mut_ptr() as usize;
        wr::<usize>(s1p + 0x18, ent_table.as_ptr() as usize);
        wr::<u32>(s1p + 0x20, 1);

        let p1: [u64; 3] = [s1p as u64, 0, 0];
        let p2: [u64; 3] = [s1p as u64, 0, 0];
        let p3: [u64; 3] = [s1p as u64, 0, 0];
        let mut v4_buf: [u64; 2] = [0; 2];
        let mut p4: [u64; 2] = [v4_buf.as_mut_ptr() as u64, 0x0000_0002_0000_0001];
        let mut p5: [u64; 2] = [v4_buf.as_mut_ptr() as u64, 0x0000_0002_0000_0000];

        f(
            p1.as_ptr() as usize,
            p2.as_ptr() as usize,
            p3.as_ptr() as usize,
            p4.as_mut_ptr() as usize,
            p5.as_mut_ptr() as usize,
        );

        wr::<[u64; 2]>(g_vec, saved_g);
        (
            rd::<[u32; 8]>(core::ptr::addr_of!(WTF0_MEM) as usize),
            rd::<[u32; 8]>(core::ptr::addr_of!(WTF1_MEM) as usize),
            FLAGS_MEM[0],
            CHILD_LOG,
            APPEND_CNT,
        )
    };

    for has_par in [false, true] {
        let o = run(f_o, has_par);
        let n = run(f_n, has_par);
        crate::diff_eq!(o, n, "141e205a0 has_par={}", has_par);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141e20c10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FLAGS_C10: [u8; 2] = [0; 2];
    static mut WTF0_C10: [f32; 8] = [0.0; 8];
    static mut WTF1_C10: [f32; 8] = [0.0; 8];
    static mut LTF0_C10: [f32; 8] = [0.0, 0.70710677, 0.0, 0.70710677, 1.0, 2.0, 3.0, 0.0];
    static mut LTF1_C10: [f32; 8] = [0.0, 0.0, 0.70710677, 0.70710677, 4.0, 5.0, 6.0, 0.0];
    static mut EMPTY_SPAN_C10: [u64; 2] = [0, 0];
    static mut ENT_OUT_TBL: [u64; 4] = [0, 0, 0xdead_beef_cafe_babe, 0];

    unsafe extern "win64" fn mock_da90(info_ptr: usize, _h: u64, _ctx: usize) -> usize {
        wr::<usize>(info_ptr, core::ptr::addr_of_mut!(FLAGS_C10) as usize);
        wr::<usize>(info_ptr + 0x08, core::ptr::addr_of_mut!(WTF0_C10) as usize);
        wr::<usize>(info_ptr + 0x10, core::ptr::addr_of_mut!(WTF1_C10) as usize);
        wr::<usize>(info_ptr + 0x18, core::ptr::addr_of_mut!(LTF0_C10) as usize);
        wr::<usize>(info_ptr + 0x20, core::ptr::addr_of_mut!(LTF1_C10) as usize);
        wr::<usize>(
            info_ptr + 0x28,
            core::ptr::addr_of_mut!(EMPTY_SPAN_C10) as usize,
        );
        wr::<usize>(
            info_ptr + 0x30,
            core::ptr::addr_of_mut!(ENT_OUT_TBL) as usize,
        );
        wr::<usize>(info_ptr + 0x38, 0);
        wr::<u8>(info_ptr + 0x40, 1);
        info_ptr
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f7_da90, mock_da90 as *const ())?;

    let run = |f: F, valid: bool| -> ([u32; 8], [u32; 8], u8, u32, u64) {
        FLAGS_C10 = [0, 0];
        WTF0_C10 = [0.0; 8];
        WTF1_C10 = [0.0; 8];

        let mut p1_buf = [0u64; 10];
        let p1p = p1_buf.as_mut_ptr() as usize;
        wr::<[f32; 8]>(
            p1p,
            [0.0, 0.0, 0.70710677, 0.70710677, 10.0, 20.0, 30.0, 0.0],
        );
        wr::<[f32; 8]>(
            p1p + 0x20,
            [0.70710677, 0.0, 0.0, 0.70710677, -1.0, -2.0, -3.0, 0.0],
        );
        wr::<u8>(p1p + 0x40, 1);

        let ent_18: [u64; 1] = [0x0002_0000_0000];
        let mut scene = [0u8; 0x30];
        let sp = scene.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x18, ent_18.as_ptr() as usize);
        wr::<u32>(sp + 0x20, 1);
        let p3: [u64; 3] = [sp as u64, 0, 0];

        let mut out_items: [u64; 2] = [0; 2];
        let mut p4: [u64; 2] = [out_items.as_mut_ptr() as u64, 0x0000_0002_0000_0000];

        let h = if valid { 0x0002_0000_0000 } else { u64::MAX };
        f(p1p, h, p3.as_ptr() as usize, p4.as_mut_ptr() as usize);

        (
            rd::<[u32; 8]>(core::ptr::addr_of!(WTF0_C10) as usize),
            rd::<[u32; 8]>(core::ptr::addr_of!(WTF1_C10) as usize),
            FLAGS_C10[0],
            rd::<u32>((p4.as_ptr() as usize) + 0x08),
            out_items[0],
        )
    };

    for valid in [false, true] {
        let o = run(f_o, valid);
        let n = run(f_n, valid);
        crate::diff_eq!(o, n, "141e20c10 valid={}", valid);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141e20fe0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FLAGS_FE0: [u8; 2] = [0, 1];
    static mut TF0_FE0: [f32; 16] = [
        0.0, 0.70710677, 0.0, 0.70710677, 1.0, 2.0, 3.0, 0.0, // row 0 (child)
        0.0, 0.0, 0.70710677, 0.70710677, 10.0, 20.0, 30.0, 0.0, // row 1 (parent)
    ];
    static mut TF1_FE0: [f32; 16] = [
        0.0, 0.0, 0.70710677, 0.70710677, 4.0, 5.0, 6.0, 0.0, // row 0 (child)
        0.70710677, 0.0, 0.0, 0.70710677, -1.0, -2.0, -3.0, 0.0, // row 1 (parent)
    ];
    static mut PAR_FE0: [u64; 2] = [1, u64::MAX];

    unsafe extern "win64" fn mock_bd90(_p2: usize, b40p: usize, h: u64) {
        wr::<usize>(b40p, core::ptr::addr_of!(FLAGS_FE0) as usize);
        wr::<usize>(b40p + 0x08, core::ptr::addr_of!(TF0_FE0) as usize);
        wr::<usize>(b40p + 0x10, core::ptr::addr_of!(TF1_FE0) as usize);
        wr::<usize>(b40p + 0x18, core::ptr::addr_of!(PAR_FE0) as usize);
        wr::<usize>(b40p + 0x28, (h & 1) as usize);
    }
    unsafe extern "win64" fn mock_d4a0(out_ptr: usize, par_h: u64, _ctx: usize) -> usize {
        wr::<i32>(out_ptr, if par_h == u64::MAX { -1 } else { 1 });
        out_ptr
    }

    let _m1 = crate::difftest::Mock::new(0x1_40f7_bd90, mock_bd90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4020_d4a0, mock_d4a0 as *const ())?;

    let run = |f: F, start_h: u64| -> (bool, [u32; 8], [u32; 8], u8) {
        let mut out_buf = [0u64; 10];
        let outp = out_buf.as_mut_ptr() as usize;
        let p2: [u64; 3] = [0x1111, 0x2222, 0x3333];

        let ret = f(outp, p2.as_ptr() as usize, start_h);
        (
            ret == outp,
            rd::<[u32; 8]>(outp),
            rd::<[u32; 8]>(outp + 0x20),
            rd::<u8>(outp + 0x40),
        )
    };

    for start_h in [1u64, 0] {
        let o = run(f_o, start_h);
        let n = run(f_n, start_h);
        crate::diff_eq!(o, n, "141e20fe0 start_h={}", start_h);
    }
    Ok(2)
}
