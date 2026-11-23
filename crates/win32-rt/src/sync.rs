//! User-mode synchronization primitives whose state lives in Windows-owned
//! memory: CRITICAL_SECTION, SRWLOCK, CONDITION_VARIABLE, INIT_ONCE,
//! WaitOnAddress, SLIST. Blocking uses `parking_lot_core` keyed by address.

use crate::types::*;
use parking_lot_core::{park, unpark_all, unpark_one, ParkResult, DEFAULT_PARK_TOKEN, DEFAULT_UNPARK_TOKEN};
use std::ffi::c_void;
use std::sync::atomic::{AtomicI32, AtomicUsize, Ordering::*};
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
        let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let base = cs as *mut u8;
        let own = std::ptr::read_unaligned(base.add(0x10) as *const usize);
        if own == tid {
            let r = std::ptr::read_unaligned(base.add(0x0c) as *const i32);
            std::ptr::write_unaligned(base.add(0x0c) as *mut i32, r.wrapping_add(1));
            return;
        }
        std::ptr::write_unaligned(base.add(0x08) as *mut i32, 0);
        std::ptr::write_unaligned(base.add(0x10) as *mut usize, tid);
        std::ptr::write_unaligned(base.add(0x0c) as *mut i32, 1);
        return;
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
        let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
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
        let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let base = cs as *mut u8;
        let r = std::ptr::read_unaligned(base.add(0x0c) as *const i32);
        std::ptr::write_unaligned(base.add(0x0c) as *mut i32, r.wrapping_sub(1));
        if r != 1 {
            return;
        }
        std::ptr::write_unaligned(base.add(0x10) as *mut usize, 0);
        let lc = std::ptr::read_unaligned(base.add(0x08) as *const i32);
        std::ptr::write_unaligned(base.add(0x08) as *mut i32, -1);
        if lc == 1 {
            unpark_one(cs as usize, |_| DEFAULT_UNPARK_TOKEN);
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

// ---------------------------------------------------------------------------
// Alignment-safe pointer-sized atomic word helper (x86_64 allows unaligned
// SRWLOCK / CONDITION_VARIABLE / INIT_ONCE in byte buffers; arm64 hardware
// atomics fault on <8-byte alignment).
// ---------------------------------------------------------------------------

static UNALIGNED_SYNC_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

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
            let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            std::ptr::read_unaligned(self.0)
        }
    }
    #[inline]
    unsafe fn store(self, val: usize, order: std::sync::atomic::Ordering) {
        if (self.0 as usize) & 7 == 0 {
            (&*(self.0 as *const AtomicUsize)).store(val, order);
        } else {
            let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
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
            let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
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
            let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
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
            let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
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
            let _g = UNALIGNED_SYNC_LOCK.lock().unwrap_or_else(|e| e.into_inner());
            let prev = std::ptr::read_unaligned(self.0);
            std::ptr::write_unaligned(self.0, prev.wrapping_sub(val));
            prev
        }
    }
}

// ---------------------------------------------------------------------------
// SRWLOCK (pointer-sized): bit0 writer, bit1 parked, readers counted from bit 2.
// ---------------------------------------------------------------------------

const SRW_WRITER: usize = 1;
const SRW_PARKED: usize = 2;
const SRW_READER: usize = 4;
const SRW_READERS_MASK: usize = !3;

#[inline]
unsafe fn srw(p: *mut c_void) -> AtomicWord {
    AtomicWord::from_ptr(p)
}

unsafe fn srw_park(lock: AtomicWord, blocked: impl Fn(usize) -> bool) {
    let s = lock.load(Relaxed);
    if !blocked(s) {
        return;
    }
    if s & SRW_PARKED == 0 {
        if lock.compare_exchange(s, s | SRW_PARKED, Relaxed, Relaxed).is_err() {
            return;
        }
    }
    let key = lock.addr();
    let _scope = crate::hitch::Scope::enter(crate::hitch::ActKind::LockWait, key as u64);
    park(
        key,
        || {
            let s = lock.load(Relaxed);
            s & SRW_PARKED != 0 && blocked(s)
        },
        || {},
        |_, _| {},
        DEFAULT_PARK_TOKEN,
        None,
    );
}

#[cold]
unsafe fn srw_wake(lock: AtomicWord) {
    if lock.fetch_and(!SRW_PARKED, Relaxed) & SRW_PARKED != 0 {
        unpark_all(lock.addr(), DEFAULT_UNPARK_TOKEN);
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
    loop {
        let s = l.load(Relaxed);
        if s & (SRW_WRITER | SRW_READERS_MASK) == 0 {
            if l.compare_exchange_weak(s, s | SRW_WRITER, Acquire, Relaxed).is_ok() {
                return;
            }
            continue;
        }
        if spins < 100 {
            spins += 1;
            std::hint::spin_loop();
            continue;
        }
        srw_park(l, |s| s & (SRW_WRITER | SRW_READERS_MASK) != 0);
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
    while s & SRW_WRITER == 0 {
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
    loop {
        let s = l.load(Relaxed);
        if s & SRW_WRITER == 0 {
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
        srw_park(l, |s| s & SRW_WRITER != 0);
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
// CONDITION_VARIABLE (pointer-sized sequence counter).
// ---------------------------------------------------------------------------

unsafe fn cv_wait(cv: *mut c_void, ms: u32, release: impl FnOnce(), reacquire: impl FnOnce()) -> BOOL {
    let seq = AtomicWord::from_ptr(cv);
    let s0 = seq.load(Acquire);
    release();
    let r = park(cv as usize, || seq.load(Relaxed) == s0, || {}, |_, _| {}, DEFAULT_PARK_TOKEN, deadline(ms));
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
    // Windows releases the CS fully (it must be held exactly once).
    let rec = (*cs).recursion_count.load(Relaxed);
    cv_wait(
        cv,
        ms,
        || {
            (*cs).recursion_count.store(1, Relaxed);
            LeaveCriticalSection(cs)
        },
        || {
            EnterCriticalSection(cs);
            (*cs).recursion_count.store(rec, Relaxed);
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
    AtomicWord::from_ptr(cv).fetch_add(1, Release);
    unpark_one(cv as usize, |_| DEFAULT_UNPARK_TOKEN);
}

#[inline(never)]
pub unsafe extern "win64" fn WakeAllConditionVariable(cv: *mut c_void) {
    AtomicWord::from_ptr(cv).fetch_add(1, Release);
    unpark_all(cv as usize, DEFAULT_UNPARK_TOKEN);
}

// ---------------------------------------------------------------------------
// INIT_ONCE (pointer-sized): low 2 bits state (0 new, 1 running, 2 done),
// upper bits = context.
// ---------------------------------------------------------------------------

const INIT_ONCE_CHECK_ONLY: u32 = 1;
const INIT_ONCE_ASYNC: u32 = 2;
const INIT_ONCE_INIT_FAILED: u32 = 4;

pub type InitOnceFn = unsafe extern "win64" fn(*mut c_void, *mut c_void, *mut *mut c_void) -> BOOL;

pub unsafe extern "win64" fn InitOnceInitialize(once: *mut c_void) {
    AtomicWord::from_ptr(once).store(0, Relaxed);
}

pub unsafe extern "win64" fn InitOnceBeginInitialize(once: *mut c_void, flags: u32, pending: *mut BOOL, ctx: *mut *mut c_void) -> BOOL {
    let a = AtomicWord::from_ptr(once);
    if flags & INIT_ONCE_ASYNC != 0 {
        stub!("INIT_ONCE_ASYNC treated as synchronous");
    }
    loop {
        let v = a.load(Acquire);
        match v & 3 {
            2 => {
                std::ptr::write_unaligned(pending, FALSE);
                if !ctx.is_null() {
                    std::ptr::write_unaligned(ctx, (v & !3) as *mut c_void);
                }
                return TRUE;
            }
            0 => {
                if flags & INIT_ONCE_CHECK_ONLY != 0 {
                    std::ptr::write_unaligned(pending, TRUE);
                    return TRUE;
                }
                if a.compare_exchange(0, 1, Acquire, Relaxed).is_ok() {
                    std::ptr::write_unaligned(pending, TRUE);
                    return TRUE;
                }
            }
            _ => {
                if flags & INIT_ONCE_CHECK_ONLY != 0 {
                    std::ptr::write_unaligned(pending, TRUE);
                    return TRUE;
                }
                park(once as usize, || a.load(Relaxed) & 3 == 1, || {}, |_, _| {}, DEFAULT_PARK_TOKEN, None);
            }
        }
    }
}

pub unsafe extern "win64" fn InitOnceComplete(once: *mut c_void, flags: u32, ctx: *mut c_void) -> BOOL {
    let a = AtomicWord::from_ptr(once);
    if flags & INIT_ONCE_INIT_FAILED != 0 {
        a.store(0, Release);
    } else {
        a.store((ctx as usize & !3) | 2, Release);
    }
    unpark_all(once as usize, DEFAULT_UNPARK_TOKEN);
    TRUE
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
        InitOnceComplete(once, 0, c);
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
// Implemented with a global lock (layout-faithful, not lock-free).
// ---------------------------------------------------------------------------

static SLIST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[repr(C, align(16))]
pub struct SListHeader {
    lo: u64,
    hi: u64,
}

impl SListHeader {
    fn depth(&self) -> u16 {
        self.lo as u16
    }
    fn seq(&self) -> u64 {
        self.lo >> 16
    }
    fn next(&self) -> *mut *mut c_void {
        ((self.hi >> 4) << 4) as *mut *mut c_void
    }
    fn set(&mut self, next: *mut *mut c_void, depth: u16, seq: u64) {
        self.lo = depth as u64 | (seq << 16);
        self.hi = (next as u64 >> 4) << 4;
    }
}

pub unsafe extern "win64" fn InitializeSListHead(h: *mut SListHeader) {
    (*h).lo = 0;
    (*h).hi = 0;
}

pub unsafe extern "win64" fn InterlockedPushEntrySList(h: *mut SListHeader, e: *mut *mut c_void) -> *mut c_void {
    let _g = SLIST_LOCK.lock().unwrap();
    let old = (*h).next();
    *e = old as *mut c_void;
    let (d, s) = ((*h).depth(), (*h).seq());
    (*h).set(e, d.wrapping_add(1), s.wrapping_add(1));
    old as *mut c_void
}

pub unsafe extern "win64" fn InterlockedPushListSListEx(h: *mut SListHeader, first: *mut *mut c_void, last: *mut *mut c_void, count: u32) -> *mut c_void {
    let _g = SLIST_LOCK.lock().unwrap();
    let old = (*h).next();
    *last = old as *mut c_void;
    let (d, s) = ((*h).depth(), (*h).seq());
    (*h).set(first, d.wrapping_add(count as u16), s.wrapping_add(1));
    old as *mut c_void
}

pub unsafe extern "win64" fn InterlockedPopEntrySList(h: *mut SListHeader) -> *mut c_void {
    let _g = SLIST_LOCK.lock().unwrap();
    let first = (*h).next();
    if first.is_null() {
        return std::ptr::null_mut();
    }
    let next = *first as *mut *mut c_void;
    let (d, s) = ((*h).depth(), (*h).seq());
    (*h).set(next, d.wrapping_sub(1), s.wrapping_add(1));
    first as *mut c_void
}

pub unsafe extern "win64" fn InterlockedFlushSList(h: *mut SListHeader) -> *mut c_void {
    let _g = SLIST_LOCK.lock().unwrap();
    let first = (*h).next();
    let s = (*h).seq();
    (*h).set(std::ptr::null_mut(), 0, s.wrapping_add(1));
    first as *mut c_void
}

pub unsafe extern "win64" fn QueryDepthSList(h: *mut SListHeader) -> u16 {
    (*h).depth()
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
}
