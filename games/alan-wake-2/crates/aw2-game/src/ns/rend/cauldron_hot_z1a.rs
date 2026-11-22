//! Decompiled `rend` hot functions shard Z1A (`0x1401deb40..0x140f939e0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline]
fn u64_to_f32(v: u64) -> f32 {
    if (v as i64) < 0 {
        let half = ((v >> 1) | (v & 1)) as i64 as f32;
        half + half
    } else {
        (v as i64) as f32
    }
}

#[inline]
unsafe fn f32_to_u64_msvc(mut x: f32) -> u64 {
    let two63 = f32::from_bits(0x5f00_0000);
    let mut adj: u64 = 0;
    if x >= two63 {
        x -= two63;
        if x < two63 {
            adj = 0x8000_0000_0000_0000;
        }
    }
    #[cfg(target_arch = "x86_64")]
    let cvt = core::arch::x86_64::_mm_cvttss_si64(core::arch::x86_64::_mm_set_ss(x)) as u64;
    #[cfg(not(target_arch = "x86_64"))]
    let cvt = if x.is_nan() || x < -9.223372e18 || x >= two63 {
        0x8000_0000_0000_0000u64
    } else {
        (x as i64) as u64
    };
    cvt.wrapping_add(adj)
}

/// `unknown_1401deb40`
/// Original: 0x1401deb40 (94 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1401deb40(param_1: usize, param_2: u64) {
    let saved_len = rd::<u32>(param_1 + 0x08);
    let dst = exe::call_va!(0x1_401d_ecf0, (param_1, param_2) -> usize);
    let byte_len = (rd::<u32>(param_1 + 0x08) as u64) << 2;
    let src = rd::<usize>(param_1);
    let _ = exe::call_va!(0x1_4183_acc8, (dst, src, byte_len) -> usize);
    exe::call_va!(0x1_401d_e830, (param_1));
    wr::<usize>(param_1, dst);
    wr::<u32>(param_1 + 0x0c, param_2 as u32);
    wr::<u32>(param_1 + 0x08, saved_len);
}

/// `unknown_1401e1660`
/// Original: 0x1401e1660 (364 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1401e1660(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    wr::<[u64; 6]>(param_1, rd::<[u64; 6]>(param_2));
    wr::<[u64; 3]>(param_1 + 0x30, [0; 3]);
    wr::<[u64; 3]>(param_1 + 0x48, rd::<[u64; 3]>(param_2 + 0x48));
    wr::<u64>(param_1 + 0x60, 0);

    if rd::<u8>(param_2 + 0x19) == 0 {
        let diff = rd::<u64>(param_2 + 0x38).wrapping_sub(rd::<u64>(param_2 + 0x30));
        let cap = exe::call_va!(0x1_401e_2020, (param_1, diff, param_3) -> u64);
        wr::<u64>(param_1 + 0x40, cap);
        let cap_f = u64_to_f32(cap);
        let v0 = f32_to_u64_msvc(cap_f * rd::<f32>(param_1 + 0x10));
        let f1 = cap_f * rd::<f32>(param_1 + 0x14);
        wr::<u64>(param_1, v0);
        let v1 = f32_to_u64_msvc(f1);
        wr::<u8>(param_1 + 0x18, 0);
        wr::<u64>(param_1 + 0x08, v1);
        return param_1;
    }

    let v0 = f32_to_u64_msvc(0.0f32 * rd::<f32>(param_1 + 0x10));
    let f1 = 0.0f32 * rd::<f32>(param_1 + 0x14);
    wr::<u64>(param_1, v0);
    let v1 = f32_to_u64_msvc(f1);
    wr::<u8>(param_1 + 0x18, 0);
    wr::<u64>(param_1 + 0x08, v1);
    exe::call_va!(0x1_401e_1c30, (param_1, param_2, param_3));
    param_1
}

/// `unknown_1401e1c30`
/// Original: 0x1401e1c30 (627 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1401e1c30(param_1: usize, param_2: usize, param_3: u64) {
    let diff = rd::<u64>(param_2 + 0x38).wrapping_sub(rd::<u64>(param_2 + 0x30));
    let rdi = exe::call_va!(0x1_401e_2020, (param_1, diff, param_3) -> u64);
    let mut table_ptr = rd::<usize>(param_1 + 0x60);

    if table_ptr == 0 || rdi != rd::<u64>(param_1 + 0x40) {
        if table_ptr != 0 {
            let r10 = table_ptr.wrapping_sub(1) & !0x01ff_ffffusize;
            let seg = (table_ptr.wrapping_sub(r10)) >> 16;
            let mut rdx_hdr = r10.wrapping_add(0x108).wrapping_add(seg.wrapping_mul(0x50));
            let back = rd::<u32>(rdx_hdr + 0x04) as usize;
            rdx_hdr = rdx_hdr.wrapping_sub(back);
            if exe::teb() == rd::<usize>(r10 + 0x100) {
                if rd::<u8>(rdx_hdr + 0x0e) == 0 {
                    wr::<usize>(table_ptr, rd::<usize>(rdx_hdr + 0x20));
                    let new_cnt = rd::<u32>(rdx_hdr + 0x18).wrapping_sub(1);
                    wr::<u32>(rdx_hdr + 0x18, new_cnt);
                    wr::<usize>(rdx_hdr + 0x20, table_ptr);
                    if new_cnt == 0 {
                        exe::call_va!(0x1_4253_71a0, (rdx_hdr));
                    }
                } else {
                    exe::call_va!(0x1_4253_3200, (r10, rdx_hdr, 1u8, table_ptr));
                }
            } else {
                exe::call_va!(0x1_4253_3200, (r10, rdx_hdr, 0u8, table_ptr));
            }
        }
        table_ptr = exe::call_va!(0x1_401e_2180, (table_ptr, rdi) -> usize);
        wr::<usize>(param_1 + 0x60, table_ptr);
    }

    let empty_entry = rd::<[u64; 3]>(param_1 + 0x48);
    let dst_end = table_ptr.wrapping_add((rdi as usize).wrapping_mul(0x18));
    let mut cur = table_ptr;
    while cur != dst_end {
        wr::<[u64; 3]>(cur, empty_entry);
        cur = cur.wrapping_add(0x18);
    }

    wr::<u64>(param_1 + 0x38, 0);
    wr::<u64>(param_1 + 0x30, 0);
    wr::<u64>(param_1 + 0x40, rdi);
    let cap_f = u64_to_f32(rdi);
    let v0 = f32_to_u64_msvc(cap_f * rd::<f32>(param_1 + 0x10));
    let f1 = cap_f * rd::<f32>(param_1 + 0x14);
    wr::<u64>(param_1, v0);
    wr::<u8>(param_1 + 0x18, 0);
    let v1 = f32_to_u64_msvc(f1);
    wr::<u64>(param_1 + 0x08, v1);

    let mut src_cur = rd::<usize>(param_2 + 0x60);
    let src_end = src_cur.wrapping_add((rd::<u64>(param_2 + 0x40) as usize).wrapping_mul(0x18));
    while src_cur != src_end {
        let key = rd::<u64>(src_cur);
        if key != rd::<u64>(param_2 + 0x48)
            && (rd::<u64>(param_2 + 0x30) == 0 || key != rd::<u64>(param_2 + 0x28))
        {
            break;
        }
        src_cur = src_cur.wrapping_add(0x18);
    }

    loop {
        let cur_src_end = rd::<usize>(param_2 + 0x60)
            .wrapping_add((rd::<u64>(param_2 + 0x40) as usize).wrapping_mul(0x18));
        if src_cur == cur_src_end {
            wr::<u32>(param_1 + 0x1c, rd::<u32>(param_1 + 0x1c).wrapping_add(1));
            return;
        }

        let mask = rd::<u64>(param_1 + 0x40).wrapping_sub(1);
        let mut num_probes: u64 = 0;
        let dst_table = rd::<usize>(param_1 + 0x60);
        let mut bucket = rd::<u64>(src_cur) & mask;
        let empty_key = rd::<u64>(param_1 + 0x48);
        while rd::<u64>(dst_table.wrapping_add((bucket as usize).wrapping_mul(0x18))) != empty_key {
            num_probes = num_probes.wrapping_add(1);
            bucket = bucket.wrapping_add(num_probes) & mask;
        }

        let dst_slot = dst_table.wrapping_add((bucket as usize).wrapping_mul(0x18));
        wr::<[u64; 3]>(dst_slot, rd::<[u64; 3]>(src_cur));
        wr::<u64>(param_1 + 0x38, rd::<u64>(param_1 + 0x38).wrapping_add(1));

        loop {
            src_cur = src_cur.wrapping_add(0x18);
            if src_cur == src_end {
                break;
            }
            let key = rd::<u64>(src_cur);
            if key != rd::<u64>(param_2 + 0x48)
                && (rd::<u64>(param_2 + 0x30) == 0 || key != rd::<u64>(param_2 + 0x28))
            {
                break;
            }
        }
    }
}

/// `sparsehash_1401e2020`
/// Original: 0x1401e2020 (173 bytes, Throws::Discard)
pub unsafe extern "win64" fn sparsehash_1401e2020(
    param_1: usize,
    param_2: u64,
    param_3: u64,
) -> u64 {
    let enlarge_factor = rd::<f32>(param_1 + 0x10);
    let mut r9: u64 = 4;
    loop {
        if r9 >= param_3 {
            let thresh = f32_to_u64_msvc(u64_to_f32(r9) * enlarge_factor);
            if param_2 < thresh {
                return r9;
            }
        }
        let next = r9.wrapping_add(r9);
        if next < r9 {
            let mut exc = [0u64; 5];
            let msg = exe::ptr::<u8>(0x1_4307_90d8) as usize;
            exe::call_va!(0x1_401d_f630, (exc.as_mut_ptr() as usize, msg));
            let ti = exe::ptr::<u8>(0x1_437f_cf88) as usize;
            exe::call_va!(0x1_4183_acce, (exc.as_mut_ptr() as usize, ti));
            return 0;
        }
        r9 = next;
    }
}

/// `unknown_1401f9050`
/// Original: 0x1401f9050 (151 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1401f9050(param_1: usize, mut param_2: usize, param_3: usize) {
    while param_2 != param_3 {
        let rem = (param_3.wrapping_sub(param_2)) as u64;
        let needed = rd::<u64>(param_1 + 0x10).wrapping_add(rem);
        if needed > rd::<u64>(param_1 + 0x18) {
            let vtbl = rd::<usize>(param_1);
            let grow_fn = rd::<usize>(vtbl);
            exe::call_va!(grow_fn, (param_1, needed));
        }
        let cur_len = rd::<u64>(param_1 + 0x10);
        let avail = rd::<u64>(param_1 + 0x18).wrapping_sub(cur_len);
        let chunk = if avail >= rem { rem } else { avail };
        if chunk != 0 {
            let dst = rd::<usize>(param_1 + 0x08).wrapping_add(cur_len as usize);
            let _ = exe::call_va!(0x1_4183_acc8, (dst, param_2, chunk) -> usize);
        }
        wr::<u64>(param_1 + 0x10, rd::<u64>(param_1 + 0x10).wrapping_add(chunk));
        param_2 = param_2.wrapping_add(chunk as usize);
    }
}

/// `sparsehash_1401fa730`
/// Original: 0x1401fa730 (321 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1401fa730(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mask = rd::<u64>(param_1 + 0x40).wrapping_sub(1);
    let table = rd::<usize>(param_1 + 0x58);
    let empty_key = rd::<u64>(param_1 + 0x48);

    let key_bytes = rd::<[u8; 8]>(param_3);
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in &key_bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x0100_0000_01b3);
    }

    let mut bucket = h & mask;
    let mut insert_pos = u64::MAX;
    let mut num_probes: u64 = 0;

    if rd::<u64>(table.wrapping_add((bucket as usize) << 4)) != empty_key {
        let use_deleted = rd::<u64>(param_1 + 0x30);
        let key_val = rd::<u64>(param_3);
        loop {
            let slot_key = rd::<u64>(table.wrapping_add((bucket as usize) << 4));
            if use_deleted != 0 && rd::<u64>(param_1 + 0x28) == slot_key {
                if insert_pos == u64::MAX {
                    insert_pos = bucket;
                }
            } else if key_val == slot_key {
                wr::<u64>(param_2, bucket);
                wr::<u64>(param_2 + 0x08, u64::MAX);
                return param_2;
            }
            num_probes = num_probes.wrapping_add(1);
            bucket = bucket.wrapping_add(num_probes) & mask;
            if rd::<u64>(table.wrapping_add((bucket as usize) << 4)) == empty_key {
                break;
            }
        }
    }

    wr::<u64>(param_2, u64::MAX);
    wr::<u64>(
        param_2 + 0x08,
        if insert_pos == u64::MAX {
            bucket
        } else {
            insert_pos
        },
    );
    param_2
}

/// `std_140200070`
/// Original: 0x140200070 (49 bytes, Throws::Discard)
pub unsafe extern "win64" fn std_140200070() {
    let mut exc = [0u64; 3];
    exe::call_va!(0x1_4020_00b0, (exc.as_mut_ptr() as usize));
    let ti = exe::ptr::<u8>(0x1_437f_d588) as usize;
    exe::call_va!(0x1_4183_acce, (exc.as_mut_ptr() as usize, ti));
}

/// `unknown_140217df0`
/// Original: 0x140217df0 (148 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140217df0(param_1: usize) -> u64 {
    if rd::<u64>(param_1) == 0 {
        return 0;
    }
    let g_ver_addr = exe::ptr::<u32>(0x1_4384_1b24) as usize;
    // Lock-free fast path: (ver, ptr) are published with xchg below; loads are acquire.
    if crate::simd_ops::ru32((param_1 + 0x10) as u64) != crate::simd_ops::ru32(g_ver_addr as u64) {
        let mut backoff: u64 = 0x64;
        loop {
            while rd::<u8>(param_1 + 0x14) != 0 {
                core::hint::spin_loop();
            }
            let prev = (*((param_1 + 0x14) as *const core::sync::atomic::AtomicU8))
                .swap(1, core::sync::atomic::Ordering::SeqCst);
            if prev == 0 {
                break;
            }
            exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
        }
        let mgr = rd::<u64>(exe::ptr::<u64>(0x1_43a0_8300) as usize);
        let new_ver = crate::simd_ops::ru32(g_ver_addr as u64);
        let val = exe::call_va!(0x1_4248_b000, (mgr, param_1) -> u64);
        let _ = crate::simd_ops::atomic_xchg_u64((param_1 + 0x08) as u64, val); // xchg qword [rbx+8]
        let _ = crate::simd_ops::atomic_xchg_u32((param_1 + 0x10) as u64, new_ver); // xchg dword [rbx+0x10]
        crate::simd_ops::wu8((param_1 + 0x14) as u64, 0); // spinlock release (x86 mov = release)
    }
    crate::simd_ops::ru64((param_1 + 0x08) as u64)
}

/// `unknown_140275010`
/// Original: 0x140275010 (500 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140275010(param_1: usize, param_2: usize) {
    let m = rd::<[f32; 9]>(param_2);
    let trace = (m[0] + m[4]) + m[8];
    if trace > 0.0f32 {
        let root = (trace + 1.0f32).sqrt();
        let w0 = root * 0.5f32;
        let w1 = if w0 > -1.0f32 { w0 } else { -1.0f32 };
        let w = if w1 < 1.0f32 { w1 } else { 1.0f32 };
        wr::<f32>(param_1, w);
        let s = 0.5f32 / root;
        wr::<f32>(param_1 + 0x04, (m[5] - m[7]) * s);
        wr::<f32>(param_1 + 0x08, (m[6] - m[2]) * s);
        wr::<f32>(param_1 + 0x0c, (m[1] - m[3]) * s);
        return;
    }

    let mut i: usize = if m[4] > m[0] { 1 } else { 0 };
    if m[8] > m[i * 4] {
        i = 2;
    }
    const NEXT: [usize; 3] = [1, 2, 0];
    let j = NEXT[i];
    let k = NEXT[j];
    let mut root = ((m[i * 4] - m[j * 4]) - m[k * 4] + 1.0f32).sqrt();
    let mut q = [0.0f32; 3];
    q[i] = root * 0.5f32;
    if root != 0.0f32 {
        root = 0.5f32 / root;
    }
    let w0 = (m[j * 3 + k] - m[k * 3 + j]) * root;
    let w1 = if w0 > -1.0f32 { w0 } else { -1.0f32 };
    let w = if w1 < 1.0f32 { w1 } else { 1.0f32 };
    wr::<f32>(param_1, w);
    q[j] = (m[i * 3 + j] + m[j * 3 + i]) * root;
    q[k] = (m[i * 3 + k] + m[k * 3 + i]) * root;
    wr::<f32>(param_1 + 0x04, q[0]);
    wr::<f32>(param_1 + 0x0c, q[2]);
    wr::<f32>(param_1 + 0x08, q[1]);
}

/// `unknown_14028e490`
/// Original: 0x14028e490 (143 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14028e490(param_1: usize, param_2: u64) {
    let old_len = rd::<u32>(param_1 + 0x08) as u64;
    let old_cap = rd::<u32>(param_1 + 0x0c) as u64;
    if param_2 > old_cap {
        exe::call_va!(0x1_402d_7910, (param_1, param_2));
        let diff = param_2.wrapping_sub(old_len);
        if diff != 0 {
            let dst = rd::<usize>(param_1).wrapping_add(rd::<u32>(param_1 + 0x08) as usize);
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff) -> usize);
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 > old_len {
        let diff = param_2.wrapping_sub(old_len);
        if diff != 0 {
            let dst = rd::<usize>(param_1).wrapping_add(old_len as usize);
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff) -> usize);
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 != old_len {
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    }
}

/// `unknown_1402d7910`
/// Original: 0x1402d7910 (112 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1402d7910(param_1: usize, param_2: u64) {
    let old_cap = rd::<u32>(param_1 + 0x0c) as u64;
    let saved_len = rd::<u32>(param_1 + 0x08);
    let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
    if new_cap < param_2 {
        new_cap = param_2;
    }
    let dst = exe::call_va!(0x1_4020_9f30, (param_1, new_cap) -> usize);
    let cur_len = rd::<u32>(param_1 + 0x08) as u64;
    let src = rd::<usize>(param_1);
    let _ = exe::call_va!(0x1_4183_acc8, (dst, src, cur_len) -> usize);
    exe::call_va!(0x1_401d_e830, (param_1));
    wr::<usize>(param_1, dst);
    wr::<u32>(param_1 + 0x08, saved_len);
    wr::<u32>(param_1 + 0x0c, new_cap as u32);
}

/// `unknown_140460d40`
/// Original: 0x140460d40 (224 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140460d40(_param_1: u64, param_2: usize, param_3: usize) {
    let count = rd::<u64>(param_2 + 0x18);
    if count != 0 {
        let mut buf = [0u64; 10];
        for idx in 0..count {
            let r8_val = rd::<u64>(param_3);
            buf[0] = rd::<u64>(param_3 + 0x08);
            buf[1] = rd::<u64>(param_3 + 0x10);
            buf[2] = rd::<u64>(param_3 + 0x18);
            buf[3] = rd::<u64>(param_2 + 0x38);
            buf[4] = rd::<u64>(param_2 + 0x20);
            buf[5] = rd::<u64>(param_2 + 0x28);
            buf[6] = rd::<u64>(param_2 + 0x30);
            buf[7] = rd::<u64>(param_2 + 0x38);
            buf[8] = rd::<u64>(param_2 + 0x08);
            buf[9] = idx;
            exe::call_va!(
                0x1_419d_f070,
                (
                    buf.as_mut_ptr().add(4) as usize,
                    buf.as_mut_ptr() as usize,
                    r8_val
                )
            );
        }
    }
}

/// `unknown_14050b980`
/// Original: 0x14050b980 (365 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14050b980(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let idx = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(idx * 8).wrapping_add(0x88));
    let count = if block != 0 {
        rd::<u32>(world.wrapping_add(idx * 4).wrapping_add(0x10088)) as u64
    } else {
        0
    };
    if count != 0 {
        let tbl = rd::<usize>(world + 0x580e0);
        let id_0d8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_00d8) as usize) as usize;
        let id_63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
        let val_0d8 = rd::<u64>(tbl.wrapping_add(id_0d8 * 8));
        let val_63c = rd::<u64>(tbl.wrapping_add(id_63c * 8));

        let mut pkt = [0u64; 6];
        pkt[0] = param_2 as u64;
        pkt[1] = block as u64;
        pkt[2] = world as u64;
        pkt[3] = count;
        let mut tmp_out = [0u64; 4];
        let ret_ptr = exe::call_va!(
            0x1_4050_f2b0,
            (pkt.as_mut_ptr() as usize, tmp_out.as_mut_ptr() as usize) -> usize
        );
        let offs = rd::<[u64; 4]>(ret_ptr);
        for i in 0..count {
            pkt[0] = offs[0];
            pkt[1] = offs[1];
            pkt[2] = offs[2];
            pkt[3] = offs[3];
            pkt[4] = block as u64;
            pkt[5] = i;
            exe::call_va!(
                0x1_41a7_2040,
                (pkt.as_mut_ptr() as usize, val_63c, val_0d8)
            );
        }
    }
}

/// `unknown_140993970`
/// Original: 0x140993970 (1074 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_140993970(
    param_1: usize,
    param_2: usize,
    param_3: u8,
) -> u64 {
    let mut local_flag = param_3;
    let crc_val = rd::<u32>(exe::ptr::<u32>(0x1_4387_ddd8) as usize);
    let name_ptr = rd::<u64>(exe::ptr::<u64>(0x1_4387_e188) as usize);
    exe::call_va!(
        0x1_401e_9fa0,
        (param_2, param_1, name_ptr, crc_val, 0x0fu32, 1u32)
    );
    exe::call_va!(0x1_406e_02b0, (param_1, param_2));
    exe::call_va!(0x1_4088_b020, (param_2, param_1 + 0x08, 1u32));

    let skip_stream = |p2: usize, n: u32| {
        let p320 = rd::<usize>(p2 + 0x320);
        if p320 != 0 {
            let inner = rd::<usize>(p320 + 0x08);
            let f = rd::<usize>(rd::<usize>(inner) + 0x38);
            exe::call_va!(f, (inner, n, 0u32));
        }
    };

    if rd::<i32>(param_2 + 0x328) >= 2 {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x80), (s, param_1 + 0x10));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u8>(param_1 + 0x10, 0);
    }

    if rd::<i32>(param_2 + 0x328) >= 0x0c {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x80), (s, param_1 + 0x11));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u8>(param_1 + 0x11, 1);
    }

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(2) <= 5
    {
        skip_stream(param_2, 1);
    }

    if rd::<i32>(param_2 + 0x328) >= 2 {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x48), (s, param_1 + 0x14));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x14, 2);
    }

    if rd::<i32>(param_2 + 0x328) >= 2 {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x38), (s, param_1 + 0x18));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x18, 0);
    }

    for _ in 0..2 {
        if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
            && rd::<u32>(param_2 + 0x328).wrapping_sub(2) <= 7
        {
            skip_stream(param_2, 4);
        }
    }

    if rd::<i32>(param_2 + 0x328) >= 3 {
        let mut local_str = [0u64; 34]; // 272B >= 264B (b3 = 0x20)
        let sp = local_str.as_mut_ptr() as usize;
        wr::<u64>(sp, 0x2000_0020);
        wr::<u8>(sp + 0x08, 0);
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x20), (s, sp));
        let s2 = rd::<usize>(param_2 + 0x338);
        let mut tmp_val: u32 = 0;
        exe::call_va!(
            rd::<usize>(rd::<usize>(s2) + 0x40),
            (s2, (&mut tmp_val as *mut u32) as usize)
        );
        wr::<u32>(param_1 + 0x2c, tmp_val);
        exe::call_va!(0x1_4240_ab10, (sp));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x2c, 0);
    }

    for _ in 0..2 {
        if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
            && rd::<u32>(param_2 + 0x328).wrapping_sub(4) <= 2
        {
            skip_stream(param_2, 1);
        }
    }

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3 && rd::<i32>(param_2 + 0x328) == 5 {
        skip_stream(param_2, 1);
    }

    let ver_c2c = rd::<i32>(param_2 + 0x328);
    if ver_c2c < 0x0e {
        if ver_c2c >= 5 {
            let s = rd::<usize>(param_2 + 0x338);
            exe::call_va!(
                rd::<usize>(rd::<usize>(s) + 0x80),
                (s, (&mut local_flag as *mut u8) as usize)
            );
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            local_flag = 0;
        }
    }

    for off in [0x12usize, 0x13] {
        if rd::<i32>(param_2 + 0x328) >= 0x0e {
            let s = rd::<usize>(param_2 + 0x338);
            exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x80), (s, param_1 + off));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u8>(param_1 + off, 1);
        }
    }

    if rd::<u32>(param_2 + 0x318).wrapping_sub(1) > 3
        && rd::<u32>(param_2 + 0x328).wrapping_sub(9) <= 5
    {
        skip_stream(param_2, 4);
    }

    if rd::<i32>(param_2 + 0x328) >= 0x0d {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x38), (s, param_1 + 0x1c));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x1c, 0xbf80_0000);
    }

    if rd::<i32>(param_2 + 0x328) >= 0x0b {
        let s = rd::<usize>(param_2 + 0x338);
        for off in [0x20usize, 0x24, 0x28] {
            exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x38), (s, param_1 + off));
        }
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x20, 0x3f80_0000);
        wr::<u32>(param_1 + 0x24, 0x3f80_0000);
        wr::<u32>(param_1 + 0x28, 0x3f80_0000);
    }

    if rd::<i32>(param_2 + 0x328) < 0x0e {
        let b = if local_flag == 0 { 1u8 } else { 0u8 };
        wr::<u8>(param_1 + 0x12, b);
        wr::<u8>(param_1 + 0x13, b);
    }

    exe::call_va!(0x1_401e_9e70, (param_2));
    0
}

/// `unknown_140a02480`
/// Original: 0x140a02480 (425 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140a02480(param_1: usize, param_2: u64) {
    let old_len = rd::<u32>(param_1 + 0x08) as u64;
    let old_cap = rd::<u32>(param_1 + 0x0c) as u64;
    if param_2 > old_cap {
        let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
        if new_cap < param_2 {
            new_cap = param_2;
        }
        exe::call_va!(0x1_40a5_b7c0, (param_1, new_cap));
        let count = param_2.wrapping_sub(old_len);
        if count != 0 {
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let mut dst = rd::<usize>(param_1).wrapping_add(cur_len.wrapping_mul(0x30));
            let c16 = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_7660) as usize);
            for _ in 0..count {
                wr::<[u32; 4]>(dst, [0x3f80_0000, 0, 0, 0]);
                wr::<[u32; 4]>(dst + 0x10, c16);
                wr::<[u32; 4]>(dst + 0x20, c16);
                dst = dst.wrapping_add(0x30);
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 > old_len {
        let count = param_2.wrapping_sub(old_len);
        if count != 0 {
            let mut dst = rd::<usize>(param_1).wrapping_add((old_len as usize).wrapping_mul(0x30));
            let c16 = rd::<[u32; 4]>(exe::ptr::<u32>(0x1_4342_7660) as usize);
            for _ in 0..count {
                wr::<[u32; 4]>(dst, [0x3f80_0000, 0, 0, 0]);
                wr::<[u32; 4]>(dst + 0x10, c16);
                wr::<[u32; 4]>(dst + 0x20, c16);
                dst = dst.wrapping_add(0x30);
            }
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 != old_len {
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    }
}

/// `content_140e4e780`
/// Original: 0x140e4e780 (750 bytes, Throws::Discard)
pub unsafe extern "win64" fn content_140e4e780(param_1: usize, param_2: usize) -> u64 {
    let crc_val = rd::<u32>(exe::ptr::<u32>(0x1_4387_fb7c) as usize);
    let name_ptr = rd::<u64>(exe::ptr::<u64>(0x1_4387_fb58) as usize);
    exe::call_va!(
        0x1_401e_9fa0,
        (param_2, param_1, name_ptr, crc_val, 0x12u32, 0x0du32)
    );

    if rd::<i32>(param_2 + 0x328) >= 0x0b {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x20), (s, param_1));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        let empty_ptr = exe::ptr::<u8>(0x1_426c_dcb0) as usize as u64;
        let mut sv: [u64; 2] = [empty_ptr, 0];
        exe::call_va!(
            0x1_4240_9b50,
            (param_1, sv.as_mut_ptr() as usize, 0u64, 0u64)
        );
    }

    if rd::<i32>(param_2 + 0x328) >= 0x10 {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(0x1_401e_8620, (param_1 + 0x48));
        exe::call_va!(rd::<usize>(rd::<usize>(s)), (s, param_1 + 0x48));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        let mut tmp = [0u64; 3];
        exe::call_va!(0x1_401e_84a0, (param_1 + 0x48, tmp.as_mut_ptr() as usize));
    }

    if rd::<i32>(param_2 + 0x328) >= 0x0b {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(0x1_401e_8620, (param_1 + 0x60));
        exe::call_va!(rd::<usize>(rd::<usize>(s)), (s, param_1 + 0x60));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        let mut tmp = [0u64; 3];
        exe::call_va!(0x1_401e_84a0, (param_1 + 0x60, tmp.as_mut_ptr() as usize));
    }

    if rd::<i32>(param_2 + 0x328) >= 0x0b {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x38), (s, param_1 + 0x78));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x78, 0);
    }

    for off in [0x7cusize, 0x7e] {
        if rd::<i32>(param_2 + 0x328) >= 0x0b {
            let s = rd::<usize>(param_2 + 0x338);
            exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x80), (s, param_1 + off));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u8>(param_1 + off, 0);
        }
    }

    exe::call_va!(0x1_4055_7390, (param_2, param_1 + 0x88, 0x0bu32));
    exe::call_va!(0x1_4055_7390, (param_2, param_1 + 0x90, 0x0bu32));

    if rd::<i32>(param_2 + 0x328) >= 0x0c {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x80), (s, param_1 + 0x7f));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u8>(param_1 + 0x7f, 0);
    }

    let ver_988 = rd::<i32>(param_2 + 0x328);
    if ver_988 < 0x11 {
        if ver_988 >= 0x0e {
            let s = rd::<usize>(param_2 + 0x338);
            exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x38), (s, param_1 + 0x80));
        } else if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u32>(param_1 + 0x80, 0);
        }
    }

    if rd::<i32>(param_2 + 0x328) >= 0x11 {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x38), (s, param_1 + 0x84));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x84, 0);
    }

    if rd::<i32>(param_2 + 0x328) >= 0x0f {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x80), (s, param_1 + 0x7d));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u8>(param_1 + 0x7d, 0);
    }

    if rd::<i32>(param_2 + 0x328) >= 0x12 {
        let s = rd::<usize>(param_2 + 0x338);
        exe::call_va!(rd::<usize>(rd::<usize>(s) + 0x38), (s, param_1 + 0x98));
    } else if rd::<u8>(param_2 + 0x369) != 0 {
        wr::<u32>(param_1 + 0x98, 0);
    }

    if rd::<i32>(param_2 + 0x328) < 0x11 {
        wr::<u32>(param_1 + 0x84, rd::<u32>(param_1 + 0x80));
    }

    exe::call_va!(0x1_401e_9e70, (param_2));
    0
}

/// `unknown_140f82270`
/// Original: 0x140f82270 (254 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f82270(param_1: usize) {
    let world = rd::<usize>(param_1);
    let arr_8 = rd::<usize>(world + 0x08);
    let map_20 = rd::<usize>(rd::<usize>(world) + 0x20);
    let read_span = |g_va: u64| -> [u64; 3] {
        let id = rd::<u16>(exe::ptr::<u16>(g_va) as usize) as usize;
        let slot = rd::<u16>(map_20.wrapping_add(id * 2)) as usize;
        let entry = arr_8.wrapping_add(slot * 0x10);
        [
            world as u64,
            rd::<u32>(entry + 0x08) as u64,
            rd::<u64>(entry),
        ]
    };

    let span_f14 = read_span(0x1_43cc_7f14);
    let span_f10 = read_span(0x1_43cc_7f10);
    let span_f0c = read_span(0x1_43cc_7f0c);

    let tbl = rd::<usize>(world + 0x580e0);
    let id_c80 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_7c80) as usize) as usize;
    let id_c84 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_7c84) as usize) as usize;
    let arg4 = rd::<u64>(tbl.wrapping_add(id_c80 * 8));
    let arg5 = rd::<u64>(tbl.wrapping_add(id_c84 * 8));

    let mut frame = [0u64; 16];
    let fp = frame.as_mut_ptr() as usize;
    wr::<[u64; 3]>(fp, span_f10);
    wr::<[u64; 3]>(fp + 0x20, span_f14);
    wr::<[u64; 3]>(fp + 0x40, span_f14);
    wr::<[u64; 3]>(fp + 0x60, span_f0c);

    exe::call_va!(0x1_41e2_05a0, (fp + 0x20, fp, fp + 0x60, arg4, arg5));
}

/// `unknown_140f939e0`
/// Original: 0x140f939e0 (181 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_140f939e0(param_1: usize) {
    let world = rd::<usize>(param_1);
    let arr_8 = rd::<usize>(world + 0x08);
    let map_20 = rd::<usize>(rd::<usize>(world) + 0x20);
    let read_span = |g_va: u64| -> [u64; 3] {
        let id = rd::<u16>(exe::ptr::<u16>(g_va) as usize) as usize;
        let slot = rd::<u16>(map_20.wrapping_add(id * 2)) as usize;
        let entry = arr_8.wrapping_add(slot * 0x10);
        [
            world as u64,
            rd::<u32>(entry + 0x08) as u64,
            rd::<u64>(entry),
        ]
    };

    let span_3bc = read_span(0x1_43cc_93bc);
    let span_3b8 = read_span(0x1_43cc_93b8);

    let tbl = rd::<usize>(world + 0x580e0);
    let id_228 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_9228) as usize) as usize;
    let arg4 = rd::<u64>(tbl.wrapping_add(id_228 * 8));

    let mut frame = [0u64; 12];
    let fp = frame.as_mut_ptr() as usize;
    wr::<[u64; 3]>(fp, span_3bc);
    wr::<[u64; 3]>(fp + 0x20, span_3bc);
    wr::<[u64; 3]>(fp + 0x40, span_3b8);

    exe::call_va!(0x1_41e3_30a0, (fp, fp + 0x40, world as u64, arg4));
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1401deb40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DST_BUF: [u32; 8] = [0; 8];
    static mut CALL_LOG: (u64, u32) = (0, 0);

    unsafe extern "win64" fn mock_ecf0(_p1: usize, cap: u64) -> usize {
        CALL_LOG.0 = cap;
        core::ptr::addr_of_mut!(DST_BUF) as usize
    }
    unsafe extern "win64" fn mock_e830(p1: usize) {
        CALL_LOG.1 += 1;
        wr::<usize>(p1, 0);
        wr::<u64>(p1 + 0x08, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_ecf0, mock_ecf0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F, len: u32, new_cap: u64| -> (usize, u32, u32, [u32; 8], (u64, u32)) {
        DST_BUF = [0xdead_beef; 8];
        CALL_LOG = (0, 0);
        let src: [u32; 4] = [0x11, 0x22, 0x33, 0x44];
        let mut hdr = [0u64; 2];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp, src.as_ptr() as usize);
        wr::<u32>(hp + 0x08, len);
        wr::<u32>(hp + 0x0c, 4);

        f(hp, new_cap);
        (
            rd::<usize>(hp),
            rd::<u32>(hp + 0x08),
            rd::<u32>(hp + 0x0c),
            DST_BUF,
            CALL_LOG,
        )
    };

    for (len, cap) in [(0u32, 8u64), (3, 16)] {
        let o = run(f_o, len, cap);
        let n = run(f_n, len, cap);
        crate::diff_eq!(o, n, "1401deb40 len={} cap={}", len, cap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1401e1660(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: (u64, u64, u64, u32) = (0, 0, 0, 0);

    unsafe extern "win64" fn mock_2020(_p1: usize, diff: u64, min_b: u64) -> u64 {
        CALL_TRACE.0 = diff;
        CALL_TRACE.1 = min_b;
        16
    }
    unsafe extern "win64" fn mock_1c30(p1: usize, _p2: usize, min_b: u64) {
        CALL_TRACE.2 = min_b;
        CALL_TRACE.3 += 1;
        wr::<u64>(p1 + 0x40, 32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_2020, mock_2020 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_1c30, mock_1c30 as *const ())?;

    let run = |f: F, flag_19: u8| -> ([u64; 13], (u64, u64, u64, u32)) {
        CALL_TRACE = (0, 0, 0, 0);
        let mut dst = [0xaaaa_bbbb_cccc_ddddu64; 13];
        let mut src = [0u64; 13];
        let sp = src.as_mut_ptr() as usize;
        for (i, w) in src.iter_mut().enumerate() {
            *w = (i as u64 + 1) * 0x101;
        }
        wr::<f32>(sp + 0x10, 0.75);
        wr::<f32>(sp + 0x14, 0.25);
        wr::<u8>(sp + 0x19, flag_19);
        wr::<u64>(sp + 0x30, 2);
        wr::<u64>(sp + 0x38, 7);

        let dp = dst.as_mut_ptr() as usize;
        let ret = f(dp, sp, 8);
        assert_eq!(ret, dp);
        (dst, CALL_TRACE)
    };

    for flag in [0u8, 1] {
        let o = run(f_o, flag);
        let n = run(f_n, flag);
        crate::diff_eq!(o, n, "1401e1660 flag={}", flag);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1401e1c30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DST_BUCKETS: [u64; 12] = [0; 12];
    static mut ALLOC_ARG: u64 = 0;

    unsafe extern "win64" fn mock_2020(_p1: usize, _diff: u64, _min_b: u64) -> u64 {
        4
    }
    unsafe extern "win64" fn mock_2180(_rcx: usize, cnt: u64) -> usize {
        ALLOC_ARG = cnt;
        core::ptr::addr_of_mut!(DST_BUCKETS) as usize
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_2020, mock_2020 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_2180, mock_2180 as *const ())?;

    let run = |f: F, prealloc: bool| -> ([u64; 13], [u64; 12], u64) {
        DST_BUCKETS = [0; 12];
        ALLOC_ARG = 0;
        let src_buckets: [u64; 12] = [
            u64::MAX,
            0,
            0,
            u64::MAX - 1,
            0,
            0,
            5,
            50,
            500,
            9,
            90,
            900,
        ];
        let mut src = [0u64; 13];
        let sp = src.as_mut_ptr() as usize;
        wr::<u64>(sp + 0x28, u64::MAX - 1);
        wr::<u64>(sp + 0x30, 1);
        wr::<u64>(sp + 0x38, 3);
        wr::<u64>(sp + 0x40, 4);
        wr::<u64>(sp + 0x48, u64::MAX);
        wr::<usize>(sp + 0x60, src_buckets.as_ptr() as usize);

        let mut dst = [0u64; 13];
        let dp = dst.as_mut_ptr() as usize;
        wr::<f32>(dp + 0x10, 0.75);
        wr::<f32>(dp + 0x14, 0.25);
        wr::<u32>(dp + 0x1c, 10);
        wr::<u64>(dp + 0x40, if prealloc { 4 } else { 0 });
        wr::<u64>(dp + 0x48, u64::MAX);
        wr::<u64>(dp + 0x50, 0x1111);
        wr::<u64>(dp + 0x58, 0x2222);
        wr::<usize>(
            dp + 0x60,
            if prealloc {
                core::ptr::addr_of_mut!(DST_BUCKETS) as usize
            } else {
                0
            },
        );

        f(dp, sp, 4);
        dst[12] = 0;
        (dst, DST_BUCKETS, ALLOC_ARG)
    };

    for prealloc in [false, true] {
        let o = run(f_o, prealloc);
        let n = run(f_n, prealloc);
        crate::diff_eq!(o, n, "1401e1c30 prealloc={}", prealloc);
    }
    Ok(2)
}

pub unsafe fn diff_sparsehash_1401e2020(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, u64) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mut ht = [0u8; 0x20];
    let hp = ht.as_mut_ptr() as usize;
    wr::<f32>(hp + 0x10, 0.5);

    for (elts, min_b) in [(0u64, 0u64), (1, 4), (5, 4), (2, 32), (100, 8)] {
        let o = f_o(hp, elts, min_b);
        let n = f_n(hp, elts, min_b);
        crate::diff_eq!(o, n, "1401e2020 elts={} min_b={}", elts, min_b);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_1401f9050(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CALLS: (u32, u64) = (0, 0);

    unsafe extern "win64" fn mock_grow(this: usize, needed: u64) {
        GROW_CALLS.0 += 1;
        GROW_CALLS.1 = needed;
        let cur_cap = rd::<u64>(this + 0x18);
        wr::<u64>(this + 0x18, cur_cap + 4);
    }

    let vtbl: [usize; 1] = [mock_grow as *const () as usize];
    let src_bytes: [u8; 7] = [10, 20, 30, 40, 50, 60, 70];

    let run = |f: F, len: usize| -> ([u8; 16], u64, u64, (u32, u64)) {
        GROW_CALLS = (0, 0);
        let mut backing = [0u8; 16];
        let mut buf_obj: [u64; 4] = [
            vtbl.as_ptr() as usize as u64,
            backing.as_mut_ptr() as usize as u64,
            1,
            3,
        ];
        let bp = buf_obj.as_mut_ptr() as usize;
        let s = src_bytes.as_ptr() as usize;
        f(bp, s, s + len);
        (backing, buf_obj[2], buf_obj[3], GROW_CALLS)
    };

    for len in [0usize, 2, 7] {
        let o = run(f_o, len);
        let n = run(f_n, len);
        crate::diff_eq!(o, n, "1401f9050 len={}", len);
    }
    Ok(3)
}

pub unsafe fn diff_sparsehash_1401fa730(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let key_a: u64 = 0x1234_5678_9abc_def0;
    let key_b: u64 = 0x0fed_cba9_8765_4321;
    let mut hash_a: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in &key_a.to_le_bytes() {
        hash_a ^= b as u64;
        hash_a = hash_a.wrapping_mul(0x0100_0000_01b3);
    }
    let idx0 = (hash_a & 7) as usize;
    let idx1 = ((idx0 + 1) & 7) as usize;

    let mut buckets = [u64::MAX; 16]; // 8 slots of [key, val]
    buckets[idx0 * 2] = u64::MAX - 1; // deleted tombstone at first probe
    buckets[idx1 * 2] = key_a;

    let mut ht = [0u8; 0x60];
    let hp = ht.as_mut_ptr() as usize;
    wr::<u64>(hp + 0x28, u64::MAX - 1);
    wr::<u64>(hp + 0x30, 1);
    wr::<u64>(hp + 0x40, 8);
    wr::<u64>(hp + 0x48, u64::MAX);
    wr::<usize>(hp + 0x58, buckets.as_ptr() as usize);

    for key in [key_a, key_b] {
        let mut out_o = [0u64; 2];
        let mut out_n = [0u64; 2];
        let ro = f_o(hp, out_o.as_mut_ptr() as usize, (&key as *const u64) as usize);
        let rn = f_n(hp, out_n.as_mut_ptr() as usize, (&key as *const u64) as usize);
        assert_eq!(ro, out_o.as_ptr() as usize);
        assert_eq!(rn, out_n.as_ptr() as usize);
        crate::diff_eq!(out_o, out_n, "1401fa730 key={:#x}", key);
    }
    Ok(2)
}

pub unsafe fn diff_std_140200070(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: ([u64; 3], [u64; 3], usize) = ([0; 3], [0; 3], 0);

    unsafe extern "win64" fn mock_00b0(exc: usize) -> usize {
        CAPTURED.0 = rd::<[u64; 3]>(exc);
        wr::<[u64; 3]>(exc, [0x1111, 0x2222, 0x3333]);
        exc
    }
    unsafe extern "win64" fn mock_acce(exc: usize, ti: usize) {
        CAPTURED.1 = rd::<[u64; 3]>(exc);
        CAPTURED.2 = ti;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4020_00b0, mock_00b0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4183_acce, mock_acce as *const ())?;

    CAPTURED = ([9; 3], [0; 3], 0);
    f_o();
    let o = CAPTURED;

    CAPTURED = ([9; 3], [0; 3], 0);
    f_n();
    let n = CAPTURED;

    crate::diff_eq!(o, n, "140200070");
    Ok(1)
}

pub unsafe fn diff_unknown_140217df0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RESOLVE_ARGS: (u64, u64) = (0, 0);

    unsafe extern "win64" fn mock_b000(mgr: u64, obj: usize) -> u64 {
        RESOLVE_ARGS = (mgr, rd::<u64>(obj));
        0xface_b00c
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_b000, mock_b000 as *const ())?;

    let g_ver = exe::ptr::<u32>(0x1_4384_1b24) as usize;
    let g_mgr = exe::ptr::<u64>(0x1_43a0_8300) as usize;
    let saved_ver = rd::<u32>(g_ver);
    let saved_mgr = rd::<u64>(g_mgr);
    wr::<u32>(g_ver, 7);
    wr::<u64>(g_mgr, 0x7777_0000);

    let run = |f: F, key: u64, ver: u32| -> (u64, [u64; 3], (u64, u64)) {
        RESOLVE_ARGS = (0, 0);
        let mut obj = [0u64; 3];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op, key);
        wr::<u64>(op + 0x08, 0x1234);
        wr::<u32>(op + 0x10, ver);
        wr::<u8>(op + 0x14, 0);
        let ret = f(op);
        (ret, obj, RESOLVE_ARGS)
    };

    for (key, ver) in [(0u64, 0u32), (0x99, 7), (0x99, 2)] {
        let o = run(f_o, key, ver);
        let n = run(f_n, key, ver);
        wr::<u32>(g_ver, saved_ver);
        wr::<u64>(g_mgr, saved_mgr);
        crate::diff_eq!(o, n, "140217df0 key={} ver={}", key, ver);
        wr::<u32>(g_ver, 7);
        wr::<u64>(g_mgr, 0x7777_0000);
    }
    wr::<u32>(g_ver, saved_ver);
    wr::<u64>(g_mgr, saved_mgr);
    Ok(3)
}

pub unsafe fn diff_unknown_140275010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let cases: [[f32; 9]; 4] = [
        [1.0, 0.2, -0.1, -0.2, 0.8, 0.3, 0.1, -0.3, 0.9],
        [0.8, 0.1, 0.2, 0.1, -0.9, 0.3, 0.2, 0.3, -0.7],
        [-0.8, 0.4, 0.1, 0.4, 0.7, -0.2, 0.1, -0.2, -0.6],
        [-0.9, 0.1, 0.3, 0.1, -0.8, 0.2, 0.3, 0.2, 0.6],
    ];

    for (idx, m) in cases.iter().enumerate() {
        let mut out_o = [0u32; 4];
        let mut out_n = [0u32; 4];
        f_o(out_o.as_mut_ptr() as usize, m.as_ptr() as usize);
        f_n(out_n.as_mut_ptr() as usize, m.as_ptr() as usize);
        crate::diff_eq!(out_o, out_n, "140275010 case={}", idx);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_14028e490(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_ARG: u64 = 0;

    unsafe extern "win64" fn mock_7910(p1: usize, req: u64) {
        GROW_ARG = req;
        wr::<u32>(p1 + 0x0c, req as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_402d_7910, mock_7910 as *const ())?;

    let run = |f: F, new_len: u64| -> ([u8; 16], u32, u32, u64) {
        GROW_ARG = 0;
        let mut buf = [0xaa_u8; 16];
        let mut hdr = [0u64; 2];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp, buf.as_mut_ptr() as usize);
        wr::<u32>(hp + 0x08, 4);
        wr::<u32>(hp + 0x0c, 8);
        f(hp, new_len);
        (buf, rd::<u32>(hp + 0x08), rd::<u32>(hp + 0x0c), GROW_ARG)
    };

    for new_len in [2u64, 4, 6, 12] {
        let o = run(f_o, new_len);
        let n = run(f_n, new_len);
        crate::diff_eq!(o, n, "14028e490 new_len={}", new_len);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_1402d7910(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DST_BUF: [u8; 32] = [0; 32];
    static mut CALL_LOG: (u64, u32) = (0, 0);

    unsafe extern "win64" fn mock_9f30(_p1: usize, cap: u64) -> usize {
        CALL_LOG.0 = cap;
        core::ptr::addr_of_mut!(DST_BUF) as usize
    }
    unsafe extern "win64" fn mock_e830(p1: usize) {
        CALL_LOG.1 += 1;
        wr::<usize>(p1, 0);
        wr::<u64>(p1 + 0x08, 0);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4020_9f30, mock_9f30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F, req: u64| -> (usize, u32, u32, [u8; 32], (u64, u32)) {
        DST_BUF = [0xff; 32];
        CALL_LOG = (0, 0);
        let src: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut hdr = [0u64; 2];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp, src.as_ptr() as usize);
        wr::<u32>(hp + 0x08, 5);
        wr::<u32>(hp + 0x0c, 10);
        f(hp, req);
        (
            rd::<usize>(hp),
            rd::<u32>(hp + 0x08),
            rd::<u32>(hp + 0x0c),
            DST_BUF,
            CALL_LOG,
        )
    };

    for req in [12u64, 25] {
        let o = run(f_o, req);
        let n = run(f_n, req);
        crate::diff_eq!(o, n, "1402d7910 req={}", req);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140460d40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLS: Vec<([u64; 6], [u64; 4], u64)> = Vec::new();

    unsafe extern "win64" fn mock_f070(rcx: usize, rdx: usize, r8: u64) {
        CALLS.push((rd::<[u64; 6]>(rcx), rd::<[u64; 4]>(rdx), r8));
    }

    let _m1 = crate::difftest::Mock::new(0x1_419d_f070, mock_f070 as *const ())?;

    let run = |f: F, cnt: u64| -> Vec<([u64; 6], [u64; 4], u64)> {
        CALLS = Vec::new();
        let p2: [u64; 8] = [0x10, 0x20, 0x30, cnt, 0x40, 0x50, 0x60, 0x70];
        let p3: [u64; 4] = [0x1111, 0x2222, 0x3333, 0x4444];
        f(0, p2.as_ptr() as usize, p3.as_ptr() as usize);
        CALLS.clone()
    };

    for cnt in [0u64, 2] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "140460d40 cnt={}", cnt);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14050b980(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_2040: Vec<([u64; 6], u64, u64)> = Vec::new();

    unsafe extern "win64" fn mock_f2b0(rcx: usize, rdx: usize) -> usize {
        let p0 = rd::<u16>(rcx) as u64;
        let p3 = rd::<u64>(rcx + 0x18);
        wr::<[u64; 4]>(rdx, [p0 + 100, p3 + 200, 300, 400]);
        rdx
    }
    unsafe extern "win64" fn mock_2040(rcx: usize, rdx: u64, r8: u64) {
        LOG_2040.push((rd::<[u64; 6]>(rcx), rdx, r8));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4050_f2b0, mock_f2b0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41a7_2040, mock_2040 as *const ())?;

    let g_0d8 = exe::ptr::<u16>(0x1_43c9_00d8) as usize;
    let g_63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let s_0d8 = rd::<u16>(g_0d8);
    let s_63c = rd::<u16>(g_63c);
    wr::<u16>(g_0d8, 1);
    wr::<u16>(g_63c, 2);

    let tbl: [u64; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    let mut world = vec![0u8; 0x58100];
    let wp = world.as_mut_ptr() as usize;
    wr::<usize>(wp + 0x580e0, tbl.as_ptr() as usize);
    wr::<usize>(wp + 0x88 + 8, 0x5000);
    wr::<u32>(wp + 0x10088 + 4, 2);

    let p1: [usize; 1] = [wp];
    for arch in [0u16, 1] {
        LOG_2040 = Vec::new();
        f_o(p1.as_ptr() as usize, arch);
        let o = LOG_2040.clone();

        LOG_2040 = Vec::new();
        f_n(p1.as_ptr() as usize, arch);
        let n = LOG_2040.clone();

        wr::<u16>(g_0d8, s_0d8);
        wr::<u16>(g_63c, s_63c);
        crate::diff_eq!(o, n, "14050b980 arch={}", arch);
        wr::<u16>(g_0d8, 1);
        wr::<u16>(g_63c, 2);
    }
    wr::<u16>(g_0d8, s_0d8);
    wr::<u16>(g_63c, s_63c);
    Ok(2)
}

pub unsafe fn diff_unknown_140993970(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;

    fn mix(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_9fa0(_p2: usize, _p1: usize, _n: u64, crc: u32, v1: u32, v2: u32) {
        mix((crc as u64) ^ ((v1 + v2) as u64));
    }
    unsafe extern "win64" fn mock_02b0(p1: usize, _p2: usize) {
        wr::<u32>(p1, 0x1111);
    }
    unsafe extern "win64" fn mock_b020(_p2: usize, dst: usize, v: u32) {
        wr::<u32>(dst, v + 0x2220);
    }
    unsafe extern "win64" fn mock_ab10(sp: usize) {
        mix(rd::<u64>(sp));
    }
    unsafe extern "win64" fn mock_9e70(_p2: usize) {
        mix(0x9e70);
    }
    unsafe extern "win64" fn v_20(_s: usize, dst: usize) {
        mix(rd::<u64>(dst));
    }
    unsafe extern "win64" fn v_38(_s: usize, dst: usize) {
        wr::<u32>(dst, 0x3838_0000);
    }
    unsafe extern "win64" fn v_40(_s: usize, dst: usize) {
        wr::<u32>(dst, 0x4040_0000);
    }
    unsafe extern "win64" fn v_48(_s: usize, dst: usize) {
        wr::<u32>(dst, 0x4848_0000);
    }
    unsafe extern "win64" fn v_80(_s: usize, dst: usize) {
        wr::<u8>(dst, 1);
    }
    unsafe extern "win64" fn v_skip(_inner: usize, n: u32, z: u32) {
        mix(((n as u64) << 8) | (z as u64));
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9fa0, mock_9fa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_406e_02b0, mock_02b0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4088_b020, mock_b020 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_401e_9e70, mock_9e70 as *const ())?;

    let mut vtbl_s = [0usize; 18];
    vtbl_s[4] = v_20 as *const () as usize;
    vtbl_s[7] = v_38 as *const () as usize;
    vtbl_s[8] = v_40 as *const () as usize;
    vtbl_s[9] = v_48 as *const () as usize;
    vtbl_s[16] = v_80 as *const () as usize;
    let s_obj: [usize; 1] = [vtbl_s.as_ptr() as usize];

    let mut vtbl_in = [0usize; 8];
    vtbl_in[7] = v_skip as *const () as usize;
    let in_obj: [usize; 1] = [vtbl_in.as_ptr() as usize];
    let p320_obj: [usize; 2] = [0, in_obj.as_ptr() as usize];

    let run = |f: F, ver: i32| -> (u64, [u8; 0x38], u64) {
        TRACE = 0;
        let mut p1 = [0xeeu8; 0x38];
        let mut p2 = [0u8; 0x380];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u32>(p2p + 0x318, 5);
        wr::<usize>(p2p + 0x320, p320_obj.as_ptr() as usize);
        wr::<i32>(p2p + 0x328, ver);
        wr::<usize>(p2p + 0x338, s_obj.as_ptr() as usize);
        wr::<u8>(p2p + 0x369, 1);

        let ret = f(p1.as_mut_ptr() as usize, p2p, 0);
        (ret, p1, TRACE)
    };

    for ver in [1i32, 5, 15] {
        let o = run(f_o, ver);
        let n = run(f_n, ver);
        crate::diff_eq!(o, n, "140993970 ver={}", ver);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140a02480(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CAP: u64 = 0;

    unsafe extern "win64" fn mock_b7c0(p1: usize, cap: u64) {
        GROW_CAP = cap;
        wr::<u32>(p1 + 0x0c, cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40a5_b7c0, mock_b7c0 as *const ())?;

    let run = |f: F, new_len: u64| -> ([u32; 48], u32, u32, u64) {
        GROW_CAP = 0;
        let mut buf = [0xdead_beefu32; 48]; // 4 elements of 0x30 bytes
        let mut hdr = [0u64; 2];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp, buf.as_mut_ptr() as usize);
        wr::<u32>(hp + 0x08, 1);
        wr::<u32>(hp + 0x0c, 2);
        f(hp, new_len);
        (buf, rd::<u32>(hp + 0x08), rd::<u32>(hp + 0x0c), GROW_CAP)
    };

    for new_len in [0u64, 1, 2, 4] {
        let o = run(f_o, new_len);
        let n = run(f_n, new_len);
        crate::diff_eq!(o, n, "140a02480 new_len={}", new_len);
    }
    Ok(4)
}

pub unsafe fn diff_content_140e4e780(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: u64 = 0;

    fn mix(v: u64) {
        unsafe {
            TRACE = TRACE.wrapping_mul(0x100_0000_01b3).wrapping_add(v);
        }
    }

    unsafe extern "win64" fn mock_9fa0(_p2: usize, _p1: usize, _n: u64, crc: u32, v1: u32, v2: u32) {
        mix((crc as u64) ^ ((v1 + v2) as u64));
    }
    unsafe extern "win64" fn mock_9b50(dst: usize, _sv: usize, a3: u64, a4: u64) {
        wr::<u32>(dst, 0x9b50 + (a3 + a4) as u32);
    }
    unsafe extern "win64" fn mock_8620(dst: usize) {
        wr::<u32>(dst, 0x8620);
    }
    unsafe extern "win64" fn mock_84a0(dst: usize, src: usize) {
        wr::<u64>(dst, rd::<u64>(src) ^ 0x84a0);
    }
    unsafe extern "win64" fn mock_7390(_p2: usize, dst: usize, ver: u32) {
        wr::<u32>(dst, ver + 0x7390);
    }
    unsafe extern "win64" fn mock_9e70(_p2: usize) {
        mix(0x9e70);
    }
    unsafe extern "win64" fn v_00(_s: usize, dst: usize) {
        wr::<u32>(dst + 4, 0x1000);
    }
    unsafe extern "win64" fn v_20(_s: usize, dst: usize) {
        wr::<u32>(dst, 0x2000);
    }
    unsafe extern "win64" fn v_38(_s: usize, dst: usize) {
        wr::<u32>(dst, 0x3800);
    }
    unsafe extern "win64" fn v_80(_s: usize, dst: usize) {
        wr::<u8>(dst, 1);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9fa0, mock_9fa0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_9b50, mock_9b50 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_8620, mock_8620 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4055_7390, mock_7390 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_401e_9e70, mock_9e70 as *const ())?;

    let mut vtbl = [0usize; 18];
    vtbl[0] = v_00 as *const () as usize;
    vtbl[4] = v_20 as *const () as usize;
    vtbl[7] = v_38 as *const () as usize;
    vtbl[16] = v_80 as *const () as usize;
    let s_obj: [usize; 1] = [vtbl.as_ptr() as usize];

    let run = |f: F, ver: i32| -> (u64, [u8; 0xa0], u64) {
        TRACE = 0;
        let mut p1 = [0xccu8; 0xa0];
        let mut p2 = [0u8; 0x380];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x328, ver);
        wr::<usize>(p2p + 0x338, s_obj.as_ptr() as usize);
        wr::<u8>(p2p + 0x369, 1);
        let ret = f(p1.as_mut_ptr() as usize, p2p);
        (ret, p1, TRACE)
    };

    for ver in [5i32, 15, 18] {
        let o = run(f_o, ver);
        let n = run(f_n, ver);
        crate::diff_eq!(o, n, "140e4e780 ver={}", ver);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140f82270(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: ([u64; 3], [u64; 3], [u64; 3], u64, u64) = ([0; 3], [0; 3], [0; 3], 0, 0);

    unsafe extern "win64" fn mock_05a0(rcx: usize, rdx: usize, r8: usize, r9: u64, s5: u64) {
        CAPTURED = (
            rd::<[u64; 3]>(rcx),
            rd::<[u64; 3]>(rdx),
            rd::<[u64; 3]>(r8),
            r9,
            s5,
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_41e2_05a0, mock_05a0 as *const ())?;

    let g_f14 = exe::ptr::<u16>(0x1_43cc_7f14) as usize;
    let g_f10 = exe::ptr::<u16>(0x1_43cc_7f10) as usize;
    let g_f0c = exe::ptr::<u16>(0x1_43cc_7f0c) as usize;
    let g_c80 = exe::ptr::<u16>(0x1_43cc_7c80) as usize;
    let g_c84 = exe::ptr::<u16>(0x1_43cc_7c84) as usize;
    let saved = (
        rd::<u16>(g_f14),
        rd::<u16>(g_f10),
        rd::<u16>(g_f0c),
        rd::<u16>(g_c80),
        rd::<u16>(g_c84),
    );
    wr::<u16>(g_f14, 0);
    wr::<u16>(g_f10, 1);
    wr::<u16>(g_f0c, 2);
    wr::<u16>(g_c80, 0);
    wr::<u16>(g_c84, 1);

    let map_20: [u16; 4] = [2, 0, 1, 0];
    let entries: [u64; 6] = [0x1000, 10, 0x2000, 20, 0x3000, 30];
    let tbl: [u64; 2] = [0xaaaa_1111, 0xbbbb_2222];

    let mut hdr_0 = [0u64; 5];
    wr::<usize>((hdr_0.as_mut_ptr() as usize) + 0x20, map_20.as_ptr() as usize);

    let mut world = vec![0u8; 0x58100];
    let wp = world.as_mut_ptr() as usize;
    wr::<usize>(wp, hdr_0.as_ptr() as usize);
    wr::<usize>(wp + 0x08, entries.as_ptr() as usize);
    wr::<usize>(wp + 0x580e0, tbl.as_ptr() as usize);

    let p1: [usize; 1] = [wp];
    CAPTURED = ([0; 3], [0; 3], [0; 3], 0, 0);
    f_o(p1.as_ptr() as usize);
    let o = CAPTURED;

    CAPTURED = ([0; 3], [0; 3], [0; 3], 0, 0);
    f_n(p1.as_ptr() as usize);
    let n = CAPTURED;

    wr::<u16>(g_f14, saved.0);
    wr::<u16>(g_f10, saved.1);
    wr::<u16>(g_f0c, saved.2);
    wr::<u16>(g_c80, saved.3);
    wr::<u16>(g_c84, saved.4);

    crate::diff_eq!(o, n, "140f82270");
    Ok(1)
}

pub unsafe fn diff_unknown_140f939e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: ([u64; 3], [u64; 3], u64, u64) = ([0; 3], [0; 3], 0, 0);

    unsafe extern "win64" fn mock_30a0(rcx: usize, rdx: usize, r8: u64, r9: u64) {
        CAPTURED = (rd::<[u64; 3]>(rcx), rd::<[u64; 3]>(rdx), r8, r9);
    }

    let _m1 = crate::difftest::Mock::new(0x1_41e3_30a0, mock_30a0 as *const ())?;

    let g_3bc = exe::ptr::<u16>(0x1_43cc_93bc) as usize;
    let g_3b8 = exe::ptr::<u16>(0x1_43cc_93b8) as usize;
    let g_228 = exe::ptr::<u16>(0x1_43cc_9228) as usize;
    let saved = (rd::<u16>(g_3bc), rd::<u16>(g_3b8), rd::<u16>(g_228));
    wr::<u16>(g_3bc, 0);
    wr::<u16>(g_3b8, 1);
    wr::<u16>(g_228, 0);

    let map_20: [u16; 2] = [1, 0];
    let entries: [u64; 4] = [0x4000, 40, 0x5000, 50];
    let tbl: [u64; 1] = [0xcccc_3333];

    let mut hdr_0 = [0u64; 5];
    wr::<usize>((hdr_0.as_mut_ptr() as usize) + 0x20, map_20.as_ptr() as usize);

    let mut world = vec![0u8; 0x58100];
    let wp = world.as_mut_ptr() as usize;
    wr::<usize>(wp, hdr_0.as_ptr() as usize);
    wr::<usize>(wp + 0x08, entries.as_ptr() as usize);
    wr::<usize>(wp + 0x580e0, tbl.as_ptr() as usize);

    let p1: [usize; 1] = [wp];
    CAPTURED = ([0; 3], [0; 3], 0, 0);
    f_o(p1.as_ptr() as usize);
    let o = CAPTURED;

    CAPTURED = ([0; 3], [0; 3], 0, 0);
    f_n(p1.as_ptr() as usize);
    let n = CAPTURED;

    wr::<u16>(g_3bc, saved.0);
    wr::<u16>(g_3b8, saved.1);
    wr::<u16>(g_228, saved.2);

    crate::diff_eq!(o, n, "140f939e0");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_401d_eb40, "unknown_1401deb40", rend::cauldron_hot_z1a::unknown_1401deb40 as *const (), rend::cauldron_hot_z1a::diff_unknown_1401deb40),
// Replacement::verified_throws(0x1_401e_1660, "unknown_1401e1660", rend::cauldron_hot_z1a::unknown_1401e1660 as *const (), Throws::Discard, rend::cauldron_hot_z1a::diff_unknown_1401e1660),
// Replacement::verified_throws(0x1_401e_1c30, "unknown_1401e1c30", rend::cauldron_hot_z1a::unknown_1401e1c30 as *const (), Throws::Discard, rend::cauldron_hot_z1a::diff_unknown_1401e1c30),
// Replacement::verified_throws(0x1_401e_2020, "sparsehash_1401e2020", rend::cauldron_hot_z1a::sparsehash_1401e2020 as *const (), Throws::Discard, rend::cauldron_hot_z1a::diff_sparsehash_1401e2020),
// Replacement::verified_throws(0x1_401f_9050, "unknown_1401f9050", rend::cauldron_hot_z1a::unknown_1401f9050 as *const (), Throws::Discard, rend::cauldron_hot_z1a::diff_unknown_1401f9050),
// Replacement::verified(0x1_401f_a730, "sparsehash_1401fa730", rend::cauldron_hot_z1a::sparsehash_1401fa730 as *const (), rend::cauldron_hot_z1a::diff_sparsehash_1401fa730),
// Replacement::verified_throws(0x1_4020_0070, "std_140200070", rend::cauldron_hot_z1a::std_140200070 as *const (), Throws::Discard, rend::cauldron_hot_z1a::diff_std_140200070),
// Replacement::verified(0x1_4021_7df0, "unknown_140217df0", rend::cauldron_hot_z1a::unknown_140217df0 as *const (), rend::cauldron_hot_z1a::diff_unknown_140217df0),
// Replacement::verified(0x1_4027_5010, "unknown_140275010", rend::cauldron_hot_z1a::unknown_140275010 as *const (), rend::cauldron_hot_z1a::diff_unknown_140275010),
// Replacement::verified(0x1_4028_e490, "unknown_14028e490", rend::cauldron_hot_z1a::unknown_14028e490 as *const (), rend::cauldron_hot_z1a::diff_unknown_14028e490),
// Replacement::verified(0x1_402d_7910, "unknown_1402d7910", rend::cauldron_hot_z1a::unknown_1402d7910 as *const (), rend::cauldron_hot_z1a::diff_unknown_1402d7910),
// Replacement::verified(0x1_4046_0d40, "unknown_140460d40", rend::cauldron_hot_z1a::unknown_140460d40 as *const (), rend::cauldron_hot_z1a::diff_unknown_140460d40),
// Replacement::verified(0x1_4050_b980, "unknown_14050b980", rend::cauldron_hot_z1a::unknown_14050b980 as *const (), rend::cauldron_hot_z1a::diff_unknown_14050b980),
// Replacement::verified_throws(0x1_4099_3970, "unknown_140993970", rend::cauldron_hot_z1a::unknown_140993970 as *const (), Throws::Caught, rend::cauldron_hot_z1a::diff_unknown_140993970),
// Replacement::verified(0x1_40a0_2480, "unknown_140a02480", rend::cauldron_hot_z1a::unknown_140a02480 as *const (), rend::cauldron_hot_z1a::diff_unknown_140a02480),
// Replacement::verified_throws(0x1_40e4_e780, "content_140e4e780", rend::cauldron_hot_z1a::content_140e4e780 as *const (), Throws::Discard, rend::cauldron_hot_z1a::diff_content_140e4e780),
// Replacement::verified(0x1_40f8_2270, "unknown_140f82270", rend::cauldron_hot_z1a::unknown_140f82270 as *const (), rend::cauldron_hot_z1a::diff_unknown_140f82270),
// Replacement::verified_throws(0x1_40f9_39e0, "unknown_140f939e0", rend::cauldron_hot_z1a::unknown_140f939e0 as *const (), Throws::Discard, rend::cauldron_hot_z1a::diff_unknown_140f939e0),
