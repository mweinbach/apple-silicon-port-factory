//! Decompiled `rend` hot functions shard Z2B (`0x1412ebe10..0x141960ba0`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline]
fn u64_to_f32(v: u64) -> f32 {
    if (v as i64) >= 0 {
        (v as i64) as f32
    } else {
        let half = ((v >> 1) | (v & 1)) as i64 as f32;
        half + half
    }
}

#[inline]
fn f32_to_u64_msvc(mut x: f32) -> u64 {
    const TWO_POW_63: f32 = f32::from_bits(0x5f00_0000);
    let mut offset: u64 = 0;
    if x >= TWO_POW_63 {
        x -= TWO_POW_63;
        if x < TWO_POW_63 {
            offset = 0x8000_0000_0000_0000;
        }
    }
    let cvt: i64;
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "vcvttss2si {}, {}",
            out(reg) cvt,
            in(xmm_reg) x,
            options(pure, nomem, nostack),
        );
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        cvt = x as i64;
    }
    (cvt as u64).wrapping_add(offset)
}

#[inline]
fn f32_min_sse(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

#[inline]
fn f32_max_sse(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

#[inline]
unsafe fn atomic_cas_i32(addr: usize, old: i32, new: i32) -> bool {
    if (addr & 3) == 0 {
        use core::sync::atomic::{AtomicI32, Ordering};
        (&*(addr as *const AtomicI32))
            .compare_exchange(old, new, Ordering::SeqCst, Ordering::SeqCst)
            .is_ok()
    } else if rd::<i32>(addr) == old {
        wr::<i32>(addr, new);
        true
    } else {
        false
    }
}

#[inline]
unsafe fn atomic_add_i32(addr: usize, delta: i32) -> i32 {
    if (addr & 3) == 0 {
        use core::sync::atomic::{AtomicI32, Ordering};
        (&*(addr as *const AtomicI32)).fetch_add(delta, Ordering::SeqCst)
    } else {
        let prev = rd::<i32>(addr);
        wr::<i32>(addr, prev.wrapping_add(delta));
        prev
    }
}

/// `rend_1412ebe10`
/// Original: 0x1412ebe10 (104 bytes, Throws::No)
pub unsafe extern "win64" fn rend_1412ebe10(param_1: usize) {
    let len_bytes = (rd::<u32>(param_1 + 0x08) as u64) << 2;
    let cap_bytes = (rd::<u32>(param_1 + 0x0c) as u64) << 2;
    if len_bytes != cap_bytes {
        let new_cap = ((len_bytes as i64) >> 2) as u64;
        let dst = exe::call_va!(0x1_401d_ecf0, (param_1, new_cap) -> usize);
        let copy_bytes = (rd::<u32>(param_1 + 0x08) as u64) << 2;
        let src = rd::<usize>(param_1);
        let _ = exe::call_va!(0x1_4183_acc8, (dst, src, copy_bytes) -> usize);
        exe::call_va!(0x1_401d_e830, (param_1));
        wr::<usize>(param_1, dst);
        wr::<u32>(param_1 + 0x08, new_cap as u32);
        wr::<u32>(param_1 + 0x0c, new_cap as u32);
    }
}

/// `sparsehash_1413a1800`
/// Original: 0x1413a1800 (217 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1413a1800(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mask = rd::<u64>(param_1 + 0x40).wrapping_sub(1);
    let empty_key = rd::<u64>(param_1 + 0x48);
    let key = rd::<u64>(param_3);
    let table = rd::<usize>(param_1 + 0x60);
    let mut num_probes: u64 = 0;
    let mut bucknum = key & mask;
    let mut insert_pos = u64::MAX;
    let mut off = (bucknum as usize).wrapping_mul(0x18);
    if empty_key != rd::<u64>(table.wrapping_add(off)) {
        let num_deleted = rd::<u64>(param_1 + 0x30);
        loop {
            let slot_key = rd::<u64>(table.wrapping_add(off));
            if num_deleted != 0 && rd::<u64>(param_1 + 0x28) == slot_key {
                if insert_pos == u64::MAX {
                    insert_pos = bucknum;
                }
            } else if key == slot_key {
                wr::<u64>(param_2, bucknum);
                wr::<u64>(param_2 + 0x08, u64::MAX);
                return param_2;
            }
            num_probes = num_probes.wrapping_add(1);
            bucknum = bucknum.wrapping_add(num_probes) & mask;
            off = (bucknum as usize).wrapping_mul(0x18);
            if empty_key == rd::<u64>(table.wrapping_add(off)) {
                break;
            }
        }
    }
    wr::<u64>(param_2, u64::MAX);
    wr::<u64>(
        param_2 + 0x08,
        if insert_pos == u64::MAX {
            bucknum
        } else {
            insert_pos
        },
    );
    param_2
}

/// `sparsehash_1413a18e0`
/// Original: 0x1413a18e0 (371 bytes, Throws::No)
pub unsafe extern "win64" fn sparsehash_1413a18e0(param_1: usize, param_2: usize) {
    let a = rd::<[u64; 13]>(param_1);
    let b = rd::<[u64; 13]>(param_2);
    wr::<[u64; 13]>(param_1, b);
    wr::<[u64; 13]>(param_2, a);

    let f_bc = u64_to_f32(rd::<u64>(param_1 + 0x40));

    let p1_0 = f32_to_u64_msvc(f_bc * rd::<f32>(param_1 + 0x10));
    let p1_8 = f32_to_u64_msvc(f_bc * rd::<f32>(param_1 + 0x14));
    wr::<u64>(param_1, p1_0);
    wr::<u8>(param_1 + 0x18, 0);
    wr::<u64>(param_1 + 0x08, p1_8);

    let p2_0 = f32_to_u64_msvc(f_bc * rd::<f32>(param_2 + 0x10));
    let p2_8 = f32_to_u64_msvc(f_bc * rd::<f32>(param_2 + 0x14));
    wr::<u64>(param_2, p2_0);
    wr::<u8>(param_2 + 0x18, 0);
    wr::<u64>(param_2 + 0x08, p2_8);
}

/// `sparsehash_1413a1c10`
/// Original: 0x1413a1c10 (606 bytes, Throws::Discard)
pub unsafe extern "win64" fn sparsehash_1413a1c10(param_1: usize, param_2: usize) -> usize {
    let mut pos_pair = [0u64; 2];
    let _ = exe::call_va!(
        0x1_413a_1800,
        (param_1, pos_pair.as_mut_ptr() as usize, param_2) -> usize
    );
    if pos_pair[0] != u64::MAX {
        return rd::<usize>(param_1 + 0x60)
            .wrapping_add((pos_pair[0] as usize).wrapping_mul(0x18));
    }

    let resized = exe::call_va!(0x1_413a_1f00, (param_1) -> u8);
    let key = rd::<u64>(param_2);
    let ins_idx = if resized == 0 {
        pos_pair[1]
    } else {
        let table = rd::<usize>(param_1 + 0x60);
        let mask = rd::<u64>(param_1 + 0x40).wrapping_sub(1);
        let empty_key = rd::<u64>(param_1 + 0x48);
        let mut num_probes: u64 = 0;
        let mut bucknum = key & mask;
        let mut insert_pos = u64::MAX;
        let mut off = (bucknum as usize).wrapping_mul(0x18);
        if empty_key != rd::<u64>(table.wrapping_add(off)) {
            loop {
                let num_deleted = rd::<u64>(param_1 + 0x30);
                let slot_key = rd::<u64>(table.wrapping_add(off));
                if num_deleted != 0 && rd::<u64>(param_1 + 0x28) == slot_key {
                    if insert_pos == u64::MAX {
                        insert_pos = bucknum;
                    }
                } else if key == slot_key {
                    insert_pos = u64::MAX;
                    if bucknum != u64::MAX {
                        return table.wrapping_add((bucknum as usize).wrapping_mul(0x18));
                    }
                    break;
                }
                num_probes = num_probes.wrapping_add(1);
                bucknum = bucknum.wrapping_add(num_probes) & mask;
                off = (bucknum as usize).wrapping_mul(0x18);
                if empty_key == rd::<u64>(table.wrapping_add(off)) {
                    if insert_pos == u64::MAX {
                        insert_pos = bucknum;
                    }
                    break;
                }
            }
            insert_pos
        } else {
            bucknum
        }
    };

    let num_elements = rd::<u64>(param_1 + 0x38);
    let num_deleted = rd::<u64>(param_1 + 0x30);
    if num_elements.wrapping_sub(num_deleted) >= 0x0aaa_aaaa_aaaa_aaaa {
        let mut exc = [0u8; 32];
        let msg = exe::ptr::<u8>(0x1_4307_aa70) as usize;
        exe::call_va!(0x1_401d_f630, (exc.as_mut_ptr() as usize, msg));
        exe::call_va!(
            0x1_4183_acce,
            (
                exc.as_mut_ptr() as usize,
                exe::ptr::<u8>(0x1_437f_cf88) as usize
            )
        );
        unreachable!()
    }

    let off = (ins_idx as usize).wrapping_mul(0x18);
    let table0 = rd::<usize>(param_1 + 0x60);
    if num_deleted != 0 && rd::<u64>(param_1 + 0x28) == rd::<u64>(table0.wrapping_add(off)) {
        wr::<u64>(param_1 + 0x30, num_deleted.wrapping_sub(1));
    } else {
        wr::<u64>(param_1 + 0x38, num_elements.wrapping_add(1));
    }

    let table1 = rd::<usize>(param_1 + 0x60);
    let dst = table1.wrapping_add(off);
    wr::<u64>(dst, key);
    wr::<u64>(dst + 0x08, 0);
    wr::<u64>(dst + 0x10, 0);
    rd::<usize>(param_1 + 0x60).wrapping_add(off)
}

/// `sparsehash_1413a1f00`
/// Original: 0x1413a1f00 (563 bytes, Throws::Discard)
pub unsafe extern "win64" fn sparsehash_1413a1f00(param_1: usize) -> u8 {
    let mut resized: u8 = 0;
    if rd::<u8>(param_1 + 0x18) != 0 {
        let active = rd::<u64>(param_1 + 0x38).wrapping_sub(rd::<u64>(param_1 + 0x30));
        let shrink_thresh = rd::<u64>(param_1 + 0x08);
        let bucket_count = rd::<u64>(param_1 + 0x40);
        if shrink_thresh == 0 || active >= shrink_thresh || bucket_count <= 0x20 {
            wr::<u8>(param_1 + 0x18, 0);
        } else {
            let shrink_ratio = rd::<f32>(param_1 + 0x14);
            let mut r8 = bucket_count >> 1;
            if r8 > 0x20 {
                let f_active = u64_to_f32(active);
                loop {
                    if u64_to_f32(r8) * shrink_ratio <= f_active {
                        break;
                    }
                    r8 >>= 1;
                    if r8 <= 0x20 {
                        break;
                    }
                }
            }
            let mut tmp = [0u64; 14];
            exe::call_va!(0x1_401e_1660, (tmp.as_mut_ptr() as usize, param_1, r8));
            exe::call_va!(0x1_413a_18e0, (param_1, tmp.as_mut_ptr() as usize));
            exe::call_va!(0x1_401e_15d0, (tmp.as_mut_ptr() as usize));
            wr::<u8>(param_1 + 0x18, 0);
            resized = 1;
        }
    }

    let num_elements = rd::<u64>(param_1 + 0x38);
    if num_elements >= 0xffff_ffff_ffff_fffe {
        let mut exc = [0u8; 32];
        let msg = exe::ptr::<u8>(0x1_4307_90d8) as usize;
        exe::call_va!(0x1_401d_f630, (exc.as_mut_ptr() as usize, msg));
        exe::call_va!(
            0x1_4183_acce,
            (
                exc.as_mut_ptr() as usize,
                exe::ptr::<u8>(0x1_437f_cf88) as usize
            )
        );
        unreachable!()
    }

    let needed = num_elements.wrapping_add(1);
    if rd::<u64>(param_1 + 0x40) < 4 || needed > rd::<u64>(param_1) {
        let rsi = exe::call_va!(0x1_401e_2020, (param_1, needed, 0u64) -> u64);
        let cur_bc = rd::<u64>(param_1 + 0x40);
        if rsi > cur_bc {
            let active_plus_1 = rd::<u64>(param_1 + 0x38)
                .wrapping_sub(rd::<u64>(param_1 + 0x30))
                .wrapping_add(1);
            let mut r8 = exe::call_va!(0x1_401e_2020, (param_1, active_plus_1, cur_bc) -> u64);
            if r8 < rsi && r8 < 0x7fff_ffff_ffff_ffff {
                let doubled = r8.wrapping_mul(2);
                let thresh = f32_to_u64_msvc(u64_to_f32(doubled) * rd::<f32>(param_1 + 0x14));
                let active_re = rd::<u64>(param_1 + 0x38)
                    .wrapping_sub(rd::<u64>(param_1 + 0x30))
                    .wrapping_add(1);
                if active_re >= thresh {
                    r8 = doubled;
                }
            }
            let mut tmp = [0u64; 14];
            exe::call_va!(0x1_401e_1660, (tmp.as_mut_ptr() as usize, param_1, r8));
            exe::call_va!(0x1_413a_18e0, (param_1, tmp.as_mut_ptr() as usize));
            exe::call_va!(0x1_401e_15d0, (tmp.as_mut_ptr() as usize));
            resized = 1;
        }
    }
    resized
}

/// `r_1413aaa50`
/// Original: 0x1413aaa50 (683 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_1413aaa50(
    param_1: usize,
    param_2: usize,
    _param_3: u64,
    _param_4: u64,
    param_5: usize,
) -> usize {
    let kind = rd::<u32>(param_5 + 0x50);
    if kind == 2 {
        let mut str_ptr = param_5.wrapping_add(0x08);
        let b3 = rd::<u8>(param_5 + 0x03) as u32;
        let cap_bits = if b3 < 0x80 {
            b3 * 8
        } else {
            (b3 << 8) - 0x7c00
        };
        let len_bits = (rd::<u32>(param_5) & 0x00ff_ffff) << 3;
        if len_bits > cap_bits {
            str_ptr = rd::<usize>(str_ptr);
        }
        let str_len = rd::<u32>(param_5 + 0x04) as u64;
        let mut arg_r8: [u64; 2] = [str_ptr as u64, str_len];
        let mut arg_rdx: [u64; 2] = [rd::<u64>(param_5 + 0x28), rd::<u64>(param_5 + 0x20)];

        let mut ctx = [0u64; 66];
        let ctx_ptr = ctx.as_mut_ptr() as usize;
        exe::call_va!(
            0x1_4248_1b20,
            (
                ctx_ptr,
                arg_rdx.as_mut_ptr() as usize,
                arg_r8.as_mut_ptr() as usize
            )
        );
        if rd::<u8>(ctx_ptr + 0x200) == 0 {
            wr::<u64>(param_2, 0);
            wr::<u64>(param_2 + 0x08, 0);
            exe::call_va!(0x1_4240_ab10, (ctx_ptr));
            return param_2;
        }

        wr::<u64>(param_2, 0);
        wr::<u64>(param_2 + 0x08, 0);
        let init_cap = rd::<u32>(param_1 + 0x08) as u64;
        if init_cap != 0 {
            exe::call_va!(0x1_413a_cb60, (param_2, init_cap));
        }

        let mut cur = rd::<usize>(param_1);
        let end = cur.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(8));
        while cur != end {
            let elem = rd::<u64>(cur);
            let mut item_buf = [0u64; 3];
            let ok = exe::call_va!(
                0x1_4248_1560,
                (ctx_ptr + 0x1d8, elem, item_buf.as_mut_ptr() as usize) -> u8
            );
            let obj = if ok != 0 {
                exe::call_va!(0x1_4248_17c0, (item_buf.as_mut_ptr() as usize, elem) -> u64)
            } else {
                0
            };
            if obj != 0 {
                let len = rd::<u32>(param_2 + 0x08) as u64;
                let cap = rd::<u32>(param_2 + 0x0c) as u64;
                if (len << 3) == (cap << 3) {
                    exe::call_va!(0x1_413a_cb60, (param_2, len.wrapping_add(1)));
                }
                let idx = rd::<u32>(param_2 + 0x08) as usize;
                let data = rd::<usize>(param_2);
                wr::<u64>(data.wrapping_add(idx.wrapping_mul(8)), obj);
                wr::<u32>(param_2 + 0x08, rd::<u32>(param_2 + 0x08).wrapping_add(1));
            }
            cur = cur.wrapping_add(8);
        }

        if rd::<u8>(ctx_ptr + 0x200) != 0 {
            exe::call_va!(0x1_4240_ab10, (ctx_ptr + 0x150));
            let p130 = rd::<usize>(ctx_ptr + 0x130);
            if p130 != 0 {
                exe::call_va!(0x1_4244_2de0, (p130));
            }
            exe::call_va!(0x1_4240_ab10, (ctx_ptr + 0x020));
            exe::call_va!(0x1_413c_6280, (ctx_ptr + 0x010));
        } else {
            exe::call_va!(0x1_4240_ab10, (ctx_ptr));
        }
        return param_2;
    }

    if kind == 3 {
        let r9 = rd::<u64>(param_5 + 0x28);
        let r8 = rd::<usize>(param_5 + 0x20);
        if rd::<u32>(r8) == 0x504b_4d44 {
            let cnt = rd::<u32>(r8 + 0x04) as u64;
            let hdr_size = cnt.wrapping_mul(8).wrapping_add(8);
            let rem_size = r9.wrapping_sub(hdr_size);
            let data_ptr = (r8 as u64).wrapping_add(hdr_size);
            let mut desc: [u64; 5] = [cnt, (r8 as u64).wrapping_add(8), rem_size, data_ptr, 1];
            exe::call_va!(
                0x1_413a_bbb0,
                (param_1, param_2, r8, rem_size, desc.as_mut_ptr() as usize)
            );
            return param_2;
        }
    }

    wr::<u64>(param_2, 0);
    wr::<u64>(param_2 + 0x08, 0);
    param_2
}

/// `r_1413ab790`
/// Original: 0x1413ab790 (518 bytes, Throws::Caught)
pub unsafe extern "win64" fn r_1413ab790(param_1: usize) {
    let mut idx: u64 = 0;
    let mut offset: usize = 0;
    let obj0 = rd::<usize>(param_1);
    if rd::<u32>(obj0 + 0x30) == 0 {
        return;
    }

    loop {
        let obj = rd::<usize>(param_1);
        if rd::<u8>(obj + 0x68) != 0 {
            return;
        }
        let ptr_10 = rd::<usize>(obj + 0x10);
        let arr_28 = rd::<usize>(obj + 0x28);
        let ticks_div = rd::<u64>(arr_28 + offset + 0x28) / 10_000_000;
        let base_time = rd::<u64>(ptr_10);
        let min_time = base_time.wrapping_add(1);
        let clamped_lo = if ticks_div < min_time {
            min_time as u32
        } else {
            ticks_div as u32
        };
        let r12d = clamped_lo.wrapping_sub(base_time as u32);

        let mut vec = [0u64; 2];
        let _ = exe::call_va!(
            0x1_413a_aa50,
            (
                rd::<usize>(obj + 0x08),
                vec.as_mut_ptr() as usize,
                rd::<u64>(arr_28 + offset),
                obj,
                arr_28 + offset + 0x10
            ) -> usize
        );

        let mut len = (vec[1] as u32) as usize;
        let mut data = vec[0] as usize;
        if data != data.wrapping_add(len.wrapping_mul(8)) {
            let dst_vec = rd::<usize>(rd::<usize>(param_1) + 0x28).wrapping_add(offset + 0x70);
            let count = len as u64;
            let cap = rd::<u32>(dst_vec + 0x0c) as u64;
            if count > cap {
                let grow = (cap >> 1).wrapping_add(cap);
                let new_cap = if grow < count { count } else { grow };
                exe::call_va!(0x1_401e_3cd0, (dst_vec, new_cap));
                len = (vec[1] as u32) as usize;
                data = vec[0] as usize;
            }

            let end = data.wrapping_add(len.wrapping_mul(8));
            let mut cur = data;
            while cur != end {
                let entry_base = rd::<usize>(rd::<usize>(param_1) + 0x28).wrapping_add(offset);
                let cur_len = rd::<u32>(entry_base + 0x78) as u64;
                let cur_cap = rd::<u32>(entry_base + 0x7c) as u64;
                if (cur_len << 3) == (cur_cap << 3) {
                    let need = cur_len.wrapping_add(1);
                    let grow = (cur_cap >> 1).wrapping_add(cur_cap);
                    let new_cap = if grow < need { need } else { grow };
                    exe::call_va!(0x1_401e_3cd0, (entry_base + 0x70, new_cap));
                }
                let out_idx = rd::<u32>(entry_base + 0x78) as usize;
                let out_buf = rd::<usize>(entry_base + 0x70);
                let val = rd::<u64>(cur);
                wr::<u64>(cur, 0);
                wr::<u64>(out_buf.wrapping_add(out_idx.wrapping_mul(8)), val);
                wr::<u32>(
                    entry_base + 0x78,
                    rd::<u32>(entry_base + 0x78).wrapping_add(1),
                );
                cur = cur.wrapping_add(8);
            }

            let arr_a = rd::<usize>(rd::<usize>(param_1) + 0x28);
            wr::<u32>(arr_a + offset + 0x08, r12d);
            let arr_b = rd::<usize>(rd::<usize>(param_1) + 0x28);
            wr::<u8>(arr_b + offset + 0x80, 1);
            len = (vec[1] as u32) as usize;
            data = vec[0] as usize;
        }

        let end = data.wrapping_add(len.wrapping_mul(8));
        let mut cur = data;
        while cur != end {
            let item = rd::<usize>(cur);
            if item != 0 {
                let vtbl = rd::<usize>(item);
                exe::call_va!(rd::<usize>(vtbl), (item, 0u32));
                exe::call_va!(0x1_4244_2de0, (item));
            }
            cur = cur.wrapping_add(8);
        }

        let data_final = vec[0] as usize;
        if data_final != 0 {
            exe::call_va!(0x1_4241_c190, (data_final));
        }

        idx = idx.wrapping_add(1);
        offset = offset.wrapping_add(0x88);
        let obj_next = rd::<usize>(param_1);
        if idx >= rd::<u32>(obj_next + 0x30) as u64 {
            break;
        }
    }
}

/// `sparsehash_1413cfa60`
/// Original: 0x1413cfa60 (382 bytes, Throws::Discard)
pub unsafe extern "win64" fn sparsehash_1413cfa60(param_1: usize, param_2: u64) -> u8 {
    let mut resized: u8 = 0;
    if rd::<u8>(param_1 + 0x18) != 0 {
        let c = exe::call_va!(0x1_413c_fbe0, (param_1) -> u8);
        if c != 0 {
            resized = 1;
        }
    }

    let num_elements = rd::<u64>(param_1 + 0x38);
    if num_elements >= !param_2 {
        let mut exc = [0u8; 32];
        let msg = exe::ptr::<u8>(0x1_4307_90d8) as usize;
        exe::call_va!(0x1_401d_f630, (exc.as_mut_ptr() as usize, msg));
        exe::call_va!(
            0x1_4183_acce,
            (
                exc.as_mut_ptr() as usize,
                exe::ptr::<u8>(0x1_437f_cf88) as usize
            )
        );
        unreachable!()
    }

    let needed = num_elements.wrapping_add(param_2);
    if rd::<u64>(param_1 + 0x40) < 4 || needed > rd::<u64>(param_1) {
        let rbp = exe::call_va!(0x1_401e_2020, (param_1, needed, 0u64) -> u64);
        let cur_bc = rd::<u64>(param_1 + 0x40);
        if rbp > cur_bc {
            let active_plus = param_2
                .wrapping_sub(rd::<u64>(param_1 + 0x30))
                .wrapping_add(rd::<u64>(param_1 + 0x38));
            let mut r8 = exe::call_va!(0x1_401e_2020, (param_1, active_plus, cur_bc) -> u64);
            if r8 < rbp && r8 < 0x7fff_ffff_ffff_ffff {
                let active_re = param_2
                    .wrapping_sub(rd::<u64>(param_1 + 0x30))
                    .wrapping_add(rd::<u64>(param_1 + 0x38));
                let doubled = r8.wrapping_mul(2);
                let thresh = f32_to_u64_msvc(u64_to_f32(doubled) * rd::<f32>(param_1 + 0x14));
                if active_re >= thresh {
                    r8 = doubled;
                }
            }
            let mut tmp = [0u64; 14];
            exe::call_va!(0x1_4022_7220, (tmp.as_mut_ptr() as usize, param_1, r8));
            exe::call_va!(0x1_40fb_f1e0, (param_1, tmp.as_mut_ptr() as usize));
            exe::call_va!(0x1_4022_7190, (tmp.as_mut_ptr() as usize));
            return 1;
        }
    }
    resized
}

/// `sparsehash_1413cfce0`
/// Original: 0x1413cfce0 (432 bytes, Throws::Discard)
pub unsafe extern "win64" fn sparsehash_1413cfce0(param_1: usize, param_2: usize) -> usize {
    let mut pos_pair = [0u64; 2];
    let _ = exe::call_va!(
        0x1_401f_a730,
        (param_1, pos_pair.as_mut_ptr() as usize, param_2) -> usize
    );
    if pos_pair[0] != u64::MAX {
        return rd::<usize>(param_1 + 0x58).wrapping_add((pos_pair[0] as usize).wrapping_shl(4));
    }

    let resized = exe::call_va!(0x1_413c_fa60, (param_1, 1u64) -> u8);
    let mut kv: [u64; 2] = [rd::<u64>(param_2), 0];
    if resized != 0 {
        let _ = exe::call_va!(
            0x1_401f_a730,
            (param_1, pos_pair.as_mut_ptr() as usize, kv.as_mut_ptr() as usize) -> usize
        );
        if pos_pair[0] != u64::MAX {
            return rd::<usize>(param_1 + 0x58)
                .wrapping_add((pos_pair[0] as usize).wrapping_shl(4));
        }
    }

    let num_elements = rd::<u64>(param_1 + 0x38);
    let num_deleted = rd::<u64>(param_1 + 0x30);
    if num_elements.wrapping_sub(num_deleted) >= 0x0fff_ffff_ffff_ffff {
        let mut exc = [0u8; 32];
        let msg = exe::ptr::<u8>(0x1_4307_aa70) as usize;
        exe::call_va!(0x1_401d_f630, (exc.as_mut_ptr() as usize, msg));
        exe::call_va!(
            0x1_4183_acce,
            (
                exc.as_mut_ptr() as usize,
                exe::ptr::<u8>(0x1_437f_cf88) as usize
            )
        );
        unreachable!()
    }

    let ins_idx = pos_pair[1] as usize;
    let off = ins_idx.wrapping_shl(4);
    let table0 = rd::<usize>(param_1 + 0x58);
    if num_deleted != 0 && rd::<u64>(param_1 + 0x28) == rd::<u64>(table0.wrapping_add(off)) {
        wr::<u64>(param_1 + 0x30, num_deleted.wrapping_sub(1));
    } else {
        wr::<u64>(param_1 + 0x38, num_elements.wrapping_add(1));
    }

    let table1 = rd::<usize>(param_1 + 0x58);
    wr::<[u64; 2]>(table1.wrapping_add(off), kv);
    rd::<usize>(param_1 + 0x58).wrapping_add(off)
}

/// `ui_141410810`
/// Original: 0x141410810 (249 bytes, Throws::Caught)
pub unsafe extern "win64" fn ui_141410810(_param_1: usize, param_2: usize) -> usize {
    let tree_addr = exe::ptr::<u8>(0x1_43c4_7f68) as usize;
    let head = rd::<usize>(tree_addr);
    let mut cur = rd::<usize>(head + 0x08);
    let mut parent = cur;
    let mut add_left: u32 = 0;
    let mut lower_bound = head;

    if rd::<u8>(cur + 0x19) == 0 {
        let key = rd::<u32>(param_2);
        loop {
            parent = cur;
            if rd::<u32>(cur + 0x20) >= key {
                add_left = 1;
                lower_bound = cur;
                cur = rd::<usize>(cur);
            } else {
                add_left = 0;
                cur = rd::<usize>(cur + 0x10);
            }
            if rd::<u8>(cur + 0x19) != 0 {
                break;
            }
        }
    }

    if rd::<u8>(lower_bound + 0x19) != 0 || rd::<u32>(param_2) < rd::<u32>(lower_bound + 0x20) {
        if rd::<u64>(tree_addr + 0x08) == 0x0555_5555_5555_5555 {
            exe::call_va!(0x1_4020_0130, ());
            unreachable!()
        }
        let new_node = exe::call_va!(0x1_4024_d8b0, (tree_addr) -> usize);
        wr::<u32>(new_node + 0x20, rd::<u32>(param_2));
        wr::<u64>(new_node + 0x28, 0);
        wr::<usize>(new_node, head);
        wr::<usize>(new_node + 0x08, head);
        wr::<usize>(new_node + 0x10, head);
        wr::<u16>(new_node + 0x18, 0);
        let mut loc: [u64; 2] = [parent as u64, add_left as u64];
        lower_bound = exe::call_va!(
            0x1_4020_8500,
            (tree_addr, loc.as_mut_ptr() as usize, new_node) -> usize
        );
    }

    lower_bound.wrapping_add(0x28)
}

/// `fmt_141461fb0`
/// Original: 0x141461fb0 (342 bytes, Throws::Caught)
pub unsafe extern "win64" fn fmt_141461fb0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: u64,
) {
    let fmt_ptr = rd::<usize>(param_2);
    let fmt_len = rd::<u64>(param_2 + 0x08);
    if fmt_len != 2 || rd::<u16>(fmt_ptr) != 0x7d7b {
        let mut fmt_copy: [u64; 2] = [fmt_ptr as u64, fmt_len];
        let mut ctx: [u64; 7] = [
            fmt_ptr as u64,
            fmt_len,
            0,
            param_1 as u64,
            rd::<u64>(param_3),
            rd::<u64>(param_3 + 0x08),
            param_4,
        ];
        exe::call_va!(
            0x1_4146_3c60,
            (fmt_copy.as_mut_ptr() as usize, ctx.as_mut_ptr() as usize)
        );
        return;
    }

    let mut arg_buf = [0u64; 3];
    let desc = rd::<u64>(param_3);
    let arg_type: u32;
    if (desc as i64) < 0 {
        let max_args = (desc & 0x7fff_ffff_ffff_ffff) as i32;
        if max_args <= 0 {
            arg_type = 0;
        } else {
            let args_ptr = rd::<usize>(param_3 + 0x08);
            arg_buf[2] = rd::<u64>(args_ptr + 0x10);
            arg_type = arg_buf[2] as u32;
            arg_buf[0] = rd::<u64>(args_ptr);
            arg_buf[1] = rd::<u64>(args_ptr + 0x08);
        }
    } else {
        arg_type = (desc as u32) & 0x0f;
        arg_buf[2] = arg_type as u64;
        if arg_type != 0 {
            let vals_ptr = rd::<usize>(param_3 + 0x08);
            arg_buf[0] = rd::<u64>(vals_ptr);
            arg_buf[1] = rd::<u64>(vals_ptr + 0x08);
        }
    }

    if arg_type != 0 {
        let mut format_ctx: [u64; 4] = [
            param_1 as u64,
            rd::<u64>(param_3),
            rd::<u64>(param_3 + 0x08),
            param_4,
        ];
        let mut dummy_ret: u64 = 0;
        exe::call_va!(
            0x1_4146_2f40,
            (
                (&mut dummy_ret as *mut u64) as usize,
                format_ctx.as_mut_ptr() as usize,
                arg_buf.as_mut_ptr() as usize
            )
        );
        return;
    }

    let mut exc_src: [u64; 2] = [exe::ptr::<u8>(0x1_4307_e0e0) as usize as u64, 1];
    let mut exc_obj: [u64; 3] = [exe::ptr::<u8>(0x1_4266_80a0) as usize as u64, 0, 0];
    exe::call_iat!(0x1_4264_6338, (exc_src.as_mut_ptr() as usize, (exc_obj.as_mut_ptr() as usize) + 0x08));
    exc_obj[0] = exe::ptr::<u8>(0x1_4341_f118) as usize as u64;
    let mut thrown_exc = [0u64; 4];
    exe::call_va!(0x1_4146_2ac0, (thrown_exc.as_mut_ptr() as usize, exc_obj.as_mut_ptr() as usize));
    exe::call_va!(0x1_4183_acce, (thrown_exc.as_mut_ptr() as usize, exe::ptr::<u8>(0x1_437f_eb90) as usize));
    unreachable!()
}

/// `unknown_141645950`
/// Original: 0x141645950 (572 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141645950(param_1: usize) -> u8 {
    let rbx = rd::<usize>(param_1 + 0x08);
    let vtbl = rd::<usize>(rbx);
    if (rd::<u8>(rbx + 0x1f8) & 1) == 0 {
        exe::call_va!(rd::<usize>(vtbl + 0x68), (rbx, 0u32));
    } else {
        exe::call_va!(rd::<usize>(vtbl + 0x70), (rbx));
        if (rd::<u8>(rbx + 0x1f8) & 0x10) != 0 {
            let vtbl2 = rd::<usize>(rbx);
            exe::call_va!(rd::<usize>(vtbl2 + 0x78), (rbx));
        }
    }

    let p8 = rd::<usize>(param_1 + 0x08);
    wr::<u32>(param_1 + 0x48, 0);
    let ok = exe::call_va!(0x1_4159_1e10, (p8) -> u8);
    if ok == 0 {
        return 0;
    }

    exe::call_va!(0x1_4159_0ae0, (rd::<usize>(param_1 + 0x08), param_1 + 0x40));
    let sub_1c0 = rd::<usize>(rd::<usize>(param_1 + 0x08) + 0x1c0);
    let vtbl_1c0 = rd::<usize>(sub_1c0);
    let mode = exe::call_va!(rd::<usize>(vtbl_1c0 + 0x70), (sub_1c0) -> u32);
    let not_10: u8 = if mode != 10 { 1 } else { 0 };

    let mut rdi = rd::<usize>(param_1 + 0x40);
    let end0 = rdi.wrapping_add((rd::<u32>(param_1 + 0x48) as usize).wrapping_shl(5));
    if rdi == end0 {
        return ok;
    }

    loop {
        let p8_cur = rd::<usize>(param_1 + 0x08);
        let p8_inner = rd::<usize>(p8_cur + 0x08);
        if rd::<u64>(p8_inner + 0x58) != rd::<u64>(rdi)
            || rd::<u32>(rdi + 0x10) == 2
            || not_10 != 0
        {
            let key = rd::<u32>(rdi + 0x08);
            let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4393_0f88) as usize);

            loop {
                let st = rd::<i32>(mgr + 0x58);
                if st < 0 {
                    exe::call_iat!(0x1_4264_52b0, (mgr + 0x60));
                    if rd::<i32>(mgr + 0x58) < 0 {
                        break;
                    }
                    exe::call_iat!(0x1_4264_52a8, (mgr + 0x60));
                } else if atomic_cas_i32(mgr + 0x58, st, st.wrapping_add(1)) {
                    break;
                }
            }

            let mut found_node: usize = 0;
            let bucket_cnt = rd::<u32>(mgr + 0xa0);
            if bucket_cnt != 0 {
                let bucket_idx = (key % bucket_cnt) as usize;
                let buckets = rd::<usize>(mgr + 0x98);
                let mut node = rd::<usize>(buckets.wrapping_add(bucket_idx.wrapping_mul(8)));
                while node != 0 {
                    if rd::<u32>(node + 0x10) == key {
                        found_node = node;
                        break;
                    }
                    node = rd::<usize>(node + 0x08);
                }
                if found_node != 0 {
                    let _ = atomic_add_i32(found_node + 0x14, 1);
                }
            }

            let mut st = rd::<i32>(mgr + 0x58);
            if st >= 0 {
                loop {
                    let next_st = st.wrapping_sub(1);
                    if atomic_cas_i32(mgr + 0x58, st, next_st) {
                        if next_st == 0 && rd::<i32>(mgr + 0x88) > 0 {
                            exe::call_iat!(0x1_4264_5560, (rd::<usize>(mgr + 0x90)));
                        }
                        break;
                    }
                    st = rd::<i32>(mgr + 0x58);
                    if st < 0 {
                        exe::call_iat!(0x1_4264_52a8, (mgr + 0x60));
                        break;
                    }
                }
            } else {
                exe::call_iat!(0x1_4264_52a8, (mgr + 0x60));
            }

            let obj = if found_node != 0 {
                found_node.wrapping_sub(0x10)
            } else {
                0
            };
            if obj != 0 {
                let c = exe::call_va!(0x1_415c_a5a0, (obj) -> u8);
                if c != 0 {
                    exe::call_va!(0x1_415e_ee30, (obj, rdi, param_1));
                }
                let ref_obj = obj.wrapping_add(0x10);
                let prev_ref = atomic_add_i32(ref_obj + 0x14, -1);
                if prev_ref == 1 {
                    let vtbl_ref = rd::<usize>(ref_obj);
                    exe::call_va!(rd::<usize>(vtbl_ref + 0x18), (ref_obj));
                }
            }
        }

        rdi = rdi.wrapping_add(0x20);
        let end = rd::<usize>(param_1 + 0x40)
            .wrapping_add((rd::<u32>(param_1 + 0x48) as usize).wrapping_shl(5));
        if rdi == end {
            break;
        }
    }
    ok
}

/// `bigfish_141950630`
/// Original: 0x141950630 (435 bytes, Throws::No)
pub unsafe extern "win64" fn bigfish_141950630(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    param_4: usize,
    param_5: u64,
    param_6: u32,
) -> u64 {
    let base = rd::<usize>(param_1 + 0x10);
    if base != 0 {
        let idx = rd::<usize>(param_1 + 0x20);
        let r10 = base.wrapping_add(idx.wrapping_mul(0xe0));
        if r10 != 0 && rd::<u8>(r10 + 0xa0) == 0 && rd::<u32>(r10 + 0x74) == 1 {
            let p2_lo = rd::<[u8; 16]>(param_2);
            let p2_x = rd::<u32>(param_2 + 0x10);
            let p2_y = rd::<u32>(param_2 + 0x14);
            let p2_z = rd::<u32>(param_2 + 0x18);
            let x = if (param_6 & 1) != 0 { p2_x } else { rd::<u32>(r10 + 0x50) };
            let y = if ((param_6 >> 1) & 1) != 0 { p2_y } else { rd::<u32>(r10 + 0x54) };
            let z = if ((param_6 >> 2) & 1) != 0 { p2_z } else { rd::<u32>(r10 + 0x58) };
            wr::<u32>(r10 + 0x50, x);
            wr::<u32>(r10 + 0x54, y);
            wr::<u32>(r10 + 0x58, z);
            let new_cnt = rd::<u32>(r10 + 0xd4).wrapping_add(1);
            wr::<u32>(r10 + 0xd4, new_cnt);
            wr::<[u8; 16]>(r10 + 0x60, p2_lo);
            let old_mask = rd::<u32>(r10 + 0x20);
            if old_mask != param_6 {
                wr::<u32>(r10 + 0x20, old_mask | param_6);
            }
            return new_cnt as u64;
        }
    }

    let mut frame = [0u8; 0xa0];
    let fb = frame.as_mut_ptr() as usize;
    let p2_val = rd::<[u8; 32]>(param_2);
    exe::call_va!(0x1_403e_2ec0, (fb + 0x20));
    let p3_val = rd::<u64>(param_3);
    wr::<u64>(fb + 0x40, p3_val);
    wr::<u64>(fb + 0x48, p3_val);
    wr::<[u8; 32]>(fb + 0x80, rd::<[u8; 32]>(fb + 0x40));
    wr::<u64>(fb + 0x88, rd::<u64>(param_4));
    wr::<[u8; 32]>(fb + 0x60, p2_val);
    wr::<[u8; 16]>(fb, rd::<[u8; 16]>(exe::ptr::<u8>(0x1_4342_81d0) as usize));
    wr::<[u8; 16]>(fb + 0x10, [0u8; 16]);
    wr::<[u64; 5]>(fb + 0x20, rd::<[u64; 5]>(param_1));
    wr::<u32>(fb + 0x98, param_6);
    exe::call_va!(0x1_4195_13a0, (fb + 0x20, fb + 0x60, param_5, fb) -> u64)
}

/// `bigfish_141950f60`
/// Original: 0x141950f60 (424 bytes, Throws::No)
pub unsafe extern "win64" fn bigfish_141950f60(
    param_1: usize,
    param_2: usize,
    param_3: usize,
    _param_4_r9: usize,
) -> usize {
    let [_, _, _, xmm3] = crate::simd_ops::take_xmm_args();
    let param_4 = f32::from_bits(xmm3 as u32);
    let p2_x = rd::<f32>(param_2);
    let p2_y = rd::<f32>(param_2 + 0x04);
    let p2_z = rd::<f32>(param_2 + 0x08);
    let p3_x = rd::<f32>(param_3);
    let p3_y = rd::<f32>(param_3 + 0x04);
    let p3_z = rd::<f32>(param_3 + 0x08);

    let sub_x = p2_x - param_4;
    let add_x = p3_x + param_4;
    let sub_y = p2_y - param_4;
    let add_y = p3_y + param_4;
    let sub_z = p2_z - param_4;
    let add_z = p3_z + param_4;

    let min_x = f32_min_sse(sub_x, add_x);
    let min_y = f32_min_sse(sub_y, add_y);
    let min_z = f32_min_sse(sub_z, add_z);
    let max_x = f32_max_sse(sub_x, add_x);
    let max_y = f32_max_sse(sub_y, add_y);
    let max_z = f32_max_sse(sub_z, add_z);

    if param_4 < 0.0 {
        wr::<[f32; 6]>(param_1, [min_x, min_y, min_z, max_x, max_y, max_z]);
        wr::<[f32; 6]>(param_1 + 0x18, [p2_x, p2_y, p2_z, p3_x, p3_y, p3_z]);
    } else {
        wr::<[f32; 6]>(param_1, [p2_x, p2_y, p2_z, p3_x, p3_y, p3_z]);
        wr::<[f32; 6]>(param_1 + 0x18, [min_x, min_y, min_z, max_x, max_y, max_z]);
    }
    wr::<u32>(param_1 + 0x30, param_4.to_bits() & 0x7fff_ffff);
    param_1
}

/// `bigfish_141960ba0`
/// Original: 0x141960ba0 (504 bytes, Throws::Discard)
pub unsafe extern "win64" fn bigfish_141960ba0(param_1: usize, param_2: usize) {
    let r10 = rd::<usize>(param_2);
    let idx_word = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ced4) as usize) as usize;
    let r10_v = rd::<usize>(r10);
    let tbl_20 = rd::<usize>(r10_v + 0x20);
    let slot_idx = rd::<u16>(tbl_20.wrapping_add(idx_word * 2)) as usize;
    let entry_ptr = rd::<usize>(r10 + 0x08).wrapping_add(slot_idx.wrapping_shl(4));
    let entry_len = rd::<u32>(entry_ptr + 0x08) as u64;
    let entry_str = rd::<u64>(entry_ptr);
    let mut ctx_30: [u64; 3] = [r10 as u64, entry_len, entry_str];

    let b3 = rd::<u8>(exe::ptr::<u8>(0x1_43a2_0b5b) as usize) as u32;
    let cap_bits = if b3 < 0x80 {
        b3 * 8
    } else {
        (b3 << 8) - 0x7c00
    };
    let hdr = rd::<u32>(exe::ptr::<u32>(0x1_43a2_0b58) as usize);
    let len_bits = (hdr & 0x00ff_ffff) << 3;
    let str_data_addr = exe::ptr::<u8>(0x1_43a2_0b60) as usize;
    let str_ptr = if len_bits > cap_bits {
        rd::<usize>(str_data_addr)
    } else {
        str_data_addr
    };
    let str_len = rd::<u32>(exe::ptr::<u32>(0x1_43a2_0b5c) as usize) as u64;
    let mut sv_20: [u64; 2] = [str_ptr as u64, str_len];
    let mut res_98: u64 = 0;

    exe::call_va!(
        0x1_401e_8cc0,
        (
            (&mut res_98 as *mut u64) as usize,
            sv_20.as_mut_ptr() as usize,
            ctx_30.as_mut_ptr() as usize
        )
    );

    let idx_eax = res_98 as u32;
    if idx_eax == u32::MAX {
        return;
    }

    let rdx = ctx_30[0] as usize;
    let arr_30 = rd::<usize>(rdx + 0x30);
    let r11 = rd::<u64>(arr_30.wrapping_add((idx_eax as usize).wrapping_mul(8)));
    if (r11 as u16) == 0xffff {
        exe::call_iat!(0x1_4264_67e8, ());
        unreachable!()
    }

    let r8d = rd::<u16>(exe::ptr::<u16>(0x1_43c8_cecc) as usize) as u32;
    let rbx = rdx.wrapping_add(0x40);
    let edi = (r11 as u16) as usize;
    let rax_32 = edi.wrapping_shl(5);
    let r14 = rd::<u64>(rbx.wrapping_add(edi.wrapping_mul(8)).wrapping_add(0x48));
    let arg_edx = rd::<u32>(rbx.wrapping_add(rax_32).wrapping_add(0x1806c));
    let arg_rcx = rd::<u64>(rbx.wrapping_add(rax_32).wrapping_add(0x18058));
    let found_i32 = exe::call_va!(0x1_4248_9ee0, (arg_rcx, arg_edx, r8d) -> i32);
    let hi_r11 = r11 >> 32;
    let off_rdx: u64 = if (found_i32 as i64) == -1 {
        0
    } else {
        let ptr_arr = rd::<usize>(rbx.wrapping_add(edi.wrapping_add(0xc03).wrapping_shl(5)));
        rd::<u32>(ptr_arr.wrapping_add((found_i32 as isize as usize).wrapping_mul(4))) as u64
    };
    let base_addr = if off_rdx != 0 {
        r14.wrapping_add(off_rdx)
    } else {
        0
    };
    let target_addr = base_addr.wrapping_add(hi_r11.wrapping_mul(0x140));

    let mut tmp_str = [0u64; 18];
    let s_ptr = exe::call_va!(0x1_41df_ae90, (tmp_str.as_mut_ptr() as usize, target_addr) -> usize);
    let eq = exe::call_va!(
        0x1_401e_6ff0,
        (s_ptr, exe::ptr::<u8>(0x1_43a2_0bd0) as usize) -> u8
    );
    exe::call_va!(0x1_4240_ab10, (tmp_str.as_mut_ptr() as usize));

    if eq != 0 {
        let g_arg = rd::<u64>(exe::ptr::<u64>(0x1_43a3_40f8) as usize);
        let res_ptr = exe::call_va!(
            0x1_420e_33e0,
            (ctx_30.as_mut_ptr() as usize, g_arg) -> usize
        );
        if rd::<u8>(res_ptr + 0x18) != 0 {
            let id = rd::<u32>(param_1 + 0x18);
            if id != u32::MAX {
                res_98 = (res_98 & !0xffff_ffffu64) | (id as u64);
                let slot_ptr = exe::call_va!(
                    0x1_4141_0810,
                    (
                        ctx_30.as_mut_ptr() as usize,
                        (&mut res_98 as *mut u64) as usize
                    ) -> usize
                );
                let inner = rd::<usize>(rd::<usize>(slot_ptr) + 0x28);
                if inner != 0 {
                    let vtbl = rd::<usize>(inner);
                    let on_any_input = exe::ptr::<u8>(0x1_4310_1400) as usize;
                    exe::call_va!(rd::<usize>(vtbl + 0x250), (inner, on_any_input));
                }
            }
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_rend_1412ebe10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut DST_BUF: [u32; 8] = [0; 8];
    static mut TRACE: (u64, u32) = (0, 0);

    unsafe extern "win64" fn mock_ecf0(_this: usize, cap: u64) -> usize {
        TRACE.0 = cap;
        core::ptr::addr_of_mut!(DST_BUF) as usize
    }
    unsafe extern "win64" fn mock_e830(_this: usize) {
        TRACE.1 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_ecf0, mock_ecf0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, mock_e830 as *const ())?;

    let run = |f: F, len: u32, cap: u32| -> (bool, u32, u32, [u32; 8], (u64, u32)) {
        DST_BUF = [0; 8];
        TRACE = (0, 0);
        let src: [u32; 4] = [0x11, 0x22, 0x33, 0x44];
        let mut vec_hdr: [u64; 2] = [src.as_ptr() as usize as u64, (len as u64) | ((cap as u64) << 32)];
        let vp = vec_hdr.as_mut_ptr() as usize;
        f(vp);
        let moved = rd::<usize>(vp) == (core::ptr::addr_of!(DST_BUF) as usize);
        (moved, rd::<u32>(vp + 0x08), rd::<u32>(vp + 0x0c), DST_BUF, TRACE)
    };

    for (len, cap) in [(3u32, 3u32), (3, 6)] {
        let o = run(f_o, len, cap);
        let n = run(f_n, len, cap);
        crate::diff_eq!(o, n, "1412ebe10 len={} cap={}", len, cap);
    }
    Ok(2)
}

pub unsafe fn diff_sparsehash_1413a1800(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, query_key: u64| -> [u64; 2] {
        // 4 buckets of 24 bytes (3 u64s each): empty = 0xffff, deleted = 0xfffe
        let mut buckets: [u64; 12] = [0xffff; 12];
        // bucket 1: deleted (0xfffe)
        buckets[3] = 0xfffe;
        // bucket 2: key 5 (5 & 3 == 1, probed +1 -> 2)
        buckets[6] = 5;
        // bucket 3: empty (0xffff)

        let mut ht = [0u8; 0x68];
        let hp = ht.as_mut_ptr() as usize;
        wr::<u64>(hp + 0x28, 0xfffe);
        wr::<u64>(hp + 0x30, 1);
        wr::<u64>(hp + 0x40, 4);
        wr::<u64>(hp + 0x48, 0xffff);
        wr::<usize>(hp + 0x60, buckets.as_mut_ptr() as usize);

        let mut out = [0u64; 2];
        let ret = f(
            hp,
            out.as_mut_ptr() as usize,
            (&query_key as *const u64) as usize,
        );
        assert_eq!(ret, out.as_ptr() as usize);
        out
    };

    for k in [5u64, 9, 0] {
        let o = run(f_o, k);
        let n = run(f_n, k);
        crate::diff_eq!(o, n, "1413a1800 k={}", k);
    }
    Ok(3)
}

pub unsafe fn diff_sparsehash_1413a18e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, bc_b: u64| -> ([u64; 13], [u64; 13]) {
        let mut a = [0u64; 13];
        let mut b = [0u64; 13];
        for i in 0..13 {
            a[i] = 0x100 + i as u64;
            b[i] = 0x200 + i as u64;
        }
        let ap = a.as_mut_ptr() as usize;
        let bp = b.as_mut_ptr() as usize;
        wr::<f32>(ap + 0x10, 0.8);
        wr::<f32>(ap + 0x14, 0.2);
        wr::<u8>(ap + 0x18, 1);
        wr::<u64>(ap + 0x40, 64);

        wr::<f32>(bp + 0x10, 0.75);
        wr::<f32>(bp + 0x14, 0.25);
        wr::<u8>(bp + 0x18, 1);
        wr::<u64>(bp + 0x40, bc_b);

        f(ap, bp);
        (a, b)
    };

    for bc in [128u64, 0x8000_0000_0000_0040] {
        let o = run(f_o, bc);
        let n = run(f_n, bc);
        crate::diff_eq!(o, n, "1413a18e0 bc={:#x}", bc);
    }
    Ok(2)
}

pub unsafe fn diff_sparsehash_1413a1c10(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RESIZE_RET: u8 = 0;

    unsafe extern "win64" fn mock_1f00(_ht: usize) -> u8 {
        RESIZE_RET
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_1f00, mock_1f00 as *const ())?;

    let run = |f: F, resize_ret: u8, query_key: u64| -> (usize, [u64; 12], u64, u64) {
        RESIZE_RET = resize_ret;
        let mut buckets: [u64; 12] = [0xffff; 12];
        buckets[3] = 0xfffe;
        buckets[6] = 5;
        buckets[7] = 0x777;

        let mut ht = [0u8; 0x68];
        let hp = ht.as_mut_ptr() as usize;
        wr::<u64>(hp + 0x28, 0xfffe);
        wr::<u64>(hp + 0x30, 1);
        wr::<u64>(hp + 0x38, 2);
        wr::<u64>(hp + 0x40, 4);
        wr::<u64>(hp + 0x48, 0xffff);
        wr::<usize>(hp + 0x60, buckets.as_mut_ptr() as usize);

        let ret = f(hp, (&query_key as *const u64) as usize);
        let rel_off = ret.wrapping_sub(buckets.as_ptr() as usize);
        (rel_off, buckets, rd::<u64>(hp + 0x30), rd::<u64>(hp + 0x38))
    };

    for (rz, k) in [(0u8, 5u64), (0, 9), (1, 9), (1, 3)] {
        let o = run(f_o, rz, k);
        let n = run(f_n, rz, k);
        crate::diff_eq!(o, n, "1413a1c10 rz={} k={}", rz, k);
    }
    Ok(4)
}

pub unsafe fn diff_sparsehash_1413a1f00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_TRACE: (u64, u32) = (0, 0);

    unsafe extern "win64" fn mock_1660(dst: usize, src: usize, r8: u64) {
        CALL_TRACE.0 = r8;
        wr::<[u64; 13]>(dst, rd::<[u64; 13]>(src));
        wr::<u64>(dst + 0x40, r8);
    }
    unsafe extern "win64" fn mock_18e0(a: usize, b: usize) {
        sparsehash_1413a18e0(a, b);
    }
    unsafe extern "win64" fn mock_15d0(_tmp: usize) {
        CALL_TRACE.1 += 1;
    }
    unsafe extern "win64" fn mock_2020(_ht: usize, needed: u64, _min_buck: u64) -> u64 {
        if needed <= 4 {
            32
        } else {
            64
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_1660, mock_1660 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413a_18e0, mock_18e0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_15d0, mock_15d0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_2020, mock_2020 as *const ())?;

    let run = |f: F, consider_shrink: u8, bc: u64, num_elems: u64| -> (u8, [u64; 13], (u64, u32)) {
        CALL_TRACE = (0, 0);
        let mut ht = [0u64; 13];
        let hp = ht.as_mut_ptr() as usize;
        wr::<u64>(hp, 10);
        wr::<u64>(hp + 0x08, 8);
        wr::<f32>(hp + 0x10, 0.8);
        wr::<f32>(hp + 0x14, 0.2);
        wr::<u8>(hp + 0x18, consider_shrink);
        wr::<u64>(hp + 0x30, 1);
        wr::<u64>(hp + 0x38, num_elems);
        wr::<u64>(hp + 0x40, bc);
        let ret = f(hp);
        (ret, ht, CALL_TRACE)
    };

    for (sh, bc, ne) in [(1u8, 128u64, 3u64), (0, 32, 12), (0, 32, 5)] {
        let o = run(f_o, sh, bc, ne);
        let n = run(f_n, sh, bc, ne);
        crate::diff_eq!(o, n, "1413a1f00 sh={} bc={} ne={}", sh, bc, ne);
    }
    Ok(3)
}

pub unsafe fn diff_r_1413aaa50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64, u64, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut VEC_STORAGE: [u64; 8] = [0; 8];
    static mut TRACE: (u64, u64, u32) = (0, 0, 0);

    unsafe extern "win64" fn mock_1b20(ctx: usize, rdx: usize, r8: usize) {
        TRACE.0 = rd::<u64>(rdx);
        TRACE.1 = rd::<u64>(r8 + 0x08);
        wr::<u8>(ctx + 0x200, 1);
        wr::<usize>(ctx + 0x130, 0x1234);
    }
    unsafe extern "win64" fn mock_cb60(vec_ptr: usize, cap: u64) {
        wr::<usize>(vec_ptr, core::ptr::addr_of_mut!(VEC_STORAGE) as usize);
        wr::<u32>(vec_ptr + 0x0c, cap as u32);
    }
    unsafe extern "win64" fn mock_1560(_ctx_1d8: usize, elem: u64, item_buf: usize) -> u8 {
        if elem == 10 {
            wr::<u64>(item_buf, 0x55);
            1
        } else {
            0
        }
    }
    unsafe extern "win64" fn mock_17c0(item_buf: usize, elem: u64) -> u64 {
        rd::<u64>(item_buf) + elem * 100
    }
    unsafe extern "win64" fn mock_ab10(_p: usize) {
        TRACE.2 += 1;
    }
    unsafe extern "win64" fn mock_2de0(_p: usize) {
        TRACE.2 += 10;
    }
    unsafe extern "win64" fn mock_6280(_p: usize) {
        TRACE.2 += 100;
    }
    unsafe extern "win64" fn mock_bbb0(
        _p1: usize,
        p2: usize,
        _r8: usize,
        rem: u64,
        desc: usize,
    ) {
        wr::<u64>(p2, rem);
        wr::<u64>(p2 + 0x08, rd::<u64>(desc));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_1b20, mock_1b20 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413a_cb60, mock_cb60 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4248_1560, mock_1560 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_17c0, mock_17c0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4244_2de0, mock_2de0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_413c_6280, mock_6280 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_413a_bbb0, mock_bbb0 as *const ())?;

    let run = |f: F, kind: u32| -> ([u64; 2], u64, (u64, u64, u32)) {
        VEC_STORAGE = [0; 8];
        TRACE = (0, 0, 0);
        let elems: [u64; 2] = [10, 20];
        let mut p1: [u64; 2] = [elems.as_ptr() as usize as u64, 2];
        let mut p2: [u64; 2] = [0xdead, 0xbeef];

        let mut dmkp: [u32; 8] = [0x504b_4d44, 2, 1, 2, 3, 4, 5, 6];
        let mut p5 = [0u8; 0x60];
        let p5p = p5.as_mut_ptr() as usize;
        wr::<u32>(p5p, 0x0400_0004);
        wr::<u32>(p5p + 0x04, 4);
        wr::<usize>(p5p + 0x20, dmkp.as_mut_ptr() as usize);
        wr::<u64>(p5p + 0x28, 64);
        wr::<u32>(p5p + 0x50, kind);

        let ret = f(p1.as_mut_ptr() as usize, p2.as_mut_ptr() as usize, 0, 0, p5p);
        assert_eq!(ret, p2.as_ptr() as usize);
        let mut out_p2 = p2;
        if kind == 2 {
            out_p2[0] = 0;
        }
        (out_p2, VEC_STORAGE[0], TRACE)
    };

    for kind in [2u32, 3, 1] {
        let o = run(f_o, kind);
        let n = run(f_n, kind);
        crate::diff_eq!(o, n, "1413aaa50 kind={}", kind);
    }
    Ok(3)
}

pub unsafe fn diff_r_1413ab790(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TMP_VEC_BUF: [u64; 2] = [0; 2];
    static mut FREE_COUNT: u32 = 0;

    unsafe extern "win64" fn mock_aa50(
        _p1: usize,
        out_vec: usize,
        _a3: u64,
        _a4: usize,
        _a5: usize,
    ) -> usize {
        TMP_VEC_BUF = [0xabc001, 0xabc002];
        wr::<usize>(out_vec, core::ptr::addr_of_mut!(TMP_VEC_BUF) as usize);
        wr::<u32>(out_vec + 0x08, 2);
        wr::<u32>(out_vec + 0x0c, 2);
        out_vec
    }
    unsafe extern "win64" fn mock_3cd0(vec_hdr: usize, new_cap: u64) {
        wr::<u32>(vec_hdr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_c190(_ptr: usize) {
        FREE_COUNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_413a_aa50, mock_aa50 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_3cd0, mock_3cd0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F| -> ([u64; 4], u32, u32, u32, u8, u32) {
        FREE_COUNT = 0;
        let mut dst_items = [0u64; 4];
        let mut entry = [0u8; 0x88];
        let ep = entry.as_mut_ptr() as usize;
        wr::<u64>(ep + 0x28, 50_000_000);
        wr::<usize>(ep + 0x70, dst_items.as_mut_ptr() as usize);
        wr::<u32>(ep + 0x78, 0);
        wr::<u32>(ep + 0x7c, 1);

        let base_time: u64 = 2;
        let mut obj = [0u8; 0x70];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x08, 0x1111);
        wr::<usize>(op + 0x10, (&base_time as *const u64) as usize);
        wr::<usize>(op + 0x28, ep);
        wr::<u32>(op + 0x30, 1);
        wr::<u8>(op + 0x68, 0);

        let mut p1: usize = op;
        f((&mut p1 as *mut usize) as usize);
        (dst_items, rd::<u32>(ep + 0x08), rd::<u32>(ep + 0x78), rd::<u32>(ep + 0x7c), rd::<u8>(ep + 0x80), FREE_COUNT)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1413ab790");
    Ok(1)
}

pub unsafe fn diff_sparsehash_1413cfa60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: (u64, u32) = (0, 0);

    unsafe extern "win64" fn mock_fbe0(ht: usize) -> u8 {
        wr::<u8>(ht + 0x18, 0);
        1
    }
    unsafe extern "win64" fn mock_2020(_ht: usize, needed: u64, _min_buck: u64) -> u64 {
        if needed <= 4 {
            16
        } else {
            64
        }
    }
    unsafe extern "win64" fn mock_7220(_tmp: usize, _ht: usize, r8: u64) {
        TRACE.0 = r8;
    }
    unsafe extern "win64" fn mock_f1e0(_ht: usize, _tmp: usize) {
        TRACE.1 += 10;
    }
    unsafe extern "win64" fn mock_7190(_tmp: usize) {
        TRACE.1 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_413c_fbe0, mock_fbe0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_2020, mock_2020 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4022_7220, mock_7220 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_40fb_f1e0, mock_f1e0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4022_7190, mock_7190 as *const ())?;

    let run = |f: F, shrink: u8, delta: u64| -> (u8, (u64, u32)) {
        TRACE = (0, 0);
        let mut ht = [0u64; 12];
        let hp = ht.as_mut_ptr() as usize;
        wr::<u64>(hp, 10);
        wr::<f32>(hp + 0x10, 0.8);
        wr::<f32>(hp + 0x14, 0.2);
        wr::<u8>(hp + 0x18, shrink);
        wr::<u64>(hp + 0x30, 2);
        wr::<u64>(hp + 0x38, 4);
        wr::<u64>(hp + 0x40, 32);
        let ret = f(hp, delta);
        (ret, TRACE)
    };

    for (sh, d) in [(1u8, 2u64), (0, 10)] {
        let o = run(f_o, sh, d);
        let n = run(f_n, sh, d);
        crate::diff_eq!(o, n, "1413cfa60 sh={} d={}", sh, d);
    }
    Ok(2)
}

pub unsafe fn diff_sparsehash_1413cfce0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut RESIZE_MODE: u8 = 0;

    unsafe extern "win64" fn mock_a730(_ht: usize, out_pair: usize, key_ptr: usize) -> usize {
        let k = rd::<u64>(key_ptr);
        if k == 42 {
            wr::<u64>(out_pair, 1);
            wr::<u64>(out_pair + 0x08, u64::MAX);
        } else {
            wr::<u64>(out_pair, u64::MAX);
            wr::<u64>(out_pair + 0x08, 2);
        }
        out_pair
    }
    unsafe extern "win64" fn mock_fa60(_ht: usize, _delta: u64) -> u8 {
        RESIZE_MODE
    }

    let _m1 = crate::difftest::Mock::new(0x1_401f_a730, mock_a730 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413c_fa60, mock_fa60 as *const ())?;

    let run = |f: F, rz: u8, key: u64, del_slot: bool| -> (usize, [u64; 8], u64, u64) {
        RESIZE_MODE = rz;
        let mut table = [0xffffu64; 8];
        if del_slot {
            table[4] = 0xfffe;
        }
        let mut ht = [0u8; 0x60];
        let hp = ht.as_mut_ptr() as usize;
        wr::<u64>(hp + 0x28, 0xfffe);
        wr::<u64>(hp + 0x30, if del_slot { 1 } else { 0 });
        wr::<u64>(hp + 0x38, 3);
        wr::<usize>(hp + 0x58, table.as_mut_ptr() as usize);

        let ret = f(hp, (&key as *const u64) as usize);
        let rel = ret.wrapping_sub(table.as_ptr() as usize);
        (rel, table, rd::<u64>(hp + 0x30), rd::<u64>(hp + 0x38))
    };

    for (rz, k, del) in [(0u8, 42u64, false), (0, 99, true), (1, 99, false)] {
        let o = run(f_o, rz, k, del);
        let n = run(f_n, rz, k, del);
        crate::diff_eq!(o, n, "1413cfce0 rz={} k={} del={}", rz, k, del);
    }
    Ok(3)
}

pub unsafe fn diff_ui_141410810(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_NODE: [u8; 0x30] = [0; 0x30];
    static mut INS_ARGS: (u32, usize) = (0, 0);

    unsafe extern "win64" fn mock_d8b0(_tree: usize) -> usize {
        core::ptr::addr_of_mut!(NEW_NODE) as usize
    }
    unsafe extern "win64" fn mock_8500(_tree: usize, loc: usize, node: usize) -> usize {
        INS_ARGS = (rd::<u32>(loc + 0x08), node);
        node
    }

    let _m1 = crate::difftest::Mock::new(0x1_4024_d8b0, mock_d8b0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4020_8500, mock_8500 as *const ())?;

    let tree_global = exe::ptr::<u8>(0x1_43c4_7f68) as usize;
    let saved_global = rd::<[u64; 2]>(tree_global);

    let run = |f: F, query_key: u32| -> (bool, u32, u64, u32) {
        NEW_NODE = [0xaa; 0x30];
        INS_ARGS = (0, 0);

        let mut head = [0u8; 0x30];
        let mut root = [0u8; 0x30];
        let hp = head.as_mut_ptr() as usize;
        let rp = root.as_mut_ptr() as usize;

        wr::<u8>(hp + 0x19, 1);
        wr::<usize>(hp + 0x08, rp);

        wr::<usize>(rp, hp);
        wr::<usize>(rp + 0x08, hp);
        wr::<usize>(rp + 0x10, hp);
        wr::<u8>(rp + 0x19, 0);
        wr::<u32>(rp + 0x20, 10);
        wr::<u64>(rp + 0x28, 0x1234_5678);

        wr::<usize>(tree_global, hp);
        wr::<u64>(tree_global + 0x08, 1);

        let ret = f(tree_global, (&query_key as *const u32) as usize);
        wr::<[u64; 2]>(tree_global, saved_global);

        let is_existing = ret == rp + 0x28;
        let nnp = core::ptr::addr_of!(NEW_NODE) as usize;
        (is_existing, rd::<u32>(nnp + 0x20), rd::<u64>(nnp + 0x28), INS_ARGS.0)
    };

    for k in [10u32, 5, 20] {
        let o = run(f_o, k);
        let n = run(f_n, k);
        crate::diff_eq!(o, n, "141410810 k={}", k);
    }
    Ok(3)
}

pub unsafe fn diff_fmt_141461fb0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CAPTURED: (u64, u64, u64, u64, u64) = (0, 0, 0, 0, 0);

    unsafe extern "win64" fn mock_3c60(fmt_ptr: usize, ctx_ptr: usize) {
        CAPTURED = (
            1,
            rd::<u64>(fmt_ptr + 0x08),
            rd::<u64>(ctx_ptr + 0x18),
            rd::<u64>(ctx_ptr + 0x20),
            rd::<u64>(ctx_ptr + 0x30),
        );
    }
    unsafe extern "win64" fn mock_2f40(_ret: usize, ctx_ptr: usize, arg_ptr: usize) {
        CAPTURED = (
            2,
            rd::<u64>(ctx_ptr),
            rd::<u64>(ctx_ptr + 0x18),
            rd::<u64>(arg_ptr),
            rd::<u32>(arg_ptr + 0x10) as u64,
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_4146_3c60, mock_3c60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4146_2f40, mock_2f40 as *const ())?;

    let run = |f: F, mode: u32| -> (u64, u64, u64, u64, u64) {
        CAPTURED = (0, 0, 0, 0, 0);
        let s_braces: [u8; 2] = *b"{}";
        let s_other: [u8; 4] = *b"{:x}";
        let fmt_sv: [u64; 2] = if mode == 0 {
            [s_other.as_ptr() as usize as u64, 4]
        } else {
            [s_braces.as_ptr() as usize as u64, 2]
        };
        let arg_data: [u64; 3] = [0x1122_3344, 0x5566_7788, 7];
        let fmt_args: [u64; 2] = if mode == 2 {
            [0x8000_0000_0000_0001, arg_data.as_ptr() as usize as u64]
        } else {
            [5, arg_data.as_ptr() as usize as u64]
        };
        f(
            0x9000,
            fmt_sv.as_ptr() as usize,
            fmt_args.as_ptr() as usize,
            0xabcd,
        );
        CAPTURED
    };

    for mode in [0u32, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "141461fb0 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_141645950(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: (u32, u32, u32) = (0, 0, 0);

    unsafe extern "win64" fn mock_v68(_this: usize, _arg: u32) {
        TRACE.0 += 1;
    }
    unsafe extern "win64" fn mock_v70_mode(_this: usize) -> u32 {
        5
    }
    unsafe extern "win64" fn mock_1e10(_p8: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_0ae0(_p8: usize, vec_hdr: usize) {
        wr::<u32>(vec_hdr + 0x08, 1);
    }
    unsafe extern "win64" fn mock_a5a0(_obj: usize) -> u8 {
        TRACE.1 += 1;
        1
    }
    unsafe extern "win64" fn mock_ee30(_obj: usize, _elem: usize, _p1: usize) {
        TRACE.2 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4159_1e10, mock_1e10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4159_0ae0, mock_0ae0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_415c_a5a0, mock_a5a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_415e_ee30, mock_ee30 as *const ())?;

    let mgr_global = exe::ptr::<usize>(0x1_4393_0f88) as usize;
    let saved_mgr = rd::<usize>(mgr_global);

    let run = |f: F| -> (u8, i32, i32, (u32, u32, u32)) {
        TRACE = (0, 0, 0);

        let mut vtbl = [0usize; 16];
        vtbl[0x68 / 8] = mock_v68 as *const () as usize;
        vtbl[0x70 / 8] = mock_v70_mode as *const () as usize;

        let mut inner_8 = [0u8; 0x60];
        let mut sub_1c0: usize = vtbl.as_ptr() as usize;

        let mut p8 = [0u8; 0x200];
        let p8p = p8.as_mut_ptr() as usize;
        wr::<usize>(p8p, vtbl.as_ptr() as usize);
        wr::<usize>(p8p + 0x08, inner_8.as_mut_ptr() as usize);
        wr::<usize>(p8p + 0x1c0, (&mut sub_1c0 as *mut usize) as usize);
        wr::<u8>(p8p + 0x1f8, 0);

        let mut target_obj = [0u8; 0x30];
        let top = target_obj.as_mut_ptr() as usize;
        let node_ptr = top + 0x10;
        wr::<u32>(node_ptr + 0x10, 7);
        wr::<i32>(node_ptr + 0x14, 2);

        let mut buckets: [usize; 4] = [0, 0, 0, node_ptr];
        let mut mgr = [0u8; 0xb0];
        let mp = mgr.as_mut_ptr() as usize;
        wr::<i32>(mp + 0x58, 0);
        wr::<usize>(mp + 0x98, buckets.as_mut_ptr() as usize);
        wr::<u32>(mp + 0xa0, 4);
        wr::<usize>(mgr_global, mp);

        let mut elem = [0u8; 0x20];
        wr::<u32>(elem.as_mut_ptr() as usize + 0x08, 7);

        let mut p1 = [0u8; 0x50];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, p8p);
        wr::<usize>(p1p + 0x40, elem.as_mut_ptr() as usize);

        let ret = f(p1p);
        wr::<usize>(mgr_global, saved_mgr);
        (ret, rd::<i32>(mp + 0x58), rd::<i32>(node_ptr + 0x14), TRACE)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141645950");
    Ok(1)
}

pub unsafe fn diff_bigfish_141950630(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize, u64, u32) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SLOW_CAPTURED: ([u64; 5], [u64; 8], u64, [u64; 4]) = ([0; 5], [0; 8], 0, [0; 4]);

    unsafe extern "win64" fn mock_2ec0(rcx: usize) {
        wr::<[u64; 8]>(rcx, [0xaa; 8]);
    }
    unsafe extern "win64" fn mock_13a0(rcx: usize, rdx: usize, r8: u64, r9: usize) -> u64 {
        SLOW_CAPTURED = (
            rd::<[u64; 5]>(rcx),
            rd::<[u64; 8]>(rdx),
            r8,
            rd::<[u64; 4]>(r9),
        );
        777
    }

    let _m1 = crate::difftest::Mock::new(0x1_403e_2ec0, mock_2ec0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4195_13a0, mock_13a0 as *const ())?;

    let run = |f: F, fast: bool, mask: u32| -> (u64, [u32; 3], [u64; 2], u32, u32, ([u64; 5], [u64; 8], u64, [u64; 4])) {
        SLOW_CAPTURED = ([0; 5], [0; 8], 0, [0; 4]);
        let mut slot = [0u8; 0xe0];
        let sp = slot.as_mut_ptr() as usize;
        wr::<u32>(sp + 0x20, 1);
        wr::<u32>(sp + 0x50, 10);
        wr::<u32>(sp + 0x54, 20);
        wr::<u32>(sp + 0x58, 30);
        wr::<u32>(sp + 0x74, if fast { 1 } else { 0 });
        wr::<u8>(sp + 0xa0, 0);
        wr::<u32>(sp + 0xd4, 4);

        let mut p1: [u64; 5] = [1, 2, sp as u64, 3, 0];
        let p2: [u32; 8] = [100, 101, 102, 103, 200, 201, 202, 203];
        let p3: u64 = 0x1111_2222_3333_4444;
        let p4: u64 = 0x5555_6666_7777_8888;

        let ret = f(p1.as_mut_ptr() as usize, p2.as_ptr() as usize, (&p3 as *const u64) as usize, (&p4 as *const u64) as usize, 0x9999, mask);
        let mut cap = SLOW_CAPTURED;
        cap.0[2] = 0;
        (ret, rd::<[u32; 3]>(sp + 0x50), rd::<[u64; 2]>(sp + 0x60), rd::<u32>(sp + 0x20), rd::<u32>(sp + 0xd4), cap)
    };

    for (fast, mask) in [(true, 0b101u32), (true, 0b010), (false, 3)] {
        let o = run(f_o, fast, mask);
        let n = run(f_n, fast, mask);
        crate::diff_eq!(o, n, "141950630 fast={} mask={}", fast, mask);
    }
    Ok(3)
}

pub unsafe fn diff_bigfish_141950f60(orig: usize, new: usize) -> Result<u32, String> {
    let run = |fn_addr: usize, d: f32| -> [u32; 13] {
        let mut out = [0u32; 13];
        let p2: [f32; 3] = [1.0, 5.0, -2.0];
        let p3: [f32; 3] = [4.0, 2.0, 6.0];
        let (ret, _) = crate::simd_ops::call_va8_xmm(
            fn_addr as u64,
            out.as_mut_ptr() as u64,
            p2.as_ptr() as u64,
            p3.as_ptr() as u64,
            0,
            0,
            0,
            0,
            0,
            [0, 0, 0, d.to_bits() as u128],
        );
        assert_eq!(ret as usize, out.as_ptr() as usize);
        out
    };

    for d in [2.5f32, -1.5f32, 0.0f32] {
        let o = run(orig, d);
        let n = run(new, d);
        crate::diff_eq!(o, n, "141950f60 d={}", d);
    }
    Ok(3)
}

pub unsafe fn diff_bigfish_141960ba0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut TRACE: (u64, u32, u64) = (0, 0, 0);
    static mut MAP_SLOT: usize = 0;
    static mut RES_33E0: [u8; 0x20] = [0; 0x20];

    unsafe extern "win64" fn mock_8cc0(out_idx: usize, _sv: usize, _ctx: usize) {
        wr::<u32>(out_idx, 0);
    }
    unsafe extern "win64" fn mock_9ee0(_rcx: u64, edx: u32, _r8: u32) -> i32 {
        TRACE.1 = edx;
        0
    }
    unsafe extern "win64" fn mock_ae90(tmp_str: usize, target: u64) -> usize {
        TRACE.0 = target;
        tmp_str
    }
    unsafe extern "win64" fn mock_6ff0(_s: usize, _pat: usize) -> u8 {
        1
    }
    unsafe extern "win64" fn mock_ab10(_tmp_str: usize) {}
    unsafe extern "win64" fn mock_33e0(_ctx: usize, _g: u64) -> usize {
        RES_33E0[0x18] = 1;
        core::ptr::addr_of_mut!(RES_33E0) as usize
    }
    unsafe extern "win64" fn mock_0810(_rcx: usize, key_ptr: usize) -> usize {
        TRACE.2 = rd::<u32>(key_ptr) as u64;
        core::ptr::addr_of_mut!(MAP_SLOT) as usize
    }
    unsafe extern "win64" fn mock_v250(_this: usize, _ev: usize) {
        TRACE.2 += 1000;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_8cc0, mock_8cc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_9ee0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_41df_ae90, mock_ae90 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_401e_6ff0, mock_6ff0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_420e_33e0, mock_33e0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4141_0810, mock_0810 as *const ())?;

    let run = |f: F| -> (u64, u32, u64) {
        TRACE = (0, 0, 0);

        let mut vtbl = [0usize; 0x50];
        vtbl[0x250 / 8] = mock_v250 as *const () as usize;
        let inner_obj: usize = vtbl.as_ptr() as usize;
        let mut outer_obj = [0u8; 0x30];
        wr::<usize>(outer_obj.as_mut_ptr() as usize + 0x28, (&inner_obj as *const usize) as usize);
        MAP_SLOT = outer_obj.as_mut_ptr() as usize;

        let idx_word = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ced4) as usize) as usize;
        let mut word_tbl = vec![0u16; idx_word + 2];
        let mut r10_v = [0u8; 0x28];
        wr::<usize>(r10_v.as_mut_ptr() as usize + 0x20, word_tbl.as_mut_ptr() as usize);

        let entry_arr: [u64; 2] = [0x1234, 5];
        let arr_30: [u64; 1] = [(2u64 << 32) | 0u64];
        let off_arr: [u32; 1] = [0x40];

        let mut r10_buf = vec![0u8; 0x18100];
        let r10p = r10_buf.as_mut_ptr() as usize;
        wr::<usize>(r10p, r10_v.as_mut_ptr() as usize);
        wr::<usize>(r10p + 0x08, entry_arr.as_ptr() as usize);
        wr::<usize>(r10p + 0x30, arr_30.as_ptr() as usize);
        wr::<u64>(r10p + 0x88, 0x1000);
        wr::<usize>(r10p + 0x40 + 0xc03 * 0x20, off_arr.as_ptr() as usize);
        wr::<u64>(r10p + 0x40 + 0x18058, 0x5555);
        wr::<u32>(r10p + 0x40 + 0x1806c, 77);

        let mut p1 = [0u8; 0x20];
        wr::<u32>(p1.as_mut_ptr() as usize + 0x18, 9);
        let p2: usize = r10p;
        f(p1.as_mut_ptr() as usize, (&p2 as *const usize) as usize);
        TRACE
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141960ba0");
    Ok(1)
}

// REGISTRY:
// Replacement::verified(0x1_412e_be10, "rend_1412ebe10", rend::cauldron_hot_z2b::rend_1412ebe10 as *const (), rend::cauldron_hot_z2b::diff_rend_1412ebe10),
// Replacement::verified(0x1_413a_1800, "sparsehash_1413a1800", rend::cauldron_hot_z2b::sparsehash_1413a1800 as *const (), rend::cauldron_hot_z2b::diff_sparsehash_1413a1800),
// Replacement::verified(0x1_413a_18e0, "sparsehash_1413a18e0", rend::cauldron_hot_z2b::sparsehash_1413a18e0 as *const (), rend::cauldron_hot_z2b::diff_sparsehash_1413a18e0),
// Replacement::verified_throws(0x1_413a_1c10, "sparsehash_1413a1c10", rend::cauldron_hot_z2b::sparsehash_1413a1c10 as *const (), Throws::Discard, rend::cauldron_hot_z2b::diff_sparsehash_1413a1c10),
// Replacement::verified_throws(0x1_413a_1f00, "sparsehash_1413a1f00", rend::cauldron_hot_z2b::sparsehash_1413a1f00 as *const (), Throws::Discard, rend::cauldron_hot_z2b::diff_sparsehash_1413a1f00),
// Replacement::verified_throws(0x1_413a_aa50, "r_1413aaa50", rend::cauldron_hot_z2b::r_1413aaa50 as *const (), Throws::Caught, rend::cauldron_hot_z2b::diff_r_1413aaa50),
// Replacement::verified_throws(0x1_413a_b790, "r_1413ab790", rend::cauldron_hot_z2b::r_1413ab790 as *const (), Throws::Caught, rend::cauldron_hot_z2b::diff_r_1413ab790),
// Replacement::verified_throws(0x1_413c_fa60, "sparsehash_1413cfa60", rend::cauldron_hot_z2b::sparsehash_1413cfa60 as *const (), Throws::Discard, rend::cauldron_hot_z2b::diff_sparsehash_1413cfa60),
// Replacement::verified_throws(0x1_413c_fce0, "sparsehash_1413cfce0", rend::cauldron_hot_z2b::sparsehash_1413cfce0 as *const (), Throws::Discard, rend::cauldron_hot_z2b::diff_sparsehash_1413cfce0),
// Replacement::verified_throws(0x1_4141_0810, "ui_141410810", rend::cauldron_hot_z2b::ui_141410810 as *const (), Throws::Caught, rend::cauldron_hot_z2b::diff_ui_141410810),
// Replacement::verified_throws(0x1_4146_1fb0, "fmt_141461fb0", rend::cauldron_hot_z2b::fmt_141461fb0 as *const (), Throws::Caught, rend::cauldron_hot_z2b::diff_fmt_141461fb0),
// Replacement::verified_throws(0x1_4164_5950, "unknown_141645950", rend::cauldron_hot_z2b::unknown_141645950 as *const (), Throws::Discard, rend::cauldron_hot_z2b::diff_unknown_141645950),
// Replacement::verified(0x1_4195_0630, "bigfish_141950630", rend::cauldron_hot_z2b::bigfish_141950630 as *const (), rend::cauldron_hot_z2b::diff_bigfish_141950630),
// Replacement::verified(0x1_4195_0f60, "bigfish_141950f60", rend::cauldron_hot_z2b::bigfish_141950f60 as *const (), rend::cauldron_hot_z2b::diff_bigfish_141950f60),
// Replacement::verified_throws(0x1_4196_0ba0, "bigfish_141960ba0", rend::cauldron_hot_z2b::bigfish_141960ba0 as *const (), Throws::Discard, rend::cauldron_hot_z2b::diff_bigfish_141960ba0),
