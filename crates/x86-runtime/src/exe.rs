//! Access to the mapped `AlanWake2.exe` image from decompiled code.
//!
//! Decompiled functions that read the original's constant data (lookup
//! tables in `.rdata`, globals in `.data`) address it by its preferred-base VA
//! through [`ptr`]; the loader calls [`set_base`] with the actual image base
//! before any replacement runs (it is relocated if 0x140000000 is taken).

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::Mutex;

/// Default ImageBase from the `AlanWake2.exe` PE header.
pub const PREFERRED_BASE: u64 = 0x1_4000_0000;

static PREFERRED_BASE_DYN: AtomicU64 = AtomicU64::new(PREFERRED_BASE);
static IMAGE_SIZE_DYN: AtomicU64 = AtomicU64::new(0x0500_0000);
static BASE: AtomicUsize = AtomicUsize::new(PREFERRED_BASE as usize);

/// Bumped whenever anything that affects [`lookup_native_va`] changes (registrations,
/// image base, host text range). Never 0, so 0 can mean "not cached".
static VA_GEN: AtomicU32 = AtomicU32::new(1);

/// Bumped whenever non-`VA_DISPATCH` dispatch state changes (`invalidate_dispatch`,
/// `set_base`, `set_preferred_base`, `set_aw2_registry_active`). Never 0.
static VA_STRUCTURAL_GEN: AtomicU32 = AtomicU32::new(1);

const VA_DELTA_CAP: usize = 512;

/// Summary of `VA_DISPATCH` / dispatch-routing changes since a previous `va_seq` snapshot.
pub(crate) enum VaDelta {
    /// A structural routing change occurred, the delta ring overflowed, or no prior snapshot
    /// existed: callers must run a full revalidation across all modules.
    Full,
    /// Exact deduplicated set of `VA_DISPATCH` keys added, updated, or removed since `prev_seq`
    /// with zero structural routing changes in between.
    ChangedVas(Vec<u64>),
}

struct VaDeltaLog {
    seq: u64,
    full_seq: u64,
    oldest_dropped_seq: u64,
    ring: VecDeque<(u64, u64)>,
}

static VA_DELTA_LOG: Mutex<VaDeltaLog> = Mutex::new(VaDeltaLog {
    seq: 1,
    full_seq: 1,
    oldest_dropped_seq: 0,
    ring: VecDeque::new(),
});

#[inline(always)]
pub fn va_gen() -> u32 {
    VA_GEN.load(Ordering::Acquire)
}

/// Race-free snapshot of `(va_seq, va_gen, va_structural_gen)` for incremental AOT revalidation.
pub(crate) fn va_snapshot_gens() -> (u64, u32, u32) {
    let log = VA_DELTA_LOG.lock().unwrap_or_else(|e| e.into_inner());
    (
        log.seq,
        VA_GEN.load(Ordering::Acquire),
        VA_STRUCTURAL_GEN.load(Ordering::Acquire),
    )
}

/// Return `(cur_seq, cur_va_gen, cur_structural_gen, delta)` summarizing every dispatch change
/// after `prev_seq`.
pub(crate) fn va_delta_since(prev_seq: u64) -> (u64, u32, u32, VaDelta) {
    let log = VA_DELTA_LOG.lock().unwrap_or_else(|e| e.into_inner());
    let cur_seq = log.seq;
    let cur_vg = VA_GEN.load(Ordering::Acquire);
    let cur_sg = VA_STRUCTURAL_GEN.load(Ordering::Acquire);
    if prev_seq == 0 || prev_seq < log.full_seq || prev_seq <= log.oldest_dropped_seq {
        return (cur_seq, cur_vg, cur_sg, VaDelta::Full);
    }
    let mut vas = Vec::new();
    for &(s, va) in &log.ring {
        if s > prev_seq {
            vas.push(va);
        }
    }
    vas.sort_unstable();
    vas.dedup();
    (cur_seq, cur_vg, cur_sg, VaDelta::ChangedVas(vas))
}

#[inline]
fn bump_va_gen_structural() {
    #[cfg(test)]
    let _w = (VA_GEN_HOLDS.try_with(|c| c.get()).unwrap_or(0) == 0)
        .then(|| TEST_VA_GEN_LOCK.write().unwrap_or_else(|e| e.into_inner()));
    let mut log = VA_DELTA_LOG.lock().unwrap_or_else(|e| e.into_inner());
    log.seq = log.seq.wrapping_add(1).max(1);
    log.full_seq = log.seq;
    log.oldest_dropped_seq = 0;
    log.ring.clear();
    let mut sg = VA_STRUCTURAL_GEN.load(Ordering::Relaxed).wrapping_add(1);
    if sg == 0 {
        sg = 1;
    }
    VA_STRUCTURAL_GEN.store(sg, Ordering::Release);
    let mut g = VA_GEN.load(Ordering::Relaxed).wrapping_add(1);
    if g == 0 {
        g = 1;
    }
    VA_GEN.store(g, Ordering::Release);
}

#[inline]
fn bump_va_gen_vas(vas: &[u64]) {
    #[cfg(test)]
    let _w = (VA_GEN_HOLDS.try_with(|c| c.get()).unwrap_or(0) == 0)
        .then(|| TEST_VA_GEN_LOCK.write().unwrap_or_else(|e| e.into_inner()));
    let mut log = VA_DELTA_LOG.lock().unwrap_or_else(|e| e.into_inner());
    log.seq = log.seq.wrapping_add(1).max(1);
    let seq = log.seq;
    if vas.len() > VA_DELTA_CAP {
        log.full_seq = seq;
        log.oldest_dropped_seq = 0;
        log.ring.clear();
    } else {
        for &va in vas {
            if log.ring.len() == VA_DELTA_CAP {
                if let Some((old_seq, _)) = log.ring.pop_front() {
                    log.oldest_dropped_seq = log.oldest_dropped_seq.max(old_seq);
                }
            }
            log.ring.push_back((seq, va));
        }
    }
    let mut g = VA_GEN.load(Ordering::Relaxed).wrapping_add(1);
    if g == 0 {
        g = 1;
    }
    VA_GEN.store(g, Ordering::Release);
}

/// Invalidate every per-thread dispatch memo (bump [`va_gen`]) after a change that affects
/// `emu::resolve_dispatch` outside the VA table (a DLL image's IAT directory registered).
pub(crate) fn invalidate_dispatch() {
    bump_va_gen_structural();
}

/// Tests: generation bumps (`register_native_va`, ...) from other test threads wait while a
/// test holds [`hold_va_gen`]: its thread-local dispatch slots are seeded for the current
/// generation, and a bump in between sent the seeded fake VAs to the emulator (SIGSEGV).
#[cfg(test)]
static TEST_VA_GEN_LOCK: std::sync::RwLock<()> = std::sync::RwLock::new(());

#[cfg(test)]
std::thread_local! {
    static VA_GEN_HOLDS: core::cell::Cell<u32> = const { core::cell::Cell::new(0) };
}

/// See [`TEST_VA_GEN_LOCK`]. Take once per test (not nested), before seeding.
#[cfg(test)]
pub(crate) struct VaGenHold(#[allow(dead_code)] std::sync::RwLockReadGuard<'static, ()>);

#[cfg(test)]
impl Drop for VaGenHold {
    fn drop(&mut self) {
        VA_GEN_HOLDS.with(|c| c.set(c.get() - 1));
    }
}

#[cfg(test)]
pub(crate) fn hold_va_gen() -> VaGenHold {
    let g = TEST_VA_GEN_LOCK.read().unwrap_or_else(|e| e.into_inner());
    VA_GEN_HOLDS.with(|c| c.set(c.get() + 1));
    VaGenHold(g)
}

/// Record the preferred image base and image size of the primary PE module.
pub fn set_preferred_base(preferred_base: u64, image_size: u64) {
    PREFERRED_BASE_DYN.store(preferred_base, Ordering::Relaxed);
    IMAGE_SIZE_DYN.store(image_size.max(0x1000), Ordering::Relaxed);
    bump_va_gen_structural();
}

#[inline(always)]
pub fn preferred_base() -> u64 {
    PREFERRED_BASE_DYN.load(Ordering::Relaxed)
}

#[inline(always)]
pub fn image_size() -> u64 {
    IMAGE_SIZE_DYN.load(Ordering::Relaxed)
}

/// Set once the `AlanWake2.exe` replacement registry has been applied to the main exe.
/// Every AW2-specific hardcoded VA/range in shared code (special-ABI CRT helpers,
/// `__chkstk`, `FLOAT_RETURN`, the `0x1_4000_0000..` replacement window) is consulted
/// only while this is set, so other games are never affected by AW2 address collisions.
static AW2_REGISTRY_ACTIVE: AtomicBool = AtomicBool::new(false);

/// Mark the AW2 replacement registry as applied (called by the loader's `apply_registry`).
pub fn set_aw2_registry_active(active: bool) {
    AW2_REGISTRY_ACTIVE.store(active, Ordering::Relaxed);
    bump_va_gen_structural();
}

/// Whether the AW2 replacement registry is applied (main exe is `AlanWake2.exe`).
#[inline(always)]
pub fn aw2_registry_active() -> bool {
    AW2_REGISTRY_ACTIVE.load(Ordering::Relaxed)
}

/// Record where the loader mapped the exe.
pub fn set_base(base: usize) {
    BASE.store(base, Ordering::Relaxed);
    bump_va_gen_structural();
    crate::eh_rt::chain::ensure_hooks();
}

/// Actual image base.
pub fn base() -> usize {
    BASE.load(Ordering::Relaxed)
}

/// Address of the exe object at preferred-base `va`.
#[inline(always)]
pub fn ptr<T>(va: u64) -> *const T {
    let pref = PREFERRED_BASE_DYN.load(Ordering::Relaxed);
    (BASE.load(Ordering::Relaxed) as u64).wrapping_add(va.wrapping_sub(pref)) as *const T
}

const VA_TABLE_BITS: u32 = 21;
const VA_TABLE_CAP: usize = 1 << VA_TABLE_BITS;
const VA_TABLE_MASK: usize = VA_TABLE_CAP - 1;

/// L1-resident 32 KiB (262,144-bit) 2-probe Bloom filter mirroring every VA ever inserted into
/// [`VA_DISPATCH`] with a non-zero pointer. Rejects >99% of negative `lookup_native_va` probes
/// in ~1.5 ns without touching the 32 MiB open-addressed table. Bits are set with `Release`
/// before inserting into `VA_DISPATCH` and never cleared on unregister (false positives always
/// fall through to the exact table).
const VA_BLOOM_WORDS: usize = 4096;
const VA_BLOOM_MASK: usize = (VA_BLOOM_WORDS * 64) - 1;
static VA_BLOOM: [AtomicU64; VA_BLOOM_WORDS] = [const { AtomicU64::new(0) }; VA_BLOOM_WORDS];

#[inline(always)]
fn va_bloom_bits(va: u64) -> (usize, u64, usize, u64) {
    let h1 = ((va.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> 46) as usize) & VA_BLOOM_MASK;
    let h2 = ((va.wrapping_mul(0x517C_C1B7_2722_0A95) >> 46) as usize) & VA_BLOOM_MASK;
    (h1 >> 6, 1u64 << (h1 & 63), h2 >> 6, 1u64 << (h2 & 63))
}

#[inline(always)]
fn va_bloom_set(va: u64) {
    let (w1, b1, w2, b2) = va_bloom_bits(va);
    VA_BLOOM[w1].fetch_or(b1, Ordering::Release);
    VA_BLOOM[w2].fetch_or(b2, Ordering::Release);
}

/// Fast lock-free Bloom query: `false` guarantees `va` is NOT registered in [`VA_DISPATCH`].
#[inline(always)]
pub(crate) fn va_bloom_may_contain(va: u64) -> bool {
    if va == 0 {
        return false;
    }
    let (w1, b1, w2, b2) = va_bloom_bits(va);
    (VA_BLOOM[w1].load(Ordering::Acquire) & b1) != 0
        && (VA_BLOOM[w2].load(Ordering::Acquire) & b2) != 0
}

/// Lock-free open-addressed dispatch table mapping preferred-base exe VAs (`0x1_4000_0000..`)
/// or actual mapped addresses to their compiled native Rust function pointers.
static VA_DISPATCH: std::sync::OnceLock<Box<[( std::sync::atomic::AtomicU64, AtomicUsize ) ]>> =
    std::sync::OnceLock::new();
static VA_TABLE_LEN: AtomicUsize = AtomicUsize::new(0);

#[inline]
fn va_table() -> &'static [(std::sync::atomic::AtomicU64, AtomicUsize)] {
    VA_DISPATCH.get_or_init(|| {
        let mut v = Vec::with_capacity(VA_TABLE_CAP);
        for _ in 0..VA_TABLE_CAP {
            v.push((
                std::sync::atomic::AtomicU64::new(0),
                AtomicUsize::new(0),
            ));
        }
        v.into_boxed_slice()
    })
}

#[inline(always)]
fn va_hash(va: u64) -> usize {
    (va.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - VA_TABLE_BITS)) as usize
}

#[inline]
fn va_table_insert(t: &[(std::sync::atomic::AtomicU64, AtomicUsize)], va: u64, ptr: usize) -> bool {
    if va == 0 {
        return false;
    }
    if ptr != 0 {
        va_bloom_set(va);
    }
    let mut idx = va_hash(va);
    for _ in 0..VA_TABLE_CAP {
        let (k, v) = &t[idx];
        let cur = k.load(Ordering::Relaxed);
        if cur == va {
            if v.load(Ordering::Acquire) == ptr {
                return false;
            }
            let old = v.swap(ptr, Ordering::Release);
            return old != ptr;
        }
        if cur == 0 {
            if ptr == 0 {
                return false;
            }
            match k.compare_exchange(0, va, Ordering::AcqRel, Ordering::Relaxed) {
                Ok(_) => {
                    v.store(ptr, Ordering::Release);
                    let len = VA_TABLE_LEN.fetch_add(1, Ordering::Relaxed) + 1;
                    if len == (VA_TABLE_CAP * 3) / 4 {
                        log::warn!("VA_DISPATCH table reached 75% capacity ({len}/{VA_TABLE_CAP})");
                    }
                    return true;
                }
                Err(actual) if actual == va => {
                    let old = v.swap(ptr, Ordering::Release);
                    return old != ptr;
                }
                Err(_) => {}
            }
        }
        idx = (idx + 1) & VA_TABLE_MASK;
    }
    panic!("VA_DISPATCH table full ({VA_TABLE_CAP} slots) while inserting va={va:#x}");
}

#[inline(always)]
fn va_table_get(t: &[(std::sync::atomic::AtomicU64, AtomicUsize)], va: u64) -> Option<usize> {
    if !va_bloom_may_contain(va) {
        return None;
    }
    let mut idx = va_hash(va);
    for _ in 0..VA_TABLE_CAP {
        let (k, v) = unsafe { t.get_unchecked(idx) };
        let cur = k.load(Ordering::Relaxed);
        if cur == va {
            let p = v.load(Ordering::Relaxed);
            return if p != 0 { Some(p) } else { None };
        }
        if cur == 0 {
            return None;
        }
        idx = (idx + 1) & VA_TABLE_MASK;
    }
    None
}

static HOST_TEXT_START: AtomicUsize = AtomicUsize::new(0);
static HOST_TEXT_END: AtomicUsize = AtomicUsize::new(0);

/// Record the `[start, end)` address range of the host Mach-O `__TEXT,__text` section
/// strictly for **negative** exclusion checks ([`in_host_text`]) — e.g. refusing to persist
/// host pointers as runtime guest callbacks or execute unregistered host text.
/// Host `__TEXT` is NEVER implicitly authorized for native dispatch.
pub fn set_host_text_range(start: usize, end: usize) {
    HOST_TEXT_START.store(start, Ordering::Release);
    HOST_TEXT_END.store(end, Ordering::Release);
}

/// True if `addr` lies inside the recorded host Mach-O `__TEXT,__text` range.
/// Used strictly for negative exclusion guards, never as a dispatch allowlist.
#[inline]
pub fn in_host_text(addr: usize) -> bool {
    let s = HOST_TEXT_START.load(Ordering::Acquire);
    let e = HOST_TEXT_END.load(Ordering::Acquire);
    s < e && addr >= s && addr < e
}

/// True if `addr` lies inside host `__TEXT,__text` and is NOT registered in `VA_DISPATCH`.
/// Used by `win32-rt` escape audits to detect unregistered host function pointers before
/// they reach guest code or `call_va8`.
#[inline]
pub fn is_unregistered_host_text(addr: usize) -> bool {
    in_host_text(addr) && lookup_native_va(addr as u64).is_none()
}

/// Register a native host function pointer `ptr` for preferred-base `va`.
pub fn register_native_va(va: u64, ptr: usize) {
    if va_table_insert(va_table(), va, ptr) {
        bump_va_gen_vas(&[va]);
    }
}

/// Remove a registered entry for `va` from the native VA dispatch table.
pub fn unregister_native_va(va: u64) {
    if va_table_insert(va_table(), va, 0) {
        bump_va_gen_vas(&[va]);
    }
}

/// Bulk-register `(va, ptr)` pairs in the native VA dispatch table.
pub fn register_native_vas(entries: impl IntoIterator<Item = (u64, usize)>) {
    let t = va_table();
    let mut changed_vas: Vec<u64> = Vec::new();
    let mut overflow = false;
    for (va, ptr) in entries {
        if va_table_insert(t, va, ptr) {
            if !overflow {
                if changed_vas.len() < VA_DELTA_CAP {
                    changed_vas.push(va);
                } else {
                    overflow = true;
                    changed_vas.clear();
                }
            }
        }
    }
    if overflow {
        bump_va_gen_structural();
    } else if !changed_vas.is_empty() {
        bump_va_gen_vas(&changed_vas);
    }
}

/// Look up the native host function pointer registered in `VA_DISPATCH` for `va_or_addr`
/// (excluding raw host `__TEXT` addresses).
#[inline]
pub fn lookup_va_only(va_or_addr: u64) -> Option<usize> {
    let t = va_table();
    if let Some(p) = va_table_get(t, va_or_addr) {
        return Some(p);
    }
    let b = BASE.load(Ordering::Relaxed) as u64;
    if aw2_registry_active() {
        // AW2: legacy semantics (constant preferred base, no upper bound).
        if b != PREFERRED_BASE && va_or_addr >= b {
            let pref = PREFERRED_BASE.wrapping_add(va_or_addr - b);
            if let Some(p) = va_table_get(t, pref) {
                return Some(p);
            }
        }
        return None;
    }
    let pref_base = preferred_base();
    if b != pref_base && va_or_addr >= b && va_or_addr - b < image_size() {
        let pref = pref_base.wrapping_add(va_or_addr - b);
        if let Some(p) = va_table_get(t, pref) {
            return Some(p);
        }
    }
    None
}

/// `[start, end)` preferred-base VA window of the primary exe image. AW2 keeps its legacy
/// constant window (`0x1_4000_0000..0x1_4500_0000`); every other exe uses its real
/// `ImageBase` / `SizeOfImage`.
#[inline(always)]
pub fn exe_va_window() -> (u64, u64) {
    if aw2_registry_active() {
        (PREFERRED_BASE, 0x1_4500_0000)
    } else {
        let p = preferred_base();
        (p, p.wrapping_add(image_size()))
    }
}

/// Look up the native host function pointer explicitly registered in `VA_DISPATCH` for `va`
/// (preferred-base VA or actual mapped address).
///
/// Unregistered addresses inside host `__TEXT` are NEVER implicitly authorized.
#[inline]
pub fn lookup_native_va(va_or_addr: u64) -> Option<usize> {
    lookup_va_only(va_or_addr)
}

/// Conversion into a 64-bit Win64 register argument for [`call_va!`] / [`call_iat!`].
pub trait IntoWin64Arg {
    fn into_win64_arg(self) -> u64;
}
impl IntoWin64Arg for u64 {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self
    }
}
impl IntoWin64Arg for usize {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as u64
    }
}
impl IntoWin64Arg for u32 {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as u64
    }
}
impl IntoWin64Arg for u16 {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as u64
    }
}
impl IntoWin64Arg for u8 {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as u64
    }
}
impl IntoWin64Arg for i64 {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as u64
    }
}
impl IntoWin64Arg for isize {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as u64
    }
}
impl IntoWin64Arg for i32 {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as i64 as u64
    }
}
impl<T> IntoWin64Arg for *const T {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as usize as u64
    }
}
impl<T> IntoWin64Arg for *mut T {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        self as usize as u64
    }
}
impl<T> IntoWin64Arg for &T {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        (self as *const T) as usize as u64
    }
}
impl<T> IntoWin64Arg for &mut T {
    #[inline(always)]
    fn into_win64_arg(self) -> u64 {
        (self as *mut T) as usize as u64
    }
}

pub trait FromWin64Ret {
    fn from_win64_ret(v: u64) -> Self;
}
impl FromWin64Ret for () {
    #[inline(always)]
    fn from_win64_ret(_: u64) {}
}
impl FromWin64Ret for u64 {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v
    }
}
impl FromWin64Ret for usize {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v as usize
    }
}
impl FromWin64Ret for u32 {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v as u32
    }
}
impl FromWin64Ret for i32 {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v as i32
    }
}
impl FromWin64Ret for i64 {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v as i64
    }
}
impl FromWin64Ret for u16 {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v as u16
    }
}
impl FromWin64Ret for u8 {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v as u8
    }
}
impl FromWin64Ret for bool {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        (v as u8) != 0
    }
}
impl<T> FromWin64Ret for *const T {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v as usize as *const T
    }
}
impl<T> FromWin64Ret for *mut T {
    #[inline(always)]
    fn from_win64_ret(v: u64) -> Self {
        v as usize as *mut T
    }
}

pub use crate::{call_iat, call_iat_native, call_native, call_va};

/// The exe function at `va` as a typed function pointer `F` (an
/// `unsafe extern "win64" fn(..)` type). Used to call callees that are not
/// decompiled (or are decompiled: the call then lands in the hook, exactly
/// like the original's `call rel32`).
#[inline(always)]
pub unsafe fn func<F: Copy>(va: u64) -> F {
    debug_assert_eq!(std::mem::size_of::<F>(), std::mem::size_of::<usize>());
    #[cfg(target_arch = "x86_64")]
    let p = ptr::<u8>(va);
    #[cfg(not(target_arch = "x86_64"))]
    let p = crate::emu::resolve_host_callable(va).unwrap_or_else(|| ptr::<u8>(va) as usize) as *const u8;
    std::mem::transmute_copy(&p)
}

/// Map a preferred-base exe VA (within [`exe_va_window`]) to its actual mapped address.
#[inline(always)]
pub fn to_mapped_addr(va_or_addr: u64) -> usize {
    let b = base() as u64;
    let (lo, hi) = exe_va_window();
    if b != lo && (lo..hi).contains(&va_or_addr) {
        (b.wrapping_add(va_or_addr - lo)) as usize
    } else {
        va_or_addr as usize
    }
}

static TRAMPOLINE_LOOKUP: AtomicUsize = AtomicUsize::new(0);

/// Register the loader's `trampoline_for(va: u64) -> Option<usize>` lookup.
pub fn set_trampoline_lookup(f: fn(u64) -> Option<usize>) {
    TRAMPOLINE_LOOKUP.store(f as usize, Ordering::Release);
}

/// Look up the trampoline address for `va` if hooked, or its mapped address.
#[inline]
pub fn lookup_trampoline_or_mapped(va: u64) -> usize {
    let f_ptr = TRAMPOLINE_LOOKUP.load(Ordering::Acquire);
    if f_ptr != 0 {
        let f: fn(u64) -> Option<usize> = unsafe { core::mem::transmute(f_ptr) };
        if let Some(t) = f(va) {
            return t;
        }
    }
    to_mapped_addr(va)
}

thread_local! {
    static CURRENT_MAPPED_VA: core::cell::Cell<u64> = const { core::cell::Cell::new(0) };
}

/// Return the active compile-time mapped VA if set by a shim.
#[inline]
pub fn current_mapped_va() -> Option<u64> {
    let v = CURRENT_MAPPED_VA.with(|c| c.get());
    if v != 0 { Some(v) } else { None }
}

/// Execute a compile-time mapped CFG function entry from `CFG_META` directly
/// (with `try_call` exception catching when `EH == 2`), avoiding any software
/// bytecode interpreter loop.
#[inline]
pub unsafe fn call_mapped_cfg<const EH: u8>(
    va: u64,
    _frame: u32,
    _c0: u64,
    _c1: u64,
    _c2: u64,
    _c3: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) -> u64 {
    let target = lookup_trampoline_or_mapped(va);
    if EH == 2 {
        let args = [rcx, rdx, r8, r9, arg5, arg6, arg7, arg8];
        match try_call(target, &args, &[]) {
            Ok(ret) => ret.rax,
            Err(e) => e.rethrow(),
        }
    } else {
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            core::mem::transmute(target);
        f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
}

/// Call a function at `va_or_addr` (preferred-base VA or actual mapped address),
/// routing directly to the compiled native function pointer.
#[inline(always)]
pub unsafe fn call_va8(
    va_or_addr: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
) -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let addr = lookup_native_va(va_or_addr).unwrap_or_else(|| to_mapped_addr(va_or_addr));
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            core::mem::transmute(addr);
        f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        crate::emu::call_va8(va_or_addr, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
}

/// Call a function at `va_or_addr` with up to 10 arguments.
#[inline(always)]
pub unsafe fn call_va10(
    va_or_addr: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
    arg9: u64,
    arg10: u64,
) -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let addr = lookup_native_va(va_or_addr).unwrap_or_else(|| to_mapped_addr(va_or_addr));
        let f: unsafe extern "win64" fn(u64, u64, u64, u64, u64, u64, u64, u64, u64, u64) -> u64 =
            core::mem::transmute(addr);
        f(rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, arg9, arg10)
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let buf: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, arg9, arg10, 0, 0, 0, 0, 0, 0];
        let base = buf.as_ptr() as u64;
        crate::emu::set_outgoing_rsp(base, base + 128);
        crate::emu::call_va8(va_or_addr, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
}

/// Call a function at `va_or_addr` with up to 12 arguments.
#[inline(always)]
pub unsafe fn call_va12(
    va_or_addr: u64,
    rcx: u64,
    rdx: u64,
    r8: u64,
    r9: u64,
    arg5: u64,
    arg6: u64,
    arg7: u64,
    arg8: u64,
    arg9: u64,
    arg10: u64,
    arg11: u64,
    arg12: u64,
) -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let addr = lookup_native_va(va_or_addr).unwrap_or_else(|| to_mapped_addr(va_or_addr));
        let f: unsafe extern "win64" fn(
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
            u64,
        ) -> u64 = core::mem::transmute(addr);
        f(
            rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12,
        )
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        let buf: [u64; 16] = [0, 0, 0, 0, 0, 0, 0, 0, arg9, arg10, arg11, arg12, 0, 0, 0, 0];
        let base = buf.as_ptr() as u64;
        crate::emu::set_outgoing_rsp(base, base + 128);
        crate::emu::call_va8(va_or_addr, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8)
    }
}

#[cfg(not(target_arch = "x86_64"))]
static GS_READER: AtomicUsize = AtomicUsize::new(0);

/// Register the host `gs_read(offset) -> usize` callback (`win32_rt::teb::gs_read`)
/// for non-`x86_64` builds.
pub fn set_gs_reader(f: unsafe fn(usize) -> usize) {
    #[cfg(not(target_arch = "x86_64"))]
    GS_READER.store(f as usize, Ordering::Release);
    #[cfg(target_arch = "x86_64")]
    let _ = f;
}

/// `gs:[0x30]` — the current thread's TEB self pointer, as the original code
/// reads it (the loader keeps it valid on every guest thread).
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn teb_self() -> usize {
    let t: usize;
    std::arch::asm!("mov {}, gs:[0x30]", out(reg) t, options(nostack, readonly, preserves_flags));
    t
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
pub unsafe fn teb_self() -> usize {
    gs_slot(0x30 / 8)
}

/// `gs:[8 * slot]` for non-`x86_64` builds. The emulated GS block address is cached in the
/// per-thread [`crate::callctx::CallCtx`] (one inline TSD read), replacing an indirect call to
/// `win32_rt::teb::gs_read` that paid a `_tlv_get_addr` call on every read.
#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
unsafe fn gs_slot(slot: usize) -> usize {
    let c = crate::callctx::ctx();
    let mut p = c.gs.get();
    if p.is_null() {
        p = gs_slots_init(c);
    }
    core::ptr::read(p.add(slot))
}

#[cfg(not(target_arch = "x86_64"))]
#[cold]
#[inline(never)]
fn gs_slots_init(c: &crate::callctx::CallCtx) -> *const usize {
    let p = win32_rt::teb::gs_slots_ptr();
    c.gs.set(p);
    p
}

/// Alias for [`teb_self`].
#[inline(always)]
pub unsafe fn teb() -> usize {
    teb_self()
}

/// `gs:[0x58]` — the TEB's ThreadLocalStoragePointer, as the original code
/// reads it. The exe's static TLS block is entry 0 (`mov rcx, [rax]`: the
/// compiler folds `_tls_index` to 0 for the main image).
#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub unsafe fn tls_pointer() -> usize {
    let t: usize;
    std::arch::asm!("mov {}, gs:[0x58]", out(reg) t, options(nostack, readonly, preserves_flags));
    t
}

#[cfg(not(target_arch = "x86_64"))]
#[inline(always)]
pub unsafe fn tls_pointer() -> usize {
    gs_slot(0x58 / 8)
}

/// Read a `T` from game memory at `addr`, with no alignment requirement.
///
/// **Use this (and [`wr`]) for every access to game memory.** x86 permits
/// misaligned loads and stores, and the original code relies on that: engine
/// headers are often only 4-aligned (e.g. `{ptr, u64}` buffers at `...69c`).
/// A typed Rust dereference (`*(p as *const u64)`, `(*hdr)[0]`, `&*p`) of a
/// misaligned pointer is UB, and debug builds abort on it with "misaligned
/// pointer dereference", which kills the whole process. These compile to the
/// same single `mov` as the original.
///
/// On aarch64, 1/2/4/8-byte accesses go through the TSO helpers in
/// [`crate::simd_ops`] (`ldapr`/`stlr` when naturally aligned), because the
/// original code relies on x86 TSO for lock releases and lock-free publication
/// done with plain `mov`s. Other sizes and unaligned addresses stay plain.
#[inline(always)]
pub unsafe fn rd<T: Copy>(addr: usize) -> T {
    #[cfg(target_arch = "aarch64")]
    {
        let a = addr as u64;
        match core::mem::size_of::<T>() {
            1 => return std::ptr::read_unaligned(&crate::simd_ops::ru8(a) as *const u8 as *const T),
            2 => return std::ptr::read_unaligned(&crate::simd_ops::ru16(a) as *const u16 as *const T),
            4 => return std::ptr::read_unaligned(&crate::simd_ops::ru32(a) as *const u32 as *const T),
            8 => return std::ptr::read_unaligned(&crate::simd_ops::ru64(a) as *const u64 as *const T),
            _ => {}
        }
    }
    std::ptr::read_unaligned(addr as *const T)
}

/// Write a `T` to game memory at `addr`, with no alignment requirement (see [`rd`]).
#[inline(always)]
pub unsafe fn wr<T: Copy>(addr: usize, v: T) {
    #[cfg(target_arch = "aarch64")]
    {
        let a = addr as u64;
        let p = &v as *const T;
        match core::mem::size_of::<T>() {
            1 => return crate::simd_ops::wu8(a, std::ptr::read_unaligned(p as *const u8)),
            2 => return crate::simd_ops::wu16(a, std::ptr::read_unaligned(p as *const u16)),
            4 => return crate::simd_ops::wu32(a, std::ptr::read_unaligned(p as *const u32)),
            8 => return crate::simd_ops::wu64(a, std::ptr::read_unaligned(p as *const u64)),
            _ => {}
        }
    }
    std::ptr::write_unaligned(addr as *mut T, v)
}

// ---------------------------------------------------------------------------------------------
// exe::try_call: catching MSVC C++ exceptions thrown by original callees
// (docs/design/ADVICE_SEH.md §3.3).
// ---------------------------------------------------------------------------------------------

/// `j__CxxThrowException` in the exe (import thunk to vcruntime140).
pub const CXX_THROW_VA: u64 = 0x1_4183_acce;

/// Decorated `TypeDescriptor` names for [`try_call`] catch lists (by-pointer catches, which is how
/// nearly every game `catch` is written).
pub mod cxx_type {
    /// `r::ObjectStreamException_ErrorsOccurred*` (1,987 of the game's 3,005 catch handlers).
    pub const OBJECT_STREAM_ERRORS: &[u8] = b".PEAVObjectStreamException_ErrorsOccurred@r@@";
    /// `r::FileException_FileOpenFailed*`.
    pub const FILE_OPEN_FAILED: &[u8] = b".PEAVFileException_FileOpenFailed@r@@";
    /// `r::FileException*`.
    pub const FILE_EXCEPTION: &[u8] = b".PEAVFileException@r@@";
    /// `r::Exception*`.
    pub const EXCEPTION: &[u8] = b".PEAVException@r@@";
}

/// An MSVC C++ exception (`0xE06D7363`) caught by [`try_call`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CxxException {
    /// Actual address of the `ThrowInfo` (`ExceptionInformation[2]`).
    pub throw_info: u64,
    /// Base of the module that threw (`ExceptionInformation[3]`); `ThrowInfo` fields are RVAs from it.
    pub image_base: u64,
    /// Address of the thrown object at raise time. Dead after the catch (it lived in the
    /// thrower's frame); use [`CxxException::value`].
    pub object: u64,
    /// Index into the catch list of the type that matched, or `u32::MAX` for catch-all.
    pub matched: u32,
    /// Bytes of `value` that are valid (the catchable type's size).
    pub size: u32,
    /// Copy of the exception object as the matched type. By-pointer catches hold the pointer,
    /// already adjusted to the caught base class (`thisDisplacement.mdisp`), like MSVC does.
    pub value: [u8; 32],
}

impl CxxException {
    /// The caught object as a pointer (by-pointer catches).
    pub fn ptr(&self) -> usize {
        u64::from_le_bytes(self.value[..8].try_into().unwrap()) as usize
    }

    /// Decorated name of the first catchable type (the thrown type), for logs.
    pub unsafe fn type_name(&self) -> Option<String> {
        let b = self.image_base;
        let cta = b.wrapping_add(rd::<i32>(self.throw_info as usize + 12) as i64 as u64) as usize;
        if rd::<i32>(cta) <= 0 {
            return None;
        }
        let ct = b.wrapping_add(rd::<i32>(cta + 4) as i64 as u64) as usize;
        let td = b.wrapping_add(rd::<i32>(ct + 4) as i64 as u64) as usize;
        Some(std::ffi::CStr::from_ptr((td + 16) as *const std::ffi::c_char).to_string_lossy().into_owned())
    }

    /// Throw it again from Rust (the `catch (...) { cleanup; throw; }` pattern), through the
    /// exe's `_CxxThrowException` with the original `ThrowInfo`. Only valid for by-pointer or
    /// trivially copyable types (the only ones [`try_call`] catches). The calling replacement must
    /// be entered through a fence (`Throws::Caught` gets one).
    pub unsafe fn rethrow(&self) -> ! {
        assert!(self.size > 0, "exe::try_call: this exception was destroyed by catch-all (not copyable), it cannot be rethrown");
        let mut copy = self.value;
        let _ = call_va!(CXX_THROW_VA, copy.as_mut_ptr(), self.throw_info);
        unreachable!()
    }
}

/// Return registers of a call made by [`try_call`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryRet {
    pub rax: u64,
    /// Low 64 bits of xmm0 (float/double returns).
    pub xmm0: u64,
}

/// Maximum arguments and catch-list entries of [`try_call`].
pub const TRY_MAX_ARGS: usize = 16;
pub const TRY_MAX_CATCH: usize = 8;

/// Shared with the loader's try thunk and its language handler (aw2-loader hooks.rs).
#[repr(C)]
pub struct TryFrame {
    pub rax: u64,
    pub xmm0: u64,
    /// `(ptr, len)` of each decorated type name to catch; `catch_len == 0` catches everything.
    pub catch: *const (usize, usize),
    pub catch_len: usize,
    pub exc: CxxException,
}

/// `thunk(target, args: *const [u64; TRY_MAX_ARGS], nargs, frame) -> 0 (returned) | 1 (caught)`.
pub type TryThunk = unsafe extern "win64" fn(usize, *const u64, usize, *mut TryFrame) -> u64;

static TRY_THUNK: AtomicUsize = AtomicUsize::new(0);

/// Called by the loader once its try thunk is in place.
pub fn set_try_thunk(thunk: usize) {
    TRY_THUNK.store(thunk, Ordering::Release);
}

/// Call the original at `target` (an actual address, e.g. `ptr::<u8>(va) as usize`) with
/// integer/pointer `args` (MSVC x64: the first four also go in xmm0-3 as raw bits, so
/// `f64::to_bits` works for double args there). If it throws an MSVC C++ exception whose
/// catchable types include one of `catch` (decorated `TypeDescriptor` names, see [`cxx_type`]),
/// the frames below are unwound (their destructors run) and `Err` is returned here. An empty
/// `catch` is `catch (...)`. Any other exception passes through to the frames above.
///
/// Only by-pointer and trivially copyable exception types are caught; others pass through. The
/// semantics differ from MSVC in one way: the exception is claimed here without first searching
/// for an outer handler, which only matters for exceptions nothing would catch.
///
/// The caller must hold no live `Drop` values across this call if exceptions not in `catch` can
/// pass through (they discard the Rust frames, see `Throws::Discard`).
pub unsafe fn try_call(target: usize, args: &[u64], catch: &[&[u8]]) -> Result<TryRet, CxxException> {
    assert!(args.len() <= TRY_MAX_ARGS && catch.len() <= TRY_MAX_CATCH, "exe::try_call: too many args/catch types");
    let mut a = [0u64; TRY_MAX_ARGS];
    a[..args.len()].copy_from_slice(args);
    let mut names = [(0usize, 0usize); TRY_MAX_CATCH];
    for (n, c) in names.iter_mut().zip(catch) {
        *n = (c.as_ptr() as usize, c.len());
    }
    // aarch64 (T4, eh_lift_plan.md): the boundary is an EH record on the T3 chain whose handler
    // claims what `cxx_match` accepts (dispatch phase, the object still alive) and unwinds the
    // frames below (their cleanups run) to it. The call is made here, as the loader's thunk
    // does, but guest code runs as a transparent emulator region (`emu_run_full_rax`): the
    // thunk's `emu_run_full` region is opaque and would stop the dispatch before the boundary.
    #[cfg(not(target_arch = "x86_64"))]
    {
        const ORIG_TRAMPOLINE_TAG: usize = 1 << 46; // aw2-loader hooks.rs
        let mut exc: Option<CxxException> = None;
        let names = &names[..catch.len()];
        let va = (target & !ORIG_TRAMPOLINE_TAG) as u64;
        let gpr: [u64; 8] = core::array::from_fn(|i| a[i]);
        let st: [u64; 8] = core::array::from_fn(|i| a[8 + i]);
        let xmm = [a[0] as u128, a[1] as u128, a[2] as u128, a[3] as u128];
        let host = if target & ORIG_TRAMPOLINE_TAG == 0 { crate::emu::resolve_host_callable(va) } else { None };
        let r = crate::eh_rt::host_try::catch_cxx_if(
            &mut |rec: &win32_rt::unwind::ExceptionRecord| {
                exc = cxx_match(rec, names);
                exc.map(|_| [0, 0])
            },
            || match host {
                Some(h) => crate::emu::call_host_for_va(va, h, gpr, xmm, st),
                None => crate::emu::emu_run_full_rax(va, gpr, xmm, st, 0),
            },
        );
        match r {
            Ok((rax, x0)) => Ok(TryRet { rax, xmm0: x0 as u64 }),
            Err(_) => Err(exc.expect("exe::try_call: landed without a matched exception")),
        }
    }
    #[cfg(target_arch = "x86_64")]
    {
        let thunk = TRY_THUNK.load(Ordering::Acquire);
        assert!(thunk != 0, "exe::try_call: the loader did not install its try thunk");
        let mut f = TryFrame {
            rax: 0,
            xmm0: 0,
            catch: names.as_ptr(),
            catch_len: catch.len(),
            exc: CxxException { throw_info: 0, image_base: 0, object: 0, matched: 0, size: 0, value: [0; 32] },
        };
        let t: TryThunk = std::mem::transmute(thunk);
        let caught = t(target, a.as_ptr(), args.len(), &mut f);
        let f = std::ptr::read_volatile(&f);
        if caught == 0 {
            Ok(TryRet { rax: f.rax, xmm0: f.xmm0 })
        } else {
            Err(f.exc)
        }
    }
}

/// Match an MSVC C++ exception against a catch list of decorated `TypeDescriptor` names
/// (`(ptr, len)`; empty = `catch (...)`), as [`try_call`] catches: by-pointer and trivially
/// copyable objects are copied (by-pointer: adjusted to the matched base, `mdisp`); a
/// `catch (...)` of an object that is neither is destroyed here (no value, no rethrow); any
/// other match is not caught. MSVC x64 layouts, all RVAs from `ExceptionInformation[3]`:
/// `ThrowInfo { attributes, pmfnUnwind, pForwardCompat, pCatchableTypeArray }`,
/// `CatchableTypeArray { n, rva[n] }`, `CatchableType { properties, pType, PMD { mdisp, pdisp,
/// vdisp }, sizeOrOffset, copyFunction }`, `TypeDescriptor { vftable, spare, name[] }`.
/// (The x86_64 loader thunk's handler does the same, aw2-loader hooks.rs.)
pub unsafe fn cxx_match(r: &win32_rt::unwind::ExceptionRecord, wanted: &[(usize, usize)]) -> Option<CxxException> {
    if r.code != win32_rt::unwind::CXX_EXCEPTION || r.number_parameters < 4 || !(0x1993_0520..=0x1993_0522).contains(&r.information[0]) {
        return None;
    }
    let (obj, ti, base) = (r.information[1], r.information[2], r.information[3]);
    if ti == 0 {
        return None; // `throw;` outside a catch block: nothing to match
    }
    let rva = |at: u64| base.wrapping_add(rd::<i32>(at as usize) as i64 as u64);
    let dtor = rd::<i32>(ti as usize + 4);
    let cta = rva(ti + 12);
    for i in 0..rd::<i32>(cta as usize).max(0) as u64 {
        let ct = rva(cta + 4 + 4 * i);
        let td = rva(ct + 4);
        let name = std::ffi::CStr::from_ptr((td + 16) as *const std::ffi::c_char).to_bytes();
        let matched = if wanted.is_empty() {
            u32::MAX
        } else {
            match wanted.iter().position(|&(p, l)| std::slice::from_raw_parts(p as *const u8, l) == name) {
                Some(m) => m as u32,
                None => continue,
            }
        };
        let props = rd::<u32>(ct as usize);
        let (mdisp, pdisp) = (rd::<i32>(ct as usize + 8), rd::<i32>(ct as usize + 12));
        let size = rd::<i32>(ct as usize + 20);
        let copy_fn = rd::<i32>(ct as usize + 24);
        let is_ptr = name.starts_with(b".PE") && size == 8;
        let copyable = (props & 1 != 0 || copy_fn == 0) && (0..=32).contains(&size) && (is_ptr || dtor == 0) && (!is_ptr || pdisp < 0);
        let mut exc = CxxException { throw_info: ti, image_base: base, object: obj, matched, size: 0, value: [0; 32] };
        if copyable {
            std::ptr::copy_nonoverlapping(obj as *const u8, exc.value.as_mut_ptr(), size as usize);
            exc.size = size as u32;
            if is_ptr {
                let p = exc.ptr() as u64;
                if p != 0 {
                    exc.value[..8].copy_from_slice(&p.wrapping_add(mdisp as i64 as u64).to_le_bytes());
                }
            }
        } else if wanted.is_empty() {
            // catch (...) of an object we can't copy: destroy it as MSVC does at the end of the
            // catch block (no value, no rethrow). The destructor is guest code (or its lift).
            if dtor != 0 {
                call_va8(base.wrapping_add(dtor as i64 as u64), obj, 0, 0, 0, 0, 0, 0, 0);
            }
        } else {
            log::warn!(
                "exe::try_call: not catching {:?} (not by-pointer or trivially copyable: props 0x{props:x}, size {size}, copy fn 0x{copy_fn:x}, dtor 0x{dtor:x})",
                String::from_utf8_lossy(name)
            );
            return None;
        }
        return Some(exc);
    }
    None
}
