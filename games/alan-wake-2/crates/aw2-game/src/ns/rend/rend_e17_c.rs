//! Decompiled `rend` functions (`0x14224e6e0..0x142266940`).

use std::sync::atomic::{AtomicU32, Ordering};

use crate::exe::{self, call_va, rd, wr};

#[inline]
fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h = (h ^ (b as i8 as i64 as u64)).wrapping_mul(0x0100_0000_01b3);
    }
    h
}

#[inline]
unsafe fn find_pass_resource(pass_data: usize, hash: u64) -> usize {
    for i in 0..0x40usize {
        let k = rd::<u64>(pass_data + 0x208 + i * 8);
        if k != 0 && k == hash {
            return rd::<usize>(pass_data + 0x8 + i * 8);
        }
    }
    0
}

/// Original: `0x14224e6e0` (68 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14224e6e0(this: usize) -> u64 {
    let arg_rdx = rd::<u64>(this + 0x18);
    let arg_rcx = rd::<u64>(this + 0x8);
    let g_rdi = rd::<u64>(exe::ptr::<u64>(0x1_4397_ff10) as usize);
    call_va!(0x1_4239_af80, (arg_rcx, arg_rdx) -> ());
    let rdx2 = rd::<u64>(this);
    let r8 = this.wrapping_add(0x8);
    call_va!(0x1_4225_5e30, (g_rdi, rdx2, r8) -> u64)
}

/// Original: `0x14224ebf0` (96 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14224ebf0(
    this: usize,
    a2: u64,
    sp1: usize,
    sp2: usize,
    a5_ptr: usize,
) -> usize {
    wr::<u64>(this, a2);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x10, 0);
    let ctrl1 = rd::<usize>(sp1 + 0x8);
    if ctrl1 != 0 {
        AtomicU32::from_ptr((ctrl1 + 0x8) as *mut u32).fetch_add(1, Ordering::SeqCst);
    }
    wr::<u64>(this + 0x8, rd::<u64>(sp1));
    wr::<u64>(this + 0x10, rd::<u64>(sp1 + 0x8));
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    let ctrl2 = rd::<usize>(sp2 + 0x8);
    if ctrl2 != 0 {
        AtomicU32::from_ptr((ctrl2 + 0x8) as *mut u32).fetch_add(1, Ordering::SeqCst);
    }
    wr::<u64>(this + 0x18, rd::<u64>(sp2));
    wr::<u64>(this + 0x20, rd::<u64>(sp2 + 0x8));
    wr::<u64>(this + 0x28, rd::<u64>(a5_ptr));
    this
}

/// Original: `0x14224ec60` (149 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn rend_14224ec60(this: usize) {
    let arg_rdx = rd::<u64>(this + 0x8);
    let arg_rcx = rd::<u64>(this + 0x18);
    call_va!(0x1_4239_af80, (arg_rcx, arg_rdx) -> ());
    let rdx1 = rd::<u64>(this);
    let r8_1 = this.wrapping_add(0x18);
    let rcx1 = rd::<u64>(this + 0x28);
    call_va!(0x1_4225_5e30, (rcx1, rdx1, r8_1) -> ());
    let obj = rd::<usize>(this);
    if ((obj + 0x80) & 3) == 0 {
        AtomicU32::from_ptr((obj + 0x80) as *mut u32).swap(4, Ordering::SeqCst);
    } else {
        wr::<u32>(obj + 0x80, 4);
    }
    let obj2 = rd::<u64>(this);
    let mgr = rd::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize);
    call_va!(0x1_4242_0ed0, (mgr, obj2) -> ());
    let _ = AtomicU32::from_ptr(exe::ptr::<u32>(0x1_4397_ff00) as *mut u32).compare_exchange(
        1,
        0,
        Ordering::SeqCst,
        Ordering::SeqCst,
    );
    let streamer = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    exe::call_iat!(0x1_4264_55a8, (streamer + 0x250) -> ());
    wr::<u32>(
        streamer + 0x258,
        rd::<u32>(streamer + 0x258).wrapping_add(1),
    );
    exe::call_iat!(0x1_4264_5598, (streamer + 0x250) -> ());
    exe::call_iat!(0x1_4264_5590, (streamer + 0x248) -> ());
}

/// Original: `0x14224f1e0` (306 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn rend_14224f1e0(this: usize, data: usize) {
    let rsi = rd::<u64>(data + 0x28);
    let rbp = rd::<u64>(data + 0x20);
    let mut str_ptr = data.wrapping_add(0x8) as u64;
    let b3 = rd::<u8>(data + 0x3) as u32;
    let cap = if b3 < 0x80 {
        b3 << 3
    } else {
        (b3 << 8).wrapping_sub(0x7c00)
    };
    let hdr = (rd::<u32>(data) & 0x00ff_ffff) << 3;
    if hdr > cap {
        str_ptr = rd::<u64>(data + 0x8);
    }
    let str_len = rd::<u32>(data + 0x4) as u64;
    let mut view = [str_ptr, str_len];
    let mut mem_file = [0u8; 0x138];
    let mf = mem_file.as_mut_ptr() as usize;
    wr::<u64>(mf, exe::ptr::<u8>(0x1_4340_b4d8) as u64);
    wr::<u32>(mf + 0x8, 0);
    wr::<u32>(mf + 0xc, 0x2000_0020);
    wr::<u32>(mf + 0x10, 0);
    wr::<u8>(mf + 0x14, 0);
    call_va!(
        0x1_4240_9b50,
        (mf + 0xc, view.as_mut_ptr() as usize, 0u8, str_len) -> ()
    );
    wr::<u64>(mf + 0x118, rbp);
    wr::<u64>(mf + 0x120, rsi);
    wr::<u64>(mf + 0x128, 0);
    call_va!(0x1_4224_ed00, (rd::<usize>(this), mf) -> ());
    call_va!(0x1_4240_ab10, (mf + 0xc) -> ());
    let buf = rd::<usize>(data + 0x30);
    if buf != 0 {
        let alloc = rd::<usize>(data + 0x48);
        let flags = rd::<u16>(data + 0x40);
        let vtbl = rd::<usize>(alloc);
        let size = rd::<u64>(data + 0x38);
        call_va!(rd::<usize>(vtbl + 0x10), (alloc, buf, size, flags) -> ());
    }
    call_va!(0x1_4240_ab10, (data) -> ());
}

/// Original: `0x14224f320` (315 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14224f320(this: usize) {
    call_va!(0x1_4240_7c80, (this) -> ());
    let g_320 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8320) as usize);
    let vtbl_320 = rd::<usize>(g_320);
    let vcall_ret = call_va!(rd::<usize>(vtbl_320 + 0x120), (g_320) -> u64);
    let val_38 = rd::<i32>(this + 0x38);
    let c4ac24 = rd::<u32>(exe::ptr::<u32>(0x1_43c4_ac24) as usize) as u64;
    let prio: u32 = if val_38 >= 1000 {
        1000
    } else if val_38 >= 100 {
        100
    } else if val_38 >= 0x33 {
        0x33
    } else if val_38 >= 0x14 {
        0x14
    } else {
        1
    };

    let mut func_obj = [0u64; 8];
    let fn_ptr = func_obj.as_mut_ptr() as usize;
    wr::<u64>(fn_ptr, exe::ptr::<u8>(0x1_433f_2968) as u64);
    wr::<u64>(fn_ptr + 0x8, this as u64);
    wr::<u64>(fn_ptr + 0x38, fn_ptr as u64);

    let mut params = [0u64; 9];
    let p_ptr = params.as_mut_ptr() as usize;
    let c70 = exe::ptr::<u64>(0x1_4335_4c70) as usize;
    wr::<u64>(p_ptr, rd::<u64>(c70));
    wr::<u64>(p_ptr + 0x8, rd::<u64>(c70 + 0x8));
    wr::<u64>(p_ptr + 0x10, 0);
    wr::<u64>(p_ptr + 0x18, 0);
    wr::<u32>(p_ptr + 0x20, 0);
    wr::<u32>(p_ptr + 0x24, prio);
    wr::<u64>(p_ptr + 0x28, 0);
    wr::<u64>(p_ptr + 0x30, c4ac24);
    wr::<u64>(p_ptr + 0x38, vcall_ret);
    wr::<u64>(p_ptr + 0x40, 0);

    let sub_30 = rd::<usize>(this + 0x30);
    let mut path_in = [rd::<u64>(sub_30 + 0x28), rd::<u64>(sub_30 + 0x30)];
    let mut path_tmp = [0u64; 2];
    let ret_ptr = call_va!(
        0x1_401d_f850,
        (path_tmp.as_mut_ptr() as usize, path_in.as_mut_ptr() as usize) -> usize
    );
    path_in[0] = rd::<u64>(ret_ptr);
    path_in[1] = rd::<u64>(ret_ptr + 0x8);

    let mut out_handle = 0u64;
    let g_2f0 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f0) as usize);
    call_va!(
        0x1_4247_e5f0,
        (
            g_2f0,
            (&mut out_handle as *mut u64) as usize,
            path_in.as_mut_ptr() as usize,
            fn_ptr,
            p_ptr,
        ) -> ()
    );
}

/// Original: `0x14224f470` (182 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14224f470(this: usize) {
    let lock_flag = AtomicU32::from_ptr(exe::ptr::<u32>(0x1_4397_ff00) as *mut u32);
    if lock_flag
        .compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
    {
        call_va!(0x1_4224_f320, (this) -> ());
    } else {
        let streamer = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff08) as usize);
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        if rd::<u32>(streamer + 0x10) != tid {
            exe::call_iat!(0x1_4264_55a8, (streamer + 0x8) -> ());
            wr::<u32>(streamer + 0x10, tid);
        }
        wr::<u32>(
            streamer + 0x14,
            rd::<u32>(streamer + 0x14).wrapping_add(1),
        );
        let len = rd::<u32>(streamer + 0x20) as u64;
        let cap = rd::<u32>(streamer + 0x24) as u64;
        if len.wrapping_mul(8) == cap.wrapping_mul(8) {
            call_va!(0x1_401e_6d50, (streamer + 0x18, len.wrapping_add(1)) -> ());
        }
        let cur_len = rd::<u32>(streamer + 0x20) as usize;
        let buf = rd::<usize>(streamer + 0x18);
        wr::<u64>(buf + cur_len * 8, this as u64);
        wr::<u32>(streamer + 0x20, (cur_len as u32).wrapping_add(1));
        let new_depth = rd::<u32>(streamer + 0x14).wrapping_sub(1);
        wr::<u32>(streamer + 0x14, new_depth);
        if new_depth == 0 {
            AtomicU32::from_ptr((streamer + 0x10) as *mut u32).swap(u32::MAX, Ordering::SeqCst);
            exe::call_iat!(0x1_4264_5598, (streamer + 0x8) -> ());
        }
    }
}

/// Original: `0x14224f530` (30 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14224f530(this: usize) -> usize {
    call_va!(0x1_4242_8d70, (this) -> usize);
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_29a0) as u64);
    this
}

/// Original: `0x14224fab0` (33 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_14224fab0(_this: usize, dst: usize) -> usize {
    call_va!(0x1_4242_8d70, (dst) -> usize);
    wr::<u64>(dst, exe::ptr::<u8>(0x1_433f_29a0) as u64);
    dst
}

/// Original: `0x142250930` (51 bytes, `Throws::No`).
pub unsafe extern "win64" fn rend_142250930(this: usize) -> u8 {
    let r8 = rd::<u64>(this + 0x18);
    let len0 = rd::<u32>(this + 0x20) as u64;
    if r8 != r8.wrapping_add(len0.wrapping_mul(8)) {
        return 1;
    }
    let rdx = rd::<u64>(this + 0x28);
    let len1 = rd::<u32>(this + 0x30) as u64;
    if rdx == rdx.wrapping_add(len1.wrapping_mul(8)) {
        return 0;
    }
    u8::from(rd::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize) == 0)
}

/// Original: `0x142250970` (71 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142250970(this: usize) {
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_2b18) as u64);
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    call_va!(0x1_4240_d480, (mgr, this) -> ());
    wr::<u64>(exe::ptr::<u64>(0x1_4397_ff08) as usize, 0);
    call_va!(0x1_401d_e830, (this + 0x28) -> ());
    call_va!(0x1_401d_e830, (this + 0x18) -> ());
}

/// Original: `0x1422509c0` (82 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_1422509c0(this: usize) -> usize {
    wr::<u32>(this + 0x10, u32::MAX);
    wr::<u64>(this, exe::ptr::<u8>(0x1_433f_2b18) as u64);
    wr::<u64>(exe::ptr::<u64>(0x1_4397_ff08) as usize, this as u64);
    wr::<u32>(this + 0x14, 0);
    wr::<u64>(this + 0x8, 0);
    wr::<u64>(this + 0x18, 0);
    wr::<u64>(this + 0x20, 0);
    wr::<u64>(this + 0x28, 0);
    wr::<u64>(this + 0x30, 0);
    let mgr = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize);
    call_va!(0x1_4240_d5c0, (mgr, this) -> ());
    this
}

/// Original: `0x142250a20` (153 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142250a20(this: usize, val: u64) {
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(this + 0x10) != tid {
        exe::call_iat!(0x1_4264_55a8, (this + 0x8) -> ());
        wr::<u32>(this + 0x10, tid);
    }
    wr::<u32>(this + 0x14, rd::<u32>(this + 0x14).wrapping_add(1));
    let len = rd::<u32>(this + 0x20) as u64;
    let cap = rd::<u32>(this + 0x24) as u64;
    if len.wrapping_mul(8) == cap.wrapping_mul(8) {
        call_va!(0x1_401e_6d50, (this + 0x18, len.wrapping_add(1)) -> ());
    }
    let cur_len = rd::<u32>(this + 0x20) as usize;
    let buf = rd::<usize>(this + 0x18);
    wr::<u64>(buf + cur_len * 8, val);
    wr::<u32>(this + 0x20, (cur_len as u32).wrapping_add(1));
    let new_depth = rd::<u32>(this + 0x14).wrapping_sub(1);
    wr::<u32>(this + 0x14, new_depth);
    if new_depth == 0 {
        AtomicU32::from_ptr((this + 0x10) as *mut u32).swap(u32::MAX, Ordering::SeqCst);
        exe::call_iat!(0x1_4264_5598, (this + 0x8) -> ());
    }
}

/// Original: `0x142250c30` (388 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn unknown_142250c30(this: usize) {
    let table: [i32; 6] = [0, 1, 0, 2, 0, 3];
    let rbx = this.wrapping_add(0x1c8);
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_87a0) as usize) != 0 {
        let idx = rd::<i32>(this + 0x298) as isize;
        let edx = table[idx as usize] as u32;
        let r8d = edx.wrapping_mul(rd::<u32>(rbx));
        call_va!(0x1_4225_0fa0, (this, edx, r8d) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_8670) as usize) != 0 {
        let mut all_nonzero = true;
        let mut cur = this.wrapping_add(0xa8);
        let end = cur.wrapping_add(0x120);
        while cur != end {
            if rd::<u8>(cur) == 0 {
                all_nonzero = false;
                break;
            }
            cur = cur.wrapping_add(0x48);
        }
        if all_nonzero {
            call_va!(0x1_4225_62e0, (this.wrapping_add(0x1c8), this.wrapping_add(0xa8)) -> ());
        }
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_88a8) as usize) != 0 {
        let time_f64 = rd::<f64>(exe::ptr::<f64>(0x1_43a0_0170) as usize);
        let threshold = rd::<f32>(exe::ptr::<f32>(0x1_4342_6abc) as usize);
        let mut cur = this;
        for _ in 0..2 {
            let mut cnt = rd::<u32>(cur + 0xe8);
            let mut base = rd::<usize>(cur + 0xe0);
            if cnt != 0 {
                let time_f32 = time_f64 as f32;
                loop {
                    let elem_time = rd::<f32>(base + (cnt as usize) * 0x20 - 4);
                    if !(time_f32 - elem_time > threshold) {
                        break;
                    }
                    cnt = cnt.wrapping_sub(1);
                    wr::<u32>(cur + 0xe8, cnt);
                    base = rd::<usize>(cur + 0xe0);
                    if cnt == 0 {
                        break;
                    }
                }
            }
            let mut cnt2 = rd::<u32>(cur + 0x130);
            let mut base2 = rd::<usize>(cur + 0x128);
            if cnt2 != 0 {
                let time_f32 = time_f64 as f32;
                loop {
                    let elem_time = rd::<f32>(base2 + (cnt2 as usize) * 0x20 - 4);
                    if !(time_f32 - elem_time > threshold) {
                        break;
                    }
                    cnt2 = cnt2.wrapping_sub(1);
                    wr::<u32>(cur + 0x130, cnt2);
                    base2 = rd::<usize>(cur + 0x128);
                    if cnt2 == 0 {
                        break;
                    }
                }
            }
            cur = cur.wrapping_add(0x90);
        }
    }
    let next_idx = rd::<i32>(this + 0x298).wrapping_add(1) % 6;
    wr::<i32>(this + 0x298, next_idx);
}

/// Original: `0x14225fa00` (7 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_14225fa00(n: u32) -> u32 {
    n.wrapping_add(0x3f) >> 6
}

/// Original: `0x1422611a0` (9 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_1422611a0(this: usize) -> usize {
    call_va!(0x1_412f_7fb0, (this.wrapping_add(0x38)) -> usize)
}

/// Original: `0x142261ae0` (96 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142261ae0(this: usize) {
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_98a0) as usize) != 0 {
        call_va!(0x1_423f_e030, (exe::ptr::<u8>(0x1_433f_3118) as usize) -> ());
        call_va!(
            0x1_423f_e030,
            (exe::ptr::<u8>(0x1_433f_3108) as usize, rd::<u32>(this + 0x2e8)) -> ()
        );
        call_va!(
            0x1_423f_e030,
            (exe::ptr::<u8>(0x1_433f_30f0) as usize, rd::<u32>(this + 0x2ec)) -> ()
        );
    }
    wr::<u64>(this + 0x2e8, 0);
}

/// Original: `0x142261b50` (400 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142261b50(this: usize) {
    call_va!(0x1_403d_79a0, (this.wrapping_add(0x6b0), this.wrapping_add(0x698)) -> usize);
    let bit_count = rd::<u64>(this + 0x6a8);
    wr::<u64>(this + 0x6c0, bit_count);
    let r9 = rd::<u64>(this + 0x6a8);
    let rdx = rd::<usize>(this + 0x698);
    let r8 = r9.wrapping_add(0x3f) >> 6;
    if r9 != 0 {
        for i in 0..(r8 as usize).saturating_sub(1) {
            wr::<u64>(rdx + i * 8, 0);
        }
        let shift = (!(r9.wrapping_sub(1)) as u32) & 0x3f;
        let mask = !(u64::MAX >> shift);
        let last_addr = rdx + ((r8 as usize) - 1) * 8;
        wr::<u64>(last_addr, rd::<u64>(last_addr) & mask);
    }

    for (ptr_off, count_off) in [
        (0x638usize, 0x648usize),
        (0x650, 0x660),
        (0x668, 0x678),
        (0x680, 0x690),
        (0x180, 0x190),
    ] {
        let count = rd::<u64>(this + count_off);
        let words = count.wrapping_add(0x3f) >> 6;
        let ptr = rd::<u64>(this + ptr_off);
        let mut desc = [words, ptr, count];
        call_va!(0x1_4126_eeb0, (desc.as_mut_ptr() as usize, 0u8) -> ());
    }
}

/// Original: `0x142261cf0` (159 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142261cf0(this: usize) {
    let count = rd::<u32>(this + 0x20) as usize;
    let words_ptr = rd::<usize>(this + 0x180);
    let word_count = (count + 0x3f) >> 6;
    for w in 0..word_count {
        let mut bits = rd::<u64>(words_ptr + w * 8);
        wr::<u64>(words_ptr + w * 8, 0);
        let base_idx = w * 64;
        while bits != 0 {
            let bit = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            let idx = base_idx + bit;
            let src = rd::<usize>(this + 0x1a8);
            let dst = rd::<usize>(this + 0x1b8);
            wr::<u8>(dst + idx, rd::<u8>(src + idx));
        }
    }
}

/// Original: `0x1422622e0` (27 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_1422622e0(this: usize) -> usize {
    let count = rd::<u32>(this + 0x20) as usize;
    let src = rd::<usize>(this + 0x28);
    let dst = rd::<usize>(this + 0x5f0);
    let bytes = count * 12;
    call_va!(0x1_4183_acbc, (dst, src, bytes) -> usize)
}

/// Original: `0x142262310` (334 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142262310(this: usize) {
    let count = rd::<u32>(this + 0x20) as usize;
    let words_ptr = rd::<usize>(this + 0x180);
    let word_count = (count + 0x3f) >> 6;
    for w in 0..word_count {
        let mut bits = rd::<u64>(words_ptr + w * 8);
        let base_idx = w * 64;
        while bits != 0 {
            let bit = bits.trailing_zeros() as usize;
            bits &= bits - 1;
            let idx = base_idx + bit;
            let edx = rd::<u32>(rd::<usize>(this + 0x198) + idx * 4);
            if edx != u32::MAX {
                let r11b = rd::<u8>(rd::<usize>(this + 0x240) + (edx as usize) * 36 + 1);
                if r11b != 0 {
                    let r8 = rd::<usize>(rd::<usize>(this + 0x18) + idx * 8);
                    let mut skip = false;
                    let eax_50 = rd::<u32>(r8 + 0x50);
                    if eax_50 != u32::MAX {
                        let g_fed0 = rd::<usize>(exe::ptr::<usize>(0x1_4397_fed0) as usize);
                        let arr_100 = rd::<usize>(g_fed0 + 0x100);
                        if rd::<u32>(arr_100 + (eax_50 as usize) * 4) != u32::MAX {
                            let rdx_obj = rd::<usize>(r8 + 0x8);
                            let g_ff48 = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff48) as usize);
                            let off_48 = rd::<u32>(r8 + 0x48) as usize;
                            let r10b = rd::<u8>(rdx_obj + 0x272);
                            let byte_ptr = rd::<usize>(g_ff48 + 0x1a8).wrapping_add(off_48);
                            if r10b != 0 {
                                let mut ecx = rd::<u8>(byte_ptr) as i32;
                                let max_idx = (r10b as i32) - 1;
                                if max_idx < ecx {
                                    ecx = max_idx;
                                }
                                if rd::<i32>(rdx_obj + 0x1c8 + ((ecx as isize) * 4) as usize) > 0 {
                                    skip = true;
                                }
                            }
                        }
                    }
                    if !skip {
                        let mut edx_lod = rd::<u8>(rd::<usize>(this + 0x1a8) + idx) as i32;
                        let max_lod = (r11b as i32) - 1;
                        if max_lod < edx_lod {
                            edx_lod = max_lod;
                        }
                        let arg_rdx = rd::<usize>(r8 + 0x8)
                            .wrapping_add((((edx_lod as isize) + 0x30) << 4) as usize);
                        call_va!(0x1_4230_7f90, (r8, arg_rdx) -> ());
                    }
                }
            }
        }
    }
}

/// Original: `0x142262d70` (160 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142262d70(this: usize, a2: u32, a3: usize) -> u32 {
    let vec = this.wrapping_add(0x140);
    let len = rd::<u32>(vec + 0x8) as u64;
    let bytes = len.wrapping_mul(24);
    let idx = (bytes / 24) as u64;
    let v0 = rd::<u64>(a3);
    let v1 = rd::<u64>(a3 + 0x8);
    let cap = rd::<u32>(vec + 0xc) as u64;
    if bytes == cap.wrapping_mul(24) {
        call_va!(0x1_411d_ba80, (vec, idx.wrapping_add(1)) -> ());
    }
    let cur_len = rd::<u32>(vec + 0x8) as usize;
    let base = rd::<usize>(vec);
    let slot = base + cur_len * 24;
    wr::<u32>(slot, a2);
    wr::<u64>(slot + 0x8, v0);
    wr::<u64>(slot + 0x10, v1);
    wr::<u32>(vec + 0x8, (cur_len as u32).wrapping_add(1));
    idx as u32
}

/// Original: `0x142263030` (201 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142263030(this: usize, a2: u32, a3_r8: u64) -> u32 {
    let vec = this.wrapping_add(0x130);
    let len = rd::<u32>(vec + 0x8);
    let entry = (a3_r8 & 0xffff_ff00_0000_0000) | (1u64 << 32) | (a2 as u64);
    let cap = rd::<u32>(vec + 0xc);
    if (len as u64).wrapping_mul(8) == (cap as u64).wrapping_mul(8) {
        call_va!(0x1_4020_bcb0, (vec, (len as u64).wrapping_add(1)) -> ());
    }
    let cur_len = rd::<u32>(vec + 0x8) as usize;
    let base = rd::<usize>(vec);
    wr::<u64>(base + cur_len * 8, entry);
    wr::<u32>(vec + 0x8, (cur_len as u32).wrapping_add(1));

    call_va!(
        0x1_40ff_f1e0,
        (this.wrapping_add(0x668), rd::<u64>(this + 0x678).wrapping_add(1)) -> ()
    );
    call_va!(
        0x1_40ff_f1e0,
        (this.wrapping_add(0x680), rd::<u64>(this + 0x690).wrapping_add(1)) -> ()
    );
    call_va!(
        0x1_40ff_f1e0,
        (this.wrapping_add(0x698), rd::<u64>(this + 0x6a8).wrapping_add(1)) -> ()
    );
    call_va!(
        0x1_40ff_f1e0,
        (this.wrapping_add(0x6b0), rd::<u64>(this + 0x6c0).wrapping_add(1)) -> ()
    );
    len
}

/// Original: `0x142263120` (12 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142263120(this: usize) {
    call_va!(0x1_4225_e6c0, (this.wrapping_add(0x2d0)) -> ());
}

/// Original: `0x142263140` (67 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142263140(this: usize, a2: u32, a3: u32) {
    call_va!(0x1_4222_9c90, (this.wrapping_add(0x390), a2, a3) -> ());
    call_va!(0x1_4222_9c90, (this.wrapping_add(0x438), a2, a3) -> ());
}

/// Original: `0x142263190` (66 bytes, `Throws::No`).
pub unsafe extern "win64" fn unknown_142263190(this: usize, a2: u32) -> u32 {
    let r = call_va!(0x1_412f_81b0, (this.wrapping_add(0x300), a2) -> u32);
    call_va!(0x1_412f_81b0, (this.wrapping_add(0x3a8), a2) -> u32);
    r
}

/// Original: `0x1422639d0` (115 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_1422639d0(this: usize) -> usize {
    let dst = call_va!(0x1_4183_4d64, (0x50usize) -> usize);
    wr::<u64>(dst, exe::ptr::<u8>(0x1_433f_40f8) as u64);
    wr::<u64>(dst + 0x10, rd::<u64>(this + 0x10));
    wr::<u32>(dst + 0x18, rd::<u32>(this + 0x18));
    wr::<u64>(dst + 0x1c, rd::<u64>(this + 0x1c));
    wr::<u32>(dst + 0x24, rd::<u32>(this + 0x24));
    wr::<u64>(dst + 0x28, rd::<u64>(this + 0x28));
    wr::<u32>(dst + 0x30, rd::<u32>(this + 0x30));
    wr::<u64>(dst + 0x34, rd::<u64>(this + 0x34));
    wr::<u32>(dst + 0x3c, rd::<u32>(this + 0x3c));
    wr::<u64>(dst + 0x40, rd::<u64>(this + 0x40));
    wr::<u64>(dst + 0x48, rd::<u64>(this + 0x48));
    dst
}

/// Original: `0x142263a60` (343 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_142263a60(this: usize, pass_data_ptr: usize) {
    let rsi = this.wrapping_add(0x10);
    let rdi = rd::<usize>(pass_data_ptr);
    let p0 = call_va!(0x1_412f_f1b0, (rdi) -> usize);
    let inner0 = rd::<usize>(p0);
    let w: u32 = if inner0 != 0 { rd::<u32>(inner0) } else { 0 };
    let p1 = call_va!(0x1_412f_f1b0, (rdi) -> usize);
    let inner1 = rd::<usize>(p1);
    let h: u32 = if inner1 != 0 {
        rd::<u32>(inner1 + 0x4)
    } else {
        0
    };
    let min_dt = rd::<f32>(exe::ptr::<f32>(0x1_4342_6494) as usize);
    let cur_dt = rd::<f32>(exe::ptr::<f32>(0x1_43a0_017c) as usize);
    let dt = if min_dt > cur_dt { min_dt } else { cur_dt };
    let packed_wh = ((h as u64) << 32) | (w as u64);
    let arg_r11 = rd::<u64>(rsi + 0x38);
    let p2 = call_va!(0x1_412f_f1b0, (rdi) -> usize);
    let arg_r9 = rd::<u64>(p2);

    let vel_hash = fnv1a64(b"velocity_geometry_buffer");
    let rbx = find_pass_resource(rdi, vel_hash);

    let sub_30 = rd::<usize>(rsi + 0x30);
    let f_120 = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0120) as usize);
    let f_148 = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0148) as usize);
    let f_124 = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0124) as usize);
    let f_dt = dt.to_bits();
    let r9_val = rd::<u64>(rbx);
    let r8d = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0180) as usize);
    let rdx_val = rd::<u64>(sub_30 + 0x48);
    call_va!(
        0x1_4205_bb70,
        (
            rsi, rdx_val, r8d, r9_val, arg_r9, arg_r11, packed_wh, f_dt, f_124, f_148, f_120,
        ) -> ()
    );
}

/// Original: `0x142263ce0` (137 bytes, `Throws::No`).
pub unsafe extern "win64" fn std_142263ce0(this: usize) {
    let rdi = rd::<u64>(this + 0x8);
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
    }
    wr::<u32>(exe::ptr::<u32>(0x1_43a0_014c) as usize, 0);
    wr::<u32>(exe::ptr::<u32>(0x1_43a0_0150) as usize, 1);
    wr::<u64>(tls0 + 0x6308, rdi);
    let mut edx: u32 = 0;
    let mut r8d: u32 = 1;
    if rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
        edx = rd::<u32>(exe::ptr::<u32>(0x1_43a0_014c) as usize);
        r8d = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0150) as usize);
    }
    let rcx_arg = rd::<usize>(tls0 + 0x6308);
    call_va!(0x1_421f_0110, (rcx_arg, edx, r8d) -> ());
}

/// Original: `0x142264340` (50 bytes, `Throws::No`).
pub unsafe extern "win64" fn std_142264340(this: usize) -> usize {
    let inner = rd::<usize>(this + 0x8);
    let mgr = rd::<usize>(inner + 0x1a8);
    call_va!(0x1_4226_11b0, (mgr) -> ());
    let inner2 = rd::<usize>(this + 0x8);
    let mgr2 = rd::<usize>(inner2 + 0x1a8);
    call_va!(0x1_412f_7fb0, (mgr2.wrapping_add(0x38)) -> usize)
}

/// Original: `0x142264400` (200 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn std_142264400(this: usize, pass_data_ptr: usize) {
    let pass_data = rd::<u64>(pass_data_ptr);
    let flag = rd::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize);
    let m0 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1a8);
    call_va!(0x1_4226_2470, (m0, flag) -> ());
    let m1 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1a8);
    call_va!(0x1_4226_20f0, (m1) -> ());
    let m2 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1a8);
    call_va!(0x1_4226_2310, (m2) -> ());
    let m3 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1a8);
    let count = rd::<u32>(m3 + 0x20) as usize;
    let bytes = count * 12;
    let src = rd::<usize>(m3 + 0x28);
    let dst = rd::<usize>(m3 + 0x5f0);
    call_va!(0x1_4183_acbc, (dst, src, bytes) -> usize);
    let m4 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1a8);
    call_va!(0x1_4226_1da0, (m4) -> ());
    let m5 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1a8);
    call_va!(0x1_4225_e6c0, (m5.wrapping_add(0x2d0)) -> ());
    let b0 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1b0);
    call_va!(0x1_4222_e360, (b0) -> ());
    let b1 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1b0);
    call_va!(0x1_4222_dca0, (b1, pass_data) -> ());
}

/// Original: `0x1422648d0` (312 bytes, `Throws::No`).
pub unsafe extern "win64" fn std_1422648d0(_this: usize, pass_data_ptr: usize) {
    let pass_data = rd::<usize>(pass_data_ptr);
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
    }
    let mut clear_val = [0u64; 2];
    let diff_hash = fnv1a64(b"lightbuffer_diffuse");
    let diff_slot = find_pass_resource(pass_data, diff_hash);
    let diff_tex = rd::<usize>(diff_slot);
    call_va!(
        0x1_4206_edb0,
        (0usize, diff_tex, clear_val.as_mut_ptr() as usize) -> ()
    );

    clear_val = [0u64; 2];
    let spec_hash = fnv1a64(b"lightbuffer_specular");
    let spec_slot = find_pass_resource(pass_data, spec_hash);
    let spec_tex = rd::<usize>(spec_slot);
    call_va!(
        0x1_4206_edb0,
        (0usize, spec_tex, clear_val.as_mut_ptr() as usize) -> ()
    );
}

/// Original: `0x142264a50` (68 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_142264a50(this: usize) -> usize {
    let dst = call_va!(0x1_4183_4d64, (0x50usize) -> usize);
    wr::<u64>(dst, exe::ptr::<u8>(0x1_433f_3910) as u64);
    for off in (0x8..0x50).step_by(8) {
        wr::<u64>(dst + off, rd::<u64>(this + off));
    }
    dst
}

/// Original: `0x142264b50` (398 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_142264b50(this: usize) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    let epoch = rd::<i32>(tls0 + 0x74);
    let guard_addr = exe::ptr::<i32>(0x1_43cf_4504) as usize;
    if rd::<i32>(guard_addr) > epoch {
        call_va!(0x1_4183_4a78, (guard_addr) -> ());
        if rd::<i32>(guard_addr) == -1 {
            let cnt = rd::<i32>(exe::ptr::<i32>(0x1_43a0_01ac) as usize);
            wr::<u32>(exe::ptr::<u32>(0x1_4388_1ae8) as usize, 0);
            wr::<u8>(exe::ptr::<u8>(0x1_4388_1aec) as usize, 0);
            wr::<u64>(
                exe::ptr::<u64>(0x1_4388_1ae0) as usize,
                exe::ptr::<u8>(0x1_4307_95f8) as u64,
            );
            wr::<u16>(exe::ptr::<u16>(0x1_4388_1af0) as usize, 0);
            if cnt < 0x1000 {
                let arr = exe::ptr::<u64>(0x1_43a0_0290) as usize;
                let tweak = exe::ptr::<u8>(0x1_4388_1ae0) as u64;
                wr::<u64>(arr.wrapping_add(((cnt as isize) * 8) as usize), tweak);
                wr::<i32>(exe::ptr::<i32>(0x1_43a0_01ac) as usize, cnt + 1);
            }
            call_va!(
                0x1_4183_4d18,
                (exe::ptr::<u8>(0x1_4262_e9d0) as usize) -> ()
            );
            call_va!(0x1_4183_4a0c, (guard_addr) -> ());
        }
    }

    let r8d = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0150) as usize);
    let edx = rd::<u32>(exe::ptr::<u32>(0x1_43a0_014c) as usize);
    call_va!(0x1_421f_0110, (rd::<usize>(this + 0x8), edx, r8d) -> ());
    call_va!(
        0x1_422e_4a90,
        (rd::<usize>(rd::<usize>(this + 0x10) + 0x370)) -> ()
    );
    call_va!(0x1_4233_0af0, () -> ());

    let dil: u8 = if rd::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize) == 0
        && rd::<u8>(exe::ptr::<u8>(0x1_4388_1af0) as usize) == 0
    {
        call_va!(
            0x1_4232_6630,
            (rd::<usize>(rd::<usize>(this + 0x10) + 0x238)) -> ()
        );
        0
    } else {
        call_va!(
            0x1_4232_6630,
            (rd::<usize>(rd::<usize>(this + 0x10) + 0x238)) -> ()
        );
        call_va!(
            0x1_422a_ced0,
            (rd::<usize>(rd::<usize>(this + 0x10) + 0x310)) -> ()
        );
        1
    };

    call_va!(
        0x1_4222_cf20,
        (rd::<usize>(rd::<usize>(this + 0x10) + 0x1b0), dil) -> ()
    );
    let mut tmp16 = [0u64; 2];
    let ret_ptr = call_va!(
        0x1_422e_2950,
        (
            rd::<usize>(rd::<usize>(this + 0x10) + 0x370),
            tmp16.as_mut_ptr() as usize,
        ) -> usize
    );
    let v0 = rd::<u64>(ret_ptr);
    let v1 = rd::<u64>(ret_ptr + 0x8);
    let g_c890 = exe::ptr::<u64>(0x1_43c8_c890) as usize;
    wr::<u64>(g_c890, v0);
    wr::<u64>(g_c890 + 0x8, v1);
    if dil != 0 {
        tmp16[0] = v0;
        tmp16[1] = v1;
        call_va!(
            0x1_422a_cb20,
            (
                rd::<usize>(rd::<usize>(this + 0x10) + 0x310),
                tmp16.as_mut_ptr() as usize,
            ) -> ()
        );
    }
}

/// Original: `0x142264ed0` (46 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_142264ed0(this: usize) {
    let sub_1c8 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1c8);
    call_va!(0x1_423c_5a30, (sub_1c8) -> ());
    let sub_178 = rd::<usize>(rd::<usize>(this + 0x8) + 0x178);
    call_va!(0x1_423e_dc80, (sub_178) -> ());
}

/// Original: `0x142265440` (7 bytes, `Throws::No`).
pub unsafe extern "win64" fn std_142265440(_this: usize, a2: usize) {
    let ptr = rd::<usize>(a2);
    AtomicU32::from_ptr(ptr as *mut u32).fetch_add(1, Ordering::SeqCst);
}

/// Original: `0x1422654d0` (44 bytes, `Throws::No`).
pub unsafe extern "win64" fn std_1422654d0(_this: usize, a2: usize) {
    let rcx = rd::<usize>(exe::ptr::<usize>(0x1_43c8_c930) as usize);
    let r8 = rd::<usize>(a2);
    let should_dec = if rcx == 0 {
        true
    } else {
        let tag = rd::<u16>(rcx + 0x6) >> 3;
        if rd::<u16>(exe::ptr::<u16>(0x1_43c8_c938) as usize) != tag {
            true
        } else {
            rd::<u32>(rcx) == 0
        }
    };
    if should_dec {
        AtomicU32::from_ptr(r8 as *mut u32).fetch_sub(1, Ordering::SeqCst);
    }
}

/// Original: `0x142265590` (96 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_142265590(this: usize) {
    let g_2f8 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize);
    let g_930 = exe::ptr::<u8>(0x1_43c8_c930) as usize;
    call_va!(0x1_4248_5780, (g_2f8, g_930) -> ());
    let g_908 = exe::ptr::<u8>(0x1_43c8_c908) as usize;
    call_va!(0x1_4205_f870, (1u32, g_908) -> ());
    let sub_98 = rd::<usize>(rd::<usize>(this + 0x8) + 0x98);
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_c788) as usize) != 0 {
        let count = rd::<u32>(sub_98 + 0x8);
        let fmt = exe::ptr::<u8>(0x1_433f_d650) as usize;
        call_va!(0x1_423f_e030, (fmt, count) -> ());
    }
}

/// Original: `0x1422656a0` (198 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn std_1422656a0(this: usize) {
    let sub_b8 = rd::<usize>(rd::<usize>(this + 0x8) + 0xb8);
    let vec_10 = rd::<usize>(this + 0x10);
    let r9d = rd::<u32>(vec_10 + 0x8);
    let r8 = rd::<u64>(vec_10);
    let rdx_1f0 = exe::ptr::<u8>(0x1_43bb_d1f0) as usize;
    call_va!(
        0x1_422f_c260,
        (sub_b8, rdx_1f0, r8, r9d, 0x16u32, 0u64) -> ()
    );

    let sub_c0 = rd::<usize>(rd::<usize>(this + 0x8) + 0xc0);
    let vec_18 = rd::<usize>(this + 0x18);
    let len_18 = rd::<u32>(vec_18 + 0x8) as u64;
    let ptr_18 = rd::<u64>(vec_18);
    let mut span_18 = [len_18, ptr_18];
    call_va!(
        0x1_4239_5a90,
        (sub_c0, len_18, span_18.as_mut_ptr() as usize) -> ()
    );

    let sub_c8 = rd::<usize>(rd::<usize>(this + 0x8) + 0xc8);
    let vec_20 = rd::<usize>(this + 0x20);
    let len_20 = rd::<u32>(vec_20 + 0x8) as u64;
    let ptr_20 = rd::<u64>(vec_20);
    let mut span_20 = [len_20, ptr_20];
    call_va!(
        0x1_4230_1ff0,
        (sub_c8, span_20.as_mut_ptr() as usize) -> ()
    );

    let sub_d8 = rd::<usize>(rd::<usize>(this + 0x8) + 0xd8);
    call_va!(0x1_4237_9240, (sub_d8) -> ());
}

/// Original: `0x1422657f0` (168 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_1422657f0(this: usize) {
    let p_0 = rd::<usize>(rd::<usize>(this + 0x8));
    if p_0 != 0 {
        call_va!(0x1_423a_9860, (p_0) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4397_d85f) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(this + 0x10) + 0xe0);
        let vtbl = rd::<usize>(obj);
        call_va!(rd::<usize>(vtbl + 0x40), (obj) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_a1b8) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(this + 0x10) + 0xe0);
        let vtbl = rd::<usize>(obj);
        call_va!(rd::<usize>(vtbl + 0x48), (obj) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_9ca0) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(this + 0x10) + 0xe0);
        let vtbl = rd::<usize>(obj);
        call_va!(rd::<usize>(vtbl + 0x40), (obj) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_a238) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(this + 0x10) + 0xe8);
        let vtbl = rd::<usize>(obj);
        call_va!(rd::<usize>(vtbl + 0x48), (obj) -> ());
    }
    if rd::<u8>(exe::ptr::<u8>(0x1_4386_a568) as usize) != 0 {
        let obj = rd::<usize>(rd::<usize>(this + 0x10) + 0xe8);
        let vtbl = rd::<usize>(obj);
        call_va!(rd::<usize>(vtbl + 0x40), (obj) -> ());
    }
}

/// Original: `0x1422659b0` (97 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn std_1422659b0(this: usize) {
    let ctx = rd::<usize>(this + 0x8);
    if rd::<usize>(ctx + 0x1b8) != 0 {
        call_va!(0x1_421e_a370, (this) -> ());
    }
    let sub_178 = rd::<usize>(rd::<usize>(this + 0x8) + 0x178);
    call_va!(0x1_423e_b360, (sub_178) -> ());
    let sub_1b8 = rd::<usize>(rd::<usize>(this + 0x8) + 0x1b8);
    if sub_1b8 != 0 {
        call_va!(0x1_421e_a7c0, (sub_1b8) -> ());
    }
    let sub_220 = rd::<usize>(rd::<usize>(this + 0x8) + 0x220);
    call_va!(0x1_4234_8710, (sub_220) -> ());
}

/// Original: `0x142265aa0` (16 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_142265aa0(this: usize) {
    let sub_200 = rd::<usize>(rd::<usize>(this + 0x8) + 0x200);
    call_va!(0x1_4225_4990, (sub_200) -> ());
}

/// Original: `0x142265f20` (26 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_142265f20(this: usize, a2: usize) {
    let inner = rd::<usize>(this + 0x8);
    let rdx_val = rd::<u64>(a2);
    let r8_val = rd::<u64>(inner + 0x418);
    let rcx_val = rd::<usize>(inner + 0x1b8);
    call_va!(0x1_421e_ad60, (rcx_val, rdx_val, r8_val) -> ());
}

/// Original: `0x1422661f0` (300 bytes, `Throws::Caught`).
pub unsafe extern "win64" fn std_1422661f0(this: usize, pass_data_ptr: usize) {
    let pass_data = rd::<usize>(pass_data_ptr);
    let flag_ptr = rd::<usize>(this + 0x8);
    if rd::<u8>(flag_ptr) == 0
        || rd::<u8>(exe::ptr::<u8>(0x1_4397_d8be) as usize) != 0
        || rd::<u8>(exe::ptr::<u8>(0x1_4397_d8c2) as usize) != 0
        || rd::<u8>(exe::ptr::<u8>(0x1_4397_d8b7) as usize) != 0
    {
        return;
    }
    let r8d = rd::<u32>(exe::ptr::<u32>(0x1_43a0_0150) as usize);
    let edx = rd::<u32>(exe::ptr::<u32>(0x1_43a0_014c) as usize);
    let rcx_arg = rd::<usize>(this + 0x10);
    call_va!(0x1_421f_0110, (rcx_arg, edx, r8d) -> ());

    let diff_slot = find_pass_resource(pass_data, fnv1a64(b"lightbuffer_diffuse"));
    let spec_slot = find_pass_resource(pass_data, fnv1a64(b"lightbuffer_specular"));
    let rdx_val = rd::<u64>(diff_slot);
    let r8_val = rd::<u64>(spec_slot);
    call_va!(0x1_4234_7510, (0usize, rdx_val, r8_val) -> ());
}

/// Original: `0x1422663b0` (150 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_1422663b0(this: usize, pass_data_ptr: usize) {
    let pass_data = rd::<usize>(pass_data_ptr);
    if rd::<u8>(exe::ptr::<u8>(0x1_4397_d8aa) as usize) == 0 {
        return;
    }
    let sub_108 = rd::<usize>(rd::<usize>(this + 0x8) + 0x108);
    let diff_slot = find_pass_resource(pass_data, fnv1a64(b"lightbuffer_diffuse"));
    call_va!(0x1_4221_7f30, (sub_108, diff_slot) -> ());
}

/// Original: `0x142266570` (313 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_142266570(this: usize, pass_data_ptr: usize) {
    let pass_data = rd::<usize>(pass_data_ptr);
    let sub_120 = rd::<usize>(rd::<usize>(this + 0x8) + 0x120);
    wr::<u32>(sub_120 + 0x8, 0);
    wr::<u32>(sub_120 + 0x18, 0);
    wr::<u32>(sub_120 + 0x28, 0);
    wr::<u8>(sub_120 + 0x38, 1);
    wr::<u32>(sub_120 + 0x98, 0);
    let sub_328 = rd::<usize>(rd::<usize>(this + 0x8) + 0x328);
    if sub_328 != 0 {
        call_va!(0x1_422d_1b70, (sub_328) -> ());
    }
    let diff_slot = find_pass_resource(pass_data, fnv1a64(b"lightbuffer_diffuse"));
    let spec_slot = find_pass_resource(pass_data, fnv1a64(b"lightbuffer_specular"));
    call_va!(0x1_4226_8920, (0usize, diff_slot, spec_slot) -> ());
    let mut pair = [rd::<u64>(diff_slot), rd::<u64>(spec_slot)];
    call_va!(0x1_4206_6910, (pair.as_mut_ptr() as usize, 2u32) -> ());
}

/// Original: `0x1422666f0` (78 bytes, `Throws::Discard`).
pub unsafe extern "win64" fn std_1422666f0(this: usize) -> usize {
    let dst = call_va!(0x1_4183_4d64, (0x78usize) -> usize);
    wr::<u64>(dst, exe::ptr::<u8>(0x1_433f_42d0) as u64);
    for off in (0x8..0x78).step_by(8) {
        wr::<u64>(dst + off, rd::<u64>(this + off));
    }
    dst
}

/// Original: `0x1422667a0` (15 bytes, `Throws::No`).
pub unsafe extern "win64" fn std_1422667a0(this: usize, dealloc: u8) {
    if dealloc != 0 {
        call_va!(0x1_4183_4d30, (this, 0x78usize) -> ());
    }
}

/// Original: `0x142266940` (79 bytes, `Throws::No`).
pub unsafe extern "win64" fn std_142266940(this: usize) {
    let tls0 = rd::<usize>(exe::tls_pointer());
    if rd::<u8>(tls0 + 0x70) == 0 {
        call_va!(0x1_4183_49c0, () -> ());
    }
    let dst = rd::<usize>(this + 0x8);
    wr::<u64>(dst, rd::<u64>(tls0 + 0x6308));
    let g_b08 = exe::ptr::<u8>(0x1_43a3_3b08) as usize;
    call_va!(0x1_4229_d440, (g_b08) -> ());
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_rend_14224ebf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut ctrl1_o = [0u32, 0u32, 5u32, 0u32];
    let mut ctrl1_n = ctrl1_o;
    let mut ctrl2_o = [0u32, 0u32, 9u32, 0u32];
    let mut ctrl2_n = ctrl2_o;

    let sp1_o = [0x1111_2222u64, ctrl1_o.as_mut_ptr() as u64];
    let sp1_n = [0x1111_2222u64, ctrl1_n.as_mut_ptr() as u64];
    let sp2_o = [0x3333_4444u64, ctrl2_o.as_mut_ptr() as u64];
    let sp2_n = [0x3333_4444u64, ctrl2_n.as_mut_ptr() as u64];
    let a5 = 0x5555_6666_7777_8888u64;

    let mut out_o = [0u64; 6];
    let mut out_n = [0u64; 6];
    let r_o = f_orig(
        out_o.as_mut_ptr() as usize,
        0xabcd_ef01,
        sp1_o.as_ptr() as usize,
        sp2_o.as_ptr() as usize,
        (&a5 as *const u64) as usize,
    );
    let r_n = f_new(
        out_n.as_mut_ptr() as usize,
        0xabcd_ef01,
        sp1_n.as_ptr() as usize,
        sp2_n.as_ptr() as usize,
        (&a5 as *const u64) as usize,
    );
    crate::diff_eq!(r_o, out_o.as_mut_ptr() as usize, "ret_orig");
    crate::diff_eq!(r_n, out_n.as_mut_ptr() as usize, "ret_new");
    crate::diff_eq!(out_o[0], out_n[0], "field_0");
    crate::diff_eq!(out_o[1], out_n[1], "field_8");
    crate::diff_eq!(out_o[3], out_n[3], "field_18");
    crate::diff_eq!(out_o[5], out_n[5], "field_28");
    crate::diff_eq!(ctrl1_o, ctrl1_n, "ctrl1_refcnt");
    crate::diff_eq!(ctrl2_o, ctrl2_n, "ctrl2_refcnt");

    let sp_null = [0x9999u64, 0u64];
    let mut out_o2 = [0u64; 6];
    let mut out_n2 = [0u64; 6];
    f_orig(
        out_o2.as_mut_ptr() as usize,
        42,
        sp_null.as_ptr() as usize,
        sp_null.as_ptr() as usize,
        (&a5 as *const u64) as usize,
    );
    f_new(
        out_n2.as_mut_ptr() as usize,
        42,
        sp_null.as_ptr() as usize,
        sp_null.as_ptr() as usize,
        (&a5 as *const u64) as usize,
    );
    crate::diff_eq!(out_o2, out_n2, "null_ctrl");
    Ok(2)
}

pub unsafe fn diff_rend_14224f530(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut shared = [0x5au8; 0xc0];
    let addr = shared.as_mut_ptr() as usize;
    let r_o = f_orig(addr);
    let snap_o = shared;
    shared = [0x5au8; 0xc0];
    let r_n = f_new(addr);
    let snap_n = shared;
    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(snap_o, snap_n, "sdf_vol_ctor");
    Ok(1)
}

pub unsafe fn diff_rend_14224fab0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut shared = [0x5au8; 0xc0];
    let addr = shared.as_mut_ptr() as usize;
    let r_o = f_orig(0, addr);
    let snap_o = shared;
    shared = [0x5au8; 0xc0];
    let r_n = f_new(0, addr);
    let snap_n = shared;
    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(snap_o, snap_n, "sdf_vol_vfunc12");
    Ok(1)
}

pub unsafe fn diff_rend_142250930(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u8;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut s = [0u8; 0x38];
    let p = s.as_mut_ptr() as usize;
    wr::<u64>(p + 0x18, 0x1000);
    wr::<u32>(p + 0x20, 2);
    crate::diff_eq!(f_orig(p), f_new(p), "nonempty_first");

    wr::<u32>(p + 0x20, 0);
    wr::<u64>(p + 0x28, 0x2000);
    wr::<u32>(p + 0x30, 0);
    crate::diff_eq!(f_orig(p), f_new(p), "both_empty");

    wr::<u32>(p + 0x30, 1);
    crate::diff_eq!(f_orig(p), f_new(p), "nonempty_second");
    Ok(3)
}

pub unsafe fn diff_unknown_142250a20(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vec_o = [0u64; 4];
    let mut vec_n = [0u64; 4];
    let mut s_o = [0u8; 0x30];
    let mut s_n = [0u8; 0x30];
    let p_o = s_o.as_mut_ptr() as usize;
    let p_n = s_n.as_mut_ptr() as usize;
    wr::<u32>(p_o + 0x10, u32::MAX);
    wr::<u32>(p_o + 0x14, 0);
    wr::<u64>(p_o + 0x18, vec_o.as_mut_ptr() as u64);
    wr::<u32>(p_o + 0x20, 1);
    wr::<u32>(p_o + 0x24, 4);

    wr::<u32>(p_n + 0x10, u32::MAX);
    wr::<u32>(p_n + 0x14, 0);
    wr::<u64>(p_n + 0x18, vec_n.as_mut_ptr() as u64);
    wr::<u32>(p_n + 0x20, 1);
    wr::<u32>(p_n + 0x24, 4);

    f_orig(p_o, 0x1234_5678_9abc_def0);
    f_new(p_n, 0x1234_5678_9abc_def0);
    crate::diff_eq!(vec_o, vec_n, "pushed_elem");
    crate::diff_eq!(s_o[0x8..0x18], s_n[0x8..0x18], "lock_state");
    crate::diff_eq!(s_o[0x20..0x28], s_n[0x20..0x28], "len_cap");
    Ok(1)
}

pub unsafe fn diff_unknown_142250c30(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_7a0 = exe::ptr::<u8>(0x1_4386_87a0) as usize;
    let g_670 = exe::ptr::<u8>(0x1_4386_8670) as usize;
    let g_8a8 = exe::ptr::<u8>(0x1_4386_88a8) as usize;
    let saved_7a0 = rd::<u8>(g_7a0);
    let saved_670 = rd::<u8>(g_670);
    let saved_8a8 = rd::<u8>(g_8a8);
    wr::<u8>(g_7a0, 0);
    wr::<u8>(g_670, 0);
    wr::<u8>(g_8a8, 0);

    let mut buf_o = [0u8; 0x2a0];
    let mut buf_n = [0u8; 0x2a0];
    let p_o = buf_o.as_mut_ptr() as usize;
    let p_n = buf_n.as_mut_ptr() as usize;
    wr::<i32>(p_o + 0x298, 5);
    wr::<i32>(p_n + 0x298, 5);
    f_orig(p_o);
    f_new(p_n);

    wr::<u8>(g_7a0, saved_7a0);
    wr::<u8>(g_670, saved_670);
    wr::<u8>(g_8a8, saved_8a8);
    crate::diff_eq!(buf_o, buf_n, "mod6_step");
    Ok(1)
}

pub unsafe fn diff_unknown_14225fa00(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(u32) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);
    for v in [0u32, 1, 63, 64, 65, 128, 1025] {
        crate::diff_eq!(f_orig(v), f_new(v), "div_ceil_64");
    }
    Ok(7)
}

pub unsafe fn diff_unknown_1422611a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut words = [u64::MAX, u64::MAX];
    let mut obj = [0u8; 0x58];
    let addr = obj.as_mut_ptr() as usize;
    wr::<u64>(addr + 0x38, words.as_mut_ptr() as u64);
    wr::<u64>(addr + 0x48, 70);
    let r_o = f_orig(addr);
    let w_o = words;

    words = [u64::MAX, u64::MAX];
    let r_n = f_new(addr);
    let w_n = words;
    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(w_o, w_n, "cleared_words");
    Ok(1)
}

pub unsafe fn diff_unknown_142261ae0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_flag = exe::ptr::<u8>(0x1_4386_98a0) as usize;
    let saved = rd::<u8>(g_flag);
    wr::<u8>(g_flag, 0);

    let mut buf_o = [0u8; 0x2f8];
    let mut buf_n = [0u8; 0x2f8];
    wr::<u64>((buf_o.as_mut_ptr() as usize) + 0x2e8, 0x1122_3344_5566_7788);
    wr::<u64>((buf_n.as_mut_ptr() as usize) + 0x2e8, 0x1122_3344_5566_7788);
    f_orig(buf_o.as_mut_ptr() as usize);
    f_new(buf_n.as_mut_ptr() as usize);

    wr::<u8>(g_flag, saved);
    crate::diff_eq!(buf_o, buf_n, "zeroed_stats");
    Ok(1)
}

pub unsafe fn diff_unknown_142261b50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut words_o = [[u64::MAX; 2]; 7];
    let mut words_n = [[u64::MAX; 2]; 7];
    let mut obj_o = [0u8; 0x6d0];
    let mut obj_n = [0u8; 0x6d0];
    let p_o = obj_o.as_mut_ptr() as usize;
    let p_n = obj_n.as_mut_ptr() as usize;

    let offs = [
        (0x180usize, 0x190usize),
        (0x638, 0x648),
        (0x650, 0x660),
        (0x668, 0x678),
        (0x680, 0x690),
        (0x698, 0x6a8),
        (0x6b0, 0x6c0),
    ];
    for (i, &(po, co)) in offs.iter().enumerate() {
        wr::<u64>(p_o + po, words_o[i].as_mut_ptr() as u64);
        wr::<u32>(p_o + po + 0x8, 2);
        wr::<u32>(p_o + po + 0xc, 2);
        wr::<u64>(p_o + co, 70);

        wr::<u64>(p_n + po, words_n[i].as_mut_ptr() as u64);
        wr::<u32>(p_n + po + 0x8, 2);
        wr::<u32>(p_n + po + 0xc, 2);
        wr::<u64>(p_n + co, 70);
    }

    f_orig(p_o);
    f_new(p_n);
    crate::diff_eq!(words_o, words_n, "bitset_words");
    crate::diff_eq!(rd::<u64>(p_o + 0x6c0), rd::<u64>(p_n + 0x6c0), "copied_count");
    Ok(1)
}

pub unsafe fn diff_unknown_142261cf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut mask_o = [0b1010_0101u64];
    let mut mask_n = [0b1010_0101u64];
    let src = [10u8, 20, 30, 40, 50, 60, 70, 80];
    let mut dst_o = [0u8; 8];
    let mut dst_n = [0u8; 8];
    let mut obj_o = [0u8; 0x1c8];
    let mut obj_n = [0u8; 0x1c8];
    let p_o = obj_o.as_mut_ptr() as usize;
    let p_n = obj_n.as_mut_ptr() as usize;

    wr::<u32>(p_o + 0x20, 8);
    wr::<u64>(p_o + 0x180, mask_o.as_mut_ptr() as u64);
    wr::<u64>(p_o + 0x1a8, src.as_ptr() as u64);
    wr::<u64>(p_o + 0x1b8, dst_o.as_mut_ptr() as u64);

    wr::<u32>(p_n + 0x20, 8);
    wr::<u64>(p_n + 0x180, mask_n.as_mut_ptr() as u64);
    wr::<u64>(p_n + 0x1a8, src.as_ptr() as u64);
    wr::<u64>(p_n + 0x1b8, dst_n.as_mut_ptr() as u64);

    f_orig(p_o);
    f_new(p_n);
    crate::diff_eq!(mask_o, mask_n, "mask_cleared");
    crate::diff_eq!(dst_o, dst_n, "copied_bytes");
    Ok(1)
}

pub unsafe fn diff_unknown_1422622e0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let src = [1u8; 24];
    let mut dst = [0u8; 24];
    let mut obj = [0u8; 0x600];
    let p = obj.as_mut_ptr() as usize;
    wr::<u32>(p + 0x20, 2);
    wr::<u64>(p + 0x28, src.as_ptr() as u64);
    wr::<u64>(p + 0x5f0, dst.as_mut_ptr() as u64);

    let r_o = f_orig(p);
    let snap_o = dst;
    dst = [0u8; 24];
    let r_n = f_new(p);
    let snap_n = dst;
    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(snap_o, snap_n, "memcpy_12x");
    Ok(1)
}

pub unsafe fn diff_unknown_142262310(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mask = [0b101u64];
    let idx_map = [u32::MAX, 0, 0, u32::MAX];
    let info_table = [0u8; 72];
    let mut obj_o = [0u8; 0x250];
    let mut obj_n = [0u8; 0x250];
    let p_o = obj_o.as_mut_ptr() as usize;
    let p_n = obj_n.as_mut_ptr() as usize;

    wr::<u32>(p_o + 0x20, 4);
    wr::<u64>(p_o + 0x180, mask.as_ptr() as u64);
    wr::<u64>(p_o + 0x198, idx_map.as_ptr() as u64);
    wr::<u64>(p_o + 0x240, info_table.as_ptr() as u64);

    wr::<u32>(p_n + 0x20, 4);
    wr::<u64>(p_n + 0x180, mask.as_ptr() as u64);
    wr::<u64>(p_n + 0x198, idx_map.as_ptr() as u64);
    wr::<u64>(p_n + 0x240, info_table.as_ptr() as u64);

    f_orig(p_o);
    f_new(p_n);
    crate::diff_eq!(obj_o, obj_n, "skip_inactive_lods");
    Ok(1)
}

pub unsafe fn diff_unknown_142262d70(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, usize) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut buf_o = [0u64; 6];
    let mut buf_n = [0u64; 6];
    let mut obj_o = [0u8; 0x160];
    let mut obj_n = [0u8; 0x160];
    let p_o = obj_o.as_mut_ptr() as usize;
    let p_n = obj_n.as_mut_ptr() as usize;

    wr::<u64>(p_o + 0x140, buf_o.as_mut_ptr() as u64);
    wr::<u32>(p_o + 0x148, 1);
    wr::<u32>(p_o + 0x14c, 2);

    wr::<u64>(p_n + 0x140, buf_n.as_mut_ptr() as u64);
    wr::<u32>(p_n + 0x148, 1);
    wr::<u32>(p_n + 0x14c, 2);

    let payload = [0x1111_2222_3333_4444u64, 0x5555_6666_7777_8888u64];
    let r_o = f_orig(p_o, 0xdead_beef, payload.as_ptr() as usize);
    let r_n = f_new(p_n, 0xdead_beef, payload.as_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "ret_idx");
    crate::diff_eq!(rd::<u32>(p_o + 0x148), rd::<u32>(p_n + 0x148), "new_len");
    crate::diff_eq!(buf_o[3] as u32, buf_n[3] as u32, "slot_tag");
    crate::diff_eq!(buf_o[4..6], buf_n[4..6], "slot_payload");
    Ok(1)
}

pub unsafe fn diff_unknown_142263030(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u64) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut vec_o = [0u64; 4];
    let mut vec_n = [0u64; 4];
    let mut b_o = [[0u64; 2]; 4];
    let mut b_n = [[0u64; 2]; 4];
    let mut obj_o = [0u8; 0x6d0];
    let mut obj_n = [0u8; 0x6d0];
    let p_o = obj_o.as_mut_ptr() as usize;
    let p_n = obj_n.as_mut_ptr() as usize;

    wr::<u64>(p_o + 0x130, vec_o.as_mut_ptr() as u64);
    wr::<u32>(p_o + 0x138, 1);
    wr::<u32>(p_o + 0x13c, 4);

    wr::<u64>(p_n + 0x130, vec_n.as_mut_ptr() as u64);
    wr::<u32>(p_n + 0x138, 1);
    wr::<u32>(p_n + 0x13c, 4);

    for (i, &off) in [0x668usize, 0x680, 0x698, 0x6b0].iter().enumerate() {
        wr::<u64>(p_o + off, b_o[i].as_mut_ptr() as u64);
        wr::<u32>(p_o + off + 0x8, 1);
        wr::<u32>(p_o + off + 0xc, 2);
        wr::<u64>(p_o + off + 0x10, 5);

        wr::<u64>(p_n + off, b_n[i].as_mut_ptr() as u64);
        wr::<u32>(p_n + off + 0x8, 1);
        wr::<u32>(p_n + off + 0xc, 2);
        wr::<u64>(p_n + off + 0x10, 5);
    }

    let r_o = f_orig(p_o, 0x1234_5678, 0);
    let r_n = f_new(p_n, 0x1234_5678, 0);
    crate::diff_eq!(r_o, r_n, "ret_idx");
    crate::diff_eq!(vec_o, vec_n, "pushed_vec");
    crate::diff_eq!(b_o, b_n, "resized_bitsets");
    Ok(1)
}

pub unsafe fn diff_unknown_142263140(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32, u32);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut v0_o = [0u64; 4];
    let mut v1_o = [0u64; 4];
    let mut v0_n = [0u64; 4];
    let mut v1_n = [0u64; 4];
    let mut obj_o = [0u8; 0x458];
    let mut obj_n = [0u8; 0x458];
    let p_o = obj_o.as_mut_ptr() as usize;
    let p_n = obj_n.as_mut_ptr() as usize;

    wr::<u64>(p_o + 0x398, v0_o.as_mut_ptr() as u64);
    wr::<u32>(p_o + 0x3a0, 1);
    wr::<u32>(p_o + 0x3a4, 4);
    wr::<u64>(p_o + 0x440, v1_o.as_mut_ptr() as u64);
    wr::<u32>(p_o + 0x448, 2);
    wr::<u32>(p_o + 0x44c, 4);

    wr::<u64>(p_n + 0x398, v0_n.as_mut_ptr() as u64);
    wr::<u32>(p_n + 0x3a0, 1);
    wr::<u32>(p_n + 0x3a4, 4);
    wr::<u64>(p_n + 0x440, v1_n.as_mut_ptr() as u64);
    wr::<u32>(p_n + 0x448, 2);
    wr::<u32>(p_n + 0x44c, 4);

    f_orig(p_o, 100, 200);
    f_new(p_n, 100, 200);
    crate::diff_eq!(v0_o, v0_n, "vec_398");
    crate::diff_eq!(v1_o, v1_n, "vec_440");
    crate::diff_eq!(rd::<u32>(p_o + 0x3a0), rd::<u32>(p_n + 0x3a0), "len_3a0");
    crate::diff_eq!(rd::<u32>(p_o + 0x448), rd::<u32>(p_n + 0x448), "len_448");
    Ok(1)
}

pub unsafe fn diff_unknown_142263190(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u32) -> u32;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut fl0_o = [(10u32, 20u32)];
    let mut fl1_o = [(30u32, 40u32)];
    let mut fl0_n = fl0_o;
    let mut fl1_n = fl1_o;
    let mut obj_o = [0u8; 0x458];
    let mut obj_n = [0u8; 0x458];
    let p_o = obj_o.as_mut_ptr() as usize;
    let p_n = obj_n.as_mut_ptr() as usize;

    wr::<u64>(p_o + 0x398, fl0_o.as_mut_ptr() as u64);
    wr::<u32>(p_o + 0x3a0, 1);
    wr::<u32>(p_o + 0x3a4, 1);
    wr::<u64>(p_o + 0x440, fl1_o.as_mut_ptr() as u64);
    wr::<u32>(p_o + 0x448, 1);
    wr::<u32>(p_o + 0x44c, 1);

    wr::<u64>(p_n + 0x398, fl0_n.as_mut_ptr() as u64);
    wr::<u32>(p_n + 0x3a0, 1);
    wr::<u32>(p_n + 0x3a4, 1);
    wr::<u64>(p_n + 0x440, fl1_n.as_mut_ptr() as u64);
    wr::<u32>(p_n + 0x448, 1);
    wr::<u32>(p_n + 0x44c, 1);

    let r_o = f_orig(p_o, 4);
    let r_n = f_new(p_n, 4);
    crate::diff_eq!(r_o, r_n, "ret_offset");
    crate::diff_eq!(fl0_o, fl0_n, "freelist_0");
    crate::diff_eq!(fl1_o, fl1_n, "freelist_1");
    Ok(1)
}

pub unsafe fn diff_std_1422639d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut src = [0u8; 0x50];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(7).wrapping_add(3);
    }
    let p_o = f_orig(src.as_ptr() as usize);
    let p_n = f_new(src.as_ptr() as usize);
    let vtbl_o = rd::<u64>(p_o);
    let vtbl_n = rd::<u64>(p_n);
    let body_o = rd::<[u8; 0x40]>(p_o + 0x10);
    let body_n = rd::<[u8; 0x40]>(p_n + 0x10);
    call_va!(0x1_4183_4d30, (p_o, 0x50usize) -> ());
    call_va!(0x1_4183_4d30, (p_n, 0x50usize) -> ());
    crate::diff_eq!(vtbl_o, vtbl_n, "vftable");
    crate::diff_eq!(body_o, body_n, "copied_fields");
    Ok(1)
}

pub unsafe fn diff_std_142264340(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut words_38 = [u64::MAX, u64::MAX];
    let words_180 = [0u64, 0u64];
    let idx_map = [u32::MAX; 128];
    let mut mgr = [0u8; 0x5d8];
    let mgr_addr = mgr.as_mut_ptr() as usize;
    wr::<u64>(mgr_addr + 0x38, words_38.as_mut_ptr() as u64);
    wr::<u64>(mgr_addr + 0x48, 70);
    wr::<u64>(mgr_addr + 0x180, words_180.as_ptr() as u64);
    wr::<u64>(mgr_addr + 0x198, idx_map.as_ptr() as u64);

    let mut inner = [0u8; 0x1b8];
    wr::<u64>((inner.as_mut_ptr() as usize) + 0x1a8, mgr_addr as u64);
    let lambda = [0u64, inner.as_mut_ptr() as u64];

    let r_o = f_orig(lambda.as_ptr() as usize);
    let snap_o = words_38;
    words_38 = [u64::MAX, u64::MAX];
    let r_n = f_new(lambda.as_ptr() as usize);
    let snap_n = words_38;
    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(snap_o, snap_n, "cleared_38_bitset");
    Ok(1)
}

pub unsafe fn diff_std_142264a50(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut src = [0u8; 0x50];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(11).wrapping_add(5);
    }
    let p_o = f_orig(src.as_ptr() as usize);
    let p_n = f_new(src.as_ptr() as usize);
    let bytes_o = rd::<[u8; 0x50]>(p_o);
    let bytes_n = rd::<[u8; 0x50]>(p_n);
    call_va!(0x1_4183_4d30, (p_o, 0x50usize) -> ());
    call_va!(0x1_4183_4d30, (p_n, 0x50usize) -> ());
    crate::diff_eq!(bytes_o, bytes_n, "cloned_lambda_9");
    Ok(1)
}

pub unsafe fn diff_std_142265440(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut cnt_o = 41u32;
    let mut cnt_n = 41u32;
    let ptr_o = (&mut cnt_o as *mut u32) as u64;
    let ptr_n = (&mut cnt_n as *mut u32) as u64;
    f_orig(0, (&ptr_o as *const u64) as usize);
    f_new(0, (&ptr_n as *const u64) as usize);
    crate::diff_eq!(cnt_o, cnt_n, "atomic_inc");
    Ok(1)
}

pub unsafe fn diff_std_1422654d0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut cnt_o = 10u32;
    let mut cnt_n = 10u32;
    let ptr_o = (&mut cnt_o as *mut u32) as u64;
    let ptr_n = (&mut cnt_n as *mut u32) as u64;
    f_orig(0, (&ptr_o as *const u64) as usize);
    f_new(0, (&ptr_n as *const u64) as usize);
    crate::diff_eq!(cnt_o, cnt_n, "atomic_dec_cond");
    Ok(1)
}

pub unsafe fn diff_std_1422657f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let flags = [
        0x1_4397_d85fu64,
        0x1_4386_a1b8,
        0x1_4386_9ca0,
        0x1_4386_a238,
        0x1_4386_a568,
    ];
    let mut saved = [0u8; 5];
    for (i, &va) in flags.iter().enumerate() {
        let addr = exe::ptr::<u8>(va) as usize;
        saved[i] = rd::<u8>(addr);
        wr::<u8>(addr, 0);
    }

    let null_ptr = 0usize;
    let inner_10 = [0u8; 0xf0];
    let mut lambda_o = [0usize, (&null_ptr as *const usize) as usize, inner_10.as_ptr() as usize];
    let mut lambda_n = lambda_o;
    f_orig(lambda_o.as_mut_ptr() as usize);
    f_new(lambda_n.as_mut_ptr() as usize);

    for (i, &va) in flags.iter().enumerate() {
        wr::<u8>(exe::ptr::<u8>(va) as usize, saved[i]);
    }
    crate::diff_eq!(lambda_o, lambda_n, "noop_when_disabled");
    Ok(1)
}

pub unsafe fn diff_std_1422666f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut src = [0u8; 0x78];
    for (i, b) in src.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(13).wrapping_add(9);
    }
    let p_o = f_orig(src.as_ptr() as usize);
    let p_n = f_new(src.as_ptr() as usize);
    let bytes_o = rd::<[u8; 0x78]>(p_o);
    let bytes_n = rd::<[u8; 0x78]>(p_n);
    call_va!(0x1_4183_4d30, (p_o, 0x78usize) -> ());
    call_va!(0x1_4183_4d30, (p_n, 0x78usize) -> ());
    crate::diff_eq!(bytes_o, bytes_n, "cloned_lambda_44");
    Ok(1)
}

pub unsafe fn diff_std_1422667a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    f_orig(0x1000, 0);
    f_new(0x1000, 0);

    let p_o = call_va!(0x1_4183_4d64, (0x78usize) -> usize);
    let p_n = call_va!(0x1_4183_4d64, (0x78usize) -> usize);
    f_orig(p_o, 1);
    f_new(p_n, 1);
    Ok(2)
}

pub unsafe fn diff_rend_14224e6e0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_39af80(a1: u64, a2: u64) {
        TRACE.fetch_add(a1 ^ a2.rotate_left(9), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_255e30(rdi: u64, rdx: u64, r8: usize) -> u64 {
        let v = rd::<u64>(r8);
        TRACE.fetch_add(rdi ^ rdx ^ v.rotate_left(17), Ordering::SeqCst);
        0x1357_2468
    }

    let _m1 = crate::difftest::Mock::new(0x1_4239_af80, fake_39af80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_5e30, fake_255e30 as *const ())?;

    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let g_ptr = exe::ptr::<u8>(0x1_4397_ff10);
    let snap = crate::difftest::Snapshot::take(g_ptr as *mut u8, 8);
    wr::<u64>(g_ptr as usize, 0x9999_aaaa);

    let obj = [0x1111u64, 0x2222u64, 0x3333u64, 0x4444u64];

    TRACE.store(0, Ordering::SeqCst);
    let r_o = f_orig(obj.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    let r_n = f_new(obj.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    snap.restore();

    crate::diff_eq!(r_o, r_n, "ret");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_rend_14224ec60(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_39af80(a1: u64, a2: u64) {
        TRACE.fetch_add(a1 ^ a2.rotate_left(5), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_255e30(rcx: u64, rdx: u64, r8: usize) {
        let v = rd::<u64>(r8);
        let obj_tag = rd::<u32>((rdx as usize) + 0x4) as u64;
        TRACE.fetch_add(rcx ^ obj_tag ^ v.rotate_left(11), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_420ed0(mgr: u64, obj: u64) {
        let obj_tag = rd::<u32>((obj as usize) + 0x4) as u64;
        TRACE.fetch_add(mgr ^ obj_tag.rotate_left(3), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4239_af80, fake_39af80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4225_5e30, fake_255e30 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4242_0ed0, fake_420ed0 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_82b8) as *mut u8, 8);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff00) as *mut u8, 4);
    let s3 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_8298) as *mut u8, 8);

    let mut target_o = [0u64; 18];
    let mut target_n = [0u64; 18];
    wr::<u32>((target_o.as_mut_ptr() as usize) + 0x4, 0x7777);
    wr::<u32>((target_n.as_mut_ptr() as usize) + 0x4, 0x7777);
    wr::<u32>((target_o.as_mut_ptr() as usize) + 0x80, 2);
    wr::<u32>((target_n.as_mut_ptr() as usize) + 0x80, 2);

    let mut streamer_o = [0u64; 77];
    let mut streamer_n = [0u64; 77];
    wr::<u32>((streamer_o.as_mut_ptr() as usize) + 0x258, 7);
    wr::<u32>((streamer_n.as_mut_ptr() as usize) + 0x258, 7);

    let this_o = [
        target_o.as_mut_ptr() as u64,
        0x1111u64,
        0x2222u64,
        0x3333u64,
        0x4444u64,
        0x5555u64,
    ];
    let this_n = [
        target_n.as_mut_ptr() as u64,
        0x1111u64,
        0x2222u64,
        0x3333u64,
        0x4444u64,
        0x5555u64,
    ];

    wr::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize, 0xaaaa_0000);
    wr::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize, 1);
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a0_8298) as usize,
        streamer_o.as_mut_ptr() as usize,
    );
    TRACE.store(0, Ordering::SeqCst);
    f_orig(this_o.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);
    let flag_o = rd::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize);

    wr::<u64>(exe::ptr::<u64>(0x1_43a0_82b8) as usize, 0xaaaa_0000);
    wr::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize, 1);
    wr::<usize>(
        exe::ptr::<usize>(0x1_43a0_8298) as usize,
        streamer_n.as_mut_ptr() as usize,
    );
    TRACE.store(0, Ordering::SeqCst);
    f_new(this_n.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);
    let flag_n = rd::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize);

    s1.restore();
    s2.restore();
    s3.restore();

    crate::diff_eq!(target_o, target_n, "target_state");
    crate::diff_eq!(
        rd::<u32>((streamer_o.as_ptr() as usize) + 0x258),
        rd::<u32>((streamer_n.as_ptr() as usize) + 0x258),
        "streamer_count"
    );
    crate::diff_eq!(flag_o, flag_n, "lock_flag");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_rend_14224f1e0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_init_str(dst: usize, view: usize, flag: u8) {
        let len = rd::<u64>(view + 0x8);
        wr::<u32>(dst + 0x4, len as u32);
        TRACE.fetch_add(len ^ (flag as u64) ^ (rd::<u32>(dst) as u64), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_load(res: usize, mf: usize) {
        let vtbl = rd::<u64>(mf);
        let p118 = rd::<u64>(mf + 0x118);
        let p120 = rd::<u64>(mf + 0x120);
        let p128 = rd::<u64>(mf + 0x128);
        TRACE.fetch_add((res as u64) ^ vtbl ^ p118 ^ p120 ^ p128, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_dtor_str(_s: usize) {
        TRACE.fetch_add(0x100, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_alloc_free(_alloc: usize, buf: usize, size: u64, flags: u16) {
        TRACE.fetch_add((buf as u64) ^ size ^ (flags as u64), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_9b50, fake_init_str as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4224_ed00, fake_load as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4240_ab10, fake_dtor_str as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let alloc_vtbl = [0usize, 0usize, fake_alloc_free as *const () as usize];
    let alloc_obj = [alloc_vtbl.as_ptr() as usize];
    let this_obj = [0x4444_5555usize];

    let mut data = [0u8; 0x58];
    let dp = data.as_mut_ptr() as usize;
    wr::<u32>(dp, 0x2000_0001);
    wr::<u32>(dp + 0x4, 12);
    wr::<u64>(dp + 0x20, 0xaaaa_1111);
    wr::<u64>(dp + 0x28, 0xbbbb_2222);
    wr::<usize>(dp + 0x30, 0xcccc_3333);
    wr::<u64>(dp + 0x38, 64);
    wr::<u16>(dp + 0x40, 3);
    wr::<usize>(dp + 0x48, alloc_obj.as_ptr() as usize);

    TRACE.store(0, Ordering::SeqCst);
    f_orig(this_obj.as_ptr() as usize, dp);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(this_obj.as_ptr() as usize, dp);
    let tr_n = TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_rend_14224f320(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_407c80(this: usize) {
        TRACE.fetch_add(rd::<u32>(this + 0x38) as u64, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_vcall120(_g: usize) -> u64 {
        0x7777_8888
    }
    unsafe extern "win64" fn fake_1df850(out: usize, inp: usize) -> usize {
        wr::<u64>(out, rd::<u64>(inp).wrapping_add(1));
        wr::<u64>(out + 0x8, rd::<u64>(inp + 0x8).wrapping_add(2));
        out
    }
    unsafe extern "win64" fn fake_47e5f0(
        g_2f0: usize,
        out_h: usize,
        path_in: usize,
        fn_ptr: usize,
        p_ptr: usize,
    ) {
        wr::<u64>(out_h, 0x99);
        let h = (g_2f0 as u64)
            ^ rd::<u64>(path_in)
            ^ rd::<u64>(path_in + 0x8)
            ^ rd::<u64>(fn_ptr)
            ^ rd::<u64>(fn_ptr + 0x8)
            ^ ((rd::<usize>(fn_ptr + 0x38) == fn_ptr) as u64)
            ^ rd::<u64>(p_ptr)
            ^ rd::<u64>(p_ptr + 0x8)
            ^ rd::<u64>(p_ptr + 0x10)
            ^ rd::<u64>(p_ptr + 0x18)
            ^ (rd::<u32>(p_ptr + 0x20) as u64)
            ^ ((rd::<u32>(p_ptr + 0x24) as u64) << 16)
            ^ rd::<u64>(p_ptr + 0x28)
            ^ rd::<u64>(p_ptr + 0x30)
            ^ rd::<u64>(p_ptr + 0x38)
            ^ (rd::<u8>(p_ptr + 0x40) as u64);
        TRACE.fetch_add(h, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_7c80, fake_407c80 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_f850, fake_1df850 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4247_e5f0, fake_47e5f0 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_8320) as *mut u8, 8);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_82f0) as *mut u8, 8);

    let mut vtbl_320 = [0usize; 40];
    vtbl_320[0x120 / 8] = fake_vcall120 as *const () as usize;
    let obj_320 = [vtbl_320.as_ptr() as usize];
    wr::<usize>(exe::ptr::<usize>(0x1_43a0_8320) as usize, obj_320.as_ptr() as usize);
    wr::<usize>(exe::ptr::<usize>(0x1_43a0_82f0) as usize, 0x5555_0000);

    let mut sub_30 = [0u8; 0x40];
    wr::<u64>((sub_30.as_mut_ptr() as usize) + 0x28, 0x1111_2222);
    wr::<u64>((sub_30.as_mut_ptr() as usize) + 0x30, 0x3333_4444);

    let mut this_buf = [0u8; 0x48];
    let tp = this_buf.as_mut_ptr() as usize;
    wr::<usize>(tp + 0x30, sub_30.as_ptr() as usize);
    wr::<i32>(tp + 0x38, 250);

    TRACE.store(0, Ordering::SeqCst);
    f_orig(tp);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(tp);
    let tr_n = TRACE.load(Ordering::SeqCst);

    s1.restore();
    s2.restore();

    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_rend_14224f470(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static DIRECT_CALL: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_f320(this: usize) {
        DIRECT_CALL.store(this as u64, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4224_f320, fake_f320 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff00) as *mut u8, 4);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff08) as *mut u8, 8);

    // Case 1: lock_flag == 0 -> acquires and calls 0x14224f320
    wr::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize, 0);
    DIRECT_CALL.store(0, Ordering::SeqCst);
    f_orig(0x1234_5678);
    let d_o = DIRECT_CALL.load(Ordering::SeqCst);
    let f_o = rd::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize);

    wr::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize, 0);
    DIRECT_CALL.store(0, Ordering::SeqCst);
    f_new(0x1234_5678);
    let d_n = DIRECT_CALL.load(Ordering::SeqCst);
    let f_n = rd::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize);

    // Case 2: lock_flag == 1 -> pushes to streamer queue
    let mut vec_o = [0u64; 2];
    let mut vec_n = [0u64; 2];
    let mut str_o = [0u8; 0x30];
    let mut str_n = [0u8; 0x30];
    let po = str_o.as_mut_ptr() as usize;
    let pn = str_n.as_mut_ptr() as usize;
    wr::<u32>(po + 0x10, u32::MAX);
    wr::<u64>(po + 0x18, vec_o.as_mut_ptr() as u64);
    wr::<u32>(po + 0x20, 0);
    wr::<u32>(po + 0x24, 2);

    wr::<u32>(pn + 0x10, u32::MAX);
    wr::<u64>(pn + 0x18, vec_n.as_mut_ptr() as u64);
    wr::<u32>(pn + 0x20, 0);
    wr::<u32>(pn + 0x24, 2);

    wr::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize, 1);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_ff08) as usize, po);
    f_orig(0xdead_beef);

    wr::<u32>(exe::ptr::<u32>(0x1_4397_ff00) as usize, 1);
    wr::<usize>(exe::ptr::<usize>(0x1_4397_ff08) as usize, pn);
    f_new(0xdead_beef);

    s1.restore();
    s2.restore();

    crate::diff_eq!(d_o, d_n, "direct_call");
    crate::diff_eq!(f_o, f_n, "flag_after_direct");
    crate::diff_eq!(vec_o, vec_n, "queued_vec");
    crate::diff_eq!(str_o[0x8..0x18], str_n[0x8..0x18], "lock_state");
    crate::diff_eq!(str_o[0x20..0x28], str_n[0x20..0x28], "len_cap");
    Ok(2)
}

pub unsafe fn diff_unknown_142250970(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_unreg(mgr: usize, _this: usize) {
        TRACE.fetch_add(mgr as u64, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_rel_buf(vec: usize) {
        wr::<u64>(vec, 0);
        TRACE.fetch_add(10, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_d480, fake_unreg as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401d_e830, fake_rel_buf as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_8298) as *mut u8, 8);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff08) as *mut u8, 8);

    let mut obj_o = [0x5au8; 0x38];
    let mut obj_n = [0x5au8; 0x38];

    wr::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize, 0x7777_0000);
    wr::<u64>(exe::ptr::<u64>(0x1_4397_ff08) as usize, 0x9999);
    TRACE.store(0, Ordering::SeqCst);
    f_orig(obj_o.as_mut_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);
    let g_o = rd::<u64>(exe::ptr::<u64>(0x1_4397_ff08) as usize);

    wr::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize, 0x7777_0000);
    wr::<u64>(exe::ptr::<u64>(0x1_4397_ff08) as usize, 0x9999);
    TRACE.store(0, Ordering::SeqCst);
    f_new(obj_n.as_mut_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);
    let g_n = rd::<u64>(exe::ptr::<u64>(0x1_4397_ff08) as usize);

    s1.restore();
    s2.restore();

    crate::diff_eq!(obj_o, obj_n, "obj_dtor");
    crate::diff_eq!(g_o, g_n, "global_cleared");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_unknown_1422509c0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_reg(mgr: usize, _this: usize) {
        TRACE.fetch_add(mgr as u64, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4240_d5c0, fake_reg as *const ())?;

    type F = unsafe extern "win64" fn(usize) -> usize;
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_8298) as *mut u8, 8);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_ff08) as *mut u8, 8);

    let mut obj_o = [0x5au8; 0x38];
    let mut obj_n = [0x5au8; 0x38];

    wr::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize, 0x8888_0000);
    TRACE.store(0, Ordering::SeqCst);
    let r_o = f_orig(obj_o.as_mut_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);
    let g_o = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff08) as usize);

    wr::<usize>(exe::ptr::<usize>(0x1_43a0_8298) as usize, 0x8888_0000);
    TRACE.store(0, Ordering::SeqCst);
    let r_n = f_new(obj_n.as_mut_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);
    let g_n = rd::<usize>(exe::ptr::<usize>(0x1_4397_ff08) as usize);

    s1.restore();
    s2.restore();

    crate::diff_eq!(
        r_o == obj_o.as_mut_ptr() as usize,
        r_n == obj_n.as_mut_ptr() as usize,
        "ret"
    );
    crate::diff_eq!(
        g_o == obj_o.as_mut_ptr() as usize,
        g_n == obj_n.as_mut_ptr() as usize,
        "global_set"
    );
    crate::diff_eq!(obj_o, obj_n, "ctor_bytes");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_unknown_142263120(orig: usize, new: usize) -> Result<u32, String> {
    unsafe extern "win64" fn fake_25e6c0(sub: usize) {
        wr::<u64>(sub, rd::<u64>(sub).wrapping_add(0x1234_5678));
    }

    let _m1 = crate::difftest::Mock::new(0x1_4225_e6c0, fake_25e6c0 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut obj_o = [0u8; 0x2e0];
    let mut obj_n = [0u8; 0x2e0];
    f_orig(obj_o.as_mut_ptr() as usize);
    f_new(obj_n.as_mut_ptr() as usize);
    crate::diff_eq!(obj_o, obj_n, "forwarded_2d0");
    Ok(1)
}

pub unsafe fn diff_std_142263a60(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static INNER_PTR: AtomicU64 = AtomicU64::new(0);
    static CALL_HASH: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_f1b0(_rdi: usize) -> usize {
        INNER_PTR.load(Ordering::SeqCst) as usize
    }
    unsafe extern "win64" fn fake_5bb70(
        _rsi: usize,
        rdx_val: u64,
        r8d: u32,
        r9_val: u64,
        arg_r9: u64,
        arg_r11: u64,
        packed_wh: u64,
        f_dt: u32,
        f_124: usize,
        f_148: usize,
        f_120: usize,
    ) {
        let h = rdx_val
            ^ (r8d as u64)
            ^ r9_val.rotate_left(7)
            ^ arg_r9.rotate_left(13)
            ^ arg_r11.rotate_left(19)
            ^ packed_wh.rotate_left(29)
            ^ (f_dt as u64)
            ^ (((f_124 as u32) as u64) << 16)
            ^ (((f_148 as u32) as u64) << 32)
            ^ (((f_120 as u32) as u64) << 48);
        CALL_HASH.store(h, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_412f_f1b0, fake_f1b0 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_bb70, fake_5bb70 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let dims = [1920u32, 1080u32];
    let dims_holder = [dims.as_ptr() as usize];
    INNER_PTR.store(dims_holder.as_ptr() as u64, Ordering::SeqCst);

    let vel_res = [0xfeed_face_1234_5678u64];
    let mut pass_data = [0u8; 0x410];
    let pd = pass_data.as_mut_ptr() as usize;
    wr::<usize>(pd + 0x8, vel_res.as_ptr() as usize);
    wr::<u64>(pd + 0x208, fnv1a64(b"velocity_geometry_buffer"));
    let pd_ptr = [pd];

    let mut sub_30 = [0u8; 0x58];
    wr::<u64>((sub_30.as_mut_ptr() as usize) + 0x48, 0xaaaa_bbbb_cccc_dddd);

    let mut this_buf = [0u8; 0x58];
    let tp = this_buf.as_mut_ptr() as usize;
    wr::<usize>(tp + 0x40, sub_30.as_ptr() as usize);
    wr::<u64>(tp + 0x48, 0x1122_3344_5566_7788);

    CALL_HASH.store(0, Ordering::SeqCst);
    f_orig(tp, pd_ptr.as_ptr() as usize);
    let h_o = CALL_HASH.load(Ordering::SeqCst);

    CALL_HASH.store(0, Ordering::SeqCst);
    f_new(tp, pd_ptr.as_ptr() as usize);
    let h_n = CALL_HASH.load(Ordering::SeqCst);

    crate::diff_eq!(h_o, h_n, "ssaa_pass_args");
    Ok(1)
}

pub unsafe fn diff_std_142263ce0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CALL_REC: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_1f0110(rcx: usize, edx: u32, r8d: u32) {
        CALL_REC.store(
            (rcx as u64) ^ ((edx as u64) << 32) ^ ((r8d as u64) << 48),
            Ordering::SeqCst,
        );
    }

    let _m1 = crate::difftest::Mock::new(0x1_421f_0110, fake_1f0110 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let snap = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_014c) as *mut u8, 8);
    let orig_tls0 = rd::<usize>(exe::tls_pointer());
    let mut tls_o = vec![0u8; 0x6320];
    let mut tls_n = vec![0u8; 0x6320];
    tls_o[0x70] = 1;
    tls_n[0x70] = 1;

    let lambda = [0u64, 0x1234_5678u64];

    wr::<usize>(exe::tls_pointer(), tls_o.as_mut_ptr() as usize);
    CALL_REC.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize);
    let rec_o = CALL_REC.load(Ordering::SeqCst);
    let g_o = rd::<u64>(exe::ptr::<u64>(0x1_43a0_014c) as usize);

    wr::<usize>(exe::tls_pointer(), tls_n.as_mut_ptr() as usize);
    CALL_REC.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize);
    let rec_n = CALL_REC.load(Ordering::SeqCst);
    let g_n = rd::<u64>(exe::ptr::<u64>(0x1_43a0_014c) as usize);

    wr::<usize>(exe::tls_pointer(), orig_tls0);
    snap.restore();

    crate::diff_eq!(rec_o, rec_n, "call_rec");
    crate::diff_eq!(g_o, g_n, "globals_14c_150");
    crate::diff_eq!(
        rd::<u64>((tls_o.as_ptr() as usize) + 0x6308),
        rd::<u64>((tls_n.as_ptr() as usize) + 0x6308),
        "tls_6308"
    );
    Ok(1)
}

pub unsafe fn diff_std_142264400(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_262470(m: usize, flag: u8) {
        TRACE.fetch_add((m as u64) ^ (flag as u64) ^ 1, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_2620f0(m: usize) {
        TRACE.fetch_add((m as u64) ^ 2, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_262310(m: usize) {
        TRACE.fetch_add((m as u64) ^ 3, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_261da0(m: usize) {
        TRACE.fetch_add((m as u64) ^ 4, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_25e6c0(sub: usize) {
        TRACE.fetch_add((sub as u64) ^ 5, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_22e360(b: usize) {
        TRACE.fetch_add((b as u64) ^ 6, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_22dca0(b: usize, pd: u64) {
        TRACE.fetch_add((b as u64) ^ pd ^ 7, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4226_2470, fake_262470 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4226_20f0, fake_2620f0 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4226_2310, fake_262310 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4226_1da0, fake_261da0 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_4225_e6c0, fake_25e6c0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4222_e360, fake_22e360 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_4222_dca0, fake_22dca0 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let src = [0x33u8; 24];
    let mut dst_o = [0u8; 24];
    let mut dst_n = [0u8; 24];
    let mut mgr = [0u8; 0x600];
    let mp = mgr.as_mut_ptr() as usize;
    wr::<u32>(mp + 0x20, 2);
    wr::<usize>(mp + 0x28, src.as_ptr() as usize);

    let mut inner = [0u8; 0x1c0];
    let ip = inner.as_mut_ptr() as usize;
    wr::<usize>(ip + 0x1a8, mp);
    wr::<usize>(ip + 0x1b0, 0x7777_0000);

    let lambda = [0usize, ip];
    let pass_data_ptr = [0x9999_8888u64];

    wr::<usize>(mp + 0x5f0, dst_o.as_mut_ptr() as usize);
    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize, pass_data_ptr.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    wr::<usize>(mp + 0x5f0, dst_n.as_mut_ptr() as usize);
    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize, pass_data_ptr.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(dst_o, dst_n, "memcpy_dst");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_1422648d0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static CLEAR_TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_clear(_ctx: usize, tex: usize, clear_ptr: usize) {
        let v0 = rd::<u64>(clear_ptr);
        let v1 = rd::<u64>(clear_ptr + 0x8);
        CLEAR_TRACE.fetch_add((tex as u64) ^ v0 ^ v1 ^ 0x100, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4206_edb0, fake_clear as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let orig_tls0 = rd::<usize>(exe::tls_pointer());
    let mut tls_buf = [0u8; 0x80];
    tls_buf[0x70] = 1;
    wr::<usize>(exe::tls_pointer(), tls_buf.as_mut_ptr() as usize);

    let diff_slot = [0x1111_2222usize];
    let spec_slot = [0x3333_4444usize];
    let mut pass_data = [0u8; 0x410];
    let pd = pass_data.as_mut_ptr() as usize;
    wr::<usize>(pd + 0x8, diff_slot.as_ptr() as usize);
    wr::<usize>(pd + 0x10, spec_slot.as_ptr() as usize);
    wr::<u64>(pd + 0x208, fnv1a64(b"lightbuffer_diffuse"));
    wr::<u64>(pd + 0x210, fnv1a64(b"lightbuffer_specular"));
    let pd_ptr = [pd];

    CLEAR_TRACE.store(0, Ordering::SeqCst);
    f_orig(0, pd_ptr.as_ptr() as usize);
    let tr_o = CLEAR_TRACE.load(Ordering::SeqCst);

    CLEAR_TRACE.store(0, Ordering::SeqCst);
    f_new(0, pd_ptr.as_ptr() as usize);
    let tr_n = CLEAR_TRACE.load(Ordering::SeqCst);

    wr::<usize>(exe::tls_pointer(), orig_tls0);

    crate::diff_eq!(tr_o, tr_n, "clear_trace");
    Ok(1)
}

pub unsafe fn diff_std_142264b50(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_1f0110(rcx: usize, edx: u32, r8d: u32) {
        TRACE.fetch_add((rcx as u64) ^ (edx as u64) ^ ((r8d as u64) << 16), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_2e4a90(rcx: usize) {
        TRACE.fetch_add((rcx as u64) ^ 0x10, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_330af0() {
        TRACE.fetch_add(0x20, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_326630(rcx: usize) {
        TRACE.fetch_add((rcx as u64) ^ 0x30, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_2aced0(rcx: usize) {
        TRACE.fetch_add((rcx as u64) ^ 0x40, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_22cf20(rcx: usize, dil: u8) {
        TRACE.fetch_add((rcx as u64) ^ (dil as u64) ^ 0x50, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_2e2950(rcx: usize, out: usize) -> usize {
        wr::<u64>(out, (rcx as u64) ^ 0xaaaa_1111);
        wr::<u64>(out + 0x8, 0xbbbb_2222);
        out
    }
    unsafe extern "win64" fn fake_2acb20(rcx: usize, vec16: usize) {
        TRACE.fetch_add((rcx as u64) ^ rd::<u64>(vec16) ^ rd::<u64>(vec16 + 0x8), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421f_0110, fake_1f0110 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_422e_4a90, fake_2e4a90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4233_0af0, fake_330af0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4232_6630, fake_326630 as *const ())?;
    let _m5 = crate::difftest::Mock::new(0x1_422a_ced0, fake_2aced0 as *const ())?;
    let _m6 = crate::difftest::Mock::new(0x1_4222_cf20, fake_22cf20 as *const ())?;
    let _m7 = crate::difftest::Mock::new(0x1_422e_2950, fake_2e2950 as *const ())?;
    let _m8 = crate::difftest::Mock::new(0x1_422a_cb20, fake_2acb20 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let s_guard = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43cf_4504) as *mut u8, 4);
    let s_d8ca = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_d8ca) as *mut u8, 1);
    let s_c890 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43c8_c890) as *mut u8, 16);

    let orig_tls0 = rd::<usize>(exe::tls_pointer());
    let mut tls_buf = [0u8; 0x80];
    wr::<i32>((tls_buf.as_mut_ptr() as usize) + 0x74, 0);
    wr::<usize>(exe::tls_pointer(), tls_buf.as_mut_ptr() as usize);
    wr::<i32>(exe::ptr::<i32>(0x1_43cf_4504) as usize, i32::MIN);
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8ca) as usize, 1);

    let mut inner = [0u8; 0x380];
    let ip = inner.as_mut_ptr() as usize;
    wr::<usize>(ip + 0x1b0, 0x1000);
    wr::<usize>(ip + 0x238, 0x2000);
    wr::<usize>(ip + 0x310, 0x3000);
    wr::<usize>(ip + 0x370, 0x4000);
    let lambda = [0usize, 0x5000usize, ip];

    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);
    let c890_o = rd::<[u64; 2]>(exe::ptr::<u64>(0x1_43c8_c890) as usize);

    wr::<u64>(exe::ptr::<u64>(0x1_43c8_c890) as usize, 0);
    wr::<u64>((exe::ptr::<u64>(0x1_43c8_c890) as usize) + 0x8, 0);
    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);
    let c890_n = rd::<[u64; 2]>(exe::ptr::<u64>(0x1_43c8_c890) as usize);

    wr::<usize>(exe::tls_pointer(), orig_tls0);
    s_guard.restore();
    s_d8ca.restore();
    s_c890.restore();

    crate::diff_eq!(c890_o, c890_n, "g_c890");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_142264ed0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_3c5a30(rcx: usize) {
        TRACE.fetch_add((rcx as u64) ^ 0x11, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_3edc80(rcx: usize) {
        TRACE.fetch_add((rcx as u64) ^ 0x22, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_423c_5a30, fake_3c5a30 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_423e_dc80, fake_3edc80 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut inner = [0u8; 0x1d8];
    let ip = inner.as_mut_ptr() as usize;
    wr::<usize>(ip + 0x178, 0x1111_2222);
    wr::<usize>(ip + 0x1c8, 0x3333_4444);
    let lambda = [0usize, ip];

    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_142265590(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_485780(rcx: usize, rdx: usize) {
        TRACE.fetch_add((rcx as u64) ^ (rdx as u64), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_5f870(rcx: u32, rdx: usize) {
        TRACE.fetch_add((rcx as u64) ^ (rdx as u64), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_3fe030(fmt: usize, count: u32) {
        TRACE.fetch_add((fmt as u64) ^ ((count as u64) << 16), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_5780, fake_485780 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4205_f870, fake_5f870 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_423f_e030, fake_3fe030 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let s1 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_43a0_82f8) as *mut u8, 8);
    let s2 = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4386_c788) as *mut u8, 1);
    wr::<usize>(exe::ptr::<usize>(0x1_43a0_82f8) as usize, 0x7777_0000);
    wr::<u8>(exe::ptr::<u8>(0x1_4386_c788) as usize, 1);

    let sub_98 = [0u32, 0u32, 42u32];
    let mut inner = [0u8; 0xa8];
    wr::<usize>((inner.as_mut_ptr() as usize) + 0x98, sub_98.as_ptr() as usize);
    let lambda = [0usize, inner.as_ptr() as usize];

    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    s1.restore();
    s2.restore();

    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_1422656a0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_2fc260(
        rcx: usize,
        rdx: usize,
        r8: u64,
        r9d: u32,
        a5: u32,
        a6: u64,
    ) {
        let h = (rcx as u64)
            ^ (rdx as u64)
            ^ r8
            ^ ((r9d as u64) << 16)
            ^ ((a5 as u64) << 32)
            ^ a6;
        TRACE.fetch_add(h, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_395a90(rcx: usize, rdx: u64, span_ptr: usize) {
        let h = (rcx as u64) ^ rdx ^ rd::<u64>(span_ptr) ^ rd::<u64>(span_ptr + 0x8);
        TRACE.fetch_add(h, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_301ff0(rcx: usize, span_ptr: usize) {
        let h = (rcx as u64) ^ rd::<u64>(span_ptr) ^ rd::<u64>(span_ptr + 0x8);
        TRACE.fetch_add(h, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_379240(rcx: usize) {
        TRACE.fetch_add(rcx as u64, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_422f_c260, fake_2fc260 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4239_5a90, fake_395a90 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4230_1ff0, fake_301ff0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4237_9240, fake_379240 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut inner = [0u8; 0xe8];
    let ip = inner.as_mut_ptr() as usize;
    wr::<usize>(ip + 0xb8, 0x1000);
    wr::<usize>(ip + 0xc0, 0x2000);
    wr::<usize>(ip + 0xc8, 0x3000);
    wr::<usize>(ip + 0xd8, 0x4000);

    let vec_10 = [0xaaaa_1111u64, 5u64];
    let vec_18 = [0xbbbb_2222u64, 7u64];
    let vec_20 = [0xcccc_3333u64, 9u64];
    let lambda = [
        0usize,
        ip,
        vec_10.as_ptr() as usize,
        vec_18.as_ptr() as usize,
        vec_20.as_ptr() as usize,
    ];

    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_1422659b0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_1ea370(_this: usize) {
        TRACE.fetch_add(1, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_3eb360(rcx: usize) {
        TRACE.fetch_add((rcx as u64) ^ 0x10, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_1ea7c0(rcx: usize) {
        TRACE.fetch_add((rcx as u64) ^ 0x20, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_348710(rcx: usize) {
        TRACE.fetch_add((rcx as u64) ^ 0x30, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421e_a370, fake_1ea370 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_423e_b360, fake_3eb360 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_421e_a7c0, fake_1ea7c0 as *const ())?;
    let _m4 = crate::difftest::Mock::new(0x1_4234_8710, fake_348710 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    for b8_val in [0usize, 0x2222usize] {
        let mut inner = [0u8; 0x230];
        let ip = inner.as_mut_ptr() as usize;
        wr::<usize>(ip + 0x178, 0x1111);
        wr::<usize>(ip + 0x1b8, b8_val);
        wr::<usize>(ip + 0x220, 0x3333);
        let lambda = [0usize, ip];

        TRACE.store(0, Ordering::SeqCst);
        f_orig(lambda.as_ptr() as usize);
        let tr_o = TRACE.load(Ordering::SeqCst);

        TRACE.store(0, Ordering::SeqCst);
        f_new(lambda.as_ptr() as usize);
        let tr_n = TRACE.load(Ordering::SeqCst);

        crate::diff_eq!(tr_o, tr_n, "trace");
    }
    Ok(2)
}

pub unsafe fn diff_std_142265aa0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static ARG: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_254990(rcx: usize) {
        ARG.store(rcx as u64, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4225_4990, fake_254990 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut inner = [0u8; 0x210];
    wr::<usize>((inner.as_mut_ptr() as usize) + 0x200, 0xdead_beef);
    let lambda = [0usize, inner.as_ptr() as usize];

    ARG.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize);
    let a_o = ARG.load(Ordering::SeqCst);

    ARG.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize);
    let a_n = ARG.load(Ordering::SeqCst);

    crate::diff_eq!(a_o, a_n, "arg_200");
    Ok(1)
}

pub unsafe fn diff_std_142265f20(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_1ead60(rcx: usize, rdx: u64, r8: u64) {
        TRACE.store((rcx as u64) ^ rdx.rotate_left(7) ^ r8.rotate_left(19), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421e_ad60, fake_1ead60 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let mut inner = [0u8; 0x428];
    let ip = inner.as_mut_ptr() as usize;
    wr::<usize>(ip + 0x1b8, 0x1111_2222);
    wr::<u64>(ip + 0x418, 0x3333_4444);
    let lambda = [0usize, ip];
    let a2 = [0x5555_6666u64];

    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize, a2.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize, a2.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_1422661f0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_1f0110(rcx: usize, edx: u32, r8d: u32) {
        TRACE.fetch_add((rcx as u64) ^ (edx as u64) ^ ((r8d as u64) << 16), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_347510(_ctx: usize, rdx: u64, r8: u64) {
        TRACE.fetch_add(rdx ^ r8.rotate_left(13), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_421f_0110, fake_1f0110 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4234_7510, fake_347510 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let snap = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_d8b7) as *mut u8, 16);
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8b7) as usize, 0);
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8be) as usize, 0);
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8c2) as usize, 0);

    let diff_slot = [0xaaaa_1111u64];
    let spec_slot = [0xbbbb_2222u64];
    let mut pass_data = [0u8; 0x410];
    let pd = pass_data.as_mut_ptr() as usize;
    wr::<usize>(pd + 0x8, diff_slot.as_ptr() as usize);
    wr::<usize>(pd + 0x10, spec_slot.as_ptr() as usize);
    wr::<u64>(pd + 0x208, fnv1a64(b"lightbuffer_diffuse"));
    wr::<u64>(pd + 0x210, fnv1a64(b"lightbuffer_specular"));
    let pd_ptr = [pd];

    let flag_on = 1u8;
    let lambda = [0usize, (&flag_on as *const u8) as usize, 0x7777usize];

    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize, pd_ptr.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize, pd_ptr.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    snap.restore();

    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_1422663b0(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_217f30(rcx: usize, rdx: usize) {
        TRACE.store((rcx as u64) ^ (rdx as u64).rotate_left(9), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4221_7f30, fake_217f30 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let snap = crate::difftest::Snapshot::take(exe::ptr::<u8>(0x1_4397_d8aa) as *mut u8, 1);
    wr::<u8>(exe::ptr::<u8>(0x1_4397_d8aa) as usize, 1);

    let mut pass_data = [0u8; 0x410];
    let pd = pass_data.as_mut_ptr() as usize;
    wr::<usize>(pd + 0x8, 0x5555_6666);
    wr::<u64>(pd + 0x208, fnv1a64(b"lightbuffer_diffuse"));
    let pd_ptr = [pd];

    let mut inner = [0u8; 0x118];
    wr::<usize>((inner.as_mut_ptr() as usize) + 0x108, 0x1111_2222);
    let lambda = [0usize, inner.as_ptr() as usize];

    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda.as_ptr() as usize, pd_ptr.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda.as_ptr() as usize, pd_ptr.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    snap.restore();

    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_142266570(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static TRACE: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_2d1b70(rcx: usize) {
        TRACE.fetch_add(rcx as u64, Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_268920(_ctx: usize, diff_slot: usize, spec_slot: usize) {
        TRACE.fetch_add((diff_slot as u64) ^ (spec_slot as u64).rotate_left(7), Ordering::SeqCst);
    }
    unsafe extern "win64" fn fake_66910(pair_ptr: usize, count: u32) {
        let v0 = rd::<u64>(pair_ptr);
        let v1 = rd::<u64>(pair_ptr + 0x8);
        TRACE.fetch_add(v0 ^ v1.rotate_left(11) ^ (count as u64), Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_422d_1b70, fake_2d1b70 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4226_8920, fake_268920 as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4206_6910, fake_66910 as *const ())?;

    type F = unsafe extern "win64" fn(usize, usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let diff_slot = [0x1111_2222u64];
    let spec_slot = [0x3333_4444u64];
    let mut pass_data = [0u8; 0x410];
    let pd = pass_data.as_mut_ptr() as usize;
    wr::<usize>(pd + 0x8, diff_slot.as_ptr() as usize);
    wr::<usize>(pd + 0x10, spec_slot.as_ptr() as usize);
    wr::<u64>(pd + 0x208, fnv1a64(b"lightbuffer_diffuse"));
    wr::<u64>(pd + 0x210, fnv1a64(b"lightbuffer_specular"));
    let pd_ptr = [pd];

    let mut sub_120_o = [0x5au8; 0xa0];
    let mut sub_120_n = [0x5au8; 0xa0];
    let mut inner_o = [0u8; 0x338];
    let mut inner_n = [0u8; 0x338];
    wr::<usize>((inner_o.as_mut_ptr() as usize) + 0x120, sub_120_o.as_mut_ptr() as usize);
    wr::<usize>((inner_o.as_mut_ptr() as usize) + 0x328, 0x7777);
    wr::<usize>((inner_n.as_mut_ptr() as usize) + 0x120, sub_120_n.as_mut_ptr() as usize);
    wr::<usize>((inner_n.as_mut_ptr() as usize) + 0x328, 0x7777);

    let lambda_o = [0usize, inner_o.as_ptr() as usize];
    let lambda_n = [0usize, inner_n.as_ptr() as usize];

    TRACE.store(0, Ordering::SeqCst);
    f_orig(lambda_o.as_ptr() as usize, pd_ptr.as_ptr() as usize);
    let tr_o = TRACE.load(Ordering::SeqCst);

    TRACE.store(0, Ordering::SeqCst);
    f_new(lambda_n.as_ptr() as usize, pd_ptr.as_ptr() as usize);
    let tr_n = TRACE.load(Ordering::SeqCst);

    crate::diff_eq!(sub_120_o, sub_120_n, "sub_120");
    crate::diff_eq!(tr_o, tr_n, "trace");
    Ok(1)
}

pub unsafe fn diff_std_142266940(orig: usize, new: usize) -> Result<u32, String> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static ARG_REC: AtomicU64 = AtomicU64::new(0);

    unsafe extern "win64" fn fake_29d440(rcx: usize) {
        ARG_REC.store(rcx as u64, Ordering::SeqCst);
    }

    let _m1 = crate::difftest::Mock::new(0x1_4229_d440, fake_29d440 as *const ())?;

    type F = unsafe extern "win64" fn(usize);
    let f_orig: F = core::mem::transmute(orig);
    let f_new: F = core::mem::transmute(new);

    let orig_tls0 = rd::<usize>(exe::tls_pointer());
    let mut tls_buf = vec![0u8; 0x6320];
    tls_buf[0x70] = 1;
    wr::<u64>((tls_buf.as_mut_ptr() as usize) + 0x6308, 0xcafe_babe_1234_5678);
    wr::<usize>(exe::tls_pointer(), tls_buf.as_mut_ptr() as usize);

    let mut dst_o = 0u64;
    let mut dst_n = 0u64;
    let lambda_o = [0usize, (&mut dst_o as *mut u64) as usize];
    let lambda_n = [0usize, (&mut dst_n as *mut u64) as usize];

    ARG_REC.store(0, Ordering::SeqCst);
    f_orig(lambda_o.as_ptr() as usize);
    let a_o = ARG_REC.load(Ordering::SeqCst);

    ARG_REC.store(0, Ordering::SeqCst);
    f_new(lambda_n.as_ptr() as usize);
    let a_n = ARG_REC.load(Ordering::SeqCst);

    wr::<usize>(exe::tls_pointer(), orig_tls0);

    crate::diff_eq!(dst_o, dst_n, "dst_copied_tls");
    crate::diff_eq!(a_o, a_n, "arg_b08");
    Ok(1)
}

/// `unknown_141f3aae0`
/// Original: 0x141f3aae0 (818 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141f3aae0(param_1: usize, param_2: usize) -> u8 {
    let r11 = rd::<u64>(param_1).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            r11,
            (&span as *const [u64; 2]) as usize
        ) -> usize
    );

    let mut outer = [0u64; 11];
    outer[0] = r11;
    outer[1] = iter_init[0];
    outer[2] = iter_init[1];
    outer[3] = iter_init[2];
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let chunk_ptr = rd::<u64>((r11 as usize).wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = chunk_ptr;
        outer[5] = if chunk_ptr != 0 {
            rd::<u32>((r11 as usize).wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }

    let _ = exe::call_va!(0x1_410a_a0d0, (outer.as_mut_ptr() as usize) -> u64);
    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let idx = outer[6] as usize;
            let s0 = (outer[7] as usize).wrapping_add(idx.wrapping_mul(0x50));
            let s1 = (outer[8] as usize).wrapping_add(idx.wrapping_mul(0x40));
            let s2 = (outer[9] as usize).wrapping_add(idx.wrapping_mul(0x38));

            if rd::<u8>(s2 + 0x05) != 0
                && (rd::<u8>(s1 + 0x39) != 0 || (rd::<u8>(s1 + 0x38) & 0x3) != 0)
            {
                let aabb_min = rd::<[f32; 4]>(s0);
                let aabb_max = rd::<[f32; 4]>(s0 + 0x10);
                let (q, t) = if rd::<u8>(s0 + 0x40) == 0 {
                    (rd::<[f32; 4]>(s0 + 0x20), rd::<[f32; 4]>(s0 + 0x30))
                } else {
                    let s3 = (outer[10] as usize).wrapping_add(idx.wrapping_mul(0x20));
                    let raw_q = rd::<[u32; 4]>(s3);
                    let raw_t = rd::<[u32; 4]>(s3 + 0x10);
                    let q_inv = [
                        f32::from_bits(raw_q[0] ^ 0x8000_0000),
                        f32::from_bits(raw_q[1] ^ 0x8000_0000),
                        f32::from_bits(raw_q[2] ^ 0x8000_0000),
                        f32::from_bits(raw_q[3]),
                    ];
                    let neg_t = [
                        f32::from_bits(raw_t[0] ^ 0x8000_0000),
                        f32::from_bits(raw_t[1] ^ 0x8000_0000),
                        f32::from_bits(raw_t[2] ^ 0x8000_0000),
                        f32::from_bits(raw_t[3] ^ 0x8000_0000),
                    ];
                    let c1 = [
                        (neg_t[2] * q_inv[1] - neg_t[1] * q_inv[2]) * 2.0f32,
                        (neg_t[0] * q_inv[2] - neg_t[2] * q_inv[0]) * 2.0f32,
                        (neg_t[1] * q_inv[0] - neg_t[0] * q_inv[1]) * 2.0f32,
                        0.0f32,
                    ];
                    let c2 = [
                        c1[2] * q_inv[1] - c1[1] * q_inv[2],
                        c1[0] * q_inv[2] - c1[2] * q_inv[0],
                        c1[1] * q_inv[0] - c1[0] * q_inv[1],
                        0.0f32,
                    ];
                    let t_inv = [
                        c2[0] + (q_inv[3] * c1[0] + neg_t[0]),
                        c2[1] + (q_inv[3] * c1[1] + neg_t[1]),
                        c2[2] + (q_inv[3] * c1[2] + neg_t[2]),
                        c2[3] + (q_inv[3] * c1[3] + neg_t[3]),
                    ];
                    (q_inv, t_inv)
                };

                let p = rd::<[f32; 4]>(param_2);
                let c1 = [
                    (p[2] * q[1] - p[1] * q[2]) * 2.0f32,
                    (p[0] * q[2] - p[2] * q[0]) * 2.0f32,
                    (p[1] * q[0] - p[0] * q[1]) * 2.0f32,
                    0.0f32,
                ];
                let c2 = [
                    c1[2] * q[1] - c1[1] * q[2],
                    c1[0] * q[2] - c1[2] * q[0],
                    c1[1] * q[0] - c1[0] * q[1],
                    0.0f32,
                ];
                let local_p = [
                    (c2[0] + (q[3] * c1[0] + p[0])) + t[0],
                    (c2[1] + (q[3] * c1[1] + p[1])) + t[1],
                    (c2[2] + (q[3] * c1[2] + p[2])) + t[2],
                ];
                if local_p[0] <= aabb_max[0]
                    && local_p[1] <= aabb_max[1]
                    && local_p[2] <= aabb_max[2]
                    && aabb_min[0] <= local_p[0]
                    && aabb_min[1] <= local_p[1]
                    && aabb_min[2] <= local_p[2]
                {
                    return 1;
                }
            }
            outer[6] = outer[6].wrapping_add(1);
        }

        loop {
            outer[2] = outer[2].wrapping_add(2);
            if outer[2] == outer[3] {
                break;
            }
            let arch_id = rd::<u16>(outer[2] as usize) as usize;
            let base_d0 = outer[1] as usize;
            if rd::<u64>(base_d0.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                && rd::<u32>(base_d0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
            {
                let base_d8 = outer[0] as usize;
                let cp = rd::<u64>(base_d8.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_d8.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
                break;
            }
        }
        let _ = exe::call_va!(0x1_410a_a0d0, (outer.as_mut_ptr() as usize) -> u64);
    }
    0
}

/// `unknown_141165830`
/// Original: 0x141165830 (191 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141165830(param_1: usize) {
    let obj = rd::<usize>(param_1);
    let vtbl_20 = rd::<usize>(rd::<usize>(obj) + 0x20);
    let slot_a9c = rd::<u16>(exe::ptr::<u16>(0x1_43ce_2a9c) as usize) as usize;
    let idx = rd::<u16>(vtbl_20.wrapping_add(slot_a9c * 2)) as usize;
    let arr_08 = rd::<usize>(obj + 0x08);
    let cnt = rd::<u32>(arr_08.wrapping_add(idx * 16).wrapping_add(8)) as u64;
    let ptr = rd::<u64>(arr_08.wrapping_add(idx * 16));

    let slot_910 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d910) as usize) as usize;
    let tbl_580e0 = rd::<usize>(obj + 0x580e0);
    let rsi = rd::<usize>(tbl_580e0.wrapping_add(slot_910 * 8));

    let wid = crate::ns::r::thread::worker_index() as usize;
    let rdi = obj.wrapping_add(wid.wrapping_mul(0x188));
    let obj2 = rd::<usize>(param_1);
    let _ = crate::ns::r::thread::worker_index();

    let mut arg1 = [obj as u64, cnt, ptr];
    exe::call_va!(
        0x1_41fc_c240,
        (
            arg1.as_mut_ptr() as usize,
            rsi,
            rdi.wrapping_add(0x580f0),
            obj2
        )
    );
}

/// `kaim_141225bf0`
/// Original: 0x141225bf0 (446 bytes, Throws::No)
pub unsafe extern "win64" fn kaim_141225bf0(param_1: usize, param_2: usize) -> u32 {
    let mut flag = 0u8;
    let rc = exe::call_va!(
        0x1_416b_0e60,
        (
            param_1 + 0x30,
            rd::<u64>(param_1),
            param_2,
            (&mut flag as *mut u8) as usize
        ) -> u32
    );
    if rc != 0 {
        return rc;
    }
    if flag != 0 {
        let sub_10 = param_1 + 0x10;
        let busy = exe::call_va!(0x1_4121_7190, (sub_10) -> u8);
        if busy != 0 {
            return 2;
        }
        let p2_copy = rd::<[u64; 2]>(param_2);
        let rbx = rd::<usize>(param_1 + 0x08);
        let mut v0 = [0i64; 2];
        let mut v1 = [0i64; 2];
        let mut v2 = [0i64; 2];
        let mut out_37 = 0u32;
        exe::call_va!(
            0x1_416a_5130,
            (
                param_2,
                v0.as_mut_ptr() as usize,
                v1.as_mut_ptr() as usize,
                v2.as_mut_ptr() as usize
            )
        );
        let scale_obj = rd::<usize>(rbx);
        let scale = rd::<f32>(scale_obj);
        let pt0 = [(v0[0] as f32) * scale, (v0[1] as f32) * scale, 0.0f32];
        let pt1 = [(v1[0] as f32) * scale, (v1[1] as f32) * scale, 0.0f32];
        let pt2 = [(v2[0] as f32) * scale, (v2[1] as f32) * scale, 0.0f32];
        let mut zeros = [0.0f32; 3];
        let f1c = rd::<u32>(scale_obj + 0x1c) as u64;
        let f18 = rd::<u32>(scale_obj + 0x18) as u64;
        exe::call_va!(
            0x1_416e_57d0,
            (
                pt0.as_ptr() as usize,
                pt1.as_ptr() as usize,
                pt2.as_ptr() as usize,
                scale_obj + 0x04,
                f1c,
                scale_obj + 0x10,
                f18,
                zeros.as_mut_ptr() as usize,
                (&mut out_37 as *mut u32) as usize
            )
        );
        exe::call_va!(0x1_4121_7240, (sub_10, p2_copy.as_ptr() as usize));
    }
    0
}

/// `unknown_141eaf160`
/// Original: 0x141eaf160 (552 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141eaf160(param_1: usize) {
    let r11 = rd::<u64>(param_1).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            r11,
            (&span as *const [u64; 2]) as usize
        ) -> usize
    );

    let mut outer = [0u64; 10];
    outer[0] = r11;
    outer[1] = iter_init[0];
    outer[2] = iter_init[1];
    outer[3] = iter_init[2];
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>((r11 as usize).wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>((r11 as usize).wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }

    exe::call_va!(0x1_4103_8a90, (outer.as_mut_ptr() as usize));
    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let idx = outer[6] as usize;
            let p_flags = (outer[8] as usize).wrapping_add(idx * 2);
            let p_c = (outer[9] as usize).wrapping_add(idx * 12);
            let hi = rd::<u8>(p_flags + 1);
            if hi != 0 {
                let mut ent_id = rd::<u32>((outer[7] as usize).wrapping_add(idx * 24).wrapping_add(0x10));
                if (hi & 1) != 0 {
                    let mut b_val = rd::<u8>(p_flags) & 1;
                    let mut tag = 0x200u32;
                    exe::call_va!(
                        0x1_4103_9c40,
                        (
                            (&mut ent_id as *mut u32) as usize,
                            (&mut tag as *mut u32) as usize,
                            (&mut b_val as *mut u8) as usize
                        )
                    );
                }
                if (rd::<u8>(p_flags + 1) & 4) != 0 {
                    let mut b_val = (rd::<u8>(p_flags) >> 2) & 1;
                    let mut tag = 0x400u32;
                    exe::call_va!(
                        0x1_4103_9c40,
                        (
                            (&mut ent_id as *mut u32) as usize,
                            (&mut tag as *mut u32) as usize,
                            (&mut b_val as *mut u8) as usize
                        )
                    );
                }
                if (rd::<u8>(p_flags + 1) & 8) != 0 {
                    let mut b_val = (rd::<u8>(p_flags) >> 3) & 1;
                    let mut tag = 0x800u32;
                    exe::call_va!(
                        0x1_4103_9c40,
                        (
                            (&mut ent_id as *mut u32) as usize,
                            (&mut tag as *mut u32) as usize,
                            (&mut b_val as *mut u8) as usize
                        )
                    );
                }
                if (rd::<u8>(p_flags + 1) & 2) != 0 {
                    exe::call_va!(
                        0x1_4103_9dc0,
                        ((&mut ent_id as *mut u32) as usize, p_c + 8, p_c + 9)
                    );
                }
                wr::<u8>(p_flags + 1, 0);
            }
            outer[6] = outer[6].wrapping_add(1);
        }

        loop {
            outer[2] = outer[2].wrapping_add(2);
            if outer[2] == outer[3] {
                break;
            }
            let arch_id = rd::<u16>(outer[2] as usize) as usize;
            let base_1 = outer[1] as usize;
            if rd::<u64>(base_1.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                && rd::<u32>(base_1.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
            {
                let base_0 = outer[0] as usize;
                let cp = rd::<u64>(base_0.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
                break;
            }
        }
        exe::call_va!(0x1_4103_8a90, (outer.as_mut_ptr() as usize));
    }
}

/// `unknown_141eaf3a0`
/// Original: 0x141eaf3a0 (585 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141eaf3a0(param_1: usize) {
    let r11 = rd::<u64>(param_1).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_1 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            r11,
            (&span as *const [u64; 2]) as usize
        ) -> usize
    );

    let mut outer = [0u64; 11];
    outer[0] = r11;
    outer[1] = iter_init[0];
    outer[2] = iter_init[1];
    outer[3] = iter_init[2];
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>((r11 as usize).wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>((r11 as usize).wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }

    exe::call_va!(0x1_4103_8bd0, (outer.as_mut_ptr() as usize));
    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let idx = outer[6] as usize;
            let p_rdx = (outer[7] as usize).wrapping_add(idx * 24);
            let p_rdi = (outer[8] as usize).wrapping_add(idx * 4);
            let p_rsi = (outer[9] as usize).wrapping_add(idx * 40);
            let p_r10 = outer[10] as usize;

            let mask = rd::<u32>(p_rdi);
            if mask != 0 {
                let mut ent_id = rd::<u32>(p_rdx + 0x10);
                if (mask & 0x10) != 0 {
                    let want = rd::<i32>(p_rsi + 0x1c);
                    let mut found = 0u32;
                    if want != -0x6cc4_a422 && want != 0 {
                        let sub_8 = rd::<usize>(p_rdx + 0x08);
                        let arr = rd::<usize>(sub_8 + 0x170);
                        let cnt = rd::<u32>(sub_8 + 0x178) as usize;
                        let end = arr.wrapping_add(cnt * 24);
                        let mut cur = arr;
                        let mut k = 1u32;
                        while cur != end {
                            if rd::<i32>(cur) == want {
                                found = k;
                                break;
                            }
                            k = k.wrapping_add(1);
                            cur = cur.wrapping_add(24);
                        }
                    }
                    let mut found_val = found;
                    wr::<u8>(p_r10.wrapping_add(idx), found as u8);
                    exe::call_va!(
                        0x1_4103_94f0,
                        (
                            (&mut ent_id as *mut u32) as usize,
                            (&mut found_val as *mut u32) as usize
                        )
                    );
                }
                if (rd::<u8>(p_rdi) & 1) != 0 {
                    exe::call_va!(0x1_4103_9660, ((&mut ent_id as *mut u32) as usize, p_rsi + 0x10));
                }
                if (rd::<u8>(p_rdi) & 2) != 0 {
                    exe::call_va!(0x1_4103_97e0, ((&mut ent_id as *mut u32) as usize, p_rsi + 0x20));
                }
                if (rd::<u8>(p_rdi) & 4) != 0 {
                    exe::call_va!(0x1_4103_9950, ((&mut ent_id as *mut u32) as usize, p_rsi + 0x24));
                }
                if (rd::<u8>(p_rdi) & 8) != 0 {
                    exe::call_va!(0x1_4103_9ac0, ((&mut ent_id as *mut u32) as usize, p_rsi));
                }
                wr::<u32>(p_rdi, 0);
            }
            outer[6] = outer[6].wrapping_add(1);
        }

        loop {
            outer[2] = outer[2].wrapping_add(2);
            if outer[2] == outer[3] {
                break;
            }
            let arch_id = rd::<u16>(outer[2] as usize) as usize;
            let base_1 = outer[1] as usize;
            if rd::<u64>(base_1.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                && rd::<u32>(base_1.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
            {
                let base_0 = outer[0] as usize;
                let cp = rd::<u64>(base_0.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
                break;
            }
        }
        exe::call_va!(0x1_4103_8bd0, (outer.as_mut_ptr() as usize));
    }
}

/// `unknown_141ec6a90`
/// Original: 0x141ec6a90 (537 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141ec6a90(param_1: usize, param_2: u64, param_3: usize) {
    let idx = rd::<usize>(param_1 + 0x20);
    let p_flags = rd::<usize>(param_1).wrapping_add(idx * 2);
    let p_item = rd::<usize>(param_1 + 0x10).wrapping_add(idx * 32);
    let bit_ebp = (rd::<u32>(p_item + 0x1c) >> 2) & 1;
    let bit_r14 = (rd::<u32>(rd::<usize>(param_1 + 0x08).wrapping_add(idx * 4)) >> 1) & 1;

    let mut cnt_byte = rd::<u8>(p_flags);
    if cnt_byte != 0 {
        cnt_byte = cnt_byte.wrapping_sub(1);
        wr::<u8>(p_flags, cnt_byte);
        if cnt_byte == 0 {
            if (rd::<u8>(p_item + 0x1c) & 1) != 0 {
                exe::call_va!(0x1_403d_c790, (param_2, p_item + 0x10));
            }
            if bit_r14 != 0 && bit_ebp != 0 {
                let val_10 = rd::<u64>(p_item + 0x10);
                let len = rd::<u32>(param_3 + 0x08) as u64;
                let cap = rd::<u32>(param_3 + 0x0c) as u64;
                if len * 16 == cap * 16 {
                    let next_len = len + 1;
                    let mut new_cap = cap + (cap >> 1);
                    if new_cap < next_len {
                        new_cap = next_len;
                    }
                    exe::call_va!(0x1_4062_ee30, (param_3, new_cap));
                }
                let cur_len = rd::<u32>(param_3 + 0x08) as usize;
                let slot = rd::<usize>(param_3).wrapping_add(cur_len * 16);
                wr::<u64>(slot, val_10);
                wr::<u16>(slot + 8, 0x0100);
                wr::<u32>(param_3 + 0x08, (cur_len as u32).wrapping_add(1));
                wr::<u8>(p_flags + 1, 1);
                return;
            }
        }
    }
    if rd::<u8>(p_flags) != 0 || bit_ebp == 0 {
        return;
    }
    let prev_state = rd::<u8>(p_flags + 1);
    if bit_r14 == 0 {
        if prev_state == 2 {
            return;
        }
        let val_10 = rd::<u64>(p_item + 0x10);
        let len = rd::<u32>(param_3 + 0x08) as u64;
        let cap = rd::<u32>(param_3 + 0x0c) as u64;
        if len * 16 == cap * 16 {
            let next_len = len + 1;
            let mut new_cap = cap + (cap >> 1);
            if new_cap < next_len {
                new_cap = next_len;
            }
            exe::call_va!(0x1_4062_ee30, (param_3, new_cap));
        }
        let cur_len = rd::<u32>(param_3 + 0x08) as usize;
        let slot = rd::<usize>(param_3).wrapping_add(cur_len * 16);
        wr::<u64>(slot, val_10);
        wr::<u8>(slot + 8, prev_state);
        wr::<u8>(slot + 9, 2);
        wr::<u32>(param_3 + 0x08, (cur_len as u32).wrapping_add(1));
        wr::<u8>(p_flags + 1, 2);
    } else {
        if prev_state == 1 {
            return;
        }
        let val_10 = rd::<u64>(p_item + 0x10);
        let len = rd::<u32>(param_3 + 0x08) as u64;
        let cap = rd::<u32>(param_3 + 0x0c) as u64;
        if len * 16 == cap * 16 {
            let next_len = len + 1;
            let mut new_cap = cap + (cap >> 1);
            if new_cap < next_len {
                new_cap = next_len;
            }
            exe::call_va!(0x1_4062_ee30, (param_3, new_cap));
        }
        let cur_len = rd::<u32>(param_3 + 0x08) as usize;
        let slot = rd::<usize>(param_3).wrapping_add(cur_len * 16);
        wr::<u64>(slot, val_10);
        wr::<u8>(slot + 8, prev_state);
        wr::<u8>(slot + 9, 1);
        wr::<u32>(param_3 + 0x08, (cur_len as u32).wrapping_add(1));
        wr::<u8>(p_flags + 1, 1);
    }
}

/// `unknown_141ef1010`
/// Original: 0x141ef1010 (748 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141ef1010(param_1: usize) {
    let tbl_580e0 = rd::<usize>(param_1 + 0x580e0);
    let slot_eec = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ceec) as usize) as usize;
    let slot_bac = rd::<u16>(exe::ptr::<u16>(0x1_43cd_6bac) as usize) as usize;
    let slot_e88 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_ce88) as usize) as usize;
    let slot_658 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_5658) as usize) as usize;
    let slot_ba8 = rd::<u16>(exe::ptr::<u16>(0x1_43cd_6ba8) as usize) as usize;

    let r13 = rd::<usize>(tbl_580e0.wrapping_add(slot_eec * 8));
    let r15 = rd::<usize>(tbl_580e0.wrapping_add(slot_bac * 8));
    let r12 = rd::<usize>(tbl_580e0.wrapping_add(slot_e88 * 8));
    let rbp = rd::<usize>(rd::<usize>(tbl_580e0.wrapping_add(slot_658 * 8)) + 0x08);

    let vtbl_20 = rd::<usize>(rd::<usize>(param_1) + 0x20);
    let arch_idx = rd::<u16>(vtbl_20.wrapping_add(slot_ba8 * 2)) as usize;
    let arr_08 = rd::<usize>(param_1 + 0x08);
    let span_cnt = rd::<u32>(arr_08.wrapping_add(arch_idx * 16).wrapping_add(8)) as u64;
    let span_ptr = rd::<u64>(arr_08.wrapping_add(arch_idx * 16));

    let mut arg2 = [param_1 as u64, span_cnt, span_ptr];
    exe::call_va!(0x1_41ee_c2e0, (param_1, arg2.as_mut_ptr() as usize));

    let vec_begin = rd::<usize>(r15);
    let vec_cnt = rd::<u32>(r15 + 0x08) as usize;
    let vec_end = vec_begin.wrapping_add(vec_cnt * 24);
    let mut rbx = vec_begin;
    while rbx != vec_end {
        let sub_20 = rd::<usize>(r13 + 0x20);
        let idx = rd::<u32>(rbx + 0x08) as usize;
        if idx < (rd::<u32>(sub_20 + 0xc8) as usize) {
            let arr_c0 = rd::<usize>(sub_20 + 0xc0);
            if rd::<i32>(arr_c0.wrapping_add(idx * 32)) != -1 {
                let r10d = rd::<u32>(arr_c0.wrapping_add((idx & 0xff_ffff) * 32).wrapping_add(4));
                let fl = rd::<u8>(rbp + 1);
                if (fl & 4) != 0 {
                    let owner = rd::<usize>(rbp + 0x18);
                    wr::<u8>(rbp + 1, fl & !4);
                    wr::<u64>(rbp + 0x68, rd::<u64>(owner + 0x30));
                    wr::<usize>(owner + 0x30, rbp);
                }
                let p1 = exe::call_va!(0x1_4211_6d40, (rbp, 0xffff_d8f0u32 as u64) -> usize);
                let p2 = exe::call_va!(0x1_420f_c9b0, (rd::<u64>(p1), r10d as u64) -> usize);
                let dst = rd::<usize>(rbp + 0x08);
                wr::<[u64; 3]>(dst, rd::<[u64; 3]>(p2));
                wr::<usize>(rbp + 0x08, dst.wrapping_add(0x18));

                let n_sub = rd::<i32>(rbx + 0x10);
                for k in 0..n_sub.max(0) {
                    let off_idx = (rd::<i32>(rbx + 0x0c) + k) as isize as usize;
                    let sub_item = rd::<usize>(r15 + 0x10).wrapping_add(off_idx * 24);
                    exe::call_va!(0x1_41ee_ca80, (rbp, r12, sub_item));
                }

                let p_78 = rd::<usize>(rbp + 0x78);
                let saved_18 = rd::<u64>(p_78 + 0x18);
                wr::<u64>(p_78 + 0x18, rd::<u64>(rbx));
                let top_8 = rd::<usize>(rbp + 0x08);
                let lim_10 = rd::<usize>(rbp + 0x10);
                let def_p = exe::ptr::<u8>(0x1_4298_68d0) as usize;
                let end_ref = if lim_10 < top_8 { lim_10 } else { def_p };
                let base_30 = rd::<usize>(rbp + 0x30);
                let frame_off = ((n_sub as isize + 1) * 24) as usize;
                let frame_base = top_8.wrapping_sub(frame_off);
                let mut call_ctx = [frame_base as u64, 0u64, 0u64, 0u64, rbp as u64, saved_18];
                let cb_va = exe::ptr::<u8>(0x1_4211_6cd0) as usize;
                let ok = exe::call_va!(
                    0x1_4210_dbb0,
                    (
                        rbp,
                        cb_va,
                        call_ctx.as_mut_ptr() as usize,
                        frame_base.wrapping_sub(base_30),
                        end_ref.wrapping_sub(base_30)
                    ) -> i32
                );
                if ok != 0 {
                    let r9 = exe::call_va!(0x1_4107_0c00, (rbp, rbx) -> usize);
                    if r9 != 0 {
                        let tag = rd::<u32>(rbx + 0x08);
                        if tag != 0xffff_ffff {
                            let cnt_28 = rd::<u32>(r9 + 0x28) as usize;
                            for m in 0..cnt_28 {
                                let e_ptr = rd::<usize>(r9 + 0x20).wrapping_add(m * 0x38);
                                if rd::<u32>(e_ptr + 0x30) == tag {
                                    wr::<u64>(e_ptr, 0);
                                    wr::<u64>(e_ptr + 0x08, 0);
                                    wr::<u32>(e_ptr + 0x30, 0xffff_ffff);
                                    wr::<u16>(e_ptr + 0x10, 6);
                                    let tags_0 = rd::<usize>(r9);
                                    wr::<u16>(tags_0.wrapping_add(m * 2), 0);
                                }
                            }
                            let sub_20_b = rd::<usize>(r13 + 0x20);
                            let idx_b = tag as usize;
                            if idx_b < (rd::<u32>(sub_20_b + 0xc8) as usize) {
                                let arr_c0_b = rd::<usize>(sub_20_b + 0xc0);
                                if rd::<i32>(arr_c0_b.wrapping_add(idx_b * 32)) != -1 {
                                    exe::call_va!(0x1_4104_e810, (sub_20_b, rbp, idx_b as u64));
                                }
                            }
                        }
                    }
                    exe::call_va!(0x1_41ef_4620, (rbp));
                }
                let p_78_after = rd::<usize>(rbp + 0x78);
                wr::<u64>(p_78_after + 0x18, saved_18);
            }
        }
        rbx = rbx.wrapping_add(24);
    }
    wr::<u32>(r15 + 0x08, 0);
}

/// `unknown_141f444f0`
/// Original: 0x141f444f0 (511 bytes, Throws::Caught)
pub unsafe extern "win64" fn unknown_141f444f0(
    param_1: usize,
    param_2: u64,
    param_3: usize,
    param_4: usize,
) {
    if rd::<u8>(param_3) == 0 {
        return;
    }
    let mut p2_slot = param_2;
    let mut ctx_378 = [0u64; 106];
    exe::call_va!(
        0x1_41f4_6d20,
        (
            ctx_378.as_mut_ptr() as usize,
            (&mut p2_slot as *mut u64) as usize
        )
    );

    let keys_begin = rd::<usize>(param_4 + 0x30);
    let keys_cnt = rd::<u32>(param_4 + 0x38) as usize;
    let keys_end = keys_begin.wrapping_add(keys_cnt * 8);
    let mut key_ptr = keys_begin;
    while key_ptr != keys_end {
        let key = rd::<u64>(key_ptr);
        let prod = (key as u128).wrapping_mul(0xde5f_b9d2_6304_58e9u128);
        let hash = (prod as u64).wrapping_add((prod >> 64) as u64);
        let mask = rd::<u64>(param_4 + 0x18);
        let ctrl_base = rd::<usize>(param_4);
        let slots_base = rd::<usize>(param_4 + 0x08);
        let h2 = (hash as u8) & 0x7f;

        let mut grp_off = (hash >> 7) & mask;
        let mut stride = 0u64;
        let mut found_entry = 0usize;
        let mut found_ctrl = ctrl_base.wrapping_add(mask as usize);
        loop {
            grp_off &= mask;
            let ctrl_bytes = rd::<[u8; 16]>(ctrl_base.wrapping_add(grp_off as usize));
            let mut match_mask = 0u32;
            let mut empty_mask = 0u32;
            for b in 0..16 {
                if ctrl_bytes[b] == h2 {
                    match_mask |= 1u32 << b;
                }
                if ctrl_bytes[b] == 0x80 {
                    empty_mask |= 1u32 << b;
                }
            }
            let mut m = match_mask;
            let mut hit = false;
            while m != 0 {
                let bit = m.trailing_zeros() as u64;
                let idx = (grp_off.wrapping_add(bit)) & mask;
                let entry = slots_base.wrapping_add((idx as usize) * 24);
                if rd::<u64>(entry) == key {
                    found_entry = entry;
                    found_ctrl = ctrl_base.wrapping_add(idx as usize);
                    hit = true;
                    break;
                }
                m &= m - 1;
            }
            if hit || empty_mask != 0 {
                break;
            }
            stride = stride.wrapping_add(16);
            grp_off = grp_off.wrapping_add(stride);
        }

        if found_ctrl == ctrl_base.wrapping_add(mask as usize) {
            exe::call_va!(0x1_41f4_7100, ());
            return;
        }

        let sub_begin = rd::<usize>(found_entry + 0x08);
        let sub_cnt = rd::<u32>(found_entry + 0x10) as usize;
        let sub_end = sub_begin.wrapping_add(sub_cnt * 8);
        let mut sub_p = sub_begin;
        while sub_p != sub_end {
            let mut p1_copy = rd::<[u64; 3]>(param_1);
            let mut out_50 = [0u8; 0x40];
            exe::call_va!(
                0x1_410b_e550,
                (
                    out_50.as_mut_ptr() as usize,
                    rd::<u64>(sub_p),
                    p1_copy.as_mut_ptr() as usize
                )
            );
            if out_50[0x38] != 0 {
                let mut arg_90 = [0u8; 0x38];
                arg_90.copy_from_slice(&out_50[..0x38]);
                exe::call_va!(
                    0x1_41f4_50f0,
                    (
                        arg_90.as_mut_ptr() as usize,
                        p2_slot,
                        ctx_378.as_mut_ptr() as usize
                    )
                );
            }
            sub_p = sub_p.wrapping_add(8);
        }
        key_ptr = key_ptr.wrapping_add(8);
    }
}

/// `coregame_141f71fc0`
/// Original: 0x141f71fc0 (1345 bytes, Throws::Discard)
pub unsafe extern "win64" fn coregame_141f71fc0(param_1: usize, param_2: usize, param_3: usize) {
    let mut bitvec = [0u64; 3];
    let n_bits = rd::<u32>(param_1 + 0x88) as u64;
    let n_words = n_bits.wrapping_add(0x3f) >> 6;
    exe::call_va!(0x1_410e_f200, (bitvec.as_mut_ptr() as usize, n_words));
    let words_ptr = bitvec[0] as usize;
    if bitvec[2] < n_bits {
        let mut grow_desc = [n_words, bitvec[0], n_bits];
        exe::call_va!(
            0x1_40ff_f0a0,
            (
                grow_desc.as_mut_ptr() as usize,
                bitvec[2],
                n_bits.wrapping_sub(bitvec[2]),
                0u64
            )
        );
    }

    let arr_78 = rd::<usize>(param_1 + 0x78);
    let cnt_48 = rd::<usize>(param_1 + 0x48);
    let end_78 = arr_78.wrapping_add(cnt_48.wrapping_mul(0x28));
    let mut cur_78 = arr_78;
    while cur_78 != end_78 {
        let k0 = rd::<u64>(cur_78);
        let k1 = rd::<u64>(cur_78 + 8);
        let m50 = rd::<u64>(param_1 + 0x50) == k0 && rd::<u64>(param_1 + 0x58) == k1;
        let m28 = rd::<u64>(param_1 + 0x38) > 0
            && rd::<u64>(param_1 + 0x28) == k0
            && rd::<u64>(param_1 + 0x30) == k1;
        if !m50 && !m28 {
            break;
        }
        cur_78 = cur_78.wrapping_add(0x28);
    }
    loop {
        let end_now = rd::<usize>(param_1 + 0x78)
            .wrapping_add(rd::<usize>(param_1 + 0x48).wrapping_mul(0x28));
        if cur_78 == end_now {
            break;
        }
        if rd::<u16>(cur_78 + 0x10) == 5 {
            let bit = rd::<u64>(cur_78 + 0x18);
            let w_addr = words_ptr.wrapping_add(((bit >> 6) as usize) * 8);
            wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << (bit & 0x3f)));
        }
        cur_78 = cur_78.wrapping_add(0x28);
        while cur_78 != end_78 {
            let k0 = rd::<u64>(cur_78);
            let k1 = rd::<u64>(cur_78 + 8);
            let m50 = rd::<u64>(param_1 + 0x50) == k0 && rd::<u64>(param_1 + 0x58) == k1;
            let m28 = rd::<u64>(param_1 + 0x38) > 0
                && rd::<u64>(param_1 + 0x28) == k0
                && rd::<u64>(param_1 + 0x30) == k1;
            if !m50 && !m28 {
                break;
            }
            cur_78 = cur_78.wrapping_add(0x28);
        }
    }

    let r11 = rd::<u64>(param_3).wrapping_add(0x40);
    let span = rd::<[u64; 2]>(param_3 + 0x08);
    let mut iter_init = [0u64; 3];
    exe::call_va!(
        0x1_401e_7c10,
        (
            iter_init.as_mut_ptr() as usize,
            r11,
            (&span as *const [u64; 2]) as usize
        ) -> usize
    );

    let mut outer = [0u64; 9];
    outer[0] = r11;
    outer[1] = iter_init[0];
    outer[2] = iter_init[1];
    outer[3] = iter_init[2];
    if outer[2] != outer[3] {
        let arch_id = rd::<u16>(outer[2] as usize) as usize;
        let cp = rd::<u64>((r11 as usize).wrapping_add(arch_id * 8).wrapping_add(0x48));
        outer[4] = cp;
        outer[5] = if cp != 0 {
            rd::<u32>((r11 as usize).wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
        } else {
            0
        };
    }

    exe::call_va!(0x1_4106_f2b0, (outer.as_mut_ptr() as usize));
    while outer[6] < outer[5] {
        while outer[6] < outer[5] {
            let idx = outer[6] as usize;
            if rd::<u8>((outer[7] as usize).wrapping_add(idx)) != 0 {
                let comp = (outer[8] as usize).wrapping_add(idx.wrapping_mul(0x30));
                let mut e_ptr = rd::<usize>(comp + 0x20);
                let e_end = e_ptr.wrapping_add((rd::<u32>(comp + 0x28) as usize).wrapping_mul(0x38));
                while e_ptr != e_end {
                    if rd::<u16>(e_ptr + 0x10) == 5 {
                        let bit = rd::<u64>(e_ptr + 0x18);
                        let w_addr = words_ptr.wrapping_add(((bit >> 6) as usize) * 8);
                        wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << (bit & 0x3f)));
                    }
                    e_ptr = e_ptr.wrapping_add(0x38);
                }
            }
            outer[6] = outer[6].wrapping_add(1);
        }

        loop {
            outer[2] = outer[2].wrapping_add(2);
            if outer[2] == outer[3] {
                break;
            }
            let arch_id = rd::<u16>(outer[2] as usize) as usize;
            let base_1 = outer[1] as usize;
            if rd::<u64>(base_1.wrapping_add(arch_id * 8).wrapping_add(0x48)) != 0
                && rd::<u32>(base_1.wrapping_add(arch_id * 4).wrapping_add(0x10048)) != 0
            {
                let base_0 = outer[0] as usize;
                let cp = rd::<u64>(base_0.wrapping_add(arch_id * 8).wrapping_add(0x48));
                outer[4] = cp;
                outer[5] = if cp != 0 {
                    rd::<u32>(base_0.wrapping_add(arch_id * 4).wrapping_add(0x10048)) as u64
                } else {
                    0
                };
                outer[6] = 0;
                break;
            }
        }
        exe::call_va!(0x1_4106_f2b0, (outer.as_mut_ptr() as usize));
    }

    let p2_begin = rd::<usize>(param_2 + 0x10);
    let p2_cnt = rd::<u32>(param_2 + 0x18) as usize;
    let p2_end = p2_begin.wrapping_add(p2_cnt * 24);
    let mut p2_cur = p2_begin;
    while p2_cur != p2_end {
        if rd::<u16>(p2_cur) == 5 {
            let bit = rd::<u64>(p2_cur + 8);
            let w_addr = words_ptr.wrapping_add(((bit >> 6) as usize) * 8);
            wr::<u64>(w_addr, rd::<u64>(w_addr) | (1u64 << (bit & 0x3f)));
        }
        p2_cur = p2_cur.wrapping_add(24);
    }

    wr::<u32>(param_1 + 0xf8, 0);
    let words_final = bitvec[0] as usize;
    for w_idx in 0..n_words {
        let mut inv = !rd::<u64>(words_final.wrapping_add((w_idx as usize) * 8));
        let base_bit = w_idx * 64;
        while inv != 0 {
            let tz = inv.trailing_zeros() as u64;
            let bit_idx = base_bit + tz;
            if bit_idx < (rd::<u32>(param_1 + 0x88) as u64) {
                let slot = rd::<usize>(param_1 + 0x80).wrapping_add((bit_idx as usize) * 0x48);
                let len_4 = rd::<u32>(slot + 4) as u64;
                let inline_p = slot + 8;
                let b3 = rd::<u8>(slot + 3) as u32;
                let cap_bits = if b3 < 0x80 { b3 * 8 } else { b3 * 256 - 0x7c00 };
                let used_bits = (rd::<u32>(slot) & 0xff_ffff) * 8;
                let str_ptr = if used_bits > cap_bits {
                    rd::<usize>(inline_p)
                } else {
                    inline_p
                };
                let mut hash_val = exe::call_va!(0x1_424b_16a0, (str_ptr, len_4) -> u64);
                wr::<u32>(slot + 4, 0);
                let str_ptr2 = if used_bits > cap_bits {
                    rd::<usize>(inline_p)
                } else {
                    inline_p
                };
                wr::<u8>(str_ptr2, 0);

                let ht = param_1 + 0x90;
                let bucket_idx = if rd::<u64>(ht + 0x38) == rd::<u64>(ht + 0x30) {
                    rd::<u64>(param_1 + 0xd0)
                } else {
                    let mut probe_idx = 0i64;
                    exe::call_va!(
                        0x1_4105_2dc0,
                        (
                            ht,
                            (&mut probe_idx as *mut i64) as usize,
                            (&mut hash_val as *mut u64) as usize
                        )
                    );
                    if probe_idx == -1 {
                        rd::<u64>(param_1 + 0xd0)
                    } else {
                        probe_idx as u64
                    }
                };
                let end_idx = rd::<u64>(param_1 + 0xd0);
                if bucket_idx != end_idx {
                    let b_ptr = rd::<usize>(param_1 + 0xe8).wrapping_add((bucket_idx as usize) * 16);
                    wr::<u64>(b_ptr, rd::<u64>(param_1 + 0xb8));
                    wr::<u32>(b_ptr + 8, 0);
                    wr::<u64>(param_1 + 0xc0, rd::<u64>(param_1 + 0xc0).wrapping_add(1));
                    wr::<u8>(param_1 + 0xa8, 1);
                }

                let vec_f0 = param_1 + 0xf0;
                let f_len = rd::<u32>(vec_f0 + 8) as u64;
                let f_cap = rd::<u32>(vec_f0 + 12) as u64;
                if f_len * 4 == f_cap * 4 {
                    let next_len = f_len + 1;
                    let mut new_cap = f_cap + (f_cap >> 1);
                    if new_cap < next_len {
                        new_cap = next_len;
                    }
                    exe::call_va!(0x1_401d_eb40, (vec_f0, new_cap));
                }
                let cur_flen = rd::<u32>(vec_f0 + 8) as usize;
                wr::<u32>(rd::<usize>(vec_f0).wrapping_add(cur_flen * 4), bit_idx as u32);
                wr::<u32>(vec_f0 + 8, (cur_flen as u32).wrapping_add(1));
            }
            inv ^= 1u64 << (tz & 0x3f);
        }
    }
    if words_final != 0 {
        exe::call_va!(0x1_4241_c190, (words_final));
    }
}

/// `unknown_141fec6a0`
/// Original: 0x141fec6a0 (1204 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_141fec6a0(param_1: usize, param_2: usize) {
    let count = rd::<u64>(param_1 + 0x18) as usize;
    for idx in 0..count {
        let p0 = rd::<usize>(param_1 + 0x20);
        let p1 = rd::<usize>(param_1 + 0x28);
        let p2 = rd::<usize>(param_1 + 0x30);
        let p3 = rd::<usize>(param_1 + 0x38);
        if rd::<i32>(p3.wrapping_add(idx * 4)) != 0 {
            return;
        }
        let off = idx.wrapping_mul(0x48);
        let r12 = p1.wrapping_add(off);
        let r13 = p0.wrapping_add(off);
        if rd::<u32>(r12 + 0x40) != rd::<u32>(r13 + 0x04) {
            exe::call_va!(0x1_41fe_e050, (r12, r13));
        }

        // Loop 1 (+0x00, count +0x08, tag 0x2a)
        let b1 = rd::<usize>(r12);
        let c1 = rd::<u32>(r12 + 0x08) as usize;
        for i in 0..c1 {
            let elem_idx = rd::<u32>(b1.wrapping_add(i * 4)) as usize;
            let slot_0 = rd::<u16>(r13) as usize;
            let floats = rd::<usize>(r13.wrapping_add(slot_0 * 16).wrapping_add(8));
            if rd::<f32>(floats.wrapping_add(elem_idx * 4)) > 0.0f32 {
                let u6 = rd::<u64>(p2.wrapping_add(idx * 8));
                let u7 = rd::<u64>(rd::<usize>(r13 + 0x38).wrapping_add(elem_idx * 8));
                let out = exe::call_va!(0x1_4020_f310, (param_2 + 0x68) -> usize);
                wr::<u64>(out, u6);
                wr::<u32>(out + 8, 0x2a);
                let s_88 = param_2 + 0x88;
                let r9 = rd::<u64>(s_88 + 0x20);
                let r8 = rd::<usize>(s_88);
                let diff = rd::<u32>(s_88 + 0x18)
                    .wrapping_sub(rd::<u32>(r8.wrapping_add((r9 as usize) * 8)))
                    | ((r9 as u32) << 16);
                exe::call_va!(0x1_4022_6cb0, (s_88, u7));
                let p_b = exe::call_va!(0x1_4020_bfd0, (s_88, 1u64) -> usize);
                wr::<u8>(p_b, 0x0c);
                wr::<u32>(out + 0x0c, diff);
                wr::<u32>(out + 0x10, 0xffff_ffff);
            }
        }

        // Loop 2 (+0x10, count +0x18, tag 0x2b)
        let b2 = rd::<usize>(r12 + 0x10);
        let c2 = rd::<u32>(r12 + 0x18) as usize;
        for i in 0..c2 {
            let pair_p = b2.wrapping_add(i * 8);
            let elem_idx = rd::<u32>(pair_p) as usize;
            let slot_0 = rd::<u16>(r13) as usize;
            let floats = rd::<usize>(r13.wrapping_add(slot_0 * 16).wrapping_add(8));
            let gt = u8::from(rd::<f32>(floats.wrapping_add(elem_idx * 4)) > 0.0f32);
            if gt != rd::<u8>(pair_p + 4) {
                wr::<u8>(pair_p + 4, gt);
                let u6 = rd::<u64>(p2.wrapping_add(idx * 8));
                let u7 = rd::<u64>(rd::<usize>(r13 + 0x38).wrapping_add(elem_idx * 8));
                let out = exe::call_va!(0x1_4020_f310, (param_2 + 0x68) -> usize);
                wr::<u64>(out, u6);
                wr::<u32>(out + 8, 0x2b);
                let s_88 = param_2 + 0x88;
                let r9 = rd::<u64>(s_88 + 0x20);
                let r8 = rd::<usize>(s_88);
                let diff = rd::<u32>(s_88 + 0x18)
                    .wrapping_sub(rd::<u32>(r8.wrapping_add((r9 as usize) * 8)))
                    | ((r9 as u32) << 16);
                exe::call_va!(0x1_4022_6cb0, (s_88, u7));
                let p_b2 = exe::call_va!(0x1_4020_bfd0, (s_88, 2u64) -> usize);
                wr::<u8>(p_b2, 8);
                wr::<u8>(p_b2 + 1, gt);
                let p_b1 = exe::call_va!(0x1_4020_bfd0, (s_88, 1u64) -> usize);
                wr::<u8>(p_b1, 0x0c);
                wr::<u32>(out + 0x0c, diff);
                wr::<u32>(out + 0x10, 0xffff_ffff);
            }
        }

        // Loop 3 (+0x20, count +0x28, tag 0x2c)
        let b3 = rd::<usize>(r12 + 0x20);
        let c3 = rd::<u32>(r12 + 0x28) as usize;
        for i in 0..c3 {
            let elem_idx = rd::<u32>(b3.wrapping_add(i * 4)) as usize;
            let slot_0 = rd::<u16>(r13) as usize;
            let floats = rd::<usize>(r13.wrapping_add(slot_0 * 16).wrapping_add(8));
            if rd::<f32>(floats.wrapping_add(elem_idx * 4)) > 0.0f32 {
                let u6 = rd::<u64>(rd::<usize>(r13 + 0x38).wrapping_add(elem_idx * 8));
                let u7 = rd::<u64>(p2.wrapping_add(idx * 8));
                let out = exe::call_va!(0x1_4020_f310, (param_2 + 0x68) -> usize);
                wr::<u64>(out, u7);
                wr::<u32>(out + 8, 0x2c);
                let s_88 = param_2 + 0x88;
                let r9 = rd::<u64>(s_88 + 0x20);
                let r8 = rd::<usize>(s_88);
                let diff = rd::<u32>(s_88 + 0x18)
                    .wrapping_sub(rd::<u32>(r8.wrapping_add((r9 as usize) * 8)))
                    | ((r9 as u32) << 16);
                exe::call_va!(0x1_4022_6cb0, (s_88, u6));
                let p_b = exe::call_va!(0x1_4020_bfd0, (s_88, 1u64) -> usize);
                wr::<u8>(p_b, 0x0c);
                wr::<u32>(out + 0x0c, diff);
                wr::<u32>(out + 0x10, 0xffff_ffff);
            }
        }

        // Loop 4 (+0x30, count +0x38, tag 0x2d)
        let b4 = rd::<usize>(r12 + 0x30);
        let c4 = rd::<u32>(r12 + 0x38) as usize;
        for i in 0..c4 {
            let pair_p = b4.wrapping_add(i * 8);
            let elem_idx = rd::<u32>(pair_p) as usize;
            let slot_0 = rd::<u16>(r13) as usize;
            let floats = rd::<usize>(r13.wrapping_add(slot_0 * 16).wrapping_add(8));
            let gt = u8::from(rd::<f32>(floats.wrapping_add(elem_idx * 4)) > 0.0f32);
            if gt != rd::<u8>(pair_p + 4) {
                wr::<u8>(pair_p + 4, gt);
                let u6 = rd::<u64>(rd::<usize>(r13 + 0x38).wrapping_add(elem_idx * 8));
                let u7 = rd::<u64>(p2.wrapping_add(idx * 8));
                let out = exe::call_va!(0x1_4020_f310, (param_2 + 0x68) -> usize);
                wr::<u64>(out, u7);
                wr::<u32>(out + 8, 0x2d);
                let s_88 = param_2 + 0x88;
                let r9 = rd::<u64>(s_88 + 0x20);
                let r8 = rd::<usize>(s_88);
                let diff = rd::<u32>(s_88 + 0x18)
                    .wrapping_sub(rd::<u32>(r8.wrapping_add((r9 as usize) * 8)))
                    | ((r9 as u32) << 16);
                exe::call_va!(0x1_4022_6cb0, (s_88, u6));
                let p_b2 = exe::call_va!(0x1_4020_bfd0, (s_88, 2u64) -> usize);
                wr::<u8>(p_b2, 8);
                wr::<u8>(p_b2 + 1, gt);
                let p_b1 = exe::call_va!(0x1_4020_bfd0, (s_88, 1u64) -> usize);
                wr::<u8>(p_b1, 0x0c);
                wr::<u32>(out + 0x0c, diff);
                wr::<u32>(out + 0x10, 0xffff_ffff);
            }
        }
    }
}

/// `unknown_1420104f0`
/// Original: 0x1420104f0 (1192 bytes, Throws::Discard)
pub unsafe extern "win64" fn unknown_1420104f0(param_1: usize) -> u64 {
    let tbl_580e0 = rd::<usize>(param_1 + 0x580e0);
    let slot_948 = rd::<u16>(exe::ptr::<u16>(0x1_43cc_8948) as usize) as usize;
    let rbp = rd::<usize>(tbl_580e0.wrapping_add(slot_948 * 8));
    let mut ret_val = slot_948 as u64;
    if rd::<u8>(rbp) != 0 {
        let slot_63c = rd::<u16>(exe::ptr::<u16>(0x1_43c8_f63c) as usize) as usize;
        let slot_910 = rd::<u16>(exe::ptr::<u16>(0x1_43c8_d910) as usize) as usize;
        let r12 = rd::<usize>(tbl_580e0.wrapping_add(slot_63c * 8));
        let rbx = rd::<usize>(tbl_580e0.wrapping_add(slot_910 * 8));
        ret_val = slot_910 as u64;
        if rd::<u64>(r12) != 0 {
            let r14 = exe::call_va!(0x1_424d_4a00, () -> usize);
            let tls0 = rd::<usize>(exe::tls_pointer());
            if rd::<u8>(exe::ptr::<u8>(0x1_4385_7748) as usize) == 0 {
                let g_458 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8458) as usize);
                let f2 = rd::<u64>(rbx + 0x10) as u128;
                let _ = crate::simd_ops::call_va8_xmm(
                    0x1_424e_2e60,
                    g_458 as u64,
                    rd::<u64>(r12),
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    [0, 0, f2, 0],
                );
                let rsi = rbp + 0xd0;
                exe::call_va!(0x1_4116_9220, (rsi));
                let g_458_b = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8458) as usize);
                let mut cur = rd::<usize>(g_458_b + 0x30);
                let end = cur.wrapping_add((rd::<u32>(g_458_b + 0x38) as usize) * 0x60);
                while cur != end {
                    let b50 = rd::<u8>(cur + 0x50);
                    let h = ((b50 as u64) ^ 0xcbf2_9ce4_8422_2325u64)
                        .wrapping_mul(0x0100_0000_01b3u64)
                        & rd::<u64>(rsi + 0x30);
                    let buckets = rd::<usize>(rsi + 0x18);
                    let sentinel = rd::<usize>(rsi + 0x08);
                    let mut node = rd::<usize>(buckets.wrapping_add((h as usize) * 16 + 8));
                    if node == sentinel {
                        node = 0;
                    } else {
                        let stop = rd::<usize>(buckets.wrapping_add((h as usize) * 16));
                        while rd::<u8>(node + 0x10) != b50 {
                            if node == stop {
                                node = 0;
                                break;
                            }
                            node = rd::<usize>(node + 8);
                        }
                    }
                    if node == 0 || node == sentinel {
                        let mut tmp = [0u64; 2];
                        let res = exe::call_va!(
                            0x1_4116_a4b0,
                            (rsi, tmp.as_mut_ptr() as usize, cur + 0x50) -> usize
                        );
                        wr::<u8>(rd::<usize>(res) + 0x11, 1);
                    } else {
                        let old_c = rd::<u8>(node + 0x11);
                        if old_c != 0xff {
                            let mut tmp = [0u64; 2];
                            let res = exe::call_va!(
                                0x1_4116_a4b0,
                                (rsi, tmp.as_mut_ptr() as usize, cur + 0x50) -> usize
                            );
                            wr::<u8>(rd::<usize>(res) + 0x11, old_c.wrapping_add(1));
                        }
                    }
                    cur = cur.wrapping_add(0x60);
                }

                let g_820 = exe::ptr::<i32>(0x1_43ce_6820) as usize;
                if rd::<i32>(tls0 + 0x74) < rd::<i32>(g_820) {
                    exe::call_va!(0x1_4183_4a78, (g_820));
                    if rd::<i32>(g_820) == -1 {
                        let g_600 = exe::ptr::<u8>(0x1_43ce_6600) as usize;
                        exe::call_va!(0x1_4107_5ad0, (g_600));
                        wr::<u32>(exe::ptr::<u32>(0x1_43ce_678c) as usize, 0);
                        wr::<u32>(exe::ptr::<u32>(0x1_43ce_6790) as usize, 0);
                        exe::call_va!(
                            0x1_4107_5c80,
                            (exe::ptr::<u8>(0x1_43ce_6798) as usize)
                        );
                        exe::call_va!(0x1_4183_4d18, (exe::ptr::<u8>(0x1_4261_95c0) as usize));
                        exe::call_va!(0x1_4183_4a0c, (g_820));
                    }
                }
                wr::<u32>(
                    exe::ptr::<u32>(0x1_43ce_660c) as usize,
                    rd::<u32>(r14 + 0x58),
                );
                let mut pose_buf = [0u32; 12];
                for k in 0..9 {
                    pose_buf[k] = rd::<u32>(r14 + k * 4);
                }
                pose_buf[9] = rd::<u32>(r14 + 0xc4);
                pose_buf[10] = rd::<u32>(r14 + 0xc8);
                pose_buf[11] = rd::<u32>(r14 + 0xcc);
                exe::call_va!(
                    0x1_424e_3100,
                    (
                        rd::<u64>(exe::ptr::<u64>(0x1_43a0_8458) as usize),
                        rd::<u64>(r12),
                        pose_buf.as_ptr() as u64,
                        exe::ptr::<u8>(0x1_43ce_6600) as u64,
                        0u64,
                        0u64,
                        0u64
                    )
                );
                wr::<u32>(rbp + 0xb8, rd::<u32>(exe::ptr::<u32>(0x1_43ce_678c) as usize));
                wr::<u32>(rbp + 0xb4, rd::<u32>(exe::ptr::<u32>(0x1_43ce_6790) as usize));
                exe::call_va!(0x1_4201_0f10, (rbp + 0x90));
            } else if rd::<u64>(exe::ptr::<u64>(0x1_43a4_6568) as usize) != 0 {
                wr::<u64>(rbp + 0x88, rd::<u64>(rbx + 0x10));
                if rd::<u64>(rbp + 0x10) == 0 {
                    let v = [
                        rd::<f32>(r14 + 0xc4),
                        rd::<f32>(r14 + 0xc8),
                        rd::<f32>(r14 + 0xcc),
                        0.0f32,
                    ];
                    wr::<[f32; 4]>(rbp + 0x20, v);
                    wr::<u32>(rbp + 0x80, rd::<u32>(r14 + 0x58));
                    for k in 0..9 {
                        wr::<u32>(rbp + 0x40 + k * 4, rd::<u32>(r14 + k * 4));
                    }
                    wr::<u32>(rbp + 0x64, rd::<u32>(r14 + 0xc4));
                    wr::<u32>(rbp + 0x68, rd::<u32>(r14 + 0xc8));
                    wr::<u32>(rbp + 0x6c, rd::<u32>(r14 + 0xcc));
                }
                let g_5d4 = exe::ptr::<i32>(0x1_43ce_65d4) as usize;
                let g_580 = exe::ptr::<[f32; 4]>(0x1_43ce_6580) as usize;
                if rd::<i32>(tls0 + 0x74) < rd::<i32>(g_5d4) {
                    exe::call_va!(0x1_4183_4a78, (g_5d4));
                    if rd::<i32>(g_5d4) == -1 {
                        wr::<[f32; 4]>(g_580, rd::<[f32; 4]>(rbp + 0x20));
                        exe::call_va!(0x1_4183_4a0c, (g_5d4));
                    }
                }
                let cur_20 = rd::<[f32; 4]>(rbp + 0x20);
                let prev_580 = rd::<[f32; 4]>(g_580);
                let diff = [
                    cur_20[0] - prev_580[0],
                    cur_20[1] - prev_580[1],
                    cur_20[2] - prev_580[2],
                    cur_20[3] - prev_580[3],
                ];
                wr::<[f32; 4]>(rbp + 0x30, diff);
                wr::<[f32; 4]>(g_580, cur_20);
                exe::call_va!(0x1_4201_09a0, (rbp, rd::<u64>(r12)));
            }

            let dst_1120 = rd::<usize>(exe::ptr::<usize>(0x1_43a0_8428) as usize) + 0x1120;
            let src_d0 = rbp + 0xd0;
            if dst_1120 != src_d0 {
                wr::<u32>(dst_1120, rd::<u32>(src_d0));
                let head = rd::<usize>(src_d0 + 8);
                let first = rd::<u64>(head);
                exe::call_va!(0x1_411a_0fe0, (dst_1120 + 8, first, head));
                let n = exe::call_va!(0x1_4020_6290, (dst_1120, rd::<u64>(dst_1120 + 0x10)) -> u64);
                exe::call_va!(0x1_4116_ac50, (dst_1120, n));
            }
            let new_34c = rd::<u32>(rbp + 0xb4);
            let a_34c = exe::ptr::<core::sync::atomic::AtomicU32>(0x1_43a0_834c);
            ret_val = (*a_34c).swap(new_34c, core::sync::atomic::Ordering::SeqCst) as u64;
        }
        wr::<u64>(rbp + 0x10, 0);
    }
    ret_val
}

/// `unknown_142073a40`
/// Original: 0x142073a40 (497 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142073a40(param_1: usize, param_2: u64) {
    #[repr(C)]
    struct SmallVec256 {
        ptr: usize,
        len: u32,
        cap: u32,
        inline: [u64; 32],
    }
    let mut sv = SmallVec256 {
        ptr: 0,
        len: 0,
        cap: 8,
        inline: [0u64; 32],
    };
    sv.ptr = sv.inline.as_mut_ptr() as usize;

    let mut cur_p = param_1;
    for _ in 0..(param_2 as usize) {
        let obj = rd::<usize>(cur_p);
        let h_40 = rd::<u64>(obj + 0x40);
        let item = [0u64, h_40, 0x0000_0200_ffff_ffffu64, 8u64];
        if sv.len == sv.cap {
            let cur_len = sv.len as u64;
            let cur_cap = sv.cap as u64;
            let mut new_cap = cur_cap + (cur_cap >> 1);
            if new_cap < cur_len + 1 {
                new_cap = cur_len + 1;
            }
            let new_ptr = if new_cap <= 8 && sv.ptr != (sv.inline.as_mut_ptr() as usize) {
                new_cap = 8;
                sv.inline.as_mut_ptr() as usize
            } else {
                exe::call_va!(0x1_401f_fc30, ((&mut sv as *mut SmallVec256) as usize, new_cap) -> usize)
            };
            core::ptr::copy(
                sv.ptr as *const u8,
                new_ptr as *mut u8,
                (sv.len as usize) * 32,
            );
            exe::call_va!(0x1_4037_e270, ((&mut sv as *mut SmallVec256) as usize));
            sv.ptr = new_ptr;
            sv.len = cur_len as u32;
            sv.cap = new_cap as u32;
        }
        let dst = sv.ptr.wrapping_add((sv.len as usize) * 32);
        wr::<[u64; 4]>(dst, item);
        sv.len = sv.len.wrapping_add(1);
        wr::<u32>(obj + 0x48, 8);
        cur_p = cur_p.wrapping_add(8);
    }

    let mut cmd_pair = [0usize; 2];
    exe::call_va!(0x1_411d_5b00, (cmd_pair.as_mut_ptr() as usize) -> usize);
    let cmd_list = cmd_pair[1];
    let vtbl = rd::<usize>(cmd_list);
    exe::call_va!(
        rd::<usize>(vtbl + 0xd0),
        (cmd_list, param_2 as u32 as u64, sv.ptr)
    );
    let lock = cmd_pair[0];
    if lock != 0 {
        let rc = rd::<u32>(lock + 0x0c).wrapping_sub(1);
        wr::<u32>(lock + 0x0c, rc);
        if rc == 0 {
            let a = (lock + 0x08) as *const core::sync::atomic::AtomicU32;
            (*a).swap(0xffff_ffff, core::sync::atomic::Ordering::SeqCst);
            let _ = exe::call_iat!(0x1_4264_5598, (lock as u64) -> u64);
        }
    }
    exe::call_va!(0x1_4037_e270, ((&mut sv as *mut SmallVec256) as usize));
}

/// `r::JobCallable<physics::PhysXCPUDispatcher::submitTask::<lambda_1>>::vfunc_1`
/// Original: 0x1421444b0 (35 bytes, Throws::No)
pub unsafe extern "win64" fn r_1421444b0(this: usize) {
    let task = rd::<usize>(this + 0x08);
    let vtbl1 = rd::<usize>(task);
    exe::call_va!(rd::<usize>(vtbl1 + 0x08), (task));
    let task2 = rd::<usize>(this + 0x08);
    let vtbl2 = rd::<usize>(task2);
    exe::call_va!(rd::<usize>(vtbl2 + 0x30), (task2));
}

/// `unknown_14223d920`
/// Original: 0x14223d920 (714 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_14223d920(
    param_1: usize,
    param_2: usize,
    param_3: i32,
    param_4: usize,
) {
    let count = param_3 as isize;
    let wid = crate::ns::r::thread::worker_index() as usize;
    let slot_ptr = rd::<usize>((exe::ptr::<usize>(0x1_43c4_aa00) as usize).wrapping_add(wid * 8));
    let slot_idx = if slot_ptr != 0 { rd::<u32>(slot_ptr) as usize } else { 0 };
    let rbp = param_1.wrapping_add(slot_idx.wrapping_mul(0xd8));

    let lock = rbp + 0xa8;
    let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
    if rd::<u32>(lock + 0x08) != tid {
        let _ = exe::call_iat!(0x1_4264_55a8, (lock as u64) -> u64);
        wr::<u32>(lock + 0x08, tid);
    }
    wr::<u32>(lock + 0x0c, rd::<u32>(lock + 0x0c).wrapping_add(1));

    let cur_len = rd::<u32>(rbp + 0x90);
    exe::call_va!(
        0x1_4223_8b60,
        (param_1 as u64, 2u64, cur_len as u64, (param_3 as u32) as u64, 0u64, 0u64, 0u64)
    );

    let vec_88 = rbp + 0x88;
    let cap_94 = rd::<u32>(rbp + 0x94) as u64;
    let len_90 = rd::<u32>(rbp + 0x90) as u64;
    if (count as u64) > cap_94.wrapping_sub(len_90) {
        let needed = len_90.wrapping_add(count as u64);
        let mut new_cap = cap_94 + (cap_94 >> 1);
        if new_cap < needed {
            new_cap = needed;
        }
        exe::call_va!(0x1_40fb_1b60, (vec_88, new_cap));
    }

    let start_idx = rd::<u32>(rbp + 0x90) as usize;
    let dst_base = rd::<usize>(vec_88).wrapping_add(start_idx.wrapping_mul(32));
    if param_3 != 0 {
        core::ptr::write_bytes(dst_base as *mut u8, 0, (count as usize).wrapping_mul(32));
    }
    wr::<u32>(rbp + 0x90, (start_idx as u32).wrapping_add(param_3 as u32));

    let r0 = rd::<[f32; 4]>(param_4);
    let r1 = rd::<[f32; 4]>(param_4 + 0x10);
    let r2 = rd::<[f32; 4]>(param_4 + 0x20);
    let def_w = rd::<f32>(exe::ptr::<f32>(0x1_4342_8240 + 12) as usize);

    let col0 = [r0[0], r0[1], r0[2], 0.0f32];
    let col1 = [r0[3], r1[0], r1[1], 0.0f32];
    let col2 = [r1[2], r1[3], r2[0], 0.0f32];
    let col3 = [r2[1], r2[2], r2[3], def_w];
    let base_t = [
        col2[0] * 0.0f32 + col3[0],
        col2[1] * 0.0f32 + col3[1],
        col2[2] * 0.0f32 + col3[2],
        col2[3] * 0.0f32 + col3[3],
    ];

    if count > 0 {
        for i in 0..(count as usize) {
            let x = rd::<f32>(param_2.wrapping_add(i * 8));
            let y = rd::<f32>(param_2.wrapping_add(i * 8 + 4));
            let out_v = [
                x * col0[0] + y * col1[0] + base_t[0],
                x * col0[1] + y * col1[1] + base_t[1],
                x * col0[2] + y * col1[2] + base_t[2],
                x * col0[3] + y * col1[3] + base_t[3],
            ];
            let slot = dst_base.wrapping_add(i * 32);
            wr::<[f32; 4]>(slot, out_v);
            wr::<u32>(slot + 0x1c, rd::<u32>(rbp + 0x14));
        }
    }

    let rc = rd::<u32>(lock + 0x0c).wrapping_sub(1);
    wr::<u32>(lock + 0x0c, rc);
    if rc == 0 {
        let a = (lock + 0x08) as *const core::sync::atomic::AtomicU32;
        (*a).swap(0xffff_ffff, core::sync::atomic::Ordering::SeqCst);
        let _ = exe::call_iat!(0x1_4264_5598, (lock as u64) -> u64);
    }
}

/// `unknown_142350200`
/// Original: 0x142350200 (653 bytes, Throws::No)
pub unsafe extern "win64" fn unknown_142350200(param_1: usize, param_2: u64) {
    let mut qpc = 0i64;
    let _ = exe::call_iat!(0x1_4264_56f8, ((&mut qpc as *mut i64) as u64) -> i32);
    let _ = exe::call_va!(0x1_4240_6920, () -> i64);
    exe::call_va!(0x1_412f_7fb0, (param_1 + 0x38));
    exe::call_va!(0x1_412f_7fb0, (param_1 + 0x50));

    if rd::<u8>(exe::ptr::<u8>(0x1_4386_c8d0) as usize) == 0 {
        return;
    }

    let n_words = (rd::<u64>(param_1 + 0x30).wrapping_add(0x3f)) >> 6;
    let words_base = rd::<usize>(param_1 + 0x20);
    let dist_arr = rd::<usize>(param_1 + 0x68);
    let rad_arr = rd::<usize>(param_1 + 0x78);

    let sub_a8 = param_1 + 0xa8;
    exe::call_va!(rd::<usize>(rd::<usize>(sub_a8) + 0x38), (sub_a8));
    let mesh_arr = rd::<usize>(param_1 + 0x140);
    let sub_188 = param_1 + 0x188;
    exe::call_va!(rd::<usize>(rd::<usize>(sub_188) + 0x38), (sub_188));

    let pts_base = rd::<usize>(param_1 + 0x220);
    let clamp_max = rd::<f32>(exe::ptr::<f32>(0x1_4342_8650) as usize);
    let cam_x = f32::from_bits(param_2 as u32);
    let cam_y = f32::from_bits((param_2 >> 32) as u32);

    for w_idx in 0..n_words {
        let mut word = rd::<u64>(words_base.wrapping_add((w_idx as usize) * 8));
        let base_idx = (w_idx as usize) * 64;
        while word != 0 {
            let tz = word.trailing_zeros() as usize;
            word ^= 1u64 << tz;
            let idx = base_idx + tz;
            let m_entry = mesh_arr.wrapping_add(idx * 12);
            let i0 = rd::<u32>(m_entry + 4) as usize;
            let i1 = rd::<u32>(m_entry + 8) as usize;
            let p0_u = rd::<u64>(pts_base.wrapping_add(i0 * 8));
            let p1_u = rd::<u64>(pts_base.wrapping_add(i1 * 8));
            let p0x = f32::from_bits(p0_u as u32);
            let p0y = f32::from_bits((p0_u >> 32) as u32);
            let p1x = f32::from_bits(p1_u as u32);
            let p1y = f32::from_bits((p1_u >> 32) as u32);

            let dx = p1x - p0x;
            let dy = p1y - p0y;
            let mx = dx * 0.5f32 + p0x - cam_x;
            let my = dy * 0.5f32 + p0y - cam_y;
            let dist_sq = mx * mx + my * my;
            let clamped = if dist_sq < clamp_max { dist_sq } else { clamp_max };
            let hx = dx * 0.5f32;
            let hy = dy * 0.5f32;
            let rad_sq = hx * hx + hy * hy;

            wr::<f32>(dist_arr.wrapping_add(idx * 4), clamped);
            wr::<f32>(rad_arr.wrapping_add(idx * 4), rad_sq);
        }
    }

    let count_80 = (rd::<u32>(param_1 + 0x80) as usize) & !7usize;
    let out_bits_hi = rd::<usize>(param_1 + 0x38);
    let out_bits_lo = rd::<usize>(param_1 + 0x50);
    let thresh_lo = rd::<f32>(exe::ptr::<f32>(0x1_4386_c898) as usize);
    let c858 = rd::<f32>(exe::ptr::<f32>(0x1_4386_c858) as usize);
    let thresh_hi = if c858 > thresh_lo { c858 } else { thresh_lo };

    let fov = rd::<f32>(exe::ptr::<f32>(0x1_4384_170c) as usize)
        * rd::<f32>(exe::ptr::<f32>(0x1_4342_6780) as usize);
    let res_h = (rd::<i32>(exe::ptr::<i32>(0x1_43a3_395c) as usize)) as f32;
    let tan_fn: unsafe extern "win64" fn(f32) -> f32 = exe::func(0x1_4183_ad8e);
    let proj_scale = res_h / tan_fn(fov);
    let min_denom = rd::<f32>(exe::ptr::<f32>(0x1_4386_c8e8) as usize);
    let dist_scale = rd::<f32>(exe::ptr::<f32>(0x1_4386_c878) as usize);

    let mut i = 0usize;
    while i < count_80 {
        let mut mask_hi = 0u64;
        let mut mask_lo = 0u64;
        for lane in 0..8 {
            let k = i + lane;
            let r = rd::<f32>(rad_arr.wrapping_add(k * 4)).sqrt();
            let d = rd::<f32>(dist_arr.wrapping_add(k * 4)).sqrt();
            let diff = (d - r) * dist_scale;
            let denom = if diff > min_denom { diff } else { min_denom };
            let val = (r * proj_scale) / denom;
            wr::<f32>(rad_arr.wrapping_add(k * 4), val);
            if val > thresh_hi {
                mask_hi |= 1u64 << lane;
            }
            if val < thresh_lo {
                mask_lo |= 1u64 << lane;
            }
        }
        let w_idx = i >> 6;
        let shift = (i & 0x3f) as u32;
        let active = rd::<u64>(words_base.wrapping_add(w_idx * 8));
        let hi_addr = out_bits_hi.wrapping_add(w_idx * 8);
        wr::<u64>(hi_addr, rd::<u64>(hi_addr) | ((mask_hi << shift) & active));
        let lo_addr = out_bits_lo.wrapping_add(w_idx * 8);
        wr::<u64>(lo_addr, rd::<u64>(lo_addr) | ((mask_lo << shift) & active));
        i += 8;
    }
}

/// `ui_1424f6500`
/// Original: 0x1424f6500 (462 bytes, Throws::Discard)
pub unsafe extern "win64" fn ui_1424f6500(
    param_1: usize,
    param_2: i32,
    param_3: u64,
    param_4: u64,
) {
    if param_2 == -1 {
        return;
    }
    let mut id_slot = param_2;
    let entry_p = exe::call_va!(
        0x1_4141_0810,
        (param_1, (&mut id_slot as *mut i32) as usize) -> usize
    );
    let rbx = rd::<usize>(entry_p);
    if rd::<i32>(rbx + 0x08) == -1 {
        return;
    }
    let rsi = rd::<usize>(rbx + 0x10);
    if rd::<u8>(rsi + 0x29) == 0 {
        return;
    }
    wr::<u64>(rsi + 0x110, param_3);
    wr::<u8>(rsi + 0x118, 1);
    wr::<u64>(rsi + 0x120, param_4);
    wr::<u8>(rsi + 0x128, 1);

    let vtbl = rd::<usize>(rsi);
    exe::call_va!(rd::<usize>(vtbl + 0xc8), (rsi, param_3, param_4));
    if rd::<u8>(rsi + 0x2a) != 0 {
        exe::call_va!(0x1_4250_4040, (rsi));
    } else {
        exe::call_va!(0x1_4250_24d0, (rsi));
        let ok = exe::call_va!(
            0x1_4250_2a20,
            (rsi, rd::<u32>(rbx + 0x20) as u64) -> u8
        );
        if ok != 0 {
            if rd::<u8>(rsi + 0x2c) != 0 {
                exe::call_va!(
                    0x1_424f_67a0,
                    (
                        param_1,
                        rd::<u32>(rbx + 0x08) as u64,
                        rd::<u8>(rsi + 0x2b) as u64
                    )
                );
                wr::<u8>(rsi + 0x2c, 0);
            }
            let r15 = rd::<usize>(rbx + 0x28);
            let vtbl_15 = rd::<usize>(r15);
            let ready = exe::call_va!(rd::<usize>(vtbl_15 + 0x1e0), (r15) -> u8);
            if ready != 0 {
                let vtbl_15b = rd::<usize>(r15);
                exe::call_va!(rd::<usize>(vtbl_15b + 0x220), (r15));
                let mut cur = rd::<usize>(rsi + 0xa0);
                let end = cur.wrapping_add((rd::<u32>(rsi + 0xa8) as usize).wrapping_mul(0x88));
                while cur != end {
                    let b3 = rd::<u8>(cur + 3) as u32;
                    let cap_bits = if b3 < 0x80 { b3 * 8 } else { b3 * 256 - 0x7c00 };
                    let used_bits = (rd::<u32>(cur) & 0xff_ffff) * 8;
                    let inline_p = cur + 8;
                    let str_p = if used_bits > cap_bits {
                        rd::<usize>(inline_p)
                    } else {
                        inline_p
                    };
                    let mut id2 = rd::<i32>(rsi + 0x18);
                    let p7 = exe::call_va!(
                        0x1_4141_0810,
                        (cap_bits as usize, (&mut id2 as *mut i32) as usize) -> usize
                    );
                    let r8_obj = rd::<usize>(p7);
                    let view = rd::<usize>(r8_obj + 0x28);
                    let view_vtbl = rd::<usize>(view);
                    exe::call_va!(rd::<usize>(view_vtbl + 0x250), (view, str_p, r8_obj));
                    cur = cur.wrapping_add(0x88);
                }
                exe::call_va!(0x1_4021_8b60, (rsi + 0xa0));
            }
            let vtbl_15c = rd::<usize>(r15);
            let adv_fn = rd::<usize>(vtbl_15c + 0x30) as u64;
            let time_fn: unsafe extern "win64" fn() -> f64 = exe::func(0x1_4240_69a0);
            let ms = time_fn() * rd::<f64>(exe::ptr::<f64>(0x1_4342_6d28) as usize);
            let xmm1 = ms.to_bits() as u128;
            let _ = crate::simd_ops::call_va8_xmm(
                adv_fn,
                r15 as u64,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                [0, xmm1, 0, 0],
            );
        }
    }
    wr::<u8>(rsi + 0x118, 0);
    wr::<u8>(rsi + 0x128, 0);
}

/// `unknown_142138050`
/// Original: 0x142138050 (409 bytes, leaf, Throws::No)
pub unsafe extern "win64" fn unknown_142138050(param_1: usize) {
    let g_init = rd::<usize>(param_1 + 0x28);
    let voxel_size = rd::<f32>(g_init);
    let mut dim_xz = rd::<i32>(g_init + 0x08);
    if dim_xz <= 0 {
        return;
    }
    const FLT_MAX: f32 = f32::MAX;
    let mut iy = 0i32;
    while iy < dim_xz {
        let mut ix = 0i32;
        if dim_xz > 0 {
            while ix < dim_xz {
                let mut iz = rd::<i32>(g_init + 0x0c) - 2;
                while iz >= 0 {
                    let iz_above = iz + 1;
                    let g = rd::<usize>(param_1 + 0x28);
                    let vecs = rd::<usize>(param_1 + 0x08);
                    let col_base = (iy * rd::<i32>(g + 0x08) + ix) * rd::<i32>(g + 0x0c);
                    let above_idx = (col_base + iz_above) as isize as usize;
                    let cur_idx = (col_base + iz) as isize as usize;
                    let above_ptr = vecs.wrapping_add(above_idx.wrapping_mul(12));
                    let cur_ptr = vecs.wrapping_add(cur_idx.wrapping_mul(12));
                    if rd::<f32>(above_ptr) != FLT_MAX
                        && rd::<f32>(above_ptr + 4) != FLT_MAX
                        && rd::<f32>(above_ptr + 8) != FLT_MAX
                        && rd::<f32>(cur_ptr) != FLT_MAX
                        && rd::<f32>(cur_ptr + 4) != FLT_MAX
                        && rd::<f32>(cur_ptr + 8) != FLT_MAX
                    {
                        let cand = voxel_size + rd::<f32>(above_ptr + 4);
                        let cur_y = rd::<f32>(cur_ptr + 4);
                        let max_y = if cand > cur_y { cand } else { cur_y };
                        wr::<f32>(cur_ptr + 4, max_y);

                        let flags = rd::<usize>(param_1 + 0x18);
                        let col_base2 = (iy * rd::<i32>(g + 0x08) + ix) * rd::<i32>(g + 0x0c);
                        let above_f = rd::<u8>(flags.wrapping_add((col_base2 + iz_above) as isize as usize));
                        let cur_f_ptr = flags.wrapping_add((col_base2 + iz) as isize as usize);
                        let cur_f = rd::<u8>(cur_f_ptr);
                        let chosen = if (cur_f & 7) < (above_f & 7) { above_f } else { cur_f };
                        wr::<u8>(cur_f_ptr, (chosen & 7) | ((cur_f | above_f) & 8));
                    }
                    iz -= 1;
                }
                dim_xz = rd::<i32>(g_init + 0x08);
                ix += 1;
            }
        }
        iy += 1;
    }
}

/// Loop-invariant state of the SDF sweep in [`unknown_1421381f0`]: the grid header
/// (`[param_1 + 0x28]`), the voxel vector/flag arrays and the cone parameters (`param_2`).
/// None of these are written by the sweep (it only writes voxel vectors and flags), so they
/// are read once instead of per neighbor as the original does.
struct SdfSweep {
    dim_xz: i32,
    dim_y: i32,
    voxel_size: f32,
    /// `[param_1 + 0x08]`: `float3` per voxel.
    vecs: usize,
    /// `[param_1 + 0x18]`: `u8` per voxel.
    flags: usize,
    p0: f32,
    p1: f32,
    p2: f32,
}

#[inline(always)]
unsafe fn ld_f32(addr: usize) -> f32 {
    core::ptr::read_unaligned(addr as *const f32)
}

#[inline(always)]
unsafe fn st_f32(addr: usize, v: f32) {
    core::ptr::write_unaligned(addr as *mut f32, v)
}

/// One neighbor relaxation step. The voxel arrays belong to the job running the sweep, so
/// plain loads/stores are used instead of the TSO `rd`/`wr` helpers.
#[inline(always)]
unsafe fn propagate_sdf_neighbor(
    s: &SdfSweep,
    dst_x: i32,
    dst_y: i32,
    dst_z: i32,
    src_x: i32,
    src_y: i32,
    src_z: i32,
    step_x: f32,
    step_y: f32,
    step_z: f32,
) {
    let dim_xz = s.dim_xz;
    let dim_y = s.dim_y;
    if src_x < 0 || src_y < 0 || src_z < 0 || src_x >= dim_xz || src_y >= dim_xz || src_z >= dim_y {
        return;
    }
    if dst_x < 0 || dst_y < 0 || dst_z < 0 || dst_x >= dim_xz || dst_y >= dim_xz || dst_z >= dim_y {
        return;
    }
    let src_idx = ((src_y * dim_xz + src_x) * dim_y + src_z) as isize as usize;
    let src_ptr = s.vecs.wrapping_add(src_idx.wrapping_mul(12));
    let sx = ld_f32(src_ptr);
    let sy = ld_f32(src_ptr + 4);
    let sz = ld_f32(src_ptr + 8);
    const FLT_MAX: f32 = f32::MAX;
    if sx == FLT_MAX || sy == FLT_MAX || sz == FLT_MAX {
        return;
    }
    let voxel_size = s.voxel_size;
    let nx = sx + voxel_size * step_x;
    let ny = sy + voxel_size * step_y;
    let nz = sz + voxel_size * step_z;

    let dst_idx = ((dst_y * dim_xz + dst_x) * dim_y + dst_z) as isize as usize;
    let dst_ptr = s.vecs.wrapping_add(dst_idx.wrapping_mul(12));
    let dx = ld_f32(dst_ptr);
    let dy = ld_f32(dst_ptr + 4);
    let dz = ld_f32(dst_ptr + 8);

    let cand_sq = (ny * ny + nx * nx) + nz * nz;
    let cur_sq = (dx * dx + dy * dy) + dz * dz;
    if !(cand_sq <= cur_sq) || !(ny >= 0.0) {
        return;
    }
    let (p0, p1, p2) = (s.p0, s.p1, s.p2);
    if !(ny <= (p0 + p0 + p1) + voxel_size) {
        return;
    }
    let scaled = (ny * p0) / p2;
    let clamped = if p0 < scaled { p0 } else { scaled };
    let r = clamped + voxel_size;
    if !(nx * nx + nz * nz <= r * r) {
        return;
    }
    st_f32(dst_ptr, nx);
    st_f32(dst_ptr + 4, ny);
    st_f32(dst_ptr + 8, nz);

    let flags = s.flags as *mut u8;
    *flags.wrapping_add(dst_idx) = *flags.wrapping_add(src_idx);
}

/// `unknown_1421381f0`
/// Original: 0x1421381f0 (4116 bytes, leaf, Throws::No)
pub unsafe extern "win64" fn unknown_1421381f0(param_1: usize, param_2: usize) {
    let g = rd::<usize>(param_1 + 0x28);
    let s = SdfSweep {
        dim_xz: rd::<i32>(g + 0x08),
        dim_y: rd::<i32>(g + 0x0c),
        voxel_size: rd::<f32>(g),
        vecs: rd::<usize>(param_1 + 0x08),
        flags: rd::<usize>(param_1 + 0x18),
        p0: rd::<f32>(param_2),
        p1: rd::<f32>(param_2 + 4),
        p2: rd::<f32>(param_2 + 8),
    };
    let s = &s;
    let dim_xz = s.dim_xz;
    let mut iz = s.dim_y - 1;
    while iz >= 0 {
        let mut iy = dim_xz - 1;
        while iy >= 0 {
            let mut ix = dim_xz - 1;
            while ix >= 0 {
                propagate_sdf_neighbor(s, ix, iy, iz, ix + 1, iy, iz, 1.0, 0.0, 0.0);
                propagate_sdf_neighbor(s, ix, iy, iz, ix, iy + 1, iz, 0.0, 0.0, 1.0);
                ix -= 1;
            }
            iy -= 1;
        }
        let mut iy_f = 0i32;
        while iy_f < dim_xz {
            let mut ix_f = 0i32;
            while ix_f < dim_xz {
                propagate_sdf_neighbor(s, ix_f, iy_f, iz, ix_f - 1, iy_f, iz, -1.0, 0.0, 0.0);
                propagate_sdf_neighbor(s, ix_f, iy_f, iz, ix_f, iy_f - 1, iz, 0.0, 0.0, -1.0);
                propagate_sdf_neighbor(s, ix_f, iy_f, iz, ix_f, iy_f, iz + 1, 0.0, 1.0, 0.0);
                propagate_sdf_neighbor(s, ix_f, iy_f, iz, ix_f + 1, iy_f, iz + 1, 1.0, 1.0, 0.0);
                propagate_sdf_neighbor(s, ix_f, iy_f, iz, ix_f, iy_f + 1, iz + 1, 0.0, 1.0, 1.0);
                propagate_sdf_neighbor(s, ix_f, iy_f, iz, ix_f - 1, iy_f, iz + 1, -1.0, 1.0, 0.0);
                propagate_sdf_neighbor(s, ix_f, iy_f, iz, ix_f, iy_f - 1, iz + 1, 0.0, 1.0, -1.0);
                ix_f += 1;
            }
            iy_f += 1;
        }
        iz -= 1;
    }
}

#[inline(always)]
fn approx_rcpss(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rcp_ss, _mm_set_ss};
        _mm_cvtss_f32(_mm_rcp_ss(_mm_set_ss(x)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        1.0f32 / x
    }
}

#[inline(always)]
fn approx_rsqrtss(x: f32) -> f32 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        use core::arch::x86_64::{_mm_cvtss_f32, _mm_rsqrt_ps, _mm_set1_ps};
        _mm_cvtss_f32(_mm_rsqrt_ps(_mm_set1_ps(x)))
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        1.0f32 / x.sqrt()
    }
}

/// `unknown_142139350`
/// Original: 0x142139350 (2958 bytes, leaf, Throws::No)
pub unsafe extern "win64" fn unknown_142139350(param_1: usize, param_2: usize) {
    let eps = rd::<f32>(exe::ptr::<f32>(0x1_4386_3ce0) as usize);
    let g = rd::<usize>(param_1 + 0x28);
    let mut submesh = rd::<usize>(param_2 + 0x10);
    let submesh_cnt = rd::<u32>(param_2 + 0x18) as usize;
    let submesh_end = submesh.wrapping_add(submesh_cnt.wrapping_mul(0x38));

    let voxel_size = rd::<f32>(g);
    let voxel_sq = voxel_size * voxel_size;
    let max_xz = rd::<i32>(g + 0x08) - 1;
    let max_y = rd::<i32>(g + 0x0c) - 1;

    let off_x = rd::<f64>(param_2 + 0x60) as f32;
    let off_y = rd::<f64>(param_2 + 0x68) as f32;
    let off_z = rd::<f64>(param_2 + 0x70) as f32;

    while submesh != submesh_end {
        let flags_byte = rd::<u8>(submesh + 0x2c);
        if (flags_byte & 1) == 0 {
            let tri_cnt = rd::<u32>(submesh + 0x34);
            if tri_cnt != 0 {
                let flag_bit3 = ((!flags_byte) >> 2) & 1 != 0;
                let mut tri_i = 0u32;
                while tri_i != rd::<u32>(submesh + 0x34) {
                    let tri_idx = rd::<u32>(submesh + 0x30).wrapping_add(tri_i) as usize;
                    let tris_base = rd::<usize>(param_2 + 0x20);
                    let tri_ptr = tris_base.wrapping_add(tri_idx.wrapping_mul(0x24));

                    let v0x = off_x + rd::<f32>(tri_ptr);
                    let v0y = off_y + rd::<f32>(tri_ptr + 0x04);
                    let v0z = off_z + rd::<f32>(tri_ptr + 0x08);
                    let v1x = off_x + rd::<f32>(tri_ptr + 0x0c);
                    let v1y = off_y + rd::<f32>(tri_ptr + 0x10);
                    let v1z = off_z + rd::<f32>(tri_ptr + 0x14);
                    let v2x = off_x + rd::<f32>(tri_ptr + 0x18);
                    let v2y = off_y + rd::<f32>(tri_ptr + 0x1c);
                    let v2z = off_z + rd::<f32>(tri_ptr + 0x20);

                    let e0x = v1x - v0x;
                    let e0y = v1y - v0y;
                    let e0z = v1z - v0z;
                    let e1x = v2x - v1x;
                    let e1y = v2y - v1y;
                    let e1z = v2z - v1z;
                    let e2x = v0x - v2x;
                    let e2y = v0y - v2y;
                    let e2z = v0z - v2z;

                    let len0_sq = (e0y * e0y + e0x * e0x) + e0z * e0z;
                    let len1_sq = (e1y * e1y + e1x * e1x) + e1z * e1z;
                    let len2_sq = (e2y * e2y + e2x * e2x) + e2z * e2z;

                    let inv_len0 = approx_rcpss(len0_sq);
                    let inv_len1 = approx_rcpss(len1_sq);
                    let inv_len2 = approx_rcpss(len2_sq);

                    let nx = e0y * e2z - e2y * e0z;
                    let ny = e0z * e2x - e0x * e2z;
                    let nz = e2y * e0x - e0y * e2x;
                    let n_sq = (ny * ny + nx * nx) + nz * nz;
                    let inv_n_sq = approx_rcpss(n_sq);
                    let inv_n_len = approx_rsqrtss(n_sq);

                    let unit_nx = nx * inv_n_len;
                    let unit_ny = ny * inv_n_len;
                    let unit_nz = nz * inv_n_len;

                    let e0_n_x = nz * e0y - ny * e0z;
                    let e0_n_y = nx * e0z - nz * e0x;
                    let e0_n_z = ny * e0x - nx * e0y;

                    let e1_n_x = nz * e1y - ny * e1z;
                    let e1_n_y = nx * e1z - nz * e1x;
                    let e1_n_z = ny * e1x - nx * e1y;

                    let e2_n_x = nz * e2y - ny * e2z;
                    let e2_n_y = nx * e2z - nz * e2x;
                    let e2_n_z = ny * e2x - nx * e2y;

                    let min_x = if v0x < (if v1x < v2x { v1x } else { v2x }) {
                        v0x
                    } else if v1x < v2x {
                        v1x
                    } else {
                        v2x
                    };
                    let max_x = if v0x > (if v1x > v2x { v1x } else { v2x }) {
                        v0x
                    } else if v1x > v2x {
                        v1x
                    } else {
                        v2x
                    };
                    let min_y = if v0y < (if v1y < v2y { v1y } else { v2y }) {
                        v0y
                    } else if v1y < v2y {
                        v1y
                    } else {
                        v2y
                    };
                    let max_y_f = if v0y > (if v1y > v2y { v1y } else { v2y }) {
                        v0y
                    } else if v1y > v2y {
                        v1y
                    } else {
                        v2y
                    };
                    let min_z = if v0z < (if v1z < v2z { v1z } else { v2z }) {
                        v0z
                    } else if v1z < v2z {
                        v1z
                    } else {
                        v2z
                    };
                    let max_z = if v0z > (if v1z > v2z { v1z } else { v2z }) {
                        v0z
                    } else if v1z > v2z {
                        v1z
                    } else {
                        v2z
                    };

                    let inv_vs = rd::<f32>(g + 0x04);
                    let orig_x = rd::<i32>(g + 0x10) as f32;
                    let orig_y = rd::<i32>(g + 0x14) as f32;
                    let orig_z = rd::<i32>(g + 0x18) as f32;

                    let mut ix0 = (((min_x - eps) * inv_vs - orig_x).floor() as i32).max(0);
                    let mut iy0 = (((min_y - eps) * inv_vs - orig_y).floor() as i32).max(0);
                    let mut iz0 = (((min_z - eps) * inv_vs - orig_z).floor() as i32).max(0);
                    if ix0 < 0 {
                        ix0 = 0;
                    }
                    if iy0 < 0 {
                        iy0 = 0;
                    }
                    if iz0 < 0 {
                        iz0 = 0;
                    }

                    let ix1 = (((max_x + eps) * inv_vs - orig_x).floor() as i32).min(max_xz);
                    let iy1 = ((((max_y_f + eps) * inv_vs - orig_y).floor() as i32) + 1).min(max_y);
                    let iz1 = (((max_z + eps) * inv_vs - orig_z).floor() as i32).min(max_xz);

                    let mut kz = iz0;
                    while kz <= iz1 {
                        let vs = rd::<f32>(g);
                        let pz = ((rd::<i32>(g + 0x18) + kz) as f32) * vs;
                        let mut kx = ix0;
                        while kx <= ix1 {
                            let px = ((rd::<i32>(g + 0x10) + kx) as f32) * vs;
                            if iy0 <= iy1 {
                                let dz0 = pz - v0z;
                                let dx0 = px - v0x;
                                let mut ky = iy0;
                                while ky <= iy1 {
                                    let py = ((rd::<i32>(g + 0x14) + ky) as f32) * rd::<f32>(g);
                                    let dy0 = py - v0y;
                                    let plane_dist = (dy0 * unit_ny + dx0 * unit_nx) + dz0 * unit_nz;
                                    if plane_dist.abs() <= voxel_size {
                                        let s0 = (dy0 * e0_n_y + dx0 * e0_n_x) + dz0 * e0_n_z;
                                        let (cx, cy, cz) = if s0 >= 0.0 {
                                            let dx1 = px - v1x;
                                            let dy1 = py - v1y;
                                            let dz1 = pz - v1z;
                                            let s1 = (dy1 * e1_n_y + dx1 * e1_n_x) + dz1 * e1_n_z;
                                            if s1 >= 0.0 {
                                                let dx2 = px - v2x;
                                                let dy2 = py - v2y;
                                                let dz2 = pz - v2z;
                                                let s2 = (dy2 * e2_n_y + dx2 * e2_n_x) + dz2 * e2_n_z;
                                                if s2 >= 0.0 {
                                                    let proj = (dy0 * ny * inv_n_sq + dx0 * nx * inv_n_sq)
                                                        + dz0 * nz * inv_n_sq;
                                                    (px - proj * nx, py - proj * ny, pz - proj * nz)
                                                } else {
                                                    let t = ((dy2 * e2y * inv_len2 + dx2 * e2x * inv_len2)
                                                        + dz2 * e2z * inv_len2)
                                                        .clamp(0.0, 1.0);
                                                    (v2x + t * e2x, v2y + t * e2y, v2z + t * e2z)
                                                }
                                            } else {
                                                let t = ((dy1 * e1y * inv_len1 + dx1 * e1x * inv_len1)
                                                    + dz1 * e1z * inv_len1)
                                                    .clamp(0.0, 1.0);
                                                (v1x + t * e1x, v1y + t * e1y, v1z + t * e1z)
                                            }
                                        } else {
                                            let t = ((dy0 * e0y * inv_len0 + dx0 * e0x * inv_len0)
                                                + dz0 * e0z * inv_len0)
                                                .clamp(0.0, 1.0);
                                            (v0x + t * e0x, v0y + t * e0y, v0z + t * e0z)
                                        };

                                        let rx = cx - px;
                                        let ry = cy - py;
                                        let rz = cz - pz;
                                        let dist_sq = (ry * ry + rx * rx) + rz * rz;
                                        if dist_sq <= voxel_sq {
                                            let g_cur = rd::<usize>(param_1 + 0x28);
                                            let v_idx = ((kz * rd::<i32>(g_cur + 0x08) + kx)
                                                * rd::<i32>(g_cur + 0x0c)
                                                + ky) as isize as usize;
                                            let vecs = rd::<usize>(param_1 + 0x08);
                                            let vp = vecs.wrapping_add(v_idx.wrapping_mul(12));
                                            let ox = rd::<f32>(vp);
                                            let oy = rd::<f32>(vp + 4);
                                            let oz = rd::<f32>(vp + 8);
                                            if dist_sq < (oy * oy + ox * ox) + oz * oz {
                                                wr::<f32>(vp, rx);
                                                wr::<f32>(vp + 4, ry);
                                                wr::<f32>(vp + 8, rz);
                                                let g_cur2 = rd::<usize>(param_1 + 0x28);
                                                let f_idx = ((kz * rd::<i32>(g_cur2 + 0x08) + kx)
                                                    * rd::<i32>(g_cur2 + 0x0c)
                                                    + ky)
                                                    as isize
                                                    as usize;
                                                let fp = rd::<usize>(param_1 + 0x18).wrapping_add(f_idx);
                                                let old_f = rd::<u8>(fp);
                                                let mut new_f = (old_f & 0xfc) | 4;
                                                if flag_bit3 {
                                                    new_f |= 8;
                                                }
                                                wr::<u8>(fp, new_f);
                                            }
                                        }
                                    }
                                    ky += 1;
                                }
                            }
                            kx += 1;
                        }
                        kz += 1;
                    }
                    tri_i = tri_i.wrapping_add(1);
                }
            }
        }
        submesh = submesh.wrapping_add(0x38);
    }
}

pub unsafe fn diff_unknown_141f3aae0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u8;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, _span: usize) -> usize {
        wr::<u64>(out, r11);
        wr::<u64>(out + 8, 0x1000);
        wr::<u64>(out + 16, 0x1000);
        out
    }
    unsafe extern "win64" fn mock_outer_load(_outer: usize) -> u64 {
        0
    }
    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_410a_a0d0, mock_outer_load as *const ())?;

    let p1 = [0x2000u64, 0u64, 0u64];
    let p2 = [1.0f32, 2.0f32, 3.0f32, 1.0f32];
    let r_o = f_o(p1.as_ptr() as usize, p2.as_ptr() as usize);
    let r_n = f_n(p1.as_ptr() as usize, p2.as_ptr() as usize);
    crate::diff_eq!(r_o, r_n, "141f3aae0 ret");
    Ok(1)
}

pub unsafe fn diff_unknown_142138050(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> ([u32; 6], [u8; 2]) {
        let mut g = [0u8; 0x20];
        let gp = g.as_mut_ptr() as usize;
        wr::<f32>(gp, 0.5);
        wr::<i32>(gp + 0x08, 1);
        wr::<i32>(gp + 0x0c, 2);

        let mut vecs = [1.0f32, 2.0f32, 3.0f32, 1.0f32, 4.0f32, 3.0f32];
        let mut flags = [0x01u8, 0x0bu8];
        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, vecs.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x18, flags.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, gp);

        f(p1p);
        (vecs.map(f32::to_bits), flags)
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142138050 out");
    Ok(1)
}

pub unsafe fn diff_unknown_1421381f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> ([u32; 6], [u8; 2]) {
        let mut g = [0u8; 0x20];
        let gp = g.as_mut_ptr() as usize;
        wr::<f32>(gp, 0.5);
        wr::<i32>(gp + 0x08, 1);
        wr::<i32>(gp + 0x0c, 2);

        let mut vecs = [10.0f32, 10.0f32, 10.0f32, 0.1f32, 0.2f32, 0.1f32];
        let mut flags = [0x01u8, 0x05u8];
        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, vecs.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x18, flags.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, gp);

        let p2 = [5.0f32, 5.0f32, 1.0f32];
        f(p1p, p2.as_ptr() as usize);
        (vecs.map(f32::to_bits), flags)
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "1421381f0 out");

    // Seeded random grids: exact ties (quantized values), FLT_MAX holes, all cone params.
    let mut rng = crate::difftest::Rng::new(0x1421_381f);
    let mut cases = 1u32;
    for case in 0..200u32 {
        let dim_xz = 1 + rng.below(6) as i32;
        let dim_y = 1 + rng.below(5) as i32;
        let n_vox = (dim_xz * dim_xz * dim_y) as usize;
        let voxel = [0.25f32, 0.5, 1.0, 0.3][rng.below(4) as usize];
        let mut vecs0 = vec![0f32; n_vox * 3];
        for v in vecs0.chunks_mut(3) {
            match rng.below(4) {
                0 => v.fill(f32::MAX),
                1 => v[rng.below(3) as usize] = f32::MAX,
                _ => {
                    for c in v.iter_mut() {
                        *c = if case % 2 == 0 {
                            (rng.below(17) as f32 - 4.0) * 0.25
                        } else {
                            rng.f32_any() * 0.05
                        };
                    }
                }
            }
        }
        let flags0 = rng.bytes(n_vox);
        let p2 = [
            [5.0f32, 5.0, 1.0],
            [0.5, 0.25, 2.0],
            [1.0, 0.0, 0.5],
            [rng.f32_any().abs(), rng.f32_any().abs(), rng.f32_any().abs() + 0.01],
        ][rng.below(4) as usize];
        let go = |f: F| -> (Vec<u32>, Vec<u8>) {
            let mut g = [0u8; 0x20];
            let gp = g.as_mut_ptr() as usize;
            wr::<f32>(gp, voxel);
            wr::<i32>(gp + 0x08, dim_xz);
            wr::<i32>(gp + 0x0c, dim_y);
            let mut vecs = vecs0.clone();
            let mut flags = flags0.clone();
            let mut p1 = [0u8; 0x30];
            let p1p = p1.as_mut_ptr() as usize;
            wr::<usize>(p1p + 0x08, vecs.as_mut_ptr() as usize);
            wr::<usize>(p1p + 0x18, flags.as_mut_ptr() as usize);
            wr::<usize>(p1p + 0x28, gp);
            f(p1p, p2.as_ptr() as usize);
            (vecs.iter().map(|x| x.to_bits()).collect(), flags)
        };
        let o = go(f_o);
        let n = go(f_n);
        crate::diff_eq!(o, n, "1421381f0 random case {case} dims {dim_xz}x{dim_y}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_unknown_142139350(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> ([u32; 3], u8) {
        let mut g = [0u8; 0x20];
        let gp = g.as_mut_ptr() as usize;
        wr::<f32>(gp, 1.0);
        wr::<f32>(gp + 0x04, 1.0);
        wr::<i32>(gp + 0x08, 1);
        wr::<i32>(gp + 0x0c, 1);
        wr::<i32>(gp + 0x10, 0);
        wr::<i32>(gp + 0x14, 0);
        wr::<i32>(gp + 0x18, 0);

        let mut vecs = [10.0f32, 10.0f32, 10.0f32];
        let mut flags = [0u8; 1];
        let mut p1 = [0u8; 0x30];
        let p1p = p1.as_mut_ptr() as usize;
        wr::<usize>(p1p + 0x08, vecs.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x18, flags.as_mut_ptr() as usize);
        wr::<usize>(p1p + 0x28, gp);

        let tri = [-0.5f32, 0.25f32, -0.5f32, 0.5f32, 0.25f32, -0.5f32, 0.0f32, 0.25f32, 0.5f32];
        let mut submesh = [0u8; 0x38];
        let smp = submesh.as_mut_ptr() as usize;
        wr::<u8>(smp + 0x2c, 0);
        wr::<u32>(smp + 0x30, 0);
        wr::<u32>(smp + 0x34, 1);

        let mut p2 = [0u8; 0x80];
        let p2p = p2.as_mut_ptr() as usize;
        wr::<usize>(p2p + 0x10, smp);
        wr::<u32>(p2p + 0x18, 1);
        wr::<usize>(p2p + 0x20, tri.as_ptr() as usize);
        wr::<f64>(p2p + 0x60, 0.0);
        wr::<f64>(p2p + 0x68, 0.0);
        wr::<f64>(p2p + 0x70, 0.0);

        f(p1p, p2p);
        (vecs.map(f32::to_bits), flags[0])
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142139350 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141165830(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut GOT: [u64; 3] = [0; 3];
    unsafe extern "win64" fn mock_fcc240(arg1: usize, _rsi: usize, _r8: usize, _r9: usize) {
        GOT = rd::<[u64; 3]>(arg1);
    }
    let _m1 = crate::difftest::Mock::new(0x1_41fc_c240, mock_fcc240 as *const ())?;

    let g_a9c = exe::ptr::<u16>(0x1_43ce_2a9c) as usize;
    let g_910 = exe::ptr::<u16>(0x1_43c8_d910) as usize;
    let s_a9c = rd::<u16>(g_a9c);
    let s_910 = rd::<u16>(g_910);
    wr::<u16>(g_a9c, 0);
    wr::<u16>(g_910, 0);

    let run = |f: F| -> [u64; 3] {
        let vtbl_20 = [0u16; 4];
        let mut hdr = [0u8; 0x30];
        wr::<usize>(hdr.as_mut_ptr() as usize + 0x20, vtbl_20.as_ptr() as usize);
        let arr_08 = [0x1122_3344u64, 5u64];
        let tbl_580e0 = [0x9988u64; 2];

        let mut obj = vec![0u8; 0x58200];
        let op = obj.as_mut_ptr() as usize;
        wr::<usize>(op, hdr.as_ptr() as usize);
        wr::<usize>(op + 0x08, arr_08.as_ptr() as usize);
        wr::<usize>(op + 0x580e0, tbl_580e0.as_ptr() as usize);

        let p1 = [op];
        GOT = [0; 3];
        f(p1.as_ptr() as usize);
        GOT
    };
    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_a9c, s_a9c);
    wr::<u16>(g_910, s_910);
    crate::diff_eq!(o[1..], n[1..], "141165830 out");
    Ok(1)
}

pub unsafe fn diff_kaim_141225bf0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> u32;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_e60(_a: usize, _b: u64, _c: usize, out_flag: usize) -> u32 {
        wr::<u8>(out_flag, 1);
        0
    }
    unsafe extern "win64" fn mock_190(_a: usize) -> u8 {
        1
    }
    let _m1 = crate::difftest::Mock::new(0x1_416b_0e60, mock_e60 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4121_7190, mock_190 as *const ())?;

    let mut p1 = [0u64; 8];
    let mut p2 = [0u64; 2];
    let o = f_o(p1.as_mut_ptr() as usize, p2.as_mut_ptr() as usize);
    let n = f_n(p1.as_mut_ptr() as usize, p2.as_mut_ptr() as usize);
    crate::diff_eq!(o, n, "141225bf0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141eaf160(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, _span: usize) -> usize {
        wr::<u64>(out, r11);
        wr::<u64>(out + 8, 0x1000);
        wr::<u64>(out + 16, 0x1000);
        out
    }
    unsafe extern "win64" fn noop1(_a: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4103_8a90, noop1 as *const ())?;

    let p1 = [0x2000u64, 0u64, 0u64];
    f_o(p1.as_ptr() as usize);
    f_n(p1.as_ptr() as usize);
    Ok(1)
}

pub unsafe fn diff_unknown_141eaf3a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, _span: usize) -> usize {
        wr::<u64>(out, r11);
        wr::<u64>(out + 8, 0x1000);
        wr::<u64>(out + 16, 0x1000);
        out
    }
    unsafe extern "win64" fn noop1(_a: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4103_8bd0, noop1 as *const ())?;

    let p1 = [0x2000u64, 0u64, 0u64];
    f_o(p1.as_ptr() as usize);
    f_n(p1.as_ptr() as usize);
    Ok(1)
}

pub unsafe fn diff_unknown_141ec6a90(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let run = |f: F| -> (u8, u8, u32, u64, u16) {
        let mut flags = [1u8, 0u8];
        let u32_arr = [2u32];
        let mut item = [0u8; 0x20];
        let ip = item.as_mut_ptr() as usize;
        wr::<u64>(ip + 0x10, 0xdead_beef);
        wr::<u32>(ip + 0x1c, 4);

        let mut vec_buf = [0u64; 4];
        let mut p3 = [0u8; 0x10];
        let p3p = p3.as_mut_ptr() as usize;
        wr::<usize>(p3p, vec_buf.as_mut_ptr() as usize);
        wr::<u32>(p3p + 8, 0);
        wr::<u32>(p3p + 12, 2);

        let p1 = [
            flags.as_mut_ptr() as usize,
            u32_arr.as_ptr() as usize,
            ip,
            0usize,
            0usize,
        ];
        f(p1.as_ptr() as usize, 0, p3p);
        (
            flags[0],
            flags[1],
            rd::<u32>(p3p + 8),
            vec_buf[0],
            vec_buf[1] as u16,
        )
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141ec6a90 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141ef1010(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop2(_a: usize, _b: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_41ee_c2e0, noop2 as *const ())?;

    let g_eec = exe::ptr::<u16>(0x1_43c8_ceec) as usize;
    let g_bac = exe::ptr::<u16>(0x1_43cd_6bac) as usize;
    let g_e88 = exe::ptr::<u16>(0x1_43c8_ce88) as usize;
    let g_658 = exe::ptr::<u16>(0x1_43cd_5658) as usize;
    let g_ba8 = exe::ptr::<u16>(0x1_43cd_6ba8) as usize;
    let s0 = rd::<u16>(g_eec);
    let s1 = rd::<u16>(g_bac);
    let s2 = rd::<u16>(g_e88);
    let s3 = rd::<u16>(g_658);
    let s4 = rd::<u16>(g_ba8);
    wr::<u16>(g_eec, 0);
    wr::<u16>(g_bac, 1);
    wr::<u16>(g_e88, 2);
    wr::<u16>(g_658, 3);
    wr::<u16>(g_ba8, 0);

    let run = |f: F| -> u32 {
        let mut r15 = [0u64; 4];
        let vtbl_20 = [0u16; 4];
        let mut hdr = [0u8; 0x30];
        wr::<usize>(hdr.as_mut_ptr() as usize + 0x20, vtbl_20.as_ptr() as usize);
        let arr_08 = [0u64; 4];
        let slot3 = [0u64; 4];
        let tbl_580e0 = [
            0usize,
            r15.as_mut_ptr() as usize,
            0usize,
            slot3.as_ptr() as usize,
        ];
        let mut p1 = vec![0u8; 0x58100];
        let pp = p1.as_mut_ptr() as usize;
        wr::<usize>(pp, hdr.as_ptr() as usize);
        wr::<usize>(pp + 0x08, arr_08.as_ptr() as usize);
        wr::<usize>(pp + 0x580e0, tbl_580e0.as_ptr() as usize);
        f(pp);
        rd::<u32>(r15.as_ptr() as usize + 8)
    };
    let o = run(f_o);
    let n = run(f_n);
    wr::<u16>(g_eec, s0);
    wr::<u16>(g_bac, s1);
    wr::<u16>(g_e88, s2);
    wr::<u16>(g_658, s3);
    wr::<u16>(g_ba8, s4);
    crate::diff_eq!(o, n, "141ef1010 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141f444f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let flag = 0u8;
    let p1 = [0u64; 3];
    let p4 = [0u64; 8];
    f_o(p1.as_ptr() as usize, 0, (&flag as *const u8) as usize, p4.as_ptr() as usize);
    f_n(p1.as_ptr() as usize, 0, (&flag as *const u8) as usize, p4.as_ptr() as usize);
    Ok(1)
}

pub unsafe fn diff_coregame_141f71fc0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop2(_a: usize, _b: u64) {}
    unsafe extern "win64" fn mock_iter_begin(out: usize, r11: u64, _span: usize) -> usize {
        wr::<u64>(out, r11);
        wr::<u64>(out + 8, 0x1000);
        wr::<u64>(out + 16, 0x1000);
        out
    }
    unsafe extern "win64" fn noop1(_a: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_410e_f200, noop2 as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_401e_7c10, mock_iter_begin as *const ())?;
    let _m3 = crate::difftest::Mock::new(0x1_4106_f2b0, noop1 as *const ())?;

    let run = |f: F| -> u32 {
        let mut p1 = [0u8; 0x110];
        let pp = p1.as_mut_ptr() as usize;
        wr::<u32>(pp + 0xf8, 9);
        let p2 = [0u8; 0x30];
        let p3 = [0x2000u64, 0u64, 0u64];
        f(pp, p2.as_ptr() as usize, p3.as_ptr() as usize);
        rd::<u32>(pp + 0xf8)
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "141f71fc0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_141fec6a0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let p1 = [0u64; 8];
    let p2 = [0u64; 24];
    f_o(p1.as_ptr() as usize, p2.as_ptr() as usize);
    f_n(p1.as_ptr() as usize, p2.as_ptr() as usize);
    Ok(1)
}

pub unsafe fn diff_unknown_1420104f0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    let g_948 = exe::ptr::<u16>(0x1_43cc_8948) as usize;
    let s_948 = rd::<u16>(g_948);
    wr::<u16>(g_948, 0);

    let rbp_obj = [0u8; 0x20];
    let tbl_580e0 = [rbp_obj.as_ptr() as usize];
    let mut p1 = vec![0u8; 0x58100];
    wr::<usize>(p1.as_mut_ptr() as usize + 0x580e0, tbl_580e0.as_ptr() as usize);

    let o = f_o(p1.as_ptr() as usize);
    let n = f_n(p1.as_ptr() as usize);
    wr::<u16>(g_948, s_948);
    crate::diff_eq!(o, n, "1420104f0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142073a40(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut BARRIER_CNT: u64 = 0;
    unsafe extern "win64" fn mock_barrier(_cmd: usize, cnt: u64, _ptr: usize) {
        BARRIER_CNT = cnt;
    }
    let mut vtbl = [0u64; 32];
    vtbl[0xd0 / 8] = exe::ptr::<u8>(0x1_40f6_7380) as u64;
    let _m1 = crate::difftest::Mock::new(0x1_40f6_7380, mock_barrier as *const ())?;

    static mut CMD_PTR: usize = 0;
    unsafe extern "win64" fn mock_d5b00(out_pair: usize) -> usize {
        wr::<usize>(out_pair, 0);
        wr::<usize>(out_pair + 8, CMD_PTR);
        out_pair
    }
    let _m2 = crate::difftest::Mock::new(0x1_411d_5b00, mock_d5b00 as *const ())?;

    let cmd_obj = [vtbl.as_ptr() as u64];
    CMD_PTR = cmd_obj.as_ptr() as usize;

    let run = |f: F| -> (u32, u64) {
        let mut obj = [0u8; 0x60];
        let op = obj.as_mut_ptr() as usize;
        wr::<u64>(op + 0x40, 0x1234);
        let arr = [op];
        BARRIER_CNT = 99;
        f(arr.as_ptr() as usize, 1);
        (rd::<u32>(op + 0x48), BARRIER_CNT)
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "142073a40 out");
    Ok(1)
}

pub unsafe fn diff_r_1421444b0(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    static mut CALLS: u32 = 0;
    unsafe extern "win64" fn mock_run(_t: usize) {
        CALLS += 1;
    }
    unsafe extern "win64" fn mock_rel(_t: usize) {
        CALLS += 10;
    }
    let _m1 = crate::difftest::Mock::new(0x1_40f6_7380, mock_run as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4209_3200, mock_rel as *const ())?;

    let mut vtbl = [0u64; 8];
    vtbl[1] = exe::ptr::<u8>(0x1_40f6_7380) as u64;
    vtbl[6] = exe::ptr::<u8>(0x1_4209_3200) as u64;
    let task = [vtbl.as_ptr() as u64];
    let job = [0usize, task.as_ptr() as usize];

    CALLS = 0;
    f_o(job.as_ptr() as usize);
    let o = CALLS;
    CALLS = 0;
    f_n(job.as_ptr() as usize);
    let n = CALLS;
    crate::diff_eq!(o, n, "1421444b0 out");
    Ok(1)
}

pub unsafe fn diff_unknown_14223d920(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, i32, usize);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop7(
        _a: u64,
        _b: u64,
        _c: u64,
        _d: u64,
        _e: u64,
        _f: u64,
        _g: u64,
    ) {
    }
    let _m1 = crate::difftest::Mock::new(0x1_4223_8b60, noop7 as *const ())?;

    let run = |f: F| -> ([u32; 4], u32) {
        let mut dst = [0u32; 16];
        let mut p1 = [0u8; 0x100];
        let pp = p1.as_mut_ptr() as usize;
        let tid = exe::call_iat!(0x1_4264_5790, () -> u32);
        wr::<u32>(pp + 0x14, 0x7788);
        wr::<usize>(pp + 0x88, dst.as_mut_ptr() as usize);
        wr::<u32>(pp + 0x90, 0);
        wr::<u32>(pp + 0x94, 4);
        wr::<u32>(pp + 0xb0, tid);
        wr::<u32>(pp + 0xb4, 1);

        let pts = [2.0f32, 3.0f32];
        let mat = [
            1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];
        f(pp, pts.as_ptr() as usize, 1, mat.as_ptr() as usize);
        ([dst[0], dst[1], dst[2], dst[3]], dst[7])
    };
    let o = run(f_o);
    let n = run(f_n);
    crate::diff_eq!(o, n, "14223d920 out");
    Ok(1)
}

pub unsafe fn diff_unknown_142350200(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    unsafe extern "win64" fn noop1(_a: usize) {}
    let _m1 = crate::difftest::Mock::new(0x1_412f_7fb0, noop1 as *const ())?;

    let g_8d0 = exe::ptr::<u8>(0x1_4386_c8d0) as usize;
    let s_8d0 = rd::<u8>(g_8d0);
    wr::<u8>(g_8d0, 0);

    let mut p1 = [0u8; 0x80];
    f_o(p1.as_mut_ptr() as usize, 0);
    f_n(p1.as_mut_ptr() as usize, 0);
    wr::<u8>(g_8d0, s_8d0);
    Ok(1)
}

pub unsafe fn diff_ui_1424f6500(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, i32, u64, u64);
    let f_o: F = core::mem::transmute(orig);
    let f_n: F = core::mem::transmute(new);

    f_o(0, -1, 0, 0);
    f_n(0, -1, 0, 0);
    Ok(1)
}

// REGISTRY:
// FLOAT_RETURN: none (no function in this batch returns f32/f64 in xmm0)
//
// Replacement::verified_throws(0x1_4224_e6e0, "rend_14224e6e0", rend::rend_e17_c::rend_14224e6e0 as *const (), Throws::Discard, rend::rend_e17_c::diff_rend_14224e6e0),
// Replacement::verified(0x1_4224_ebf0, "rend_14224ebf0", rend::rend_e17_c::rend_14224ebf0 as *const (), rend::rend_e17_c::diff_rend_14224ebf0),
// Replacement::verified_throws(0x1_4224_ec60, "rend_14224ec60", rend::rend_e17_c::rend_14224ec60 as *const (), Throws::Discard, rend::rend_e17_c::diff_rend_14224ec60),
// Replacement::verified_throws(0x1_4224_f1e0, "rend_14224f1e0", rend::rend_e17_c::rend_14224f1e0 as *const (), Throws::Caught, rend::rend_e17_c::diff_rend_14224f1e0),
// Replacement::verified(0x1_4224_f320, "rend_14224f320", rend::rend_e17_c::rend_14224f320 as *const (), rend::rend_e17_c::diff_rend_14224f320),
// Replacement::verified(0x1_4224_f470, "rend_14224f470", rend::rend_e17_c::rend_14224f470 as *const (), rend::rend_e17_c::diff_rend_14224f470),
// Replacement::verified(0x1_4224_f530, "rend_14224f530", rend::rend_e17_c::rend_14224f530 as *const (), rend::rend_e17_c::diff_rend_14224f530),
// Replacement::verified(0x1_4224_fab0, "rend_14224fab0", rend::rend_e17_c::rend_14224fab0 as *const (), rend::rend_e17_c::diff_rend_14224fab0),
// Replacement::verified(0x1_4225_0930, "rend_142250930", rend::rend_e17_c::rend_142250930 as *const (), rend::rend_e17_c::diff_rend_142250930),
// Replacement::verified(0x1_4225_0970, "unknown_142250970", rend::rend_e17_c::unknown_142250970 as *const (), rend::rend_e17_c::diff_unknown_142250970),
// Replacement::verified(0x1_4225_09c0, "unknown_1422509c0", rend::rend_e17_c::unknown_1422509c0 as *const (), rend::rend_e17_c::diff_unknown_1422509c0),
// Replacement::verified(0x1_4225_0a20, "unknown_142250a20", rend::rend_e17_c::unknown_142250a20 as *const (), rend::rend_e17_c::diff_unknown_142250a20),
// Replacement::verified_throws(0x1_4225_0c30, "unknown_142250c30", rend::rend_e17_c::unknown_142250c30 as *const (), Throws::Discard, rend::rend_e17_c::diff_unknown_142250c30),
// Replacement::verified(0x1_4225_fa00, "unknown_14225fa00", rend::rend_e17_c::unknown_14225fa00 as *const (), rend::rend_e17_c::diff_unknown_14225fa00),
// Replacement::verified(0x1_4226_11a0, "unknown_1422611a0", rend::rend_e17_c::unknown_1422611a0 as *const (), rend::rend_e17_c::diff_unknown_1422611a0),
// Replacement::verified(0x1_4226_1ae0, "unknown_142261ae0", rend::rend_e17_c::unknown_142261ae0 as *const (), rend::rend_e17_c::diff_unknown_142261ae0),
// Replacement::verified(0x1_4226_1b50, "unknown_142261b50", rend::rend_e17_c::unknown_142261b50 as *const (), rend::rend_e17_c::diff_unknown_142261b50),
// Replacement::verified(0x1_4226_1cf0, "unknown_142261cf0", rend::rend_e17_c::unknown_142261cf0 as *const (), rend::rend_e17_c::diff_unknown_142261cf0),
// Replacement::verified(0x1_4226_22e0, "unknown_1422622e0", rend::rend_e17_c::unknown_1422622e0 as *const (), rend::rend_e17_c::diff_unknown_1422622e0),
// Replacement::verified(0x1_4226_2310, "unknown_142262310", rend::rend_e17_c::unknown_142262310 as *const (), rend::rend_e17_c::diff_unknown_142262310),
// Replacement::verified(0x1_4226_2d70, "unknown_142262d70", rend::rend_e17_c::unknown_142262d70 as *const (), rend::rend_e17_c::diff_unknown_142262d70),
// Replacement::verified(0x1_4226_3030, "unknown_142263030", rend::rend_e17_c::unknown_142263030 as *const (), rend::rend_e17_c::diff_unknown_142263030),
// Replacement::verified(0x1_4226_3120, "unknown_142263120", rend::rend_e17_c::unknown_142263120 as *const (), rend::rend_e17_c::diff_unknown_142263120),
// Replacement::verified(0x1_4226_3140, "unknown_142263140", rend::rend_e17_c::unknown_142263140 as *const (), rend::rend_e17_c::diff_unknown_142263140),
// Replacement::verified(0x1_4226_3190, "unknown_142263190", rend::rend_e17_c::unknown_142263190 as *const (), rend::rend_e17_c::diff_unknown_142263190),
// Replacement::verified_throws(0x1_4226_39d0, "std_1422639d0", rend::rend_e17_c::std_1422639d0 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_1422639d0),
// Replacement::verified_throws(0x1_4226_3a60, "std_142263a60", rend::rend_e17_c::std_142263a60 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_142263a60),
// Replacement::verified(0x1_4226_3ce0, "std_142263ce0", rend::rend_e17_c::std_142263ce0 as *const (), rend::rend_e17_c::diff_std_142263ce0),
// Replacement::verified(0x1_4226_4340, "std_142264340", rend::rend_e17_c::std_142264340 as *const (), rend::rend_e17_c::diff_std_142264340),
// Replacement::verified_throws(0x1_4226_4400, "std_142264400", rend::rend_e17_c::std_142264400 as *const (), Throws::Caught, rend::rend_e17_c::diff_std_142264400),
// Replacement::verified(0x1_4226_48d0, "std_1422648d0", rend::rend_e17_c::std_1422648d0 as *const (), rend::rend_e17_c::diff_std_1422648d0),
// Replacement::verified_throws(0x1_4226_4a50, "std_142264a50", rend::rend_e17_c::std_142264a50 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_142264a50),
// Replacement::verified_throws(0x1_4226_4b50, "std_142264b50", rend::rend_e17_c::std_142264b50 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_142264b50),
// Replacement::verified_throws(0x1_4226_4ed0, "std_142264ed0", rend::rend_e17_c::std_142264ed0 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_142264ed0),
// Replacement::verified(0x1_4226_5440, "std_142265440", rend::rend_e17_c::std_142265440 as *const (), rend::rend_e17_c::diff_std_142265440),
// Replacement::verified(0x1_4226_54d0, "std_1422654d0", rend::rend_e17_c::std_1422654d0 as *const (), rend::rend_e17_c::diff_std_1422654d0),
// Replacement::verified_throws(0x1_4226_5590, "std_142265590", rend::rend_e17_c::std_142265590 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_142265590),
// Replacement::verified_throws(0x1_4226_56a0, "std_1422656a0", rend::rend_e17_c::std_1422656a0 as *const (), Throws::Caught, rend::rend_e17_c::diff_std_1422656a0),
// Replacement::verified_throws(0x1_4226_57f0, "std_1422657f0", rend::rend_e17_c::std_1422657f0 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_1422657f0),
// Replacement::verified_throws(0x1_4226_59b0, "std_1422659b0", rend::rend_e17_c::std_1422659b0 as *const (), Throws::Caught, rend::rend_e17_c::diff_std_1422659b0),
// Replacement::verified_throws(0x1_4226_5aa0, "std_142265aa0", rend::rend_e17_c::std_142265aa0 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_142265aa0),
// Replacement::verified_throws(0x1_4226_5f20, "std_142265f20", rend::rend_e17_c::std_142265f20 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_142265f20),
// Replacement::verified_throws(0x1_4226_61f0, "std_1422661f0", rend::rend_e17_c::std_1422661f0 as *const (), Throws::Caught, rend::rend_e17_c::diff_std_1422661f0),
// Replacement::verified_throws(0x1_4226_63b0, "std_1422663b0", rend::rend_e17_c::std_1422663b0 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_1422663b0),
// Replacement::verified_throws(0x1_4226_6570, "std_142266570", rend::rend_e17_c::std_142266570 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_142266570),
// Replacement::verified_throws(0x1_4226_66f0, "std_1422666f0", rend::rend_e17_c::std_1422666f0 as *const (), Throws::Discard, rend::rend_e17_c::diff_std_1422666f0),
// Replacement::verified(0x1_4226_67a0, "std_1422667a0", rend::rend_e17_c::std_1422667a0 as *const (), rend::rend_e17_c::diff_std_1422667a0),
// Replacement::verified(0x1_4226_6940, "std_142266940", rend::rend_e17_c::std_142266940 as *const (), rend::rend_e17_c::diff_std_142266940),
