//! Per-encoder GPU intervals (`AW2_PASS_TIMING=1`, diagnostics).
//!
//! Every encoder writes a relaxed-granularity timestamp when it opens and
//! before it ends (relaxed timestamps snap to encoder boundaries and don't
//! split or serialize work). The logs of a submission are resolved on the CPU
//! when its timeline value completes. Intervals are walked in submission
//! order: an encoder's duration is `end - start`; the GPU idle before it is
//! how far its start lies past the latest end seen so far. [`dump`] logs
//! per-frame busy time, idle by encoder kind / barrier / transition, and the
//! costliest encoders.
//!
//! Logs use fixed slices of a few shared counter heaps (the system limits
//! the number and size of counter heaps; the app needs its own).

use super::query::MtlCounterHeap;
use objc2::rc::Retained;
use objc2_foundation::NSRange;
use objc2_metal::*;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

/// Heap entries per list recording (two per encoder; split mode needs one
/// encoder per dispatch/draw).
const SLICE: usize = 512;
/// Entries per counter heap (`8` slices of `512` timestamps = `4096` entries).
const HEAP_ENTRIES: usize = 4096;
const PER_HEAP: usize = HEAP_ENTRIES / SLICE;
/// Counter heaps created (`8 * 4096 * 8B = 256 KB` total). Apple AGX enforces a
/// ~31–32 `MTL4CounterHeap` per-process ceiling; keeping `HEAPS == 8` leaves
/// 23–24 hardware counter heaps free for Northlight's D3D12 `CreateQueryHeap`.
const HEAPS: usize = 8;
const _: () = assert!(PER_HEAP == 8);
/// Gaps longer than this are idle between frames, not pass bubbles.
const MAX_GAP_NS: u64 = 20_000_000;

/// Encoder kinds.
pub mod kind {
    pub const DRAW: u8 = 0;
    pub const CLEAR: u8 = 1;
    pub const UTIL: u8 = 2;
    pub const COMPUTE: u8 = 3;
    pub const COUNT: usize = 4;
    pub const NAMES: [&str; COUNT] = ["draw", "clear", "util", "compute"];
}

/// Synthetic shader keys for internal utility / copy / clear / RT encoders
/// that do not bind a converted DXIL game shader (previously logged as `0`).
pub mod synth {
    pub const CLEAR_CS: u64 = 0xffff_ff00_0000_0001;
    pub const CLEAR_UAV: u64 = 0xffff_ff00_0000_0002;
    pub const UPSCALE: u64 = 0xffff_ff00_0000_0003;
    pub const COPY_BLIT: u64 = 0xffff_ff00_0000_0004;
    pub const EI_EXPAND: u64 = 0xffff_ff00_0000_0005;
    pub const RT_ACCEL: u64 = 0xffff_ff00_0000_0006;
    pub const RT_RAYS: u64 = 0xffff_ff00_0000_0007;
    pub const CLEAR_PASS: u64 = 0xffff_ff00_0000_0008;
    pub const RESOLVE: u64 = 0xffff_ff00_0000_0009;
    pub const FILL_OCCL: u64 = 0xffff_ff00_0000_000a;

    pub fn name(key: u64) -> Option<&'static str> {
        match key {
            CLEAR_CS => Some("aw2:clear_cs"),
            CLEAR_UAV => Some("aw2:clear_uav"),
            UPSCALE => Some("aw2:upscale"),
            COPY_BLIT => Some("aw2:copy_blit"),
            EI_EXPAND => Some("aw2:ei_expand"),
            RT_ACCEL => Some("aw2:rt_accel"),
            RT_RAYS => Some("aw2:rt_rays"),
            CLEAR_PASS => Some("aw2:clear_pass"),
            RESOLVE => Some("aw2:resolve"),
            FILL_OCCL => Some("aw2:fill_occl"),
            _ => None,
        }
    }
}

/// What one encoder recorded.
#[derive(Clone, Copy, Default)]
pub struct PassMeta {
    pub kind: u8,
    /// Opening queue barrier: 0 none, 1 stage-masked, 2 all → all.
    pub sync: u8,
    pub draws: u32,
    pub dispatches: u32,
    pub icb: u32,
    pub w: u32,
    pub h: u32,
    /// Shader identity of the encoder's last dispatch/draw (low 64 bits of the
    /// compiled shader cache key; graphics: VS key ^ PS key rotated).
    pub shader: u64,
}

struct PoolState {
    n_heaps: usize,
    active_count: [u8; HEAPS],
    dirty_mask: [u8; HEAPS],
    clean_mask: [u8; HEAPS],
    open_heap: usize,
    rr_heap: usize,
}

impl PoolState {
    fn new(n_heaps: usize) -> Self {
        let n = n_heaps.min(HEAPS);
        let mut clean_mask = [0u8; HEAPS];
        for m in clean_mask.iter_mut().take(n) {
            *m = 0xFF;
        }
        Self {
            n_heaps: n,
            active_count: [0; HEAPS],
            dirty_mask: [0; HEAPS],
            clean_mask,
            open_heap: 0,
            rr_heap: 0,
        }
    }

    fn take_clean_on(&mut self, h: usize) -> u32 {
        let sub = self.clean_mask[h].trailing_zeros() as usize;
        debug_assert!(sub < PER_HEAP);
        self.clean_mask[h] &= !(1u8 << sub);
        self.active_count[h] += 1;
        (h * PER_HEAP + sub) as u32
    }

    fn acquire_slice(&mut self) -> Option<u32> {
        if self.n_heaps == 0 {
            return None;
        }
        // Pack into an already-active heap with remaining clean slices so a frame's
        // ~5–10 lists stay within 1–2 heaps while the other 6–7 heaps drain completely.
        for off in 0..self.n_heaps {
            let h = (self.open_heap + off) % self.n_heaps;
            if self.active_count[h] > 0 && self.clean_mask[h] != 0 {
                self.open_heap = h;
                return Some(self.take_clean_on(h));
            }
        }
        // Otherwise open the next quiescent heap in FIFO ring order.
        for off in 0..self.n_heaps {
            let h = (self.rr_heap + off) % self.n_heaps;
            if self.clean_mask[h] != 0 {
                self.open_heap = h;
                self.rr_heap = (h + 1) % self.n_heaps;
                return Some(self.take_clean_on(h));
            }
        }
        None
    }

    /// Release `slice` (`used` slots written). Returns `Some(h)` iff heap `h` is
    /// 100% quiescent (`active_count[h] == 0`) and has dirty slices to invalidate.
    fn release_slice(&mut self, slice: u32, used: usize) -> Option<usize> {
        let h = slice as usize / PER_HEAP;
        let sub = slice as usize % PER_HEAP;
        if h >= self.n_heaps || sub >= PER_HEAP {
            return None;
        }
        debug_assert!(self.active_count[h] > 0);
        self.active_count[h] = self.active_count[h].saturating_sub(1);
        let bit = 1u8 << sub;
        if used > 0 {
            self.dirty_mask[h] |= bit;
        } else {
            self.clean_mask[h] |= bit;
        }
        if self.active_count[h] == 0 && self.dirty_mask[h] != 0 {
            self.clean_mask[h] |= self.dirty_mask[h];
            self.dirty_mask[h] = 0;
            Some(h)
        } else {
            None
        }
    }
}

struct Shared {
    heaps: Vec<Retained<MtlCounterHeap>>,
    pool: Mutex<PoolState>,
}
// Counter heaps are thread-safe Metal objects.
unsafe impl Send for Shared {}
unsafe impl Sync for Shared {}

fn shared() -> Option<&'static Shared> {
    static S: OnceLock<Option<Shared>> = OnceLock::new();
    S.get_or_init(|| {
        let d = super::device()?;
        let mut heaps = Vec::new();
        for _ in 0..HEAPS {
            let desc = MTL4CounterHeapDescriptor::new();
            desc.setType(MTL4CounterHeapType::Timestamp);
            unsafe { desc.setCount(HEAP_ENTRIES) };
            match d.dev.newCounterHeapWithDescriptor_error(&desc) {
                Ok(h) => heaps.push(h),
                Err(e) => {
                    log::warn!(target: "d3d", "pass timing: newCounterHeap({HEAP_ENTRIES}): {e}");
                    break;
                }
            }
        }
        let n_heaps = heaps.len();
        let slices = n_heaps * PER_HEAP;
        log::info!(target: "d3d", "pass timing: {slices} slices of {SLICE} timestamps in {n_heaps} heaps");
        (slices > 0).then(|| Shared { heaps, pool: Mutex::new(PoolState::new(n_heaps)) })
    })
    .as_ref()
}

#[cfg(test)]
pub(crate) fn dedicated_heap_pool_counts() -> Option<(usize, usize)> {
    let s = shared()?;
    Some((s.heaps.len(), PER_HEAP))
}

/// The timestamp log of one list recording.
pub struct PassLog {
    slice: u32,
    metas: Vec<PassMeta>,
    cur: PassMeta,
    /// The open encoder has a start timestamp.
    open: bool,
    overflow: u32,
    /// Submission ordinal of the list within its frame.
    ord: u32,
    /// CPU times (mach ticks) of ExecuteCommandLists and of the commit.
    t_ecl: u64,
    t_commit: u64,
    queue: u8,
    /// `Queue::pending_wait` consumed by this submission.
    wait: u8,
}

static LIST_ORD: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static NO_SLICE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

unsafe extern "C" {
    fn mach_absolute_time() -> u64;
}

/// CPU clock for submission times. Counter-heap timestamps on Apple GPUs
/// are in the same domain (mach ticks at `timestamp_frequency`), so CPU and
/// GPU times compare directly.
pub fn mach_now() -> u64 {
    unsafe { mach_absolute_time() }
}

/// A frame was presented (submissions before it have been made).
pub fn frame_mark() {
    LIST_ORD.store(0, std::sync::atomic::Ordering::Relaxed);
}

pub fn enabled() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("AW2_PASS_TIMING").is_some_and(|v| v != "0"))
}

/// `AW2_PASS_TIMING=2`: give every dispatch/draw its own encoder so each is
/// timed alone (serializes the GPU; per-shader cost ranking only).
pub fn split() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| std::env::var_os("AW2_PASS_TIMING").is_some_and(|v| v == "2"))
}

impl PassLog {
    /// A log for a list starting to record (None when no slice is free).
    pub fn acquire() -> Option<PassLog> {
        let s = shared()?;
        let Some(slice) = s.pool.lock().unwrap().acquire_slice() else {
            NO_SLICE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return None;
        };
        Some(PassLog { slice, metas: Vec::with_capacity(64), cur: PassMeta::default(), open: false, overflow: 0, ord: 0, t_ecl: 0, t_commit: 0, queue: 0, wait: 0 })
    }

    pub fn heap(&self) -> &'static MtlCounterHeap {
        &shared().expect("pass log without heap").heaps[self.slice as usize / PER_HEAP]
    }

    /// First heap index of the slice.
    fn base(&self) -> usize {
        self.slice as usize % PER_HEAP * SLICE
    }

    /// The list is being submitted (assigns its ordinal within the frame).
    /// `t_ecl`: [`mach_now`] when the app called ExecuteCommandLists; the
    /// commit time is taken now.
    pub fn submitted(&mut self, t_ecl: u64, queue: u8, wait: u8) {
        self.ord = LIST_ORD.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.t_ecl = t_ecl;
        self.t_commit = mach_now();
        self.queue = queue;
        self.wait = wait;
    }

    /// An encoder opened: heap index for its start timestamp (None when the
    /// slice is full).
    pub fn open(&mut self, kind: u8, sync: u8) -> Option<usize> {
        self.cur = PassMeta { kind, sync, ..PassMeta::default() };
        let k = self.metas.len();
        self.open = 2 * k + 1 < SLICE;
        if !self.open {
            self.overflow += 1;
            return None;
        }
        Some(self.base() + 2 * k)
    }

    pub fn cur(&mut self) -> &mut PassMeta {
        &mut self.cur
    }

    /// The open encoder already recorded a dispatch, draw or ICB execution.
    pub fn has_work(&self) -> bool {
        self.open && self.cur.draws + self.cur.dispatches + self.cur.icb > 0
    }

    /// The encoder is ending: heap index for its end timestamp.
    pub fn close(&mut self) -> Option<usize> {
        if !std::mem::take(&mut self.open) {
            return None;
        }
        let k = self.metas.len();
        self.metas.push(self.cur);
        Some(self.base() + 2 * k + 1)
    }
}

impl Drop for PassLog {
    fn drop(&mut self) {
        if let Some(s) = shared() {
            let used = (2 * self.metas.len() + usize::from(self.open) * 2).min(SLICE);
            let mut pool = s.pool.lock().unwrap();
            if let Some(h) = pool.release_slice(self.slice, used) {
                unsafe { s.heaps[h].invalidateCounterRange(NSRange::new(0, HEAP_ENTRIES)) };
            }
        }
    }
}

#[derive(Default)]
struct PassAgg {
    dur: u64,
    /// Part of the interval not overlapping earlier encoders.
    excl: u64,
    gap: u64,
    n: u64,
    draws: u64,
    dispatches: u64,
    icb: u64,
    sync: [u64; 3],
}

#[derive(Default)]
struct Agg {
    /// Union of encoder intervals.
    busy: u64,
    kind_dur: [u64; kind::COUNT],
    kind_excl: [u64; kind::COUNT],
    kind_n: [u64; kind::COUNT],
    /// Idle before encoders of a kind, inside a list.
    kind_gap: [u64; kind::COUNT],
    /// Idle before encoders by opening barrier, inside a list.
    sync_gap: [u64; 3],
    /// Idle by (previous kind, kind), inside a list.
    trans_gap: [[u64; kind::COUNT]; kind::COUNT],
    /// Idle before the first encoder of a list (submission boundaries).
    list_gap: u64,
    lists: u64,
    bad: u64,
    overflow: u64,
    /// By (list ordinal in frame, encoder ordinal, kind, width, height).
    passes: HashMap<(u32, u32, u8, u32, u32), PassAgg>,
    /// By (kind, shader key, width, height): split mode makes this per dispatch/draw.
    shaders: HashMap<(u8, u64, u32, u32), PassAgg>,
    /// Submission latency by (list ordinal in frame, queue type, preceding wait kind).
    subs: HashMap<(u32, u8, u8), SubAgg>,
}

/// CPU→GPU latency of the lists submitted at one ordinal (ns sums).
#[derive(Default)]
struct SubAgg {
    n: u64,
    /// ExecuteCommandLists → commit (CS-thread encode backlog).
    ecl_commit: u64,
    /// Commit → first encoder start on the GPU.
    commit_start: i64,
    /// Lists whose first encoder found the GPU idle.
    n_gap: u64,
    gap: u64,
    /// For those: commit minus the time the GPU went idle (> 0: the CPU
    /// submitted late).
    commit_vs_idle: i64,
}

struct State {
    agg: Agg,
    /// Latest primary (`Direct` / `Compute`) encoder end seen (ticks).
    last_end: u64,
    /// Latest encoder end seen per `ListType` (`0..4`), isolating async `Copy` (`q3`)
    /// background DMA from `Direct` (`q0`) `last_end` / `busy` / `list_gap` (`AW2_COPY_QUEUE_LIGHT=1`).
    last_end_by_q: [u64; 4],
}

static STATE: Mutex<Option<State>> = Mutex::new(None);

#[allow(clippy::too_many_arguments)]
fn resolve_slice_into(
    st: &mut State,
    ord: u32,
    queue: u8,
    wait: u8,
    t_ecl: u64,
    t_commit: u64,
    metas: &[PassMeta],
    ts: &[u64],
    copy_light: bool,
    freq: u128,
) {
    let ns = |t: u64| (t as u128 * 1_000_000_000 / freq) as u64;
    let dns = |a: u64, b: u64| (a as i128 - b as i128) * 1_000_000_000 / freq as i128;
    if ts.len() < 2 * metas.len() {
        return;
    }
    st.agg.lists += 1;
    let isolate_copy_q = copy_light && queue == 3;
    let mut prev_kind: Option<usize> = None;
    for (i, m) in metas.iter().enumerate() {
        let (mut s, mut e) = (ts[2 * i], ts[2 * i + 1]);
        // On Apple TBDR, a render pass that closed with 0 draws/ICBs may never
        // execute the vertex phase (`s == 0` or `MTLCounterErrorValue` while
        // `Fragment` tile load/store wrote a valid `e`), and a 0-dispatch Blit/
        // barrier-only compute encoder (`L1#0` on `q0` or `q3`) may snap to one
        // boundary endpoint. Coalesce the missing endpoint rather than dropping
        // the pass as `bad`.
        let valid = |t: u64| t != 0 && t < u64::MAX - 1;
        let zero_work_coalesce = (m.kind != kind::COMPUTE && m.draws == 0 && m.icb == 0)
            || (m.draws == 0 && m.dispatches == 0 && m.icb == 0);
        if zero_work_coalesce {
            if !valid(s) && valid(e) {
                s = e;
            } else if !valid(e) && valid(s) {
                e = s;
            }
        }
        if !valid(s) || !valid(e) || e < s {
            st.agg.bad += 1;
            continue;
        }
        let k = (m.kind as usize).min(kind::COUNT - 1);
        let dur = ns(e - s);
        let q_idx = (queue as usize) & 3;
        let (prev_end, gap, excl) = if isolate_copy_q {
            let pe = st.last_end_by_q[q_idx];
            let g = if pe != 0 && s > pe { ns(s - pe) } else { 0 };
            let g = if g > MAX_GAP_NS { 0 } else { g };
            let bf = s.max(pe);
            let ex = if e > bf { ns(e - bf) } else { 0 };
            st.last_end_by_q[q_idx] = pe.max(e);
            (pe, g, ex)
        } else {
            let pe = st.last_end;
            let g = if pe != 0 && s > pe { ns(s - pe) } else { 0 };
            let g = if g > MAX_GAP_NS { 0 } else { g };
            let bf = s.max(pe);
            let ex = if e > bf { ns(e - bf) } else { 0 };
            st.agg.busy += ex;
            st.last_end = pe.max(e);
            st.last_end_by_q[q_idx] = st.last_end_by_q[q_idx].max(e);
            (pe, g, ex)
        };
        let a = &mut st.agg;
        a.kind_dur[k] += dur;
        if !isolate_copy_q {
            a.kind_excl[k] += excl;
        }
        a.kind_n[k] += 1;
        match prev_kind {
            None => {
                if !isolate_copy_q {
                    a.list_gap += gap;
                }
                let sa = a.subs.entry((ord, queue, wait)).or_default();
                sa.n += 1;
                sa.ecl_commit += dns(t_commit, t_ecl).max(0) as u64;
                sa.commit_start += dns(s, t_commit) as i64;
                if gap > 0 {
                    sa.n_gap += 1;
                    sa.gap += gap;
                    sa.commit_vs_idle += dns(t_commit, prev_end) as i64;
                }
            }
            Some(pk) => {
                if !isolate_copy_q {
                    a.kind_gap[k] += gap;
                    a.sync_gap[(m.sync as usize).min(2)] += gap;
                    a.trans_gap[pk][k] += gap;
                }
            }
        }
        prev_kind = Some(k);
        let p = a.passes.entry((ord, i as u32, m.kind, m.w, m.h)).or_default();
        p.dur += dur;
        p.excl += excl;
        p.gap += gap;
        p.n += 1;
        p.draws += m.draws as u64;
        p.dispatches += m.dispatches as u64;
        p.icb += m.icb as u64;
        p.sync[(m.sync as usize).min(2)] += 1;
        let q = a.shaders.entry((m.kind, m.shader, m.w, m.h)).or_default();
        q.dur += dur;
        q.excl += excl;
        q.gap += gap;
        q.n += 1;
        q.draws += m.draws as u64;
        q.dispatches += m.dispatches as u64;
        q.icb += m.icb as u64;
    }
}

fn resolve_into(st: &mut State, logs: &[PassLog], copy_light: bool) {
    let freq = super::query::timestamp_frequency().max(1) as u128;
    for log in logs {
        st.agg.overflow += log.overflow as u64;
        let n = log.metas.len();
        if n == 0 {
            continue;
        }
        let Some(data) = (unsafe { log.heap().resolveCounterRange(NSRange::new(log.base(), 2 * n)) }) else { continue };
        let bytes = data.to_vec();
        let ts: Vec<u64> = bytes.chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
        resolve_slice_into(st, log.ord, log.queue, log.wait, log.t_ecl, log.t_commit, &log.metas, &ts, copy_light, freq);
    }
}

/// Resolve the logs of a completed submission (in submission order); the
/// slices return to the pool when the logs drop.
pub fn resolve(logs: Vec<PassLog>) {
    if shared().is_none() {
        return;
    }
    let copy_light = super::cmdlist::copy_queue_light();
    let mut g = STATE.lock().unwrap();
    let st = g.get_or_insert_with(|| State { agg: Agg::default(), last_end: 0, last_end_by_q: [0; 4] });
    resolve_into(st, &logs, copy_light);
    drop(g);
    drop(logs);
}

#[cfg(test)]
pub(crate) struct TestPassSummary {
    pub bad: u64,
    pub kind_n: [u64; kind::COUNT],
    /// `(kind, shader_key, count, dispatches)`
    pub shaders: Vec<(u8, u64, u64, u64)>,
    pub raw_ts: Vec<(u64, u64)>,
}

#[cfg(test)]
pub(crate) fn resolve_for_test(logs: Vec<PassLog>) -> TestPassSummary {
    resolve_for_test_with(logs, super::cmdlist::copy_queue_light())
}

#[cfg(test)]
pub(crate) fn resolve_for_test_with(logs: Vec<PassLog>, copy_light: bool) -> TestPassSummary {
    let mut raw_ts = Vec::new();
    for log in &logs {
        let n = log.metas.len();
        if n > 0 {
            if let Some(data) = unsafe { log.heap().resolveCounterRange(NSRange::new(log.base(), 2 * n)) } {
                let bytes = data.to_vec();
                let ts: Vec<u64> = bytes.chunks_exact(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
                for i in 0..n {
                    if 2 * i + 1 < ts.len() {
                        raw_ts.push((ts[2 * i], ts[2 * i + 1]));
                    }
                }
            }
        }
    }
    let mut st = State { agg: Agg::default(), last_end: 0, last_end_by_q: [0; 4] };
    resolve_into(&mut st, &logs, copy_light);
    let shaders = st
        .agg
        .shaders
        .into_iter()
        .map(|((k, key, _w, _h), p)| (k, key, p.n, p.dispatches))
        .collect();
    drop(logs);
    TestPassSummary { bad: st.agg.bad, kind_n: st.agg.kind_n, shaders, raw_ts }
}

/// Log per-frame averages over the last `frames` frames and reset.
pub fn dump(frames: u64) {
    let a = {
        let mut g = STATE.lock().unwrap();
        let Some(st) = g.as_mut() else { return };
        std::mem::take(&mut st.agg)
    };
    let f = frames.max(1) as f64;
    let ms = |ns: u64| ns as f64 / 1e6 / f;
    let gap_in: u64 = a.kind_gap.iter().sum();
    let mut s = format!(
        "passtime/frame: lists={:.1} busy={:.3}ms idle_in_list={:.3}ms idle_list_start={:.3}ms",
        a.lists as f64 / f,
        ms(a.busy),
        ms(gap_in),
        ms(a.list_gap)
    );
    for k in 0..kind::COUNT {
        s.push_str(&format!(" {}=dur {:.3} excl {:.3} idle {:.3} n {:.1};", kind::NAMES[k], ms(a.kind_dur[k]), ms(a.kind_excl[k]), ms(a.kind_gap[k]), a.kind_n[k] as f64 / f));
    }
    s.push_str(&format!(
        " idle_by_barrier none={:.3} masked={:.3} full={:.3} bad={} overflow={} no_slice={}",
        ms(a.sync_gap[0]),
        ms(a.sync_gap[1]),
        ms(a.sync_gap[2]),
        a.bad,
        a.overflow,
        NO_SLICE.swap(0, std::sync::atomic::Ordering::Relaxed)
    ));
    log::info!(target: "d3d", "{s}");
    let mut t = String::from("passtime idle by transition (ms/frame):");
    for p in 0..kind::COUNT {
        for k in 0..kind::COUNT {
            if a.trans_gap[p][k] != 0 {
                t.push_str(&format!(" {}->{}={:.3}", kind::NAMES[p], kind::NAMES[k], ms(a.trans_gap[p][k])));
            }
        }
    }
    log::info!(target: "d3d", "{t}");
    let mut subs: Vec<_> = a.subs.into_iter().collect();
    subs.sort_by_key(|x| x.0);
    for ((lord, q, w), sa) in subs {
        let n = sa.n.max(1) as f64;
        let ng = sa.n_gap.max(1) as f64;
        log::info!(
            target: "d3d",
            "passtime submit: L{} q{} w{} n/f={:.2} ecl->commit={:6.1}us commit->start={:6.1}us idle={:.3}ms/f idle_n/f={:.2} commit_after_idle={:6.1}us",
            lord,
            q,
            w,
            sa.n as f64 / f,
            sa.ecl_commit as f64 / 1e3 / n,
            sa.commit_start as f64 / 1e3 / n,
            ms(sa.gap),
            sa.n_gap as f64 / f,
            sa.commit_vs_idle as f64 / 1e3 / ng,
        );
    }
    let mut v: Vec<_> = a.passes.into_iter().collect();
    v.sort_by(|x, y| (y.1.excl + y.1.gap).cmp(&(x.1.excl + x.1.gap)));
    for ((lord, ord, k, w, h), p) in v.into_iter().take(60) {
        let n = p.n.max(1) as f64;
        log::info!(
            target: "d3d",
            "passtime: excl={:6.3} dur={:6.3} idle={:6.3}ms/f n/f={:.2} mean={:6.1}us L{}#{} {} {}x{} draws={:.1} disp={:.1} icb={:.1} sync={}/{}/{}",
            ms(p.excl),
            ms(p.dur),
            ms(p.gap),
            p.n as f64 / f,
            p.dur as f64 / 1e3 / n,
            lord,
            ord,
            kind::NAMES[(k as usize).min(kind::COUNT - 1)],
            w,
            h,
            p.draws as f64 / n,
            p.dispatches as f64 / n,
            p.icb as f64 / n,
            p.sync[0],
            p.sync[1],
            p.sync[2],
        );
    }
    let total: u64 = a.shaders.values().map(|p| p.dur).sum();
    let mut v: Vec<_> = a.shaders.into_iter().collect();
    v.sort_by(|x, y| y.1.dur.cmp(&x.1.dur));
    for ((k, key, w, h), p) in v.into_iter().take(60) {
        let n = p.n.max(1) as f64;
        let suffix = synth::name(key).map_or_else(String::new, |tag| format!(" [{tag}]"));
        log::info!(
            target: "d3d",
            "passtime shader: dur={:6.3}ms/f ({:4.1}%) n/f={:5.1} mean={:6.1}us {} {:016x} {}x{} draws={:.1} disp={:.1} icb={:.1}{suffix}",
            ms(p.dur),
            p.dur as f64 * 100.0 / total.max(1) as f64,
            p.n as f64 / f,
            p.dur as f64 / 1e3 / n,
            kind::NAMES[(k as usize).min(kind::COUNT - 1)],
            key,
            w,
            h,
            p.draws as f64 / n,
            p.dispatches as f64 / n,
            p.icb as f64 / n,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_queue_light_isolates_q3_timeline_and_coalesces_zero_work_blit_stamps() {
        // 1 GHz synthetic clock (`1 tick == 1 ns`) for exact nanosecond assertions.
        let freq = 1_000_000_000_u128;

        let q0_pass0 = [PassMeta {
            kind: kind::COMPUTE,
            sync: 2,
            dispatches: 4,
            shader: 0x1111,
            ..Default::default()
        }];
        // Interleaved async `Copy` (`q3`) DMA running past `q0_pass0` (`[1_200, 5_000]`),
        // followed by a second `q0` submission starting at `2_500..4_500`.
        let q3_copy = [PassMeta {
            kind: kind::UTIL,
            sync: 1,
            shader: synth::COPY_BLIT,
            ..Default::default()
        }];
        let q0_pass1 = [PassMeta {
            kind: kind::DRAW,
            sync: 2,
            draws: 10,
            w: 1280,
            h: 720,
            shader: 0x2222,
            ..Default::default()
        }];

        // Legacy (`copy_light = false`): `q3` advances shared `st.last_end` to `5_000`,
        // erasing `q0_pass1`'s true `500 ns` idle gap (`2_500 - 2_000`) AND hiding `q0_pass1`'s
        // exclusive GPU busy time (`[2_500, 4_500]` lies inside `5_000`)!
        let mut st_legacy = State { agg: Agg::default(), last_end: 0, last_end_by_q: [0; 4] };
        resolve_slice_into(&mut st_legacy, 0, 0, 0, 900, 950, &q0_pass0, &[1_000, 2_000], false, freq);
        resolve_slice_into(&mut st_legacy, 1, 3, 0, 950, 980, &q3_copy, &[1_200, 5_000], false, freq);
        resolve_slice_into(&mut st_legacy, 2, 0, 0, 2_100, 2_200, &q0_pass1, &[2_500, 4_500], false, freq);
        assert_eq!(st_legacy.last_end, 5_000);
        assert_eq!(st_legacy.agg.list_gap, 0, "legacy shared timeline loses q0 idle gap behind q3 end");
        assert_eq!(st_legacy.agg.kind_excl[kind::DRAW as usize], 0);

        // `AW2_COPY_QUEUE_LIGHT=1` (`copy_light = true`): `q3` tracks its own `last_end_by_q[3] = 5_000`
        // without polluting `st.last_end` (`2_000 -> 4_500`), preserving `q0`'s exact `500 ns` `list_gap`
        // and `2_000 ns` `DRAW` exclusive busy time (`busy = 1_000 + 2_000 = 3_000 ns`)!
        let mut st_iso = State { agg: Agg::default(), last_end: 0, last_end_by_q: [0; 4] };
        resolve_slice_into(&mut st_iso, 0, 0, 0, 900, 950, &q0_pass0, &[1_000, 2_000], true, freq);
        resolve_slice_into(&mut st_iso, 1, 3, 0, 950, 980, &q3_copy, &[1_200, 5_000], true, freq);
        resolve_slice_into(&mut st_iso, 2, 0, 0, 2_100, 2_200, &q0_pass1, &[2_500, 4_500], true, freq);
        assert_eq!(st_iso.last_end, 4_500);
        assert_eq!(st_iso.last_end_by_q[3], 5_000);
        assert_eq!(st_iso.agg.list_gap, 500, "isolated q0 timeline must preserve exact 500ns list_gap");
        assert_eq!(st_iso.agg.kind_excl[kind::DRAW as usize], 2_000);
        assert_eq!(st_iso.agg.busy, 3_000);
        assert_eq!(st_iso.agg.kind_dur[kind::UTIL as usize], 3_800, "q3 pass duration still reported in kind_dur");

        // Zero-work render/clear/util and zero-dispatch compute/Blit encoders missing one
        // boundary timestamp (`[0, 6_000]` or `[6_000, u64::MAX]`) coalesce cleanly (`bad == 0`,
        // `dur == 0`) regardless of `copy_light`, while non-empty passes (`draws > 0` /
        // `dispatches > 0`) still flag `bad == 1` on missing or inverted timestamps.
        let zero_work_clear = [PassMeta {
            kind: kind::CLEAR,
            sync: 1,
            shader: synth::CLEAR_PASS,
            ..Default::default()
        }];
        let zero_work_cs = [PassMeta {
            kind: kind::COMPUTE,
            sync: 1,
            shader: synth::COPY_BLIT,
            ..Default::default()
        }];
        let real_work_cs = [PassMeta {
            kind: kind::COMPUTE,
            sync: 1,
            dispatches: 1,
            shader: 0x3333,
            ..Default::default()
        }];

        resolve_slice_into(&mut st_legacy, 3, 0, 0, 5_000, 5_100, &zero_work_clear, &[0, 5_500], false, freq);
        resolve_slice_into(&mut st_legacy, 4, 0, 0, 5_500, 5_600, &zero_work_cs, &[6_000, u64::MAX], false, freq);
        assert_eq!(st_legacy.agg.bad, 0, "0-work clear and 0-work compute must coalesce even when copy_light=false");

        resolve_slice_into(&mut st_iso, 3, 0, 0, 5_000, 5_100, &zero_work_cs, &[6_000, u64::MAX], true, freq);
        resolve_slice_into(&mut st_iso, 4, 3, 0, 5_200, 5_300, &zero_work_cs, &[0, 6_500], true, freq);
        assert_eq!(st_iso.agg.bad, 0, "0-work Blit/compute boundary missing one endpoint must coalesce cleanly");

        resolve_slice_into(&mut st_iso, 5, 0, 0, 7_000, 7_100, &real_work_cs, &[7_500, u64::MAX], true, freq);
        assert_eq!(st_iso.agg.bad, 1, "non-empty dispatch missing end timestamp must still count as bad");
    }

    #[test]
    fn quiescent_heap_pool_counts_and_deferred_invalidation() {
        let mut p = PoolState::new(HEAPS);
        // First 3 acquisitions pack onto heap 0 (`slices 0, 1, 2`), leaving heaps 1..7 untouched.
        let s0 = p.acquire_slice().expect("s0");
        let s1 = p.acquire_slice().expect("s1");
        let s2 = p.acquire_slice().expect("s2");
        assert_eq!((s0, s1, s2), (0, 1, 2));
        assert_eq!(p.active_count[0], 3);
        assert_eq!(p.clean_mask[0], 0b1111_1000);

        // Dropping `s0` (`used = 4`) while `s1` and `s2` are still active MUST NOT invalidate heap 0,
        // and MUST NOT return slice 0 to `clean_mask[0]` prematurely.
        assert_eq!(p.release_slice(s0, 4), None);
        assert_eq!(p.active_count[0], 2);
        assert_eq!(p.dirty_mask[0], 0b0000_0001);
        assert_eq!(p.clean_mask[0], 0b1111_1000);

        // Next acquisition on active heap 0 skips dirty slice 0 and takes clean slice 3.
        let s3 = p.acquire_slice().expect("s3");
        assert_eq!(s3, 3);
        assert_eq!(p.active_count[0], 3);

        // Dropping an unused slice (`s2`, `used = 0`) returns its bit directly to `clean_mask[0]`
        // without invalidating while `s1` and `s3` remain active.
        assert_eq!(p.release_slice(s2, 0), None);
        assert_eq!(p.active_count[0], 2);
        assert_eq!(p.clean_mask[0] & (1 << 2), 1 << 2);

        // Dropping `s1` (`used = 2`) still leaves `s3` active -> still no invalidation!
        assert_eq!(p.release_slice(s1, 2), None);
        assert_eq!(p.active_count[0], 1);
        assert_eq!(p.dirty_mask[0], 0b0000_0011);

        // Dropping the LAST active slice `s3` (`used = 6`) brings `active_count[0] == 0` with
        // `dirty_mask[0] != 0` -> triggers exactly ONE quiescent whole-heap invalidation on heap 0
        // and restores all 8 slices of heap 0 to `clean_mask[0] == 0xFF`!
        assert_eq!(p.release_slice(s3, 6), Some(0));
        assert_eq!(p.active_count[0], 0);
        assert_eq!(p.dirty_mask[0], 0);
        assert_eq!(p.clean_mask[0], 0xFF);

        // Next burst after heap 0 drained advances FIFO ring cursor to heap 1 (`slice == 8`).
        let next = p.acquire_slice().expect("next burst");
        assert_eq!(next, PER_HEAP as u32);
        assert_eq!(p.release_slice(next, 0), None, "0-used quiescent release needs no invalidation");

        // Exhaust all 64 slices across all 8 heaps, verify 65th returns None, then drain cleanly.
        let mut all = Vec::new();
        while let Some(sl) = p.acquire_slice() {
            all.push(sl);
        }
        assert_eq!(all.len(), HEAPS * PER_HEAP);
        assert_eq!(p.acquire_slice(), None);
        let mut invalidated_heaps = Vec::new();
        for sl in all {
            if let Some(h) = p.release_slice(sl, 2) {
                invalidated_heaps.push(h);
            }
        }
        invalidated_heaps.sort_unstable();
        assert_eq!(invalidated_heaps, (0..HEAPS).collect::<Vec<_>>());

        if super::super::device().is_some() {
            let (heaps, per_heap) = dedicated_heap_pool_counts().expect("passtiming shared pool");
            assert_eq!(heaps, 8, "passtiming must allocate strictly 8 MTL4CounterHeaps on AGX");
            assert_eq!(per_heap, 8, "each 4096-entry heap must expose 8 slices of 512 timestamps");
        }
    }
}
