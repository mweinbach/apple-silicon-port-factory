//! Decompiled hot functions shard W3 (`0x141353470..0x141c6a020`).

use crate::exe::{self, rd, wr};

/// `rend_141353470` (`rend::TrackedSparseUploadBuffer<rend::GeometryTransform::LeafBone>::vfunc_1`)
/// Original: 0x141353470 (656 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_141353470(this: usize) {
    let words_base = rd::<usize>(this + 0xa8);
    let word_cnt = rd::<u64>(this + 0xb8).wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32).wrapping_shl(6).wrapping_add(tz);
            if (elem_idx as u64) < rd::<u64>(this + 0x90) {
                let vec_ptr = this + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if cur_len.wrapping_mul(20) == cur_cap.wrapping_mul(20) {
                    let min_cap = cur_len.wrapping_add(1);
                    let grown = (cur_cap >> 1).wrapping_add(cur_cap);
                    let new_cap = if grown < min_cap { min_cap } else { grown };
                    exe::call_va!(0x1_4134_dfb0, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot = rd::<usize>(vec_ptr).wrapping_add(idx_now * 20);
                wr::<u64>(slot, 0);
                wr::<u64>(slot + 0x0c, 0);
                wr::<u32>(slot + 0x08, 0xffff_ffff);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot_end = rd::<usize>(vec_ptr).wrapping_add(new_len * 20);
                wr::<u32>(slot_end - 0x14, elem_idx);
                let src_ptr = rd::<usize>(this + 0x98).wrapping_add((elem_idx as usize) * 12);
                wr::<u64>(slot_end - 0x0c, rd::<u64>(src_ptr));
                wr::<u32>(slot_end - 0x04, rd::<u32>(src_ptr + 0x08));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    exe::call_va!(
        0x1_412e_c110,
        (this + 0x08, rd::<u32>(this + 0xa0) as u64, this + 0x58)
    );
    let count = rd::<u32>(this + 0xc8);
    if count != 0 {
        exe::call_va!(
            0x1_412f_9440,
            (this + 0x10, count as u64, this + 0x70, this + 0x88)
        );
        let byte_len = (count as u64).wrapping_mul(20);
        let mapped = exe::call_va!(
            0x1_4209_c560,
            (rd::<usize>(this + 0x10), 0usize, byte_len, 2u32) -> u64
        );
        wr::<u64>(this + 0x38, mapped);
        wr::<u8>(this + 0x30, 1);
        wr::<u32>(this + 0x20, count);

        let prev_off = rd::<u32>(this + 0x40);
        wr::<u32>(this + 0x40, prev_off.wrapping_add(count));
        let dst_ptr = rd::<usize>(this + 0x38).wrapping_add((prev_off as usize) * 20);
        let src_ptr = rd::<usize>(this + 0xc0);
        let _ = exe::call_va!(0x1_4183_acbc, (dst_ptr, src_ptr, byte_len) -> usize);
        if rd::<u32>(this + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<usize>(this + 0x10), 0u32));
            wr::<u64>(this + 0x38, 0);
            wr::<u8>(this + 0x30, 0);
            wr::<u32>(this + 0x40, 0);
        }
    }

    let pending = rd::<u32>(this + 0x20);
    if pending != 0 && rd::<u8>(this + 0x29) == 0 {
        exe::call_va!(
            0x1_4237_d500,
            (
                rd::<usize>(this + 0x08),
                rd::<usize>(this + 0x10),
                pending,
                0x0cu32,
                0u8
            )
        );
        wr::<u32>(this + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let vec_ptr = this + 0xc0;
        let len = rd::<u32>(vec_ptr + 0x08) as u64;
        let len_bytes = len.wrapping_mul(20);
        if len_bytes > 0x10_0000 {
            let cap_bytes = (rd::<u32>(vec_ptr + 0x0c) as u64).wrapping_mul(20);
            if len_bytes != cap_bytes {
                exe::call_va!(0x1_4134_dfb0, (vec_ptr, len));
            }
        }
    }
    wr::<u32>(this + 0xc8, 0);
}

/// `rend_141353b70` (`rend::TrackedSparseUploadBuffer<rend::GeometryTransform::CompressedGeometryId>::vfunc_1`)
/// Original: 0x141353b70 (631 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_141353b70(this: usize) {
    let words_base = rd::<usize>(this + 0xa8);
    let word_cnt = rd::<u64>(this + 0xb8).wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32).wrapping_shl(6).wrapping_add(tz);
            if (elem_idx as u64) < rd::<u64>(this + 0x90) {
                let vec_ptr = this + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if (cur_len << 4) == (cur_cap << 4) {
                    let min_cap = cur_len.wrapping_add(1);
                    let grown = (cur_cap >> 1).wrapping_add(cur_cap);
                    let new_cap = if grown < min_cap { min_cap } else { grown };
                    exe::call_va!(0x1_40a5_b830, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot = rd::<usize>(vec_ptr).wrapping_add(idx_now << 4);
                wr::<u128>(slot, 0);
                wr::<u32>(slot + 0x0c, 0xffff_ffff);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot_end = rd::<usize>(vec_ptr).wrapping_add(new_len << 4);
                wr::<u32>(slot_end - 0x10, elem_idx);
                let src_ptr = rd::<usize>(this + 0x98).wrapping_add((elem_idx as usize) * 8);
                wr::<u64>(slot_end - 0x08, rd::<u64>(src_ptr));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    let rcx_bytes = rd::<u64>(this + 0x60).wrapping_mul(rd::<u32>(this + 0xa0) as u64);
    let mut r9_sz = rd::<u64>(this + 0x68);
    if rcx_bytes >= r9_sz {
        let sum = r9_sz.wrapping_sub(1).wrapping_add(rcx_bytes);
        r9_sz = sum.wrapping_sub(sum % r9_sz);
    }
    let cur_buf_sz = rd::<u64>(rd::<usize>(this + 0x08) + 0x10);
    if cur_buf_sz < r9_sz
        || (rd::<u8>(exe::ptr::<u8>(0x1_4386_8280) as usize) != 0 && cur_buf_sz > r9_sz)
    {
        exe::call_va!(0x1_422e_2520, (this + 0x08, r9_sz));
    }

    let count = rd::<u32>(this + 0xc8);
    if count != 0 {
        exe::call_va!(
            0x1_412f_9440,
            (this + 0x10, count as u64, this + 0x70, this + 0x88)
        );
        let byte_len = (count as u64) << 4;
        let mapped = exe::call_va!(
            0x1_4209_c560,
            (rd::<usize>(this + 0x10), 0usize, byte_len, 2u32) -> u64
        );
        wr::<u64>(this + 0x38, mapped);
        wr::<u8>(this + 0x30, 1);
        wr::<u32>(this + 0x20, count);

        let prev_off = rd::<u32>(this + 0x40);
        wr::<u32>(this + 0x40, prev_off.wrapping_add(count));
        let dst_ptr = rd::<usize>(this + 0x38).wrapping_add((prev_off as usize) << 4);
        let src_ptr = rd::<usize>(this + 0xc0);
        let _ = exe::call_va!(0x1_4183_acbc, (dst_ptr, src_ptr, byte_len) -> usize);
        if rd::<u32>(this + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<usize>(this + 0x10), 0u32));
            wr::<u64>(this + 0x38, 0);
            wr::<u8>(this + 0x30, 0);
            wr::<u32>(this + 0x40, 0);
        }
    }

    let pending = rd::<u32>(this + 0x20);
    if pending != 0 && rd::<u8>(this + 0x29) == 0 {
        exe::call_va!(
            0x1_4237_d500,
            (
                rd::<usize>(this + 0x08),
                rd::<usize>(this + 0x10),
                pending,
                0x08u32,
                0u8
            )
        );
        wr::<u32>(this + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let vec_ptr = this + 0xc0;
        let len = rd::<u32>(vec_ptr + 0x08) as u64;
        let len_bytes = len << 4;
        if len_bytes > 0x10_0000 {
            let cap_bytes = (rd::<u32>(vec_ptr + 0x0c) as u64) << 4;
            if len_bytes != cap_bytes {
                exe::call_va!(0x1_40a5_b830, (vec_ptr, len));
            }
        }
    }
    wr::<u32>(this + 0xc8, 0);
}

/// `rend_141354150` (`rend::TrackedSparseUploadBuffer<rend::GeometryTransform::BoundsUpdateInstance>::vfunc_1`)
/// Original: 0x141354150 (700 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_141354150(this: usize) {
    let words_base = rd::<usize>(this + 0xa8);
    let word_cnt = rd::<u64>(this + 0xb8).wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32).wrapping_shl(6).wrapping_add(tz);
            if (elem_idx as u64) < rd::<u64>(this + 0x90) {
                let vec_ptr = this + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if cur_len.wrapping_mul(24) == cur_cap.wrapping_mul(24) {
                    let min_cap = cur_len.wrapping_add(1);
                    let grown = (cur_cap >> 1).wrapping_add(cur_cap);
                    let new_cap = if grown < min_cap { min_cap } else { grown };
                    exe::call_va!(0x1_408c_e8a0, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot = rd::<usize>(vec_ptr).wrapping_add(idx_now * 24);
                wr::<u128>(slot, 0);
                wr::<u64>(slot + 0x10, 0);
                wr::<u32>(slot + 0x08, 0xffff_ffff);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot_end = rd::<usize>(vec_ptr).wrapping_add(new_len * 24);
                wr::<u32>(slot_end - 0x18, elem_idx);
                let src_ptr = rd::<usize>(this + 0x98).wrapping_add((elem_idx as usize) * 16);
                wr::<u128>(slot_end - 0x10, rd::<u128>(src_ptr));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    let rcx_bytes = rd::<u64>(this + 0x60).wrapping_mul(rd::<u32>(this + 0xa0) as u64);
    let mut r9_sz = rd::<u64>(this + 0x68);
    if rcx_bytes >= r9_sz {
        let sum = r9_sz.wrapping_sub(1).wrapping_add(rcx_bytes);
        r9_sz = sum.wrapping_sub(sum % r9_sz);
    }
    let cur_buf_sz = rd::<u64>(rd::<usize>(this + 0x08) + 0x10);
    if cur_buf_sz < r9_sz
        || (rd::<u8>(exe::ptr::<u8>(0x1_4386_8280) as usize) != 0 && cur_buf_sz > r9_sz)
    {
        exe::call_va!(0x1_4226_1020, (this + 0x08, r9_sz));
    }

    let count = rd::<u32>(this + 0xc8);
    if count != 0 {
        exe::call_va!(
            0x1_412f_9440,
            (this + 0x10, count as u64, this + 0x70, this + 0x88)
        );
        let byte_len = (count as u64).wrapping_mul(24);
        let mapped = exe::call_va!(
            0x1_4209_c560,
            (rd::<usize>(this + 0x10), 0usize, byte_len, 2u32) -> u64
        );
        wr::<u64>(this + 0x38, mapped);
        wr::<u8>(this + 0x30, 1);
        wr::<u32>(this + 0x20, count);

        let prev_off = rd::<u32>(this + 0x40);
        wr::<u32>(this + 0x40, prev_off.wrapping_add(count));
        let dst_ptr = rd::<usize>(this + 0x38).wrapping_add((prev_off as usize) * 24);
        let src_ptr = rd::<usize>(this + 0xc0);
        let _ = exe::call_va!(0x1_4183_acbc, (dst_ptr, src_ptr, byte_len) -> usize);
        if rd::<u32>(this + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<usize>(this + 0x10), 0u32));
            wr::<u64>(this + 0x38, 0);
            wr::<u8>(this + 0x30, 0);
            wr::<u32>(this + 0x40, 0);
        }
    }

    let pending = rd::<u32>(this + 0x20);
    if pending != 0 && rd::<u8>(this + 0x29) == 0 {
        exe::call_va!(
            0x1_4237_d500,
            (
                rd::<usize>(this + 0x08),
                rd::<usize>(this + 0x10),
                pending,
                0x10u32,
                0u8
            )
        );
        wr::<u32>(this + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let vec_ptr = this + 0xc0;
        let len = rd::<u32>(vec_ptr + 0x08) as u64;
        let len_bytes = len.wrapping_mul(24);
        if len_bytes > 0x10_0000 {
            let cap_bytes = (rd::<u32>(vec_ptr + 0x0c) as u64).wrapping_mul(24);
            if len_bytes != cap_bytes {
                exe::call_va!(0x1_408c_e8a0, (vec_ptr, len));
            }
        }
    }
    wr::<u32>(this + 0xc8, 0);
}

/// `rend_1413558a0` (`rend::SparseUploadBuffer<rend::VertexBindingData>::vfunc_1`)
/// Original: 0x1413558a0 (862 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_1413558a0(this: usize) {
    let words_base = rd::<usize>(this + 0xa8);
    let word_cnt = rd::<u64>(this + 0xb8).wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32).wrapping_shl(6).wrapping_add(tz);
            if (elem_idx as u64) < rd::<u64>(this + 0x90) {
                let vec_ptr = this + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if cur_len.wrapping_mul(48) == cur_cap.wrapping_mul(48) {
                    let min_cap = cur_len.wrapping_add(1);
                    let grown = (cur_cap >> 1).wrapping_add(cur_cap);
                    let new_cap = if grown < min_cap { min_cap } else { grown };
                    exe::call_va!(0x1_4070_1e10, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot = rd::<usize>(vec_ptr).wrapping_add(idx_now * 48);
                wr::<[u64; 6]>(slot, [0; 6]);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot_end = rd::<usize>(vec_ptr).wrapping_add(new_len * 48);
                wr::<u32>(slot_end - 0x30, elem_idx);
                let src_ptr = rd::<usize>(this + 0x98).wrapping_add((elem_idx as usize) * 40);
                wr::<[u64; 5]>(slot_end - 0x28, rd::<[u64; 5]>(src_ptr));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    exe::call_va!(
        0x1_412e_c110,
        (this + 0x08, rd::<u32>(this + 0xa0) as u64, this + 0x58)
    );
    let count = rd::<u32>(this + 0xc8);
    if count != 0 {
        let needed = (count as u64).wrapping_mul(rd::<u64>(this + 0x78));
        let mut alloc_sz = rd::<u64>(this + 0x80);
        if needed >= alloc_sz {
            let sum = alloc_sz.wrapping_add(needed).wrapping_sub(1);
            alloc_sz = sum.wrapping_sub(sum % alloc_sz);
        }
        let (_, now_xmm0) = crate::simd_ops::call_va8_xmm(
            0x1_4240_69a0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            [0; 4],
        );
        let now_sec = f64::from_bits(now_xmm0 as u64);
        let r14 = rd::<usize>(this + 0x10);
        let cur_sz = rd::<u64>(r14 + 0x10);
        let thresh = rd::<f64>(exe::ptr::<f64>(0x1_4342_6a48) as usize);
        if cur_sz < alloc_sz
            || (rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0
                && cur_sz > alloc_sz
                && (now_sec - rd::<f64>(this + 0x88)) > thresh)
        {
            exe::call_va!(0x1_4209_d670, (r14));
            exe::call_va!(
                0x1_4207_5af0,
                (alloc_sz, 0u64, 0u64, 0x11u32, 0u32, 0u64, r14)
            );
            wr::<u32>(r14 + 0x50, u32::from(alloc_sz > 0x40_0000) + 2);
            wr::<f64>(this + 0x88, now_sec);
        }

        let byte_len = (count as u64).wrapping_mul(48);
        let mapped = exe::call_va!(
            0x1_4209_c560,
            (rd::<usize>(this + 0x10), 0usize, byte_len, 2u32) -> u64
        );
        wr::<u64>(this + 0x38, mapped);
        wr::<u8>(this + 0x30, 1);
        wr::<u32>(this + 0x20, count);

        let prev_off = rd::<u32>(this + 0x40);
        wr::<u32>(this + 0x40, prev_off.wrapping_add(count));
        let dst_ptr = rd::<usize>(this + 0x38).wrapping_add((prev_off as usize) * 48);
        let src_ptr = rd::<usize>(this + 0xc0);
        let _ = exe::call_va!(0x1_4183_acbc, (dst_ptr, src_ptr, byte_len) -> usize);
        if rd::<u32>(this + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<usize>(this + 0x10), 0u32));
            wr::<u64>(this + 0x38, 0);
            wr::<u8>(this + 0x30, 0);
            wr::<u32>(this + 0x40, 0);
        }
    }

    let pending = rd::<u32>(this + 0x20);
    if pending != 0 && rd::<u8>(this + 0x29) == 0 {
        exe::call_va!(
            0x1_4238_9840,
            (
                rd::<usize>(this + 0x08),
                rd::<usize>(this + 0x10),
                pending,
                0x28u32,
                0u8
            )
        );
        wr::<u32>(this + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let vec_ptr = this + 0xc0;
        let len = rd::<u32>(vec_ptr + 0x08) as u64;
        let len_bytes = len.wrapping_mul(48);
        if len_bytes > 0x10_0000 {
            let cap_bytes = (rd::<u32>(vec_ptr + 0x0c) as u64).wrapping_mul(48);
            if len_bytes != cap_bytes {
                exe::call_va!(0x1_4070_1e10, (vec_ptr, len));
            }
        }
    }
    wr::<u32>(this + 0xc8, 0);
}

/// `rend_141356660` (`rend::SparseUploadBuffer<rend::GeometryData>::vfunc_1`)
/// Original: 0x141356660 (720 bytes, Throws::Caught)
pub unsafe extern "win64" fn rend_141356660(this: usize) {
    let words_base = rd::<usize>(this + 0xa8);
    let word_cnt = rd::<u64>(this + 0xb8).wrapping_add(0x3f) >> 6;
    for w_idx in 0..word_cnt {
        let word_ptr = words_base.wrapping_add((w_idx as usize) * 8);
        let mut word = rd::<u64>(word_ptr);
        while word != 0 {
            let tz = word.trailing_zeros();
            let elem_idx = (w_idx as u32).wrapping_shl(6).wrapping_add(tz);
            if (elem_idx as u64) < rd::<u64>(this + 0x90) {
                let vec_ptr = this + 0xc0;
                let cur_len = rd::<u32>(vec_ptr + 0x08) as u64;
                let cur_cap = rd::<u32>(vec_ptr + 0x0c) as u64;
                if cur_len.wrapping_mul(40) == cur_cap.wrapping_mul(40) {
                    let min_cap = cur_len.wrapping_add(1);
                    let grown = (cur_cap >> 1).wrapping_add(cur_cap);
                    let new_cap = if grown < min_cap { min_cap } else { grown };
                    exe::call_va!(0x1_411e_8620, (vec_ptr, new_cap));
                }
                let idx_now = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot = rd::<usize>(vec_ptr).wrapping_add(idx_now * 40);
                wr::<[u64; 5]>(slot, [0; 5]);
                wr::<u32>(vec_ptr + 0x08, (idx_now as u32).wrapping_add(1));

                let new_len = rd::<u32>(vec_ptr + 0x08) as usize;
                let slot_end = rd::<usize>(vec_ptr).wrapping_add(new_len * 40);
                wr::<u32>(slot_end - 0x28, elem_idx);
                let src_ptr = rd::<usize>(this + 0x98).wrapping_add((elem_idx as usize) * 32);
                wr::<[u64; 4]>(slot_end - 0x20, rd::<[u64; 4]>(src_ptr));
            }
            word = rd::<u64>(word_ptr) ^ (1u64 << (tz & 0x3f));
            wr::<u64>(word_ptr, word);
        }
    }

    let rcx_bytes = rd::<u64>(this + 0x60).wrapping_mul(rd::<u32>(this + 0xa0) as u64);
    let mut r9_sz = rd::<u64>(this + 0x68);
    if rcx_bytes >= r9_sz {
        let sum = r9_sz.wrapping_sub(1).wrapping_add(rcx_bytes);
        r9_sz = sum.wrapping_sub(sum % r9_sz);
    }
    let cur_buf_sz = rd::<u64>(rd::<usize>(this + 0x08) + 0x10);
    if cur_buf_sz < r9_sz
        || (rd::<u8>(exe::ptr::<u8>(0x1_4386_8280) as usize) != 0 && cur_buf_sz > r9_sz)
    {
        exe::call_va!(0x1_4237_df60, (this + 0x08, r9_sz));
    }

    let count = rd::<u32>(this + 0xc8);
    if count != 0 {
        exe::call_va!(
            0x1_412f_9440,
            (this + 0x10, count as u64, this + 0x70, this + 0x88)
        );
        let byte_len = (count as u64).wrapping_mul(40);
        let mapped = exe::call_va!(
            0x1_4209_c560,
            (rd::<usize>(this + 0x10), 0usize, byte_len, 2u32) -> u64
        );
        wr::<u64>(this + 0x38, mapped);
        wr::<u8>(this + 0x30, 1);
        wr::<u32>(this + 0x20, count);

        let prev_off = rd::<u32>(this + 0x40);
        wr::<u32>(this + 0x40, prev_off.wrapping_add(count));
        let dst_ptr = rd::<usize>(this + 0x38).wrapping_add((prev_off as usize) * 40);
        let src_ptr = rd::<usize>(this + 0xc0);
        let _ = exe::call_va!(0x1_4183_acbc, (dst_ptr, src_ptr, byte_len) -> usize);
        if rd::<u32>(this + 0x20) != 0 {
            exe::call_va!(0x1_4209_d6f0, (rd::<usize>(this + 0x10), 0u32));
            wr::<u64>(this + 0x38, 0);
            wr::<u8>(this + 0x30, 0);
            wr::<u32>(this + 0x40, 0);
        }
    }

    let pending = rd::<u32>(this + 0x20);
    if pending != 0 && rd::<u8>(this + 0x29) == 0 {
        exe::call_va!(
            0x1_4238_9840,
            (
                rd::<usize>(this + 0x08),
                rd::<usize>(this + 0x10),
                pending,
                0x20u32,
                0u8
            )
        );
        wr::<u32>(this + 0x20, 0);
    }

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8298) as usize) != 0 {
        let vec_ptr = this + 0xc0;
        let len = rd::<u32>(vec_ptr + 0x08) as u64;
        let len_bytes = len.wrapping_mul(40);
        if len_bytes > 0x10_0000 {
            let cap_bytes = (rd::<u32>(vec_ptr + 0x0c) as u64).wrapping_mul(40);
            if len_bytes != cap_bytes {
                exe::call_va!(0x1_411e_8620, (vec_ptr, len));
            }
        }
    }
    wr::<u32>(this + 0xc8, 0);
}

/// `r_1413ab4d0` (`r::MetadataManager::LoadBatch::vfunc_1`)
/// Original: 0x1413ab4d0 (691 bytes, Throws::No)
pub unsafe extern "win64" fn r_1413ab4d0(param_1: usize, param_2: usize) {
    let r8 = rd::<usize>(param_2 + 0x58);
    let r9 = rd::<usize>(param_1 + 0x28).wrapping_add(r8.wrapping_mul(0x88));
    let src_item = rd::<usize>(param_1 + 0x18).wrapping_add(r8.wrapping_mul(0x18));
    wr::<u64>(r9, rd::<u64>(src_item));
    let rdi = r9 + 0x10;
    let _ = exe::call_va!(0x1_4240_a730, (rdi, param_2) -> usize);
    wr::<u64>(rdi + 0x18, rd::<u64>(param_2 + 0x18));
    let _ = exe::call_va!(0x1_4249_68a0, (rdi + 0x20, param_2 + 0x20) -> usize);
    wr::<u32>(rdi + 0x50, rd::<u32>(param_2 + 0x50));
    wr::<u64>(rdi + 0x58, rd::<u64>(param_2 + 0x58));

    let prev = crate::simd_ops::atomic_xadd_u32((param_1 + 0x38) as u64, 0xffff_ffff);
    if prev == 1 {
        if rd::<u64>(exe::ptr::<u64>(0x1_43a0_82d0) as usize) == 0 {
            let r12 = rd::<u32>(param_1 + 0x30) as u64;
            let r13 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
            let denom = rd::<u64>(r13);
            let split = if r12 > denom { r12 / denom } else { 1 };
            let rsi_sched = rd::<usize>(r13 + 0x10);
            let wid1 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool1 = rd::<usize>(rsi_sched.wrapping_add(wid1 << 6).wrapping_add(0x48))
                .wrapping_add(0x40);
            let mut h_counter = [0u64; 2];
            let _ = exe::call_va!(
                0x1_4248_5aa0,
                (pool1, h_counter.as_mut_ptr() as usize) -> usize
            );
            let cnt_ptr = h_counter[0] as usize;
            let _ = crate::simd_ops::atomic_xchg_u32(cnt_ptr as u64, 1);
            wr::<u64>(cnt_ptr + 0x08, 0);
            wr::<usize>(cnt_ptr + 0x10, exe::ptr::<u8>(0x1_4340_e320) as usize);
            wr::<u64>(cnt_ptr + 0x18, 0);
            wr::<u64>(cnt_ptr + 0x20, 0);
            let (ok, _) = crate::simd_ops::atomic_cmpxchg_u8((cnt_ptr + 0x04) as u64, 1, 3);
            if ok {
                let _ = crate::simd_ops::atomic_add_u32(cnt_ptr as u64, 0xffff);
            }
            wr::<u8>((h_counter.as_mut_ptr() as usize) + 0x0a, 1);

            if r12 != 0 {
                let rdi_sched = rd::<usize>(r13 + 0x10);
                let wid2 = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
                let pool2 = rd::<usize>(rdi_sched.wrapping_add(wid2 << 6).wrapping_add(0x48))
                    .wrapping_add(0xc0);
                let mut h_job = [0u64; 2];
                let _ = exe::call_va!(
                    0x1_4248_5aa0,
                    (pool2, h_job.as_mut_ptr() as usize) -> usize
                );
                let job_ptr = h_job[0] as usize;
                let _ = crate::simd_ops::atomic_xchg_u32(job_ptr as u64, 1);
                wr::<u64>(job_ptr + 0x08, 0);
                wr::<usize>(job_ptr + 0x10, exe::ptr::<u8>(0x1_4340_a3d0) as usize);
                wr::<u64>(job_ptr + 0x18, h_counter[0]);
                wr::<u64>(job_ptr + 0x20, h_counter[1]);
                wr::<u64>(job_ptr + 0x28, 3);
                wr::<u64>(job_ptr + 0x30, r12);
                wr::<usize>(job_ptr + 0x38, param_1);
                wr::<u64>(job_ptr + 0x40, split);
                let _ = crate::simd_ops::atomic_add_u32(h_counter[0], 1);
                wr::<u64>(job_ptr + 0x08, h_counter[0]);
                if rd::<u32>(h_job[0] as usize) == 1 {
                    let sched = rd::<usize>(r13 + 0x10);
                    exe::call_va!(0x1_4248_6460, (sched, h_job[0] as usize));
                    let sem_owner = rd::<usize>(sched);
                    let _ = exe::call_iat!(
                        0x1_4264_5580,
                        (rd::<u64>(sem_owner + 0x1008), 1u32, 0usize) -> i32
                    );
                }
            }
            wr::<u64>(param_1 + 0x58, h_counter[0]);
            wr::<u64>(param_1 + 0x60, h_counter[1]);
        } else {
            let r13 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
            let rdi_sched = rd::<usize>(r13 + 0x10);
            let wid = exe::call_va!(0x1_4240_7c80, () -> u32) as usize;
            let pool = rd::<usize>(rdi_sched.wrapping_add(wid << 6).wrapping_add(0x48))
                .wrapping_add(0xc0);
            let mut h_job = [0u64; 2];
            let _ = exe::call_va!(
                0x1_4248_5aa0,
                (pool, h_job.as_mut_ptr() as usize) -> usize
            );
            let job_ptr = h_job[0] as usize;
            let _ = crate::simd_ops::atomic_xchg_u32(job_ptr as u64, 1);
            wr::<u64>(job_ptr + 0x08, 0);
            wr::<usize>(job_ptr + 0x10, exe::ptr::<u8>(0x1_4340_a418) as usize);
            wr::<usize>(job_ptr + 0x18, param_1);
            wr::<u64>(param_1 + 0x58, h_job[0]);
            wr::<u64>(param_1 + 0x60, h_job[1]);
            exe::call_va!(
                0x1_4248_5930,
                (
                    rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize),
                    param_1 + 0x58
                )
            );
        }
        exe::call_iat!(0x1_4264_55a8, (param_1 + 0x48));
        wr::<u32>(param_1 + 0x50, rd::<u32>(param_1 + 0x50).wrapping_add(1));
        exe::call_iat!(0x1_4264_5598, (param_1 + 0x48));
        exe::call_iat!(0x1_4264_55c0, (param_1 + 0x40));
    }

    let rdx_30 = rd::<u64>(param_2 + 0x30);
    if rdx_30 != 0 {
        let rcx_48 = rd::<usize>(param_2 + 0x48);
        let r9d = rd::<u16>(param_2 + 0x40) as u32;
        let vtbl = rd::<usize>(rcx_48);
        let r8_38 = rd::<u64>(param_2 + 0x38);
        exe::call_va!(rd::<usize>(vtbl + 0x10), (rcx_48, rdx_30, r8_38, r9d));
    }
    exe::call_va!(0x1_4240_ab10, (param_2));
}

/// `ui_141410620` (`ui::PageContent::vfunc_7`)
/// Original: 0x141410620 (16 bytes, Throws::Discard)
pub unsafe extern "win64" fn ui_141410620(param_1: usize) {
    if rd::<u32>(param_1 + 0x08) != 0xffff_ffff {
        exe::call_va!(0x1_4250_2f50, (rd::<u64>(param_1 + 0x10)));
    }
}

/// `unknown_1415ee330`
/// Original: 0x1415ee330 (1077 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1415ee330() {
    let mut rbx = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c10) as usize);
    let r8 = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize) as usize;
    if rbx != rbx.wrapping_add(r8 * 8) {
        loop {
            let rcx = rd::<usize>(rbx);
            if rd::<u32>(rcx + 0x100) == 1 {
                exe::call_va!(0x1_4160_0880, (rcx));
            }
            rbx = rbx.wrapping_add(8);
            let cur_r8 = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize) as usize;
            let cur_rdx = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c10) as usize);
            if rbx == cur_rdx.wrapping_add(cur_r8 * 8) {
                break;
            }
        }
    }

    exe::call_va!(0x1_4164_7860, ());
    let _ = exe::call_va!(0x1_415f_2390, () -> u32);

    if rd::<u32>(exe::ptr::<u32>(0x1_4393_1c08) as usize) > 0 {
        let c_8d0 = rd::<f32>(exe::ptr::<f32>(0x1_4342_68d0) as usize);
        let c_48c = rd::<f32>(exe::ptr::<f32>(0x1_4267_648c) as usize);
        let c_e1c = rd::<f32>(exe::ptr::<f32>(0x1_4342_6e1c) as usize);
        let c_eb8 = rd::<f32>(exe::ptr::<f32>(0x1_4342_6eb8) as usize);
        let c_64c = rd::<f32>(exe::ptr::<f32>(0x1_4267_664c) as usize);
        let mut esi: u32 = 0;
        loop {
            let arr = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c00) as usize);
            let node = rd::<usize>(arr.wrapping_add((esi as usize) * 8));
            let cl = rd::<u8>(node + 0x1b4);
            if (cl & 1) == 0 || (cl & 2) != 0 {
                let edx: u32 = if (cl & 1) == 0 { 0x0f } else { 0 };
                exe::call_va!(0x1_4163_bee0, (node, edx));
            }
            let rdi = rd::<usize>(node + 0x08);
            if rdi != 0 {
                let vtbl = rd::<usize>(rdi);
                if (rd::<u8>(rdi + 0x1f8) & 1) == 0 {
                    exe::call_va!(rd::<usize>(vtbl + 0x68), (rdi, 0u32));
                    exe::call_va!(0x1_415f_2db0, (node));
                } else {
                    exe::call_va!(rd::<usize>(vtbl + 0x70), (rdi));
                    if (rd::<u8>(rdi + 0x1f8) & 0x10) != 0 {
                        let vtbl2 = rd::<usize>(rdi);
                        exe::call_va!(rd::<usize>(vtbl2 + 0x78), (rdi));
                    }
                    exe::call_va!(0x1_415f_2db0, (node));
                }
            } else {
                wr::<u32>(node + 0x10, 0x3f80_0000);
            }

            if rd::<usize>(node + 0x08) != 0 && rd::<usize>(node + 0x38) != 0 {
                exe::call_va!(0x1_4164_5fc0, (node, 0u8));
            } else {
                let bits = rd::<u32>(node + 0x10);
                let mant_f = f32::from_bits((bits & 0x007f_ffff).wrapping_add(0x3f80_0000));
                let exp_u = ((bits >> 23) & 0xff) as f32;
                let mut xmm2 = mant_f * mant_f;
                let xmm1 = mant_f + c_8d0;
                xmm2 *= mant_f;
                let mut xmm0 = xmm1 * xmm1;
                xmm2 -= c_8d0;
                xmm0 *= xmm1;
                xmm2 *= c_48c;
                xmm2 /= xmm0;
                xmm2 *= c_e1c;
                let term2 = (exp_u - c_eb8) * c_64c;
                xmm2 += term2;
                wr::<f32>(node + 0x14, xmm2);
            }

            esi = esi.wrapping_add(1);
            if esi >= rd::<u32>(exe::ptr::<u32>(0x1_4393_1c08) as usize) {
                break;
            }
        }
    }

    let _ = exe::call_va!(0x1_415f_2390, () -> u32);

    let mut rbx2 = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c10) as usize);
    let cnt2 = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize) as usize;
    if rbx2 != rbx2.wrapping_add(cnt2 * 8) {
        loop {
            let rcx = rd::<usize>(rbx2);
            if rd::<u32>(rcx + 0x100) == 1 {
                exe::call_va!(0x1_4164_5fc0, (rcx, 1u8));
            }
            rbx2 = rbx2.wrapping_add(8);
            let cur_cnt = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize) as usize;
            let cur_base = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c10) as usize);
            if rbx2 == cur_base.wrapping_add(cur_cnt * 8) {
                break;
            }
        }
    }

    let mut idx_back =
        (rd::<i32>(exe::ptr::<i32>(0x1_4393_1c08) as usize).wrapping_sub(1)) as isize;
    while idx_back >= 0 {
        let arr = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c00) as usize);
        let rcx = rd::<usize>(arr.wrapping_add((idx_back as usize) * 8));
        if (rd::<u8>(rcx + 0x1c4) & 4) != 0 {
            exe::call_va!(0x1_415e_e820, (rcx));
        }
        idx_back -= 1;
    }

    let count_c60 = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c60) as usize) as usize;
    if count_c60 != 0 {
        let mut cur = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c58) as usize);
        let neg_inf = rd::<f32>(exe::ptr::<f32>(0x1_4342_7174) as usize);
        let mut xmm7 = neg_inf;
        let mut edi = rd::<u32>(cur);
        let init_end = cur.wrapping_add(count_c60 * 24);
        if cur != init_end {
            loop {
                if edi != rd::<u32>(cur) {
                    let mut param_buf = [0u64; 6];
                    let pbp = param_buf.as_mut_ptr() as usize;
                    wr::<u64>(pbp, 0);
                    wr::<u32>(pbp + 0x08, 0);
                    wr::<u32>(pbp + 0x10, 0);
                    wr::<u8>(pbp + 0x18, 0xff);
                    wr::<u8>(pbp + 0x20, 0xff);
                    wr::<u64>(pbp + 0x28, 0);
                    let rcx_mgr = rd::<u64>(exe::ptr::<u64>(0x1_4393_0fa0) as usize);
                    let _ = crate::simd_ops::call_va8_xmm(
                        0x1_415c_4f90,
                        rcx_mgr,
                        edi as u64,
                        pbp as u64,
                        0,
                        0,
                        1,
                        0,
                        0,
                        [0, 0, 0, xmm7.to_bits() as u128],
                    );
                    edi = rd::<u32>(cur);
                    xmm7 = neg_inf;
                }
                let xmm6 = rd::<f32>(cur + 0x10);
                let _ = crate::simd_ops::call_va8_xmm(
                    0x1_415f_1190,
                    0,
                    edi as u64,
                    rd::<u64>(cur + 0x08),
                    0,
                    0,
                    0,
                    0,
                    0,
                    [xmm6.to_bits() as u128, 0, 0, 0],
                );
                xmm7 = if xmm6 > xmm7 { xmm6 } else { xmm7 };
                cur = cur.wrapping_add(24);
                let cur_c60 = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c60) as usize) as usize;
                let cur_c58 = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c58) as usize);
                if cur == cur_c58.wrapping_add(cur_c60 * 24) {
                    break;
                }
            }
        }
        let mut param_buf = [0u64; 6];
        let pbp = param_buf.as_mut_ptr() as usize;
        wr::<u64>(pbp, 0);
        wr::<u32>(pbp + 0x08, 0);
        wr::<u32>(pbp + 0x10, 0);
        wr::<u8>(pbp + 0x18, 0xff);
        wr::<u8>(pbp + 0x20, 0xff);
        wr::<u64>(pbp + 0x28, 0);
        let rcx_mgr = rd::<u64>(exe::ptr::<u64>(0x1_4393_0fa0) as usize);
        let _ = crate::simd_ops::call_va8_xmm(
            0x1_415c_4f90,
            rcx_mgr,
            edi as u64,
            pbp as u64,
            0,
            0,
            1,
            0,
            0,
            [0, 0, 0, xmm7.to_bits() as u128],
        );
        wr::<u32>(exe::ptr::<u32>(0x1_4393_1c60) as usize, 0);
    }

    let mut rbx3 = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c10) as usize);
    let cnt3 = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize) as usize;
    if rbx3 != rbx3.wrapping_add(cnt3 * 8) {
        loop {
            let rcx = rd::<usize>(rbx3);
            if rd::<u32>(rcx + 0x100) == 1 {
                exe::call_va!(0x1_4164_6770, (rcx));
            }
            rbx3 = rbx3.wrapping_add(8);
            let cur_cnt = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize) as usize;
            let cur_base = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c10) as usize);
            if rbx3 == cur_base.wrapping_add(cur_cnt * 8) {
                break;
            }
        }
    }
}

/// `unknown_1415f1590`
/// Original: 0x1415f1590 (610 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_1415f1590(param_1: usize, param_2: u8) -> usize {
    let mut rbx = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c10) as usize);
    let cnt = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize) as usize;
    if rbx != rbx.wrapping_add(cnt * 8) {
        loop {
            let rsi = rd::<usize>(rbx);
            let rdi = rsi + 0x128;
            wr::<u32>(rdi + 0x30, 0x2c);
            if rd::<u32>(rsi + 0x100) == 1 {
                let ok = exe::call_va!(0x1_4160_1880, (rsi, rdi) -> u8);
                if ok != 0 && param_2 != 0 {
                    wr::<u16>(rdi + 0x12, 0);
                    wr::<u64>(rdi + 0x18, 0);
                    wr::<u32>(rdi + 0x20, 0xffff_ffff);
                    wr::<u32>(rdi + 0x30, 0x2b);
                }
            }
            let rcx_e8 = rd::<usize>(rsi + 0xe8);
            let vtbl = rd::<usize>(rcx_e8);
            exe::call_va!(rd::<usize>(vtbl + 0xe8), (rcx_e8));
            rbx = rbx.wrapping_add(8);
            let cur_cnt = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize) as usize;
            let cur_base = rd::<usize>(exe::ptr::<usize>(0x1_4393_1c10) as usize);
            if rbx == cur_base.wrapping_add(cur_cnt * 8) {
                break;
            }
        }
    }

    let mut buf_90 = [0u64; 4];
    let b90_ptr = buf_90.as_mut_ptr() as usize;
    let _ = exe::call_va!(0x1_415b_17b0, (b90_ptr, 1u32) -> usize);
    let _ = exe::call_va!(0x1_415b_17b0, (b90_ptr + 0x10, 1u32) -> usize);
    wr::<u8>(exe::ptr::<u8>(0x1_4393_1bea) as usize, 1);

    if param_2 != 0 {
        let ebx = rd::<u32>(exe::ptr::<u32>(0x1_4393_1c18) as usize);
        if ebx != 0 {
            let g_1bf0 = exe::ptr::<u32>(0x1_4393_1bf0) as usize;
            wr::<u32>(g_1bf0, 0);
            let mut buf_50 = [0u64; 2];
            let rdx_50 = exe::call_va!(
                0x1_415b_17b0,
                (buf_50.as_mut_ptr() as usize, 1u32) -> usize
            );
            let r8_1c10 = rd::<u64>(exe::ptr::<u64>(0x1_4393_1c10) as usize);
            let mut buf_60 = [0u64; 2];
            let rdx_60 = exe::call_va!(
                0x1_415f_2bb0,
                (
                    buf_60.as_mut_ptr() as usize,
                    rdx_50,
                    r8_1c10,
                    g_1bf0,
                    ebx,
                    10u32
                ) -> usize
            );
            let _ = exe::call_va!(0x1_415b_18a0, (b90_ptr, rdx_60) -> usize);
        }
    }

    let mut buf_70 = [0u64; 2];
    let str_when_all = exe::ptr::<u8>(0x1_4268_de30) as usize;
    let rbx_70 = exe::call_va!(
        0x1_415b_28c0,
        (
            buf_70.as_mut_ptr() as usize,
            b90_ptr,
            2u32,
            str_when_all,
            1u32
        ) -> usize
    );

    let mut slot_f0: u64 = u64::MAX;
    let mut slot_f8: u64 = 0;
    let str_post = exe::ptr::<u8>(0x1_4268_de48) as usize;
    exe::call_va!(
        0x1_415b_1e50,
        (
            (&mut slot_f8 as *mut u64) as usize,
            (&mut slot_f0 as *mut u64) as usize,
            0x10u32,
            0x08u32,
            str_post,
            0u32,
            1u32
        )
    );
    if slot_f8 != 0 {
        wr::<usize>(slot_f8 as usize, exe::ptr::<u8>(0x1_4268_dec8) as usize);
    }
    slot_f8 = slot_f0;
    slot_f0 = u64::MAX;
    let _ = slot_f0;
    let mut buf_40 = [0u64; 2];
    let _ = exe::call_va!(
        0x1_415b_1720,
        (
            buf_40.as_mut_ptr() as usize,
            (&mut slot_f8 as *mut u64) as usize,
            1u32,
            rd::<u64>(rbx_70 + 0x08)
        ) -> usize
    );

    if param_2 != 0 {
        let mut buf_80 = [0u64; 2];
        let rdx_80 = exe::call_va!(
            0x1_415b_1780,
            (buf_80.as_mut_ptr() as usize, buf_40.as_mut_ptr() as usize) -> usize
        );
        let _ = exe::call_va!(0x1_415e_f8b0, (param_1, rdx_80) -> usize);
    } else {
        let _ = exe::call_va!(
            0x1_415b_1780,
            (param_1, buf_40.as_mut_ptr() as usize) -> usize
        );
    }
    param_1
}

/// `std_14183df80` (`std::_Func_impl_no_alloc<anonymous_namespace::MetadataProvider::buildMap::<lambda_1>,void,?bref6::ResourceID,?bref6::PathView>::vfunc_2`)
/// Original: 0x14183df80 (138 bytes, Throws::Discard)
pub unsafe extern "win64" fn std_14183df80(param_1: usize, param_2: usize, param_3: usize) {
    let mut key_slot = rd::<u64>(param_2);
    let rsi = rd::<usize>(param_1 + 0x08);
    let src_ptr = rd::<usize>(param_3);
    let rdi_len = rd::<u64>(param_3 + 0x08);
    let rbx_dst = exe::call_va!(0x1_4242_dcc0, (rsi + 0x68, rdi_len) -> usize);
    let _ = exe::call_va!(0x1_4183_acc8, (rbx_dst, src_ptr, rdi_len) -> usize);
    let mut view: [u64; 2] = [rbx_dst as u64, rdi_len];
    let mut out_pv: [u64; 2] = [0; 2];
    let _ = exe::call_va!(
        0x1_401d_f850,
        (out_pv.as_mut_ptr() as usize, view.as_mut_ptr() as usize) -> usize
    );
    let slot = exe::call_va!(
        0x1_413a_1c10,
        (rsi, (&mut key_slot as *mut u64) as usize) -> usize
    );
    wr::<[u64; 2]>(slot + 0x08, out_pv);
}

/// `unknown_1418584f0`
/// Original: 0x1418584f0 (297 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1418584f0(param_1: usize) -> u8 {
    let len = rd::<u64>(param_1 + 0x08);
    if len < 4 {
        return 1;
    }
    let str_ptr = rd::<usize>(param_1);
    let pat_data = exe::ptr::<u8>(0x1_4307_dd78) as usize;
    let lut = exe::ptr::<u8>(0x1_426d_01e0) as usize;
    for i in 0..4usize {
        let b_in = rd::<u8>(str_ptr + i) as usize;
        let b_pat = rd::<u8>(pat_data + i) as usize;
        if (rd::<u8>(lut + b_in) as i8) != (rd::<u8>(lut + b_pat) as i8) {
            return 1;
        }
    }

    let mut src_view: [u64; 2] = [exe::ptr::<u8>(0x1_4307_ddc8) as usize as u64, 0x0c];
    let mut pv_30: [u64; 2] = [0; 2];
    let _ = exe::call_va!(
        0x1_401d_f850,
        (pv_30.as_mut_ptr() as usize, src_view.as_mut_ptr() as usize) -> usize
    );
    let mut in_20 = rd::<[u64; 2]>(param_1);
    let mut out_40 = [0u64; 8];
    let _ = exe::call_va!(
        0x1_401f_5ea0,
        (
            out_40.as_mut_ptr() as usize,
            in_20.as_mut_ptr() as usize,
            pv_30.as_mut_ptr() as usize
        ) -> usize
    );
    if out_40[6] == u64::MAX {
        return 1;
    }

    src_view = [exe::ptr::<u8>(0x1_4307_dd88) as usize as u64, 0x0c];
    let _ = exe::call_va!(
        0x1_401d_f850,
        (pv_30.as_mut_ptr() as usize, src_view.as_mut_ptr() as usize) -> usize
    );
    in_20 = rd::<[u64; 2]>(param_1);
    let _ = exe::call_va!(
        0x1_401f_5ea0,
        (
            out_40.as_mut_ptr() as usize,
            in_20.as_mut_ptr() as usize,
            pv_30.as_mut_ptr() as usize
        ) -> usize
    );
    u8::from(out_40[6] == u64::MAX)
}

/// `content_141c6a020`
/// Original: 0x141c6a020 (108 bytes, Throws::No, FLOAT_RETURN)
#[cfg(not(target_arch = "aarch64"))]
pub unsafe extern "win64" fn content_141c6a020(param_1: usize, val: f32) -> f32 {
    content_141c6a020_impl(param_1, val)
}

#[cfg(target_arch = "aarch64")]
pub unsafe extern "win64" fn content_141c6a020(param_1: usize, _xmm0: f32, val: f32) -> f32 {
    content_141c6a020_impl(param_1, val)
}

#[inline(always)]
unsafe fn content_141c6a020_impl(param_1: usize, val: f32) -> f32 {
    let _ = crate::simd_ops::take_tls_xmm_in_if_set();
    let eax = rd::<u32>(param_1 + 0x28) as u64;
    let rdx = if eax == 0 { 0 } else { eax - 1 };
    let mut res = 0.0f32;
    if rdx != 0 {
        let mut rcx = rd::<usize>(param_1);
        let count = rd::<u32>(param_1 + 0x08) as usize;
        let end = rcx.wrapping_add(count.wrapping_mul(0x54));
        let min_b = rd::<f32>(param_1 + 0x10);
        let max_b = rd::<f32>(param_1 + 0x18);
        let mut xmm5 = if val > min_b { val } else { min_b };
        xmm5 = if xmm5 < max_b { xmm5 } else { max_b };
        while rcx != end {
            if xmm5 >= rd::<f32>(rcx + 0x04) && rd::<f32>(rcx + 0x0c) >= xmm5 {
                let (_, out_xmm0) = crate::simd_ops::call_va8_xmm(
                    0x1_41c6_96e0,
                    rcx as u64,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, xmm5.to_bits() as u128, 0, 0],
                );
                res = f32::from_bits(out_xmm0 as u32);
                break;
            }
            rcx = rcx.wrapping_add(0x54);
        }
    }
    crate::simd_ops::set_xmm_ret(res.to_bits() as u128);
    res
}

// =============================================================================
// Differential tests
// =============================================================================

pub unsafe fn diff_rend_141353470(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut MAPPED_BUF: [u32; 15] = [0; 15];

    unsafe extern "win64" fn mock_dfb0(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        LOG_SUM = LOG_SUM.wrapping_add(new_cap);
    }
    unsafe extern "win64" fn mock_c110(_p08: usize, a0: u64, _p58: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(a0 * 0x100);
    }
    unsafe extern "win64" fn mock_9440(_p10: usize, count: u64, _p70: usize, _p88: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(count * 0x1000);
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _rdx: usize, byte_len: u64, flags: u32) -> u64 {
        LOG_SUM = LOG_SUM.wrapping_add(byte_len).wrapping_add(flags as u64);
        core::ptr::addr_of_mut!(MAPPED_BUF) as usize as u64
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, rdx: u32) {
        LOG_SUM = LOG_SUM.wrapping_add(0x5000 + rdx as u64);
    }
    unsafe extern "win64" fn mock_d500(
        p08_val: usize,
        p10_val: usize,
        pending: u32,
        stride: u32,
        flag: u8,
    ) {
        LOG_SUM = LOG_SUM
            .wrapping_add(p08_val as u64)
            .wrapping_add(p10_val as u64)
            .wrapping_add(pending as u64)
            .wrapping_add(stride as u64)
            .wrapping_add(flag as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4134_dfb0, mock_dfb0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_c110, mock_c110 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_9440, mock_9440 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4237_d500, mock_d500 as *const ())?;

    let run = |f: F| -> (u64, [u32; 15], [u32; 15], u64, u32, u32) {
        LOG_SUM = 0;
        MAPPED_BUF = [0; 15];
        let mut word = 0b1011u64;
        let src_vals: [u32; 12] = [
            0x1111_0001,
            0x1111_0002,
            0x1111_0003,
            0x2222_0001,
            0x2222_0002,
            0x2222_0003,
            0x3333_0001,
            0x3333_0002,
            0x3333_0003,
            0x4444_0001,
            0x4444_0002,
            0x4444_0003,
        ];
        let mut vec_data = [0u32; 15];
        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x08, 0xaaaa);
        wr::<usize>(tp + 0x10, 0xbbbb);
        wr::<u8>(tp + 0x29, 0);
        wr::<u64>(tp + 0x90, 3);
        wr::<usize>(tp + 0x98, src_vals.as_ptr() as usize);
        wr::<u32>(tp + 0xa0, 7);
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_data.as_mut_ptr() as usize);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 1);

        f(tp);
        (
            word,
            vec_data,
            MAPPED_BUF,
            LOG_SUM,
            rd::<u32>(tp + 0xc8),
            rd::<u32>(tp + 0xcc),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141353470 out");
    Ok(1)
}

pub unsafe fn diff_rend_141353b70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut MAPPED_BUF: [u64; 8] = [0; 8];

    unsafe extern "win64" fn mock_b830(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        LOG_SUM = LOG_SUM.wrapping_add(new_cap);
    }
    unsafe extern "win64" fn mock_2520(_p08: usize, new_sz: u64) {
        LOG_SUM = LOG_SUM.wrapping_add(new_sz);
    }
    unsafe extern "win64" fn mock_9440(_p10: usize, count: u64, _p70: usize, _p88: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(count * 0x100);
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _rdx: usize, byte_len: u64, flags: u32) -> u64 {
        LOG_SUM = LOG_SUM.wrapping_add(byte_len).wrapping_add(flags as u64);
        core::ptr::addr_of_mut!(MAPPED_BUF) as usize as u64
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, rdx: u32) {
        LOG_SUM = LOG_SUM.wrapping_add(0x4000 + rdx as u64);
    }
    unsafe extern "win64" fn mock_d500(
        _p08_val: usize,
        p10_val: usize,
        pending: u32,
        stride: u32,
        flag: u8,
    ) {
        LOG_SUM = LOG_SUM
            .wrapping_add(p10_val as u64)
            .wrapping_add(pending as u64)
            .wrapping_add(stride as u64)
            .wrapping_add(flag as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_40a5_b830, mock_b830 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422e_2520, mock_2520 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_9440, mock_9440 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4237_d500, mock_d500 as *const ())?;

    let run = |f: F| -> (u64, [u64; 8], [u64; 8], u64, u32) {
        LOG_SUM = 0;
        MAPPED_BUF = [0; 8];
        let mut word = 0b0101u64;
        let src_vals: [u64; 4] = [0xaaaa_1111_2222_3333, 0, 0xbbbb_4444_5555_6666, 0];
        let mut vec_data = [0u64; 8];
        let mut obj_08 = [0u64; 4];
        obj_08[2] = 0x100; // +0x10 = current buffer size

        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x08, obj_08.as_mut_ptr() as usize);
        wr::<usize>(tp + 0x10, 0xcccc);
        wr::<u8>(tp + 0x29, 0);
        wr::<u64>(tp + 0x60, 16);
        wr::<u64>(tp + 0x68, 256);
        wr::<u64>(tp + 0x90, 4);
        wr::<usize>(tp + 0x98, src_vals.as_ptr() as usize);
        wr::<u32>(tp + 0xa0, 20);
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_data.as_mut_ptr() as usize);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 1);

        f(tp);
        (word, vec_data, MAPPED_BUF, LOG_SUM, rd::<u32>(tp + 0xcc))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141353b70 out");
    Ok(1)
}

pub unsafe fn diff_rend_141354150(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut MAPPED_BUF: [u64; 12] = [0; 12];

    unsafe extern "win64" fn mock_e8a0(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        LOG_SUM = LOG_SUM.wrapping_add(new_cap);
    }
    unsafe extern "win64" fn mock_1020(_p08: usize, new_sz: u64) {
        LOG_SUM = LOG_SUM.wrapping_add(new_sz);
    }
    unsafe extern "win64" fn mock_9440(_p10: usize, count: u64, _p70: usize, _p88: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(count * 0x100);
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _rdx: usize, byte_len: u64, flags: u32) -> u64 {
        LOG_SUM = LOG_SUM.wrapping_add(byte_len).wrapping_add(flags as u64);
        core::ptr::addr_of_mut!(MAPPED_BUF) as usize as u64
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, rdx: u32) {
        LOG_SUM = LOG_SUM.wrapping_add(0x4000 + rdx as u64);
    }
    unsafe extern "win64" fn mock_d500(
        _p08_val: usize,
        p10_val: usize,
        pending: u32,
        stride: u32,
        flag: u8,
    ) {
        LOG_SUM = LOG_SUM
            .wrapping_add(p10_val as u64)
            .wrapping_add(pending as u64)
            .wrapping_add(stride as u64)
            .wrapping_add(flag as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_408c_e8a0, mock_e8a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4226_1020, mock_1020 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_9440, mock_9440 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4237_d500, mock_d500 as *const ())?;

    let run = |f: F| -> (u64, [u64; 12], [u64; 12], u64, u32) {
        LOG_SUM = 0;
        MAPPED_BUF = [0; 12];
        let mut word = 0b0011u64;
        let src_vals: [u64; 4] = [0x1111_2222, 0x3333_4444, 0x5555_6666, 0x7777_8888];
        let mut vec_data = [0u64; 12];
        let mut obj_08 = [0u64; 4];
        obj_08[2] = 0x80;

        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x08, obj_08.as_mut_ptr() as usize);
        wr::<usize>(tp + 0x10, 0xdddd);
        wr::<u8>(tp + 0x29, 0);
        wr::<u64>(tp + 0x60, 24);
        wr::<u64>(tp + 0x68, 256);
        wr::<u64>(tp + 0x90, 4);
        wr::<usize>(tp + 0x98, src_vals.as_ptr() as usize);
        wr::<u32>(tp + 0xa0, 15);
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_data.as_mut_ptr() as usize);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 1);

        f(tp);
        (word, vec_data, MAPPED_BUF, LOG_SUM, rd::<u32>(tp + 0xcc))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141354150 out");
    Ok(1)
}

pub unsafe fn diff_rend_1413558a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut MAPPED_BUF: [u64; 12] = [0; 12];

    unsafe extern "win64" fn mock_1e10(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        LOG_SUM = LOG_SUM.wrapping_add(new_cap);
    }
    unsafe extern "win64" fn mock_c110(_p08: usize, a0: u64, _p58: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(a0 * 0x100);
    }
    unsafe extern "win64" fn mock_69a0() {
        crate::simd_ops::set_xmm_ret(3.5f64.to_bits() as u128);
    }
    unsafe extern "win64" fn mock_d670(_r14: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0x700);
    }
    unsafe extern "win64" fn mock_5af0(
        alloc_sz: u64,
        rdx: u64,
        r8: u64,
        flags: u32,
        a5: u32,
        _a6: u64,
        _r14: usize,
    ) {
        LOG_SUM = LOG_SUM
            .wrapping_add(alloc_sz)
            .wrapping_add(rdx)
            .wrapping_add(r8)
            .wrapping_add(flags as u64)
            .wrapping_add(a5 as u64);
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _rdx: usize, byte_len: u64, flags: u32) -> u64 {
        LOG_SUM = LOG_SUM.wrapping_add(byte_len).wrapping_add(flags as u64);
        core::ptr::addr_of_mut!(MAPPED_BUF) as usize as u64
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, rdx: u32) {
        LOG_SUM = LOG_SUM.wrapping_add(0x4000 + rdx as u64);
    }
    unsafe extern "win64" fn mock_9840(
        p08_val: usize,
        _p10_val: usize,
        pending: u32,
        stride: u32,
        flag: u8,
    ) {
        LOG_SUM = LOG_SUM
            .wrapping_add(p08_val as u64)
            .wrapping_add(pending as u64)
            .wrapping_add(stride as u64)
            .wrapping_add(flag as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4070_1e10, mock_1e10 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_412e_c110, mock_c110 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_69a0, mock_69a0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_d670, mock_d670 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4207_5af0, mock_5af0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_4238_9840, mock_9840 as *const ())?;

    let run = |f: F| -> (u64, [u64; 12], [u64; 12], u64, u32, u64) {
        LOG_SUM = 0;
        MAPPED_BUF = [0; 12];
        let mut word = 0b0011u64;
        let src_vals: [u64; 10] = [
            10, 20, 30, 40, 50,
            60, 70, 80, 90, 100,
        ];
        let mut vec_data = [0u64; 12];
        let mut obj_10 = [0u8; 0x60];
        let o10p = obj_10.as_mut_ptr() as usize;
        wr::<u64>(o10p + 0x10, 0x100); // cur_sz < alloc_sz

        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x08, 0x5555);
        wr::<usize>(tp + 0x10, o10p);
        wr::<u8>(tp + 0x29, 0);
        wr::<u64>(tp + 0x78, 48);
        wr::<u64>(tp + 0x80, 512);
        wr::<f64>(tp + 0x88, 1.0);
        wr::<u64>(tp + 0x90, 4);
        wr::<usize>(tp + 0x98, src_vals.as_ptr() as usize);
        wr::<u32>(tp + 0xa0, 9);
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_data.as_mut_ptr() as usize);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 1);

        f(tp);
        (
            word,
            vec_data,
            MAPPED_BUF,
            LOG_SUM,
            rd::<u32>(o10p + 0x50),
            rd::<u64>(tp + 0x88),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1413558a0 out");
    Ok(1)
}

pub unsafe fn diff_rend_141356660(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut MAPPED_BUF: [u64; 10] = [0; 10];

    unsafe extern "win64" fn mock_8620(vec_ptr: usize, new_cap: u64) {
        wr::<u32>(vec_ptr + 0x0c, new_cap as u32);
        LOG_SUM = LOG_SUM.wrapping_add(new_cap);
    }
    unsafe extern "win64" fn mock_df60(_p08: usize, new_sz: u64) {
        LOG_SUM = LOG_SUM.wrapping_add(new_sz);
    }
    unsafe extern "win64" fn mock_9440(_p10: usize, count: u64, _p70: usize, _p88: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(count * 0x100);
    }
    unsafe extern "win64" fn mock_c560(_buf: usize, _rdx: usize, byte_len: u64, flags: u32) -> u64 {
        LOG_SUM = LOG_SUM.wrapping_add(byte_len).wrapping_add(flags as u64);
        core::ptr::addr_of_mut!(MAPPED_BUF) as usize as u64
    }
    unsafe extern "win64" fn mock_d6f0(_buf: usize, rdx: u32) {
        LOG_SUM = LOG_SUM.wrapping_add(0x4000 + rdx as u64);
    }
    unsafe extern "win64" fn mock_9840(
        _p08_val: usize,
        p10_val: usize,
        pending: u32,
        stride: u32,
        flag: u8,
    ) {
        LOG_SUM = LOG_SUM
            .wrapping_add(p10_val as u64)
            .wrapping_add(pending as u64)
            .wrapping_add(stride as u64)
            .wrapping_add(flag as u64);
    }

    let _m1 = crate::difftest::Mock::new(0x1_411e_8620, mock_8620 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4237_df60, mock_df60 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_412f_9440, mock_9440 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4209_c560, mock_c560 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4209_d6f0, mock_d6f0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4238_9840, mock_9840 as *const ())?;

    let run = |f: F| -> (u64, [u64; 10], [u64; 10], u64, u32) {
        LOG_SUM = 0;
        MAPPED_BUF = [0; 10];
        let mut word = 0b0011u64;
        let src_vals: [u64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
        let mut vec_data = [0u64; 10];
        let mut obj_08 = [0u64; 4];
        obj_08[2] = 0x80;

        let mut this_buf = [0u8; 0x100];
        let tp = this_buf.as_mut_ptr() as usize;
        wr::<usize>(tp + 0x08, obj_08.as_mut_ptr() as usize);
        wr::<usize>(tp + 0x10, 0xeeee);
        wr::<u8>(tp + 0x29, 0);
        wr::<u64>(tp + 0x60, 40);
        wr::<u64>(tp + 0x68, 256);
        wr::<u64>(tp + 0x90, 4);
        wr::<usize>(tp + 0x98, src_vals.as_ptr() as usize);
        wr::<u32>(tp + 0xa0, 12);
        wr::<usize>(tp + 0xa8, (&mut word as *mut u64) as usize);
        wr::<u64>(tp + 0xb8, 64);
        wr::<usize>(tp + 0xc0, vec_data.as_mut_ptr() as usize);
        wr::<u32>(tp + 0xc8, 0);
        wr::<u32>(tp + 0xcc, 1);

        f(tp);
        (word, vec_data, MAPPED_BUF, LOG_SUM, rd::<u32>(tp + 0xcc))
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141356660 out");
    Ok(1)
}

pub unsafe fn diff_r_1413ab4d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ALLOC_IDX: usize = 0;
    static mut NODES: [[u64; 10]; 2] = [[0; 10]; 2];
    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_a730(dst: usize, src: usize) -> usize {
        wr::<u64>(dst, rd::<u64>(src) ^ 0x1111);
        dst
    }
    unsafe extern "win64" fn mock_68a0(dst: usize, src: usize) -> usize {
        wr::<u64>(dst, rd::<u64>(src) ^ 0x2222);
        dst
    }
    unsafe extern "win64" fn mock_7c80() -> u32 {
        0
    }
    unsafe extern "win64" fn mock_5aa0(_pool: usize, out_handle: usize) -> usize {
        let idx = ALLOC_IDX;
        ALLOC_IDX += 1;
        let ptr = core::ptr::addr_of_mut!(NODES[idx]) as usize;
        wr::<u64>(out_handle, ptr as u64);
        wr::<u64>(out_handle + 0x08, 0x0007 + (idx as u64) * 0x10);
        out_handle
    }
    unsafe extern "win64" fn mock_6460(_sched: usize, job_ptr: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(job_ptr + 0x30));
    }
    unsafe extern "win64" fn mock_5930(_sched_mgr: usize, handle_ptr: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(handle_ptr + 0x08));
    }
    unsafe extern "win64" fn mock_vtbl_10(_this: usize, rdx: u64, r8: u64, r9: u32) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rdx)
            .wrapping_add(r8)
            .wrapping_add(r9 as u64);
    }
    unsafe extern "win64" fn mock_ab10(p2: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u64>(p2 + 0x58) + 0x900);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_a730, mock_a730 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4249_68a0, mock_68a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_7c80, mock_7c80 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4248_5aa0, mock_5aa0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4248_6460, mock_6460 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4248_5930, mock_5930 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4240_ab10, mock_ab10 as *const ())?;

    let g_2d0 = exe::ptr::<u64>(0x1_43a0_82d0) as usize;
    let g_2f8 = exe::ptr::<usize>(0x1_43a0_82f8) as usize;
    let s_2d0 = rd::<u64>(g_2d0);
    let s_2f8 = rd::<usize>(g_2f8);

    let run = |f: F, init_cnt: u32, mode_2d0: u64| -> ([u8; 0x90], u32, u32, u64, [[u64; 10]; 2], u64) {
        ALLOC_IDX = 0;
        NODES = [[0; 10]; 2];
        wr::<u8>((core::ptr::addr_of_mut!(NODES[0]) as usize) + 0x04, 1);
        LOG_SUM = 0;

        let sem_owner = vec![0u8; 0x1020];
        let mut sched = [0u8; 0x80];
        let sp = sched.as_mut_ptr() as usize;
        wr::<usize>(sp, sem_owner.as_ptr() as usize);
        wr::<usize>(sp + 0x48, 0x20_0000);

        let mut mgr_2f8 = [0u64; 4];
        mgr_2f8[0] = 2; // denom
        mgr_2f8[2] = sp as u64; // +0x10 = sched

        wr::<u64>(g_2d0, mode_2d0);
        wr::<usize>(g_2f8, mgr_2f8.as_ptr() as usize);

        let src_items: [u64; 6] = [0xaaaa, 0, 0, 0xbbbb_cccc, 0, 0];
        let mut dst_items = [0u8; 0x88 * 2];
        let dp = dst_items.as_mut_ptr() as usize;

        let mut p1 = [0u8; 0x70];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x18, src_items.as_ptr() as usize);
        wr::<usize>(p1p + 0x28, dp);
        wr::<u32>(p1p + 0x30, 6);
        wr::<u32>(p1p + 0x38, init_cnt);
        wr::<u32>(p1p + 0x50, 10);

        let mut vtbl = [0usize; 4];
        vtbl[2] = mock_vtbl_10 as *const () as usize;
        let obj_48 = [vtbl.as_ptr() as usize];

        let mut p2 = [0u8; 0x60];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<u64>(p2p, 0x1234);
        wr::<u64>(p2p + 0x18, 0x5678);
        wr::<u64>(p2p + 0x20, 0x9abc);
        wr::<u64>(p2p + 0x30, 0x100);
        wr::<u64>(p2p + 0x38, 0x200);
        wr::<u16>(p2p + 0x40, 7);
        wr::<usize>(p2p + 0x48, obj_48.as_ptr() as usize);
        wr::<u32>(p2p + 0x50, 42);
        wr::<u64>(p2p + 0x58, 1);

        f(p1p, p2p);

        let mut out_item = [0u8; 0x90];
        out_item[..0x88].copy_from_slice(&dst_items[0x88..0x110]);
        // Normalize pointer fields and uninitialized stack upper bytes in NODES
        let mut nodes_snap = NODES;
        nodes_snap[0][1] = u64::from(nodes_snap[0][1] != 0);
        nodes_snap[1][1] = u64::from(nodes_snap[1][1] != 0);
        nodes_snap[1][3] = u64::from(nodes_snap[1][3] != 0);
        nodes_snap[1][5] &= 0xffff; // low u16 at +0x28 is 3; upper 6 bytes are stack padding in orig
        nodes_snap[1][7] = u64::from(nodes_snap[1][7] != 0);

        (
            out_item,
            rd::<u32>(p1p + 0x38),
            rd::<u32>(p1p + 0x50),
            rd::<u64>(p1p + 0x60),
            nodes_snap,
            LOG_SUM,
        )
    };

    for (cnt, m2d0) in [(2u32, 0u64), (1, 0), (1, 1)] {
        let o = run(f_o, cnt, m2d0);
        let n = run(f_n, cnt, m2d0);
        wr::<u64>(g_2d0, s_2d0);
        wr::<usize>(g_2f8, s_2f8);
        crate::diff_eq!(o, n, "1413ab4d0 cnt={} m2d0={}", cnt, m2d0);
    }
    Ok(3)
}

pub unsafe fn diff_ui_141410620(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLED_ARG: u64 = 0;
    unsafe extern "win64" fn mock_2f50(rcx: u64) {
        CALLED_ARG = rcx;
    }

    let _m1 = crate::difftest::Mock::new(0x1_4250_2f50, mock_2f50 as *const ())?;

    for state in [u32::MAX, 0, 5] {
        let run = |f: F| -> u64 {
            CALLED_ARG = 0;
            let mut obj = [0u8; 0x20];
            let op = obj.as_mut_ptr() as usize;
            wr::<u32>(op + 0x08, state);
            wr::<u64>(op + 0x10, 0x1234_5678_9abc_def0);
            f(op);
            CALLED_ARG
        };
        let o = run(f_o);
        let n = run(f_n);
        crate::diff_eq!(o, n, "141410620 state={:#x}", state);
    }
    Ok(3)
}

pub unsafe fn diff_unknown_1415ee330(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn();
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;

    unsafe extern "win64" fn mock_0880(rcx: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u32>(rcx) as u64 + 0x10);
    }
    unsafe extern "win64" fn mock_7860() {
        LOG_SUM = LOG_SUM.wrapping_add(0x20);
    }
    unsafe extern "win64" fn mock_2390() -> u32 {
        LOG_SUM = LOG_SUM.wrapping_add(0x30);
        1
    }
    unsafe extern "win64" fn mock_bee0(rcx: usize, edx: u32) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rd::<u32>(rcx) as u64)
            .wrapping_add(edx as u64);
    }
    unsafe extern "win64" fn mock_vtbl_68(_this: usize, edx: u32) {
        LOG_SUM = LOG_SUM.wrapping_add(0x68 + edx as u64);
    }
    unsafe extern "win64" fn mock_vtbl_70(_this: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0x70);
    }
    unsafe extern "win64" fn mock_vtbl_78(_this: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0x78);
    }
    unsafe extern "win64" fn mock_2db0(node: usize) {
        wr::<f32>(node + 0x10, 2.5);
        LOG_SUM = LOG_SUM.wrapping_add(0x2db);
    }
    unsafe extern "win64" fn mock_5fc0(rcx: usize, dl: u8) {
        LOG_SUM = LOG_SUM
            .wrapping_add(rd::<u32>(rcx) as u64)
            .wrapping_add((dl as u64) * 0x100);
    }
    unsafe extern "win64" fn mock_e820(rcx: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u32>(rcx) as u64 + 0x820);
    }
    unsafe extern "win64" fn mock_4f90(
        rcx: u64,
        rdx: u64,
        r8: usize,
        _r9: u64,
        arg5: u64,
        arg6: u32,
        arg7: u8,
    ) -> u32 {
        let xmm = crate::simd_ops::take_xmm_args();
        let f_bits = xmm[3] as u32 as u64;
        let id = rdx as u32 as u64;
        let b_type = ((rdx >> 32) & 0xff) as u8 as u64;
        let b18 = rd::<u8>(r8 + 0x18) as u64;
        let b20 = rd::<u8>(r8 + 0x20) as u64;
        LOG_SUM = LOG_SUM
            .wrapping_add(rcx)
            .wrapping_add(id)
            .wrapping_add(b_type)
            .wrapping_add(f_bits)
            .wrapping_add(b18)
            .wrapping_add(b20)
            .wrapping_add(arg5)
            .wrapping_add(arg6 as u64)
            .wrapping_add(arg7 as u64);
        1
    }
    unsafe extern "win64" fn mock_1190(_rcx: u64, rdx: u32, r8: u64) {
        let xmm = crate::simd_ops::take_xmm_args();
        let f_bits = xmm[0] as u32 as u64;
        LOG_SUM = LOG_SUM
            .wrapping_add(rdx as u64)
            .wrapping_add(r8)
            .wrapping_add(f_bits);
    }
    unsafe extern "win64" fn mock_6770(rcx: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u32>(rcx) as u64 + 0x770);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4160_0880, mock_0880 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4164_7860, mock_7860 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_415f_2390, mock_2390 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4163_bee0, mock_bee0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_415f_2db0, mock_2db0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4164_5fc0, mock_5fc0 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_415e_e820, mock_e820 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_415c_4f90, mock_4f90 as *const ())?;
    let _m9 = crate::difftest::Mock::new(0x1_415f_1190, mock_1190 as *const ())?;
    let _m10 = crate::difftest::Mock::new(0x1_4164_6770, mock_6770 as *const ())?;

    let g_c10 = exe::ptr::<usize>(0x1_4393_1c10) as usize;
    let g_c18 = exe::ptr::<u32>(0x1_4393_1c18) as usize;
    let g_c00 = exe::ptr::<usize>(0x1_4393_1c00) as usize;
    let g_c08 = exe::ptr::<u32>(0x1_4393_1c08) as usize;
    let g_c58 = exe::ptr::<usize>(0x1_4393_1c58) as usize;
    let g_c60 = exe::ptr::<u32>(0x1_4393_1c60) as usize;
    let g_fa0 = exe::ptr::<u64>(0x1_4393_0fa0) as usize;
    let saved = (
        rd::<usize>(g_c10),
        rd::<u32>(g_c18),
        rd::<usize>(g_c00),
        rd::<u32>(g_c08),
        rd::<usize>(g_c58),
        rd::<u32>(g_c60),
        rd::<u64>(g_fa0),
    );

    let run = |f: F| -> (u64, u32, u32, u32) {
        LOG_SUM = 0;

        let mut item_a = [0u8; 0x110];
        let mut item_b = [0u8; 0x110];
        wr::<u32>(item_a.as_mut_ptr() as usize, 101);
        wr::<u32>(item_a.as_mut_ptr() as usize + 0x100, 1);
        wr::<u32>(item_b.as_mut_ptr() as usize, 102);
        wr::<u32>(item_b.as_mut_ptr() as usize + 0x100, 0);
        let list_c10: [usize; 2] = [item_a.as_mut_ptr() as usize, item_b.as_mut_ptr() as usize];

        let mut vtbl = [0usize; 16];
        vtbl[0x68 / 8] = mock_vtbl_68 as *const () as usize;
        vtbl[0x70 / 8] = mock_vtbl_70 as *const () as usize;
        vtbl[0x78 / 8] = mock_vtbl_78 as *const () as usize;

        let mut sub_obj1 = [0u8; 0x200];
        wr::<usize>(sub_obj1.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<u8>(sub_obj1.as_mut_ptr() as usize + 0x1f8, 0x11);

        let mut node0 = [0u8; 0x1d0];
        let n0p = node0.as_mut_ptr() as usize;
        wr::<u32>(n0p, 201);
        wr::<usize>(n0p + 0x08, 0);
        wr::<u8>(n0p + 0x1b4, 0);
        wr::<u8>(n0p + 0x1c4, 4);

        let mut node1 = [0u8; 0x1d0];
        let n1p = node1.as_mut_ptr() as usize;
        wr::<u32>(n1p, 202);
        wr::<usize>(n1p + 0x08, sub_obj1.as_mut_ptr() as usize);
        wr::<usize>(n1p + 0x38, 0);
        wr::<u8>(n1p + 0x1b4, 3);
        wr::<u8>(n1p + 0x1c4, 0);

        let list_c00: [usize; 2] = [n0p, n1p];

        let mut c58_entries = [0u8; 48];
        let c58p = c58_entries.as_mut_ptr() as usize;
        wr::<u32>(c58p, 11);
        wr::<u64>(c58p + 0x08, 0x1111);
        wr::<f32>(c58p + 0x10, -3.5);
        wr::<u32>(c58p + 0x18, 22);
        wr::<u64>(c58p + 0x20, 0x2222);
        wr::<f32>(c58p + 0x28, 1.25);

        wr::<usize>(g_c10, list_c10.as_ptr() as usize);
        wr::<u32>(g_c18, 2);
        wr::<usize>(g_c00, list_c00.as_ptr() as usize);
        wr::<u32>(g_c08, 2);
        wr::<usize>(g_c58, c58p);
        wr::<u32>(g_c60, 2);
        wr::<u64>(g_fa0, 0x9999);

        f();
        (
            LOG_SUM,
            rd::<u32>(n0p + 0x14),
            rd::<u32>(n1p + 0x14),
            rd::<u32>(g_c60),
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    wr::<usize>(g_c10, saved.0);
    wr::<u32>(g_c18, saved.1);
    wr::<usize>(g_c00, saved.2);
    wr::<u32>(g_c08, saved.3);
    wr::<usize>(g_c58, saved.4);
    wr::<u32>(g_c60, saved.5);
    wr::<u64>(g_fa0, saved.6);

    crate::diff_eq!(o, n, "1415ee330 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1415f1590(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8) -> usize;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut LOG_SUM: u64 = 0;
    static mut TASK_OBJ: [u64; 2] = [0; 2];

    unsafe extern "win64" fn mock_1880(rsi: usize, _rdi: usize) -> u8 {
        LOG_SUM = LOG_SUM.wrapping_add(rd::<u32>(rsi) as u64);
        1
    }
    unsafe extern "win64" fn mock_vtbl_e8(_this: usize) {
        LOG_SUM = LOG_SUM.wrapping_add(0xe8);
    }
    unsafe extern "win64" fn mock_2bb0(
        out: usize,
        rdx_in: usize,
        _r8: u64,
        r9_1bf0: usize,
        cnt: u32,
        prio: u32,
    ) -> usize {
        wr::<u32>(r9_1bf0, cnt);
        wr::<u32>(out, rd::<u32>(rdx_in) + prio);
        wr::<u64>(out + 0x08, 0x3333);
        out
    }
    unsafe extern "win64" fn mock_28c0(
        out: usize,
        b90_ptr: usize,
        count: u32,
        _name: usize,
        flag: u32,
    ) -> usize {
        wr::<u32>(out, flag + count);
        wr::<u64>(out + 0x08, rd::<u64>(b90_ptr + 0x08) ^ 0x4444);
        out
    }
    unsafe extern "win64" fn mock_1e50(
        out_task: usize,
        out_handle: usize,
        sz: u32,
        align: u32,
        _name: usize,
        a6: u32,
        a7: u32,
    ) {
        LOG_SUM = LOG_SUM
            .wrapping_add(sz as u64)
            .wrapping_add(align as u64)
            .wrapping_add(a6 as u64)
            .wrapping_add(a7 as u64);
        wr::<usize>(out_task, core::ptr::addr_of_mut!(TASK_OBJ) as usize);
        wr::<u64>(out_handle, 0x5555);
    }
    unsafe extern "win64" fn mock_1720(
        out: usize,
        handle_ptr: usize,
        flag: u32,
        dep: u64,
    ) -> usize {
        wr::<u32>(out, flag);
        wr::<u64>(out + 0x08, rd::<u64>(handle_ptr).wrapping_add(dep));
        out
    }
    unsafe extern "win64" fn mock_f8b0(out: usize, src: usize) -> usize {
        wr::<u32>(out, rd::<u32>(src) + 0x10);
        wr::<u64>(out + 0x08, rd::<u64>(src + 0x08) + 0x100);
        out
    }

    let _m1 = crate::difftest::Mock::new(0x1_4160_1880, mock_1880 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_415f_2bb0, mock_2bb0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_415b_28c0, mock_28c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_415b_1e50, mock_1e50 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_415b_1720, mock_1720 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_415e_f8b0, mock_f8b0 as *const ())?;

    let g_c10 = exe::ptr::<usize>(0x1_4393_1c10) as usize;
    let g_c18 = exe::ptr::<u32>(0x1_4393_1c18) as usize;
    let g_bea = exe::ptr::<u8>(0x1_4393_1bea) as usize;
    let g_bf0 = exe::ptr::<u32>(0x1_4393_1bf0) as usize;
    let saved = (
        rd::<usize>(g_c10),
        rd::<u32>(g_c18),
        rd::<u8>(g_bea),
        rd::<u32>(g_bf0),
    );

    let run = |f: F, p2: u8| -> ([u64; 2], [u8; 0x40], u64, u64, u8, u32) {
        LOG_SUM = 0;
        TASK_OBJ = [0; 2];

        let mut vtbl = [0usize; 32];
        vtbl[0xe8 / 8] = mock_vtbl_e8 as *const () as usize;
        let sub_e8 = [vtbl.as_ptr() as usize];

        let mut item = [0u8; 0x170];
        let ip = item.as_mut_ptr() as usize;
        wr::<u32>(ip, 77);
        wr::<usize>(ip + 0xe8, sub_e8.as_ptr() as usize);
        wr::<u32>(ip + 0x100, 1);

        let list = [ip];
        wr::<usize>(g_c10, list.as_ptr() as usize);
        wr::<u32>(g_c18, 1);
        wr::<u8>(g_bea, 0);
        wr::<u32>(g_bf0, 99);

        let mut out = [0u64; 2];
        let ret = f(out.as_mut_ptr() as usize, p2);
        assert_eq!(ret, out.as_mut_ptr() as usize);

        let mut slice_128 = [0u8; 0x40];
        slice_128.copy_from_slice(&item[0x128..0x168]);
        (
            out,
            slice_128,
            TASK_OBJ[0],
            LOG_SUM,
            rd::<u8>(g_bea),
            rd::<u32>(g_bf0),
        )
    };

    for p2 in [0u8, 1] {
        let o = run(f_o, p2);
        let n = run(f_n, p2);
        wr::<usize>(g_c10, saved.0);
        wr::<u32>(g_c18, saved.1);
        wr::<u8>(g_bea, saved.2);
        wr::<u32>(g_bf0, saved.3);
        crate::diff_eq!(o, n, "1415f1590 p2={}", p2);
    }
    Ok(2)
}

pub unsafe fn diff_std_14183df80(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut ARENA_BUF: [u8; 32] = [0; 32];
    static mut ENTRY_BUF: [u64; 3] = [0; 3];
    static mut ALLOC_SZ: u64 = 0;

    unsafe extern "win64" fn mock_dcc0(_arena: usize, sz: u64) -> usize {
        ALLOC_SZ = sz;
        core::ptr::addr_of_mut!(ARENA_BUF) as usize
    }
    unsafe extern "win64" fn mock_1c10(_map: usize, key_ptr: usize) -> usize {
        let ep = core::ptr::addr_of_mut!(ENTRY_BUF) as usize;
        wr::<u64>(ep, rd::<u64>(key_ptr));
        ep
    }

    let _m1 = crate::difftest::Mock::new(0x1_4242_dcc0, mock_dcc0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_413a_1c10, mock_1c10 as *const ())?;

    let run = |f: F| -> ([u8; 32], u64, usize, u64, u64) {
        ARENA_BUF = [0; 32];
        ENTRY_BUF = [0; 3];
        ALLOC_SZ = 0;

        let p1: [usize; 2] = [0, 0x2000];
        let key: u64 = 0xdead_beef_cafe_1234;
        let path_bytes = b"data/textures/hero.tex//";
        let pv: [u64; 2] = [path_bytes.as_ptr() as usize as u64, path_bytes.len() as u64];

        f(
            p1.as_ptr() as usize,
            (&key as *const u64) as usize,
            pv.as_ptr() as usize,
        );
        let base = core::ptr::addr_of!(ARENA_BUF) as usize;
        (
            ARENA_BUF,
            ENTRY_BUF[0],
            (ENTRY_BUF[1] as usize).wrapping_sub(base),
            ENTRY_BUF[2],
            ALLOC_SZ,
        )
    };

    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14183df80 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1418584f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let cases: [&[u8]; 6] = [
        b"ab",
        b"root/something",
        b"data/textures/hero.tex",
        b"data/shaders/uber.fx",
        b"data/lualibs/core.lua",
        b"DATA\\SHADERS\\post.fx",
    ];

    for (idx, &bytes) in cases.iter().enumerate() {
        let pv: [u64; 2] = [bytes.as_ptr() as usize as u64, bytes.len() as u64];
        let o = f_o(pv.as_ptr() as usize);
        let n = f_n(pv.as_ptr() as usize);
        crate::diff_eq!(o, n, "1418584f0 case={}", idx);
    }
    Ok(cases.len() as u32)
}

pub unsafe fn diff_content_141c6a020(orig: usize, new: usize) -> Result<u32, String> {
    let mut segs = [0u8; 0x54 * 2];
    let sp = segs.as_mut_ptr() as usize;

    // Segment 0: mode 2, range [0.0, 0.5]
    wr::<u32>(sp, 2);
    wr::<f32>(sp + 0x04, 0.0);
    wr::<f32>(sp + 0x0c, 0.5);
    wr::<f32>(sp + 0x14, 0.0);
    wr::<f32>(sp + 0x18, 10.0);
    wr::<f32>(sp + 0x1c, 0.5);
    wr::<f32>(sp + 0x20, 2.0);

    // Segment 1: mode 4 (constant), range [0.6, 1.0]
    let s1 = sp + 0x54;
    wr::<u32>(s1, 4);
    wr::<f32>(s1 + 0x04, 0.6);
    wr::<f32>(s1 + 0x0c, 1.0);
    wr::<f32>(s1 + 0x18, 42.5);

    let mut hdr = [0u8; 0x30];
    let hp = hdr.as_mut_ptr() as usize;
    wr::<usize>(hp, sp);
    wr::<u32>(hp + 0x08, 2);
    wr::<f32>(hp + 0x10, 0.1); // min clamp
    wr::<f32>(hp + 0x18, 0.9); // max clamp

    let mut tests = 0u32;
    for flag_28 in [0u32, 1, 2] {
        wr::<u32>(hp + 0x28, flag_28);
        for val in [-0.5f32, 0.25, 0.55, 0.75, 1.5] {
            let r_o = crate::simd_ops::call_va8_xmm(
                orig as u64,
                hp as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, val.to_bits() as u128, 0, 0],
            )
            .1 as u32;
            let r_n = crate::simd_ops::call_va8_xmm(
                new as u64,
                hp as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, val.to_bits() as u128, 0, 0],
            )
            .1 as u32;
            crate::diff_eq!(r_o, r_n, "141c6a020 flag={} val={}", flag_28, val);
            tests += 1;
        }
    }
    Ok(tests)
}

// REGISTRY (for games/alan-wake-2/crates/aw2-game/src/registry/core.rs):
// FLOAT_RETURN: 0x1_41c6_a020 (`content_141c6a020` returns `f32` in `xmm0`)
// Replacement::verified_throws(0x1_4135_3470, "rend_141353470", rend::cauldron_hot_w3::rend_141353470 as *const (), Throws::Caught, rend::cauldron_hot_w3::diff_rend_141353470),
// Replacement::verified_throws(0x1_4135_3b70, "rend_141353b70", rend::cauldron_hot_w3::rend_141353b70 as *const (), Throws::Caught, rend::cauldron_hot_w3::diff_rend_141353b70),
// Replacement::verified_throws(0x1_4135_4150, "rend_141354150", rend::cauldron_hot_w3::rend_141354150 as *const (), Throws::Caught, rend::cauldron_hot_w3::diff_rend_141354150),
// Replacement::verified_throws(0x1_4135_58a0, "rend_1413558a0", rend::cauldron_hot_w3::rend_1413558a0 as *const (), Throws::Caught, rend::cauldron_hot_w3::diff_rend_1413558a0),
// Replacement::verified_throws(0x1_4135_6660, "rend_141356660", rend::cauldron_hot_w3::rend_141356660 as *const (), Throws::Caught, rend::cauldron_hot_w3::diff_rend_141356660),
// Replacement::verified(0x1_413a_b4d0, "r_1413ab4d0", rend::cauldron_hot_w3::r_1413ab4d0 as *const (), rend::cauldron_hot_w3::diff_r_1413ab4d0),
// Replacement::verified_throws(0x1_4141_0620, "ui_141410620", rend::cauldron_hot_w3::ui_141410620 as *const (), Throws::Discard, rend::cauldron_hot_w3::diff_ui_141410620),
// Replacement::verified(0x1_415e_e330, "unknown_1415ee330", rend::cauldron_hot_w3::unknown_1415ee330 as *const (), rend::cauldron_hot_w3::diff_unknown_1415ee330),
// Replacement::verified(0x1_415f_1590, "unknown_1415f1590", rend::cauldron_hot_w3::unknown_1415f1590 as *const (), rend::cauldron_hot_w3::diff_unknown_1415f1590),
// Replacement::verified_throws(0x1_4183_df80, "std_14183df80", rend::cauldron_hot_w3::std_14183df80 as *const (), Throws::Discard, rend::cauldron_hot_w3::diff_std_14183df80),
// Replacement::verified_throws(0x1_4185_84f0, "unknown_1418584f0", rend::cauldron_hot_w3::unknown_1418584f0 as *const (), Throws::Discard, rend::cauldron_hot_w3::diff_unknown_1418584f0),
// Replacement::verified(0x1_41c6_a020, "content_141c6a020", rend::cauldron_hot_w3::content_141c6a020 as *const (), rend::cauldron_hot_w3::diff_content_141c6a020),
