//! AsyncStreamer, LZ4 block decompression, and VFS File/MemoryFile I/O hot-path
//! functions (`0x14246b9e0`, `0x14208a430`, `0x1424434d0`, `0x142443400`,
//! `0x142498890`, `0x142497cc0`, `0x142496ae0`, `0x142475db0`, `0x14243b0b0`,
//! `0x142052510`) and differential tests.

use crate::difftest::{self, Rng};
use crate::exe;
use std::sync::atomic::{AtomicI32, Ordering};

const J_MEMCPY_VA: u64 = 0x1_4183_acbc;
const J_MEMMOVE_VA: u64 = 0x1_4183_acc8;
const J_MEMSET_VA: u64 = 0x1_4183_acc2;

const IAT_ACQUIRE_SRW_EXCLUSIVE: u64 = 0x1_4264_55a8;
const IAT_RELEASE_SRW_EXCLUSIVE: u64 = 0x1_4264_5598;
const IAT_GET_CURRENT_THREAD_ID: u64 = 0x1_4264_5790;
const IAT_XBAD_FUNCTION_CALL: u64 = 0x1_4264_5878;

const VFS_GLOBAL_PTR_VA: u64 = 0x1_43a0_8320;
const STREAMER_MGR_PTR_VA: u64 = 0x1_4397_fc60;
const SLAB_ALLOC_GLOBAL_VA: u64 = 0x1_4397_fb98;

#[inline(always)]
fn lz4_err(src: usize, ip: usize) -> i32 {
    (src as u32).wrapping_sub(ip as u32).wrapping_sub(1) as i32
}

// @ 0x14246b9e0 verified — LZ4_decompress_safe
pub unsafe extern "win64" fn lz4_decompress_safe(
    src: *const u8,
    dst: *mut u8,
    compressed_size: i32,
    dst_capacity: i32,
) -> i32 {
    if src.is_null() {
        return -1;
    }
    let src_base = src as usize;
    let dst_base = dst as usize;
    let mut ip = src_base;
    let mut op = dst_base;
    let iend = src_base.wrapping_add(compressed_size as isize as usize);
    let oend = dst_base.wrapping_add(dst_capacity as isize as usize);

    if dst_capacity == 0 {
        if compressed_size == 1 && exe::rd::<u8>(ip) == 0 {
            return 0;
        }
        return -1;
    }
    if compressed_size == 0 {
        return -1;
    }

    const INC32: [usize; 8] = [0, 1, 2, 1, 0, 4, 4, 4];
    const DEC64: [isize; 8] = [0, 0, 0, -1, -4, 1, 2, 3];

    let mut token: usize;
    let mut lit_len: usize;
    let mut cpy: usize;
    let mut match_src: usize;
    let mut offset: usize;
    let mut match_len: usize;

    enum State {
        FastLoop,
        SafeLoop,
        SafeLiteralCopy,
        SafeMatchLen,
        SafeMatchCheck,
        SafeMatchCopy,
        EndOfBlock,
    }

    let mut state = if (dst_capacity as i64) >= 0x40 {
        State::FastLoop
    } else {
        State::SafeLoop
    };
    token = 0;
    lit_len = 0;
    cpy = 0;
    match_src = 0;
    offset = 0;
    match_len = 0;

    loop {
        match state {
            State::FastLoop => {
                token = exe::rd::<u8>(ip) as usize;
                ip = ip.wrapping_add(1);
                lit_len = token >> 4;
                if lit_len == 15 {
                    let lim = iend.wrapping_sub(0x0f);
                    if ip >= lim {
                        return lz4_err(src_base, ip);
                    }
                    let mut s = 0u32;
                    loop {
                        let b = exe::rd::<u8>(ip) as u32;
                        ip = ip.wrapping_add(1);
                        s = s.wrapping_add(b);
                        if ip >= lim || b != 0xff {
                            break;
                        }
                    }
                    lit_len = (s as usize).wrapping_add(15);
                    cpy = op.wrapping_add(lit_len);
                    if cpy < op {
                        return lz4_err(src_base, ip);
                    }
                    let ip_after = ip.wrapping_add(lit_len);
                    if ip_after < ip {
                        return lz4_err(src_base, ip);
                    }
                    if cpy > oend.wrapping_sub(0x20) || ip_after > iend.wrapping_sub(0x20) {
                        state = State::SafeLiteralCopy;
                        continue;
                    }
                    let mut p = ip;
                    let mut d = op;
                    while d < cpy {
                        exe::wr::<u128>(d, exe::rd::<u128>(p));
                        exe::wr::<u128>(d + 16, exe::rd::<u128>(p + 16));
                        p = p.wrapping_add(32);
                        d = d.wrapping_add(32);
                    }
                    ip = ip_after;
                } else {
                    cpy = op.wrapping_add(lit_len);
                    if ip > iend.wrapping_sub(0x11) {
                        state = State::SafeLiteralCopy;
                        continue;
                    }
                    exe::wr::<u128>(op, exe::rd::<u128>(ip));
                    ip = ip.wrapping_add(lit_len);
                }
                op = cpy;
                offset = exe::rd::<u16>(ip) as usize;
                match_src = cpy.wrapping_sub(offset);
                ip = ip.wrapping_add(2);
                match_len = token & 0x0f;
                if match_len == 15 {
                    if match_src < dst_base {
                        return lz4_err(src_base, ip);
                    }
                    let lim = iend.wrapping_sub(4);
                    let mut s = 0u32;
                    loop {
                        let b = exe::rd::<u8>(ip) as u32;
                        ip = ip.wrapping_add(1);
                        s = s.wrapping_add(b);
                        if ip >= lim {
                            return lz4_err(src_base, ip);
                        }
                        if b != 0xff {
                            break;
                        }
                    }
                    match_len = (s as usize).wrapping_add(15);
                    if cpy.wrapping_add(match_len) < cpy {
                        return lz4_err(src_base, ip);
                    }
                    match_len = match_len.wrapping_add(4);
                    let m_end = cpy.wrapping_add(match_len);
                    if m_end >= oend.wrapping_sub(0x40) {
                        state = State::SafeMatchCopy;
                        continue;
                    }
                } else {
                    match_len = match_len.wrapping_add(4);
                    let m_end = cpy.wrapping_add(match_len);
                    if m_end >= oend.wrapping_sub(0x40) {
                        state = State::SafeMatchCheck;
                        continue;
                    }
                    if match_src < dst_base {
                        return lz4_err(src_base, ip);
                    }
                    if offset >= 8 {
                        exe::wr::<u64>(cpy, exe::rd::<u64>(match_src));
                        exe::wr::<u64>(cpy + 8, exe::rd::<u64>(match_src + 8));
                        exe::wr::<u16>(cpy + 16, exe::rd::<u16>(match_src + 16));
                        op = m_end;
                        continue;
                    }
                }
                let m_end = cpy.wrapping_add(match_len);
                if offset < 16 {
                    if offset == 1 || offset == 2 || offset == 4 {
                        let pat: u64 = if offset == 1 {
                            let b = exe::rd::<u8>(match_src) as u64;
                            b * 0x0101_0101_0101_0101
                        } else if offset == 2 {
                            let w = exe::rd::<u16>(match_src) as u64;
                            w | (w << 16) | (w << 32) | (w << 48)
                        } else {
                            let d = exe::rd::<u32>(match_src) as u64;
                            d | (d << 32)
                        };
                        exe::wr::<u64>(op, pat);
                        op = op.wrapping_add(8);
                        if op <= m_end {
                            let nq = m_end.wrapping_sub(op).wrapping_add(7) >> 3;
                            for q in 0..nq {
                                exe::wr::<u64>(op + q * 8, pat);
                            }
                        }
                        op = m_end;
                        continue;
                    }
                    let mut m_ptr = if offset < 8 {
                        exe::wr::<u32>(op, 0);
                        for k in 0..4 {
                            exe::wr::<u8>(op + k, exe::rd::<u8>(match_src + k));
                        }
                        let mp = match_src.wrapping_add(INC32[offset]);
                        exe::wr::<u32>(op + 4, exe::rd::<u32>(mp));
                        mp.wrapping_sub(DEC64[offset] as usize)
                    } else {
                        exe::wr::<u64>(op, exe::rd::<u64>(match_src));
                        match_src.wrapping_add(8)
                    };
                    op = op.wrapping_add(8);
                    while op < m_end {
                        exe::wr::<u64>(op, exe::rd::<u64>(m_ptr));
                        op = op.wrapping_add(8);
                        m_ptr = m_ptr.wrapping_add(8);
                    }
                    op = m_end;
                    continue;
                }
                let mut m_ptr = match_src;
                let mut d_ptr = op;
                while d_ptr < m_end {
                    exe::wr::<u128>(d_ptr, exe::rd::<u128>(m_ptr));
                    exe::wr::<u128>(d_ptr + 16, exe::rd::<u128>(m_ptr + 16));
                    m_ptr = m_ptr.wrapping_add(32);
                    d_ptr = d_ptr.wrapping_add(32);
                }
                op = m_end;
            }
            State::SafeLoop => {
                token = exe::rd::<u8>(ip) as usize;
                ip = ip.wrapping_add(1);
                lit_len = token >> 4;
                if lit_len != 15 {
                    loop {
                        if op <= oend.wrapping_sub(0x20) && ip < iend.wrapping_sub(0x10) {
                            exe::wr::<u128>(op, exe::rd::<u128>(ip));
                            ip = ip.wrapping_add(lit_len);
                            op = op.wrapping_add(lit_len);
                            match_len = token & 0x0f;
                            offset = exe::rd::<u16>(ip) as usize;
                            match_src = op.wrapping_sub(offset);
                            ip = ip.wrapping_add(2);
                            if match_len == 15 {
                                state = State::SafeMatchLen;
                                break;
                            }
                            if offset < 8 || match_src < dst_base {
                                state = State::SafeMatchLen;
                                break;
                            }
                            match_len = match_len.wrapping_add(4);
                            exe::wr::<u64>(op, exe::rd::<u64>(match_src));
                            exe::wr::<u64>(op + 8, exe::rd::<u64>(match_src + 8));
                            exe::wr::<u16>(op + 16, exe::rd::<u16>(match_src + 16));
                            op = op.wrapping_add(match_len);
                            token = exe::rd::<u8>(ip) as usize;
                            ip = ip.wrapping_add(1);
                            lit_len = token >> 4;
                            if lit_len != 15 {
                                continue;
                            }
                        }
                        break;
                    }
                    if matches!(state, State::SafeMatchLen) {
                        continue;
                    }
                }
                if lit_len == 15 {
                    let lim = iend.wrapping_sub(0x0f);
                    if ip >= lim {
                        return lz4_err(src_base, ip);
                    }
                    let mut s = 0u32;
                    loop {
                        let b = exe::rd::<u8>(ip) as u32;
                        ip = ip.wrapping_add(1);
                        s = s.wrapping_add(b);
                        if ip >= lim || b != 0xff {
                            break;
                        }
                    }
                    lit_len = (s as usize).wrapping_add(lit_len);
                    if op.wrapping_add(lit_len) < op || ip.wrapping_add(lit_len) < ip {
                        return lz4_err(src_base, ip);
                    }
                }
                cpy = op.wrapping_add(lit_len);
                state = State::SafeLiteralCopy;
            }
            State::SafeLiteralCopy => {
                let ip_after = ip.wrapping_add(lit_len);
                if cpy > oend.wrapping_sub(0x0c) || ip_after > iend.wrapping_sub(8) {
                    state = State::EndOfBlock;
                    continue;
                }
                let mut p = ip;
                while op < cpy {
                    exe::wr::<u64>(op, exe::rd::<u64>(p));
                    op = op.wrapping_add(8);
                    p = p.wrapping_add(8);
                }
                ip = ip_after;
                op = cpy;
                offset = exe::rd::<u16>(ip) as usize;
                match_src = cpy.wrapping_sub(offset);
                ip = ip.wrapping_add(2);
                match_len = token & 0x0f;
                state = State::SafeMatchLen;
            }
            State::SafeMatchLen => {
                if match_len == 15 {
                    let lim = iend.wrapping_sub(4);
                    let mut s = 0u32;
                    loop {
                        let b = exe::rd::<u8>(ip) as u32;
                        ip = ip.wrapping_add(1);
                        s = s.wrapping_add(b);
                        if ip >= lim {
                            return lz4_err(src_base, ip);
                        }
                        if b != 0xff {
                            break;
                        }
                    }
                    match_len = match_len.wrapping_add(s as usize);
                    if op.wrapping_add(match_len) < op {
                        return lz4_err(src_base, ip);
                    }
                }
                match_len = match_len.wrapping_add(4);
                state = State::SafeMatchCheck;
            }
            State::SafeMatchCheck => {
                if match_src < dst_base {
                    return lz4_err(src_base, ip);
                }
                state = State::SafeMatchCopy;
            }
            State::SafeMatchCopy => {
                let m_end = op.wrapping_add(match_len);
                if offset < 8 {
                    exe::wr::<u32>(op, 0);
                    for k in 0..4 {
                        exe::wr::<u8>(op + k, exe::rd::<u8>(match_src + k));
                    }
                    match_src = match_src.wrapping_add(INC32[offset]);
                    exe::wr::<u32>(op + 4, exe::rd::<u32>(match_src));
                    match_src = match_src.wrapping_sub(DEC64[offset] as usize);
                } else {
                    exe::wr::<u64>(op, exe::rd::<u64>(match_src));
                    match_src = match_src.wrapping_add(8);
                }
                op = op.wrapping_add(8);
                if m_end > oend.wrapping_sub(0x0c) {
                    if m_end > oend.wrapping_sub(5) {
                        return lz4_err(src_base, ip);
                    }
                    let olimit = oend.wrapping_sub(7);
                    if op < olimit {
                        let mut d = op;
                        let mut s = match_src;
                        while d < olimit {
                            exe::wr::<u64>(d, exe::rd::<u64>(s));
                            d = d.wrapping_add(8);
                            s = s.wrapping_add(8);
                        }
                        let adv = olimit.wrapping_sub(op);
                        op = olimit;
                        match_src = match_src.wrapping_add(adv);
                    }
                    while op < m_end {
                        exe::wr::<u8>(op, exe::rd::<u8>(match_src));
                        op = op.wrapping_add(1);
                        match_src = match_src.wrapping_add(1);
                    }
                } else {
                    exe::wr::<u64>(op, exe::rd::<u64>(match_src));
                    if match_len > 16 {
                        op = op.wrapping_add(8);
                        match_src = match_src.wrapping_add(8);
                        while op < m_end {
                            exe::wr::<u64>(op, exe::rd::<u64>(match_src));
                            op = op.wrapping_add(8);
                            match_src = match_src.wrapping_add(8);
                        }
                    }
                }
                op = m_end;
                state = State::SafeLoop;
            }
            State::EndOfBlock => {
                if ip.wrapping_add(lit_len) != iend || cpy > oend {
                    return lz4_err(src_base, ip);
                }
                let _ = exe::call_va!(J_MEMMOVE_VA, op as *mut u8, ip as *const u8, lit_len);
                return cpy.wrapping_sub(dst_base) as i32;
            }
        }
    }
}

// @ 0x14208a430 verified — anonymous_namespace::AsyncStreamer::vfunc_2
pub unsafe extern "win64" fn async_streamer_vfunc_2(this: *mut u8) {
    let t = this as usize;
    let lock_outer = t + 0x28;
    let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock_outer) -> ());

    let dev = exe::rd::<usize>(t + 0xd0);
    let dev_vtbl = exe::rd::<usize>(dev);
    let fence_val = exe::call_va!(exe::rd::<u64>(dev_vtbl + 0x40), (dev) -> u64);
    if fence_val == u64::MAX {
        let _ = exe::call_va!(0x1_4205_d410, (dev, 0x887a_0005u32) -> ());
    }

    let arr_begin = exe::rd::<usize>(t + 0x68);
    let arr_count = exe::rd::<u32>(t + 0x70) as usize;
    let arr_end = arr_begin.wrapping_add(arr_count * 16);
    let mut cur = arr_begin;
    while cur != arr_end {
        if exe::rd::<u64>(cur + 8) > fence_val {
            break;
        }
        cur = cur.wrapping_add(16);
    }

    if arr_begin != cur {
        if cur == arr_end {
            let mut p = arr_begin;
            while p != arr_end {
                let obj = exe::rd::<usize>(p);
                if obj != 0 {
                    let vtbl = exe::rd::<usize>(obj);
                    let _ = exe::call_va!(exe::rd::<u64>(vtbl + 0x10), (obj) -> ());
                }
                exe::wr::<usize>(p, 0);
                p = p.wrapping_add(16);
            }
            exe::wr::<u32>(t + 0x70, 0);
        } else {
            let mut dst_p = arr_begin;
            let mut src_p = cur;
            while src_p != arr_end {
                let obj = exe::rd::<usize>(dst_p);
                if obj != 0 {
                    let vtbl = exe::rd::<usize>(obj);
                    let _ = exe::call_va!(exe::rd::<u64>(vtbl + 0x10), (obj) -> ());
                }
                exe::wr::<usize>(dst_p, 0);
                let moved = exe::rd::<usize>(src_p);
                exe::wr::<usize>(src_p, 0);
                exe::wr::<usize>(dst_p, moved);
                exe::wr::<u64>(dst_p + 8, exe::rd::<u64>(src_p + 8));
                dst_p = dst_p.wrapping_add(16);
                src_p = src_p.wrapping_add(16);
            }
            let cur_end = exe::rd::<usize>(t + 0x68)
                .wrapping_add((exe::rd::<u32>(t + 0x70) as usize) * 16);
            let _ = exe::call_va!(0x1_411e_0430, (t, dst_p, cur_end) -> ());
            let removed = ((cur.wrapping_sub(arr_begin)) >> 4) as u32;
            let old_len = exe::rd::<u32>(t + 0x70);
            exe::wr::<u32>(t + 0x70, old_len.wrapping_sub(removed));
        }
    }

    exe::wr::<u8>(t + 0x78, 0);

    let lock_inner = t + 0x8;
    let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock_inner) -> ());
    let p30 = exe::rd::<u64>(t + 0x30);
    let p10 = exe::rd::<u64>(t + 0x10);
    exe::wr::<u64>(t + 0x30, p10);
    exe::wr::<u64>(t + 0x10, p30);
    for off in [0usize, 4, 8, 12] {
        let a = exe::rd::<u32>(t + 0x38 + off);
        let b = exe::rd::<u32>(t + 0x18 + off);
        exe::wr::<u32>(t + 0x38 + off, b);
        exe::wr::<u32>(t + 0x18 + off, a);
    }
    let _ = exe::call_va!(0x1_4208_52e0, (t + 0x10) -> ());
    let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock_inner) -> ());

    #[repr(C)]
    struct LocalVec {
        ptr: usize,
        len: u32,
        cap: u32,
    }
    let mut vec = LocalVec {
        ptr: 0,
        len: 0,
        cap: 0,
    };
    let mut max_val: u64 = 0;

    let q_count = exe::rd::<u32>(t + 0x40);
    for i in 0..q_count {
        let head = exe::rd::<u32>(t + 0x38) as u64;
        let cap = exe::rd::<u32>(t + 0x44) as u64;
        let idx = (head.wrapping_add(i as u64) % cap) as usize;
        let item = exe::rd::<usize>(t + 0x30).wrapping_add(idx * 96);
        let item_val = exe::rd::<u64>(item + 0x58);
        let mut ctx: [u64; 5] = [
            (&mut vec as *mut LocalVec) as u64,
            (&mut max_val as *mut u64) as u64,
            item_val,
            t as u64,
            item_val,
        ];
        let _ = exe::call_va!(0x1_411e_0be0, (ctx.as_mut_ptr() as usize, item) -> ());
    }

    if exe::rd::<u32>(t + 0x40) != 0 {
        let base = exe::rd::<usize>(t + 0x30);
        let last_off = (exe::rd::<u32>(t + 0x40).wrapping_sub(1) as u64)
            .wrapping_add(exe::rd::<u32>(t + 0x38) as u64);
        let cap = exe::rd::<u32>(t + 0x44) as u64;
        let idx = (last_off % cap) as usize;
        let last_val = exe::rd::<u64>(base + idx * 96 + 0x58);
        if exe::rd::<u64>(t + 0xd8) < last_val {
            let _ = exe::call_va!(0x1_4208_9c40, (t, last_val) -> ());
            let sink = exe::rd::<usize>(t + 0x48);
            exe::wr::<u64>(t + 0xd8, last_val);
            let sink_vtbl = exe::rd::<usize>(sink);
            let _ = exe::call_va!(
                exe::rd::<u64>(sink_vtbl + 0x70),
                (sink, exe::rd::<usize>(t + 0xd0), last_val) -> ()
            );
        }
    }

    let _ = exe::call_va!(0x1_4208_52e0, (t + 0x30) -> ());

    if vec.len != 0 {
        let mgr_holder = exe::rd::<usize>(STREAMER_MGR_PTR_VA as usize);
        let mgr = exe::rd::<usize>(mgr_holder + 0x10);
        let mgr_vtbl = exe::rd::<usize>(mgr);
        let _ = exe::call_va!(
            exe::rd::<u64>(mgr_vtbl + 0x78),
            (mgr, exe::rd::<usize>(t + 0xd0), max_val) -> ()
        );

        let mut cur_e = vec.ptr;
        let end_e = cur_e.wrapping_add((vec.len as usize) * 64);
        while cur_e != end_e {
            let cb = exe::rd::<usize>(cur_e + 0x38);
            if cb == 0 {
                let _ = exe::call_iat!(IAT_XBAD_FUNCTION_CALL, () -> ());
                return;
            }
            let cb_vtbl = exe::rd::<usize>(cb);
            let _ = exe::call_va!(exe::rd::<u64>(cb_vtbl + 0x10), (cb) -> ());
            cur_e = cur_e.wrapping_add(64);
        }
    }

    let _ = exe::call_va!(0x1_4208_8c20, (t) -> ());

    let mut cur_e = vec.ptr;
    let end_e = cur_e.wrapping_add((vec.len as usize) * 64);
    while cur_e != end_e {
        let cb = exe::rd::<usize>(cur_e + 0x38);
        if cb != 0 {
            let cb_vtbl = exe::rd::<usize>(cb);
            let is_ext = u8::from(cb != cur_e);
            let _ = exe::call_va!(exe::rd::<u64>(cb_vtbl + 0x20), (cb, is_ext) -> ());
            exe::wr::<usize>(cur_e + 0x38, 0);
        }
        cur_e = cur_e.wrapping_add(64);
    }
    if vec.ptr != 0 {
        let _ = exe::call_va!(0x1_4241_c190, (vec.ptr) -> ());
    }

    let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock_outer) -> ());
}

// @ 0x1424434d0 verified — r::MemoryFile::vfunc_5 (read)
pub unsafe extern "win64" fn memory_file_vfunc_5(this: *mut u8, dst: *mut u8, len: usize) {
    let t = this as usize;
    let pos = exe::rd::<usize>(t + 0x128);
    let size = exe::rd::<usize>(t + 0x120);
    if pos.wrapping_add(len) <= size {
        let src = exe::rd::<usize>(t + 0x118).wrapping_add(pos);
        let _ = exe::call_va!(J_MEMCPY_VA, dst, src as *const u8, len);
        exe::wr::<usize>(t + 0x128, exe::rd::<usize>(t + 0x128).wrapping_add(len));
        return;
    }
    let mut rem = size.wrapping_sub(pos);
    let mut msg1 = 0u64;
    let msg1_ptr = exe::call_va!(
        0x1_413b_5730,
        (
            0x1_4340_b470usize,
            &mut msg1 as *mut u64 as usize,
            t + 0x128,
            &mut rem as *mut usize as usize
        ) -> usize
    );
    let name_cstr = exe::call_va!(0x1_401d_f680, (t + 0x0c) -> usize);
    msg1 = name_cstr as u64;
    let msg2_ptr = exe::call_va!(
        0x1_401e_22d0,
        (0x1_4340_b3e8usize, &mut msg1 as *mut u64 as usize) -> usize
    );
    let mut exc_buf = [0u8; 64];
    let exc_ptr = exe::call_va!(
        0x1_413b_55f0,
        (exc_buf.as_mut_ptr() as usize, msg2_ptr, msg1_ptr) -> usize
    );
    let _ = exe::call_va!(0x1_413b_5740, (exc_ptr) -> ());
}

// @ 0x142443400 verified — r::MemoryFile::vfunc_7 (seek)
pub unsafe extern "win64" fn memory_file_vfunc_7(this: *mut u8, offset: i64, origin: u32) {
    let t = this as usize;
    if origin == 0 {
        let cur = exe::rd::<i64>(t + 0x128);
        exe::wr::<i64>(t + 0x128, cur.wrapping_add(offset));
    } else if origin == 1 {
        exe::wr::<i64>(t + 0x128, offset);
    } else if origin == 2 {
        let size = exe::rd::<i64>(t + 0x120);
        exe::wr::<i64>(t + 0x128, size.wrapping_add(offset));
    }
    let new_pos = exe::rd::<i64>(t + 0x128);
    let size = exe::rd::<i64>(t + 0x120);
    if new_pos >= 0 && new_pos <= size {
        return;
    }
    let mut orig_slot = origin;
    let mut tmp = 0u64;
    let msg1_ptr = exe::call_va!(
        0x1_413b_5820,
        (
            t + 0x120,
            &mut tmp as *mut u64 as usize,
            &mut orig_slot as *mut u32 as usize,
            t + 0x128,
            t + 0x120
        ) -> usize
    );
    let name_cstr = exe::call_va!(0x1_401d_f680, (t + 0x0c) -> usize);
    let mut name_slot = name_cstr as u64;
    let msg2_ptr = exe::call_va!(
        0x1_401e_22d0,
        (0x1_4340_b3e8usize, &mut name_slot as *mut u64 as usize) -> usize
    );
    let mut exc_buf = [0u8; 64];
    let exc_ptr = exe::call_va!(
        0x1_413b_56b0,
        (exc_buf.as_mut_ptr() as usize, msg2_ptr, msg1_ptr) -> usize
    );
    let _ = exe::call_va!(0x1_413b_5850, (exc_ptr) -> ());
}

// @ 0x142498890 verified — r::File::vfunc_5 (buffered file read)
pub unsafe extern "win64" fn file_vfunc_5(this: *mut u8, dst: *mut u8, len: usize) {
    let t = this as usize;
    let buf_pos = exe::rd::<usize>(t + 0x138);
    let buf_end = exe::rd::<usize>(t + 0x140);
    let avail = buf_end.wrapping_sub(buf_pos).min(len);
    let buf_base = exe::rd::<usize>(t + 0x128);
    let _ = exe::call_va!(
        J_MEMCPY_VA,
        dst,
        buf_base.wrapping_add(buf_pos) as *const u8,
        avail
    );
    let dst_rem = (dst as usize).wrapping_add(avail);
    exe::wr::<usize>(t + 0x138, exe::rd::<usize>(t + 0x138).wrapping_add(avail));
    let rem = len.wrapping_sub(avail);
    if rem == 0 {
        return;
    }

    exe::wr::<usize>(t + 0x138, 0);
    exe::wr::<usize>(t + 0x140, 0);

    let file_size = exe::rd::<usize>(t + 0x120);
    let file_pos = exe::rd::<usize>(t + 0x130);
    let file_rem = file_size.wrapping_sub(file_pos);
    if rem > file_rem {
        let mut msg_buf = [0u8; 0x400];
        let _ = exe::call_va!(
            0x1_4137_9cf0,
            (
                msg_buf.as_mut_ptr() as usize,
                0x400usize,
                0x1_4340_ebd0usize,
                rem,
                file_rem,
                file_pos,
                file_size
            ) -> i32
        );
        let name_cstr = exe::call_va!(0x1_401d_f680, (t + 0x8) -> usize);
        let mut exc_buf = [0u8; 64];
        let exc_ptr = exe::call_va!(
            0x1_413b_55f0,
            (exc_buf.as_mut_ptr() as usize, name_cstr, msg_buf.as_ptr() as usize) -> usize
        );
        let _ = exe::call_va!(0x1_413b_5740, (exc_ptr) -> ());
        return;
    }

    const CHUNK_CAP: usize = 0x20000;
    let (read_dst, read_len) = if rem < CHUNK_CAP {
        (exe::rd::<usize>(t + 0x128), file_rem.min(CHUNK_CAP))
    } else {
        (dst_rem, rem)
    };

    let mut tries = 5i32;
    let got = loop {
        let vfs = exe::rd::<usize>(VFS_GLOBAL_PTR_VA as usize);
        let vfs_vtbl = exe::rd::<usize>(vfs);
        let n = exe::call_va!(
            exe::rd::<u64>(vfs_vtbl + 0x98),
            (vfs, exe::rd::<u64>(t + 0x118), read_dst, read_len) -> usize
        );
        if n != 0 {
            break n;
        }
        tries -= 1;
        if tries <= 0 {
            break 0;
        }
        let mut ms: u64 = 200;
        let mut dur: u64 = 0;
        let dur_ptr = exe::call_va!(
            0x1_4049_aa50,
            (&mut dur as *mut u64 as usize, &mut ms as *mut u64 as usize) -> usize
        );
        let _ = exe::call_va!(0x1_423f_0a10, (exe::rd::<u64>(dur_ptr)) -> ());
    };

    if tries == 0 {
        let mut msg_buf = [0u8; 0x400];
        let _ = exe::call_va!(0x1_423f_3530, (msg_buf.as_mut_ptr() as usize, 0x400u32) -> ());
        let name_cstr = exe::call_va!(0x1_401d_f680, (t + 0x8) -> usize);
        let mut exc_buf = [0u8; 64];
        let exc_ptr = exe::call_va!(
            0x1_413b_55f0,
            (exc_buf.as_mut_ptr() as usize, name_cstr, msg_buf.as_ptr() as usize) -> usize
        );
        let _ = exe::call_va!(0x1_413b_5740, (exc_ptr) -> ());
        return;
    }

    if got != read_len {
        let mut msg_buf = [0u8; 0x400];
        let _ = exe::call_va!(
            0x1_4137_9cf0,
            (
                msg_buf.as_mut_ptr() as usize,
                0x400usize,
                0x1_4340_ec38usize,
                rem,
                got
            ) -> i32
        );
        let name_cstr = exe::call_va!(0x1_401d_f680, (t + 0x8) -> usize);
        let mut exc_buf = [0u8; 64];
        let exc_ptr = exe::call_va!(
            0x1_413b_55f0,
            (exc_buf.as_mut_ptr() as usize, name_cstr, msg_buf.as_ptr() as usize) -> usize
        );
        let _ = exe::call_va!(0x1_413b_5740, (exc_ptr) -> ());
        return;
    }

    exe::wr::<usize>(t + 0x130, exe::rd::<usize>(t + 0x130).wrapping_add(got));
    if rem < CHUNK_CAP {
        exe::wr::<usize>(t + 0x140, got);
        let _ = exe::call_va!(
            J_MEMCPY_VA,
            dst_rem as *mut u8,
            exe::rd::<*const u8>(t + 0x128),
            rem
        );
        exe::wr::<usize>(t + 0x138, exe::rd::<usize>(t + 0x138).wrapping_add(rem));
    }
}

// @ 0x142497cc0 verified — r::File::vfunc_7 (buffered file seek)
pub unsafe extern "win64" fn file_vfunc_7(this: *mut u8, mut offset: i64, mut origin: u32) {
    let t = this as usize;
    if exe::rd::<u32>(t + 0x110) == 0 {
        let buf_end = exe::rd::<i64>(t + 0x130);
        let buf_start = buf_end.wrapping_sub(exe::rd::<i64>(t + 0x140));
        let target = if origin == 1 {
            offset
        } else if origin == 2 {
            let vtbl = exe::rd::<usize>(t);
            let sz = exe::call_va!(exe::rd::<u64>(vtbl + 0x18), (t) -> i64);
            sz.wrapping_add(offset)
        } else {
            buf_start
                .wrapping_add(exe::rd::<i64>(t + 0x138))
                .wrapping_add(offset)
        };
        if target >= buf_start && target < buf_end {
            exe::wr::<i64>(t + 0x138, target.wrapping_sub(buf_start));
            return;
        }
        exe::wr::<u64>(t + 0x140, 0);
        exe::wr::<u64>(t + 0x138, 0);
        if origin == 0 {
            offset = target;
            origin = 1;
        }
    } else {
        if origin == 0 {
            let cur = exe::rd::<i64>(t + 0x138).wrapping_add(exe::rd::<i64>(t + 0x130));
            offset = offset.wrapping_add(cur);
            origin = 1;
        }
        let pending = exe::rd::<usize>(t + 0x138);
        if pending != 0 {
            let vfs = exe::rd::<usize>(VFS_GLOBAL_PTR_VA as usize);
            let vfs_vtbl = exe::rd::<usize>(vfs);
            let written = exe::call_va!(
                exe::rd::<u64>(vfs_vtbl + 0xa0),
                (vfs, exe::rd::<u64>(t + 0x118), exe::rd::<usize>(t + 0x128), pending) -> usize
            );
            if written == 0 {
                let mut msg_buf = [0u8; 0x400];
                let _ = exe::call_va!(J_MEMSET_VA, msg_buf.as_mut_ptr(), 0u32, 0x400usize);
                let _ = exe::call_va!(0x1_423f_3530, (msg_buf.as_mut_ptr() as usize, 0x400u32) -> ());
                let name_cstr = exe::call_va!(0x1_401d_f680, (t + 0x8) -> usize);
                let mut exc_buf = [0u8; 64];
                let exc_ptr = exe::call_va!(
                    0x1_413b_5650,
                    (exc_buf.as_mut_ptr() as usize, name_cstr, msg_buf.as_ptr() as usize) -> usize
                );
                let _ = exe::call_va!(0x1_413b_57b0, (exc_ptr) -> ());
                return;
            }
            exe::wr::<u64>(t + 0x138, 0);
            exe::wr::<u64>(
                t + 0x130,
                exe::rd::<u64>(t + 0x130).wrapping_add(written as u64),
            );
        }
    }

    let vfs = exe::rd::<usize>(VFS_GLOBAL_PTR_VA as usize);
    let vfs_vtbl = exe::rd::<usize>(vfs);
    let new_pos = exe::call_va!(
        exe::rd::<u64>(vfs_vtbl + 0x80),
        (vfs, exe::rd::<u64>(t + 0x118), offset, origin) -> i64
    );
    if new_pos == -1 {
        let mut msg_buf = [0u8; 0x400];
        let _ = exe::call_va!(J_MEMSET_VA, msg_buf.as_mut_ptr(), 0u32, 0x400usize);
        let _ = exe::call_va!(0x1_423f_3530, (msg_buf.as_mut_ptr() as usize, 0x400u32) -> ());
        let name_cstr = exe::call_va!(0x1_401d_f680, (t + 0x8) -> usize);
        let mut exc_buf = [0u8; 64];
        let exc_ptr = exe::call_va!(
            0x1_413b_56b0,
            (exc_buf.as_mut_ptr() as usize, name_cstr, msg_buf.as_ptr() as usize) -> usize
        );
        let _ = exe::call_va!(0x1_413b_5850, (exc_ptr) -> ());
        return;
    }
    exe::wr::<i64>(t + 0x130, new_pos);
}

// @ 0x142496ae0 verified — r::FileArchive::File::vfunc_5 (archive subfile read)
pub unsafe extern "win64" fn file_archive_file_vfunc_5(
    this: *mut u8,
    dst: *mut u8,
    len: usize,
) {
    let t = this as usize;
    let archive = exe::rd::<usize>(t + 0x8);
    let stream = exe::rd::<usize>(archive + 0x130);
    let target_off = exe::rd::<u64>(t + 0x10).wrapping_add(exe::rd::<u64>(t + 0x18));
    let stream_vtbl = exe::rd::<usize>(stream);
    let _ = exe::call_va!(
        exe::rd::<u64>(stream_vtbl + 0x38),
        (stream, target_off, 1u32) -> ()
    );

    let archive2 = exe::rd::<usize>(t + 0x8);
    let stream2 = exe::rd::<usize>(archive2 + 0x130);
    let stream2_vtbl = exe::rd::<usize>(stream2);
    let _ = exe::call_va!(
        exe::rd::<u64>(stream2_vtbl + 0x28),
        (stream2, dst, len) -> ()
    );

    exe::wr::<usize>(t + 0x18, exe::rd::<usize>(t + 0x18).wrapping_add(len));
}

// @ 0x142475db0 verified — ring-buffer IO completion callback drainer
pub unsafe extern "win64" fn drain_io_completion_callbacks(this: *mut u8, max_count: usize) {
    let t = this as usize;
    let mut processed: i32 = 0;

    if max_count != 0 {
        loop {
            if exe::rd::<u32>(t + 0x90) == 0 {
                let lock = t + 0x18;
                let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock) -> ());
                if exe::rd::<u32>(t + 0x30) == 0 {
                    let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
                    break;
                }
                let p80 = exe::rd::<u64>(t + 0x80);
                let p20 = exe::rd::<u64>(t + 0x20);
                exe::wr::<u64>(t + 0x80, p20);
                exe::wr::<u64>(t + 0x20, p80);
                for off in [0usize, 4, 8, 12] {
                    let a = exe::rd::<u32>(t + 0x88 + off);
                    let b = exe::rd::<u32>(t + 0x28 + off);
                    exe::wr::<u32>(t + 0x88 + off, b);
                    exe::wr::<u32>(t + 0x28 + off, a);
                }
                let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
            }

            while (processed as isize as usize) < max_count && exe::rd::<u32>(t + 0x90) != 0 {
                let base = exe::rd::<usize>(t + 0x80);
                let head = exe::rd::<u32>(t + 0x88) as u64;
                let cap = exe::rd::<u32>(t + 0x94) as u64;
                let idx = (head % cap) as usize;
                let entry = base.wrapping_add(idx * 96);

                let mut local_str = [0u8; 0x60];
                let ls = local_str.as_mut_ptr() as usize;
                let hdr = (exe::rd::<u32>(ls) & 0xff00_0002) | 0x2;
                exe::wr::<u32>(ls, hdr);
                exe::wr::<u8>(ls + 3, 2);
                exe::wr::<u32>(ls + 4, 0);
                exe::wr::<u8>(ls + 8, 0);
                let _ = exe::call_va!(0x1_4240_a730, (ls, entry) -> usize);

                let f18 = exe::rd::<u64>(entry + 0x18);
                let f20 = exe::rd::<u64>(entry + 0x20);
                exe::wr::<u64>(entry + 0x20, 0);
                let f28 = exe::rd::<u64>(entry + 0x28);
                exe::wr::<u64>(entry + 0x28, 0);
                let f30 = exe::rd::<usize>(entry + 0x30);
                exe::wr::<usize>(entry + 0x30, 0);
                let f38 = exe::rd::<u64>(entry + 0x38);
                exe::wr::<u64>(entry + 0x38, 0);
                let f40 = exe::rd::<u16>(entry + 0x40);
                exe::wr::<u16>(entry + 0x40, 0x10);
                let f48 = exe::rd::<usize>(entry + 0x48);
                exe::wr::<usize>(entry + 0x48, 0);
                let f50 = exe::rd::<u32>(entry + 0x50);
                let f58 = exe::rd::<usize>(entry + 0x58);

                exe::wr::<u64>(ls + 0x18, f18);
                exe::wr::<u64>(ls + 0x20, f20);
                exe::wr::<u64>(ls + 0x28, f28);
                exe::wr::<usize>(ls + 0x30, f30);
                exe::wr::<u64>(ls + 0x38, f38);
                exe::wr::<u16>(ls + 0x40, f40);
                exe::wr::<usize>(ls + 0x48, f48);
                exe::wr::<u32>(ls + 0x50, f50);
                exe::wr::<usize>(ls + 0x58, f58);

                let mut f50 = f50;
                let mut f58 = f58;
                if exe::rd::<u32>(t + 0x90) != 0 {
                    let h2 = exe::rd::<u32>(t + 0x88) as usize;
                    let e2 = exe::rd::<usize>(t + 0x80).wrapping_add(h2 * 96);
                    let e2_30 = exe::rd::<usize>(e2 + 0x30);
                    if e2_30 != 0 {
                        let e2_48 = exe::rd::<usize>(e2 + 0x48);
                        let e2_40 = exe::rd::<u16>(e2 + 0x40) as u32;
                        let e2_38 = exe::rd::<u64>(e2 + 0x38);
                        let vtbl = exe::rd::<usize>(e2_48);
                        let _ = exe::call_va!(
                            exe::rd::<u64>(vtbl + 0x10),
                            (e2_48, e2_30, e2_38, e2_40) -> ()
                        );
                    }
                    let _ = exe::call_va!(0x1_4240_ab10, (e2) -> ());
                    let next_h = exe::rd::<u32>(t + 0x88).wrapping_add(1);
                    exe::wr::<u32>(
                        t + 0x88,
                        if next_h == exe::rd::<u32>(t + 0x94) {
                            0
                        } else {
                            next_h
                        },
                    );
                    exe::wr::<u32>(t + 0x90, exe::rd::<u32>(t + 0x90).wrapping_sub(1));
                    f58 = exe::rd::<usize>(ls + 0x58);
                    f50 = exe::rd::<u32>(ls + 0x50);
                }

                if f50 != 0 {
                    let cb = exe::rd::<usize>(f58 + 0x38);
                    if cb == 0 {
                        let _ = exe::call_iat!(IAT_XBAD_FUNCTION_CALL, () -> ());
                        return;
                    }
                    let vtbl = exe::rd::<usize>(cb);
                    let _ = exe::call_va!(exe::rd::<u64>(vtbl + 0x10), (cb, ls) -> ());
                }

                if f58 != 0 {
                    let cb = exe::rd::<usize>(f58 + 0x38);
                    if cb != 0 {
                        let vtbl = exe::rd::<usize>(cb);
                        let _ = exe::call_va!(
                            exe::rd::<u64>(vtbl + 0x20),
                            (cb, u8::from(cb != f58)) -> ()
                        );
                        exe::wr::<usize>(f58 + 0x38, 0);
                    }
                    let _ = exe::call_va!(0x1_4243_b3e0, (t + 0x38, f58) -> ());
                }

                let rem_30 = exe::rd::<usize>(ls + 0x30);
                if rem_30 != 0 {
                    let rem_40 = exe::rd::<u16>(ls + 0x40) as u32;
                    let rem_48 = exe::rd::<usize>(ls + 0x48);
                    let rem_38 = exe::rd::<u64>(ls + 0x38);
                    let vtbl = exe::rd::<usize>(rem_48);
                    let _ = exe::call_va!(
                        exe::rd::<u64>(vtbl + 0x10),
                        (rem_48, rem_30, rem_38, rem_40) -> ()
                    );
                }

                let _ = exe::call_va!(0x1_4240_ab10, (ls) -> ());
                processed = processed.wrapping_add(1);
            }

            if (processed as isize as usize) >= max_count {
                break;
            }
        }
    }

    let neg = processed.wrapping_neg();
    let counter = &*((t + 0x78) as *const AtomicI32);
    counter.fetch_add(neg, Ordering::SeqCst);
}

// @ 0x14243b0b0 verified — push_back 48-byte entry into vector at [rcx, rcx+8, rcx+0xc]
pub unsafe extern "win64" fn push_back_entry_48(
    this: *mut u8,
    a0: u64,
    a1: u64,
    _r9_unused: u64,
    s0: u32,
    s1: u32,
) {
    let t = this as usize;
    let len = exe::rd::<u32>(t + 0x8) as usize;
    let cap = exe::rd::<u32>(t + 0xc) as usize;
    if len * 48 == cap * 48 {
        let min_cap = len + 1;
        let grown = (cap + (cap >> 1)).max(min_cap);
        let _ = exe::call_va!(0x1_413a_a3e0, (t, grown) -> ());
    }
    let cur_len = exe::rd::<u32>(t + 0x8) as usize;
    let slot0 = exe::rd::<usize>(t).wrapping_add(cur_len * 48);
    for off in (0..48).step_by(8) {
        exe::wr::<u64>(slot0 + off, 0);
    }
    let new_len = (cur_len as u32).wrapping_add(1);
    exe::wr::<u32>(t + 0x8, new_len);
    let end_ptr = exe::rd::<usize>(t).wrapping_add((new_len as usize) * 48);
    exe::wr::<u64>(end_ptr - 0x30, a0);
    exe::wr::<u64>(end_ptr - 0x28, a1);
    exe::wr::<u32>(end_ptr - 0x20, s0);
    exe::wr::<u32>(end_ptr - 0x1c, s1);
    exe::wr::<u64>(end_ptr - 0x18, 0);
    exe::wr::<u32>(end_ptr - 0x08, 0xffff_ffff);
    exe::wr::<u64>(end_ptr - 0x10, 0);

    let p320 = exe::rd::<usize>(t + 0x320);
    if p320 != 0 {
        let sub = exe::rd::<usize>(p320 + 0x8);
        if sub != 0 {
            let vtbl = exe::rd::<usize>(sub);
            let v = exe::call_va!(exe::rd::<u64>(vtbl + 0x10), (sub) -> u64);
            exe::wr::<u64>(end_ptr - 0x18, v);
        }
    }
}

// @ 0x142052510 verified — recursive-SRWLock bitmask slab allocator
pub unsafe extern "win64" fn bitmask_slab_alloc(_this: usize, req_size: usize) -> usize {
    let mgr = exe::rd::<usize>(SLAB_ALLOC_GLOBAL_VA as usize);
    let lock_base = mgr + 0xc0;
    let tid = exe::call_iat!(IAT_GET_CURRENT_THREAD_ID, () -> u32);
    if exe::rd::<u32>(lock_base + 0x8) != tid {
        let _ = exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (lock_base) -> ());
        exe::wr::<u32>(lock_base + 0x8, tid);
    }
    exe::wr::<u32>(
        lock_base + 0xc,
        exe::rd::<u32>(lock_base + 0xc).wrapping_add(1),
    );

    let node = exe::call_va!(0x1_4243_b450, (mgr + 0x68) -> usize);
    if node != 0 {
        for off in (0..0x28).step_by(8) {
            exe::wr::<u64>(node + off, 0);
        }
    }

    let block_size = exe::rd::<u64>(mgr + 0xb8);
    let align_mask = block_size.wrapping_sub(1);
    let aligned_size = (req_size as u64).wrapping_add(align_mask) & !align_mask;
    let num_blocks = aligned_size / block_size;
    let mut block_mask = 0u64;
    for b in 0..num_blocks {
        block_mask |= 1u64 << (b & 63);
    }

    let mut chosen_page: usize = 0;
    let mut chosen_bit: u32 = 0;
    let num_pages = exe::rd::<u32>(mgr + 0x8) as usize;
    if num_pages != 0 {
        let mut p_idx: u32 = 0;
        loop {
            if chosen_page != 0 {
                break;
            }
            let pages_ptr = exe::rd::<usize>(mgr);
            let page = exe::rd::<usize>(pages_ptr + (p_idx as usize) * 8);
            if exe::rd::<u64>(page + 0x10) >= aligned_size {
                let used = exe::rd::<u64>(page + 0x18);
                let max_shift = 64u64.wrapping_sub(num_blocks);
                if max_shift != 0 {
                    for sh in 0..(max_shift as u32) {
                        let shifted = block_mask.wrapping_shl(sh);
                        if (used & shifted) == 0 {
                            chosen_page = page;
                            chosen_bit = sh;
                            block_mask = shifted;
                            break;
                        }
                    }
                }
            }
            p_idx = p_idx.wrapping_add(1);
            let cur_pages = exe::rd::<u32>(mgr + 0x8) as usize;
            if (p_idx as usize) >= cur_pages {
                break;
            }
        }
    }

    if chosen_page == 0 {
        chosen_page = exe::call_va!(0x1_4205_2270, (mgr) -> usize);
        chosen_bit = 0;
    }

    exe::wr::<usize>(node + 0x20, mgr);
    exe::wr::<usize>(node, chosen_page);
    let byte_off = (chosen_bit as i32 as i64 as u64).wrapping_mul(exe::rd::<u64>(mgr + 0xb8));
    exe::wr::<u64>(node + 0x8, byte_off);
    exe::wr::<u64>(node + 0x18, block_mask);
    exe::wr::<u64>(
        chosen_page + 0x18,
        exe::rd::<u64>(chosen_page + 0x18) | block_mask,
    );
    exe::wr::<u64>(node + 0x10, aligned_size);
    exe::wr::<u64>(
        chosen_page + 0x10,
        exe::rd::<u64>(chosen_page + 0x10).wrapping_sub(aligned_size),
    );
    exe::wr::<u64>(
        mgr + 0xd8,
        exe::rd::<u64>(mgr + 0xd8).wrapping_add(aligned_size),
    );

    let new_depth = exe::rd::<u32>(lock_base + 0xc).wrapping_sub(1);
    exe::wr::<u32>(lock_base + 0xc, new_depth);
    if new_depth == 0 {
        let lock_tid = &*((lock_base + 0x8) as *const AtomicI32);
        lock_tid.swap(-1, Ordering::SeqCst);
        let _ = exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock_base) -> ());
    }

    node
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

pub unsafe fn diff_lz4_decompress_safe(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8, i32, i32) -> i32;
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut cases = 0u32;

    // 1. Null source
    let mut d_a = [0u8; 64];
    let mut d_b = [0u8; 64];
    difftest::eq(
        "null src",
        o(core::ptr::null(), d_a.as_mut_ptr(), 0, 64),
        n(core::ptr::null(), d_b.as_mut_ptr(), 0, 64),
    )?;
    cases += 1;

    // 2. Empty block (`[0]` -> 0 bytes when dst_capacity == 0 or > 0)
    let zero_src = [0u8];
    difftest::eq(
        "empty cap=0",
        o(zero_src.as_ptr(), d_a.as_mut_ptr(), 1, 0),
        n(zero_src.as_ptr(), d_b.as_mut_ptr(), 1, 0),
    )?;
    cases += 1;

    // 3. Literal-only & match-copy streams across both <64 and >=64 capacities
    let stream1: &[u8] = &[
        0x40, b'A', b'B', b'C', b'D', 0x04, 0x00, 0x50, b'1', b'2', b'3', b'4', b'5',
    ];
    let mut stream2 = Vec::new();
    stream2.push(0x84); // 8 literals, match len 4+4=8
    stream2.extend_from_slice(b"ABCDEFGH");
    stream2.extend_from_slice(&[0x08, 0x00]); // offset 8
    stream2.push(0xf1); // 15+5=20 literals, match len 1+4=5
    stream2.push(0x05);
    stream2.extend_from_slice(b"0123456789abcdefghij");
    stream2.extend_from_slice(&[0x01, 0x00]); // RLE offset 1
    stream2.push(0x60); // 6 trailing literals
    stream2.extend_from_slice(b"XYZ!@#");

    for (idx, s) in [stream1, &stream2[..]].iter().enumerate() {
        for cap in [17i32, 32, 64, 128, 256] {
            let mut out_a = vec![0xcd_u8; cap as usize];
            let mut out_b = vec![0xcd_u8; cap as usize];
            let ra = o(s.as_ptr(), out_a.as_mut_ptr(), s.len() as i32, cap);
            let rb = n(s.as_ptr(), out_b.as_mut_ptr(), s.len() as i32, cap);
            difftest::eq(&format!("lz4 ret s={idx} cap={cap}"), ra, rb)?;
            difftest::eq(&format!("lz4 buf s={idx} cap={cap}"), &out_a[..], &out_b[..])?;
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_async_streamer_vfunc_2(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut cases = 0u32;

    unsafe extern "win64" fn mock_query_fence(_dev: usize) -> u64 {
        100
    }
    unsafe extern "win64" fn mock_vfunc_10(obj: usize) {
        exe::wr::<u32>(obj + 8, exe::rd::<u32>(obj + 8).wrapping_add(1));
    }

    let mut dev_vtbl = [0usize; 16];
    dev_vtbl[0x40 / 8] = mock_query_fence as *const () as usize;
    let dev_obj = dev_vtbl.as_ptr() as usize;

    let mut cb_vtbl = [0usize; 8];
    cb_vtbl[0x10 / 8] = mock_vfunc_10 as *const () as usize;

    for mode in [0usize, 1, 2] {
        let mut cb_a = [cb_vtbl.as_ptr() as usize, 0usize];
        let mut cb_b = [cb_vtbl.as_ptr() as usize, 0usize];

        let fence2 = if mode == 1 { 50u64 } else { 200u64 };
        let mut arr_a: [u64; 4] = [cb_a.as_mut_ptr() as u64, 50, 0, fence2];
        let mut arr_b: [u64; 4] = [cb_b.as_mut_ptr() as u64, 50, 0, fence2];

        let mut st_a = vec![0u8; 0x100];
        let mut st_b = vec![0u8; 0x100];
        for (st, arr) in [(&mut st_a, &mut arr_a), (&mut st_b, &mut arr_b)] {
            let p = st.as_mut_ptr() as usize;
            exe::wr::<u32>(p + 0x24, 4);
            exe::wr::<u32>(p + 0x44, 4);
            exe::wr::<usize>(p + 0x68, arr.as_mut_ptr() as usize);
            exe::wr::<u32>(p + 0x70, if mode == 0 { 0 } else { 2 });
            exe::wr::<u8>(p + 0x78, 1);
            exe::wr::<usize>(p + 0xd0, (&dev_obj as *const usize) as usize);
        }

        o(st_a.as_mut_ptr());
        n(st_b.as_mut_ptr());

        difftest::eq(&format!("cb mode={mode}"), cb_a[1], cb_b[1])?;
        difftest::eq(
            &format!("len mode={mode}"),
            exe::rd::<u32>(st_a.as_ptr() as usize + 0x70),
            exe::rd::<u32>(st_b.as_ptr() as usize + 0x70),
        )?;
        difftest::eq(
            &format!("flag78 mode={mode}"),
            st_a[0x78],
            st_b[0x78],
        )?;
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_memory_file_vfunc_5(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = Rng::new(0x1424_434d);
    let mut cases = 0u32;

    let data: Vec<u8> = (0..128).map(|_| rng.next_u32() as u8).collect();
    for (pos, len) in [(0usize, 0usize), (0, 16), (16, 48), (64, 64)] {
        let mut mf_a = vec![0u8; 0x140];
        let mut mf_b = vec![0u8; 0x140];
        for mf in [&mut mf_a, &mut mf_b] {
            let p = mf.as_mut_ptr() as usize;
            exe::wr::<usize>(p + 0x118, data.as_ptr() as usize);
            exe::wr::<usize>(p + 0x120, data.len());
            exe::wr::<usize>(p + 0x128, pos);
        }
        let mut out_a = vec![0xaa_u8; 128];
        let mut out_b = vec![0xaa_u8; 128];
        o(mf_a.as_mut_ptr(), out_a.as_mut_ptr(), len);
        n(mf_b.as_mut_ptr(), out_b.as_mut_ptr(), len);
        difftest::eq("mf_read buf", &out_a[..], &out_b[..])?;
        difftest::eq("mf_read state", &mf_a[..], &mf_b[..])?;
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_memory_file_vfunc_7(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, i64, u32);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut cases = 0u32;

    for (init_pos, off, origin) in [
        (32i64, 16i64, 0u32),
        (32, -16, 0),
        (32, 80, 1),
        (32, 0, 1),
        (32, -20, 2),
        (32, 0, 2),
    ] {
        let mut mf_a = vec![0u8; 0x140];
        let mut mf_b = vec![0u8; 0x140];
        for mf in [&mut mf_a, &mut mf_b] {
            let p = mf.as_mut_ptr() as usize;
            exe::wr::<i64>(p + 0x120, 100);
            exe::wr::<i64>(p + 0x128, init_pos);
        }
        o(mf_a.as_mut_ptr(), off, origin);
        n(mf_b.as_mut_ptr(), off, origin);
        difftest::eq("mf_seek state", &mf_a[..], &mf_b[..])?;
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_file_vfunc_5(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = Rng::new(0x1424_9889);
    let mut cases = 0u32;

    let buf_data: Vec<u8> = (0..256).map(|_| rng.next_u32() as u8).collect();
    for (buf_pos, buf_end, read_len) in [(0usize, 128usize, 32usize), (16, 128, 64), (64, 64, 0)] {
        let mut f_a = vec![0u8; 0x150];
        let mut f_b = vec![0u8; 0x150];
        for f in [&mut f_a, &mut f_b] {
            let p = f.as_mut_ptr() as usize;
            exe::wr::<usize>(p + 0x128, buf_data.as_ptr() as usize);
            exe::wr::<usize>(p + 0x138, buf_pos);
            exe::wr::<usize>(p + 0x140, buf_end);
        }
        let mut dst_a = vec![0x55u8; 128];
        let mut dst_b = vec![0x55u8; 128];
        o(f_a.as_mut_ptr(), dst_a.as_mut_ptr(), read_len);
        n(f_b.as_mut_ptr(), dst_b.as_mut_ptr(), read_len);
        difftest::eq("file_read dst", &dst_a[..], &dst_b[..])?;
        difftest::eq("file_read state", &f_a[..], &f_b[..])?;
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_file_vfunc_7(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, i64, u32);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut cases = 0u32;

    let saved_vfs = exe::rd::<usize>(VFS_GLOBAL_PTR_VA as usize);
    unsafe extern "win64" fn mock_vfs_seek(
        _vfs: usize,
        _handle: u64,
        offset: i64,
        origin: u32,
    ) -> i64 {
        offset + (origin as i64) * 1000
    }
    let mut vfs_vtbl = [0usize; 24];
    vfs_vtbl[0x80 / 8] = mock_vfs_seek as *const () as usize;
    let vfs_obj = vfs_vtbl.as_ptr() as usize;
    exe::wr::<usize>(VFS_GLOBAL_PTR_VA as usize, (&vfs_obj as *const usize) as usize);

    for (off, origin) in [(10i64, 0u32), (120, 1), (500, 1)] {
        let mut f_a = vec![0u8; 0x150];
        let mut f_b = vec![0u8; 0x150];
        for f in [&mut f_a, &mut f_b] {
            let p = f.as_mut_ptr() as usize;
            exe::wr::<u32>(p + 0x110, 0);
            exe::wr::<i64>(p + 0x130, 200);
            exe::wr::<i64>(p + 0x138, 20);
            exe::wr::<i64>(p + 0x140, 100);
        }
        o(f_a.as_mut_ptr(), off, origin);
        n(f_b.as_mut_ptr(), off, origin);
        difftest::eq("file_seek state", &f_a[..], &f_b[..])?;
        cases += 1;
    }

    exe::wr::<usize>(VFS_GLOBAL_PTR_VA as usize, saved_vfs);
    Ok(cases)
}

pub unsafe fn diff_file_archive_file_vfunc_5(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut cases = 0u32;

    unsafe extern "win64" fn mock_stream_seek(this: usize, pos: u64, origin: u32) {
        exe::wr::<u64>(this + 8, pos);
        exe::wr::<u32>(this + 16, origin);
    }
    unsafe extern "win64" fn mock_stream_read(this: usize, dst: *mut u8, len: usize) {
        let pos = exe::rd::<u64>(this + 8) as u8;
        for i in 0..len {
            exe::wr::<u8>(dst as usize + i, pos.wrapping_add(i as u8));
        }
    }

    let mut vtbl = [0usize; 10];
    vtbl[0x28 / 8] = mock_stream_read as *const () as usize;
    vtbl[0x38 / 8] = mock_stream_seek as *const () as usize;

    for (base_off, cur_off, len) in [(100u64, 20u64, 16usize), (0, 0, 32), (50, 15, 0)] {
        let mut stream_a = [0u64; 4];
        let mut stream_b = [0u64; 4];
        stream_a[0] = vtbl.as_ptr() as u64;
        stream_b[0] = vtbl.as_ptr() as u64;

        let mut arch_a = vec![0u8; 0x140];
        let mut arch_b = vec![0u8; 0x140];
        exe::wr::<usize>(arch_a.as_mut_ptr() as usize + 0x130, stream_a.as_mut_ptr() as usize);
        exe::wr::<usize>(arch_b.as_mut_ptr() as usize + 0x130, stream_b.as_mut_ptr() as usize);

        let mut sub_a = [0u64; 4];
        let mut sub_b = [0u64; 4];
        sub_a[1] = arch_a.as_mut_ptr() as u64;
        sub_a[2] = base_off;
        sub_a[3] = cur_off;
        sub_b[1] = arch_b.as_mut_ptr() as u64;
        sub_b[2] = base_off;
        sub_b[3] = cur_off;

        let mut dst_a = vec![0u8; 48];
        let mut dst_b = vec![0u8; 48];

        o(sub_a.as_mut_ptr() as *mut u8, dst_a.as_mut_ptr(), len);
        n(sub_b.as_mut_ptr() as *mut u8, dst_b.as_mut_ptr(), len);

        difftest::eq("arch_read dst", &dst_a[..], &dst_b[..])?;
        difftest::eq("arch_read pos", sub_a[3], sub_b[3])?;
        difftest::eq("arch_read stream", &stream_a[1..], &stream_b[1..])?;
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_drain_io_completion_callbacks(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut cases = 0u32;

    for (max_cnt, init_c) in [(0usize, 5i32), (4, 10), (8, 0)] {
        let mut obj_a = vec![0u8; 0xc0];
        let mut obj_b = vec![0u8; 0xc0];
        exe::wr::<i32>(obj_a.as_mut_ptr() as usize + 0x78, init_c);
        exe::wr::<i32>(obj_b.as_mut_ptr() as usize + 0x78, init_c);

        o(obj_a.as_mut_ptr(), max_cnt);
        n(obj_b.as_mut_ptr(), max_cnt);
        difftest::eq("drain empty", &obj_a[..], &obj_b[..])?;
        cases += 1;
    }

    // Also test producer -> consumer queue swap + 1 entry drain
    let mut ring_a = vec![0u8; 2 * 96];
    let mut ring_b = vec![0u8; 2 * 96];
    for ring in [&mut ring_a, &mut ring_b] {
        let e = ring.as_mut_ptr() as usize;
        exe::wr::<u32>(e, 0x0000_0002);
        exe::wr::<u8>(e + 3, 2);
        exe::wr::<u32>(e + 4, 0);
        exe::wr::<u8>(e + 8, 0);
    }
    let mut obj_a = vec![0u8; 0xc0];
    let mut obj_b = vec![0u8; 0xc0];
    for (obj, ring) in [(&mut obj_a, &mut ring_a), (&mut obj_b, &mut ring_b)] {
        let p = obj.as_mut_ptr() as usize;
        exe::wr::<usize>(p + 0x20, ring.as_mut_ptr() as usize);
        exe::wr::<u32>(p + 0x28, 0);
        exe::wr::<u32>(p + 0x30, 1);
        exe::wr::<u32>(p + 0x34, 2);
        exe::wr::<i32>(p + 0x78, 5);
    }
    o(obj_a.as_mut_ptr(), 2);
    n(obj_b.as_mut_ptr(), 2);
    difftest::eq(
        "drain swap+1 ptr80",
        exe::rd::<usize>(obj_a.as_ptr() as usize + 0x80) == ring_a.as_ptr() as usize,
        exe::rd::<usize>(obj_b.as_ptr() as usize + 0x80) == ring_b.as_ptr() as usize,
    )?;
    difftest::eq("drain swap+1 28..80", &obj_a[0x28..0x80], &obj_b[0x28..0x80])?;
    difftest::eq("drain swap+1 88..", &obj_a[0x88..], &obj_b[0x88..])?;
    cases += 1;

    Ok(cases)
}

pub unsafe fn diff_push_back_entry_48(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, u64, u64, u64, u32, u32);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut rng = Rng::new(0x1424_3b0b);
    let mut cases = 0u32;

    for len in [0u32, 1, 2] {
        let mut buf_a = vec![0xffu8; 4 * 48];
        let mut buf_b = vec![0xffu8; 4 * 48];
        let mut obj_a = vec![0u8; 0x340];
        let mut obj_b = vec![0u8; 0x340];
        exe::wr::<usize>(obj_a.as_mut_ptr() as usize, buf_a.as_mut_ptr() as usize);
        exe::wr::<u32>(obj_a.as_mut_ptr() as usize + 8, len);
        exe::wr::<u32>(obj_a.as_mut_ptr() as usize + 0xc, 4);

        exe::wr::<usize>(obj_b.as_mut_ptr() as usize, buf_b.as_mut_ptr() as usize);
        exe::wr::<u32>(obj_b.as_mut_ptr() as usize + 8, len);
        exe::wr::<u32>(obj_b.as_mut_ptr() as usize + 0xc, 4);

        let a0 = rng.next_u64();
        let a1 = rng.next_u64();
        let s0 = rng.next_u32();
        let s1 = rng.next_u32();

        o(obj_a.as_mut_ptr(), a0, a1, 0, s0, s1);
        n(obj_b.as_mut_ptr(), a0, a1, 0, s0, s1);

        difftest::eq("push48 len", &obj_a[8..16], &obj_b[8..16])?;
        difftest::eq("push48 buf", &buf_a[..], &buf_b[..])?;
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_bitmask_slab_alloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let mut cases = 0u32;

    let saved_global = exe::rd::<usize>(SLAB_ALLOC_GLOBAL_VA as usize);
    static mut NEXT_NODE: usize = 0;
    unsafe extern "win64" fn mock_alloc_node(_pool: usize) -> usize {
        unsafe { NEXT_NODE }
    }
    let _guard = crate::difftest::Mock::new(0x1_4243_b450, mock_alloc_node as *const ())?;

    for (req_size, init_mask) in [(4096usize, 0u64), (8192, 0b0011), (12288, 0b0101_0001)] {
        let mut node_a = [0xffu8; 0x30];
        let mut node_b = [0xffu8; 0x30];
        let mut page_a = [0u64; 4];
        let mut page_b = [0u64; 4];
        page_a[2] = 64 * 4096;
        page_a[3] = init_mask;
        page_b[2] = 64 * 4096;
        page_b[3] = init_mask;

        let pages_a = [page_a.as_mut_ptr() as usize];
        let pages_b = [page_b.as_mut_ptr() as usize];

        let mut mgr_a = vec![0u8; 0x100];
        let mut mgr_b = vec![0u8; 0x100];
        for (mgr, pages) in [(&mut mgr_a, &pages_a), (&mut mgr_b, &pages_b)] {
            let m = mgr.as_mut_ptr() as usize;
            exe::wr::<usize>(m, pages.as_ptr() as usize);
            exe::wr::<u32>(m + 0x8, 1);
            exe::wr::<u64>(m + 0xb8, 4096);
            exe::wr::<i32>(m + 0xc8, -1);
        }

        NEXT_NODE = node_a.as_mut_ptr() as usize;
        exe::wr::<usize>(SLAB_ALLOC_GLOBAL_VA as usize, mgr_a.as_mut_ptr() as usize);
        let ra = o(0, req_size);

        NEXT_NODE = node_b.as_mut_ptr() as usize;
        exe::wr::<usize>(SLAB_ALLOC_GLOBAL_VA as usize, mgr_b.as_mut_ptr() as usize);
        let rb = n(0, req_size);

        difftest::eq("slab ret", ra == node_a.as_ptr() as usize, rb == node_b.as_ptr() as usize)?;
        difftest::eq("slab page", page_a, page_b)?;
        difftest::eq("slab node mid", &node_a[8..0x20], &node_b[8..0x20])?;
        difftest::eq("slab mgr d8", &mgr_a[0xd8..0xe8], &mgr_b[0xd8..0xe8])?;
        cases += 1;
    }

    exe::wr::<usize>(SLAB_ALLOC_GLOBAL_VA as usize, saved_global);
    Ok(cases)
}
