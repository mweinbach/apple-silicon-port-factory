//! Decompiled hot functions shard W1 (`0x1401e8620..0x14109b600`).

use crate::exe::{self, rd, wr};

/// `unknown_1401e8620`
/// Original: 0x1401e8620 (88 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1401e8620(param_1: usize) -> u64 {
    let mut backoff: u64 = 0x64;
    loop {
        while rd::<u8>(param_1 + 0x14) != 0 {
            core::hint::spin_loop();
        }
        if crate::simd_ops::atomic_xchg_u8((param_1 + 0x14) as u64, 1) == 0 {
            break;
        }
        exe::call_va!(0x1_4241_7470, ((&mut backoff as *mut u64) as usize));
    }
    let old_val = crate::simd_ops::atomic_xchg_u64((param_1 + 0x08) as u64, 0);
    let _ = crate::simd_ops::atomic_xchg_u32((param_1 + 0x10) as u64, 0);
    crate::simd_ops::wu8((param_1 + 0x14) as u64, 0); // spinlock release (x86 mov = release)
    old_val
}

/// `unknown_14021d610`
/// Original: 0x14021d610 (1083 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14021d610(
    param_1: usize,
    param_2: usize,
    param_3: i64,
    param_4: u8,
) {
    let mut rbp = rd::<usize>(param_2);
    let sil = param_4;
    let mut r14 = rd::<usize>(param_1);
    let mut r15 = param_3;

    while ((rbp.wrapping_sub(r14) & !3usize) as isize) > 0x80 {
        let diff_bytes = rbp.wrapping_sub(r14) as isize;
        if r15 <= 0 {
            let n = diff_bytes >> 2;
            let mut r8 = n >> 1;
            if r8 > 0 {
                let r11 = (n - 1) >> 1;
                while r8 > 0 {
                    let r10d = rd::<u32>(r14.wrapping_add(((r8 - 1) as usize) * 4));
                    r8 -= 1;
                    let mut rdx = r8;
                    let mut rcx = r8;
                    while rcx < r11 {
                        rcx *= 2;
                        let left = rd::<u32>(r14.wrapping_add((rcx as usize) * 4 + 4));
                        let right = rd::<u32>(r14.wrapping_add((rcx as usize) * 4 + 8));
                        let step: isize = if right < left { 1 } else { 2 };
                        rcx += step;
                        let child_val = rd::<u32>(r14.wrapping_add((rcx as usize) * 4));
                        wr::<u32>(r14.wrapping_add((rdx as usize) * 4), child_val);
                        rdx = rcx;
                    }
                    if rdx == r11 && (n & 1) == 0 {
                        let last_val = rd::<u32>(r14.wrapping_add(((n - 1) as usize) * 4));
                        wr::<u32>(r14.wrapping_add((rdx as usize) * 4), last_val);
                        rdx = n - 1;
                    }
                    let mut rax = (rdx - 1) >> 1;
                    while r8 < rdx {
                        let parent_val = rd::<u32>(r14.wrapping_add((rax as usize) * 4));
                        if parent_val >= r10d {
                            break;
                        }
                        wr::<u32>(r14.wrapping_add((rdx as usize) * 4), parent_val);
                        rdx = rax;
                        rax = (rax - 1) >> 1;
                    }
                    wr::<u32>(r14.wrapping_add((rdx as usize) * 4), r10d);
                }
            }

            let mut rcx_end = rd::<usize>(param_2);
            let rdi_base = rd::<usize>(param_1);
            if ((rcx_end.wrapping_sub(rdi_base) & !3usize) as isize) >= 8 {
                loop {
                    let rbx = rcx_end.wrapping_sub(4);
                    if ((rcx_end.wrapping_sub(rdi_base) & !3usize) as isize) >= 8 {
                        let mut val_slot: u32 = rd::<u32>(rbx);
                        let top_val = rd::<u32>(rdi_base);
                        wr::<u32>(rbx, top_val);
                        let bottom = (rbx.wrapping_sub(rdi_base) as isize) >> 2;
                        let mut first_slot: usize = rdi_base;
                        exe::call_va!(
                            0x1_4022_18e0,
                            (
                                (&mut first_slot as *mut usize) as usize,
                                0i64,
                                bottom as i64,
                                (&mut val_slot as *mut u32) as usize,
                                sil
                            )
                        );
                    }
                    rcx_end = rbx;
                    if ((rbx.wrapping_sub(rdi_base) & !3usize) as isize) < 8 {
                        break;
                    }
                }
            }
            return;
        }

        let mid_idx = diff_bytes >> 3;
        let mut local_first: usize = r14;
        let mut rbx = r14.wrapping_add((mid_idx as usize) * 4);
        let mut local_mid: usize = rbx;
        let mut local_last_m1: usize = rbp.wrapping_sub(4);
        exe::call_va!(
            0x1_4022_19a0,
            (
                (&mut local_first as *mut usize) as usize,
                (&mut local_mid as *mut usize) as usize,
                (&mut local_last_m1 as *mut usize) as usize,
                sil
            )
        );

        let mut rdi = rbx.wrapping_add(4);
        if r14 < rbx {
            let mut pivot = rd::<u32>(rbx);
            loop {
                let prev = rbx.wrapping_sub(4);
                let v = rd::<u32>(prev);
                if v != pivot {
                    break;
                }
                rbx = prev;
                pivot = v;
                if r14 >= prev {
                    break;
                }
            }
        }
        if rdi < rbp {
            let pivot = rd::<u32>(rbx);
            while rdi < rbp {
                if rd::<u32>(rdi) != pivot {
                    break;
                }
                rdi = rdi.wrapping_add(4);
            }
        }

        let mut rax = rdi;
        let mut rcx = rbx;
        loop {
            while rax < rbp {
                let r8d = rd::<u32>(rax);
                let piv = rd::<u32>(rbx);
                if piv < r8d {
                    rax = rax.wrapping_add(4);
                    continue;
                }
                if piv > r8d {
                    break;
                }
                if rdi != rax {
                    let tmp = rd::<u32>(rdi);
                    wr::<u32>(rdi, r8d);
                    wr::<u32>(rax, tmp);
                }
                rdi = rdi.wrapping_add(4);
                rax = rax.wrapping_add(4);
            }

            if rcx > r14 {
                let mut rdx = rcx.wrapping_sub(4);
                loop {
                    let r9d = rd::<u32>(rdx);
                    let piv = rd::<u32>(rbx);
                    if r9d < piv {
                        // continue
                    } else if r9d > piv {
                        break;
                    } else {
                        rbx = rbx.wrapping_sub(4);
                        if rbx != rdx {
                            let tmp = rd::<u32>(rbx);
                            wr::<u32>(rbx, r9d);
                            wr::<u32>(rdx, tmp);
                        }
                    }
                    rcx = rcx.wrapping_sub(4);
                    rdx = rdx.wrapping_sub(4);
                    if r14 >= rcx {
                        break;
                    }
                }
            }

            if rcx == r14 {
                if rax == rbp {
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
                if rax == rbp {
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

        let cur_end = rd::<usize>(param_2);
        let cur_beg = rd::<usize>(param_1);
        let right_bytes = (cur_end.wrapping_sub(rdi) & !3usize) as isize;
        r15 = (r15 >> 1) + (r15 >> 2);
        let left_bytes = (rbx.wrapping_sub(cur_beg) & !3usize) as isize;
        if left_bytes < right_bytes {
            let mut sub_beg = cur_beg;
            let mut sub_end = rbx;
            exe::call_va!(
                0x1_4021_d610,
                (
                    (&mut sub_beg as *mut usize) as usize,
                    (&mut sub_end as *mut usize) as usize,
                    r15,
                    sil
                )
            );
            wr::<usize>(param_1, rdi);
        } else {
            let mut sub_beg = rdi;
            let mut sub_end = cur_end;
            exe::call_va!(
                0x1_4021_d610,
                (
                    (&mut sub_beg as *mut usize) as usize,
                    (&mut sub_end as *mut usize) as usize,
                    r15,
                    sil
                )
            );
            wr::<usize>(param_2, rbx);
        }
        rbp = rd::<usize>(param_2);
        r14 = rd::<usize>(param_1);
    }

    if r14 != rbp {
        let mut rcx = r14.wrapping_add(4);
        while rcx != rbp {
            let r9d = rd::<u32>(rcx);
            let mut rdx = rcx;
            if r9d < rd::<u32>(r14) {
                if r14 != rcx {
                    let mut rax = rcx;
                    let mut r8 = rcx.wrapping_add(4);
                    loop {
                        let edx = rd::<u32>(rax.wrapping_sub(4));
                        r8 = r8.wrapping_sub(4);
                        rax = rax.wrapping_sub(4);
                        wr::<u32>(r8, edx);
                        if rax == r14 {
                            break;
                        }
                    }
                }
                wr::<u32>(r14, r9d);
            } else {
                let mut r8d = rd::<u32>(rcx.wrapping_sub(4));
                let mut rax = rcx.wrapping_sub(4);
                while r9d < r8d {
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
}

/// `unknown_14055a0a0`
/// Original: 0x14055a0a0 (133 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_14055a0a0(param_1: usize, param_2: usize) -> u64 {
    let type_name = rd::<u64>(exe::ptr::<u64>(0x1_4388_1f70) as usize);
    let type_crc = rd::<u32>(exe::ptr::<u32>(0x1_4388_1f78) as usize);
    exe::call_va!(
        0x1_401e_9b60,
        (param_2, param_1, type_name, type_crc, 1u32, 1u32)
    );
    if rd::<i32>(param_2 + 0x328) < 0 {
        if rd::<u8>(param_2 + 0x369) != 0 {
            wr::<u64>(param_1, 0);
        }
    } else {
        let sub = rd::<usize>(param_2 + 0x338);
        let cur = rd::<usize>(sub + 0x120);
        wr::<u64>(param_1, rd::<u64>(cur));
        wr::<usize>(sub + 0x120, cur.wrapping_add(8));
    }
    exe::call_va!(0x1_401e_9a20, (param_2));
    0
}

/// `unknown_140571d70`
/// Original: 0x140571d70 (353 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_140571d70(
    param_1: usize,
    param_2: usize,
    param_3: u64,
) -> usize {
    let cap = rd::<u64>(param_1 + 0x18);
    let size = rd::<u64>(param_1 + 0x10);
    if param_3 <= cap.wrapping_sub(size) {
        wr::<u64>(param_1 + 0x10, size.wrapping_add(param_3));
        let base = if cap <= 0xf {
            param_1
        } else {
            rd::<usize>(param_1)
        };
        let dst = base.wrapping_add(size as usize);
        let _ = exe::call_va!(0x1_4183_acc8, (dst, param_2, param_3) -> usize);
        wr::<u8>(dst.wrapping_add(param_3 as usize), 0);
        return param_1;
    }

    const MAX_STR: u64 = 0x7fff_ffff_ffff_ffff;
    if MAX_STR.wrapping_sub(size) < param_3 {
        exe::call_va!(0x1_4020_0110, ());
        return param_1;
    }

    let new_size = size.wrapping_add(param_3);
    let rcx_cand = new_size | 0xf;
    let mut new_cap = MAX_STR;
    if rcx_cand <= MAX_STR {
        let half = cap >> 1;
        if cap <= MAX_STR.wrapping_sub(half) {
            let grown = half.wrapping_add(cap);
            new_cap = if rcx_cand < grown { grown } else { rcx_cand };
        }
    }

    let new_buf = exe::call_va!(0x1_401e_9110, (new_cap.wrapping_add(1)) -> usize);
    wr::<u64>(param_1 + 0x10, new_size);
    wr::<u64>(param_1 + 0x18, new_cap);
    let dst_tail = new_buf.wrapping_add(size as usize);

    if cap <= 0xf {
        let _ = exe::call_va!(0x1_4183_acbc, (new_buf, param_1, size) -> usize);
        let _ = exe::call_va!(0x1_4183_acbc, (dst_tail, param_2, param_3) -> usize);
        wr::<u8>(dst_tail.wrapping_add(param_3 as usize), 0);
    } else {
        let old_buf = rd::<usize>(param_1);
        let _ = exe::call_va!(0x1_4183_acbc, (new_buf, old_buf, size) -> usize);
        let _ = exe::call_va!(0x1_4183_acbc, (dst_tail, param_2, param_3) -> usize);
        wr::<u8>(dst_tail.wrapping_add(param_3 as usize), 0);
        let mut free_ptr = old_buf;
        if cap.wrapping_add(1) >= 0x1000 {
            let raw_ptr = rd::<usize>(old_buf.wrapping_sub(8));
            if old_buf.wrapping_sub(raw_ptr).wrapping_sub(8) > 0x1f {
                exe::call_iat!(0x1_4264_67f0, ());
                return param_1;
            }
            free_ptr = raw_ptr;
        }
        exe::call_va!(0x1_4183_4d30, (free_ptr));
    }

    wr::<usize>(param_1, new_buf);
    param_1
}

/// `r_14059bfa0`
/// Original: 0x14059bfa0 (555 bytes, Throws::No)
pub unsafe extern "win64" fn r_14059bfa0(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let world = rd::<usize>(param_1);
        let id_d10 = rd::<u16>(exe::ptr::<u16>(0x1_43cb_5d10) as usize) as usize;
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let v0 = rd::<usize>(world);
        let world_plus_40 = world.wrapping_add(0x40);
        let lut = rd::<usize>(v0 + 0x20);
        let idx_d10 = rd::<u16>(lut.wrapping_add(id_d10 * 2)) as usize;
        let entry_d10 = rd::<usize>(world + 0x08).wrapping_add(idx_d10 * 16);
        let len_d10 = rd::<u32>(entry_d10 + 0x08) as u64;
        let ptr_d10 = rd::<u64>(entry_d10);

        let id_110 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d110) as usize) as usize;
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_110 = rd::<u64>(tbl_580e0.wrapping_add(id_110 * 8));

        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr) as usize;
            let block = rd::<usize>(world_plus_40.wrapping_add(arch * 8).wrapping_add(0x48));
            let ent_count = if block != 0 {
                rd::<u32>(world_plus_40.wrapping_add(arch * 4).wrapping_add(0x10048)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_fe8 = rd::<u16>(exe::ptr::<u16>(0x1_43c9_afe8) as usize) as u32;
                let arch_off = arch.wrapping_shl(5);
                let search_len =
                    rd::<u32>(world_plus_40.wrapping_add(arch_off).wrapping_add(0x1806c));
                let search_ptr =
                    rd::<u64>(world_plus_40.wrapping_add(arch_off).wrapping_add(0x18058));

                let found1 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_fe8) -> i32);
                let off_slot = world_plus_40.wrapping_add(arch.wrapping_add(0xc03).wrapping_shl(5));
                let mut off1 = if (found1 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found1 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_358 = rd::<u16>(exe::ptr::<u16>(0x1_43cb_5358) as usize) as u32;
                let found2 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_358) -> i32);
                let mut off2 = if (found2 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found2 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                if off1 != 0 {
                    off1 = off1.wrapping_add(block);
                }
                if off2 != 0 {
                    off2 = off2.wrapping_add(block);
                }

                for idx in 0..ent_count {
                    let mut pkt: [u64; 4] = [off1 as u64, off2 as u64, block as u64, idx];
                    let mut arg2: [u64; 3] = [world as u64, len_d10, ptr_d10];
                    exe::call_va!(
                        0x1_41ac_b750,
                        (
                            pkt.as_mut_ptr() as usize,
                            arg2.as_mut_ptr() as usize,
                            val_110
                        )
                    );
                }
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `content_1408b4ad0`
/// Original: 0x1408b4ad0 (179 bytes, Throws::No)
pub unsafe extern "win64" fn content_1408b4ad0(param_1: usize, param_2: u64) {
    let old_len = rd::<u32>(param_1 + 0x08) as u64;
    let old_cap = rd::<u32>(param_1 + 0x0c) as u64;
    if param_2 > old_cap {
        let grown = (old_cap >> 1).wrapping_add(old_cap);
        let new_cap = if grown < param_2 { param_2 } else { grown };
        exe::call_va!(0x1_401d_eba0, (param_1, new_cap));
        let diff = param_2.wrapping_sub(old_len);
        if diff != 0 {
            let cur_len = rd::<u32>(param_1 + 0x08) as usize;
            let dst = rd::<usize>(param_1).wrapping_add(cur_len * 8);
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff.wrapping_mul(8)) -> usize);
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 > old_len {
        let diff = param_2.wrapping_sub(old_len);
        if diff != 0 {
            let dst = rd::<usize>(param_1).wrapping_add((old_len as usize) * 8);
            let _ = exe::call_va!(0x1_4183_acc2, (dst, 0u32, diff.wrapping_mul(8)) -> usize);
        }
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    } else if param_2 != old_len {
        wr::<u32>(param_1 + 0x08, param_2 as u32);
    }
}

/// `unknown_140a6e980`
/// Original: 0x140a6e980 (138 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140a6e980(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mut src = param_1;
    let mut dst = param_3;
    while src != param_2 {
        wr::<u64>(dst, rd::<u64>(src));
        wr::<u32>(dst + 0x08, rd::<u32>(src + 0x08));
        wr::<u64>(dst + 0x0c, rd::<u64>(src + 0x0c));
        wr::<u32>(dst + 0x14, rd::<u32>(src + 0x14));
        wr::<u64>(dst + 0x18, rd::<u64>(src + 0x18));
        wr::<u32>(dst + 0x20, rd::<u32>(src + 0x20));
        wr::<u64>(dst + 0x24, rd::<u64>(src + 0x24));
        wr::<u32>(dst + 0x2c, rd::<u32>(src + 0x2c));
        src = src.wrapping_add(0x30);
        dst = dst.wrapping_add(0x30);
    }
    dst
}

/// `unknown_140f79560`
/// Original: 0x140f79560 (551 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f79560(param_1: usize, param_2: usize, param_3: u16) {
    let span_base = rd::<usize>(param_2).wrapping_add((param_3 as usize).wrapping_mul(0x30));
    let count = rd::<u32>(span_base + 0x08) as usize;
    let mut cur_ptr = rd::<usize>(span_base);
    if count != 0 {
        let id_63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
        let end_ptr = cur_ptr.wrapping_add(count * 2);
        let world = rd::<usize>(param_1);
        let tbl_580e0 = rd::<usize>(world + 0x580e0);
        let val_63c = rd::<u64>(tbl_580e0.wrapping_add(id_63c * 8));

        let mut pkt = [0u64; 7];
        while cur_ptr != end_ptr {
            let arch = rd::<u16>(cur_ptr);
            let arch_usize = arch as usize;
            let block = rd::<usize>(world.wrapping_add(arch_usize * 8).wrapping_add(0x88));
            let ent_count = if block != 0 {
                rd::<u32>(world.wrapping_add(arch_usize * 4).wrapping_add(0x10088)) as u64
            } else {
                0
            };
            if ent_count != 0 {
                let want_d00 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_6d00) as usize) as u32;
                let arch_off = arch_usize.wrapping_shl(5);
                let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac));
                let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
                let off_slot = world
                    .wrapping_add(arch_usize.wrapping_add(0xc03).wrapping_shl(5))
                    .wrapping_add(0x40);

                let found1 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_d00) -> i32);
                let mut off1 = if (found1 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found1 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_6a4 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_76a4) as usize) as u32;
                let found2 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_6a4) -> i32);
                let mut off2 = if (found2 as isize) != -1 {
                    let tbl = rd::<usize>(off_slot);
                    rd::<u32>(tbl.wrapping_add((found2 as isize as usize) * 4)) as usize
                } else {
                    0
                };

                let want_120 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d120) as usize) as u32;
                let found3 =
                    exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_120) -> i32);
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

                pkt[0] = (pkt[0] & !0xffffu64) | (arch as u64);
                pkt[1] = block as u64;
                pkt[2] = world as u64;
                pkt[3] = ent_count;
                pkt[4] = off1 as u64;
                pkt[5] = off2 as u64;
                pkt[6] = off3 as u64;
                exe::call_va!(0x1_41e1_f040, (pkt.as_mut_ptr() as usize, val_63c));
            }
            cur_ptr = cur_ptr.wrapping_add(2);
        }
    }
}

/// `unknown_141001050`
/// Original: 0x141001050 (166 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_141001050(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let ent_count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088)) as u64
    } else {
        0
    };
    if ent_count != 0 {
        let mut buf1 = [0u64; 16];
        let _ = exe::call_va!(
            0x1_4100_13c0,
            (
                buf1.as_mut_ptr() as usize,
                param_2 as u64,
                block as u64,
                ent_count,
                world as u64
            ) -> usize
        );
        let mut buf2 = buf1;
        let mut world_slot: u64 = world as u64;
        exe::call_va!(
            0x1_4100_1100,
            (
                0usize,
                buf2.as_mut_ptr() as usize,
                (&mut world_slot as *mut u64) as usize
            )
        );
    }
}

/// `unknown_14101f7d0`
/// Original: 0x14101f7d0 (174 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14101f7d0(param_1: usize, param_2: usize) -> usize {
    wr::<u32>(param_1, 0x0200_0002);
    wr::<u32>(param_1 + 0x04, 0);
    wr::<u8>(param_1 + 0x08, 0);
    let _ = exe::call_va!(0x1_4240_a730, (param_1, param_2) -> usize);

    wr::<u64>(param_1 + 0x18, rd::<u64>(param_2 + 0x18));

    let v20 = rd::<u64>(param_2 + 0x20);
    wr::<u64>(param_2 + 0x20, 0);
    wr::<u64>(param_1 + 0x20, v20);

    let v28 = rd::<u64>(param_2 + 0x28);
    wr::<u64>(param_2 + 0x28, 0);
    wr::<u64>(param_1 + 0x28, v28);

    let v30 = rd::<u64>(param_2 + 0x30);
    wr::<u64>(param_2 + 0x30, 0);
    wr::<u64>(param_1 + 0x30, v30);

    let v38 = rd::<u64>(param_2 + 0x38);
    wr::<u64>(param_2 + 0x38, 0);
    wr::<u64>(param_1 + 0x38, v38);

    let v40 = rd::<u16>(param_2 + 0x40);
    wr::<u16>(param_2 + 0x40, 0x10);
    wr::<u16>(param_1 + 0x40, v40);

    let v48 = rd::<u64>(param_2 + 0x48);
    wr::<u64>(param_2 + 0x48, 0);
    wr::<u64>(param_1 + 0x48, v48);

    wr::<u32>(param_1 + 0x50, rd::<u32>(param_2 + 0x50));
    wr::<u64>(param_1 + 0x58, rd::<u64>(param_2 + 0x58));
    param_1
}

/// `coregame_14108aa30`
/// Original: 0x14108aa30 (569 bytes, Throws::No)
pub unsafe extern "win64" fn coregame_14108aa30(param_1: u64, param_2: usize) -> u64 {
    const FNV_PRIME: u64 = 0x0100_0000_01b3;
    const FNV_BASIS: u64 = 0xcbf2_9ce4_8422_2325;
    const GOLDEN: u64 = 0x9e37_79b9;

    let mut rdi = exe::call_va!(0x1_4108_4400, (param_1, param_2) -> u64);
    let mut rcx_in = rdi;
    for off in [0x0cusize, 0x18, 0x24] {
        let v = exe::call_va!(0x1_4108_4400, (rcx_in, param_2.wrapping_add(off)) -> u64);
        let mut h = FNV_BASIS;
        for &b in &v.to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(FNV_PRIME);
        }
        let mix = h
            .wrapping_add(rdi.wrapping_shl(6))
            .wrapping_add((rdi >> 2).wrapping_add(GOLDEN));
        rcx_in = mix;
        rdi ^= mix;
    }
    rdi
}

/// `coregame_14109b600`
/// Original: 0x14109b600 (248 bytes, Throws::No)
pub unsafe extern "win64" fn coregame_14109b600(param_1: usize, param_2: u16) {
    let world = rd::<usize>(param_1);
    let arch = param_2 as usize;
    let block = rd::<usize>(world.wrapping_add(arch * 8).wrapping_add(0x88));
    let ent_count = if block != 0 {
        rd::<u32>(world.wrapping_add(arch * 4).wrapping_add(0x10088)) as u64
    } else {
        0
    };
    if ent_count != 0 {
        let want_e6c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_de6c) as usize) as u32;
        let arch_off = arch.wrapping_shl(5);
        let search_len = rd::<u32>(world.wrapping_add(arch_off).wrapping_add(0x180ac));
        let search_ptr = rd::<u64>(world.wrapping_add(arch_off).wrapping_add(0x18098));
        let found = exe::call_va!(0x1_4248_9ee0, (search_ptr, search_len, want_e6c) -> i32);
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
        for idx in 0..ent_count {
            let mut pkt: [u64; 3] = [comp_off as u64, block as u64, idx];
            exe::call_va!(0x1_41f2_d380, (pkt.as_mut_ptr() as usize, world as u64));
        }
    }
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_unknown_1401e8620(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, init_val: u64, init_cnt: u32| -> (u64, [u8; 24]) {
        let mut obj = [0u8; 24];
        let p = obj.as_mut_ptr() as usize;
        wr::<u64>(p + 0x08, init_val);
        wr::<u32>(p + 0x10, init_cnt);
        wr::<u8>(p + 0x14, 0);
        let ret = f(p);
        (ret, obj)
    };

    for (v, c) in [(0x1234_5678_9abc_def0u64, 7u32), (0, 0)] {
        let o = run(f_o, v, c);
        let n = run(f_n, v, c);
        crate::diff_eq!(o, n, "1401e8620 v={:#x} c={}", v, c);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14021d610(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i64, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, len: usize, ideal: i64| -> (Vec<u32>, usize, usize) {
        let mut data = Vec::with_capacity(len);
        let mut s: u32 = 0x9e37_79b9;
        for i in 0..len {
            s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            // Include duplicate values to exercise equal-to-pivot three-way partitioning
            let val = if i % 5 == 0 { 500 } else { s % 1000 };
            data.push(val);
        }
        let base = data.as_mut_ptr() as usize;
        let mut p1: usize = base;
        let mut p2: usize = base.wrapping_add(len * 4);
        f(
            (&mut p1 as *mut usize) as usize,
            (&mut p2 as *mut usize) as usize,
            ideal,
            0,
        );
        (data, p1.wrapping_sub(base), p2.wrapping_sub(base))
    };

    for (len, ideal) in [(16usize, 16i64), (48, 0), (64, 64), (96, 4)] {
        let o = run(f_o, len, ideal);
        let n = run(f_n, len, ideal);
        crate::diff_eq!(o, n, "14021d610 len={} ideal={}", len, ideal);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_14055a0a0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static B60_LOG: AtomicU64 = AtomicU64::new(0);
    static A20_CALLED: AtomicU32 = AtomicU32::new(0);

    unsafe extern "win64" fn mock_9b60(
        _stream: usize,
        _out: usize,
        type_name: u64,
        type_crc: u32,
        a5: u32,
        a6: u32,
    ) {
        let v = type_name
            .wrapping_add(type_crc as u64)
            .wrapping_add(a5 as u64)
            .wrapping_add((a6 as u64) << 8);
        B60_LOG.store(v, Ordering::Relaxed);
    }
    unsafe extern "win64" fn mock_9a20(_stream: usize) {
        A20_CALLED.fetch_add(1, Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9b60, mock_9b60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_9a20, mock_9a20 as *const ())?;

    let run = |f: F, status: i32, clear_flag: u8| -> (u64, u64, usize, u64, u32) {
        B60_LOG.store(0, Ordering::Relaxed);
        A20_CALLED.store(0, Ordering::Relaxed);
        let stream_vals: [u64; 2] = [0xfeed_face_1234_5678, 0x9999];
        let mut sub_obj = [0u8; 0x130];
        let sp = sub_obj.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x120, stream_vals.as_ptr() as usize);

        let mut p2 = [0u8; 0x370];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<i32>(p2p + 0x328, status);
        wr::<usize>(p2p + 0x338, sp);
        wr::<u8>(p2p + 0x369, clear_flag);

        let mut out_val: u64 = 0xaaaa_bbbb_cccc_dddd;
        let ret = f((&mut out_val as *mut u64) as usize, p2p);
        let ptr_off = rd::<usize>(sp + 0x120).wrapping_sub(stream_vals.as_ptr() as usize);
        (
            ret,
            out_val,
            ptr_off,
            B60_LOG.load(Ordering::Relaxed),
            A20_CALLED.load(Ordering::Relaxed),
        )
    };

    for (status, clear_flag) in [(0i32, 0u8), (-1, 1), (-1, 0)] {
        let o = run(f_o, status, clear_flag);
        let n = run(f_n, status, clear_flag);
        crate::diff_eq!(o, n, "14055a0a0 status={} clear={}", status, clear_flag);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140571d70(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static ALLOC_PTR: AtomicUsize = AtomicUsize::new(0);
    static LAST_ALLOC_REQ: AtomicU64 = AtomicU64::new(0);
    static LAST_FREED_OFF: AtomicUsize = AtomicUsize::new(0);

    unsafe extern "win64" fn mock_alloc(bytes: u64) -> usize {
        LAST_ALLOC_REQ.store(bytes, Ordering::Relaxed);
        ALLOC_PTR.load(Ordering::Relaxed)
    }
    unsafe extern "win64" fn mock_free(ptr: usize) {
        LAST_FREED_OFF.store(ptr, Ordering::Relaxed);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_9110, mock_alloc as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4183_4d30, mock_free as *const ())?;

    let run = |f: F, mode: u32| -> (u64, u64, Vec<u8>, u64, usize) {
        let mut alloc_buf = [0xeeu8; 0x1100];
        ALLOC_PTR.store(alloc_buf.as_mut_ptr() as usize, Ordering::Relaxed);
        LAST_ALLOC_REQ.store(0, Ordering::Relaxed);
        LAST_FREED_OFF.store(0, Ordering::Relaxed);

        let mut old_heap = [0u8; 0x1040];
        let ohp = old_heap.as_mut_ptr() as usize;
        let append_src = *b"0123456789abcdef";

        let mut s = [0u8; 32];
        let sp = s.as_mut_ptr() as usize;

        let (append_len, base_for_free) = match mode {
            0 => {
                // In-place SSO append
                s[..4].copy_from_slice(b"ABCD");
                wr::<u64>(sp + 0x10, 4);
                wr::<u64>(sp + 0x18, 15);
                (5u64, 0usize)
            }
            1 => {
                // In-place heap append
                old_heap[..6].copy_from_slice(b"ABCDEF");
                wr::<usize>(sp, ohp);
                wr::<u64>(sp + 0x10, 6);
                wr::<u64>(sp + 0x18, 31);
                (6u64, 0usize)
            }
            2 => {
                // Realloc from SSO
                s[..12].copy_from_slice(b"ABCDEFGHIJKL");
                wr::<u64>(sp + 0x10, 12);
                wr::<u64>(sp + 0x18, 15);
                (8u64, 0usize)
            }
            3 => {
                // Realloc from small heap (cap + 1 < 0x1000)
                old_heap[..26].copy_from_slice(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ");
                wr::<usize>(sp, ohp);
                wr::<u64>(sp + 0x10, 26);
                wr::<u64>(sp + 0x18, 31);
                (10u64, ohp)
            }
            _ => {
                // Realloc from large aligned heap (cap + 1 >= 0x1000)
                let user_ptr = ohp + 0x18;
                wr::<usize>(user_ptr - 8, ohp);
                for i in 0..0xff8usize {
                    wr::<u8>(user_ptr + i, b'A' + ((i % 26) as u8));
                }
                wr::<usize>(sp, user_ptr);
                wr::<u64>(sp + 0x10, 0xff8);
                wr::<u64>(sp + 0x18, 0x1000);
                (12u64, ohp)
            }
        };

        let ret = f(sp, append_src.as_ptr() as usize, append_len);
        assert_eq!(ret, sp);

        let new_len = rd::<u64>(sp + 0x10);
        let new_cap = rd::<u64>(sp + 0x18);
        let data_ptr = if new_cap <= 15 { sp } else { rd::<usize>(sp) };
        let mut out_bytes = vec![0u8; (new_len as usize) + 1];
        for (i, b) in out_bytes.iter_mut().enumerate() {
            *b = rd::<u8>(data_ptr + i);
        }
        let raw_freed = LAST_FREED_OFF.load(Ordering::Relaxed);
        let freed_rel = if base_for_free != 0 {
            raw_freed.wrapping_sub(base_for_free) + 1
        } else {
            raw_freed
        };
        (
            new_len,
            new_cap,
            out_bytes,
            LAST_ALLOC_REQ.load(Ordering::Relaxed),
            freed_rel,
        )
    };

    for mode in 0..5 {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "140571d70 mode={}", mode);
    }
    Ok(5)
}

pub unsafe fn diff_r_14059bfa0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x5550 && n == 6 {
            match want {
                13 => 0,
                29 => 1,
                _ => -1,
            }
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_b750(pkt_ptr: usize, arg2_ptr: usize, r8_110: u64) {
        let pkt = rd::<[u64; 4]>(pkt_ptr);
        let mut a2 = rd::<[u64; 3]>(arg2_ptr);
        let world = a2[0] as usize;
        a2[0] = 0;
        let idx = pkt[3] as usize;
        let dst = world + 0x200 + idx * (8 * 8);
        wr::<[u64; 4]>(dst, pkt);
        wr::<[u64; 3]>(dst + 32, a2);
        wr::<u64>(dst + 56, r8_110);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41ac_b750, mock_b750 as *const ())?;

    let g_d10 = exe::ptr::<u16>(0x1_43cb_5d10) as usize;
    let g_110 = exe::ptr::<u16>(0x1_43c8_d110) as usize;
    let g_fe8 = exe::ptr::<u16>(0x1_43c9_afe8) as usize;
    let g_358 = exe::ptr::<u16>(0x1_43cb_5358) as usize;
    let saved = (
        rd::<u16>(g_d10),
        rd::<u16>(g_110),
        rd::<u16>(g_fe8),
        rd::<u16>(g_358),
    );
    wr::<u16>(g_d10, 1);
    wr::<u16>(g_110, 1);
    wr::<u16>(g_fe8, 13);
    wr::<u16>(g_358, 29);

    let run = |f: F| -> [u64; 16] {
        let lut: [u16; 4] = [0, 1, 0, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x30];
        let ap = arr_08.as_mut_ptr() as usize;
        wr::<u64>(ap + 0x10, 0xabcd_1234);
        wr::<u32>(ap + 0x18, 9);

        let tbl_580e0: [u64; 2] = [0, 0x7777_8888];
        let off_tbl: [u32; 2] = [0x120, 0x360];

        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, ap);
        wr::<usize>(wp + 0x88 + 1 * 8, 0x40_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);
        wr::<u64>(wp + 0x40 + 0x20 + 0x18058, 0x5550);
        wr::<u32>(wp + 0x40 + 0x20 + 0x1806c, 6);
        wr::<usize>(wp + 0x40 + (1 + 0xc03) * 0x20, off_tbl.as_ptr() as usize);
        wr::<usize>(wp + 0x580e0, tbl_580e0.as_ptr() as usize);

        let arch_list: [u16; 2] = [0, 1];
        let mut span_arr = [0u8; 0x60];
        let sp = span_arr.as_mut_ptr() as usize;
        wr::<usize>(sp + 0x30, arch_list.as_ptr() as usize);
        wr::<u32>(sp + 0x38, 2);

        let p1 = [wp];
        let p2 = [sp];
        f(p1.as_ptr() as usize, p2.as_ptr() as usize, 1);
        rd::<[u64; 16]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_d10, saved.0);
    wr::<u16>(g_110, saved.1);
    wr::<u16>(g_fe8, saved.2);
    wr::<u16>(g_358, saved.3);

    crate::diff_eq!(o, n, "14059bfa0 out");
    Ok(1)
}

pub unsafe fn diff_content_1408b4ad0(orig: usize, new: usize) -> Result<u32, String> {
    use core::sync::atomic::{AtomicU64, Ordering};
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static GROW_CAP: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn mock_eba0(vec_ptr: usize, new_cap: u64) {
        GROW_CAP.store(new_cap, Ordering::Relaxed);
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }

    let _m1 = crate::difftest::Mock::new(0x1_401d_eba0, mock_eba0 as *const ())?;

    let run = |f: F, init_len: u32, init_cap: u32, req_len: u64| -> (u32, u32, u64, [u64; 12]) {
        GROW_CAP.store(0, Ordering::Relaxed);
        let mut data = [0xdead_beef_cafe_babeu64; 12];
        let mut hdr = [0u8; 16];
        let hp = hdr.as_mut_ptr() as usize;
        wr::<usize>(hp, data.as_mut_ptr() as usize);
        wr::<u32>(hp + 0x08, init_len);
        wr::<u32>(hp + 0x0c, init_cap);

        f(hp, req_len);
        (
            rd::<u32>(hp + 0x08),
            rd::<u32>(hp + 0x0c),
            GROW_CAP.load(Ordering::Relaxed),
            data,
        )
    };

    for (len, cap, req) in [(2u32, 4u32, 8u64), (2, 8, 5), (6, 8, 3), (4, 8, 4)] {
        let o = run(f_o, len, cap, req);
        let n = run(f_n, len, cap, req);
        crate::diff_eq!(o, n, "1408b4ad0 len={} cap={} req={}", len, cap, req);
    }
    Ok(4)
}

pub unsafe fn diff_unknown_140a6e980(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, count: usize| -> ([u32; 36], usize) {
        let mut src = [0u32; 36];
        for (i, v) in src.iter_mut().enumerate() {
            *v = 0x1000_0000 + (i as u32) * 0x1111;
        }
        let mut dst = [0xeeee_ffffu32; 36];
        let sp = src.as_ptr() as usize;
        let dp = dst.as_mut_ptr() as usize;
        let ret = f(sp, sp + count * 0x30, dp);
        (dst, ret.wrapping_sub(dp))
    };

    for cnt in [0usize, 1, 3] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        crate::diff_eq!(o, n, "140a6e980 cnt={}", cnt);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140f79560(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x7770 && n == 5 {
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
    unsafe extern "win64" fn mock_f040(pkt_ptr: usize, rdx_63c: u64) {
        let mut pkt = rd::<[u64; 7]>(pkt_ptr);
        let world = pkt[2] as usize;
        pkt[0] &= 0xffff;
        pkt[2] = 0;
        let dst = world + 0x200;
        wr::<[u64; 7]>(dst, pkt);
        wr::<u64>(dst + 56, rdx_63c);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41e1_f040, mock_f040 as *const ())?;

    let g_63c = exe::ptr::<u16>(0x1_43c8_f63c) as usize;
    let g_d00 = exe::ptr::<u16>(0x1_43cc_6d00) as usize;
    let g_6a4 = exe::ptr::<u16>(0x1_43cc_76a4) as usize;
    let g_120 = exe::ptr::<u16>(0x1_43c8_d120) as usize;
    let saved = (
        rd::<u16>(g_63c),
        rd::<u16>(g_d00),
        rd::<u16>(g_6a4),
        rd::<u16>(g_120),
    );
    wr::<u16>(g_63c, 1);
    wr::<u16>(g_d00, 10);
    wr::<u16>(g_6a4, 20);
    wr::<u16>(g_120, 30);

    let run = |f: F| -> [u64; 8] {
        let tbl_580e0: [u64; 2] = [0, 0x1234_5678_9abc];
        let off_tbl: [u32; 3] = [0x100, 0x200, 0x300];

        let mut world = vec![0u8; 0x58200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 1 * 8, 0x80_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 4);
        wr::<u64>(wp + 0x18098 + 0x20, 0x7770);
        wr::<u32>(wp + 0x180ac + 0x20, 5);
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
        rd::<[u64; 8]>(wp + 0x200)
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_63c, saved.0);
    wr::<u16>(g_d00, saved.1);
    wr::<u16>(g_6a4, saved.2);
    wr::<u16>(g_120, saved.3);

    crate::diff_eq!(o, n, "140f79560 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141001050(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_13c0(
        out_buf: usize,
        arch: u16,
        block: u64,
        cnt: u64,
        world: usize,
    ) -> usize {
        for i in 0..16usize {
            wr::<u64>(
                out_buf + i * 8,
                block
                    .wrapping_add((arch as u64) << 16)
                    .wrapping_add(cnt * 0x10)
                    .wrapping_add(i as u64)
                    .wrapping_add((world & 0xff) as u64),
            );
        }
        out_buf
    }
    unsafe extern "win64" fn mock_1100(_rcx: usize, rdx_buf: usize, r8_world_ptr: usize) {
        let world = rd::<usize>(r8_world_ptr);
        let vals = rd::<[u64; 16]>(rdx_buf);
        wr::<[u64; 16]>(world + 0x200, vals);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4100_13c0, mock_13c0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4100_1100, mock_1100 as *const ())?;

    let run = |f: F, arch: u16| -> [u64; 16] {
        let mut world = vec![0u8; 0x10200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 2 * 8, 0x90_0000);
        wr::<u32>(wp + 0x10088 + 2 * 4, 7);

        let p1 = [wp];
        f(p1.as_ptr() as usize, arch);
        let mut out = rd::<[u64; 16]>(wp + 0x200);
        if arch == 2 {
            for v in &mut out {
                *v = v.wrapping_sub((wp & 0xff) as u64);
            }
        }
        out
    };

    for arch in [1u16, 2u16] {
        let o = run(f_o, arch);
        let n = run(f_n, arch);
        crate::diff_eq!(o, n, "141001050 arch={}", arch);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14101f7d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_a730(dst: usize, src: usize) -> usize {
        // Record the pre-initialized RString header bytes at dst into dst + 0x08..0x18
        let h0 = rd::<u32>(dst);
        let h4 = rd::<u32>(dst + 0x04);
        let h8 = rd::<u8>(dst + 0x08) as u64;
        wr::<u64>(dst + 0x08, ((h0 as u64) << 32) | (h4 as u64) | (h8 << 24));
        wr::<u64>(dst + 0x10, rd::<u64>(src + 0x08));
        dst
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_a730, mock_a730 as *const ())?;

    let run = |f: F| -> ([u8; 0x60], [u8; 0x60]) {
        let mut dst = [0xffu8; 0x60];
        let mut src = [0u8; 0x60];
        let dp = dst.as_mut_ptr() as usize;
        let sp = src.as_mut_ptr() as usize;
        wr::<u64>(sp + 0x08, 0x1111_2222_3333_4444);
        wr::<u64>(sp + 0x18, 0xaaaa_0018);
        wr::<u64>(sp + 0x20, 0xaaaa_0020);
        wr::<u64>(sp + 0x28, 0xaaaa_0028);
        wr::<u64>(sp + 0x30, 0xaaaa_0030);
        wr::<u64>(sp + 0x38, 0xaaaa_0038);
        wr::<u16>(sp + 0x40, 0x7788);
        wr::<u64>(sp + 0x48, 0xaaaa_0048);
        wr::<u32>(sp + 0x50, 0x1234_5678);
        wr::<u64>(sp + 0x58, 0xaaaa_0058);

        let ret = f(dp, sp);
        assert_eq!(ret, dp);
        (dst, src)
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14101f7d0 out");
    Ok(1)
}

pub unsafe fn diff_coregame_14108aa30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u64, usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let mat1: [f32; 12] = [
        1.0, -0.0, 2.5, -3.25, 0.0, 4.0, 100.5, -200.25, 0.125, 7.0, 8.0, 9.0,
    ];
    let mat2: [f32; 12] = [
        -1.5, 2.0, -3.0, 4.5, -5.0, 6.0, -7.5, 8.0, -9.0, 10.5, -11.0, 12.0,
    ];

    for mat in [&mat1, &mat2] {
        let p = mat.as_ptr() as usize;
        let o = f_o(0, p);
        let n = f_n(0, p);
        crate::diff_eq!(o, n, "14108aa30");
    }
    Ok(2)
}

pub unsafe fn diff_coregame_14109b600(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_find(p: u64, n: u32, want: u16) -> i32 {
        if p == 0x6660 && n == 4 && want == 42 {
            1
        } else {
            -1
        }
    }
    unsafe extern "win64" fn mock_d380(pkt_ptr: usize, rdx_world: usize) {
        let pkt = rd::<[u64; 3]>(pkt_ptr);
        let idx = pkt[2] as usize;
        let dst = rdx_world + 0x200 + idx * (3 * 8);
        wr::<[u64; 3]>(dst, pkt);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_9ee0, mock_find as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f2_d380, mock_d380 as *const ())?;

    let g_e6c = exe::ptr::<u16>(0x1_43c8_de6c) as usize;
    let s_e6c = rd::<u16>(g_e6c);
    wr::<u16>(g_e6c, 42);

    let run = |f: F, arch: u16| -> [u64; 6] {
        let off_tbl: [u32; 2] = [0x20, 0x180];
        let mut world = vec![0u8; 0x30200];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp + 0x88 + 1 * 8, 0x30_0000);
        wr::<u32>(wp + 0x10088 + 1 * 4, 2);
        wr::<u64>(wp + 0x18098 + 0x20, 0x6660);
        wr::<u32>(wp + 0x180ac + 0x20, 4);
        wr::<usize>(wp + (1 + 0xc03) * 0x20 + 0x40, off_tbl.as_ptr() as usize);

        let p1 = [wp];
        f(p1.as_ptr() as usize, arch);
        rd::<[u64; 6]>(wp + 0x200)
    };

    for arch in [0u16, 1u16] {
        let o = run(f_o, arch);
        let n = run(f_n, arch);
        wr::<u16>(g_e6c, s_e6c);
        wr::<u16>(g_e6c, 42);
        crate::diff_eq!(o, n, "14109b600 arch={}", arch);
    }
    wr::<u16>(g_e6c, s_e6c);
    Ok(2)
}

// REGISTRY:
// Replacement::verified(0x1_401e_8620, "unknown_1401e8620", rend::cauldron_hot_w1::unknown_1401e8620 as *const (), rend::cauldron_hot_w1::diff_unknown_1401e8620),
// Replacement::verified(0x1_4021_d610, "unknown_14021d610", rend::cauldron_hot_w1::unknown_14021d610 as *const (), rend::cauldron_hot_w1::diff_unknown_14021d610),
// Replacement::verified_throws(0x1_4055_a0a0, "unknown_14055a0a0", rend::cauldron_hot_w1::unknown_14055a0a0 as *const (), Throws::Discard, rend::cauldron_hot_w1::diff_unknown_14055a0a0),
// Replacement::verified_throws(0x1_4057_1d70, "unknown_140571d70", rend::cauldron_hot_w1::unknown_140571d70 as *const (), Throws::Discard, rend::cauldron_hot_w1::diff_unknown_140571d70),
// Replacement::verified(0x1_4059_bfa0, "r_14059bfa0", rend::cauldron_hot_w1::r_14059bfa0 as *const (), rend::cauldron_hot_w1::diff_r_14059bfa0),
// Replacement::verified(0x1_408b_4ad0, "content_1408b4ad0", rend::cauldron_hot_w1::content_1408b4ad0 as *const (), rend::cauldron_hot_w1::diff_content_1408b4ad0),
// Replacement::verified(0x1_40a6_e980, "unknown_140a6e980", rend::cauldron_hot_w1::unknown_140a6e980 as *const (), rend::cauldron_hot_w1::diff_unknown_140a6e980),
// Replacement::verified(0x1_40f7_9560, "unknown_140f79560", rend::cauldron_hot_w1::unknown_140f79560 as *const (), rend::cauldron_hot_w1::diff_unknown_140f79560),
// Replacement::verified_throws(0x1_4100_1050, "unknown_141001050", rend::cauldron_hot_w1::unknown_141001050 as *const (), Throws::Discard, rend::cauldron_hot_w1::diff_unknown_141001050),
// Replacement::verified(0x1_4101_f7d0, "unknown_14101f7d0", rend::cauldron_hot_w1::unknown_14101f7d0 as *const (), rend::cauldron_hot_w1::diff_unknown_14101f7d0),
// Replacement::verified(0x1_4108_aa30, "coregame_14108aa30", rend::cauldron_hot_w1::coregame_14108aa30 as *const (), rend::cauldron_hot_w1::diff_coregame_14108aa30),
// Replacement::verified(0x1_4109_b600, "coregame_14109b600", rend::cauldron_hot_w1::coregame_14109b600 as *const (), rend::cauldron_hot_w1::diff_coregame_14109b600),
