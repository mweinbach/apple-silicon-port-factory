//! Differential tests for `r::mem`.

use super::*;

// ---------------------------------------------------------------------------
// Differential tests: a fake 32 MB-aligned chunk with page/span descriptors.
// The two out-of-line free routines are mocked as recorders.
// ---------------------------------------------------------------------------

use crate::diff_eq;
use crate::difftest::{Mock, Rng, Snapshot, SKIP};
use std::cell::RefCell;

thread_local! {
    static FREE_CALLS: RefCell<Vec<(&'static str, usize, usize, u8, usize)>> = const { RefCell::new(Vec::new()) };
}

unsafe extern "win64" fn fake_free_slow(chunk: usize, span: usize, owner: u64, p: usize) {
    // Only r8b is defined (the original sets it with `mov r8b, 1` / `xor r8d, r8d`).
    FREE_CALLS.with(|c| c.borrow_mut().push(("slow", chunk, span, owner as u8, p)));
}

unsafe extern "win64" fn fake_span_empty(span: usize) {
    FREE_CALLS.with(|c| c.borrow_mut().push(("empty", 0, span, 0, 0)));
}

/// A zeroed 32 MB-aligned fake chunk, freed on drop.
pub(crate) struct FakeChunk {
    pub base: usize,
    layout: std::alloc::Layout,
}

/// Page descriptors the test randomizes (data pages 1..PAGES).
pub(crate) const FAKE_PAGES: usize = 24;

impl FakeChunk {
    pub fn new() -> Result<FakeChunk, String> {
        const CHUNK: usize = 32 << 20;
        let layout = std::alloc::Layout::from_size_align(CHUNK, CHUNK).unwrap();
        let base = unsafe { std::alloc::alloc_zeroed(layout) } as usize;
        if base == 0 {
            return Err(format!("{SKIP} cannot allocate a 32 MB-aligned chunk"));
        }
        Ok(FakeChunk { base, layout })
    }

    /// Owner, page and span descriptors covered by the randomization.
    pub fn descriptors(&self) -> Snapshot {
        unsafe { Snapshot::take((self.base + 0x100) as *mut u8, 8 + FAKE_PAGES * 0x50) }
    }

    /// Randomize the owner and one page/span and return a block in that page
    /// (its first qword randomized), plus a description.
    pub unsafe fn setup(&self, r: &mut Rng) -> (usize, String) {
        let teb = exe::teb_self();
        *((self.base + 0x100) as *mut usize) = if r.below(5) == 0 { teb ^ 0x1000 } else { teb };
        let k = 1 + r.below(FAKE_PAGES as u64 - 1) as usize;
        let back = r.below(k.min(4) as u64) as usize;
        let page = self.base + 0x108 + k * 0x50;
        let span = page - back * 0x50;
        *((page + 4) as *mut u32) = (back * 0x50) as u32;
        *((span + 0xe) as *mut u8) = (r.below(4) == 0) as u8;
        *((span + 0x18) as *mut u32) = match r.below(3) {
            0 => 1,
            1 => 2,
            _ => r.next_u32(),
        };
        *((span + 0x20) as *mut usize) = r.next_u64() as usize;
        let p = self.base + k * 0x10000 + 8 * r.below(0x2000) as usize;
        *(p as *mut u64) = r.next_u64();
        (p, format!("page {k} back {back}"))
    }
}

impl Drop for FakeChunk {
    fn drop(&mut self) {
        unsafe { std::alloc::dealloc(self.base as *mut u8, self.layout) }
    }
}

/// Install the free-path recorders; `take_free_calls` drains the log.
pub(crate) fn mock_free_paths() -> Result<(Mock, Mock), String> {
    Ok((
        Mock::new(REGION_FREE_SLOW_VA, fake_free_slow as *const ())?,
        Mock::new(REGION_SPAN_EMPTY_VA, fake_span_empty as *const ())?,
    ))
}

pub(crate) fn take_free_calls() -> Vec<(&'static str, usize, usize, u8, usize)> {
    FREE_CALLS.with(|c| std::mem::take(&mut *c.borrow_mut()))
}

pub unsafe fn diff_heap_resource_deallocate(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _mocks = mock_free_paths()?;
    let mut r = Rng::new(0x14020b500);
    const CASES: u64 = 1500;
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (p, what) = chunk.setup(&mut r);
        let arg = if r.below(10) == 0 { 0 } else { p };
        let this = r.next_u64() as usize as *mut u8;
        let (pre_desc, pre_p) = (desc.current(), *(p as *const u64));
        take_free_calls();
        o(this, arg as *mut u8);
        let (desc_a, p_a, calls_a) = (desc.current(), *(p as *const u64), take_free_calls());
        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        *(p as *mut u64) = pre_p;
        n(this, arg as *mut u8);
        let (desc_b, p_b, calls_b) = (desc.current(), *(p as *const u64), take_free_calls());
        diff_eq!(calls_a, calls_b, "free calls, case {i} {what} null {}", arg == 0);
        diff_eq!(p_a, p_b, "freed block link, case {i} {what}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, case {i} {what}"));
        }
        desc.restore();
    }
    Ok(CASES as u32)
}

pub unsafe fn diff_region_free_ptr(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _mocks = mock_free_paths()?;
    let mut r = Rng::new(0x142442de0);
    const CASES: u64 = 1500;
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (p, what) = chunk.setup(&mut r);
        let arg = if r.below(10) == 0 { 0 } else { p };
        let (pre_desc, pre_p) = (desc.current(), *(p as *const u64));
        take_free_calls();
        o(arg as *mut u8);
        let (desc_a, p_a, calls_a) = (desc.current(), *(p as *const u64), take_free_calls());
        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        *(p as *mut u64) = pre_p;
        n(arg as *mut u8);
        let (desc_b, p_b, calls_b) = (desc.current(), *(p as *const u64), take_free_calls());
        diff_eq!(calls_a, calls_b, "free calls, case {i} {what} null {}", arg == 0);
        diff_eq!(p_a, p_b, "freed block link, case {i} {what}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, case {i} {what}"));
        }
        desc.restore();
    }
    Ok(CASES as u32)
}

pub unsafe fn diff_free_array(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _mocks = mock_free_paths()?;
    let mut r = Rng::new(0x142442c80);
    const CASES: u64 = 1500;
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (raw, what) = chunk.setup(&mut r);
        let align_val: u32 = match r.below(6) {
            0 => 0,
            1 => 4,
            2 => 8,
            3 => 16,
            4 => 32,
            _ => 64,
        };
        let hdr = (align_val as usize).max(0x10);
        let p = raw + hdr;
        exe::wr::<u32>(p - 4, align_val);
        let arg = if r.below(10) == 0 { 0 } else { p };
        let (pre_desc, pre_raw) = (desc.current(), exe::rd::<u64>(raw));
        take_free_calls();
        o(arg as *mut u8);
        let (desc_a, raw_a, calls_a) = (desc.current(), exe::rd::<u64>(raw), take_free_calls());
        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        exe::wr::<u64>(raw, pre_raw);
        exe::wr::<u32>(p - 4, align_val);
        n(arg as *mut u8);
        let (desc_b, raw_b, calls_b) = (desc.current(), exe::rd::<u64>(raw), take_free_calls());
        diff_eq!(calls_a, calls_b, "free calls, case {i} {what} align {align_val}");
        diff_eq!(raw_a, raw_b, "freed block link, case {i} {what} align {align_val}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, case {i} {what}"));
        }
        desc.restore();
    }
    Ok(CASES as u32)
}

pub unsafe fn diff_release_buffer(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut [usize; 2]);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _mocks = mock_free_paths()?;
    let mut r = Rng::new(0x1401de830);
    const CASES: u64 = 1500;
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (p, what) = chunk.setup(&mut r);
        let ptr = if r.below(10) == 0 { 0 } else { p };
        // The header is often only 4-aligned in the game (M3: `...69c`): place it at every
        // byte offset inside an 8-aligned buffer.
        let off = (i % 8) as usize;
        let mut hdr0 = [0u8; 24];
        hdr0[off..off + 8].copy_from_slice(&ptr.to_le_bytes());
        hdr0[off + 8..off + 16].copy_from_slice(&r.next_u64().to_le_bytes());
        let (pre_desc, pre_p) = (desc.current(), *(p as *const u64));
        let mut ha = [0u64; 3];
        std::ptr::copy_nonoverlapping(hdr0.as_ptr(), ha.as_mut_ptr() as *mut u8, 24);
        take_free_calls();
        o((ha.as_mut_ptr() as *mut u8).add(off) as *mut [usize; 2]);
        let (desc_a, p_a, calls_a) = (desc.current(), *(p as *const u64), take_free_calls());
        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        *(p as *mut u64) = pre_p;
        let mut hb = [0u64; 3];
        std::ptr::copy_nonoverlapping(hdr0.as_ptr(), hb.as_mut_ptr() as *mut u8, 24);
        n((hb.as_mut_ptr() as *mut u8).add(off) as *mut [usize; 2]);
        let (desc_b, p_b, calls_b) = (desc.current(), *(p as *const u64), take_free_calls());
        diff_eq!(ha, hb, "header, case {i} {what} offset {off}");
        diff_eq!(calls_a, calls_b, "free calls, case {i} {what} null {}", ptr == 0);
        diff_eq!(p_a, p_b, "freed block link, case {i} {what}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, case {i} {what}"));
        }
        desc.restore();
    }
    Ok(CASES as u32)
}

pub unsafe fn diff_clear_pod_vector(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _mocks = mock_free_paths()?;
    let mut r = Rng::new(0x1401f2bc0);
    const CASES: u64 = 1500;
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (p, what) = chunk.setup(&mut r);
        let ptr = if r.below(10) == 0 { 0 } else { p };
        let off = (i % 8) as usize;
        let mut hdr0 = [0u8; 24];
        hdr0[off..off + 8].copy_from_slice(&ptr.to_le_bytes());
        hdr0[off + 8..off + 12].copy_from_slice(&r.next_u32().to_le_bytes());
        let cap_flags = match r.below(4) {
            0 => r.next_u32() | 0x4000_0000,
            1 => r.next_u32() & !0x4000_0000,
            2 => (r.next_u32() & !0x4000_0000) | 0x8000_0000,
            _ => r.next_u32(),
        };
        hdr0[off + 12..off + 16].copy_from_slice(&cap_flags.to_le_bytes());
        let (pre_desc, pre_p) = (desc.current(), *(p as *const u64));
        let mut ha = [0u64; 3];
        std::ptr::copy_nonoverlapping(hdr0.as_ptr(), ha.as_mut_ptr() as *mut u8, 24);
        take_free_calls();
        o((ha.as_mut_ptr() as *mut u8).add(off));
        let (desc_a, p_a, calls_a) = (desc.current(), *(p as *const u64), take_free_calls());
        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        *(p as *mut u64) = pre_p;
        let mut hb = [0u64; 3];
        std::ptr::copy_nonoverlapping(hdr0.as_ptr(), hb.as_mut_ptr() as *mut u8, 24);
        n((hb.as_mut_ptr() as *mut u8).add(off));
        let (desc_b, p_b, calls_b) = (desc.current(), *(p as *const u64), take_free_calls());
        diff_eq!(ha, hb, "header, case {i} {what} offset {off} flags {cap_flags:#x}");
        diff_eq!(calls_a, calls_b, "free calls, case {i} {what} flags {cap_flags:#x}");
        diff_eq!(p_a, p_b, "freed block link, case {i} {what} flags {cap_flags:#x}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, case {i} {what}"));
        }
        desc.restore();
    }
    Ok(CASES as u32)
}

// ---------------------------------------------------------------------------
// Allocation difftests: a fake heap (`pages_free_direct`), fake pages and a
// block area in one arena, installed in the thread's TLS heap slot only for
// the duration of each call. `_mi_malloc_generic`, the aligned slow path,
// `GetCurrentProcess` and `TerminateProcess` are recorders.
// ---------------------------------------------------------------------------

use crate::difftest::IatMock;
use std::cell::Cell;
use std::sync::atomic::{AtomicUsize, Ordering};

type AllocCall = (&'static str, usize, usize, u64, u64);

thread_local! {
    static ALLOC_CALLS: RefCell<Vec<AllocCall>> = const { RefCell::new(Vec::new()) };
    static ALLOC_RET: Cell<usize> = const { Cell::new(0) };
    static FAKE_PROCESS: Cell<usize> = const { Cell::new(0) };
    /// Only the test thread records; other threads reach the real imports.
    static ALLOC_TEST_THREAD: Cell<bool> = const { Cell::new(false) };
}
static REAL_GET_CURRENT_PROCESS: AtomicUsize = AtomicUsize::new(0);
static REAL_TERMINATE_PROCESS: AtomicUsize = AtomicUsize::new(0);

fn record_alloc(c: AllocCall) {
    ALLOC_CALLS.with(|v| v.borrow_mut().push(c));
}

fn take_alloc_calls() -> Vec<AllocCall> {
    ALLOC_CALLS.with(|v| std::mem::take(&mut *v.borrow_mut()))
}

unsafe extern "win64" fn fake_malloc_generic(heap: usize, size: usize, zero: u64, huge: u64) -> usize {
    record_alloc(("generic", heap, size, zero, huge));
    ALLOC_RET.with(|r| r.get())
}

unsafe extern "win64" fn fake_malloc_aligned(heap: usize, size: usize, align: usize) -> usize {
    record_alloc(("aligned", heap, size, align as u64, 0));
    ALLOC_RET.with(|r| r.get())
}

unsafe extern "win64" fn fake_mi_malloc_aligned(size: usize, align: usize) -> usize {
    record_alloc(("mi_aligned", size, align, 0, 0));
    ALLOC_RET.with(|r| r.get())
}

unsafe extern "win64" fn fake_get_current_process() -> usize {
    if !ALLOC_TEST_THREAD.with(|t| t.get()) {
        return exe::call_va!(REAL_GET_CURRENT_PROCESS.load(Ordering::Relaxed) as u64, () -> usize);
    }
    record_alloc(("GetCurrentProcess", 0, 0, 0, 0));
    FAKE_PROCESS.with(|p| p.get())
}

unsafe extern "win64" fn fake_terminate_process(process: usize, code: u32) -> i32 {
    if !ALLOC_TEST_THREAD.with(|t| t.get()) {
        return exe::call_va!(
            REAL_TERMINATE_PROCESS.load(Ordering::Relaxed) as u64,
            (process, code) -> i32
        );
    }
    record_alloc(("TerminateProcess", process, code as usize, 0, 0));
    1
}

/// The recorders for one allocation difftest, plus the thread's TLS heap
/// slot (restored on drop).
struct AllocMocks {
    _generic: Mock,
    _aligned: Mock,
    _mi_aligned: Mock,
    _gcp: IatMock,
    _term: IatMock,
    slot: usize,
    saved: usize,
}

impl AllocMocks {
    unsafe fn new() -> Result<AllocMocks, String> {
        // A thread without a TEB is adopted by its first Windows API call; make
        // that happen now, not inside the original.
        let tid = exe::rd::<usize>(exe::ptr::<u8>(0x1_4264_5790) as usize);
        if tid == 0 {
            return Err(format!("{SKIP} GetCurrentThreadId is not bound"));
        }
        let _ = exe::call_va!(tid as u64, () -> u32);
        let tls = exe::tls_pointer();
        if tls == 0 || exe::rd::<usize>(tls) == 0 {
            return Err(format!("{SKIP} no exe TLS block on this thread"));
        }
        let slot = exe::rd::<usize>(tls) + HEAP_TLS_OFFSET;
        let gcp = IatMock::new(GET_CURRENT_PROCESS_IAT, fake_get_current_process as *const ())?;
        REAL_GET_CURRENT_PROCESS.store(gcp.original(), Ordering::Relaxed);
        let term = IatMock::new(TERMINATE_PROCESS_IAT, fake_terminate_process as *const ())?;
        REAL_TERMINATE_PROCESS.store(term.original(), Ordering::Relaxed);
        ALLOC_TEST_THREAD.with(|t| t.set(true));
        Ok(AllocMocks {
            _generic: Mock::new(MALLOC_GENERIC_VA, fake_malloc_generic as *const ())?,
            _aligned: Mock::new(MALLOC_ALIGNED_VA, fake_malloc_aligned as *const ())?,
            _mi_aligned: Mock::new(MI_MALLOC_ALIGNED_VA, fake_mi_malloc_aligned as *const ())?,
            _gcp: gcp,
            _term: term,
            slot,
            saved: exe::rd::<usize>(slot),
        })
    }
}

impl Drop for AllocMocks {
    fn drop(&mut self) {
        ALLOC_TEST_THREAD.with(|t| t.set(false));
        unsafe { exe::wr::<usize>(self.slot, self.saved) };
    }
}

/// Arena layout (byte offsets; `mis` shifts the heap and the pages by 0 or 4
/// so the descriptors are under-aligned in half the cases).
const A_HEAP: usize = 0x10;
const A_PAGES: usize = 0x500;
const A_PAGE_STRIDE: usize = 0x30;
const A_NPAGES: usize = 8;
const A_BLOCKS: usize = 0x800;
const A_SIZE: usize = 0x1000;

struct AllocArena {
    mem: Vec<u64>,
}

impl AllocArena {
    fn new() -> AllocArena {
        AllocArena { mem: vec![0u64; A_SIZE / 8] }
    }

    fn base(&self) -> usize {
        self.mem.as_ptr() as usize
    }

    /// Randomize everything; returns the heap address.
    unsafe fn setup(&mut self, r: &mut Rng, align: usize) -> usize {
        for w in self.mem.iter_mut() {
            *w = r.next_u64();
        }
        let base = self.base();
        let mis = 4 * r.below(2) as usize;
        let heap = base + A_HEAP + mis;
        let pages = base + A_PAGES + mis;
        for k in 0..A_NPAGES {
            let page = pages + k * A_PAGE_STRIDE;
            let blk = base + A_BLOCKS;
            let free = match r.below(5) {
                0 => 0,
                1 => blk + 16 * r.below(0x40) as usize,
                2 => blk + 16 * r.below(0x40) as usize + 8,
                3 => blk + align * r.below((0x400 / align) as u64) as usize,
                _ => blk + 8 * r.below(0x100) as usize,
            };
            exe::wr::<usize>(page + 0x10, free);
            if r.below(4) == 0 {
                exe::wr::<u32>(page + 0x18, u32::MAX);
            }
        }
        for i in 0..=129 {
            exe::wr::<usize>(heap + 8 + 8 * i, pages + A_PAGE_STRIDE * r.below(A_NPAGES as u64) as usize);
        }
        heap
    }

    /// A random allocator result: null, a block, or garbage.
    fn ret(&self, r: &mut Rng) -> usize {
        match r.below(5) {
            0 => 0,
            1 | 2 => self.base() + A_BLOCKS + 8 * r.below(0x100) as usize,
            _ => r.next_u64() as usize,
        }
    }
}

/// One original-vs-replacement comparison of an allocation entry: same arena,
/// heap slot and recorder results; compares the result, the calls and the arena.
unsafe fn alloc_case(
    mocks: &AllocMocks,
    arena: &mut AllocArena,
    heap: usize,
    run: &dyn Fn(bool) -> usize,
) -> Result<(), String> {
    let before = arena.mem.clone();
    let mut out = Vec::new();
    for use_orig in [true, false] {
        arena.mem.copy_from_slice(&before);
        take_alloc_calls();
        exe::wr::<usize>(mocks.slot, heap);
        let ret = run(use_orig);
        exe::wr::<usize>(mocks.slot, mocks.saved);
        out.push((ret, take_alloc_calls(), arena.mem.clone()));
    }
    let (a, b) = (&out[0], &out[1]);
    diff_eq!(a.0, b.0, "result");
    diff_eq!(a.1, b.1, "calls");
    if a.2 != b.2 {
        let i = a.2.iter().zip(&b.2).position(|(x, y)| x != y).unwrap_or(0);
        return Err(format!("arena differs at +0x{:x}: orig={:#x} new={:#x}", 8 * i, a.2[i], b.2[i]));
    }
    Ok(())
}

/// Element counts that cover the paths for an `elem`-byte element: tiny,
/// both sides of the 0x400 small limit, powers of two, huge (wrapping) and
/// around `isize::MAX`.
fn alloc_count(r: &mut Rng, elem: usize) -> usize {
    let e = elem as u64;
    (match r.below(8) {
        0 => r.below(3),
        1..=3 => r.below(0x440 / e + 2),
        4 => {
            let s = 1u64 << r.below(12);
            if s % e == 0 {
                s / e
            } else {
                r.below(0x440 / e + 2)
            }
        }
        5 => r.next_u64(),
        6 => ((1u64 << 63) / e).wrapping_add(r.below(3)).wrapping_sub(1),
        _ => r.below(0x10000),
    }) as usize
}

pub unsafe fn diff_stl_allocate<const ELEM: usize>(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x5a110c ^ ELEM as u64);
    const CASES: u32 = 600;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, 16);
        let count = alloc_count(&mut r, ELEM);
        let this = r.next_u64() as usize as *mut u8;
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| (if use_orig { o } else { n })(this, count) as usize)
            .map_err(|e| format!("case {i} n {count:#x} size {:#x}: {e}", count.wrapping_mul(ELEM)))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_region_alloc_aligned<const ALIGN: usize>(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0xa110c0 ^ ALIGN as u64);
    const CASES: u32 = 600;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, ALIGN);
        let size = alloc_count(&mut r, 1);
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| (if use_orig { o } else { n })(size) as usize)
            .map_err(|e| format!("case {i} size {size:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_alloc_aligned(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x142442e70);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let heap = arena.base();
        let size = alloc_count(&mut r, 1);
        let align = match r.below(6) {
            0 => 0,
            1 => 4,
            2 => 8,
            3 => 16,
            4 => 32,
            _ => r.next_u64() as usize,
        };
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| (if use_orig { o } else { n })(size, align) as usize)
            .map_err(|e| format!("case {i} size {size:#x} align {align:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_heap_resource_allocate(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize, usize) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x1413b58c0);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let heap = arena.base();
        let this = r.next_u64() as usize as *mut u8;
        let size = alloc_count(&mut r, 1);
        let align = r.next_u64() as usize;
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| (if use_orig { o } else { n })(this, size, align) as usize)
            .map_err(|e| format!("case {i} size {size:#x} align {align:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_alloc_array(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x142442d30);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let heap = arena.base();
        let elem_size = match r.below(4) {
            0 => 0,
            1 => 1 + r.below(64) as usize,
            _ => r.next_u64() as usize,
        };
        let count = match r.below(4) {
            0 => 0,
            1 => 1 + r.below(64) as usize,
            _ => r.next_u64() as usize,
        };
        let align = match r.below(6) {
            0 => 0,
            1 => 8,
            2 => 16,
            3 => 32,
            4 => 64,
            _ => (16 + 8 * r.below(8)) as usize,
        };
        let hdr = align.max(0x10);
        // Point ALLOC_RET inside the arena so both original and replacement write
        // the 16-byte header into `arena.mem` (even when OOM recovery proceeds).
        let raw_buf = arena.base() + A_BLOCKS + 8 * r.below(0x40) as usize;
        // note: if OOM (ret == 0), original does `xor eax, eax; add rax, rbx; mov [rax-0x10], ...`
        // which would write to address `hdr - 0x10` (null page!) if `mi_malloc_aligned` returned 0!
        // So in the difftest, `mi_malloc_aligned` must return a valid buffer inside `arena`.
        let _ = hdr;
        ALLOC_RET.with(|c| c.set(raw_buf));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| (if use_orig { o } else { n })(elem_size, count, align) as usize)
            .map_err(|e| format!("case {i} elem {elem_size:#x} count {count:#x} align {align:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_release_inplace_buffer<const ICAP: u32>(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _mocks = mock_free_paths()?;
    let mut r = Rng::new(0x1401ee280 ^ ICAP as u64);
    const CASES: u64 = 1000;
    let mut h = [0u64; 3];
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (p, what) = chunk.setup(&mut r);
        let off = (i % 8) as usize;
        let hdr_addr = (h.as_ptr() as usize) + off;
        let ptr = match r.below(6) {
            0 => 0,
            1 | 2 => hdr_addr.wrapping_add(0x10),
            _ => p,
        };
        let mut hdr0 = [0u8; 24];
        hdr0[off..off + 8].copy_from_slice(&ptr.to_le_bytes());
        hdr0[off + 8..off + 16].copy_from_slice(&r.next_u64().to_le_bytes());
        let (pre_desc, pre_p) = (desc.current(), *(p as *const u64));

        std::ptr::copy_nonoverlapping(hdr0.as_ptr(), h.as_mut_ptr() as *mut u8, 24);
        take_free_calls();
        o((h.as_mut_ptr() as *mut u8).add(off));
        let (ha, desc_a, p_a, calls_a) = (h, desc.current(), *(p as *const u64), take_free_calls());

        std::ptr::copy_nonoverlapping(pre_desc.as_ptr(), (chunk.base + 0x100) as *mut u8, pre_desc.len());
        *(p as *mut u64) = pre_p;
        std::ptr::copy_nonoverlapping(hdr0.as_ptr(), h.as_mut_ptr() as *mut u8, 24);
        n((h.as_mut_ptr() as *mut u8).add(off));
        let (hb, desc_b, p_b, calls_b) = (h, desc.current(), *(p as *const u64), take_free_calls());

        diff_eq!(ha, hb, "header, case {i} {what} offset {off}");
        diff_eq!(calls_a, calls_b, "free calls, case {i} {what}");
        diff_eq!(p_a, p_b, "freed block link, case {i} {what}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, case {i} {what}"));
        }
        desc.restore();
    }
    Ok(CASES as u32)
}

unsafe extern "win64" fn fake_vec_memmove(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    record_alloc(("memmove", dst as usize, src as usize, size as u64, 0));
    dst
}

unsafe extern "win64" fn fake_vec_release(vec: *mut u8) {
    let v = vec as usize;
    record_alloc(("release", v, exe::rd::<usize>(v), exe::rd::<u32>(v + 8) as u64, exe::rd::<u32>(v + 0x0c) as u64));
    exe::wr::<usize>(v, 0xdead_beef);
    exe::wr::<u32>(v + 0x08, 0);
    exe::wr::<u32>(v + 0x0c, 0x1234_5678);
}

pub unsafe fn diff_pod_vec_realloc<const ELEM: usize, const RELEASE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let _m_move = Mock::new(J_MEMMOVE_VA, fake_vec_memmove as *const ())?;
    let _m_rel = Mock::new(RELEASE_VA, fake_vec_release as *const ())?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x1408746b0 ^ ELEM as u64 ^ RELEASE_VA);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, 16);
        let vec_ptr = (arena.base() + 0x460 + 4 * r.below(2) as usize) as *mut u8;
        let new_cap = alloc_count(&mut r, ELEM);
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| {
            (if use_orig { o } else { n })(vec_ptr, new_cap);
            0
        })
        .map_err(|e| format!("case {i} new_cap {new_cap:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_pod_vec_grow<const ELEM: usize, const RELEASE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let _m_move = Mock::new(J_MEMMOVE_VA, fake_vec_memmove as *const ())?;
    let _m_rel = Mock::new(RELEASE_VA, fake_vec_release as *const ())?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x14020afa0 ^ ELEM as u64 ^ RELEASE_VA);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, 16);
        let vec_ptr = (arena.base() + 0x460 + 4 * r.below(2) as usize) as *mut u8;
        let old_cap = match r.below(5) {
            0 => 0,
            1 => r.below(32) as u32,
            2 => (0x400 / ELEM.max(1)) as u32,
            3 => u32::MAX,
            _ => r.next_u32(),
        };
        exe::wr::<u32>(vec_ptr as usize + 0x0c, old_cap);
        let min_cap = match r.below(4) {
            0 => 0,
            1 => (old_cap as usize) + ((old_cap as usize) >> 1),
            _ => alloc_count(&mut r, ELEM),
        };
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| {
            (if use_orig { o } else { n })(vec_ptr, min_cap);
            0
        })
        .map_err(|e| format!("case {i} old_cap {old_cap:#x} min_cap {min_cap:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_inplace_vec_realloc<
    const ELEM: usize,
    const ICAP: usize,
    const RELEASE_VA: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let _m_move = Mock::new(J_MEMMOVE_VA, fake_vec_memmove as *const ())?;
    let _m_rel = Mock::new(RELEASE_VA, fake_vec_release as *const ())?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x1413aa3e0 ^ ELEM as u64 ^ ICAP as u64);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, 16);
        let vec_ptr = (arena.base() + 0x460 + 4 * r.below(2) as usize) as *mut u8;
        if r.below(3) == 0 {
            exe::wr::<usize>(vec_ptr as usize, (vec_ptr as usize).wrapping_add(0x10));
        }
        let new_cap = match r.below(5) {
            0 => 0,
            1 => ICAP.saturating_sub(1),
            2 => ICAP,
            3 => ICAP + 1,
            _ => alloc_count(&mut r, ELEM),
        };
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| {
            (if use_orig { o } else { n })(vec_ptr, new_cap);
            0
        })
        .map_err(|e| format!("case {i} new_cap {new_cap:#x}: {e}"))?;
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_stream_memcpy(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    record_alloc(("memcpy", dst as usize, src as usize, size as u64, 0));
    dst
}

unsafe extern "win64" fn fake_stream_free(p: *mut u8) {
    record_alloc(("free", p as usize, 0, 0, 0));
}

pub unsafe fn diff_memory_stream_grow(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let _m_cpy = Mock::new(0x1_4183_acbc, fake_stream_memcpy as *const ())?;
    let _m_free = Mock::new(0x1_4244_2de0, fake_stream_free as *const ())?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x1424b48d0);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, 16);
        // Place the stream's +0x108..+0x138 fields at `arena.base() + 0x450`
        // (`this = arena.base() + 0x450 - 0x108`), which is unused by `A_HEAP`/`A_PAGES`.
        let s = arena.base() + 0x450 - 0x108;
        let old_base = if r.below(3) == 0 { 0 } else { 0x1000_0000usize + r.below(0x1000) as usize };
        let old_cap = match r.below(4) {
            0 => 0,
            1 => r.below(1024) as usize,
            _ => alloc_count(&mut r, 1),
        };
        let old_cur = if r.below(3) == 0 {
            0
        } else {
            old_base.wrapping_add(r.below(1024) as usize)
        };
        let old_wlen = if r.below(3) == 0 { 0 } else { r.below(1024) as usize };
        let is_ext = (r.below(2) == 0) as u8;
        let min_cap = alloc_count(&mut r, 1);

        exe::wr::<usize>(s + 0x110, old_base);
        exe::wr::<usize>(s + 0x118, old_cap);
        exe::wr::<usize>(s + 0x120, old_cur);
        exe::wr::<u8>(s + 0x128, is_ext);
        exe::wr::<usize>(s + 0x130, old_wlen);

        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));

        alloc_case(&mocks, &mut arena, heap, &|use_orig| {
            (if use_orig { o } else { n })(s as *mut u8, min_cap);
            0
        })
        .map_err(|e| format!("case {i} min_cap {min_cap:#x}: {e}"))?;
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_vec_memset(dst: *mut u8, val: i32, size: usize) -> *mut u8 {
    record_alloc(("memset", dst as usize, val as usize, size as u64, 0));
    dst
}

pub unsafe fn diff_calloc_array<const ELEM: usize>(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, usize, usize, usize) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let _m_set = Mock::new(J_MEMSET_VA, fake_vec_memset as *const ())?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x14020c080 ^ ELEM as u64);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, 16);
        let a0 = r.next_u64() as usize;
        let a1 = r.next_u64() as usize;
        let a2 = r.next_u64() as usize;
        let count = alloc_count(&mut r, ELEM);
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| {
            (if use_orig { o } else { n })(a0, a1, a2, count) as usize
        })
        .map_err(|e| format!("case {i} count {count:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_pod_vec_ctor_zero<const ELEM: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize) -> *mut u8;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let _m_set = Mock::new(J_MEMSET_VA, fake_vec_memset as *const ())?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x1411db3a0 ^ ELEM as u64);
    const CASES: u32 = 400;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, 16);
        let vec_ptr = (arena.base() + 0x460 + 4 * r.below(2) as usize) as *mut u8;
        let count = alloc_count(&mut r, ELEM);
        ALLOC_RET.with(|c| c.set(arena.ret(&mut r)));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| {
            (if use_orig { o } else { n })(vec_ptr, count) as usize
        })
        .map_err(|e| format!("case {i} count {count:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_pod_vec_back_48(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e7b20);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let mut buf = [0u8; 24];
        let off = (i as usize) % 8;
        let p = buf.as_mut_ptr().add(off);
        exe::wr::<u64>(p as usize, r.next_u64());
        exe::wr::<u32>(p as usize + 8, r.next_u32());
        diff_eq!(o(p), n(p), "case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_pod_vec_penultimate_48(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e7b40);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let mut buf = [0u8; 24];
        let off = (i as usize) % 8;
        let p = buf.as_mut_ptr().add(off);
        exe::wr::<u64>(p as usize, r.next_u64());
        exe::wr::<u32>(p as usize + 8, r.next_u32());
        diff_eq!(o(p), n(p), "case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_comp_span_iter_begin(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e7c10);
    let mut table = vec![0u8; 0x10048 + 4 * 64];
    const CASES: u32 = 300;
    for i in 0..CASES {
        for idx in 0..64usize {
            let p = if r.below(2) == 0 { 0 } else { r.next_u64() };
            let c = if r.below(2) == 0 { 0 } else { r.next_u32() };
            exe::wr::<u64>(table.as_mut_ptr() as usize + 0x48 + idx * 8, p);
            exe::wr::<u32>(table.as_mut_ptr() as usize + 0x10048 + idx * 4, c);
        }
        let count = r.below(16) as usize;
        let mut indices = vec![0u16; count.max(1)];
        for x in &mut indices {
            *x = r.below(64) as u16;
        }
        let span: [u64; 2] = [count as u64, indices.as_ptr() as u64];
        let mut out_a = [0u64; 3];
        let mut out_b = [0u64; 3];
        let ra = o(out_a.as_mut_ptr() as *mut u8, table.as_ptr(), span.as_ptr() as *const u8);
        let rb = n(out_b.as_mut_ptr() as *mut u8, table.as_ptr(), span.as_ptr() as *const u8);
        diff_eq!(ra - out_a.as_ptr() as usize, rb - out_b.as_ptr() as usize, "ret, case {i}");
        diff_eq!(out_a, out_b, "out, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_comp_span_iter_deref(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u16) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e7c60);
    const CASES: u32 = 100;
    for i in 0..CASES {
        let val = r.next_u32() as u16;
        let it: [u64; 2] = [r.next_u64(), &val as *const u16 as u64];
        let (mut oa, mut ob) = (0u16, 0u16);
        let ra = o(it.as_ptr() as *const u8, &mut oa);
        let rb = n(it.as_ptr() as *const u8, &mut ob);
        diff_eq!(ra - &oa as *const u16 as usize, rb - &ob as *const u16 as usize, "ret, case {i}");
        diff_eq!(oa, ob, "val, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_comp_span_iter_next(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e7c70);
    let mut table = vec![0u8; 0x10048 + 4 * 64];
    const CASES: u32 = 300;
    for i in 0..CASES {
        for idx in 0..64usize {
            let p = if r.below(2) == 0 { 0 } else { r.next_u64() };
            let c = if r.below(2) == 0 { 0 } else { r.next_u32() };
            exe::wr::<u64>(table.as_mut_ptr() as usize + 0x48 + idx * 8, p);
            exe::wr::<u32>(table.as_mut_ptr() as usize + 0x10048 + idx * 4, c);
        }
        let count = (1 + r.below(16)) as usize;
        let mut indices = vec![0u16; count];
        for x in &mut indices {
            *x = r.below(64) as u16;
        }
        let base = indices.as_ptr() as u64;
        let end = unsafe { indices.as_ptr().add(count) } as u64;
        let mut it_a: [u64; 3] = [table.as_ptr() as u64, base, end];
        let mut it_b = it_a;
        let ra = o(it_a.as_mut_ptr() as *mut u8);
        let rb = n(it_b.as_mut_ptr() as *mut u8);
        diff_eq!(ra - it_a.as_ptr() as usize, rb - it_b.as_ptr() as usize, "ret, case {i}");
        diff_eq!(it_a, it_b, "it, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_comp_outer_iter_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize, *const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e7cd0);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let ctx = r.next_u64() as usize;
        let inner = [r.next_u64(), r.next_u64(), r.next_u64()];
        let mut out_a = [r.next_u64(); 7];
        let mut out_b = out_a;
        let ra = o(out_a.as_mut_ptr() as *mut u8, ctx, inner.as_ptr() as *const u8);
        let rb = n(out_b.as_mut_ptr() as *mut u8, ctx, inner.as_ptr() as *const u8);
        diff_eq!(ra - out_a.as_ptr() as usize, rb - out_b.as_ptr() as usize, "ret, case {i}");
        diff_eq!(out_a, out_b, "out, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_comp_outer_iter_load(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1401e7d10);
    let mut table = vec![0u8; 0x10048 + 4 * 64];
    const CASES: u32 = 200;
    for i in 0..CASES {
        let idx = r.below(64) as u16;
        let ptr = if r.below(2) == 0 { 0 } else { r.next_u64() };
        let cnt = r.next_u32();
        exe::wr::<u64>(table.as_mut_ptr() as usize + 0x48 + (idx as usize) * 8, ptr);
        exe::wr::<u32>(table.as_mut_ptr() as usize + 0x10048 + (idx as usize) * 4, cnt);
        let cur_ptr = &idx as *const u16 as u64;
        let end_ptr = if r.below(3) == 0 { cur_ptr } else { cur_ptr + 2 };
        let mut it_a = [
            table.as_ptr() as u64,
            r.next_u64(),
            cur_ptr,
            end_ptr,
            r.next_u64(),
            r.next_u64(),
            r.next_u64(),
        ];
        let mut it_b = it_a;
        o(it_a.as_mut_ptr() as *mut u8);
        n(it_b.as_mut_ptr() as *mut u8);
        diff_eq!(it_a, it_b, "it, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_j_free_sized(p: *mut u8, size: usize) {
    record_alloc(("j_free_sized", p as usize, size, 0, 0));
}

pub unsafe fn diff_scalar_deleting_dtor<const SIZE: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(J_FREE_SIZED_VA, fake_j_free_sized as *const ())?;
    let mut r = Rng::new(0x1401ed460 ^ SIZE as u64);
    const CASES: u32 = 120;
    for i in 0..CASES {
        let this = r.next_u64() as usize as *mut u8;
        let flags = r.next_u32() as u8;
        take_alloc_calls();
        let ra = o(this, flags);
        let ca = take_alloc_calls();
        let rb = n(this, flags);
        let cb = take_alloc_calls();
        diff_eq!(ra, rb, "ret, case {i}");
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_vtbl_dtor_0(this: *mut u8, flags: u32) {
    record_alloc(("vtbl_dtor_0", this as usize, flags as usize, 0, 0));
}

unsafe extern "win64" fn fake_vtbl_dtor_8(this: *mut u8, flags: u32) {
    record_alloc(("vtbl_dtor_8", this as usize, flags as usize, 0, 0));
}

pub unsafe fn diff_unique_ptr_dtor<const VTBL_OFF: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_free = Mock::new(REGION_FREE_PTR_VA, fake_stream_free as *const ())?;
    let vtbl: [usize; 2] = [
        fake_vtbl_dtor_0 as *const () as usize,
        fake_vtbl_dtor_8 as *const () as usize,
    ];
    let obj: [usize; 2] = [vtbl.as_ptr() as usize, 0x1234_5678];
    let mut r = Rng::new(0x1401e18a0 ^ VTBL_OFF as u64);
    const CASES: u32 = 120;
    for i in 0..CASES {
        let mut holder = [0u8; 24];
        let off = (i as usize) % 8;
        let p = if r.below(3) == 0 {
            0usize
        } else {
            obj.as_ptr() as usize
        };
        exe::wr::<usize>(holder.as_mut_ptr() as usize + off, p);
        take_alloc_calls();
        o(holder.as_mut_ptr().add(off));
        let ca = take_alloc_calls();
        exe::wr::<usize>(holder.as_mut_ptr() as usize + off, p);
        n(holder.as_mut_ptr().add(off));
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_scratch_free_ptr(p: *mut u8) {
    record_alloc(("scratch_free", p as usize, 0, 0, 0));
}

pub unsafe fn diff_scratch_vec_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(SCRATCH_FREE_VA, fake_scratch_free_ptr as *const ())?;
    let mut r = Rng::new(0x1401e68b0);
    const CASES: u32 = 150;
    for i in 0..CASES {
        let off = (i as usize) % 8;
        let p = if r.below(3) == 0 { 0 } else { r.next_u64() };
        let w1 = r.next_u64();
        let mut ba = [0u8; 32];
        exe::wr::<u64>(ba.as_mut_ptr() as usize + off, p);
        exe::wr::<u64>(ba.as_mut_ptr() as usize + off + 8, w1);
        let mut bb = ba;
        take_alloc_calls();
        o(ba.as_mut_ptr().add(off));
        let ca = take_alloc_calls();
        n(bb.as_mut_ptr().add(off));
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i}");
        diff_eq!(ba, bb, "buf, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_rep_destroy(rep: *mut u8) {
    record_alloc(("rep_destroy", rep as usize, 0, 0, 0));
}

unsafe extern "win64" fn fake_rep_delete_this(rep: *mut u8) {
    record_alloc(("rep_delete_this", rep as usize, 0, 0, 0));
}

pub unsafe fn diff_shared_ptr_dtor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let vtbl: [usize; 2] = [
        fake_rep_destroy as *const () as usize,
        fake_rep_delete_this as *const () as usize,
    ];
    let mut r = Rng::new(0x140200d90);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let uses = 1 + r.below(3) as u32;
        let weaks = 1 + r.below(3) as u32;
        let is_null = r.below(4) == 0;
        let mut rep_a: [u64; 2] = [vtbl.as_ptr() as u64, (uses as u64) | ((weaks as u64) << 32)];
        let mut rep_b = rep_a;
        let sp_a: [usize; 2] = [
            r.next_u64() as usize,
            if is_null { 0 } else { rep_a.as_mut_ptr() as usize },
        ];
        let sp_b: [usize; 2] = [
            sp_a[0],
            if is_null { 0 } else { rep_b.as_mut_ptr() as usize },
        ];
        take_alloc_calls();
        o(sp_a.as_ptr() as *mut u8);
        let mut ca = take_alloc_calls();
        for c in &mut ca {
            c.1 = 1;
        }
        n(sp_b.as_ptr() as *mut u8);
        let mut cb = take_alloc_calls();
        for c in &mut cb {
            c.1 = 1;
        }
        diff_eq!(rep_a, rep_b, "rep, case {i}");
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_swiss_init_slots<const ALIGN: usize, const SLOT: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mocks = AllocMocks::new()?;
    let _m_set = Mock::new(J_MEMSET_VA, fake_vec_memset as *const ())?;
    let mut arena = AllocArena::new();
    let mut r = Rng::new(0x1401de6e0 ^ ((ALIGN as u64) << 16) ^ SLOT as u64);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let heap = arena.setup(&mut r, 16);
        let tbl_ptr = (arena.base() + 0x450 + 4 * r.below(2) as usize) as *mut u8;
        let cap = match r.below(5) {
            0 => 0,
            1 => 7,
            2 => 15,
            3 => 31,
            _ => r.below(64) as usize,
        };
        let raw_buf = arena.base() + A_BLOCKS + 16 * r.below(0x20) as usize;
        ALLOC_RET.with(|c| c.set(raw_buf));
        FAKE_PROCESS.with(|p| p.set(r.next_u64() as usize));
        alloc_case(&mocks, &mut arena, heap, &|use_orig| {
            (if use_orig { o } else { n })(tbl_ptr, cap);
            0
        })
        .map_err(|e| format!("case {i} cap {cap:#x}: {e}"))?;
    }
    Ok(CASES)
}

pub unsafe fn diff_rb_tree_insert_node(orig: usize, new: usize) -> Result<u32, String> {
    use exe::{rd, wr};
    type F = unsafe extern "win64" fn(*mut u8, *const u8, *mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x140208500);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let count = 1 + (r.below(14) as usize);
        let mut keys = Vec::with_capacity(count);
        for _ in 0..count {
            keys.push(r.next_u32());
        }
        // Build identical trees in `mem_a` and `mem_b`:
        // +0x00..0x10: tree header `{ head: *mut Node, size: u64 }`
        // +0x20..: 16 nodes of 32 bytes (`left`, `parent`, `right`, `color: u8`, `is_nil: u8`, `key: u32` at +0x1c)
        let mut mem_a = [0u8; 0x240];
        let mut mem_b = [0u8; 0x240];
        let run_seq = |mem: &mut [u8; 0x240], f: F| {
            let base = mem.as_mut_ptr() as usize;
            let head = base + 0x20;
            wr::<usize>(base, head);
            wr::<u64>(base + 0x08, 0);
            wr::<usize>(head, head);
            wr::<usize>(head + 0x08, head);
            wr::<usize>(head + 0x10, head);
            wr::<u8>(head + 0x18, 1);
            wr::<u8>(head + 0x19, 1);
            for (idx, &k) in keys.iter().enumerate() {
                let node = head + (idx + 1) * 0x20;
                wr::<usize>(node, head);
                wr::<usize>(node + 0x08, head);
                wr::<usize>(node + 0x10, head);
                wr::<u8>(node + 0x18, 0);
                wr::<u8>(node + 0x19, 0);
                wr::<u32>(node + 0x1c, k);
                // Find insertion parent + side
                let mut p = head;
                let mut cur = rd::<usize>(head + 0x08);
                let mut left_side = 1u32;
                while rd::<u8>(cur + 0x19) == 0 {
                    p = cur;
                    if k < rd::<u32>(cur + 0x1c) {
                        left_side = 1;
                        cur = rd::<usize>(cur);
                    } else {
                        left_side = 0;
                        cur = rd::<usize>(cur + 0x10);
                    }
                }
                let mut loc = [0u8; 16];
                wr::<usize>(loc.as_mut_ptr() as usize, p);
                wr::<u32>(loc.as_mut_ptr() as usize + 8, left_side);
                let ret = f(base as *mut u8, loc.as_ptr(), node as *mut u8);
                assert_eq!(ret, node);
            }
        };
        run_seq(&mut mem_a, o);
        run_seq(&mut mem_b, n);
        // Normalize pointers relative to `mem_a` / `mem_b` base before comparing
        let base_a = mem_a.as_ptr() as usize;
        let base_b = mem_b.as_ptr() as usize;
        for off in (0..0x240).step_by(8) {
            let mut va = rd::<usize>(base_a + off);
            let mut vb = rd::<usize>(base_b + off);
            if (base_a..base_a + 0x240).contains(&va) {
                va -= base_a;
            }
            if (base_b..base_b + 0x240).contains(&vb) {
                vb -= base_b;
            }
            diff_eq!(va, vb, "word +{off:#x}, case {i}");
        }
    }
    Ok(CASES)
}

pub unsafe fn diff_rwlock_wait_and_add(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u64, usize, u64) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x141379040);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let init = r.next_u32() as u64; // upper 32 bits must be 0 so it doesn't spin forever
        let unused = r.next_u64() as usize;
        let delta = r.next_u64();
        let (mut la, mut lb) = (init, init);
        let ra = o(&mut la, unused, delta);
        let rb = n(&mut lb, unused, delta);
        diff_eq!(ra, rb, "ret, case {i}");
        diff_eq!(la, lb, "lock, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_get_tid() -> u32 {
    0x4242_1337
}

pub unsafe fn diff_rwlock_acquire_write(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_tid = IatMock::new(GET_CURRENT_THREAD_ID_IAT, fake_get_tid as *const ());
    let mut r = Rng::new(0x141378fb0);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let same_tid = r.below(2) == 0;
        let state = if same_tid {
            r.next_u64()
        } else {
            0u64 // no active writer or reader so it acquires without blocking
        };
        let writer_tid = if same_tid {
            0x4242_1337u64
        } else {
            r.next_u32() as u64
        };
        let mut la: [u64; 2] = [state, writer_tid];
        let mut lb = la;
        o(la.as_mut_ptr() as *mut u8);
        n(lb.as_mut_ptr() as *mut u8);
        diff_eq!(la, lb, "lock, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_hash_map_find_u64(orig: usize, new: usize) -> Result<u32, String> {
    use exe::wr;
    type F = unsafe extern "win64" fn(*const u8, *mut *mut u8, *const u64) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1413790c0);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let mut head = [0u64; 4];
        let head_addr = head.as_mut_ptr() as usize;
        // 8 buckets (mask = 7), each 16 bytes `{ first, last }`
        let mut buckets = [head_addr; 16];
        // 4 chained nodes of 24 bytes `{ prev, next, key }`
        let mut nodes = [[0u64; 3]; 4];
        for nd in &mut nodes {
            nd[2] = r.next_u64();
        }
        nodes[0][1] = nodes[1].as_ptr() as u64;
        nodes[1][1] = nodes[2].as_ptr() as u64;
        nodes[2][1] = nodes[3].as_ptr() as u64;
        nodes[3][1] = head_addr as u64;
        let target_b = (r.below(8) as usize) * 2;
        if r.below(3) != 0 {
            buckets[target_b] = nodes[3].as_ptr() as usize; // first (stop limit)
            buckets[target_b + 1] = nodes[0].as_ptr() as usize; // last (chain start)
        }
        let mut tbl = [0u8; 0x38];
        wr::<usize>(tbl.as_mut_ptr() as usize + 0x08, head_addr);
        wr::<usize>(tbl.as_mut_ptr() as usize + 0x18, buckets.as_ptr() as usize);
        wr::<u64>(tbl.as_mut_ptr() as usize + 0x30, 7);
        let query_key = if r.below(2) == 0 {
            nodes[r.below(4) as usize][2]
        } else {
            r.next_u64()
        };
        let (mut oa, mut ob) = (std::ptr::null_mut(), std::ptr::null_mut());
        let ra = o(tbl.as_ptr(), &mut oa, &query_key);
        let rb = n(tbl.as_ptr(), &mut ob, &query_key);
        diff_eq!(ra - &oa as *const _ as usize, rb - &ob as *const _ as usize, "ret, case {i}");
        diff_eq!(oa as usize, ob as usize, "found node, case {i}");
    }
    Ok(CASES)
}

thread_local! {
    static FAKE_OFFSETS: Cell<[usize; 16]> = const { Cell::new([0; 16]) };
}

unsafe extern "win64" fn fake_offset_getter<const N: usize>(
    out: *mut usize,
    _rdx: usize,
    r8: usize,
    r9: u32,
) -> *mut usize {
    record_alloc(("offset_getter", r8, r9 as usize, N as u64, 0));
    let offs = FAKE_OFFSETS.with(|c| c.get());
    let mut k = 0usize;
    while k < N {
        exe::wr::<usize>(out as usize + 8 * k, offs[k]);
        k += 1;
    }
    out
}

pub unsafe fn diff_ecs_tuple_iter_ctor<const N: usize, const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, u16, usize, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_offset_getter::<N> as *const ())?;
    let mut r = Rng::new(0x14022f120 ^ ((N as u64) << 16) ^ CALLEE_VA);
    const CASES: u32 = 120;
    for i in 0..CASES {
        let mut offs = [0usize; 16];
        for k in 0..N {
            offs[k] = if r.below(3) == 0 {
                0
            } else {
                r.next_u64() as usize
            };
        }
        FAKE_OFFSETS.with(|c| c.set(offs));
        let bucket_idx = r.next_u32() as u16;
        let base_ptr = r.next_u64() as usize;
        let count = r.next_u64() as usize;
        let ctx = r.next_u64() as usize;
        let mut out_a = [0u64; 20];
        let mut out_b = [0u64; 20];
        take_alloc_calls();
        let ra = o(out_a.as_mut_ptr() as *mut u8, bucket_idx, base_ptr, count, ctx);
        let ca = take_alloc_calls();
        let rb = n(out_b.as_mut_ptr() as *mut u8, bucket_idx, base_ptr, count, ctx);
        let cb = take_alloc_calls();
        diff_eq!(
            ra - out_a.as_ptr() as usize,
            rb - out_b.as_ptr() as usize,
            "ret, case {i}"
        );
        diff_eq!(ca, cb, "calls, case {i}");
        diff_eq!(out_a, out_b, "out, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_ecs_outer_iter_load_components<const N: usize, const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_offset_getter::<N> as *const ())?;
    let mut r = Rng::new(0x1405ef040 ^ ((N as u64) << 16) ^ CALLEE_VA);
    const CASES: u32 = 120;
    for i in 0..CASES {
        let mut offs = [0usize; 16];
        for k in 0..N {
            offs[k] = if r.below(3) == 0 {
                0
            } else {
                r.next_u64() as usize
            };
        }
        FAKE_OFFSETS.with(|c| c.set(offs));
        let bucket_idx = r.next_u32() as u16;
        let cur_ptr = &bucket_idx as *const u16 as u64;
        let end_ptr = if r.below(4) == 0 { cur_ptr } else { cur_ptr + 2 };
        let mut it_a = [0u64; 24];
        it_a[0] = r.next_u64(); // ctx
        it_a[2] = cur_ptr;
        it_a[3] = end_ptr;
        it_a[4] = r.next_u64(); // base_ptr
        let mut it_b = it_a;
        take_alloc_calls();
        o(it_a.as_mut_ptr() as *mut u8);
        let ca = take_alloc_calls();
        n(it_b.as_mut_ptr() as *mut u8);
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i}");
        diff_eq!(it_a, it_b, "it, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_outer_iter_load(this: *mut u8) {
    let t = this as usize;
    record_alloc((
        "outer_load",
        exe::rd::<usize>(t + 0x10),
        exe::rd::<usize>(t + 0x20),
        exe::rd::<u64>(t + 0x28),
        exe::rd::<u64>(t + 0x30),
    ));
}

pub unsafe fn diff_ecs_outer_iter_next<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_outer_iter_load as *const ())?;
    let mut r = Rng::new(0x140425fd0 ^ CALLEE_VA);
    let mut table = vec![0u8; 0x10048 + 4 * 32];
    let mut ctx = vec![0u8; 0x10048 + 4 * 32];
    const CASES: u32 = 150;
    for i in 0..CASES {
        for idx in 0..32usize {
            let tp = if r.below(2) == 0 { 0 } else { r.next_u64() };
            let tc = if r.below(2) == 0 { 0 } else { r.next_u32() };
            exe::wr::<u64>(table.as_mut_ptr() as usize + 0x48 + idx * 8, tp);
            exe::wr::<u32>(table.as_mut_ptr() as usize + 0x10048 + idx * 4, tc);
            let cp = if r.below(3) == 0 { 0 } else { r.next_u64() };
            let cc = r.next_u32();
            exe::wr::<u64>(ctx.as_mut_ptr() as usize + 0x48 + idx * 8, cp);
            exe::wr::<u32>(ctx.as_mut_ptr() as usize + 0x10048 + idx * 4, cc);
        }
        let count = (1 + r.below(8)) as usize;
        let mut indices = vec![0u16; count];
        for x in &mut indices {
            *x = r.below(32) as u16;
        }
        let cur_ptr = indices.as_ptr() as u64;
        let end_ptr = indices.as_ptr().add(count) as u64;
        let elem_count = r.below(4);
        let elem_idx = r.below(4);
        let mut it_a: [u64; 8] = [
            ctx.as_ptr() as u64,
            table.as_ptr() as u64,
            cur_ptr,
            end_ptr,
            r.next_u64(),
            elem_count,
            elem_idx,
            0,
        ];
        let mut it_b = it_a;
        take_alloc_calls();
        let ra = o(it_a.as_mut_ptr() as *mut u8);
        let ca = take_alloc_calls();
        let rb = n(it_b.as_mut_ptr() as *mut u8);
        let cb = take_alloc_calls();
        diff_eq!(
            ra - it_a.as_ptr() as usize,
            rb - it_b.as_ptr() as usize,
            "ret, case {i}"
        );
        diff_eq!(ca, cb, "calls, case {i}");
        diff_eq!(it_a, it_b, "it, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_span_dispatch_callee(
    rcx: usize,
    _rdx: *const u8,
    r8: usize,
    r9: usize,
    count: usize,
    _unused: usize,
    flag: usize,
) {
    record_alloc((
        "span_dispatch",
        rcx,
        r8,
        r9 as u64,
        (count as u64) | (((flag & 0xff) as u64) << 32),
    ));
}

pub unsafe fn diff_span_flag_dispatch<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *const u8, usize, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_span_dispatch_callee as *const ())?;
    let mut r = Rng::new(0x1402c5c30 ^ CALLEE_VA);
    const CASES: u32 = 120;
    for i in 0..CASES {
        let mut this_buf = [0u8; 0x28];
        exe::wr::<usize>(this_buf.as_mut_ptr() as usize, r.next_u64() as usize);
        let flag = if r.below(2) == 0 {
            0u8
        } else {
            (1 + r.below(255)) as u8
        };
        exe::wr::<u8>(this_buf.as_mut_ptr() as usize + 0x20, flag);
        let mut span_buf = [0u8; 0x10];
        exe::wr::<usize>(span_buf.as_mut_ptr() as usize, r.next_u64() as usize);
        let count = if r.below(2) == 0 { 0u32 } else { r.next_u32() };
        exe::wr::<u32>(span_buf.as_mut_ptr() as usize + 0x08, count);
        let r8 = r.next_u64() as usize;
        let r9 = r.next_u64() as usize;
        take_alloc_calls();
        o(this_buf.as_ptr(), span_buf.as_ptr(), r8, r9);
        let ca = take_alloc_calls();
        n(this_buf.as_ptr(), span_buf.as_ptr(), r8, r9);
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_cond_release_callee(p: *mut u8) {
    record_alloc(("cond_release", p as usize, 0, 0, 0));
}

pub unsafe fn diff_cond_release_dtor<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_cond_release_callee as *const ())?;
    let mut r = Rng::new(0x140379b20 ^ CALLEE_VA);
    const CASES: u32 = 120;
    for i in 0..CASES {
        let mut buf = [0u8; 0x18];
        let p = if r.below(3) == 0 {
            0usize
        } else {
            r.next_u64() as usize
        };
        let owned = if r.below(2) == 0 {
            0u8
        } else {
            (1 + r.below(255)) as u8
        };
        exe::wr::<usize>(buf.as_mut_ptr() as usize + 0x08, p);
        exe::wr::<u8>(buf.as_mut_ptr() as usize + 0x10, owned);
        take_alloc_calls();
        o(buf.as_mut_ptr());
        let ca = take_alloc_calls();
        n(buf.as_mut_ptr());
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_copy_nodes(
    _this: *mut u8,
    other_root: usize,
    _my_head: usize,
) -> usize {
    record_alloc(("copy_nodes", other_root, 0, 0, 0));
    other_root
}

pub unsafe fn diff_rb_tree_copy<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    use exe::wr;
    type F = unsafe extern "win64" fn(*mut u8, *const u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_copy_nodes as *const ())?;
    let mut r = Rng::new(0x14024ed60 ^ CALLEE_VA);
    const CASES: u32 = 120;
    for i in 0..CASES {
        // Build a 3-level fake node pool where node 0 is nil (`+0x19 = 1`) and nodes 1..=3 form a chain.
        let mut nodes = [[0u8; 0x20]; 4];
        let nil = nodes[0].as_mut_ptr() as usize;
        let n1 = nodes[1].as_mut_ptr() as usize;
        let n2 = nodes[2].as_mut_ptr() as usize;
        let n3 = nodes[3].as_mut_ptr() as usize;
        wr::<u8>(nil + 0x19, 1);
        // n2 is root; left child n1 (whose left is nil); right child n3 (whose right is nil)
        wr::<usize>(n2, if r.below(3) == 0 { nil } else { n1 });
        wr::<usize>(n2 + 0x10, if r.below(3) == 0 { nil } else { n3 });
        wr::<usize>(n1, nil);
        wr::<usize>(n1 + 0x10, nil);
        wr::<usize>(n3, nil);
        wr::<usize>(n3 + 0x10, nil);

        let root_ptr = if r.below(3) == 0 { nil } else { n2 };
        let mut other_head = [0usize; 4];
        other_head[1] = root_ptr;
        let other_tree: [usize; 2] = [other_head.as_ptr() as usize, r.next_u64() as usize];

        let mut head_a = [0usize; 4];
        let mut head_b = [0usize; 4];
        wr::<u8>(head_a.as_mut_ptr() as usize + 0x19, 1);
        wr::<u8>(head_b.as_mut_ptr() as usize + 0x19, 1);
        let mut tree_a: [usize; 2] = [head_a.as_mut_ptr() as usize, 0];
        let mut tree_b: [usize; 2] = [head_b.as_mut_ptr() as usize, 0];

        take_alloc_calls();
        o(tree_a.as_mut_ptr() as *mut u8, other_tree.as_ptr() as *const u8);
        let ca = take_alloc_calls();
        n(tree_b.as_mut_ptr() as *mut u8, other_tree.as_ptr() as *const u8);
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i}");
        diff_eq!(tree_a[1], tree_b[1], "size, case {i}");
        for k in 0..3 {
            let va = if head_a[k] == head_a.as_ptr() as usize {
                0
            } else {
                head_a[k]
            };
            let vb = if head_b[k] == head_b.as_ptr() as usize {
                0
            } else {
                head_b[k]
            };
            diff_eq!(va, vb, "head[{k}], case {i}");
        }
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_entity_callee_a(ctx: usize, h: *const u8, r8: usize) -> usize {
    let w0 = exe::rd::<u64>(h as usize);
    let w1 = exe::rd::<u32>(h as usize + 8) as u64;
    record_alloc(("entity_a", ctx, r8, w0, w1));
    (w0 as usize).wrapping_add(w1 as usize)
}

pub unsafe fn diff_entity_handle_thunk_a<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, *const u8, *const usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_entity_callee_a as *const ())?;
    let mut r = Rng::new(0x14021c410 ^ CALLEE_VA);
    const CASES: u32 = 100;
    for i in 0..CASES {
        let rcx = r.next_u64() as usize;
        let ctx = r.next_u64() as usize;
        let mut h = [0u8; 16];
        exe::wr::<u64>(h.as_mut_ptr() as usize, r.next_u64());
        exe::wr::<u32>(h.as_mut_ptr() as usize + 8, r.next_u32());
        take_alloc_calls();
        let ra = o(rcx, h.as_ptr(), &ctx);
        let ca = take_alloc_calls();
        let rb = n(rcx, h.as_ptr(), &ctx);
        let cb = take_alloc_calls();
        diff_eq!(ra, rb, "ret, case {i}");
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_entity_callee_b(
    rcx: usize,
    h: *const u8,
    r8: usize,
    r9: usize,
) -> usize {
    let w0 = exe::rd::<u64>(h as usize);
    let w1 = exe::rd::<u32>(h as usize + 8) as u64;
    record_alloc(("entity_b", rcx ^ r9, r8, w0, w1));
    (w0 as usize).wrapping_add(w1 as usize)
}

pub unsafe fn diff_entity_handle_thunk_b<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, *const u8, usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_entity_callee_b as *const ())?;
    let mut r = Rng::new(0x140210440 ^ CALLEE_VA);
    const CASES: u32 = 100;
    for i in 0..CASES {
        let rcx = r.next_u64() as usize;
        let r8 = r.next_u64() as usize;
        let r9 = r.next_u64() as usize;
        let mut h = [0u8; 16];
        exe::wr::<u64>(h.as_mut_ptr() as usize, r.next_u64());
        exe::wr::<u32>(h.as_mut_ptr() as usize + 8, r.next_u32());
        take_alloc_calls();
        let ra = o(rcx, h.as_ptr(), r8, r9);
        let ca = take_alloc_calls();
        let rb = n(rcx, h.as_ptr(), r8, r9);
        let cb = take_alloc_calls();
        diff_eq!(ra, rb, "ret, case {i}");
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_entity_handle_thunk_c<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize, *const u8, *const usize, usize) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_entity_callee_b as *const ())?;
    let mut r = Rng::new(0x14023df80 ^ CALLEE_VA);
    const CASES: u32 = 100;
    for i in 0..CASES {
        let rcx = r.next_u64() as usize;
        let ctx = r.next_u64() as usize;
        let r9 = r.next_u64() as usize;
        let mut h = [0u8; 16];
        exe::wr::<u64>(h.as_mut_ptr() as usize, r.next_u64());
        exe::wr::<u32>(h.as_mut_ptr() as usize + 8, r.next_u32());
        take_alloc_calls();
        let ra = o(rcx, h.as_ptr(), &ctx, r9);
        let ca = take_alloc_calls();
        let rb = n(rcx, h.as_ptr(), &ctx, r9);
        let cb = take_alloc_calls();
        diff_eq!(ra, rb, "ret, case {i}");
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_preset_copy_sub(dst: *mut u8, src: *const u8) -> usize {
    let w = exe::rd::<u64>(src as usize);
    exe::wr::<u64>(dst as usize, w);
    record_alloc(("preset_sub", 0, 0, w, 0));
    dst as usize
}

pub unsafe fn diff_preset_setting_copy_ctor<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *const u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_preset_copy_sub as *const ())?;
    let mut r = Rng::new(0x141dda2a0 ^ CALLEE_VA);
    const CASES: u32 = 100;
    for i in 0..CASES {
        let mut src = [0u64; 4];
        for x in &mut src {
            *x = r.next_u64();
        }
        let mut dst_a = [0u64; 4];
        let mut dst_b = [0u64; 4];
        take_alloc_calls();
        let ra = o(dst_a.as_mut_ptr() as *mut u8, src.as_ptr() as *const u8);
        let ca = take_alloc_calls();
        let rb = n(dst_b.as_mut_ptr() as *mut u8, src.as_ptr() as *const u8);
        let cb = take_alloc_calls();
        diff_eq!(
            ra - dst_a.as_ptr() as usize,
            rb - dst_b.as_ptr() as usize,
            "ret, case {i}"
        );
        diff_eq!(ca, cb, "calls, case {i}");
        diff_eq!(dst_a, dst_b, "dst, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_preset_setting_copy_to<const CALLEE_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m = Mock::new(CALLEE_VA, fake_preset_copy_sub as *const ())?;
    let mut r = Rng::new(0x141debd00 ^ CALLEE_VA);
    const CASES: u32 = 100;
    for i in 0..CASES {
        let mut src = [0u64; 4];
        for x in &mut src {
            *x = r.next_u64();
        }
        let mut dst_a = [0u64; 4];
        let mut dst_b = [0u64; 4];
        take_alloc_calls();
        let ra = o(src.as_ptr() as *const u8, dst_a.as_mut_ptr() as *mut u8);
        let ca = take_alloc_calls();
        let rb = n(src.as_ptr() as *const u8, dst_b.as_mut_ptr() as *mut u8);
        let cb = take_alloc_calls();
        diff_eq!(
            ra - dst_a.as_ptr() as usize,
            rb - dst_b.as_ptr() as usize,
            "ret, case {i}"
        );
        diff_eq!(ca, cb, "calls, case {i}");
        diff_eq!(dst_a, dst_b, "dst, case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_vec_clear(this: *mut u8) {
    record_alloc(("vec_clear", this as usize, 0, 0, 0));
}

pub unsafe fn diff_vec_dtor_with_clear<const CLEAR_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let chunk = FakeChunk::new()?;
    let _mocks = mock_free_paths()?;
    let _m_clear = Mock::new(CLEAR_VA, fake_vec_clear as *const ())?;
    let mut r = Rng::new(0x1401ffa20 ^ CLEAR_VA);
    const CASES: u64 = 300;
    for i in 0..CASES {
        let desc = chunk.descriptors();
        let (p, what) = chunk.setup(&mut r);
        let ptr = if r.below(8) == 0 { 0 } else { p };
        let off = (i % 8) as usize;
        let mut hdr0 = [0u8; 24];
        hdr0[off..off + 8].copy_from_slice(&ptr.to_le_bytes());
        hdr0[off + 8..off + 12].copy_from_slice(&r.next_u32().to_le_bytes());
        let cap_flags = match r.below(4) {
            0 => r.next_u32() | 0x4000_0000,
            1 => r.next_u32() & !0x4000_0000,
            2 => (r.next_u32() & !0x4000_0000) | 0x8000_0000,
            _ => r.next_u32(),
        };
        hdr0[off + 12..off + 16].copy_from_slice(&cap_flags.to_le_bytes());
        let (pre_desc, pre_p) = (desc.current(), *(p as *const u64));
        let mut ha = [0u64; 3];
        std::ptr::copy_nonoverlapping(hdr0.as_ptr(), ha.as_mut_ptr() as *mut u8, 24);
        take_free_calls();
        take_alloc_calls();
        let arg_a = (ha.as_mut_ptr() as *mut u8).add(off);
        o(arg_a);
        let (desc_a, p_a, free_a, clear_a) = (
            desc.current(),
            *(p as *const u64),
            take_free_calls(),
            take_alloc_calls(),
        );
        std::ptr::copy_nonoverlapping(
            pre_desc.as_ptr(),
            (chunk.base + 0x100) as *mut u8,
            pre_desc.len(),
        );
        *(p as *mut u64) = pre_p;
        let mut hb = [0u64; 3];
        std::ptr::copy_nonoverlapping(hdr0.as_ptr(), hb.as_mut_ptr() as *mut u8, 24);
        let arg_b = (hb.as_mut_ptr() as *mut u8).add(off);
        n(arg_b);
        let (desc_b, p_b, free_b, clear_b) = (
            desc.current(),
            *(p as *const u64),
            take_free_calls(),
            take_alloc_calls(),
        );
        diff_eq!(clear_a.len(), clear_b.len(), "clear len, case {i}");
        diff_eq!(
            clear_a[0].1 - arg_a as usize,
            clear_b[0].1 - arg_b as usize,
            "clear arg, case {i}"
        );
        diff_eq!(ha, hb, "header, case {i} {what} offset {off}");
        diff_eq!(free_a, free_b, "free calls, case {i} {what}");
        diff_eq!(p_a, p_b, "freed block link, case {i} {what}");
        if desc_a != desc_b {
            return Err(format!("descriptor memory differs, case {i} {what}"));
        }
        desc.restore();
    }
    Ok(CASES as u32)
}

pub unsafe fn diff_std_vec_calculate_growth_8(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(usize) -> u64;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x141a935a0);
    const CASES: u32 = 300;
    for i in 0..CASES {
        let cap = match i {
            0..=16 => i as usize,
            17 => 0x3fff_fffe,
            18 => 0x3fff_ffff,
            19 => 0x4000_0000,
            20 => usize::MAX,
            _ => match r.below(4) {
                0 => r.below(32) as usize,
                1 => r.next_u32() as usize,
                _ => r.next_u64() as usize,
            },
        };
        diff_eq!(o(cap), n(cap), "cap {cap:#x}, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_pod_vec_move_ctor(orig: usize, new: usize) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, *mut u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x140235980);
    const CASES: u32 = 200;
    for i in 0..CASES {
        let src_init = [r.next_u64(), r.next_u64()];
        let dst_init = [r.next_u64(), r.next_u64()];
        let (mut sa, mut sb) = (src_init, src_init);
        let (mut da, mut db) = (dst_init, dst_init);
        let ra = o(da.as_mut_ptr() as *mut u8, sa.as_mut_ptr() as *mut u8);
        let rb = n(db.as_mut_ptr() as *mut u8, sb.as_mut_ptr() as *mut u8);
        diff_eq!(ra - da.as_ptr() as usize, rb - db.as_ptr() as usize, "ret, case {i}");
        diff_eq!(da, db, "dst, case {i}");
        diff_eq!(sa, sb, "src, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_rb_tree_iter_inc(orig: usize, new: usize) -> Result<u32, String> {
    use exe::{rd, wr};
    type IncFn = unsafe extern "win64" fn(*mut usize) -> usize;
    let (o, n): (IncFn, IncFn) = (std::mem::transmute(orig), std::mem::transmute(new));
    let mut r = Rng::new(0x1403b31f0);
    const CASES: u32 = 150;
    for i in 0..CASES {
        let count = 1 + (r.below(12) as usize);
        let mut mem = [0u8; 0x240];
        let base = mem.as_mut_ptr() as usize;
        let head = base + 0x20;
        wr::<usize>(base, head);
        wr::<u64>(base + 0x08, 0);
        wr::<usize>(head, head);
        wr::<usize>(head + 0x08, head);
        wr::<usize>(head + 0x10, head);
        wr::<u8>(head + 0x18, 1);
        wr::<u8>(head + 0x19, 1);
        for idx in 0..count {
            let k = r.next_u32();
            let node = head + (idx + 1) * 0x20;
            wr::<usize>(node, head);
            wr::<usize>(node + 0x08, head);
            wr::<usize>(node + 0x10, head);
            wr::<u8>(node + 0x18, 0);
            wr::<u8>(node + 0x19, 0);
            wr::<u32>(node + 0x1c, k);
            let mut p = head;
            let mut cur = rd::<usize>(head + 0x08);
            let mut left_side = 1u32;
            while rd::<u8>(cur + 0x19) == 0 {
                p = cur;
                if k < rd::<u32>(cur + 0x1c) {
                    left_side = 1;
                    cur = rd::<usize>(cur);
                } else {
                    left_side = 0;
                    cur = rd::<usize>(cur + 0x10);
                }
            }
            let mut loc = [0u8; 16];
            wr::<usize>(loc.as_mut_ptr() as usize, p);
            wr::<u32>(loc.as_mut_ptr() as usize + 8, left_side);
            rb_tree_insert_node(base as *mut u8, loc.as_ptr(), node as *mut u8);
        }
        let mut it_a = rd::<usize>(head);
        let mut it_b = it_a;
        for step in 0..count {
            let ra = o(&mut it_a);
            let rb = n(&mut it_b);
            diff_eq!(
                ra - &it_a as *const usize as usize,
                rb - &it_b as *const usize as usize,
                "ret case {i} step {step}"
            );
            diff_eq!(it_a, it_b, "node case {i} step {step}");
        }
        diff_eq!(it_a, head, "end at head case {i}");
    }
    Ok(CASES)
}

unsafe extern "win64" fn fake_type_info_dtor(p: *mut u8) {
    record_alloc(("type_info_dtor", p as usize, 0, 0, 0));
}

unsafe extern "win64" fn fake_free_array_ptr(p: *mut u8) {
    record_alloc(("free_array", p as usize, 0, 0, 0));
}

pub unsafe fn diff_type_info_delete_dtor<const DTOR_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_dtor = Mock::new(DTOR_VA, fake_type_info_dtor as *const ())?;
    let _m_free = Mock::new(REGION_FREE_PTR_VA, fake_stream_free as *const ())?;
    let mut r = Rng::new(0x1418e5e20 ^ DTOR_VA);
    const CASES: u32 = 120;
    for i in 0..CASES {
        let this = r.next_u64() as usize as *const u8;
        let p = if r.below(4) == 0 {
            std::ptr::null_mut()
        } else {
            r.next_u64() as usize as *mut u8
        };
        take_alloc_calls();
        o(this, p);
        let ca = take_alloc_calls();
        n(this, p);
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_type_info_delete_array_dtor<const DTOR_VA: u64>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_dtor = Mock::new(DTOR_VA, fake_type_info_dtor as *const ())?;
    let _m_free = Mock::new(FREE_ARRAY_VA, fake_free_array_ptr as *const ())?;
    let mut r = Rng::new(0x1418e5dc0 ^ DTOR_VA);
    const CASES: u32 = 120;
    for i in 0..CASES {
        let this = r.next_u64() as usize as *const u8;
        if r.below(5) == 0 {
            take_alloc_calls();
            o(this, std::ptr::null_mut());
            let ca = take_alloc_calls();
            n(this, std::ptr::null_mut());
            let cb = take_alloc_calls();
            diff_eq!(ca, cb, "null calls, case {i}");
            continue;
        }
        let mut buf = [0u64; 4];
        let count = r.below(6) as u32;
        let elem_size = 8 * (1 + r.below(16) as usize);
        let base = unsafe { (buf.as_mut_ptr() as *mut u8).add(0x10) };
        exe::wr::<usize>(base as usize - 0x10, elem_size);
        exe::wr::<u32>(base as usize - 0x08, count);
        exe::wr::<u32>(base as usize - 0x04, 16);
        take_alloc_calls();
        o(this, base);
        let ca = take_alloc_calls();
        n(this, base);
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i} count {count} elem_size {elem_size}");
    }
    Ok(CASES)
}

pub unsafe fn diff_subobj_dtor_set_vtable<
    const SUB_OFF: usize,
    const DTOR_VA: u64,
    const VTBL_VA: u64,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_dtor = Mock::new(DTOR_VA, fake_type_info_dtor as *const ())?;
    let mut r = Rng::new(0x1418531d0 ^ DTOR_VA ^ VTBL_VA ^ (SUB_OFF as u64));
    const CASES: u32 = 100;
    for i in 0..CASES {
        let mut ba = [0u64; 16];
        for w in &mut ba {
            *w = r.next_u64();
        }
        let mut bb = ba;
        let pa = ba.as_mut_ptr() as *mut u8;
        let pb = bb.as_mut_ptr() as *mut u8;
        take_alloc_calls();
        o(pa);
        let ca = take_alloc_calls();
        n(pb);
        let cb = take_alloc_calls();
        diff_eq!(ca.len(), cb.len(), "calls len, case {i}");
        diff_eq!(ca[0].1 - pa as usize, cb[0].1 - pb as usize, "dtor arg, case {i}");
        diff_eq!(ba, bb, "buf, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_subobj_scalar_deleting_dtor<
    const SUB_OFF: usize,
    const DTOR_VA: u64,
    const VTBL_VA: u64,
    const SIZE: usize,
>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, u8) -> usize;
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_dtor = Mock::new(DTOR_VA, fake_type_info_dtor as *const ())?;
    let _m_free = Mock::new(J_FREE_SIZED_VA, fake_j_free_sized as *const ())?;
    let mut r = Rng::new(0x1401f4bc0 ^ DTOR_VA ^ VTBL_VA ^ (SIZE as u64));
    const CASES: u32 = 100;
    for i in 0..CASES {
        let flags = r.next_u32() as u8;
        let mut ba = [0u64; 16];
        for w in &mut ba {
            *w = r.next_u64();
        }
        let mut bb = ba;
        let pa = ba.as_mut_ptr() as *mut u8;
        let pb = bb.as_mut_ptr() as *mut u8;
        take_alloc_calls();
        let ra = o(pa, flags);
        let mut ca = take_alloc_calls();
        for c in &mut ca {
            c.1 -= pa as usize;
        }
        let rb = n(pb, flags);
        let mut cb = take_alloc_calls();
        for c in &mut cb {
            c.1 -= pb as usize;
        }
        diff_eq!(ra - pa as usize, rb - pb as usize, "ret, case {i}");
        diff_eq!(ca, cb, "calls, case {i}");
        diff_eq!(ba, bb, "buf, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_array_vfunc0_dtor<const STRIDE: usize>(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*mut u8, usize);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let vtbl: [usize; 1] = [fake_vtbl_dtor_0 as *const () as usize];
    let mut r = Rng::new(0x141876670 ^ (STRIDE as u64));
    let mut buf = vec![0u8; STRIDE * 8 + 16];
    let base = buf.as_mut_ptr() as usize;
    for k in 0..8 {
        exe::wr::<usize>(base + k * STRIDE, vtbl.as_ptr() as usize);
    }
    const CASES: u32 = 80;
    for i in 0..CASES {
        let count = r.below(7) as usize;
        take_alloc_calls();
        o(base as *mut u8, count);
        let ca = take_alloc_calls();
        n(base as *mut u8, count);
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i} count {count}");
    }
    Ok(CASES)
}

pub unsafe fn diff_type_info_vfunc0_delete_dtor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_free = Mock::new(REGION_FREE_PTR_VA, fake_stream_free as *const ())?;
    let vtbl: [usize; 1] = [fake_vtbl_dtor_0 as *const () as usize];
    let obj: [usize; 2] = [vtbl.as_ptr() as usize, 0xdead_beef];
    let mut r = Rng::new(0x1413594b0);
    const CASES: u32 = 100;
    for i in 0..CASES {
        let this = r.next_u64() as usize as *const u8;
        let p = if r.below(4) == 0 {
            std::ptr::null_mut()
        } else {
            obj.as_ptr() as *mut u8
        };
        take_alloc_calls();
        o(this, p);
        let ca = take_alloc_calls();
        n(this, p);
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i}");
    }
    Ok(CASES)
}

pub unsafe fn diff_type_info_vfunc0_delete_array_dtor(
    orig: usize,
    new: usize,
) -> Result<u32, String> {
    type F = unsafe extern "win64" fn(*const u8, *mut u8);
    let (o, n): (F, F) = (std::mem::transmute(orig), std::mem::transmute(new));
    let _m_free = Mock::new(FREE_ARRAY_VA, fake_free_array_ptr as *const ())?;
    let vtbl: [usize; 1] = [fake_vtbl_dtor_0 as *const () as usize];
    let mut r = Rng::new(0x141849ac0);
    let mut raw = vec![0u8; 0x20 + 6 * 64];
    const CASES: u32 = 100;
    for i in 0..CASES {
        let this = r.next_u64() as usize as *const u8;
        if r.below(5) == 0 {
            take_alloc_calls();
            o(this, std::ptr::null_mut());
            let ca = take_alloc_calls();
            n(this, std::ptr::null_mut());
            let cb = take_alloc_calls();
            diff_eq!(ca, cb, "null calls, case {i}");
            continue;
        }
        let count = r.below(6) as u32;
        let elem_size = 8 * (1 + r.below(8) as usize);
        let base = raw.as_mut_ptr() as usize + 0x10;
        exe::wr::<usize>(base - 0x10, elem_size);
        exe::wr::<u32>(base - 0x08, count);
        exe::wr::<u32>(base - 0x04, 16);
        for k in 0..(count as usize) {
            exe::wr::<usize>(base + k * elem_size, vtbl.as_ptr() as usize);
        }
        take_alloc_calls();
        o(this, base as *mut u8);
        let ca = take_alloc_calls();
        n(this, base as *mut u8);
        let cb = take_alloc_calls();
        diff_eq!(ca, cb, "calls, case {i} count {count} elem_size {elem_size}");
    }
    Ok(CASES)
}






