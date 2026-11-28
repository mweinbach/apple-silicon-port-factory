//! Guest memory-ordering helpers and code-modification diagnostics for the interpreter.
//!
//! x86-64 is TSO: every load has acquire and every store has release semantics. The
//! interpreter performs guest loads/stores as plain host accesses, and aarch64 is weakly
//! ordered, so lock-free guest code that relies on x86 ordering without fences (std::atomic
//! acquire/release compiles to plain `mov` on x64) can observe reordered memory on the host.
//! Setting [`TSO`] to `true` makes every interpreted load an acquire (`ldapr`/`ldar`) and every
//! store a release (`stlr`); unaligned accesses fall back to a plain access plus a `dmb`.
//! It is a compile-time switch (off) because a runtime check costs ~2-3% of interpreter time
//! and enabling it did not change the V8 crash that motivated it (that was `rep movs`).

use core::sync::atomic::{fence, AtomicU16, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};

/// Default TSO emulation switch for guest loads/stores (`PORT_JIT_TSO=0` disables TSO
/// acquire/release ordering on non-atomic guest loads/stores while preserving explicit
/// `lock` atomics and fences).
const TSO_DEFAULT: bool = true;

/// Default PE `IMAGE_VOLATILE_METADATA` TSO elision switch (`PORT_JIT_VOLATILE_META=1` opts in).
const VOLATILE_META_DEFAULT: bool = false;

#[inline]
fn parse_opt_in_bool(s: &str) -> bool {
    matches!(
        s.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on"
    )
}

#[cfg(not(test))]
static TSO_STATE: AtomicU8 = AtomicU8::new(0);
#[cfg(not(test))]
static TSO_VEC_STATE: AtomicU8 = AtomicU8::new(0);
#[cfg(not(test))]
static TSO_MEMCPY_STATE: AtomicU8 = AtomicU8::new(0);
#[cfg(not(test))]
static VOLATILE_META_STATE: AtomicU8 = AtomicU8::new(0);

#[cfg(not(test))]
#[cold]
#[inline(never)]
fn init_tso() -> bool {
    let on = std::env::var("PORT_JIT_TSO").map_or(TSO_DEFAULT, |s| s.trim() != "0");
    if !on {
        log::info!("emu: non-TSO non-atomic guest loads/stores (PORT_JIT_TSO=0)");
    }
    TSO_STATE.store(if on { 2 } else { 1 }, Ordering::Relaxed);
    on
}

#[cfg(not(test))]
#[cold]
#[inline(never)]
fn init_tso_vec() -> bool {
    let on = std::env::var("PORT_JIT_TSO_VEC").map_or_else(|_| tso(), |s| s.trim() != "0");
    if !on {
        log::info!("emu: non-TSO vector loads/stores (PORT_JIT_TSO_VEC=0)");
    }
    TSO_VEC_STATE.store(if on { 2 } else { 1 }, Ordering::Relaxed);
    on
}

#[cfg(not(test))]
#[cold]
#[inline(never)]
fn init_tso_memcpy() -> bool {
    let on = std::env::var("PORT_JIT_TSO_MEMCPY").map_or_else(|_| tso(), |s| s.trim() != "0");
    if !on {
        log::info!("emu: non-TSO bulk string/memcpy ops (PORT_JIT_TSO_MEMCPY=0)");
    }
    TSO_MEMCPY_STATE.store(if on { 2 } else { 1 }, Ordering::Relaxed);
    on
}

#[cfg(not(test))]
#[cold]
#[inline(never)]
fn init_volatile_meta() -> bool {
    let on = std::env::var("PORT_JIT_VOLATILE_META")
        .map_or(VOLATILE_META_DEFAULT, |s| parse_opt_in_bool(&s));
    if on {
        log::info!("emu: PE VolatileMetadata TSO elision enabled (PORT_JIT_VOLATILE_META=1)");
    }
    VOLATILE_META_STATE.store(if on { 2 } else { 1 }, Ordering::Relaxed);
    on
}

/// Whether TSO emulation is enabled for non-atomic guest loads/stores.
#[inline(always)]
pub(super) fn tso() -> bool {
    #[cfg(test)]
    {
        TSO_DEFAULT && !TEST_NO_TSO.with(|c| c.get())
    }
    #[cfg(not(test))]
    {
        match TSO_STATE.load(Ordering::Relaxed) {
            2 => true,
            1 => false,
            _ => init_tso(),
        }
    }
}

/// Whether TSO barriers are enabled for 128/256-bit vector loads/stores (`PORT_JIT_TSO_VEC`,
/// inheriting `PORT_JIT_TSO` when unset).
#[inline(always)]
pub(super) fn tso_vec() -> bool {
    #[cfg(test)]
    {
        if let Some(v) = TEST_TSO_VEC.with(|c| c.get()) {
            return v;
        }
        if let Ok(s) = std::env::var("PORT_JIT_TSO_VEC") {
            return s.trim() != "0";
        }
        tso()
    }
    #[cfg(not(test))]
    {
        match TSO_VEC_STATE.load(Ordering::Relaxed) {
            2 => true,
            1 => false,
            _ => init_tso_vec(),
        }
    }
}

/// Whether TSO ordering is enabled for bulk string/memcpy ops (`PORT_JIT_TSO_MEMCPY`,
/// inheriting `PORT_JIT_TSO` when unset).
#[inline(always)]
pub(super) fn tso_memcpy() -> bool {
    #[cfg(test)]
    {
        if let Some(v) = TEST_TSO_MEMCPY.with(|c| c.get()) {
            return v;
        }
        if let Ok(s) = std::env::var("PORT_JIT_TSO_MEMCPY") {
            return s.trim() != "0";
        }
        tso()
    }
    #[cfg(not(test))]
    {
        match TSO_MEMCPY_STATE.load(Ordering::Relaxed) {
            2 => true,
            1 => false,
            _ => init_tso_memcpy(),
        }
    }
}

/// Whether PE `IMAGE_LOAD_CONFIG_DIRECTORY64.VolatileMetadataPointer` TSO elision is enabled
/// (`PORT_JIT_VOLATILE_META`, default off; opt in with `PORT_JIT_VOLATILE_META=1`).
///
/// Kept opt-in because real-world Windows x86-64 binaries frequently rely on hardware x86-TSO
/// load-load and store-store ordering for non-volatile lock-free structures (e.g. in-band MPSC
/// ring-buffer slot headers) that MSVC `/volatileMetadata` omits from `VolatileAccessTable`.
/// Eliding TSO on those accesses causes ARM64 reordering races even when all stores retain `stlr`.
#[inline(always)]
pub(super) fn volatile_meta_enabled() -> bool {
    #[cfg(test)]
    {
        if let Some(v) = TEST_VOLATILE_META.with(|c| c.get()) {
            return v;
        }
        if let Ok(s) = std::env::var("PORT_JIT_VOLATILE_META") {
            return parse_opt_in_bool(&s);
        }
        VOLATILE_META_DEFAULT
    }
    #[cfg(not(test))]
    {
        match VOLATILE_META_STATE.load(Ordering::Relaxed) {
            2 => true,
            1 => false,
            _ => init_volatile_meta(),
        }
    }
}

#[cfg(test)]
std::thread_local! {
    /// Test-only: disable TSO emulation on this thread (interpreter and translations made
    /// on it), for measuring its cost (`jit_bench`).
    pub(super) static TEST_NO_TSO: core::cell::Cell<bool> = const { core::cell::Cell::new(false) };
    /// Test-only override for `PORT_JIT_TSO_VEC` on this thread.
    pub(super) static TEST_TSO_VEC: core::cell::Cell<Option<bool>> = const { core::cell::Cell::new(None) };
    /// Test-only override for `PORT_JIT_TSO_MEMCPY` on this thread.
    pub(super) static TEST_TSO_MEMCPY: core::cell::Cell<Option<bool>> = const { core::cell::Cell::new(None) };
    /// Test-only override for `PORT_JIT_VOLATILE_META` on this thread.
    pub(super) static TEST_VOLATILE_META: core::cell::Cell<Option<bool>> = const { core::cell::Cell::new(None) };
}

#[cold]
#[inline(never)]
unsafe fn load_acq_unaligned(p: usize, bits: u32) -> u64 {
    let v = match bits {
        16 => core::ptr::read_unaligned(p as *const u16) as u64,
        32 => core::ptr::read_unaligned(p as *const u32) as u64,
        _ => core::ptr::read_unaligned(p as *const u64),
    };
    fence(Ordering::Acquire);
    v
}

/// Acquire load of `bits` (8/16/32/64) at `p`.
#[inline(always)]
pub(super) unsafe fn load_acq(p: usize, bits: u32) -> u64 {
    match bits {
        8 => (*(p as *const AtomicU8)).load(Ordering::Acquire) as u64,
        16 => {
            if (p & 1) == 0 {
                (*(p as *const AtomicU16)).load(Ordering::Acquire) as u64
            } else {
                load_acq_unaligned(p, 16)
            }
        }
        32 => {
            if (p & 3) == 0 {
                (*(p as *const AtomicU32)).load(Ordering::Acquire) as u64
            } else {
                load_acq_unaligned(p, 32)
            }
        }
        64 => {
            if (p & 7) == 0 {
                (*(p as *const AtomicU64)).load(Ordering::Acquire)
            } else {
                load_acq_unaligned(p, 64)
            }
        }
        _ => load_acq_unaligned(p, bits),
    }
}

#[cold]
#[inline(never)]
unsafe fn store_rel_unaligned(p: usize, bits: u32, v: u64) {
    fence(Ordering::Release);
    match bits {
        16 => core::ptr::write_unaligned(p as *mut u16, v as u16),
        32 => core::ptr::write_unaligned(p as *mut u32, v as u32),
        _ => core::ptr::write_unaligned(p as *mut u64, v),
    }
}

/// Release store of the low `bits` (8/16/32/64) of `v` at `p`.
#[inline(always)]
pub(super) unsafe fn store_rel(p: usize, bits: u32, v: u64) {
    match bits {
        8 => (*(p as *const AtomicU8)).store(v as u8, Ordering::Release),
        16 => {
            if (p & 1) == 0 {
                (*(p as *const AtomicU16)).store(v as u16, Ordering::Release);
            } else {
                store_rel_unaligned(p, 16, v);
            }
        }
        32 => {
            if (p & 3) == 0 {
                (*(p as *const AtomicU32)).store(v as u32, Ordering::Release);
            } else {
                store_rel_unaligned(p, 32, v);
            }
        }
        64 => {
            if (p & 7) == 0 {
                (*(p as *const AtomicU64)).store(v, Ordering::Release);
            } else {
                store_rel_unaligned(p, 64, v);
            }
        }
        _ => store_rel_unaligned(p, bits, v),
    }
}

/// 128-bit load with acquire ordering (x86 does not make 16-byte accesses atomic, only ordered).
#[inline(always)]
pub(super) unsafe fn load128_acq(p: usize) -> u128 {
    let v = core::ptr::read_unaligned(p as *const u128);
    fence(Ordering::Acquire);
    v
}

/// 128-bit store with release ordering.
#[inline(always)]
pub(super) unsafe fn store128_rel(p: usize, v: u128) {
    fence(Ordering::Release);
    core::ptr::write_unaligned(p as *mut u128, v);
}

// ---------------------------------------------------------------------------------------------
// Code-modification diagnostics (`AW2_EMU_CODECHECK=1`).
// ---------------------------------------------------------------------------------------------

static CODECHECK_STATE: AtomicU8 = AtomicU8::new(0);

#[cold]
#[inline(never)]
fn init_codecheck() -> bool {
    let on = std::env::var("AW2_EMU_CODECHECK").is_ok_and(|s| s.trim() == "1");
    CODECHECK_STATE.store(if on { 2 } else { 1 }, Ordering::Relaxed);
    on
}

/// Whether code-modification diagnostics are enabled.
#[inline(always)]
pub(super) fn codecheck() -> bool {
    match CODECHECK_STATE.load(Ordering::Relaxed) {
        2 => true,
        1 => false,
        _ => init_codecheck(),
    }
}

std::thread_local! {
    static FILL_HASHES: std::cell::RefCell<std::collections::HashMap<u64, (u64, u32)>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

static CHANGED: AtomicU64 = AtomicU64::new(0);

fn fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// Called on every block-cache fill: remembers the hash of the block's bytes per thread and
/// reports when a re-fill at the same guest IP finds different bytes (i.e. the code at that
/// address was rewritten, so any still-cached decode of it would have been stale).
pub(super) unsafe fn on_fill(ip: u64, mapped: usize, byte_len: usize) {
    if byte_len == 0 {
        return;
    }
    let h = fnv(core::slice::from_raw_parts(mapped as *const u8, byte_len));
    let changed = FILL_HASHES.with(|m| {
        let mut m = m.borrow_mut();
        match m.insert(ip, (h, byte_len as u32)) {
            Some((old, old_len)) => old != h || old_len != byte_len as u32,
            None => false,
        }
    });
    if changed {
        let n = CHANGED.fetch_add(1, Ordering::Relaxed) + 1;
        if n <= 50 || n.is_power_of_two() {
            log::error!("emu codecheck: code at guest ip {ip:#x} changed since it was last decoded on this thread (#{n})");
        }
    }
}

// ---------------------------------------------------------------------------------------------
// JIT code tracking: decoded blocks from run-time generated code (V8's code space) must be
// revalidated after the guest rewrites that memory, otherwise the per-thread block cache would
// keep executing the instructions of whatever code previously lived at the same address.
// ---------------------------------------------------------------------------------------------

/// Generation of guest-executable memory; bumped on every protection/commit change of an
/// executable region (V8 toggles its code pages RWX <-> RX around every code write). Never 0
/// (0 marks image blocks, which are never revalidated).
static CODE_GEN: AtomicU32 = AtomicU32::new(1);

const MAX_JIT_REGIONS: usize = 32;

/// Contiguous `[lo, hi)` pair kept adjacent in one 16-byte slot so `in_jit` / `jit_overlaps`
/// hit a single cache line per region (4 regions per 64-byte line).
#[repr(C)]
struct JitRegion {
    lo: AtomicUsize,
    hi: AtomicUsize,
}

impl JitRegion {
    const fn zero() -> Self {
        Self { lo: AtomicUsize::new(0), hi: AtomicUsize::new(0) }
    }
}

static JIT_REGIONS: [JitRegion; MAX_JIT_REGIONS] = [const { JitRegion::zero() }; MAX_JIT_REGIONS];
static JIT_N: AtomicUsize = AtomicUsize::new(0);
static JIT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Called (via the loader-installed hook in `win32_rt::kernel32::memory`) whenever guest
/// memory in `[lo, hi)` becomes, stops being, or changes protection while executable.
pub fn note_code_region(lo: usize, hi: usize) {
    if hi <= lo {
        return;
    }
    {
        let _g = JIT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let n = JIT_N.load(Ordering::Relaxed);
        // Merge with an existing region if overlapping or close (V8 commits code pages
        // piecewise inside one reservation).
        const SLACK: usize = 16 << 20;
        let mut grew = false;
        let mut merged = false;
        for r in &JIT_REGIONS[..n.min(MAX_JIT_REGIONS)] {
            let (a, b) = (r.lo.load(Ordering::Relaxed), r.hi.load(Ordering::Relaxed));
            if lo <= b.saturating_add(SLACK) && hi.saturating_add(SLACK) >= a {
                let nlo = a.min(lo);
                let nhi = b.max(hi);
                if nlo != a || nhi != b {
                    r.lo.store(nlo, Ordering::Relaxed);
                    r.hi.store(nhi, Ordering::Relaxed);
                    grew = true;
                }
                merged = true;
                break;
            }
        }
        if !merged {
            grew = true;
            if n < MAX_JIT_REGIONS {
                JIT_REGIONS[n].lo.store(lo, Ordering::Relaxed);
                JIT_REGIONS[n].hi.store(hi, Ordering::Relaxed);
                JIT_N.store(n + 1, Ordering::Release);
                log::info!("emu: tracking guest JIT code region {lo:#x}..{hi:#x}");
            } else {
                let last = &JIT_REGIONS[MAX_JIT_REGIONS - 1];
                last.lo.store(last.lo.load(Ordering::Relaxed).min(lo), Ordering::Relaxed);
                last.hi.store(last.hi.load(Ordering::Relaxed).max(hi), Ordering::Relaxed);
            }
        }
        if grew {
            let g = JIT_REGIONS_GEN.fetch_add(1, Ordering::Release).wrapping_add(1);
            if g == 0 {
                JIT_REGIONS_GEN.fetch_add(1, Ordering::Release);
            }
        }
    }
    let mut g = CODE_GEN.fetch_add(1, Ordering::Release).wrapping_add(1);
    if g == 0 {
        g = CODE_GEN.fetch_add(1, Ordering::Release).wrapping_add(1);
    }
    let _ = g;
    // AOT code of an image that now overlaps run-time code is dropped (see `aot.rs`).
    #[cfg(target_arch = "aarch64")]
    super::aot::note_code_region();
}

/// Unsigned half-open range membership test: `p in [lo, hi)` assuming `lo <= hi`.
#[inline(always)]
fn range_contains(p: usize, lo: usize, hi: usize) -> bool {
    p.wrapping_sub(lo) < hi.wrapping_sub(lo)
}

#[inline(never)]
fn jit_overlaps_slow(lo: usize, hi: usize) -> bool {
    let n = JIT_N.load(Ordering::Acquire).min(MAX_JIT_REGIONS);
    for r in &JIT_REGIONS[..n] {
        let rlo = r.lo.load(Ordering::Relaxed);
        let rhi = r.hi.load(Ordering::Relaxed);
        if lo < rhi && rlo < hi {
            return true;
        }
    }
    false
}

/// Whether `[lo, hi)` overlaps a tracked JIT region.
#[allow(dead_code)]
#[inline(always)]
pub(super) fn jit_overlaps(lo: usize, hi: usize) -> bool {
    if lo >= hi || JIT_N.load(Ordering::Relaxed) == 0 {
        return false;
    }
    jit_overlaps_slow(lo, hi)
}

/// Monotonic counter bumped whenever a new guest JIT/SMC region is added or expanded.
/// Used by per-thread translation caches to flush inline caches and direct links that may
/// have targeted an address before it became dynamic/self-modifying code.
static JIT_REGIONS_GEN: AtomicU32 = AtomicU32::new(1);

/// Current tracked-JIT-regions generation (never 0).
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
#[inline(always)]
pub(super) fn jit_regions_gen() -> u32 {
    let g = JIT_REGIONS_GEN.load(Ordering::Acquire);
    if g == 0 { 1 } else { g }
}

/// Current executable-memory generation (never 0).
#[inline(always)]
pub(super) fn code_gen() -> u32 {
    let g = CODE_GEN.load(Ordering::Relaxed);
    if g == 0 { 1 } else { g }
}

/// Address of the executable-memory generation counter (read by translated code).
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
pub(super) fn code_gen_ptr() -> usize {
    &CODE_GEN as *const AtomicU32 as usize
}

#[inline(never)]
fn in_jit_slow(p: usize) -> bool {
    let n = JIT_N.load(Ordering::Acquire).min(MAX_JIT_REGIONS);
    for r in &JIT_REGIONS[..n] {
        let lo = r.lo.load(Ordering::Relaxed);
        let hi = r.hi.load(Ordering::Relaxed);
        if range_contains(p, lo, hi) {
            return true;
        }
    }
    false
}

/// Whether host address `p` lies in a tracked JIT region.
#[inline(always)]
pub(super) fn in_jit(p: usize) -> bool {
    if JIT_N.load(Ordering::Relaxed) == 0 {
        return false;
    }
    in_jit_slow(p)
}

std::thread_local! {
    /// Per-thread hash of the bytes each cached JIT block was decoded from.
    static JIT_HASH: std::cell::RefCell<std::collections::HashMap<u64, u64>> =
        std::cell::RefCell::new(std::collections::HashMap::new());
}

/// Fast hash of `len` code bytes at `p` (also mixes in the length).
#[inline]
pub(super) unsafe fn code_hash(p: usize, len: usize) -> u64 {
    let mut h: u64 = 0x9E37_79B9_7F4A_7C15 ^ (len as u64);
    let mut i = 0;
    while i + 8 <= len {
        let w = core::ptr::read_unaligned((p + i) as *const u64);
        h = (h ^ w).wrapping_mul(0xff51_afd7_ed55_8ccd).rotate_left(29);
        i += 8;
    }
    while i < len {
        h = (h ^ *((p + i) as *const u8) as u64).wrapping_mul(0x100_0000_01b3);
        i += 1;
    }
    h
}

/// Remember the bytes a JIT block at `ip` (host address `p`, `len` bytes) was decoded from.
pub(super) unsafe fn jit_record(ip: u64, p: usize, len: usize) {
    let h = code_hash(p, len);
    JIT_HASH.with(|m| {
        m.borrow_mut().insert(ip, h);
    });
}

static STALE: AtomicU64 = AtomicU64::new(0);

/// After a code-generation bump: whether the cached JIT block at `ip` still matches memory.
/// Logs (rate-limited) when it does not, i.e. when the stale decode would have executed.
pub(super) unsafe fn jit_unchanged(ip: u64, p: usize, len: usize) -> bool {
    let h = code_hash(p, len);
    let same = JIT_HASH.with(|m| m.borrow().get(&ip).is_some_and(|&old| old == h));
    if !same {
        let n = STALE.fetch_add(1, Ordering::Relaxed) + 1;
        if n <= 20 || n.is_power_of_two() {
            log::warn!("emu: guest JIT code at {ip:#x} was rewritten; re-decoding the cached block (stale #{n})");
        }
    }
    same
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_range_contains_unsigned_boundaries() {
        assert!(!range_contains(0, 0, 0));
        assert!(!range_contains(0x1000, 0x1000, 0x1000));
        assert!(range_contains(0x1000, 0x1000, 0x2000));
        assert!(range_contains(0x1FFF, 0x1000, 0x2000));
        assert!(!range_contains(0x0FFF, 0x1000, 0x2000));
        assert!(!range_contains(0x2000, 0x1000, 0x2000));
        assert!(!range_contains(0, 1, usize::MAX));
        assert!(range_contains(1, 1, usize::MAX));
        assert!(range_contains(usize::MAX - 1, 1, usize::MAX));
        assert!(!range_contains(usize::MAX, 1, usize::MAX));
    }

    #[test]
    fn mem_load_acq_store_rel_aligned_and_unaligned_all_widths() {
        let mut buf = [0u8; 64];
        let base = buf.as_mut_ptr() as usize;

        for off in 0usize..16 {
            let p = base + off;
            unsafe {
                // 8-bit
                store_rel(p, 8, 0xAB);
                assert_eq!(load_acq(p, 8), 0xAB, "8-bit off={off}");

                // 16-bit (aligned on even offsets, unaligned on odd offsets)
                store_rel(p, 16, 0xBEEF);
                assert_eq!(load_acq(p, 16), 0xBEEF, "16-bit off={off}");

                // 32-bit
                store_rel(p, 32, 0xCAFE_BABE);
                assert_eq!(load_acq(p, 32), 0xCAFE_BABE, "32-bit off={off}");

                // 64-bit
                store_rel(p, 64, 0x0123_4567_89AB_CDEF);
                assert_eq!(load_acq(p, 64), 0x0123_4567_89AB_CDEF, "64-bit off={off}");

                // 128-bit
                let v128: u128 = 0xFEDC_BA98_7654_3210_1122_3344_5566_7788;
                store128_rel(p, v128);
                assert_eq!(load128_acq(p), v128, "128-bit off={off}");
            }
        }
    }

    #[test]
    fn mem_note_code_region_and_jit_queries() {
        // Use high non-heap sentinel addresses that cannot collide with real process allocations.
        let lo = 0x7FFF_E000_0000usize;
        let hi = 0x7FFF_E001_0000usize;
        let gen0 = code_gen();
        assert_ne!(gen0, 0);

        note_code_region(lo, hi);
        assert_ne!(code_gen(), gen0);
        assert!(in_jit(lo));
        assert!(in_jit(hi - 1));
        assert!(!in_jit(lo - 1));
        assert!(!in_jit(hi));

        assert!(jit_overlaps(lo - 0x100, lo + 1));
        assert!(jit_overlaps(hi - 1, hi + 0x100));
        assert!(jit_overlaps(lo + 0x100, hi - 0x100));
        assert!(!jit_overlaps(lo - 0x1000, lo));
        assert!(!jit_overlaps(hi, hi + 0x1000));
        assert!(!jit_overlaps(lo + 0x100, lo + 0x100)); // empty interval
        assert!(!jit_overlaps(hi, lo)); // inverted interval
    }
}
