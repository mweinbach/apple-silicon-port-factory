//! Parallel AOT block assembly, multi-bank chunk packing, profile-free offline prewarming
//! (`aot_prewarm_all_mapped`), profile-driven builds (`build_all`), and offline differential
//! verification batch execution (`VerifyAotBatch`).

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

use super::bg::{delete_other_keys, read_tsv};
use super::cfg_discover::{discover_pe_roots, discover_with_threads};
use super::format::{
    cache_file, encode, module_key, profile_file, write_atomic, Image, AOT_BANK_SPAN, BUILD_VBASE,
    MAX_CODE,
};
use super::runtime::{reach, store_b, try_patch_cond_branch, JitWrite, MAPPED};
use super::{
    aot_closure_depth, cache_dir, fallback_cache_dirs, prewarm_threads, PREWARMING,
};
use crate::emu::exec_mem::{alloc_jit, free_jit, icache_invalidate, jit_writable};
use crate::emu::jit::{self, AotStubOffs, AOT_STUBS_BYTES};
use crate::emu::BlockCache;

/// Virtual block offset from `vbase` used during parallel translation. Every stub sits in
/// `[0, AOT_STUBS_BYTES)` (`4 KiB`) and every block is at most `AOT_MAX_BLOCK_BYTES`
/// (`256 KiB`), so `64 MiB` cleanly separates PC-relative `b`/`bl` branches to shared stubs
/// from intra-block branches while staying well inside ARM64's ±128 MiB branch range.
const PARALLEL_VOFF: usize = 64 << 20;
/// Minimum candidate blocks before spawning parallel worker threads for block translation.
pub(super) const PARALLEL_MIN_BLOCKS: usize = 64;
/// Stack size for parallel prewarm / build worker threads (macOS default is 512 KiB).
pub(super) const PARALLEL_WORKER_STACK: usize = 8 << 20;

/// Reusable per-worker [`BlockCache`] pool for parallel offline prewarming / builds.
/// Each [`BlockCache`] reserves ~74 MiB of lazily-committed virtual memory; reusing one
/// pool across BFS levels, translation, and all mapped modules avoids repeated `mmap`/
/// `munmap` TLB shootdowns while giving every worker exclusive `&mut BlockCache` ownership.
pub(super) struct WorkerCaches(Vec<Box<BlockCache>>);

impl WorkerCaches {
    pub(super) fn new() -> Self {
        Self(Vec::new())
    }
    pub(super) fn get_one(&mut self) -> *mut BlockCache {
        if self.0.is_empty() {
            self.0.push(Box::new(BlockCache::new()));
        }
        &mut *self.0[0]
    }
    pub(super) fn ensure(&mut self, n: usize) -> &mut [Box<BlockCache>] {
        while self.0.len() < n {
            self.0.push(Box::new(BlockCache::new()));
        }
        &mut self.0[..n]
    }
}

/// Rebase a block's PC-relative instructions when placing its hot main path `[0, main)` at
/// `hot_off` and its cold out-of-line tail `[main, words.len())` at `cold_off` inside stub
/// bank `target_bank_off` (translated originally as a single contiguous block at `from_off`
/// targeting stubs at `from_bank_off`). Only instruction words in `[0, b.cold_code_end)` are
/// scanned; literal-pool data words are untouched.
pub(super) fn split_and_rebase_block(
    b: &mut jit::AotBlock,
    from_off: usize,
    from_bank_off: usize,
    hot_off: usize,
    cold_off: usize,
    target_bank_off: usize,
    offs: &AotStubOffs,
) {
    let from_rel_word = ((from_off - from_bank_off) / 4) as i64;
    let hot_word = (hot_off / 4) as i64;
    let cold_word = (cold_off / 4) as i64;
    let target_bank_word = (target_bank_off / 4) as i64;
    let main = b.main as usize;
    let delta = cold_word - (hot_word + main as i64);
    let stubs_words = (AOT_STUBS_BYTES / 4) as i64;
    let scan_end = (b.cold_code_end as usize).min(b.words.len());
    if from_off - from_bank_off == hot_off - target_bank_off && delta == 0 {
        return;
    }
    for i in 0..scan_end {
        let w = b.words[i];
        let in_hot = i < main;
        if (w & 0x7C00_0000) == 0x1400_0000 {
            // ARM64 B or BL <imm26>.
            let imm26 = ((((w & 0x03FF_FFFF) as i32) << 6) >> 6) as i64;
            let local_t = i as i64 + imm26;
            let stub_t = from_rel_word + local_t;
            if (0..stubs_words).contains(&stub_t)
                && offs.iter().any(|&st| st != u32::MAX && st as i64 == stub_t)
            {
                let new_pc = if in_hot {
                    hot_word + i as i64
                } else {
                    cold_word + (i - main) as i64
                };
                let new_off = (target_bank_word + stub_t) - new_pc;
                debug_assert!(
                    (-(1i64 << 25)..(1i64 << 25)).contains(&new_off),
                    "imm26 stub branch overflow {new_off}"
                );
                b.words[i] = (w & 0xFC00_0000) | ((new_off as u32) & 0x03FF_FFFF);
            } else if delta != 0 {
                if in_hot && local_t >= main as i64 {
                    let new_off = imm26 + delta;
                    b.words[i] = (w & 0xFC00_0000) | ((new_off as u32) & 0x03FF_FFFF);
                } else if !in_hot && local_t < main as i64 {
                    let new_off = imm26 - delta;
                    b.words[i] = (w & 0xFC00_0000) | ((new_off as u32) & 0x03FF_FFFF);
                }
            }
        } else if delta != 0
            && ((w & 0xFF00_0010) == 0x5400_0000
                || (w & 0x7E00_0000) == 0x3400_0000
                || (w & 0xFF00_0000) == 0x5800_0000
                || (w & 0xFF00_0000) == 0x9C00_0000)
        {
            // ARM64 B.cond / CBZ / CBNZ / LDR Xt,lit / LDR Qt,lit (<imm19> in bits 23..5).
            let imm19 = ((((w >> 5) & 0x7_FFFF) as i32) << 13) >> 13;
            let local_t = i as i64 + imm19 as i64;
            let new_off = if in_hot && local_t >= main as i64 {
                Some(imm19 as i64 + delta)
            } else if !in_hot && local_t < main as i64 {
                Some(imm19 as i64 - delta)
            } else {
                None
            };
            if let Some(off) = new_off {
                debug_assert!((-(1i64 << 18)..(1i64 << 18)).contains(&off), "imm19 overflow {off}");
                b.words[i] = (w & 0xFF00_001F) | (((off as u32) & 0x7_FFFF) << 5);
            }
        } else if delta != 0 && (w & 0x7E00_0000) == 0x3600_0000 {
            // ARM64 TBZ / TBNZ (<imm14> in bits 18..5, ±32 KiB).
            let imm14 = ((((w >> 5) & 0x3FFF) as i32) << 18) >> 18;
            let local_t = i as i64 + imm14 as i64;
            let new_off = if in_hot && local_t >= main as i64 {
                Some(imm14 as i64 + delta)
            } else if !in_hot && local_t < main as i64 {
                Some(imm14 as i64 - delta)
            } else {
                None
            };
            if let Some(off) = new_off {
                debug_assert!((-(1i64 << 13)..(1i64 << 13)).contains(&off), "imm14 overflow {off}");
                b.words[i] = (w & 0xFFF8_001F) | (((off as u32) & 0x3FFF) << 5);
            }
        }
    }
}

/// Chunked multi-bank hot/cold assembler shared by [`build_image`] and
/// [`build_image_with_threads`] so single-threaded and multi-threaded AOT builds produce
/// bit-for-bit identical images across arbitrarily large binaries.
struct ChunkAssembler {
    pending: Vec<(u64, usize, jit::AotBlock)>,
    hot_w: usize,
    cold_w: usize,
    cur_bank_off: usize,
    bank_span: usize,
    max_code: usize,
}

impl ChunkAssembler {
    fn with_limits(bank_span: usize, max_code: usize) -> Self {
        Self {
            pending: Vec::new(),
            hot_w: 0,
            cold_w: 0,
            cur_bank_off: 0,
            bank_span,
            max_code,
        }
    }

    fn push(
        &mut self,
        img: &mut Image,
        offs: &AotStubOffs,
        ip: u64,
        from_off: usize,
        b: jit::AotBlock,
    ) -> bool {
        const MAX_CHUNK_HOT: usize = 3072; // 12 KiB of dense hot code per chunk
        const MAX_CHUNK_SPAN: usize = 7000; // < 8192 words (±32 KiB tbz/tbnz limit)
        let main = b.main as usize;
        let cold = (b.words.len() - main) + 3;
        if !self.pending.is_empty()
            && (self.hot_w + main > MAX_CHUNK_HOT
                || self.hot_w + self.cold_w + main + cold > MAX_CHUNK_SPAN)
        {
            self.flush(img, offs);
        }
        // Worst-case bytes added by the pending chunk + this block + 16B chunk trailing pad.
        let est_after =
            AOT_STUBS_BYTES + img.code_bytes() + (self.hot_w + self.cold_w + main + cold + 3) * 4;
        if est_after > self.cur_bank_off + self.bank_span {
            self.flush(img, offs);
            let after_flush = AOT_STUBS_BYTES + img.code_bytes();
            let block_bytes = (main + cold + 3) * 4;
            // Only open a new bank if the current bank actually contains code (prevents empty banks).
            if after_flush > self.cur_bank_off + AOT_STUBS_BYTES {
                if after_flush + AOT_STUBS_BYTES + block_bytes > self.max_code {
                    log::warn!(
                        "emu aot: code limit ({} MiB) reached at {ip:#x}; remaining blocks left to the JIT",
                        self.max_code >> 20
                    );
                    return false;
                }
                self.cur_bank_off = img.start_new_bank();
            }
        }
        let est_after =
            AOT_STUBS_BYTES + img.code_bytes() + (self.hot_w + self.cold_w + main + cold + 3) * 4;
        if est_after > self.max_code {
            self.flush(img, offs);
            log::warn!(
                "emu aot: code limit ({} MiB) reached at {ip:#x}; remaining blocks left to the JIT",
                self.max_code >> 20
            );
            return false;
        }
        self.hot_w += main;
        self.cold_w += cold;
        self.pending.push((ip, from_off, b));
        true
    }

    fn flush(&mut self, img: &mut Image, offs: &AotStubOffs) {
        if !self.pending.is_empty() {
            let chunk = core::mem::take(&mut self.pending);
            img.push_chunk(chunk, offs, self.cur_bank_off);
            self.hot_w = 0;
            self.cold_w = 0;
        }
    }
}

/// Translate `ips` (sorted, deduplicated) in AOT emission mode into an image laid out for a
/// region at `vbase`. Blocks the translator rejects are skipped (the JIT handles them).
#[cfg_attr(not(test), allow(dead_code))]
pub(super) unsafe fn build_image(bc: *mut BlockCache, ips: &[u64], vbase: usize) -> Option<Image> {
    build_image_with_limits(bc, ips, vbase, AOT_BANK_SPAN, MAX_CODE)
}

pub(super) unsafe fn build_image_with_limits(
    bc: *mut BlockCache,
    ips: &[u64],
    vbase: usize,
    bank_span: usize,
    max_code: usize,
) -> Option<Image> {
    let mut stub_buf = vec![0u32; AOT_STUBS_BYTES / 4];
    let offs = jit::aot_emit_stubs(stub_buf.as_mut_ptr(), vbase)?;
    let mut img = Image { stubs: offs, ..Image::default() };
    let mut asm = ChunkAssembler::with_limits(bank_span, max_code);
    let mut scratch = Vec::new();
    let mut tr_scratch = jit::TrScratch::new();
    for &ip in ips {
        if let Some(b) = jit::aot_translate(
            bc,
            ip,
            vbase + PARALLEL_VOFF,
            vbase,
            &offs,
            &mut scratch,
            &mut tr_scratch,
        ) {
            if !asm.push(&mut img, &offs, ip, PARALLEL_VOFF, b) {
                break;
            }
        }
    }
    asm.flush(&mut img, &offs);
    Some(img)
}

#[cfg(test)]
pub(in crate::emu) unsafe fn build_image_for_test(bc: *mut BlockCache, ips: &[u64]) -> Option<Image> {
    build_image(bc, ips, BUILD_VBASE)
}

#[cfg(test)]
pub(in crate::emu) unsafe fn build_image_for_test_with_limits(
    bc: *mut BlockCache,
    ips: &[u64],
    bank_span: usize,
    max_code: usize,
) -> Option<Image> {
    build_image_with_limits(bc, ips, BUILD_VBASE, bank_span, max_code)
}

/// Translate `ips` (sorted, deduplicated) in AOT emission mode using up to `threads` worker
/// threads, assembling blocks in exact `ips` order so output is bit-for-bit identical to
/// [`build_image`] regardless of thread count.
///
/// Workers never touch executable pages: each worker exclusively borrows a [`BlockCache`]
/// from `pool` and a private heap scratch buffer, translates chunks at [`PARALLEL_VOFF`],
/// and the main thread rebases stub branches to each block's deterministic stub bank via
/// [`split_and_rebase_block`].
pub(super) unsafe fn build_image_with_threads(
    pool: &mut WorkerCaches,
    ips: &[u64],
    vbase: usize,
    threads: usize,
) -> Option<Image> {
    build_image_with_threads_and_limits(pool, ips, vbase, threads, AOT_BANK_SPAN, MAX_CODE)
}

pub(super) unsafe fn build_image_with_threads_and_limits(
    pool: &mut WorkerCaches,
    ips: &[u64],
    vbase: usize,
    threads: usize,
    bank_span: usize,
    max_code: usize,
) -> Option<Image> {
    if threads <= 1 || ips.len() < 2 {
        return build_image_with_limits(pool.get_one(), ips, vbase, bank_span, max_code);
    }

    let mut stub_buf = vec![0u32; AOT_STUBS_BYTES / 4];
    let offs = jit::aot_emit_stubs(stub_buf.as_mut_ptr(), vbase)?;
    // Ensure process-wide codegen knob OnceLocks are initialized on the caller thread
    // before spawning workers.
    let _ = jit::aot_knob_bits();

    let workers = threads.min(ips.len());
    let chunk_len = (ips.len() / (workers * 8)).clamp(1, 256);
    let chunks: Vec<&[u64]> = ips.chunks(chunk_len).collect();
    let n_chunks = chunks.len();
    let next_chunk = AtomicUsize::new(0);
    let worker_bcs = pool.ensure(workers);

    let mut all_chunks: Vec<(usize, Vec<(u64, jit::AotBlock)>)> = Vec::with_capacity(n_chunks);
    std::thread::scope(|scope| {
        let mut handles = Vec::with_capacity(workers);
        for (wi, local_bc) in worker_bcs.iter_mut().enumerate() {
            let chunks_ref = &chunks;
            let next_ref = &next_chunk;
            let h = std::thread::Builder::new()
                .name(format!("port-aot-w{wi}"))
                .stack_size(PARALLEL_WORKER_STACK)
                .spawn_scoped(scope, move || {
                    let bcp: *mut BlockCache = &mut **local_bc;
                    let mut scratch = Vec::with_capacity(jit::AOT_MAX_BLOCK_BYTES / 4);
                    let mut tr_scratch = jit::TrScratch::new();
                    let mut done: Vec<(usize, Vec<(u64, jit::AotBlock)>)> = Vec::new();
                    loop {
                        let ci = next_ref.fetch_add(1, Ordering::Relaxed);
                        if ci >= n_chunks {
                            break;
                        }
                        let slice = chunks_ref[ci];
                        let mut out = Vec::with_capacity(slice.len());
                        for &ip in slice {
                            if let Some(b) = jit::aot_translate(
                                bcp,
                                ip,
                                vbase + PARALLEL_VOFF,
                                vbase,
                                &offs,
                                &mut scratch,
                                &mut tr_scratch,
                            ) {
                                out.push((ip, b));
                            }
                        }
                        done.push((ci, out));
                    }
                    done
                })
                .expect("spawn parallel AOT worker");
            handles.push(h);
        }
        for h in handles {
            all_chunks.extend(h.join().expect("parallel AOT worker panicked"));
        }
    });

    all_chunks.sort_unstable_by_key(|(ci, _)| *ci);
    let mut img = Image { stubs: offs, ..Image::default() };
    let mut asm = ChunkAssembler::with_limits(bank_span, max_code);
    'assemble: for (_, blocks) in all_chunks {
        for (ip, b) in blocks {
            if !asm.push(&mut img, &offs, ip, PARALLEL_VOFF, b) {
                break 'assemble;
            }
        }
    }
    asm.flush(&mut img, &offs);
    Some(img)
}

/// Outcome of profile-free offline `.aot` block-cache precompilation for one mapped PE module.
#[derive(Debug, Clone)]
pub struct AotPrewarmReport {
    pub module: String,
    pub roots: usize,
    pub blocks: usize,
    pub code_bytes: usize,
    pub cache_path: Option<PathBuf>,
    pub skipped_reason: Option<String>,
}

/// Profile-free offline `.aot` block-cache precompiler for every mapped PE module registered
/// via [`super::module_mapped`]. Discovers function roots directly from each mapped PE image
/// (`.pdata` `RUNTIME_FUNCTION` entries, `AddressOfEntryPoint`, and Export Directory code
/// RVAs), expands each root with the static CFG closure (`PORT_AOT_CLOSURE` /
/// `AW2_DLL_AOT_CLOSURE`, default 128), translates via `jit`'s AOT emitter, and writes the
/// standard `<cache>/<module>-<key>.aot` file loaded by [`super::activate`].
pub fn aot_prewarm_all_mapped(max_roots_per_module: usize) -> Vec<AotPrewarmReport> {
    struct PrewarmGuard(bool);
    impl Drop for PrewarmGuard {
        fn drop(&mut self) {
            PREWARMING.store(self.0, Ordering::Release);
        }
    }
    let _guard = PrewarmGuard(PREWARMING.swap(true, Ordering::AcqRel));

    let mapped: Vec<(String, usize, usize)> = match MAPPED.lock() {
        Ok(v) => v.iter().map(|m| (m.name.clone(), m.base, m.size)).collect(),
        Err(_) => return Vec::new(),
    };
    if mapped.is_empty() {
        return Vec::new();
    }

    let depth = aot_closure_depth();
    let threads = prewarm_threads();
    let mut pool = WorkerCaches::new();
    let mut reports = Vec::with_capacity(mapped.len());

    for (name, base, size) in mapped {
        if crate::exe::aw2_registry_active() && base == crate::exe::base() {
            log::info!(
                "emu aot: prewarm {name}: skipped (native replacement registry active for exe)"
            );
            reports.push(AotPrewarmReport {
                module: name,
                roots: 0,
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("native replacement registry active for exe".into()),
            });
            continue;
        }

        let mut roots = unsafe { discover_pe_roots(base, size, 0) };
        let replayed = unsafe { super::callbacks::load_replayed_callback_roots(&name, base, size) };
        let replayed_count = replayed.len();
        if !replayed.is_empty() {
            roots.extend(replayed.into_iter().map(|c| c.rva));
            roots.sort_unstable();
            roots.dedup();
        }
        if max_roots_per_module > 0 && roots.len() > max_roots_per_module {
            roots.truncate(max_roots_per_module);
        }
        if roots.is_empty() {
            log::info!("emu aot: prewarm {name}: no executable code roots found; skipped");
            reports.push(AotPrewarmReport {
                module: name,
                roots: 0,
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("no executable code roots found in PE".into()),
            });
            continue;
        }
        if replayed_count > 0 {
            log::info!(
                "emu aot: prewarm {name}: merged {replayed_count} runtime-replayed callback root(s) by image fingerprint"
            );
        }

        let key = unsafe { module_key(base, size) };
        let t0 = std::time::Instant::now();
        let (ips, cl) =
            unsafe { discover_with_threads(&mut pool, base, size, &roots, depth, threads) };
        if ips.is_empty() {
            reports.push(AotPrewarmReport {
                module: name,
                roots: roots.len(),
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("no translatable blocks discovered".into()),
            });
            continue;
        }
        let build_threads = if ips.len() >= PARALLEL_MIN_BLOCKS { threads } else { 1 };
        let Some(img) =
            (unsafe { build_image_with_threads(&mut pool, &ips, BUILD_VBASE, build_threads) })
        else {
            log::error!("emu aot: prewarm {name}: shared stubs don't fit; not built");
            reports.push(AotPrewarmReport {
                module: name,
                roots: roots.len(),
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("shared stubs did not fit".into()),
            });
            continue;
        };
        if img.blocks.is_empty() {
            reports.push(AotPrewarmReport {
                module: name,
                roots: roots.len(),
                blocks: 0,
                code_bytes: 0,
                cache_path: None,
                skipped_reason: Some("0 blocks translated".into()),
            });
            continue;
        }
        let path = cache_file(&name, &key);
        match write_atomic(&path, &encode(&name, &key, &img, 0)) {
            Ok(()) => {
                let evicted = delete_other_keys(&cache_dir(), &name, &path);
                log::info!(
                    "emu aot: prewarm {name}: built {} blocks ({} roots, {} candidates; closure depth {depth}: {} jump tables / {} targets, {} leaf callees, {} lea leaves, {} cross-function successors skipped, .pdata {} functions, {} chained fragments merged, {} broken; {} bank(s), {build_threads} thread(s)), {} KiB code, {} links, {} relocs, {} assumptions in {:.2}s -> {} ({evicted} cache(s) of other keys removed)",
                    img.blocks.len(),
                    roots.len(),
                    ips.len(),
                    cl.jt_tables,
                    cl.jt_targets,
                    cl.leaf_callees,
                    cl.lea_leaves,
                    cl.cross,
                    cl.functions,
                    cl.chained,
                    cl.broken,
                    img.stub_banks().len(),
                    img.code_bytes() >> 10,
                    img.links.len(),
                    img.relocs.len(),
                    img.assume.len(),
                    t0.elapsed().as_secs_f64(),
                    path.display(),
                );
                reports.push(AotPrewarmReport {
                    module: name,
                    roots: roots.len(),
                    blocks: img.blocks.len(),
                    code_bytes: img.code_bytes(),
                    cache_path: Some(path),
                    skipped_reason: None,
                });
            }
            Err(e) => {
                log::error!("emu aot: prewarm {name}: writing {}: {e}", path.display());
                reports.push(AotPrewarmReport {
                    module: name,
                    roots: roots.len(),
                    blocks: img.blocks.len(),
                    code_bytes: img.code_bytes(),
                    cache_path: None,
                    skipped_reason: Some(format!("write failed: {e}")),
                });
            }
        }
    }

    reports
}

/// Profile RVAs of a module (most frequent first), or empty.
pub(super) fn read_profile(name: &str, image_hash: &[u64; 2]) -> Vec<u32> {
    let mut map = read_tsv(&profile_file(name, image_hash));
    if map.is_empty() {
        for fb_dir in fallback_cache_dirs() {
            let fb = fb_dir
                .join("profile")
                .join(format!("{name}-{:016x}.tsv", image_hash[0]));
            let candidate = read_tsv(&fb);
            if !candidate.is_empty() {
                map = candidate;
                break;
            }
        }
    }
    let mut v: Vec<(u32, u64)> = map.into_iter().collect();
    v.sort_unstable_by_key(|&(rva, n)| (core::cmp::Reverse(n), rva));
    v.into_iter().map(|(rva, _)| rva).collect()
}

/// Build the caches of the selected pending modules (`PORT_AOT_BUILD` / `AW2_DLL_AOT_BUILD`).
pub(super) unsafe fn build_all(pending: &[(String, usize, usize)], sel: &[String]) {
    let depth = aot_closure_depth();
    let threads = prewarm_threads();
    let mut pool = WorkerCaches::new();
    for (name, base, size) in pending {
        if !sel.is_empty() && !sel.iter().any(|s| name.contains(s.as_str())) {
            continue;
        }
        let (base, size) = (*base, *size);
        let key = module_key(base, size);
        let prof = read_profile(name, &key.image_hash);
        if prof.is_empty() {
            log::info!(
                "emu aot: {name}: no profile ({}); skipped",
                profile_file(name, &key.image_hash).display()
            );
            continue;
        }
        let t0 = std::time::Instant::now();
        let (ips, cl) = discover_with_threads(&mut pool, base, size, &prof, depth, threads);
        let build_threads = if ips.len() >= PARALLEL_MIN_BLOCKS { threads } else { 1 };
        let Some(img) = build_image_with_threads(&mut pool, &ips, BUILD_VBASE, build_threads) else {
            log::error!("emu aot: {name}: shared stubs don't fit; not built");
            continue;
        };
        let path = cache_file(name, &key);
        match write_atomic(&path, &encode(name, &key, &img, 0)) {
            Ok(()) => log::info!(
                "emu aot: {name}: built {} blocks ({} profiled, {} candidates; closure depth {depth}: {} jump tables / {} targets, {} leaf callees, {} lea leaves, {} cross-function successors skipped, .pdata {} functions, {} chained fragments merged, {} broken; {} bank(s), {build_threads} thread(s)), {} KiB code, {} links, {} relocs, {} assumptions in {:.2}s -> {} ({} cache(s) of other keys removed)",
                img.blocks.len(),
                prof.len(),
                ips.len(),
                cl.jt_tables,
                cl.jt_targets,
                cl.leaf_callees,
                cl.lea_leaves,
                cl.cross,
                cl.functions,
                cl.chained,
                cl.broken,
                img.stub_banks().len(),
                img.code_bytes() >> 10,
                img.links.len(),
                img.relocs.len(),
                img.assume.len(),
                t0.elapsed().as_secs_f64(),
                path.display(),
                delete_other_keys(&cache_dir(), name, &path)
            ),
            Err(e) => log::error!("emu aot: {name}: writing {}: {e}", path.display()),
        }
    }
}

/// Self-contained in-memory AOT region for offline differential verification of real PE blocks
/// (exercises `build_image_with_threads`, `ChunkAssembler`, `split_and_rebase_block`,
/// `aot_emit_stubs`, and intra-batch `link_pass` including fallthrough-`nop` and `cond_patch`).
pub(in crate::emu) struct VerifyAotBatch {
    region: usize,
    region_len: usize,
    bank_shifts: Vec<usize>,
    img: Image,
    /// `ip -> (bank_enter_stub, block_code_addr, block_idx)`
    blocks: HashMap<u64, (usize, usize, usize)>,
}

impl VerifyAotBatch {
    pub(in crate::emu) unsafe fn build(ips: &[u64], threads: usize) -> Option<Self> {
        if ips.is_empty() || !jit_writable() {
            return None;
        }
        let mut sorted = ips.to_vec();
        sorted.sort_unstable();
        sorted.dedup();
        let mut pool = WorkerCaches::new();
        let build_threads = if sorted.len() >= PARALLEL_MIN_BLOCKS {
            threads.max(1)
        } else {
            1
        };
        let img = build_image_with_threads(&mut pool, &sorted, BUILD_VBASE, build_threads)?;
        let banks_snap: Vec<(usize, usize)> = img
            .stub_banks()
            .iter()
            .enumerate()
            .map(|(bi, &bo)| (bo as usize, img.bank_end(bi)))
            .collect();
        let nbanks = banks_snap.len();
        let mut bank_shifts = Vec::with_capacity(nbanks);
        let mut bank_rt_offs = Vec::with_capacity(nbanks);
        let mut cursor = 0usize;
        for &(bo, b_end) in &banks_snap {
            cursor = (cursor + 15) & !15;
            bank_rt_offs.push(cursor);
            bank_shifts.push(cursor - bo);
            cursor += b_end - bo;
        }
        let len = (cursor + 0x3FFF) & !0x3FFF;
        let region = alloc_jit(len)? as usize;
        {
            let _w = JitWrite::new();
            for bi in 0..nbanks {
                let rt_base = region + bank_rt_offs[bi];
                if jit::aot_emit_stubs(rt_base as *mut u32, rt_base) != Some(img.stubs) {
                    drop(_w);
                    free_jit(region as *mut u8, len);
                    return None;
                }
                let code_start_off = banks_snap[bi].0 + AOT_STUBS_BYTES;
                let code_end_off = banks_snap[bi].1;
                if code_end_off > code_start_off {
                    let word_lo = (code_start_off - AOT_STUBS_BYTES) / 4;
                    let word_len = (code_end_off - code_start_off) / 4;
                    core::ptr::copy_nonoverlapping(
                        img.code.as_ptr().add(word_lo),
                        (rt_base + AOT_STUBS_BYTES) as *mut u32,
                        word_len,
                    );
                }
            }
        }
        icache_invalidate(region as *mut u8, len);
        let mut blocks = HashMap::with_capacity(img.blocks.len());
        for (bi, b) in img.blocks.iter().enumerate() {
            let bank_idx = img.bank_index_of(b.code_off);
            let shift = bank_shifts[bank_idx];
            let bank_rt_base = region + bank_rt_offs[bank_idx];
            let enter = bank_rt_base + img.stubs[0] as usize * 4;
            let code = region + b.code_off as usize + shift;
            blocks.insert(b.ip, (enter, code, bi));
        }
        Some(Self { region, region_len: len, bank_shifts, img, blocks })
    }

    #[inline]
    fn host_addr(&self, off: u32) -> usize {
        let shift = if self.bank_shifts.len() <= 1 {
            0
        } else {
            self.bank_shifts[self.img.bank_index_of(off)]
        };
        self.region + off as usize + shift
    }

    #[inline]
    pub(in crate::emu) fn block_entry(&self, ip: u64) -> Option<(usize, usize)> {
        self.blocks.get(&ip).map(|&(enter, code, _)| (enter, code))
    }

    #[inline]
    pub(in crate::emu) fn contains(&self, ip: u64) -> bool {
        self.blocks.contains_key(&ip)
    }

    /// Apply intra-batch `link_pass` (direct `b`/`bl`, fallthrough `nop`, and `try_patch_cond_branch`).
    /// Returns `(total_linked, fallthrough_nops, cond_branches_patched)`.
    pub(in crate::emu) unsafe fn apply_intra_links(&mut self) -> (usize, usize, usize) {
        let mut total = 0usize;
        let mut nops = 0usize;
        let mut conds = 0usize;
        {
            let _w = JitWrite::new();
            for b in &self.img.blocks {
                let lo = b.link_lo as usize;
                let hi = lo + b.link_n as usize;
                for li in lo..hi {
                    let l = self.img.links[li];
                    let Some(&(_, code, tb)) = self.blocks.get(&l.target) else {
                        continue;
                    };
                    if self.img.blocks[tb].host_entry {
                        continue;
                    }
                    let site = self.host_addr(l.site_off);
                    if !reach(site, code) {
                        continue;
                    }
                    let cell = &*(site as *const AtomicU32);
                    let old = cell.load(Ordering::Relaxed);
                    if code == site + 4 && (old & 0xFC00_0000) == 0x1400_0000 {
                        cell.store(0xD503_201F, Ordering::Relaxed);
                        nops += 1;
                    } else {
                        store_b(site, code);
                    }
                    if l.cond_off != u32::MAX {
                        let cond_site = self.host_addr(l.cond_off);
                        if try_patch_cond_branch(cond_site, code).is_some() {
                            conds += 1;
                        }
                    }
                    total += 1;
                }
            }
        }
        icache_invalidate(self.region as *mut u8, self.region_len);
        (total, nops, conds)
    }
}

impl Drop for VerifyAotBatch {
    fn drop(&mut self) {
        unsafe { free_jit(self.region as *mut u8, self.region_len) };
    }
}
