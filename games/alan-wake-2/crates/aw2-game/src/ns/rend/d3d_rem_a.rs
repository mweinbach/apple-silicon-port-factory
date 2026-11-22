//! Decompiled `d3d` renderer functions (`0x14204f540..0x1420768e0`).

use crate::exe::{self, rd, wr};

#[inline]
unsafe fn lock_recursive_srw(lock: usize) {
    let tid: u32 = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock + 0x8) != tid {
        exe::call_iat!(0x1_4264_55a8, (lock) -> ());
        wr::<u32>(lock + 0x8, tid);
    }
    wr::<u32>(lock + 0xc, rd::<u32>(lock + 0xc).wrapping_add(1));
}

#[inline]
unsafe fn unlock_recursive_srw(lock: usize) {
    let next = rd::<u32>(lock + 0xc).wrapping_sub(1);
    wr::<u32>(lock + 0xc, next);
    if next == 0 {
        wr::<u32>(lock + 0x8, u32::MAX);
        exe::call_iat!(0x1_4264_5598, (lock) -> ());
    }
}

/// Original: `0x14204f540` (380 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_14204f540(
    this: usize,
    desc: usize,
    _a3: usize,
    a4: usize,
) -> usize {
    wr::<usize>(this, 0);
    let mut tmp_ptr: usize = a4;
    let d0 = rd::<u32>(desc);
    let d1 = rd::<u32>(desc + 0x4);
    let d2 = rd::<u32>(desc + 0x8);
    let d3 = rd::<u32>(desc + 0xc);
    let d4 = rd::<u8>(desc + 0x10);
    let d5 = rd::<u32>(desc + 0x14);
    let ret_ptr = exe::call_va!(
        0x1_4206_a100,
        ((&mut tmp_ptr as *mut usize) as usize, d0, d1, d2, d3, d4, d5) -> usize
    );
    let new_heap = rd::<usize>(ret_ptr);
    wr::<usize>(ret_ptr, 0);
    let old_heap = rd::<usize>(this);
    wr::<usize>(this, new_heap);
    if old_heap != 0 {
        exe::call_va!(0x1_4206_b020, (old_heap) -> ());
        exe::call_va!(0x1_4244_2de0, (old_heap) -> ());
    }
    if tmp_ptr != 0 {
        exe::call_va!(0x1_4206_b020, (tmp_ptr) -> ());
        exe::call_va!(0x1_4244_2de0, (tmp_ptr) -> ());
    }
    let cur_heap = rd::<usize>(this);
    let stride = rd::<u32>(cur_heap + 0xc);
    wr::<u32>(this + 0xc, rd::<u32>(desc));
    wr::<u32>(this + 0x10, rd::<u32>(desc + 0x4));
    wr::<u32>(this + 0x14, rd::<u32>(desc + 0x8));
    wr::<u8>(this + 0x1c, rd::<u8>(desc + 0x10));
    wr::<u32>(this + 0x18, stride);
    exe::call_va!(0x1_4183_acc2, (this + 0x20, 0u32, 0x200usize) -> usize);
    let v = exe::call_va!(
        0x1_4204_e5e0,
        (
            rd::<u32>(desc),
            rd::<u32>(desc + 0x4),
            rd::<u32>(desc + 0x8),
            rd::<u32>(desc + 0xc),
            rd::<u8>(desc + 0x10),
        ) -> u32
    );
    wr::<u32>(this + 0x8, v);
    this
}

/// Original: `0x14204f6c0` (331 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn d3d_14204f6c0(this: usize, _a2: usize, idx: i32) -> usize {
    if idx == 0 {
        return rd::<usize>(this);
    }
    let idx_isize = idx as isize;
    let vec = this + 0x20;
    let base = rd::<usize>(vec);
    let len = rd::<u32>(vec + 0x8) as u64;
    let needed = (idx.wrapping_add(1) as i64) as u64;
    let off = (idx_isize as usize).wrapping_mul(8);
    if len < needed || rd::<usize>(base.wrapping_add(off)) == 0 {
        let lock = this + 0x30;
        lock_recursive_srw(lock);
        exe::call_va!(0x1_411d_2ea0, (vec, 0x10u32) -> ());
        let cur_base = rd::<usize>(vec);
        if rd::<usize>(cur_base.wrapping_add(off)) == 0 {
            let mut tmp_ptr: usize = 0;
            let root = rd::<usize>(this);
            let flag = rd::<u8>(root + 0x18);
            exe::call_va!(
                0x1_4206_73e0,
                ((&mut tmp_ptr as *mut usize) as usize, root, idx as u32, 0u32, flag) -> usize
            );
            let new_obj = tmp_ptr;
            tmp_ptr = 0;
            let slot = rd::<usize>(vec).wrapping_add(off);
            let old_obj = rd::<usize>(slot);
            wr::<usize>(slot, new_obj);
            if old_obj != 0 {
                exe::call_va!(0x1_4206_b020, (old_obj) -> ());
                exe::call_va!(0x1_4244_2de0, (old_obj) -> ());
            }
            if tmp_ptr != 0 {
                exe::call_va!(0x1_4206_b020, (tmp_ptr) -> ());
                exe::call_va!(0x1_4244_2de0, (tmp_ptr) -> ());
            }
        }
        unlock_recursive_srw(lock);
    }
    rd::<usize>(rd::<usize>(vec).wrapping_add(off))
}

/// Original: `0x142050470` (277 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142050470(this: usize, size: u64) -> usize {
    let raw = exe::call_va!(0x1_4243_b450, (this + 0xb0) -> usize);
    let out = if raw != 0 {
        wr::<u64>(raw, 0);
        wr::<u64>(raw + 0x8, 0);
        wr::<u64>(raw + 0x10, 0);
        raw
    } else {
        0
    };
    if size > 0x10_0000 {
        let blk = exe::call_va!(0x1_4205_0230, (this, size) -> usize);
        wr::<usize>(out, blk);
        wr::<u64>(out + 0x8, 0);
        wr::<u64>(out + 0x10, size);
        return out;
    }
    let lock = this + 0x130;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let mut blk = rd::<usize>(this + 0x138);
    let need_alloc = if blk == 0 {
        true
    } else if size <= rd::<u64>(blk + 0x60) {
        false
    } else {
        if rd::<u64>(blk + 0x50) == 0 {
            exe::call_va!(0x1_4207_5a20, (blk) -> ());
            exe::call_va!(0x1_4243_b3e0, (this + 0xf0, blk) -> ());
        }
        true
    };
    if need_alloc {
        blk = exe::call_va!(0x1_4205_0230, (this, 0x20_0000u64) -> usize);
        wr::<usize>(this + 0x138, blk);
    }
    wr::<usize>(out, blk);
    let cur = rd::<usize>(this + 0x138);
    wr::<u64>(out + 0x8, rd::<u64>(cur + 0x58));
    wr::<u64>(out + 0x10, size);
    wr::<u64>(cur + 0x50, rd::<u64>(cur + 0x50).wrapping_add(1));
    wr::<u64>(cur + 0x58, rd::<u64>(cur + 0x58).wrapping_add(size));
    wr::<u64>(cur + 0x60, rd::<u64>(cur + 0x60).wrapping_sub(size));
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
    out
}

/// Original: `0x142050690` (291 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142050690(this: usize, out: usize, size: u64) -> usize {
    wr::<u64>(out, 0);
    wr::<u64>(out + 0x10, 0);
    wr::<u64>(out + 0x8, size);
    let mut small_idx: u64 = 0;
    if size != 0 {
        let m1 = size.wrapping_sub(1);
        small_idx = m1 >> 4;
        if small_idx >= 8 {
            let bit: u64 = if m1 == 0 {
                0
            } else {
                (63 - m1.leading_zeros()) as u64
            };
            let pow2 = 1u64 << (bit & 0x3f);
            let step = pow2 >> 2;
            let num = size.wrapping_sub(pow2).wrapping_sub(1);
            let quot = num / step;
            let bucket = quot.wrapping_add(bit.wrapping_sub(7).wrapping_mul(4));
            if bucket < 0x27 {
                let table = rd::<usize>(this + 0x20);
                let pool = rd::<usize>(table + (bucket as usize) * 8);
                let mut pair = [0u64; 2];
                exe::call_va!(0x1_411d_35d0, (pool, pair.as_mut_ptr() as usize) -> usize);
                wr::<u64>(out + 0x10, pair[0]);
                wr::<u64>(out, pair[1]);
            } else {
                let raw = exe::call_va!(0x1_4243_b450, (this + 0x30) -> usize);
                if raw != 0 {
                    let res = exe::call_va!(0x1_411d_32d0, (raw, size, 1u64) -> u64);
                    wr::<u64>(out + 0x10, res);
                    wr::<u64>(out, 0);
                } else {
                    wr::<u64>(out, 0);
                    wr::<u64>(out + 0x10, 0);
                }
            }
            return out;
        }
    }
    let table = rd::<usize>(this + 0x10);
    let pool = rd::<usize>(table + (small_idx as usize) * 8);
    let mut pair = [0u64; 2];
    exe::call_va!(0x1_411d_35d0, (pool, pair.as_mut_ptr() as usize) -> usize);
    wr::<u64>(out + 0x10, pair[0]);
    wr::<u64>(out, pair[1]);
    out
}

/// Original: `0x142050ef0` (319 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142050ef0(this: usize) {
    if rd::<u64>(this + 0x50) == 0 {
        return;
    }
    exe::call_va!(0x1_4207_5970, (this) -> ());
    let val_40 = rd::<u64>(this + 0x40);
    let size = rd::<u64>(this + 0x48);
    let ptr_50 = rd::<usize>(this + 0x50);
    let g_addr = exe::ptr::<usize>(0x1_4397_fb88) as usize;
    let pool = if size == 0 {
        let g = rd::<usize>(g_addr);
        rd::<usize>(rd::<usize>(g + 0x10))
    } else {
        let m1 = size.wrapping_sub(1);
        let small_idx = m1 >> 4;
        if small_idx < 8 {
            let g = rd::<usize>(g_addr);
            rd::<usize>(rd::<usize>(g + 0x10) + (small_idx as usize) * 8)
        } else {
            let bit: u64 = if m1 == 0 {
                0
            } else {
                (63 - m1.leading_zeros()) as u64
            };
            let pow2 = 1u64 << (bit & 0x3f);
            let step = pow2 >> 2;
            let num = size.wrapping_sub(pow2).wrapping_sub(1);
            let quot = num / step;
            let bucket = quot.wrapping_add(bit.wrapping_sub(7).wrapping_mul(4));
            if bucket >= 0x27 {
                let g = rd::<usize>(g_addr);
                if ptr_50 != 0 {
                    if rd::<u64>(ptr_50 + 0xd8) == 0 {
                        exe::call_va!(0x1_4207_5a20, (ptr_50 + 0x10) -> ());
                        if rd::<u64>(ptr_50 + 0xa0) != 0 {
                            exe::call_va!(0x1_4207_5a20, (ptr_50 + 0x60) -> ());
                        }
                    }
                    exe::call_va!(0x1_401d_e830, (ptr_50 + 0xc0) -> ());
                    exe::call_va!(0x1_4243_b3e0, (g + 0x30, ptr_50) -> ());
                }
                wr::<u64>(this + 0x40, 0);
                wr::<u64>(this + 0x48, 0);
                wr::<u64>(this + 0x50, 0);
                return;
            }
            let g = rd::<usize>(g_addr);
            rd::<usize>(rd::<usize>(g + 0x20) + (bucket as usize) * 8)
        }
    };
    let g = rd::<usize>(g_addr);
    exe::call_va!(0x1_4205_0340, (g, pool, ptr_50, val_40, size) -> ());
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0x50, 0);
}

/// Original: `0x142051a20` (369 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142051a20(this: usize) -> usize {
    let lock = this + 0xe0;
    lock_recursive_srw(lock);
    let raw = exe::call_va!(0x1_4243_b450, (this + 0x38) -> usize);
    let node = if raw != 0 {
        wr::<u64>(raw, 0);
        wr::<u64>(raw + 0x10, 0);
        wr::<u64>(raw + 0x8, 0);
        raw
    } else {
        0
    };
    unlock_recursive_srw(lock);

    let mut desc = [0u8; 0x30];
    let d_base = desc.as_mut_ptr() as usize;
    wr::<u64>(d_base, rd::<u64>(this + 0xc0));
    wr::<u32>(d_base + 0x8, rd::<u32>(this + 0x100));
    wr::<u64>(d_base + 0xc, 0);
    wr::<u32>(d_base + 0x14, 1);
    wr::<u32>(d_base + 0x18, 1);
    wr::<u64>(d_base + 0x20, 0);
    wr::<u32>(d_base + 0x28, 0xc0);

    let dev_addr = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let mut dev = rd::<usize>(dev_addr);
    let create_heap = rd::<usize>(rd::<usize>(dev) + 0xe0);
    let old_tag = rd::<u32>(node);
    let old_obj = rd::<usize>(node + 0x8);
    if old_obj != 0 {
        exe::call_va!(0x1_4209_a1e0, (old_obj, old_tag) -> ());
        dev = rd::<usize>(dev_addr);
    }
    wr::<usize>(node + 0x8, 0);
    wr::<u32>(node, old_tag);
    let riid = exe::ptr::<u8>(0x1_433d_bfe8) as usize;
    let hr = exe::call_va!(
        create_heap,
        (dev, d_base, riid, node + 0x8, old_tag, 0usize) -> u32
    );
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
    }
    wr::<u32>(node, 1);
    node
}

/// Original: `0x142051ba0` (286 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142051ba0(_a1: usize, size: u64) -> usize {
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fb90) as usize);
    let lock = mgr + 0xd0;
    lock_recursive_srw(lock);
    let raw = exe::call_va!(0x1_4243_b450, (mgr + 0x78) -> usize);
    let out = if raw != 0 {
        wr::<[u8; 16]>(raw, [0u8; 16]);
        wr::<u64>(raw + 0x10, 0);
        raw
    } else {
        0
    };
    let align = rd::<u64>(mgr + 0xc8);
    let mask = align.wrapping_sub(1);
    let aligned_size = (!mask) & mask.wrapping_add(size);
    let count = rd::<u32>(mgr + 0x18);
    let arr = rd::<usize>(mgr + 0x10);
    let mut blk: usize = 0;
    if count != 0 {
        let mut i: u32 = 0;
        loop {
            let idx = (i as i32 as isize) as usize;
            let cand = rd::<usize>(arr + idx * 8);
            if (rd::<u32>(cand + 0x10) as u64) >= aligned_size {
                blk = cand;
                break;
            }
            i = i.wrapping_add(1);
            if ((i as i32 as i64) as u64) >= (count as u64) {
                break;
            }
        }
    }
    if blk == 0 {
        blk = exe::call_va!(0x1_4205_1710, (mgr) -> usize);
    }
    wr::<usize>(out + 0x10, mgr);
    wr::<usize>(out, blk);
    let off = rd::<u32>(mgr + 0xc0).wrapping_sub(rd::<u32>(blk + 0x10));
    wr::<u32>(out + 0x8, off);
    wr::<u32>(out + 0xc, aligned_size as u32);
    wr::<u32>(
        blk + 0x10,
        rd::<u32>(blk + 0x10).wrapping_sub(aligned_size as u32),
    );
    wr::<u32>(blk + 0x14, rd::<u32>(blk + 0x14).wrapping_add(1));
    wr::<u64>(mgr + 0xf8, rd::<u64>(mgr + 0xf8).wrapping_add(aligned_size));
    unlock_recursive_srw(lock);
    out
}

/// Original: `0x142051f70` (397 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142051f70(
    this: usize,
    a2: u64,
    a3: u64,
    count: u64,
    flags: u32,
) -> usize {
    for off in (0..0x38usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    exe::call_va!(
        0x1_4243_b5f0,
        (this + 0x38, a2, a3, (count as u32).wrapping_mul(2), 0x18u64, 0x80u32, 0x8u64) -> usize
    );
    exe::call_va!(
        0x1_4243_b5f0,
        (this + 0x78, 0usize, 0usize, 0x2004u32, 0x18u64, 0x80u32, 0x8u64) -> usize
    );
    wr::<u64>(this + 0xb8, count);
    wr::<u64>(this + 0xc0, a2);
    wr::<u64>(this + 0xc8, a3);
    wr::<u32>(this + 0xd8, u32::MAX);
    wr::<u32>(this + 0xdc, 0);
    wr::<u64>(this + 0xd0, 0);
    wr::<u32>(this + 0xe8, u32::MAX);
    wr::<u32>(this + 0xec, 0);
    wr::<u64>(this + 0xe0, 0);
    wr::<u32>(this + 0x100, flags);
    wr::<u64>(this + 0xf0, 0);
    wr::<u64>(this + 0xf8, 0);
    exe::call_va!(0x1_4034_97a0, (this + 0x20, count as u32) -> ());
    if count != 0 {
        let mut i: u32 = 0;
        loop {
            let item = exe::call_va!(0x1_4205_1a20, (this) -> usize);
            wr::<u64>(item + 0x10, 0);
            let r_len = rd::<u32>(this + 0x30);
            if r_len == rd::<u32>(this + 0x34) {
                exe::call_va!(0x1_4034_97a0, (this + 0x20, r_len.wrapping_add(1)) -> ());
            }
            let tail = rd::<u32>(this + 0x2c) as usize;
            let buf = rd::<usize>(this + 0x20);
            wr::<usize>(buf + tail * 8, item);
            let next_tail = rd::<u32>(this + 0x2c).wrapping_add(1);
            wr::<u32>(this + 0x2c, next_tail);
            if next_tail == rd::<u32>(this + 0x34) {
                wr::<u32>(this + 0x2c, 0);
            }
            wr::<u32>(this + 0x30, rd::<u32>(this + 0x30).wrapping_add(1));
            i = i.wrapping_add(1);
            if (i as u64) >= count {
                break;
            }
        }
    }
    this
}

/// Original: `0x142052100` (349 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142052100(this: usize, item: usize) {
    let lock = this + 0xc0;
    lock_recursive_srw(lock);
    let mut base = rd::<usize>(this);
    let mut len = rd::<u32>(this + 0x8) as usize;
    let mut cur = base;
    while cur != base + len * 8 {
        if rd::<usize>(cur) == item {
            let last = base + (len - 1) * 8;
            if cur != last {
                wr::<usize>(cur, rd::<usize>(last));
                base = rd::<usize>(this);
            }
            let new_len = rd::<u32>(this + 0x8).wrapping_sub(1);
            wr::<u32>(this + 0x8, new_len);
            len = new_len as usize;
        } else {
            cur += 8;
        }
    }
    wr::<u64>(item + 0x10, 0);
    wr::<u64>(item + 0x18, u64::MAX);
    let chunk_sz = rd::<u64>(this + 0xa8);
    wr::<u64>(this + 0xd0, rd::<u64>(this + 0xd0).wrapping_sub(chunk_sz));
    if rd::<u16>(item + 0x20) == 0 {
        let tag = rd::<u32>(item);
        let obj = rd::<usize>(item + 0x8);
        if obj != 0 {
            exe::call_va!(0x1_4209_a1e0, (obj, tag) -> ());
        }
        wr::<usize>(item + 0x8, 0);
        wr::<u32>(item, tag);
        exe::call_va!(0x1_4243_b3e0, (this + 0x28, item) -> ());
    } else {
        let r_len = rd::<u32>(this + 0x20);
        if r_len == rd::<u32>(this + 0x24) {
            exe::call_va!(0x1_4034_97a0, (this + 0x10, r_len.wrapping_add(1)) -> ());
        }
        let tail = rd::<u32>(this + 0x1c) as usize;
        let buf = rd::<usize>(this + 0x10);
        wr::<usize>(buf + tail * 8, item);
        let next_tail = rd::<u32>(this + 0x1c).wrapping_add(1);
        wr::<u32>(this + 0x1c, next_tail);
        if next_tail == rd::<u32>(this + 0x24) {
            wr::<u32>(this + 0x1c, 0);
        }
        wr::<u32>(this + 0x20, rd::<u32>(this + 0x20).wrapping_add(1));
    }
    unlock_recursive_srw(lock);
}

/// Original: `0x142052270` (333 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142052270(this: usize) -> usize {
    let lock = this + 0xc0;
    lock_recursive_srw(lock);
    let r_len = rd::<u32>(this + 0x20);
    let item = if r_len == 0 {
        let created = exe::call_va!(0x1_4205_23d0, (this) -> usize);
        wr::<u16>(created + 0x20, 0);
        created
    } else {
        let buf = rd::<usize>(this + 0x10);
        let cap = rd::<u32>(this + 0x24) as u64;
        let head = rd::<u32>(this + 0x18);
        let idx = ((head as u64) % cap) as usize;
        let popped = rd::<usize>(buf + idx * 8);
        let next_head = head.wrapping_add(1);
        wr::<u32>(this + 0x18, if next_head == (cap as u32) { 0 } else { next_head });
        wr::<u32>(this + 0x20, r_len.wrapping_sub(1));
        popped
    };
    let chunk_sz = rd::<u64>(this + 0xa8);
    wr::<u64>(item + 0x10, chunk_sz);
    wr::<u64>(item + 0x18, 0);
    wr::<u64>(this + 0xd0, rd::<u64>(this + 0xd0).wrapping_add(chunk_sz));

    let v_len = rd::<u32>(this + 0x8);
    let v_cap = rd::<u32>(this + 0xc);
    let mut v_buf = rd::<usize>(this);
    if (v_len as u64) * 8 == (v_cap as u64) * 8 {
        let min_cap = (v_len as u64).wrapping_add(1);
        let grow = (v_cap as u64).wrapping_add((v_cap >> 1) as u64);
        let new_cap = if grow < min_cap { min_cap } else { grow };
        v_buf = exe::call_va!(0x1_401d_ec00, (this, new_cap) -> usize);
        let bytes = (rd::<u32>(this + 0x8) as usize).wrapping_mul(8);
        exe::call_va!(0x1_4183_acc8, (v_buf, rd::<usize>(this), bytes) -> usize);
        exe::call_va!(0x1_401d_e830, (this) -> ());
        wr::<usize>(this, v_buf);
        wr::<u32>(this + 0x8, v_len);
        wr::<u32>(this + 0xc, new_cap as u32);
    }
    wr::<usize>(v_buf + (v_len as usize) * 8, item);
    wr::<u32>(this + 0x8, rd::<u32>(this + 0x8).wrapping_add(1));
    unlock_recursive_srw(lock);
    item
}

/// Original: `0x1420523d0` (311 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420523d0(this: usize) -> usize {
    let raw = exe::call_va!(0x1_4243_b450, (this + 0x28) -> usize);
    let node = if raw != 0 {
        wr::<u64>(raw, 0);
        wr::<u64>(raw + 0x10, 0);
        wr::<u64>(raw + 0x18, 0);
        wr::<u64>(raw + 0x20, 0);
        wr::<u64>(raw + 0x8, 0);
        raw
    } else {
        0
    };
    let kind = rd::<u32>(this + 0xe0);
    let flags = if kind == 2 { 0u32 } else { 0x44u32 };
    let mut desc = [0u8; 0x30];
    let d_base = desc.as_mut_ptr() as usize;
    wr::<u64>(d_base, rd::<u64>(this + 0xa8));
    wr::<u32>(d_base + 0x8, kind);
    wr::<u64>(d_base + 0xc, 0);
    wr::<u32>(d_base + 0x14, 1);
    wr::<u32>(d_base + 0x18, 1);
    wr::<u64>(d_base + 0x20, 0);
    wr::<u32>(d_base + 0x28, flags);

    let dev_addr = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let mut dev = rd::<usize>(dev_addr);
    let create_heap = rd::<usize>(rd::<usize>(dev) + 0xe0);
    let old_tag = rd::<u32>(node);
    let old_obj = rd::<usize>(node + 0x8);
    if old_obj != 0 {
        exe::call_va!(0x1_4209_a1e0, (old_obj, old_tag) -> ());
        dev = rd::<usize>(dev_addr);
    }
    wr::<usize>(node + 0x8, 0);
    wr::<u32>(node, old_tag);
    let riid = exe::ptr::<u8>(0x1_433d_bfe8) as usize;
    let hr = exe::call_va!(
        create_heap,
        (dev, d_base, riid, node + 0x8, old_tag, 0usize) -> u32
    );
    if hr == 0x8007_000e || hr.wrapping_add(0x7785_fffb) <= 2 {
        exe::call_va!(0x1_4205_d410, (0usize, hr) -> ());
    }
    node
}

/// Original: `0x1420527c0` (288 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420527c0(this: usize) {
    let lock = this + 0xc0;
    lock_recursive_srw(lock);
    let ring = this + 0x10;
    while rd::<u32>(this + 0x20) != 0 {
        let r_buf = rd::<usize>(ring);
        let r_len = rd::<u32>(ring + 0x10);
        let new_len = r_len.wrapping_sub(1);
        let r_cap = rd::<u32>(ring + 0x14) as u64;
        let r_head = rd::<u32>(ring + 0x8) as u64;
        let idx = ((new_len as u64).wrapping_add(r_head) % r_cap) as usize;
        let item = rd::<usize>(r_buf + idx * 8);
        if r_len != 0 {
            let tail = rd::<u32>(this + 0x1c) as u64;
            let new_tail = (tail.wrapping_sub(1).wrapping_add(r_cap) % r_cap) as u32;
            wr::<u32>(this + 0x1c, new_tail);
            wr::<u32>(this + 0x20, new_len);
        }
        let tag = rd::<u32>(item);
        let obj = rd::<usize>(item + 0x8);
        if obj != 0 {
            exe::call_va!(0x1_4209_a1e0, (obj, tag) -> ());
        }
        wr::<usize>(item + 0x8, 0);
        wr::<u32>(item, tag);
        exe::call_va!(0x1_4243_b3e0, (this + 0x28, item) -> ());
    }
    unlock_recursive_srw(lock);
    exe::call_va!(0x1_4243_b540, (this + 0x68) -> ());
    exe::call_va!(0x1_4243_b540, (this + 0x28) -> ());
    exe::call_va!(0x1_4034_96f0, (ring) -> ());
    exe::call_va!(0x1_401d_e830, (this) -> ());
}

/// Original: `0x1420528f0` (380 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420528f0(
    this: usize,
    a2: u64,
    a3: u64,
    a4: u64,
    count: u64,
    kind: u32,
) -> usize {
    for off in (0..0x28usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    exe::call_va!(
        0x1_4243_b5f0,
        (this + 0x28, a2, a3, count as u32, 0x28u64, 0x80u32, 0x8u64) -> usize
    );
    exe::call_va!(
        0x1_4243_b5f0,
        (this + 0x68, 0usize, 0usize, 0x2004u32, 0x28u64, 0x80u32, 0x8u64) -> usize
    );
    wr::<u64>(this + 0xa8, a2);
    wr::<u64>(this + 0xb0, a3);
    wr::<u64>(this + 0xb8, a4);
    wr::<u32>(this + 0xc8, u32::MAX);
    wr::<u32>(this + 0xcc, 0);
    wr::<u64>(this + 0xc0, 0);
    wr::<u32>(this + 0xe0, kind);
    wr::<u64>(this + 0xd0, 0);
    wr::<u64>(this + 0xd8, 0);
    exe::call_va!(0x1_4034_97a0, (this + 0x10, count as u32) -> ());
    if count != 0 {
        let mut i: u32 = 0;
        loop {
            let item = exe::call_va!(0x1_4205_23d0, (this) -> usize);
            wr::<u64>(item + 0x10, 0);
            wr::<u64>(item + 0x18, u64::MAX);
            wr::<u16>(item + 0x20, 1);
            let r_len = rd::<u32>(this + 0x20);
            if r_len == rd::<u32>(this + 0x24) {
                exe::call_va!(0x1_4034_97a0, (this + 0x10, r_len.wrapping_add(1)) -> ());
            }
            let tail = rd::<u32>(this + 0x1c) as usize;
            let buf = rd::<usize>(this + 0x10);
            wr::<usize>(buf + tail * 8, item);
            let next_tail = rd::<u32>(this + 0x1c).wrapping_add(1);
            wr::<u32>(this + 0x1c, next_tail);
            if next_tail == rd::<u32>(this + 0x24) {
                wr::<u32>(this + 0x1c, 0);
            }
            wr::<u32>(this + 0x20, rd::<u32>(this + 0x20).wrapping_add(1));
            i = i.wrapping_add(1);
            if (i as u64) >= count {
                break;
            }
        }
    }
    this
}

/// Original: `0x142055df0` (286 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142055df0(this: usize, handle_ptr: usize) {
    if rd::<i32>(this) == -1 {
        return;
    }
    let idx = exe::call_va!(0x1_4205_5f80, (handle_ptr) -> i32) as isize as usize;
    let table = rd::<usize>(this + 0x50);
    let entry = rd::<usize>(table + idx * 8);
    exe::call_va!(0x1_4206_32b0, (rd::<usize>(entry + 0x8)) -> ());
    let get_time: unsafe extern "win64" fn() -> f64 = crate::exe::func(0x1_4240_69a0);
    let now = get_time();
    wr::<u64>(entry + 0x18, now.to_bits());

    let lock = exe::ptr::<u8>(0x1_4385_79c0) as usize;
    lock_recursive_srw(lock);
    let ring_arr = rd::<usize>(this + 0x28);
    let len_m1 = rd::<u32>(this + 0x38).wrapping_sub(1) as u64;
    let head = rd::<u32>(this + 0x30) as u64;
    let cap = rd::<u32>(this + 0x3c) as u64;
    let ring_idx = (len_m1.wrapping_add(head) % cap) as usize;
    let sub_ring = rd::<usize>(ring_arr + ring_idx * 8);
    let h_val = rd::<u32>(handle_ptr);
    let s_len = rd::<u32>(sub_ring + 0x10);
    if s_len == rd::<u32>(sub_ring + 0x14) {
        exe::call_va!(0x1_411d_5000, (sub_ring, s_len.wrapping_add(1)) -> ());
    }
    let s_tail = rd::<u32>(sub_ring + 0xc) as usize;
    let dst = rd::<usize>(sub_ring) + s_tail * 16;
    wr::<usize>(dst, entry);
    wr::<u32>(dst + 0x8, h_val);
    let next_tail = rd::<u32>(sub_ring + 0xc).wrapping_add(1);
    wr::<u32>(
        sub_ring + 0xc,
        if next_tail == rd::<u32>(sub_ring + 0x14) {
            0
        } else {
            next_tail
        },
    );
    wr::<u32>(sub_ring + 0x10, rd::<u32>(sub_ring + 0x10).wrapping_add(1));
    unlock_recursive_srw(lock);

    let table_after = rd::<usize>(this + 0x50);
    wr::<usize>(table_after + idx * 8, 0);
}

/// Original: `0x142056040` (338 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142056040(this: usize) {
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc08) as usize, 0);
    let mut p0 = rd::<usize>(this + 0x8);
    let end0 = p0 + (rd::<u32>(this + 0x10) as usize) * 8;
    while p0 != end0 {
        let item = rd::<usize>(p0);
        if item != 0 {
            let sub1 = rd::<usize>(item + 0x8);
            if sub1 != 0 {
                exe::call_va!(0x1_4244_2de0, (sub1) -> ());
            }
            let sub0 = rd::<usize>(item);
            if sub0 != 0 {
                exe::call_va!(0x1_4244_2de0, (sub0) -> ());
            }
            exe::call_va!(0x1_4244_2de0, (item) -> ());
        }
        p0 += 8;
    }

    let mut p1 = rd::<usize>(this + 0x60);
    let end1 = p1 + (rd::<u32>(this + 0x68) as usize) * 8;
    while p1 != end1 {
        let item = rd::<usize>(p1);
        if item != 0 {
            exe::call_va!(0x1_4244_2de0, (item) -> ());
        }
        p1 += 8;
    }

    let mut p2 = rd::<usize>(this + 0x80);
    let end2 = p2 + (rd::<u32>(this + 0x88) as usize) * 8;
    while p2 != end2 {
        let item = rd::<usize>(p2);
        if item != 0 {
            exe::call_va!(0x1_4034_96f0, (item) -> ());
            exe::call_va!(0x1_4244_2de0, (item) -> ());
        }
        p2 += 8;
    }

    exe::call_va!(0x1_401d_e830, (this + 0x90) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x80) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x70) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x60) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x50) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x40) -> ());
    exe::call_va!(0x1_4034_96f0, (this + 0x28) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x18) -> ());
    exe::call_va!(0x1_401d_e830, (this + 0x8) -> ());
}

/// Original: `0x1420561a0` (309 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420561a0(this: usize) -> usize {
    wr::<u32>(this, u32::MAX);
    for off in (0x8..0xa0usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    wr::<usize>(exe::ptr::<usize>(0x1_4397_fc08) as usize, this);
    let old_len = rd::<u32>(this + 0x58) as u64;
    let old_cap = rd::<u32>(this + 0x5c) as u64;
    let target: u64 = 0x2000;
    if old_cap < target {
        let grow = old_cap.wrapping_add(old_cap >> 1);
        let new_cap = if grow < target { target } else { grow };
        exe::call_va!(0x1_4087_46b0, (this + 0x50, new_cap) -> ());
        let cur_len = rd::<u32>(this + 0x58) as usize;
        let base = rd::<usize>(this + 0x50);
        let rem = target.wrapping_sub(old_len);
        if rem != 0 {
            exe::call_va!(0x1_4183_acc2, (base + cur_len * 8, 0u32, (rem as usize) * 8) -> usize);
        }
    } else if old_len < target {
        let rem = target.wrapping_sub(old_len);
        if rem != 0 {
            let base = rd::<usize>(this + 0x50);
            exe::call_va!(0x1_4183_acc2, (base + (old_len as usize) * 8, 0u32, (rem as usize) * 8) -> usize);
        }
    } else if old_len == target {
        return this;
    }
    wr::<u32>(this + 0x58, target as u32);
    this
}

/// Original: `0x142057130` (301 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142057130(this: usize) {
    wr::<usize>(this, exe::ptr::<u8>(0x1_433d_c338) as usize);
    if rd::<u64>(this + 0x8) == u64::MAX {
        return;
    }
    let lock = exe::ptr::<u8>(0x1_4385_7a40) as usize;
    lock_recursive_srw(lock);
    let vec_addr = exe::ptr::<u8>(0x1_43a6_3238) as usize;
    let base = rd::<usize>(vec_addr);
    let len = rd::<u32>(vec_addr + 0x8) as usize;
    let end = base.wrapping_add(len * 8);
    let mut dst = base;
    while dst != end && rd::<usize>(dst) != this {
        dst = dst.wrapping_add(8);
    }
    if dst != end {
        let mut src = dst.wrapping_add(8);
        while src != end {
            let v = rd::<usize>(src);
            if v != this {
                wr::<usize>(dst, v);
                dst = dst.wrapping_add(8);
            }
            src = src.wrapping_add(8);
        }
    }
    let cur_len = rd::<u32>(vec_addr + 0x8) as usize;
    let cur_base = rd::<usize>(vec_addr);
    let cur_end = cur_base.wrapping_add(cur_len * 8);
    if dst != cur_end {
        if dst == cur_base {
            wr::<u32>(vec_addr + 0x8, 0);
        } else {
            let move_bytes = (cur_len * 8).wrapping_sub(cur_end).wrapping_add(cur_base);
            exe::call_va!(0x1_4183_acc8, (dst, cur_end, move_bytes) -> usize);
            let removed = ((cur_end.wrapping_sub(dst)) as isize >> 3) as u32;
            wr::<u32>(
                vec_addr + 0x8,
                rd::<u32>(vec_addr + 0x8).wrapping_sub(removed),
            );
        }
    }
    unlock_recursive_srw(lock);
}

/// Original: `0x142057410` (301 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142057410(this: usize) {
    wr::<usize>(this, exe::ptr::<u8>(0x1_433d_c350) as usize);
    if rd::<u64>(this + 0x8) == u64::MAX {
        return;
    }
    let lock = exe::ptr::<u8>(0x1_4385_7a30) as usize;
    lock_recursive_srw(lock);
    let vec_addr = exe::ptr::<u8>(0x1_43a6_3228) as usize;
    let base = rd::<usize>(vec_addr);
    let len = rd::<u32>(vec_addr + 0x8) as usize;
    let end = base.wrapping_add(len * 8);
    let mut dst = base;
    while dst != end && rd::<usize>(dst) != this {
        dst = dst.wrapping_add(8);
    }
    if dst != end {
        let mut src = dst.wrapping_add(8);
        while src != end {
            let v = rd::<usize>(src);
            if v != this {
                wr::<usize>(dst, v);
                dst = dst.wrapping_add(8);
            }
            src = src.wrapping_add(8);
        }
    }
    let cur_len = rd::<u32>(vec_addr + 0x8) as usize;
    let cur_base = rd::<usize>(vec_addr);
    let cur_end = cur_base.wrapping_add(cur_len * 8);
    if dst != cur_end {
        if dst == cur_base {
            wr::<u32>(vec_addr + 0x8, 0);
        } else {
            let move_bytes = (cur_len * 8).wrapping_sub(cur_end).wrapping_add(cur_base);
            exe::call_va!(0x1_4183_acc8, (dst, cur_end, move_bytes) -> usize);
            let removed = ((cur_end.wrapping_sub(dst)) as isize >> 3) as u32;
            wr::<u32>(
                vec_addr + 0x8,
                rd::<u32>(vec_addr + 0x8).wrapping_sub(removed),
            );
        }
    }
    unlock_recursive_srw(lock);
}

/// Original: `0x142058010` (292 bytes, `Throws::No`).
#[allow(clippy::too_many_arguments)]
pub unsafe extern "win64" fn d3d_142058010(
    _a1: usize,
    _a2: usize,
    a3: u32,
    a4: u8,
    a5: u8,
    a6: u8,
    a7: u32,
    a8: u32,
    a9: usize,
    a10: usize,
    a11: usize,
    a12: usize,
    m0: usize,
    m1: usize,
    m2: usize,
    m3: usize,
) {
    wr::<[u8; 64]>(exe::ptr::<u8>(0x1_4385_7a50) as usize, rd::<[u8; 64]>(m0));
    wr::<[u8; 64]>(exe::ptr::<u8>(0x1_4385_7a90) as usize, rd::<[u8; 64]>(m1));
    wr::<[u8; 64]>(exe::ptr::<u8>(0x1_4385_7ad0) as usize, rd::<[u8; 64]>(m2));
    wr::<[u8; 64]>(exe::ptr::<u8>(0x1_4385_7b10) as usize, rd::<[u8; 64]>(m3));
    wr::<u32>(exe::ptr::<u32>(0x1_4385_7bdc) as usize, a3);
    let blend = if a6 == 0 {
        0u32
    } else {
        rd::<u32>(exe::ptr::<u32>(0x1_4342_68d0) as usize)
    };
    wr::<u32>(exe::ptr::<u32>(0x1_4385_7b98) as usize, blend);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_7ba0) as usize, a9 as u32);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_7b9c) as usize, a8);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_7ba8) as usize, a11 as u32);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_7ba4) as usize, a10 as u32);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_7bcc) as usize, a7);
    wr::<u32>(exe::ptr::<u32>(0x1_4385_7bc0) as usize, a12 as u32);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_7be0) as usize, a4 ^ 1);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_7be1) as usize, 0);
    wr::<u8>(exe::ptr::<u8>(0x1_4385_7be2) as usize, a5);
}

/// Original: `0x14206d170` (359 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_14206d170(
    a1: i32,
    a2: i32,
    a3: u64,
    s1_ptr: usize,
    s2_ptr: usize,
) {
    let idx = a1.wrapping_add(a2.wrapping_mul(3)) as i64 as isize as usize;
    let off = idx.wrapping_mul(0x120);
    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let empty_cstr = exe::ptr::<u8>(0x1_426c_dcb0) as usize;

    let base0 = rd::<usize>(g_addr).wrapping_add(off);
    let cap0 = rd::<u32>(base0 + 0x1360) as u64;
    exe::call_va!(0x1_423e_f2b0, (base0 + 0x1350, cap0) -> ());
    let mut conv0: u64 = a3;
    exe::call_iat!(
        0x1_4264_6508,
        (
            (&mut conv0 as *mut u64) as usize,
            rd::<usize>(base0 + 0x1350),
            rd::<u32>(base0 + 0x135c) as u64,
            empty_cstr,
            0u64,
        ) -> i32
    );

    let base1 = rd::<usize>(g_addr).wrapping_add(off);
    wr::<u32>(base0 + 0x1360, 0);
    let str1 = if s1_ptr != 0 { s1_ptr } else { empty_cstr };
    let mut len1: usize = 0;
    while rd::<u8>(str1 + len1) != 0 {
        len1 += 1;
    }
    let len1_u64 = (len1 as u32) as u64;
    let old1 = rd::<u32>(base1 + 0x20e0) as u64;
    let req1 = if len1_u64 >= old1 { len1_u64 } else { old1 };
    exe::call_va!(0x1_423e_f2b0, (base1 + 0x20d0, req1) -> ());
    let mut conv1: u64 = 0;
    exe::call_iat!(
        0x1_4264_6508,
        (
            (&mut conv1 as *mut u64) as usize,
            rd::<usize>(base1 + 0x20d0),
            rd::<u32>(base1 + 0x20dc) as u64,
            str1,
            len1_u64,
        ) -> i32
    );
    wr::<u32>(base1 + 0x20e0, len1 as u32);

    let base2 = rd::<usize>(g_addr).wrapping_add(off);
    let str2 = if s2_ptr != 0 { s2_ptr } else { empty_cstr };
    let mut len2: usize = 0;
    while rd::<u8>(str2 + len2) != 0 {
        len2 += 1;
    }
    let len2_u64 = (len2 as u32) as u64;
    let old2 = rd::<u32>(base2 + 0x2e60) as u64;
    let req2 = if len2_u64 >= old2 { len2_u64 } else { old2 };
    exe::call_va!(0x1_423e_f2b0, (base2 + 0x2e50, req2) -> ());
    exe::call_iat!(
        0x1_4264_6508,
        (
            (&mut conv0 as *mut u64) as usize,
            rd::<usize>(base2 + 0x2e50),
            rd::<u32>(base2 + 0x2e5c) as u64,
            str2,
            len2_u64,
        ) -> i32
    );
    wr::<u32>(base2 + 0x2e60, len2 as u32);
}

/// Original: `0x14206ea00` (344 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14206ea00(this: usize, val: usize) {
    exe::call_va!(0x1_4206_93a0, (this) -> ());
    let ctx0 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    exe::call_va!(
        0x1_4209_3d00,
        (ctx0, 0x9au32, ctx0, ctx0 + 0x10, ctx0 + 0x20) -> ()
    );
    exe::call_va!(
        0x1_4209_2930,
        (ctx0, 0u32, ctx0 + 0x8, ctx0 + 0x18, ctx0 + 0x28) -> ()
    );
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let g_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    exe::call_va!(
        0x1_4207_1360,
        (tls0 + 0x467e0, rd::<usize>(g_dev + 0x20)) -> ()
    );
    let ctx1 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    let stride0 = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let off_4d = stride0.wrapping_mul(0x4d);
    let cpu_base = rd::<u64>(ctx1 + 0x10).wrapping_add(off_4d);
    let gpu_base = rd::<u64>(ctx1).wrapping_add(off_4d);
    let dst_cpu = cpu_base.wrapping_add(stride0.wrapping_mul(0x3d));
    let d3d_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let copy_desc = rd::<usize>(rd::<usize>(d3d_dev) + 0xc0);
    exe::call_va!(
        copy_desc,
        (d3d_dev, 1u32, dst_cpu, rd::<u64>(this + 0x90), 0u32) -> ()
    );
    let stride1 = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let dst_gpu = gpu_base.wrapping_add(stride1.wrapping_mul(0x3d));
    let mut cmd_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = cmd_pair[1];
    let clear_uav = rd::<usize>(rd::<usize>(cmd_list) + 0x188);
    exe::call_va!(
        clear_uav,
        (
            cmd_list,
            dst_gpu,
            rd::<u64>(this + 0x90),
            rd::<usize>(this + 0x70),
            val,
            0u32,
            0usize,
        ) -> ()
    );
    if cmd_pair[0] != 0 {
        unlock_recursive_srw(cmd_pair[0]);
    }
}

/// Original: `0x14206eb60` (344 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14206eb60(this: usize, val: usize) {
    exe::call_va!(0x1_4206_93a0, (this) -> ());
    let ctx0 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    exe::call_va!(
        0x1_4209_3d00,
        (ctx0, 0x9au32, ctx0, ctx0 + 0x10, ctx0 + 0x20) -> ()
    );
    exe::call_va!(
        0x1_4209_2930,
        (ctx0, 0u32, ctx0 + 0x8, ctx0 + 0x18, ctx0 + 0x28) -> ()
    );
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let g_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    exe::call_va!(
        0x1_4207_1360,
        (tls0 + 0x467e0, rd::<usize>(g_dev + 0x20)) -> ()
    );
    let ctx1 = exe::call_va!(0x1_411d_cdf0, () -> usize);
    let stride0 = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let off_4d = stride0.wrapping_mul(0x4d);
    let cpu_base = rd::<u64>(ctx1 + 0x10).wrapping_add(off_4d);
    let gpu_base = rd::<u64>(ctx1).wrapping_add(off_4d);
    let dst_cpu = cpu_base.wrapping_add(stride0.wrapping_mul(0x3d));
    let d3d_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let copy_desc = rd::<usize>(rd::<usize>(d3d_dev) + 0xc0);
    exe::call_va!(
        copy_desc,
        (d3d_dev, 1u32, dst_cpu, rd::<u64>(this + 0x90), 0u32) -> ()
    );
    let stride1 = rd::<u32>(exe::ptr::<u32>(0x1_4397_fcb0) as usize) as u64;
    let dst_gpu = gpu_base.wrapping_add(stride1.wrapping_mul(0x3d));
    let mut cmd_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = cmd_pair[1];
    let clear_uav = rd::<usize>(rd::<usize>(cmd_list) + 0x190);
    exe::call_va!(
        clear_uav,
        (
            cmd_list,
            dst_gpu,
            rd::<u64>(this + 0x90),
            rd::<usize>(this + 0x70),
            val,
            0u32,
            0usize,
        ) -> ()
    );
    if cmd_pair[0] != 0 {
        unlock_recursive_srw(cmd_pair[0]);
    }
}

/// Original: `0x14206ee70` (159 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14206ee70(this: usize, groups: usize) {
    let mut cmd_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = cmd_pair[1];
    let set_pso = rd::<usize>(rd::<usize>(cmd_list) + 0xc8);
    let pso = exe::call_va!(0x1_4206_04d0, (rd::<usize>(this + 0xc58)) -> usize);
    exe::call_va!(set_pso, (cmd_list, pso) -> ());
    exe::call_va!(0x1_4207_11e0, (cmd_list, this) -> ());
    let dispatch = rd::<usize>(rd::<usize>(cmd_list) + 0x70);
    exe::call_va!(
        dispatch,
        (
            cmd_list,
            rd::<u32>(groups),
            rd::<u32>(groups + 0x4),
            rd::<u32>(groups + 0x8),
        ) -> ()
    );
    if cmd_pair[0] != 0 {
        unlock_recursive_srw(cmd_pair[0]);
    }
}

/// Original: `0x14206ef20` (239 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14206ef20(
    this: usize,
    _a2: usize,
    _a3: usize,
    start_index: u32,
    prim_count: u32,
    instance_count: u32,
) {
    if rd::<usize>(this + 0x48) == 0 {
        let sub_obj = rd::<usize>(this + 0xc48);
        if sub_obj == 0 {
            return;
        }
        let p_idx = rd::<usize>(this + 0xb18);
        let edx_val = rd::<u32>(p_idx).wrapping_add(2);
        let slot = exe::call_va!(0x1_411d_ce60, (sub_obj, edx_val, p_idx) -> usize);
        wr::<usize>(this + 0x48, slot);
        if slot == 0 {
            return;
        }
    }
    let index_count = prim_count.wrapping_mul(3);
    exe::call_va!(0x1_4206_f3a0, (this, 4u32) -> ());
    exe::call_va!(0x1_4206_f290, (this) -> ());
    if rd::<u8>(this + 0xa80) == 0 && exe::call_va!(0x1_4207_0fb0, (this, this) -> u8) == 0 {
        return;
    }
    let g_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    exe::call_va!(0x1_4207_1360, (this, rd::<usize>(g_dev + 0x20)) -> ());
    let mut cmd_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = cmd_pair[1];
    let draw_indexed = rd::<usize>(rd::<usize>(cmd_list) + 0x68);
    exe::call_va!(
        draw_indexed,
        (cmd_list, index_count, instance_count, start_index, 0u32, 0u32) -> ()
    );
    if cmd_pair[0] != 0 {
        unlock_recursive_srw(cmd_pair[0]);
    }
}

/// Original: `0x14206f020` (315 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14206f020(
    this: usize,
    topology: u32,
    base_vertex: u32,
    _a4: usize,
    _a5: usize,
    start_index: u32,
    prim_count: u32,
) {
    if rd::<usize>(this + 0x48) == 0 {
        let sub_obj = rd::<usize>(this + 0xc48);
        if sub_obj == 0 {
            return;
        }
        let p_idx = rd::<usize>(this + 0xb18);
        let edx_val = rd::<u32>(p_idx).wrapping_add(2);
        let slot = exe::call_va!(0x1_411d_ce60, (sub_obj, edx_val, p_idx) -> usize);
        wr::<usize>(this + 0x48, slot);
        if slot == 0 {
            return;
        }
    }
    let index_count = match topology {
        1 => prim_count,
        2 => prim_count.wrapping_mul(2),
        4 => prim_count.wrapping_mul(3),
        5 => prim_count.wrapping_add(2),
        _ => 0,
    };
    exe::call_va!(0x1_4206_f3a0, (this, topology) -> ());
    exe::call_va!(0x1_4206_f290, (this) -> ());
    if rd::<u8>(this + 0xa80) == 0 && exe::call_va!(0x1_4207_0fb0, (this, this) -> u8) == 0 {
        return;
    }
    let g_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    exe::call_va!(0x1_4207_1360, (this, rd::<usize>(g_dev + 0x20)) -> ());
    let mut cmd_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = cmd_pair[1];
    let draw_indexed = rd::<usize>(rd::<usize>(cmd_list) + 0x68);
    exe::call_va!(
        draw_indexed,
        (cmd_list, index_count, 1u32, start_index, base_vertex, 0u32) -> ()
    );
    if cmd_pair[0] != 0 {
        unlock_recursive_srw(cmd_pair[0]);
    }
}

/// Original: `0x14206f170` (267 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14206f170(
    this: usize,
    topology: u32,
    _a3: usize,
    prim_count: u32,
    instance_count: u32,
) {
    if rd::<usize>(this + 0x48) == 0 {
        let sub_obj = rd::<usize>(this + 0xc48);
        if sub_obj == 0 {
            return;
        }
        let p_idx = rd::<usize>(this + 0xb18);
        let edx_val = rd::<u32>(p_idx).wrapping_add(2);
        let slot = exe::call_va!(0x1_411d_ce60, (sub_obj, edx_val, p_idx) -> usize);
        wr::<usize>(this + 0x48, slot);
        if slot == 0 {
            return;
        }
    }
    let vertex_count = match topology {
        1 => prim_count,
        2 => prim_count.wrapping_mul(2),
        4 => prim_count.wrapping_mul(3),
        5 => prim_count.wrapping_add(2),
        _ => 0,
    };
    exe::call_va!(0x1_4206_f3a0, (this, topology) -> ());
    exe::call_va!(0x1_4206_f290, (this) -> ());
    if rd::<u8>(this + 0xa80) == 0 && exe::call_va!(0x1_4207_0fb0, (this, this) -> u8) == 0 {
        return;
    }
    let g_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    exe::call_va!(0x1_4207_1360, (this, rd::<usize>(g_dev + 0x20)) -> ());
    let mut cmd_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = cmd_pair[1];
    let draw_instanced = rd::<usize>(rd::<usize>(cmd_list) + 0x60);
    exe::call_va!(
        draw_instanced,
        (cmd_list, vertex_count, instance_count, 0u32, 0u32) -> ()
    );
    if cmd_pair[0] != 0 {
        unlock_recursive_srw(cmd_pair[0]);
    }
}

/// Original: `0x14206f680` (371 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_14206f680(
    this: usize,
    src: usize,
    handles: usize,
    flag: u8,
) {
    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    if rd::<u64>(src + 0x10) != 0 {
        let ctx = exe::call_va!(0x1_411d_cdf0, () -> usize);
        let b9c = rd::<u8>(rd::<usize>(g_addr) + 0x9c);
        exe::call_va!(0x1_4209_64f0, (ctx, b9c, flag) -> ());
        for off in (0x9e0..0xa60usize).step_by(8) {
            wr::<u64>(this + off, 0);
        }
    } else {
        let has_20 = u8::from(rd::<u64>(src + 0x20) != 0);
        let ctx = exe::call_va!(0x1_411d_cdf0, () -> usize);
        let b9c = rd::<u8>(rd::<usize>(g_addr) + 0x9c);
        exe::call_va!(0x1_4209_65a0, (ctx, b9c, flag, has_20) -> ());
    }
    for i in 0..6usize {
        let h = rd::<usize>(handles + i * 8);
        if h != 0 {
            exe::call_va!(0x1_4209_ba00, (h) -> ());
        }
    }
    let cmp = exe::call_va!(0x1_4183_acda, (this + 0xc48, src, 0x30u32) -> i32);
    if cmp != 0 {
        wr::<u8>(this + 0xa80, 0);
        if rd::<u64>(src) != rd::<u64>(this + 0xc48) {
            wr::<u64>(this + 0x48, 0);
        }
        wr::<[u8; 48]>(this + 0xc48, rd::<[u8; 48]>(src));
    }
}

/// Original: `0x142072cd0` (266 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142072cd0(
    this: usize,
    topology: u32,
    _a3: usize,
    prim_count: u32,
) {
    if rd::<usize>(this + 0x48) == 0 {
        let sub_obj = rd::<usize>(this + 0xc48);
        if sub_obj == 0 {
            return;
        }
        let p_idx = rd::<usize>(this + 0xb18);
        let edx_val = rd::<u32>(p_idx).wrapping_add(2);
        let slot = exe::call_va!(0x1_411d_ce60, (sub_obj, edx_val, p_idx) -> usize);
        wr::<usize>(this + 0x48, slot);
        if slot == 0 {
            return;
        }
    }
    let vertex_count = match topology {
        1 => prim_count,
        2 => prim_count.wrapping_mul(2),
        4 => prim_count.wrapping_mul(3),
        5 => prim_count.wrapping_add(2),
        _ => 0,
    };
    exe::call_va!(0x1_4206_f3a0, (this, topology) -> ());
    exe::call_va!(0x1_4206_f290, (this) -> ());
    if rd::<u8>(this + 0xa80) == 0 && exe::call_va!(0x1_4207_0fb0, (this, this) -> u8) == 0 {
        return;
    }
    let g_dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc58) as usize);
    exe::call_va!(0x1_4207_1360, (this, rd::<usize>(g_dev + 0x20)) -> ());
    let mut cmd_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
    let cmd_list = cmd_pair[1];
    let draw_instanced = rd::<usize>(rd::<usize>(cmd_list) + 0x60);
    exe::call_va!(
        draw_instanced,
        (cmd_list, vertex_count, 1u32, 0u32, 0u32) -> ()
    );
    if cmd_pair[0] != 0 {
        unlock_recursive_srw(cmd_pair[0]);
    }
}

/// Original: `0x1420734e0` (332 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420734e0(out: usize, flag: i32) -> usize {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let lock = if rd::<usize>(tls0 + 0x6300) == 0 {
        exe::ptr::<u8>(0x1_4385_7f30) as usize
    } else {
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        rd::<usize>(tls0 + 0x6318)
    };
    if lock != 0 {
        lock_recursive_srw(lock);
    }
    if rd::<u8>(tls0 + 0x70) == 0 {
        exe::call_va!(0x1_4183_49c0, () -> ());
    }
    let ctx = if rd::<usize>(tls0 + 0x6300) != 0 {
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        rd::<usize>(tls0 + 0x6300)
    } else {
        rd::<usize>(exe::ptr::<usize>(0x1_4397_fc68) as usize)
    };
    wr::<[u8; 16]>(out, rd::<[u8; 16]>(ctx + 0x8));
    if flag == 0 {
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        exe::call_va!(0x1_4206_df80, (rd::<usize>(tls0 + 0x6328)) -> ());
        if rd::<u8>(tls0 + 0x70) == 0 {
            exe::call_va!(0x1_4183_49c0, () -> ());
        }
        exe::call_va!(0x1_4207_6db0, (rd::<usize>(tls0 + 0x6328)) -> ());
        exe::call_va!(0x1_4206_e7f0, () -> ());
    }
    if lock != 0 {
        unlock_recursive_srw(lock);
    }
    out
}

/// Original: `0x1420738b0` (391 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420738b0(mut items: usize, mut count: u64) {
    if count == 0 {
        return;
    }
    let mut barriers = [[0u64; 4]; 64];
    while count != 0 {
        let count_i32 = count as i32;
        let batch = if count_i32 < 0x40 { count_i32 } else { 0x40 };
        let batch_isize = batch as isize;
        if batch > 0 {
            for i in 0..(batch as usize) {
                let item = rd::<usize>(items + i * 8);
                let flags = rd::<u32>(item + 0x4c);
                let mut st = (flags >> 1) & 1;
                if ((flags >> 2) & 1) != 0 {
                    st |= 0x2;
                }
                if ((flags >> 5) & 1) != 0 {
                    st |= 0x40;
                }
                let res = rd::<u64>(item + 0x40);
                if (flags & 1) != 0 {
                    st |= 0xc0;
                }
                barriers[i] = [0u64, res, 0x0000_0008_ffff_ffffu64, st as u64];
                let item2 = rd::<usize>(items + i * 8);
                let flags2 = rd::<u32>(item2 + 0x4c);
                let mut st2 = (flags2 >> 1) & 1;
                if ((flags2 >> 2) & 1) != 0 {
                    st2 |= 0x2;
                }
                if ((flags2 >> 5) & 1) != 0 {
                    st2 |= 0x40;
                }
                if (flags2 & 1) != 0 {
                    st2 |= 0xc0;
                }
                wr::<u32>(item2 + 0x48, st2);
            }
        }
        let mut cmd_pair = [0usize; 2];
        exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
        let cmd_list = cmd_pair[1];
        let rb_fn = rd::<usize>(rd::<usize>(cmd_list) + 0xd0);
        exe::call_va!(rb_fn, (cmd_list, batch as u32, barriers.as_ptr() as usize) -> ());
        if cmd_pair[0] != 0 {
            unlock_recursive_srw(cmd_pair[0]);
        }
        count = count.wrapping_sub(batch_isize as u64);
        items = (items as isize).wrapping_add(batch_isize * 8) as usize;
    }
}

/// Original: `0x142074340` (332 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_142074340(mut items: usize, mut count: u64) {
    if count == 0 {
        return;
    }
    let mut barriers = [[0u64; 4]; 64];
    while count != 0 {
        let count_i32 = count as i32;
        let batch = if count_i32 < 0x40 { count_i32 } else { 0x40 };
        let batch_isize = batch as isize;
        if batch > 0 {
            for i in 0..(batch as usize) {
                let item = rd::<usize>(items + i * 8);
                let flags = rd::<u32>(item + 0x4c);
                let mut st = (flags >> 1) & 1;
                if ((flags >> 2) & 1) != 0 {
                    st |= 0x2;
                }
                if ((flags >> 5) & 1) != 0 {
                    st |= 0x40;
                }
                let res = rd::<u64>(item + 0x40);
                if (flags & 1) != 0 {
                    st |= 0xc0;
                }
                barriers[i] = [0u64, res, 0xffff_ffffu64 | ((st as u64) << 32), 0x8u64];
                let item2 = rd::<usize>(items + i * 8);
                wr::<u32>(item2 + 0x48, 0x8);
            }
        }
        let mut cmd_pair = [0usize; 2];
        exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> ());
        let cmd_list = cmd_pair[1];
        let rb_fn = rd::<usize>(rd::<usize>(cmd_list) + 0xd0);
        exe::call_va!(rb_fn, (cmd_list, batch as u32, barriers.as_ptr() as usize) -> ());
        if cmd_pair[0] != 0 {
            unlock_recursive_srw(cmd_pair[0]);
        }
        count = count.wrapping_sub(batch_isize as u64);
        items = (items as isize).wrapping_add(batch_isize * 8) as usize;
    }
}

/// Original: `0x142074b80` (322 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142074b80(
    dst_res: usize,
    mut dst_off: u64,
    mut src_ptr: usize,
    mut size: u64,
) {
    if size == 0 {
        return;
    }
    let lock = exe::ptr::<u8>(0x1_4385_7ee0) as usize;
    while size != 0 {
        let chunk = if size > 0x10_0000 { 0x10_0000 } else { size };
        let mut tmp = [0u8; 96];
        let res_ptr = exe::call_va!(
            0x1_4205_f290,
            (tmp.as_mut_ptr() as usize, chunk, (-2i64) as u64) -> usize
        );
        let mut staging = rd::<[u8; 96]>(res_ptr);
        let staging_base = staging.as_mut_ptr() as usize;
        let mapped_dst = rd::<usize>(staging_base + 0x50);
        exe::call_va!(0x1_4183_acbc, (mapped_dst, src_ptr, chunk) -> usize);
        lock_recursive_srw(lock);
        let entry = exe::call_va!(0x1_411d_a370, (0usize, (-2i64) as u64) -> usize);
        wr::<u64>(entry + 0x28, rd::<u64>(entry + 0x28).wrapping_sub(1));
        unlock_recursive_srw(lock);
        let off_58 = rd::<u32>(staging_base + 0x58);
        exe::call_va!(
            0x1_4207_5bc0,
            (dst_res, dst_off, staging_base, off_58, chunk) -> ()
        );
        src_ptr = src_ptr.wrapping_add(chunk as usize);
        dst_off = dst_off.wrapping_add(chunk);
        size = size.wrapping_sub(chunk);
    }
}

/// Original: `0x142075f60` (270 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_142075f60(this: usize) {
    let mut desc = [0u8; 0x28];
    let d_base = desc.as_mut_ptr() as usize;
    wr::<u32>(d_base, 0x27);
    wr::<u32>(d_base + 0x4, 1);
    wr::<u64>(d_base + 0x8, 0x1688);
    wr::<u64>(d_base + 0x10, 0);
    wr::<u64>(d_base + 0x18, 0x4_0000);
    wr::<u64>(d_base + 0x20, 1);

    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_4397_fcb8) as usize);
    let lock = mgr + 0x30;
    lock_recursive_srw(lock);
    let ring = mgr + 0xc0;
    if rd::<u32>(ring + 0x20) == 0 {
        exe::call_va!(0x1_4209_3aa0, (ring, rd::<u32>(ring + 0x44)) -> ());
    }
    let r_buf = rd::<usize>(ring + 0x10);
    let r_cap = rd::<u32>(ring + 0x24) as u64;
    let r_head = rd::<u32>(ring + 0x18);
    let idx = ((r_head as u64) % r_cap) as usize;
    let handle = rd::<u64>(r_buf + idx * 8);
    let r_len = rd::<u32>(ring + 0x20);
    if r_len != 0 {
        let next_head = r_head.wrapping_add(1);
        wr::<u32>(
            ring + 0x18,
            if next_head == (r_cap as u32) {
                0
            } else {
                next_head
            },
        );
        wr::<u32>(ring + 0x20, r_len.wrapping_sub(1));
    }
    unlock_recursive_srw(lock);

    let dev = rd::<usize>(exe::ptr::<usize>(0x1_4397_fc80) as usize);
    let create_srv = rd::<usize>(rd::<usize>(dev) + 0x90);
    exe::call_va!(create_srv, (dev, rd::<usize>(this), d_base, handle) -> ());
    wr::<u64>(this + 0x18, handle);
}

/// Original: `0x1420761a0` (332 bytes, `Throws::No`).
pub unsafe extern "win64" fn d3d_1420761a0(this: usize) {
    exe::call_va!(0x1_4183_acc2, (this + 0xb30, 0u32, 0x100usize) -> usize);
    exe::call_va!(0x1_4183_acc2, (this + 0x50, 0u32, 0x990usize) -> usize);
    for off in (0x9e0..0xa60usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    wr::<u64>(this + 0xb18, 0);
    wr::<u64>(this + 0xb20, 0);
    wr::<u64>(this + 0xc30, 0);
    wr::<u64>(this + 0xc38, 0);
    for off in (0xc48..0xc78usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    wr::<[u8; 16]>(this + 0xa84, [0u8; 16]);
    wr::<u32>(this + 0xc40, 0);
    wr::<u64>(this + 0x48, 0);
    wr::<u64>(this + 0xa60, 0);
    wr::<u64>(this + 0xa68, 0);
    wr::<u64>(this + 0xa70, 0);
    wr::<u64>(this + 0xa78, 0);
    wr::<u64>(this + 0xa94, 0);
    wr::<[u8; 16]>(this + 0xa9c, [0u8; 16]);
    for off in (0..0x40usize).step_by(8) {
        wr::<u64>(this + off, 0);
    }
    wr::<u8>(this + 0xa80, 0);
    if rd::<i32>(this + 0xb28) > 0 {
        let mut i: i32 = 0;
        let mut p = this + 0xb30;
        while i < rd::<i32>(this + 0xb28) {
            wr::<u64>(p, 0);
            wr::<u64>(p + 0x8, 0);
            i += 1;
            p += 0x10;
        }
    }
    wr::<u32>(this + 0xb28, 0);
    wr::<u8>(this + 0xb2c, 1);
    wr::<u64>(this + 0xb10, 0);
    wr::<u64>(this + 0xb00, 0);
    wr::<u64>(this + 0x40, 0);
    wr::<u64>(this + 0xaf8, 0);
    wr::<u64>(this + 0xaf0, 0);
}

/// Original: `0x1420768e0` (56 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn d3d_1420768e0(this: usize) {
    let lock = this + 0x38;
    exe::call_iat!(0x1_4264_55a8, (lock) -> ());
    let _ = crate::simd_ops::atomic_xchg_u8((this + 0x40) as u64, 0); // xchg byte [rbx+0x40]
    exe::call_va!(0x1_4207_6960, (this) -> ());
    exe::call_iat!(0x1_4264_5598, (lock) -> ());
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_d3d_14204f6c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let arr = [0x1111usize, 0x2222usize, 0x3333usize];
    let mut obj = [0u8; 0x50];
    let base = obj.as_mut_ptr() as usize;
    wr::<usize>(base, 0x9999_8888);
    wr::<usize>(base + 0x20, arr.as_ptr() as usize);
    wr::<u32>(base + 0x28, 3);

    for idx in [0i32, 1, 2] {
        let ro = f_orig(base, 0, idx);
        let rn = f_new(base, 0, idx);
        crate::diff_eq!(ro, rn, "d3d_14204f6c0 cached lookup");
    }
    Ok(3)
}

pub unsafe fn diff_d3d_142050470(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut node_o = [0u64; 3];
    let mut node_n = [0u64; 3];
    let mut blk_o = [0u8; 0x70];
    let mut blk_n = [0u8; 0x70];
    let mut this_o = [0u8; 0x150];
    let mut this_n = [0u8; 0x150];

    for (this, node, blk) in [
        (&mut this_o, &mut node_o, &mut blk_o),
        (&mut this_n, &mut node_n, &mut blk_n),
    ] {
        let tb = this.as_mut_ptr() as usize;
        let nb = node.as_mut_ptr() as usize;
        let bb = blk.as_mut_ptr() as usize;
        wr::<usize>(tb + 0xb0 + 0x18, nb);
        wr::<u64>(bb + 0x50, 2);
        wr::<u64>(bb + 0x58, 0x400);
        wr::<u64>(bb + 0x60, 0x10000);
        wr::<usize>(tb + 0x138, bb);
    }

    let ro = f_orig(this_o.as_mut_ptr() as usize, 0x800);
    let rn = f_new(this_n.as_mut_ptr() as usize, 0x800);
    crate::diff_eq!(ro, node_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, node_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(node_o[1..3], node_n[1..3], "node offset and size");
    crate::diff_eq!(&blk_o[0x50..0x68], &blk_n[0x50..0x68], "blk state");
    Ok(1)
}

pub unsafe fn diff_d3d_142050690(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u64) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut this_o = [0u8; 0x80];
    let mut this_n = [0u8; 0x80];
    let mut out_o = [0u64; 3];
    let mut out_n = [0u64; 3];

    for sz in [0x4_0000u64, 0x8_0000u64] {
        let ro = f_orig(
            this_o.as_mut_ptr() as usize,
            out_o.as_mut_ptr() as usize,
            sz,
        );
        let rn = f_new(
            this_n.as_mut_ptr() as usize,
            out_n.as_mut_ptr() as usize,
            sz,
        );
        crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret orig");
        crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret new");
        crate::diff_eq!(out_o, out_n, "large bucket empty freelist");
    }
    Ok(2)
}

pub unsafe fn diff_d3d_142050ef0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    // Case 1: [this + 0x50] == 0
    let mut t0_o = [0u8; 0x60];
    let mut t0_n = [0u8; 0x60];
    wr::<u64>(t0_o.as_mut_ptr() as usize + 0x40, 0x1234);
    wr::<u64>(t0_n.as_mut_ptr() as usize + 0x40, 0x1234);
    f_orig(t0_o.as_mut_ptr() as usize);
    f_new(t0_n.as_mut_ptr() as usize);
    crate::diff_eq!(t0_o, t0_n, "null ptr_50 early exit");

    // Case 2: [this + 0x50] != 0 with large size (bucket >= 0x27)
    let g_addr = exe::ptr::<usize>(0x1_4397_fb88) as usize;
    let saved_g = rd::<usize>(g_addr);
    let mut fake_g = [0u8; 0x80];
    wr::<usize>(g_addr, fake_g.as_mut_ptr() as usize);

    let mut blk_o = [0u8; 0xf0];
    let mut blk_n = [0u8; 0xf0];
    wr::<u64>(blk_o.as_mut_ptr() as usize + 0x10, 0xaaaa_bbbb);
    wr::<u64>(blk_n.as_mut_ptr() as usize + 0x10, 0xaaaa_bbbb);
    let mut t1_o = [0u8; 0x60];
    let mut t1_n = [0u8; 0x60];
    for (t, blk) in [(&mut t1_o, &mut blk_o), (&mut t1_n, &mut blk_n)] {
        let tb = t.as_mut_ptr() as usize;
        wr::<u64>(tb + 0x40, 0x100);
        wr::<u64>(tb + 0x48, 0x4_0000);
        wr::<usize>(tb + 0x50, blk.as_mut_ptr() as usize);
    }
    f_orig(t1_o.as_mut_ptr() as usize);
    f_new(t1_n.as_mut_ptr() as usize);
    wr::<usize>(g_addr, saved_g);
    crate::diff_eq!(t1_o, t1_n, "cleared this");
    crate::diff_eq!(blk_o, blk_n, "cleared block");
    Ok(2)
}

unsafe extern "win64" fn mock_create_heap(
    dev: usize,
    desc: usize,
    _riid: usize,
    out_ptr: usize,
    _a5: u32,
    _a6: usize,
) -> u32 {
    wr::<[u8; 0x30]>(dev + 0x10, rd::<[u8; 0x30]>(desc));
    // Zero 4-byte struct padding holes in D3D12_HEAP_DESC (+0x1c and +0x2c) left uninitialized on stack by MSVC
    wr::<u32>(dev + 0x10 + 0x1c, 0);
    wr::<u32>(dev + 0x10 + 0x2c, 0);
    wr::<usize>(out_ptr, 0x7777_8888);
    0
}

pub unsafe fn diff_d3d_142051a20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 32];
    vtbl[0xe0 / 8] = mock_create_heap as *const () as usize;
    let mut dev_o = [0u8; 0x50];
    let mut dev_n = [0u8; 0x50];
    wr::<usize>(dev_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<usize>(dev_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

    let mut node_o = [0u64; 3];
    let mut node_n = [0u64; 3];
    let mut this_o = [0u8; 0x110];
    let mut this_n = [0u8; 0x110];
    for (t, node) in [(&mut this_o, &mut node_o), (&mut this_n, &mut node_n)] {
        let tb = t.as_mut_ptr() as usize;
        wr::<usize>(tb + 0x38 + 0x18, node.as_mut_ptr() as usize);
        wr::<u64>(tb + 0xc0, 0x10_0000);
        wr::<u32>(tb + 0xe8, u32::MAX);
        wr::<u32>(tb + 0x100, 3);
    }

    let dev_addr = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_dev = rd::<usize>(dev_addr);
    wr::<usize>(dev_addr, dev_o.as_mut_ptr() as usize);
    let ro = f_orig(this_o.as_mut_ptr() as usize);
    wr::<usize>(dev_addr, dev_n.as_mut_ptr() as usize);
    let rn = f_new(this_n.as_mut_ptr() as usize);
    wr::<usize>(dev_addr, saved_dev);

    crate::diff_eq!(ro, node_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, node_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(node_o, node_n, "node state");
    crate::diff_eq!(&dev_o[0x10..0x40], &dev_n[0x10..0x40], "heap desc");
    Ok(1)
}

pub unsafe fn diff_d3d_142051ba0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut node_o = [0u64; 3];
    let mut node_n = [0u64; 3];
    let mut blk0_o = [0u32; 8];
    let mut blk1_o = [0u32; 8];
    let mut blk0_n = [0u32; 8];
    let mut blk1_n = [0u32; 8];
    blk0_o[4] = 0x80;
    blk1_o[4] = 0x1000;
    blk1_o[5] = 2;
    blk0_n[4] = 0x80;
    blk1_n[4] = 0x1000;
    blk1_n[5] = 2;

    let arr_o = [blk0_o.as_mut_ptr() as usize, blk1_o.as_mut_ptr() as usize];
    let arr_n = [blk0_n.as_mut_ptr() as usize, blk1_n.as_mut_ptr() as usize];
    let mut mgr_o = [0u8; 0x110];
    let mut mgr_n = [0u8; 0x110];
    for (m, arr, node) in [
        (&mut mgr_o, &arr_o, &mut node_o),
        (&mut mgr_n, &arr_n, &mut node_n),
    ] {
        let mb = m.as_mut_ptr() as usize;
        wr::<usize>(mb + 0x10, arr.as_ptr() as usize);
        wr::<u32>(mb + 0x18, 2);
        wr::<usize>(mb + 0x78 + 0x18, node.as_mut_ptr() as usize);
        wr::<u32>(mb + 0xc0, 0x4000);
        wr::<u64>(mb + 0xc8, 0x100);
        wr::<u32>(mb + 0xd8, u32::MAX);
    }

    let g_addr = exe::ptr::<usize>(0x1_4397_fb90) as usize;
    let saved_g = rd::<usize>(g_addr);
    wr::<usize>(g_addr, mgr_o.as_mut_ptr() as usize);
    let ro = f_orig(0, 0x120);
    wr::<usize>(g_addr, mgr_n.as_mut_ptr() as usize);
    let rn = f_new(0, 0x120);
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(ro, node_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, node_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(node_o[1], node_n[1], "node offset and aligned size");
    crate::diff_eq!(blk1_o, blk1_n, "selected block updated");
    crate::diff_eq!(
        rd::<u64>(mgr_o.as_ptr() as usize + 0xf8),
        rd::<u64>(mgr_n.as_ptr() as usize + 0xf8),
        "mgr total allocated"
    );
    Ok(1)
}

pub unsafe fn diff_d3d_142052100(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut item_o = [0u8; 0x28];
    let mut item_n = [0u8; 0x28];
    wr::<u16>(item_o.as_mut_ptr() as usize + 0x20, 1);
    wr::<u16>(item_n.as_mut_ptr() as usize + 0x20, 1);

    let mut active_o = [item_o.as_mut_ptr() as usize, 0x5555usize];
    let mut active_n = [item_n.as_mut_ptr() as usize, 0x5555usize];
    let mut ring_o = [0usize; 4];
    let mut ring_n = [0usize; 4];
    let mut this_o = [0u8; 0xe0];
    let mut this_n = [0u8; 0xe0];
    for (t, act, ring) in [
        (&mut this_o, &mut active_o, &mut ring_o),
        (&mut this_n, &mut active_n, &mut ring_n),
    ] {
        let tb = t.as_mut_ptr() as usize;
        wr::<usize>(tb, act.as_mut_ptr() as usize);
        wr::<u32>(tb + 0x8, 2);
        wr::<usize>(tb + 0x10, ring.as_mut_ptr() as usize);
        wr::<u32>(tb + 0x1c, 3);
        wr::<u32>(tb + 0x20, 1);
        wr::<u32>(tb + 0x24, 4);
        wr::<u64>(tb + 0xa8, 0x1000);
        wr::<u32>(tb + 0xc8, u32::MAX);
        wr::<u64>(tb + 0xd0, 0x3000);
    }

    f_orig(this_o.as_mut_ptr() as usize, item_o.as_mut_ptr() as usize);
    f_new(this_n.as_mut_ptr() as usize, item_n.as_mut_ptr() as usize);
    crate::diff_eq!(active_o[0], active_n[0], "swap removed active[0]");
    crate::diff_eq!(item_o, item_n, "item cleared");
    crate::diff_eq!(&this_o[0x8..0x10], &this_n[0x8..0x10], "vec counts");
    crate::diff_eq!(&this_o[0x18..0x28], &this_n[0x18..0x28], "ring state");
    crate::diff_eq!(
        rd::<u64>(this_o.as_ptr() as usize + 0xd0),
        rd::<u64>(this_n.as_ptr() as usize + 0xd0),
        "total bytes"
    );
    Ok(1)
}

pub unsafe fn diff_d3d_142052270(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut item_o = [0u8; 0x28];
    let mut item_n = [0u8; 0x28];
    let mut active_o = [0usize; 4];
    let mut active_n = [0usize; 4];
    let ring_o = [0usize, item_o.as_mut_ptr() as usize];
    let ring_n = [0usize, item_n.as_mut_ptr() as usize];
    let mut this_o = [0u8; 0xe0];
    let mut this_n = [0u8; 0xe0];
    for (t, act, ring) in [
        (&mut this_o, &mut active_o, &ring_o),
        (&mut this_n, &mut active_n, &ring_n),
    ] {
        let tb = t.as_mut_ptr() as usize;
        wr::<usize>(tb, act.as_mut_ptr() as usize);
        wr::<u32>(tb + 0x8, 1);
        wr::<u32>(tb + 0xc, 4);
        wr::<usize>(tb + 0x10, ring.as_ptr() as usize);
        wr::<u32>(tb + 0x18, 1);
        wr::<u32>(tb + 0x20, 1);
        wr::<u32>(tb + 0x24, 2);
        wr::<u64>(tb + 0xa8, 0x2000);
        wr::<u32>(tb + 0xc8, u32::MAX);
        wr::<u64>(tb + 0xd0, 0x1000);
    }

    let ro = f_orig(this_o.as_mut_ptr() as usize);
    let rn = f_new(this_n.as_mut_ptr() as usize);
    crate::diff_eq!(ro, item_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, item_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(item_o, item_n, "item fields");
    crate::diff_eq!(&this_o[0x8..0x10], &this_n[0x8..0x10], "vec counts");
    crate::diff_eq!(&this_o[0x18..0x28], &this_n[0x18..0x28], "ring counts");
    crate::diff_eq!(
        rd::<u64>(this_o.as_ptr() as usize + 0xd0),
        rd::<u64>(this_n.as_ptr() as usize + 0xd0),
        "total bytes"
    );
    Ok(1)
}

pub unsafe fn diff_d3d_1420523d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 32];
    vtbl[0xe0 / 8] = mock_create_heap as *const () as usize;
    let dev_addr = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_dev = rd::<usize>(dev_addr);

    for kind in [1u32, 2u32] {
        let mut dev_o = [0u8; 0x50];
        let mut dev_n = [0u8; 0x50];
        wr::<usize>(dev_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<usize>(dev_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

        let mut node_o = [0u64; 5];
        let mut node_n = [0u64; 5];
        let mut this_o = [0u8; 0xf0];
        let mut this_n = [0u8; 0xf0];
        for (t, node) in [(&mut this_o, &mut node_o), (&mut this_n, &mut node_n)] {
            let tb = t.as_mut_ptr() as usize;
            wr::<usize>(tb + 0x28 + 0x18, node.as_mut_ptr() as usize);
            wr::<u64>(tb + 0xa8, 0x40_0000);
            wr::<u32>(tb + 0xe0, kind);
        }

        wr::<usize>(dev_addr, dev_o.as_mut_ptr() as usize);
        let ro = f_orig(this_o.as_mut_ptr() as usize);
        wr::<usize>(dev_addr, dev_n.as_mut_ptr() as usize);
        let rn = f_new(this_n.as_mut_ptr() as usize);
        crate::diff_eq!(ro, node_o.as_mut_ptr() as usize, "ret orig");
        crate::diff_eq!(rn, node_n.as_mut_ptr() as usize, "ret new");
        crate::diff_eq!(node_o, node_n, "node state");
        crate::diff_eq!(&dev_o[0x10..0x40], &dev_n[0x10..0x40], "heap desc");
    }
    wr::<usize>(dev_addr, saved_dev);
    Ok(2)
}

pub unsafe fn diff_d3d_1420527c0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut this_o = [0u8; 0xe0];
    let mut this_n = [0u8; 0xe0];
    for t in [&mut this_o, &mut this_n] {
        let tb = t.as_mut_ptr() as usize;
        wr::<u64>(tb + 0x8, 0x1111_2222);
        wr::<u64>(tb + 0x18, 0x3333_4444);
        wr::<u32>(tb + 0x24, 8);
        wr::<u32>(tb + 0xc8, u32::MAX);
    }
    f_orig(this_o.as_mut_ptr() as usize);
    f_new(this_n.as_mut_ptr() as usize);
    crate::diff_eq!(this_o, this_n, "destroyed state");
    Ok(1)
}

pub unsafe fn diff_d3d_142056040(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_addr = exe::ptr::<usize>(0x1_4397_fc08) as usize;
    let saved_g = rd::<usize>(g_addr);
    let mut this_o = [0u8; 0xa8];
    let mut this_n = [0u8; 0xa8];
    for t in [&mut this_o, &mut this_n] {
        let tb = t.as_mut_ptr() as usize;
        wr::<u32>(tb + 0x14, 4);
        wr::<u32>(tb + 0x24, 4);
        wr::<u64>(tb + 0x30, 0x1234);
        wr::<u32>(tb + 0x4c, 4);
        wr::<u32>(tb + 0x5c, 4);
        wr::<u32>(tb + 0x6c, 4);
        wr::<u32>(tb + 0x7c, 4);
        wr::<u32>(tb + 0x8c, 4);
        wr::<u32>(tb + 0x9c, 4);
    }

    wr::<usize>(g_addr, 0xdead_beef);
    f_orig(this_o.as_mut_ptr() as usize);
    let go = rd::<usize>(g_addr);
    wr::<usize>(g_addr, 0xdead_beef);
    f_new(this_n.as_mut_ptr() as usize);
    let gn = rd::<usize>(g_addr);
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(go, gn, "global cleared");
    crate::diff_eq!(this_o, this_n, "containers cleared");
    Ok(1)
}

pub unsafe fn diff_d3d_142057130(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    // Case 1: this[1] == u64::MAX
    let mut obj0_o = [0usize, usize::MAX];
    let mut obj0_n = [0usize, usize::MAX];
    f_orig(obj0_o.as_mut_ptr() as usize);
    f_new(obj0_n.as_mut_ptr() as usize);
    crate::diff_eq!(obj0_o, obj0_n, "vftable set when inactive");

    // Case 2: this[1] != u64::MAX, remove from global vector 0x143a63238
    let vec_addr = exe::ptr::<u8>(0x1_43a6_3238) as usize;
    let saved_vec = rd::<[u8; 16]>(vec_addr);
    let mut obj1_o = [0usize, 1usize];
    let mut obj1_n = [0usize, 1usize];
    let mut arr_o = [0x1111usize, obj1_o.as_mut_ptr() as usize, 0x2222usize];
    let mut arr_n = [0x1111usize, obj1_n.as_mut_ptr() as usize, 0x2222usize];

    wr::<usize>(vec_addr, arr_o.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 3);
    f_orig(obj1_o.as_mut_ptr() as usize);
    let len_o = rd::<u32>(vec_addr + 0x8);

    wr::<usize>(vec_addr, arr_n.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 3);
    f_new(obj1_n.as_mut_ptr() as usize);
    let len_n = rd::<u32>(vec_addr + 0x8);
    wr::<[u8; 16]>(vec_addr, saved_vec);

    crate::diff_eq!(obj1_o, obj1_n, "vftable set when active");
    crate::diff_eq!(len_o, len_n, "vec len decremented");
    crate::diff_eq!(arr_o[0..2], arr_n[0..2], "vec compacted");
    Ok(2)
}

pub unsafe fn diff_d3d_142057410(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut obj0_o = [0usize, usize::MAX];
    let mut obj0_n = [0usize, usize::MAX];
    f_orig(obj0_o.as_mut_ptr() as usize);
    f_new(obj0_n.as_mut_ptr() as usize);
    crate::diff_eq!(obj0_o, obj0_n, "vftable set when inactive");

    let vec_addr = exe::ptr::<u8>(0x1_43a6_3228) as usize;
    let saved_vec = rd::<[u8; 16]>(vec_addr);
    let mut obj1_o = [0usize, 1usize];
    let mut obj1_n = [0usize, 1usize];
    let mut arr_o = [0x3333usize, obj1_o.as_mut_ptr() as usize, 0x4444usize];
    let mut arr_n = [0x3333usize, obj1_n.as_mut_ptr() as usize, 0x4444usize];

    wr::<usize>(vec_addr, arr_o.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 3);
    f_orig(obj1_o.as_mut_ptr() as usize);
    let len_o = rd::<u32>(vec_addr + 0x8);

    wr::<usize>(vec_addr, arr_n.as_mut_ptr() as usize);
    wr::<u32>(vec_addr + 0x8, 3);
    f_new(obj1_n.as_mut_ptr() as usize);
    let len_n = rd::<u32>(vec_addr + 0x8);
    wr::<[u8; 16]>(vec_addr, saved_vec);

    crate::diff_eq!(obj1_o, obj1_n, "vftable set when active");
    crate::diff_eq!(len_o, len_n, "vec len decremented");
    crate::diff_eq!(arr_o[0..2], arr_n[0..2], "vec compacted");
    Ok(2)
}

pub unsafe fn diff_d3d_142058010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(
        usize,
        usize,
        u32,
        u8,
        u8,
        u8,
        u32,
        u32,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
        usize,
    );
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_base = exe::ptr::<u8>(0x1_4385_7a50) as usize;
    let saved = rd::<[u8; 0x194]>(g_base);
    let m0 = [0x11u8; 64];
    let m1 = [0x22u8; 64];
    let m2 = [0x33u8; 64];
    let m3 = [0x44u8; 64];

    for a6 in [0u8, 1u8] {
        wr::<[u8; 0x194]>(g_base, [0u8; 0x194]);
        f_orig(
            0,
            0,
            10,
            1,
            2,
            a6,
            100,
            200,
            300,
            400,
            500,
            600,
            m0.as_ptr() as usize,
            m1.as_ptr() as usize,
            m2.as_ptr() as usize,
            m3.as_ptr() as usize,
        );
        let snap_o = rd::<[u8; 0x194]>(g_base);

        wr::<[u8; 0x194]>(g_base, [0u8; 0x194]);
        f_new(
            0,
            0,
            10,
            1,
            2,
            a6,
            100,
            200,
            300,
            400,
            500,
            600,
            m0.as_ptr() as usize,
            m1.as_ptr() as usize,
            m2.as_ptr() as usize,
            m3.as_ptr() as usize,
        );
        let snap_n = rd::<[u8; 0x194]>(g_base);
        crate::diff_eq!(snap_o, snap_n, "globals 0x143857a50..0x143857be4");
    }
    wr::<[u8; 0x194]>(g_base, saved);
    Ok(2)
}

pub unsafe fn diff_d3d_14206d170(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(i32, i32, u64, usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g = rd::<usize>(g_addr);

    let mut w0_o = [0u16; 32];
    let mut w1_o = [0u16; 32];
    let mut w2_o = [0u16; 32];
    let mut w0_n = [0u16; 32];
    let mut w1_n = [0u16; 32];
    let mut w2_n = [0u16; 32];
    let mut state_o = vec![0u8; 0x3400];
    let mut state_n = vec![0u8; 0x3400];
    for (st, w0, w1, w2) in [
        (&mut state_o, &mut w0_o, &mut w1_o, &mut w2_o),
        (&mut state_n, &mut w0_n, &mut w1_n, &mut w2_n),
    ] {
        let b = st.as_mut_ptr() as usize + 0x120;
        wr::<usize>(b + 0x1350, w0.as_mut_ptr() as usize);
        wr::<u32>(b + 0x135c, 32);
        wr::<u32>(b + 0x1360, 4);
        wr::<usize>(b + 0x20d0, w1.as_mut_ptr() as usize);
        wr::<u32>(b + 0x20dc, 32);
        wr::<usize>(b + 0x2e50, w2.as_mut_ptr() as usize);
        wr::<u32>(b + 0x2e5c, 32);
    }

    let s1 = b"Alpha\0";
    let s2 = b"Beta\0";
    wr::<usize>(g_addr, state_o.as_mut_ptr() as usize);
    f_orig(1, 0, 0, s1.as_ptr() as usize, s2.as_ptr() as usize);
    wr::<usize>(g_addr, state_n.as_mut_ptr() as usize);
    f_new(1, 0, 0, s1.as_ptr() as usize, s2.as_ptr() as usize);
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(w0_o, w0_n, "w0 wide str");
    crate::diff_eq!(w1_o, w1_n, "w1 wide str");
    crate::diff_eq!(w2_o, w2_n, "w2 wide str");
    let bo = state_o.as_ptr() as usize + 0x120;
    let bn = state_n.as_ptr() as usize + 0x120;
    crate::diff_eq!(rd::<u32>(bo + 0x1360), rd::<u32>(bn + 0x1360), "len0");
    crate::diff_eq!(rd::<u32>(bo + 0x20e0), rd::<u32>(bn + 0x20e0), "len1");
    crate::diff_eq!(rd::<u32>(bo + 0x2e60), rd::<u32>(bn + 0x2e60), "len2");
    Ok(1)
}

unsafe extern "win64" fn mock_create_srv(dev: usize, res: usize, desc: usize, handle: u64) {
    wr::<usize>(dev + 0x8, res);
    wr::<u64>(dev + 0x10, handle);
    wr::<[u8; 0x28]>(dev + 0x18, rd::<[u8; 0x28]>(desc));
}

pub unsafe fn diff_d3d_142075f60(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 24];
    vtbl[0x90 / 8] = mock_create_srv as *const () as usize;
    let mut dev_o = [0u8; 0x50];
    let mut dev_n = [0u8; 0x50];
    wr::<usize>(dev_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<usize>(dev_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

    let handles = [0x1111u64, 0xcafe_babe_dead_beefu64];
    let mut mgr_o = [0u8; 0x100];
    let mut mgr_n = [0u8; 0x100];
    for m in [&mut mgr_o, &mut mgr_n] {
        let mb = m.as_mut_ptr() as usize;
        wr::<u32>(mb + 0x38, u32::MAX);
        wr::<usize>(mb + 0xd0, handles.as_ptr() as usize);
        wr::<u32>(mb + 0xd8, 1);
        wr::<u32>(mb + 0xe0, 1);
        wr::<u32>(mb + 0xe4, 2);
    }
    let mut this_o = [0x4242usize, 0, 0, 0];
    let mut this_n = [0x4242usize, 0, 0, 0];

    let mgr_addr = exe::ptr::<usize>(0x1_4397_fcb8) as usize;
    let dev_addr = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_mgr = rd::<usize>(mgr_addr);
    let saved_dev = rd::<usize>(dev_addr);

    wr::<usize>(mgr_addr, mgr_o.as_mut_ptr() as usize);
    wr::<usize>(dev_addr, dev_o.as_mut_ptr() as usize);
    f_orig(this_o.as_mut_ptr() as usize);

    wr::<usize>(mgr_addr, mgr_n.as_mut_ptr() as usize);
    wr::<usize>(dev_addr, dev_n.as_mut_ptr() as usize);
    f_new(this_n.as_mut_ptr() as usize);

    wr::<usize>(mgr_addr, saved_mgr);
    wr::<usize>(dev_addr, saved_dev);

    crate::diff_eq!(this_o, this_n, "this handle stored");
    crate::diff_eq!(&dev_o[0x8..0x40], &dev_n[0x8..0x40], "srv call args");
    crate::diff_eq!(&mgr_o[0xc0..0xf0], &mgr_n[0xc0..0xf0], "ring updated");
    Ok(1)
}

pub unsafe fn diff_d3d_1420761a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut ctx_o = vec![0x5au8; 0xc80];
    let mut ctx_n = vec![0x5au8; 0xc80];
    wr::<i32>(ctx_o.as_mut_ptr() as usize + 0xb28, 3);
    wr::<i32>(ctx_n.as_mut_ptr() as usize + 0xb28, 3);

    f_orig(ctx_o.as_mut_ptr() as usize);
    f_new(ctx_n.as_mut_ptr() as usize);
    crate::diff_eq!(ctx_o, ctx_n, "reset context state");
    Ok(1)
}

pub unsafe fn diff_d3d_1420768e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut this_o = [0u8; 0x50];
    let mut this_n = [0u8; 0x50];
    this_o[0x40] = 1;
    this_n[0x40] = 1;

    f_orig(this_o.as_mut_ptr() as usize);
    f_new(this_n.as_mut_ptr() as usize);
    crate::diff_eq!(this_o, this_n, "locked flush and flag reset");
    Ok(1)
}

unsafe extern "win64" fn mock_14206a100(
    out_ptr: usize,
    d0: u32,
    d1: u32,
    d2: u32,
    d3: u32,
    d4: u8,
    d5: u32,
) -> usize {
    let heap_ptr = rd::<usize>(out_ptr);
    wr::<u32>(heap_ptr, d0 ^ d1 ^ d2 ^ d3 ^ (d4 as u32) ^ d5);
    out_ptr
}

pub unsafe fn diff_d3d_14204f540(orig: usize, new: usize) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4206_a100, mock_14206a100 as *const ())?;
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut heap_o = [0u8; 0x20];
    let mut heap_n = [0u8; 0x20];
    wr::<u32>(heap_o.as_mut_ptr() as usize + 0xc, 0x80);
    wr::<u32>(heap_n.as_mut_ptr() as usize + 0xc, 0x80);

    let mut desc = [0u8; 0x18];
    let db = desc.as_mut_ptr() as usize;
    wr::<u32>(db, 16);
    wr::<u32>(db + 0x4, 16);
    wr::<u32>(db + 0x8, 1);
    wr::<u32>(db + 0xc, 2);
    wr::<u8>(db + 0x10, 1);
    wr::<u32>(db + 0x14, 0x20);

    let mut this_o = vec![0x5au8; 0x230];
    let mut this_n = vec![0x5au8; 0x230];
    let ro = f_orig(
        this_o.as_mut_ptr() as usize,
        db,
        0,
        heap_o.as_mut_ptr() as usize,
    );
    let rn = f_new(
        this_n.as_mut_ptr() as usize,
        db,
        0,
        heap_n.as_mut_ptr() as usize,
    );
    crate::diff_eq!(ro, this_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, this_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(
        rd::<usize>(this_o.as_ptr() as usize),
        heap_o.as_ptr() as usize,
        "heap ptr orig"
    );
    crate::diff_eq!(
        rd::<usize>(this_n.as_ptr() as usize),
        heap_n.as_ptr() as usize,
        "heap ptr new"
    );
    crate::diff_eq!(heap_o, heap_n, "heap recorded args");
    crate::diff_eq!(&this_o[0x8..0x1d], &this_n[0x8..0x1d], "header fields");
    crate::diff_eq!(&this_o[0x20..0x220], &this_n[0x20..0x220], "zeroed table");
    Ok(1)
}

static mut SCRATCH_RING_ITEMS: [u8; 0x100] = [0u8; 0x100];

unsafe extern "win64" fn mock_init_pool_14243b5f0(
    pool: usize,
    _a2: u64,
    _a3: u64,
    cap: u32,
    elem_sz: u64,
    align: u32,
    a7: u64,
) -> usize {
    wr::<u64>(pool, 0);
    wr::<u64>(pool + 0x8, 0);
    wr::<u64>(pool + 0x10, (cap as u64) | ((align as u64) << 32));
    wr::<u64>(pool + 0x18, elem_sz ^ a7);
    pool
}

unsafe extern "win64" fn mock_ring_reserve_1403497a0(ring: usize, new_cap: u32) {
    wr::<usize>(ring, core::ptr::addr_of_mut!(SCRATCH_RING_ITEMS) as usize);
    wr::<u32>(ring + 0x14, new_cap.max(4));
}

unsafe extern "win64" fn mock_alloc_item_pool(_this: usize) -> usize {
    let base = core::ptr::addr_of_mut!(SCRATCH_RING_ITEMS) as usize;
    let idx = rd::<u32>(base + 0xf0);
    wr::<u32>(base + 0xf0, idx.wrapping_add(1));
    base + 0x40 + (idx as usize) * 0x30
}

pub unsafe fn diff_d3d_142051f70(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4243_b5f0, mock_init_pool_14243b5f0 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4034_97a0, mock_ring_reserve_1403497a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_1a20, mock_alloc_item_pool as *const ())?;
    type F = unsafe extern "win64" fn(usize, u64, u64, u64, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut this_o = [0x5au8; 0x110];
    let mut this_n = [0x5au8; 0x110];
    let s_base = core::ptr::addr_of_mut!(SCRATCH_RING_ITEMS) as usize;

    wr::<[u8; 0x100]>(s_base, [0x5au8; 0x100]);
    wr::<u32>(s_base + 0xf0, 0);
    let ro = f_orig(this_o.as_mut_ptr() as usize, 0x1000, 0x2000, 2, 7);
    let snap_o = rd::<[u8; 0x100]>(s_base);

    wr::<[u8; 0x100]>(s_base, [0x5au8; 0x100]);
    wr::<u32>(s_base + 0xf0, 0);
    let rn = f_new(this_n.as_mut_ptr() as usize, 0x1000, 0x2000, 2, 7);
    let snap_n = rd::<[u8; 0x100]>(s_base);

    crate::diff_eq!(ro, this_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, this_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(this_o, this_n, "this state");
    crate::diff_eq!(snap_o, snap_n, "ring and items state");
    Ok(1)
}

pub unsafe fn diff_d3d_1420528f0(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4243_b5f0, mock_init_pool_14243b5f0 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4034_97a0, mock_ring_reserve_1403497a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_23d0, mock_alloc_item_pool as *const ())?;
    type F = unsafe extern "win64" fn(usize, u64, u64, u64, u64, u32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut this_o = [0x5au8; 0xf0];
    let mut this_n = [0x5au8; 0xf0];
    let s_base = core::ptr::addr_of_mut!(SCRATCH_RING_ITEMS) as usize;

    wr::<[u8; 0x100]>(s_base, [0x5au8; 0x100]);
    wr::<u32>(s_base + 0xf0, 0);
    let ro = f_orig(this_o.as_mut_ptr() as usize, 0x4000, 0x8000, 0xc000, 2, 3);
    let snap_o = rd::<[u8; 0x100]>(s_base);

    wr::<[u8; 0x100]>(s_base, [0x5au8; 0x100]);
    wr::<u32>(s_base + 0xf0, 0);
    let rn = f_new(this_n.as_mut_ptr() as usize, 0x4000, 0x8000, 0xc000, 2, 3);
    let snap_n = rd::<[u8; 0x100]>(s_base);

    crate::diff_eq!(ro, this_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, this_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(this_o, this_n, "this state");
    crate::diff_eq!(snap_o, snap_n, "ring and items state");
    Ok(1)
}

unsafe extern "win64" fn mock_142055f80(handle_ptr: usize) -> i32 {
    (rd::<u32>(handle_ptr) & 3) as i32
}

unsafe extern "win64" fn mock_1420632b0(sub: usize) {
    wr::<u32>(sub, rd::<u32>(sub).wrapping_add(1));
}

unsafe extern "win64" fn mock_get_time_1424069a0() -> f64 {
    let r = 123.5f64;
    crate::simd_ops::set_xmm_ret(r.to_bits() as u128);
    r
}

pub unsafe fn diff_d3d_142055df0(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4205_5f80, mock_142055f80 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4206_32b0, mock_1420632b0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4240_69a0, mock_get_time_1424069a0 as *const ())?;
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut early_o = [0u8; 0x60];
    let mut early_n = [0u8; 0x60];
    wr::<i32>(early_o.as_mut_ptr() as usize, -1);
    wr::<i32>(early_n.as_mut_ptr() as usize, -1);
    let h_val: u32 = 1;
    let h_ptr = (&h_val as *const u32) as usize;
    f_orig(early_o.as_mut_ptr() as usize, h_ptr);
    f_new(early_n.as_mut_ptr() as usize, h_ptr);
    crate::diff_eq!(early_o, early_n, "early return on -1");

    let lock_addr = exe::ptr::<u8>(0x1_4385_79c0) as usize;
    let snap_lock = crate::difftest::Snapshot::take(lock_addr as *mut u8, 16);

    let mut sub_obj_o = [0u32; 4];
    let mut sub_obj_n = [0u32; 4];
    let mut entry_o = [0u8; 0x20];
    let mut entry_n = [0u8; 0x20];
    wr::<usize>(entry_o.as_mut_ptr() as usize + 0x8, sub_obj_o.as_mut_ptr() as usize);
    wr::<usize>(entry_n.as_mut_ptr() as usize + 0x8, sub_obj_n.as_mut_ptr() as usize);

    let mut table_o = [0usize, entry_o.as_mut_ptr() as usize, 0, 0];
    let mut table_n = [0usize, entry_n.as_mut_ptr() as usize, 0, 0];
    let mut slots_o = [0u8; 0x40];
    let mut slots_n = [0u8; 0x40];
    let mut sub_ring_o = [0u8; 0x20];
    let mut sub_ring_n = [0u8; 0x20];
    let mut ring_arr_o = [sub_ring_o.as_mut_ptr() as usize; 2];
    let mut ring_arr_n = [sub_ring_n.as_mut_ptr() as usize; 2];
    let mut this_o = [0u8; 0x60];
    let mut this_n = [0u8; 0x60];

    for (t, r_arr, s_ring, slots, tbl) in [
        (&mut this_o, &mut ring_arr_o, &mut sub_ring_o, &mut slots_o, &mut table_o),
        (&mut this_n, &mut ring_arr_n, &mut sub_ring_n, &mut slots_n, &mut table_n),
    ] {
        let tb = t.as_mut_ptr() as usize;
        let sb = s_ring.as_mut_ptr() as usize;
        wr::<usize>(sb, slots.as_mut_ptr() as usize);
        wr::<u32>(sb + 0xc, 1);
        wr::<u32>(sb + 0x10, 1);
        wr::<u32>(sb + 0x14, 4);
        wr::<i32>(tb, 1);
        wr::<usize>(tb + 0x28, r_arr.as_mut_ptr() as usize);
        wr::<u32>(tb + 0x30, 0);
        wr::<u32>(tb + 0x38, 1);
        wr::<u32>(tb + 0x3c, 2);
        wr::<usize>(tb + 0x50, tbl.as_mut_ptr() as usize);
    }

    wr::<u64>(lock_addr, 0);
    wr::<u32>(lock_addr + 0x8, u32::MAX);
    wr::<u32>(lock_addr + 0xc, 0);
    f_orig(this_o.as_mut_ptr() as usize, h_ptr);

    wr::<u64>(lock_addr, 0);
    wr::<u32>(lock_addr + 0x8, u32::MAX);
    wr::<u32>(lock_addr + 0xc, 0);
    f_new(this_n.as_mut_ptr() as usize, h_ptr);
    snap_lock.restore();

    crate::diff_eq!(sub_obj_o, sub_obj_n, "sub_obj updated");
    crate::diff_eq!(&entry_o[0x18..0x20], &entry_n[0x18..0x20], "timestamp stored");
    crate::diff_eq!(table_o[1], table_n[1], "table slot cleared");
    crate::diff_eq!(&sub_ring_o[0xc..0x18], &sub_ring_n[0xc..0x18], "sub_ring tail/len");
    crate::diff_eq!(
        rd::<usize>(slots_o.as_ptr() as usize + 0x10),
        entry_o.as_ptr() as usize,
        "slot entry ptr orig"
    );
    crate::diff_eq!(
        rd::<usize>(slots_n.as_ptr() as usize + 0x10),
        entry_n.as_ptr() as usize,
        "slot entry ptr new"
    );
    crate::diff_eq!(
        rd::<u32>(slots_o.as_ptr() as usize + 0x18),
        rd::<u32>(slots_n.as_ptr() as usize + 0x18),
        "slot handle val"
    );
    Ok(2)
}

static mut VEC_BUF_561A0: usize = 0;

unsafe extern "win64" fn mock_reserve_1408746b0(vec_ptr: usize, new_cap: u64) {
    wr::<usize>(vec_ptr, VEC_BUF_561A0);
    wr::<u32>(vec_ptr + 0xc, new_cap as u32);
}

pub unsafe fn diff_d3d_1420561a0(orig: usize, new: usize) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_4087_46b0, mock_reserve_1408746b0 as *const ())?;
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_addr = exe::ptr::<usize>(0x1_4397_fc08) as usize;
    let saved_g = rd::<usize>(g_addr);

    let mut vec_buf = vec![0x5au8; 0x10000];
    VEC_BUF_561A0 = vec_buf.as_mut_ptr() as usize;

    let mut this_o = [0x5au8; 0xa0];
    let mut this_n = [0x5au8; 0xa0];

    let ro = f_orig(this_o.as_mut_ptr() as usize);
    let go = rd::<usize>(g_addr);
    let zero_o = vec_buf.iter().all(|&b| b == 0);

    vec_buf.fill(0x5a);
    let rn = f_new(this_n.as_mut_ptr() as usize);
    let gn = rd::<usize>(g_addr);
    let zero_n = vec_buf.iter().all(|&b| b == 0);
    wr::<usize>(g_addr, saved_g);

    crate::diff_eq!(ro, this_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, this_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(go, this_o.as_mut_ptr() as usize, "global orig");
    crate::diff_eq!(gn, this_n.as_mut_ptr() as usize, "global new");
    crate::diff_eq!(this_o, this_n, "this state");
    crate::diff_eq!(zero_o, true, "vec zeroed orig");
    crate::diff_eq!(zero_n, true, "vec zeroed new");
    Ok(1)
}

static mut FAKE_CTX_6EA00: [u64; 8] = [0u64; 8];
static mut FAKE_CMD_LIST_PTR: usize = 0;

unsafe extern "win64" fn mock_1420693a0(this: usize) {
    wr::<u32>(this, rd::<u32>(this).wrapping_add(1));
}

unsafe extern "win64" fn mock_1411dcdf0() -> usize {
    core::ptr::addr_of_mut!(FAKE_CTX_6EA00) as usize
}

unsafe extern "win64" fn mock_142093d00(
    _unused_rcx: usize,
    code: u32,
    a3: usize,
    a4: usize,
    a5: usize,
) {
    if a4 == a3 + 0x10 && a5 == a3 + 0x20 {
        wr::<u32>(a3 + 0x30, code);
    }
}

unsafe extern "win64" fn mock_142092930(
    _unused_rcx: usize,
    code: u32,
    a3: usize,
    a4: usize,
    a5: usize,
) {
    let ctx = a3.wrapping_sub(0x8);
    if a4 == ctx + 0x18 && a5 == ctx + 0x28 {
        wr::<u32>(ctx + 0x34, code.wrapping_add(0x55));
    }
}

unsafe extern "win64" fn mock_142071360(_a1: usize, a2: usize) {
    let ctx = core::ptr::addr_of_mut!(FAKE_CTX_6EA00) as usize;
    wr::<u64>(ctx + 0x38, a2 as u64);
}

unsafe extern "win64" fn mock_1411d5b00(out_pair: usize) {
    wr::<usize>(out_pair, 0);
    wr::<usize>(out_pair + 0x8, FAKE_CMD_LIST_PTR);
}

unsafe extern "win64" fn mock_copy_desc_simple(
    dev: usize,
    num: u32,
    dst_cpu: u64,
    src_cpu: u64,
    heap_type: u32,
) {
    wr::<u32>(dev + 0x8, num);
    wr::<u64>(dev + 0x10, dst_cpu);
    wr::<u64>(dev + 0x18, src_cpu);
    wr::<u32>(dev + 0x20, heap_type);
}

unsafe extern "win64" fn mock_clear_uav(
    cmd_list: usize,
    gpu_h: u64,
    cpu_h: u64,
    res: usize,
    val: usize,
    num_rects: u32,
    rects: usize,
) {
    wr::<u64>(cmd_list + 0x8, gpu_h);
    wr::<u64>(cmd_list + 0x10, cpu_h);
    wr::<u64>(cmd_list + 0x18, res as u64);
    wr::<u64>(cmd_list + 0x20, val as u64);
    wr::<u32>(cmd_list + 0x28, num_rects);
    wr::<u64>(cmd_list + 0x30, rects as u64);
}

unsafe fn run_clear_uav_difftest(orig: usize, new: usize, vtbl_slot: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4206_93a0, mock_1420693a0 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_411d_cdf0, mock_1411dcdf0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_3d00, mock_142093d00 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_2930, mock_142092930 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_1360, mock_142071360 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411d_5b00, mock_1411d5b00 as *const ())?;

    let _ = exe::call_iat!(0x1_4264_5790u64, () -> u32);
    let tp = exe::tls_pointer();
    let saved_tls = rd::<usize>(tp);
    let mut fake_tls = [0u8; 0x100];
    fake_tls[0x70] = 1;
    wr::<usize>(tp, fake_tls.as_mut_ptr() as usize);

    let g_dev_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let stride_addr = exe::ptr::<u32>(0x1_4397_fcb0) as usize;
    let d3d_dev_addr = exe::ptr::<usize>(0x1_4397_fc80) as usize;
    let saved_g_dev = rd::<usize>(g_dev_addr);
    let saved_stride = rd::<u32>(stride_addr);
    let saved_d3d_dev = rd::<usize>(d3d_dev_addr);

    let mut fake_g_dev = [0u8; 0x40];
    wr::<usize>(fake_g_dev.as_mut_ptr() as usize + 0x20, 0x7788_99aa);
    wr::<usize>(g_dev_addr, fake_g_dev.as_mut_ptr() as usize);
    wr::<u32>(stride_addr, 0x20);

    let mut dev_vtbl = [0usize; 32];
    dev_vtbl[0xc0 / 8] = mock_copy_desc_simple as *const () as usize;
    let mut cmd_vtbl = [0usize; 56];
    cmd_vtbl[vtbl_slot / 8] = mock_clear_uav as *const () as usize;

    let mut dev_o = [0u8; 0x30];
    let mut dev_n = [0u8; 0x30];
    wr::<usize>(dev_o.as_mut_ptr() as usize, dev_vtbl.as_ptr() as usize);
    wr::<usize>(dev_n.as_mut_ptr() as usize, dev_vtbl.as_ptr() as usize);

    let mut cmd_o = [0u8; 0x40];
    let mut cmd_n = [0u8; 0x40];
    wr::<usize>(cmd_o.as_mut_ptr() as usize, cmd_vtbl.as_ptr() as usize);
    wr::<usize>(cmd_n.as_mut_ptr() as usize, cmd_vtbl.as_ptr() as usize);

    let mut this_o = [0u8; 0xa0];
    let mut this_n = [0u8; 0xa0];
    for t in [&mut this_o, &mut this_n] {
        let tb = t.as_mut_ptr() as usize;
        wr::<usize>(tb + 0x70, 0x1122_3344);
        wr::<u64>(tb + 0x90, 0x5566_7788);
    }

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let ctx_base = core::ptr::addr_of_mut!(FAKE_CTX_6EA00) as usize;
    wr::<[u64; 8]>(ctx_base, [0x10_0000, 0, 0x20_0000, 0, 0, 0, 0, 0]);
    wr::<usize>(d3d_dev_addr, dev_o.as_mut_ptr() as usize);
    FAKE_CMD_LIST_PTR = cmd_o.as_mut_ptr() as usize;
    f_orig(this_o.as_mut_ptr() as usize, 0x9999);
    let ctx_o = rd::<[u64; 8]>(ctx_base);

    wr::<[u64; 8]>(ctx_base, [0x10_0000, 0, 0x20_0000, 0, 0, 0, 0, 0]);
    wr::<usize>(d3d_dev_addr, dev_n.as_mut_ptr() as usize);
    FAKE_CMD_LIST_PTR = cmd_n.as_mut_ptr() as usize;
    f_new(this_n.as_mut_ptr() as usize, 0x9999);
    let ctx_n = rd::<[u64; 8]>(ctx_base);

    wr::<usize>(d3d_dev_addr, saved_d3d_dev);
    wr::<u32>(stride_addr, saved_stride);
    wr::<usize>(g_dev_addr, saved_g_dev);
    wr::<usize>(tp, saved_tls);

    crate::diff_eq!(this_o, this_n, "this state");
    crate::diff_eq!(ctx_o, ctx_n, "ctx state");
    crate::diff_eq!(&dev_o[0x8..], &dev_n[0x8..], "copy_desc args");
    crate::diff_eq!(&cmd_o[0x8..], &cmd_n[0x8..], "clear_uav args");
    Ok(1)
}

pub unsafe fn diff_d3d_14206ea00(orig: usize, new: usize) -> Result<u32, String> {
    run_clear_uav_difftest(orig, new, 0x188)
}

pub unsafe fn diff_d3d_14206eb60(orig: usize, new: usize) -> Result<u32, String> {
    run_clear_uav_difftest(orig, new, 0x190)
}

unsafe extern "win64" fn mock_1420604d0(arg: usize) -> usize {
    arg ^ 0x5555
}

unsafe extern "win64" fn mock_1420711e0(_rcx: usize, ctx: usize) {
    wr::<u32>(ctx, 0x77);
}

unsafe extern "win64" fn mock_cmd_set_pso(cmd_list: usize, pso: usize) {
    wr::<usize>(cmd_list + 0x8, pso);
}

unsafe extern "win64" fn mock_cmd_dispatch(cmd_list: usize, x: u32, y: u32, z: u32) {
    wr::<u32>(cmd_list + 0x10, x);
    wr::<u32>(cmd_list + 0x14, y);
    wr::<u32>(cmd_list + 0x18, z);
}

pub unsafe fn diff_d3d_14206ee70(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_411d_5b00, mock_1411d5b00 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4206_04d0, mock_1420604d0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_11e0, mock_1420711e0 as *const ())?;
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vtbl = [0usize; 32];
    vtbl[0x70 / 8] = mock_cmd_dispatch as *const () as usize;
    vtbl[0xc8 / 8] = mock_cmd_set_pso as *const () as usize;

    let mut cmd_o = [0u8; 0x28];
    let mut cmd_n = [0u8; 0x28];
    wr::<usize>(cmd_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<usize>(cmd_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

    let mut this_o = vec![0u8; 0xc68];
    let mut this_n = vec![0u8; 0xc68];
    wr::<usize>(this_o.as_mut_ptr() as usize + 0xc58, 0x1234_0000);
    wr::<usize>(this_n.as_mut_ptr() as usize + 0xc58, 0x1234_0000);
    let groups = [4u32, 8u32, 2u32];

    FAKE_CMD_LIST_PTR = cmd_o.as_mut_ptr() as usize;
    f_orig(this_o.as_mut_ptr() as usize, groups.as_ptr() as usize);

    FAKE_CMD_LIST_PTR = cmd_n.as_mut_ptr() as usize;
    f_new(this_n.as_mut_ptr() as usize, groups.as_ptr() as usize);

    crate::diff_eq!(this_o, this_n, "this state");
    crate::diff_eq!(&cmd_o[0x8..], &cmd_n[0x8..], "cmd_list pso and dispatch");
    Ok(1)
}

unsafe extern "win64" fn mock_1411dce60(sub_obj: usize, edx_val: u32, p_idx: usize) -> usize {
    sub_obj
        .wrapping_add(edx_val as usize)
        .wrapping_add(rd::<u32>(p_idx) as usize)
}

unsafe extern "win64" fn mock_14206f3a0(this: usize, topo: u32) {
    wr::<u32>(this + 0x50, topo);
}

unsafe extern "win64" fn mock_14206f290(this: usize) {
    wr::<u32>(this + 0x54, 0x4242);
}

unsafe extern "win64" fn mock_142070fb0(_rcx: usize, this: usize) -> u8 {
    rd::<u8>(this + 0xa81)
}

unsafe extern "win64" fn mock_cmd_draw_instanced(
    cmd_list: usize,
    vtx_cnt: u32,
    inst_cnt: u32,
    start_vtx: u32,
    start_inst: u32,
) {
    wr::<u32>(cmd_list + 0x8, vtx_cnt);
    wr::<u32>(cmd_list + 0xc, inst_cnt);
    wr::<u32>(cmd_list + 0x10, start_vtx);
    wr::<u32>(cmd_list + 0x14, start_inst);
}

unsafe extern "win64" fn mock_cmd_draw_indexed(
    cmd_list: usize,
    idx_cnt: u32,
    inst_cnt: u32,
    start_idx: u32,
    base_vtx: u32,
    start_inst: u32,
) {
    wr::<u32>(cmd_list + 0x8, idx_cnt);
    wr::<u32>(cmd_list + 0xc, inst_cnt);
    wr::<u32>(cmd_list + 0x10, start_idx);
    wr::<u32>(cmd_list + 0x14, base_vtx);
    wr::<u32>(cmd_list + 0x18, start_inst);
}

pub unsafe fn diff_d3d_14206ef20(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_411d_ce60, mock_1411dce60 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4206_f3a0, mock_14206f3a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_f290, mock_14206f290 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_0fb0, mock_142070fb0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_1360, mock_142071360 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411d_5b00, mock_1411d5b00 as *const ())?;
    type F = unsafe extern "win64" fn(usize, usize, usize, u32, u32, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_dev_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g_dev = rd::<usize>(g_dev_addr);
    let mut fake_g_dev = [0u8; 0x30];
    wr::<usize>(fake_g_dev.as_mut_ptr() as usize + 0x20, 0x3344_5566);
    wr::<usize>(g_dev_addr, fake_g_dev.as_mut_ptr() as usize);

    let mut vtbl = [0usize; 16];
    vtbl[0x68 / 8] = mock_cmd_draw_indexed as *const () as usize;
    let mut cmd_o = [0u8; 0x20];
    let mut cmd_n = [0u8; 0x20];
    wr::<usize>(cmd_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<usize>(cmd_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

    let idx_val: u32 = 5;
    let mut this_o = vec![0u8; 0xc60];
    let mut this_n = vec![0u8; 0xc60];
    for t in [&mut this_o, &mut this_n] {
        let tb = t.as_mut_ptr() as usize;
        wr::<u8>(tb + 0xa81, 1);
        wr::<usize>(tb + 0xb18, (&idx_val as *const u32) as usize);
        wr::<usize>(tb + 0xc48, 0x1000);
    }

    FAKE_CMD_LIST_PTR = cmd_o.as_mut_ptr() as usize;
    f_orig(this_o.as_mut_ptr() as usize, 0, 0, 12, 10, 3);

    FAKE_CMD_LIST_PTR = cmd_n.as_mut_ptr() as usize;
    f_new(this_n.as_mut_ptr() as usize, 0, 0, 12, 10, 3);
    wr::<usize>(g_dev_addr, saved_g_dev);

    crate::diff_eq!(this_o, this_n, "this state");
    crate::diff_eq!(&cmd_o[0x8..], &cmd_n[0x8..], "draw_indexed args");
    Ok(1)
}

pub unsafe fn diff_d3d_14206f020(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_411d_ce60, mock_1411dce60 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4206_f3a0, mock_14206f3a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_f290, mock_14206f290 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_0fb0, mock_142070fb0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_1360, mock_142071360 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411d_5b00, mock_1411d5b00 as *const ())?;
    type F = unsafe extern "win64" fn(usize, u32, u32, usize, usize, u32, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_dev_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g_dev = rd::<usize>(g_dev_addr);
    let mut fake_g_dev = [0u8; 0x30];
    wr::<usize>(fake_g_dev.as_mut_ptr() as usize + 0x20, 0x3344_5566);
    wr::<usize>(g_dev_addr, fake_g_dev.as_mut_ptr() as usize);

    let mut vtbl = [0usize; 16];
    vtbl[0x68 / 8] = mock_cmd_draw_indexed as *const () as usize;

    let idx_val: u32 = 3;
    for topo in [1u32, 2, 4, 5, 7] {
        let mut cmd_o = [0u8; 0x20];
        let mut cmd_n = [0u8; 0x20];
        wr::<usize>(cmd_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<usize>(cmd_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

        let mut this_o = vec![0u8; 0xc60];
        let mut this_n = vec![0u8; 0xc60];
        for t in [&mut this_o, &mut this_n] {
            let tb = t.as_mut_ptr() as usize;
            wr::<u8>(tb + 0xa80, 1);
            wr::<usize>(tb + 0xb18, (&idx_val as *const u32) as usize);
            wr::<usize>(tb + 0xc48, 0x2000);
        }

        FAKE_CMD_LIST_PTR = cmd_o.as_mut_ptr() as usize;
        f_orig(this_o.as_mut_ptr() as usize, topo, 7, 0, 0, 15, 20);

        FAKE_CMD_LIST_PTR = cmd_n.as_mut_ptr() as usize;
        f_new(this_n.as_mut_ptr() as usize, topo, 7, 0, 0, 15, 20);

        crate::diff_eq!(this_o, this_n, "this state");
        crate::diff_eq!(&cmd_o[0x8..], &cmd_n[0x8..], "draw_indexed args");
    }
    wr::<usize>(g_dev_addr, saved_g_dev);
    Ok(5)
}

pub unsafe fn diff_d3d_14206f170(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_411d_ce60, mock_1411dce60 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4206_f3a0, mock_14206f3a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_f290, mock_14206f290 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_0fb0, mock_142070fb0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_1360, mock_142071360 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411d_5b00, mock_1411d5b00 as *const ())?;
    type F = unsafe extern "win64" fn(usize, u32, usize, u32, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_dev_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g_dev = rd::<usize>(g_dev_addr);
    let mut fake_g_dev = [0u8; 0x30];
    wr::<usize>(fake_g_dev.as_mut_ptr() as usize + 0x20, 0x3344_5566);
    wr::<usize>(g_dev_addr, fake_g_dev.as_mut_ptr() as usize);

    let mut vtbl = [0usize; 16];
    vtbl[0x60 / 8] = mock_cmd_draw_instanced as *const () as usize;

    let idx_val: u32 = 2;
    for topo in [1u32, 2, 4, 5, 8] {
        let mut cmd_o = [0u8; 0x20];
        let mut cmd_n = [0u8; 0x20];
        wr::<usize>(cmd_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<usize>(cmd_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

        let mut this_o = vec![0u8; 0xc60];
        let mut this_n = vec![0u8; 0xc60];
        for t in [&mut this_o, &mut this_n] {
            let tb = t.as_mut_ptr() as usize;
            wr::<u8>(tb + 0xa80, 1);
            wr::<usize>(tb + 0xb18, (&idx_val as *const u32) as usize);
            wr::<usize>(tb + 0xc48, 0x3000);
        }

        FAKE_CMD_LIST_PTR = cmd_o.as_mut_ptr() as usize;
        f_orig(this_o.as_mut_ptr() as usize, topo, 0, 9, 4);

        FAKE_CMD_LIST_PTR = cmd_n.as_mut_ptr() as usize;
        f_new(this_n.as_mut_ptr() as usize, topo, 0, 9, 4);

        crate::diff_eq!(this_o, this_n, "this state");
        crate::diff_eq!(&cmd_o[0x8..], &cmd_n[0x8..], "draw_instanced args");
    }
    wr::<usize>(g_dev_addr, saved_g_dev);
    Ok(5)
}

unsafe extern "win64" fn mock_1420964f0(ctx: usize, b9c: u8, flag: u8) {
    wr::<u8>(ctx, b9c);
    wr::<u8>(ctx + 1, flag);
}

unsafe extern "win64" fn mock_1420965a0(ctx: usize, b9c: u8, flag: u8, has_20: u8) {
    wr::<u8>(ctx + 2, b9c);
    wr::<u8>(ctx + 3, flag);
    wr::<u8>(ctx + 4, has_20);
}

unsafe extern "win64" fn mock_14209ba00(h: usize) {
    wr::<u32>(h, rd::<u32>(h).wrapping_add(1));
}

pub unsafe fn diff_d3d_14206f680(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_411d_cdf0, mock_1411dcdf0 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4209_64f0, mock_1420964f0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_65a0, mock_1420965a0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4209_ba00, mock_14209ba00 as *const ())?;
    type F = unsafe extern "win64" fn(usize, usize, usize, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_dev_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g_dev = rd::<usize>(g_dev_addr);
    let mut fake_g_dev = [0u8; 0xb0];
    fake_g_dev[0x9c] = 0x33;
    wr::<usize>(g_dev_addr, fake_g_dev.as_mut_ptr() as usize);

    for has_10 in [true, false] {
        let mut h0_o = [0u32; 2];
        let mut h1_o = [0u32; 2];
        let mut h0_n = [0u32; 2];
        let mut h1_n = [0u32; 2];
        let handles_o = [
            h0_o.as_mut_ptr() as usize,
            0,
            h1_o.as_mut_ptr() as usize,
            0,
            0,
            0,
        ];
        let handles_n = [
            h0_n.as_mut_ptr() as usize,
            0,
            h1_n.as_mut_ptr() as usize,
            0,
            0,
            0,
        ];

        let mut src = [0x11u8; 48];
        wr::<u64>(src.as_mut_ptr() as usize + 0x10, if has_10 { 1 } else { 0 });
        wr::<u64>(src.as_mut_ptr() as usize + 0x20, 9);

        let mut this_o = vec![0x5au8; 0xc80];
        let mut this_n = vec![0x5au8; 0xc80];
        let ctx_base = core::ptr::addr_of_mut!(FAKE_CTX_6EA00) as usize;

        wr::<[u64; 8]>(ctx_base, [0u64; 8]);
        f_orig(
            this_o.as_mut_ptr() as usize,
            src.as_ptr() as usize,
            handles_o.as_ptr() as usize,
            7,
        );
        let ctx_o = rd::<[u64; 8]>(ctx_base);

        wr::<[u64; 8]>(ctx_base, [0u64; 8]);
        f_new(
            this_n.as_mut_ptr() as usize,
            src.as_ptr() as usize,
            handles_n.as_ptr() as usize,
            7,
        );
        let ctx_n = rd::<[u64; 8]>(ctx_base);

        crate::diff_eq!(this_o, this_n, "this state");
        crate::diff_eq!(ctx_o, ctx_n, "ctx state");
        crate::diff_eq!(h0_o, h0_n, "h0 updated");
        crate::diff_eq!(h1_o, h1_n, "h1 updated");
    }
    wr::<usize>(g_dev_addr, saved_g_dev);
    Ok(2)
}

pub unsafe fn diff_d3d_142072cd0(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_411d_ce60, mock_1411dce60 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4206_f3a0, mock_14206f3a0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_f290, mock_14206f290 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4207_0fb0, mock_142070fb0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4207_1360, mock_142071360 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_411d_5b00, mock_1411d5b00 as *const ())?;
    type F = unsafe extern "win64" fn(usize, u32, usize, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_dev_addr = exe::ptr::<usize>(0x1_4397_fc58) as usize;
    let saved_g_dev = rd::<usize>(g_dev_addr);
    let mut fake_g_dev = [0u8; 0x30];
    wr::<usize>(fake_g_dev.as_mut_ptr() as usize + 0x20, 0x3344_5566);
    wr::<usize>(g_dev_addr, fake_g_dev.as_mut_ptr() as usize);

    let mut vtbl = [0usize; 16];
    vtbl[0x60 / 8] = mock_cmd_draw_instanced as *const () as usize;

    let idx_val: u32 = 4;
    for topo in [1u32, 2, 4, 5, 9] {
        let mut cmd_o = [0u8; 0x20];
        let mut cmd_n = [0u8; 0x20];
        wr::<usize>(cmd_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
        wr::<usize>(cmd_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

        let mut this_o = vec![0u8; 0xc60];
        let mut this_n = vec![0u8; 0xc60];
        for t in [&mut this_o, &mut this_n] {
            let tb = t.as_mut_ptr() as usize;
            wr::<u8>(tb + 0xa80, 1);
            wr::<usize>(tb + 0xb18, (&idx_val as *const u32) as usize);
            wr::<usize>(tb + 0xc48, 0x4000);
        }

        FAKE_CMD_LIST_PTR = cmd_o.as_mut_ptr() as usize;
        f_orig(this_o.as_mut_ptr() as usize, topo, 0, 11);

        FAKE_CMD_LIST_PTR = cmd_n.as_mut_ptr() as usize;
        f_new(this_n.as_mut_ptr() as usize, topo, 0, 11);

        crate::diff_eq!(this_o, this_n, "this state");
        crate::diff_eq!(&cmd_o[0x8..], &cmd_n[0x8..], "draw_instanced args");
    }
    wr::<usize>(g_dev_addr, saved_g_dev);
    Ok(5)
}

unsafe extern "win64" fn mock_14206df80(arg: usize) {
    wr::<u32>(arg, rd::<u32>(arg).wrapping_add(10));
}

unsafe extern "win64" fn mock_142076db0(arg: usize) {
    wr::<u32>(arg + 4, rd::<u32>(arg + 4).wrapping_add(20));
}

unsafe extern "win64" fn mock_14206e7f0() {
    let ctx = core::ptr::addr_of_mut!(FAKE_CTX_6EA00) as usize;
    wr::<u32>(ctx, rd::<u32>(ctx).wrapping_add(1));
}

pub unsafe fn diff_d3d_1420734e0(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4206_df80, mock_14206df80 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_4207_6db0, mock_142076db0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4206_e7f0, mock_14206e7f0 as *const ())?;
    type F = unsafe extern "win64" fn(usize, i32) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let _ = exe::call_iat!(0x1_4264_5790u64, () -> u32);
    let tp = exe::tls_pointer();
    let saved_tls = rd::<usize>(tp);
    let mut fake_tls = vec![0u8; 0x6400];
    let tls_base = fake_tls.as_mut_ptr() as usize;
    fake_tls[0x70] = 1;
    wr::<usize>(tp, tls_base);

    let lock_addr = exe::ptr::<u8>(0x1_4385_7f30) as usize;
    let snap_lock = crate::difftest::Snapshot::take(lock_addr as *mut u8, 16);
    let g_ctx_addr = exe::ptr::<usize>(0x1_4397_fc68) as usize;
    let saved_g_ctx = rd::<usize>(g_ctx_addr);

    let mut fake_ctx = [0u64, 0x1122_3344_5566_7788u64, 0x99aa_bbcc_ddee_ff00u64];
    wr::<usize>(g_ctx_addr, fake_ctx.as_mut_ptr() as usize);

    let mut sub_o = [0u32; 2];
    let mut sub_n = [0u32; 2];
    let mut out_o = [0u8; 16];
    let mut out_n = [0u8; 16];

    let ctx_base = core::ptr::addr_of_mut!(FAKE_CTX_6EA00) as usize;
    wr::<u32>(ctx_base, 0);
    wr::<u64>(lock_addr, 0);
    wr::<u32>(lock_addr + 0x8, u32::MAX);
    wr::<u32>(lock_addr + 0xc, 0);
    wr::<usize>(tls_base + 0x6328, sub_o.as_mut_ptr() as usize);
    let ro = f_orig(out_o.as_mut_ptr() as usize, 0);
    let cnt_o = rd::<u32>(ctx_base);

    wr::<u32>(ctx_base, 0);
    wr::<u64>(lock_addr, 0);
    wr::<u32>(lock_addr + 0x8, u32::MAX);
    wr::<u32>(lock_addr + 0xc, 0);
    wr::<usize>(tls_base + 0x6328, sub_n.as_mut_ptr() as usize);
    let rn = f_new(out_n.as_mut_ptr() as usize, 0);
    let cnt_n = rd::<u32>(ctx_base);

    wr::<usize>(g_ctx_addr, saved_g_ctx);
    snap_lock.restore();
    wr::<usize>(tp, saved_tls);

    crate::diff_eq!(ro, out_o.as_mut_ptr() as usize, "ret orig");
    crate::diff_eq!(rn, out_n.as_mut_ptr() as usize, "ret new");
    crate::diff_eq!(out_o, out_n, "copied 16 bytes");
    crate::diff_eq!(sub_o, sub_n, "tls 0x6328 callbacks");
    crate::diff_eq!(cnt_o, cnt_n, "0x14206e7f0 called");
    Ok(1)
}

unsafe extern "win64" fn mock_cmd_resource_barrier(
    cmd_list: usize,
    count: u32,
    barriers: usize,
) {
    wr::<u32>(cmd_list + 0x8, count);
    let n = (count as usize).min(4) * 32;
    for i in 0..n {
        wr::<u8>(cmd_list + 0x10 + i, rd::<u8>(barriers + i));
    }
}

unsafe fn run_barrier_batch_difftest(orig: usize, new: usize) -> Result<u32, String> {
    let _m = crate::difftest::Mock::new(0x1_411d_5b00, mock_1411d5b00 as *const ())?;
    type F = unsafe extern "win64" fn(usize, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    f_orig(0, 0);
    f_new(0, 0);

    let mut vtbl = [0usize; 32];
    vtbl[0xd0 / 8] = mock_cmd_resource_barrier as *const () as usize;
    let mut cmd_o = [0u8; 0xa0];
    let mut cmd_n = [0u8; 0xa0];
    wr::<usize>(cmd_o.as_mut_ptr() as usize, vtbl.as_ptr() as usize);
    wr::<usize>(cmd_n.as_mut_ptr() as usize, vtbl.as_ptr() as usize);

    let mut it0_o = [0u8; 0x50];
    let mut it1_o = [0u8; 0x50];
    let mut it2_o = [0u8; 0x50];
    let mut it0_n = [0u8; 0x50];
    let mut it1_n = [0u8; 0x50];
    let mut it2_n = [0u8; 0x50];
    for (i0, i1, i2) in [
        (&mut it0_o, &mut it1_o, &mut it2_o),
        (&mut it0_n, &mut it1_n, &mut it2_n),
    ] {
        wr::<u64>(i0.as_mut_ptr() as usize + 0x40, 0x1001);
        wr::<u32>(i0.as_mut_ptr() as usize + 0x4c, 0x07);
        wr::<u64>(i1.as_mut_ptr() as usize + 0x40, 0x1002);
        wr::<u32>(i1.as_mut_ptr() as usize + 0x4c, 0x22);
        wr::<u64>(i2.as_mut_ptr() as usize + 0x40, 0x1003);
        wr::<u32>(i2.as_mut_ptr() as usize + 0x4c, 0x00);
    }
    let arr_o = [
        it0_o.as_mut_ptr() as usize,
        it1_o.as_mut_ptr() as usize,
        it2_o.as_mut_ptr() as usize,
    ];
    let arr_n = [
        it0_n.as_mut_ptr() as usize,
        it1_n.as_mut_ptr() as usize,
        it2_n.as_mut_ptr() as usize,
    ];

    FAKE_CMD_LIST_PTR = cmd_o.as_mut_ptr() as usize;
    f_orig(arr_o.as_ptr() as usize, 3);

    FAKE_CMD_LIST_PTR = cmd_n.as_mut_ptr() as usize;
    f_new(arr_n.as_ptr() as usize, 3);

    crate::diff_eq!(it0_o, it0_n, "item 0 state");
    crate::diff_eq!(it1_o, it1_n, "item 1 state");
    crate::diff_eq!(it2_o, it2_n, "item 2 state");
    crate::diff_eq!(&cmd_o[0x8..], &cmd_n[0x8..], "recorded barriers");
    Ok(2)
}

pub unsafe fn diff_d3d_1420738b0(orig: usize, new: usize) -> Result<u32, String> {
    run_barrier_batch_difftest(orig, new)
}

pub unsafe fn diff_d3d_142074340(orig: usize, new: usize) -> Result<u32, String> {
    run_barrier_batch_difftest(orig, new)
}

static mut STAGING_DST_74B80: [u8; 64] = [0u8; 64];
static mut ENTRY_74B80: [u64; 8] = [0u64; 8];

unsafe extern "win64" fn mock_14205f290(out_buf: usize, chunk: u64, tag: u64) -> usize {
    wr::<[u8; 96]>(out_buf, [0u8; 96]);
    wr::<u64>(out_buf, chunk ^ tag);
    wr::<usize>(
        out_buf + 0x50,
        core::ptr::addr_of_mut!(STAGING_DST_74B80) as usize,
    );
    wr::<u32>(out_buf + 0x58, 0x340);
    out_buf
}

unsafe extern "win64" fn mock_1411da370(_a1: usize, _a2: u64) -> usize {
    core::ptr::addr_of_mut!(ENTRY_74B80) as usize
}

unsafe extern "win64" fn mock_142075bc0(
    dst_res: usize,
    dst_off: u64,
    staging_base: usize,
    off_58: u32,
    chunk: u64,
) {
    wr::<u64>(dst_res, dst_off);
    wr::<u64>(dst_res + 0x8, rd::<u64>(staging_base));
    wr::<u32>(dst_res + 0x10, off_58);
    wr::<u64>(dst_res + 0x18, chunk);
}

pub unsafe fn diff_d3d_142074b80(orig: usize, new: usize) -> Result<u32, String> {
    let _m0 = crate::difftest::Mock::new(0x1_4205_f290, mock_14205f290 as *const ())?;
    let _m1 = crate::difftest::Mock::new(0x1_411d_a370, mock_1411da370 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4207_5bc0, mock_142075bc0 as *const ())?;
    type F = unsafe extern "win64" fn(usize, u64, usize, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    f_orig(0, 0, 0, 0);
    f_new(0, 0, 0, 0);

    let lock_addr = exe::ptr::<u8>(0x1_4385_7ee0) as usize;
    let snap_lock = crate::difftest::Snapshot::take(lock_addr as *mut u8, 16);

    let src = [0x3cu8; 32];
    let mut dst_o = [0u8; 0x20];
    let mut dst_n = [0u8; 0x20];
    let entry_base = core::ptr::addr_of_mut!(ENTRY_74B80) as usize;
    let stage_base = core::ptr::addr_of_mut!(STAGING_DST_74B80) as usize;

    wr::<[u8; 64]>(stage_base, [0u8; 64]);
    wr::<u64>(entry_base + 0x28, 10);
    wr::<u64>(lock_addr, 0);
    wr::<u32>(lock_addr + 0x8, u32::MAX);
    wr::<u32>(lock_addr + 0xc, 0);
    f_orig(dst_o.as_mut_ptr() as usize, 0x200, src.as_ptr() as usize, 32);
    let stage_o = rd::<[u8; 64]>(stage_base);
    let ref_o = rd::<u64>(entry_base + 0x28);

    wr::<[u8; 64]>(stage_base, [0u8; 64]);
    wr::<u64>(entry_base + 0x28, 10);
    wr::<u64>(lock_addr, 0);
    wr::<u32>(lock_addr + 0x8, u32::MAX);
    wr::<u32>(lock_addr + 0xc, 0);
    f_new(dst_n.as_mut_ptr() as usize, 0x200, src.as_ptr() as usize, 32);
    let stage_n = rd::<[u8; 64]>(stage_base);
    let ref_n = rd::<u64>(entry_base + 0x28);
    snap_lock.restore();

    crate::diff_eq!(dst_o, dst_n, "dst_res copy args");
    crate::diff_eq!(stage_o, stage_n, "staged memcpy bytes");
    crate::diff_eq!(ref_o, ref_n, "entry refcount decremented");
    Ok(2)
}

// REGISTRY:
// FLOAT_RETURN: none (no function in this module returns f32/f64 in xmm0)
//
// Replacement::verified_throws(0x1_4204_f540, "d3d::d3d_14204f540", rend::d3d_rem_a::d3d_14204f540 as *const (), Throws::Caught, rend::d3d_rem_a::diff_d3d_14204f540),
// Replacement::verified_throws(0x1_4204_f6c0, "d3d::d3d_14204f6c0", rend::d3d_rem_a::d3d_14204f6c0 as *const (), Throws::Caught, rend::d3d_rem_a::diff_d3d_14204f6c0),
// Replacement::verified_throws(0x1_4205_0470, "d3d::d3d_142050470", rend::d3d_rem_a::d3d_142050470 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142050470),
// Replacement::verified_throws(0x1_4205_0690, "d3d::d3d_142050690", rend::d3d_rem_a::d3d_142050690 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142050690),
// Replacement::verified(0x1_4205_0ef0, "d3d::d3d_142050ef0", rend::d3d_rem_a::d3d_142050ef0 as *const (), rend::d3d_rem_a::diff_d3d_142050ef0),
// Replacement::verified_throws(0x1_4205_1a20, "d3d::d3d_142051a20", rend::d3d_rem_a::d3d_142051a20 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142051a20),
// Replacement::verified_throws(0x1_4205_1ba0, "d3d::d3d_142051ba0", rend::d3d_rem_a::d3d_142051ba0 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142051ba0),
// Replacement::verified_throws(0x1_4205_1f70, "d3d::d3d_142051f70", rend::d3d_rem_a::d3d_142051f70 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142051f70),
// Replacement::verified(0x1_4205_2100, "d3d::d3d_142052100", rend::d3d_rem_a::d3d_142052100 as *const (), rend::d3d_rem_a::diff_d3d_142052100),
// Replacement::verified_throws(0x1_4205_2270, "d3d::d3d_142052270", rend::d3d_rem_a::d3d_142052270 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142052270),
// Replacement::verified_throws(0x1_4205_23d0, "d3d::d3d_1420523d0", rend::d3d_rem_a::d3d_1420523d0 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_1420523d0),
// Replacement::verified(0x1_4205_27c0, "d3d::d3d_1420527c0", rend::d3d_rem_a::d3d_1420527c0 as *const (), rend::d3d_rem_a::diff_d3d_1420527c0),
// Replacement::verified_throws(0x1_4205_28f0, "d3d::d3d_1420528f0", rend::d3d_rem_a::d3d_1420528f0 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_1420528f0),
// Replacement::verified_throws(0x1_4205_5df0, "d3d::d3d_142055df0", rend::d3d_rem_a::d3d_142055df0 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142055df0),
// Replacement::verified(0x1_4205_6040, "d3d::d3d_142056040", rend::d3d_rem_a::d3d_142056040 as *const (), rend::d3d_rem_a::diff_d3d_142056040),
// Replacement::verified(0x1_4205_61a0, "d3d::d3d_1420561a0", rend::d3d_rem_a::d3d_1420561a0 as *const (), rend::d3d_rem_a::diff_d3d_1420561a0),
// Replacement::verified(0x1_4205_7130, "d3d::d3d_142057130", rend::d3d_rem_a::d3d_142057130 as *const (), rend::d3d_rem_a::diff_d3d_142057130),
// Replacement::verified(0x1_4205_7410, "d3d::d3d_142057410", rend::d3d_rem_a::d3d_142057410 as *const (), rend::d3d_rem_a::diff_d3d_142057410),
// Replacement::verified(0x1_4205_8010, "d3d::d3d_142058010", rend::d3d_rem_a::d3d_142058010 as *const (), rend::d3d_rem_a::diff_d3d_142058010),
// Replacement::verified(0x1_4206_d170, "d3d::d3d_14206d170", rend::d3d_rem_a::d3d_14206d170 as *const (), rend::d3d_rem_a::diff_d3d_14206d170),
// Replacement::verified_throws(0x1_4206_ea00, "d3d::d3d_14206ea00", rend::d3d_rem_a::d3d_14206ea00 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_14206ea00),
// Replacement::verified_throws(0x1_4206_eb60, "d3d::d3d_14206eb60", rend::d3d_rem_a::d3d_14206eb60 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_14206eb60),
// Replacement::verified_throws(0x1_4206_ee70, "d3d::d3d_14206ee70", rend::d3d_rem_a::d3d_14206ee70 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_14206ee70),
// Replacement::verified_throws(0x1_4206_ef20, "d3d::d3d_14206ef20", rend::d3d_rem_a::d3d_14206ef20 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_14206ef20),
// Replacement::verified_throws(0x1_4206_f020, "d3d::d3d_14206f020", rend::d3d_rem_a::d3d_14206f020 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_14206f020),
// Replacement::verified_throws(0x1_4206_f170, "d3d::d3d_14206f170", rend::d3d_rem_a::d3d_14206f170 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_14206f170),
// Replacement::verified_throws(0x1_4206_f680, "d3d::d3d_14206f680", rend::d3d_rem_a::d3d_14206f680 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_14206f680),
// Replacement::verified_throws(0x1_4207_2cd0, "d3d::d3d_142072cd0", rend::d3d_rem_a::d3d_142072cd0 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142072cd0),
// Replacement::verified_throws(0x1_4207_34e0, "d3d::d3d_1420734e0", rend::d3d_rem_a::d3d_1420734e0 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_1420734e0),
// Replacement::verified(0x1_4207_38b0, "d3d::d3d_1420738b0", rend::d3d_rem_a::d3d_1420738b0 as *const (), rend::d3d_rem_a::diff_d3d_1420738b0),
// Replacement::verified(0x1_4207_4340, "d3d::d3d_142074340", rend::d3d_rem_a::d3d_142074340 as *const (), rend::d3d_rem_a::diff_d3d_142074340),
// Replacement::verified_throws(0x1_4207_4b80, "d3d::d3d_142074b80", rend::d3d_rem_a::d3d_142074b80 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142074b80),
// Replacement::verified_throws(0x1_4207_5f60, "d3d::d3d_142075f60", rend::d3d_rem_a::d3d_142075f60 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_142075f60),
// Replacement::verified(0x1_4207_61a0, "d3d::d3d_1420761a0", rend::d3d_rem_a::d3d_1420761a0 as *const (), rend::d3d_rem_a::diff_d3d_1420761a0),
// Replacement::verified_throws(0x1_4207_68e0, "d3d::d3d_1420768e0", rend::d3d_rem_a::d3d_1420768e0 as *const (), Throws::Discard, rend::d3d_rem_a::diff_d3d_1420768e0),
