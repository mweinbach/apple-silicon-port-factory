//! Decompiled `rend` hot functions shard Z1B (`0x140fbf1e0..0x1411ed7e0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    atomic_add_u32, atomic_xchg_u64, cvt_f32_i64, rd_acq_u32, rd_acq_u64, rd_acq_usize,
    vpbroadcastb, vpcmpeqb, vpmovmskb, vpsignb, wr_rel_u64,
};

#[inline(always)]
fn u64_to_f32_msvc(u: u64) -> f32 {
    if (u as i64) < 0 {
        let half = ((u >> 1) | (u & 1)) as i64 as f32;
        half + half
    } else {
        (u as i64) as f32
    }
}

#[inline(always)]
fn f32_to_u64_msvc(mut v: f32) -> u64 {
    const TWO63: f32 = f32::from_bits(0x5f00_0000);
    let mut adj: u64 = 0;
    if v >= TWO63 {
        v -= TWO63;
        if v < TWO63 {
            adj = 0x8000_0000_0000_0000;
        }
    }
    cvt_f32_i64(v).wrapping_add(adj)
}

#[inline(always)]
unsafe fn fnv1a_u64_8(addr: usize) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut i = 0usize;
    while i < 8 {
        h ^= rd::<u8>(addr + i) as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
        i += 1;
    }
    h
}

/// `unknown_140fbf1e0`
/// Original: 0x140fbf1e0 (349 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fbf1e0(param_1: usize, param_2: usize) {
    let a0 = rd::<[u64; 4]>(param_1);
    let b0 = rd::<[u64; 4]>(param_2);
    wr::<[u64; 4]>(param_1, b0);
    wr::<[u64; 4]>(param_2, a0);

    for off in [0x28usize, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58] {
        let va = rd::<u64>(param_1 + off);
        let vb = rd::<u64>(param_2 + off);
        wr::<u64>(param_1 + off, vb);
        wr::<u64>(param_2 + off, va);
    }

    let cap_f = u64_to_f32_msvc(rd::<u64>(param_1 + 0x40));

    let p1_v0 = f32_to_u64_msvc(cap_f * rd::<f32>(param_1 + 0x10));
    let p1_v1 = f32_to_u64_msvc(cap_f * rd::<f32>(param_1 + 0x14));
    wr::<u64>(param_1, p1_v0);
    wr::<u8>(param_1 + 0x18, 0);
    wr::<u64>(param_1 + 0x08, p1_v1);

    let p2_v0 = f32_to_u64_msvc(cap_f * rd::<f32>(param_2 + 0x10));
    let p2_v1 = f32_to_u64_msvc(cap_f * rd::<f32>(param_2 + 0x14));
    wr::<u64>(param_2, p2_v0);
    wr::<u8>(param_2 + 0x18, 0);
    wr::<u64>(param_2 + 0x08, p2_v1);
}

/// `sparsehash_140fbf4b0`
/// Original: 0x140fbf4b0 (401 bytes, Throws::Discard)
pub unsafe extern "win64" fn sparsehash_140fbf4b0(param_1: usize, param_2: usize, param_3: u64) {
    let diff = rd::<u64>(param_2 + 0x38).wrapping_sub(rd::<u64>(param_2 + 0x30));
    let new_buckets = exe::call_va!(0x1_401e_2020, (param_1, diff, param_3) -> u64);
    exe::call_va!(0x1_40fb_f650, (param_1, new_buckets));

    let mut cur = rd::<usize>(param_2 + 0x58);
    let end = cur.wrapping_add((rd::<u64>(param_2 + 0x40) as usize).wrapping_shl(4));
    if cur != end {
        let empty2 = rd::<u64>(param_2 + 0x48);
        while cur != end {
            let k = rd::<u64>(cur);
            if empty2 != k && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != k) {
                break;
            }
            cur = cur.wrapping_add(0x10);
        }
    }

    loop {
        let end_check = rd::<usize>(param_2 + 0x58)
            .wrapping_add((rd::<u64>(param_2 + 0x40) as usize).wrapping_shl(4));
        if cur == end_check {
            wr::<u32>(param_1 + 0x1c, rd::<u32>(param_1 + 0x1c).wrapping_add(1));
            return;
        }

        let mask1 = rd::<u64>(param_1 + 0x40).wrapping_sub(1);
        let table1 = rd::<usize>(param_1 + 0x58);
        let empty1 = rd::<u64>(param_1 + 0x48);
        let mut idx = fnv1a_u64_8(cur) & mask1;
        let mut probe: u64 = 0;
        while rd::<u64>(table1.wrapping_add((idx as usize) * 0x10)) != empty1 {
            probe = probe.wrapping_add(1);
            idx = idx.wrapping_add(probe) & mask1;
        }

        let entry = rd::<[u64; 2]>(cur);
        wr::<[u64; 2]>(table1.wrapping_add((idx as usize) * 0x10), entry);
        wr::<u64>(param_1 + 0x38, rd::<u64>(param_1 + 0x38).wrapping_add(1));

        loop {
            cur = cur.wrapping_add(0x10);
            if cur == end {
                break;
            }
            let k = rd::<u64>(cur);
            if rd::<u64>(param_2 + 0x48) != k
                && (rd::<u64>(param_2 + 0x30) == 0 || rd::<u64>(param_2 + 0x28) != k)
            {
                break;
            }
        }
    }
}

/// `unknown_140fbf650`
/// Original: 0x140fbf650 (375 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fbf650(param_1: usize, param_2: u64) {
    let mut table = rd::<usize>(param_1 + 0x58);
    let mut need_alloc = true;
    if table != 0 {
        if param_2 == rd::<u64>(param_1 + 0x40) {
            need_alloc = false;
        } else {
            let chunk_base = table.wrapping_sub(1) & !0x01ff_ffffusize;
            let page_idx = (table.wrapping_sub(chunk_base)) >> 16;
            let desc_hdr = chunk_base
                .wrapping_add(0x108)
                .wrapping_add(page_idx.wrapping_mul(0x50));
            let desc = desc_hdr.wrapping_sub(rd::<u32>(desc_hdr + 0x04) as usize);
            if rd::<usize>(chunk_base + 0x100) == exe::teb() {
                if rd::<u8>(desc + 0x0e) == 0 {
                    wr::<usize>(table, rd::<usize>(desc + 0x20));
                    let new_cnt = rd::<u32>(desc + 0x18).wrapping_sub(1);
                    wr::<u32>(desc + 0x18, new_cnt);
                    wr::<usize>(desc + 0x20, table);
                    if new_cnt == 0 {
                        exe::call_va!(0x1_4253_71a0, (desc));
                    }
                } else {
                    let flag = ((table as u64) & !0xffu64) | 1;
                    exe::call_va!(0x1_4253_3200, (chunk_base, desc, flag, table));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (chunk_base, desc, 0u64, table));
            }
        }
    }

    if need_alloc {
        table = exe::call_va!(0x1_4021_5e80, (param_1, param_2) -> usize);
        wr::<usize>(param_1 + 0x58, table);
    }

    let end = table.wrapping_add((param_2 as usize).wrapping_shl(4));
    let mut cur = table;
    while cur != end {
        let empty_pair = rd::<[u64; 2]>(param_1 + 0x48);
        wr::<[u64; 2]>(cur, empty_pair);
        cur = cur.wrapping_add(0x10);
    }

    wr::<u64>(param_1 + 0x40, param_2);
    wr::<u64>(param_1 + 0x38, 0);
    wr::<u64>(param_1 + 0x30, 0);

    let cap_f = u64_to_f32_msvc(param_2);
    let v0 = f32_to_u64_msvc(cap_f * rd::<f32>(param_1 + 0x10));
    let v1 = f32_to_u64_msvc(cap_f * rd::<f32>(param_1 + 0x14));
    wr::<u64>(param_1, v0);
    wr::<u8>(param_1 + 0x18, 0);
    wr::<u64>(param_1 + 0x08, v1);
}

/// `unknown_14104e610`
/// Original: 0x14104e610 (508 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14104e610(param_1: usize, param_2: usize) -> u32 {
    wr::<u64>(param_2 + 0x08, u64::MAX);
    let vec_ptr = param_1 + 0xc0;
    let free_cnt = rd::<u32>(param_1 + 0xd8);
    let idx: u32;
    if free_cnt != 0 {
        let free_arr = rd::<usize>(param_1 + 0xd0);
        let next_free = free_cnt.wrapping_sub(1);
        idx = rd::<u32>(free_arr.wrapping_add((next_free as usize) * 4));
        wr::<u32>(param_1 + 0xd8, next_free);
        let dst = rd::<usize>(vec_ptr).wrapping_add((idx as usize) << 5);
        wr::<u32>(dst, rd::<u32>(param_2));
        wr::<u32>(dst + 0x04, rd::<u32>(param_2 + 0x04));
        wr::<u32>(dst + 0x08, rd::<u32>(param_2 + 0x08));
        wr::<u32>(dst + 0x0c, rd::<u32>(param_2 + 0x0c));
        wr::<u64>(dst + 0x10, rd::<u64>(param_2 + 0x10));
        wr::<u64>(dst + 0x18, rd::<u64>(param_2 + 0x18));
    } else {
        idx = rd::<u32>(param_1 + 0xc8);
        let cur_len = rd::<u32>(vec_ptr + 0x08);
        let cap = rd::<u32>(vec_ptr + 0x0c) & 0x3fff_ffff;
        let dst = rd::<usize>(vec_ptr).wrapping_add((cur_len as usize) << 5);
        if cur_len == cap {
            let mut slot_ptr: usize = dst;
            exe::call_va!(
                0x1_4105_1e30,
                (vec_ptr, (&mut slot_ptr as *mut usize) as usize, param_2)
            );
        } else {
            wr::<u32>(vec_ptr + 0x08, cur_len.wrapping_add(1));
            wr::<[u64; 4]>(dst, rd::<[u64; 4]>(param_2));
        }
    }

    let mut iter_a = [0u64; 9];
    let mut iter_b = [0u64; 9];
    exe::call_va!(
        0x1_4104_ebe0,
        (param_1, iter_a.as_mut_ptr() as usize, param_2 + 0x10)
    );
    exe::call_va!(0x1_4104_eeb0, (param_1, iter_b.as_mut_ptr() as usize));
    if iter_a[1] != iter_b[1]
        || iter_a[2] != iter_b[2]
        || iter_a[3] != iter_b[3]
        || (iter_a[3] != iter_a[2] && iter_a[4] != iter_b[4])
    {
        let base = rd::<usize>(vec_ptr);
        let prev_val = rd::<u32>((iter_a[4] as usize) + 0x08);
        wr::<u32>(
            base.wrapping_add(((idx & 0x00ff_ffff) as usize) << 5) + 0x08,
            prev_val,
        );
    }

    let slot1 = exe::call_va!(0x1_4104_ee00, (param_1, param_2 + 0x10) -> usize);
    wr::<u32>(slot1, idx);

    let map2 = param_1 + 0x60;
    exe::call_va!(
        0x1_4104_ebe0,
        (map2, iter_b.as_mut_ptr() as usize, param_2 + 0x18)
    );
    exe::call_va!(0x1_4104_eeb0, (map2, iter_a.as_mut_ptr() as usize));
    if iter_b[1] != iter_a[1]
        || iter_b[2] != iter_a[2]
        || iter_b[3] != iter_a[3]
        || (iter_b[3] != iter_b[2] && iter_b[4] != iter_a[4])
    {
        let base = rd::<usize>(param_1 + 0xc0);
        let prev_val = rd::<u32>((iter_b[4] as usize) + 0x08);
        wr::<u32>(
            base.wrapping_add(((idx & 0x00ff_ffff) as usize) << 5) + 0x0c,
            prev_val,
        );
    }

    let slot2 = exe::call_va!(0x1_4104_eb30, (map2, param_2 + 0x18) -> usize);
    wr::<u32>(slot2, idx);
    idx
}

/// `sparsehash_14105a770`
/// Original: 0x14105a770 (426 bytes, Throws::Discard)
pub unsafe extern "win64" fn sparsehash_14105a770(param_1: usize, param_2: usize) -> usize {
    let mut pos: [i64; 2] = [0; 2];
    exe::call_va!(
        0x1_401f_a730,
        (param_1, pos.as_mut_ptr() as usize, param_2)
    );
    if pos[0] != -1 {
        return rd::<usize>(param_1 + 0x58).wrapping_add((pos[0] as usize) << 4);
    }

    let resized = exe::call_va!(0x1_4105_a960, (param_1) -> u8);
    let mut kv: [u64; 2] = [rd::<u64>(param_2), 0];
    if resized != 0 {
        exe::call_va!(
            0x1_401f_a730,
            (param_1, pos.as_mut_ptr() as usize, kv.as_mut_ptr() as usize)
        );
        if pos[0] != -1 {
            return rd::<usize>(param_1 + 0x58).wrapping_add((pos[0] as usize) << 4);
        }
    }

    let used = rd::<u64>(param_1 + 0x38);
    let deleted = rd::<u64>(param_1 + 0x30);
    if used.wrapping_sub(deleted) >= 0x0fff_ffff_ffff_ffff {
        let mut exc = [0u64; 4];
        let msg = exe::ptr::<u8>(0x1_4307_aa70) as usize;
        exe::call_va!(0x1_401d_f630, (exc.as_mut_ptr() as usize, msg));
        let ti = exe::ptr::<u8>(0x1_437f_cf88) as usize;
        exe::call_va!(0x1_4183_acce, (exc.as_mut_ptr() as usize, ti));
        return 0;
    }

    let ins_idx = pos[1] as usize;
    let table = rd::<usize>(param_1 + 0x58);
    if deleted != 0 && rd::<u64>(table.wrapping_add(ins_idx * 0x10)) == rd::<u64>(param_1 + 0x28) {
        wr::<u64>(param_1 + 0x30, deleted.wrapping_sub(1));
    } else {
        wr::<u64>(param_1 + 0x38, used.wrapping_add(1));
    }

    let dst = rd::<usize>(param_1 + 0x58).wrapping_add(ins_idx << 4);
    wr::<[u64; 2]>(dst, kv);
    rd::<usize>(param_1 + 0x58).wrapping_add(ins_idx << 4)
}

/// `unknown_141068680`
/// Original: 0x141068680 (478 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141068680(param_1: usize) -> usize {
    let m00 = rd::<f32>(param_1);
    let m01 = rd::<f32>(param_1 + 0x04);
    let m02 = rd::<f32>(param_1 + 0x08);
    let m03 = rd::<f32>(param_1 + 0x0c);
    let m10 = rd::<f32>(param_1 + 0x10);
    let m11 = rd::<f32>(param_1 + 0x14);
    let m12 = rd::<f32>(param_1 + 0x18);
    let m13 = rd::<f32>(param_1 + 0x1c);
    let m20 = rd::<f32>(param_1 + 0x20);
    let m21 = rd::<f32>(param_1 + 0x24);
    let m22 = rd::<f32>(param_1 + 0x28);
    let m23 = rd::<f32>(param_1 + 0x2c);
    let m30 = rd::<f32>(param_1 + 0x30);
    let m31 = rd::<f32>(param_1 + 0x34);
    let m32 = rd::<f32>(param_1 + 0x38);
    let m33 = rd::<f32>(param_1 + 0x3c);

    let s0 = m33 * m22 - m32 * m23;
    let s1 = m33 * m21 - m31 * m23;
    let s2 = m32 * m21 - m31 * m22;
    let s3 = m33 * m20 - m30 * m23;
    let s4 = m31 * m20 - m30 * m21;
    let s5 = m32 * m20 - m30 * m22;

    let c0 = (s0 * m11 - s1 * m12) + s2 * m13;
    let c1 = (s0 * m10 - s3 * m12) + s5 * m13;
    let c2 = (s1 * m10 - s3 * m11) + s4 * m13;
    let c3 = (s2 * m10 - s5 * m11) + s4 * m12;

    let det = ((c0 * m00 - c1 * m01) + c2 * m02) - c3 * m03;
    let inv_det = if !det.is_nan() && det == 0.0 {
        0.0f32
    } else {
        1.0f32 / det
    };

    exe::call_va!(0x1_4106_9350, (param_1));
    for i in 0..16usize {
        let off = i * 4;
        wr::<f32>(param_1 + off, inv_det * rd::<f32>(param_1 + off));
    }
    param_1
}

/// `unknown_141092770`
/// Original: 0x141092770 (352 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141092770(param_1: usize, param_2: u64) {
    let r8 = rd::<usize>(param_1);
    let idx = (param_2 & 0xffff) as usize;
    let r10 = rd::<u64>(r8 + idx * 8 + 0x88);
    let r9d = if r10 == 0 {
        0u32
    } else {
        rd::<u32>(r8 + idx * 4 + 0x10088)
    };
    if r9d != 0 {
        let tbl = rd::<usize>(r8 + 0x580e0);
        let idx_a = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d904) as usize) as usize;
        let idx_b = rd::<u16>(exe::ptr::<u16>(0x1_43c9_04a0) as usize) as usize;
        let rdi = rd::<u64>(tbl + idx_b * 8);
        let r13 = rd::<u64>(tbl + idx_a * 8);

        let mut buf60 = [0u64; 8];
        exe::call_va!(
            0x1_4109_35a0,
            (buf60.as_mut_ptr() as usize, param_2, r10, r9d, r8)
        );
        let count = buf60[3];
        if count != 0 {
            let rbx = buf60[4];
            let rbp = buf60[6];
            let r14 = buf60[5];
            let r15 = buf60[1];
            for i in 0..count {
                buf60[0] = rbx;
                buf60[1] = r14;
                buf60[2] = rbp;
                buf60[3] = r15;
                buf60[4] = i;
                exe::call_va!(0x1_41f2_88b0, (buf60.as_mut_ptr() as usize, rdi, r13));
            }
        }
    }
}

/// `unknown_1410d6540`
/// Original: 0x1410d6540 (281 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1410d6540(param_1: usize, param_2: u64) {
    let old_cap = rd::<u32>(param_1 + 0x0c) as u64;
    let old_len = rd::<u32>(param_1 + 0x08);
    let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
    if new_cap < param_2 {
        new_cap = param_2;
    }

    let new_buf = exe::call_va!(0x1_4241_c0a0, (new_cap.wrapping_mul(0x28), 8u32) -> usize);
    let mut src = rd::<usize>(param_1);
    let end = src.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(0x28));
    if src != end {
        let mut dst = new_buf;
        while src != end {
            wr::<u64>(dst, 0);
            wr::<u64>(dst + 0x08, 0);
            wr::<u32>(dst + 0x10, 0);
            wr::<u8>(dst + 0x14, 0);
            exe::call_va!(0x1_401e_84a0, (dst, src));
            wr::<u8>(dst + 0x18, rd::<u8>(src + 0x18));
            wr::<u8>(dst + 0x19, rd::<u8>(src + 0x19));
            wr::<u32>(dst + 0x1c, rd::<u32>(src + 0x1c));
            wr::<u32>(dst + 0x20, rd::<u32>(src + 0x20));
            wr::<u32>(dst + 0x24, rd::<u32>(src + 0x24));
            dst = dst.wrapping_add(0x28);
            src = src.wrapping_add(0x28);
        }
    }

    let mut cur = rd::<usize>(param_1);
    let end2 = cur.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(0x28));
    while cur != end2 {
        atomic_xchg_u64((cur + 0x08) as u64, 0);
        cur = cur.wrapping_add(0x28);
    }

    let old_buf = rd::<usize>(param_1);
    if old_buf != 0 {
        exe::call_va!(0x1_4241_c190, (old_buf));
    }
    wr::<usize>(param_1, new_buf);
    wr::<u32>(param_1 + 0x08, old_len);
    wr::<u32>(param_1 + 0x0c, new_cap as u32);
}

/// `unknown_1410d6c60`
/// Original: 0x1410d6c60 (51 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1410d6c60(param_1: usize) -> u64 {
    let rdx = rd::<usize>(param_1);
    let tbl = rd::<usize>(rdx + 0x580e0);
    let i_028 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d028) as usize) as usize;
    let r9 = rd::<u64>(tbl + i_028 * 8);
    let i_080 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d080) as usize) as usize;
    let r8 = rd::<u64>(tbl + i_080 * 8);
    let i_170 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d170) as usize) as usize;
    let ptr_170 = rd::<usize>(tbl + i_170 * 8);
    let rcx = rd::<u64>(ptr_170);
    exe::call_va!(0x1_41f5_a270, (rcx, rdx, r8, r9) -> u64)
}

/// `unknown_14110b9a0`
/// Original: 0x14110b9a0 (356 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14110b9a0(param_1: usize) {
    let g_ptr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let g = rd::<usize>(g_ptr);
    let rdi = rd::<usize>(g + 0x168);
    exe::call_iat!(0x1_4264_55a8, (rdi + 0x18));

    let mut rax = rd_acq_usize(rdi + 0x20);
    let mut rsi = rd_acq_usize(rdi + 0x28);
    let mut r8 = rsi.wrapping_add(0x18);
    let end = rd::<usize>(rdi + 0x10);
    if r8 > end {
        while rsi <= rax && r8 > rax && rd_acq_u32(rdi + 0x30) != rd_acq_u32(rdi + 0x34) {
            core::hint::spin_loop();
            rax = rd_acq_usize(rdi + 0x20);
        }
        if rsi <= end.wrapping_sub(8) {
            wr_rel_u64(rsi, 0x00ff_ffff_ffff);
        }
        rsi = rd::<usize>(rdi + 0x08);
        r8 = rsi.wrapping_add(0x18);
    }

    while rsi <= rax && r8 > rax && rd_acq_u32(rdi + 0x30) != rd_acq_u32(rdi + 0x34) {
        core::hint::spin_loop();
        rax = rd_acq_usize(rdi + 0x20);
    }

    wr::<u64>(rsi, 0x69fa_fafa_faff);
    atomic_xchg_u64((rdi + 0x28) as u64, r8 as u64);
    exe::call_iat!(0x1_4264_5598, (rdi + 0x18));

    wr::<u32>(rsi + 0x08, 0x20b);
    wr::<u64>(rsi + 0x0c, rd::<u64>(param_1));
    let g2 = rd::<usize>(g_ptr);
    let hdr_dw = (rd::<u32>(rsi + 0x08) & 0x1ff) | 0x200;
    wr::<u32>(rsi + 0x08, hdr_dw);
    let rdx_ring = rd::<usize>(g2 + 0x168);
    wr_rel_u64(rsi, 0x0c);
    atomic_add_u32((rdx_ring + 0x30) as u64, 1);
    if rd::<u8>(g2 + 0x198) != 0 {
        exe::call_iat!(0x1_4264_5580, (rd::<u64>(g2 + 0x1a0), 1u32, 0usize));
    }
}

/// `unknown_141170720`
/// Original: 0x141170720 (356 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141170720(
    param_1: usize,
    param_2: usize,
    param_3: u32,
) -> usize {
    let rbp = rd::<usize>(param_1);
    let tbl_30 = rd::<usize>(rbp + 0x30);
    let r14 = rd::<u64>(tbl_30.wrapping_add((param_3 as usize) * 8));
    if (r14 as u16) == 0xffff {
        exe::call_iat!(0x1_4264_67e8, ());
        return param_2;
    }

    let ebx = (r14 as u16) as usize;
    let base_val = rd::<u64>(rbp + ebx * 8 + 0x88);
    let slot_off = ebx << 5;
    let r12d = rd::<u32>(rbp + slot_off + 0x180ac);
    let r13 = rd::<u64>(rbp + slot_off + 0x18098);
    let off_c03 = (ebx + 0xc03) << 5;

    let k1 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_14bc) as usize) as u32;
    let idx1 = exe::call_va!(0x1_4248_9ee0, (r13, r12d, k1) -> i32) as i64;
    let mut v1 = if idx1 != -1 {
        let arr = rd::<usize>(rbp + off_c03 + 0x40);
        rd::<u32>(arr.wrapping_add((idx1 as usize) * 4)) as u64
    } else {
        0
    };

    let k2 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d120) as usize) as u32;
    let idx2 = exe::call_va!(0x1_4248_9ee0, (r13, r12d, k2) -> i32) as i64;
    let mut v2 = if idx2 != -1 {
        let arr = rd::<usize>(rbp + off_c03 + 0x40);
        rd::<u32>(arr.wrapping_add((idx2 as usize) * 4)) as u64
    } else {
        0
    };

    let k3 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f6d8) as usize) as u32;
    let idx3 = exe::call_va!(0x1_4248_9ee0, (r13, r12d, k3) -> i32) as i64;
    let mut v3 = if idx3 != -1 {
        let arr = rd::<usize>(rbp + off_c03 + 0x40);
        rd::<u32>(arr.wrapping_add((idx3 as usize) * 4)) as u64
    } else {
        0
    };

    if v1 != 0 {
        v1 = v1.wrapping_add(base_val);
    }
    wr::<u64>(param_2, v1);
    if v2 != 0 {
        v2 = v2.wrapping_add(base_val);
    }
    wr::<u64>(param_2 + 0x08, v2);
    if v3 != 0 {
        v3 = v3.wrapping_add(base_val);
    }
    wr::<u64>(param_2 + 0x10, v3);
    wr::<u64>(param_2 + 0x18, base_val);
    wr::<u64>(param_2 + 0x20, r14 >> 32);
    param_2
}

/// `unknown_1411b90b0`
/// Original: 0x1411b90b0 (215 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1411b90b0(param_1: usize, param_2: usize) {
    let elem = rd::<usize>(param_2 + 0x08);
    exe::call_va!(0x1_4042_7310, (elem + 0x60));
    exe::call_va!(0x1_4042_7310, (elem + 0x10));

    let ctrl_ptr = rd::<usize>(param_2);
    wr::<u64>(param_1 + 0x10, rd::<u64>(param_1 + 0x10).wrapping_sub(1));
    let ctrl_base = rd::<usize>(param_1);
    let rel_idx = ctrl_ptr.wrapping_sub(ctrl_base);

    let xmm_cur = rd::<u128>(ctrl_ptr);
    let mask_a = vpmovmskb(vpsignb(xmm_cur, xmm_cur)) as u32;

    let prev_rel = rel_idx.wrapping_sub(0x10);
    let mask = rd::<u64>(param_1 + 0x18) as usize;
    let xmm_prev = rd::<u128>(ctrl_base.wrapping_add(prev_rel & mask));
    let mask_c = vpmovmskb(vpsignb(xmm_prev, xmm_prev)) as u32;

    let use_empty: u8 = if mask_c != 0 && mask_a != 0 {
        let tz = mask_a.trailing_zeros();
        let sh = mask_c << 16;
        let lz = if sh == 0 { 32 } else { sh.leading_zeros() };
        if lz.wrapping_add(tz) < 16 {
            1
        } else {
            0
        }
    } else {
        0
    };

    let ctrl_byte: u8 = if use_empty != 0 { 0x80 } else { 0xfe };
    wr::<u8>(ctrl_base.wrapping_add(rel_idx), ctrl_byte);

    let mask2 = rd::<u64>(param_1 + 0x18) as usize;
    let clone_off = (prev_rel & mask2).wrapping_add(mask2 & 0x0f).wrapping_add(1);
    let ctrl_base2 = rd::<usize>(param_1);
    wr::<u8>(ctrl_base2.wrapping_add(clone_off), ctrl_byte);
    wr::<u64>(
        param_1 + 0x28,
        rd::<u64>(param_1 + 0x28).wrapping_add(use_empty as u64),
    );
}

/// `unknown_1411b9b80`
/// Original: 0x1411b9b80 (403 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1411b9b80(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let fnv = fnv1a_u64_8(param_3);
    let prod = (fnv as u128) * (0xde5f_b9d2_6304_58e9u64 as u128);
    let hash = (prod as u64).wrapping_add((prod >> 64) as u64);

    let h2 = (hash as u8) & 0x7f;
    let h2_vec = vpbroadcastb(h2);
    let mask = rd::<u64>(param_1 + 0x18);
    let ctrl_base = rd::<usize>(param_1);
    let mut offset = (hash >> 7) & mask;
    let mut stride: u64 = 0;

    let (slot_idx, inserted): (u64, u8) = 'probe: loop {
        let ctrl_vec = rd::<u128>(ctrl_base.wrapping_add(offset as usize));
        let mut match_mask = vpmovmskb(vpcmpeqb(h2_vec, ctrl_vec)) as u32;
        if match_mask != 0 {
            let slots_base = rd::<usize>(param_1 + 0x08);
            let target_key = rd::<u64>(param_3);
            while match_mask != 0 {
                let bit = match_mask.trailing_zeros() as u64;
                let idx = offset.wrapping_add(bit) & mask;
                if rd::<u64>(slots_base.wrapping_add((idx as usize) * 0x70)) == target_key {
                    break 'probe (idx, 0u8);
                }
                match_mask &= match_mask - 1;
            }
        }

        let empty_mask = vpmovmskb(vpsignb(ctrl_vec, ctrl_vec)) as u32;
        if empty_mask != 0 {
            let idx = exe::call_va!(0x1_411b_a030, (param_1, hash) -> u64);
            let slot_ptr = rd::<usize>(param_1 + 0x08).wrapping_add((idx as usize) * 0x70);
            wr::<u64>(slot_ptr, rd::<u64>(param_3));
            wr::<[u64; 12]>(slot_ptr + 0x10, [0u64; 12]);
            break (idx, 1u8);
        }

        stride = stride.wrapping_add(0x10);
        offset = offset.wrapping_add(stride) & mask;
    };

    let out_ctrl = rd::<usize>(param_1).wrapping_add(slot_idx as usize);
    let out_slot = rd::<usize>(param_1 + 0x08).wrapping_add((slot_idx as usize) * 0x70);
    wr::<usize>(param_2, out_ctrl);
    wr::<usize>(param_2 + 0x08, out_slot);
    wr::<u8>(param_2 + 0x10, inserted);
    param_2
}

/// `d3d_1411d3b20`
/// Original: 0x1411d3b20 (248 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_1411d3b20(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let word_count = rd_acq_u64(param_1);
    let mut word_idx = param_3 >> 6;
    let first_mask = u64::MAX << ((param_3 as u32) & 0x3f);

    let word0 = if word_idx < word_count {
        let tail_mask = if word_idx == word_count - 1 {
            let total_bits = rd_acq_u64(param_1 + 0x10);
            let shift = (!(total_bits.wrapping_sub(1)) as u32) & 0x3f;
            u64::MAX >> shift
        } else {
            u64::MAX
        };
        let arr = rd_acq_usize(param_1 + 0x08);
        rd_acq_u64(arr.wrapping_add((word_idx as usize) * 8)) & tail_mask
    } else {
        0
    };

    let mut tz = (word0 & first_mask).trailing_zeros() as u64;
    loop {
        if tz < 64 {
            let bit_index = (word_idx << 6).wrapping_add(tz);
            wr::<u64>(param_2, bit_index);
            wr::<u64>(param_2 + 0x08, 1);
            return param_2;
        }
        word_idx = word_idx.wrapping_add(1);
        if word_idx >= word_count {
            break;
        }
        let tail_mask = if word_idx == word_count - 1 {
            let total_bits = rd_acq_u64(param_1 + 0x10);
            let shift = (!(total_bits.wrapping_sub(1)) as u32) & 0x3f;
            u64::MAX >> shift
        } else {
            u64::MAX
        };
        let arr = rd_acq_usize(param_1 + 0x08);
        let w = rd_acq_u64(arr.wrapping_add((word_idx as usize) * 8)) & tail_mask;
        tz = w.trailing_zeros() as u64;
    }

    wr::<u8>(param_2 + 0x08, 0);
    param_2
}

/// `d3d_1411dcdf0`
/// Original: 0x1411dcdf0 (109 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_1411dcdf0() -> u64 {
    let tls0 = rd_acq_usize(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, ());
    }
    if rd_acq_u64(tls0 + 0x6340) != 0 {
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, ());
        }
        return rd_acq_u64(tls0 + 0x6340);
    }
    rd_acq_u64(exe::ptr::<u64>(0x1_4397_fc88) as usize)
}

/// `unknown_1411e4980`
/// Original: 0x1411e4980 (1476 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1411e4980(param_1: usize) {
    exe::call_va!(0x1_4209_b0c0, (param_1));
    let ctx0 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    let stride_cb0_0 = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let base_57 = rd::<u64>(ctx0 + 0x10).wrapping_add(stride_cb0_0.wrapping_mul(0x57));

    let ctx1 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    let stride_b7c_0 = rd::<u32>(exe::ptr::<u32>(0x1_4397_db7c) as usize) as u64;
    let count_90 = rd::<u32>(param_1 + 0x90);
    let base_sampler = rd::<u64>(ctx1 + 0x18).wrapping_add(stride_b7c_0.wrapping_mul(4));

    let tbl_37b0 = exe::ptr::<i32>(0x1_43a6_37b0) as usize;
    let dev_global = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let def_global = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let cb0_global = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let b7c_global = exe::ptr::<u32>(0x1_4397_db7c) as usize;

    if count_90 != 0 {
        let tls0 = rd_acq_usize(exe::tls_pointer());
        let mut off = 0usize;
        for _ in 0..count_90 {
            let arr_88 = rd::<usize>(param_1 + 0x88);
            let rdi = exe::call_va!(0x1_411d_cdf0, () -> usize);
            if rd::<u8>(tls0 + 0x70) == 0 {
                exe::call_va!(0x1_4183_49c0, ());
            }
            let idx_10 = rd::<i32>(arr_88 + off + 0x10) as isize;
            let tls_entry = tls0
                .wrapping_add(0x5b460)
                .wrapping_add((idx_10 * 0x28) as usize);
            let slot = rd::<i32>(arr_88 + off);
            let addr_val = rd::<u64>(tls_entry + 0x08)
                .wrapping_add((rd::<u32>(tls_entry + 0x20) as u64) << 5);
            let size_val = rd::<u32>(tls_entry + 0x24) << 4;
            let mut cbv_desc: [u64; 2] = [addr_val, size_val as u64];
            if slot < 4 {
                wr::<u64>((rdi as isize + (slot as isize) * 8 + 0x80) as usize, addr_val);
            } else {
                let dev = rd::<usize>(dev_global);
                let fn_88 = rd::<usize>(rd::<usize>(dev) + 0x88);
                let step = rd::<u32>(cb0_global) as u64;
                let idx_r8 = ((slot.wrapping_sub(4) as i64) + 0x4d) as u64;
                let dest_desc = rd::<u64>(rdi + 0x10).wrapping_add(idx_r8.wrapping_mul(step));
                exe::call_va!(fn_88, (dev, cbv_desc.as_mut_ptr() as usize, dest_desc));
            }
            off = off.wrapping_add(0x28);
        }
    }

    let mut rbx = rd::<usize>(param_1 + 0x98);
    let end_98 = rbx.wrapping_add((rd::<u32>(param_1 + 0xa0) as usize) * 0x28);
    if rbx != end_98 {
        let tls0 = rd_acq_usize(exe::tls_pointer());
        while rbx != end_98 {
            let idx5 = rd::<i32>(rbx + 0x14) as isize;
            let slot = rd::<i32>(rbx);
            let tbl_off = rd::<i32>((tbl_37b0 as isize + idx5 * 16) as usize) as isize;
            let r14 = (tls0 as isize + 0x63d0 + tbl_off) as usize;
            let r10 = exe::call_va!(0x1_411d_cdf0, () -> usize);
            if slot < 4 {
                wr::<u64>(
                    (r10 as isize + (slot as isize) * 8 + 0x80) as usize,
                    rd::<u64>(r14),
                );
            } else {
                let dev = rd::<usize>(dev_global);
                let fn_88 = rd::<usize>(rd::<usize>(dev) + 0x88);
                let step = rd::<u32>(cb0_global) as u64;
                let idx_r8 = ((slot.wrapping_sub(4) as i64) + 0x4d) as u64;
                let dest_desc = rd::<u64>(r10 + 0x10).wrapping_add(idx_r8.wrapping_mul(step));
                exe::call_va!(fn_88, (dev, r14, dest_desc));
            }
            rbx = rbx.wrapping_add(0x28);
        }
    }

    let mut rbx = rd::<usize>(param_1 + 0x08);
    let end_08 = rbx.wrapping_add((rd::<u32>(param_1 + 0x10) as usize) * 0x1c);
    if rbx != end_08 {
        let tls0 = rd_acq_usize(exe::tls_pointer());
        while rbx != end_08 {
            let idx0 = rd::<i32>(rbx) as isize;
            let idx3 = rd::<i32>(rbx + 0x0c) as isize;
            let mut ebp = rd::<i32>(rbx + 0x04);
            let count = rd::<i32>(rbx + 0x08);
            let tbl_off = rd::<i32>((tbl_37b0 as isize + idx0 * 16) as usize) as isize;
            let mut r15_ptr = (tls0 as isize + 0x63d0 + tbl_off + (idx3 << 4)) as usize;
            if count > 0 {
                for _ in 0..count {
                    let r14 = rd::<u64>(r15_ptr + 0x08);
                    if rd::<u8>(tls0 + 0x70) == 0 {
                        exe::call_va!(0x1_4183_49c0, ());
                    }
                    if rd::<u64>(tls0 + 0x6300) == 0 && r14 != 0 {
                        exe::call_va!(0x1_4206_94a0, (r14));
                    }
                    let dev = rd::<usize>(dev_global);
                    let fn_c0 = rd::<usize>(rd::<usize>(dev) + 0xc0);
                    let mut r9_desc = rd::<u64>(r15_ptr);
                    if r9_desc == 0 {
                        r9_desc = rd::<u64>(rd::<usize>(def_global) + 0x88);
                    }
                    let step = rd::<u32>(cb0_global) as u64;
                    let r8_dest = base_57.wrapping_add(((ebp as i64) as u64).wrapping_mul(step));
                    exe::call_va!(fn_c0, (dev, 1u32, r8_dest, r9_desc, 0u32));
                    ebp = ebp.wrapping_add(1);
                    r15_ptr = r15_ptr.wrapping_add(0x10);
                }
            }
            rbx = rbx.wrapping_add(0x1c);
        }
    }

    for kind in [3isize, 4isize] {
        let elem_stride: isize = match kind {
            0 | 2 | 5 => 0x10,
            1 | 3 | 4 | 6 | 7 => 8,
            _ => 0,
        };
        let mut rbx = rd::<usize>((param_1 as isize + kind * 0x10 + 0x08) as usize);
        let cnt = rd::<u32>((param_1 as isize + kind * 0x10 + 0x10) as usize) as usize;
        let end_k = rbx.wrapping_add(cnt * 0x1c);
        if rbx != end_k {
            let tls0 = rd_acq_usize(exe::tls_pointer());
            while rbx != end_k {
                let idx3 = rd::<i32>(rbx + 0x0c) as isize;
                let idx0 = rd::<i32>(rbx) as isize;
                let mut r15d = rd::<i32>(rbx + 0x04);
                let count = rd::<i32>(rbx + 0x08);
                let tbl_off = rd::<i32>((tbl_37b0 as isize + idx0 * 16) as usize) as isize;
                let mut r14_ptr =
                    (tls0 as isize + 0x63d0 + tbl_off + idx3 * elem_stride) as usize;
                if count > 0 {
                    for _ in 0..count {
                        let dev = rd::<usize>(dev_global);
                        let fn_c0 = rd::<usize>(rd::<usize>(dev) + 0xc0);
                        let mut r9_desc = if r14_ptr != 0 { rd::<u64>(r14_ptr) } else { 0 };
                        if r9_desc == 0 {
                            r9_desc = rd::<u64>(rd::<usize>(def_global) + 0x98);
                        }
                        let step = rd::<u32>(cb0_global) as u64;
                        let r8_dest =
                            base_57.wrapping_add(((r15d as i64) as u64).wrapping_mul(step));
                        exe::call_va!(fn_c0, (dev, 1u32, r8_dest, r9_desc, 0u32));
                        r15d = r15d.wrapping_add(1);
                        r14_ptr = (r14_ptr as isize + elem_stride) as usize;
                    }
                }
                rbx = rbx.wrapping_add(0x1c);
            }
        }
    }

    let mut rbx = rd::<usize>(param_1 + 0x18);
    let end_18 = rbx.wrapping_add((rd::<u32>(param_1 + 0x20) as usize) * 0x1c);
    if rbx != end_18 {
        let tls0 = rd_acq_usize(exe::tls_pointer());
        while rbx != end_18 {
            let idx0 = rd::<i32>(rbx) as isize;
            let idx3 = rd::<i32>(rbx + 0x0c) as isize;
            let mut r14d = rd::<i32>(rbx + 0x04);
            let count = rd::<i32>(rbx + 0x08);
            let tbl_off = rd::<i32>((tbl_37b0 as isize + idx0 * 16) as usize) as isize;
            let mut rsi_ptr = (tls0 as isize + 0x63d0 + tbl_off + idx3 * 8) as usize;
            if count > 0 {
                for _ in 0..count {
                    let dev = rd::<usize>(dev_global);
                    let fn_c0 = rd::<usize>(rd::<usize>(dev) + 0xc0);
                    let p = rd::<usize>(rsi_ptr);
                    let r9_desc = if p != 0 {
                        rd::<u64>(p)
                    } else {
                        rd::<u64>(rd::<usize>(def_global) + 0x80)
                    };
                    let step = rd::<u32>(b7c_global) as u64;
                    let r8_dest =
                        base_sampler.wrapping_add(((r14d as i64) as u64).wrapping_mul(step));
                    exe::call_va!(fn_c0, (dev, 1u32, r8_dest, r9_desc, 1u32));
                    r14d = r14d.wrapping_add(1);
                    rsi_ptr = rsi_ptr.wrapping_add(8);
                }
            }
            rbx = rbx.wrapping_add(0x1c);
        }
    }
}

/// `unknown_1411ed7e0`
/// Original: 0x1411ed7e0 (162 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411ed7e0(param_1: usize) {
    let ptr = rd_acq_usize(param_1);
    if ptr != 0 && ptr != param_1 + 0x10 {
        let chunk_base = ptr.wrapping_sub(1) & !0x01ff_ffffusize;
        let page_idx = (ptr.wrapping_sub(chunk_base)) >> 16;
        let desc_hdr = chunk_base
            .wrapping_add(0x108)
            .wrapping_add(page_idx.wrapping_mul(0x50));
        let desc = desc_hdr.wrapping_sub(rd::<u32>(desc_hdr + 0x04) as usize);
        if rd::<usize>(chunk_base + 0x100) == exe::teb() {
            if rd::<u8>(desc + 0x0e) == 0 {
                wr::<usize>(ptr, rd::<usize>(desc + 0x20));
                let new_cnt = rd::<u32>(desc + 0x18).wrapping_sub(1);
                wr::<u32>(desc + 0x18, new_cnt);
                wr::<usize>(desc + 0x20, ptr);
                if new_cnt == 0 {
                    exe::call_va!(0x1_4253_71a0, (desc));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (chunk_base, desc, 1u8, ptr));
            }
        } else {
            exe::call_va!(0x1_4253_3200, (chunk_base, desc, 0u32, ptr));
        }
    }
    wr::<u32>(param_1 + 0x08, 0);
    wr::<usize>(param_1, param_1 + 0x10);
    wr::<u32>(param_1 + 0x0c, 0x80);
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_140fbf1e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, cap_a: u64, cap_b: u64| -> ([u64; 12], [u64; 12]) {
        let mut a = [0u64; 12];
        let mut b = [0u64; 12];
        for i in 0..12 {
            a[i] = 0x1111_0000_0000_0000u64 + i as u64;
            b[i] = 0x2222_0000_0000_0000u64 + (i as u64) * 10;
        }
        let pa = a.as_mut_ptr() as usize;
        let pb = b.as_mut_ptr() as usize;
        wr::<f32>(pa + 0x10, 0.75);
        wr::<f32>(pa + 0x14, 0.25);
        wr::<u8>(pa + 0x18, 1);
        wr::<u64>(pa + 0x40, cap_a);

        wr::<f32>(pb + 0x10, 0.5);
        wr::<f32>(pb + 0x14, 0.125);
        wr::<u8>(pb + 0x18, 1);
        wr::<u64>(pb + 0x40, cap_b);

        f(pa, pb);
        (a, b)
    };

    for (ca, cb) in [(32u64, 64u64), (128, 256)] {
        let o = run(f_o, ca, cb);
        let n = run(f_n, ca, cb);
        crate::diff_eq!(o, n, "140fbf1e0 ca={} cb={}", ca, cb);
    }
    Ok(2)
}

pub unsafe fn diff_sparsehash_140fbf4b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TARGET_TABLE: [u64; 16] = [0; 16];

    unsafe extern "win64" fn mock_2020(_p1: usize, _diff: u64) -> u64 {
        8
    }
    unsafe extern "win64" fn mock_f650(p1: usize, cap: u64) {
        let tp = core::ptr::addr_of_mut!(TARGET_TABLE) as usize;
        let empty = rd::<[u64; 2]>(p1 + 0x48);
        for i in 0..(cap as usize) {
            wr::<[u64; 2]>(tp + i * 16, empty);
        }
        wr::<usize>(p1 + 0x58, tp);
        wr::<u64>(p1 + 0x40, cap);
        wr::<u64>(p1 + 0x38, 0);
        wr::<u64>(p1 + 0x30, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_2020, mock_2020 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_40fb_f650, mock_f650 as *const ())?;

    let run = |f: F, with_items: bool| -> ([u64; 16], u64, u32) {
        TARGET_TABLE = [0; 16];
        let mut src_table = [0u64; 8];
        let stp = src_table.as_mut_ptr() as usize;
        for i in 0..4 {
            wr::<u64>(stp + i * 16, u64::MAX);
            wr::<u64>(stp + i * 16 + 8, 0);
        }
        if with_items {
            wr::<u64>(stp + 16, 0x1234_5678);
            wr::<u64>(stp + 24, 0xaaaa);
            wr::<u64>(stp + 48, 0x8765_4321);
            wr::<u64>(stp + 56, 0xbbbb);
        }

        let mut p1 = [0u64; 12];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u32>(p1p + 0x1c, 5);
        wr::<u64>(p1p + 0x48, u64::MAX);
        wr::<u64>(p1p + 0x50, 0);

        let mut p2 = [0u64; 12];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p + 0x28, u64::MAX - 1);
        wr::<u64>(p2p + 0x30, 0);
        wr::<u64>(p2p + 0x38, if with_items { 2 } else { 0 });
        wr::<u64>(p2p + 0x40, 4);
        wr::<u64>(p2p + 0x48, u64::MAX);
        wr::<usize>(p2p + 0x58, stp);

        f(p1p, p2p);
        (TARGET_TABLE, rd::<u64>(p1p + 0x38), rd::<u32>(p1p + 0x1c))
    };

    for items in [false, true] {
        let o = run(f_o, items);
        let n = run(f_n, items);
        crate::diff_eq!(o, n, "140fbf4b0 items={}", items);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140fbf650(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_TABLE: [u64; 16] = [0; 16];
    static mut ALLOC_CALLED: u64 = 0;

    unsafe extern "win64" fn mock_5e80(_p1: usize, cap: u64) -> usize {
        ALLOC_CALLED = cap;
        core::ptr::addr_of_mut!(ALLOC_TABLE) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4021_5e80, mock_5e80 as *const ())?;

    let run = |f: F, pre_table: bool| -> ([u64; 16], [u64; 12], u64) {
        ALLOC_TABLE = [0x99; 16];
        ALLOC_CALLED = 0;
        let tbl_ptr = core::ptr::addr_of_mut!(ALLOC_TABLE) as usize;

        let mut p1 = [0u64; 12];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<f32>(p1p + 0x10, 0.75);
        wr::<f32>(p1p + 0x14, 0.25);
        wr::<u8>(p1p + 0x18, 1);
        wr::<u64>(p1p + 0x30, 3);
        wr::<u64>(p1p + 0x38, 5);
        wr::<u64>(p1p + 0x40, 4);
        wr::<u64>(p1p + 0x48, u64::MAX);
        wr::<u64>(p1p + 0x50, 0x1122_3344);
        wr::<usize>(p1p + 0x58, if pre_table { tbl_ptr } else { 0 });

        f(p1p, 4);
        p1[11] = p1[11].wrapping_sub(tbl_ptr as u64);
        (ALLOC_TABLE, p1, ALLOC_CALLED)
    };

    for pre in [false, true] {
        let o = run(f_o, pre);
        let n = run(f_n, pre);
        crate::diff_eq!(o, n, "140fbf650 pre={}", pre);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14104e610(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOT1_VAL: u32 = 0;
    static mut SLOT2_VAL: u32 = 0;
    static mut NODE_A: [u32; 4] = [0, 0, 0x77, 0];
    static mut NODE_B: [u32; 4] = [0, 0, 0x88, 0];

    unsafe extern "win64" fn mock_ebe0(map: usize, out_iter: usize, _key: usize) {
        wr::<u64>(out_iter + 0x08, map as u64);
        wr::<u64>(out_iter + 0x10, 1);
        wr::<u64>(out_iter + 0x18, 2);
        wr::<usize>(out_iter + 0x20, core::ptr::addr_of_mut!(NODE_A) as usize);
    }
    unsafe extern "win64" fn mock_eeb0(map: usize, out_iter: usize) {
        wr::<u64>(out_iter + 0x08, map as u64);
        wr::<u64>(out_iter + 0x10, 1);
        wr::<u64>(out_iter + 0x18, 2);
        wr::<usize>(out_iter + 0x20, core::ptr::addr_of_mut!(NODE_B) as usize);
    }
    unsafe extern "win64" fn mock_ee00(_map: usize, _key: usize) -> usize {
        core::ptr::addr_of_mut!(SLOT1_VAL) as usize
    }
    unsafe extern "win64" fn mock_eb30(_map: usize, _key: usize) -> usize {
        core::ptr::addr_of_mut!(SLOT2_VAL) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_4104_ebe0, mock_ebe0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4104_eeb0, mock_eeb0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4104_ee00, mock_ee00 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4104_eb30, mock_eb30 as *const ())?;

    let run = |f: F, use_freelist: bool| -> (u32, [u64; 16], u32, u32, u32, u32) {
        SLOT1_VAL = 0;
        SLOT2_VAL = 0;
        let mut vec_buf = [0u64; 16];
        let mut free_arr: [u32; 4] = [2, 1, 0, 0];
        let mut p1 = [0u8; 0xf0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0xc0, vec_buf.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0xc8, 1);
        wr::<u32>(p1p + 0xcc, 4);
        wr::<usize>(p1p + 0xd0, free_arr.as_mut_ptr() as usize);
        wr::<u32>(p1p + 0xd8, if use_freelist { 1 } else { 0 });

        let mut p2: [u64; 4] = [0x1111_2222_3333_4444, 0, 0xaaaa_bbbb, 0xcccc_dddd];
        let ret = f(p1p, p2.as_mut_ptr() as usize);
        (
            ret,
            vec_buf,
            rd::<u32>(p1p + 0xc8),
            rd::<u32>(p1p + 0xd8),
            SLOT1_VAL,
            SLOT2_VAL,
        )
    };

    for fl in [false, true] {
        let o = run(f_o, fl);
        let n = run(f_n, fl);
        crate::diff_eq!(o, n, "14104e610 fl={}", fl);
    }
    Ok(2)
}

pub unsafe fn diff_sparsehash_14105a770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FOUND_IDX: i64 = -1;
    static mut INS_IDX: i64 = 2;

    unsafe extern "win64" fn mock_a730(_p1: usize, out_pos: usize, _key: usize) {
        wr::<i64>(out_pos, FOUND_IDX);
        wr::<i64>(out_pos + 8, INS_IDX);
    }
    unsafe extern "win64" fn mock_a960(_p1: usize) -> u8 {
        0
    }

    let _m1 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4105_a960, mock_a960 as *const ())?;

    let run = |f: F, found: i64| -> (usize, u64, u32, u64, u64) {
        FOUND_IDX = found;
        INS_IDX = 2;
        let mut table = [0u64; 8];
        let tp = table.as_mut_ptr() as usize;
        let mut p1 = [0u64; 12];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<u64>(p1p + 0x28, u64::MAX - 1);
        wr::<u64>(p1p + 0x30, 0);
        wr::<u64>(p1p + 0x38, 1);
        wr::<usize>(p1p + 0x58, tp);

        let mut key: u64 = 0xdead_beef_cafe_1234;
        let ret = f(p1p, (&mut key as *mut u64) as usize);
        (
            ret.wrapping_sub(tp),
            rd::<u64>(tp + 2 * 16),
            rd::<u32>(tp + 2 * 16 + 8),
            rd::<u64>(p1p + 0x30),
            rd::<u64>(p1p + 0x38),
        )
    };

    for found in [1i64, -1] {
        let o = run(f_o, found);
        let n = run(f_n, found);
        crate::diff_eq!(o, n, "14105a770 found={}", found);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141068680(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let cases: [[f32; 16]; 2] = [
        [
            2.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 1.0, 2.0, 3.0, 1.0,
        ],
        [
            1.0, 0.5, 0.0, 0.0, 0.0, 2.0, 0.5, 0.0, 0.25, 0.0, 1.0, 0.0, 1.0, -1.0, 2.0, 1.0,
        ],
    ];

    for (idx, mat) in cases.iter().enumerate() {
        let mut mo = *mat;
        let mut mn = *mat;
        let ro = f_o(mo.as_mut_ptr() as usize).wrapping_sub(mo.as_ptr() as usize);
        let rn = f_n(mn.as_mut_ptr() as usize).wrapping_sub(mn.as_ptr() as usize);
        let bo: [u32; 16] = core::mem::transmute(mo);
        let bn: [u32; 16] = core::mem::transmute(mn);
        crate::diff_eq!((ro, bo), (rn, bn), "141068680 case={}", idx);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141092770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_LOG: Vec<(u64, u64, u64, u64, u64, u64, u64)> = Vec::new();

    unsafe extern "win64" fn mock_35a0(out: usize, p2: u64, r10: u64, r9d: u32, _r8: usize) {
        wr::<u64>(out + 0x08, p2 + 10);
        wr::<u64>(out + 0x18, 2);
        wr::<u64>(out + 0x20, r10);
        wr::<u64>(out + 0x28, r9d as u64);
        wr::<u64>(out + 0x30, 0x9999);
    }
    unsafe extern "win64" fn mock_88b0(arg: usize, rdi: u64, r13: u64) {
        CALL_LOG.push((
            rd::<u64>(arg),
            rd::<u64>(arg + 0x08),
            rd::<u64>(arg + 0x10),
            rd::<u64>(arg + 0x18),
            rd::<u64>(arg + 0x20),
            rdi,
            r13,
        ));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4109_35a0, mock_35a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f2_88b0, mock_88b0 as *const ())?;

    let i_a = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d904) as usize) as usize;
    let i_b = rd::<u16>(exe::ptr::<u16>(0x1_43c9_04a0) as usize) as usize;
    let max_idx = i_a.max(i_b) + 2;

    let run = |f: F, has_ptr: bool| -> Vec<(u64, u64, u64, u64, u64, u64, u64)> {
        CALL_LOG.clear();
        let mut tbl = vec![0u64; max_idx];
        tbl[i_a] = 0xaaaa_1111;
        tbl[i_b] = 0xbbbb_2222;
        let mut ctx = vec![0u8; 0x58100];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<u64>(cp + 0x88 + 3 * 8, if has_ptr { 0x1234_0000 } else { 0 });
        wr::<u32>(cp + 0x10088 + 3 * 4, 7);
        wr::<usize>(cp + 0x580e0, tbl.as_mut_ptr() as usize);
        let mut p1: usize = cp;
        f((&mut p1 as *mut usize) as usize, 3);
        CALL_LOG.clone()
    };

    for has_ptr in [false, true] {
        let o = run(f_o, has_ptr);
        let n = run(f_n, has_ptr);
        crate::diff_eq!(o, n, "141092770 has_ptr={}", has_ptr);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1410d6540(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF: [u8; 0x28 * 4] = [0; 0x28 * 4];
    static mut FREED_PTR: usize = 0;

    unsafe extern "win64" fn mock_c0a0(_sz: u64, _align: u32) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF) as usize
    }
    unsafe extern "win64" fn mock_84a0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u64>(dst + 8, rd::<u64>(src + 8));
    }
    unsafe extern "win64" fn mock_c190(p: usize) {
        FREED_PTR = p;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F, len: u32| -> ([u8; 0x28 * 4], [u8; 0x28 * 2], u32, u32, bool) {
        NEW_BUF = [0xee; 0x28 * 4];
        FREED_PTR = 0;
        let mut old_buf = [0u8; 0x28 * 2];
        for (i, b) in old_buf.iter_mut().enumerate() {
            *b = (i as u8).wrapping_add(1);
        }
        let obp = old_buf.as_mut_ptr() as usize;
        let mut vec_hdr: [u64; 2] = [obp as u64, (len as u64) | (2u64 << 32)];
        f(vec_hdr.as_mut_ptr() as usize, 4);
        let hp = vec_hdr.as_ptr() as usize;
        (
            NEW_BUF,
            old_buf,
            rd::<u32>(hp + 0x08),
            rd::<u32>(hp + 0x0c),
            FREED_PTR == obp,
        )
    };

    for len in [0u32, 2] {
        let o = run(f_o, len);
        let n = run(f_n, len);
        crate::diff_eq!(o, n, "1410d6540 len={}", len);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1410d6c60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_ARGS: (u64, u64, u64) = (0, 0, 0);

    unsafe extern "win64" fn mock_a270(rcx: u64, _rdx: usize, r8: u64, r9: u64) -> u64 {
        CALL_ARGS = (rcx, r8, r9);
        rcx ^ r8 ^ r9
    }

    let _m1 = crate::difftest::Mock::new(0x1_41f5_a270, mock_a270 as *const ())?;

    let i1 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d028) as usize) as usize;
    let i2 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d080) as usize) as usize;
    let i3 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d170) as usize) as usize;
    let max_i = i1.max(i2).max(i3) + 2;

    let run = |f: F, seed: u64| -> (u64, (u64, u64, u64)) {
        CALL_ARGS = (0, 0, 0);
        let mut inner_val: u64 = 0x1111_0000 + seed;
        let mut tbl = vec![0u64; max_i];
        tbl[i1] = 0x2222_0000 + seed;
        tbl[i2] = 0x3333_0000 + seed;
        tbl[i3] = (&mut inner_val as *mut u64) as usize as u64;

        let mut ctx = vec![0u8; 0x58100];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp + 0x580e0, tbl.as_mut_ptr() as usize);
        let mut p1: usize = cp;
        let ret = f((&mut p1 as *mut usize) as usize);
        (ret, CALL_ARGS)
    };

    for seed in [1u64, 2] {
        let o = run(f_o, seed);
        let n = run(f_n, seed);
        crate::diff_eq!(o, n, "1410d6c60 seed={}", seed);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14110b9a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SEM_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_acq(_lock: usize) {}
    unsafe extern "win64" fn mock_rel(_lock: usize) {}
    unsafe extern "win64" fn mock_sem(_h: u64, cnt: u32, _prev: usize) -> i32 {
        SEM_COUNT += cnt;
        1
    }

    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_acq as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, mock_rel as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5580, mock_sem as *const ())?;

    let g_slot = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_g = rd::<usize>(g_slot);

    let run = |f: F, wrap: bool| -> ([u64; 8], usize, u32, u32) {
        SEM_COUNT = 0;
        let mut ring_mem = [0u64; 8];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let mut ring_hdr = [0u8; 0x40];
        let rhp = ring_hdr.as_mut_ptr() as usize;
        wr::<usize>(rhp + 0x08, rmp);
        wr::<usize>(rhp + 0x10, rmp + 0x30);
        wr::<usize>(rhp + 0x20, rmp + 0x30);
        wr::<usize>(rhp + 0x28, if wrap { rmp + 0x20 } else { rmp });
        wr::<u32>(rhp + 0x30, 5);
        wr::<u32>(rhp + 0x34, 5);

        let mut g_obj = [0u8; 0x1b0];
        let gp = g_obj.as_mut_ptr() as usize;
        wr::<usize>(gp + 0x168, rhp);
        wr::<u8>(gp + 0x198, 1);
        wr::<u64>(gp + 0x1a0, 0x777);
        wr::<usize>(g_slot, gp);

        let val: u64 = 0x1234_5678_9abc_def0;
        f((&val as *const u64) as usize);
        wr::<usize>(g_slot, saved_g);

        (
            ring_mem,
            rd::<usize>(rhp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rhp + 0x30),
            SEM_COUNT,
        )
    };

    for wrap in [false, true] {
        let o = run(f_o, wrap);
        let n = run(f_n, wrap);
        crate::diff_eq!(o, n, "14110b9a0 wrap={}", wrap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_141170720(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u32) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_9ee0(_r13: u64, r12d: u32, key: u32) -> i32 {
        if r12d == 0 {
            -1
        } else {
            (key & 1) as i32
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;

    let run = |f: F, r12d: u32| -> [u64; 5] {
        let mut entries: [u64; 2] = [0, (7u64 << 32) | 1u64];
        let mut offsets: [u32; 4] = [0x10, 0x20, 0x30, 0x40];
        let mut ctx = vec![0u8; 0x18150];
        let cp = ctx.as_mut_ptr() as usize;
        wr::<usize>(cp + 0x30, entries.as_mut_ptr() as usize);
        wr::<u64>(cp + 0x88 + 8, 0x1000);
        wr::<u64>(cp + 0x20 + 0x18098, 0x5555);
        wr::<u32>(cp + 0x20 + 0x180ac, r12d);
        wr::<usize>(
            cp + ((1 + 0xc03) << 5) + 0x40,
            offsets.as_mut_ptr() as usize,
        );

        let mut p1: usize = cp;
        let mut out = [0u64; 5];
        f(
            (&mut p1 as *mut usize) as usize,
            out.as_mut_ptr() as usize,
            1,
        );
        out
    };

    for r12d in [0u32, 3] {
        let o = run(f_o, r12d);
        let n = run(f_n, r12d);
        crate::diff_eq!(o, n, "141170720 r12d={}", r12d);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411b90b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DTOR_OFFS: [usize; 2] = [0; 2];
    static mut DTOR_CNT: usize = 0;

    unsafe extern "win64" fn mock_7310(p: usize) {
        if DTOR_CNT < 2 {
            DTOR_OFFS[DTOR_CNT] = p;
            DTOR_CNT += 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4042_7310, mock_7310 as *const ())?;

    let run = |f: F, empty_byte: u8| -> ([u8; 48], u64, u64, [usize; 2]) {
        DTOR_OFFS = [0; 2];
        DTOR_CNT = 0;
        let mut ctrl = [empty_byte; 48];
        ctrl[16] = 0x12;
        let cp = ctrl.as_mut_ptr() as usize;
        let mut dummy_elem = [0u8; 0x80];
        let ep = dummy_elem.as_mut_ptr() as usize;

        let mut p1: [u64; 6] = [cp as u64, ep as u64, 5, 0x1f, 0, 10];
        let mut p2: [usize; 2] = [cp + 16, ep];
        f(p1.as_mut_ptr() as usize, p2.as_mut_ptr() as usize);
        (
            ctrl,
            p1[2],
            p1[5],
            [DTOR_OFFS[0].wrapping_sub(ep), DTOR_OFFS[1].wrapping_sub(ep)],
        )
    };

    for b in [0x80u8, 0x01] {
        let o = run(f_o, b);
        let n = run(f_n, b);
        crate::diff_eq!(o, n, "1411b90b0 byte={:#x}", b);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411b9b80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_a030(_p1: usize, _hash: u64) -> u64 {
        1
    }

    let _m1 = crate::difftest::Mock::new(0x1_411b_a030, mock_a030 as *const ())?;

    let run = |f: F, pre_populate: bool| -> (usize, usize, u8, [u64; 28]) {
        let key: u64 = 0x1122_3344_5566_7788;
        let fnv = fnv1a_u64_8((&key as *const u64) as usize);
        let prod = (fnv as u128) * (0xde5f_b9d2_6304_58e9u64 as u128);
        let h2 = ((prod as u64).wrapping_add((prod >> 64) as u64) as u8) & 0x7f;

        let mut ctrl = [0x80u8; 32];
        let mut slots = [0xaaaa_bbbb_cccc_ddddu64; 28];
        let cp = ctrl.as_mut_ptr() as usize;
        let sp = slots.as_mut_ptr() as usize;
        if pre_populate {
            wr::<u8>(cp, h2);
            wr::<u64>(sp, key);
        }

        let mut p1: [u64; 6] = [cp as u64, sp as u64, 0, 0x0f, 0, 0];
        let mut out = [0u64; 3];
        let out_p = out.as_mut_ptr() as usize;
        f(
            p1.as_mut_ptr() as usize,
            out_p,
            (&key as *const u64) as usize,
        );
        (
            rd::<usize>(out_p).wrapping_sub(cp),
            rd::<usize>(out_p + 0x08).wrapping_sub(sp),
            rd::<u8>(out_p + 0x10),
            slots,
        )
    };

    for pre in [false, true] {
        let o = run(f_o, pre);
        let n = run(f_n, pre);
        crate::diff_eq!(o, n, "1411b9b80 pre={}", pre);
    }
    Ok(2)
}

pub unsafe fn diff_d3d_1411d3b20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, start_bit: u64| -> (u8, u64) {
        let words: [u64; 2] = [1u64 << 5, 1u64 << 3];
        let hdr: [u64; 3] = [2, words.as_ptr() as usize as u64, 70];
        let mut out = [0u64; 2];
        let op = out.as_mut_ptr() as usize;
        f(hdr.as_ptr() as usize, op, start_bit);
        let has = rd::<u8>(op + 0x08);
        let val = if has != 0 { rd::<u64>(op) } else { 0 };
        (has, val)
    };

    for start in [0u64, 6, 68] {
        let o = run(f_o, start);
        let n = run(f_n, start);
        crate::diff_eq!(o, n, "1411d3b20 start={}", start);
    }
    Ok(3)
}

pub unsafe fn diff_d3d_1411dcdf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);
    let g_c88 = exe::ptr::<u64>(0x1_4397_fc88) as usize;
    let saved_c88 = rd::<u64>(g_c88);

    let run = |f: F, tls_val: u64| -> u64 {
        let mut tls_blk = vec![0u8; 0x6360];
        let tp = tls_blk.as_mut_ptr() as usize;
        wr::<u8>(tp + 0x70, 1);
        wr::<u64>(tp + 0x6340, tls_val);
        wr::<u64>(g_c88, 0xfeed_face_cafe_beef);
        wr::<usize>(tls_slot, tp);
        let r = f();
        wr::<usize>(tls_slot, saved_tls0);
        wr::<u64>(g_c88, saved_c88);
        r
    };

    for tv in [0u64, 0x1234_5678_9abc_def0] {
        let o = run(f_o, tv);
        let n = run(f_n, tv);
        crate::diff_eq!(o, n, "1411dcdf0 tv={:#x}", tv);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411e4980(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut B0C0_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_b0c0(_p1: usize) {
        B0C0_CALLED += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4209_b0c0, mock_b0c0 as *const ())?;

    let tls_slot = exe::tls_pointer();
    let saved_tls0 = rd::<usize>(tls_slot);

    let run = |f: F, count_90: u32| -> (u32, [u64; 4]) {
        B0C0_CALLED = 0;
        let mut ctx_obj = [0u8; 0xc0];
        let cop = ctx_obj.as_mut_ptr() as usize;
        wr::<u64>(cop + 0x10, 0x1000);
        wr::<u64>(cop + 0x18, 0x2000);

        let mut tls_blk = vec![0u8; 0x5b500];
        let tp = tls_blk.as_mut_ptr() as usize;
        wr::<u8>(tp + 0x70, 1);
        wr::<usize>(tp + 0x6340, cop);
        wr::<u64>(tp + 0x5b460 + 0x08, 0x5000);
        wr::<u32>(tp + 0x5b460 + 0x20, 2);
        wr::<u32>(tp + 0x5b460 + 0x24, 4);

        let mut entry_88 = [0u8; 0x28];
        let ep = entry_88.as_mut_ptr() as usize;
        wr::<i32>(ep, 1);
        wr::<i32>(ep + 0x10, 0);

        let mut p1 = [0u8; 0xb0];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x88, ep);
        wr::<u32>(p1p + 0x90, count_90);

        wr::<usize>(tls_slot, tp);
        f(p1p);
        wr::<usize>(tls_slot, saved_tls0);

        (B0C0_CALLED, rd::<[u64; 4]>(cop + 0x80))
    };

    for c90 in [0u32, 1] {
        let o = run(f_o, c90);
        let n = run(f_n, c90);
        crate::diff_eq!(o, n, "1411e4980 c90={}", c90);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411ed7e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut FREE_CALLED: u32 = 0;

    unsafe extern "win64" fn mock_3200(_chunk: usize, _desc: usize, _flag: u32, _ptr: usize) {
        FREE_CALLED += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;

    let mut arena_vec = vec![0u8; 0x0210_0000];
    let raw = arena_vec.as_mut_ptr() as usize;
    let arena = (raw + 0x01ff_ffff) & !0x01ff_ffffusize;
    let fake_heap_ptr = arena + 0x1_0000;

    let run = |f: F, mode: u8| -> (usize, u32, u32, u32) {
        FREE_CALLED = 0;
        let mut obj = [0u64; 4];
        let op = obj.as_mut_ptr() as usize;
        let init_ptr = match mode {
            0 => 0,
            1 => op + 0x10,
            _ => fake_heap_ptr,
        };
        wr::<usize>(op, init_ptr);
        wr::<u32>(op + 0x08, 42);
        wr::<u32>(op + 0x0c, 64);

        f(op);
        (
            rd::<usize>(op).wrapping_sub(op),
            rd::<u32>(op + 0x08),
            rd::<u32>(op + 0x0c),
            FREE_CALLED,
        )
    };

    for mode in [0u8, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1411ed7e0 mode={}", mode);
    }
    Ok(3)
}
