//! PE32+ root discovery (`.pdata`, EntryPoint, Exports, TLS callbacks, Load Config CFG,
//! DIR64 Base Relocations) and parallel static CFG closure.

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::builder::{WorkerCaches, PARALLEL_WORKER_STACK};
use super::format::{data_sections, exec_sections};
use super::jumptable::{
    analyze_block, is_clean_code_entry, is_plausible_code_entry, merge_jt_regs, valid_code_target,
    BlockStep, EdgeKind, JtRegs, JtVal, MAX_JT_REANALYZE,
};
use super::{env_u32_any, jumptables_enabled, reloc_roots_enabled};
use crate::emu::BlockCache;

/// Minimum BFS frontier size before spawning parallel worker threads for CFG discovery.
const PARALLEL_DISCOVER_MIN_BLOCKS: usize = 512;

/// PE32+ data directory `idx` of the image at `[base, base + size)`: `(rva, size)`, None if
/// absent or out of the image.
pub(super) unsafe fn data_dir(base: usize, size: usize, idx: usize) -> Option<(u32, u32)> {
    if base == 0 || size < 0x400 {
        return None;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return None;
    }
    let nt = rd32(0x3C) as usize;
    let opt = nt + 24;
    if opt + 112 + 16 * 8 > size || rd32(nt) != 0x0000_4550 || rd16(opt) != 0x20B {
        return None;
    }
    if rd32(opt + 108) as usize <= idx {
        return None;
    }
    let (rva, len) = (rd32(opt + 112 + idx * 8), rd32(opt + 112 + idx * 8 + 4));
    (len != 0 && rva as u64 + len as u64 <= size as u64).then_some((rva, len))
}

/// `.pdata` function map of a mapped PE32+ image: every RUNTIME_FUNCTION range with the
/// begin RVA of its chain root. Not every RUNTIME_FUNCTION is a function start: entries
/// with `UNW_FLAG_CHAININFO` (fragments the compiler split off a function: cold parts,
/// shrink-wrapped regions) and `RUNTIME_FUNCTION_INDIRECT` entries are merged into the
/// function they continue, so all fragments of one root count as one function even when
/// they are not contiguous.
pub(super) struct FuncMap {
    /// `(begin, end, root)` RVAs, sorted by `begin`.
    pub(super) ranges: Vec<(u32, u32, u32)>,
    /// Entries merged into another root.
    pub(super) chained: usize,
    /// Entries whose chain is unreadable or cyclic (kept as their own root).
    pub(super) broken: usize,
}

impl FuncMap {
    pub(super) unsafe fn read(base: usize, size: usize) -> FuncMap {
        const UNW_FLAG_CHAININFO: u8 = 4;
        let mut fm = FuncMap { ranges: Vec::new(), chained: 0, broken: 0 };
        let Some((rva, len)) = data_dir(base, size, 3) else { return fm };
        let fits = |off: u64, n: u64| off.checked_add(n).is_some_and(|e| e <= size as u64);
        let rd8 = |off: u64| *((base as u64 + off) as *const u8);
        let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
        let entry = |off: u64| (rd32(off), rd32(off + 4), rd32(off + 8));
        for i in 0..len as u64 / 12 {
            let (b, e, mut ui) = entry(rva as u64 + i * 12);
            if b >= e || !fits(b as u64, (e - b) as u64) {
                continue;
            }
            let (mut root, mut ok, mut hops) = (b, true, 0);
            loop {
                hops += 1;
                if hops > 32 {
                    ok = false;
                    break;
                }
                let next = if ui & 1 != 0 {
                    // RUNTIME_FUNCTION_INDIRECT: UnwindData is the RVA of the real entry.
                    ui as u64 & !1
                } else {
                    let u = ui as u64;
                    if !fits(u, 4) {
                        ok = false;
                        break;
                    }
                    if (rd8(u) >> 3) & UNW_FLAG_CHAININFO == 0 {
                        break;
                    }
                    // The parent RUNTIME_FUNCTION follows the (even-padded) unwind codes.
                    u + 4 + ((rd8(u + 2) as u64 + 1) & !1) * 2
                };
                if !fits(next, 12) {
                    ok = false;
                    break;
                }
                let (b2, _, u2) = entry(next);
                root = b2;
                ui = u2;
            }
            if !ok {
                fm.broken += 1;
                root = b;
            } else if root != b {
                fm.chained += 1;
            }
            fm.ranges.push((b, e, root));
        }
        fm.ranges.sort_unstable();
        fm
    }

    pub(super) fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    /// Root begin RVA of the function containing `rva` (None: no `.pdata` entry, e.g. a
    /// leaf function).
    pub(super) fn root_of(&self, rva: u32) -> Option<u32> {
        let i = self.ranges.partition_point(|r| r.0 <= rva);
        let &(_, e, root) = self.ranges.get(i.checked_sub(1)?)?;
        (rva < e).then_some(root)
    }

    /// Distinct roots (functions).
    pub(super) fn functions(&self) -> usize {
        self.ranges.iter().filter(|r| r.0 == r.2).count()
    }
}

/// Closure statistics of [`discover`].
#[derive(Default)]
pub(super) struct Closure {
    /// Static successors left out because they lie in another `.pdata` function.
    pub(super) cross: usize,
    /// Indirect switch / jump tables discovered during static CFG closure.
    pub(super) jt_tables: usize,
    /// Unique switch / jump-table case targets added to the static closure.
    pub(super) jt_targets: usize,
    /// Direct `.pdata`-less leaf callees (calls / tail-jumps) discovered during closure.
    pub(super) leaf_callees: usize,
    /// `.pdata`-less leaf function entries discovered via `LEA reg, [rip + disp32]`.
    pub(super) lea_leaves: usize,
    pub(super) functions: usize,
    pub(super) chained: usize,
    pub(super) broken: usize,
}

/// Block set for a module: the seed block IPs plus their static successors (including
/// RIP-relative switch / jump-table targets and `.pdata`-less leaf callees) inside the
/// module's executable sections, up to `depth` steps away. With a `.pdata` map (unless
/// `PORT_AOT_PDATA=0` / `AW2_DLL_AOT_PDATA=0`) intra-function successors stay in the same
/// function (chain roots merged), while direct calls / tail-jumps into `.pdata`-less leaf
/// functions are followed so leaf helpers aren't left out of AOT caches.
pub(super) unsafe fn discover(
    bc: *mut BlockCache,
    base: usize,
    size: usize,
    prof: &[u32],
    depth: u32,
) -> (Vec<u64>, Closure) {
    discover_inner(Some(bc), None, base, size, prof, depth, 1)
}

/// Parallel-capable version of [`discover`]: expands large BFS frontiers across up to
/// `threads` worker threads reusing `pool`, merging results in deterministic order.
pub(super) unsafe fn discover_with_threads(
    pool: &mut WorkerCaches,
    base: usize,
    size: usize,
    prof: &[u32],
    depth: u32,
    threads: usize,
) -> (Vec<u64>, Closure) {
    discover_inner(None, Some(pool), base, size, prof, depth, threads)
}

unsafe fn discover_inner(
    bc: Option<*mut BlockCache>,
    mut pool: Option<&mut WorkerCaches>,
    base: usize,
    size: usize,
    prof: &[u32],
    depth: u32,
    threads: usize,
) -> (Vec<u64>, Closure) {
    let text = exec_sections(base, size);
    let in_text = |ip: u64| text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip));
    let fmap = if env_u32_any(&["PORT_AOT_PDATA", "AW2_DLL_AOT_PDATA"], 1) != 0 {
        FuncMap::read(base, size)
    } else {
        FuncMap { ranges: Vec::new(), chained: 0, broken: 0 }
    };
    let jt_on = jumptables_enabled();
    let func = |ip: u64| fmap.root_of(ip.wrapping_sub(base as u64) as u32);
    let mut st = Closure {
        functions: fmap.functions(),
        chained: fmap.chained,
        broken: fmap.broken,
        ..Closure::default()
    };
    let mut seen: HashSet<u64> = HashSet::new();
    // Best register hint each analyzed block has been seen with (populated lazily when hints exist).
    let mut seen_hints: HashMap<u64, JtRegs> = HashMap::new();
    // Blocks known to end in an indirect `jmp` (eligible for hint-change re-analysis).
    let mut indirect_blocks: HashSet<u64> = HashSet::new();
    // Indirect-branch blocks that already counted toward `st.jt_tables` (prevents double-count).
    let mut jt_blocks: HashSet<u64> = HashSet::new();
    // Per-block count of hint-upgrade re-analyses (capped at `MAX_JT_REANALYZE`).
    let mut reanalyzed: HashMap<u64, u8> = HashMap::new();
    // `(ip, incoming_hint, is_reanalysis)`.
    let mut frontier: Vec<(u64, Option<Box<JtRegs>>, bool)> = prof
        .iter()
        .map(|&r| (base as u64 + r as u64, None, false))
        .filter(|&(ip, _, _)| in_text(ip))
        .collect();

    for d in 0..=depth {
        let mut batch: Vec<(u64, Option<Box<JtRegs>>)> = Vec::new();
        for (ip, hint, is_re) in frontier {
            if is_re {
                if d == depth {
                    continue;
                }
                batch.push((ip, hint));
            } else {
                if !seen.insert(ip) || d == depth {
                    continue;
                }
                if let Some(ref h) = hint {
                    seen_hints.insert(ip, **h);
                }
                batch.push((ip, hint));
            }
        }
        if batch.is_empty() {
            break;
        }

        let steps: Vec<BlockStep> = if threads > 1
            && batch.len() >= PARALLEL_DISCOVER_MIN_BLOCKS
            && pool.is_some()
        {
            let workers = threads.min(batch.len());
            let chunk_len = (batch.len() / (workers * 8)).clamp(1, 256);
            let chunks: Vec<&[(u64, Option<Box<JtRegs>>)]> = batch.chunks(chunk_len).collect();
            let n_chunks = chunks.len();
            let next_chunk = AtomicUsize::new(0);
            let worker_bcs = pool.as_deref_mut().unwrap().ensure(workers);
            let mut all_chunks: Vec<(usize, Vec<BlockStep>)> = Vec::with_capacity(n_chunks);
            std::thread::scope(|scope| {
                let mut handles = Vec::with_capacity(workers);
                for (wi, local_bc) in worker_bcs.iter_mut().enumerate() {
                    let chunks_ref = &chunks;
                    let next_ref = &next_chunk;
                    let text_ref = &text;
                    let fmap_ref = &fmap;
                    let h = std::thread::Builder::new()
                        .name(format!("port-aot-d{wi}"))
                        .stack_size(PARALLEL_WORKER_STACK)
                        .spawn_scoped(scope, move || {
                            let bcp: *mut BlockCache = &mut **local_bc;
                            let mut done: Vec<(usize, Vec<BlockStep>)> = Vec::new();
                            loop {
                                let ci = next_ref.fetch_add(1, Ordering::Relaxed);
                                if ci >= n_chunks {
                                    break;
                                }
                                let slice = chunks_ref[ci];
                                let mut out = Vec::with_capacity(slice.len());
                                for (ip, hint) in slice {
                                    out.push(analyze_block(
                                        bcp,
                                        *ip,
                                        hint.as_deref(),
                                        base,
                                        size,
                                        text_ref,
                                        fmap_ref,
                                        jt_on,
                                    ));
                                }
                                done.push((ci, out));
                            }
                            done
                        })
                        .expect("spawn parallel CFG discovery worker");
                    handles.push(h);
                }
                for h in handles {
                    all_chunks.extend(h.join().expect("parallel CFG discovery worker panicked"));
                }
            });
            all_chunks.sort_unstable_by_key(|(ci, _)| *ci);
            let mut flat = Vec::with_capacity(batch.len());
            for (_, chunk_steps) in all_chunks {
                flat.extend(chunk_steps);
            }
            flat
        } else {
            let bcp = match (bc, pool.as_deref_mut()) {
                (Some(p), _) => p,
                (None, Some(p)) => p.get_one(),
                (None, None) => unreachable!("discover_inner requires bc or pool"),
            };
            batch
                .iter()
                .map(|(ip, hint)| {
                    analyze_block(bcp, *ip, hint.as_deref(), base, size, &text, &fmap, jt_on)
                })
                .collect()
        };

        let mut next: Vec<(u64, Option<Box<JtRegs>>, bool)> = Vec::new();
        let mut next_pos: HashMap<u64, usize> = HashMap::new();
        for ((ip, _), step) in batch.into_iter().zip(steps) {
            let f = func(ip);
            if step.ends_indirect_jmp {
                indirect_blocks.insert(ip);
            }
            if step.jt_targets > 0 && jt_blocks.insert(ip) {
                st.jt_tables += 1;
            }
            for (s, kind, mut out_hint) in step.edges {
                if !in_text(s) {
                    continue;
                }
                let mut is_leaf_entry = false;
                if !fmap.is_empty() {
                    let sf = func(s);
                    match kind {
                        EdgeKind::Intra | EdgeKind::JumpTable => {
                            if sf != f {
                                st.cross += 1;
                                continue;
                            }
                        }
                        EdgeKind::DirectCall => {
                            if sf.is_some() && sf != f {
                                st.cross += 1;
                                continue;
                            }
                            if sf.is_none() && f.is_some() {
                                is_leaf_entry = true;
                            }
                            out_hint = None;
                        }
                        EdgeKind::TailBranch => {
                            if sf != f {
                                if sf.is_none() && valid_code_target(s, &text) {
                                    // Direct tail-jump from a `.pdata` function into a
                                    // `.pdata`-less leaf helper: follow cleanly without leaking
                                    // caller switch hints.
                                    is_leaf_entry = true;
                                    out_hint = None;
                                } else {
                                    st.cross += 1;
                                    continue;
                                }
                            }
                        }
                        EdgeKind::LeaLeaf => {
                            if sf.is_some() {
                                continue;
                            }
                            out_hint = None;
                        }
                    }
                }

                if seen.contains(&s) {
                    // Re-analyze an already-seen block if it receives strictly richer register
                    // hints (e.g. a switch dispatch block reached first without base/bound
                    // hints, or an intermediate block on the way to one).
                    if jt_on {
                        if let Some(ref incoming) = out_hint {
                            let count = reanalyzed.get(&s).copied().unwrap_or(0);
                            if count < MAX_JT_REANALYZE {
                                let prev = seen_hints.entry(s).or_insert([JtVal::Unknown; 16]);
                                let upgraded = merge_jt_regs(prev, incoming);
                                let indirect_alt = !upgraded
                                    && indirect_blocks.contains(&s)
                                    && *prev != **incoming;
                                if upgraded || indirect_alt {
                                    let re_hint = if indirect_alt { **incoming } else { *prev };
                                    if let Some(&idx) = next_pos.get(&s) {
                                        if let Some(ref mut cur) = next[idx].1 {
                                            merge_jt_regs(cur, &re_hint);
                                        } else {
                                            next[idx].1 = Some(Box::new(re_hint));
                                        }
                                    } else {
                                        reanalyzed.insert(s, count + 1);
                                        next_pos.insert(s, next.len());
                                        next.push((s, Some(Box::new(re_hint)), true));
                                    }
                                }
                            }
                        }
                    }
                    continue;
                }

                if let Some(&idx) = next_pos.get(&s) {
                    if let Some(ref incoming) = out_hint {
                        if let Some(ref mut cur) = next[idx].1 {
                            merge_jt_regs(cur, incoming);
                        } else {
                            next[idx].1 = out_hint;
                        }
                    }
                    continue;
                }
                if kind == EdgeKind::JumpTable {
                    st.jt_targets += 1;
                }
                if is_leaf_entry {
                    st.leaf_callees += 1;
                }
                if kind == EdgeKind::LeaLeaf {
                    st.lea_leaves += 1;
                }
                next_pos.insert(s, next.len());
                next.push((s, out_hint, false));
            }
        }
        frontier = next;
    }
    let mut v: Vec<u64> = seen.into_iter().collect();
    v.sort_unstable();
    (v, st)
}

/// PE32+ `AddressOfEntryPoint` RVA of the image at `[base, base + size)`, if present and non-zero.
unsafe fn entry_point_rva(base: usize, size: usize) -> Option<u32> {
    if base == 0 || size < 0x400 {
        return None;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return None;
    }
    let nt = rd32(0x3C) as usize;
    let opt = nt.checked_add(24)?;
    if opt.checked_add(20)? > size || rd32(nt) != 0x0000_4550 || rd16(opt) != 0x20B {
        return None;
    }
    let entry = rd32(opt + 16);
    (entry != 0 && (entry as usize) < size).then_some(entry)
}

/// Exported function RVAs from the PE32+ Export Directory (index 0) at `[base, base + size)`,
/// skipping empty slots and forwarder strings.
unsafe fn export_function_rvas(base: usize, size: usize) -> Vec<u32> {
    let mut out = Vec::new();
    let Some((dir_rva, dir_len)) = data_dir(base, size, 0) else {
        return out;
    };
    if dir_len < 40 {
        return out;
    }
    let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
    let n_funcs = rd32(dir_rva as u64 + 20) as u64;
    let funcs_rva = rd32(dir_rva as u64 + 28) as u64;
    let Some(table_bytes) = n_funcs.checked_mul(4) else {
        return out;
    };
    let Some(table_end) = funcs_rva.checked_add(table_bytes) else {
        return out;
    };
    if table_end > size as u64 {
        return out;
    }
    let dir_start = dir_rva as u64;
    let dir_end = dir_start + dir_len as u64;
    for i in 0..n_funcs {
        let rva = rd32(funcs_rva + i * 4);
        if rva == 0 {
            continue;
        }
        let rva64 = rva as u64;
        // Forwarder exports point to a "DLL.Symbol" ASCII string inside the export directory.
        if rva64 >= dir_start && rva64 < dir_end {
            continue;
        }
        if rva64 < size as u64 {
            out.push(rva);
        }
    }
    out
}

/// OptionalHeader `ImageBase` of a PE32+ image mapped at `[base, base + size)`.
pub(super) unsafe fn opt_image_base(base: usize, size: usize) -> Option<u64> {
    if base == 0 || size < 0x400 {
        return None;
    }
    let rd16 = |off: usize| core::ptr::read_unaligned((base + off) as *const u16);
    let rd32 = |off: usize| core::ptr::read_unaligned((base + off) as *const u32);
    if rd16(0) != 0x5A4D {
        return None;
    }
    let nt = rd32(0x3C) as usize;
    let opt = nt.checked_add(24)?;
    if opt.checked_add(32)? > size || rd32(nt) != 0x0000_4550 || rd16(opt) != 0x20B {
        return None;
    }
    Some(core::ptr::read_unaligned((base + opt + 24) as *const u64))
}

/// Convert a VA inside a mapped PE32+ image (`[base, base + size)`, or fallback to
/// `OptionalHeader.ImageBase` if unrelocated) into an in-bounds RVA.
#[inline]
pub(super) fn pe_va_to_rva(base: usize, size: usize, opt_base: Option<u64>, va: u64) -> Option<u32> {
    let b = base as u64;
    let sz = size as u64;
    if va >= b && va - b < sz {
        return u32::try_from(va - b).ok();
    }
    if let Some(ob) = opt_base {
        if ob != 0 && ob != b && va >= ob && va - ob < sz {
            return u32::try_from(va - ob).ok();
        }
    }
    None
}

/// TLS callback RVAs from `IMAGE_TLS_DIRECTORY64::AddressOfCallBacks` (DataDirectory[9]).
unsafe fn tls_callback_rvas(base: usize, size: usize) -> Vec<u32> {
    const MAX_TLS_CALLBACKS: usize = 1024;
    let mut out = Vec::new();
    let Some((dir_rva, _dir_len)) = data_dir(base, size, 9) else {
        return out;
    };
    if (dir_rva as u64).saturating_add(32) > size as u64 {
        return out;
    }
    let rd64 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u64);
    let opt_base = opt_image_base(base, size);
    let cb_table_va = rd64(dir_rva as u64 + 24);
    if cb_table_va == 0 {
        return out;
    }
    let Some(table_rva) = pe_va_to_rva(base, size, opt_base, cb_table_va) else {
        return out;
    };
    for i in 0..MAX_TLS_CALLBACKS {
        let off = (table_rva as u64).saturating_add((i as u64) * 8);
        if off.saturating_add(8) > size as u64 {
            break;
        }
        let cb_va = rd64(off);
        if cb_va == 0 {
            break;
        }
        let Some(rva) = pe_va_to_rva(base, size, opt_base, cb_va) else {
            break;
        };
        out.push(rva);
    }
    out
}

/// Control Flow Guard (CFG) valid indirect-call target RVAs from
/// `IMAGE_LOAD_CONFIG_DIRECTORY64` (DataDirectory[10]): `GuardCFFunctionTable` (respecting
/// `IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK` extra bytes per entry) and
/// `GuardLongJumpTargetTable` where present.
unsafe fn load_config_cfg_rvas(base: usize, size: usize) -> Vec<u32> {
    const GUARD_CF_FUNCTION_TABLE_PRESENT: u32 = 0x0000_0400;
    const GUARD_CF_LONGJUMP_TABLE_PRESENT: u32 = 0x0001_0000;
    const MAX_CFG_ENTRIES: usize = 4_000_000;
    let mut out = Vec::new();
    let Some((dir_rva, _dir_len)) = data_dir(base, size, 10) else {
        return out;
    };
    if (dir_rva as u64).saturating_add(4) > size as u64 {
        return out;
    }
    let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
    let rd64 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u64);
    let avail = (size as u64).saturating_sub(dir_rva as u64);
    let lc_size = (rd32(dir_rva as u64) as u64).min(avail);
    if lc_size < 0x94 {
        return out;
    }
    let opt_base = opt_image_base(base, size);
    let guard_flags = rd32(dir_rva as u64 + 0x90);

    if (guard_flags & GUARD_CF_FUNCTION_TABLE_PRESENT) != 0 {
        let tbl_va = rd64(dir_rva as u64 + 0x80);
        let count = rd64(dir_rva as u64 + 0x88);
        let extra = ((guard_flags & 0xF000_0000) >> 28) as u64;
        let stride = 4u64 + extra;
        if tbl_va != 0 && count != 0 {
            if let Some(tbl_rva) = pe_va_to_rva(base, size, opt_base, tbl_va) {
                let max_fit = (size as u64).saturating_sub(tbl_rva as u64) / stride;
                let n = (count.min(max_fit) as usize).min(MAX_CFG_ENTRIES);
                out.reserve(n);
                for i in 0..n {
                    let off = tbl_rva as u64 + (i as u64) * stride;
                    let rva = rd32(off);
                    if rva != 0 && (rva as u64) < size as u64 {
                        out.push(rva);
                    }
                }
            }
        }
    }

    if lc_size >= 0xC0 && (guard_flags & GUARD_CF_LONGJUMP_TABLE_PRESENT) != 0 {
        let tbl_va = rd64(dir_rva as u64 + 0xB0);
        let count = rd64(dir_rva as u64 + 0xB8);
        if tbl_va != 0 && count != 0 {
            if let Some(tbl_rva) = pe_va_to_rva(base, size, opt_base, tbl_va) {
                let max_fit = (size as u64).saturating_sub(tbl_rva as u64) / 4;
                let n = (count.min(max_fit) as usize).min(MAX_CFG_ENTRIES);
                for i in 0..n {
                    let off = tbl_rva as u64 + (i as u64) * 4;
                    let rva = rd32(off);
                    if rva != 0 && (rva as u64) < size as u64 {
                        out.push(rva);
                    }
                }
            }
        }
    }

    out
}

/// Executable-section target RVAs referenced by PE Base Relocations (`DataDirectory[5]`,
/// `IMAGE_REL_BASED_DIR64`) — typically C++ vtables, function-pointer tables, and global
/// callback tables in `.rdata`/`.data` pointing to `.pdata`-less leaf methods and adjustor
/// thunks.
///
/// When `DataDirectory[5]` is absent/empty (stripped `.reloc`), falls back to a conservative
/// 8-byte-aligned scan of non-executable initialized data sections for pointers into `text`
/// that land on clean function boundaries.
unsafe fn base_reloc_code_rvas(base: usize, size: usize, text: &[(usize, usize)]) -> Vec<u32> {
    const IMAGE_REL_BASED_DIR64: u16 = 10;
    const MAX_RELOC_ENTRIES: usize = 4_000_000;

    let mut out = Vec::new();
    if base == 0 || size < 0x400 || text.is_empty() {
        return out;
    }
    let opt_base = opt_image_base(base, size);

    if let Some((dir_rva, dir_len)) = data_dir(base, size, 5).filter(|&(r, _)| r != 0) {
        let Some(dir_end) = (dir_rva as u64).checked_add(dir_len as u64) else {
            return out;
        };
        if dir_end > size as u64 {
            return out;
        }
        let rd16 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u16);
        let rd32 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u32);
        let rd64 = |off: u64| core::ptr::read_unaligned((base as u64 + off) as *const u64);

        let mut off = dir_rva as u64;
        while off.checked_add(8).is_some_and(|e| e <= dir_end) {
            let page_rva = rd32(off);
            let block_size = rd32(off + 4);
            if block_size < 8 {
                break;
            }
            let Some(next_off) = off.checked_add(block_size as u64) else {
                break;
            };
            if next_off > dir_end {
                break;
            }
            let n_entries = ((block_size - 8) / 2) as u64;
            for i in 0..n_entries {
                let entry = rd16(off + 8 + i * 2);
                let ty = entry >> 12;
                if ty != IMAGE_REL_BASED_DIR64 {
                    continue;
                }
                let rel_off = (entry & 0x0FFF) as u32;
                let Some(slot_rva) = page_rva.checked_add(rel_off) else {
                    continue;
                };
                if (slot_rva as u64) + 8 > size as u64 {
                    continue;
                }
                let va = rd64(slot_rva as u64);
                let Some(target_rva) = pe_va_to_rva(base, size, opt_base, va) else {
                    continue;
                };
                let target_ip = base as u64 + target_rva as u64;
                if is_plausible_code_entry(target_ip, text) {
                    out.push(target_rva);
                    if out.len() >= MAX_RELOC_ENTRIES {
                        return out;
                    }
                }
            }
            off = next_off;
        }
        return out;
    }

    // Fallback when `DataDirectory[5]` is absent/empty (stripped `.reloc`): scan 8-byte-aligned
    // slots in non-executable initialized data sections for pointers into `text` landing on
    // clean function boundaries. Require a non-zero `OptionalHeader.ImageBase` so synthetic
    // images with zeroed headers cannot turn small integers into bogus RVAs.
    if !opt_base.is_some_and(|ob| ob != 0) {
        return out;
    }
    let rd64 = |addr: usize| core::ptr::read_unaligned(addr as *const u64);
    for (sec_s, sec_e) in data_sections(base, size) {
        let Some(rel_s) = (sec_s - base).checked_add(7).map(|x| x & !7) else {
            continue;
        };
        let rel_e = sec_e - base;
        let mut rel = rel_s;
        while rel.checked_add(8).is_some_and(|e| e <= rel_e) {
            let va = rd64(base + rel);
            if let Some(target_rva) = pe_va_to_rva(base, size, opt_base, va) {
                let target_ip = base as u64 + target_rva as u64;
                if is_clean_code_entry(target_ip, text) {
                    out.push(target_rva);
                    if out.len() >= MAX_RELOC_ENTRIES {
                        return out;
                    }
                }
            }
            rel += 8;
        }
    }
    out
}

/// Discover profile-free function root RVAs directly from a mapped PE32+ image:
/// 1. Every `.pdata` (`RUNTIME_FUNCTION`) entry's `begin_rva`,
/// 2. `AddressOfEntryPoint`,
/// 3. Export Directory function RVAs (excluding forwarders and non-code data exports),
/// 4. TLS Directory callback RVAs (`AddressOfCallBacks`),
/// 5. Load Config Control Flow Guard (`GuardCFFunctionTable` / `GuardLongJumpTargetTable`),
/// 6. PE Base Relocation (`DataDirectory[5]`, `IMAGE_REL_BASED_DIR64`) executable-section
///    targets (vtables / function-pointer tables; with a stripped-`.reloc` fallback scan),
///    when `PORT_AOT_RELOC_ROOTS` is enabled (default 1).
///
/// Filtered to executable sections, sorted deterministically, deduplicated, and capped at
/// `max_roots` when `max_roots > 0`.
pub(super) unsafe fn discover_pe_roots(base: usize, size: usize, max_roots: usize) -> Vec<u32> {
    let text = exec_sections(base, size);
    if text.is_empty() {
        return Vec::new();
    }
    let in_text = |rva: u32| -> bool {
        let ip = base as u64 + rva as u64;
        text.iter().any(|&(s, e)| (s as u64..e as u64).contains(&ip))
    };
    let mut roots = Vec::new();
    let fmap = FuncMap::read(base, size);
    for &(begin, _, _) in &fmap.ranges {
        if in_text(begin) {
            roots.push(begin);
        }
    }
    if let Some(ep) = entry_point_rva(base, size) {
        if in_text(ep) {
            roots.push(ep);
        }
    }
    for rva in export_function_rvas(base, size)
        .into_iter()
        .chain(tls_callback_rvas(base, size))
        .chain(load_config_cfg_rvas(base, size))
    {
        if in_text(rva) {
            roots.push(rva);
        }
    }
    if reloc_roots_enabled() {
        roots.extend(base_reloc_code_rvas(base, size, &text));
    }
    for cb in super::callbacks::discover_static_callback_roots(base, size, &text) {
        if in_text(cb.rva) {
            roots.push(cb.rva);
        }
    }
    roots.sort_unstable();
    roots.dedup();
    if max_roots > 0 && roots.len() > max_roots {
        roots.truncate(max_roots);
    }
    roots
}

/// Discover all PE roots and reachable basic block IPs in a mapped PE image `[base, base+size)`.
pub(in crate::emu) unsafe fn discover_blocks_for_verify(
    base: usize,
    size: usize,
    max_roots: usize,
    depth: u32,
    threads: usize,
) -> (Vec<u32>, Vec<u64>) {
    let roots = discover_pe_roots(base, size, max_roots);
    if roots.is_empty() {
        return (roots, Vec::new());
    }
    let mut pool = WorkerCaches::new();
    let (ips, _cl) = discover_with_threads(&mut pool, base, size, &roots, depth, threads.max(1));
    (roots, ips)
}
