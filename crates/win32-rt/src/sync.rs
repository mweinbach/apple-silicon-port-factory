//! User-mode synchronization primitives whose state lives in Windows-owned
//! memory: CRITICAL_SECTION, SRWLOCK, CONDITION_VARIABLE, INIT_ONCE,
//! WaitOnAddress, SLIST. Blocking uses `parking_lot_core` keyed by address.

use crate::types::*;
use parking_lot_core::{
    park, unpark_all, unpark_filter, unpark_one, FilterOp, ParkResult, ParkToken, UnparkToken,
    DEFAULT_PARK_TOKEN, DEFAULT_UNPARK_TOKEN,
};
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, AtomicU64, AtomicUsize, Ordering::*};
use std::time::{Duration, Instant};

fn deadline(ms: u32) -> Option<Instant> {
    (ms != INFINITE).then(|| Instant::now() + Duration::from_millis(ms as u64))
}

// ---------------------------------------------------------------------------
// CRITICAL_SECTION (40 bytes):
//   +0x00 DebugInfo  +0x08 LockCount(i32)  +0x0C RecursionCount(i32)
//   +0x10 OwningThread  +0x18 LockSemaphore  +0x20 SpinCount
// LockCount is used as a 3-state futex word: -1 free, 0 locked, 1 locked+waiters
// (Drepper "mutex3" offset by -1 so a freshly initialized CS reads -1 like on
// Windows).
// ---------------------------------------------------------------------------

#[repr(C)]
pub struct CriticalSection {
    pub debug_info: usize,
    pub lock_count: AtomicI32,
    pub recursion_count: AtomicI32,
    pub owning_thread: AtomicUsize,
    pub lock_semaphore: usize,
    pub spin_count: usize,
}

unsafe fn cs_lock_slow(cs: &CriticalSection) {
    let key = cs as *const _ as usize;
    let mut c = cs.lock_count.load(Relaxed);
    // Brief spin first.
    for _ in 0..(cs.spin_count & 0xFFFF).min(4000) {
        if c == -1 && cs.lock_count.compare_exchange(-1, 0, Acquire, Relaxed).is_ok() {
            return;
        }
        std::hint::spin_loop();
        c = cs.lock_count.load(Relaxed);
    }
    if c != 1 {
        c = cs.lock_count.swap(1, Acquire);
    }
    if c != -1 {
        let _scope = crate::hitch::Scope::enter(crate::hitch::ActKind::LockWait, key as u64);
        while c != -1 {
            park(key, || cs.lock_count.load(Relaxed) == 1, || {}, |_, _| {}, DEFAULT_PARK_TOKEN, None);
            c = cs.lock_count.swap(1, Acquire);
        }
    }
}

pub unsafe extern "win64" fn InitializeCriticalSection(cs: *mut CriticalSection) {
    InitializeCriticalSectionEx(cs, 0, 0);
}

pub unsafe extern "win64" fn InitializeCriticalSectionAndSpinCount(cs: *mut CriticalSection, spin: u32) -> BOOL {
    InitializeCriticalSectionEx(cs, spin, 0)
}

pub unsafe extern "win64" fn InitializeCriticalSectionEx(cs: *mut CriticalSection, spin: u32, _flags: u32) -> BOOL {
    let base = cs as *mut u8;
    std::ptr::write_unaligned(base as *mut usize, usize::MAX);
    std::ptr::write_unaligned(base.add(0x08) as *mut i32, -1);
    std::ptr::write_unaligned(base.add(0x0c) as *mut i32, 0);
    std::ptr::write_unaligned(base.add(0x10) as *mut usize, 0);
    std::ptr::write_unaligned(base.add(0x18) as *mut usize, 0);
    std::ptr::write_unaligned(base.add(0x20) as *mut usize, spin as usize);
    TRUE
}

pub unsafe extern "win64" fn SetCriticalSectionSpinCount(cs: *mut CriticalSection, spin: u32) -> u32 {
    let p = (cs as *mut u8).add(0x20) as *mut usize;
    let old = std::ptr::read_unaligned(p) as u32;
    std::ptr::write_unaligned(p, spin as usize);
    old
}

pub unsafe extern "win64" fn DeleteCriticalSection(_cs: *mut CriticalSection) {}

#[inline(never)]
pub unsafe extern "win64" fn EnterCriticalSection(cs: *mut CriticalSection) {
    let tid = crate::teb::thread_id() as usize;
    if (cs as usize) & 7 != 0 {
        let key = cs as usize;
        let base = cs as *mut u8;
        {
            let _g = unaligned_lock(key);
            let own = std::ptr::read_unaligned(base.add(0x10) as *const usize);
            if own == tid {
                let r = std::ptr::read_unaligned(base.add(0x0c) as *const i32);
                std::ptr::write_unaligned(base.add(0x0c) as *mut i32, r.wrapping_add(1));
                return;
            }
            let lc = std::ptr::read_unaligned(base.add(0x08) as *const i32);
            if lc == -1 {
                std::ptr::write_unaligned(base.add(0x08) as *mut i32, 0);
                std::ptr::write_unaligned(base.add(0x10) as *mut usize, tid);
                std::ptr::write_unaligned(base.add(0x0c) as *mut i32, 1);
                return;
            }
            // Mark contended so LeaveCriticalSection wakes a waiter.
            std::ptr::write_unaligned(base.add(0x08) as *mut i32, 1);
        }
        let _scope = crate::hitch::Scope::enter(crate::hitch::ActKind::LockWait, key as u64);
        loop {
            park(
                key,
                || {
                    let _g = unaligned_lock(key);
                    std::ptr::read_unaligned(base.add(0x08) as *const i32) == 1
                },
                || {},
                |_, _| {},
                DEFAULT_PARK_TOKEN,
                None,
            );
            let _g = unaligned_lock(key);
            let lc = std::ptr::read_unaligned(base.add(0x08) as *const i32);
            if lc == -1 {
                // Keep lock_count = 1 (contended) in case other waiters are still parked.
                std::ptr::write_unaligned(base.add(0x08) as *mut i32, 1);
                std::ptr::write_unaligned(base.add(0x10) as *mut usize, tid);
                std::ptr::write_unaligned(base.add(0x0c) as *mut i32, 1);
                return;
            }
            if lc != 1 {
                std::ptr::write_unaligned(base.add(0x08) as *mut i32, 1);
            }
        }
    }
    let cs = &*cs;
    if cs.owning_thread.load(Relaxed) == tid {
        cs.recursion_count.fetch_add(1, Relaxed);
        return;
    }
    if cs.lock_count.compare_exchange(-1, 0, Acquire, Relaxed).is_err() {
        cs_lock_slow(cs);
    }
    cs.owning_thread.store(tid, Relaxed);
    cs.recursion_count.store(1, Relaxed);
}

pub unsafe extern "win64" fn TryEnterCriticalSection(cs: *mut CriticalSection) -> BOOL {
    let tid = crate::teb::thread_id() as usize;
    if (cs as usize) & 7 != 0 {
        let _g = unaligned_lock(cs as usize);
        let base = cs as *mut u8;
        let own = std::ptr::read_unaligned(base.add(0x10) as *const usize);
        if own == tid {
            let r = std::ptr::read_unaligned(base.add(0x0c) as *const i32);
            std::ptr::write_unaligned(base.add(0x0c) as *mut i32, r.wrapping_add(1));
            return TRUE;
        }
        let lc = std::ptr::read_unaligned(base.add(0x08) as *const i32);
        if lc == -1 {
            std::ptr::write_unaligned(base.add(0x08) as *mut i32, 0);
            std::ptr::write_unaligned(base.add(0x10) as *mut usize, tid);
            std::ptr::write_unaligned(base.add(0x0c) as *mut i32, 1);
            return TRUE;
        }
        return FALSE;
    }
    let cs = &*cs;
    if cs.owning_thread.load(Relaxed) == tid {
        cs.recursion_count.fetch_add(1, Relaxed);
        return TRUE;
    }
    if cs.lock_count.compare_exchange(-1, 0, Acquire, Relaxed).is_ok() {
        cs.owning_thread.store(tid, Relaxed);
        cs.recursion_count.store(1, Relaxed);
        TRUE
    } else {
        FALSE
    }
}

#[inline(never)]
pub unsafe extern "win64" fn LeaveCriticalSection(cs: *mut CriticalSection) {
    if (cs as usize) & 7 != 0 {
        let key = cs as usize;
        let wake = {
            let _g = unaligned_lock(key);
            let base = cs as *mut u8;
            let r = std::ptr::read_unaligned(base.add(0x0c) as *const i32);
            std::ptr::write_unaligned(base.add(0x0c) as *mut i32, r.wrapping_sub(1));
            if r != 1 {
                return;
            }
            std::ptr::write_unaligned(base.add(0x10) as *mut usize, 0);
            let lc = std::ptr::read_unaligned(base.add(0x08) as *const i32);
            std::ptr::write_unaligned(base.add(0x08) as *mut i32, -1);
            lc == 1
        };
        if wake {
            unpark_one(key, |_| DEFAULT_UNPARK_TOKEN);
        }
        return;
    }
    let cs = &*cs;
    if cs.recursion_count.fetch_sub(1, Relaxed) != 1 {
        return;
    }
    cs.owning_thread.store(0, Relaxed);
    if cs.lock_count.swap(-1, Release) == 1 {
        unpark_one(cs as *const _ as usize, |_| DEFAULT_UNPARK_TOKEN);
    }
}

/// Alignment-safe check for `RtlIsCriticalSectionLockedByThread`.
pub(crate) unsafe fn is_cs_locked_by_thread(cs: *const CriticalSection, tid: usize) -> bool {
    if (cs as usize) & 7 != 0 {
        let _g = unaligned_lock(cs as usize);
        let own = std::ptr::read_unaligned((cs as *const u8).add(0x10) as *const usize);
        own == tid
    } else {
        (*cs).owning_thread.load(Relaxed) == tid
    }
}

#[inline]
unsafe fn cs_get_recursion(cs: *mut CriticalSection) -> i32 {
    if (cs as usize) & 7 != 0 {
        let _g = unaligned_lock(cs as usize);
        std::ptr::read_unaligned((cs as *const u8).add(0x0c) as *const i32)
    } else {
        (*cs).recursion_count.load(Relaxed)
    }
}

#[inline]
unsafe fn cs_set_recursion(cs: *mut CriticalSection, val: i32) {
    if (cs as usize) & 7 != 0 {
        let _g = unaligned_lock(cs as usize);
        std::ptr::write_unaligned((cs as *mut u8).add(0x0c) as *mut i32, val);
    } else {
        (*cs).recursion_count.store(val, Relaxed);
    }
}

// ---------------------------------------------------------------------------
// Alignment-safe pointer-sized atomic word helper (x86_64 allows unaligned
// SRWLOCK / CONDITION_VARIABLE / INIT_ONCE in byte buffers; arm64 hardware
// atomics fault on <8-byte alignment).
// ---------------------------------------------------------------------------

const UNALIGNED_SHARDS: usize = 64;
static UNALIGNED_SYNC_LOCKS: [std::sync::Mutex<()>; UNALIGNED_SHARDS] =
    [const { std::sync::Mutex::new(()) }; UNALIGNED_SHARDS];

#[inline]
fn unaligned_lock(addr: usize) -> std::sync::MutexGuard<'static, ()> {
    let idx = (addr.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 58) & (UNALIGNED_SHARDS - 1);
    UNALIGNED_SYNC_LOCKS[idx].lock().unwrap_or_else(|e| e.into_inner())
}

#[derive(Copy, Clone)]
struct AtomicWord(*mut usize);

impl AtomicWord {
    #[inline]
    fn from_ptr(p: *mut c_void) -> Self {
        Self(p as *mut usize)
    }
    #[inline]
    fn addr(self) -> usize {
        self.0 as usize
    }
    #[inline]
    unsafe fn load(self, order: std::sync::atomic::Ordering) -> usize {
        if (self.0 as usize) & 7 == 0 {
            (&*(self.0 as *const AtomicUsize)).load(order)
        } else {
            let _g = unaligned_lock(self.0 as usize);
            std::ptr::read_unaligned(self.0)
        }
    }
    #[inline]
    unsafe fn store(self, val: usize, order: std::sync::atomic::Ordering) {
        if (self.0 as usize) & 7 == 0 {
            (&*(self.0 as *const AtomicUsize)).store(val, order);
        } else {
            let _g = unaligned_lock(self.0 as usize);
            std::ptr::write_unaligned(self.0, val);
        }
    }
    #[inline]
    unsafe fn compare_exchange(
        self,
        current: usize,
        new: usize,
        success: std::sync::atomic::Ordering,
        failure: std::sync::atomic::Ordering,
    ) -> Result<usize, usize> {
        if (self.0 as usize) & 7 == 0 {
            (&*(self.0 as *const AtomicUsize)).compare_exchange(current, new, success, failure)
        } else {
            let _g = unaligned_lock(self.0 as usize);
            let prev = std::ptr::read_unaligned(self.0);
            if prev == current {
                std::ptr::write_unaligned(self.0, new);
                Ok(prev)
            } else {
                Err(prev)
            }
        }
    }
    #[inline]
    unsafe fn compare_exchange_weak(
        self,
        current: usize,
        new: usize,
        success: std::sync::atomic::Ordering,
        failure: std::sync::atomic::Ordering,
    ) -> Result<usize, usize> {
        self.compare_exchange(current, new, success, failure)
    }
    #[inline]
    unsafe fn fetch_and(self, val: usize, order: std::sync::atomic::Ordering) -> usize {
        if (self.0 as usize) & 7 == 0 {
            (&*(self.0 as *const AtomicUsize)).fetch_and(val, order)
        } else {
            let _g = unaligned_lock(self.0 as usize);
            let prev = std::ptr::read_unaligned(self.0);
            std::ptr::write_unaligned(self.0, prev & val);
            prev
        }
    }
    #[inline]
    unsafe fn fetch_add(self, val: usize, order: std::sync::atomic::Ordering) -> usize {
        if (self.0 as usize) & 7 == 0 {
            (&*(self.0 as *const AtomicUsize)).fetch_add(val, order)
        } else {
            let _g = unaligned_lock(self.0 as usize);
            let prev = std::ptr::read_unaligned(self.0);
            std::ptr::write_unaligned(self.0, prev.wrapping_add(val));
            prev
        }
    }
    #[inline]
    unsafe fn fetch_sub(self, val: usize, order: std::sync::atomic::Ordering) -> usize {
        if (self.0 as usize) & 7 == 0 {
            (&*(self.0 as *const AtomicUsize)).fetch_sub(val, order)
        } else {
            let _g = unaligned_lock(self.0 as usize);
            let prev = std::ptr::read_unaligned(self.0);
            std::ptr::write_unaligned(self.0, prev.wrapping_sub(val));
            prev
        }
    }
}

// ---------------------------------------------------------------------------
// SRWLOCK (pointer-sized):
//   bit 0: SRW_WRITER          (held exclusively)
//   bit 1: SRW_PARKED          (one or more threads parked)
//   bit 2: SRW_WAITING_WRITER  (writer waiting -> new readers yield)
//   bits 3..: reader count     (step SRW_READER = 8)
// ---------------------------------------------------------------------------

const SRW_WRITER: usize = 1;
const SRW_PARKED: usize = 2;
const SRW_WAITING_WRITER: usize = 4;
const SRW_READER: usize = 8;
const SRW_READERS_MASK: usize = !7;

const SRW_TOKEN_READER: ParkToken = ParkToken(0);
const SRW_TOKEN_WRITER: ParkToken = ParkToken(1);
const SRW_UNPARK_ALLOW_READER: UnparkToken = UnparkToken(1);
const SRW_UNPARK_WRITER_HANDOFF: UnparkToken = UnparkToken(2);

#[inline]
unsafe fn srw(p: *mut c_void) -> AtomicWord {
    AtomicWord::from_ptr(p)
}

unsafe fn srw_park_writer(lock: AtomicWord) -> bool {
    let s = lock.load(Relaxed);
    if s & (SRW_WRITER | SRW_READERS_MASK) == 0 {
        return false;
    }
    let want = s | SRW_PARKED | SRW_WAITING_WRITER;
    if s != want && lock.compare_exchange(s, want, Relaxed, Relaxed).is_err() {
        return false;
    }
    let key = lock.addr();
    let _scope = crate::hitch::Scope::enter(crate::hitch::ActKind::LockWait, key as u64);
    let r = park(
        key,
        || {
            let s = lock.load(Relaxed);
            (s & (SRW_PARKED | SRW_WAITING_WRITER) == (SRW_PARKED | SRW_WAITING_WRITER))
                && (s & (SRW_WRITER | SRW_READERS_MASK) != 0)
        },
        || {},
        |_, _| {},
        SRW_TOKEN_WRITER,
        None,
    );
    matches!(r, ParkResult::Unparked(tok) if tok == SRW_UNPARK_WRITER_HANDOFF)
}

unsafe fn srw_park_reader(lock: AtomicWord, block_mask: usize) -> bool {
    let s = lock.load(Relaxed);
    if s & block_mask == 0 {
        return false;
    }
    if s & SRW_PARKED == 0 && lock.compare_exchange(s, s | SRW_PARKED, Relaxed, Relaxed).is_err() {
        return false;
    }
    let key = lock.addr();
    let _scope = crate::hitch::Scope::enter(crate::hitch::ActKind::LockWait, key as u64);
    let r = park(
        key,
        || {
            let s = lock.load(Relaxed);
            s & SRW_PARKED != 0 && (s & block_mask != 0)
        },
        || {},
        |_, _| {},
        SRW_TOKEN_READER,
        None,
    );
    matches!(r, ParkResult::Unparked(tok) if tok == SRW_UNPARK_ALLOW_READER)
}

#[cold]
unsafe fn srw_wake(lock: AtomicWord) {
    use std::cell::Cell;
    // Writer-priority wake: if SRW_WAITING_WRITER is set, scan strictly for the first waiting
    // writer (skipping any readers queued ahead of it) and keep SRW_WAITING_WRITER set across the
    // wake so barging readers cannot starve the woken writer before it acquires SRW_WRITER.
    let prefer_writer = lock.load(Relaxed) & SRW_WAITING_WRITER != 0;
    let woke_writer = Cell::new(false);
    let woke_reader = Cell::new(false);
    let has_remaining_writer = Cell::new(false);
    let retry_readers = Cell::new(false);
    unpark_filter(
        lock.addr(),
        |token| {
            if token == SRW_TOKEN_WRITER {
                if woke_writer.get() || woke_reader.get() {
                    has_remaining_writer.set(true);
                    FilterOp::Skip
                } else {
                    woke_writer.set(true);
                    FilterOp::Unpark
                }
            } else if prefer_writer || woke_writer.get() {
                FilterOp::Skip
            } else {
                woke_reader.set(true);
                FilterOp::Unpark
            }
        },
        |res| {
            let mut clear = 0usize;
            if !res.have_more_threads {
                clear |= SRW_PARKED;
                if !woke_writer.get() {
                    clear |= SRW_WAITING_WRITER;
                }
            } else if !woke_writer.get() && !has_remaining_writer.get() {
                clear |= SRW_WAITING_WRITER;
                if prefer_writer {
                    retry_readers.set(true);
                }
            }
            if clear != 0 {
                lock.fetch_and(!clear, Relaxed);
            }
            if woke_writer.get() {
                SRW_UNPARK_WRITER_HANDOFF
            } else if woke_reader.get() {
                SRW_UNPARK_ALLOW_READER
            } else {
                DEFAULT_UNPARK_TOKEN
            }
        },
    );
    // SRW_WAITING_WRITER was set (e.g., from a prior handoff) but no writer remained in the queue;
    // wake any queued readers now that SRW_WAITING_WRITER has been cleared.
    if retry_readers.get() {
        let woke_writer2 = Cell::new(false);
        let woke_reader2 = Cell::new(false);
        let has_remaining_writer2 = Cell::new(false);
        unpark_filter(
            lock.addr(),
            |token| {
                if token == SRW_TOKEN_WRITER {
                    if woke_writer2.get() || woke_reader2.get() {
                        has_remaining_writer2.set(true);
                        FilterOp::Skip
                    } else {
                        woke_writer2.set(true);
                        FilterOp::Unpark
                    }
                } else if woke_writer2.get() {
                    FilterOp::Skip
                } else {
                    woke_reader2.set(true);
                    FilterOp::Unpark
                }
            },
            |res| {
                let mut clear = 0usize;
                if !res.have_more_threads {
                    clear |= SRW_PARKED;
                    if !woke_writer2.get() {
                        clear |= SRW_WAITING_WRITER;
                    }
                } else if !woke_writer2.get() && !has_remaining_writer2.get() {
                    clear |= SRW_WAITING_WRITER;
                }
                if clear != 0 {
                    lock.fetch_and(!clear, Relaxed);
                }
                if woke_writer2.get() {
                    SRW_UNPARK_WRITER_HANDOFF
                } else if woke_reader2.get() {
                    SRW_UNPARK_ALLOW_READER
                } else {
                    DEFAULT_UNPARK_TOKEN
                }
            },
        );
    }
}

pub unsafe extern "win64" fn InitializeSRWLock(p: *mut c_void) {
    srw(p).store(0, Relaxed);
}

#[inline(never)]
pub unsafe extern "win64" fn TryAcquireSRWLockExclusive(p: *mut c_void) -> BOOLEAN {
    let l = srw(p);
    let s = l.load(Relaxed);
    (s & (SRW_WRITER | SRW_READERS_MASK) == 0 && l.compare_exchange(s, s | SRW_WRITER, Acquire, Relaxed).is_ok()) as u8
}

#[inline(never)]
pub unsafe extern "win64" fn AcquireSRWLockExclusive(p: *mut c_void) {
    let l = srw(p);
    let mut spins = 0;
    let mut owns_waiting_bit = false;
    loop {
        let s = l.load(Relaxed);
        if s & (SRW_WRITER | SRW_READERS_MASK) == 0 {
            let next = if owns_waiting_bit && (s & SRW_PARKED == 0) {
                (s & !SRW_WAITING_WRITER) | SRW_WRITER
            } else {
                s | SRW_WRITER
            };
            if l.compare_exchange_weak(s, next, Acquire, Relaxed).is_ok() {
                return;
            }
            continue;
        }
        if spins < 100 {
            spins += 1;
            std::hint::spin_loop();
            continue;
        }
        if srw_park_writer(l) {
            owns_waiting_bit = true;
        }
        spins = 0;
    }
}

#[inline(never)]
pub unsafe extern "win64" fn ReleaseSRWLockExclusive(p: *mut c_void) {
    let l = srw(p);
    let prev = l.fetch_and(!SRW_WRITER, Release);
    if prev & SRW_PARKED != 0 {
        srw_wake(l);
    }
}

#[inline(never)]
pub unsafe extern "win64" fn TryAcquireSRWLockShared(p: *mut c_void) -> BOOLEAN {
    let l = srw(p);
    let mut s = l.load(Relaxed);
    while s & (SRW_WRITER | SRW_WAITING_WRITER) == 0 {
        match l.compare_exchange_weak(s, s + SRW_READER, Acquire, Relaxed) {
            Ok(_) => return 1,
            Err(v) => s = v,
        }
    }
    0
}

#[inline(never)]
pub unsafe extern "win64" fn AcquireSRWLockShared(p: *mut c_void) {
    let l = srw(p);
    let mut spins = 0;
    let mut allow_bypass = false;
    loop {
        let block_mask = if allow_bypass {
            SRW_WRITER
        } else {
            SRW_WRITER | SRW_WAITING_WRITER
        };
        let s = l.load(Relaxed);
        if s & block_mask == 0 {
            if l.compare_exchange_weak(s, s + SRW_READER, Acquire, Relaxed).is_ok() {
                return;
            }
            continue;
        }
        if spins < 100 {
            spins += 1;
            std::hint::spin_loop();
            continue;
        }
        allow_bypass = srw_park_reader(l, block_mask);
        spins = 0;
    }
}

#[inline(never)]
pub unsafe extern "win64" fn ReleaseSRWLockShared(p: *mut c_void) {
    let l = srw(p);
    let prev = l.fetch_sub(SRW_READER, Release);
    if (prev & SRW_PARKED != 0) && ((prev - SRW_READER) & SRW_READERS_MASK == 0) {
        srw_wake(l);
    }
}

// ---------------------------------------------------------------------------
// CONDITION_VARIABLE (pointer-sized):
//   low 24 bits: active waiter count (0-waiter fast path skips parking_lot_core)
//   upper bits:  sequence counter
// ---------------------------------------------------------------------------

const CV_WAITERS_MASK: usize = 0x00FF_FFFF;
const CV_SEQ_SHIFT: u32 = 24;
const CV_SEQ_INC: usize = 1usize << CV_SEQ_SHIFT;

unsafe fn cv_wait(cv: *mut c_void, ms: u32, release: impl FnOnce(), reacquire: impl FnOnce()) -> BOOL {
    let word = AtomicWord::from_ptr(cv);
    let s0 = word.fetch_add(1, SeqCst);
    let seq0 = s0 >> CV_SEQ_SHIFT;
    release();
    let r = park(
        cv as usize,
        || (word.load(Relaxed) >> CV_SEQ_SHIFT) == seq0,
        || {},
        |_, _| {},
        DEFAULT_PARK_TOKEN,
        deadline(ms),
    );
    word.fetch_sub(1, Release);
    reacquire();
    if let ParkResult::TimedOut = r {
        crate::teb::set_last_error(err::ERROR_TIMEOUT);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn InitializeConditionVariable(cv: *mut c_void) {
    AtomicWord::from_ptr(cv).store(0, Relaxed);
}

pub unsafe extern "win64" fn SleepConditionVariableCS(cv: *mut c_void, cs: *mut CriticalSection, ms: u32) -> BOOL {
    // Windows releases the CS fully (and restores recursion count on reacquire).
    let rec = cs_get_recursion(cs);
    cv_wait(
        cv,
        ms,
        || {
            cs_set_recursion(cs, 1);
            LeaveCriticalSection(cs);
        },
        || {
            EnterCriticalSection(cs);
            cs_set_recursion(cs, rec);
        },
    )
}

pub unsafe extern "win64" fn SleepConditionVariableSRW(cv: *mut c_void, lock: *mut c_void, ms: u32, flags: u32) -> BOOL {
    const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1;
    let t0 = crate::hitch::span_begin();
    let r = if flags & CONDITION_VARIABLE_LOCKMODE_SHARED != 0 {
        cv_wait(cv, ms, || ReleaseSRWLockShared(lock), || AcquireSRWLockShared(lock))
    } else {
        cv_wait(cv, ms, || ReleaseSRWLockExclusive(lock), || AcquireSRWLockExclusive(lock))
    };
    crate::hitch::span_end(t0, crate::hitch::SPAN_CV_WAIT);
    r
}

#[inline(never)]
pub unsafe extern "win64" fn WakeConditionVariable(cv: *mut c_void) {
    // Pair with waiter's `fetch_add(1, SeqCst)` even when caller wrote predicate
    // without a SeqCst barrier.
    std::sync::atomic::fence(SeqCst);
    let w = AtomicWord::from_ptr(cv);
    if (w.load(Acquire) & CV_WAITERS_MASK) == 0 {
        return;
    }
    w.fetch_add(CV_SEQ_INC, Release);
    unpark_one(cv as usize, |_| DEFAULT_UNPARK_TOKEN);
}

#[inline(never)]
pub unsafe extern "win64" fn WakeAllConditionVariable(cv: *mut c_void) {
    std::sync::atomic::fence(SeqCst);
    let w = AtomicWord::from_ptr(cv);
    if (w.load(Acquire) & CV_WAITERS_MASK) == 0 {
        return;
    }
    w.fetch_add(CV_SEQ_INC, Release);
    unpark_all(cv as usize, DEFAULT_UNPARK_TOKEN);
}

// ---------------------------------------------------------------------------
// INIT_ONCE (pointer-sized): low 2 bits state (0 uninitialized, 1 sync running,
// 2 complete, 3 async running), upper bits = 4-byte-aligned context pointer.
// Matches Windows RTL_RUN_ONCE / InitOnce* state machine.
// ---------------------------------------------------------------------------

const INIT_ONCE_CHECK_ONLY: u32 = 1;
const INIT_ONCE_ASYNC: u32 = 2;
const INIT_ONCE_INIT_FAILED: u32 = 4;

pub type InitOnceFn = unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut *mut c_void) -> BOOL;

pub unsafe extern "win64" fn InitOnceInitialize(once: *mut c_void) {
    if !once.is_null() {
        AtomicWord::from_ptr(once).store(0, Relaxed);
    }
}

pub unsafe extern "win64" fn InitOnceBeginInitialize(
    once: *mut c_void,
    flags: u32,
    pending: *mut BOOL,
    ctx: *mut *mut c_void,
) -> BOOL {
    if once.is_null()
        || pending.is_null()
        || (flags & !(INIT_ONCE_CHECK_ONLY | INIT_ONCE_ASYNC)) != 0
        || (flags & (INIT_ONCE_CHECK_ONLY | INIT_ONCE_ASYNC)) == (INIT_ONCE_CHECK_ONLY | INIT_ONCE_ASYNC)
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let word = AtomicWord::from_ptr(once);
    loop {
        let value = word.load(Acquire);
        match value & 3 {
            2 => {
                pending.write_unaligned(FALSE);
                if !ctx.is_null() {
                    ctx.write_unaligned((value & !3) as *mut c_void);
                }
                return TRUE;
            }
            state => {
                if flags & INIT_ONCE_CHECK_ONLY != 0 {
                    // Per Microsoft Learn (InitOnceBeginInitialize): INIT_ONCE_CHECK_ONLY
                    // returns FALSE when initialization has not yet completed.
                    std::ptr::write_unaligned(pending, FALSE);
                    crate::teb::set_last_error(err::ERROR_GEN_FAILURE);
                    return FALSE;
                }
                if flags & INIT_ONCE_ASYNC != 0 {
                    match state {
                        0 => {
                            if word.compare_exchange(0, 3, AcqRel, Acquire).is_ok() {
                                std::ptr::write_unaligned(pending, TRUE);
                                return TRUE;
                            }
                        }
                        3 => {
                            // Multiple threads may race asynchronous initialization without blocking.
                            std::ptr::write_unaligned(pending, TRUE);
                            return TRUE;
                        }
                        _ => {
                            // Synchronous initialization is already in progress; cannot mix modes.
                            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                            return FALSE;
                        }
                    }
                } else {
                    match state {
                        0 => {
                            if word.compare_exchange(0, 1, AcqRel, Acquire).is_ok() {
                                std::ptr::write_unaligned(pending, TRUE);
                                return TRUE;
                            }
                        }
                        1 => {
                            park(
                                once as usize,
                                || word.load(Acquire) & 3 == 1,
                                || {},
                                |_, _| {},
                                DEFAULT_PARK_TOKEN,
                                None,
                            );
                        }
                        _ => {
                            // Asynchronous initialization is already in progress; synchronous call fails.
                            crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                            return FALSE;
                        }
                    }
                }
            }
        }
    }
}

pub unsafe extern "win64" fn InitOnceComplete(once: *mut c_void, flags: u32, ctx: *mut c_void) -> BOOL {
    if once.is_null()
        || (flags & !(INIT_ONCE_ASYNC | INIT_ONCE_INIT_FAILED)) != 0
        || (flags & (INIT_ONCE_ASYNC | INIT_ONCE_INIT_FAILED)) == (INIT_ONCE_ASYNC | INIT_ONCE_INIT_FAILED)
        || ((ctx as usize) & 3) != 0
        || (flags == INIT_ONCE_INIT_FAILED && !ctx.is_null())
    {
        crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
        return FALSE;
    }
    let a = AtomicWord::from_ptr(once);
    if flags & INIT_ONCE_ASYNC != 0 {
        let done = (ctx as usize & !3) | 2;
        match a.compare_exchange(3, done, AcqRel, Acquire) {
            Ok(_) => {
                unpark_all(once as usize, DEFAULT_UNPARK_TOKEN);
                TRUE
            }
            Err(cur) if (cur & 3) == 2 => {
                // Another racing INIT_ONCE_ASYNC thread won completion first.
                crate::teb::set_last_error(err::ERROR_GEN_FAILURE);
                FALSE
            }
            Err(_) => {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                FALSE
            }
        }
    } else if flags & INIT_ONCE_INIT_FAILED != 0 {
        match a.compare_exchange(1, 0, AcqRel, Acquire) {
            Ok(_) => {
                unpark_all(once as usize, DEFAULT_UNPARK_TOKEN);
                TRUE
            }
            Err(_) => {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                FALSE
            }
        }
    } else {
        let done = (ctx as usize & !3) | 2;
        match a.compare_exchange(1, done, AcqRel, Acquire) {
            Ok(_) => {
                unpark_all(once as usize, DEFAULT_UNPARK_TOKEN);
                TRUE
            }
            Err(_) => {
                crate::teb::set_last_error(err::ERROR_INVALID_PARAMETER);
                FALSE
            }
        }
    }
}

pub unsafe extern "win64" fn InitOnceExecuteOnce(once: *mut c_void, f: InitOnceFn, param: *mut c_void, ctx: *mut *mut c_void) -> BOOL {
    let mut pending = FALSE;
    if InitOnceBeginInitialize(once, 0, &mut pending, ctx) == FALSE {
        return FALSE;
    }
    if pending == FALSE {
        return TRUE;
    }
    let mut c: *mut c_void = std::ptr::null_mut();
    let ok = crate::call_guest8(
        f as usize,
        once as u64,
        param as u64,
        (&mut c as *mut *mut c_void) as u64,
        0,
        0,
        0,
        0,
        0,
    ) as BOOL;
    if ok != FALSE {
        if InitOnceComplete(once, 0, c) == FALSE {
            InitOnceComplete(once, INIT_ONCE_INIT_FAILED, std::ptr::null_mut());
            return FALSE;
        }
        if !ctx.is_null() {
            std::ptr::write_unaligned(ctx, (c as usize & !3) as *mut c_void);
        }
        TRUE
    } else {
        InitOnceComplete(once, INIT_ONCE_INIT_FAILED, std::ptr::null_mut());
        FALSE
    }
}

// ---------------------------------------------------------------------------
// WaitOnAddress family (api-ms-win-core-synch-l1-2-0)
// ---------------------------------------------------------------------------

unsafe fn mem_equal(a: *const c_void, b: *const c_void, size: usize) -> bool {
    use std::sync::atomic::{fence, AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};
    // Acquire load on the shared `Address` so lockless `WaitOnAddress` fast-path returns
    // establish a real happens-before edge on ARM64 with prior guest/host producer writes.
    match size {
        1 => {
            let av = (*(a as *const AtomicU8)).load(Ordering::Acquire);
            av == std::ptr::read_unaligned(b as *const u8)
        }
        2 => {
            let av = if (a as usize) & 1 == 0 {
                (*(a as *const AtomicU16)).load(Ordering::Acquire)
            } else {
                let v = std::ptr::read_unaligned(a as *const u16);
                fence(Ordering::Acquire);
                v
            };
            av == std::ptr::read_unaligned(b as *const u16)
        }
        4 => {
            let av = if (a as usize) & 3 == 0 {
                (*(a as *const AtomicU32)).load(Ordering::Acquire)
            } else {
                let v = std::ptr::read_unaligned(a as *const u32);
                fence(Ordering::Acquire);
                v
            };
            av == std::ptr::read_unaligned(b as *const u32)
        }
        8 => {
            let av = if (a as usize) & 7 == 0 {
                (*(a as *const AtomicU64)).load(Ordering::Acquire)
            } else {
                let v = std::ptr::read_unaligned(a as *const u64);
                fence(Ordering::Acquire);
                v
            };
            av == std::ptr::read_unaligned(b as *const u64)
        }
        _ => false,
    }
}

pub unsafe extern "win64" fn WaitOnAddress(addr: *mut c_void, cmp: *const c_void, size: usize, ms: u32) -> BOOL {
    if !mem_equal(addr, cmp, size) {
        return TRUE;
    }
    let r = park(addr as usize, || mem_equal(addr, cmp, size), || {}, |_, _| {}, DEFAULT_PARK_TOKEN, deadline(ms));
    if let ParkResult::TimedOut = r {
        crate::teb::set_last_error(err::ERROR_TIMEOUT);
        return FALSE;
    }
    TRUE
}

pub unsafe extern "win64" fn WakeByAddressSingle(addr: *mut c_void) {
    // Pair with `WaitOnAddress`'s Acquire load even when prior writes came from non-TSO host
    // helpers (`memcpy`/etc.) or when no waiter is parked in `parking_lot_core`.
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    unpark_one(addr as usize, |_| DEFAULT_UNPARK_TOKEN);
}

pub unsafe extern "win64" fn WakeByAddressAll(addr: *mut c_void) {
    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);
    unpark_all(addr as usize, DEFAULT_UNPARK_TOKEN);
}

// ---------------------------------------------------------------------------
// Interlocked SLIST (x64 SLIST_HEADER, 16 bytes):
//   low u64: Depth:16 | Sequence:48     high u64: Reserved:4 | NextEntry:60 (ptr >> 4)
// Lock-free 128-bit CAS on 16-byte-aligned headers on aarch64 (caspal / ldaxp+stlxp);
// sharded fallback lock for unaligned headers or non-aarch64 hosts.
// ---------------------------------------------------------------------------

#[repr(C, align(16))]
pub struct SListHeader {
    lo: u64,
    hi: u64,
}

#[inline(always)]
fn slist_depth(lo: u64) -> u16 {
    lo as u16
}
#[inline(always)]
fn slist_seq(lo: u64) -> u64 {
    lo >> 16
}
#[inline(always)]
fn slist_next(hi: u64) -> *mut *mut c_void {
    ((hi >> 4) << 4) as *mut *mut c_void
}
#[inline(always)]
fn slist_pack(next: *mut *mut c_void, depth: u16, seq: u64) -> (u64, u64) {
    (depth as u64 | (seq << 16), (next as u64 >> 4) << 4)
}

#[cfg(all(target_arch = "aarch64", target_feature = "lse"))]
#[inline]
unsafe fn cas128(ptr: *mut u64, mut exp_lo: u64, mut exp_hi: u64, new_lo: u64, new_hi: u64) -> Result<(), (u64, u64)> {
    let orig_lo = exp_lo;
    let orig_hi = exp_hi;
    std::arch::asm!(
        "caspal x0, x1, x2, x3, [{ptr}]",
        ptr = in(reg) ptr,
        inout("x0") exp_lo,
        inout("x1") exp_hi,
        in("x2") new_lo,
        in("x3") new_hi,
        options(nostack, preserves_flags),
    );
    if exp_lo == orig_lo && exp_hi == orig_hi {
        Ok(())
    } else {
        Err((exp_lo, exp_hi))
    }
}

#[cfg(all(target_arch = "aarch64", not(target_feature = "lse")))]
#[inline]
unsafe fn cas128(ptr: *mut u64, exp_lo: u64, exp_hi: u64, new_lo: u64, new_hi: u64) -> Result<(), (u64, u64)> {
    let cur_lo: u64;
    let cur_hi: u64;
    std::arch::asm!(
        "1:",
        "ldaxp {cur_lo}, {cur_hi}, [{ptr}]",
        "cmp {cur_lo}, {exp_lo}",
        "ccmp {cur_hi}, {exp_hi}, #0, eq",
        "b.ne 2f",
        "stlxp {status:w}, {new_lo}, {new_hi}, [{ptr}]",
        "cbnz {status:w}, 1b",
        "b 3f",
        "2:",
        "clrex",
        "3:",
        ptr = in(reg) ptr,
        exp_lo = in(reg) exp_lo,
        exp_hi = in(reg) exp_hi,
        new_lo = in(reg) new_lo,
        new_hi = in(reg) new_hi,
        cur_lo = out(reg) cur_lo,
        cur_hi = out(reg) cur_hi,
        status = out(reg) _,
        options(nostack),
    );
    if cur_lo == exp_lo && cur_hi == exp_hi {
        Ok(())
    } else {
        Err((cur_lo, cur_hi))
    }
}

#[cfg(target_arch = "aarch64")]
#[inline]
unsafe fn load128_acquire(h: *const SListHeader) -> (u64, u64) {
    let p = h as *const AtomicU64;
    let hi = (*p.add(1)).load(Acquire);
    let lo = (*p).load(Acquire);
    (lo, hi)
}

#[inline]
unsafe fn read_slist_unaligned(h: *const SListHeader) -> (u64, u64) {
    let p = h as *const u64;
    (std::ptr::read_unaligned(p), std::ptr::read_unaligned(p.add(1)))
}

#[inline]
unsafe fn write_slist_unaligned(h: *mut SListHeader, lo: u64, hi: u64) {
    let p = h as *mut u64;
    std::ptr::write_unaligned(p, lo);
    std::ptr::write_unaligned(p.add(1), hi);
}

pub unsafe extern "win64" fn InitializeSListHead(h: *mut SListHeader) {
    if (h as usize) & 15 == 0 {
        let p = h as *const AtomicU64;
        (*p).store(0, Release);
        (*p.add(1)).store(0, Release);
    } else {
        let _g = unaligned_lock(h as usize);
        write_slist_unaligned(h, 0, 0);
    }
}

pub unsafe extern "win64" fn InterlockedPushEntrySList(h: *mut SListHeader, e: *mut *mut c_void) -> *mut c_void {
    #[cfg(target_arch = "aarch64")]
    if (h as usize) & 15 == 0 {
        let (mut lo, mut hi) = load128_acquire(h);
        loop {
            let old = slist_next(hi);
            std::ptr::write_unaligned(e, old as *mut c_void);
            let (new_lo, new_hi) = slist_pack(e, slist_depth(lo).wrapping_add(1), slist_seq(lo).wrapping_add(1));
            match cas128(h as *mut u64, lo, hi, new_lo, new_hi) {
                Ok(()) => return old as *mut c_void,
                Err((c_lo, c_hi)) => {
                    lo = c_lo;
                    hi = c_hi;
                }
            }
        }
    }
    let _g = unaligned_lock(h as usize);
    let (lo, hi) = read_slist_unaligned(h);
    let old = slist_next(hi);
    std::ptr::write_unaligned(e, old as *mut c_void);
    let (new_lo, new_hi) = slist_pack(e, slist_depth(lo).wrapping_add(1), slist_seq(lo).wrapping_add(1));
    write_slist_unaligned(h, new_lo, new_hi);
    old as *mut c_void
}

pub unsafe extern "win64" fn InterlockedPushListSListEx(
    h: *mut SListHeader,
    first: *mut *mut c_void,
    last: *mut *mut c_void,
    count: u32,
) -> *mut c_void {
    #[cfg(target_arch = "aarch64")]
    if (h as usize) & 15 == 0 {
        let (mut lo, mut hi) = load128_acquire(h);
        loop {
            let old = slist_next(hi);
            std::ptr::write_unaligned(last, old as *mut c_void);
            let (new_lo, new_hi) =
                slist_pack(first, slist_depth(lo).wrapping_add(count as u16), slist_seq(lo).wrapping_add(1));
            match cas128(h as *mut u64, lo, hi, new_lo, new_hi) {
                Ok(()) => return old as *mut c_void,
                Err((c_lo, c_hi)) => {
                    lo = c_lo;
                    hi = c_hi;
                }
            }
        }
    }
    let _g = unaligned_lock(h as usize);
    let (lo, hi) = read_slist_unaligned(h);
    let old = slist_next(hi);
    std::ptr::write_unaligned(last, old as *mut c_void);
    let (new_lo, new_hi) =
        slist_pack(first, slist_depth(lo).wrapping_add(count as u16), slist_seq(lo).wrapping_add(1));
    write_slist_unaligned(h, new_lo, new_hi);
    old as *mut c_void
}

pub unsafe extern "win64" fn InterlockedPopEntrySList(h: *mut SListHeader) -> *mut c_void {
    #[cfg(target_arch = "aarch64")]
    if (h as usize) & 15 == 0 {
        let (mut lo, mut hi) = load128_acquire(h);
        loop {
            let first = slist_next(hi);
            if first.is_null() {
                return std::ptr::null_mut();
            }
            let next = std::ptr::read_unaligned(first) as *mut *mut c_void;
            let (new_lo, new_hi) = slist_pack(next, slist_depth(lo).wrapping_sub(1), slist_seq(lo).wrapping_add(1));
            match cas128(h as *mut u64, lo, hi, new_lo, new_hi) {
                Ok(()) => return first as *mut c_void,
                Err((c_lo, c_hi)) => {
                    lo = c_lo;
                    hi = c_hi;
                }
            }
        }
    }
    let _g = unaligned_lock(h as usize);
    let (lo, hi) = read_slist_unaligned(h);
    let first = slist_next(hi);
    if first.is_null() {
        return std::ptr::null_mut();
    }
    let next = std::ptr::read_unaligned(first) as *mut *mut c_void;
    let (new_lo, new_hi) = slist_pack(next, slist_depth(lo).wrapping_sub(1), slist_seq(lo).wrapping_add(1));
    write_slist_unaligned(h, new_lo, new_hi);
    first as *mut c_void
}

pub unsafe extern "win64" fn InterlockedFlushSList(h: *mut SListHeader) -> *mut c_void {
    #[cfg(target_arch = "aarch64")]
    if (h as usize) & 15 == 0 {
        let (mut lo, mut hi) = load128_acquire(h);
        loop {
            let first = slist_next(hi);
            if first.is_null() {
                return std::ptr::null_mut();
            }
            let (new_lo, new_hi) = slist_pack(std::ptr::null_mut(), 0, slist_seq(lo).wrapping_add(1));
            match cas128(h as *mut u64, lo, hi, new_lo, new_hi) {
                Ok(()) => return first as *mut c_void,
                Err((c_lo, c_hi)) => {
                    lo = c_lo;
                    hi = c_hi;
                }
            }
        }
    }
    let _g = unaligned_lock(h as usize);
    let (lo, hi) = read_slist_unaligned(h);
    let first = slist_next(hi);
    let (new_lo, new_hi) = slist_pack(std::ptr::null_mut(), 0, slist_seq(lo).wrapping_add(1));
    write_slist_unaligned(h, new_lo, new_hi);
    first as *mut c_void
}

pub unsafe extern "win64" fn QueryDepthSList(h: *mut SListHeader) -> u16 {
    if (h as usize) & 15 == 0 {
        slist_depth((*(h as *const AtomicU64)).load(Acquire))
    } else {
        let _g = unaligned_lock(h as usize);
        slist_depth(read_slist_unaligned(h).0)
    }
}

// ---------------------------------------------------------------------------
// Shared host-side reentrant mutex (`ReentrantLock`) for subsystem DLLs
// ---------------------------------------------------------------------------

/// Reentrant host mutex keyed by `std::thread::ThreadId`, usable directly in
/// `static` items (`const fn new()`). Shared by `mfplat.rs` (`IMFAttributes`
/// `LockStore`/`UnlockStore`) and `bink2w64.rs` (`BinkUtilMutex`).
#[allow(dead_code)]
pub(crate) struct ReentrantLock {
    state: std::sync::Mutex<(Option<std::thread::ThreadId>, u32)>,
    cv: std::sync::Condvar,
}

#[allow(dead_code)]
impl ReentrantLock {
    pub(crate) const fn new() -> Self {
        Self {
            state: std::sync::Mutex::new((None, 0)),
            cv: std::sync::Condvar::new(),
        }
    }

    pub(crate) fn lock(&self) {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        while let Some(owner) = st.0 {
            if owner == me {
                st.1 = st.1.saturating_add(1);
                return;
            }
            st = self.cv.wait(st).unwrap_or_else(|e| e.into_inner());
        }
        st.0 = Some(me);
        st.1 = 1;
    }

    pub(crate) fn try_lock(&self) -> bool {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        match st.0 {
            None => {
                st.0 = Some(me);
                st.1 = 1;
                true
            }
            Some(owner) if owner == me => {
                st.1 = st.1.saturating_add(1);
                true
            }
            Some(_) => false,
        }
    }

    pub(crate) fn lock_timeout(&self, timeout: Duration) -> bool {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if st.0 == Some(me) {
            st.1 = st.1.saturating_add(1);
            return true;
        }
        if st.0.is_none() {
            st.0 = Some(me);
            st.1 = 1;
            return true;
        }
        if timeout.is_zero() {
            return false;
        }
        let deadline = Instant::now() + timeout;
        while st.0.is_some() {
            let Some(rem) = deadline.checked_duration_since(Instant::now()) else {
                return false;
            };
            let (next_st, res) = self
                .cv
                .wait_timeout(st, rem)
                .unwrap_or_else(|e| e.into_inner());
            st = next_st;
            if st.0.is_none() {
                break;
            }
            if res.timed_out() {
                return false;
            }
        }
        st.0 = Some(me);
        st.1 = 1;
        true
    }

    /// Release one recursion level. Returns `true` if the current thread owned the lock.
    pub(crate) fn unlock(&self) -> bool {
        let me = std::thread::current().id();
        let mut st = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if st.0 != Some(me) || st.1 == 0 {
            return false;
        }
        st.1 -= 1;
        if st.1 == 0 {
            st.0 = None;
            self.cv.notify_one();
        }
        true
    }

    /// Execute `f` while holding this reentrant lock, unlocking on exit/unwind.
    pub(crate) fn with_lock<R>(&self, f: impl FnOnce() -> R) -> R {
        self.lock();
        struct Guard<'a>(&'a ReentrantLock);
        impl Drop for Guard<'_> {
            fn drop(&mut self) {
                self.0.unlock();
            }
        }
        let _g = Guard(self);
        f()
    }
}

impl Default for ReentrantLock {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicU32;
    use std::sync::Arc;

    #[test]
    fn critical_section_contention() {
        struct Shared(std::cell::UnsafeCell<(CriticalSection, u64)>);
        unsafe impl Sync for Shared {}
        unsafe impl Send for Shared {}
        let s = Arc::new(Shared(std::cell::UnsafeCell::new((unsafe { std::mem::zeroed() }, 0))));
        unsafe { InitializeCriticalSection(&mut (*s.0.get()).0) };
        let hs: Vec<_> = (0..8)
            .map(|_| {
                let s = s.clone();
                std::thread::spawn(move || unsafe {
                    let p = s.0.get();
                    for _ in 0..10_000 {
                        EnterCriticalSection(&mut (*p).0);
                        EnterCriticalSection(&mut (*p).0); // recursion
                        (*p).1 += 1;
                        LeaveCriticalSection(&mut (*p).0);
                        LeaveCriticalSection(&mut (*p).0);
                    }
                    crate::teb::uninstall();
                })
            })
            .collect();
        for h in hs {
            h.join().unwrap();
        }
        assert_eq!(unsafe { (*s.0.get()).1 }, 80_000);
    }

    #[test]
    fn srw_and_cv() {
        let lock = Box::leak(Box::new(AtomicUsize::new(0))) as *mut AtomicUsize as usize;
        let cv = Box::leak(Box::new(AtomicUsize::new(0))) as *mut AtomicUsize as usize;
        let flag = Arc::new(AtomicU32::new(0));
        let f2 = flag.clone();
        let t = std::thread::spawn(move || unsafe {
            AcquireSRWLockExclusive(lock as _);
            while f2.load(SeqCst) == 0 {
                SleepConditionVariableSRW(cv as _, lock as _, INFINITE, 0);
            }
            ReleaseSRWLockExclusive(lock as _);
            crate::teb::uninstall();
        });
        std::thread::sleep(Duration::from_millis(20));
        unsafe {
            AcquireSRWLockExclusive(lock as _);
            flag.store(1, SeqCst);
            ReleaseSRWLockExclusive(lock as _);
            WakeAllConditionVariable(cv as _);
        }
        t.join().unwrap();
    }

    #[test]
    fn srw_mixed_exclusive_and_shared_contention() {
        struct Shared(std::cell::UnsafeCell<(AtomicUsize, u64)>);
        unsafe impl Sync for Shared {}
        unsafe impl Send for Shared {}
        let s = Arc::new(Shared(std::cell::UnsafeCell::new((AtomicUsize::new(0), 0))));
        let writers: Vec<_> = (0..4)
            .map(|_| {
                let s = s.clone();
                std::thread::spawn(move || unsafe {
                    let p = s.0.get();
                    for _ in 0..2000 {
                        AcquireSRWLockExclusive(&(*p).0 as *const AtomicUsize as *mut c_void);
                        (*p).1 += 1;
                        ReleaseSRWLockExclusive(&(*p).0 as *const AtomicUsize as *mut c_void);
                    }
                    crate::teb::uninstall();
                })
            })
            .collect();
        let readers: Vec<_> = (0..4)
            .map(|_| {
                let s = s.clone();
                std::thread::spawn(move || unsafe {
                    let p = s.0.get();
                    let mut last = 0u64;
                    for _ in 0..4000 {
                        AcquireSRWLockShared(&(*p).0 as *const AtomicUsize as *mut c_void);
                        let cur = (*p).1;
                        assert!(cur >= last, "SRWLock reader observed non-monotonic value");
                        last = cur;
                        ReleaseSRWLockShared(&(*p).0 as *const AtomicUsize as *mut c_void);
                    }
                    crate::teb::uninstall();
                })
            })
            .collect();
        for h in writers.into_iter().chain(readers) {
            h.join().unwrap();
        }
        assert_eq!(unsafe { (*s.0.get()).1 }, 8000);
        assert_eq!(unsafe { (*s.0.get()).0.load(SeqCst) }, 0, "SRWLock state must return to 0 with SRW_PARKED cleared");
    }

    #[test]
    fn slist() {
        unsafe {
            let mut h = SListHeader { lo: 0, hi: 0 };
            #[repr(C, align(16))]
            struct E(*mut c_void, u64);
            let mut a = E(std::ptr::null_mut(), 1);
            let mut b = E(std::ptr::null_mut(), 2);
            InterlockedPushEntrySList(&mut h, &mut a as *mut E as _);
            InterlockedPushEntrySList(&mut h, &mut b as *mut E as _);
            assert_eq!(QueryDepthSList(&mut h), 2);
            assert_eq!(InterlockedPopEntrySList(&mut h), &mut b as *mut E as *mut c_void);
            assert_eq!(InterlockedPopEntrySList(&mut h), &mut a as *mut E as *mut c_void);
            assert!(InterlockedPopEntrySList(&mut h).is_null());
        }
    }

    #[test]
    fn unaligned_critical_section_mutual_exclusion() {
        #[repr(align(16))]
        struct Buf([u8; 64]);
        struct Shared(std::cell::UnsafeCell<(Buf, Buf, u64, u32)>);
        unsafe impl Sync for Shared {}
        unsafe impl Send for Shared {}

        let s = Arc::new(Shared(std::cell::UnsafeCell::new((Buf([0; 64]), Buf([0; 64]), 0, 0))));
        unsafe {
            let p = s.0.get();
            let cs = (*p).0.0.as_mut_ptr().add(1) as *mut CriticalSection;
            assert_ne!((cs as usize) & 7, 0);
            InitializeCriticalSection(cs);
            let cv = (*p).1.0.as_mut_ptr().add(3) as *mut c_void;
            assert_ne!((cv as usize) & 7, 0);
            InitializeConditionVariable(cv);
        }

        // 1. Multi-threaded mutual exclusion + recursion on unaligned CRITICAL_SECTION.
        let hs: Vec<_> = (0..8)
            .map(|_| {
                let s = s.clone();
                std::thread::spawn(move || unsafe {
                    let p = s.0.get();
                    let cs = (*p).0.0.as_mut_ptr().add(1) as *mut CriticalSection;
                    let tid = crate::teb::thread_id() as usize;
                    for _ in 0..4_000 {
                        EnterCriticalSection(cs);
                        assert!(is_cs_locked_by_thread(cs, tid));
                        assert_eq!(TryEnterCriticalSection(cs), TRUE); // recursive
                        let v = (*p).2;
                        std::hint::spin_loop();
                        (*p).2 = v + 1;
                        LeaveCriticalSection(cs);
                        LeaveCriticalSection(cs);
                    }
                    crate::teb::uninstall();
                })
            })
            .collect();
        for h in hs {
            h.join().unwrap();
        }
        assert_eq!(unsafe { (*s.0.get()).2 }, 32_000);

        // 2. SleepConditionVariableCS with recursive unaligned CS + unaligned CV.
        let s2 = s.clone();
        let waiter = std::thread::spawn(move || unsafe {
            let p = s2.0.get();
            let cs = (*p).0.0.as_mut_ptr().add(1) as *mut CriticalSection;
            let cv = (*p).1.0.as_mut_ptr().add(3) as *mut c_void;
            EnterCriticalSection(cs);
            EnterCriticalSection(cs); // recursion depth = 2
            while (*p).3 == 0 {
                assert_eq!(SleepConditionVariableCS(cv, cs, INFINITE), TRUE);
            }
            assert_eq!(cs_get_recursion(cs), 2);
            LeaveCriticalSection(cs);
            LeaveCriticalSection(cs);
            crate::teb::uninstall();
        });
        std::thread::sleep(Duration::from_millis(15));
        unsafe {
            let p = s.0.get();
            let cs = (*p).0.0.as_mut_ptr().add(1) as *mut CriticalSection;
            let cv = (*p).1.0.as_mut_ptr().add(3) as *mut c_void;
            EnterCriticalSection(cs);
            (*p).3 = 1;
            LeaveCriticalSection(cs);
            WakeConditionVariable(cv);
        }
        waiter.join().unwrap();
    }

    #[test]
    fn srw_writer_priority_and_clean_state() {
        use std::sync::atomic::AtomicBool;
        let lock = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let writer_done = Arc::new(AtomicBool::new(false));

        // Continuous readers holding overlapping shared locks.
        let readers: Vec<_> = (0..4)
            .map(|_| {
                let l = lock.clone();
                let st = stop.clone();
                std::thread::spawn(move || unsafe {
                    let p = &*l as *const AtomicUsize as *mut c_void;
                    while !st.load(Relaxed) {
                        AcquireSRWLockShared(p);
                        std::thread::sleep(Duration::from_micros(200));
                        ReleaseSRWLockShared(p);
                    }
                    crate::teb::uninstall();
                })
            })
            .collect();

        std::thread::sleep(Duration::from_millis(10));
        let l_w = lock.clone();
        let wd = writer_done.clone();
        let writer = std::thread::spawn(move || unsafe {
            let p = &*l_w as *const AtomicUsize as *mut c_void;
            for _ in 0..20 {
                AcquireSRWLockExclusive(p);
                ReleaseSRWLockExclusive(p);
            }
            wd.store(true, Release);
            crate::teb::uninstall();
        });

        let deadline = Instant::now() + Duration::from_secs(5);
        while !writer_done.load(Acquire) {
            assert!(Instant::now() < deadline, "SRWLock writer starved by continuous readers");
            std::thread::sleep(Duration::from_millis(5));
        }
        stop.store(true, Release);
        writer.join().unwrap();
        for r in readers {
            r.join().unwrap();
        }
        assert_eq!(lock.load(SeqCst), 0);
    }

    #[test]
    fn cv_zero_waiter_wake_fast_path() {
        let mut cv: usize = 0;
        let p = &mut cv as *mut usize as *mut c_void;
        unsafe {
            InitializeConditionVariable(p);
            // Zero-waiter wakes must be a no-op without modifying the CV word.
            WakeConditionVariable(p);
            WakeAllConditionVariable(p);
            assert_eq!(cv, 0);
        }
    }

    #[test]
    fn slist_concurrent_push_pop_and_unaligned() {
        #[repr(C, align(16))]
        struct Node {
            _next: *mut c_void,
            _tag: usize,
        }
        struct SharedHeader(std::cell::UnsafeCell<SListHeader>);
        unsafe impl Sync for SharedHeader {}
        unsafe impl Send for SharedHeader {}

        let hdr = Arc::new(SharedHeader(std::cell::UnsafeCell::new(SListHeader { lo: 0, hi: 0 })));
        unsafe { InitializeSListHead(hdr.0.get()) };

        const THREADS: usize = 4;
        const PER_THREAD: usize = 512;
        let popped = Arc::new(AtomicUsize::new(0));
        // Keep all nodes alive until every thread finishes to avoid ABA use-after-free.
        let mut pools: Vec<Box<[Node]>> = (0..THREADS)
            .map(|t| {
                (0..PER_THREAD)
                    .map(|i| Node { _next: std::ptr::null_mut(), _tag: t * PER_THREAD + i })
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
            })
            .collect();
        let raw_pools: Vec<usize> = pools.iter_mut().map(|b| b.as_mut_ptr() as usize).collect();

        let hs: Vec<_> = raw_pools
            .into_iter()
            .map(|base| {
                let hdr = hdr.clone();
                let popped = popped.clone();
                std::thread::spawn(move || unsafe {
                    let h = hdr.0.get();
                    let nodes = base as *mut Node;
                    for i in 0..PER_THREAD {
                        let n = nodes.add(i);
                        InterlockedPushEntrySList(h, n as *mut *mut c_void);
                        if i % 2 == 1 && !InterlockedPopEntrySList(h).is_null() {
                            popped.fetch_add(1, Relaxed);
                        }
                    }
                    while !InterlockedPopEntrySList(h).is_null() {
                        popped.fetch_add(1, Relaxed);
                    }
                })
            })
            .collect();
        for h in hs {
            h.join().unwrap();
        }
        assert_eq!(popped.load(Acquire), THREADS * PER_THREAD);
        assert_eq!(unsafe { QueryDepthSList(hdr.0.get()) }, 0);
        assert!(unsafe { InterlockedPopEntrySList(hdr.0.get()) }.is_null());
        drop(pools);

        // Unaligned SListHeader fallback path.
        #[repr(align(16))]
        struct Raw([u8; 32]);
        let mut raw = Raw([0u8; 32]);
        unsafe {
            let uh = raw.0.as_mut_ptr().add(1) as *mut SListHeader;
            assert_ne!((uh as usize) & 15, 0);
            InitializeSListHead(uh);
            let mut a = Node { _next: std::ptr::null_mut(), _tag: 1 };
            let mut b = Node { _next: std::ptr::null_mut(), _tag: 2 };
            InterlockedPushEntrySList(uh, &mut a as *mut Node as _);
            InterlockedPushEntrySList(uh, &mut b as *mut Node as _);
            assert_eq!(QueryDepthSList(uh), 2);
            assert_eq!(InterlockedPopEntrySList(uh), &mut b as *mut Node as *mut c_void);
            assert_eq!(InterlockedFlushSList(uh), &mut a as *mut Node as *mut c_void);
            assert_eq!(QueryDepthSList(uh), 0);
        }
    }

    #[test]
    fn reentrant_lock_recursion_timeout_and_contention() {
        let lock = Arc::new(ReentrantLock::new());
        assert!(!lock.unlock(), "unlocking unowned lock must return false");

        // Same-thread reentrancy via lock / try_lock / with_lock.
        lock.lock();
        assert!(lock.try_lock());
        let nested = lock.with_lock(|| 42u32);
        assert_eq!(nested, 42);

        // Other thread sees try_lock == false, lock_timeout == false, unlock == false.
        let l2 = lock.clone();
        let probe = std::thread::spawn(move || {
            assert!(!l2.try_lock());
            assert!(!l2.lock_timeout(Duration::from_millis(15)));
            assert!(!l2.unlock());
        });
        probe.join().unwrap();

        // Still requires 2 unlocks (one for lock(), one for try_lock()).
        assert!(lock.unlock());
        let l3 = lock.clone();
        let probe2 = std::thread::spawn(move || !l3.try_lock());
        assert!(probe2.join().unwrap(), "must remain held after 1 of 2 unlocks");
        assert!(lock.unlock());
        assert!(!lock.unlock());

        // Multi-threaded mutual exclusion + recursion stress.
        let counter = Arc::new(std::cell::UnsafeCell::new(0u64));
        struct SyncCounter(Arc<std::cell::UnsafeCell<u64>>);
        unsafe impl Send for SyncCounter {}
        unsafe impl Sync for SyncCounter {}
        let shared = Arc::new(SyncCounter(counter));

        let hs: Vec<_> = (0..6)
            .map(|_| {
                let l = lock.clone();
                let sc = shared.clone();
                std::thread::spawn(move || {
                    for _ in 0..1000 {
                        l.with_lock(|| {
                            l.with_lock(|| unsafe {
                                *sc.0.get() += 1;
                            });
                        });
                    }
                })
            })
            .collect();
        for h in hs {
            h.join().unwrap();
        }
        assert_eq!(unsafe { *shared.0.get() }, 6000);
    }

    #[test]
    fn init_once_async_parallel_race_and_check_only() {
        use std::sync::Barrier;

        unsafe {
            let mut once: usize = 0;
            let p = &mut once as *mut usize as *mut c_void;
            InitOnceInitialize(p);

            // INIT_ONCE_CHECK_ONLY before completion returns FALSE with ERROR_GEN_FAILURE.
            let mut pending: BOOL = TRUE;
            let mut ctx: *mut c_void = std::ptr::null_mut();
            assert_eq!(InitOnceBeginInitialize(p, INIT_ONCE_CHECK_ONLY, &mut pending, &mut ctx), FALSE);
            assert_eq!(pending, FALSE);
            assert_eq!(crate::teb::last_error(), err::ERROR_GEN_FAILURE);

            // Invalid flag combination: INIT_ONCE_CHECK_ONLY | INIT_ONCE_ASYNC.
            assert_eq!(
                InitOnceBeginInitialize(p, INIT_ONCE_CHECK_ONLY | INIT_ONCE_ASYNC, &mut pending, &mut ctx),
                FALSE
            );
            assert_eq!(crate::teb::last_error(), err::ERROR_INVALID_PARAMETER);

            // 4 threads concurrently call InitOnceBeginInitialize(INIT_ONCE_ASYNC) without blocking each other.
            // Exactly ONE thread wins InitOnceComplete(INIT_ONCE_ASYNC); the other 3 get FALSE.
            let shared_once = Arc::new(AtomicUsize::new(0));
            let barrier = Arc::new(Barrier::new(4));
            let wins = Arc::new(AtomicUsize::new(0));
            let losses = Arc::new(AtomicUsize::new(0));
            let hs: Vec<_> = (1usize..=4)
                .map(|id| {
                    let so = Arc::clone(&shared_once);
                    let b = Arc::clone(&barrier);
                    let w = Arc::clone(&wins);
                    let l = Arc::clone(&losses);
                    std::thread::spawn(move || {
                        let op = &*so as *const AtomicUsize as *mut c_void;
                        let mut pend: BOOL = FALSE;
                        let mut out_ctx: *mut c_void = std::ptr::null_mut();
                        assert_eq!(InitOnceBeginInitialize(op, INIT_ONCE_ASYNC, &mut pend, &mut out_ctx), TRUE);
                        assert_eq!(pend, TRUE, "all racing async threads must enter pending without blocking");
                        // Barrier proves all 4 threads entered INIT_ONCE_ASYNC simultaneously (would deadlock if treated as sync!).
                        b.wait();
                        let val = (id * 0x100) as *mut c_void;
                        if InitOnceComplete(op, INIT_ONCE_ASYNC, val) == TRUE {
                            w.fetch_add(1, AcqRel);
                        } else {
                            assert_eq!(crate::teb::last_error(), err::ERROR_GEN_FAILURE);
                            l.fetch_add(1, AcqRel);
                        }
                        crate::teb::uninstall();
                    })
                })
                .collect();
            for h in hs {
                h.join().unwrap();
            }
            assert_eq!(wins.load(Acquire), 1);
            assert_eq!(losses.load(Acquire), 3);

            // Subsequent INIT_ONCE_CHECK_ONLY succeeds and returns the winning context.
            let op = &*shared_once as *const AtomicUsize as *mut c_void;
            let mut final_pend: BOOL = TRUE;
            let mut final_ctx: *mut c_void = std::ptr::null_mut();
            assert_eq!(InitOnceBeginInitialize(op, INIT_ONCE_CHECK_ONLY, &mut final_pend, &mut final_ctx), TRUE);
            assert_eq!(final_pend, FALSE);
            let v = final_ctx as usize;
            assert!([0x100, 0x200, 0x300, 0x400].contains(&v), "unexpected winning ctx {v:#x}");
        }
    }
}
