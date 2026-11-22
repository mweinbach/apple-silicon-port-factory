//! Small `r::` thread/time helpers that sit inside the engine's polling loops
//! (render-thread job pump `rend::RendererThreading::vfunc_1`, job-queue pops).
//! Each is a few instructions on its fast path but is called millions of times
//! per second, so running it natively removes most of the interpreter overhead
//! of those loops.
#![allow(static_mut_refs)]

use crate::exe::{self, rd, wr};

/// Function-local static guard (`_Init_thread` pattern) for the cached frequency.
const FREQ_GUARD_VA: u64 = 0x1_43d0_1018;
/// Cached `QueryPerformanceFrequency` value.
const FREQ_VA: u64 = 0x1_43d0_1028;
/// `_Init_thread_header(int* guard)`.
const INIT_THREAD_HEADER_VA: u64 = 0x1_4183_4a78;
/// `_Init_thread_footer(int* guard)`.
const INIT_THREAD_FOOTER_VA: u64 = 0x1_4183_4a0c;
/// IAT slot of `kernel32!QueryPerformanceFrequency`.
const IAT_QPF: u64 = 0x1_4264_5700;

/// `QueryPerformanceFrequency`, cached in a function-local static.
///
/// Original: 0x142406920 (117 bytes). Fast path: `[[gs:0x58]+0x74]` (the
/// thread's `_Init_thread_epoch`) `>= guard` returns the cached value. Slow
/// path: `_Init_thread_header(&guard)`; if the guard is then -1, calls QPF
/// through the IAT, stores the value (0 if QPF failed) and runs
/// `_Init_thread_footer(&guard)`. Returns the cached value (i64).
pub unsafe extern "win64" fn perf_frequency() -> i64 {
    let tls0 = rd::<u64>(exe::tls_pointer());
    let epoch = rd::<i32>(tls0 as usize + 0x74);
    let guard = exe::ptr::<i32>(FREQ_GUARD_VA) as usize;
    if rd::<i32>(guard) > epoch {
        exe::call_va!(INIT_THREAD_HEADER_VA, (guard));
        if rd::<i32>(guard) == -1 {
            let mut f: i64 = 0;
            let ok = exe::call_iat!(IAT_QPF, (&mut f as *mut i64) -> i32);
            wr::<i64>(exe::ptr::<i64>(FREQ_VA) as usize, if ok != 0 { f } else { 0 });
            exe::call_va!(INIT_THREAD_FOOTER_VA, (guard));
        }
    }
    rd::<i64>(exe::ptr::<i64>(FREQ_VA) as usize)
}

/// Returns the slot allocator for worker indices (SRW lock at +0, free-list
/// count at +0x108, free list of u32 at +4.., next fresh index at +0x110).
const WORKER_INDEX_POOL_VA: u64 = 0x1_4240_7b30;
const IAT_ACQUIRE_SRW_EXCLUSIVE: u64 = 0x1_4264_55a8;
const IAT_RELEASE_SRW_EXCLUSIVE: u64 = 0x1_4264_5598;
/// `__tlregdtor(void (*)(void))` registers a thread-exit callback.
const TLREGDTOR_VA: u64 = 0x1_4183_4e48;
/// Thread-exit callback that returns this thread's index to the pool.
const RELEASE_WORKER_INDEX_VA: u64 = 0x1_4263_f620;

/// This thread's small worker index, allocated on first use.
///
/// Original: 0x142407c80 (183 bytes). Thread-local state in the exe's TLS
/// block (`[gs:0x58]` entry 0): flags u32 at +0xa86b0 (bit 0 = initialized),
/// index u32 at +0xa86b4. Fast path returns the cached index. Slow path sets
/// the flag, takes the pool lock, pops a recycled index from the free list
/// (`list[count]` at +4 + count*4 with `count` at +0x108, then `count -= 1`)
/// or hands out `next++` (+0x110), unlocks, stores the index, registers the
/// release callback 0x14263f620 with `__tlregdtor`, and returns the stored index.
pub unsafe extern "win64" fn worker_index() -> u32 {
    let tls0 = rd::<u64>(exe::tls_pointer()) as usize;
    let flags = rd::<u32>(tls0 + 0xa86b0);
    if flags & 1 != 0 {
        return rd::<u32>(tls0 + 0xa86b4);
    }
    wr::<u32>(tls0 + 0xa86b0, flags | 1);
    let pool = exe::call_va!(WORKER_INDEX_POOL_VA, () -> u64) as usize;
    exe::call_iat!(IAT_ACQUIRE_SRW_EXCLUSIVE, (pool));
    let count = rd::<u64>(pool + 0x108);
    let idx = if count == 0 {
        let n = rd::<u32>(pool + 0x110);
        wr::<u32>(pool + 0x110, n.wrapping_add(1));
        n
    } else {
        let v = rd::<u32>(pool + 4 * count as usize + 4);
        wr::<u64>(pool + 0x108, count.wrapping_sub(1));
        v
    };
    exe::call_iat!(IAT_RELEASE_SRW_EXCLUSIVE, (pool));
    wr::<u32>(tls0 + 0xa86b4, idx);
    exe::call_va!(TLREGDTOR_VA, (exe::ptr::<u8>(RELEASE_WORKER_INDEX_VA)));
    rd::<u32>(tls0 + 0xa86b4)
}

// ---------------------------------------------------------------------------
// Differential tests
// ---------------------------------------------------------------------------

use crate::difftest::SKIP;

/// Make sure this thread has a TEB and the exe's static TLS (adopted by any
/// kernel32 import call), else skip.
unsafe fn adopt_thread() -> Result<usize, String> {
    // GetCurrentThreadId through the IAT adopts the calling host thread.
    let _ = exe::call_iat!(0x1_4264_5790u64, () -> u32);
    let tp = exe::tls_pointer();
    if tp == 0 || rd::<u64>(tp) == 0 {
        return Err(format!("{SKIP} no game TLS on this thread"));
    }
    Ok(rd::<u64>(tp) as usize)
}

pub unsafe fn diff_perf_frequency(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> i64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let tls0 = adopt_thread()?;
    let guard = exe::ptr::<i32>(FREQ_GUARD_VA) as usize;
    let freq = exe::ptr::<i64>(FREQ_VA) as usize;
    let (g0, f0, e0) = (rd::<i32>(guard), rd::<i64>(freq), rd::<i32>(tls0 + 0x74));
    let mut cases = 0;
    // Slow path: reset the guard/epoch so the static is initialized again.
    for _ in 0..3 {
        wr::<i32>(guard, 0);
        wr::<i64>(freq, 0x1234);
        wr::<i32>(tls0 + 0x74, i32::MIN);
        let a = o();
        let (ga, fa) = (rd::<i32>(guard), rd::<i64>(freq));
        wr::<i32>(guard, 0);
        wr::<i64>(freq, 0x1234);
        wr::<i32>(tls0 + 0x74, i32::MIN);
        let b = n();
        let (gb, fb) = (rd::<i32>(guard), rd::<i64>(freq));
        crate::diff_eq!(a, b, "slow path result");
        crate::diff_eq!(fa, fb, "slow path cached value");
        crate::diff_eq!(ga > 0, gb > 0, "slow path guard published");
        cases += 1;
        // Fast path afterwards (epoch now >= guard).
        crate::diff_eq!(o(), n(), "fast path");
        cases += 1;
    }
    wr::<i32>(guard, g0);
    wr::<i64>(freq, f0);
    wr::<i32>(tls0 + 0x74, e0);
    Ok(cases)
}

pub unsafe fn diff_worker_index(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let tls0 = adopt_thread()?;
    let pool = exe::call_va!(WORKER_INDEX_POOL_VA, () -> u64) as usize;
    let snap = |p: usize| -> Vec<u8> { (0..0x118).map(|k| rd::<u8>(p + k)).collect() };
    let restore = |p: usize, s: &[u8]| {
        for (k, b) in s.iter().enumerate() {
            wr::<u8>(p + k, *b);
        }
    };
    let saved_pool = snap(pool);
    let (sf, si) = (rd::<u32>(tls0 + 0xa86b0), rd::<u32>(tls0 + 0xa86b4));
    let mut cases = 0;
    // Slow path with an empty free list, then with recycled entries.
    for free in [0u64, 1, 3] {
        let mut st = saved_pool.clone();
        st[0x108..0x110].copy_from_slice(&free.to_le_bytes());
        for k in 0..free as usize {
            st[4 + 4 * (k + 1)..8 + 4 * (k + 1)].copy_from_slice(&(100 + k as u32).to_le_bytes());
        }
        st[0x110..0x114].copy_from_slice(&7u32.to_le_bytes());
        restore(pool, &st);
        wr::<u32>(tls0 + 0xa86b0, 0x10);
        let a = o();
        let (pa, fa) = (snap(pool), rd::<u32>(tls0 + 0xa86b0));
        restore(pool, &st);
        wr::<u32>(tls0 + 0xa86b0, 0x10);
        let b = n();
        let (pb, fb) = (snap(pool), rd::<u32>(tls0 + 0xa86b0));
        crate::diff_eq!(a, b, "slow path, free list {free}");
        crate::diff_eq!(fa, fb, "flags, free list {free}");
        if pa != pb {
            return Err(format!("pool state differs, free list {free}"));
        }
        crate::diff_eq!(o(), n(), "fast path");
        cases += 2;
    }
    restore(pool, &saved_pool);
    wr::<u32>(tls0 + 0xa86b0, sf);
    wr::<u32>(tls0 + 0xa86b4, si);
    Ok(cases)
}

// ---------------------------------------------------------------------------
// High-resolution timing (`perf_counter` 0x142406a50, `perf_elapsed_seconds` 0x1424069a0)
// ---------------------------------------------------------------------------

/// IAT slot of `kernel32!QueryPerformanceCounter`.
const IAT_QPC: u64 = 0x1_4264_56f8;
/// Function-local static guard for `perf_elapsed_seconds` start timestamp.
const START_QPC_GUARD_VA: u64 = 0x1_43d0_1014;
/// Cached start `QueryPerformanceCounter` value for `perf_elapsed_seconds`.
const START_QPC_VA: u64 = 0x1_43d0_1020;

/// Thin wrapper around `QueryPerformanceCounter` returning the raw 64-bit tick count.
///
/// Original: 0x142406a50 (25 bytes).
pub unsafe extern "win64" fn perf_counter() -> i64 {
    qpc_iat()
}

/// `QueryPerformanceCounter(&now)` through the exe's IAT slot; when the slot
/// holds our kernel32 builtin (always, outside difftest mocks) it is called
/// directly instead of through the generic host-call dispatch.
#[inline]
pub(crate) unsafe fn qpc_iat() -> i64 {
    let slot = rd::<u64>(exe::ptr::<u64>(IAT_QPC) as usize);
    let mut now: i64 = 0;
    if crate::emu::resolve_dispatch(slot).0 == win32_rt::kernel32::time::QueryPerformanceCounter as *const () as usize {
        win32_rt::kernel32::time::QueryPerformanceCounter(&mut now);
    } else {
        let _ = exe::call_va!(slot, (&mut now as *mut i64) -> i32);
    }
    now
}

/// Elapsed seconds (`f64` in `xmm0`) since the first call's `QueryPerformanceCounter` baseline.
///
/// Original: 0x1424069a0 (160 bytes). Calls `QueryPerformanceCounter(&now)`, checks
/// `_Init_thread_epoch` (`[[gs:0x58]+0x74]`) against the static guard at `0x143d01014`
/// (initializing `0x143d01020` via `QueryPerformanceCounter` on first run), and returns
/// `((now - start) as f64) / (perf_frequency() as f64)`.
pub unsafe extern "win64" fn perf_elapsed_seconds() -> f64 {
    let now = qpc_iat();
    let tls0 = rd::<u64>(exe::tls_pointer()) as usize;
    let epoch = rd::<i32>(tls0 + 0x74);
    let guard = exe::ptr::<i32>(START_QPC_GUARD_VA) as usize;
    if rd::<i32>(guard) > epoch {
        exe::call_va!(INIT_THREAD_HEADER_VA, (guard));
        if rd::<i32>(guard) == -1 {
            let start = qpc_iat();
            wr::<i64>(exe::ptr::<i64>(START_QPC_VA) as usize, start);
            exe::call_va!(INIT_THREAD_FOOTER_VA, (guard));
        }
    }
    let start = rd::<i64>(exe::ptr::<i64>(START_QPC_VA) as usize);
    let delta = now.wrapping_sub(start) as f64;
    let freq = exe::call_native!(0x1_4240_6920u64, perf_frequency, () -> i64) as f64;
    let r = delta / freq;
    crate::simd_ops::set_xmm_ret(r.to_bits() as u128);
    r
}

// ---------------------------------------------------------------------------
// Northlight Job / Task Scheduler hot functions
// ---------------------------------------------------------------------------

use std::sync::atomic::{
    fence, AtomicI32, AtomicU16, AtomicU32, AtomicU8, AtomicUsize, Ordering,
};

/// Global table of per-worker descriptor pointers (`[0x143c4aa00 + worker_idx * 8]`).
const WORKER_DESC_TABLE_VA: u64 = 0x1_43c4_aa00;
/// Global cluster count (`u32` at `0x143c1bfb0`) and cluster array pointer (`0x143c1bfa8`).
const CLUSTER_COUNT_VA: u64 = 0x1_43c1_bfb0;
const CLUSTER_ARRAY_VA: u64 = 0x1_43c1_bfa8;
/// IAT slot of `kernel32!ReleaseSemaphore`.
const IAT_RELEASE_SEMAPHORE: u64 = 0x1_4264_5580;

#[inline(always)]
unsafe fn load_acq_u8(addr: usize) -> u8 {
    AtomicU8::from_ptr(addr as *mut u8).load(Ordering::Acquire)
}

#[inline(always)]
unsafe fn load_acq_u16(addr: usize) -> u16 {
    if addr & 1 == 0 {
        AtomicU16::from_ptr(addr as *mut u16).load(Ordering::Acquire)
    } else {
        let v = rd::<u16>(addr);
        fence(Ordering::Acquire);
        v
    }
}

#[inline(always)]
unsafe fn store_rel_u16(addr: usize, val: u16) {
    if addr & 1 == 0 {
        AtomicU16::from_ptr(addr as *mut u16).store(val, Ordering::Release);
    } else {
        fence(Ordering::Release);
        wr::<u16>(addr, val);
    }
}

#[inline(always)]
unsafe fn load_acq_u32(addr: usize) -> u32 {
    if addr & 3 == 0 {
        AtomicU32::from_ptr(addr as *mut u32).load(Ordering::Acquire)
    } else {
        let v = rd::<u32>(addr);
        fence(Ordering::Acquire);
        v
    }
}

#[inline(always)]
unsafe fn load_acq_i32(addr: usize) -> i32 {
    if addr & 3 == 0 {
        AtomicI32::from_ptr(addr as *mut i32).load(Ordering::Acquire)
    } else {
        let v = rd::<i32>(addr);
        fence(Ordering::Acquire);
        v
    }
}

#[inline(always)]
unsafe fn store_rel_i32(addr: usize, val: i32) {
    if addr & 3 == 0 {
        AtomicI32::from_ptr(addr as *mut i32).store(val, Ordering::Release);
    } else {
        fence(Ordering::Release);
        wr::<i32>(addr, val);
    }
}

#[inline(always)]
unsafe fn load_acq_usize(addr: usize) -> usize {
    if addr & 7 == 0 {
        AtomicUsize::from_ptr(addr as *mut usize).load(Ordering::Acquire)
    } else {
        let v = rd::<usize>(addr);
        fence(Ordering::Acquire);
        v
    }
}

#[inline(always)]
unsafe fn store_rel_usize(addr: usize, val: usize) {
    if addr & 7 == 0 {
        AtomicUsize::from_ptr(addr as *mut usize).store(val, Ordering::Release);
    } else {
        fence(Ordering::Release);
        wr::<usize>(addr, val);
    }
}

#[inline(always)]
unsafe fn atomic_cmpxchg_u8(addr: usize, expected: u8, desired: u8) -> bool {
    AtomicU8::from_ptr(addr as *mut u8)
        .compare_exchange(expected, desired, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

#[inline(always)]
unsafe fn atomic_cmpxchg_i32(addr: usize, expected: i32, desired: i32) -> bool {
    AtomicI32::from_ptr(addr as *mut i32)
        .compare_exchange(expected, desired, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

#[inline(always)]
unsafe fn atomic_xadd_i32(addr: usize, delta: i32) -> i32 {
    AtomicI32::from_ptr(addr as *mut i32).fetch_add(delta, Ordering::SeqCst)
}

#[inline(always)]
unsafe fn bump_job_tag(node: usize) {
    let old_w = load_acq_u16(node + 6);
    let added = old_w.wrapping_add(8);
    let new_w = ((added ^ old_w) & 7) ^ added;
    store_rel_u16(node + 6, new_w);
}

/// Clear job node state (`[rdx + 4]` from `expected` to `0`) and advance its
/// 3-bit generation tag in bits 3..15 of `[rdx + 6]` (preserving low 3 priority bits).
///
/// Original: 0x142485a70 (39 bytes).
pub unsafe extern "win64" fn job_node_unlock_and_bump_tag(
    _unused: usize,
    node: usize,
    expected: u8,
) {
    let _ = atomic_cmpxchg_u8(node + 4, expected, 0);
    bump_job_tag(node);
}

/// Pop or steal a runnable job of priority class `prio` (`dx`) from `sched` (`rcx`).
///
/// Original: 0x142485c10 (760 bytes). Checks if `sched->ready_counts[prio]` (`+0x1040 + prio*4`)
/// is non-zero; if so, attempts to pop from the calling worker's Chase-Lev deque for `prio`.
/// If the local deque is empty and `allow_steal != 0` and stealing is enabled for this worker,
/// scans peer worker deques across the topology clusters (`0x143c1bfa8`/`0x143c1bfb0`) and
/// remaining worker slots up to 64.
pub unsafe extern "win64" fn scheduler_pop_job(
    sched: usize,
    prio: u16,
    allow_steal: u8,
) -> usize {
    let r15 = prio as usize;
    if load_acq_u32(sched + 0x1040 + r15 * 4) == 0 {
        return 0;
    }
    let w = exe::call_native!(0x1_4240_7c80u64, worker_index, () -> u32) as usize;
    let rbp = w as u32;
    let r13 = sched + (w << 6);
    let q_offset = ((prio as u8) as usize) << 6;
    let r8 = rd::<usize>(r13 + 0x48) + q_offset;

    let old_tail = atomic_xadd_i32(r8 + 0x24, -1);
    fence(Ordering::SeqCst);
    let head = load_acq_i32(r8 + 0x20);
    let new_tail = old_tail.wrapping_sub(1);
    let diff = old_tail.wrapping_sub(head);
    let next_head = head.wrapping_add(1);

    if diff >= 0 {
        if diff == 0 {
            let _ = atomic_xadd_i32(r8 + 0x24, 1);
        } else {
            let buf = rd::<usize>(r8 + 0x30);
            let mask = rd::<u32>(r8 + 0x28) as u64;
            let mut take_ok = true;
            let job = if new_tail == head {
                let idx = (mask & (new_tail as u32 as u64)) as usize;
                let j = load_acq_usize(buf + idx * 8);
                let cas_ok = atomic_cmpxchg_i32(r8 + 0x20, head, next_head);
                store_rel_i32(r8 + 0x24, next_head);
                if !cas_ok {
                    take_ok = false;
                }
                j
            } else {
                let idx = (mask & (new_tail as u32 as u64)) as usize;
                load_acq_usize(buf + idx * 8)
            };
            if take_ok && job != 0 {
                let tag_prio = (load_acq_u16(job + 6) as usize) & 7;
                let _ = atomic_xadd_i32(sched + 0x1040 + tag_prio * 4, -1);
                return job;
            }
        }
    } else {
        store_rel_i32(r8 + 0x24, head);
    }

    // Steal path (`0x142485c7e`).
    let rbx0 = rd::<usize>(sched);
    let w2 = exe::call_native!(0x1_4240_7c80u64, worker_index, () -> u32) as usize;
    let val64 = rd::<u64>(rbx0 + (w2 << 6) + 0xc);
    let val_hi = (val64 >> 32) as u32;
    if allow_steal == 0 || (val64 as u8) == 0 {
        return 0;
    }

    let rbx1 = rd::<usize>(sched);
    let w3 = exe::call_native!(0x1_4240_7c80u64, worker_index, () -> u32) as usize;
    let mut edx = rd::<u32>(r13 + 0x40);
    let r12d = rd::<u32>(rbx1 + (w3 << 6) + 0x8);
    if edx == u32::MAX {
        edx = val_hi;
        wr::<u32>(r13 + 0x40, val_hi);
        wr::<u32>(r13 + 0x44, r12d);
    }

    let mut cluster_count = rd::<u32>(exe::ptr::<u32>(CLUSTER_COUNT_VA) as usize);
    let mut ebx: u32 = 0;
    let mut edi = rd::<u32>(r13 + 0x44);
    let mut esi: u32 = 0;

    if cluster_count != 0 {
        let mut cluster_arr = rd::<usize>(exe::ptr::<usize>(CLUSTER_ARRAY_VA) as usize);
        let mut r9 = cluster_arr + (edi as usize) * 12;
        loop {
            let mut r11d: u32 = 0;
            if rd::<u32>(r9) > 0 {
                loop {
                    if edx != rbp {
                        let r10 = rd::<usize>(sched + ((edx as usize) << 6) + 0x48) + q_offset;
                        let h = load_acq_i32(r10 + 0x20);
                        fence(Ordering::SeqCst);
                        let t = load_acq_i32(r10 + 0x24);
                        if t.wrapping_sub(h) > 0 {
                            let mask = rd::<u32>(r10 + 0x28) as u64;
                            let buf = rd::<usize>(r10 + 0x30);
                            let idx = (mask & (h as u32 as u64)) as usize;
                            let job = load_acq_usize(buf + idx * 8);
                            if job != 0 && atomic_cmpxchg_i32(r10 + 0x20, h, h.wrapping_add(1)) {
                                wr::<u32>(r13 + 0x40, edx);
                                wr::<u32>(r13 + 0x44, edi);
                                let tag_prio = (load_acq_u16(job + 6) as usize) & 7;
                                let _ = atomic_xadd_i32(sched + 0x1040 + tag_prio * 4, -1);
                                return job;
                            }
                        }
                        if edx >= ebx {
                            ebx = edx;
                        }
                    }
                    edx = edx.wrapping_add(1);
                    if edx > rd::<u32>(r9 + 8) {
                        edx = rd::<u32>(r9 + 4);
                    }
                    r11d = r11d.wrapping_add(1);
                    if r11d >= rd::<u32>(r9) {
                        break;
                    }
                }
                cluster_count = rd::<u32>(exe::ptr::<u32>(CLUSTER_COUNT_VA) as usize);
                cluster_arr = rd::<usize>(exe::ptr::<usize>(CLUSTER_ARRAY_VA) as usize);
            }
            let next_edi = edi.wrapping_add(1);
            edi = if next_edi < cluster_count { next_edi } else { 0 };
            esi = esi.wrapping_add(1);
            r9 = cluster_arr + (edi as usize) * 12;
            edx = rd::<u32>(r9 + 4);
            if esi >= cluster_count {
                break;
            }
        }
    }

    ebx = ebx.wrapping_add(1);
    if ebx < 0x40 {
        let mut r9 = sched + ((ebx as usize) << 6) + 0x48;
        while ebx < 0x40 {
            let r8_peer = rd::<usize>(r9) + q_offset;
            let h = load_acq_i32(r8_peer + 0x20);
            fence(Ordering::SeqCst);
            let t = load_acq_i32(r8_peer + 0x24);
            if t.wrapping_sub(h) > 0 {
                let mask = rd::<u32>(r8_peer + 0x28) as u64;
                let buf = rd::<usize>(r8_peer + 0x30);
                let idx = (mask & (h as u32 as u64)) as usize;
                let job = load_acq_usize(buf + idx * 8);
                if job != 0 && atomic_cmpxchg_i32(r8_peer + 0x20, h, h.wrapping_add(1)) {
                    let tag_prio = (load_acq_u16(job + 6) as usize) & 7;
                    let _ = atomic_xadd_i32(sched + 0x1040 + tag_prio * 4, -1);
                    return job;
                }
            }
            ebx = ebx.wrapping_add(1);
            r9 += 0x40;
        }
    }

    wr::<u32>(r13 + 0x40, val_hi);
    wr::<u32>(r13 + 0x44, r12d);
    0
}

/// Pop a job from `sched` (`rcx`) according to the current worker's priority policy,
/// forwarding `allow_steal` (`dl`) to `scheduler_pop_job` (`0x142485c10`).
///
/// Original: 0x142486380 (207 bytes).
pub unsafe extern "win64" fn scheduler_pop_any_job(sched: usize, allow_steal: u8) -> usize {
    let w = exe::call_native!(0x1_4240_7c80u64, worker_index, () -> u32) as usize;
    let table = exe::ptr::<usize>(WORKER_DESC_TABLE_VA) as usize;
    let desc = rd::<usize>(table + w * 8);
    if desc != 0 {
        let mode = rd::<u32>(desc);
        if mode == 2 {
            let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 0u16, allow_steal) -> usize);
            return if job != 0 { job } else { 0 };
        }
        if mode == 1 {
            let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 4u16, allow_steal) -> usize);
            if job != 0 {
                return job;
            }
            let job2 = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 2u16, allow_steal) -> usize);
            return if job2 != 0 { job2 } else { 0 };
        }
    }
    for prio in 0u16..4 {
        let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, prio, allow_steal) -> usize);
        if job != 0 {
            return job;
        }
    }
    0
}

/// Pop a job from `sched` (`rcx`) using the default per-priority steal policy
/// (mode 2 -> prio 0 no steal; mode 1 -> prio 4 then prio 2 with steal;
/// default -> prio 0..4 with `allow_steal = (prio == 2)`).
///
/// Original: 0x142485fe0 (178 bytes).
pub unsafe extern "win64" fn scheduler_pop_default_job(sched: usize) -> usize {
    let w = exe::call_native!(0x1_4240_7c80u64, worker_index, () -> u32) as usize;
    let table = exe::ptr::<usize>(WORKER_DESC_TABLE_VA) as usize;
    let desc = rd::<usize>(table + w * 8);
    if desc != 0 {
        let mode = rd::<u32>(desc);
        if mode == 2 {
            let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 0u16, 0u8) -> usize);
            return if job != 0 { job } else { 0 };
        }
        if mode == 1 {
            let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 4u16, 1u8) -> usize);
            if job != 0 {
                return job;
            }
            let job2 = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 2u16, 1u8) -> usize);
            return if job2 != 0 { job2 } else { 0 };
        }
    }
    for prio in 0u16..4 {
        let allow_steal = u8::from(prio == 2);
        let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, prio, allow_steal) -> usize);
        if job != 0 {
            return job;
        }
    }
    0
}

/// Spin up to 1000 iterations trying to pop or steal a job from `sched` (`rcx`).
///
/// Original: 0x142407210 (216 bytes).
pub unsafe extern "win64" fn scheduler_spin_pop_job(sched: usize) -> usize {
    let table = exe::ptr::<usize>(WORKER_DESC_TABLE_VA) as usize;
    for _ in 0..1000 {
        let w = exe::call_native!(0x1_4240_7c80u64, worker_index, () -> u32) as usize;
        let desc = rd::<usize>(table + w * 8);
        if desc != 0 {
            let mode = rd::<u32>(desc);
            if mode == 2 {
                let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 0u16, 1u8) -> usize);
                if job != 0 {
                    return job;
                }
                continue;
            }
            if mode == 1 {
                let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 4u16, 1u8) -> usize);
                if job != 0 {
                    return job;
                }
                let job2 = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 2u16, 1u8) -> usize);
                if job2 != 0 {
                    return job2;
                }
                continue;
            }
        }
        for prio in 0u16..4 {
            let job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, prio, 1u8) -> usize);
            if job != 0 {
                return job;
            }
        }
    }
    0
}

/// Execute a job inline when the worker ring buffer is full, then walk its parent chain
/// (`node + 8`) and inline-execute any parent whose refcount drops from 2 to 1.
///
/// Original: 0x142485f10 (201 bytes).
pub unsafe extern "win64" fn scheduler_run_job_inline(_sched: usize, mut node: usize) {
    let vtable = rd::<usize>(node + 0x10);
    let run_fn = rd::<usize>(vtable + 0x8);
    exe::call_va!(run_fn as u64, (node + 0x10));
    let vtable2 = rd::<usize>(node + 0x10);
    let dtor_fn = rd::<usize>(vtable2);
    exe::call_va!(dtor_fn as u64, (node + 0x10, 0u32));
    let _ = atomic_xadd_i32(node, -1);
    let mut parent = rd::<usize>(node + 0x8);
    let _ = atomic_cmpxchg_u8(node + 0x4, 1, 0);
    bump_job_tag(node);

    while parent != 0 {
        let old_ref = atomic_xadd_i32(parent, -1);
        if old_ref != 2 {
            break;
        }
        node = parent;
        let vt = rd::<usize>(node + 0x10);
        let rfn = rd::<usize>(vt + 0x8);
        exe::call_va!(rfn as u64, (node + 0x10));
        let vt2 = rd::<usize>(node + 0x10);
        let dfn = rd::<usize>(vt2);
        exe::call_va!(dfn as u64, (node + 0x10, 0u32));
        let _ = atomic_xadd_i32(node, -1);
        parent = rd::<usize>(node + 0x8);
        let _ = atomic_cmpxchg_u8(node + 0x4, 1, 0);
        bump_job_tag(node);
    }
}

/// Enqueue `node` (`rdx`) into the current worker's ring buffer in `sched` (`rcx`),
/// or run it inline via `0x142485f10` if the ring buffer is full.
///
/// Original: 0x142486460 (188 bytes).
pub unsafe extern "win64" fn scheduler_enqueue_job(sched: usize, node: usize) {
    if !atomic_cmpxchg_u8(node + 0x4, 1, 2) {
        return;
    }
    let prio = (load_acq_u16(node + 0x6) as usize) & 7;
    let _ = atomic_xadd_i32(sched + 0x1040 + prio * 4, 1);
    let w = exe::call_native!(0x1_4240_7c80u64, worker_index, () -> u32) as usize;
    let q_prio = (load_acq_u8(node + 0x6) as usize) & 7;
    let r8 = rd::<usize>(sched + (w << 6) + 0x48) + (q_prio << 6);
    let head = load_acq_i32(r8 + 0x20);
    let tail = load_acq_i32(r8 + 0x24);
    let used = (tail.wrapping_sub(head) as i64) as u64;
    let cap = rd::<u32>(r8 + 0x38) as u64;
    if used < cap {
        let mask = rd::<u32>(r8 + 0x28) as u64;
        let buf = rd::<usize>(r8 + 0x30);
        let idx = (mask & (tail as u32 as u64)) as usize;
        store_rel_usize(buf + idx * 8, node);
        fence(Ordering::SeqCst);
        store_rel_i32(r8 + 0x24, tail.wrapping_add(1));
        fence(Ordering::SeqCst);
    } else {
        let p2 = (load_acq_u16(node + 0x6) as usize) & 7;
        let _ = atomic_xadd_i32(sched + 0x1040 + p2 * 4, -1);
        let _ = atomic_cmpxchg_u8(node + 0x4, 2, 1);
        exe::call_va!(0x1_4248_5f10u64, (sched, node));
    }
}

/// Complete a cancelled/skipped job node (transitioning state `4 -> 2 -> 0`, decrementing
/// its refcount, bumping its tag, and if its parent refcount drops from 2 to 1, enqueueing
/// the parent and waking one worker semaphore).
///
/// Original: 0x142486230 (132 bytes).
pub unsafe extern "win64" fn scheduler_cancel_job(sched: usize, node: usize) {
    if !atomic_cmpxchg_u8(node + 0x4, 4, 2) {
        return;
    }
    let _ = atomic_xadd_i32(node, -1);
    let parent = load_acq_usize(node + 0x8);
    let _ = atomic_cmpxchg_u8(node + 0x4, 2, 0);
    bump_job_tag(node);
    if parent != 0 {
        let old_ref = atomic_xadd_i32(parent, -1);
        if old_ref == 2 {
            exe::call_va!(0x1_4248_6460u64, (sched, parent));
            let mgr = rd::<usize>(sched);
            let sem = rd::<usize>(mgr + 0x1008);
            let _ = exe::call_iat!(IAT_RELEASE_SEMAPHORE, (sem, 1i32, 0usize) -> i32);
        }
    }
}

/// Execute a popped job `node` (`rdx`) on `sched` (`rcx`), or re-enqueue it if its
/// dependency count (`[node]`) is still `> 1`.
///
/// Original: 0x1424862c0 (171 bytes).
pub unsafe extern "win64" fn scheduler_run_job(sched: usize, node: usize) {
    if load_acq_u32(node) == 1 {
        let vt = rd::<usize>(node + 0x10);
        let run_fn = rd::<usize>(vt + 0x8);
        let jp = crate::jobprof::begin();
        exe::call_va!(run_fn as u64, (node + 0x10));
        crate::jobprof::end(jp, crate::jobprof::K_JOB, run_fn as u64, 0, 0);
        let vt2 = rd::<usize>(node + 0x10);
        let dtor_fn = rd::<usize>(vt2);
        exe::call_va!(dtor_fn as u64, (node + 0x10, 0u32));
        let _ = atomic_xadd_i32(node, -1);
        let parent = load_acq_usize(node + 0x8);
        let _ = atomic_cmpxchg_u8(node + 0x4, 2, 0);
        bump_job_tag(node);
        if parent != 0 {
            let old_ref = atomic_xadd_i32(parent, -1);
            if old_ref == 2 {
                exe::call_va!(0x1_4248_6460u64, (sched, parent));
                let mgr = rd::<usize>(sched);
                let sem = rd::<usize>(mgr + 0x1008);
                let _ = exe::call_iat!(IAT_RELEASE_SEMAPHORE, (sem, 1i32, 0usize) -> i32);
            }
        }
    } else if load_acq_u32(node) > 1 {
        exe::call_va!(0x1_4248_6460u64, (sched, node));
        let mgr = rd::<usize>(sched);
        let sem = rd::<usize>(mgr + 0x1008);
        let _ = exe::call_iat!(IAT_RELEASE_SEMAPHORE, (sem, 1i32, 0usize) -> i32);
    }
}

/// Run `job` (`rcx`) on `sched` (`rdx`) and drain all subsequent runnable jobs from `sched`.
///
/// Original: 0x1424072f0 (48 bytes).
pub unsafe extern "win64" fn scheduler_run_and_drain_jobs( mut job: usize, sched: usize) {
    if job == 0 {
        return;
    }
    loop {
        exe::call_va!(0x1_4248_62c0u64, (sched, job));
        job = exe::call_native!(0x1_4248_6380u64, scheduler_pop_any_job, (sched, 1u8) -> usize);
        if job == 0 {
            break;
        }
    }
}

/// Pop one job from `[this + 0x10]` via `0x142486380` (forwarding `allow_steal` in `dl`)
/// and if non-null execute it via `0x1424862c0`, returning the popped job pointer.
///
/// Original: 0x1424854b0 (56 bytes).
pub unsafe extern "win64" fn scheduler_pop_and_run_one(this: usize, allow_steal: u8) -> usize {
    let sched = rd::<usize>(this + 0x10);
    let job = exe::call_native!(0x1_4248_6380u64, scheduler_pop_any_job, (sched, allow_steal) -> usize);
    if job != 0 {
        let sched2 = rd::<usize>(this + 0x10);
        exe::call_va!(0x1_4248_62c0u64, (sched2, job));
    }
    job
}

/// Pop one job from `[this + 0x10]` using the default per-priority steal policy and
/// execute it via `0x1424862c0`, returning the popped job pointer (or 0 if none).
///
/// Original: 0x1424854f0 (241 bytes).
pub unsafe extern "win64" fn scheduler_pump_one(this: usize) -> usize {
    let sched = rd::<usize>(this + 0x10);
    let w = exe::call_native!(0x1_4240_7c80u64, worker_index, () -> u32) as usize;
    let table = exe::ptr::<usize>(WORKER_DESC_TABLE_VA) as usize;
    let desc = rd::<usize>(table + w * 8);
    let mut job = 0usize;
    if desc != 0 {
        let mode = rd::<u32>(desc);
        if mode == 2 {
            job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 0u16, 0u8) -> usize);
            if job == 0 {
                return 0;
            }
        } else if mode == 1 {
            job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 4u16, 1u8) -> usize);
            if job == 0 {
                job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, 2u16, 1u8) -> usize);
                if job == 0 {
                    return 0;
                }
            }
        } else {
            for prio in 0u16..4 {
                let allow_steal = u8::from(prio == 2);
                job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, prio, allow_steal) -> usize);
                if job != 0 {
                    break;
                }
            }
            if job == 0 {
                return 0;
            }
        }
    } else {
        for prio in 0u16..4 {
            let allow_steal = u8::from(prio == 2);
            job = exe::call_native!(0x1_4248_5c10u64, scheduler_pop_job, (sched, prio, allow_steal) -> usize);
            if job != 0 {
                break;
            }
        }
        if job == 0 {
            return 0;
        }
    }
    let sched2 = rd::<usize>(this + 0x10);
    exe::call_va!(0x1_4248_62c0u64, (sched2, job));
    job
}

/// Enqueue `node` (`rdx`) on `sched` (`rcx`) and release 1 count on the worker semaphore.
///
/// Original: 0x1424860a0 (43 bytes).
pub unsafe extern "win64" fn scheduler_enqueue_and_wake(sched: usize, node: usize) -> i32 {
    exe::call_va!(0x1_4248_6460u64, (sched, node));
    let mgr = rd::<usize>(sched);
    let sem = rd::<usize>(mgr + 0x1008);
    exe::call_iat!(IAT_RELEASE_SEMAPHORE, (sem, 1i32, 0usize) -> i32)
}

/// If `handle->node` (`[rdx]`) has refcount `== 1`, enqueue it on `[this + 0x10]` and
/// release 1 count on the worker semaphore.
///
/// Original: 0x142485930 (75 bytes).
pub unsafe extern "win64" fn scheduler_enqueue_if_ready(this: usize, handle: usize) {
    let node = rd::<usize>(handle);
    if rd::<u32>(node) == 1 {
        let node2 = rd::<usize>(handle);
        let sched = rd::<usize>(this + 0x10);
        exe::call_va!(0x1_4248_6460u64, (sched, node2));
        let mgr = rd::<usize>(sched);
        let sem = rd::<usize>(mgr + 0x1008);
        let _ = exe::call_iat!(IAT_RELEASE_SEMAPHORE, (sem, 1i32, 0usize) -> i32);
    }
}

/// Batch-enqueue all ready (`refcount == 1`) job handles in `{ count: u64, data: *const JobHandle }`
/// (`rdx`, stride 16 bytes) onto `[this + 0x10]` and release the worker semaphore by the enqueued count.
///
/// Original: 0x1424855f0 (134 bytes).
pub unsafe extern "win64" fn scheduler_enqueue_batch(this: usize, batch: usize) {
    let mut cur = rd::<usize>(batch + 0x8);
    let count = rd::<usize>(batch);
    let end = cur.wrapping_add(count << 4);
    if cur == end {
        return;
    }
    let mut enqueued: usize = 0;
    while cur != end {
        let node = rd::<usize>(cur);
        if rd::<u32>(node) == 1 {
            let node2 = rd::<usize>(cur);
            let sched = rd::<usize>(this + 0x10);
            exe::call_va!(0x1_4248_6460u64, (sched, node2));
            enqueued = enqueued.wrapping_add(1);
        }
        cur = cur.wrapping_add(0x10);
    }
    if enqueued != 0 {
        let sched = rd::<usize>(this + 0x10);
        let mgr = rd::<usize>(sched);
        let sem = rd::<usize>(mgr + 0x1008);
        let _ = exe::call_iat!(IAT_RELEASE_SEMAPHORE, (sem, enqueued as i32, 0usize) -> i32);
    }
}

/// Global job-system instance pointer (`[0x143a082f8]`).
const GLOBAL_JOB_SYSTEM_VA: u64 = 0x1_43a0_82f8;
/// IAT slot of `kernel32!SwitchToThread`.
const IAT_SWITCH_TO_THREAD: u64 = 0x1_4264_5868;
/// IAT slot of `kernel32!WaitForSingleObject`.
const IAT_WAIT_FOR_SINGLE_OBJECT: u64 = 0x1_4264_5748;

/// Allocate a 128-byte job node slot from the per-worker ring pool `pool` (`rcx`) and
/// initialize the job handle at `out_handle` (`rdx`), returning `out_handle`.
///
/// Original: 0x142485aa0 (214 bytes).
pub unsafe extern "win64" fn job_node_alloc(pool: usize, out_handle: usize) -> usize {
    let idx0 = rd::<u32>(pool + 0x10);
    let next0 = idx0.wrapping_add(1) & rd::<u32>(pool + 0x14);
    wr::<u32>(pool + 0x10, next0);
    let mut node = rd::<usize>(pool).wrapping_add((idx0 as usize) << 7);

    if load_acq_u32(node) != 0 {
        while load_acq_u32(node) != 0 {
            let sys = rd::<usize>(exe::ptr::<usize>(GLOBAL_JOB_SYSTEM_VA) as usize);
            let sched = rd::<usize>(sys + 0x10);
            let job = exe::call_native!(0x1_4248_6380u64, scheduler_pop_any_job, (sched, 0u8) -> usize);
            if job == 0 {
                break;
            }
            let sched2 = rd::<usize>(sys + 0x10);
            exe::call_va!(0x1_4248_62c0u64, (sched2, job));
        }
    }

    if !atomic_cmpxchg_u8(node + 0x4, 0, 1) {
        let mut cur_idx = rd::<u32>(pool + 0x10);
        let mut next_idx = cur_idx.wrapping_add(1);
        loop {
            next_idx &= rd::<u32>(pool + 0x14);
            wr::<u32>(pool + 0x10, next_idx);
            node = rd::<usize>(pool).wrapping_add((cur_idx as usize) << 7);
            if atomic_cmpxchg_u8(node + 0x4, 0, 1) {
                break;
            }
            let _ = load_acq_u8(node + 0x4);
            cur_idx = rd::<u32>(pool + 0x10);
            next_idx = cur_idx.wrapping_add(1);
        }
    }

    let tag = load_acq_u16(node + 0x6) >> 3;
    wr::<u16>(out_handle + 0x8, tag);
    wr::<usize>(out_handle, node);
    wr::<u16>(out_handle + 0xa, 0);
    wr::<u8>(out_handle + 0xc, 0);
    out_handle
}

#[inline(always)]
unsafe fn mark_node_cancelling(node: usize) {
    while load_acq_u8(node + 0x4) == 3 {
        if atomic_cmpxchg_u8(node + 0x4, 3, 4) {
            let _ = atomic_xadd_i32(node, -0xffff); // 0xffff0001 as i32
        }
    }
}

#[inline(always)]
unsafe fn handle_is_active(handle: usize) -> bool {
    let node = load_acq_usize(handle);
    if node == 0 {
        return false;
    }
    let tag = load_acq_u16(node + 0x6) >> 3;
    if load_acq_u16(handle + 0x8) != tag {
        return false;
    }
    load_acq_u32(node) != 0
}

/// Mark `handle->node` (`[rdx]`) for cancellation if in state 3 and pump jobs via
/// `0x1424854f0` until `handle` completes.
///
/// Original: 0x142484bc0 (150 bytes).
pub unsafe extern "win64" fn scheduler_wait_cancel_default(this: usize, handle: usize) {
    let node0 = load_acq_usize(handle);
    mark_node_cancelling(node0);
    while handle_is_active(handle) {
        let node = load_acq_usize(handle);
        if load_acq_u32(node) == 1 {
            let sched = rd::<usize>(this + 0x10);
            exe::call_va!(0x1_4248_6230u64, (sched, node));
        } else {
            let job = exe::call_va!(0x1_4248_54f0u64, (this) -> usize);
            if job == 0 {
                let _ = exe::call_iat!(IAT_SWITCH_TO_THREAD, () -> i32);
            }
        }
    }
}

/// Pump jobs via `0x1424854f0` until `handle` (`rdx`) completes.
///
/// Original: 0x142484c60 (80 bytes).
pub unsafe extern "win64" fn scheduler_wait_default(this: usize, handle: usize) {
    while handle_is_active(handle) {
        let job = exe::call_va!(0x1_4248_54f0u64, (this) -> usize);
        if job == 0 {
            let _ = exe::call_iat!(IAT_SWITCH_TO_THREAD, () -> i32);
        }
    }
}

/// Mark `handle->node` (`[rdx]`) for cancellation if in state 3 and pump jobs via
/// `0x142486380`/`0x1424862c0` until `handle` completes.
///
/// Original: 0x142484cc0 (163 bytes).
pub unsafe extern "win64" fn scheduler_wait_cancel_any(this: usize, handle: usize) {
    let node0 = load_acq_usize(handle);
    mark_node_cancelling(node0);
    while handle_is_active(handle) {
        let node = load_acq_usize(handle);
        let sched = rd::<usize>(this + 0x10);
        if load_acq_u32(node) == 1 {
            exe::call_va!(0x1_4248_6230u64, (sched, node));
        } else {
            let job = exe::call_native!(0x1_4248_6380u64, scheduler_pop_any_job, (sched, 1u8) -> usize);
            if job != 0 {
                let sched2 = rd::<usize>(this + 0x10);
                exe::call_va!(0x1_4248_62c0u64, (sched2, job));
            } else {
                let _ = exe::call_iat!(IAT_SWITCH_TO_THREAD, () -> i32);
            }
        }
    }
}

/// Pump jobs via `0x142486380`/`0x1424862c0` until `handle` (`rdx`) completes.
///
/// Original: 0x142484d70 (97 bytes).
pub unsafe extern "win64" fn scheduler_wait_any(this: usize, handle: usize) {
    while handle_is_active(handle) {
        let sched = rd::<usize>(this + 0x10);
        let job = exe::call_native!(0x1_4248_6380u64, scheduler_pop_any_job, (sched, 1u8) -> usize);
        if job != 0 {
            let sched2 = rd::<usize>(this + 0x10);
            exe::call_va!(0x1_4248_62c0u64, (sched2, job));
        } else {
            let _ = exe::call_iat!(IAT_SWITCH_TO_THREAD, () -> i32);
        }
    }
}

/// Wait on `handle` (`rdx`) using `0x1424854f0` if `handle+0xb == 0`, setting
/// `handle+0xb = 1` on entry and `handle+0xc = 1` on completion.
///
/// Original: 0x142485680 (241 bytes).
pub unsafe extern "win64" fn scheduler_wait_handle_default(this: usize, handle: usize) {
    if rd::<u8>(handle + 0xb) != 0 {
        return;
    }
    let can_cancel = rd::<u8>(handle + 0xa) == 1;
    wr::<u8>(handle + 0xb, 1);
    let jp = crate::jobprof::begin();
    if can_cancel {
        scheduler_wait_cancel_default(this, handle);
    } else {
        scheduler_wait_default(this, handle);
    }
    crate::jobprof::end(jp, crate::jobprof::K_WAITH, handle as u64, 0, 0);
    wr::<u8>(handle + 0xc, 1);
}

/// Wait on `handle` (`rdx`) using `0x142486380`/`0x1424862c0` if `handle+0xb == 0`,
/// setting `handle+0xb = 1` on entry and `handle+0xc = 1` on completion.
///
/// Original: 0x142485780 (278 bytes).
pub unsafe extern "win64" fn scheduler_wait_handle_any(this: usize, handle: usize) {
    if rd::<u8>(handle + 0xb) != 0 {
        return;
    }
    let can_cancel = rd::<u8>(handle + 0xa) == 1;
    wr::<u8>(handle + 0xb, 1);
    let jp = crate::jobprof::begin();
    if can_cancel {
        scheduler_wait_cancel_any(this, handle);
    } else {
        scheduler_wait_any(this, handle);
    }
    crate::jobprof::end(jp, crate::jobprof::K_WAITH, handle as u64, 1, 0);
    wr::<u8>(handle + 0xc, 1);
}

/// Main worker thread loop (`r::ThreadManager::Worker::vfunc_1`).
///
/// Original: 0x1424070e0 (289 bytes).
pub unsafe extern "win64" fn worker_thread_main(worker: usize) -> u32 {
    let r8d = rd::<u32>(worker + 0x134);
    let mgr0 = rd::<usize>(worker + 0x120);
    exe::call_va!(0x1_4240_6a80u64, (mgr0, 4u32, r8d));
    let _ = AtomicU8::from_ptr((worker + 0x131) as *mut u8).swap(1, Ordering::SeqCst);

    if rd::<u8>(worker + 0x130) != 0 {
        loop {
            let sched0 = rd::<usize>(worker + 0x128);
            let mut job = exe::call_native!(0x1_4248_6380u64, scheduler_pop_any_job, (sched0, 1u8) -> usize);
            if job != 0 {
                'drain: loop {
                    let sched = rd::<usize>(worker + 0x128);
                    loop {
                        exe::call_va!(0x1_4248_62c0u64, (sched, job));
                        job = exe::call_native!(0x1_4248_6380u64, scheduler_pop_any_job, (sched, 1u8) -> usize);
                        if job == 0 {
                            break;
                        }
                    }
                    let sched_spin = rd::<usize>(worker + 0x128);
                    for _ in 0..1000 {
                        job = exe::call_native!(0x1_4248_6380u64, scheduler_pop_any_job, (sched_spin, 1u8) -> usize);
                        if job != 0 {
                            continue 'drain;
                        }
                    }
                    break 'drain;
                }
            }

            let mgr = rd::<usize>(worker + 0x120);
            let sem = rd::<usize>(mgr + 0x1008);
            let mut tries: i32 = 1;
            loop {
                let prev = tries;
                tries -= 1;
                if prev <= 0 {
                    break;
                }
                let res = exe::call_iat!(IAT_WAIT_FOR_SINGLE_OBJECT, (sem, u32::MAX) -> i32);
                if res == -1 {
                    break;
                }
            }

            let scratch = exe::call_va!(0x1_4241_bbe0u64, () -> usize);
            if rd::<usize>(scratch + 0x10) != 0 {
                exe::call_va!(0x1_424b_3fa0u64, (scratch + 0x8));
            }

            if rd::<u8>(worker + 0x130) == 0 {
                break;
            }
        }
    }

    let _ = AtomicU8::from_ptr((worker + 0x131) as *mut u8).swap(0, Ordering::SeqCst);
    0
}

// ---------------------------------------------------------------------------
// Differential tests for timing & scheduler functions
// ---------------------------------------------------------------------------

pub unsafe fn diff_perf_counter(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> i64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut cases = 0;
    for _ in 0..4 {
        let t1 = o();
        let t2 = n();
        let t3 = o();
        if !(t1 <= t2 && t2 <= t3 && t1 > 0) {
            return Err(format!("non-monotonic QPC ticks: {t1}, {t2}, {t3}"));
        }
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_perf_elapsed_seconds(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> f64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let tls0 = adopt_thread()?;
    let guard = exe::ptr::<i32>(START_QPC_GUARD_VA) as usize;
    let start_va = exe::ptr::<i64>(START_QPC_VA) as usize;
    let (g0, s0, e0) = (rd::<i32>(guard), rd::<i64>(start_va), rd::<i32>(tls0 + 0x74));
    let mut cases = 0;

    // Slow path initialization test
    wr::<i32>(guard, 0);
    wr::<i64>(start_va, 0);
    wr::<i32>(tls0 + 0x74, i32::MIN);
    let a = o();
    let (ga, sa) = (rd::<i32>(guard), rd::<i64>(start_va));

    wr::<i32>(guard, 0);
    wr::<i64>(start_va, 0);
    wr::<i32>(tls0 + 0x74, i32::MIN);
    let b = n();
    let (gb, sb) = (rd::<i32>(guard), rd::<i64>(start_va));

    crate::diff_eq!(ga > 0, gb > 0, "slow path guard");
    crate::diff_eq!(sa > 0, sb > 0, "slow path start qpc");
    if !(a > -0.1 && a <= 0.0 && b > -0.1 && b <= 0.0) {
        return Err(format!("unexpected slow path elapsed: a={a}, b={b}"));
    }
    cases += 1;

    // Fast path monotonic check with fixed baseline
    let mut base: i64 = 0;
    let _ = exe::call_iat!(IAT_QPC, (&mut base as *mut i64) -> i32);
    wr::<i64>(start_va, base - 10_000_000);
    let e1 = o();
    let e2 = n();
    let e3 = o();
    if !(e1 <= e2 && e2 <= e3 && e1 > 0.0) {
        return Err(format!("fast path non-monotonic elapsed: {e1}, {e2}, {e3}"));
    }
    cases += 1;

    wr::<i32>(guard, g0);
    wr::<i64>(start_va, s0);
    wr::<i32>(tls0 + 0x74, e0);
    Ok(cases)
}

pub unsafe fn diff_job_node_unlock_and_bump_tag(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut buf_a = [0u8; 32];
    let mut buf_b = [0u8; 32];
    let mut cases = 0;
    for offset in [0usize, 1, 4] {
        for state in [0u8, 1, 2, 3, 4] {
            for expected in [0u8, 1, 2, 4] {
                for tag in [0u16, 1, 7, 0x0008, 0x0038, 0xfff8, 0xffff, 0x1234] {
                    buf_a.fill(0xaa);
                    buf_b.fill(0xaa);
                    let pa = buf_a.as_mut_ptr() as usize + offset;
                    let pb = buf_b.as_mut_ptr() as usize + offset;
                    wr::<u8>(pa + 4, state);
                    wr::<u16>(pa + 6, tag);
                    wr::<u8>(pb + 4, state);
                    wr::<u16>(pb + 6, tag);
                    o(0xdead_beef, pa, expected);
                    n(0xdead_beef, pb, expected);
                    if buf_a != buf_b {
                        return Err(format!(
                            "mismatch at offset={offset} state={state} expected={expected} tag={tag:#x}"
                        ));
                    }
                    cases += 1;
                }
            }
        }
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_pop_job(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u16, u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let tls0 = adopt_thread()?;
    let (sf, si) = (rd::<u32>(tls0 + 0xa86b0), rd::<u32>(tls0 + 0xa86b4));
    // Pin worker_index() to 0 on fast path.
    wr::<u32>(tls0 + 0xa86b0, 1);
    wr::<u32>(tls0 + 0xa86b4, 0);

    let cluster_cnt_ptr = exe::ptr::<u32>(CLUSTER_COUNT_VA) as usize;
    let cluster_arr_ptr = exe::ptr::<usize>(CLUSTER_ARRAY_VA) as usize;
    let saved_cluster_cnt = rd::<u32>(cluster_cnt_ptr);
    let saved_cluster_arr = rd::<usize>(cluster_arr_ptr);

    let mut sched_a = vec![0u8; 0x1100];
    let mut sched_b = vec![0u8; 0x1100];
    let mut queues_a = vec![0u8; 0x400];
    let mut queues_b = vec![0u8; 0x400];
    let mut ring_a = [0usize; 8];
    let mut ring_b = [0usize; 8];
    let mut job_a = [0u8; 32];
    let mut job_b = [0u8; 32];
    let mut mgr_a = vec![0u8; 0x100];
    let mut mgr_b = vec![0u8; 0x100];

    let mut cases = 0;

    // Case 1: ready_counts[prio] == 0 -> immediate 0
    {
        sched_a.fill(0);
        sched_b.fill(0);
        let ra = o(sched_a.as_mut_ptr() as usize, 2, 1);
        let rb = n(sched_b.as_mut_ptr() as usize, 2, 1);
        crate::diff_eq!(ra, rb, "empty count ret");
        crate::diff_eq!(sched_a, sched_b, "empty count sched");
        cases += 1;
    }

    // Case 2: local queue has 2 items (new_tail != head) and 1 item (new_tail == head)
    for (head, tail) in [(0i32, 2i32), (3i32, 4i32), (5i32, 5i32), (5i32, 4i32)] {
        sched_a.fill(0);
        sched_b.fill(0);
        queues_a.fill(0);
        queues_b.fill(0);
        ring_a.fill(0);
        ring_b.fill(0);
        job_a.fill(0);
        job_b.fill(0);
        mgr_a.fill(0);
        mgr_b.fill(0);

        let sa = sched_a.as_mut_ptr() as usize;
        let sb = sched_b.as_mut_ptr() as usize;
        let qa = queues_a.as_mut_ptr() as usize;
        let qb = queues_b.as_mut_ptr() as usize;
        let ja = job_a.as_mut_ptr() as usize;
        let jb = job_b.as_mut_ptr() as usize;

        wr::<u16>(ja + 6, 2); // prio 2 in low 3 bits
        wr::<u16>(jb + 6, 2);
        for slot in 0..8 {
            ring_a[slot] = ja;
            ring_b[slot] = jb;
        }

        // Set up worker 0 queue for prio 2 (`qa + 2 * 0x40`)
        for (s, q, r, m) in [
            (sa, qa, ring_a.as_mut_ptr() as usize, mgr_a.as_mut_ptr() as usize),
            (sb, qb, ring_b.as_mut_ptr() as usize, mgr_b.as_mut_ptr() as usize),
        ] {
            wr::<usize>(s, m);
            wr::<u32>(s + 0x1040 + 2 * 4, 5);
            wr::<usize>(s + 0x48, q);
            let q2 = q + 2 * 0x40;
            wr::<i32>(q2 + 0x20, head);
            wr::<i32>(q2 + 0x24, tail);
            wr::<u32>(q2 + 0x28, 7); // mask
            wr::<usize>(q2 + 0x30, r);
        }

        let ra = o(sa, 2, 0);
        let rb = n(sb, 2, 0);
        crate::diff_eq!(ra != 0, rb != 0, "local pop non-null (head={head}, tail={tail})");
        if ra != 0 {
            crate::diff_eq!(ra, ja, "orig returned job_a");
            crate::diff_eq!(rb, jb, "new returned job_b");
        }
        crate::diff_eq!(
            rd::<u32>(sa + 0x1040 + 8),
            rd::<u32>(sb + 0x1040 + 8),
            "ready count after pop"
        );
        crate::diff_eq!(
            rd::<i32>(qa + 2 * 0x40 + 0x20),
            rd::<i32>(qb + 2 * 0x40 + 0x20),
            "head after pop"
        );
        crate::diff_eq!(
            rd::<i32>(qa + 2 * 0x40 + 0x24),
            rd::<i32>(qb + 2 * 0x40 + 0x24),
            "tail after pop"
        );
        cases += 1;
    }

    // Restore globals & TLS
    wr::<u32>(cluster_cnt_ptr, saved_cluster_cnt);
    wr::<usize>(cluster_arr_ptr, saved_cluster_arr);
    wr::<u32>(tls0 + 0xa86b0, sf);
    wr::<u32>(tls0 + 0xa86b4, si);
    Ok(cases)
}

unsafe fn run_pop_dispatch_difftest(
    orig: usize,
    new: usize,
    has_steal_arg: bool,
) -> Result<u32, String> {
    let tls0 = adopt_thread()?;
    let (sf, si) = (rd::<u32>(tls0 + 0xa86b0), rd::<u32>(tls0 + 0xa86b4));
    wr::<u32>(tls0 + 0xa86b0, 1);
    wr::<u32>(tls0 + 0xa86b4, 0);

    let table = exe::ptr::<usize>(WORKER_DESC_TABLE_VA) as usize;
    let saved_desc = rd::<usize>(table);

    static mut CALL_LOG: Vec<(usize, u16, u8)> = Vec::new();
    static mut TARGET_PRIO: u16 = 0xffff;

    unsafe extern "win64" fn mock_pop(sched: usize, prio: u16, allow_steal: u8) -> usize {
        unsafe {
            CALL_LOG.push((sched, prio, allow_steal));
            if prio == TARGET_PRIO {
                0xdead_0000 + prio as usize
            } else {
                0
            }
        }
    }

    let _mock = crate::difftest::Mock::new(0x1_4248_5c10, mock_pop as *const ())?;
    let mut desc_val: u32;
    let mut cases = 0;

    for mode_opt in [None, Some(0u32), Some(1u32), Some(2u32), Some(3u32)] {
        match mode_opt {
            None => wr::<usize>(table, 0),
            Some(m) => {
                desc_val = m;
                wr::<usize>(table, &mut desc_val as *mut u32 as usize);
            }
        }
        for hit in [0xffffu16, 0, 1, 2, 3, 4] {
            TARGET_PRIO = hit;
            for steal in [0u8, 1u8] {
                CALL_LOG.clear();
                let ra = if has_steal_arg {
                    let f: unsafe extern "win64" fn(usize, u8) -> usize =
                        std::mem::transmute(orig);
                    f(0x1111_0000, steal)
                } else {
                    let f: unsafe extern "win64" fn(usize) -> usize = std::mem::transmute(orig);
                    f(0x1111_0000)
                };
                let log_a = CALL_LOG.clone();

                CALL_LOG.clear();
                let rb = if has_steal_arg {
                    let f: unsafe extern "win64" fn(usize, u8) -> usize =
                        std::mem::transmute(new);
                    f(0x1111_0000, steal)
                } else {
                    let f: unsafe extern "win64" fn(usize) -> usize = std::mem::transmute(new);
                    f(0x1111_0000)
                };
                let log_b = CALL_LOG.clone();

                crate::diff_eq!(ra, rb, "ret mode={mode_opt:?} hit={hit} steal={steal}");
                crate::diff_eq!(log_a, log_b, "calls mode={mode_opt:?} hit={hit} steal={steal}");
                cases += 1;
            }
        }
    }

    wr::<usize>(table, saved_desc);
    wr::<u32>(tls0 + 0xa86b0, sf);
    wr::<u32>(tls0 + 0xa86b4, si);
    Ok(cases)
}

pub unsafe fn diff_scheduler_pop_any_job(orig: usize, new: usize) -> Result<u32, String> {
    run_pop_dispatch_difftest(orig, new, true)
}

pub unsafe fn diff_scheduler_pop_default_job(orig: usize, new: usize) -> Result<u32, String> {
    run_pop_dispatch_difftest(orig, new, false)
}

pub unsafe fn diff_scheduler_spin_pop_job(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let tls0 = adopt_thread()?;
    let (sf, si) = (rd::<u32>(tls0 + 0xa86b0), rd::<u32>(tls0 + 0xa86b4));
    wr::<u32>(tls0 + 0xa86b0, 1);
    wr::<u32>(tls0 + 0xa86b4, 0);

    let table = exe::ptr::<usize>(WORKER_DESC_TABLE_VA) as usize;
    let saved_desc = rd::<usize>(table);

    static mut CALL_COUNT: u32 = 0;
    static mut HIT_ON_CALL: u32 = 0;

    unsafe extern "win64" fn mock_pop(_sched: usize, prio: u16, allow_steal: u8) -> usize {
        unsafe {
            CALL_COUNT += 1;
            if CALL_COUNT == HIT_ON_CALL {
                0xbeef_0000 + ((prio as usize) << 8) + allow_steal as usize
            } else {
                0
            }
        }
    }

    let _mock = crate::difftest::Mock::new(0x1_4248_5c10, mock_pop as *const ())?;
    let mut desc_val: u32;
    let mut cases = 0;

    for mode_opt in [None, Some(1u32), Some(2u32)] {
        match mode_opt {
            None => wr::<usize>(table, 0),
            Some(m) => {
                desc_val = m;
                wr::<usize>(table, &mut desc_val as *mut u32 as usize);
            }
        }
        for hit_at in [1u32, 2, 5, 50_000] {
            CALL_COUNT = 0;
            HIT_ON_CALL = hit_at;
            let ra = o(0x2222_0000);
            let ca = CALL_COUNT;

            CALL_COUNT = 0;
            HIT_ON_CALL = hit_at;
            let rb = n(0x2222_0000);
            let cb = CALL_COUNT;

            crate::diff_eq!(ra, rb, "spin ret mode={mode_opt:?} hit_at={hit_at}");
            crate::diff_eq!(ca, cb, "spin call count mode={mode_opt:?} hit_at={hit_at}");
            cases += 1;
        }
    }

    wr::<usize>(table, saved_desc);
    wr::<u32>(tls0 + 0xa86b0, sf);
    wr::<u32>(tls0 + 0xa86b4, si);
    Ok(cases)
}

pub unsafe fn diff_scheduler_enqueue_job(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let tls0 = adopt_thread()?;
    let (sf, si) = (rd::<u32>(tls0 + 0xa86b0), rd::<u32>(tls0 + 0xa86b4));
    wr::<u32>(tls0 + 0xa86b0, 1);
    wr::<u32>(tls0 + 0xa86b4, 0);

    static mut INLINE_CALLS: u32 = 0;
    unsafe extern "win64" fn mock_inline(_sched: usize, _node: usize) {
        unsafe {
            INLINE_CALLS += 1;
        }
    }
    let _mock = crate::difftest::Mock::new(0x1_4248_5f10, mock_inline as *const ())?;

    let mut sched_a = vec![0u8; 0x1100];
    let mut sched_b = vec![0u8; 0x1100];
    let mut queues_a = vec![0u8; 0x400];
    let mut queues_b = vec![0u8; 0x400];
    let mut ring_a = [0usize; 8];
    let mut ring_b = [0usize; 8];
    let mut node_a = [0u8; 32];
    let mut node_b = [0u8; 32];

    let mut cases = 0;

    for state in [0u8, 1u8, 2u8] {
        for (head, tail, cap) in [(0i32, 2i32, 8u32), (0i32, 8i32, 8u32)] {
            sched_a.fill(0);
            sched_b.fill(0);
            queues_a.fill(0);
            queues_b.fill(0);
            ring_a.fill(0);
            ring_b.fill(0);
            node_a.fill(0);
            node_b.fill(0);

            let sa = sched_a.as_mut_ptr() as usize;
            let sb = sched_b.as_mut_ptr() as usize;
            let qa = queues_a.as_mut_ptr() as usize;
            let qb = queues_b.as_mut_ptr() as usize;
            let na = node_a.as_mut_ptr() as usize;
            let nb = node_b.as_mut_ptr() as usize;

            wr::<u8>(na + 4, state);
            wr::<u16>(na + 6, 3);
            wr::<u8>(nb + 4, state);
            wr::<u16>(nb + 6, 3);

            for (s, q, r) in [
                (sa, qa, ring_a.as_mut_ptr() as usize),
                (sb, qb, ring_b.as_mut_ptr() as usize),
            ] {
                wr::<usize>(s + 0x48, q);
                let q3 = q + 3 * 0x40;
                wr::<i32>(q3 + 0x20, head);
                wr::<i32>(q3 + 0x24, tail);
                wr::<u32>(q3 + 0x28, 7);
                wr::<usize>(q3 + 0x30, r);
                wr::<u32>(q3 + 0x38, cap);
            }

            INLINE_CALLS = 0;
            o(sa, na);
            let ic_a = INLINE_CALLS;

            INLINE_CALLS = 0;
            n(sb, nb);
            let ic_b = INLINE_CALLS;

            crate::diff_eq!(ic_a, ic_b, "inline calls state={state} tail={tail}");
            crate::diff_eq!(node_a, node_b, "node state={state} tail={tail}");
            crate::diff_eq!(
                rd::<u32>(sa + 0x1040 + 3 * 4),
                rd::<u32>(sb + 0x1040 + 3 * 4),
                "ready count"
            );
            crate::diff_eq!(
                rd::<i32>(qa + 3 * 0x40 + 0x24),
                rd::<i32>(qb + 3 * 0x40 + 0x24),
                "tail"
            );
            crate::diff_eq!(
                ring_a[(tail as usize) & 7] != 0,
                ring_b[(tail as usize) & 7] != 0,
                "ring slot written"
            );
            cases += 1;
        }
    }

    wr::<u32>(tls0 + 0xa86b0, sf);
    wr::<u32>(tls0 + 0xa86b4, si);
    Ok(cases)
}

pub unsafe fn diff_scheduler_run_job_inline(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut VCALL_LOG: Vec<u32> = Vec::new();
    unsafe extern "win64" fn fake_dtor(_this: usize, flag: u32) {
        unsafe {
            VCALL_LOG.push(0x100 + flag);
        }
    }
    unsafe extern "win64" fn fake_run(_this: usize) {
        unsafe {
            VCALL_LOG.push(0x200);
        }
    }
    let vtable: [usize; 2] = [fake_dtor as *const () as usize, fake_run as *const () as usize];

    let mut cases = 0;
    for parent_ref in [0i32, 2i32, 3i32] {
        let mut child_a = [0u8; 32];
        let mut child_b = [0u8; 32];
        let mut parent_a = [0u8; 32];
        let mut parent_b = [0u8; 32];

        for (c, p) in [
            (child_a.as_mut_ptr() as usize, parent_a.as_mut_ptr() as usize),
            (child_b.as_mut_ptr() as usize, parent_b.as_mut_ptr() as usize),
        ] {
            wr::<i32>(c, 1);
            wr::<u8>(c + 4, 1);
            wr::<u16>(c + 6, 0x12);
            wr::<usize>(c + 8, if parent_ref > 0 { p } else { 0 });
            wr::<usize>(c + 0x10, vtable.as_ptr() as usize);

            wr::<i32>(p, parent_ref);
            wr::<u8>(p + 4, 1);
            wr::<u16>(p + 6, 0x23);
            wr::<usize>(p + 8, 0);
            wr::<usize>(p + 0x10, vtable.as_ptr() as usize);
        }

        VCALL_LOG.clear();
        o(0, child_a.as_mut_ptr() as usize);
        let log_a = VCALL_LOG.clone();

        VCALL_LOG.clear();
        n(0, child_b.as_mut_ptr() as usize);
        let log_b = VCALL_LOG.clone();

        // Compare fields (excluding parent pointer at +8 which differs between a and b)
        wr::<usize>(child_a.as_mut_ptr() as usize + 8, 0);
        wr::<usize>(child_b.as_mut_ptr() as usize + 8, 0);
        crate::diff_eq!(log_a, log_b, "vcall log parent_ref={parent_ref}");
        crate::diff_eq!(child_a, child_b, "child state parent_ref={parent_ref}");
        crate::diff_eq!(parent_a, parent_b, "parent state parent_ref={parent_ref}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_cancel_job(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut ENQ_COUNT: u32 = 0;
    unsafe extern "win64" fn mock_enq(_sched: usize, _node: usize) {
        unsafe {
            ENQ_COUNT += 1;
        }
    }
    let _mock = crate::difftest::Mock::new(0x1_4248_6460, mock_enq as *const ())?;

    let mut mgr = vec![0u8; 0x1020];
    let mut sched = [0u8; 16];
    wr::<usize>(sched.as_mut_ptr() as usize, mgr.as_mut_ptr() as usize);

    let mut cases = 0;
    for init_state in [2u8, 4u8] {
        for parent_ref in [0i32, 2i32, 3i32] {
            let mut node_a = [0u8; 32];
            let mut node_b = [0u8; 32];
            let mut parent_a = [0u8; 32];
            let mut parent_b = [0u8; 32];

            for (nd, pr) in [
                (node_a.as_mut_ptr() as usize, parent_a.as_mut_ptr() as usize),
                (node_b.as_mut_ptr() as usize, parent_b.as_mut_ptr() as usize),
            ] {
                wr::<i32>(nd, 1);
                wr::<u8>(nd + 4, init_state);
                wr::<u16>(nd + 6, 0x15);
                wr::<usize>(nd + 8, if parent_ref > 0 { pr } else { 0 });
                wr::<i32>(pr, parent_ref);
            }

            ENQ_COUNT = 0;
            o(sched.as_ptr() as usize, node_a.as_mut_ptr() as usize);
            let ea = ENQ_COUNT;

            ENQ_COUNT = 0;
            n(sched.as_ptr() as usize, node_b.as_mut_ptr() as usize);
            let eb = ENQ_COUNT;

            wr::<usize>(node_a.as_mut_ptr() as usize + 8, 0);
            wr::<usize>(node_b.as_mut_ptr() as usize + 8, 0);
            crate::diff_eq!(ea, eb, "enq count");
            crate::diff_eq!(node_a, node_b, "node");
            crate::diff_eq!(parent_a, parent_b, "parent");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_run_job(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut EVENTS: Vec<u32> = Vec::new();
    unsafe extern "win64" fn fake_dtor(_this: usize, flag: u32) {
        unsafe {
            EVENTS.push(10 + flag);
        }
    }
    unsafe extern "win64" fn fake_run(_this: usize) {
        unsafe {
            EVENTS.push(20);
        }
    }
    unsafe extern "win64" fn mock_enq(_sched: usize, _node: usize) {
        unsafe {
            EVENTS.push(30);
        }
    }
    let _mock = crate::difftest::Mock::new(0x1_4248_6460, mock_enq as *const ())?;
    let vtable: [usize; 2] = [fake_dtor as *const () as usize, fake_run as *const () as usize];

    let mut mgr = vec![0u8; 0x1020];
    let mut sched = [0u8; 16];
    wr::<usize>(sched.as_mut_ptr() as usize, mgr.as_mut_ptr() as usize);

    let mut cases = 0;
    for node_ref in [0u32, 1u32, 2u32] {
        for parent_ref in [0i32, 2i32, 3i32] {
            let mut node_a = [0u8; 32];
            let mut node_b = [0u8; 32];
            let mut parent_a = [0u8; 32];
            let mut parent_b = [0u8; 32];

            for (nd, pr) in [
                (node_a.as_mut_ptr() as usize, parent_a.as_mut_ptr() as usize),
                (node_b.as_mut_ptr() as usize, parent_b.as_mut_ptr() as usize),
            ] {
                wr::<u32>(nd, node_ref);
                wr::<u8>(nd + 4, 2);
                wr::<u16>(nd + 6, 0x12);
                wr::<usize>(nd + 8, if parent_ref > 0 { pr } else { 0 });
                wr::<usize>(nd + 0x10, vtable.as_ptr() as usize);
                wr::<i32>(pr, parent_ref);
            }

            EVENTS.clear();
            o(sched.as_ptr() as usize, node_a.as_mut_ptr() as usize);
            let ev_a = EVENTS.clone();

            EVENTS.clear();
            n(sched.as_ptr() as usize, node_b.as_mut_ptr() as usize);
            let ev_b = EVENTS.clone();

            wr::<usize>(node_a.as_mut_ptr() as usize + 8, 0);
            wr::<usize>(node_b.as_mut_ptr() as usize + 8, 0);
            crate::diff_eq!(ev_a, ev_b, "events node_ref={node_ref} parent_ref={parent_ref}");
            crate::diff_eq!(node_a, node_b, "node");
            crate::diff_eq!(parent_a, parent_b, "parent");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_run_and_drain_jobs(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut RUN_LOG: Vec<(usize, usize)> = Vec::new();
    static mut POP_QUEUE: Vec<usize> = Vec::new();

    unsafe extern "win64" fn mock_run(sched: usize, job: usize) {
        unsafe {
            RUN_LOG.push((sched, job));
        }
    }
    unsafe extern "win64" fn mock_pop(_sched: usize, _steal: u8) -> usize {
        unsafe { POP_QUEUE.pop().unwrap_or(0) }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_62c0, mock_run as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_6380, mock_pop as *const ())?;

    let mut cases = 0;
    for init_job in [0usize, 0x1000] {
        for extras in [vec![], vec![0x3000, 0x2000]] {
            RUN_LOG.clear();
            POP_QUEUE = extras.clone();
            o(init_job, 0x9999);
            let la = RUN_LOG.clone();

            RUN_LOG.clear();
            POP_QUEUE = extras.clone();
            n(init_job, 0x9999);
            let lb = RUN_LOG.clone();

            crate::diff_eq!(la, lb, "drain log");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_pop_and_run_one(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut LOG: Vec<(usize, usize)> = Vec::new();
    static mut NEXT_JOB: usize = 0;

    unsafe extern "win64" fn mock_pop(sched: usize, steal: u8) -> usize {
        unsafe {
            LOG.push((sched, steal as usize));
            NEXT_JOB
        }
    }
    unsafe extern "win64" fn mock_run(sched: usize, job: usize) {
        unsafe {
            LOG.push((sched, job));
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_6380, mock_pop as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_62c0, mock_run as *const ())?;

    let mut obj = [0u8; 32];
    wr::<usize>(obj.as_mut_ptr() as usize + 0x10, 0x5555_0000);
    let mut cases = 0;
    for job in [0usize, 0xabc0] {
        for steal in [0u8, 1u8] {
            NEXT_JOB = job;
            LOG.clear();
            let ra = o(obj.as_ptr() as usize, steal);
            let la = LOG.clone();

            LOG.clear();
            let rb = n(obj.as_ptr() as usize, steal);
            let lb = LOG.clone();

            crate::diff_eq!(ra, rb, "ret");
            crate::diff_eq!(la, lb, "log");
            cases += 1;
        }
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_pump_one(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let tls0 = adopt_thread()?;
    let (sf, si) = (rd::<u32>(tls0 + 0xa86b0), rd::<u32>(tls0 + 0xa86b4));
    wr::<u32>(tls0 + 0xa86b0, 1);
    wr::<u32>(tls0 + 0xa86b4, 0);

    let table = exe::ptr::<usize>(WORKER_DESC_TABLE_VA) as usize;
    let saved_desc = rd::<usize>(table);

    static mut LOG: Vec<(usize, u16, u8)> = Vec::new();
    static mut TARGET_PRIO: u16 = 0xffff;

    unsafe extern "win64" fn mock_pop(sched: usize, prio: u16, steal: u8) -> usize {
        unsafe {
            LOG.push((sched, prio, steal));
            if prio == TARGET_PRIO {
                0x7777_0000 + prio as usize
            } else {
                0
            }
        }
    }
    unsafe extern "win64" fn mock_run(sched: usize, job: usize) {
        unsafe {
            LOG.push((sched, (job & 0xff) as u16, 99));
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_5c10, mock_pop as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_62c0, mock_run as *const ())?;

    let mut obj = [0u8; 32];
    wr::<usize>(obj.as_mut_ptr() as usize + 0x10, 0x8888_0000);
    let mut desc_val: u32;
    let mut cases = 0;

    for mode_opt in [None, Some(1u32), Some(2u32), Some(3u32)] {
        match mode_opt {
            None => wr::<usize>(table, 0),
            Some(m) => {
                desc_val = m;
                wr::<usize>(table, &mut desc_val as *mut u32 as usize);
            }
        }
        for hit in [0xffffu16, 0, 1, 2, 3, 4] {
            TARGET_PRIO = hit;
            LOG.clear();
            let ra = o(obj.as_ptr() as usize);
            let la = LOG.clone();

            LOG.clear();
            let rb = n(obj.as_ptr() as usize);
            let lb = LOG.clone();

            crate::diff_eq!(ra, rb, "pump_one ret mode={mode_opt:?} hit={hit}");
            crate::diff_eq!(la, lb, "pump_one log mode={mode_opt:?} hit={hit}");
            cases += 1;
        }
    }

    wr::<usize>(table, saved_desc);
    wr::<u32>(tls0 + 0xa86b0, sf);
    wr::<u32>(tls0 + 0xa86b4, si);
    Ok(cases)
}

pub unsafe fn diff_scheduler_enqueue_and_wake(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> i32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut ENQ_LOG: Vec<(usize, usize)> = Vec::new();
    unsafe extern "win64" fn mock_enq(sched: usize, node: usize) {
        unsafe {
            ENQ_LOG.push((sched, node));
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4248_6460, mock_enq as *const ())?;

    let mut mgr = vec![0u8; 0x1020];
    let mut sched = [0u8; 16];
    wr::<usize>(sched.as_mut_ptr() as usize, mgr.as_mut_ptr() as usize);

    ENQ_LOG.clear();
    let ra = o(sched.as_ptr() as usize, 0x1234);
    let la = ENQ_LOG.clone();

    ENQ_LOG.clear();
    let rb = n(sched.as_ptr() as usize, 0x1234);
    let lb = ENQ_LOG.clone();

    crate::diff_eq!(ra, rb, "ret");
    crate::diff_eq!(la, lb, "enq log");
    Ok(1)
}

pub unsafe fn diff_scheduler_enqueue_if_ready(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut ENQ_LOG: Vec<(usize, usize)> = Vec::new();
    unsafe extern "win64" fn mock_enq(sched: usize, node: usize) {
        unsafe {
            ENQ_LOG.push((sched, node));
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4248_6460, mock_enq as *const ())?;

    let mut mgr = vec![0u8; 0x1020];
    let mut sched = [0u8; 16];
    wr::<usize>(sched.as_mut_ptr() as usize, mgr.as_mut_ptr() as usize);
    let mut this_obj = [0u8; 32];
    wr::<usize>(this_obj.as_mut_ptr() as usize + 0x10, sched.as_ptr() as usize);

    let mut cases = 0;
    for rc in [0u32, 1u32, 2u32] {
        let mut node = [0u8; 16];
        wr::<u32>(node.as_mut_ptr() as usize, rc);
        let mut handle = [0u8; 16];
        wr::<usize>(handle.as_mut_ptr() as usize, node.as_ptr() as usize);

        ENQ_LOG.clear();
        o(this_obj.as_ptr() as usize, handle.as_ptr() as usize);
        let la = ENQ_LOG.clone();

        ENQ_LOG.clear();
        n(this_obj.as_ptr() as usize, handle.as_ptr() as usize);
        let lb = ENQ_LOG.clone();

        crate::diff_eq!(la, lb, "rc={rc}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_enqueue_batch(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut ENQ_LOG: Vec<(usize, usize)> = Vec::new();
    unsafe extern "win64" fn mock_enq(sched: usize, node: usize) {
        unsafe {
            ENQ_LOG.push((sched, node));
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4248_6460, mock_enq as *const ())?;

    let mut mgr = vec![0u8; 0x1020];
    let mut sched = [0u8; 16];
    wr::<usize>(sched.as_mut_ptr() as usize, mgr.as_mut_ptr() as usize);
    let mut this_obj = [0u8; 32];
    wr::<usize>(this_obj.as_mut_ptr() as usize + 0x10, sched.as_ptr() as usize);

    let mut nodes = [[0u8; 16]; 4];
    wr::<u32>(nodes[0].as_mut_ptr() as usize, 1);
    wr::<u32>(nodes[1].as_mut_ptr() as usize, 2);
    wr::<u32>(nodes[2].as_mut_ptr() as usize, 1);
    wr::<u32>(nodes[3].as_mut_ptr() as usize, 0);

    let mut handles = [0u8; 64];
    for i in 0..4 {
        wr::<usize>(handles.as_mut_ptr() as usize + i * 16, nodes[i].as_ptr() as usize);
    }

    let mut cases = 0;
    for cnt in [0usize, 1, 2, 4] {
        let mut batch = [0u8; 16];
        wr::<usize>(batch.as_mut_ptr() as usize, cnt);
        wr::<usize>(batch.as_mut_ptr() as usize + 8, handles.as_ptr() as usize);

        ENQ_LOG.clear();
        o(this_obj.as_ptr() as usize, batch.as_ptr() as usize);
        let la = ENQ_LOG.clone();

        ENQ_LOG.clear();
        n(this_obj.as_ptr() as usize, batch.as_ptr() as usize);
        let lb = ENQ_LOG.clone();

        crate::diff_eq!(la, lb, "batch cnt={cnt}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_job_node_alloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    let mut nodes_a = vec![0u8; 4 * 128];
    let mut nodes_b = vec![0u8; 4 * 128];
    let mut pool_a = [0u8; 32];
    let mut pool_b = [0u8; 32];
    let mut h_a = [0u8; 16];
    let mut h_b = [0u8; 16];

    let mut cases = 0;
    for (start_idx, first_busy) in [(0u32, false), (1u32, true), (3u32, true)] {
        nodes_a.fill(0);
        nodes_b.fill(0);
        pool_a.fill(0);
        pool_b.fill(0);
        h_a.fill(0xff);
        h_b.fill(0xff);

        let na = nodes_a.as_mut_ptr() as usize;
        let nb = nodes_b.as_mut_ptr() as usize;
        for slot in 0..4 {
            wr::<u16>(na + slot * 128 + 6, ((slot as u16 + 1) << 3) | 2);
            wr::<u16>(nb + slot * 128 + 6, ((slot as u16 + 1) << 3) | 2);
        }
        if first_busy {
            wr::<u8>(na + (start_idx as usize) * 128 + 4, 1);
            wr::<u8>(nb + (start_idx as usize) * 128 + 4, 1);
        }

        wr::<usize>(pool_a.as_mut_ptr() as usize, na);
        wr::<u32>(pool_a.as_mut_ptr() as usize + 0x10, start_idx);
        wr::<u32>(pool_a.as_mut_ptr() as usize + 0x14, 3);

        wr::<usize>(pool_b.as_mut_ptr() as usize, nb);
        wr::<u32>(pool_b.as_mut_ptr() as usize + 0x10, start_idx);
        wr::<u32>(pool_b.as_mut_ptr() as usize + 0x14, 3);

        let ra = o(pool_a.as_mut_ptr() as usize, h_a.as_mut_ptr() as usize);
        let rb = n(pool_b.as_mut_ptr() as usize, h_b.as_mut_ptr() as usize);

        crate::diff_eq!(ra, h_a.as_mut_ptr() as usize, "ret a");
        crate::diff_eq!(rb, h_b.as_mut_ptr() as usize, "ret b");
        let off_a = rd::<usize>(h_a.as_ptr() as usize) - na;
        let off_b = rd::<usize>(h_b.as_ptr() as usize) - nb;
        crate::diff_eq!(off_a, off_b, "allocated slot offset");
        wr::<usize>(h_a.as_mut_ptr() as usize, 0);
        wr::<usize>(h_b.as_mut_ptr() as usize, 0);
        crate::diff_eq!(h_a, h_b, "handle fields");
        crate::diff_eq!(nodes_a, nodes_b, "nodes array");
        crate::diff_eq!(
            rd::<u32>(pool_a.as_ptr() as usize + 0x10),
            rd::<u32>(pool_b.as_ptr() as usize + 0x10),
            "pool next index"
        );
        cases += 1;
    }
    Ok(cases)
}

unsafe fn run_wait_difftest(
    orig: usize,
    new: usize,
    uses_pump_one: bool,
    is_handle_wrapper: bool,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut ACTIVE_NODE: usize = 0;
    static mut EVENTS: Vec<u32> = Vec::new();

    unsafe extern "win64" fn mock_cancel(_sched: usize, node: usize) {
        unsafe {
            EVENTS.push(10);
            wr::<u32>(node, 0);
        }
    }
    unsafe extern "win64" fn mock_pump_one(_this: usize) -> usize {
        unsafe {
            EVENTS.push(20);
            if ACTIVE_NODE != 0 {
                let r = rd::<u32>(ACTIVE_NODE);
                wr::<u32>(ACTIVE_NODE, r.saturating_sub(1));
            }
            0x1234
        }
    }
    unsafe extern "win64" fn mock_pop_any(_sched: usize, _steal: u8) -> usize {
        unsafe {
            EVENTS.push(30);
            0x5678
        }
    }
    unsafe extern "win64" fn mock_run(_sched: usize, _job: usize) {
        unsafe {
            EVENTS.push(40);
            if ACTIVE_NODE != 0 {
                let r = rd::<u32>(ACTIVE_NODE);
                wr::<u32>(ACTIVE_NODE, r.saturating_sub(1));
            }
        }
    }

    let _m1 = crate::difftest::Mock::new(0x1_4248_6230, mock_cancel as *const ())?;
    let _m2 = if uses_pump_one {
        Some(crate::difftest::Mock::new(0x1_4248_54f0, mock_pump_one as *const ())?)
    } else {
        None
    };
    let _m3 = if !uses_pump_one {
        Some(crate::difftest::Mock::new(0x1_4248_6380, mock_pop_any as *const ())?)
    } else {
        None
    };
    let _m4 = if !uses_pump_one {
        Some(crate::difftest::Mock::new(0x1_4248_62c0, mock_run as *const ())?)
    } else {
        None
    };

    let mut this_obj = [0u8; 32];
    wr::<usize>(this_obj.as_mut_ptr() as usize + 0x10, 0x7777_0000);

    let mut cases = 0;
    for (init_state, init_ref, flag_a) in [
        // 1) Already completed (`ref == 0`): fast exit (`state == 2` skips `mark_node_cancelling`)
        (2u8, 0u32, 0u8),
        (2u8, 0u32, 1u8),
        // 2) Cancel path (`state == 3, ref == 0x10000` -> `mark_node_cancelling` sets `state = 4, ref = 1`,
        //    then loop sees `ref == 1` and calls `scheduler_cancel_job` (`mock_cancel` sets `ref -> 0`))
        (3u8, 0x10000u32, if is_handle_wrapper { 1u8 } else { 0u8 }),
        // 3) Direct cancel path (`state == 2, ref == 1`): loop sees `ref == 1` and calls `scheduler_cancel_job`
        (2u8, 1u32, if is_handle_wrapper { 1u8 } else { 0u8 }),
        // 4) Pump path (`state == 2, ref == 2`): pumps (`mock_pump_one` / `mock_run`) until `ref == 0`
        (2u8, 2u32, 0u8),
        (2u8, 2u32, 1u8),
    ] {
        if !is_handle_wrapper && flag_a == 1 {
            continue;
        }
        let mut node_a = [0u8; 16];
        let mut node_b = [0u8; 16];
        let mut h_a = [0u8; 16];
        let mut h_b = [0u8; 16];

        for (nd, hd) in [
            (node_a.as_mut_ptr() as usize, h_a.as_mut_ptr() as usize),
            (node_b.as_mut_ptr() as usize, h_b.as_mut_ptr() as usize),
        ] {
            wr::<u32>(nd, init_ref);
            wr::<u8>(nd + 4, init_state);
            wr::<u16>(nd + 6, 5 << 3);
            wr::<usize>(hd, nd);
            wr::<u16>(hd + 8, 5);
            wr::<u8>(hd + 0xa, flag_a);
            wr::<u8>(hd + 0xb, 0);
            wr::<u8>(hd + 0xc, 0);
        }

        EVENTS.clear();
        ACTIVE_NODE = node_a.as_mut_ptr() as usize;
        o(this_obj.as_ptr() as usize, h_a.as_mut_ptr() as usize);
        let ev_a = EVENTS.clone();

        EVENTS.clear();
        ACTIVE_NODE = node_b.as_mut_ptr() as usize;
        n(this_obj.as_ptr() as usize, h_b.as_mut_ptr() as usize);
        let ev_b = EVENTS.clone();

        wr::<usize>(h_a.as_mut_ptr() as usize, 0);
        wr::<usize>(h_b.as_mut_ptr() as usize, 0);
        crate::diff_eq!(ev_a, ev_b, "events state={init_state} ref={init_ref:#x} fa={flag_a}");
        crate::diff_eq!(node_a, node_b, "node state={init_state} ref={init_ref:#x}");
        crate::diff_eq!(h_a, h_b, "handle state={init_state} ref={init_ref:#x}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_wait_cancel_default(orig: usize, new: usize) -> Result<u32, String> {
    run_wait_difftest(orig, new, true, false)
}

pub unsafe fn diff_scheduler_wait_default(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut ACTIVE_NODE: usize = 0;
    static mut COUNT: u32 = 0;
    unsafe extern "win64" fn mock_pump(_this: usize) -> usize {
        unsafe {
            COUNT += 1;
            let r = rd::<u32>(ACTIVE_NODE);
            wr::<u32>(ACTIVE_NODE, r.saturating_sub(1));
            1
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4248_54f0, mock_pump as *const ())?;

    let this_obj = [0u8; 32];
    let mut cases = 0;
    for r in [0u32, 1, 3] {
        let mut node_a = [0u8; 16];
        let mut node_b = [0u8; 16];
        let mut h_a = [0u8; 16];
        let mut h_b = [0u8; 16];
        for (nd, hd) in [
            (node_a.as_mut_ptr() as usize, h_a.as_mut_ptr() as usize),
            (node_b.as_mut_ptr() as usize, h_b.as_mut_ptr() as usize),
        ] {
            wr::<u32>(nd, r);
            wr::<u16>(nd + 6, 2 << 3);
            wr::<usize>(hd, nd);
            wr::<u16>(hd + 8, 2);
        }
        COUNT = 0;
        ACTIVE_NODE = node_a.as_mut_ptr() as usize;
        o(this_obj.as_ptr() as usize, h_a.as_mut_ptr() as usize);
        let ca = COUNT;

        COUNT = 0;
        ACTIVE_NODE = node_b.as_mut_ptr() as usize;
        n(this_obj.as_ptr() as usize, h_b.as_mut_ptr() as usize);
        let cb = COUNT;

        crate::diff_eq!(ca, cb, "pump count r={r}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_wait_cancel_any(orig: usize, new: usize) -> Result<u32, String> {
    run_wait_difftest(orig, new, false, false)
}

pub unsafe fn diff_scheduler_wait_any(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut ACTIVE_NODE: usize = 0;
    static mut COUNT: u32 = 0;
    unsafe extern "win64" fn mock_pop(_sched: usize, _steal: u8) -> usize {
        0x1111
    }
    unsafe extern "win64" fn mock_run(_sched: usize, _job: usize) {
        unsafe {
            COUNT += 1;
            let r = rd::<u32>(ACTIVE_NODE);
            wr::<u32>(ACTIVE_NODE, r.saturating_sub(1));
        }
    }
    let _m1 = crate::difftest::Mock::new(0x1_4248_6380, mock_pop as *const ())?;
    let _m2 = crate::difftest::Mock::new(0x1_4248_62c0, mock_run as *const ())?;

    let this_obj = [0u8; 32];
    let mut cases = 0;
    for r in [0u32, 1, 3] {
        let mut node_a = [0u8; 16];
        let mut node_b = [0u8; 16];
        let mut h_a = [0u8; 16];
        let mut h_b = [0u8; 16];
        for (nd, hd) in [
            (node_a.as_mut_ptr() as usize, h_a.as_mut_ptr() as usize),
            (node_b.as_mut_ptr() as usize, h_b.as_mut_ptr() as usize),
        ] {
            wr::<u32>(nd, r);
            wr::<u16>(nd + 6, 2 << 3);
            wr::<usize>(hd, nd);
            wr::<u16>(hd + 8, 2);
        }
        COUNT = 0;
        ACTIVE_NODE = node_a.as_mut_ptr() as usize;
        o(this_obj.as_ptr() as usize, h_a.as_mut_ptr() as usize);
        let ca = COUNT;

        COUNT = 0;
        ACTIVE_NODE = node_b.as_mut_ptr() as usize;
        n(this_obj.as_ptr() as usize, h_b.as_mut_ptr() as usize);
        let cb = COUNT;

        crate::diff_eq!(ca, cb, "run count r={r}");
        cases += 1;
    }
    Ok(cases)
}

pub unsafe fn diff_scheduler_wait_handle_default(orig: usize, new: usize) -> Result<u32, String> {
    run_wait_difftest(orig, new, true, true)
}

pub unsafe fn diff_scheduler_wait_handle_any(orig: usize, new: usize) -> Result<u32, String> {
    run_wait_difftest(orig, new, false, true)
}

pub unsafe fn diff_worker_thread_main(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u32;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));

    static mut LOG: Vec<u32> = Vec::new();
    unsafe extern "win64" fn mock_init(_mgr: usize, kind: u32, idx: u32) {
        unsafe {
            LOG.push(100 + kind * 10 + idx);
        }
    }
    let _m = crate::difftest::Mock::new(0x1_4240_6a80, mock_init as *const ())?;

    let mut worker_a = vec![0u8; 0x150];
    let mut worker_b = vec![0u8; 0x150];
    wr::<u32>(worker_a.as_mut_ptr() as usize + 0x134, 3);
    wr::<u32>(worker_b.as_mut_ptr() as usize + 0x134, 3);
    // running flag (+0x130) == 0 exercises init + atomic xchg + clean exit
    LOG.clear();
    let ra = o(worker_a.as_mut_ptr() as usize);
    let la = LOG.clone();

    LOG.clear();
    let rb = n(worker_b.as_mut_ptr() as usize);
    let lb = LOG.clone();

    crate::diff_eq!(ra, rb, "ret");
    crate::diff_eq!(la, lb, "log");
    crate::diff_eq!(worker_a, worker_b, "worker state");
    Ok(1)
}

