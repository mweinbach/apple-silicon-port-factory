//! Live AOT module directory, lock-free shared probe table, linking/unlinking, `va_gen`
//! revalidation & revival, cache loading, and loader activation entry points.

use std::collections::{HashMap, HashSet};
use std::hash::{BuildHasherDefault, Hasher};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock, RwLock};

use super::bg::{bg_enqueue, bg_job_for_module, sweep_start, touch};
use super::builder::{build_all, split_and_rebase_block};
#[cfg(test)]
use super::builder::build_image;
use super::format::{
    codegen_hash, decode_for_runtime, image_hash, module_key_hashed, read_cache_file,
    read_cache_file_in, set_flags, word_to_off, write_atomic, AotBytes, AotKey, BlockEnt, Image,
    LinkEnt, RelocEnt, AOT_FAR_SLAB, AOT_LSE2_SLAB, FLAG_VERIFY_PENDING,
};
#[cfg(test)]
use super::format::{decode, encode, BUILD_VBASE};
use super::{
    aot_mode, build_request, cache_dir, env_u32_any, fallback_cache_dirs_for, profile_enabled,
    verify_mode, ACTIVATED,
};
#[cfg(test)]
use super::{
    TEST_AOT, TEST_HELD, TEST_NO_REVIVE, TEST_REBASED, TEST_VERIFIED, TEST_VERIFY_MISMATCHES,
};
use crate::emu::exec_mem::{
    alloc_jit, bind_static_aot_module, free_jit, has_bound_static_aot, icache_invalidate,
    jit_writable, jit_write_protect, lookup_static_aot, static_probe_table,
};
use crate::emu::jit::{
    self, AOT_STUBS_BYTES, RELOC_HOST_FN, RELOC_QPC_STATE, RELOC_SLOT_VAL, RELOC_XINPUT_STATE,
};
use crate::emu::BlockCache;

/// Fast non-cryptographic hasher for internal guest-VA / host-PC maps (never exposed to untrusted
/// hash-flooding inputs; avoids SipHash overhead across millions of AOT blocks and link sites).
#[derive(Default)]
pub(super) struct FastIntHasher(u64);

impl Hasher for FastIntHasher {
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ (b as u64)).wrapping_mul(0x517c_c1b7_2722_0a95);
        }
    }
    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
    #[inline]
    fn write_usize(&mut self, i: usize) {
        self.write_u64(i as u64);
    }
    #[inline]
    fn finish(&self) -> u64 {
        // Spread entropy into both top 7 bits (hashbrown H2 tag) and low bits (bucket index).
        let h = self.0.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        h ^ (h >> 32)
    }
}

pub(super) type FastMap<K, V> = HashMap<K, V, BuildHasherDefault<FastIntHasher>>;
pub(super) type FastSet<K> = HashSet<K, BuildHasherDefault<FastIntHasher>>;

/// Minimum block/link count to fan out assumption checking or initial linking across workers.
const PARALLEL_THRESHOLD: usize = 32_768;

/// Worker thread count for parallel AOT install / revalidation passes.
#[inline]
fn aot_install_workers() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get()).clamp(1, 8)
}

/// Fast zero-initialized boxed atomic byte slice (avoids per-element iterator init over millions
/// of blocks by delegating to `alloc_zeroed`).
fn alloc_atomic_u8_zeroed(n: usize) -> Box<[AtomicU8]> {
    const {
        assert!(core::mem::size_of::<AtomicU8>() == 1 && core::mem::align_of::<AtomicU8>() == 1);
    }
    let raw = Box::into_raw(vec![0u8; n].into_boxed_slice());
    unsafe { Box::from_raw(raw as *mut [AtomicU8]) }
}

/// Per-bank far-branch island slab: 16-byte immutable veneers (`ldr x16, .+8; br x16; .quad dst`)
/// deduplicated by destination host code address so cross-bank and cross-module links beyond
/// ±128 MiB `b`/`bl` range chain directly while preserving `x30` (`LR`) for calls.
struct FarSlab {
    lo: usize,
    used: usize,
    len: usize,
    islands: FastMap<usize, usize>,
}

pub(super) struct Module {
    pub(super) name: String,
    base: usize,
    size: usize,
    /// `MAP_JIT` region: bank 0 stubs at offset 0, followed by per-bank code, LSE2, and far slabs.
    region: usize,
    /// Total byte length of the `MAP_JIT` allocation starting at `region`.
    region_len: usize,
    /// Per-bank runtime byte shift relative to `img`'s flat offsets:
    /// `host_addr(off_in_bank_k) == region + off + bank_shifts[k]`.
    /// Single-bank images have `[0]`; multi-bank images insert LSE2 + far-island slabs after each
    /// bank so every instruction stays well within ±128 MiB `b_reach` of its bank's slabs.
    bank_shifts: Vec<usize>,
    /// Per-bank far-branch island slabs (same length as `bank_shifts`).
    far_slabs: Vec<FarSlab>,
    img: Image,
    /// Per block: 0 = alive, [`DEAD_PERM`] or [`DEAD_ASSUME`].
    dead: Box<[AtomicU8]>,
    /// Per link: currently patched to its target.
    linked: Vec<bool>,
    /// Indices of currently unlinked link sites in `0..img.links.len()` (valid once
    /// `unlinked_init` is true). Avoids O(M^2) rescans of already-linked sites across modules.
    unlinked: Vec<u32>,
    /// True after this module's initial `link_pass` populated `unlinked`.
    unlinked_init: bool,
    /// True when this module's `MAP_JIT` region was installed during a deferred startup batch
    /// and still needs its bulk icache invalidation after batch linking completes.
    needs_icache_flush: bool,
    /// Per block: has a held import slot ([`rebase_block`]); never verified (its
    /// re-translation sees a different slot state, its guarded call is still correct).
    slot_held: Vec<bool>,
    /// Sorted block indices in `img.blocks` that must still be checked on an out-of-module
    /// `VA_DISPATCH` delta revalidation: blocks with relocations, `host == true` assumptions,
    /// external assumptions, primary-EXE `FF 25` thunk or DLL IAT-stub assumptions, or
    /// `DEAD_ASSUME`/`DEAD_HELD` revival candidates.
    watch_blocks: Vec<u32>,
    /// `exe::va_structural_gen()` when `watch_blocks` was last built.
    watch_struct_gen: u32,
    /// Image overlapped run-time code: every block dead.
    killed: bool,
    /// Removed (tests).
    pub(super) gone: bool,
    /// Test module installed by this thread (tests only touch their own modules).
    #[cfg_attr(not(test), allow(dead_code))]
    owner: Option<std::thread::ThreadId>,
}

impl Module {
    /// Translate a flat `Image` byte offset `img_off` to its live `MAP_JIT` host address.
    #[inline]
    fn host_addr(&self, img_off: u32) -> usize {
        if self.bank_shifts.len() <= 1 {
            self.region + img_off as usize
        } else {
            let bi = self.img.bank_index_of(img_off);
            self.region + img_off as usize + self.bank_shifts[bi]
        }
    }

    /// Return (or emit and icache-flush) a 16-byte far-branch island in `site_off`'s bank
    /// jumping to `target_code`. Code memory must be writable on this thread.
    unsafe fn far_island(&mut self, site_off: u32, target_code: usize) -> Option<usize> {
        let bi = if self.bank_shifts.len() <= 1 {
            0
        } else {
            self.img.bank_index_of(site_off)
        };
        let slab = self.far_slabs.get_mut(bi)?;
        if let Some(&isl) = slab.islands.get(&target_code) {
            return Some(isl);
        }
        if slab.used + 16 > slab.len {
            return None;
        }
        let isl = slab.lo + slab.used;
        slab.used += 16;
        let p = isl as *mut u32;
        *p.add(0) = 0x5800_0050; // ldr x16, .+8
        *p.add(1) = 0xD61F_0200; // br x16 (preserves x30 for `bl`)
        core::ptr::write(p.add(2) as *mut u64, target_code as u64);
        icache_invalidate(isl as *mut u8, 16);
        slab.islands.insert(target_code, isl);
        Some(isl)
    }

    /// Translate a live `MAP_JIT` host PC back to a flat `Image` byte offset, or `None`
    /// if `pc` lies in an LSE2 patch slab or outside mapped banks.
    fn img_off_of_pc(&self, pc: usize) -> Option<u64> {
        if pc < self.region || pc >= self.region + self.region_len {
            return None;
        }
        let rel = pc - self.region;
        let banks = self.img.stub_banks();
        if self.bank_shifts.len() <= 1 {
            let end = AOT_STUBS_BYTES + self.img.code_bytes();
            return (rel < end).then_some(rel as u64);
        }
        for (bi, &bo) in banks.iter().enumerate() {
            let shift = self.bank_shifts[bi];
            let rt_lo = bo as usize + shift;
            let rt_hi = self.img.bank_end(bi) + shift;
            if rel >= rt_lo && rel < rt_hi {
                return Some((rel - shift) as u64);
            }
        }
        None
    }
}

pub(super) struct Dir {
    pub(super) modules: Vec<Module>,
    /// Live block IP -> (module, block).
    pub(super) map: FastMap<u64, (u32, u32)>,
    /// True if any block IP has ever overlapped across modules (guards sequential `probe_rebuild`).
    has_overlaps: bool,
    /// Shared probe table published in [`PROBE`] (kept in sync with `map` / `dead`).
    probe: ProbeTab,
    /// Earliest `(seq, va_gen)` captured at the start of an initial multi-module deferred batch
    /// when no prior live modules existed; published into `CHECKED_SEQ`/`CHECKED_VG` once the
    /// batch finishes linking and flushing icache.
    batch_init_gens: Option<(u64, u32)>,
}

// ---------------------------------------------------------------------------------------------
// Shared probe table (`AW2_DLL_AOT_PROBE`, read by the `lookup_*` stubs)
// ---------------------------------------------------------------------------------------------

/// Published probe table (0 = none): `[shift, mask, (ip, fast) * (mask + 1)]` (u64 words),
/// open addressing with linear probing from `(ip * JMUL) >> shift`; `ip == 0` ends a chain.
/// `fast` = code of the live, non-host-entry block owning `ip` in [`Dir::map`], else 0 (a
/// tombstone: entries are never removed, so chains never break). Writers hold the `DIR`
/// write lock; growth publishes a new table (old ones are leaked: stubs of running
/// translations may still read them until their next `JitCache::run`).
static PROBE: AtomicUsize = AtomicUsize::new(0);

/// The current [`PROBE`] table for [`crate::emu::jit::JitScratch::aot_dir`].
/// Falls back to the bound static AOT probe table when running without `MAP_JIT`.
#[inline]
pub(in crate::emu) fn probe_table() -> usize {
    let p = PROBE.load(Ordering::Acquire);
    if p != 0 {
        p
    } else {
        static_probe_table()
    }
}

const PROBE_MIN_BITS: u32 = 10;

/// Writer-side state of the published probe table.
#[derive(Default)]
struct ProbeTab {
    /// Table address (0 = none yet).
    base: usize,
    /// Entries (a power of two).
    cap: usize,
    /// Entries with `ip != 0` (live or tombstone); kept at most `cap / 2`.
    used: usize,
}

impl ProbeTab {
    /// Entry holding `ip`, or the empty entry ending its chain.
    unsafe fn slot(&self, ip: u64) -> &[AtomicU64; 2] {
        let ents = (self.base + 16) as *const [AtomicU64; 2];
        let mut i = (ip.wrapping_mul(jit::JMUL) >> (64 - self.cap.trailing_zeros())) as usize;
        loop {
            let e = &*ents.add(i);
            let k = e[0].load(Ordering::Relaxed);
            if k == ip || k == 0 {
                return e;
            }
            i = (i + 1) & (self.cap - 1);
        }
    }

    /// Read-only lookup of `ip` in a populated probe table (`0` if absent or dead/host-entry).
    #[inline(always)]
    unsafe fn lookup(&self, ip: u64) -> usize {
        if self.base == 0 || ip == 0 {
            return 0;
        }
        let ents = (self.base + 16) as *const [AtomicU64; 2];
        let shift = 64 - self.cap.trailing_zeros();
        let mask = self.cap - 1;
        let mut i = (ip.wrapping_mul(jit::JMUL) >> shift) as usize;
        loop {
            let e = &*ents.add(i);
            let k = e[0].load(Ordering::Relaxed);
            if k == ip {
                return e[1].load(Ordering::Relaxed) as usize;
            }
            if k == 0 {
                return 0;
            }
            i = (i + 1) & mask;
        }
    }

    /// Wait-free parallel insert of a unique non-zero `(ip, v)` into an unpublished table
    /// during [`Dir::probe_rebuild`]. Callers guarantee `ip != 0` and `used <= cap / 2`.
    #[inline(always)]
    unsafe fn insert_unique_parallel(&self, ip: u64, v: u64) {
        if ip == 0 {
            return;
        }
        let ents = (self.base + 16) as *const [AtomicU64; 2];
        let shift = 64 - self.cap.trailing_zeros();
        let mask = self.cap - 1;
        let mut i = (ip.wrapping_mul(jit::JMUL) >> shift) as usize;
        loop {
            let e = &*ents.add(i);
            let mut k = e[0].load(Ordering::Relaxed);
            if k == 0 {
                match e[0].compare_exchange(0, ip, Ordering::Relaxed, Ordering::Relaxed) {
                    Ok(_) => {
                        e[1].store(v, Ordering::Relaxed);
                        return;
                    }
                    Err(actual) => k = actual,
                }
            }
            if k == ip {
                e[1].store(v, Ordering::Relaxed);
                return;
            }
            i = (i + 1) & mask;
        }
    }
}

impl Dir {
    /// Probe-table value for `(mi, bi)` without re-looking up `self.map`.
    #[inline]
    fn probe_val_entry_opts(&self, mi: u32, bi: u32, include_staged: bool) -> u64 {
        let m = &self.modules[mi as usize];
        let b = &m.img.blocks[bi as usize];
        if m.gone
            || m.killed
            || (!include_staged && m.needs_icache_flush)
            || b.host_entry
            || m.dead[bi as usize].load(Ordering::Relaxed) != 0
        {
            return 0;
        }
        m.host_addr(b.code_off) as u64
    }

    #[inline]
    fn probe_val_entry(&self, mi: u32, bi: u32) -> u64 {
        self.probe_val_entry_opts(mi, bi, false)
    }

    /// Probe-table value for `ip`: the code of its live, non-host-entry owner, else 0.
    fn probe_val(&self, ip: u64) -> u64 {
        let Some(&(mi, bi)) = self.map.get(&ip) else { return 0 };
        self.probe_val_entry(mi, bi)
    }

    /// Bring the probe entry of `ip` in line with the map (after any change to its owner or
    /// the owner's `dead` flag). Existing entries are always updated (kills reach every
    /// reader); new ones only with [`jit::aot_probe_enabled`].
    pub(super) unsafe fn probe_sync(&mut self, ip: u64) {
        let v = self.probe_val(ip);
        if self.probe.base != 0 {
            let e = self.probe.slot(ip);
            if e[0].load(Ordering::Relaxed) == ip {
                e[1].store(v, Ordering::Release);
                return;
            }
            if v != 0 && jit::aot_probe_enabled() && 2 * (self.probe.used + 1) <= self.probe.cap {
                // `fast` before `ip`: a reader that sees the new `ip` with the old (0)
                // `fast` just misses.
                e[1].store(v, Ordering::Relaxed);
                e[0].store(ip, Ordering::Release);
                self.probe.used += 1;
                return;
            }
        }
        if v != 0 && jit::aot_probe_enabled() {
            self.probe_rebuild();
        }
    }

    /// Bulk-publish `(ip, code)` entries already installed in `self.map` (with `code == 0`
    /// clearing an overwritten prior entry). Triggers at most one `probe_rebuild()` instead of
    /// rebuilding mid-loop and re-probing millions of entries.
    unsafe fn probe_insert_batch(&mut self, entries: &[(u64, u64)]) {
        if entries.is_empty() {
            return;
        }
        let enabled = jit::aot_probe_enabled();
        if self.probe.base == 0 {
            if enabled && entries.iter().any(|&(_, v)| v != 0) {
                self.probe_rebuild();
            }
            return;
        }
        if enabled {
            let add_upper = entries.iter().filter(|&&(_, v)| v != 0).count();
            if 2 * (self.probe.used + add_upper) > self.probe.cap {
                self.probe_rebuild();
                return;
            }
        }
        for &(ip, v) in entries {
            let e = self.probe.slot(ip);
            if e[0].load(Ordering::Relaxed) == ip {
                e[1].store(v, Ordering::Release);
            } else if v != 0 && enabled {
                if 2 * (self.probe.used + 1) > self.probe.cap {
                    self.probe_rebuild();
                    return;
                }
                e[1].store(v, Ordering::Relaxed);
                e[0].store(ip, Ordering::Release);
                self.probe.used += 1;
            }
        }
    }

    /// Build a table sized for every live entry of the map and publish it.
    unsafe fn probe_rebuild(&mut self) {
        self.probe_rebuild_with_workers(aot_install_workers());
    }

    /// Build a fresh probe table covering every live non-host-entry block without publishing
    /// it to [`PROBE`] yet (lets batch startup link against the table before icache flush).
    /// When `include_staged` is false, modules with `needs_icache_flush == true` stay excluded.
    unsafe fn build_probe_table(&self, workers: usize, include_staged: bool) -> ProbeTab {
        let mut total_blocks = 0usize;
        let mut chunks: Vec<(usize, usize, usize)> = Vec::new();
        if !self.has_overlaps {
            const CHUNK: usize = 16_384;
            for (mi, m) in self.modules.iter().enumerate() {
                if m.gone || m.killed || (!include_staged && m.needs_icache_flush) {
                    continue;
                }
                let nb = m.img.blocks.len();
                total_blocks += nb;
                let mut lo = 0;
                while lo < nb {
                    let hi = (lo + CHUNK).min(nb);
                    chunks.push((mi, lo, hi));
                    lo = hi;
                }
            }
        }
        let use_parallel = !self.has_overlaps
            && workers > 1
            && (total_blocks >= PARALLEL_THRESHOLD || (cfg!(test) && total_blocks > 0));

        let live_count = if use_parallel {
            let n_workers = workers.min(chunks.len().max(1));
            let next = AtomicUsize::new(0);
            let total = AtomicUsize::new(0);
            let modules = &self.modules;
            let chunks_ref = &chunks;
            std::thread::scope(|s| {
                for _ in 0..n_workers {
                    let next = &next;
                    let total = &total;
                    s.spawn(move || {
                        let mut local = 0usize;
                        loop {
                            let ci = next.fetch_add(1, Ordering::Relaxed);
                            let Some(&(mi, lo, hi)) = chunks_ref.get(ci) else { break };
                            let m = &modules[mi];
                            for bi in lo..hi {
                                if !m.img.blocks[bi].host_entry
                                    && m.dead[bi].load(Ordering::Relaxed) == 0
                                {
                                    local += 1;
                                }
                            }
                        }
                        if local != 0 {
                            total.fetch_add(local, Ordering::Relaxed);
                        }
                    });
                }
            });
            total.load(Ordering::Relaxed)
        } else if !self.has_overlaps {
            let mut c = 0usize;
            for m in &self.modules {
                if m.gone || m.killed || (!include_staged && m.needs_icache_flush) {
                    continue;
                }
                for (bi, b) in m.img.blocks.iter().enumerate() {
                    if !b.host_entry && m.dead[bi].load(Ordering::Relaxed) == 0 {
                        c += 1;
                    }
                }
            }
            c
        } else {
            let mut c = 0usize;
            for &(mi, bi) in self.map.values() {
                if self.probe_val_entry_opts(mi, bi, include_staged) != 0 {
                    c += 1;
                }
            }
            c
        };

        let bits = (2 * live_count).max(2).next_power_of_two().trailing_zeros().max(PROBE_MIN_BITS);
        let cap = 1usize << bits;
        let p = Box::leak(vec![0u64; 2 + 2 * cap].into_boxed_slice()).as_mut_ptr();
        *p = 64 - bits as u64;
        *p.add(1) = cap as u64 - 1;
        let t = ProbeTab { base: p as usize, cap, used: live_count };
        if use_parallel {
            let n_workers = workers.min(chunks.len().max(1));
            let next = AtomicUsize::new(0);
            let modules = &self.modules;
            let chunks_ref = &chunks;
            let t_ref = &t;
            std::thread::scope(|s| {
                for _ in 0..n_workers {
                    let next = &next;
                    s.spawn(move || loop {
                        let ci = next.fetch_add(1, Ordering::Relaxed);
                        let Some(&(mi, lo, hi)) = chunks_ref.get(ci) else { break };
                        let m = &modules[mi];
                        for bi in lo..hi {
                            let b = &m.img.blocks[bi];
                            if !b.host_entry && m.dead[bi].load(Ordering::Relaxed) == 0 {
                                let v = m.host_addr(b.code_off) as u64;
                                t_ref.insert_unique_parallel(b.ip, v);
                            }
                        }
                    });
                }
            });
        } else if !self.has_overlaps {
            for m in &self.modules {
                if m.gone || m.killed || (!include_staged && m.needs_icache_flush) {
                    continue;
                }
                for (bi, b) in m.img.blocks.iter().enumerate() {
                    if !b.host_entry && m.dead[bi].load(Ordering::Relaxed) == 0 {
                        let v = m.host_addr(b.code_off) as u64;
                        let e = t.slot(b.ip);
                        e[1].store(v, Ordering::Relaxed);
                        e[0].store(b.ip, Ordering::Relaxed);
                    }
                }
            }
        } else {
            for (&ip, &(mi, bi)) in &self.map {
                let v = self.probe_val_entry_opts(mi, bi, include_staged);
                if v != 0 {
                    let e = t.slot(ip);
                    e[1].store(v, Ordering::Relaxed);
                    e[0].store(ip, Ordering::Relaxed);
                }
            }
        }
        t
    }

    /// [`Self::probe_rebuild`] parameterized by worker count (also called directly by tests).
    unsafe fn probe_rebuild_with_workers(&mut self, workers: usize) {
        let t = self.build_probe_table(workers, false);
        PROBE.store(t.base, Ordering::Release);
        self.probe = t;
    }

    /// Test helper: whether any live module currently holds a patched link targeting `ip`.
    #[cfg(test)]
    pub(super) fn has_incoming_links(&self, ip: u64) -> bool {
        self.modules.iter().any(|m| {
            !m.gone
                && !m.killed
                && m.img
                    .links
                    .iter()
                    .enumerate()
                    .any(|(li, l)| m.linked[li] && l.target == ip)
        })
    }
}

/// [`Module::dead`]: verify mismatch, stale import slot or module killed (never revived).
const DEAD_PERM: u8 = 1;
/// [`Module::dead`]: host-callable assumption / relocation check failed; re-checked on every
/// `va_gen` change and revived when it holds again ([`revalidate_dir`]).
const DEAD_ASSUME: u8 = 2;
/// [`Module::dead`]: a [`DEAD_ASSUME`] block whose checks pass again while revival is off
/// (`AW2_DLL_AOT_REVIVE=0`; counted once in [`AotCounts::would_revive`]); revived like
/// [`DEAD_ASSUME`] blocks if revival is on at a later pass.
const DEAD_HELD: u8 = 3;

/// `PORT_AOT_REVIVE` / `AW2_DLL_AOT_REVIVE` (default 1): revive [`DEAD_ASSUME`] blocks whose
/// checks pass again (0: only count them, see [`AotCounts::would_revive`]).
pub(in crate::emu) fn revive_enabled() -> bool {
    #[cfg(test)]
    if TEST_NO_REVIVE.with(|c| c.get()) {
        return false;
    }
    static V: OnceLock<bool> = OnceLock::new();
    *V.get_or_init(|| env_u32_any(&["PORT_AOT_REVIVE", "AW2_DLL_AOT_REVIVE"], 1) != 0)
}

/// Process-wide AOT invalidation counters (`AW2_JIT_STATS` report, [`counts`]).
pub(in crate::emu) struct AotCounts {
    pub kills: u64,
    pub revived: u64,
    pub would_revive: u64,
}

static KILLS: AtomicU64 = AtomicU64::new(0);
static REVIVED: AtomicU64 = AtomicU64::new(0);
static WOULD_REVIVE: AtomicU64 = AtomicU64::new(0);

pub(in crate::emu) fn counts() -> AotCounts {
    AotCounts {
        kills: KILLS.load(Ordering::Relaxed),
        revived: REVIVED.load(Ordering::Relaxed),
        would_revive: WOULD_REVIVE.load(Ordering::Relaxed),
    }
}

/// AOT lookups can hit (a dynamic or static AOT cache is installed).
#[inline]
pub(in crate::emu) fn active() -> bool {
    (HAVE.load(Ordering::Relaxed) && aot_mode() != 0) || has_bound_static_aot()
}

pub(super) static DIR: RwLock<Option<Dir>> = RwLock::new(None);
/// Serialises revalidation and module installation (lock order: REVALIDATE, then DIR).
pub(super) static REVALIDATE: Mutex<()> = Mutex::new(());
/// `va_gen` every live block was last checked against.
static CHECKED_VG: AtomicU32 = AtomicU32::new(0);
/// Monotonic `exe::va_seq` corresponding to [`CHECKED_VG`] for delta revalidation.
static CHECKED_SEQ: AtomicU64 = AtomicU64::new(0);
/// Bumped when a module is killed (per-thread caches may hold links into it).
static EPOCH: AtomicU32 = AtomicU32::new(0);
/// Any module installed.
static HAVE: AtomicBool = AtomicBool::new(false);
/// Test-only: force `link_pass` to route direct links through per-bank far-branch islands.
#[cfg(test)]
pub(super) static TEST_FORCE_FAR_LINK: AtomicBool = AtomicBool::new(false);

/// AOT module kills so far ([`crate::emu::jit::JitCache::run`] flushes when it changes).
#[inline(always)]
pub(in crate::emu) fn epoch() -> u32 {
    EPOCH.load(Ordering::Relaxed)
}

/// Code memory of all AOT regions writable on this thread while alive.
pub(super) struct JitWrite;

impl JitWrite {
    pub(super) unsafe fn new() -> Self {
        jit_write_protect(false);
        JitWrite
    }
}

impl Drop for JitWrite {
    fn drop(&mut self) {
        unsafe { jit_write_protect(true) };
    }
}

/// Atomically re-point the `b`/`bl` (or fallthrough `nop`) at `site` to `target` (link bit kept).
pub(super) unsafe fn store_b(site: usize, target: usize) {
    let cell = &*(site as *const AtomicU32);
    let old = cell.load(Ordering::Relaxed);
    let link_bit = if old == 0xD503_201F { 0 } else { old & 0x8000_0000 };
    let off = (((target as i64 - site as i64) >> 2) as u32) & 0x03FF_FFFF;
    cell.store(0x1400_0000 | link_bit | off, Ordering::Release);
}

/// Patch a `b.cond` / `cbz` / `cbnz` at `cond_site` directly to `target` if within ±1 MiB.
pub(super) unsafe fn try_patch_cond_branch(cond_site: usize, target: usize) -> Option<usize> {
    static NO_COND: OnceLock<bool> = OnceLock::new();
    if *NO_COND.get_or_init(|| {
        std::env::var("PORT_AOT_NO_COND_PATCH").is_ok_and(|v| v.trim() != "0" && !v.trim().is_empty())
    }) {
        return None;
    }
    let off = (target as i64 - cond_site as i64) >> 2;
    if !(-(1i64 << 18)..(1i64 << 18)).contains(&off) {
        return None;
    }
    let cell = &*(cond_site as *const AtomicU32);
    let old = cell.load(Ordering::Relaxed);
    debug_assert!(
        (old & 0xFF00_0010) == 0x5400_0000 || (old & 0x7E00_0000) == 0x3400_0000,
        "unexpected cond_off insn {old:#010x}"
    );
    let new_w = (old & 0xFF00_001F) | (((off as u32) & 0x7_FFFF) << 5);
    cell.store(new_w, Ordering::Relaxed);
    Some(cond_site)
}

pub(super) fn reach(site: usize, target: usize) -> bool {
    let d = target as i64 - site as i64;
    (-(128i64 << 20)..(128i64 << 20)).contains(&d)
}

/// Host-callable (or emulator-routed DLL helper/stub) now, by the same rule as `jit::resolved_host`.
fn host_now(va: u64) -> bool {
    crate::emu::resolve_host_callable(va)
        .filter(|&h| !crate::emu::thunk_elided(va, h))
        .is_some()
        || crate::emu::is_routed_dll_chkstk(va)
        || crate::emu::is_routed_dll_stack_pos(va)
        || crate::emu::is_routed_regfile(va)
        || crate::emu::is_routed_dll_stub(va)
}

/// Shared code for guest `ip`: `(code, host_entry)`, or None (JIT it). Called from
/// [`crate::emu::jit::JitCache`] on an index miss; the caller installs
/// `JEnt { ip, fast: if host_entry { 0 } else { code }, gen: 0, code }`.
pub(in crate::emu) fn find(ip: u64) -> Option<(usize, bool)> {
    if aot_mode() != 0 && HAVE.load(Ordering::Acquire) {
        let vg = crate::exe::va_gen();
        if CHECKED_VG.load(Ordering::Acquire) != vg {
            revalidate(vg);
        }
        if let Ok(g) = DIR.read() {
            if let Some(d) = g.as_ref() {
                if let Some(&(mi, bi)) = d.map.get(&ip) {
                    let m = &d.modules[mi as usize];
                    if !m.needs_icache_flush
                        && m.dead[bi as usize].load(Ordering::Acquire) == 0
                        && !crate::emu::mem::in_jit(crate::emu::to_mapped_addr(ip))
                    {
                        let b = &m.img.blocks[bi as usize];
                        return Some((m.host_addr(b.code_off), b.host_entry));
                    }
                }
            }
        }
    }
    lookup_static_aot(ip)
}

/// Immutable snapshot of process-wide dispatch/routing state built once per `check_install_blocks`
/// or `revalidate_dir` pass so workers never contend on `vm_lock()` (`is_speculative_read_safe`)
/// or `IAT_SPANS` seqlocks across millions of plain guest-to-guest assumptions.
#[derive(Clone, Copy)]
struct HostCheckCtx {
    exe_base: usize,
    exe_end: usize,
    exe_pref: u64,
    exe_size: u64,
    aw2_active: bool,
    exe_all_readable: bool,
    iat_lo: u64,
    iat_span: u64,
}

/// Per-module routing pre-filter derived once from [`HostCheckCtx`].
#[derive(Clone, Copy)]
struct ModuleRouteGate {
    m_lo: u64,
    m_hi: u64,
    overlaps_iat_gate: bool,
    exact_iat: bool,
    overlaps_exe: bool,
}

impl HostCheckCtx {
    fn snapshot() -> Self {
        let exe_base = crate::exe::base();
        let exe_size = crate::exe::image_size();
        let exe_pref = crate::exe::preferred_base();
        let aw2_active = crate::exe::aw2_registry_active();
        let exe_end = exe_base.saturating_add(exe_size as usize);
        // Check once whether every page of the mapped primary executable is committed and
        // readable (zero `PAGE_NOACCESS` / `PAGE_GUARD` holes). In unit tests, `exe::base()`
        // defaults to an unmapped synthetic VA (`0x1_4000_0000`), so leave `false` there.
        let exe_all_readable = !cfg!(test)
            && exe_base != 0
            && exe_size != 0
            && exe_base.checked_add(exe_size as usize).is_some()
            && win32_rt::kernel32::memory::is_speculative_read_safe(exe_base, exe_size as usize);
        let (iat_lo, iat_span) = crate::emu::routes::dll_route_gate();
        Self {
            exe_base,
            exe_end,
            exe_pref,
            exe_size,
            aw2_active,
            exe_all_readable,
            iat_lo,
            iat_span,
        }
    }

    fn module_gate(&self, m: &Module) -> ModuleRouteGate {
        let m_lo = m.base as u64;
        let m_hi = m.base.saturating_add(m.size) as u64;
        let overlaps_iat_gate = self.iat_span != 0
            && m_lo < self.iat_lo.saturating_add(self.iat_span)
            && m_hi > self.iat_lo;
        let exact_iat = overlaps_iat_gate
            && crate::dll_native::iat_image_bounds(m_lo)
                == Some((m.base, m.base.wrapping_add(m.size)));
        let overlaps_exe = self.exe_base != 0
            && ((m_lo < self.exe_end as u64 && m_hi > self.exe_base as u64)
                || (self.exe_pref != 0
                    && m_lo < self.exe_pref.saturating_add(self.exe_size)
                    && m_hi > self.exe_pref));
        ModuleRouteGate { m_lo, m_hi, overlaps_iat_gate, exact_iat, overlaps_exe }
    }

    /// True if `va` inside `[mg.m_lo, mg.m_hi)` could flip `host_now(va)` via an *external*
    /// `VA_DISPATCH` change (a primary-EXE `FF 25` delay-load/IAT thunk or a DLL IAT stub).
    /// Plain guest functions inside `m` can only flip via an in-module `VA_DISPATCH` change
    /// (which triggers `module_affected_by_va` -> full scan) or a structural `va_gen` bump.
    #[inline]
    unsafe fn may_flip_on_external_va(&self, va: u64, mg: &ModuleRouteGate) -> bool {
        if mg.overlaps_exe {
            let eb = self.exe_base as u64;
            let in_pref = self.exe_pref != 0
                && va >= self.exe_pref
                && va - self.exe_pref < self.exe_size;
            let in_base = va >= eb && va - eb < self.exe_size;
            if in_pref || in_base {
                let mapped = if eb != self.exe_pref && in_pref {
                    eb.wrapping_add(va - self.exe_pref) as usize
                } else {
                    va as usize
                };
                if mapped >= self.exe_base && mapped.saturating_add(6) <= self.exe_end {
                    if !self.exe_all_readable
                        || core::ptr::read_unaligned(mapped as *const u16) == 0x25ff
                    {
                        return true;
                    }
                }
            }
        }
        if mg.overlaps_iat_gate {
            if mg.exact_iat && va.saturating_add(2) <= mg.m_hi {
                let w = core::ptr::read_unaligned(va as usize as *const u16);
                if (w & 0xff) == 0xe9 || w == 0x25ff || w == 0xff48 {
                    return true;
                }
            } else {
                return true;
            }
        }
        false
    }

    /// True when `va` (which must lie inside `[mg.m_lo, mg.m_hi)`) is guaranteed to have
    /// `host_now(va) == false` AND is not a primary-EXE `FF 25` thunk or DLL IAT stub.
    ///
    /// Returning `false` is always conservative and falls back to `hm` + exact `host_now(va)`.
    #[inline]
    unsafe fn is_plain_guest_in_module(&self, va: u64, mg: &ModuleRouteGate) -> bool {
        if crate::exe::va_bloom_may_contain(va) {
            return false;
        }
        let eb = self.exe_base as u64;
        if self.aw2_active {
            if eb != crate::exe::PREFERRED_BASE && va >= eb {
                let pref = crate::exe::PREFERRED_BASE.wrapping_add(va - eb);
                if crate::exe::va_bloom_may_contain(pref) {
                    return false;
                }
            }
        } else if eb != self.exe_pref && va >= eb && va - eb < self.exe_size {
            let pref = self.exe_pref.wrapping_add(va - eb);
            if crate::exe::va_bloom_may_contain(pref) {
                return false;
            }
        }
        if self.may_flip_on_external_va(va, mg) {
            return false;
        }
        if crate::emu::is_routed_dll_chkstk(va)
            || crate::emu::is_routed_dll_stack_pos(va)
            || crate::emu::is_routed_regfile(va)
        {
            return false;
        }
        true
    }

    /// True if a `VA_DISPATCH` change at `va` could affect any direct non-watch assumption inside
    /// module `m` (either at `m`'s mapped span or via the primary-EXE preferred-base alias).
    #[inline]
    fn module_affected_by_va(&self, m: &Module, va: u64) -> bool {
        let m_lo = m.base as u64;
        let m_sz = m.size as u64;
        if va >= m_lo && va - m_lo < m_sz {
            return true;
        }
        let eb = self.exe_base as u64;
        if self.exe_base != 0
            && m.base == self.exe_base
            && self.exe_pref != 0
            && va >= self.exe_pref
            && va - self.exe_pref < self.exe_size
        {
            return true;
        }
        if self.aw2_active && eb != crate::exe::PREFERRED_BASE && va >= crate::exe::PREFERRED_BASE {
            let mapped = eb.wrapping_add(va - crate::exe::PREFERRED_BASE);
            if mapped >= m_lo && mapped - m_lo < m_sz {
                return true;
            }
        }
        false
    }
}

/// Re-check a block's assumptions and relocations against `ctx`/`mg` (memoising only the small
/// non-plain subset in `hm`/`fm`). Returns `(Result, needs_watch)`, where `needs_watch` is true
/// if this block must remain in `m.watch_blocks` for out-of-module `VA_DISPATCH` delta passes.
unsafe fn check_block(
    m: &Module,
    b: &BlockEnt,
    ctx: &HostCheckCtx,
    mg: &ModuleRouteGate,
    hm: &mut FastMap<u64, bool>,
    fm: &mut FastMap<u64, Option<(usize, bool)>>,
    rewrites: &mut Vec<(usize, u64)>,
) -> (Result<(), &'static str>, bool) {
    if crate::emu::is_routed_dll_chkstk(b.ip)
        || crate::emu::is_routed_dll_stack_pos(b.ip)
        || crate::emu::is_routed_regfile(b.ip)
    {
        return (Err("routed dll helper"), true);
    }
    let b_in_mod = b.ip >= mg.m_lo && b.ip < mg.m_hi;
    let b_is_stub = if b_in_mod {
        if !mg.overlaps_iat_gate {
            false
        } else if mg.exact_iat && b.ip.saturating_add(2) <= mg.m_hi {
            let w = core::ptr::read_unaligned(b.ip as usize as *const u16);
            ((w & 0xff) == 0xe9 || w == 0x25ff || w == 0xff48)
                && crate::emu::is_routed_dll_stub(b.ip)
        } else {
            crate::emu::is_routed_dll_stub(b.ip)
        }
    } else {
        crate::emu::is_routed_dll_stub(b.ip)
    };
    if b_is_stub {
        return (Err("routed dll helper"), true);
    }
    let mut needs_watch = b.reloc_n != 0;
    for &(va, host) in m.img.assume_of(b) {
        let in_mod = va >= mg.m_lo && va < mg.m_hi;
        if in_mod && ctx.is_plain_guest_in_module(va, mg) {
            if host {
                return (Err("host-callable assumption"), true);
            }
            continue;
        }
        if *hm.entry(va).or_insert_with(|| host_now(va)) != host {
            return (Err("host-callable assumption"), true);
        }
        if host || !in_mod || ctx.may_flip_on_external_va(va, mg) {
            needs_watch = true;
        }
    }
    for r in m.img.relocs_of(b) {
        if r.va == SLOT_DISABLED {
            // Held import slot whose guard never matches (rebase_block): nothing to check.
            continue;
        }
        if r.kind == RELOC_SLOT_VAL {
            // The value itself (rebased at load); its HOST_FN sibling checks callability.
            let slot = m.host_addr(r.slot_off);
            if (*(slot as *const AtomicU64)).load(Ordering::Relaxed) != r.va {
                rewrites.push((slot, r.va));
            }
            continue;
        }
        if r.kind == RELOC_QPC_STATE {
            let h = win32_rt::kernel32::time::qpc_fast_state_ptr() as u64;
            let slot = m.host_addr(r.slot_off);
            if (*(slot as *const AtomicU64)).load(Ordering::Relaxed) != h {
                rewrites.push((slot, h));
            }
            continue;
        }
        if r.kind == RELOC_XINPUT_STATE {
            let h = win32_rt::misc::xinput1_4::xinput_fast_table_ptr() as u64;
            let slot = m.host_addr(r.slot_off);
            if (*(slot as *const AtomicU64)).load(Ordering::Relaxed) != h {
                rewrites.push((slot, h));
            }
            continue;
        }
        match *fm.entry(r.va).or_insert_with(|| jit::inline_host_fn(r.va)) {
            Some((h, bail)) if bail == r.bail => {
                let slot = m.host_addr(r.slot_off);
                if (*(slot as *const AtomicU64)).load(Ordering::Relaxed) != h as u64 {
                    rewrites.push((slot, h as u64));
                }
            }
            _ => return (Err("inline host fn"), true),
        }
    }
    (Ok(()), needs_watch)
}

/// Parallel or serial block assumption/relocation check for module installation (`install_image`).
/// Writes `DEAD_ASSUME` into disjoint `m.dead[bi]` slots, collects `(slot, value)` rewrites for
/// the caller (which holds `JitWrite`) to apply, and returns the sorted `watch_blocks` list.
unsafe fn check_install_blocks(
    m: &Module,
    workers: usize,
) -> (usize, Vec<(usize, u64)>, Vec<u32>) {
    let n = m.img.blocks.len();
    let ctx = HostCheckCtx::snapshot();
    let mg = ctx.module_gate(m);
    if workers <= 1 || n < 2 {
        let (mut hm, mut fm, mut rewrites, mut watch) =
            (FastMap::default(), FastMap::default(), Vec::new(), Vec::new());
        let mut bad = 0usize;
        for bi in 0..n {
            let st = m.dead[bi].load(Ordering::Relaxed);
            if st != 0 {
                if st == DEAD_ASSUME || st == DEAD_HELD {
                    watch.push(bi as u32);
                }
                continue;
            }
            let b = &m.img.blocks[bi];
            let (res, nw) = check_block(m, b, &ctx, &mg, &mut hm, &mut fm, &mut rewrites);
            if res.is_err() {
                m.dead[bi].store(DEAD_ASSUME, Ordering::Relaxed);
                bad += 1;
                watch.push(bi as u32);
            } else if nw {
                watch.push(bi as u32);
            }
        }
        return (bad, rewrites, watch);
    }
    let chunk = n.div_ceil(workers);
    let results: Vec<(usize, Vec<(usize, u64)>, Vec<u32>)> = std::thread::scope(|s| {
        let mut handles = Vec::with_capacity(workers);
        let mut lo = 0usize;
        while lo < n {
            let hi = (lo + chunk).min(n);
            handles.push(s.spawn(move || {
                let mut hm = FastMap::default();
                let mut fm = FastMap::default();
                let mut rewrites = Vec::new();
                let mut watch = Vec::new();
                let mut bad = 0usize;
                for bi in lo..hi {
                    let st = m.dead[bi].load(Ordering::Relaxed);
                    if st != 0 {
                        if st == DEAD_ASSUME || st == DEAD_HELD {
                            watch.push(bi as u32);
                        }
                        continue;
                    }
                    let b = &m.img.blocks[bi];
                    let (res, nw) = check_block(m, b, &ctx, &mg, &mut hm, &mut fm, &mut rewrites);
                    if res.is_err() {
                        m.dead[bi].store(DEAD_ASSUME, Ordering::Relaxed);
                        bad += 1;
                        watch.push(bi as u32);
                    } else if nw {
                        watch.push(bi as u32);
                    }
                }
                (bad, rewrites, watch)
            }));
            lo = hi;
        }
        handles
            .into_iter()
            .map(|h| h.join().expect("aot check_block worker panicked"))
            .collect()
    });
    let mut bad = 0usize;
    let total_rw: usize = results.iter().map(|(_, r, _)| r.len()).sum();
    let total_watch: usize = results.iter().map(|(_, _, w)| w.len()).sum();
    let mut rewrites = Vec::with_capacity(total_rw);
    let mut watch = Vec::with_capacity(total_watch);
    for (b, rw, w) in results {
        bad += b;
        rewrites.extend(rw);
        watch.extend(w);
    }
    (bad, rewrites, watch)
}

/// Mark a batch of `(module, block)` entries dead (`why`: [`DEAD_PERM`] / [`DEAD_ASSUME`])
/// and unpatch every currently-linked AOT site targeting any of their IPs (code must be
/// writable on this thread). Avoids maintaining 85+ MiB of per-block/per-link incoming chains
/// on the hot install path by scanning module link tables in parallel on this rare cold path.
unsafe fn kill_batch(
    d: &mut Dir,
    kills: &[(usize, usize)],
    why: u8,
    patched: &mut Vec<usize>,
) {
    if kills.is_empty() {
        return;
    }
    let mut min_ip = u64::MAX;
    let mut max_ip = 0u64;
    let mut killed_ips: FastSet<u64> =
        FastSet::with_capacity_and_hasher(kills.len(), Default::default());
    for &(mi, bi) in kills {
        let ip = {
            let m = &d.modules[mi];
            m.dead[bi].store(why, Ordering::Release);
            m.img.blocks[bi].ip
        };
        d.probe_sync(ip);
        min_ip = min_ip.min(ip);
        max_ip = max_ip.max(ip);
        killed_ips.insert(ip);
    }
    let single_ip = (killed_ips.len() == 1).then_some(min_ip);
    let workers = if cfg!(test) { 1 } else { aot_install_workers() };
    for sm in 0..d.modules.len() {
        if d.modules[sm].gone {
            continue;
        }
        let nlinks = d.modules[sm].img.links.len();
        if nlinks == 0 {
            continue;
        }
        let hits: Vec<u32> = if workers > 1 && nlinks >= PARALLEL_THRESHOLD {
            let m = &d.modules[sm];
            let links = &m.img.links;
            let linked = &m.linked;
            let killed_ref = &killed_ips;
            let chunk = nlinks.div_ceil(workers);
            std::thread::scope(|s| {
                let mut handles = Vec::with_capacity(workers);
                let mut lo = 0usize;
                while lo < nlinks {
                    let hi = (lo + chunk).min(nlinks);
                    handles.push(s.spawn(move || {
                        let mut out = Vec::new();
                        if let Some(only) = single_ip {
                            for sl in lo..hi {
                                if linked[sl] && links[sl].target == only {
                                    out.push(sl as u32);
                                }
                            }
                        } else {
                            for sl in lo..hi {
                                let tgt = links[sl].target;
                                if linked[sl]
                                    && tgt >= min_ip
                                    && tgt <= max_ip
                                    && killed_ref.contains(&tgt)
                                {
                                    out.push(sl as u32);
                                }
                            }
                        }
                        out
                    }));
                    lo = hi;
                }
                let mut merged = Vec::new();
                for h in handles {
                    merged.extend(h.join().expect("aot kill_batch scan worker panicked"));
                }
                merged
            })
        } else {
            let m = &d.modules[sm];
            let mut out = Vec::new();
            if let Some(only) = single_ip {
                for (sl, l) in m.img.links.iter().enumerate() {
                    if m.linked[sl] && l.target == only {
                        out.push(sl as u32);
                    }
                }
            } else {
                for (sl, l) in m.img.links.iter().enumerate() {
                    if m.linked[sl]
                        && l.target >= min_ip
                        && l.target <= max_ip
                        && killed_ips.contains(&l.target)
                    {
                        out.push(sl as u32);
                    }
                }
            }
            out
        };
        if hits.is_empty() {
            continue;
        }
        let m = &mut d.modules[sm];
        for sl_u32 in hits {
            let sl = sl_u32 as usize;
            if !m.linked[sl] {
                continue;
            }
            let l = m.img.links[sl];
            let site = m.host_addr(l.site_off);
            if l.cond_off != u32::MAX {
                let cond_site = m.host_addr(l.cond_off);
                if let Some(cs) = try_patch_cond_branch(cond_site, site) {
                    patched.push(cs);
                }
            }
            store_b(site, m.host_addr(l.fallback_off));
            m.linked[sl] = false;
            if m.unlinked_init {
                m.unlinked.push(sl_u32);
            }
            patched.push(site);
        }
    }
}

/// Attempt to patch link `li` of module `mi` in-place. Returns `true` if patched.
#[inline]
unsafe fn link_one(
    d: &mut Dir,
    mi: usize,
    li: usize,
    force_far: bool,
    probe: Option<&ProbeTab>,
    allow_staged: bool,
    skip_icache: &[(usize, usize)],
    patched: &mut Vec<usize>,
) -> bool {
    let l = d.modules[mi].img.links[li];
    let code = if let Some(pt) = probe {
        let c = pt.lookup(l.target);
        if c == 0 {
            return false;
        }
        c
    } else {
        let Some(&(tm, tb)) = d.map.get(&l.target) else { return false };
        let t = &d.modules[tm as usize];
        let tb_ = &t.img.blocks[tb as usize];
        if t.gone
            || t.killed
            || (!allow_staged && t.needs_icache_flush)
            || t.dead[tb as usize].load(Ordering::Relaxed) != 0
            || tb_.host_entry
        {
            return false;
        }
        t.host_addr(tb_.code_off)
    };
    let (site, cond_site) = {
        let m = &d.modules[mi];
        let s = m.host_addr(l.site_off);
        let c = if l.cond_off == u32::MAX { 0 } else { m.host_addr(l.cond_off) };
        (s, c)
    };
    let dest = if !force_far && reach(site, code) {
        code
    } else {
        let Some(isl) = d.modules[mi].far_island(l.site_off, code) else {
            return false;
        };
        debug_assert!(reach(site, isl), "far-branch island out of b reach");
        isl
    };
    let cell = &*(site as *const AtomicU32);
    let old = cell.load(Ordering::Relaxed);
    if dest == site + 4 && (old & 0xFC00_0000) == 0x1400_0000 {
        // Fallthrough directly into the immediately-following hot block: 0-cycle `nop`
        // avoids taking a branch and keeps Apple Silicon's 8-wide fetch group intact!
        cell.store(0xD503_201F, Ordering::Release);
    } else {
        store_b(site, dest);
    }
    let record = |addr: usize, out: &mut Vec<usize>| {
        if !skip_icache.iter().any(|&(lo, hi)| (lo..hi).contains(&addr)) {
            out.push(addr);
        }
    };
    if cond_site != 0 {
        if let Some(cs) = try_patch_cond_branch(cond_site, dest) {
            record(cs, patched);
        }
    }
    d.modules[mi].linked[li] = true;
    record(site, patched);
    true
}

/// Parallel initial link pass for a newly installed module `mi` (`!unlinked_init`, `!force_far`):
/// resolves targets via a single read-only `ProbeTab::lookup` when available (or `d.map`),
/// patches in-range `site`/`cond_site` words in disjoint link chunks (per-thread `JitWrite`),
/// and finishes rare out-of-range (`far_island`) sites serially.
unsafe fn link_module_initial_parallel(
    d: &mut Dir,
    mi: usize,
    workers: usize,
    probe: Option<&ProbeTab>,
    allow_staged: bool,
    skip_icache: &[(usize, usize)],
    patched: &mut Vec<usize>,
) {
    let nlinks = d.modules[mi].img.links.len();
    let mut linked = core::mem::take(&mut d.modules[mi].linked);
    let chunk = nlinks.div_ceil(workers);
    let worker_out: Vec<(Vec<u32>, Vec<u32>, Vec<usize>)> = {
        let d_ref: &Dir = d;
        std::thread::scope(|s| {
            let mut handles = Vec::with_capacity(workers);
            for (ci, l_chunk) in linked.chunks_mut(chunk).enumerate() {
                let base_li = ci * chunk;
                handles.push(s.spawn(move || {
                    let _jw = JitWrite::new();
                    let m = &d_ref.modules[mi];
                    let links = &m.img.links;
                    let mut local_unlinked = Vec::new();
                    let mut local_far = Vec::new();
                    let mut local_patched = Vec::new();
                    for offset in 0..l_chunk.len() {
                        if l_chunk[offset] {
                            continue;
                        }
                        let li = base_li + offset;
                        let l = links[li];
                        let code = if let Some(pt) = probe {
                            let c = pt.lookup(l.target);
                            if c == 0 {
                                local_unlinked.push(li as u32);
                                continue;
                            }
                            c
                        } else {
                            let Some(&(tm, tb)) = d_ref.map.get(&l.target) else {
                                local_unlinked.push(li as u32);
                                continue;
                            };
                            let t = &d_ref.modules[tm as usize];
                            let tb_ = &t.img.blocks[tb as usize];
                            if t.gone
                                || t.killed
                                || (!allow_staged && t.needs_icache_flush)
                                || t.dead[tb as usize].load(Ordering::Relaxed) != 0
                                || tb_.host_entry
                            {
                                local_unlinked.push(li as u32);
                                continue;
                            }
                            t.host_addr(tb_.code_off)
                        };
                        let site = m.host_addr(l.site_off);
                        if !reach(site, code) {
                            local_far.push(li as u32);
                            continue;
                        }
                        let cell = &*(site as *const AtomicU32);
                        let old = cell.load(Ordering::Relaxed);
                        if code == site + 4 && (old & 0xFC00_0000) == 0x1400_0000 {
                            cell.store(0xD503_201F, Ordering::Release);
                        } else {
                            store_b(site, code);
                        }
                        if l.cond_off != u32::MAX {
                            let cond_site = m.host_addr(l.cond_off);
                            if let Some(cs) = try_patch_cond_branch(cond_site, code) {
                                if !skip_icache.iter().any(|&(slo, shi)| (slo..shi).contains(&cs)) {
                                    local_patched.push(cs);
                                }
                            }
                        }
                        l_chunk[offset] = true;
                        if !skip_icache.iter().any(|&(slo, shi)| (slo..shi).contains(&site)) {
                            local_patched.push(site);
                        }
                    }
                    (local_unlinked, local_far, local_patched)
                }));
            }
            handles
                .into_iter()
                .map(|h| h.join().expect("aot link worker panicked"))
                .collect()
        })
    };
    d.modules[mi].linked = linked;
    let total_unlinked: usize = worker_out.iter().map(|(u, _, _)| u.len()).sum();
    let mut unlinked = Vec::with_capacity(total_unlinked);
    let mut far_sites = Vec::new();
    for (u, f, p) in worker_out {
        unlinked.extend(u);
        far_sites.extend(f);
        patched.extend(p);
    }
    for li in far_sites {
        if !link_one(d, mi, li as usize, false, probe, allow_staged, skip_icache, patched) {
            unlinked.push(li);
        }
    }
    d.modules[mi].unlinked = unlinked;
    d.modules[mi].unlinked_init = true;
}

/// Patch every unlinked AOT link site whose target is a live, linkable block in range
/// (code must be writable on this thread). Returns patched sites outside `skip_icache`
/// (callers that bulk-invalidate newly installed regions pass those `(region, region + len)`
/// ranges). When `probe` is `Some`, target resolution uses a single lock-free `ProbeTab::lookup`.
unsafe fn link_pass_with_probe(
    d: &mut Dir,
    probe: Option<&ProbeTab>,
    skip_icache: &[(usize, usize)],
    allow_staged: bool,
) -> Vec<usize> {
    static NO_LINK: OnceLock<bool> = OnceLock::new();
    if *NO_LINK.get_or_init(|| {
        std::env::var("PORT_AOT_NO_LINK").is_ok_and(|v| v.trim() != "0" && !v.trim().is_empty())
    }) {
        return Vec::new();
    }
    let force_far = {
        #[cfg(test)]
        {
            TEST_FORCE_FAR_LINK.load(Ordering::Relaxed)
        }
        #[cfg(not(test))]
        {
            false
        }
    };
    let workers = aot_install_workers();
    let mut patched = Vec::new();
    for mi in 0..d.modules.len() {
        if d.modules[mi].gone
            || d.modules[mi].killed
            || (!allow_staged && d.modules[mi].needs_icache_flush)
        {
            continue;
        }
        if !d.modules[mi].unlinked_init {
            let nlinks = d.modules[mi].img.links.len();
            if !force_far && workers > 1 && nlinks >= PARALLEL_THRESHOLD {
                link_module_initial_parallel(
                    d,
                    mi,
                    workers,
                    probe,
                    allow_staged,
                    skip_icache,
                    &mut patched,
                );
            } else {
                let mut unlinked = Vec::new();
                for li in 0..nlinks {
                    if !d.modules[mi].linked[li]
                        && !link_one(
                            d,
                            mi,
                            li,
                            force_far,
                            probe,
                            allow_staged,
                            skip_icache,
                            &mut patched,
                        )
                    {
                        unlinked.push(li as u32);
                    }
                }
                d.modules[mi].unlinked = unlinked;
                d.modules[mi].unlinked_init = true;
            }
        } else {
            let mut unlinked = core::mem::take(&mut d.modules[mi].unlinked);
            let mut keep = 0usize;
            for i in 0..unlinked.len() {
                let li = unlinked[i] as usize;
                if d.modules[mi].linked[li]
                    || link_one(
                        d,
                        mi,
                        li,
                        force_far,
                        probe,
                        allow_staged,
                        skip_icache,
                        &mut patched,
                    )
                {
                    continue;
                }
                unlinked[keep] = li as u32;
                keep += 1;
            }
            unlinked.truncate(keep);
            d.modules[mi].unlinked = unlinked;
        }
    }
    patched
}

unsafe fn link_pass(d: &mut Dir, skip_icache: Option<(usize, usize)>) -> Vec<usize> {
    let slice = skip_icache.as_slice();
    link_pass_with_probe(d, None, slice, false)
}

/// After a `va_gen` change (from `JitCache::run` / [`find`]): re-check affected blocks'
/// assumptions and relocations once per generation; kill failing blocks (unpatching their
/// incoming links) and rewrite moved host-fn slots. Threads still inside a killed block
/// finish it with the old decisions (the JIT's window today); killed code is never freed.
pub(in crate::emu) fn revalidate(vg: u32) {
    if aot_mode() == 0 || CHECKED_VG.load(Ordering::Acquire) == vg {
        return;
    }
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let vg = crate::exe::va_gen();
    if CHECKED_VG.load(Ordering::Acquire) == vg {
        return;
    }
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    let (new_seq, new_vg, new_sg, delta) =
        crate::exe::va_delta_since(CHECKED_SEQ.load(Ordering::Acquire));
    if let Some(d) = g.as_mut() {
        unsafe { revalidate_dir(d, new_sg, &delta) };
    }
    CHECKED_SEQ.store(new_seq, Ordering::Release);
    CHECKED_VG.store(new_vg, Ordering::Release);
}

/// Re-check live and [`DEAD_ASSUME`]/[`DEAD_HELD`] blocks affected by `delta`: kill live blocks
/// that fail, revive dead ones that pass again (relocation slots rewritten, back in the map,
/// re-linked; with `AW2_DLL_AOT_VERIFY` re-verified first). Modules untouched by an out-of-module
/// `VA_DISPATCH` delta check only their precomputed `watch_blocks` list. Returns blocks killed.
unsafe fn revalidate_dir(d: &mut Dir, cur_sg: u32, delta: &crate::exe::VaDelta) -> usize {
    #[cfg(test)]
    TEST_REVALIDATE_CALLS.fetch_add(1, Ordering::Relaxed);
    let par_workers = if cfg!(test) { 1 } else { aot_install_workers() };
    let ctx = HostCheckCtx::snapshot();
    let (mut kills, mut rewrites, mut revive) = (Vec::new(), Vec::new(), Vec::new());
    let (mut hm, mut fm) = (FastMap::default(), FastMap::default());
    for mi in 0..d.modules.len() {
        if d.modules[mi].gone || d.modules[mi].killed {
            continue;
        }
        let mg = ctx.module_gate(&d.modules[mi]);
        let full_scan = match delta {
            crate::exe::VaDelta::Full => true,
            crate::exe::VaDelta::ChangedVas(vas) => {
                d.modules[mi].watch_struct_gen != cur_sg
                    || vas.iter().any(|&va| ctx.module_affected_by_va(&d.modules[mi], va))
            }
        };
        if !full_scan {
            let m = &d.modules[mi];
            let nw = m.watch_blocks.len();
            #[cfg(test)]
            TEST_REVALIDATE_BLOCKS_CHECKED.fetch_add(nw, Ordering::Relaxed);
            if par_workers > 1 && nw >= PARALLEL_THRESHOLD {
                let chunk = nw.div_ceil(par_workers);
                type RevChunk = (
                    Vec<(usize, usize)>,
                    Vec<(usize, u64)>,
                    Vec<(usize, usize, Vec<(usize, u64)>)>,
                );
                let parts: Vec<RevChunk> = std::thread::scope(|s| {
                    let mut handles = Vec::with_capacity(par_workers);
                    let mut lo = 0usize;
                    while lo < nw {
                        let hi = (lo + chunk).min(nw);
                        handles.push(s.spawn(move || {
                            let mut lhm = FastMap::default();
                            let mut lfm = FastMap::default();
                            let (mut lk, mut lrw, mut lrev) = (Vec::new(), Vec::new(), Vec::new());
                            for &bi_u32 in &m.watch_blocks[lo..hi] {
                                let bi = bi_u32 as usize;
                                let b = &m.img.blocks[bi];
                                match m.dead[bi].load(Ordering::Relaxed) {
                                    0 => {
                                        if check_block(m, b, &ctx, &mg, &mut lhm, &mut lfm, &mut lrw)
                                            .0
                                            .is_err()
                                        {
                                            lk.push((mi, bi));
                                        }
                                    }
                                    st @ (DEAD_ASSUME | DEAD_HELD) => {
                                        let mut rw = Vec::new();
                                        if check_block(m, b, &ctx, &mg, &mut lhm, &mut lfm, &mut rw)
                                            .0
                                            .is_ok()
                                        {
                                            lrev.push((mi, bi, rw));
                                        } else if st == DEAD_HELD {
                                            m.dead[bi].store(DEAD_ASSUME, Ordering::Relaxed);
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            (lk, lrw, lrev)
                        }));
                        lo = hi;
                    }
                    handles
                        .into_iter()
                        .map(|h| h.join().expect("aot revalidate worker panicked"))
                        .collect()
                });
                for (lk, lrw, lrev) in parts {
                    kills.extend(lk);
                    rewrites.extend(lrw);
                    revive.extend(lrev);
                }
            } else {
                for &bi_u32 in &m.watch_blocks {
                    let bi = bi_u32 as usize;
                    let b = &m.img.blocks[bi];
                    match m.dead[bi].load(Ordering::Relaxed) {
                        0 => {
                            if check_block(m, b, &ctx, &mg, &mut hm, &mut fm, &mut rewrites)
                                .0
                                .is_err()
                            {
                                kills.push((mi, bi));
                            }
                        }
                        st @ (DEAD_ASSUME | DEAD_HELD) => {
                            let mut rw = Vec::new();
                            if check_block(m, b, &ctx, &mg, &mut hm, &mut fm, &mut rw).0.is_ok() {
                                revive.push((mi, bi, rw));
                            } else if st == DEAD_HELD {
                                m.dead[bi].store(DEAD_ASSUME, Ordering::Relaxed);
                            }
                        }
                        _ => {}
                    }
                }
            }
            continue;
        }
        let n = d.modules[mi].img.blocks.len();
        #[cfg(test)]
        TEST_REVALIDATE_BLOCKS_CHECKED.fetch_add(n, Ordering::Relaxed);
        let new_watch = if par_workers > 1 && n >= PARALLEL_THRESHOLD {
            let m = &d.modules[mi];
            let chunk = n.div_ceil(par_workers);
            type RevChunk = (
                Vec<(usize, usize)>,
                Vec<(usize, u64)>,
                Vec<(usize, usize, Vec<(usize, u64)>)>,
                Vec<u32>,
            );
            let parts: Vec<RevChunk> = std::thread::scope(|s| {
                let mut handles = Vec::with_capacity(par_workers);
                let mut lo = 0usize;
                while lo < n {
                    let hi = (lo + chunk).min(n);
                    handles.push(s.spawn(move || {
                        let mut lhm = FastMap::default();
                        let mut lfm = FastMap::default();
                        let (mut lk, mut lrw, mut lrev, mut lw) =
                            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
                        for bi in lo..hi {
                            let b = &m.img.blocks[bi];
                            match m.dead[bi].load(Ordering::Relaxed) {
                                0 => {
                                    let (res, nw) =
                                        check_block(m, b, &ctx, &mg, &mut lhm, &mut lfm, &mut lrw);
                                    if res.is_err() {
                                        lk.push((mi, bi));
                                        lw.push(bi as u32);
                                    } else if nw {
                                        lw.push(bi as u32);
                                    }
                                }
                                st @ (DEAD_ASSUME | DEAD_HELD) => {
                                    let mut rw = Vec::new();
                                    let (res, _nw) =
                                        check_block(m, b, &ctx, &mg, &mut lhm, &mut lfm, &mut rw);
                                    lw.push(bi as u32);
                                    if res.is_ok() {
                                        lrev.push((mi, bi, rw));
                                    } else if st == DEAD_HELD {
                                        m.dead[bi].store(DEAD_ASSUME, Ordering::Relaxed);
                                    }
                                }
                                _ => {}
                            }
                        }
                        (lk, lrw, lrev, lw)
                    }));
                    lo = hi;
                }
                handles
                    .into_iter()
                    .map(|h| h.join().expect("aot revalidate worker panicked"))
                    .collect()
            });
            let total_w: usize = parts.iter().map(|(_, _, _, w)| w.len()).sum();
            let mut merged_w = Vec::with_capacity(total_w);
            for (lk, lrw, lrev, lw) in parts {
                kills.extend(lk);
                rewrites.extend(lrw);
                revive.extend(lrev);
                merged_w.extend(lw);
            }
            merged_w
        } else {
            let m = &d.modules[mi];
            let mut lw = Vec::new();
            for (bi, b) in m.img.blocks.iter().enumerate() {
                match m.dead[bi].load(Ordering::Relaxed) {
                    0 => {
                        let (res, nw) =
                            check_block(m, b, &ctx, &mg, &mut hm, &mut fm, &mut rewrites);
                        if res.is_err() {
                            kills.push((mi, bi));
                            lw.push(bi as u32);
                        } else if nw {
                            lw.push(bi as u32);
                        }
                    }
                    st @ (DEAD_ASSUME | DEAD_HELD) => {
                        let mut rw = Vec::new();
                        let (res, _nw) =
                            check_block(m, b, &ctx, &mg, &mut hm, &mut fm, &mut rw);
                        lw.push(bi as u32);
                        if res.is_ok() {
                            revive.push((mi, bi, rw));
                        } else if st == DEAD_HELD {
                            m.dead[bi].store(DEAD_ASSUME, Ordering::Relaxed);
                        }
                    }
                    _ => {}
                }
            }
            lw
        };
        d.modules[mi].watch_blocks = new_watch;
        d.modules[mi].watch_struct_gen = cur_sg;
    }
    // Blocks whose checks started passing in this pass (DEAD_HELD ones were counted before).
    let would = revive
        .iter()
        .filter(|&&(mi, bi, _)| d.modules[mi].dead[bi].load(Ordering::Relaxed) == DEAD_ASSUME)
        .count();
    WOULD_REVIVE.fetch_add(would as u64, Ordering::Relaxed);
    if !revive_enabled() {
        for &(mi, bi, _) in &revive {
            d.modules[mi].dead[bi].store(DEAD_HELD, Ordering::Relaxed);
        }
        revive.clear();
    }
    if kills.is_empty() && rewrites.is_empty() && revive.is_empty() {
        return 0;
    }
    let mut patched = Vec::new();
    let mut revived = 0usize;
    {
        let _w = JitWrite::new();
        for &(slot, h) in &rewrites {
            (*(slot as *const AtomicU64)).store(h, Ordering::Relaxed);
        }
        kill_batch(d, &kills, DEAD_ASSUME, &mut patched);
        let mut vbc: Option<Box<BlockCache>> = None;
        let mut scratch = Vec::new();
        let mut tr_scratch = jit::TrScratch::new();
        for (mi, bi, rw) in revive {
            for &(slot, h) in &rw {
                (*(slot as *const AtomicU64)).store(h, Ordering::Relaxed);
            }
            let m = &d.modules[mi];
            let b = m.img.blocks[bi];
            if verify_mode() != 0 && !m.slot_held[bi] {
                let bc = vbc.get_or_insert_with(|| Box::new(BlockCache::new()));
                let bcp: *mut BlockCache = &mut **bc;
                let bank_vbase = m.host_addr(m.img.bank_of_off(b.code_off));
                if let Err(e) =
                    verify_block(bcp, bank_vbase, &m.img, &b, &mut scratch, &mut tr_scratch)
                {
                    log::warn!("emu aot: {}: revived block {:#x} fails verify: {e}", m.name, b.ip);
                    m.dead[bi].store(DEAD_PERM, Ordering::Release);
                    continue;
                }
            }
            icache_invalidate(m.host_addr(b.code_off) as *mut u8, b.main as usize * 4);
            if b.words > b.main {
                icache_invalidate(
                    m.host_addr(b.cold_off) as *mut u8,
                    (b.words - b.main) as usize * 4,
                );
            }
            m.dead[bi].store(0, Ordering::Release);
            // Another module's live block may own the IP (overlapping images): keep it.
            let owner_live = d.map.get(&b.ip).is_some_and(|&(m2, b2)| {
                (m2 as usize, b2 as usize) != (mi, bi)
                    && d.modules[m2 as usize].dead[b2 as usize].load(Ordering::Relaxed) == 0
            });
            if !owner_live {
                if let Some((prev_m, prev_b)) = d.map.insert(b.ip, (mi as u32, bi as u32)) {
                    if (prev_m as usize, prev_b as usize) != (mi, bi) {
                        d.has_overlaps = true;
                    }
                }
                d.probe_sync(b.ip);
            } else {
                d.has_overlaps = true;
            }
            revived += 1;
        }
        if revived != 0 {
            patched.extend(link_pass(d, None));
        }
    }
    for s in patched {
        icache_invalidate(s as *mut u8, 4);
    }
    KILLS.fetch_add(kills.len() as u64, Ordering::Relaxed);
    REVIVED.fetch_add(revived as u64, Ordering::Relaxed);
    if !kills.is_empty() || would != 0 {
        log::info!(
            "emu aot: {} block(s) invalidated, {} revived ({} would revive{}), {} host-fn slot(s) rewritten after a host dispatch change",
            kills.len(),
            revived,
            would,
            if revive_enabled() { "" } else { "; AW2_DLL_AOT_REVIVE=0" },
            rewrites.len()
        );
    }
    kills.len()
}

/// Executable memory changed (`mem::note_code_region`): a module whose image now overlaps
/// a tracked run-time code region is killed entirely.
pub(in crate::emu) fn note_code_region() {
    if !HAVE.load(Ordering::Relaxed) {
        return;
    }
    let hit = |m: &Module| {
        !m.gone && !m.killed && crate::emu::mem::jit_overlaps(m.base, m.base + m.size)
    };
    {
        let Ok(g) = DIR.read() else { return };
        if !g.as_ref().is_some_and(|d| d.modules.iter().any(hit)) {
            return;
        }
    }
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    let Some(d) = g.as_mut() else { return };
    let mut patched = Vec::new();
    unsafe {
        let _w = JitWrite::new();
        let mut kills = Vec::new();
        for mi in 0..d.modules.len() {
            if !hit(&d.modules[mi]) {
                continue;
            }
            d.modules[mi].killed = true;
            log::warn!(
                "emu aot: {} overlaps run-time generated code; its AOT code is dropped",
                d.modules[mi].name
            );
            for bi in 0..d.modules[mi].img.blocks.len() {
                if d.modules[mi].dead[bi].load(Ordering::Relaxed) != DEAD_PERM {
                    kills.push((mi, bi));
                }
            }
        }
        kill_batch(d, &kills, DEAD_PERM, &mut patched);
    }
    for s in patched {
        unsafe { icache_invalidate(s as *mut u8, 4) };
    }
    EPOCH.fetch_add(1, Ordering::Release);
}

/// Guest import slot `src` of a [`RelocEnt`] may be read (same rule as `const_slot_plain`).
fn import_slot_ok(src: u64) -> bool {
    let b = crate::exe::base() as u64;
    let in_image = (b..b.wrapping_add(crate::exe::image_size())).contains(&src)
        || crate::dll_native::in_mapped_image(src);
    (in_image || cfg!(test)) && (0x1_0000_0000..1u64 << 47).contains(&src) && src & 7 == 0
}

/// Point block `b`'s import-slot relocations (`src` != 0) at the slot's current value (it
/// differs between processes when the slot holds a host-side thunk VA) and rewrite its
/// `RELOC_SLOT_VAL` words (code writable on this thread; `RELOC_HOST_FN` words follow in
/// [`check_block`]). Each slot is read once per install (`slots`).
///
/// A slot whose current value is not inline-callable with the recorded bail (e.g. a
/// delay-load resolver thunk before the first call, where a background build saw the
/// resolved target) is *held*: the code compares the slot with the recorded value at run
/// time, so its calls take the generic path while they differ. If the recorded value is
/// still inline-callable here it is kept (the inline path applies once the slot holds it);
/// otherwise the slot's relocations become [`SLOT_DISABLED`] (guard never matches, host fn
/// word unused). Held slots go to `held` as `(slot, recorded, current)`.
///
/// Returns the number of relocations moved, or None (stale block) if a slot is unreadable.
unsafe fn rebase_block(
    img: &mut Image,
    region: usize,
    bank_shifts: &[usize],
    b: &BlockEnt,
    slots: &mut FastMap<u64, u64>,
    held: &mut Vec<(u64, u64, u64)>,
) -> Option<usize> {
    let (lo, hi) = (b.reloc_lo as usize, (b.reloc_lo + b.reloc_n) as usize);
    // (slot, keep the recorded value) for this block's held slots.
    let mut keep: Vec<(u64, bool)> = Vec::new();
    for i in lo..hi {
        let r = img.relocs[i];
        if r.src == 0 {
            continue;
        }
        if !import_slot_ok(r.src) {
            return None;
        }
        let cur = *slots
            .entry(r.src)
            .or_insert_with(|| core::ptr::read_volatile(r.src as *const u64));
        // The slot's HOST_FN relocation decides callability (a SLOT_VAL one is only the
        // guard value / bail target of the same call).
        let callable = |v: u64| matches!(jit::inline_host_fn(v), Some((_, bail)) if bail == r.bail);
        if r.kind == RELOC_HOST_FN
            && r.va != SLOT_DISABLED
            && cur != r.va
            && !callable(cur)
            && !keep.iter().any(|k| k.0 == r.src)
        {
            keep.push((r.src, callable(r.va)));
            held.push((r.src, r.va, cur));
        }
    }
    let mut moved = 0;
    for i in lo..hi {
        let r = img.relocs[i];
        if r.src == 0 || r.va == SLOT_DISABLED {
            continue;
        }
        let new = match keep.iter().find(|k| k.0 == r.src) {
            Some(&(_, true)) => continue,
            Some(&(_, false)) => SLOT_DISABLED,
            None => slots[&r.src],
        };
        if new == r.va {
            continue;
        }
        img.relocs[i].va = new;
        if r.kind == RELOC_SLOT_VAL {
            let shift = if bank_shifts.len() <= 1 {
                0
            } else {
                bank_shifts[img.bank_index_of(r.slot_off)]
            };
            (*((region + r.slot_off as usize + shift) as *const AtomicU64))
                .store(new, Ordering::Relaxed);
        }
        moved += (new != SLOT_DISABLED) as usize;
    }
    Some(moved)
}

/// [`RelocEnt::va`] of a held import slot's relocations whose recorded value is not
/// inline-callable in this process ([`rebase_block`]): the guard compares the slot with
/// this (no user-space VA), so the inline path is never taken.
const SLOT_DISABLED: u64 = u64::MAX;

/// Re-translate block `b` at its final address inside its stub bank (`bank_vbase`) and
/// compare it with the image.
unsafe fn verify_block(
    bc: *mut BlockCache,
    bank_vbase: usize,
    img: &Image,
    b: &BlockEnt,
    scratch: &mut Vec<u32>,
    tr_scratch: &mut jit::TrScratch,
) -> Result<(), String> {
    let ho = b.code_off;
    let co = b.cold_off;
    let bank_off = img.bank_of_off(ho);
    let ho_rel = (ho - bank_off) as usize;
    let co_rel = (co - bank_off) as usize;
    let Some(mut t) =
        jit::aot_translate(bc, b.ip, bank_vbase + ho_rel, bank_vbase, &img.stubs, scratch, tr_scratch)
    else {
        return Err("no longer translatable".into());
    };
    if t.words.len() != b.words as usize
        || t.main != b.main
        || t.guest != b.guest
        || t.host_entry != b.host_entry
    {
        return Err(format!(
            "shape: words {}/{} main {}/{} guest {}/{} host {}/{}",
            t.words.len(),
            b.words,
            t.main,
            b.main,
            t.guest,
            b.guest,
            t.host_entry,
            b.host_entry
        ));
    }
    split_and_rebase_block(&mut t, ho_rel, 0, ho_rel, co_rel, 0, &img.stubs);
    let main = b.main;
    let links: Vec<LinkEnt> = t
        .links
        .iter()
        .map(|&(s, f, c, tg)| LinkEnt {
            site_off: word_to_off(s, main, ho, co),
            fallback_off: word_to_off(f, main, ho, co),
            cond_off: if c == u32::MAX {
                u32::MAX
            } else {
                word_to_off(c, main, ho, co)
            },
            target: tg,
        })
        .collect();
    if links[..] != *img.links_of(b) {
        return Err("links differ".into());
    }
    let relocs: Vec<RelocEnt> = t.relocs.iter().map(|r| RelocEnt::of_split(ho, co, main, r)).collect();
    // Compared by (site, kind, bail, guest VA, import slot): host pointers are not part of
    // a RelocEnt (the slot words themselves are skipped below).
    let file_relocs = img.relocs_of(b);
    if relocs[..] != *file_relocs {
        let i = relocs
            .iter()
            .zip(file_relocs)
            .position(|(x, y)| x != y)
            .unwrap_or(relocs.len().min(file_relocs.len()));
        return Err(format!(
            "relocations differ ({} now, {} in the file; first difference #{i}: now {:?}, file {:?})",
            relocs.len(),
            file_relocs.len(),
            relocs.get(i),
            file_relocs.get(i)
        ));
    }
    if t.assume[..] != *img.assume_of(b) {
        return Err("assumptions differ".into());
    }
    let skip: HashSet<usize> =
        t.relocs.iter().flat_map(|r| [r.word as usize, r.word as usize + 1]).collect();
    let main_u = main as usize;
    let file_hot = img.hot_words_of(b);
    for (i, (&x, &y)) in t.words[..main_u].iter().zip(file_hot).enumerate() {
        if x != y && !skip.contains(&i) {
            return Err(format!("word {i}: now {x:#010x} (main), file {y:#010x}"));
        }
    }
    let file_cold = img.cold_words_of(b);
    for (ci, (&x, &y)) in t.words[main_u..].iter().zip(file_cold).enumerate() {
        let i = main_u + ci;
        if x != y && !skip.contains(&i) {
            return Err(format!("word {i}: now {x:#010x} (out-of-line), file {y:#010x}"));
        }
    }
    Ok(())
}

/// Which blocks [`install_image`] re-translates and compares ([`verify_block`]).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Verify {
    None,
    All,
    /// About `pct`% of the blocks, picked by hashing the block IP with `seed`.
    Sample { pct: u32, seed: u64 },
}

impl Verify {
    pub(super) fn picks(self, ip: u64) -> bool {
        match self {
            Verify::None => false,
            Verify::All => true,
            Verify::Sample { pct, seed } => {
                let mut x = ip ^ seed;
                x = (x ^ (x >> 33)).wrapping_mul(0xFF51_AFD7_ED55_8CCD);
                x = (x ^ (x >> 33)).wrapping_mul(0xC4CE_B9FE_1A85_EC53);
                (x ^ (x >> 33)) % 100 < pct as u64
            }
        }
    }
}

/// [`install_image`] result.
#[derive(Clone, Copy, Debug)]
pub(super) struct Installed {
    pub(super) mi: usize,
    pub(super) live: usize,
    /// Blocks re-translated and compared ...
    pub(super) verified: usize,
    /// ... of which mismatched (dropped).
    pub(super) mismatches: usize,
}

/// Copy `img` behind fresh stubs into a new `MAP_JIT` region, check/relocate/link it and
/// publish its live blocks (None: not installed).
pub(super) unsafe fn install_image(
    name: &str,
    base: usize,
    size: usize,
    img: Image,
    verify: Verify,
    test: bool,
) -> Option<Installed> {
    install_image_opts(name, base, size, img, None, verify, test, false)
}

unsafe fn install_image_opts(
    name: &str,
    base: usize,
    size: usize,
    img: Image,
    raw_code: Option<&[u8]>,
    verify: Verify,
    test: bool,
    defer_probe: bool,
) -> Option<Installed> {
    if !test && crate::emu::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!(
            "emu aot: {name}: overlaps run-time generated/modified code; skipping AOT install"
        );
        return None;
    }
    if !jit_writable() {
        log::info!(
            "emu aot: {name}: MAP_JIT unavailable; skipping dynamic .aot install (using static AOT / interpreter)"
        );
        return None;
    }
    let code_bytes = img.code_bytes();
    if let Some(raw) = raw_code {
        if raw.len() != code_bytes {
            log::warn!(
                "emu aot: {name}: raw code slice len {} != expected {code_bytes}; ignored",
                raw.len()
            );
            return None;
        }
    } else if img.code.len() * 4 != code_bytes {
        log::warn!(
            "emu aot: {name}: heap code buffer len {} != expected {code_bytes}; ignored",
            img.code.len() * 4
        );
        return None;
    }
    debug_assert!(verify == Verify::None || img.code.len() * 4 == code_bytes);
    let banks_snap: Vec<(usize, usize)> = img
        .stub_banks()
        .iter()
        .enumerate()
        .map(|(bi, &bo)| (bo as usize, img.bank_end(bi)))
        .collect();
    let nbanks = banks_snap.len();
    let mut bank_shifts: Vec<usize> = Vec::with_capacity(nbanks);
    let mut bank_rt_offs: Vec<usize> = Vec::with_capacity(nbanks);
    let mut bank_slab_offs: Vec<usize> = Vec::with_capacity(nbanks);
    let mut bank_far_offs: Vec<usize> = Vec::with_capacity(nbanks);
    let mut cursor = 0usize;
    for &(bo, b_end) in &banks_snap {
        cursor = (cursor + 15) & !15;
        bank_rt_offs.push(cursor);
        bank_shifts.push(cursor - bo);
        let bank_len = b_end - bo;
        let slab_off = (cursor + bank_len + 15) & !15;
        bank_slab_offs.push(slab_off);
        let far_off = slab_off + AOT_LSE2_SLAB;
        bank_far_offs.push(far_off);
        cursor = far_off + AOT_FAR_SLAB;
    }
    let len = (cursor + 0x3FFF) & !0x3FFF;
    let Some(p) = alloc_jit(len) else {
        log::error!("emu aot: {name}: mmap(MAP_JIT, {len:#x}) failed");
        return None;
    };
    let region = p as usize;
    let mut lse2_slots: Vec<usize> = Vec::with_capacity(nbanks);
    let mut far_slabs: Vec<FarSlab> = Vec::with_capacity(nbanks);
    for bi in 0..nbanks {
        let rt_base = region + bank_rt_offs[bi];
        let bank_len = banks_snap[bi].1 - banks_snap[bi].0;
        let code_lo = rt_base + AOT_STUBS_BYTES;
        let code_hi = rt_base + bank_len;
        let slab_lo = region + bank_slab_offs[bi];
        let slab_hi = slab_lo + AOT_LSE2_SLAB;
        // Guard `b_reach` (±128 MiB) so `patch_lse2_unaligned` can reach every slot in the slab
        // from every instruction in this bank.
        if code_lo < code_hi && reach(code_lo, slab_hi) && reach(slab_hi, code_lo) {
            if let Some(slot) = jit::register_lse2_region(code_lo, code_hi, slab_lo, slab_hi) {
                lse2_slots.push(slot);
            }
        }
        let far_lo = region + bank_far_offs[bi];
        let far_hi = far_lo + AOT_FAR_SLAB;
        let far_len = if code_lo < code_hi && reach(code_lo, far_hi) && reach(far_hi, code_lo) {
            AOT_FAR_SLAB
        } else {
            0
        };
        far_slabs.push(FarSlab { lo: far_lo, used: 0, len: far_len, islands: FastMap::default() });
    }
    let t0 = std::time::Instant::now();
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let (seq_before, vg_before, sg_before) = crate::exe::va_snapshot_gens();
    let n = img.blocks.len();
    let nlinks = img.links.len();
    let mut m = Module {
        name: name.to_string(),
        base,
        size,
        region,
        region_len: len,
        bank_shifts,
        far_slabs,
        img,
        dead: alloc_atomic_u8_zeroed(n),
        linked: vec![false; nlinks],
        unlinked: Vec::new(),
        unlinked_init: false,
        needs_icache_flush: defer_probe,
        slot_held: vec![false; n],
        watch_blocks: Vec::new(),
        watch_struct_gen: sg_before,
        killed: false,
        gone: false,
        owner: test.then(|| std::thread::current().id()),
    };
    let w = JitWrite::new();
    for bi in 0..nbanks {
        let rt_base = region + bank_rt_offs[bi];
        if jit::aot_emit_stubs(rt_base as *mut u32, rt_base) != Some(m.img.stubs) {
            drop(w);
            for slot in lse2_slots {
                jit::unregister_lse2_region(slot);
            }
            free_jit(region as *mut u8, len);
            log::warn!(
                "emu aot: {name}: shared stub layout differs from the cache (bank {bi}); ignored"
            );
            return None;
        }
    }
    // Stream machine code straight from `raw_code` (mmap'd `.aot`) or `m.img.code` into `MAP_JIT`.
    // For multi-megabyte modules, split into 2 MiB chunks across worker threads so initial page
    // faults and copies scale across cores.
    let copy_workers = if test || code_bytes < (4 << 20) { 1 } else { aot_install_workers() };
    let src_bytes: &[u8] = match raw_code {
        Some(raw) => raw,
        None => core::slice::from_raw_parts(m.img.code.as_ptr() as *const u8, code_bytes),
    };
    if copy_workers <= 1 {
        for bi in 0..nbanks {
            let rt_base = region + bank_rt_offs[bi];
            let code_start_off = banks_snap[bi].0 + AOT_STUBS_BYTES;
            let code_end_off = banks_snap[bi].1;
            if code_end_off > code_start_off {
                let byte_lo = code_start_off - AOT_STUBS_BYTES;
                let byte_len = code_end_off - code_start_off;
                core::ptr::copy_nonoverlapping(
                    src_bytes.as_ptr().add(byte_lo),
                    (rt_base + AOT_STUBS_BYTES) as *mut u8,
                    byte_len,
                );
            }
        }
    } else {
        const COPY_CHUNK: usize = 2 << 20;
        let mut copy_chunks: Vec<(usize, usize, usize)> = Vec::new();
        let src_base = src_bytes.as_ptr() as usize;
        for bi in 0..nbanks {
            let rt_base = region + bank_rt_offs[bi];
            let code_start_off = banks_snap[bi].0 + AOT_STUBS_BYTES;
            let code_end_off = banks_snap[bi].1;
            if code_end_off > code_start_off {
                let mut byte_lo = code_start_off - AOT_STUBS_BYTES;
                let mut dst_addr = rt_base + AOT_STUBS_BYTES;
                let mut rem = code_end_off - code_start_off;
                while rem > 0 {
                    let take = rem.min(COPY_CHUNK);
                    copy_chunks.push((src_base + byte_lo, dst_addr, take));
                    byte_lo += take;
                    dst_addr += take;
                    rem -= take;
                }
            }
        }
        let n_workers = copy_workers.min(copy_chunks.len().max(1));
        let next = AtomicUsize::new(0);
        let chunks_ref = &copy_chunks;
        std::thread::scope(|s| {
            for _ in 0..n_workers {
                let next = &next;
                s.spawn(move || {
                    let _ww = JitWrite::new();
                    loop {
                        let ci = next.fetch_add(1, Ordering::Relaxed);
                        let Some(&(src_addr, dst_addr, chunk_len)) = chunks_ref.get(ci) else {
                            break;
                        };
                        core::ptr::copy_nonoverlapping(
                            src_addr as *const u8,
                            dst_addr as *mut u8,
                            chunk_len,
                        );
                    }
                });
            }
        });
    }
    let (mut bad_verify, mut bad_slot, mut rebased) = (0usize, 0usize, 0usize);
    let mut slots: FastMap<u64, u64> = FastMap::default();
    // Blocks with a held import slot (see rebase_block): correct (guarded) but not verified,
    // since their re-translation now sees a different slot state.
    let mut held_blocks = 0usize;
    let mut held = Vec::new();
    for bi in 0..n {
        if m.img.blocks[bi].reloc_n == 0 {
            continue;
        }
        let b = m.img.blocks[bi];
        held.clear();
        match rebase_block(&mut m.img, region, &m.bank_shifts, &b, &mut slots, &mut held) {
            Some(k) => rebased += k,
            None => {
                m.dead[bi].store(DEAD_PERM, Ordering::Relaxed);
                bad_slot += 1;
                continue;
            }
        }
        if held.is_empty() {
            continue;
        }
        m.slot_held[bi] = true;
        if held_blocks < 4 {
            for &(slot, rec, cur) in &held {
                let off = if m
                    .img
                    .relocs_of(&b)
                    .iter()
                    .any(|r| r.src == slot && r.va == SLOT_DISABLED)
                {
                    "; recorded value not callable here: disabled"
                } else {
                    ""
                };
                log::info!(
                    "emu aot: {name}: block {:#x}: import slot {slot:#x} holds {cur:#x}, cache recorded {rec:#x} (guarded call takes the generic path{off})",
                    b.ip
                );
            }
        }
        held_blocks += 1;
    }
    #[cfg(test)]
    {
        TEST_REBASED.with(|c| c.set(c.get() + rebased));
        TEST_HELD.with(|c| c.set(c.get() + held_blocks));
    }
    // Assumptions first: a block built for another host-callable state is dead-but-revivable
    // (its re-translation now would differ, so it is not verified until revived).
    let check_workers = if test || n < PARALLEL_THRESHOLD { 1 } else { aot_install_workers() };
    let (bad_check, rewrites, watch_blocks) = check_install_blocks(&m, check_workers);
    m.watch_blocks = watch_blocks;
    for &(slot, h) in &rewrites {
        (*(slot as *const AtomicU64)).store(h, Ordering::Relaxed);
    }
    let mut verified = 0usize;
    if verify != Verify::None {
        let mut bc = Box::new(BlockCache::new());
        let bcp: *mut BlockCache = &mut *bc;
        let mut scratch = Vec::new();
        let mut tr_scratch = jit::TrScratch::new();
        for bi in 0..n {
            let b = m.img.blocks[bi];
            if m.dead[bi].load(Ordering::Relaxed) != 0 || m.slot_held[bi] || !verify.picks(b.ip) {
                continue;
            }
            verified += 1;
            let bank_vbase = m.host_addr(m.img.bank_of_off(b.code_off));
            if let Err(e) = verify_block(bcp, bank_vbase, &m.img, &b, &mut scratch, &mut tr_scratch)
            {
                if bad_verify < 8 {
                    log::warn!("emu aot: {name}: verify mismatch in block {:#x}: {e}", b.ip);
                }
                bad_verify += 1;
                m.dead[bi].store(DEAD_PERM, Ordering::Relaxed);
            }
        }
    }
    #[cfg(test)]
    {
        TEST_VERIFIED.with(|c| c.set(c.get() + verified));
        TEST_VERIFY_MISMATCHES.with(|c| c.set(c.get() + bad_verify));
    }
    // Machine code has already been copied into `MAP_JIT` and any initial load-time
    // verification is complete. Free the redundant heap `Vec<u32>` when `verify_mode() == 0`;
    // retain it whenever `PORT_AOT_VERIFY` is active because `revalidate_dir` may later
    // invalidate and revive blocks (even if `bad_check == 0` at initial load) and needs
    // `m.img.code` for `verify_block`.
    if !test && verify_mode() == 0 {
        m.img.release_code_heap();
    }
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    let d = g.get_or_insert_with(|| Dir {
        modules: Vec::new(),
        map: FastMap::default(),
        has_overlaps: false,
        probe: ProbeTab::default(),
        batch_init_gens: None,
    });
    let had_live_modules = d.modules.iter().any(|existing| !existing.gone && !existing.killed);
    let mi = d.modules.len() as u32;
    let aot_filt = crate::diag::aot_filter_active();
    let mut filt_skipped = 0usize;
    let mut live = 0usize;
    d.map.reserve(n);
    let mut probe_updates: Vec<(u64, u64)> = if defer_probe {
        Vec::new()
    } else {
        Vec::with_capacity(n)
    };
    for (bi, b) in m.img.blocks.iter().enumerate() {
        if m.dead[bi].load(Ordering::Relaxed) == 0 {
            if aot_filt && !crate::diag::aot_allows(b.ip) {
                m.dead[bi].store(DEAD_PERM, Ordering::Relaxed);
                filt_skipped += 1;
                continue;
            }
            let replaced = d.map.insert(b.ip, (mi, bi as u32)).is_some();
            if replaced {
                d.has_overlaps = true;
            }
            if !defer_probe {
                if !b.host_entry {
                    probe_updates.push((b.ip, m.host_addr(b.code_off) as u64));
                } else if replaced {
                    probe_updates.push((b.ip, 0));
                }
            }
            live += 1;
        }
    }
    if filt_skipped != 0 {
        log::info!(
            "emu aot: {name}: PORT_AOT_SKIP/ALLOW excluded {filt_skipped}/{n} blocks (falling back to live JIT)"
        );
    }
    d.modules.push(m);
    if defer_probe {
        // Multi-module startup batch: defer linking, bulk icache invalidation, `PROBE`, and
        // `HAVE` publication to `flush_probe_table()` so all modules link in a single pass
        // directly against the freshly built `ProbeTab`.
        if !had_live_modules && d.batch_init_gens.is_none() {
            d.batch_init_gens = Some((seq_before, vg_before));
        }
        drop(w);
        drop(g);
        log::info!(
            "emu aot: {name}: {live}/{n} blocks live ({} KiB code across {nbanks} bank(s) at {region:#x}; {bad_verify} verify mismatches ({verified} verified), {bad_check} stale (revivable), {bad_slot} stale import slots, {held_blocks} slot state differs (guarded, unverified); {rebased} slot relocations rebased; {nlinks} links deferred; {:.1} ms)",
            code_bytes >> 10,
            t0.elapsed().as_secs_f64() * 1e3
        );
        return Some(Installed { mi: mi as usize, live, verified, mismatches: bad_verify });
    }
    let patched = link_pass(d, Some((region, region + len)));
    let linked = {
        let m_ref = &d.modules[mi as usize];
        if m_ref.unlinked_init {
            nlinks - m_ref.unlinked.len()
        } else {
            m_ref.linked.iter().filter(|&&b| b).count()
        }
    };
    let miss = unlinked_reasons(d, mi as usize);
    drop(w);
    let ic_workers = if test || len < (4 << 20) { 1 } else { aot_install_workers() };
    icache_invalidate_regions_parallel(&[(region, region + len)], ic_workers);
    for &s in &patched {
        icache_invalidate(s as *mut u8, 4);
    }
    // Lock-free readers (the stubs' probe) only see the code once it is coherent.
    d.probe_insert_batch(&probe_updates);
    if !had_live_modules {
        // Every block in this first live module was just validated against `>= vg_before`, and
        // no pre-existing live modules could be at an older `va_gen`. Publishing `seq_before` /
        // `vg_before` avoids a redundant full-directory revalidation on the very first `find()`
        // while still triggering revalidation if `va_gen()` advanced during install.
        CHECKED_SEQ.store(seq_before, Ordering::Release);
        CHECKED_VG.store(vg_before, Ordering::Release);
    }
    HAVE.store(true, Ordering::Release);
    drop(g);
    log::info!(
        "emu aot: {name}: {live}/{n} blocks live ({} KiB code across {nbanks} bank(s) at {region:#x}; {bad_verify} verify mismatches ({verified} verified), {bad_check} stale (revivable), {bad_slot} stale import slots, {held_blocks} slot state differs (guarded, unverified); {rebased} slot relocations rebased; {linked}/{nlinks} links patched, unpatched: {} target not in a cache, {} target dead, {} host-callable target, {} out of b range; {:.1} ms)",
        code_bytes >> 10,
        miss[0],
        miss[1],
        miss[2],
        miss[3],
        t0.elapsed().as_secs_f64() * 1e3
    );
    Some(Installed { mi: mi as usize, live, verified, mismatches: bad_verify })
}

/// Why module `mi`'s unpatched link sites are unpatched: `[target not in any loaded cache
/// (not built, or in a module not loaded yet), target block dead, target host-callable
/// (never a direct-link target), target out of `b` range]`.
fn unlinked_reasons(d: &Dir, mi: usize) -> [usize; 4] {
    let m = &d.modules[mi];
    let mut miss = [0usize; 4];
    let mut classify = |l: &LinkEnt| {
        let k = match d.map.get(&l.target) {
            Some(&(tm, tb)) => {
                let t = &d.modules[tm as usize];
                if t.dead[tb as usize].load(Ordering::Relaxed) != 0 {
                    1
                } else if t.img.blocks[tb as usize].host_entry {
                    2
                } else {
                    3
                }
            }
            // Blocks are sorted by IP; dead ones are not in the directory map.
            None => match m.img.blocks.binary_search_by_key(&l.target, |b| b.ip) {
                Ok(_) => 1,
                Err(_) => 0,
            },
        };
        miss[k] += 1;
    };
    if m.unlinked_init {
        for &li in &m.unlinked {
            classify(&m.img.links[li as usize]);
        }
    } else {
        for (li, l) in m.img.links.iter().enumerate() {
            if !m.linked[li] {
                classify(l);
            }
        }
    }
    miss
}

/// Pre-read and checksum-verified AOT cache ready to install on the main thread.
struct DecodedModule {
    path: PathBuf,
    from_fallback: bool,
    img: Image,
    flags: u32,
    /// Retained mmap/backing file when either `img.code` was not copied to heap (so
    /// `install_image_opts` copies straight from `&bytes[code_byte_off..]` into `MAP_JIT`) or
    /// `FLAG_VERIFY_PENDING` is set on a writable cache (so we can clear the flag on disk).
    /// Dropped immediately after `install_decoded_module` finishes.
    backing: Option<(AotBytes, usize)>,
}

unsafe fn read_and_decode_module(
    name: &str,
    base: usize,
    size: usize,
    key: &AotKey,
    dirs: Option<(&Path, &[PathBuf])>,
    retain_code: bool,
) -> Option<DecodedModule> {
    if crate::emu::mem::jit_overlaps(base, base.wrapping_add(size)) {
        log::info!(
            "emu aot: {name}: overlaps run-time generated/modified code; skipping AOT load"
        );
        return None;
    }
    let t0 = std::time::Instant::now();
    let (path, hit) = match dirs {
        Some((primary, fallbacks)) => read_cache_file_in(primary, fallbacks, name, key),
        None => read_cache_file(name, key),
    };
    let Some((bytes, from_fallback)) = hit else {
        log::info!("emu aot: {name}: no cache for this build/knobs ({})", path.display());
        return None;
    };
    let Some((fname, fkey, img, flags, code_byte_off)) = decode_for_runtime(&bytes, retain_code)
    else {
        log::warn!("emu aot: {name}: {} is corrupt; ignored", path.display());
        return None;
    };
    if fname != name || fkey != *key {
        log::warn!("emu aot: {name}: {} has a different key; ignored", path.display());
        return None;
    }
    log::info!(
        "emu aot: {name}: read {} KiB in {:.1} ms",
        bytes.len() >> 10,
        t0.elapsed().as_secs_f64() * 1e3
    );
    let pending = flags & FLAG_VERIFY_PENDING != 0;
    let need_raw_code = img.code.is_empty() && img.code_bytes() > 0;
    let backing = (need_raw_code || (pending && !from_fallback)).then_some((bytes, code_byte_off));
    Some(DecodedModule { path, from_fallback, img, flags, backing })
}

/// Invalidate instruction caches across `regions` (`(lo, hi)` byte spans), fanning out
/// multi-megabyte regions in 2 MiB chunks across `workers` so `dc cvau`/`ic ivau` scales
/// across cores. All workers join before returning (inner-shareable `dsb ish` per chunk).
unsafe fn icache_invalidate_regions_parallel(regions: &[(usize, usize)], workers: usize) {
    const ICACHE_CHUNK: usize = 2 << 20;
    let mut chunks: Vec<(usize, usize)> = Vec::new();
    for &(lo, hi) in regions {
        let mut cur = lo;
        while cur < hi {
            let take = (hi - cur).min(ICACHE_CHUNK);
            chunks.push((cur, take));
            cur += take;
        }
    }
    if workers <= 1 || chunks.len() <= 1 {
        for &(addr, len) in &chunks {
            icache_invalidate(addr as *mut u8, len);
        }
        return;
    }
    let n_workers = workers.min(chunks.len());
    let next = AtomicUsize::new(0);
    let chunks_ref = &chunks;
    std::thread::scope(|s| {
        for _ in 0..n_workers {
            let next = &next;
            s.spawn(move || loop {
                let ci = next.fetch_add(1, Ordering::Relaxed);
                let Some(&(addr, len)) = chunks_ref.get(ci) else { break };
                icache_invalidate(addr as *mut u8, len);
            });
        }
    });
}

/// Finalize a multi-module deferred `activate()` batch: build the coalesced probe table,
/// link all staged modules in parallel directly against `ProbeTab::lookup`, bulk-invalidate
/// staged `MAP_JIT` regions, and publish `PROBE` + `CHECKED_VG` + `HAVE`.
unsafe fn flush_probe_table() {
    let t0 = std::time::Instant::now();
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    let Some(d) = g.as_mut() else { return };
    if d.modules.is_empty() {
        return;
    }
    let workers = aot_install_workers();
    let t_probe0 = std::time::Instant::now();
    let new_probe = if jit::aot_probe_enabled() && !d.map.is_empty() {
        Some(d.build_probe_table(workers, true))
    } else {
        None
    };
    let probe_ms = t_probe0.elapsed().as_secs_f64() * 1e3;

    let mut pending_icache: Vec<(usize, usize)> = Vec::new();
    for m in &d.modules {
        if m.needs_icache_flush {
            pending_icache.push((m.region, m.region + m.region_len));
        }
    }
    let t_link0 = std::time::Instant::now();
    let patched = {
        let _w = JitWrite::new();
        link_pass_with_probe(d, new_probe.as_ref(), &pending_icache, true)
    };
    let link_ms = t_link0.elapsed().as_secs_f64() * 1e3;

    let t_ic0 = std::time::Instant::now();
    icache_invalidate_regions_parallel(&pending_icache, workers);
    for &s in &patched {
        icache_invalidate(s as *mut u8, 4);
    }
    for m in &mut d.modules {
        m.needs_icache_flush = false;
    }
    let icache_ms = t_ic0.elapsed().as_secs_f64() * 1e3;

    if let Some(t) = new_probe {
        PROBE.store(t.base, Ordering::Release);
        d.probe = t;
    }
    if let Some((seq, vg)) = d.batch_init_gens.take() {
        CHECKED_SEQ.store(seq, Ordering::Release);
        CHECKED_VG.store(vg, Ordering::Release);
    }
    if d.modules.iter().any(|m| !m.gone && !m.killed) {
        HAVE.store(true, Ordering::Release);
    }
    let mut total_links = 0usize;
    let mut total_linked = 0usize;
    for m in &d.modules {
        if m.gone || m.killed {
            continue;
        }
        let nl = m.img.links.len();
        total_links += nl;
        total_linked += if m.unlinked_init {
            nl - m.unlinked.len()
        } else {
            m.linked.iter().filter(|&&b| b).count()
        };
    }
    log::info!(
        "emu aot: published shared probe table ({} live entries, cap {}) and patched {total_linked}/{total_links} links ({probe_ms:.1} ms probe, {link_ms:.1} ms link, {icache_ms:.1} ms icache, {:.1} ms total)",
        d.probe.used,
        d.probe.cap,
        t0.elapsed().as_secs_f64() * 1e3
    );
}

/// RAII guard ensuring a deferred multi-module batch always links, flushes icache, and
/// publishes `PROBE` + `HAVE` before returning.
struct ProbeBatchGuard(bool);

impl Drop for ProbeBatchGuard {
    fn drop(&mut self) {
        if self.0 {
            unsafe { flush_probe_table() };
        }
    }
}

/// Count how many RVAs in `prof` are absent from installed module `mi`'s `img.blocks`
/// (live or dead), using O(log N) binary search when `img.blocks_sorted` holds.
pub(super) fn count_missing_profile_ips(mi: usize, base: usize, prof: &[u32]) -> usize {
    let g = DIR.read().unwrap_or_else(|e| e.into_inner());
    let Some(m) = g.as_ref().and_then(|d| d.modules.get(mi)) else {
        return prof.len();
    };
    let blocks = &m.img.blocks;
    if m.img.blocks_sorted {
        prof.iter()
            .filter(|&&rva| {
                let ip = (base as u64).wrapping_add(rva as u64);
                blocks.binary_search_by_key(&ip, |b| b.ip).is_err()
            })
            .count()
    } else {
        let mut have: FastSet<u64> =
            FastSet::with_capacity_and_hasher(blocks.len(), Default::default());
        for b in blocks {
            have.insert(b.ip);
        }
        prof.iter()
            .filter(|&&rva| !have.contains(&(base as u64).wrapping_add(rva as u64)))
            .count()
    }
}

unsafe fn install_decoded_module(
    name: &str,
    base: usize,
    size: usize,
    decoded: DecodedModule,
    defer_probe: bool,
) -> Option<usize> {
    let DecodedModule { path, from_fallback, img, flags, backing } = decoded;
    if !from_fallback {
        touch(&path);
    }
    let nblocks = img.blocks.len();
    let pending = flags & FLAG_VERIFY_PENDING != 0;
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos() as u64);
    let verify = load_verify(verify_mode() != 0, pending, nblocks, seed);
    let code_bytes = img.code_bytes();
    let r = {
        let raw_code = if img.code.is_empty() && code_bytes > 0 {
            backing
                .as_ref()
                .and_then(|(b, off)| b.as_slice().get(*off..(*off).checked_add(code_bytes)?))
        } else {
            None
        };
        install_image_opts(name, base, size, img, raw_code, verify, false, defer_probe)?
    };
    if pending {
        let cleared = !from_fallback
            && r.mismatches == 0
            && backing.is_some_and(|(b, _)| {
                let mut owned = b.as_slice().to_vec();
                set_flags(&mut owned, flags & !FLAG_VERIFY_PENDING)
                    && write_atomic(&path, &owned).is_ok()
            });
        log::info!(
            "emu aot: {name}: first load of a background-built cache: {} of {} blocks verified ({}), {} mismatches, {} live; {}",
            r.verified,
            nblocks,
            match verify {
                Verify::Sample { pct, .. } => format!("{pct}% sample"),
                _ => "all".into(),
            },
            r.mismatches,
            r.live,
            if cleared {
                "flag cleared"
            } else if from_fallback {
                "read-only fallback left untouched"
            } else {
                "verified again at the next load"
            }
        );
    }
    Some(r.mi)
}

/// Load the cache of image `name` mapped at `[base, base + size)` if it matches `key`
/// (computed at activation). Returns the IPs of the installed image's blocks (live or not),
/// or None if no valid cache was installed.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) unsafe fn load_module(
    name: &str,
    base: usize,
    size: usize,
    key: &AotKey,
) -> Option<Vec<u64>> {
    let decoded = read_and_decode_module(name, base, size, key, None, verify_mode() != 0)?;
    let mi = install_decoded_module(name, base, size, decoded, false)?;
    let g = DIR.read().unwrap_or_else(|e| e.into_inner());
    let m = g.as_ref()?.modules.get(mi)?;
    Some(m.img.blocks.iter().map(|b| b.ip).collect())
}

/// Blocks above which the first load of a background-built cache verifies a sample ...
pub(super) const FIRST_LOAD_FULL_MAX: usize = 50_000;
/// ... of this many percent of the blocks.
pub(super) const FIRST_LOAD_SAMPLE_PCT: u32 = 2;

/// What [`load_module`] verifies: everything with `AW2_DLL_AOT_VERIFY=1` (`explicit`);
/// a background-built cache's first load (`pending`): all of a module with fewer than
/// [`FIRST_LOAD_FULL_MAX`] blocks, else a [`FIRST_LOAD_SAMPLE_PCT`]% sample (load cost
/// bounded); otherwise nothing.
pub(super) fn load_verify(explicit: bool, pending: bool, blocks: usize, seed: u64) -> Verify {
    if explicit || (pending && blocks < FIRST_LOAD_FULL_MAX) {
        Verify::All
    } else if pending {
        Verify::Sample { pct: FIRST_LOAD_SAMPLE_PCT, seed }
    } else {
        Verify::None
    }
}

// ---------------------------------------------------------------------------------------------
// Mapped images / activation (loader entry points)
// ---------------------------------------------------------------------------------------------

pub(super) struct Mapped {
    pub(super) name: String,
    pub(super) base: usize,
    pub(super) size: usize,
    active: bool,
    /// Image hash at activation (`AW2_JIT_PROFILE`): names the profile file, so it matches
    /// the build-time key even if the code is patched later.
    pub(super) phash: Option<[u64; 2]>,
    /// Relocation-invariant image fingerprint for runtime callback RVA replay.
    pub(super) fingerprint: Option<super::callbacks::ImageFingerprint>,
    /// Cached executable section ranges `[start, end)` for O(1) code-vs-data checks.
    pub(super) exec: Vec<(usize, usize)>,
}

pub(super) static MAPPED: Mutex<Vec<Mapped>> = Mutex::new(Vec::new());

/// A PE image was mapped, relocated and import-bound (loader).
pub fn module_mapped(name: &str, base: usize, size: usize) {
    let _ = bind_static_aot_module(name, base, size);
    let norm = name.to_ascii_lowercase();
    unsafe { super::volatile::register_mapped_module(&norm, base, size) };
    let fingerprint = unsafe { super::callbacks::image_fingerprint(base, size) };
    let exec = unsafe { super::format::exec_sections(base, size) };
    if let Ok(mut v) = MAPPED.lock() {
        if let Some(existing) = v
            .iter_mut()
            .find(|m| m.name == norm && m.base == base && m.size == size)
        {
            existing.active = false;
            existing.phash = None;
            existing.fingerprint = fingerprint;
            existing.exec = exec;
            return;
        }
        v.push(Mapped {
            name: norm,
            base,
            size,
            active: false,
            phash: None,
            fingerprint,
            exec,
        });
    }
}

/// Loader activation point (before DllMains run, after the AW2 registry is applied):
/// build caches (`AW2_DLL_AOT_BUILD`, then exit) or load them (`AW2_DLL_AOT=1|2`; with 2,
/// missing / outgrown caches are rebuilt in the background) for every image mapped since
/// the last call.
pub fn activate() {
    crate::pc_sample::start(resolve_host_pc, resolve_guest_va);
    crate::diag::install_boundary_hook();
    let pending: Vec<(String, usize, usize)> = match MAPPED.lock() {
        Ok(mut v) => v
            .iter_mut()
            .filter(|m| !m.active)
            .map(|m| {
                m.active = true;
                (m.name.clone(), m.base, m.size)
            })
            .collect(),
        Err(_) => return,
    };
    if pending.is_empty() || super::disk_aot_disabled() {
        return;
    }
    let hashes: Vec<Option<[u64; 2]>> = if jit_writable() && profile_enabled() {
        // Hash mapped executable sections in parallel outside `MAPPED.lock()` so large images
        // (e.g. primary EXE + middleware DLLs) overlap across cores instead of serializing.
        let computed: Vec<[u64; 2]> = if pending.len() == 1 {
            vec![unsafe { image_hash(pending[0].1, pending[0].2) }]
        } else {
            std::thread::scope(|s| {
                let handles: Vec<_> = pending
                    .iter()
                    .map(|&(_, base, size)| s.spawn(move || unsafe { image_hash(base, size) }))
                    .collect();
                handles
                    .into_iter()
                    .map(|h| h.join().expect("aot phash worker panicked"))
                    .collect()
            })
        };
        if let Ok(mut v) = MAPPED.lock() {
            for ((name, base, size), &h) in pending.iter().zip(computed.iter()) {
                if let Some(m) =
                    v.iter_mut().find(|m| m.name == *name && m.base == *base && m.size == *size)
                {
                    m.phash = Some(h);
                }
            }
        }
        computed.into_iter().map(Some).collect()
    } else {
        vec![None; pending.len()]
    };
    if let Some(sel) = build_request() {
        unsafe { build_all(&pending, &sel) };
        log::info!("emu aot: AW2_DLL_AOT_BUILD done; exiting");
        std::process::exit(0);
    }
    if aot_mode() == 0 || !jit_writable() {
        return;
    }
    ACTIVATED.get_or_init(std::time::Instant::now);
    if aot_mode() == 2 {
        sweep_start();
    }
    let mut targets: Vec<(String, usize, usize, Option<[u64; 2]>)> = Vec::with_capacity(pending.len());
    for ((name, base, size), h) in pending.into_iter().zip(hashes) {
        if crate::exe::aw2_registry_active() && base == crate::exe::base() {
            // Every AlanWake2.exe function is a native replacement, and the emulator routes the
            // exe import thunks / special-ABI helpers natively (`emu::exe_route`). So the exe
            // gets no translation cache: older caches only hold import-thunk blocks, and the
            // shared probe table would otherwise run them from translated DLL code.
            log::info!(
                "emu aot: {name}: skipped (AW2 exe: native replacements, import thunks routed by the emulator)"
            );
            continue;
        }
        targets.push((name, base, size, h));
    }
    if targets.is_empty() {
        return;
    }
    let mut jobs = Vec::new();
    if targets.len() == 1 {
        let (name, base, size, h) = &targets[0];
        let key = unsafe { module_key_hashed(*base, *size, *h) };
        let loaded_mi = unsafe {
            read_and_decode_module(name, *base, *size, &key, None, verify_mode() != 0)
                .and_then(|d| install_decoded_module(name, *base, *size, d, false))
        };
        if aot_mode() == 2 {
            jobs.extend(bg_job_for_module(name, *base, *size, key, loaded_mi));
        }
    } else {
        // Schedule largest images first (stable tie-break) so the biggest `.aot` cache decodes
        // and installs immediately instead of stalling the pipeline at the tail of the batch.
        targets.sort_by(|a, b| b.2.cmp(&a.2));
        // Compute thread-local-sensitive key fields once on the caller thread, then hash all
        // mapped executable sections in parallel before pipelining read/decode with install.
        let cg = codegen_hash();
        let knobs = jit::aot_knob_bits();
        let keys: Vec<AotKey> = std::thread::scope(|s| {
            let handles: Vec<_> = targets
                .iter()
                .map(|(_, base, size, h)| {
                    let (base, size, h) = (*base, *size, *h);
                    s.spawn(move || AotKey {
                        image_hash: h.unwrap_or_else(|| unsafe { image_hash(base, size) }),
                        image_base: base as u64,
                        image_size: size as u64,
                        codegen_hash: cg,
                        knobs,
                    })
                })
                .collect();
            handles.into_iter().map(|h| h.join().expect("aot image_hash worker panicked")).collect()
        });
        let primary_dir = cache_dir();
        let fallbacks = fallback_cache_dirs_for(&primary_dir);
        let mode = aot_mode();
        let retain_code = verify_mode() != 0;
        // Coalesce startup `probe_rebuild` + linking + icache invalidation into one final pass
        // only for the initial unpublished directory (`!HAVE && PROBE == 0`), so later
        // multi-module activations (including under `AW2_DLL_AOT_PROBE=0`) never stage modules
        // behind an already-active directory.
        let defer_probe = !HAVE.load(Ordering::Acquire) && PROBE.load(Ordering::Acquire) == 0;
        let _probe_guard = ProbeBatchGuard(defer_probe);
        // Overlap disk I/O + checksum + decode of the next module with `install_image` of the
        // current module while capping buffered decoded images to 1 (strictly deterministic order).
        std::thread::scope(|s| {
            let (tx, rx) = std::sync::mpsc::sync_channel::<Option<DecodedModule>>(1);
            let targets_ref = &targets;
            let keys_ref = &keys;
            let primary_ref = &primary_dir;
            let fallbacks_ref = &fallbacks;
            s.spawn(move || {
                for ((name, base, size, _), key) in targets_ref.iter().zip(keys_ref.iter()) {
                    let dec = unsafe {
                        read_and_decode_module(
                            name,
                            *base,
                            *size,
                            key,
                            Some((primary_ref.as_path(), fallbacks_ref.as_slice())),
                            retain_code,
                        )
                    };
                    if tx.send(dec).is_err() {
                        break;
                    }
                }
            });
            for ((name, base, size, _), key) in targets.iter().zip(keys.iter()) {
                let decoded = rx.recv().ok().flatten();
                let loaded_mi = decoded.and_then(|d| unsafe {
                    install_decoded_module(name, *base, *size, d, defer_probe)
                });
                if mode == 2 {
                    jobs.extend(bg_job_for_module(name, *base, *size, *key, loaded_mi));
                }
            }
        });
    }
    if !jobs.is_empty() {
        bg_enqueue(jobs);
    }
}

/// Diagnostics ([`crate::pc_sample`], [`crate::diag`]): the mapped PE image (name, base)
/// containing guest address `va`. Never blocks.
pub(crate) fn resolve_guest_va(va: usize) -> Option<(String, usize)> {
    let v = MAPPED.try_lock().ok()?;
    v.iter()
        .find(|m| va >= m.base && va < m.base + m.size)
        .map(|m| (m.name.clone(), m.base))
}

/// Diagnostics ([`crate::pc_sample`]): the module, image base and guest block IP whose live
/// AOT translation contains host address `pc` (`ip` 0: the region's shared stubs or padding).
/// Never blocks (sampled PCs may belong to a thread that holds [`DIR`]).
fn resolve_host_pc(pc: usize) -> Option<(String, usize, u64)> {
    let g = DIR.try_read().ok()?;
    let d = g.as_ref()?;
    for m in &d.modules {
        if m.gone || pc < m.region || pc >= m.region + m.region_len {
            continue;
        }
        let Some(off) = m.img_off_of_pc(pc) else {
            return Some((m.name.clone(), m.base, 0));
        };
        let bs = &m.img.blocks;
        let i = bs.partition_point(|b| b.code_off as u64 <= off);
        let mut ip = match i.checked_sub(1).map(|j| &bs[j]) {
            Some(b) if off < b.code_off as u64 + b.main as u64 * 4 => b.ip,
            _ => 0,
        };
        if ip == 0 {
            let ci = bs.partition_point(|b| b.cold_off as u64 <= off);
            if let Some(b) = ci.checked_sub(1).map(|j| &bs[j]) {
                if b.words > b.main && off < b.cold_off as u64 + (b.words - b.main) as u64 * 4 {
                    ip = b.ip;
                }
            }
        }
        return Some((m.name.clone(), m.base, ip));
    }
    None
}

// ---------------------------------------------------------------------------------------------
// Test-only harness helpers
// ---------------------------------------------------------------------------------------------

/// Test: translate the blocks at `ips` with this thread's knobs into an in-memory image,
/// run `mid` (e.g. change guest state between "build" and "load"), install it
/// (cross-checked like `AW2_DLL_AOT_VERIFY=1`) and turn AOT lookups on for this thread.
/// Returns `(live blocks, relocations, links)`.
#[cfg(test)]
pub(in crate::emu) unsafe fn test_install(ips: &[u64], mid: impl FnOnce()) -> (usize, usize, usize) {
    TEST_AOT.with(|c| c.set(true));
    let mut ips = ips.to_vec();
    ips.sort_unstable();
    ips.dedup();
    let mut bc = Box::new(BlockCache::new());
    let bcp: *mut BlockCache = &mut *bc;
    let img = build_image(bcp, &ips, BUILD_VBASE).expect("aot stubs");
    // Round-trip through the file format.
    let (lo, hi) = (ips.first().copied().unwrap_or(0), ips.last().copied().unwrap_or(0) + 1);
    let key = AotKey { image_base: lo, image_size: hi - lo, ..AotKey::default() };
    let (name, key2, img, flags) = decode(&encode("test", &key, &img, 0)).expect("aot image decodes");
    assert!(name == "test" && key2 == key && flags == 0);
    let (relocs, links) = (img.relocs.len(), img.links.len());
    mid();
    let live = install_image("test", lo as usize, (hi - lo) as usize, img, Verify::All, true)
        .map_or(0, |r| r.live);
    (live, relocs, links)
}

/// Test: revalidations run on this thread revive blocks (`on`, the default) or not.
#[cfg(test)]
pub(in crate::emu) fn test_set_revive(on: bool) {
    TEST_NO_REVIVE.with(|c| c.set(!on));
}

/// Test: import-slot relocations rebased by installs on this thread so far.
#[cfg(test)]
pub(in crate::emu) fn test_rebased() -> usize {
    TEST_REBASED.with(|c| c.get())
}

/// Test: blocks installed on this thread so far with a held import slot.
#[cfg(test)]
pub(in crate::emu) fn test_held() -> usize {
    TEST_HELD.with(|c| c.get())
}

/// True when `ip` exists in the AOT directory (or a module covering `ip` was killed) and is
/// currently dead/killed due to revalidation or code-modification invalidation.
pub(in crate::emu) fn was_killed_at(ip: u64) -> bool {
    let Ok(g) = DIR.try_read() else { return false };
    let Some(d) = g.as_ref() else { return false };
    if let Some(&(mi, bi)) = d.map.get(&ip) {
        let m = &d.modules[mi as usize];
        return m.killed || m.dead[bi as usize].load(Ordering::Relaxed) != 0;
    }
    let a = ip as usize;
    d.modules
        .iter()
        .any(|m| m.killed && a >= m.base && a - m.base < m.size)
}

/// Test: dead blocks in this thread's installed test modules.
#[cfg(test)]
pub(in crate::emu) fn test_dead() -> usize {
    let me = Some(std::thread::current().id());
    let g = DIR.read().unwrap_or_else(|e| e.into_inner());
    g.as_ref().map_or(0, |d| {
        d.modules
            .iter()
            .filter(|m| m.owner == me && !m.gone)
            .map(|m| m.dead.iter().filter(|x| x.load(Ordering::Relaxed) != 0).count())
            .sum()
    })
}

/// Test: the published probe table (what the stubs read) agrees with the directory for
/// every block of this thread's test modules: live non-host-entry owners present with their
/// code, everything else absent or 0. Returns the number of live entries checked.
#[cfg(test)]
pub(in crate::emu) fn test_probe_check() -> Result<usize, String> {
    let me = Some(std::thread::current().id());
    let g = DIR.read().unwrap_or_else(|e| e.into_inner());
    let Some(d) = g.as_ref() else { return Ok(0) };
    let base = PROBE.load(Ordering::Acquire);
    if base != d.probe.base {
        return Err(format!("published table {base:#x} is not the directory's {:#x}", d.probe.base));
    }
    let mut live = 0;
    for m in d.modules.iter().filter(|m| m.owner == me && !m.gone) {
        for b in m.img.blocks.iter() {
            let want = d.probe_val(b.ip);
            let got = if base == 0 {
                0
            } else {
                let e = unsafe { d.probe.slot(b.ip) };
                if e[0].load(Ordering::Relaxed) == b.ip {
                    e[1].load(Ordering::Relaxed)
                } else {
                    0
                }
            };
            if got != want {
                return Err(format!(
                    "block {:#x}: probe table has {got:#x}, directory says {want:#x}",
                    b.ip
                ));
            }
            live += (want != 0) as usize;
        }
    }
    Ok(live)
}

/// Test: remove this thread's test modules (their code stays mapped) and turn AOT lookups
/// off for this thread.
#[cfg(test)]
pub(in crate::emu) fn test_uninstall() {
    TEST_AOT.with(|c| c.set(false));
    let me = Some(std::thread::current().id());
    let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
    let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
    let Some(d) = g.as_mut() else { return };
    let gone: Vec<u32> = (0..d.modules.len() as u32)
        .filter(|&i| d.modules[i as usize].owner == me && !d.modules[i as usize].gone)
        .collect();
    for &i in &gone {
        d.modules[i as usize].gone = true;
    }
    let ips: Vec<u64> = d
        .map
        .iter()
        .filter(|(_, &(m, _))| gone.contains(&m))
        .map(|(&ip, _)| ip)
        .collect();
    d.map.retain(|_, &mut (m, _)| !gone.contains(&m));
    for ip in ips {
        unsafe { d.probe_sync(ip) };
    }
}

#[cfg(test)]
static TEST_REVALIDATE_CALLS: AtomicUsize = AtomicUsize::new(0);
#[cfg(test)]
static TEST_REVALIDATE_BLOCKS_CHECKED: AtomicUsize = AtomicUsize::new(0);

#[cfg(test)]
mod fast_load_tests {
    use super::*;

    extern "C" fn dummy_host_va(_c: &mut crate::callctx::CallCtx) -> u64 {
        99
    }

    /// Installing the first live AOT module must synchronize `CHECKED_VG` so the first `find()`
    /// does not run a redundant full-directory `revalidate_dir()`, while still revalidating when
    /// `va_gen()` advances after install OR between two module installs, and cleanly unlinking +
    /// reviving cross-module links via the intrusive `incoming` chain and `unlinked` list.
    #[test]
    fn install_skips_redundant_initial_revalidate_and_revalidates_on_va_gen_bump() {
        use iced_x86::code_asm::*;

        const DUMMY_VA: u64 = 0x7FFF_1234_0000;
        struct Cleanup(u64);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                jit::test_set_knobs(jit::TestKnobs::NONE);
                crate::exe::unregister_native_va(DUMMY_VA);
                crate::exe::unregister_native_va(self.0);
                test_uninstall();
            }
        }

        // Allocate small guest code buffer (fn_a -> call fn_b; fn_b -> ret).
        let mut sl = vec![0xCCu8; 0x1000];
        let base = sl.as_ptr() as u64;
        let fn_a = base + 0x100;
        let fn_b = base + 0x200;
        let _c = Cleanup(fn_b);
        jit::test_set_knobs(jit::TestKnobs { aot_probe: Some(true), ..jit::TestKnobs::NONE });

        let mut a = CodeAssembler::new(64).unwrap();
        a.call(fn_b).unwrap();
        a.add(eax, 1).unwrap();
        a.ret().unwrap();
        let ba = a.assemble(fn_a).unwrap();
        sl[0x100..0x100 + ba.len()].copy_from_slice(&ba);

        let mut a = CodeAssembler::new(64).unwrap();
        a.mov(eax, 41).unwrap();
        a.ret().unwrap();
        let bb = a.assemble(fn_b).unwrap();
        sl[0x200..0x200 + bb.len()].copy_from_slice(&bb);

        // Start from a clean directory with a fresh va_gen.
        let host_ptr = dummy_host_va as *const () as usize;
        test_uninstall();
        crate::exe::register_native_va(DUMMY_VA, host_ptr);
        crate::exe::unregister_native_va(DUMMY_VA);

        // Install module 0 (only fn_a): its link to fn_b stays in `m.unlinked`.
        let r0 = TEST_REVALIDATE_CALLS.load(Ordering::Relaxed);
        let (live0, _, links0) = unsafe { test_install(&[fn_a], || {}) };
        assert_eq!(live0, 1);
        assert!(links0 >= 1);

        // First `find()` must hit directly with ZERO redundant `revalidate_dir()` calls!
        assert!(find(fn_a).is_some(), "fn_a must be found in AOT");
        assert_eq!(
            TEST_REVALIDATE_CALLS.load(Ordering::Relaxed),
            r0,
            "first find() after install_image must not run redundant revalidate_dir()"
        );
        assert!(test_probe_check().is_ok(), "probe table must agree after batch insert");

        // Install module 1 (fn_b) at the same va_gen: must resolve module 0's unlinked call to fn_b.
        let (live1, _, _) = unsafe { test_install(&[fn_b], || {}) };
        assert_eq!(live1, 1);
        assert!(find(fn_b).is_some(), "fn_b must be found in AOT");
        assert_eq!(
            TEST_REVALIDATE_CALLS.load(Ordering::Relaxed),
            r0,
            "second module install at same va_gen must not trigger redundant revalidate_dir()"
        );
        assert!(test_probe_check().is_ok(), "probe table must agree after second module");
        {
            let g = DIR.read().unwrap();
            let d = g.as_ref().unwrap();
            assert!(
                d.has_incoming_links(fn_b),
                "cross-module link from module 0 to module 1 must populate per-block incoming_head chain"
            );
        }

        // Registering a native hook on `fn_b` bumps `va_gen()`: next `find()` must revalidate,
        // kill `fn_a` (its non-host assumption on `fn_b` fails), and unpatch module 0's link.
        crate::exe::register_native_va(fn_b, host_ptr);
        assert!(find(fn_a).is_none(), "fn_a must be killed when fn_b becomes host-callable");
        assert_eq!(
            TEST_REVALIDATE_CALLS.load(Ordering::Relaxed),
            r0 + 1,
            "va_gen bump must trigger revalidate_dir()"
        );

        // Removing the hook on `fn_b` must revive `fn_a` and re-link module 0 -> module 1!
        crate::exe::unregister_native_va(fn_b);
        assert!(find(fn_a).is_some(), "fn_a must revive after hook removal");
        assert_eq!(TEST_REVALIDATE_CALLS.load(Ordering::Relaxed), r0 + 2);
        {
            let g = DIR.read().unwrap();
            let d = g.as_ref().unwrap();
            assert!(
                d.has_incoming_links(fn_b),
                "revived module 0 must re-link its call to fn_b via m.unlinked"
            );
        }

        // If `va_gen()` bumps BEFORE installing another module while live modules exist,
        // `install_image` must NOT suppress revalidation of the pre-existing modules.
        let fn_c = base + 0x300;
        sl[0x300..0x306].copy_from_slice(&[0xB8, 0x07, 0, 0, 0, 0xC3]);
        crate::exe::register_native_va(DUMMY_VA, host_ptr);
        let (live2, _, _) = unsafe { test_install(&[fn_c], || {}) };
        assert_eq!(live2, 1);
        assert!(find(fn_c).is_some());
        assert_eq!(
            TEST_REVALIDATE_CALLS.load(Ordering::Relaxed),
            r0 + 3,
            "mixed va_gen across pre-existing live modules must still trigger revalidate_dir()"
        );
        assert!(test_probe_check().is_ok());
    }

    /// Parallel assumption checking (`check_install_blocks`), parallel initial linking with
    /// wait-free per-block `incoming_head` pushes (`link_module_initial_parallel`), complete
    /// unpatching on `kill()`, and coalesced batch probe publication (`defer_probe` +
    /// `flush_probe_table`).
    #[test]
    fn parallel_check_link_and_coalesced_probe_batch() {
        use iced_x86::code_asm::*;

        struct Cleanup(u64);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                jit::test_set_knobs(jit::TestKnobs::NONE);
                if self.0 != 0 {
                    crate::exe::unregister_native_va(self.0);
                }
                test_uninstall();
            }
        }
        jit::test_set_knobs(jit::TestKnobs { aot_probe: Some(true), ..jit::TestKnobs::NONE });
        test_uninstall();
        TEST_AOT.with(|c| c.set(true));

        // Build 64 caller blocks all calling `callee_va`, plus 16 blocks calling `hook_target_va`.
        const N_CALLERS: usize = 64;
        const N_HOOKED: usize = 16;
        let mut sl = vec![0xCCu8; 0x4000];
        let base = sl.as_ptr() as u64;
        let callee_va = base + 0x100;
        let hook_target_va = base + 0x180;
        let _c = Cleanup(hook_target_va);
        {
            let mut a = CodeAssembler::new(64).unwrap();
            a.mov(eax, 7).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(callee_va).unwrap();
            sl[0x100..0x100 + bytes.len()].copy_from_slice(&bytes);

            let mut a = CodeAssembler::new(64).unwrap();
            a.mov(eax, 9).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(hook_target_va).unwrap();
            sl[0x180..0x180 + bytes.len()].copy_from_slice(&bytes);
        }
        let mut ips = vec![callee_va, hook_target_va];
        for i in 0..N_CALLERS {
            let off = 0x200 + i * 0x20;
            let ip = base + off as u64;
            let mut a = CodeAssembler::new(64).unwrap();
            a.call(callee_va).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(ip).unwrap();
            sl[off..off + bytes.len()].copy_from_slice(&bytes);
            ips.push(ip);
        }
        for i in 0..N_HOOKED {
            let off = 0x200 + (N_CALLERS + i) * 0x20;
            let ip = base + off as u64;
            let mut a = CodeAssembler::new(64).unwrap();
            a.call(hook_target_va).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(ip).unwrap();
            sl[off..off + bytes.len()].copy_from_slice(&bytes);
            ips.push(ip);
        }
        ips.sort_unstable();
        ips.dedup();

        let mut bc = Box::new(BlockCache::new());
        let img = unsafe { build_image(&mut *bc, &ips, BUILD_VBASE) }.expect("build_image");

        // Round-trip via `decode_for_runtime(..., retain_code_if_clean=false)` so `img.code` is
        // NOT copied onto the heap and `install_image_opts` streams machine code directly from
        // `raw_code` into `MAP_JIT`, with `defer_probe = true` followed by parallel `probe_rebuild`.
        let key = AotKey {
            image_base: base,
            image_size: sl.len() as u64,
            ..AotKey::default()
        };
        let encoded = encode("par_batch", &key, &img, 0);
        let (_, _, img_nocopy, _, code_off) =
            decode_for_runtime(&encoded, false).expect("decode_for_runtime");
        assert!(
            img_nocopy.code.is_empty() && img_nocopy.code_bytes() == img.code_bytes(),
            "zero-copy runtime decode must omit heap Vec<u32> while preserving code_bytes"
        );
        let raw_code = &encoded[code_off..code_off + img_nocopy.code_bytes()];
        let inst = unsafe {
            install_image_opts(
                "par_batch",
                base as usize,
                sl.len(),
                img_nocopy,
                Some(raw_code),
                Verify::None,
                true,
                true,
            )
        }
        .expect("install_image_opts");
        assert_eq!(inst.live, ips.len());
        // Staged deferred modules must remain invisible to `find()` until `flush_probe_table()`
        // finishes linking and icache invalidation, even when `HAVE` is already true (e.g. under
        // `AW2_DLL_AOT_PROBE=0`).
        HAVE.store(true, Ordering::Release);
        assert!(
            find(callee_va).is_none(),
            "staged module with unflushed icache must stay hidden from find()"
        );
        // Verify `img_nocopy.blocks_sorted` and zero-allocation `count_missing_profile_ips`
        // (both the sorted O(log N) binary-search path and the unsorted fallback path).
        let inst_mi = inst.mi;
        let sample_profs: Vec<u32> = vec![
            0x100, // callee_va (present)
            0x180, // hook_target_va (present)
            0x200, // caller 0 (present)
            0x0F0, // missing
            0x3FF0, // missing
        ];
        assert_eq!(
            count_missing_profile_ips(inst_mi, base as usize, &sample_profs),
            2,
            "sorted binary-search profile check must count exactly 2 missing RVAs"
        );
        {
            let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
            let m = &mut g.as_mut().unwrap().modules[inst_mi];
            assert!(m.img.blocks_sorted, "decoded blocks must be detected as sorted");
            m.img.blocks_sorted = false;
        }
        assert_eq!(
            count_missing_profile_ips(inst_mi, base as usize, &sample_profs),
            2,
            "unsorted fallback profile check must agree with sorted binary-search path"
        );
        {
            let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
            g.as_mut().unwrap().modules[inst_mi].img.blocks_sorted = true;
        }

        // Flush deferred batch: builds probe table, links via ProbeTab::lookup, flushes icache,
        // and publishes PROBE + HAVE.
        unsafe { flush_probe_table() };
        assert!(find(callee_va).is_some(), "flushed module must become visible to find()");
        assert!(test_probe_check().is_ok(), "coalesced flush_probe_table must match directory");

        // Reset the installed module's link state and exercise `link_module_initial_parallel`
        // directly across 4 worker threads using `ProbeTab::lookup`, followed by `kill_batch`.
        let host_ptr = dummy_host_va as *const () as usize;
        unsafe {
            let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
            let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
            let d = g.as_mut().unwrap();
            let &(mi, callee_bi) = d.map.get(&callee_va).expect("callee in map");
            let mi = mi as usize;
            let callee_bi = callee_bi as usize;
            let region = d.modules[mi].region;
            let region_len = d.modules[mi].region_len;

            // Unpatch all existing links in `mi`, then re-link in parallel using `d.probe`!
            let _w = JitWrite::new();
            for li in 0..d.modules[mi].linked.len() {
                if d.modules[mi].linked[li] {
                    let l = d.modules[mi].img.links[li];
                    let site = d.modules[mi].host_addr(l.site_off);
                    store_b(site, d.modules[mi].host_addr(l.fallback_off));
                    d.modules[mi].linked[li] = false;
                }
            }
            d.modules[mi].unlinked.clear();
            d.modules[mi].unlinked_init = false;

            let probe_snap = ProbeTab {
                base: d.probe.base,
                cap: d.probe.cap,
                used: d.probe.used,
            };
            let mut patched = Vec::new();
            link_module_initial_parallel(
                d,
                mi,
                4,
                Some(&probe_snap),
                false,
                &[(region, region + region_len)],
                &mut patched,
            );

            // Every caller targeting `callee_va` must be linked now.
            let linked_to_callee = d.modules[mi]
                .img
                .links
                .iter()
                .enumerate()
                .filter(|&(li, l)| d.modules[mi].linked[li] && l.target == callee_va)
                .count();
            assert_eq!(
                linked_to_callee, N_CALLERS,
                "parallel probe-backed link_module_initial_parallel must patch all {N_CALLERS} caller links"
            );

            // Killing `callee_bi` via `kill_batch` must unpatch every single caller cleanly.
            let mut kill_patched = Vec::new();
            kill_batch(d, &[(mi, callee_bi)], DEAD_ASSUME, &mut kill_patched);
            assert!(!d.has_incoming_links(callee_va), "kill_batch must unpatch all incoming links");
            assert!(
                kill_patched.len() >= N_CALLERS,
                "kill_batch must unpatch all {N_CALLERS} caller sites (got {})",
                kill_patched.len()
            );

            // Now hook `hook_target_va` and verify `check_install_blocks(&m, 4)` detects all
            // `N_HOOKED` callers PLUS `hook_target_va` itself across 4 worker threads.
            crate::exe::register_native_va(hook_target_va, host_ptr);
            let (bad, _, _) = check_install_blocks(&d.modules[mi], 4);
            crate::exe::unregister_native_va(hook_target_va);
            assert_eq!(
                bad,
                N_HOOKED + 1,
                "parallel check_install_blocks must flag hook_target_va and all {N_HOOKED} callers"
            );
        }
    }

    /// Out-of-module `register_native_va` bumps (e.g. lazily registered COM vtable methods) must
    /// run incremental delta revalidation checking only `watch_blocks` instead of all blocks in
    /// untouched modules, while still invalidating & reviving external watch callers, running a
    /// full scan on in-module hooks or structural `invalidate_dispatch()`, and preserving exact
    /// `VA_BLOOM` lookup semantics across register/unregister cycles.
    #[test]
    fn delta_revalidation_skips_plain_blocks_and_bloom_matches_va_dispatch() {
        use iced_x86::code_asm::*;

        const UNRELATED_HOST_VA: u64 = 0x7FFF_5555_1000;
        const EXT_TARGET_VA: u64 = 0x7FFF_5555_2000;
        struct Cleanup(u64);
        impl Drop for Cleanup {
            fn drop(&mut self) {
                jit::test_set_knobs(jit::TestKnobs::NONE);
                crate::exe::unregister_native_va(UNRELATED_HOST_VA);
                crate::exe::unregister_native_va(EXT_TARGET_VA);
                if self.0 != 0 {
                    crate::exe::unregister_native_va(self.0);
                }
                test_uninstall();
            }
        }

        // Verify VA_BLOOM rejects unregistered VAs and tracks register/unregister cleanly.
        let host_ptr = dummy_host_va as *const () as usize;
        assert!(crate::exe::lookup_native_va(UNRELATED_HOST_VA).is_none());
        crate::exe::register_native_va(UNRELATED_HOST_VA, host_ptr);
        assert!(crate::exe::va_bloom_may_contain(UNRELATED_HOST_VA));
        assert_eq!(crate::exe::lookup_native_va(UNRELATED_HOST_VA), Some(host_ptr));
        crate::exe::unregister_native_va(UNRELATED_HOST_VA);
        assert!(crate::exe::lookup_native_va(UNRELATED_HOST_VA).is_none());

        jit::test_set_knobs(jit::TestKnobs { aot_probe: Some(true), ..jit::TestKnobs::NONE });
        test_uninstall();
        TEST_AOT.with(|c| c.set(true));

        // Build 32 plain intra-module blocks calling `in_mod_callee` + 1 watch block calling
        // external `EXT_TARGET_VA` (via indirect/direct code outside `[base, base + sl.len())`).
        const N_PLAIN: usize = 32;
        let mut sl = vec![0xCCu8; 0x2000];
        let base = sl.as_ptr() as u64;
        let in_mod_callee = base + 0x100;
        let _c = Cleanup(in_mod_callee);
        {
            let mut a = CodeAssembler::new(64).unwrap();
            a.mov(eax, 123).unwrap();
            a.ret().unwrap();
            let b = a.assemble(in_mod_callee).unwrap();
            sl[0x100..0x100 + b.len()].copy_from_slice(&b);
        }
        let mut ips = vec![in_mod_callee];
        for i in 0..N_PLAIN {
            let off = 0x200 + i * 0x20;
            let ip = base + off as u64;
            let mut a = CodeAssembler::new(64).unwrap();
            a.call(in_mod_callee).unwrap();
            a.ret().unwrap();
            let b = a.assemble(ip).unwrap();
            sl[off..off + b.len()].copy_from_slice(&b);
            ips.push(ip);
        }
        // Build an image containing all N_PLAIN + 1 intra-module blocks, plus a separate tiny
        // caller module whose sole block calls `in_mod_callee` (external to that module!).
        let ext_caller_ip = base + 0x1000;
        {
            let mut a = CodeAssembler::new(64).unwrap();
            a.call(in_mod_callee).unwrap();
            a.ret().unwrap();
            let b = a.assemble(ext_caller_ip).unwrap();
            sl[0x1000..0x1000 + b.len()].copy_from_slice(&b);
        }

        let mut bc = Box::new(BlockCache::new());
        let img = unsafe { build_image(&mut *bc, &ips, BUILD_VBASE) }.expect("build_image");
        let inst = unsafe {
            install_image("delta_plain", base as usize, 0x0F00, img, Verify::None, true)
        }
        .expect("install_image");
        assert_eq!(inst.live, N_PLAIN + 1);
        // Install the separate 1-block module whose sole block calls `in_mod_callee` (external to
        // `[base + 0x1000, base + 0x1100)`), verifying external-assumption watch blocks still get
        // killed and revived during delta revalidation.
        let ext_img =
            unsafe { build_image(&mut *bc, &[ext_caller_ip], BUILD_VBASE) }.expect("build_image");
        let ext_inst = unsafe {
            install_image("delta_ext", (base + 0x1000) as usize, 0x100, ext_img, Verify::None, true)
        }
        .expect("install_image ext");
        assert_eq!(ext_inst.live, 1);
        assert!(find(in_mod_callee).is_some());
        assert!(find(ext_caller_ip).is_some());

        // 1. Out-of-module `register_native_va(UNRELATED_HOST_VA, host_ptr)` must take the fast
        //    delta path and check 0 plain intra-module blocks!
        let b0 = TEST_REVALIDATE_BLOCKS_CHECKED.load(Ordering::Relaxed);
        crate::exe::register_native_va(UNRELATED_HOST_VA, host_ptr);
        assert!(find(in_mod_callee).is_some());
        assert!(find(ext_caller_ip).is_some());
        let checked_unrelated = TEST_REVALIDATE_BLOCKS_CHECKED.load(Ordering::Relaxed) - b0;
        assert!(
            checked_unrelated < N_PLAIN,
            "out-of-module VA registration must skip plain blocks (checked {checked_unrelated} of {})",
            N_PLAIN + 2
        );

        // 2. Hooking `in_mod_callee` must full-scan `delta_plain` AND check the external watch
        //    block `ext_caller_ip` in `delta_ext`, killing both, then reviving both on unhook.
        let b1 = TEST_REVALIDATE_BLOCKS_CHECKED.load(Ordering::Relaxed);
        crate::exe::register_native_va(in_mod_callee, host_ptr);
        assert!(find(base + 0x200).is_none(), "caller of hooked in-module callee must be killed");
        assert!(find(ext_caller_ip).is_none(), "external watch caller must be killed");
        let checked_in_mod = TEST_REVALIDATE_BLOCKS_CHECKED.load(Ordering::Relaxed) - b1;
        assert!(
            checked_in_mod >= N_PLAIN + 2,
            "in-module VA registration must full-scan delta_plain + watch delta_ext (checked {checked_in_mod})"
        );
        crate::exe::unregister_native_va(in_mod_callee);
        assert!(find(base + 0x200).is_some(), "caller must revive after unhook");
        assert!(find(ext_caller_ip).is_some(), "external watch caller must revive after unhook");

        // 3. Structural `invalidate_dispatch()` must force a full scan and rebuild `watch_blocks`,
        //    after which another out-of-module registration again skips all plain blocks!
        let b2 = TEST_REVALIDATE_BLOCKS_CHECKED.load(Ordering::Relaxed);
        crate::exe::invalidate_dispatch();
        assert!(find(in_mod_callee).is_some());
        let checked_struct = TEST_REVALIDATE_BLOCKS_CHECKED.load(Ordering::Relaxed) - b2;
        assert!(
            checked_struct >= N_PLAIN + 2,
            "structural invalidate_dispatch must full-scan all modules (checked {checked_struct})"
        );

        let b3 = TEST_REVALIDATE_BLOCKS_CHECKED.load(Ordering::Relaxed);
        crate::exe::register_native_va(EXT_TARGET_VA, host_ptr);
        assert!(find(in_mod_callee).is_some());
        let checked_after_struct = TEST_REVALIDATE_BLOCKS_CHECKED.load(Ordering::Relaxed) - b3;
        assert!(
            checked_after_struct < N_PLAIN,
            "out-of-module registration after structural scan must skip plain blocks (checked {checked_after_struct})"
        );
    }

    /// With `AW2_DLL_AOT_PROBE=0`, `PROBE` stays 0 even after live modules are published.
    /// Verify that:
    /// 1. `defer_probe` (`!HAVE && PROBE == 0`) is false once `HAVE` is already set.
    /// 2. Any staged module (`needs_icache_flush == true`) remains hidden from `find()` and
    ///    non-batch `link_pass` until `flush_probe_table()` completes icache invalidation.
    /// 3. `flush_probe_table()` with `aot_probe = false` links staged modules via `d.map`,
    ///    flushes their icache, clears `needs_icache_flush`, and exposes them to `find()`.
    #[test]
    fn deferred_staged_module_hidden_from_find_until_flushed_even_with_probe_off() {
        use iced_x86::code_asm::*;

        struct Cleanup;
        impl Drop for Cleanup {
            fn drop(&mut self) {
                jit::test_set_knobs(jit::TestKnobs::NONE);
                test_uninstall();
            }
        }
        let _c = Cleanup;
        jit::test_set_knobs(jit::TestKnobs { aot_probe: Some(false), ..jit::TestKnobs::NONE });
        test_uninstall();
        TEST_AOT.with(|c| c.set(true));

        let mut sl = vec![0xCCu8; 0x2000];
        let base = sl.as_ptr() as u64;
        let live_caller = base + 0x100;
        let staged_callee = base + 0x200;
        let staged_caller = base + 0x300;
        {
            let mut a = CodeAssembler::new(64).unwrap();
            a.call(staged_callee).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(live_caller).unwrap();
            sl[0x100..0x100 + bytes.len()].copy_from_slice(&bytes);

            let mut a = CodeAssembler::new(64).unwrap();
            a.mov(eax, 123).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(staged_callee).unwrap();
            sl[0x200..0x200 + bytes.len()].copy_from_slice(&bytes);

            let mut a = CodeAssembler::new(64).unwrap();
            a.call(staged_callee).unwrap();
            a.ret().unwrap();
            let bytes = a.assemble(staged_caller).unwrap();
            sl[0x300..0x300 + bytes.len()].copy_from_slice(&bytes);
        }

        // 1. Install `mod_live` immediately (non-deferred) with probe disabled.
        let mut bc = Box::new(BlockCache::new());
        let img_live = unsafe { build_image(&mut *bc, &[live_caller], BUILD_VBASE) }.unwrap();
        let inst_live = unsafe {
            install_image_opts(
                "probe_off_live",
                base as usize,
                0x180,
                img_live,
                None,
                Verify::None,
                true,
                false,
            )
        }
        .expect("install mod_live");
        assert_eq!(inst_live.live, 1);
        assert!(HAVE.load(Ordering::Acquire), "HAVE must be set after publishing mod_live");
        assert!(
            !(!HAVE.load(Ordering::Acquire) && PROBE.load(Ordering::Acquire) == 0),
            "subsequent activation must not defer once HAVE is already true"
        );
        assert!(find(live_caller).is_some(), "live_caller must be visible");

        // 2. Stage `mod_staged` with `defer_probe = true` while `HAVE` is already true:
        //    its blocks must stay hidden from `find()` and non-batch `link_pass` until flushed.
        let img_staged =
            unsafe { build_image(&mut *bc, &[staged_callee, staged_caller], BUILD_VBASE) }.unwrap();
        let inst_staged = unsafe {
            install_image_opts(
                "probe_off_staged",
                (base + 0x200) as usize,
                0x200,
                img_staged,
                None,
                Verify::None,
                true,
                true,
            )
        }
        .expect("stage mod_staged");
        assert_eq!(inst_staged.live, 2);
        assert!(
            find(staged_callee).is_none() && find(staged_caller).is_none(),
            "staged module with unflushed icache must stay hidden from find()"
        );
        {
            let _r = REVALIDATE.lock().unwrap_or_else(|e| e.into_inner());
            let mut g = DIR.write().unwrap_or_else(|e| e.into_inner());
            let d = g.as_mut().unwrap();
            let patched = unsafe {
                let _w = JitWrite::new();
                link_pass(d, None)
            };
            assert!(
                patched.is_empty() && !d.has_incoming_links(staged_callee),
                "non-batch link_pass must not link into an unflushed staged module"
            );
        }

        // 3. Flush the batch with `aot_probe = false`: must link both intra-staged and
        //    cross-module calls via `d.map`, flush icache, clear `needs_icache_flush`,
        //    and expose the blocks to `find()`.
        unsafe { flush_probe_table() };
        assert!(
            find(staged_callee).is_some() && find(staged_caller).is_some(),
            "flushed module must become visible to find() even with AW2_DLL_AOT_PROBE=0"
        );
        {
            let g = DIR.read().unwrap_or_else(|e| e.into_inner());
            let d = g.as_ref().unwrap();
            assert!(
                d.has_incoming_links(staged_callee),
                "flush_probe_table with probe off must still link callers to staged_callee"
            );
            assert!(
                !d.modules[inst_staged.mi].needs_icache_flush,
                "needs_icache_flush must be cleared after flush_probe_table"
            );
        }
    }
}
