//! Per-thread LIFO scratch allocator (`r::`, names inferred).
//!
//! 0x14241bbe0 returns the calling thread's state block. It's created lazily
//! and kept in a TLS slot at `ThreadLocalStoragePointer[0] + 0x6368`. The
//! fields used here (offsets into the state):
//!
//! | off  | type  | use |
//! |------|-------|-----|
//! | 0x08 | ptr   | block pool. `&state+8` goes to the acquire routine, `[state+8]` to the release routine |
//! | 0x10 | ptr   | current (top) block; the next block is linked via `block+0x28` (low 48 bits) |
//! | 0x18 | u64   | block-size threshold ("big block" when `top - base` exceeds it) |
//! | 0x20 | i32   | bytes in use |
//! | 0x28 | i32   | peak bytes in use |
//! | 0x2c | u32   | decremented by the low 32 bits of `+0x18` on every block release |
//!
//! A block has `base` at +0, `bottom` at +8, `top` at +0x18, the most recent
//! allocation header at +0x20 and the next-block link at +0x28. Each
//! allocation has a 16-byte header just before the returned pointer:
//! `u32` tag (bit 31 preserved, the rest set to 0x0aaaaaaa on allocation and
//! cleared on free), a live bit at byte +4, and the previous header's address
//! at +8. Freeing marks the header dead, then pops trailing dead headers off
//! the top block, and hands emptied (or oversized) blocks back to the pool.
//!
//! 0x14241c0a0 has 562 callers and 0x14241c190 has 2484. The pool's acquire
//! routine 0x1424b3910 and release routine 0x1424b3cb0 are not decompiled.
//! Their only throwing path is `std::_Xbad_function_call` on an empty pool
//! callback, which is a contract violation and not a recoverable error.

use crate::exe;

pub(crate) const STATE_VA: u64 = 0x1_4241_bbe0;
pub(crate) const ACQUIRE_VA: u64 = 0x1_424b_3910;
pub(crate) const RELEASE_VA: u64 = 0x1_424b_3cb0;

const LINK_MASK: usize = 0xffff_ffff_ffff;

#[inline(always)]
unsafe fn q(a: usize) -> usize {
    (a as *const usize).read_unaligned()
}
#[inline(always)]
unsafe fn set_q(a: usize, v: usize) {
    (a as *mut usize).write_unaligned(v)
}
#[inline(always)]
unsafe fn d(a: usize) -> u32 {
    (a as *const u32).read_unaligned()
}
#[inline(always)]
unsafe fn set_d(a: usize, v: u32) {
    (a as *mut u32).write_unaligned(v)
}
#[inline(always)]
unsafe fn add_i32(a: usize, v: u32) {
    set_d(a, d(a).wrapping_add(v));
}

/// Allocate `size` bytes aligned to `max(align, 16)` from the calling
/// thread's scratch stack. Returns null if the pool can't supply a block.
///
/// Original: 0x14241c0a0 (219 bytes).
pub unsafe extern "win64" fn scratch_alloc(size: usize, align: usize) -> usize {
    let align = align.max(16);
    let st = exe::call_va!(STATE_VA) as usize;
    let mut need = size.wrapping_add(16);
    if need > q(st + 0x18) {
        need = size.wrapping_add(align);
    }
    let blk = exe::call_va!(ACQUIRE_VA, st + 8, need, align) as usize;
    if blk == 0 {
        return 0;
    }
    let top = q(blk + 0x18);
    let user = align.wrapping_add(0xf).wrapping_add(top) & !align.wrapping_sub(1);
    let hdr = user.wrapping_sub(0x10);
    if q(blk + 8) == top {
        set_q(blk + 8, hdr);
        set_q(blk + 0x18, hdr);
    }
    // The header is only 16-aligned for power-of-two alignments.
    set_d(hdr, (d(hdr) & 0x8aaa_aaaa) | 0x0aaa_aaaa);
    *((hdr + 4) as *mut u8) |= 1;
    set_q(hdr + 8, q(blk + 0x20));
    let bottom = q(blk + 8);
    let used_before = q(blk + 0x18).wrapping_sub(bottom);
    set_q(blk + 0x20, hdr);
    let new_top = user.wrapping_add(size);
    set_q(blk + 0x18, new_top);
    let delta = (new_top as u32).wrapping_sub(bottom as u32).wrapping_sub(used_before as u32);
    add_i32(st + 0x20, delta);
    let used = d(st + 0x20) as i32;
    set_d(st + 0x28, (d(st + 0x28) as i32).max(used) as u32);
    user
}

/// Free a pointer returned by [`scratch_alloc`]. Only the top allocation
/// actually releases memory; freeing anything else just marks its header
/// dead, so it gets popped later when everything above it has been freed.
///
/// Original: 0x14241c190 (242 bytes, 2 fragments). One deviation: the
/// original reads `[current_block + 0x20]` before it checks the block for
/// null, so it faults when there's no current block. This version returns
/// instead.
pub unsafe extern "win64" fn scratch_free(p: *mut u8) {
    let st = exe::call_va!(STATE_VA) as usize;
    *p.wrapping_sub(0xc) &= !1;
    let hdr = (p as usize).wrapping_sub(0x10);
    set_d(hdr, d(hdr) & 0x8000_0000);
    let mut blk = q(st + 0x10);
    if blk == 0 || q(blk + 0x20) != hdr {
        return;
    }
    loop {
        let head = q(blk + 0x20);
        let mut release = false;
        if *((head + 4) as *const u8) & 1 == 0 {
            let top = q(blk + 0x18);
            if top.wrapping_sub(q(blk)) > q(st + 0x18) {
                // Oversized block: give it back without popping.
                add_i32(st + 0x20, (q(blk + 8) as u32).wrapping_sub(top as u32));
                release = true;
            } else {
                let used_before = top.wrapping_sub(q(blk + 8)) as u32;
                let mut cur = head;
                let mut prev_lo = top as u32;
                let mut new_lo;
                loop {
                    new_lo = prev_lo;
                    if *((cur + 4) as *const u8) & 1 != 0 {
                        break;
                    }
                    new_lo = cur as u32;
                    set_q(blk + 0x18, cur);
                    cur = q(cur + 8);
                    prev_lo = new_lo;
                    if cur == 0 {
                        break;
                    }
                }
                let delta = new_lo.wrapping_sub(q(blk + 8) as u32).wrapping_sub(used_before);
                set_q(blk + 0x20, cur);
                add_i32(st + 0x20, delta);
            }
        }
        if !release && q(blk + 8) != q(blk + 0x18) {
            return;
        }
        let done = q(st + 0x10);
        if done != 0 {
            set_q(st + 0x10, q(done + 0x28) & LINK_MASK);
        }
        let threshold_lo = d(st + 0x18);
        let _ = exe::call_va!(RELEASE_VA, q(st + 8), done);
        set_d(st + 0x2c, d(st + 0x2c).wrapping_sub(threshold_lo));
        blk = q(st + 0x10);
        if blk == 0 {
            return;
        }
    }
}

/// A `{ u16* data, u32 len, u32 cap }` array that lives in the scratch
/// allocator. The element type is from the 2-byte stride and alignment; the
/// container's real name is unknown.
#[repr(C, packed)]
pub struct ScratchArrayU16 {
    pub data: *mut u16,
    pub len: u32,
    pub cap: u32,
}

pub(crate) const ARRAY_U16_REALLOC_VA: u64 = 0x1_40fb_3a60;
const SCRATCH_ALLOC_VA: u64 = 0x1_4241_c0a0;
const SCRATCH_FREE_VA: u64 = 0x1_4241_c190;
const J_MEMMOVE_VA: u64 = 0x1_4183_acc8;

/// Reallocate to `cap` elements: `scratch_alloc(cap * 2, 2)`, move `len`
/// elements over, free the old buffer if there was one, then set
/// `cap = (u32)cap`. `len` keeps the value read on entry. The alloc result
/// isn't null-checked.
///
/// Original: 0x140fb3a60 (107 bytes, 15 callers).
pub unsafe extern "win64" fn array_u16_reallocate(a: *mut ScratchArrayU16, cap: u64) {
    let len = (*a).len;
    let dst = exe::call_va!(SCRATCH_ALLOC_VA, (cap as usize).wrapping_add(cap as usize), 2usize) as usize;
    let _ = exe::call_va!(J_MEMMOVE_VA, dst as *mut u8, (*a).data as *const u8, (*a).len as usize * 2);
    if !(*a).data.is_null() {
        let _ = exe::call_va!(SCRATCH_FREE_VA, (*a).data as usize);
    }
    (*a).data = dst as *mut u16;
    (*a).len = len;
    (*a).cap = cap as u32;
}

/// `push_back(const u16&)`. When full, grows to `max(cap + cap / 2, len + 1)`
/// through 0x140fb3a60.
///
/// Original: 0x140faef90 (93 bytes).
pub unsafe extern "win64" fn array_u16_push_back(a: *mut ScratchArrayU16, v: *const u16) {
    let (len, cap) = ((*a).len as u64, (*a).cap as u64);
    if len * 2 == cap * 2 {
        let grown = (cap >> 1) + cap;
        let _ = exe::call_va!(ARRAY_U16_REALLOC_VA, a, grown.max(len + 1));
    }
    exe::wr::<u16>((*a).data.add((*a).len as usize) as usize, exe::rd::<u16>(v as usize));
    (*a).len = (*a).len.wrapping_add(1);
}

// ---------------------------------------------------------------------------
// Per-thread state getter.
// ---------------------------------------------------------------------------

/// Offset of the state pointer in the exe's TLS block (`ThreadLocalStoragePointer[0]`).
const STATE_TLS_OFFSET: usize = 0x6368;
/// Creates a fresh state into a `unique_ptr`-like out slot (not ported; the
/// state's memory comes from mimalloc).
const MAKE_STATE_VA: u64 = 0x1_4139_dd10;
/// Spin backoff `(state: &mut u64)`, which yields after a number of spins (not ported).
const BACKOFF_VA: u64 = 0x1_4241_7470;
/// Grows the global state registry to `new_cap` entries. `rcx` is dead in
/// this LTCG-specialized function; it works on the globals below.
const REGISTRY_GROW_VA: u64 = 0x1_4139_df30;
const REGISTRY_LOCK_VA: u64 = 0x1_43a2_8df7;
const REGISTRY_DATA_VA: u64 = 0x1_43c1_6448;
const REGISTRY_COUNT_VA: u64 = 0x1_43c1_6450;
const REGISTRY_CAP_VA: u64 = 0x1_43c1_6454;
/// IAT slot of `KERNEL32!GetCurrentThreadId`.
const GET_CURRENT_THREAD_ID_IAT: u64 = 0x1_4264_5790;

/// Mutable pointer to exe data at `va`.
#[inline(always)]
fn gptr<T>(va: u64) -> *mut T {
    exe::ptr::<T>(va) as *mut T
}

/// `gs:[0x58]`, the TEB's ThreadLocalStoragePointer, read the same way as the original.
#[inline(always)]
unsafe fn tls_pointer() -> usize {
    exe::tls_pointer()
}

#[inline(always)]
unsafe fn state_slot() -> usize {
    exe::rd::<usize>(tls_pointer()).wrapping_add(STATE_TLS_OFFSET)
}

/// Get the calling thread's scratch-allocator state, creating it on first use.
///
/// On creation: make a fresh state (0x14139dd10), take the global registry
/// spinlock (test-and-test-and-set with 0x142417470 backoff), push the state
/// onto the registry vector (`{data, u32 count, u32 cap}`, which grows to
/// `max(cap + cap / 2, count + 1)` via 0x14139df30), and unlock. Then store
/// `GetCurrentThreadId()` in the state's first `u32` and cache the state in
/// the TLS slot.
///
/// Original: 0x14241bbe0 (294 bytes).
pub unsafe extern "win64" fn scratch_state() -> usize {
    use exe::{rd, wr};
    use std::sync::atomic::{AtomicU8, Ordering};
    let slot = state_slot();
    if rd::<usize>(slot) != 0 {
        return rd::<usize>(slot);
    }
    let mut fresh: usize = 0;
    let _ = exe::call_va!(MAKE_STATE_VA, &mut fresh);
    let mut backoff: u64 = 0x64;
    let lock = &*gptr::<AtomicU8>(REGISTRY_LOCK_VA);
    loop {
        while lock.load(Ordering::Relaxed) != 0 {
            std::hint::spin_loop();
        }
        if lock.swap(1, Ordering::Acquire) == 0 {
            break;
        }
        let _ = exe::call_va!(BACKOFF_VA, &mut backoff);
    }
    let data = gptr::<usize>(REGISTRY_DATA_VA) as usize;
    let count = gptr::<u32>(REGISTRY_COUNT_VA) as usize;
    let cap = gptr::<u32>(REGISTRY_CAP_VA) as usize;
    if rd::<u32>(count) == rd::<u32>(cap) {
        let (n, c) = (rd::<u32>(count) as u64, rd::<u32>(cap) as u64);
        let _ = exe::call_va!(REGISTRY_GROW_VA, rd::<usize>(data), (c + (c >> 1)).max(n + 1));
    }
    wr::<usize>(rd::<usize>(data).wrapping_add(8 * rd::<u32>(count) as usize), fresh);
    wr::<u32>(count, rd::<u32>(count).wrapping_add(1));
    let st = rd::<usize>(rd::<usize>(data).wrapping_add((rd::<u32>(count) as usize).wrapping_sub(1).wrapping_mul(8)));
    lock.store(0, Ordering::Release);
    let tid = exe::call_iat!(GET_CURRENT_THREAD_ID_IAT) as u32;
    wr::<u32>(st, tid);
    wr::<usize>(slot, st);
    st
}

// ---------------------------------------------------------------------------
// Differential tests. One arena holds the state (at +0), up to three block
// descriptors (at +0x100) and their data areas (64 KB each from +0x1000).
// The state getter, acquire and release routines are mocked. The arena is
// snapshotted and restored between the original and the replacement runs.
// ---------------------------------------------------------------------------

use crate::diff_eq;
use crate::difftest::{Mock, Rng, Snapshot, SKIP};
use std::cell::{Cell, RefCell};

thread_local! {
    static STATE: Cell<usize> = const { Cell::new(0) };
    static ACQUIRE_RET: Cell<usize> = const { Cell::new(0) };
    static CALLS: RefCell<Vec<(&'static str, usize, usize, usize)>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_state() -> usize {
    STATE.with(|s| s.get())
}
unsafe extern "win64" fn fake_acquire(pool: usize, bytes: usize, align: usize) -> usize {
    CALLS.with(|c| c.borrow_mut().push(("acquire", pool, bytes, align)));
    ACQUIRE_RET.with(|r| r.get())
}
unsafe extern "win64" fn fake_release(pool: usize, block: usize) {
    CALLS.with(|c| c.borrow_mut().push(("release", pool, block, 0)));
}
fn take_calls() -> Vec<(&'static str, usize, usize, usize)> {
    CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()))
}

const ARENA: usize = 0x40000;
const BLOCKS: usize = 0x100;
const DATA: usize = 0x1000;
const DATA_STRIDE: usize = 0x10000;

struct Arena {
    base: usize,
    layout: std::alloc::Layout,
    _mocks: (Mock, Mock, Mock),
}

impl Arena {
    fn new() -> Result<Arena, String> {
        let layout = std::alloc::Layout::from_size_align(ARENA, 0x1000).unwrap();
        let base = unsafe { std::alloc::alloc_zeroed(layout) } as usize;
        if base == 0 {
            return Err(format!("{SKIP} cannot allocate the test arena"));
        }
        let mocks = (
            Mock::new(STATE_VA, fake_state as *const ())?,
            Mock::new(ACQUIRE_VA, fake_acquire as *const ())?,
            Mock::new(RELEASE_VA, fake_release as *const ())?,
        );
        STATE.with(|s| s.set(base));
        Ok(Arena { base, layout, _mocks: mocks })
    }
    fn block(&self, i: usize) -> usize {
        self.base + BLOCKS + i * 0x40
    }
    fn data(&self, i: usize) -> usize {
        self.base + DATA + i * DATA_STRIDE
    }
    unsafe fn randomize_state(&self, r: &mut Rng) {
        let st = self.base;
        for off in (0..0x40).step_by(8) {
            set_q(st + off, r.next_u64() as usize);
        }
        set_q(st + 0x18, match r.below(3) {
            0 => r.below(0x100) as usize,
            1 => r.below(0x8000) as usize,
            _ => 0x10_0000,
        });
        if r.below(2) == 0 {
            // Keep the counters in a realistic range part of the time.
            *((st + 0x20) as *mut i32) = r.below(0x10000) as i32;
            *((st + 0x28) as *mut i32) = r.below(0x20000) as i32;
        }
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        STATE.with(|s| s.set(0));
        unsafe { std::alloc::dealloc(self.base as *mut u8, self.layout) }
    }
}

/// Run `f` on the original and on the replacement from the same arena
/// contents; return both results with the arena bytes and mock calls each run
/// produced.
unsafe fn both<R>(a: &Arena, f: impl Fn(bool) -> R) -> ((R, Vec<u8>, Vec<(&'static str, usize, usize, usize)>), (R, Vec<u8>, Vec<(&'static str, usize, usize, usize)>)) {
    let snap = Snapshot::take(a.base as *mut u8, ARENA);
    take_calls();
    let ra = f(true);
    let ma = snap.current();
    let ca = take_calls();
    snap.restore();
    let rb = f(false);
    let mb = snap.current();
    let cb = take_calls();
    ((ra, ma, ca), (rb, mb, cb))
}

fn first_diff(a: &[u8], b: &[u8]) -> usize {
    a.iter().zip(b).position(|(x, y)| x != y).unwrap_or(0)
}

pub unsafe fn diff_scratch_alloc(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let a = Arena::new()?;
    let mut r = Rng::new(0x14241c0a0);
    const CASES: u64 = 3000;
    for i in 0..CASES {
        std::ptr::write_bytes(a.base as *mut u8, 0, ARENA);
        a.randomize_state(&mut r);
        let blk = a.block(0);
        let data = a.data(0);
        // Random header garbage so the tag masking is observable.
        for off in (0..0x9000).step_by(8) {
            set_q(data + off, r.next_u64() as usize);
        }
        let bottom = data + 16 * r.below(0x100) as usize;
        set_q(blk, data);
        set_q(blk + 8, bottom);
        set_q(blk + 0x18, if r.below(3) == 0 { bottom } else { bottom + r.below(0x4000) as usize });
        set_q(blk + 0x20, r.next_u64() as usize);
        set_q(blk + 0x28, r.next_u64() as usize);
        ACQUIRE_RET.with(|x| x.set(if r.below(8) == 0 { 0 } else { blk }));
        let size = match r.below(8) {
            0 => 0,
            1 => r.next_u64() as usize,
            _ => r.below(0x3000) as usize,
        };
        let align = match r.below(10) {
            0 => 0,
            1 => 1 + r.below(100) as usize,
            _ => 1usize << r.below(13),
        };
        let ((ra, ma, ca), (rb, mb, cb)) = both(&a, |orig| if orig { o(size, align) } else { n(size, align) });
        diff_eq!(ra, rb, "return, case {i} size {size:#x} align {align:#x}");
        diff_eq!(ca, cb, "acquire calls, case {i} size {size:#x} align {align:#x}");
        if ma != mb {
            return Err(format!(
                "arena differs at +{:#x}, case {i} size {size:#x} align {align:#x}",
                first_diff(&ma, &mb)
            ));
        }
    }
    Ok(CASES as u32)
}

pub unsafe fn diff_scratch_free(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let a = Arena::new()?;
    let mut r = Rng::new(0x14241c190);
    const CASES: u64 = 3000;
    let mut releases = 0u32;
    for i in 0..CASES {
        std::ptr::write_bytes(a.base as *mut u8, 0, ARENA);
        a.randomize_state(&mut r);
        let nblocks = 1 + r.below(3) as usize;
        let mut freed = 0usize;
        for b in 0..nblocks {
            let blk = a.block(b);
            let data = a.data(b);
            let mut h = data + 0x40 + 16 * r.below(8) as usize;
            let first = h;
            let mut prev = 0usize;
            let nh = 1 + r.below(6) as usize;
            let mut hdrs = Vec::with_capacity(nh);
            for k in 0..nh {
                *(h as *mut u32) = r.next_u32();
                // Live bit: mostly live, so popping stops somewhere.
                *((h + 4) as *mut u8) = (r.next_u32() as u8 & !1) | (r.below(3) != 0) as u8;
                set_q(h + 8, if k == 0 && r.below(8) != 0 { 0 } else { prev });
                hdrs.push(h);
                prev = h;
                h += 0x10 + 16 * r.below(0x40) as usize;
            }
            set_q(blk, data - 16 * r.below(4) as usize);
            set_q(blk + 8, if r.below(4) == 0 { data } else { first });
            set_q(blk + 0x18, h);
            set_q(blk + 0x20, prev);
            let hi = (r.next_u64() as usize) & !LINK_MASK;
            set_q(blk + 0x28, hi | if b + 1 < nblocks { a.block(b + 1) } else { 0 });
            if b == 0 {
                freed = if r.below(6) == 0 { hdrs[r.below(nh as u64) as usize] } else { prev };
            }
        }
        set_q(a.base + 0x10, a.block(0));
        let p = (freed + 0x10) as *mut u8;
        let ((_, ma, ca), (_, mb, cb)) = both(&a, |orig| if orig { o(p) } else { n(p) });
        diff_eq!(ca, cb, "release calls, case {i} blocks {nblocks}");
        releases += ca.len() as u32;
        if ma != mb {
            return Err(format!("arena differs at +{:#x}, case {i} blocks {nblocks}", first_diff(&ma, &mb)));
        }
    }
    if releases == 0 {
        return Err("no case reached the block-release path".into());
    }
    Ok(CASES as u32)
}

// --- ScratchArrayU16: alloc/free mocked as recorders over a test buffer.

thread_local! {
    static ARR_CALLS: RefCell<Vec<(&'static str, usize, usize)>> = const { RefCell::new(Vec::new()) };
    static ARR_NEXT: Cell<usize> = const { Cell::new(0) };
}

unsafe extern "win64" fn fake_scratch_alloc(size: usize, align: usize) -> usize {
    ARR_CALLS.with(|c| c.borrow_mut().push(("alloc", size, align)));
    ARR_NEXT.with(|n| n.get())
}
unsafe extern "win64" fn fake_scratch_free(p: usize) {
    ARR_CALLS.with(|c| c.borrow_mut().push(("free", p, 0)));
}

/// Shared driver: `buf` holds the old data (offset 0) and the fixture's
/// fresh block (offset 0x10000), so pointers can be compared directly.
unsafe fn drive_array(salt: u64, mut call: impl FnMut(&mut Rng, *mut ScratchArrayU16, bool)) -> Result<u32, String> {
    let _a = Mock::new(SCRATCH_ALLOC_VA, fake_scratch_alloc as *const ())?;
    let _f = Mock::new(SCRATCH_FREE_VA, fake_scratch_free as *const ())?;
    let mut buf = vec![0u8; 0x20000];
    let base = buf.as_mut_ptr() as usize;
    ARR_NEXT.with(|n| n.set(base + 0x10000));
    let mut r = Rng::new(salt);
    const CASES: u32 = 3000;
    for i in 0..CASES {
        for b in buf.iter_mut() {
            *b = r.next_u32() as u8;
        }
        let cap = match r.below(4) {
            0 => 0,
            _ => r.below(0x2000) as u32,
        };
        let len = if r.below(3) == 0 { cap } else { r.below(cap as u64 + 1) as u32 };
        let null = cap == 0 && len == 0 && r.below(2) == 0;
        // Odd data addresses every other case (the original's word stores tolerate them).
        let dbase = base + (i as usize % 2);
        let arr0 = ScratchArrayU16 { data: if null { std::ptr::null_mut() } else { dbase as *mut u16 }, len, cap };
        let snap = buf.clone();
        let saved = r.0;
        let mut a = ScratchArrayU16 { ..arr0 };
        ARR_CALLS.with(|c| c.borrow_mut().clear());
        call(&mut r, &mut a, true);
        let (ma, ca) = (buf.clone(), ARR_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut())));
        buf.copy_from_slice(&snap);
        r.0 = saved;
        let mut b = ScratchArrayU16 { ..arr0 };
        call(&mut r, &mut b, false);
        let (mb, cb) = (buf.clone(), ARR_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut())));
        let ctx = format!("case {i} len {len} cap {cap} null {null}");
        diff_eq!((a.data as usize, a.len, a.cap), (b.data as usize, b.len, b.cap), "array header, {ctx}");
        diff_eq!(ca, cb, "alloc/free calls, {ctx}");
        if ma != mb {
            return Err(format!("buffer differs at +{:#x}, {ctx}", first_diff(&ma, &mb)));
        }
    }
    Ok(CASES)
}

pub unsafe fn diff_array_u16_reallocate(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut ScratchArrayU16, u64);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_array(0x140fb3a60, |r, a, first| {
        // New capacity >= len (as callers guarantee), sometimes > u32.
        let len = (*a).len as u64;
        let cap = match r.below(6) {
            0 => len,
            1 => len + r.below(0x1000) + (1 << 32),
            _ => len + r.below(0x1000),
        };
        (if first { o } else { n })(a, cap)
    })
}

pub unsafe fn diff_array_u16_push_back(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut ScratchArrayU16, *const u16);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    drive_array(0x140faef90, |r, a, first| {
        // The value at an odd address.
        let mut vb = [0u8; 4];
        vb[1..3].copy_from_slice(&(r.next_u32() as u16).to_le_bytes());
        (if first { o } else { n })(a, vb.as_ptr().add(1) as *const u16)
    })
}

// --- scratch_state: the TLS slot and registry globals are saved, pointed at
// fixtures, and restored. Creation and growth are mocked; the fixture state
// and grow buffer are at fixed addresses, so both runs produce identical values.

thread_local! {
    static ST_CALLS: RefCell<Vec<(&'static str, u64)>> = const { RefCell::new(Vec::new()) };
    static ST_FRESH: Cell<usize> = const { Cell::new(0) };
    static ST_GROW_BUF: Cell<usize> = const { Cell::new(0) };
}

unsafe extern "win64" fn fake_make_state(out: *mut usize) {
    ST_CALLS.with(|c| c.borrow_mut().push(("make", 0)));
    *out = ST_FRESH.with(|f| f.get());
}
unsafe extern "win64" fn fake_registry_grow(_dead: usize, new_cap: u64) {
    ST_CALLS.with(|c| c.borrow_mut().push(("grow", new_cap)));
    let data = gptr::<usize>(REGISTRY_DATA_VA);
    let count = *gptr::<u32>(REGISTRY_COUNT_VA) as usize;
    let buf = ST_GROW_BUF.with(|b| b.get());
    std::ptr::copy_nonoverlapping(*data as *const usize, buf as *mut usize, count);
    *data = buf;
    *gptr::<u32>(REGISTRY_CAP_VA) = new_cap as u32;
}

pub unsafe fn diff_scratch_state(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn() -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    // A thread without a TEB is adopted by the first Windows API call; do that
    // now so it can't happen in the middle of the original's run.
    if *gptr::<usize>(GET_CURRENT_THREAD_ID_IAT) == 0 {
        return Err(format!("{SKIP} GetCurrentThreadId is not bound"));
    }
    let _ = exe::call_iat!(GET_CURRENT_THREAD_ID_IAT);
    let tls = tls_pointer();
    if tls == 0 || *(tls as *const usize) == 0 {
        return Err(format!("{SKIP} no exe TLS block on this thread"));
    }
    if *gptr::<u8>(REGISTRY_LOCK_VA) != 0 {
        return Err(format!("{SKIP} registry lock is held"));
    }
    let _m = Mock::new(MAKE_STATE_VA, fake_make_state as *const ())?;
    let _g = Mock::new(REGISTRY_GROW_VA, fake_registry_grow as *const ())?;
    let slot = state_slot() as *mut usize;
    let globals = Snapshot::take(gptr::<u8>(REGISTRY_DATA_VA), 16);
    let saved_slot = *slot;
    // [0..0x40) registry entries, [0x40..0x80) grow buffer, [0x80..0x88) fresh state.
    let mut arena = vec![0usize; 0x88];
    let base = arena.as_mut_ptr() as usize;
    ST_GROW_BUF.with(|b| b.set(base + 0x40 * 8));
    ST_FRESH.with(|f| f.set(base + 0x80 * 8));
    let mut r = Rng::new(0x14241bbe0);
    const CASES: u32 = 500;
    let mut result = Ok(CASES);
    for i in 0..CASES {
        for w in arena.iter_mut() {
            *w = r.next_u64() as usize;
        }
        let count = r.below(0x20) as u32;
        let cap = if r.below(3) == 0 { count } else { count + r.below(8) as u32 };
        let fast = r.below(4) == 0;
        let slot0 = if fast { r.next_u64() as usize | 1 } else { 0 };
        let before = arena.clone();
        let run = |f: F, arena: &mut Vec<usize>| {
            arena.copy_from_slice(&before);
            *gptr::<usize>(REGISTRY_DATA_VA) = base;
            *gptr::<u32>(REGISTRY_COUNT_VA) = count;
            *gptr::<u32>(REGISTRY_CAP_VA) = cap;
            *slot = slot0;
            ST_CALLS.with(|c| c.borrow_mut().clear());
            let ret = f();
            let g = (*gptr::<usize>(REGISTRY_DATA_VA), *gptr::<u32>(REGISTRY_COUNT_VA), *gptr::<u32>(REGISTRY_CAP_VA));
            (ret, *slot, g, ST_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut())), arena.clone(), *gptr::<u8>(REGISTRY_LOCK_VA))
        };
        let a = run(o, &mut arena);
        let b = run(n, &mut arena);
        if a != b {
            result = Err(format!("case {i} count {count} cap {cap} fast {fast}: orig={:?} new={:?}", (a.0, a.1, a.2, &a.3, a.5), (b.0, b.1, b.2, &b.3, b.5)));
            break;
        }
    }
    *slot = saved_slot;
    globals.restore();
    result
}

// ---------------------------------------------------------------------------
// Per-thread ring buffer of 2 KB (`0x800`) temporary format strings.
// ---------------------------------------------------------------------------

const FMT_RING_INIT_OFFSET: usize = 0x6312;
const FMT_RING_END2_OFFSET: usize = 0x463d0;
const FMT_RING_CURSOR_OFFSET: usize = 0x463d8;
const FMT_RING_START_OFFSET: usize = 0x4b460;
const FMT_RING_LIMIT_OFFSET: usize = 0x5b460;
const FMT_RING_END2_VAL_OFFSET: usize = 0x83460;
const FMT_RING_STRIDE: usize = 0x800;

const FORMAT_TEMP_ALLOC_VA: u64 = 0x1_423f_dfc0;
const VSNPRINTF_S_VA: u64 = 0x1_4137_3300;

/// Allocate the next 2 KB (`0x800`-byte) buffer from the calling thread's
/// 64 KB (`0x10000`-byte, 32-slot) TLS ring buffer `[tls0 + 0x4b460 .. tls0 + 0x5b460)`.
///
/// Original: 0x1423fdfc0 (102 bytes, 136 call sites).
pub unsafe extern "win64" fn format_temp_alloc_buf() -> *mut u8 {
    use exe::{rd, wr};
    let tls0 = rd::<usize>(tls_pointer());
    let start = tls0.wrapping_add(FMT_RING_START_OFFSET);
    if rd::<u8>(tls0.wrapping_add(FMT_RING_INIT_OFFSET)) == 0 {
        wr::<usize>(tls0.wrapping_add(FMT_RING_CURSOR_OFFSET), start);
        wr::<u8>(tls0.wrapping_add(FMT_RING_INIT_OFFSET), 1);
        wr::<usize>(
            tls0.wrapping_add(FMT_RING_END2_OFFSET),
            tls0.wrapping_add(FMT_RING_END2_VAL_OFFSET),
        );
    }
    let cur = rd::<usize>(tls0.wrapping_add(FMT_RING_CURSOR_OFFSET));
    let limit = tls0.wrapping_add(FMT_RING_LIMIT_OFFSET);
    let next = cur.wrapping_add(FMT_RING_STRIDE);
    wr::<usize>(
        tls0.wrapping_add(FMT_RING_CURSOR_OFFSET),
        if next < limit { next } else { start },
    );
    cur as *mut u8
}

/// `r::formatTemp(fmt, ...)` (name inferred) — spills register varargs into
/// the caller's shadow space, acquires a 2 KB TLS buffer via `0x1423fdfc0`
/// ([`format_temp_alloc_buf`]), formats into it with `0x141373300`
/// (`_vsnprintf_s_l(buf, 0x800, 0x7ff, fmt, locale, va_list)`), and returns `buf`.
///
/// Original: 0x1423fe030 (77 bytes, 1,516 call sites).
pub unsafe extern "win64" fn format_temp(
    fmt: *const u8,
    a0: u64,
    a1: u64,
    a2: u64,
) -> *mut u8 {
    #[cfg(target_arch = "x86_64")]
    {
        let mut rbp: usize;
        std::arch::asm!("mov {}, rbp", out(reg) rbp, options(nomem, nostack, preserves_flags));
        exe::wr::<*const u8>(rbp + 0x10, fmt);
        exe::wr::<u64>(rbp + 0x18, a0);
        exe::wr::<u64>(rbp + 0x20, a1);
        exe::wr::<u64>(rbp + 0x28, a2);
        let buf = exe::call_va!(FORMAT_TEMP_ALLOC_VA) as usize as *mut u8;
        let _ = exe::call_va!(
            VSNPRINTF_S_VA,
            buf,
            FMT_RING_STRIDE,
            FMT_RING_STRIDE - 1,
            fmt,
            0usize,
            (rbp + 0x18) as *const u64,
        );
        buf
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let (mut a3, mut a4, mut a5, mut a6) = (0u64, 0u64, 0u64, 0u64);
        #[cfg(target_arch = "aarch64")]
        std::arch::asm!(
            "mov {a3}, x4",
            "mov {a4}, x5",
            "mov {a5}, x6",
            "mov {a6}, x7",
            a3 = out(reg) a3,
            a4 = out(reg) a4,
            a5 = out(reg) a5,
            a6 = out(reg) a6,
            options(nomem, nostack, preserves_flags),
        );
        let va = [a0, a1, a2, a3, a4, a5, a6, 0];
        let buf = exe::call_va!(FORMAT_TEMP_ALLOC_VA) as usize as *mut u8;
        let _ = exe::call_va!(
            VSNPRINTF_S_VA,
            buf,
            FMT_RING_STRIDE,
            FMT_RING_STRIDE - 1,
            fmt,
            0usize,
            va.as_ptr(),
        );
        buf
    }
}

pub unsafe fn diff_format_temp_alloc_buf(orig: usize, new: usize) -> Result<u32, String> {
    use exe::{rd, wr};
    type F = unsafe extern "win64" fn() -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    if *gptr::<usize>(GET_CURRENT_THREAD_ID_IAT) == 0 {
        return Err(format!("{SKIP} GetCurrentThreadId is not bound"));
    }
    let _ = exe::call_iat!(GET_CURRENT_THREAD_ID_IAT);
    let tls = tls_pointer();
    if tls == 0 || rd::<usize>(tls) == 0 {
        return Err(format!("{SKIP} no exe TLS block on this thread"));
    }
    let tls0 = rd::<usize>(tls);
    let p_init = tls0 + FMT_RING_INIT_OFFSET;
    let p_end2 = tls0 + FMT_RING_END2_OFFSET;
    let p_cur = tls0 + FMT_RING_CURSOR_OFFSET;
    let saved = (rd::<u8>(p_init), rd::<usize>(p_end2), rd::<usize>(p_cur));

    let mut r = Rng::new(0x1423fdfc0);
    const CASES: u32 = 300;
    let mut res = Ok(CASES);
    for i in 0..CASES {
        let init = if r.below(4) == 0 { 0u8 } else { (1 + r.below(255)) as u8 };
        let end2 = r.next_u64() as usize;
        let cur = match r.below(5) {
            0 => tls0 + FMT_RING_START_OFFSET + FMT_RING_STRIDE * r.below(32) as usize,
            1 => tls0 + FMT_RING_LIMIT_OFFSET - FMT_RING_STRIDE,
            2 => tls0 + FMT_RING_LIMIT_OFFSET - 1,
            3 => tls0 + FMT_RING_LIMIT_OFFSET,
            _ => r.next_u64() as usize,
        };
        let run = |f: F| {
            wr::<u8>(p_init, init);
            wr::<usize>(p_end2, end2);
            wr::<usize>(p_cur, cur);
            let ret = f();
            (ret, rd::<u8>(p_init), rd::<usize>(p_end2), rd::<usize>(p_cur))
        };
        let a = run(o);
        let b = run(n);
        if a != b {
            res = Err(format!("case {i}: orig={a:#x?} new={b:#x?}"));
            break;
        }
    }
    wr::<u8>(p_init, saved.0);
    wr::<usize>(p_end2, saved.1);
    wr::<usize>(p_cur, saved.2);
    res
}

thread_local! {
    static FMT_TEMP_CALLS: RefCell<Vec<(usize, usize, usize, usize, u64, u64, u64)>> = const { RefCell::new(Vec::new()) };
    static FMT_TEMP_BUF: Cell<usize> = const { Cell::new(0) };
}

unsafe extern "win64" fn fake_format_temp_alloc() -> *mut u8 {
    FMT_TEMP_BUF.with(|b| b.get() as *mut u8)
}

unsafe extern "win64" fn fake_vsnprintf_s(
    buf: *mut u8,
    buf_size: usize,
    max_count: usize,
    fmt: *const u8,
    _locale: usize,
    va: *const u64,
) -> i32 {
    let a0 = exe::rd::<u64>(va as usize);
    let a1 = exe::rd::<u64>(va as usize + 8);
    let a2 = exe::rd::<u64>(va as usize + 16);
    FMT_TEMP_CALLS.with(|c| {
        c.borrow_mut().push((buf as usize, buf_size, max_count, fmt as usize, a0, a1, a2))
    });
    0
}

pub unsafe fn diff_format_temp(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, u64, u64, u64) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_alloc = Mock::new(FORMAT_TEMP_ALLOC_VA, fake_format_temp_alloc as *const ())?;
    let _m_vsnp = Mock::new(VSNPRINTF_S_VA, fake_vsnprintf_s as *const ())?;
    let mut r = Rng::new(0x1423fe030);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let fake_buf = r.next_u64() as usize;
        let fmt = r.next_u64() as usize as *const u8;
        let (x0, x1, x2) = (r.next_u64(), r.next_u64(), r.next_u64());
        FMT_TEMP_BUF.with(|b| b.set(fake_buf));
        FMT_TEMP_CALLS.with(|c| c.borrow_mut().clear());
        let ra = o(fmt, x0, x1, x2);
        let ca = FMT_TEMP_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        let rb = n(fmt, x0, x1, x2);
        let cb = FMT_TEMP_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()));
        diff_eq!(ra, rb, "return, case {i}");
        diff_eq!(ca, cb, "vsnprintf_s call, case {i}");
    }
    Ok(CASES)
}
