//! Low-overhead profiler for the `x86-64` interpreter ([`crate::emu`]).
//!
//! Answers "which guest code is the interpreter burning time on, and which
//! instructions / native calls are slow?" so we know which functions are worth
//! porting to Rust (see the `decompile-function` skill) or which interpreter
//! paths are worth optimizing.
//!
//! `AW2_EMU_PROF` selects the level:
//! - `0` (default): off (interpreter hot loop pays one predictable branch).
//! - `1`: per-entry-function call/instruction counts, statistical IP
//!   sampling every [`SAMPLE_PERIOD`] guest instructions, native host calls made
//!   from guest code (count + inclusive time), decode misses and unhandled
//!   mnemonics. Costs one decrement+branch per guest instruction plus a few
//!   relaxed stores per guest function call.
//! - `2`: additionally counts every mnemonic executed by the generic (non-fast) path;
//!   `3` counts fast-path mnemonics too (adds ~1 load+store per
//!   instruction).
//!
//! Every counter lives in a per-thread table written only by its owning thread
//! (relaxed loads/stores, no RMW, no locks) and read racily by the reporter.
//! Each slot also carries a per-frame counter tagged with the frame epoch
//! ([`on_frame`] bumps it at every `Present`), so a `[HITCH:FRAME]` report can
//! show exactly what the interpreter did during the slow frame.
//!
//! Reports are produced by [`append_frame_report`] (called by
//! `win32_rt::hitch` when a frame hitch fires) and [`append_cumulative_report`]
//! (written periodically to `out/hitch/emu-prof-<pid>.log`).

use iced_x86::Mnemonic;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

/// Guest instructions between IP samples (power of two not required).
pub const SAMPLE_PERIOD: u32 = 1024;

const FUNC_SLOTS: usize = 4096;
const IP_SLOTS: usize = 32768;
const HOST_SLOTS: usize = 1024;
const CALLER_SLOTS: usize = 4096;
const UNHANDLED_SLOTS: usize = 256;
const MNEM_SLOTS: usize = 2048;
const PROBE: usize = 32;
/// IP samples are bucketed to 16 bytes. MSVC/clang align function starts to 16, so
/// bucketing never moves a sample into the previous function, and it keeps the table
/// from overflowing in large functions.
const IP_QUANT: u64 = !0xF;

static LEVEL: OnceLock<u8> = OnceLock::new();

/// Current frame epoch (bumped once per `Present`).
static EPOCH: AtomicU32 = AtomicU32::new(1);

/// Profiling level from `AW2_EMU_PROF` (default 0: level 1 cost ~1 fps in gameplay A/B).
#[inline(always)]
pub fn level() -> u8 {
    *LEVEL.get_or_init(|| {
        std::env::var("AW2_EMU_PROF").ok().and_then(|s| s.trim().parse::<u8>().ok()).unwrap_or(0)
    })
}

/// Advance the frame epoch. Call once per `Present`, after any hitch report for the
/// frame that just ended has been produced.
#[inline(always)]
pub fn on_frame() {
    let e = EPOCH.fetch_add(1, Ordering::Relaxed);
    if e == from_frame() {
        rebase_all();
    }
}

/// `AW2_EMU_PROF_FROM_FRAME=<n>`: at frame epoch `n` the cumulative counters are rebased, so the
/// periodic `emu-prof-<pid>.log` covers only the window since then (e.g. gameplay, not loading).
fn from_frame() -> u32 {
    static F: OnceLock<u32> = OnceLock::new();
    *F.get_or_init(|| {
        std::env::var("AW2_EMU_PROF_FROM_FRAME").ok().and_then(|s| s.trim().parse().ok()).unwrap_or(0)
    })
}

static REBASED_AT: AtomicU32 = AtomicU32::new(0);

#[cold]
#[inline(never)]
fn rebase_all() {
    let threads: Vec<&'static ThreadProf> = THREADS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    for t in threads {
        for tab in [&t.funcs, &t.ips, &t.host, &t.callers, &t.unhandled] {
            tab.rebase();
        }
        t.totals.rebase();
    }
    REBASED_AT.store(from_frame(), Ordering::Relaxed);
}

#[inline(always)]
fn epoch() -> u32 {
    EPOCH.load(Ordering::Relaxed)
}

#[inline(always)]
fn bump(a: &AtomicU64, n: u64) {
    // Owner-only writer: plain load + store (no locked RMW).
    a.store(a.load(Ordering::Relaxed).wrapping_add(n), Ordering::Relaxed);
}

#[inline(always)]
fn nz(k: u64) -> u64 {
    if k == 0 { 1 } else { k }
}

#[inline(always)]
fn hash(k: u64, bits: u32) -> usize {
    (k.wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - bits)) as usize
}

/// A keyed slot with two cumulative counters plus their current-frame shadows.
struct Slot {
    key: AtomicU64,
    a: AtomicU64,
    b: AtomicU64,
    frame_epoch: AtomicU32,
    fa: AtomicU64,
    fb: AtomicU64,
    /// `a` / `b` at the `AW2_EMU_PROF_FROM_FRAME` rebase (0 = no rebase).
    ba: AtomicU64,
    bb: AtomicU64,
}

impl Slot {
    const fn new() -> Self {
        Self {
            key: AtomicU64::new(0),
            a: AtomicU64::new(0),
            b: AtomicU64::new(0),
            frame_epoch: AtomicU32::new(0),
            fa: AtomicU64::new(0),
            fb: AtomicU64::new(0),
            ba: AtomicU64::new(0),
            bb: AtomicU64::new(0),
        }
    }

    fn rebase(&self) {
        self.ba.store(self.a.load(Ordering::Relaxed), Ordering::Relaxed);
        self.bb.store(self.b.load(Ordering::Relaxed), Ordering::Relaxed);
    }

    #[inline(always)]
    fn add(&self, a: u64, b: u64) {
        bump(&self.a, a);
        bump(&self.b, b);
        let e = epoch();
        if self.frame_epoch.load(Ordering::Relaxed) != e {
            self.fa.store(a, Ordering::Relaxed);
            self.fb.store(b, Ordering::Relaxed);
            self.frame_epoch.store(e, Ordering::Relaxed);
        } else {
            bump(&self.fa, a);
            bump(&self.fb, b);
        }
    }

    fn frame_vals(&self, e: u32) -> (u64, u64) {
        if self.frame_epoch.load(Ordering::Relaxed) == e {
            (self.fa.load(Ordering::Relaxed), self.fb.load(Ordering::Relaxed))
        } else {
            (0, 0)
        }
    }
}

struct Table {
    slots: Box<[Slot]>,
    bits: u32,
    /// Hits that could not find a slot (table full along the probe sequence).
    overflow: AtomicU64,
}

impl Table {
    fn new(len: usize) -> Self {
        assert!(len.is_power_of_two());
        Self {
            slots: (0..len).map(|_| Slot::new()).collect(),
            bits: len.trailing_zeros(),
            overflow: AtomicU64::new(0),
        }
    }

    #[inline(always)]
    fn add(&self, key: u64, a: u64, b: u64) {
        let key = nz(key); // key 0 marks an empty slot
        let mask = self.slots.len() - 1;
        let mut i = hash(key, self.bits);
        for _ in 0..PROBE {
            let s = &self.slots[i];
            let k = s.key.load(Ordering::Relaxed);
            if k == key {
                s.add(a, b);
                return;
            }
            if k == 0 {
                s.key.store(key, Ordering::Relaxed);
                s.add(a, b);
                return;
            }
            i = (i + 1) & mask;
        }
        bump(&self.overflow, 1);
    }

    /// Count one hit for `key` and store `tag` in `b` (and `fb`) instead of summing it.
    #[inline(always)]
    fn add_keep_b(&self, key: u64, tag: u64) {
        let key = nz(key);
        let mask = self.slots.len() - 1;
        let mut i = hash(key, self.bits);
        for _ in 0..PROBE {
            let s = &self.slots[i];
            let k = s.key.load(Ordering::Relaxed);
            if k == 0 {
                s.key.store(key, Ordering::Relaxed);
            }
            if k == key || k == 0 {
                s.add(1, 0);
                s.b.store(tag, Ordering::Relaxed);
                return;
            }
            i = (i + 1) & mask;
        }
        bump(&self.overflow, 1);
    }

    fn for_each(&self, mut f: impl FnMut(&Slot, u64)) {
        for s in self.slots.iter() {
            let k = s.key.load(Ordering::Relaxed);
            if k != 0 {
                f(s, k);
            }
        }
    }

    fn rebase(&self) {
        for s in self.slots.iter() {
            s.rebase();
        }
    }
}

/// Per-thread profile tables. Leaked on first use (threads are long-lived and few).
pub struct ThreadProf {
    win_tid: u32,
    /// Interpreter entry functions (native -> guest): a = calls, b = guest instructions.
    funcs: Table,
    /// IP samples: a = samples.
    ips: Table,
    /// Native host functions called from guest code: a = calls, b = inclusive ns.
    host: Table,
    /// Guest call sites of native host functions: key = guest return address,
    /// a = calls, b = host function (last one seen at this site).
    callers: Table,
    /// Unhandled mnemonics: key = ip, a = hits, b = mnemonic.
    unhandled: Table,
    /// Total instructions / decode misses (cumulative + frame).
    totals: Slot,
    mnem: Box<[AtomicU64]>,
    mnem_frame: Box<[AtomicU64]>,
    mnem_epoch: AtomicU32,
    /// Remaining instructions until the next IP sample (persists across calls).
    pub(crate) sample_ctr: std::cell::Cell<u32>,
}

// SAFETY: `sample_ctr` is only touched by the owning thread; every other field is atomic.
unsafe impl Sync for ThreadProf {}

static THREADS: Mutex<Vec<&'static ThreadProf>> = Mutex::new(Vec::new());

std::thread_local! {
    static TLS_PROF: std::cell::Cell<Option<&'static ThreadProf>> = const { std::cell::Cell::new(None) };
}

/// This thread's profile tables, or `None` when profiling is disabled.
#[inline]
pub fn thread_prof() -> Option<&'static ThreadProf> {
    if level() == 0 {
        return None;
    }
    TLS_PROF.with(|c| {
        if let Some(p) = c.get() {
            return Some(p);
        }
        let p = new_thread_prof();
        c.set(Some(p));
        Some(p)
    })
}

#[cold]
fn new_thread_prof() -> &'static ThreadProf {
    let teb = unsafe { crate::exe::teb_self() };
    let win_tid = if teb != 0 { unsafe { *((teb + 0x48) as *const u32) } } else { 0 };
    let mnem_len = if level() >= 2 { MNEM_SLOTS } else { 0 };
    let p: &'static ThreadProf = Box::leak(Box::new(ThreadProf {
        win_tid,
        funcs: Table::new(FUNC_SLOTS),
        ips: Table::new(IP_SLOTS),
        host: Table::new(HOST_SLOTS),
        callers: Table::new(CALLER_SLOTS),
        unhandled: Table::new(UNHANDLED_SLOTS),
        totals: Slot::new(),
        mnem: (0..mnem_len).map(|_| AtomicU64::new(0)).collect(),
        mnem_frame: (0..mnem_len).map(|_| AtomicU64::new(0)).collect(),
        mnem_epoch: AtomicU32::new(0),
        sample_ctr: std::cell::Cell::new(SAMPLE_PERIOD),
    }));
    THREADS.lock().unwrap_or_else(|e| e.into_inner()).push(p);
    p
}

impl ThreadProf {
    /// Record a finished interpreter invocation that started at `entry_ip`.
    #[inline(always)]
    pub fn func_exit(&self, entry_ip: u64, steps: u64, decode_misses: u64) {
        self.funcs.add(entry_ip, 1, steps);
        self.totals.add(steps, decode_misses);
    }

    /// Credit instructions executed so far by a still-running invocation (called at
    /// sample points so long-running guest loops that never return are visible).
    #[inline(always)]
    pub fn func_progress(&self, entry_ip: u64, steps: u64, decode_misses: u64) {
        self.funcs.add(entry_ip, 0, steps);
        self.totals.add(steps, decode_misses);
    }

    /// Record one IP sample (called every [`SAMPLE_PERIOD`] instructions).
    #[inline(always)]
    pub fn sample_ip(&self, ip: u64) {
        self.ips.add(ip & IP_QUANT, 1, 0);
    }

    /// Record a native host call made from guest code at guest return address `ret_ip`.
    #[inline(always)]
    pub fn host_call(&self, host_fn: usize, ret_ip: u64, ns: u64) {
        self.host.add(host_fn as u64, 1, ns);
        self.callers.add_keep_b(ret_ip, host_fn as u64);
    }

    #[inline(always)]
    pub fn unhandled(&self, ip: u64, mn: Mnemonic) {
        self.unhandled.add(ip, 1, 0);
        // Remember the mnemonic in `b` (overwritten each time; same ip => same instruction).
        let mask = self.unhandled.slots.len() - 1;
        let key = nz(ip);
        let mut i = hash(key, self.unhandled.bits);
        for _ in 0..PROBE {
            let s = &self.unhandled.slots[i];
            if s.key.load(Ordering::Relaxed) == key {
                s.b.store(mn as u64, Ordering::Relaxed);
                return;
            }
            i = (i + 1) & mask;
        }
    }

    /// Count one executed mnemonic (level >= 2 only).
    #[inline(always)]
    pub fn mnemonic(&self, mn: Mnemonic) {
        let i = mn as usize;
        if i < self.mnem.len() {
            bump(&self.mnem[i], 1);
            let e = epoch();
            if self.mnem_epoch.load(Ordering::Relaxed) != e {
                for c in self.mnem_frame.iter() {
                    c.store(0, Ordering::Relaxed);
                }
                self.mnem_epoch.store(e, Ordering::Relaxed);
            }
            bump(&self.mnem_frame[i], 1);
        }
    }
}

/// Monotonic nanoseconds (commpage, no syscall).
#[inline(always)]
pub fn now_ns() -> u64 {
    std::time::Instant::now().duration_since(*START.get_or_init(std::time::Instant::now)).as_nanos() as u64
}
static START: OnceLock<std::time::Instant> = OnceLock::new();

// ---------------------------------------------------------------------------
// Reporting (cold)
// ---------------------------------------------------------------------------

/// Address resolution supplied by the caller (win32-rt owns the loader/symbols).
pub struct Resolver<'a> {
    /// Start address of the guest function containing `pc` (from `.pdata`), if known.
    pub func_start: &'a dyn Fn(u64) -> Option<u64>,
    /// Human-readable symbol for an address (guest or host).
    pub symbolize: &'a dyn Fn(u64) -> String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Window {
    Frame(u32),
    Cumulative,
}

fn vals(s: &Slot, w: Window) -> (u64, u64) {
    match w {
        Window::Frame(e) => s.frame_vals(e),
        Window::Cumulative => (
            s.a.load(Ordering::Relaxed).wrapping_sub(s.ba.load(Ordering::Relaxed)),
            s.b.load(Ordering::Relaxed).wrapping_sub(s.bb.load(Ordering::Relaxed)),
        ),
    }
}

fn fmt_count(n: u64) -> String {
    match n {
        0..=9_999 => format!("{n}"),
        10_000..=9_999_999 => format!("{:.1}k", n as f64 / 1e3),
        _ => format!("{:.1}M", n as f64 / 1e6),
    }
}

fn pct(part: u64, total: u64) -> f64 {
    if total == 0 { 0.0 } else { part as f64 * 100.0 / total as f64 }
}

fn mnemonic_name(i: usize) -> String {
    Mnemonic::try_from(i).map(|m| format!("{m:?}")).unwrap_or_else(|_| format!("#{i}"))
}

fn build_report(out: &mut String, w: Window, r: &Resolver<'_>, top: usize, indent: &str) {
    let threads: Vec<&'static ThreadProf> = THREADS.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if threads.is_empty() {
        return;
    }

    // Totals + per-thread instruction counts.
    let mut total_steps = 0u64;
    let mut total_misses = 0u64;
    let mut per_thread: Vec<(u32, u64)> = Vec::new();
    for t in &threads {
        let (steps, misses) = vals(&t.totals, w);
        total_steps += steps;
        total_misses += misses;
        if steps > 0 {
            per_thread.push((t.win_tid, steps));
        }
    }
    if total_steps == 0 {
        return;
    }
    per_thread.sort_by(|a, b| b.1.cmp(&a.1));
    let since = REBASED_AT.load(Ordering::Relaxed);
    let label = match w {
        Window::Frame(_) => "this frame".to_string(),
        Window::Cumulative if since != 0 => format!("since frame epoch {since}"),
        Window::Cumulative => "since start".to_string(),
    };
    let _ = writeln!(
        out,
        "{indent}emu: {} guest instructions {label} ({} decode misses, {:.2}% of instrs); busiest threads: {}",
        fmt_count(total_steps),
        fmt_count(total_misses),
        pct(total_misses, total_steps),
        per_thread
            .iter()
            .take(6)
            .map(|(tid, n)| format!("0x{tid:x}={:.0}%", pct(*n, total_steps)))
            .collect::<Vec<_>>()
            .join(" ")
    );

    // 1) Hot guest functions by IP samples (exclusive; where instructions actually execute).
    static FUNC_START_CACHE: OnceLock<Mutex<HashMap<u64, u64>>> = OnceLock::new();
    let mut cache = FUNC_START_CACHE.get_or_init(|| Mutex::new(HashMap::new())).lock().unwrap_or_else(|e| e.into_inner());
    let mut by_func: HashMap<u64, u64> = HashMap::new();
    let mut per_thread_funcs: HashMap<u32, HashMap<u64, u64>> = HashMap::new();
    let mut total_samples = 0u64;
    for t in &threads {
        let tf = per_thread_funcs.entry(t.win_tid).or_default();
        t.ips.for_each(|s, ip| {
            let (n, _) = vals(s, w);
            if n > 0 {
                let f = *cache.entry(ip).or_insert_with(|| (r.func_start)(ip).unwrap_or(ip));
                *by_func.entry(f).or_default() += n;
                *tf.entry(f).or_default() += n;
                total_samples += n;
            }
        });
    }
    drop(cache);
    if total_samples > 0 {
        let mut v: Vec<(u64, u64)> = by_func.into_iter().collect();
        v.sort_by(|a, b| b.1.cmp(&a.1));
        let _ = writeln!(
            out,
            "{indent}  hot guest functions (IP samples every {SAMPLE_PERIOD} instrs, {} samples; port these to Rust first):",
            total_samples
        );
        for (f, n) in v.iter().take(top) {
            let _ = writeln!(
                out,
                "{indent}    {:5.1}%  ~{:>7} instrs  0x{f:x} {}",
                pct(*n, total_samples),
                fmt_count(n * SAMPLE_PERIOD as u64),
                (r.symbolize)(*f)
            );
        }
        // Per-thread view of the busiest threads (the game/render thread is the frame's
        // critical path; spinning workers burn cores without delaying the frame).
        let per_top = if top > 10 { 12 } else { 10 };
        for (tid, _) in per_thread.iter().take(6) {
            let Some(tf) = per_thread_funcs.get(tid) else { continue };
            let tsum: u64 = tf.values().sum();
            if tsum == 0 {
                continue;
            }
            let mut tv: Vec<(u64, u64)> = tf.iter().map(|(k, v)| (*k, *v)).collect();
            tv.sort_by(|a, b| b.1.cmp(&a.1));
            let _ = writeln!(
                out,
                "{indent}  thread 0x{tid:x} hot functions ({:.0}% of samples, ~{} instrs):",
                pct(tsum, total_samples),
                fmt_count(tsum * SAMPLE_PERIOD as u64)
            );
            for (f, n) in tv.iter().take(per_top) {
                let _ = writeln!(out, "{indent}    {:5.1}%  0x{f:x} {}", pct(*n, tsum), (r.symbolize)(*f));
            }
        }
    }

    // 2) Interpreter entry points (native -> guest calls), inclusive of guest callees.
    let mut entries: HashMap<u64, (u64, u64)> = HashMap::new();
    for t in &threads {
        t.funcs.for_each(|s, k| {
            let (calls, steps) = vals(s, w);
            if calls > 0 || steps > 0 {
                let e = entries.entry(k).or_default();
                e.0 += calls;
                e.1 += steps;
            }
        });
    }
    if !entries.is_empty() {
        let mut v: Vec<(u64, (u64, u64))> = entries.into_iter().collect();
        v.sort_by(|a, b| b.1 .1.cmp(&a.1 .1));
        let _ = writeln!(out, "{indent}  interpreter entry points (native->guest, incl. guest callees):");
        for (k, (calls, steps)) in v.iter().take(top) {
            let _ = writeln!(
                out,
                "{indent}    {:5.1}%  {:>7} instrs  {:>7} calls  {:>6} instrs/call  0x{k:x} {}",
                pct(*steps, total_steps),
                fmt_count(*steps),
                fmt_count(*calls),
                fmt_count(steps / (*calls).max(1)),
                (r.symbolize)(*k)
            );
        }
    }

    // 3) Native host functions called from guest code (inclusive time).
    let mut host: HashMap<u64, (u64, u64)> = HashMap::new();
    for t in &threads {
        t.host.for_each(|s, k| {
            let (calls, ns) = vals(s, w);
            if calls > 0 {
                let e = host.entry(k).or_default();
                e.0 += calls;
                e.1 += ns;
            }
        });
    }
    if !host.is_empty() {
        let v: Vec<(String, u64, u64)> = host.into_iter().map(|(k, (c, ns))| ((r.symbolize)(k), c, ns)).collect();
        let is_blocking = |s: &str| {
            ["WaitFor", "Sleep", "GetMessage", "SignalObjectAndWait", "MsgWaitFor", "SwitchToThread", "GetQueuedCompletionStatus"]
                .iter()
                .any(|p| s.contains(p))
        };
        let (mut blocking, mut work): (Vec<_>, Vec<_>) = v.into_iter().partition(|x| is_blocking(&x.0));
        let row = |out: &mut String, (s, calls, ns): &(String, u64, u64)| {
            let _ = writeln!(
                out,
                "{indent}    {:9.2} ms  {:>7} calls  {:>8.2} us/call  {s}",
                *ns as f64 / 1e6,
                fmt_count(*calls),
                *ns as f64 / 1e3 / (*calls).max(1) as f64,
            );
        };
        work.sort_by(|a, b| b.2.cmp(&a.2));
        let _ = writeln!(out, "{indent}  native calls from guest code by time (inclusive, all threads):");
        for x in work.iter().take(top) {
            row(out, x);
        }
        work.sort_by(|a, b| b.1.cmp(&a.1));
        let _ = writeln!(out, "{indent}  native calls from guest code by count (each costs a guest->host transition):");
        for x in work.iter().take(top.min(10)) {
            row(out, x);
        }
        // Who makes those calls (guest return addresses).
        let mut sites: HashMap<u64, (u64, u64)> = HashMap::new();
        for t in &threads {
            t.callers.for_each(|s, ret| {
                let (calls, _) = vals(s, w);
                if calls > 0 {
                    let e = sites.entry(ret).or_default();
                    e.0 += calls;
                    e.1 = s.b.load(Ordering::Relaxed);
                }
            });
        }
        if !sites.is_empty() {
            let mut sv: Vec<(u64, (u64, u64))> = sites.into_iter().collect();
            sv.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
            let _ = writeln!(out, "{indent}  hottest guest call sites of native functions (return address -> callee):");
            for (ret, (calls, host_fn)) in sv.iter().take(top.min(12)) {
                let _ = writeln!(
                    out,
                    "{indent}    {:>7} calls  0x{ret:x} {}  ->  {}",
                    fmt_count(*calls),
                    (r.symbolize)(*ret),
                    (r.symbolize)(*host_fn)
                );
            }
        }
        if !blocking.is_empty() {
            blocking.sort_by(|a, b| b.2.cmp(&a.2));
            let _ = writeln!(out, "{indent}  blocking waits from guest code (idle time, not CPU):");
            for x in blocking.iter().take(top.min(8)) {
                row(out, x);
            }
        }
    }

    // 4) Unhandled mnemonics (silently skipped => potential wrong results).
    let mut unh: HashMap<u64, (u64, u64)> = HashMap::new();
    for t in &threads {
        t.unhandled.for_each(|s, ip| {
            let (hits, _) = vals(s, w);
            if hits > 0 {
                let e = unh.entry(ip).or_default();
                e.0 += hits;
                e.1 = s.b.load(Ordering::Relaxed);
            }
        });
    }
    if !unh.is_empty() {
        let mut v: Vec<(u64, (u64, u64))> = unh.into_iter().collect();
        v.sort_by(|a, b| b.1 .0.cmp(&a.1 .0));
        let _ = writeln!(out, "{indent}  UNHANDLED mnemonics (skipped, results may be wrong):");
        for (ip, (hits, mn)) in v.iter().take(top) {
            let _ = writeln!(
                out,
                "{indent}    {:>7} hits  {:<14} at 0x{ip:x} {}",
                fmt_count(*hits),
                mnemonic_name(*mn as usize),
                (r.symbolize)(*ip)
            );
        }
    }

    // 5) Mnemonic histogram (level 2).
    if level() >= 2 {
        let mut counts = vec![0u64; MNEM_SLOTS];
        for t in &threads {
            let frame_ok = match w {
                Window::Frame(e) => t.mnem_epoch.load(Ordering::Relaxed) == e,
                Window::Cumulative => true,
            };
            let src = if matches!(w, Window::Frame(_)) { &t.mnem_frame } else { &t.mnem };
            if !frame_ok {
                continue;
            }
            for (i, c) in src.iter().enumerate() {
                counts[i] += c.load(Ordering::Relaxed);
            }
        }
        let sum: u64 = counts.iter().sum();
        if sum > 0 {
            let mut v: Vec<(usize, u64)> = counts.into_iter().enumerate().filter(|x| x.1 > 0).collect();
            v.sort_by(|a, b| b.1.cmp(&a.1));
            let _ = writeln!(
                out,
                "{indent}  executed mnemonics: {}",
                v.iter()
                    .take(top * 2)
                    .map(|(i, n)| format!("{} {:.1}%", mnemonic_name(*i), pct(*n, sum)))
                    .collect::<Vec<_>>()
                    .join(", ")
            );
        }
    }

    let sum = |f: fn(&ThreadProf) -> &Table| -> u64 { threads.iter().map(|t| f(t).overflow.load(Ordering::Relaxed)).sum() };
    let (of_funcs, of_ips, of_host, of_callers) = (sum(|t| &t.funcs), sum(|t| &t.ips), sum(|t| &t.host), sum(|t| &t.callers));
    if of_funcs + of_ips + of_host + of_callers > 0 {
        let _ = writeln!(
            out,
            "{indent}  (profile tables full, dropped updates since start: funcs={} ips={} host={} callers={})",
            fmt_count(of_funcs),
            fmt_count(of_ips),
            fmt_count(of_host),
            fmt_count(of_callers)
        );
    }
}

/// Append the interpreter profile for the frame that is ending (epoch not yet bumped).
pub fn append_frame_report(out: &mut String, r: &Resolver<'_>) {
    if level() == 0 {
        return;
    }
    build_report(out, Window::Frame(epoch()), r, 8, "  ");
}

/// Append the cumulative interpreter profile since process start.
pub fn append_cumulative_report(out: &mut String, r: &Resolver<'_>) {
    if level() == 0 {
        return;
    }
    build_report(out, Window::Cumulative, r, 40, "");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_counts_and_frame_window() {
        let t = Table::new(64);
        t.add(0x1234, 1, 10);
        t.add(0x1234, 1, 5);
        t.add(0, 1, 1); // key 0 is remapped, must not be lost
        let e = epoch();
        let mut seen = 0;
        t.for_each(|s, k| {
            if k == 0x1234 {
                assert_eq!(vals(s, Window::Cumulative), (2, 15));
                assert_eq!(vals(s, Window::Frame(e)), (2, 15));
                seen += 1;
            }
        });
        assert_eq!(seen, 1);
        on_frame();
        t.add(0x1234, 1, 1);
        t.for_each(|s, k| {
            if k == 0x1234 {
                assert_eq!(vals(s, Window::Cumulative), (3, 16));
                assert_eq!(vals(s, Window::Frame(epoch())), (1, 1));
                assert_eq!(vals(s, Window::Frame(e)), (0, 0));
            }
        });
    }
}
