//! `ui::MemoryAllocator`: the game's `cohtml::IAllocator` implementation (vftable
//! `0x14341a050`). cohtml/RenoirCore call it from guest code hundreds of times per
//! frame, so every slot that is hot in the emulator profile is native here:
//!
//! | VA | slot / role | Rust |
//! |---|---|---|
//! | `0x142504510` | vfunc_0 `Allocate(size, tag)` | [`memory_allocator_allocate`] |
//! | `0x142504500` | vfunc_2 `Deallocate(ptr, tag)` (`add rcx, 8; jmp 0x14248c9d0`) | [`memory_allocator_deallocate`] |
//! | `0x1425042e0` | vfunc_3 `AlignedAllocate(size, align, tag)` | [`memory_allocator_aligned_allocate`] |
//! | `0x1425042d0` | vfunc_4 `AlignedDeallocate(ptr, tag)` (same thunk) | [`memory_allocator_deallocate`] |
//! | `0x14248c910` | locked heap `alloc_aligned(heap, size, align)` | [`ui_allocator_alloc_aligned`] |
//! | `0x14248c9d0` | locked heap `free(heap, ptr) -> owned` | [`ui_allocator_free`] |
//!
//! The heap object (`MemoryAllocator + 8`) is `{ u8 lock; ..; u8 lock_enabled @0x10;
//! mspace @0x18; segments @0x28 (24-byte {base, size, ..}); u32 segment_count @0x30 }`;
//! see [`crate::ns::r::scheduler`] for `contains_ptr` (`0x14248c1e0`) and
//! `alloc_zeroed` (`0x14248c4e0`). The mspace functions themselves (`0x142512760`
//! malloc, `0x14250fcc0` memalign, `0x142512b10` free) are dlmalloc (third-party, chunk
//! header at `ptr - 8` with `PINUSE|CINUSE` bits) and stay original, called through
//! their VAs.

use crate::exe;
use crate::ns::r::scheduler::{
    spin_lock_acquire_if_enabled, spin_lock_release_if_held, ui_allocator_alloc_zeroed,
    ui_allocator_contains_ptr,
};

#[inline(always)]
unsafe fn rd<T: Copy>(addr: u64) -> T {
    exe::rd::<T>(addr as usize)
}

/// dlmalloc `mspace_malloc` (original, third-party).
const MSPACE_MALLOC: u64 = 0x1_4251_2760;
/// dlmalloc `mspace_memalign(msp, alignment, bytes)` (original, third-party).
const MSPACE_MEMALIGN: u64 = 0x1_4250_fcc0;
/// dlmalloc `mspace_free` (original, third-party).
const MSPACE_FREE: u64 = 0x1_4251_2b10;
/// Formatting fatal-error logger `(const string_view* fmt, const void* args)`.
const LOG_FATAL_FMT: u64 = 0x1_4021_a780;
/// `"Could not allocate memory for ui allocator, tried to allocate {} bytes"` (0x46 bytes).
const ALLOC_FAILED_MSG: u64 = 0x1_4341_a000;
const GET_CURRENT_PROCESS_IAT: u64 = 0x1_4264_56c8;
const TERMINATE_PROCESS_IAT: u64 = 0x1_4264_53a0;

/// dlmalloc `mspace_usable_size` as inlined by the callers: 0 for a free chunk
/// (`(head & 3) == 1`), else chunk size minus 16 (mmapped, `(head & 3) == 0`) or 8.
#[inline(always)]
unsafe fn usable_size(ptr: u64) -> u64 {
    let hdr = rd::<u64>(ptr.wrapping_sub(8));
    let tag = hdr & 3;
    if tag == 1 {
        0
    } else {
        let overhead = if tag == 0 { 0x10u64 } else { 0x08u64 };
        (hdr & !7u64).wrapping_sub(overhead)
    }
}

/// Shared out-of-memory tail of vfunc_0/vfunc_3: log the size with the format at
/// `0x14341a000`, then `TerminateProcess(GetCurrentProcess(), 0xdeadd00d)`. The
/// original passes the address of its spilled `edx` (home slot `[rsp + 0x10]`) as the
/// `{}` argument.
#[cold]
#[inline(never)]
unsafe fn alloc_failed(size: u32) {
    let mut size_slot: u64 = size as u64;
    let fmt: [u64; 2] = [exe::ptr::<u8>(ALLOC_FAILED_MSG) as u64, 0x46];
    exe::call_va!(LOG_FATAL_FMT, (fmt.as_ptr() as u64, (&mut size_slot as *mut u64) as u64));
    let proc = exe::call_iat!(GET_CURRENT_PROCESS_IAT, () -> usize);
    exe::call_iat!(TERMINATE_PROCESS_IAT, (proc, 0xdead_d00du32));
}

/// `sub_14248c910` — original @ 0x14248c910 (173 bytes).
///
/// Locked aligned allocation from the ui heap: takes the optional byte spinlock, then
/// `mspace_malloc(heap->mspace, size)` when `align <= 16` (unsigned 64-bit compare of
/// `r8`), else `mspace_memalign(heap->mspace, align, size)`. The original's unwind map
/// only releases the lock guard.
pub unsafe extern "win64" fn ui_allocator_alloc_aligned(heap: u64, size: u64, align: u64) -> u64 {
    let held = spin_lock_acquire_if_enabled(heap);
    let msp = rd::<u64>(heap + 0x18);
    let p = if align <= 0x10 {
        exe::call_va!(MSPACE_MALLOC, (msp, size) -> u64)
    } else {
        exe::call_va!(MSPACE_MEMALIGN, (msp, align, size) -> u64)
    };
    spin_lock_release_if_held(heap, held);
    p
}

/// `sub_14248c9d0` — original @ 0x14248c9d0 (167 bytes).
///
/// Frees `ptr` if it lies in one of the heap's segments (`0x14248c1e0`, which takes the
/// lock itself); then, under the lock, `mspace_free(heap->mspace, ptr)`. Returns the
/// ownership bool (`al`; 1 when freed, else the `contains` result).
pub unsafe extern "win64" fn ui_allocator_free(heap: u64, ptr: u64) -> u64 {
    let owned = ui_allocator_contains_ptr(heap, ptr);
    if owned as u8 == 0 {
        return owned;
    }
    let held = spin_lock_acquire_if_enabled(heap);
    exe::call_va!(MSPACE_FREE, (rd::<u64>(heap + 0x18), ptr));
    spin_lock_release_if_held(heap, held);
    owned as u8 as u64
}

/// `ui::MemoryAllocator::vfunc_0` — original @ 0x142504510 (105 bytes).
///
/// `Allocate(u32 size, tag)`: zeroed allocation from the heap at `this + 8`
/// (`0x14248c4e0`); on failure logs and terminates the process (see [`alloc_failed`]),
/// returning 0 if `TerminateProcess` comes back.
pub unsafe extern "win64" fn memory_allocator_allocate(this: u64, size: u32) -> u64 {
    let p = ui_allocator_alloc_zeroed(this.wrapping_add(8), size as u64);
    if p == 0 {
        alloc_failed(size);
    }
    p
}

/// `ui::MemoryAllocator::vfunc_3` — original @ 0x1425042e0 (167 bytes).
///
/// `AlignedAllocate(u32 size, u32 align, tag)`: [`ui_allocator_alloc_aligned`] on the heap
/// at `this + 8`, then zero-fills the chunk's usable size (the original calls `j_memset`
/// even for a null result, with length 0); on failure logs and terminates like vfunc_0.
pub unsafe extern "win64" fn memory_allocator_aligned_allocate(this: u64, size: u32, align: u32) -> u64 {
    let p = ui_allocator_alloc_aligned(this.wrapping_add(8), size as u64, align as u64);
    if p == 0 {
        alloc_failed(size);
        return 0;
    }
    let n = usable_size(p);
    if n != 0 {
        core::ptr::write_bytes(p as *mut u8, 0, n as usize);
    }
    p
}

/// `ui::MemoryAllocator::vfunc_2` / `vfunc_4` — originals @ 0x142504500 and 0x1425042d0
/// (9 bytes each: `add rcx, 8; jmp 0x14248c9d0`).
///
/// `Deallocate(ptr, tag)` / `AlignedDeallocate(ptr, tag)`: [`ui_allocator_free`] on the
/// heap at `this + 8` (the tail jump leaves its `rax` as the result).
pub unsafe extern "win64" fn memory_allocator_deallocate(this: u64, ptr: u64) -> u64 {
    ui_allocator_free(this.wrapping_add(8), ptr)
}

// ---------------------------------------------------------------------------
// Difftests
// ---------------------------------------------------------------------------

use crate::difftest::{eq, IatMock, Mock, Rng};
use std::cell::RefCell;

thread_local! {
    /// Calls seen by the mocks: `[kind, a, b, c, heap lock byte]` (1 malloc, 2 memalign, 3 free, 4 log,
    /// 5 GetCurrentProcess, 6 TerminateProcess).
    static CALLS: RefCell<Vec<[u64; 5]>> = const { RefCell::new(Vec::new()) };
    /// Result of the next mocked allocation.
    static NEXT: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
    /// Heap object whose lock byte the mocks record (to see the lock is held).
    static HEAP: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

fn rec(c: [u64; 5]) {
    CALLS.with(|v| v.borrow_mut().push(c));
}

fn take_calls() -> Vec<[u64; 5]> {
    CALLS.with(|v| core::mem::take(&mut *v.borrow_mut()))
}

unsafe fn heap_lock_byte() -> u64 {
    let h = HEAP.with(|h| h.get());
    if h == 0 { 0xff } else { rd::<u8>(h) as u64 }
}

unsafe extern "win64" fn mock_malloc(msp: u64, size: u64) -> u64 {
    rec([1, msp, size, 0, heap_lock_byte()]);
    NEXT.with(|n| n.get())
}

unsafe extern "win64" fn mock_memalign(msp: u64, align: u64, size: u64) -> u64 {
    rec([2, msp, align, size, heap_lock_byte()]);
    NEXT.with(|n| n.get())
}

unsafe extern "win64" fn mock_free(msp: u64, ptr: u64) {
    rec([3, msp, ptr, 0, heap_lock_byte()]);
}

unsafe extern "win64" fn mock_log(fmt: u64, args: u64) {
    rec([4, rd::<u64>(fmt), rd::<u64>(fmt + 8), rd::<u32>(args) as u64, heap_lock_byte()]);
}

unsafe extern "win64" fn mock_get_current_process() -> u64 {
    rec([5, 0, 0, 0, 0]);
    0x5150
}

unsafe extern "win64" fn mock_terminate_process(h: u64, code: u32) -> u32 {
    rec([6, h, code as u64, 0, 0]);
    1
}

/// Fake heap object plus dlmalloc-style chunks: `buf[0..0x40]` heap, segment table at
/// `0x40`, chunks of 0x80 bytes from `0x100` (header at `payload - 8`).
struct Fixture {
    buf: Vec<u64>,
    /// Byte offset of the heap object inside `buf` (0 or 4 for under-aligned runs).
    skew: u64,
}

impl Fixture {
    const BYTES: usize = 0x800;
    fn new(skew: u64) -> Self {
        Self { buf: vec![0u64; Self::BYTES / 8 + 1], skew }
    }
    fn base(&self) -> u64 {
        self.buf.as_ptr() as u64
    }
    fn heap(&self) -> u64 {
        self.base() + self.skew
    }
    /// Payload address of chunk `i` (0..12).
    fn chunk(&self, i: u64) -> u64 {
        self.base() + 0x110 + i * 0x80
    }
    unsafe fn reset(&mut self, lock_en: u8, rng: &mut Rng, hdr_tag: u64) {
        let b = self.base() as *mut u8;
        core::ptr::write_bytes(b, 0xcd, Self::BYTES);
        let h = self.heap();
        exe::wr::<u8>(h as usize, 0);
        exe::wr::<u8>((h + 0x10) as usize, lock_en);
        exe::wr::<u64>((h + 0x18) as usize, 0x7777_0000_1234);
        // Two segments: chunks 0..6 and 8..12 (chunk 6/7 and outside are foreign).
        let seg = self.base() + 0x40;
        exe::wr::<u64>((h + 0x28) as usize, seg);
        exe::wr::<u32>((h + 0x30) as usize, 2);
        exe::wr::<u64>(seg as usize, self.chunk(0) - 0x10);
        exe::wr::<u64>((seg + 8) as usize, 6 * 0x80);
        exe::wr::<u64>((seg + 24) as usize, self.chunk(8) - 0x10);
        exe::wr::<u64>((seg + 32) as usize, 4 * 0x80);
        for i in 0..12 {
            let size = 0x20 + (rng.below(6) * 8);
            exe::wr::<u64>((self.chunk(i) - 8) as usize, size | hdr_tag);
        }
        HEAP.with(|c| c.set(h));
    }
    unsafe fn snapshot(&self) -> Vec<u8> {
        std::slice::from_raw_parts(self.base() as *const u8, Self::BYTES).to_vec()
    }
}

unsafe fn with_mocks<R>(f: impl FnOnce() -> Result<R, String>) -> Result<R, String> {
    let _m1 = Mock::new(MSPACE_MALLOC, mock_malloc as *const ())?;
    let _m2 = Mock::new(MSPACE_MEMALIGN, mock_memalign as *const ())?;
    let _m3 = Mock::new(MSPACE_FREE, mock_free as *const ())?;
    let _m4 = Mock::new(LOG_FATAL_FMT, mock_log as *const ())?;
    let _i1 = IatMock::new(GET_CURRENT_PROCESS_IAT, mock_get_current_process as *const ())?;
    let _i2 = IatMock::new(TERMINATE_PROCESS_IAT, mock_terminate_process as *const ())?;
    let r = f();
    HEAP.with(|c| c.set(0));
    r
}

type Observed = (u64, Vec<[u64; 5]>, Vec<u8>);

/// Run `call(fixture)` on the reset fixture (the same buffer for both sides, so the
/// pointers stored in it compare equal); returns (`rax` normalised against the
/// fixture base, mock calls with fixture addresses normalised, fixture bytes).
unsafe fn observe(fx: &mut Fixture, seed: u64, lock_en: u8, tag: u64, next: impl Fn(&Fixture) -> u64, call: impl Fn(&Fixture) -> u64, ret_mask: u64) -> Observed {
    let mut rng = Rng::new(seed);
    fx.reset(lock_en, &mut rng, tag);
    NEXT.with(|n| n.set(next(fx)));
    let _ = take_calls();
    let r = call(fx) & ret_mask;
    let base = fx.base();
    let norm = |v: u64| if v >= base && v < base + Fixture::BYTES as u64 { v - base + 0xf000_0000 } else { v };
    let calls = take_calls().into_iter().map(|c| [c[0], norm(c[1]), norm(c[2]), norm(c[3]), c[4]]).collect();
    (norm(r), calls, fx.snapshot())
}

/// The cases of one allocator entry point: lock on/off, heap at offset 0/4, all
/// chunk-header tags, success and failure.
unsafe fn run_cases(
    name: &str,
    orig: usize,
    new: usize,
    call: unsafe fn(usize, &Fixture, &mut Rng) -> u64,
    next: unsafe fn(&Fixture, u64) -> u64,
    ret_mask: u64,
) -> Result<u32, String> {
    with_mocks(|| {
        let mut n = 0u32;
        for seed in 0..24u64 {
            let lock_en = (seed & 1) as u8;
            let skew = if seed & 2 != 0 { 4 } else { 0 };
            let tag = (seed >> 2) & 3;
            let which = seed % 7;
            let mut fx = Fixture::new(skew);
            let o = observe(&mut fx, seed, lock_en, tag, |fx| next(fx, which), |fx| call(orig, fx, &mut Rng::new(seed ^ 0x55)), ret_mask);
            let r = observe(&mut fx, seed, lock_en, tag, |fx| next(fx, which), |fx| call(new, fx, &mut Rng::new(seed ^ 0x55)), ret_mask);
            eq(&format!("{name} seed={seed} ret"), o.0, r.0)?;
            eq(&format!("{name} seed={seed} calls"), &o.1, &r.1)?;
            if o.2 != r.2 {
                let i = o.2.iter().zip(&r.2).position(|(a, b)| a != b).unwrap_or(0);
                return Err(format!("mismatch: {name} seed={seed} memory differs at fixture+0x{i:x} (orig {:#x} new {:#x})", o.2[i], r.2[i]));
            }
            n += 1;
        }
        Ok(n)
    })
}

/// Allocation result for case `which`: null, an owned chunk, or a foreign pointer.
unsafe fn next_alloc(fx: &Fixture, which: u64) -> u64 {
    match which {
        0 => 0,
        6 => fx.chunk(6),
        w => fx.chunk(w + 1),
    }
}

/// Free paths never allocate.
unsafe fn no_alloc(_fx: &Fixture, _which: u64) -> u64 {
    0
}

/// Pointer to free for case `which`: owned chunks in both segments, a foreign chunk, a
/// segment end, and an unrelated address.
unsafe fn free_ptr(fx: &Fixture, which: u64) -> u64 {
    match which {
        0 => fx.chunk(0),
        1 => fx.chunk(5),
        2 => fx.chunk(6),
        3 => fx.chunk(9),
        4 => fx.chunk(0) - 0x10 + 6 * 0x80,
        5 => 0x10,
        _ => fx.chunk(11),
    }
}

pub unsafe fn diff_ui_allocator_alloc_aligned(orig: usize, new: usize) -> Result<u32, String> {
    unsafe fn call(f: usize, fx: &Fixture, rng: &mut Rng) -> u64 {
        let f: unsafe extern "win64" fn(u64, u64, u64) -> u64 = core::mem::transmute(f);
        let align = [0u64, 8, 0x10, 0x11, 0x20, 0x100, 1 << 40][rng.below(7) as usize];
        f(fx.heap(), 0x10 + rng.below(0x200), align)
    }
    run_cases("ui_allocator_alloc_aligned", orig, new, call, next_alloc, u64::MAX)
}

pub unsafe fn diff_ui_allocator_free(orig: usize, new: usize) -> Result<u32, String> {
    unsafe fn call(f: usize, fx: &Fixture, rng: &mut Rng) -> u64 {
        let f: unsafe extern "win64" fn(u64, u64) -> u64 = core::mem::transmute(f);
        f(fx.heap(), free_ptr(fx, rng.below(7)))
    }
    run_cases("ui_allocator_free", orig, new, call, no_alloc, 0xff)
}

pub unsafe fn diff_memory_allocator_allocate(orig: usize, new: usize) -> Result<u32, String> {
    unsafe fn call(f: usize, fx: &Fixture, rng: &mut Rng) -> u64 {
        let f: unsafe extern "win64" fn(u64, u64) -> u64 = core::mem::transmute(f);
        // Upper half of rdx must be ignored (`mov edx, edx`).
        f(fx.heap() - 8, (rng.next_u64() << 32) | (0x8 + rng.below(0x300)))
    }
    run_cases("memory_allocator_allocate", orig, new, call, next_alloc, u64::MAX)
}

pub unsafe fn diff_memory_allocator_aligned_allocate(orig: usize, new: usize) -> Result<u32, String> {
    unsafe fn call(f: usize, fx: &Fixture, rng: &mut Rng) -> u64 {
        let f: unsafe extern "win64" fn(u64, u64, u64, u64) -> u64 = core::mem::transmute(f);
        let align = [0u64, 8, 0x10, 0x11, 0x40, 0x1000][rng.below(6) as usize];
        // Upper halves of rdx/r8 must be ignored (`mov edx, edx; mov r8d, r8d`).
        let hi = rng.next_u64() << 32;
        f(fx.heap() - 8, hi | (0x8 + rng.below(0x300)), hi | align, rng.next_u64())
    }
    run_cases("memory_allocator_aligned_allocate", orig, new, call, next_alloc, u64::MAX)
}

pub unsafe fn diff_memory_allocator_deallocate(orig: usize, new: usize) -> Result<u32, String> {
    unsafe fn call(f: usize, fx: &Fixture, rng: &mut Rng) -> u64 {
        let f: unsafe extern "win64" fn(u64, u64, u64) -> u64 = core::mem::transmute(f);
        f(fx.heap() - 8, free_ptr(fx, rng.below(7)), rng.next_u64())
    }
    run_cases("memory_allocator_deallocate", orig, new, call, no_alloc, 0xff)
}
