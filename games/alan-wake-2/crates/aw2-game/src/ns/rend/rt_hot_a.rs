//! Render-thread hot helpers that were lifted drafts (picked by render-thread call rate, callprof
//! cip1): small Remedy array/pool routines called 50-100k times per second.

use crate::difftest::{Mock, Snapshot};
use crate::exe::{self, rd, wr};
use std::cell::RefCell;

/// `Array<T>::reserve(this, n)` for 0x30-byte elements (`{T* data; u32 len; u32 cap}`).
const ARRAY48_RESERVE_VA: u64 = 0x1_4020_afa0;

/// Append a zero-initialized 0x30-byte element to `{T* data; u32 len; u32 cap}` and return it.
///
/// Original: 0x1413103c0 (131 bytes, FH4 noexcept, Throws::No: only an indirect allocator
/// call below). When `len * 0x30 == cap * 0x30` it calls the reserve 0x14020afa0 with
/// `len + 1` (the original derives it as `len * 0x30 / 0x30 + 1` via a magic multiply);
/// then zeroes `data[len]` (one 32-byte + one 16-byte store), increments `len`, and
/// returns `data + len * 0x30 - 0x30` re-read after the increment.
/// ~58k calls/s on the render thread (callprof cip1).
pub unsafe extern "win64" fn array48_push_zeroed(arr: usize) -> usize {
    let len = rd::<u32>(arr + 0x08) as u64;
    let cap = rd::<u32>(arr + 0x0c) as u64;
    if len * 0x30 == cap * 0x30 {
        exe::call_va!(ARRAY48_RESERVE_VA, (arr, len + 1));
    }
    let len = rd::<u32>(arr + 0x08) as usize;
    let p = rd::<usize>(arr).wrapping_add(len * 0x30);
    wr::<u128>(p, 0);
    wr::<u128>(p + 0x10, 0);
    wr::<u128>(p + 0x20, 0);
    wr::<u32>(arr + 0x08, (len as u32).wrapping_add(1));
    let len = rd::<u32>(arr + 0x08) as usize;
    (len * 0x30).wrapping_sub(0x30).wrapping_add(rd::<usize>(arr))
}

/// Current owner key compared against each pooled object's `+8` (`-1` = free).
const POOL_KEY_VA: u64 = 0x1_43a3_5918;
/// `claim(obj, key)`: binds a pooled object to `key`.
const POOL_CLAIM_VA: u64 = 0x1_4205_7060;
/// Allocates and constructs a new pooled object.
const POOL_NEW_VA: u64 = 0x1_411d_9a10;
/// `Array<T*>::reserve(this, n)`.
const PTR_ARRAY_RESERVE_VA: u64 = 0x1_4020_bcb0;

/// Find-or-claim the pooled object for the current key in `{T** data; u32 len; u32 cap}`.
///
/// Original: 0x1411d9920 (225 bytes; FH4 cleanup map, Throws::No: the only throwing path is
/// an indirect allocator call). Reads the key from 0x143a35918 once. First pass: return the
/// element whose `+8` equals the key. Second pass: the first element whose `+8` is `-1` is
/// claimed with 0x142057060(obj, key) and returned. Otherwise a new object from 0x1411d9a10
/// is claimed and appended (reserve 0x14020bcb0 with `len + 1` when `len == cap`).
/// ~103k calls/s on the render thread (callprof cip1).
pub unsafe extern "win64" fn pool_acquire_for_key(arr: usize) -> usize {
    let key = rd::<u64>(exe::ptr::<u64>(POOL_KEY_VA) as usize);
    let data = rd::<usize>(arr);
    let end = data.wrapping_add(rd::<u32>(arr + 0x08) as usize * 8);
    let mut p = data;
    while p != end {
        let obj = rd::<usize>(p);
        if rd::<u64>(obj + 0x08) == key {
            return obj;
        }
        p += 8;
    }
    let mut p = data;
    while p != end {
        let obj = rd::<usize>(p);
        if rd::<u64>(obj + 0x08) == u64::MAX {
            exe::call_va!(POOL_CLAIM_VA, (obj, key));
            return obj;
        }
        p += 8;
    }
    let obj = exe::call_va!(POOL_NEW_VA, () -> usize);
    exe::call_va!(POOL_CLAIM_VA, (obj, key));
    let len = rd::<u32>(arr + 0x08) as u64;
    let cap = rd::<u32>(arr + 0x0c) as u64;
    if len * 8 == cap * 8 {
        exe::call_va!(PTR_ARRAY_RESERVE_VA, (arr, len + 1));
    }
    let len = rd::<u32>(arr + 0x08);
    wr::<usize>(rd::<usize>(arr) + len as usize * 8, obj);
    wr::<u32>(arr + 0x08, len.wrapping_add(1));
    obj
}

// ---------------------------------------------------------------------------------------------
// Difftests

thread_local! {
    static LOG: RefCell<Vec<(u64, u64, u64)>> = const { RefCell::new(Vec::new()) };
    static BUFS: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

fn log_call(tag: u64, a: u64, b: u64) {
    LOG.with(|l| l.borrow_mut().push((tag, a, b)));
}

fn take_log() -> Vec<(u64, u64, u64)> {
    LOG.with(|l| std::mem::take(&mut *l.borrow_mut()))
}

/// Test buffer from a per-thread pool (kept alive for the whole difftest), filled with 0xCD.
fn fake_alloc(size: usize) -> usize {
    let mut b = vec![0xCDu8; size + 8];
    let p = b.as_mut_ptr() as usize;
    BUFS.with(|v| v.borrow_mut().push(b));
    p
}

/// Fixture reserve for `{T* data; u32 len; u32 cap}`: new 0xCD buffer, copies `len` elements.
unsafe fn fake_reserve(arr: usize, n: u64, elem: usize, tag: u64) {
    log_call(tag, rd::<u32>(arr + 8) as u64, n);
    let len = rd::<u32>(arr + 8) as usize;
    let nb = fake_alloc(n as usize * elem);
    core::ptr::copy_nonoverlapping(rd::<usize>(arr) as *const u8, nb as *mut u8, len * elem);
    wr::<usize>(arr, nb);
    wr::<u32>(arr + 0x0c, n as u32);
}

unsafe extern "win64" fn fake_reserve48(arr: usize, n: u64) {
    fake_reserve(arr, n, 0x30, 1);
}

unsafe extern "win64" fn fake_reserve_ptr(arr: usize, n: u64) {
    fake_reserve(arr, n, 8, 2);
}

unsafe extern "win64" fn fake_claim(obj: usize, key: u64) {
    log_call(3, rd::<u64>(obj + 8), key);
    wr::<u64>(obj + 8, key);
}

thread_local! {
    static NEW_OBJS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_new() -> usize {
    let o = fake_alloc(0x20);
    wr::<u64>(o + 8, 0x1234);
    NEW_OBJS.with(|v| v.borrow_mut().push(o));
    log_call(4, 0, 0);
    o
}

/// Array header `{data, len, cap}` at a possibly 4-misaligned address inside `hdr`.
unsafe fn hdr_at(hdr: &mut [u8; 0x20], mis: usize) -> usize {
    hdr.as_mut_ptr() as usize + mis
}

pub unsafe fn diff_array48_push_zeroed(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m = Mock::new(ARRAY48_RESERVE_VA, fake_reserve48 as *const ())?;
    let mut rng = crate::difftest::Rng::new(0x48_48);
    let mut cases = 0u32;
    for &(len, cap) in &[(0u32, 0u32), (0, 1), (1, 1), (3, 8), (7, 8), (8, 8), (15, 16), (16, 16), (5, 100)] {
        for mis in [0usize, 4] {
            let seed = rng.next_u64();
            let mut res = Vec::new();
            for f in [o, n] {
                let mut hdr = [0u8; 0x20];
                let a = hdr_at(&mut hdr, mis);
                let buf = fake_alloc(cap as usize * 0x30);
                let mut r2 = crate::difftest::Rng::new(seed);
                for i in 0..cap as usize * 0x30 {
                    wr::<u8>(buf + i, r2.next_u32() as u8);
                }
                wr::<usize>(a, buf);
                wr::<u32>(a + 8, len);
                wr::<u32>(a + 0x0c, cap);
                take_log();
                let ret = f(a);
                let data = rd::<usize>(a);
                let nl = rd::<u32>(a + 8);
                let nc = rd::<u32>(a + 0x0c);
                let contents = std::slice::from_raw_parts(data as *const u8, nl as usize * 0x30).to_vec();
                res.push((ret.wrapping_sub(data), nl, nc, contents, take_log(), data == buf));
            }
            crate::diff_eq!(res[0], res[1], "len={len} cap={cap} mis={mis}");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_pool_acquire_for_key(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = Mock::new(POOL_CLAIM_VA, fake_claim as *const ())?;
    let _m2 = Mock::new(POOL_NEW_VA, fake_new as *const ())?;
    let _m3 = Mock::new(PTR_ARRAY_RESERVE_VA, fake_reserve_ptr as *const ())?;
    let key_snap = Snapshot::take(exe::ptr::<u8>(POOL_KEY_VA) as *mut u8, 8);
    let key: u64 = 0x5eed_0000_0000_0042;
    wr::<u64>(exe::ptr::<u64>(POOL_KEY_VA) as usize, key);
    // Object +8 values: k = key, f = free (-1), x = other.
    let layouts: &[&[u8]] = &[b"", b"k", b"x", b"f", b"xxk", b"xfk", b"fxk", b"xxf", b"xfxf", b"xxx", b"kk", b"ff"];
    let mut cases = 0u32;
    let result = (|| -> Result<(), String> {
        for (li, lay) in layouts.iter().enumerate() {
            for &extra_cap in &[0u32, 2] {
                for mis in [0usize, 4] {
                    let mut res = Vec::new();
                    for f in [o, n] {
                        let len = lay.len() as u32;
                        let cap = len + extra_cap;
                        let mut hdr = [0u8; 0x20];
                        let a = hdr_at(&mut hdr, mis);
                        let data = fake_alloc(cap as usize * 8);
                        let mut objs = Vec::new();
                        for (i, c) in lay.iter().enumerate() {
                            let obj = fake_alloc(0x20);
                            let v = match c { b'k' => key, b'f' => u64::MAX, _ => 0x1000 + i as u64 };
                            wr::<u64>(obj + 8, v);
                            wr::<usize>(data + i * 8, obj);
                            objs.push(obj);
                        }
                        wr::<usize>(a, data);
                        wr::<u32>(a + 8, len);
                        wr::<u32>(a + 0x0c, cap);
                        take_log();
                        NEW_OBJS.with(|v| v.borrow_mut().clear());
                        let ret = f(a);
                        let new_objs = NEW_OBJS.with(|v| v.borrow().clone());
                        // Classify the returned pointer: index into the original objects or new object.
                        let which = objs.iter().position(|&x| x == ret).map(|i| i as i64)
                            .or_else(|| new_objs.iter().position(|&x| x == ret).map(|i| -1 - i as i64))
                            .unwrap_or(i64::MIN);
                        let nl = rd::<u32>(a + 8);
                        let nc = rd::<u32>(a + 0x0c);
                        let d = rd::<usize>(a);
                        let elems: Vec<i64> = (0..nl as usize).map(|i| {
                            let e = rd::<usize>(d + i * 8);
                            objs.iter().position(|&x| x == e).map(|i| i as i64)
                                .or_else(|| new_objs.iter().position(|&x| x == e).map(|i| -1 - i as i64))
                                .unwrap_or(i64::MIN)
                        }).collect();
                        let tags: Vec<u64> = objs.iter().map(|&x| rd::<u64>(x + 8)).collect();
                        let log = take_log();
                        res.push((which, nl, nc, elems, tags, log));
                    }
                    crate::diff_eq!(res[0], res[1], "layout#{li} {:?} extra_cap={extra_cap} mis={mis}", std::str::from_utf8(lay).unwrap_or("?"));
                    cases += 1;
                }
            }
        }
        Ok(())
    })();
    key_snap.restore();
    result.map(|_| cases)
}

/// Handle table that owns the `+0x38` handle released by [`reset_handle_and_ref`].
const HANDLE_TABLE_VA: u64 = 0x1_43a3_65b8;
/// `release(table, &handle)`.
const HANDLE_RELEASE_VA: u64 = 0x1_4220_4820;
/// Last-reference callback `mgr->on_zero(mgr, obj)`.
const REF_ZERO_VA: u64 = 0x1_4241_d6f0;

/// Release the `+0x38` handle (16-byte `{u64; u32; u16}` slot) and the `+0x10` ref-counted
/// object, then clear both; returns `this`.
///
/// Original: 0x141369ca0 (128 bytes, FH4 noexcept; Throws::Discard: 0x142204820 can throw).
/// If `[this+0x38] != 0`: 0x142204820([0x143a365b8], this + 0x38), then the slot is
/// overwritten by a 16-byte copy of a stack temporary whose first 14 bytes are zeroed (bytes
/// 14..16 are uninitialized stack in the original; zero here, as in the zero-initialized lifted
/// frame). If `[this+0x10] != 0`: `lock xadd [obj+0x24], -1`, and when the old count was 1
/// and `[obj+0x28] != 0`, 0x14241d6f0(mgr, obj). Finally `[this+0x10] = 0`.
/// ~60k calls/s on the render thread (callprof cip1).
pub unsafe extern "win64" fn reset_handle_and_ref(this: usize) -> usize {
    if rd::<u64>(this + 0x38) != 0 {
        let table = rd::<usize>(exe::ptr::<u64>(HANDLE_TABLE_VA) as usize);
        exe::call_va!(HANDLE_RELEASE_VA, (table, this + 0x38));
        wr::<u128>(this + 0x38, 0);
    }
    let obj = rd::<usize>(this + 0x10);
    if obj != 0 {
        let old = crate::simd_ops::atomic_xadd_u32((obj + 0x24) as u64, u32::MAX);
        if old == 1 {
            let mgr = rd::<usize>(obj + 0x28);
            if mgr != 0 {
                exe::call_va!(REF_ZERO_VA, (mgr, obj));
            }
        }
    }
    wr::<u64>(this + 0x10, 0);
    this
}

unsafe extern "win64" fn fake_handle_release(table: usize, h: usize) {
    log_call(5, table as u64, rd::<u64>(h));
}

unsafe extern "win64" fn fake_ref_zero(mgr: usize, obj: usize) {
    log_call(6, mgr as u64, rd::<u32>(obj + 0x24) as u64);
}

pub unsafe fn diff_reset_handle_and_ref(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = Mock::new(HANDLE_RELEASE_VA, fake_handle_release as *const ())?;
    let _m2 = Mock::new(REF_ZERO_VA, fake_ref_zero as *const ())?;
    let mut cases = 0u32;
    // (handle, has_obj, refcount, mgr)
    let shapes: &[(u64, bool, u32, u64)] = &[
        (0, false, 0, 0),
        (0x77, false, 0, 0),
        (0, true, 1, 0x5000),
        (0, true, 2, 0x5000),
        (0, true, 1, 0),
        (0x1234_5678_9abc, true, 1, 0x6000),
        (0x99, true, 0, 0x6000),
        (u64::MAX, true, 3, 0),
    ];
    for (si, &(handle, has_obj, rc, mgr)) in shapes.iter().enumerate() {
        for mis in [0usize, 4] {
            let mut res = Vec::new();
            for f in [o, n] {
                let mut buf = [0xA5u8; 0x60];
                let t = buf.as_mut_ptr() as usize + mis;
                wr::<u64>(t + 0x38, handle);
                let mut obj = [0x3Cu8; 0x40];
                let op = obj.as_mut_ptr() as usize;
                wr::<u32>(op + 0x24, rc);
                wr::<u64>(op + 0x28, mgr);
                wr::<usize>(t + 0x10, if has_obj { op } else { 0 });
                take_log();
                let ret = f(t);
                let mut bytes = std::slice::from_raw_parts(t as *const u8, 0x58).to_vec();
                // Bytes 0x46..0x48: uninitialized stack in the original's copy.
                if handle != 0 {
                    bytes[0x46] = 0;
                    bytes[0x47] = 0;
                }
                res.push((ret.wrapping_sub(t), bytes, rd::<u32>(op + 0x24), take_log()));
            }
            crate::diff_eq!(res[0], res[1], "shape#{si} mis={mis}");
            cases += 1;
        }
    }
    Ok(cases)
}

/// Locks the shared command list and fills `{lock*, ID3D12GraphicsCommandList*}`.
const LOCKED_CMDLIST_VA: u64 = 0x1_411d_5b00;
const IAT_RELEASE_SRW_EXCLUSIVE: u64 = 0x1_4264_5598;

/// Transition a resource to its default read state (`((flags & 2) | 0xc) << 4`) with one
/// `ResourceBarrier` on the shared command list, if it is not already in that state.
///
/// Original: 0x1420694a0 (180 bytes; FH4 cleanup for the lock guard; Throws::No: only the
/// indirect `ResourceBarrier` call below). Skips when `[this+0x70]` (the resource) is null or
/// `byte [this+0x50] != 0`. State word: `[holder+0x18]` with holder = `[this+0x40]` or
/// `this + 0x30`. Barrier: `{type 0, flags 0, pResource, subresource -1, before, after, 0}`.
/// Guard from 0x1411d5b00; vtable slot 0xd0 (`ResourceBarrier(1, &b)`); guard release:
/// `--depth`, on zero `xchg owner, -1` + `ReleaseSRWLockExclusive`. Stores `after` last.
/// ~57k calls/s on the render thread (callprof cip1).
pub unsafe extern "win64" fn transition_to_default_state(this: usize) {
    let res = rd::<usize>(this + 0x70);
    if res == 0 || rd::<u8>(this + 0x50) != 0 {
        return;
    }
    let mut holder = rd::<usize>(this + 0x40);
    if holder == 0 {
        holder = this + 0x30;
    }
    let before = rd::<u32>(holder + 0x18);
    let after = ((rd::<u32>(this + 0x10) & 2) | 0xc) << 4;
    if after == before {
        return;
    }
    let mut barrier = [0u32; 8];
    barrier[2] = res as u32;
    barrier[3] = (res >> 32) as u32;
    barrier[4] = u32::MAX;
    barrier[5] = before;
    barrier[6] = after;
    let mut guard = [0usize; 2];
    exe::call_va!(LOCKED_CMDLIST_VA, (guard.as_mut_ptr() as usize));
    let cl = guard[1];
    let rb = rd::<usize>(rd::<usize>(cl) + 0xd0);
    exe::call_va!(rb, (cl, 1u64, barrier.as_ptr() as usize));
    let lock = guard[0];
    if lock != 0 {
        let d = rd::<u32>(lock + 0x0c).wrapping_sub(1);
        wr::<u32>(lock + 0x0c, d);
        if d == 0 {
            crate::simd_ops::atomic_xchg_u32((lock + 0x08) as u64, u32::MAX);
            exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (lock) -> ());
        }
    }
    wr::<u32>(holder + 0x18, after);
}

thread_local! {
    static FAKE_GUARD: core::cell::Cell<(usize, usize)> = const { core::cell::Cell::new((0, 0)) };
}

unsafe extern "win64" fn fake_locked_cmdlist(out: usize) -> usize {
    let (lock, cl) = FAKE_GUARD.with(|g| g.get());
    log_call(7, 0, 0);
    wr::<usize>(out, lock);
    wr::<usize>(out + 8, cl);
    out
}

unsafe extern "win64" fn fake_resource_barrier(cl: usize, n: u32, b: usize) {
    let w: Vec<u32> = (0..8).map(|i| rd::<u32>(b + i * 4)).collect();
    log_call(8, (cl as u64) << 8 | n as u64, w.iter().fold(0u64, |h, &x| h.rotate_left(7) ^ x as u64));
}

unsafe extern "win64" fn fake_release_srw(lock: usize) {
    log_call(9, rd::<u32>(lock + 8) as u64, rd::<u32>(lock + 0xc) as u64);
}

pub unsafe fn diff_transition_to_default_state(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize);
    let (o, n): (F, F) = (core::mem::transmute(orig), core::mem::transmute(new));
    let _m1 = Mock::new(LOCKED_CMDLIST_VA, fake_locked_cmdlist as *const ())?;
    let _m2 = crate::difftest::IatMock::new(IAT_RELEASE_SRW_EXCLUSIVE, fake_release_srw as *const ())?;
    let mut vtbl = [0usize; 0x20];
    vtbl[0xd0 / 8] = fake_resource_barrier as *const () as usize;
    let mut cl_obj = [0usize; 2];
    cl_obj[0] = vtbl.as_ptr() as usize;
    let cl = cl_obj.as_mut_ptr() as usize;
    let mut cases = 0u32;
    // (res, skip byte, use holder ptr, flags, before, lock depth (0 = no lock))
    let shapes: &[(u64, u8, bool, u32, u32, u32)] = &[
        (0, 0, false, 0, 0, 1),
        (0xabc0, 1, false, 0, 0, 1),
        (0xabc0, 0, false, 0, 0xc0, 1),
        (0xabc0, 0, false, 2, 0xe0, 1),
        (0xabc0, 0, false, 0, 0x1, 1),
        (0xabc0, 0, true, 2, 0x4, 1),
        (0xabc0, 0, true, 0x7fff_fffd, 0xc0, 2),
        (0x1_0000_abc0, 0, false, 0, 0x800, 0),
        (0x1_0000_abc0, 0, true, 3, 0xc0, 3),
    ];
    for (si, &(res, skip, use_holder, flags, before, depth)) in shapes.iter().enumerate() {
        for mis in [0usize, 4] {
            let mut out = Vec::new();
            for f in [o, n] {
                let mut buf = [0x5Au8; 0x90];
                let t = buf.as_mut_ptr() as usize + mis;
                let mut hold = [0x11u8; 0x30];
                let h = hold.as_mut_ptr() as usize + mis;
                wr::<u64>(t + 0x70, res);
                wr::<u8>(t + 0x50, skip);
                wr::<u32>(t + 0x10, flags);
                wr::<usize>(t + 0x40, if use_holder { h } else { 0 });
                let state_at = if use_holder { h } else { t + 0x30 };
                wr::<u32>(state_at + 0x18, before);
                let mut lock = [0u32; 4];
                lock[2] = 0x4242;
                lock[3] = depth;
                let lp = if depth == 0 { 0 } else { lock.as_mut_ptr() as usize };
                FAKE_GUARD.with(|g| g.set((lp, cl)));
                take_log();
                f(t);
                let bytes = std::slice::from_raw_parts(t as *const u8, 0x88).to_vec();
                let hb = std::slice::from_raw_parts(h as *const u8, 0x28).to_vec();
                out.push((bytes, hb, lock, take_log()));
            }
            crate::diff_eq!(out[0], out[1], "shape#{si} mis={mis}");
            cases += 1;
        }
    }
    Ok(cases)
}
