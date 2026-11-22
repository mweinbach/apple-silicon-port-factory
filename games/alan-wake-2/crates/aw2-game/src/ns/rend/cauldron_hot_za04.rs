//! Decompiled hot functions shard ZA04 (`0x140fff3d0..0x1411db770`).
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};
use crate::simd_ops::{
    atomic_add_u32, atomic_xadd_u32, atomic_xchg_u64, rd_acq_u32, rd_acq_usize, vpmovmskb,
    vpsignb, wr_rel_u64,
};

#[inline(always)]
unsafe fn ring_alloc(alloc_bytes: usize) -> usize {
    let g_ptr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let mgr = rd::<usize>(g_ptr);
    let ring = rd::<usize>(mgr + 0x168);
    exe::call_iat!(0x1_4264_55a8, (ring + 0x18));

    let mut rd_ptr = rd_acq_usize(ring + 0x20);
    let mut wr_ptr = rd_acq_usize(ring + 0x28);
    let mut next_wr = wr_ptr.wrapping_add(alloc_bytes);
    let ring_end = rd::<usize>(ring + 0x10);
    if next_wr > ring_end {
        while wr_ptr <= rd_ptr
            && rd_ptr < next_wr
            && rd_acq_u32(ring + 0x30) != rd_acq_u32(ring + 0x34)
        {
            core::hint::spin_loop();
            rd_ptr = rd_acq_usize(ring + 0x20);
        }
        if wr_ptr <= ring_end.wrapping_sub(8) {
            wr_rel_u64(wr_ptr, 0x00ff_ffff_ffff);
        }
        wr_ptr = rd::<usize>(ring + 0x08);
        next_wr = wr_ptr.wrapping_add(alloc_bytes);
    }
    while wr_ptr <= rd_ptr
        && rd_ptr < next_wr
        && rd_acq_u32(ring + 0x30) != rd_acq_u32(ring + 0x34)
    {
        core::hint::spin_loop();
        rd_ptr = rd_acq_usize(ring + 0x20);
    }

    wr_rel_u64(wr_ptr, 0x69fa_fafa_faff);
    atomic_xchg_u64((ring + 0x28) as u64, next_wr as u64);
    exe::call_iat!(0x1_4264_5598, (ring + 0x18));
    wr_ptr
}

#[inline(always)]
unsafe fn ring_commit(wr_ptr: usize, pkt_len: u64) {
    let g_ptr = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let mgr = rd::<usize>(g_ptr);
    let hdr = (rd::<u32>(wr_ptr + 0x08) & 0x1ff) | 0x200;
    wr::<u32>(wr_ptr + 0x08, hdr);
    let ring = rd::<usize>(mgr + 0x168);
    wr_rel_u64(wr_ptr, pkt_len);
    atomic_add_u32((ring + 0x30) as u64, 1);
    if rd::<u8>(mgr + 0x198) != 0 {
        exe::call_iat!(0x1_4264_5580, (rd::<u64>(mgr + 0x1a0), 1u32, 0usize));
    }
}

#[inline(always)]
unsafe fn free_custom_block(ptr: usize) {
    let r10 = ptr.wrapping_sub(1) & !0x01ff_ffffusize;
    let same_thread = exe::teb() == rd::<usize>(r10 + 0x100);
    let seg_idx = (ptr.wrapping_sub(r10)) >> 16;
    let page = r10.wrapping_add(0x108).wrapping_add(seg_idx.wrapping_mul(0x50));
    let rdx_hdr = page.wrapping_sub(rd::<u32>(page + 0x04) as usize);
    if same_thread {
        if rd::<u8>(rdx_hdr + 0x0e) == 0 {
            wr::<u64>(ptr, rd::<u64>(rdx_hdr + 0x20));
            wr::<usize>(rdx_hdr + 0x20, ptr);
            let used = rd::<u32>(rdx_hdr + 0x18).wrapping_sub(1);
            wr::<u32>(rdx_hdr + 0x18, used);
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

/// `unknown_140fff3d0`
/// Original: 0x140fff3d0 (373 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fff3d0(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) {
    let wr_ptr = ring_alloc(0x18);
    wr::<u32>(wr_ptr + 0x08, 0x24f);
    wr::<u32>(wr_ptr + 0x0c, rd::<u32>(param_1));
    wr::<u32>(wr_ptr + 0x10, rd::<u32>(param_2));
    wr::<u8>(wr_ptr + 0x14, rd::<u8>(param_3));
    ring_commit(wr_ptr, 0x10);
}

/// `unknown_140fff550`
/// Original: 0x140fff550 (355 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fff550(param_1: usize) {
    let wr_ptr = ring_alloc(0x10);
    wr::<u32>(wr_ptr + 0x08, 0x25d);
    wr::<u32>(wr_ptr + 0x0c, rd::<u32>(param_1));
    ring_commit(wr_ptr, 0x08);
}

/// `unknown_140fff6c0`
/// Original: 0x140fff6c0 (399 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fff6c0(param_1: usize, param_2: usize) {
    let wr_ptr = ring_alloc(0x28);
    wr::<u32>(wr_ptr + 0x08, 0x25c);
    wr::<u32>(wr_ptr + 0x0c, rd::<u32>(param_1));
    wr::<u64>(wr_ptr + 0x14, 0);
    wr::<u64>(wr_ptr + 0x1c, 0);
    let ref_blk = rd::<usize>(param_2 + 0x08);
    if ref_blk != 0 {
        atomic_add_u32((ref_blk + 0x08) as u64, 1);
    }
    wr::<u64>(wr_ptr + 0x14, rd::<u64>(param_2));
    wr::<u64>(wr_ptr + 0x1c, rd::<u64>(param_2 + 0x08));
    ring_commit(wr_ptr, 0x1c);
}

/// `unknown_140fff850`
/// Original: 0x140fff850 (399 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fff850(param_1: usize, param_2: usize) {
    let wr_ptr = ring_alloc(0x28);
    wr::<u32>(wr_ptr + 0x08, 0x25a);
    wr::<u32>(wr_ptr + 0x0c, rd::<u32>(param_1));
    wr::<u64>(wr_ptr + 0x14, 0);
    wr::<u64>(wr_ptr + 0x1c, 0);
    let ref_blk = rd::<usize>(param_2 + 0x08);
    if ref_blk != 0 {
        atomic_add_u32((ref_blk + 0x08) as u64, 1);
    }
    wr::<u64>(wr_ptr + 0x14, rd::<u64>(param_2));
    wr::<u64>(wr_ptr + 0x1c, rd::<u64>(param_2 + 0x08));
    ring_commit(wr_ptr, 0x1c);
}

/// `unknown_140fff9e0`
/// Original: 0x140fff9e0 (367 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_140fff9e0(param_1: usize, param_2: usize) {
    let wr_ptr = ring_alloc(0x18);
    wr::<u32>(wr_ptr + 0x08, 0x254);
    wr::<u32>(wr_ptr + 0x0c, rd::<u32>(param_1));
    wr::<u32>(wr_ptr + 0x10, rd::<u32>(param_2));
    ring_commit(wr_ptr, 0x0c);
}

/// `unknown_1410b0350`
/// Original: 0x1410b0350 (321 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1410b0350(param_1: usize, param_2: u64) {
    let world = rd::<usize>(param_1);
    let arch = (param_2 as u16) as usize;
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
        exe::call_va!(
            0x1_410b_0e90,
            (
                buf.as_mut_ptr() as usize,
                param_2,
                block,
                count as u64,
                world
            )
        );
        let total = buf[3];
        let b1 = buf[1];
        let b4 = buf[4];
        let b5 = buf[5];
        for idx in 0..total {
            let mut arg1: [u64; 4] = [b4, b5, b1, idx];
            let mut arg2: [u64; 3] = [world as u64, len_0f4, ptr_0f4];
            exe::call_va!(
                0x1_41f3_a990,
                (arg1.as_mut_ptr() as usize, arg2.as_mut_ptr() as usize)
            );
        }
    }
}

/// `unknown_1410d3510`
/// Original: 0x1410d3510 (82 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1410d3510(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let len = rd::<u32>(param_1 + 0x08) as usize;
    if len != 0 {
        let base = rd::<usize>(param_1);
        let target = rd::<u64>(param_3);
        for i in 0..len {
            let ptr = base.wrapping_add(i * 0x18);
            if rd::<u64>(ptr) == target {
                wr::<usize>(param_2, ptr);
                return param_2;
            }
        }
    }
    let end = rd::<usize>(param_1).wrapping_add(len * 0x18);
    wr::<usize>(param_2, end);
    param_2
}

/// `unknown_1410d3570`
/// Original: 0x1410d3570 (260 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1410d3570(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let diff = (rd::<usize>(param_3).wrapping_sub(rd::<usize>(param_1))) as i64;
    let hi = ((diff as i128) * (0x2aaa_aaaa_aaaa_aaabi64 as i128)) >> 64;
    let sar = (hi as i64) >> 2;
    let r15 = (sar as u64).wrapping_add((sar as u64) >> 63);

    let r14 = rd::<u32>(param_1 + 0x08) as u64;
    let next_idx = r15.wrapping_add(1);
    if next_idx < r14 {
        let mut rbx = (next_idx as usize).wrapping_mul(0x18);
        let mut rem = r14.wrapping_sub(next_idx);
        while rem != 0 {
            let base = rd::<usize>(param_1);
            atomic_xchg_u64((base.wrapping_add(rbx).wrapping_sub(0x10)) as u64, 0);
            let base2 = rd::<usize>(param_1);
            let dst = base2.wrapping_add(rbx).wrapping_sub(0x18);
            let src = base2.wrapping_add(rbx);
            wr::<u64>(dst, 0);
            wr::<u64>(dst + 0x08, 0);
            wr::<u32>(dst + 0x10, 0);
            wr::<u8>(dst + 0x14, 0);
            exe::call_va!(0x1_401e_84a0, (dst, src));
            rbx = rbx.wrapping_add(0x18);
            rem = rem.wrapping_sub(1);
        }
    }

    let mut base = rd::<usize>(param_1);
    let end = base.wrapping_add((r14 as usize).wrapping_mul(0x18));
    let mut cur = base.wrapping_add((r14.wrapping_sub(1) as usize).wrapping_mul(0x18));
    if cur != end {
        while cur != end {
            atomic_xchg_u64((cur + 0x08) as u64, 0);
            cur = cur.wrapping_add(0x18);
        }
        base = rd::<usize>(param_1);
    }

    wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_sub(1));
    wr::<usize>(param_2, base.wrapping_add((r15 as usize).wrapping_mul(0x18)));
    param_2
}

/// `unknown_1410d3d20`
/// Original: 0x1410d3d20 (326 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1410d3d20(
    param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mask = rd::<u64>(param_1 + 0x40).wrapping_sub(1);
    let table = rd::<usize>(param_1 + 0x60);
    let empty_key = rd::<u64>(param_1 + 0x48);

    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for i in 0..8 {
        h = (h ^ (rd::<u8>(param_3 + i) as u64)).wrapping_mul(0x0100_0000_01b3);
    }

    let mut idx = h & mask;
    let mut tombstone_idx = u64::MAX;
    let mut step: u64 = 0;

    if rd::<u64>(table.wrapping_add((idx as usize).wrapping_mul(0x18))) != empty_key {
        let has_tombstones = rd::<u64>(param_1 + 0x30) != 0;
        loop {
            let slot_addr = table.wrapping_add((idx as usize).wrapping_mul(0x18));
            let slot_key = rd::<u64>(slot_addr);
            if has_tombstones && rd::<u64>(param_1 + 0x28) == slot_key {
                if tombstone_idx == u64::MAX {
                    tombstone_idx = idx;
                }
            } else if rd::<u64>(param_3) == slot_key {
                wr::<u64>(param_2, idx);
                wr::<u64>(param_2 + 0x08, u64::MAX);
                return param_2;
            }
            step = step.wrapping_add(1);
            idx = idx.wrapping_add(step) & mask;
            if rd::<u64>(table.wrapping_add((idx as usize).wrapping_mul(0x18))) == empty_key {
                break;
            }
        }
    }

    wr::<u64>(param_2, u64::MAX);
    wr::<u64>(
        param_2 + 0x08,
        if tombstone_idx == u64::MAX {
            idx
        } else {
            tombstone_idx
        },
    );
    param_2
}

/// `unknown_1410d5810`
/// Original: 0x1410d5810 (481 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1410d5810(
    _param_1: usize,
    param_2: usize,
    param_3: usize,
) -> usize {
    let mut frame = [0u64; 5];
    frame[2] = rd::<u64>(param_3);
    let vec_a = frame.as_mut_ptr() as usize;
    let vec_b = vec_a + 0x18;

    exe::call_va!(0x1_410d_52e0, (vec_b, vec_a));

    wr::<u64>(param_2, rd::<u64>(vec_a + 0x10));
    wr::<u64>(param_2 + 0x08, 0);
    wr::<u64>(param_2 + 0x10, 0);
    exe::call_va!(0x1_410d_52e0, (param_2 + 0x08, vec_b));

    let len_b = rd::<u32>(vec_b + 0x08) as usize;
    let mut cur_b = rd::<usize>(vec_b);
    let end_b = cur_b.wrapping_add(len_b.wrapping_mul(0x28));
    while cur_b != end_b {
        atomic_xchg_u64((cur_b + 0x08) as u64, 0);
        cur_b = cur_b.wrapping_add(0x28);
    }
    let ptr_b = rd::<usize>(vec_b);
    if (rd::<u32>(vec_b + 0x0c) & 0x4000_0000) == 0 && ptr_b != 0 {
        free_custom_block(ptr_b);
    }

    let len_a = rd::<u32>(vec_a + 0x08) as usize;
    let mut cur_a = rd::<usize>(vec_a);
    let end_a = cur_a.wrapping_add(len_a.wrapping_mul(0x28));
    while cur_a != end_a {
        atomic_xchg_u64((cur_a + 0x08) as u64, 0);
        cur_a = cur_a.wrapping_add(0x28);
    }
    let ptr_a = rd::<usize>(vec_a);
    if (rd::<u32>(vec_a + 0x0c) & 0x4000_0000) == 0 && ptr_a != 0 {
        free_custom_block(ptr_a);
    }

    param_2
}

/// `unknown_1410d6660`
/// Original: 0x1410d6660 (459 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1410d6660(
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
    let hi = ((diff as i128) * (0x2aaa_aaaa_aaaa_aaabi64 as i128)) >> 64;
    let sar = (hi as i64) >> 2;
    let idx = (sar as u64).wrapping_add((sar as u64) >> 63);

    let new_buf = exe::call_va!(0x1_401e_2180, (diff as u64, new_cap as u32) -> usize);
    let ins_ptr = new_buf.wrapping_add((idx as usize).wrapping_mul(0x18));
    wr::<u64>(ins_ptr, 0);
    wr::<u64>(ins_ptr + 0x08, 0);
    wr::<u32>(ins_ptr + 0x10, 0);
    wr::<u8>(ins_ptr + 0x14, 0);
    exe::call_va!(0x1_401e_84a0, (ins_ptr, param_3));

    let old_begin = rd::<usize>(param_1);
    let old_end = old_begin.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(0x18));
    let pos = rd::<usize>(param_2);
    if pos == old_end {
        let mut it_first: usize = old_begin;
        let mut it_last: usize = old_end;
        exe::call_va!(
            0x1_410d_6bf0,
            (
                (&mut it_first as *mut usize) as usize,
                (&mut it_last as *mut usize) as usize,
                new_buf
            )
        );
    } else {
        let mut it_first1: usize = old_begin;
        let mut it_last1: usize = pos;
        exe::call_va!(
            0x1_410d_6bf0,
            (
                (&mut it_first1 as *mut usize) as usize,
                (&mut it_last1 as *mut usize) as usize,
                new_buf
            )
        );
        let old_begin2 = rd::<usize>(param_1);
        let old_end2 =
            old_begin2.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(0x18));
        let mut it_first2: usize = rd::<usize>(param_2);
        let mut it_last2: usize = old_end2;
        exe::call_va!(
            0x1_410d_6bf0,
            (
                (&mut it_first2 as *mut usize) as usize,
                (&mut it_last2 as *mut usize) as usize,
                ins_ptr.wrapping_add(0x18)
            )
        );
    }

    let mut cur = rd::<usize>(param_1);
    let end = cur.wrapping_add((rd::<u32>(param_1 + 0x08) as usize).wrapping_mul(0x18));
    while cur != end {
        atomic_xchg_u64((cur + 0x08) as u64, 0);
        cur = cur.wrapping_add(0x18);
    }

    let old_ptr = rd::<usize>(param_1);
    if (rd::<u32>(param_1 + 0x0c) & 0x4000_0000) == 0 && old_ptr != 0 {
        free_custom_block(old_ptr);
    }

    wr::<usize>(param_1, new_buf);
    wr::<u32>(param_1 + 0x08, rd::<u32>(param_1 + 0x08).wrapping_add(1));
    wr::<u32>(param_1 + 0x0c, (new_cap as u32) & 0x3fff_ffff);
    ins_ptr
}

/// `unknown_14119f940`
/// Original: 0x14119f940 (163 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14119f940(param_1: usize) {
    let count = rd::<u64>(param_1 + 0x10);
    if count != 0 {
        if (rd::<u64>(param_1 + 0x38) >> 3) > count {
            let head = rd::<usize>(param_1 + 0x08);
            let first = rd::<usize>(head);
            exe::call_va!(0x1_4069_5d40, (param_1, first, head));
            return;
        }
        let head = rd::<usize>(param_1 + 0x08);
        let prev = rd::<usize>(head + 0x08);
        wr::<u64>(prev, 0);
        let mut node = rd::<usize>(head);
        while node != 0 {
            let next = rd::<usize>(node);
            exe::call_va!(0x1_401e_66f0, (head, node));
            node = next;
        }
        let head1 = rd::<usize>(param_1 + 0x08);
        wr::<usize>(head1, head1);
        let head2 = rd::<usize>(param_1 + 0x08);
        wr::<usize>(head2 + 0x08, head2);
        wr::<u64>(param_1 + 0x10, 0);
        let mut head3: usize = rd::<usize>(param_1 + 0x08);
        let p20 = rd::<usize>(param_1 + 0x20);
        let p18 = rd::<usize>(param_1 + 0x18);
        exe::call_va!(
            0x1_401e_6780,
            (p18, p20, (&mut head3 as *mut usize) as usize)
        );
    }
}

/// `unknown_1411bc750`
/// Original: 0x1411bc750 (170 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1411bc750(param_1: usize, param_2: usize) {
    wr::<u64>(param_1 + 0x10, rd::<u64>(param_1 + 0x10).wrapping_sub(1));
    let r11 = rd::<usize>(param_1);
    let rax = rd::<usize>(param_2);
    let mask = rd::<u64>(param_1 + 0x18);
    let r10 = (rax.wrapping_sub(r11)) as u64;
    let rbx = r10.wrapping_sub(0x10);

    let xmm0_a = rd::<u128>(rax);
    let mask_a = vpmovmskb(vpsignb(xmm0_a, xmm0_a)) as u32;
    let rcx_off = (mask & rbx) as usize;
    let xmm0_b = rd::<u128>(r11.wrapping_add(rcx_off));
    let mask_b = vpmovmskb(vpsignb(xmm0_b, xmm0_b)) as u32;

    let r8b: u8 = if mask_b != 0 && mask_a != 0 {
        let r8d = mask_a.trailing_zeros();
        let ecx_shl = mask_b << 16;
        let eax_lz = if ecx_shl == 0 {
            0x20
        } else {
            let edx = 31 - ecx_shl.leading_zeros();
            0x1f - edx
        };
        if eax_lz.wrapping_add(r8d) < 0x10 {
            1
        } else {
            0
        }
    } else {
        0
    };

    let dl: u8 = if r8b != 0 { 0x80 } else { 0xfe };
    wr::<u8>(r11.wrapping_add(r10 as usize), dl);
    let mask2 = rd::<u64>(param_1 + 0x18);
    let clone_off = ((mask2 & rbx).wrapping_add(mask2 & 0xf)) as usize;
    let ctrl2 = rd::<usize>(param_1);
    wr::<u8>(ctrl2.wrapping_add(clone_off).wrapping_add(1), dl);
    wr::<u64>(
        param_1 + 0x28,
        rd::<u64>(param_1 + 0x28).wrapping_add(r8b as u64),
    );
}

/// `unknown_1411c2820`
/// Original: 0x1411c2820 (147 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1411c2820(param_1: usize, param_2: usize) -> usize {
    let cap_bytes = (rd::<u32>(param_1 + 0x0c) as u64) << 3;
    let len_bytes = (rd::<u32>(param_1 + 0x08) as u64) << 3;
    if len_bytes == cap_bytes {
        let min_cap = ((len_bytes as i64) >> 3).wrapping_add(1) as u64;
        let old_cap = ((cap_bytes as i64) >> 3) as u64;
        let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
        if new_cap < min_cap {
            new_cap = min_cap;
        }
        exe::call_va!(0x1_411c_8030, (param_1, new_cap));
    }

    let val = rd::<usize>(param_2);
    let idx = rd::<u32>(param_1 + 0x08) as usize;
    let base = rd::<usize>(param_1);
    wr::<usize>(base.wrapping_add(idx * 8), val);
    if val != 0 {
        let old = atomic_xadd_u32((val + 0x24) as u64, 1);
        if old.wrapping_add(1) == 1 {
            let cb_ctx = rd::<usize>(val + 0x28);
            if cb_ctx != 0 {
                exe::call_va!(0x1_4241_d6f0, (cb_ctx, val));
            }
        }
    }

    let next_len = rd::<u32>(param_1 + 0x08).wrapping_add(1);
    wr::<u32>(param_1 + 0x08, next_len);
    let final_len = rd::<u32>(param_1 + 0x08) as usize;
    rd::<usize>(param_1)
        .wrapping_add(final_len * 8)
        .wrapping_sub(8)
}

/// `d3d_1411daad0`
/// Original: 0x1411daad0 (506 bytes, Throws::No)
pub unsafe extern "win64" fn d3d_1411daad0(param_1: usize, param_2: u32) {
    let old_cap = rd::<u32>(param_1 + 0x14);
    if param_2 <= old_cap {
        return;
    }

    let mut new_cap = (old_cap >> 1).wrapping_add(old_cap);
    let old_buf = rd::<usize>(param_1);
    if new_cap < 8 {
        new_cap = 8;
    }
    if new_cap < param_2 {
        new_cap = param_2;
    }

    let rdx_bytes = (new_cap as u64) << 3;
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);

    let new_buf = if (rdx_bytes & rdx_bytes.wrapping_sub(1)) == 0
        && rdx_bytes.wrapping_sub(0x10) <= 0x3f0
    {
        let page = rd::<usize>(heap.wrapping_add(rdx_bytes as usize).wrapping_add(0x08));
        let block = rd::<usize>(page + 0x10);
        if block == 0 {
            exe::call_va!(0x1_4253_7280, (heap, rdx_bytes, 0u32, 0u32) -> usize)
        } else {
            wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
            wr::<usize>(page + 0x10, rd::<usize>(block));
            block
        }
    } else {
        let mut fast_blk = 0usize;
        if rdx_bytes.wrapping_sub(0x10) <= 0x3f0 {
            let page = rd::<usize>(heap.wrapping_add(rdx_bytes as usize).wrapping_add(0x08));
            let block = rd::<usize>(page + 0x10);
            if block != 0 && (block & 0xf) == 0 {
                wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
                wr::<usize>(page + 0x10, rd::<usize>(block));
                fast_blk = block;
            }
        }
        if fast_blk != 0 {
            fast_blk
        } else {
            exe::call_va!(0x1_4253_5070, (heap, rdx_bytes, 0x10u32) -> usize)
        }
    };

    if new_buf == 0 {
        let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
        exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
    }

    let mut idx: u32 = 0;
    if rd::<u32>(param_1 + 0x10) > 0 {
        loop {
            let cap = rd::<u32>(param_1 + 0x14) as u64;
            let head = rd::<u32>(param_1 + 0x08) as u64;
            let slot = (head.wrapping_add(idx as u64) % cap) as usize;
            let src = rd::<usize>(param_1);
            wr::<u64>(
                new_buf.wrapping_add((idx as usize) * 8),
                rd::<u64>(src.wrapping_add(slot * 8)),
            );
            idx = idx.wrapping_add(1);
            if idx >= rd::<u32>(param_1 + 0x10) {
                break;
            }
        }
    }

    if old_buf != 0 {
        free_custom_block(old_buf);
    }

    let len = rd::<u32>(param_1 + 0x10);
    wr::<usize>(param_1, new_buf);
    wr::<u32>(param_1 + 0x08, 0);
    wr::<u32>(param_1 + 0x14, new_cap);
    wr::<u32>(param_1 + 0x0c, len);
}

/// `d3d_1411db770`
/// Original: 0x1411db770 (154 bytes, Throws::Caught)
pub unsafe extern "win64" fn d3d_1411db770(
    _param_1: usize,
    _param_2: usize,
    param_3: usize,
    param_4: u64,
) -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let heap = rd::<usize>(tls0 + 0x6348);
    let page = rd::<usize>(heap + 0x40);
    let block = rd::<usize>(page + 0x10);
    let ptr = if block != 0 && (block & 0xf) == 0 {
        wr::<u32>(page + 0x18, rd::<u32>(page + 0x18).wrapping_add(1));
        wr::<usize>(page + 0x10, rd::<usize>(block));
        block
    } else {
        let alloc = exe::call_va!(0x1_4253_5070, (heap, 0x38u32, 0x10u32) -> usize);
        if alloc != 0 {
            alloc
        } else {
            let proc = exe::call_iat!(0x1_4264_56c8, () -> usize);
            exe::call_iat!(0x1_4264_53a0, (proc, 0xdead_d00du32));
            0
        }
    };
    let val = rd::<u32>(param_3);
    exe::call_va!(0x1_4206_3c40, (ptr, val, param_4) -> usize)
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

static mut RING_SEM_COUNT: u32 = 0;

unsafe extern "win64" fn mock_ring_acq(_lock: usize) {}
unsafe extern "win64" fn mock_ring_rel(_lock: usize) {}
unsafe extern "win64" fn mock_ring_sem(_h: u64, cnt: u32, _prev: usize) -> i32 {
    RING_SEM_COUNT += cnt;
    1
}

pub unsafe fn diff_unknown_140fff3d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_ring_acq as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, mock_ring_rel as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5580, mock_ring_sem as *const ())?;

    let g_slot = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_g = rd::<usize>(g_slot);

    let run = |f: F, wrap: bool| -> ([u64; 10], usize, u32, u32) {
        RING_SEM_COUNT = 0;
        let mut ring_mem = [0u64; 10];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let mut ring_hdr = [0u8; 0x40];
        let rhp = ring_hdr.as_mut_ptr() as usize;
        wr::<usize>(rhp + 0x08, rmp);
        wr::<usize>(rhp + 0x10, rmp + 0x30);
        wr::<usize>(rhp + 0x20, rmp + 0x30);
        wr::<usize>(rhp + 0x28, if wrap { rmp + 0x20 } else { rmp });
        wr::<u32>(rhp + 0x30, 4);
        wr::<u32>(rhp + 0x34, 4);

        let mut g_obj = [0u8; 0x1b0];
        let gp = g_obj.as_mut_ptr() as usize;
        wr::<usize>(gp + 0x168, rhp);
        wr::<u8>(gp + 0x198, 1);
        wr::<u64>(gp + 0x1a0, 0x555);
        wr::<usize>(g_slot, gp);

        let p1: u32 = 0x1122_3344;
        let p2: u32 = 0x5566_7788;
        let p3: u8 = 0x99;
        f(
            (&p1 as *const u32) as usize,
            (&p2 as *const u32) as usize,
            (&p3 as *const u8) as usize,
        );
        wr::<usize>(g_slot, saved_g);
        (
            ring_mem,
            rd::<usize>(rhp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rhp + 0x30),
            RING_SEM_COUNT,
        )
    };

    for wrap in [false, true] {
        let o = run(f_o, wrap);
        let n = run(f_n, wrap);
        crate::diff_eq!(o, n, "140fff3d0 wrap={}", wrap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140fff550(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_ring_acq as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, mock_ring_rel as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5580, mock_ring_sem as *const ())?;

    let g_slot = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_g = rd::<usize>(g_slot);

    let run = |f: F, wrap: bool| -> ([u64; 10], usize, u32, u32) {
        RING_SEM_COUNT = 0;
        let mut ring_mem = [0u64; 10];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let mut ring_hdr = [0u8; 0x40];
        let rhp = ring_hdr.as_mut_ptr() as usize;
        wr::<usize>(rhp + 0x08, rmp);
        wr::<usize>(rhp + 0x10, rmp + 0x28);
        wr::<usize>(rhp + 0x20, rmp + 0x28);
        wr::<usize>(rhp + 0x28, if wrap { rmp + 0x20 } else { rmp });
        wr::<u32>(rhp + 0x30, 7);
        wr::<u32>(rhp + 0x34, 7);

        let mut g_obj = [0u8; 0x1b0];
        let gp = g_obj.as_mut_ptr() as usize;
        wr::<usize>(gp + 0x168, rhp);
        wr::<u8>(gp + 0x198, 1);
        wr::<u64>(gp + 0x1a0, 0x666);
        wr::<usize>(g_slot, gp);

        let p1: u32 = 0xaabb_ccdd;
        f((&p1 as *const u32) as usize);
        wr::<usize>(g_slot, saved_g);
        (
            ring_mem,
            rd::<usize>(rhp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rhp + 0x30),
            RING_SEM_COUNT,
        )
    };

    for wrap in [false, true] {
        let o = run(f_o, wrap);
        let n = run(f_n, wrap);
        crate::diff_eq!(o, n, "140fff550 wrap={}", wrap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140fff6c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_ring_acq as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, mock_ring_rel as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5580, mock_ring_sem as *const ())?;

    static mut REF_BLK: [u64; 2] = [0; 2];

    let g_slot = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_g = rd::<usize>(g_slot);

    let run = |f: F, wrap: bool| -> ([u64; 12], usize, u32, u32, u32) {
        RING_SEM_COUNT = 0;
        REF_BLK = [0, 10];
        let rbp_ref = core::ptr::addr_of_mut!(REF_BLK) as usize;

        let mut ring_mem = [0u64; 12];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let mut ring_hdr = [0u8; 0x40];
        let rhp = ring_hdr.as_mut_ptr() as usize;
        wr::<usize>(rhp + 0x08, rmp);
        wr::<usize>(rhp + 0x10, rmp + 0x40);
        wr::<usize>(rhp + 0x20, rmp + 0x40);
        wr::<usize>(rhp + 0x28, if wrap { rmp + 0x30 } else { rmp });
        wr::<u32>(rhp + 0x30, 2);
        wr::<u32>(rhp + 0x34, 2);

        let mut g_obj = [0u8; 0x1b0];
        let gp = g_obj.as_mut_ptr() as usize;
        wr::<usize>(gp + 0x168, rhp);
        wr::<u8>(gp + 0x198, 1);
        wr::<u64>(gp + 0x1a0, 0x777);
        wr::<usize>(g_slot, gp);

        let p1: u32 = 0x1357_2468;
        let p2: [u64; 2] = [0xcafe_babe_1234_5678, if wrap { 0 } else { rbp_ref as u64 }];
        f((&p1 as *const u32) as usize, p2.as_ptr() as usize);
        wr::<usize>(g_slot, saved_g);
        (
            ring_mem,
            rd::<usize>(rhp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rhp + 0x30),
            rd::<u32>(rbp_ref + 0x08),
            RING_SEM_COUNT,
        )
    };

    for wrap in [false, true] {
        let o = run(f_o, wrap);
        let n = run(f_n, wrap);
        crate::diff_eq!(o, n, "140fff6c0 wrap={}", wrap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140fff850(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_ring_acq as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, mock_ring_rel as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5580, mock_ring_sem as *const ())?;

    static mut REF_BLK_850: [u64; 2] = [0; 2];

    let g_slot = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_g = rd::<usize>(g_slot);

    let run = |f: F, wrap: bool| -> ([u64; 12], usize, u32, u32, u32) {
        RING_SEM_COUNT = 0;
        REF_BLK_850 = [0, 20];
        let rbp_ref = core::ptr::addr_of_mut!(REF_BLK_850) as usize;

        let mut ring_mem = [0u64; 12];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let mut ring_hdr = [0u8; 0x40];
        let rhp = ring_hdr.as_mut_ptr() as usize;
        wr::<usize>(rhp + 0x08, rmp);
        wr::<usize>(rhp + 0x10, rmp + 0x40);
        wr::<usize>(rhp + 0x20, rmp + 0x40);
        wr::<usize>(rhp + 0x28, if wrap { rmp + 0x30 } else { rmp });
        wr::<u32>(rhp + 0x30, 9);
        wr::<u32>(rhp + 0x34, 9);

        let mut g_obj = [0u8; 0x1b0];
        let gp = g_obj.as_mut_ptr() as usize;
        wr::<usize>(gp + 0x168, rhp);
        wr::<u8>(gp + 0x198, 1);
        wr::<u64>(gp + 0x1a0, 0x888);
        wr::<usize>(g_slot, gp);

        let p1: u32 = 0x2468_1357;
        let p2: [u64; 2] = [0xdead_beef_cafe_f00d, if wrap { 0 } else { rbp_ref as u64 }];
        f((&p1 as *const u32) as usize, p2.as_ptr() as usize);
        wr::<usize>(g_slot, saved_g);
        (
            ring_mem,
            rd::<usize>(rhp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rhp + 0x30),
            rd::<u32>(rbp_ref + 0x08),
            RING_SEM_COUNT,
        )
    };

    for wrap in [false, true] {
        let o = run(f_o, wrap);
        let n = run(f_n, wrap);
        crate::diff_eq!(o, n, "140fff850 wrap={}", wrap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_140fff9e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let _m1 = crate::difftest::IatMock::new(0x1_4264_55a8, mock_ring_acq as *const ())?;
    let _m2 = crate::difftest::IatMock::new(0x1_4264_5598, mock_ring_rel as *const ())?;
    let _m3 = crate::difftest::IatMock::new(0x1_4264_5580, mock_ring_sem as *const ())?;

    let g_slot = exe::ptr::<usize>(0x1_4397_ff58) as usize;
    let saved_g = rd::<usize>(g_slot);

    let run = |f: F, wrap: bool| -> ([u64; 10], usize, u32, u32) {
        RING_SEM_COUNT = 0;
        let mut ring_mem = [0u64; 10];
        let rmp = ring_mem.as_mut_ptr() as usize;
        let mut ring_hdr = [0u8; 0x40];
        let rhp = ring_hdr.as_mut_ptr() as usize;
        wr::<usize>(rhp + 0x08, rmp);
        wr::<usize>(rhp + 0x10, rmp + 0x30);
        wr::<usize>(rhp + 0x20, rmp + 0x30);
        wr::<usize>(rhp + 0x28, if wrap { rmp + 0x20 } else { rmp });
        wr::<u32>(rhp + 0x30, 1);
        wr::<u32>(rhp + 0x34, 1);

        let mut g_obj = [0u8; 0x1b0];
        let gp = g_obj.as_mut_ptr() as usize;
        wr::<usize>(gp + 0x168, rhp);
        wr::<u8>(gp + 0x198, 1);
        wr::<u64>(gp + 0x1a0, 0x999);
        wr::<usize>(g_slot, gp);

        let p1: u32 = 0xaaaa_bbbb;
        let p2: u32 = 0xcccc_dddd;
        f(
            (&p1 as *const u32) as usize,
            (&p2 as *const u32) as usize,
        );
        wr::<usize>(g_slot, saved_g);
        (
            ring_mem,
            rd::<usize>(rhp + 0x28).wrapping_sub(rmp),
            rd::<u32>(rhp + 0x30),
            RING_SEM_COUNT,
        )
    };

    for wrap in [false, true] {
        let o = run(f_o, wrap);
        let n = run(f_n, wrap);
        crate::diff_eq!(o, n, "140fff9e0 wrap={}", wrap);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1410b0350(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_0e90(
        out: usize,
        arch: u64,
        block: usize,
        cnt: u64,
        world: usize,
    ) -> usize {
        wr::<u64>(out, arch);
        wr::<u64>(out + 0x08, block as u64);
        wr::<u64>(out + 0x10, world as u64);
        wr::<u64>(out + 0x18, cnt);
        wr::<u64>(out + 0x20, block.wrapping_add(0x100) as u64);
        wr::<u64>(out + 0x28, block.wrapping_add(0x200) as u64);
        out
    }
    unsafe extern "win64" fn mock_a990(a1_ptr: usize, a2_ptr: usize) {
        let a1 = rd::<[u64; 4]>(a1_ptr);
        let mut a2 = rd::<[u64; 3]>(a2_ptr);
        let world = a2[0] as usize;
        a2[0] = 0;
        let idx = a1[3] as usize;
        let dst = world + 0x200 + idx * 0x38;
        wr::<[u64; 4]>(dst, a1);
        wr::<[u64; 3]>(dst + 0x20, a2);
    }

    let _m1 = crate::difftest::Mock::new(0x1_410b_0e90, mock_0e90 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_41f3_a990, mock_a990 as *const ())?;

    let g_0f4 = exe::ptr::<u16>(0x1_43cd_a0f4) as usize;
    let s_0f4 = rd::<u16>(g_0f4);
    wr::<u16>(g_0f4, 2);

    let run = |f: F, cnt: u32| -> [u64; 14] {
        let lut: [u16; 4] = [0, 0, 1, 0];
        let mut v0 = [0u8; 0x30];
        wr::<usize>(v0.as_mut_ptr() as usize + 0x20, lut.as_ptr() as usize);

        let mut arr_08 = [0u8; 0x30];
        let a08p = arr_08.as_mut_ptr() as usize;
        wr::<u64>(a08p + 0x10, 0x9999_0000);
        wr::<u32>(a08p + 0x18, 5);

        let mut world = vec![0u8; 0x10100];
        let wp = world.as_mut_ptr() as usize;
        wr::<usize>(wp, v0.as_ptr() as usize);
        wr::<usize>(wp + 0x08, a08p);
        wr::<u64>(wp + 0x88 + 8, 0x4000);
        wr::<u32>(wp + 0x10088 + 4, cnt);

        let p1 = [wp];
        f(p1.as_ptr() as usize, 1);
        rd::<[u64; 14]>(wp + 0x200)
    };

    for cnt in [0u32, 2] {
        let o = run(f_o, cnt);
        let n = run(f_n, cnt);
        wr::<u16>(g_0f4, s_0f4);
        crate::diff_eq!(o, n, "1410b0350 cnt={}", cnt);
        wr::<u16>(g_0f4, 2);
    }
    wr::<u16>(g_0f4, s_0f4);
    Ok(2)
}

pub unsafe fn diff_unknown_1410d3510(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARR_3510: [u64; 9] = [0; 9];

    let run = |f: F, key: u64, len: u32| -> (usize, bool) {
        ARR_3510 = [10, 1, 2, 20, 3, 4, 30, 5, 6];
        let base = core::ptr::addr_of_mut!(ARR_3510) as usize;
        let mut hdr: [u64; 2] = [base as u64, (len as u64) | (3u64 << 32)];
        let mut out_ptr: usize = 0;
        let ret = f(
            hdr.as_mut_ptr() as usize,
            (&mut out_ptr as *mut usize) as usize,
            (&key as *const u64) as usize,
        );
        (
            out_ptr.wrapping_sub(base),
            ret == (&mut out_ptr as *mut usize) as usize,
        )
    };

    for (key, len) in [(20u64, 3u32), (99u64, 3u32), (10u64, 0u32)] {
        let o = run(f_o, key, len);
        let n = run(f_n, key, len);
        crate::diff_eq!(o, n, "1410d3510 key={} len={}", key, len);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1410d3570(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_84a0(dst: usize, src: usize) {
        wr::<[u64; 3]>(dst, rd::<[u64; 3]>(src));
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;

    static mut ARR_3570: [u64; 9] = [0; 9];

    let run = |f: F, erase_idx: usize| -> ([u64; 9], u32, usize, bool) {
        ARR_3570 = [11, 12, 13, 21, 22, 23, 31, 32, 33];
        let base = core::ptr::addr_of_mut!(ARR_3570) as usize;
        let mut hdr: [u64; 2] = [base as u64, 3u64 | (4u64 << 32)];
        let mut out_it: usize = 0;
        let pos: usize = base + erase_idx * 0x18;
        let ret = f(
            hdr.as_mut_ptr() as usize,
            (&mut out_it as *mut usize) as usize,
            (&pos as *const usize) as usize,
        );
        (
            ARR_3570,
            rd::<u32>(hdr.as_ptr() as usize + 0x08),
            out_it.wrapping_sub(base),
            ret == (&mut out_it as *mut usize) as usize,
        )
    };

    for erase_idx in [0usize, 1, 2] {
        let o = run(f_o, erase_idx);
        let n = run(f_n, erase_idx);
        crate::diff_eq!(o, n, "1410d3570 erase_idx={}", erase_idx);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1410d3d20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, mode: u8| -> ([u64; 2], bool) {
        let mut tbl = [0u64; 8 * 3];
        for i in 0..8 {
            tbl[i * 3] = u64::MAX;
        }
        let key: u64 = 0x0102_0304_0506_0708;
        let mut h: u64 = 0xcbf2_9ce4_8422_2325;
        for b in key.to_le_bytes() {
            h = (h ^ (b as u64)).wrapping_mul(0x0100_0000_01b3);
        }
        let idx0 = (h & 7) as usize;
        let idx1 = ((idx0 + 1) & 7) as usize;

        let mut ht = [0u8; 0x70];
        let htp = ht.as_mut_ptr() as usize;
        wr::<u64>(htp + 0x28, u64::MAX - 1);
        wr::<u64>(htp + 0x30, if mode == 2 { 1 } else { 0 });
        wr::<u64>(htp + 0x40, 8);
        wr::<u64>(htp + 0x48, u64::MAX);
        let tp = tbl.as_mut_ptr() as usize;
        wr::<usize>(htp + 0x60, tp);

        match mode {
            0 => {
                wr::<u64>(tp + idx0 * 24, 0x9999);
                wr::<u64>(tp + idx1 * 24, key);
            }
            1 => {
                wr::<u64>(tp + idx0 * 24, 0x9999);
            }
            _ => {
                wr::<u64>(tp + idx0 * 24, u64::MAX - 1);
                wr::<u64>(tp + idx1 * 24, 0x9999);
            }
        }

        let mut out: [u64; 2] = [0, 0];
        let ret = f(
            htp,
            out.as_mut_ptr() as usize,
            (&key as *const u64) as usize,
        );
        (out, ret == out.as_mut_ptr() as usize)
    };

    for mode in [0u8, 1, 2] {
        let o = run(f_o, mode);
        let n = run(f_n, mode);
        crate::diff_eq!(o, n, "1410d3d20 mode={}", mode);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1410d5810(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_STEP: u32 = 0;
    static mut BUF_A_PTR: usize = 0;
    static mut BUF_B_PTR: usize = 0;
    static mut FREE_3200_CNT: u32 = 0;
    static mut RETIRE_71A0_CNT: u32 = 0;

    unsafe extern "win64" fn mock_52e0(dst: usize, src: usize) {
        CALL_STEP += 1;
        if CALL_STEP == 1 {
            wr::<usize>(dst, BUF_B_PTR);
            wr::<u32>(dst + 0x08, 1);
            wr::<u32>(dst + 0x0c, 2);
            wr::<usize>(src, BUF_A_PTR);
            wr::<u32>(src + 0x08, 1);
            wr::<u32>(src + 0x0c, 2);
        } else {
            wr::<usize>(dst, 0x7777_8888);
            wr::<u32>(dst + 0x08, 3);
            wr::<u32>(dst + 0x0c, 4);
        }
    }
    unsafe extern "win64" fn mock_3200(_seg: usize, _hdr: usize, _flag: u8, _ptr: usize) {
        FREE_3200_CNT += 1;
    }
    unsafe extern "win64" fn mock_71a0(_hdr: usize) {
        RETIRE_71A0_CNT += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_410d_52e0, mock_52e0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4253_3200, mock_3200 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4253_71a0, mock_71a0 as *const ())?;

    let mut arena_vec = vec![0u8; 0x0203_0000];
    let r10 = (arena_vec.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let run = |f: F, local_owner: bool| -> ([u64; 3], u64, u64, u32, u32, bool) {
        CALL_STEP = 0;
        FREE_3200_CNT = 0;
        RETIRE_71A0_CNT = 0;
        core::ptr::write_bytes(r10 as *mut u8, 0, 0x2_1000);

        BUF_A_PTR = r10 + 0x1_0000;
        BUF_B_PTR = r10 + 0x1_0100;
        wr::<u64>(BUF_A_PTR + 0x08, 0xaaaa_bbbb);
        wr::<u64>(BUF_B_PTR + 0x08, 0xcccc_dddd);

        let hdr = r10 + 0x108 + 0x50;
        wr::<u32>(hdr + 0x04, 0);
        wr::<u32>(hdr + 0x18, 2);
        wr::<usize>(r10 + 0x100, if local_owner { exe::teb() } else { 0 });

        let mut out: [u64; 3] = [0; 3];
        let key: u64 = 0x1234_5678_9abc_def0;
        let ret = f(
            0,
            out.as_mut_ptr() as usize,
            (&key as *const u64) as usize,
        );
        (
            out,
            rd::<u64>(BUF_A_PTR + 0x08),
            rd::<u64>(BUF_B_PTR + 0x08),
            FREE_3200_CNT,
            RETIRE_71A0_CNT,
            ret == out.as_mut_ptr() as usize,
        )
    };

    for local_owner in [true, false] {
        let o = run(f_o, local_owner);
        let n = run(f_n, local_owner);
        crate::diff_eq!(o, n, "1410d5810 local_owner={}", local_owner);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1410d6660(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut NEW_BUF_6660: [u64; 8 * 3] = [0; 8 * 3];
    static mut MOVE_CALLS: u32 = 0;

    unsafe extern "win64" fn mock_2180(_diff: u64, _cap: u32) -> usize {
        core::ptr::addr_of_mut!(NEW_BUF_6660) as usize
    }
    unsafe extern "win64" fn mock_84a0(dst: usize, src: usize) {
        wr::<[u64; 3]>(dst, rd::<[u64; 3]>(src));
    }
    unsafe extern "win64" fn mock_6bf0(first_ptr: usize, last_ptr: usize, mut dst: usize) {
        MOVE_CALLS += 1;
        let mut cur = rd::<usize>(first_ptr);
        let end = rd::<usize>(last_ptr);
        while cur != end {
            wr::<[u64; 3]>(dst, rd::<[u64; 3]>(cur));
            cur = cur.wrapping_add(0x18);
            dst = dst.wrapping_add(0x18);
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_401e_2180, mock_2180 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_84a0, mock_84a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_410d_6bf0, mock_6bf0 as *const ())?;

    static mut OLD_BUF_6660: [u64; 2 * 3] = [0; 2 * 3];

    let run = |f: F, ins_idx: usize| -> ([u64; 24], [u64; 6], u32, u32, usize, u32) {
        NEW_BUF_6660 = [0; 24];
        OLD_BUF_6660 = [1, 2, 3, 4, 5, 6];
        MOVE_CALLS = 0;

        let obp = core::ptr::addr_of_mut!(OLD_BUF_6660) as usize;
        let nbp = core::ptr::addr_of_mut!(NEW_BUF_6660) as usize;
        let mut vec_hdr: [u64; 2] = [obp as u64, 2u64 | ((0x4000_0002u64) << 32)];
        let pos: usize = obp + ins_idx * 0x18;
        let item: [u64; 3] = [91, 92, 93];

        let ret = f(
            vec_hdr.as_mut_ptr() as usize,
            (&pos as *const usize) as usize,
            item.as_ptr() as usize,
        );
        let hp = vec_hdr.as_ptr() as usize;
        (
            NEW_BUF_6660,
            OLD_BUF_6660,
            rd::<u32>(hp + 0x08),
            rd::<u32>(hp + 0x0c),
            ret.wrapping_sub(nbp),
            MOVE_CALLS,
        )
    };

    for ins_idx in [1usize, 2] {
        let o = run(f_o, ins_idx);
        let n = run(f_n, ins_idx);
        crate::diff_eq!(o, n, "1410d6660 ins_idx={}", ins_idx);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_14119f940(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALL_5D40: u32 = 0;
    static mut FREED_NODES: u32 = 0;
    static mut CALL_6780: (u64, u64) = (0, 0);

    unsafe extern "win64" fn mock_5d40(_this: usize, _first: usize, _head: usize) {
        CALL_5D40 += 1;
    }
    unsafe extern "win64" fn mock_66f0(_head: usize, _node: usize) {
        FREED_NODES += 1;
    }
    unsafe extern "win64" fn mock_6780(p18: u64, p20: u64, _head_ptr: usize) {
        CALL_6780 = (p18, p20);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4069_5d40, mock_5d40 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_66f0, mock_66f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_401e_6780, mock_6780 as *const ())?;

    let run = |f: F, small_table: bool| -> (u32, u32, (u64, u64), u64, bool, bool) {
        CALL_5D40 = 0;
        FREED_NODES = 0;
        CALL_6780 = (0, 0);

        let mut head = [0usize; 2];
        let mut n1 = [0usize; 2];
        let mut n2 = [0usize; 2];
        let hp = head.as_mut_ptr() as usize;
        let n1p = n1.as_mut_ptr() as usize;
        let n2p = n2.as_mut_ptr() as usize;
        wr::<usize>(hp, n1p);
        wr::<usize>(hp + 0x08, n2p);
        wr::<usize>(n1p, n2p);
        wr::<usize>(n2p, hp);

        let mut obj = [0u8; 0x40];
        let op = obj.as_mut_ptr() as usize;
        wr::<usize>(op + 0x08, hp);
        wr::<u64>(op + 0x10, 2);
        wr::<u64>(op + 0x18, 0x1111);
        wr::<u64>(op + 0x20, 0x2222);
        wr::<u64>(op + 0x38, if small_table { 8 } else { 64 });

        f(op);
        (
            CALL_5D40,
            FREED_NODES,
            CALL_6780,
            rd::<u64>(op + 0x10),
            rd::<usize>(hp) == hp,
            rd::<usize>(hp + 0x08) == hp,
        )
    };

    for small_table in [false, true] {
        let o = run(f_o, small_table);
        let n = run(f_n, small_table);
        crate::diff_eq!(o, n, "14119f940 small_table={}", small_table);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411bc750(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F, empty_nearby: bool| -> ([u8; 48], u64, u64) {
        let mut ctrl = [0x01u8; 48];
        if empty_nearby {
            ctrl[15] = 0x80;
            ctrl[18] = 0x80;
        }
        let cp = ctrl.as_mut_ptr() as usize;
        let mut tbl = [0u64; 6];
        let tp = tbl.as_mut_ptr() as usize;
        wr::<usize>(tp, cp);
        wr::<u64>(tp + 0x10, 5);
        wr::<u64>(tp + 0x18, 0x1f);
        wr::<u64>(tp + 0x28, 10);

        let it_ptr: usize = cp + 16;
        f(tp, (&it_ptr as *const usize) as usize);
        (ctrl, rd::<u64>(tp + 0x10), rd::<u64>(tp + 0x28))
    };

    for empty_nearby in [true, false] {
        let o = run(f_o, empty_nearby);
        let n = run(f_n, empty_nearby);
        crate::diff_eq!(o, n, "1411bc750 empty_nearby={}", empty_nearby);
    }
    Ok(2)
}

pub unsafe fn diff_unknown_1411c2820(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GROW_CAP: u64 = 0;
    static mut CB_D6F0: u32 = 0;
    static mut VEC_BUF_2820: [usize; 4] = [0; 4];

    unsafe extern "win64" fn mock_8030(vec_ptr: usize, new_cap: u64) {
        GROW_CAP = new_cap;
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
    }
    unsafe extern "win64" fn mock_d6f0(_ctx: usize, _obj: usize) {
        CB_D6F0 += 1;
    }

    let _m1 = crate::difftest::Mock::new(0x1_411c_8030, mock_8030 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4241_d6f0, mock_d6f0 as *const ())?;

    let run = |f: F, init_rc: u32| -> (u64, u32, u32, u32, usize, bool) {
        GROW_CAP = 0;
        CB_D6F0 = 0;
        VEC_BUF_2820 = [0; 4];
        let vbp = core::ptr::addr_of_mut!(VEC_BUF_2820) as usize;

        let mut item = [0u8; 0x38];
        let ip = item.as_mut_ptr() as usize;
        wr::<u32>(ip + 0x24, init_rc);
        wr::<usize>(ip + 0x28, 0x1234);

        let mut vec_hdr: [u64; 2] = [vbp as u64, 2u64 | (2u64 << 32)];
        let ret = f(
            vec_hdr.as_mut_ptr() as usize,
            (&ip as *const usize) as usize,
        );
        let hp = vec_hdr.as_ptr() as usize;
        (
            GROW_CAP,
            CB_D6F0,
            rd::<u32>(ip + 0x24),
            rd::<u32>(hp + 0x08),
            ret.wrapping_sub(vbp),
            VEC_BUF_2820[2] == ip,
        )
    };

    for init_rc in [0u32, 1] {
        let o = run(f_o, init_rc);
        let n = run(f_n, init_rc);
        crate::diff_eq!(o, n, "1411c2820 init_rc={}", init_rc);
    }
    Ok(2)
}

pub unsafe fn diff_d3d_1411daad0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    #[repr(align(16))]
    struct AlignedBuf([u64; 16]);
    static mut NEW_RING_BUF: AlignedBuf = AlignedBuf([0; 16]);

    let mut arena_vec = vec![0u8; 0x0203_0000];
    let r10 = (arena_vec.as_mut_ptr() as usize + 0x01ff_ffff) & !0x01ff_ffffusize;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, req_cap: u32| -> ([u64; 16], u32, u32, u32, u32, u32, u32) {
        NEW_RING_BUF = AlignedBuf([0; 16]);
        let nbp = core::ptr::addr_of_mut!(NEW_RING_BUF.0) as usize;

        core::ptr::write_bytes(r10 as *mut u8, 0, 0x1_1000);
        let old_buf = r10 + 0x1_0000;
        wr::<[u64; 4]>(old_buf, [100, 200, 300, 400]);
        let hdr = r10 + 0x108 + 0x50;
        wr::<u32>(hdr + 0x04, 0);
        wr::<u32>(hdr + 0x18, 2);
        wr::<usize>(r10 + 0x100, exe::teb());

        let mut page = [0u8; 0x30];
        let pp = page.as_mut_ptr() as usize;
        wr::<usize>(pp + 0x10, nbp);
        wr::<u32>(pp + 0x18, 4);

        let mut heap = [0u8; 0x100];
        let hp = heap.as_mut_ptr() as usize;
        wr::<usize>(hp + 0x48, pp);
        wr::<usize>(tls0 + 0x6348, hp);

        let mut q = [0u8; 0x20];
        let qp = q.as_mut_ptr() as usize;
        wr::<usize>(qp, old_buf);
        wr::<u32>(qp + 0x08, 2);
        wr::<u32>(qp + 0x0c, 1);
        wr::<u32>(qp + 0x10, 3);
        wr::<u32>(qp + 0x14, 4);

        f(qp, req_cap);
        wr::<usize>(tls0 + 0x6348, saved_heap);
        (
            NEW_RING_BUF.0,
            rd::<u32>(qp + 0x08),
            rd::<u32>(qp + 0x0c),
            rd::<u32>(qp + 0x10),
            rd::<u32>(qp + 0x14),
            rd::<u32>(pp + 0x18),
            rd::<u32>(hdr + 0x18),
        )
    };

    for req_cap in [4u32, 8] {
        let o = run(f_o, req_cap);
        let n = run(f_n, req_cap);
        crate::diff_eq!(o, n, "1411daad0 req_cap={}", req_cap);
    }
    Ok(2)
}

pub unsafe fn diff_d3d_1411db770(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, u64) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    #[repr(align(16))]
    struct AlignedNode([u8; 0x40]);
    static mut ALLOC_NODE: AlignedNode = AlignedNode([0; 0x40]);
    static mut CTOR_ARGS: (usize, u32, u64) = (0, 0, 0);

    unsafe extern "win64" fn mock_5070(_heap: usize, _sz: u32, _align: u32) -> usize {
        core::ptr::addr_of_mut!(ALLOC_NODE.0) as usize
    }
    unsafe extern "win64" fn mock_3c40(this: usize, v: u32, p4: u64) -> usize {
        CTOR_ARGS = (this, v, p4);
        this
    }

    let _m1 = crate::difftest::Mock::new(0x1_4253_5070, mock_5070 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_3c40, mock_3c40 as *const ())?;

    let tls0 = rd::<usize>(exe::tls_pointer());
    let saved_heap = rd::<usize>(tls0 + 0x6348);

    let run = |f: F, use_freelist: bool| -> (usize, (usize, u32, u64), u32, usize) {
        ALLOC_NODE = AlignedNode([0; 0x40]);
        CTOR_ARGS = (0, 0, 0);
        let np = core::ptr::addr_of_mut!(ALLOC_NODE.0) as usize;
        wr::<usize>(np, 0x1234_5678);

        let mut page = [0u8; 0x30];
        let pp = page.as_mut_ptr() as usize;
        wr::<usize>(pp + 0x10, if use_freelist { np } else { 0 });
        wr::<u32>(pp + 0x18, 7);

        let mut heap = [0u8; 0x60];
        let hp = heap.as_mut_ptr() as usize;
        wr::<usize>(hp + 0x40, pp);
        wr::<usize>(tls0 + 0x6348, hp);

        let v: u32 = 0xfeed_1234;
        let ret = f(0, 0, (&v as *const u32) as usize, 0xcafe_0001);
        wr::<usize>(tls0 + 0x6348, saved_heap);
        (ret, CTOR_ARGS, rd::<u32>(pp + 0x18), rd::<usize>(pp + 0x10))
    };

    for use_freelist in [true, false] {
        let o = run(f_o, use_freelist);
        let n = run(f_n, use_freelist);
        crate::diff_eq!(o, n, "1411db770 use_freelist={}", use_freelist);
    }
    Ok(2)
}
