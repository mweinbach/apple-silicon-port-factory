//! Decompiled hot functions shard ZA03B (`0x140f8cf30..0x140fb7530`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

#[inline(always)]
fn sdiv_0x30(diff: i64) -> i64 {
    let hi = ((diff as i128) * (0x2aaa_aaaa_aaaa_aaabi64 as i128)) >> 64;
    let sar = (hi as i64) >> 3;
    sar.wrapping_add(((sar as u64) >> 63) as i64)
}

#[inline(always)]
unsafe fn free_custom_block(ptr: usize) {
    let r10 = ptr.wrapping_sub(1) & !0x01ff_ffffusize;
    let seg_idx = (ptr.wrapping_sub(r10)) >> 16;
    let page = r10.wrapping_add(0x108).wrapping_add(seg_idx.wrapping_mul(0x50));
    let rdx_hdr = page.wrapping_sub(rd::<u32>(page + 0x04) as usize);
    if exe::teb() == rd::<usize>(r10 + 0x100) {
        if rd::<u8>(rdx_hdr + 0x0e) == 0 {
            wr::<u64>(ptr, rd::<u64>(rdx_hdr + 0x20));
            let used = rd::<u32>(rdx_hdr + 0x18).wrapping_sub(1);
            wr::<u32>(rdx_hdr + 0x18, used);
            wr::<usize>(rdx_hdr + 0x20, ptr);
            if used == 0 {
                exe::call_va!(0x1_4253_71a0, (rdx_hdr));
            }
            return;
        }
        exe::call_va!(0x1_4253_3200, (r10, rdx_hdr, 1u8, ptr));
    } else {
        exe::call_va!(0x1_4253_3200, (r10, rdx_hdr, 0u8, ptr));
    }
}

/// `coregame_140f8cf30`
/// Original: 0x140f8cf30 (704 bytes, Throws::No)
pub unsafe extern "win64" fn coregame_140f8cf30(
    param_1: usize,
    param_2: usize,
    param_3: i64,
    param_4: u8,
) {
    let mut r10 = rd::<usize>(param_2);
    let mut r9 = rd::<usize>(param_1);
    let mut diff = (r10.wrapping_sub(r9)) as i64;
    let mut r14 = param_3;
    let bl = param_4;

    if diff >= 0x630 {
        loop {
            let mut local_40: usize = r10;
            let mut local_48: usize = r9;
            if r14 <= 0 {
                exe::call_va!(
                    0x1_40f9_0c40,
                    (
                        (&mut local_48 as *mut usize) as usize,
                        (&mut local_40 as *mut usize) as usize,
                        bl
                    )
                );
                let mut rcx = rd::<usize>(param_2);
                let rsi = rd::<usize>(param_1);
                if (rcx.wrapping_sub(rsi) as i64) >= 0x60 {
                    loop {
                        let rdi = rcx.wrapping_sub(0x30);
                        let cnt = sdiv_0x30((rcx.wrapping_sub(rsi)) as i64);
                        if cnt >= 2 {
                            let mut tmp = rd::<[u64; 6]>(rdi);
                            let head = rd::<[u64; 6]>(rsi);
                            wr::<[u64; 6]>(rdi, head);
                            local_40 = rsi;
                            let sub_cnt = sdiv_0x30((rdi.wrapping_sub(rsi)) as i64);
                            exe::call_va!(
                                0x1_40f9_30a0,
                                (
                                    (&mut local_40 as *mut usize) as usize,
                                    0u64,
                                    sub_cnt,
                                    tmp.as_mut_ptr() as usize,
                                    bl
                                )
                            );
                        }
                        rcx = rdi;
                        if (rcx.wrapping_sub(rsi) as i64) < 0x60 {
                            break;
                        }
                    }
                }
                return;
            }

            let mut part_pair = [0usize; 2];
            exe::call_va!(
                0x1_40f9_0dd0,
                (
                    part_pair.as_mut_ptr() as usize,
                    (&mut local_48 as *mut usize) as usize,
                    (&mut local_40 as *mut usize) as usize,
                    bl
                )
            );
            r10 = rd::<usize>(param_2);
            let rdi = part_pair[1];
            let rsi = part_pair[0];
            r9 = rd::<usize>(param_1);
            r14 = (r14 >> 1).wrapping_add(r14 >> 2);

            let right_cnt = sdiv_0x30((r10.wrapping_sub(rdi)) as i64);
            let left_cnt = sdiv_0x30((rsi.wrapping_sub(r9)) as i64);
            if left_cnt < right_cnt {
                local_48 = r9;
                local_40 = rsi;
                exe::call_va!(
                    0x1_40f8_cf30,
                    (
                        (&mut local_48 as *mut usize) as usize,
                        (&mut local_40 as *mut usize) as usize,
                        r14,
                        bl
                    )
                );
                wr::<usize>(param_1, rdi);
            } else {
                local_40 = r10;
                local_48 = rdi;
                exe::call_va!(
                    0x1_40f8_cf30,
                    (
                        (&mut local_48 as *mut usize) as usize,
                        (&mut local_40 as *mut usize) as usize,
                        r14,
                        bl
                    )
                );
                wr::<usize>(param_2, rsi);
            }

            r10 = rd::<usize>(param_2);
            r9 = rd::<usize>(param_1);
            diff = (r10.wrapping_sub(r9)) as i64;
            if diff < 0x630 {
                break;
            }
        }
    }

    if r9 == r10 {
        return;
    }
    let mut rdx = r9.wrapping_add(0x30);
    if rdx == r10 {
        return;
    }

    loop {
        let elem = rd::<[u64; 6]>(rdx);
        let key = elem[2];
        let mut rcx = rdx;
        if key < rd::<u64>(r9 + 0x10) {
            if r9 != rdx {
                let mut rax = rdx;
                let mut dst = rdx.wrapping_add(0x30);
                loop {
                    let prev = rd::<[u64; 6]>(rax.wrapping_sub(0x30));
                    rax = rax.wrapping_sub(0x30);
                    dst = dst.wrapping_sub(0x30);
                    wr::<[u64; 6]>(dst, prev);
                    if rax == r9 {
                        break;
                    }
                }
            }
            wr::<[u64; 6]>(r9, elem);
        } else {
            let mut rax = rdx.wrapping_sub(0x30);
            if key < rd::<u64>(rdx.wrapping_sub(0x20)) {
                loop {
                    let prev = rd::<[u64; 6]>(rax);
                    wr::<[u64; 6]>(rcx, prev);
                    rcx = rax;
                    rax = rax.wrapping_sub(0x30);
                    if key >= rd::<u64>(rax + 0x10) {
                        break;
                    }
                }
            }
            wr::<[u64; 6]>(rcx, elem);
        }
        rdx = rdx.wrapping_add(0x30);
        if rdx == r10 {
            break;
        }
    }
}

/// `unknown_140f90730`
/// Original: 0x140f90730 (516 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140f90730(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let old_cap = (rd::<u32>(param_1 + 0x0c) & 0x3fff_ffff) as u64;
    let mut new_cap = old_cap + (old_cap >> 1);
    if new_cap < 8 {
        new_cap = 8;
    } else if new_cap > 0x3fff_ffff {
        new_cap = 0x3fff_ffff;
    }

    let diff = (rd::<usize>(param_2).wrapping_sub(rd::<usize>(param_1))) as i64;
    let hi = ((diff as i128) * (0x6666_6666_6666_6667i64 as i128)) >> 64;
    let sar = (hi as i64) >> 4;
    let rbx_idx = (sar as u64).wrapping_add((sar as u64) >> 63) as usize;

    let r15 = exe::call_va!(0x1_401e_c420, (diff as usize, new_cap as u32) -> usize);
    let rbx_ptr = r15.wrapping_add(rbx_idx.wrapping_mul(0x28));
    wr::<[u64; 5]>(rbx_ptr, rd::<[u64; 5]>(param_3));

    let len = rd::<u32>(param_1 + 0x08) as usize;
    let mut rdx = rd::<usize>(param_1);
    let rcx = rd::<usize>(param_2);
    let end = rdx.wrapping_add(len.wrapping_mul(0x28));
    if rcx == end {
        let mut dst = r15;
        while rdx != end {
            wr::<[u64; 5]>(dst, rd::<[u64; 5]>(rdx));
            rdx = rdx.wrapping_add(0x28);
            dst = dst.wrapping_add(0x28);
        }
    } else {
        let mut dst = r15;
        while rdx != rcx {
            wr::<[u64; 5]>(dst, rd::<[u64; 5]>(rdx));
            rdx = rdx.wrapping_add(0x28);
            dst = dst.wrapping_add(0x28);
        }
        let len2 = rd::<u32>(param_1 + 0x08) as usize;
        let mut dst2 = rbx_ptr.wrapping_add(0x28);
        let end2 = rd::<usize>(param_1).wrapping_add(len2.wrapping_mul(0x28));
        let mut src2 = rd::<usize>(param_2);
        while src2 != end2 {
            wr::<[u64; 5]>(dst2, rd::<[u64; 5]>(src2));
            src2 = src2.wrapping_add(0x28);
            dst2 = dst2.wrapping_add(0x28);
        }
    }

    let old_ptr = rd::<usize>(param_1);
    if (rd::<u32>(param_1 + 0x0c) & 0x4000_0000) == 0 && old_ptr != 0 {
        free_custom_block(old_ptr);
    }

    wr::<usize>(param_1, r15);
    wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_add(1));
    wr::<u32>(param_1 + 0x0c, (new_cap as u32) & 0x3fff_ffff);
    rbx_ptr
}

/// `unknown_140fb7530`
/// Original: 0x140fb7530 (2279 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_140fb7530(param_1: usize, param_2: u32) {
    let r13 = (param_2 as i32) as i64;
    let mut r14: usize = 0;
    if param_2 != 0 {
        let byte_len = (r13 as u64).wrapping_mul(8);
        r14 = exe::call_va!(0x1_4241_c0a0, (byte_len, 8u32) -> usize);
        exe::call_va!(0x1_4183_acc2, (r14, 0u32, byte_len));
    }

    let mut h0 = [0u32; 1024];
    let mut h1 = [0u32; 1024];
    let mut h2 = [0u32; 2048];
    let mut h3 = [0u32; 2048];
    let mut h4 = [0u32; 2048];
    let mut h5 = [0u32; 2048];
    let mut offsets = [0u32; 2048];

    if r13 > 0 {
        let count = r13 as usize;
        for i in 0..count {
            let val = rd::<u64>(param_1.wrapping_add(i * 8));
            let b0 = (val as usize) & 0x3ff;
            let b1 = ((val >> 10) as usize) & 0x3ff;
            let b2 = ((val >> 20) as usize) & 0x7ff;
            let b3 = ((val >> 31) as usize) & 0x7ff;
            let b4 = ((val >> 42) as usize) & 0x7ff;
            let b5 = ((val >> 53) as usize) & 0x7ff;
            h0[b0] = h0[b0].wrapping_add(1);
            h1[b1] = h1[b1].wrapping_add(1);
            h2[b2] = h2[b2].wrapping_add(1);
            h3[b3] = h3[b3].wrapping_add(1);
            h4[b4] = h4[b4].wrapping_add(1);
            h5[b5] = h5[b5].wrapping_add(1);
        }
    }

    let n = r13.max(0) as usize;

    // Pass 0: bits 0..10 (1024 buckets), param_1 -> r14
    offsets[0] = 0;
    for b in 0..1023 {
        offsets[b + 1] = offsets[b].wrapping_add(h0[b]);
    }
    for i in 0..n {
        let val = rd::<u64>(param_1.wrapping_add(i * 8));
        let b = (val as usize) & 0x3ff;
        let dst_idx = (offsets[b] as i32) as isize;
        offsets[b] = offsets[b].wrapping_add(1);
        wr::<u64>(r14.wrapping_add((dst_idx as usize).wrapping_mul(8)), val);
    }

    // Pass 1: bits 10..20 (1024 buckets), r14 -> param_1
    offsets[0] = 0;
    for b in 0..1023 {
        offsets[b + 1] = offsets[b].wrapping_add(h1[b]);
    }
    for i in 0..n {
        let val = rd::<u64>(r14.wrapping_add(i * 8));
        let b = ((val >> 10) as usize) & 0x3ff;
        let dst_idx = (offsets[b] as i32) as isize;
        offsets[b] = offsets[b].wrapping_add(1);
        wr::<u64>(param_1.wrapping_add((dst_idx as usize).wrapping_mul(8)), val);
    }

    // Pass 2: bits 20..31 (2048 buckets), param_1 -> r14
    offsets[0] = 0;
    for b in 0..2047 {
        offsets[b + 1] = offsets[b].wrapping_add(h2[b]);
    }
    for i in 0..n {
        let val = rd::<u64>(param_1.wrapping_add(i * 8));
        let b = ((val >> 20) as usize) & 0x7ff;
        let dst_idx = (offsets[b] as i32) as isize;
        offsets[b] = offsets[b].wrapping_add(1);
        wr::<u64>(r14.wrapping_add((dst_idx as usize).wrapping_mul(8)), val);
    }

    // Pass 3: bits 31..42 (2048 buckets), r14 -> param_1
    offsets[0] = 0;
    for b in 0..2047 {
        offsets[b + 1] = offsets[b].wrapping_add(h3[b]);
    }
    for i in 0..n {
        let val = rd::<u64>(r14.wrapping_add(i * 8));
        let b = ((val >> 31) as usize) & 0x7ff;
        let dst_idx = (offsets[b] as i32) as isize;
        offsets[b] = offsets[b].wrapping_add(1);
        wr::<u64>(param_1.wrapping_add((dst_idx as usize).wrapping_mul(8)), val);
    }

    // Pass 4: bits 42..53 (2048 buckets), param_1 -> r14
    offsets[0] = 0;
    for b in 0..2047 {
        offsets[b + 1] = offsets[b].wrapping_add(h4[b]);
    }
    for i in 0..n {
        let val = rd::<u64>(param_1.wrapping_add(i * 8));
        let b = ((val >> 42) as usize) & 0x7ff;
        let dst_idx = (offsets[b] as i32) as isize;
        offsets[b] = offsets[b].wrapping_add(1);
        wr::<u64>(r14.wrapping_add((dst_idx as usize).wrapping_mul(8)), val);
    }

    // Pass 5: bits 53..64 (2048 buckets), r14 -> param_1
    offsets[0] = 0;
    for b in 0..2047 {
        offsets[b + 1] = offsets[b].wrapping_add(h5[b]);
    }
    for i in 0..n {
        let val = rd::<u64>(r14.wrapping_add(i * 8));
        let b = ((val >> 53) as usize) & 0x7ff;
        let dst_idx = (offsets[b] as i32) as isize;
        offsets[b] = offsets[b].wrapping_add(1);
        wr::<u64>(param_1.wrapping_add((dst_idx as usize).wrapping_mul(8)), val);
    }

    if r14 != 0 {
        exe::call_va!(0x1_4241_c190, (r14));
    }
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_coregame_140f8cf30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i64, u8);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BUF_CF30: [[u64; 6]; 40] = [[0; 6]; 40];

    let run = |f: F, count: usize, depth: i64| -> (Vec<[u64; 6]>, usize, usize) {
        let mut rng = crate::difftest::Rng::new(0x140f_8cf3 ^ (count as u64) ^ (depth as u64));
        for i in 0..40 {
            let key = if i < count {
                (rng.next_u64() & 0xff) ^ ((40 - i as u64) << 4)
            } else {
                0xdead_beef_0000 + i as u64
            };
            BUF_CF30[i] = [
                0x100 + i as u64,
                0x200 + i as u64,
                key,
                0x300 + i as u64,
                0x400 + i as u64,
                0x500 + i as u64,
            ];
        }
        if count >= 4 {
            BUF_CF30[count - 1][2] = 0;
            BUF_CF30[count / 2][2] = BUF_CF30[1][2];
        }

        let base = core::ptr::addr_of_mut!(BUF_CF30) as usize;
        let mut first: usize = base;
        let mut last: usize = base + count * 0x30;
        f(
            (&mut first as *mut usize) as usize,
            (&mut last as *mut usize) as usize,
            depth,
            0,
        );
        (
            BUF_CF30[..count].to_vec(),
            first.wrapping_sub(base),
            last.wrapping_sub(base),
        )
    };

    for (count, depth) in [(0usize, 4i64), (1, 4), (10, 4), (36, 0), (40, 8)] {
        let o = run(f_o, count, depth);
        let n = run(f_n, count, depth);
        crate::diff_eq!(o, n, "140f8cf30 count={} depth={}", count, depth);
    }
    Ok(5)
}

pub unsafe fn diff_unknown_140f90730(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF_0730: [[u64; 5]; 12] = [[0; 5]; 12];
    static mut ALLOC_ARGS_0730: (usize, u32) = (0, 0);
    static mut FREE_3200_CNT: u32 = 0;
    static mut RETIRE_71A0_CNT: u32 = 0;

    unsafe extern "win64" fn mock_c420(byte_off: usize, cap: u32) -> usize {
        ALLOC_ARGS_0730 = (byte_off, cap);
        core::ptr::addr_of_mut!(NEW_BUF_0730) as usize
    }
    unsafe extern "win64" fn mock_3200(_seg: usize, _hdr: usize, _flag: u8, _ptr: usize) {
        FREE_3200_CNT += 1;
    }
    unsafe extern "win64" fn mock_71a0(_hdr: usize) {
        RETIRE_71A0_CNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_c420, mock_c420 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_71a0, mock_71a0 as *const ())?;

    let mut arena_vec = vec![0u8; 0x0203_0000];
    let r10 = (arena_vec.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F,
               ins_idx: usize,
               owned: bool,
               old_cap: u32|
     -> ([[u64; 5]; 6], (usize, u32), usize, u32, u32, usize, u32, u32) {
        NEW_BUF_0730 = [[0; 5]; 12];
        ALLOC_ARGS_0730 = (0, 0);
        FREE_3200_CNT = 0;
        RETIRE_71A0_CNT = 0;

        core::ptr::write_bytes(r10 as *mut u8, 0, 0x2_1000);
        let obp = r10 + 0x1_0000;
        for i in 0..3usize {
            let base_v = ((i + 1) * 10) as u64;
            wr::<[u64; 5]>(
                obp + i * 0x28,
                [base_v, base_v + 1, base_v + 2, base_v + 3, base_v + 4],
            );
        }

        let hdr = r10 + 0x108 + 0x50;
        wr::<u32>(hdr + 0x04, 0);
        wr::<u32>(hdr + 0x18, 1);
        wr::<usize>(r10 + 0x100, exe::teb());

        let nbp = core::ptr::addr_of_mut!(NEW_BUF_0730) as usize;
        let cap_field = old_cap | if owned { 0 } else { 0x4000_0000 };
        let mut vec_hdr: [u64; 2] = [obp as u64, 3u64 | ((cap_field as u64) << 32)];
        let pos: usize = obp + ins_idx * 0x28;
        let item: [u64; 5] = [90, 91, 92, 93, 94];

        let ret = f(
            vec_hdr.as_mut_ptr() as usize,
            (&pos as *const usize) as usize,
            item.as_ptr() as usize,
        );

        let mut out_items = [[0u64; 5]; 6];
        out_items.copy_from_slice(&NEW_BUF_0730[..6]);
        (
            out_items,
            ALLOC_ARGS_0730,
            (vec_hdr[0] as usize).wrapping_sub(nbp),
            vec_hdr[1] as u32,
            (vec_hdr[1] >> 32) as u32,
            ret.wrapping_sub(nbp),
            FREE_3200_CNT,
            RETIRE_71A0_CNT,
        )
    };

    for (ins_idx, owned, old_cap) in [(3usize, false, 3u32), (1, false, 10), (2, true, 4)] {
        let o = run(f_o, ins_idx, owned, old_cap);
        let n = run(f_n, ins_idx, owned, old_cap);
        crate::diff_eq!(
            o,
            n,
            "140f90730 idx={} owned={} cap={}",
            ins_idx,
            owned,
            old_cap
        );
    }
    Ok(3)
}

pub unsafe fn diff_unknown_140fb7530(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut SCRATCH_7530: [u64; 32] = [0; 32];
    static mut DATA_7530: [u64; 32] = [0; 32];
    static mut ALLOC_CALL_7530: (u64, u32) = (0, 0);
    static mut FREE_CNT_7530: u32 = 0;

    unsafe extern "win64" fn mock_c0a0(size: u64, align: u32) -> usize {
        ALLOC_CALL_7530 = (size, align);
        core::ptr::addr_of_mut!(SCRATCH_7530) as usize
    }
    unsafe extern "win64" fn mock_c190(ptr: usize) {
        if ptr == core::ptr::addr_of_mut!(SCRATCH_7530) as usize {
            FREE_CNT_7530 += 1;
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4241_c0a0, mock_c0a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_c190, mock_c190 as *const ())?;

    let run = |f: F, count: u32| -> (Vec<u64>, (u64, u32), u32) {
        SCRATCH_7530 = [0xaaaa_bbbb_cccc_ddddu64; 32];
        ALLOC_CALL_7530 = (0, 0);
        FREE_CNT_7530 = 0;

        let mut rng = crate::difftest::Rng::new(0x140f_b753 ^ (count as u64));
        for i in 0..32 {
            DATA_7530[i] = rng.next_u64();
        }
        if count >= 4 {
            DATA_7530[count as usize - 1] = DATA_7530[0];
        }

        let ptr = core::ptr::addr_of_mut!(DATA_7530) as usize;
        f(ptr, count);
        (
            DATA_7530[..count as usize].to_vec(),
            ALLOC_CALL_7530,
            FREE_CNT_7530,
        )
    };

    for count in [0u32, 1, 4, 7, 19] {
        let o = run(f_o, count);
        let n = run(f_n, count);
        crate::diff_eq!(o, n, "140fb7530 count={}", count);
    }
    Ok(5)
}
