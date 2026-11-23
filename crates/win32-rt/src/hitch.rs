//! Zero-overhead hitch, stutter, and multi-thread stall detector.
//!
//! Designed to stay enabled at all times (including on end-user laptops) with
//! negligible steady-state cost (~5 ns per tracked boundary: a commpage clock
//! read and relaxed atomic stores, with zero heap allocations, locks, or log I/O
//! during normal frames).
//!
//! Diagnostics are emitted **only** when an anomaly occurs:
//! 1. **Slow individual operation** (`>= AW2_HITCH_OP_MS`, default `20 ms`):
//!    logs the slow PSO compile, disk read, GPU fence wait, resource allocation,
//!    or contended lock/handle wait along with the calling thread and its
//!    symbolized mixed PE (`.pdata`) + host (`rbp`) backtrace.
//! 2. **Frame-time spike / stutter** (`>= AW2_HITCH_MS`, default `50 ms`, and
//!    `>= 2.5x` rolling frame-time EMA or `>= 100 ms`):
//!    emits a per-frame breakdown at `IDXGISwapChain::Present` attributing the
//!    spike across PSO compilation (cold compiles vs archive/memory hits),
//!    command submission, GPU fence waits, drawable present, resource allocation,
//!    disk file I/O, and lock/handle waits.
//! 3. **Live multi-thread stall watchdog** (`>= AW2_STALL_MS`, default `250 ms`
//!    without a `Present`, with exponential backoff per frame):
//!    wakes a background thread that inspects all Windows threads' lock-free
//!    activity slots and captures live `SIGUSR2` mixed guest/host backtraces of
//!    stalled threads while the freeze is still happening.
//!
//! All hitch/stall reports are logged to target `hitch` (`WARN`) and appended to
//! `out/hitch/hitch-<pid>.log` (override with `AW2_HITCH_LOG=/path/to/file` or
//! disable entirely with `AW2_HITCH_MS=0`).

use std::fmt::Write as _;
use std::fs::OpenOptions;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

/// Activity category stored in each thread's lock-free `ThreadObj::act_kind` slot.
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActKind {
    Idle = 0,
    Present = 1,
    CmdSubmit = 2,
    FenceWait = 3,
    PsoCompile = 4,
    ResourceAlloc = 5,
    FileIo = 6,
    LockWait = 7,
    HandleWait = 8,
}

impl ActKind {
    pub fn from_u8(v: u8) -> Self {
        match v {
            1 => Self::Present,
            2 => Self::CmdSubmit,
            3 => Self::FenceWait,
            4 => Self::PsoCompile,
            5 => Self::ResourceAlloc,
            6 => Self::FileIo,
            7 => Self::LockWait,
            8 => Self::HandleWait,
            _ => Self::Idle,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Present => "Present",
            Self::CmdSubmit => "ExecuteCommandLists",
            Self::FenceWait => "FenceWait",
            Self::PsoCompile => "PsoCompile",
            Self::ResourceAlloc => "ResourceAlloc",
            Self::FileIo => "FileIo",
            Self::LockWait => "LockWait",
            Self::HandleWait => "HandleWait",
        }
    }
}

extern "C" {
    fn clock_gettime_nsec_np(clock_id: libc::clockid_t) -> u64;
}

/// Monotonic nanoseconds from the macOS commpage (`CLOCK_UPTIME_RAW`, no syscall).
#[inline(always)]
pub fn now_ns() -> u64 {
    unsafe { clock_gettime_nsec_np(libc::CLOCK_UPTIME_RAW) }
}

/// Span ids reported through [`set_span_hook`].
pub const SPAN_CV_WAIT: u64 = 1;
pub const SPAN_PRESENT: u64 = 2;
pub const SPAN_OBJ_WAIT: u64 = 3;

/// Optional external timeline sink (aw2-game's job profiler): `f(id, t0_ns, t1_ns)` on the
/// [`now_ns`] clock. Diagnostic only; when unset each span costs one relaxed load.
static SPAN_HOOK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn set_span_hook(f: fn(u64, u64, u64)) {
    SPAN_HOOK.store(f as usize, std::sync::atomic::Ordering::Release);
}

/// Queue operations reported through [`set_queue_hook`].
/// App-thread calls: `ExecuteCommandLists` (`a` = list count), `Signal`, `Wait` (`a` = fence, `b` = value).
pub const QOP_ECL_CALL: u32 = 0;
pub const QOP_SIGNAL_CALL: u32 = 1;
pub const QOP_WAIT_CALL: u32 = 2;
/// The same `Signal`/`Wait` when the command-stream thread hands it to Metal.
pub const QOP_SIGNAL_EXEC: u32 = 3;
pub const QOP_WAIT_EXEC: u32 = 4;

/// Optional D3D12 queue-operation sink: `f(op, queue_kind, a, b)` at [`now_ns`] (diagnostic only).
static QUEUE_HOOK: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

pub fn set_queue_hook(f: fn(u32, u32, u64, u64)) {
    QUEUE_HOOK.store(f as usize, std::sync::atomic::Ordering::Release);
}

/// Report a queue operation (one relaxed load when no hook is installed).
#[inline(always)]
pub fn queue_op(op: u32, kind: u32, a: u64, b: u64) {
    let h = QUEUE_HOOK.load(std::sync::atomic::Ordering::Relaxed);
    if h != 0 {
        let f: fn(u32, u32, u64, u64) = unsafe { std::mem::transmute(h) };
        f(op, kind, a, b);
    }
}

/// GPU commit timeline sink (see `d3d12_metal::metal::queue::GpuFeedbackHook`).
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn set_gpu_hook(f: d3d12_metal::metal::queue::GpuFeedbackHook) {
    d3d12_metal::metal::queue::set_feedback_hook(f);
}

/// Start of a hooked span (0 when no hook is installed).
#[inline(always)]
pub fn span_begin() -> u64 {
    if SPAN_HOOK.load(std::sync::atomic::Ordering::Relaxed) != 0 {
        now_ns()
    } else {
        0
    }
}

/// End a span started with [`span_begin`].
#[inline(always)]
pub fn span_end(t0: u64, id: u64) {
    if t0 != 0 {
        span_end_slow(t0, id);
    }
}

#[cold]
#[inline(never)]
fn span_end_slow(t0: u64, id: u64) {
    let h = SPAN_HOOK.load(std::sync::atomic::Ordering::Acquire);
    if h != 0 {
        let f: fn(u64, u64, u64) = unsafe { std::mem::transmute(h) };
        f(id, t0, now_ns());
    }
}

struct Config {
    enabled: bool,
    /// Frame delta threshold in nanoseconds (`AW2_HITCH_MS`, default 50 ms).
    frame_threshold_ns: u64,
    /// Single-operation threshold in nanoseconds (`AW2_HITCH_OP_MS`, default 20 ms).
    op_threshold_ns: u64,
    /// Live watchdog stall threshold in nanoseconds (`AW2_STALL_MS`, default 250 ms; 0 = off).
    stall_threshold_ns: u64,
    /// Optional log file path (`AW2_HITCH_LOG` or `out/hitch/hitch-<pid>.log`).
    log_path: Option<PathBuf>,
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name).ok().and_then(|s| s.trim().parse::<u64>().ok()).unwrap_or(default)
}

#[inline(always)]
fn cfg() -> &'static Config {
    static CFG: OnceLock<Config> = OnceLock::new();
    CFG.get_or_init(|| {
        let frame_ms = env_u64("AW2_HITCH_MS", 50);
        let op_ms = env_u64("AW2_HITCH_OP_MS", 20);
        let stall_ms = env_u64("AW2_STALL_MS", 2000);
        let enabled = frame_ms > 0;
        let log_path = if !enabled {
            None
        } else if let Some(p) = std::env::var_os("AW2_HITCH_LOG") {
            (!p.is_empty() && p != "0").then(|| PathBuf::from(p))
        } else if cfg!(test) {
            None
        } else {
            let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
            let log_dir_name = crate::config::app_config().log_dir_name;
            let dir = if repo_root.join("Cargo.toml").is_file() {
                repo_root.join("out/hitch")
            } else if let Some(home) = std::env::var_os("HOME").map(PathBuf::from) {
                home.join("Library/Logs").join(&log_dir_name).join("hitch")
            } else {
                PathBuf::from("/tmp").join(&log_dir_name).join("hitch")
            };
            Some(dir.join(format!("hitch-{}.log", std::process::id())))
        };
        Config {
            enabled,
            frame_threshold_ns: frame_ms.saturating_mul(1_000_000),
            op_threshold_ns: op_ms.saturating_mul(1_000_000),
            stall_threshold_ns: stall_ms.saturating_mul(1_000_000),
            log_path,
        }
    })
}

// ---------------------------------------------------------------------------
// Lock-free per-frame accumulators (swapped to 0 once per Present)
// ---------------------------------------------------------------------------

struct FrameAccum {
    cmd_submit_ns: AtomicU64,
    cmd_submit_cbs: AtomicU64,
    fence_wait_ns: AtomicU64,
    fence_wait_calls: AtomicU64,
    pso_ns: AtomicU64,
    pso_calls: AtomicU64,
    res_alloc_ns: AtomicU64,
    res_alloc_count: AtomicU64,
    res_alloc_bytes: AtomicU64,
    file_io_ns: AtomicU64,
    file_io_calls: AtomicU64,
    file_io_bytes: AtomicU64,
    lock_wait_ns: AtomicU64,
    lock_wait_calls: AtomicU64,
    handle_wait_ns: AtomicU64,
    handle_wait_calls: AtomicU64,
}

static ACCUM: FrameAccum = FrameAccum {
    cmd_submit_ns: AtomicU64::new(0),
    cmd_submit_cbs: AtomicU64::new(0),
    fence_wait_ns: AtomicU64::new(0),
    fence_wait_calls: AtomicU64::new(0),
    pso_ns: AtomicU64::new(0),
    pso_calls: AtomicU64::new(0),
    res_alloc_ns: AtomicU64::new(0),
    res_alloc_count: AtomicU64::new(0),
    res_alloc_bytes: AtomicU64::new(0),
    file_io_ns: AtomicU64::new(0),
    file_io_calls: AtomicU64::new(0),
    file_io_bytes: AtomicU64::new(0),
    lock_wait_ns: AtomicU64::new(0),
    lock_wait_calls: AtomicU64::new(0),
    handle_wait_ns: AtomicU64::new(0),
    handle_wait_calls: AtomicU64::new(0),
};

#[derive(Default, Clone, Copy)]
struct FrameSnapshot {
    cmd_submit_ns: u64,
    cmd_submit_cbs: u64,
    fence_wait_ns: u64,
    fence_wait_calls: u64,
    pso_ns: u64,
    pso_calls: u64,
    res_alloc_ns: u64,
    res_alloc_count: u64,
    res_alloc_bytes: u64,
    file_io_ns: u64,
    file_io_calls: u64,
    file_io_bytes: u64,
    lock_wait_ns: u64,
    lock_wait_calls: u64,
    handle_wait_ns: u64,
    handle_wait_calls: u64,
}

fn take_frame_snapshot() -> FrameSnapshot {
    FrameSnapshot {
        cmd_submit_ns: ACCUM.cmd_submit_ns.swap(0, Ordering::Relaxed),
        cmd_submit_cbs: ACCUM.cmd_submit_cbs.swap(0, Ordering::Relaxed),
        fence_wait_ns: ACCUM.fence_wait_ns.swap(0, Ordering::Relaxed),
        fence_wait_calls: ACCUM.fence_wait_calls.swap(0, Ordering::Relaxed),
        pso_ns: ACCUM.pso_ns.swap(0, Ordering::Relaxed),
        pso_calls: ACCUM.pso_calls.swap(0, Ordering::Relaxed),
        res_alloc_ns: ACCUM.res_alloc_ns.swap(0, Ordering::Relaxed),
        res_alloc_count: ACCUM.res_alloc_count.swap(0, Ordering::Relaxed),
        res_alloc_bytes: ACCUM.res_alloc_bytes.swap(0, Ordering::Relaxed),
        file_io_ns: ACCUM.file_io_ns.swap(0, Ordering::Relaxed),
        file_io_calls: ACCUM.file_io_calls.swap(0, Ordering::Relaxed),
        file_io_bytes: ACCUM.file_io_bytes.swap(0, Ordering::Relaxed),
        lock_wait_ns: ACCUM.lock_wait_ns.swap(0, Ordering::Relaxed),
        lock_wait_calls: ACCUM.lock_wait_calls.swap(0, Ordering::Relaxed),
        handle_wait_ns: ACCUM.handle_wait_ns.swap(0, Ordering::Relaxed),
        handle_wait_calls: ACCUM.handle_wait_calls.swap(0, Ordering::Relaxed),
    }
}

// Frame timing & PSO cache baselines
static LAST_PRESENT_NS: AtomicU64 = AtomicU64::new(0);
static EMA_FRAME_NS: AtomicU64 = AtomicU64::new(16_666_667); // 60 FPS initial baseline
static CURRENT_FRAME: AtomicU32 = AtomicU32::new(0);
static LAST_PRESENT_TID: AtomicU32 = AtomicU32::new(0);
static LAST_PRESENT_PTHREAD: AtomicUsize = AtomicUsize::new(0);

static PREV_PSO_ARCHIVE_HITS: AtomicU64 = AtomicU64::new(0);
static PREV_PSO_MEMORY_HITS: AtomicU64 = AtomicU64::new(0);
static PREV_PSO_COMPILES: AtomicU64 = AtomicU64::new(0);
static PREV_PSO_COMPILE_NS: AtomicU64 = AtomicU64::new(0);

static TOTAL_FRAME_HITCHES: AtomicU64 = AtomicU64::new(0);
static TOTAL_OP_HITCHES: AtomicU64 = AtomicU64::new(0);
static TOTAL_STALL_REPORTS: AtomicU64 = AtomicU64::new(0);

// ---------------------------------------------------------------------------
// RAII activity scope for potentially blocking calls
// ---------------------------------------------------------------------------

pub struct Scope {
    kind: ActKind,
    start_ns: u64,
    arg: u64,
    prev: (u8, u64, u64),
}

impl Scope {
    #[inline(always)]
    pub fn enter(kind: ActKind, arg: u64) -> Self {
        let start_ns = now_ns();
        let prev = crate::thread::enter_activity(kind as u8, start_ns, arg);
        Self { kind, start_ns, arg, prev }
    }

    #[inline(always)]
    pub fn elapsed_ns(&self) -> u64 {
        now_ns().saturating_sub(self.start_ns)
    }
}

impl Drop for Scope {
    #[inline(always)]
    fn drop(&mut self) {
        crate::thread::leave_activity(self.prev);
        let elapsed = now_ns().saturating_sub(self.start_ns);
        match self.kind {
            ActKind::FenceWait => {
                ACCUM.fence_wait_ns.fetch_add(elapsed, Ordering::Relaxed);
                ACCUM.fence_wait_calls.fetch_add(1, Ordering::Relaxed);
            }
            ActKind::LockWait => {
                ACCUM.lock_wait_ns.fetch_add(elapsed, Ordering::Relaxed);
                ACCUM.lock_wait_calls.fetch_add(1, Ordering::Relaxed);
            }
            ActKind::HandleWait => {
                ACCUM.handle_wait_ns.fetch_add(elapsed, Ordering::Relaxed);
                ACCUM.handle_wait_calls.fetch_add(1, Ordering::Relaxed);
            }
            _ => {}
        }
        let c = cfg();
        if c.enabled && elapsed >= c.op_threshold_ns && matches!(self.kind, ActKind::FenceWait | ActKind::LockWait) {
            report_slow_scope_cold(self.kind, self.arg, elapsed);
        }
    }
}

// ---------------------------------------------------------------------------
// Operation recording hooks (hot path = 1 clock difference + relaxed adds)
// ---------------------------------------------------------------------------

#[inline(always)]
pub fn record_pso(stage: &str, name: &str, elapsed_ns: u64) {
    ACCUM.pso_ns.fetch_add(elapsed_ns, Ordering::Relaxed);
    ACCUM.pso_calls.fetch_add(1, Ordering::Relaxed);
    let c = cfg();
    if c.enabled && elapsed_ns >= c.op_threshold_ns {
        report_slow_pso_cold(stage, name, elapsed_ns);
    }
}

#[inline(always)]
pub fn record_cmd_submit(num_cbs: u64, elapsed_ns: u64) {
    ACCUM.cmd_submit_ns.fetch_add(elapsed_ns, Ordering::Relaxed);
    ACCUM.cmd_submit_cbs.fetch_add(num_cbs, Ordering::Relaxed);
    let c = cfg();
    if c.enabled && elapsed_ns >= c.op_threshold_ns {
        report_slow_op_cold("ExecuteCommandLists", &format!("{num_cbs} command buffers"), elapsed_ns);
    }
}

#[inline(always)]
pub fn record_resource_alloc(bytes: u64, detail: &str, elapsed_ns: u64) {
    ACCUM.res_alloc_ns.fetch_add(elapsed_ns, Ordering::Relaxed);
    ACCUM.res_alloc_count.fetch_add(1, Ordering::Relaxed);
    ACCUM.res_alloc_bytes.fetch_add(bytes, Ordering::Relaxed);
    let c = cfg();
    if c.enabled && elapsed_ns >= c.op_threshold_ns {
        report_slow_op_cold(
            "CreateResource",
            &format!("{detail} ({:.2} MiB)", bytes as f64 / (1024.0 * 1024.0)),
            elapsed_ns,
        );
    }
}

#[inline(always)]
pub fn record_file_io(op: &str, path: &str, bytes: u64, elapsed_ns: u64) {
    ACCUM.file_io_ns.fetch_add(elapsed_ns, Ordering::Relaxed);
    ACCUM.file_io_calls.fetch_add(1, Ordering::Relaxed);
    ACCUM.file_io_bytes.fetch_add(bytes, Ordering::Relaxed);
    let c = cfg();
    if c.enabled && elapsed_ns >= c.op_threshold_ns {
        report_slow_op_cold(
            op,
            &format!("{path:?} ({} bytes)", bytes),
            elapsed_ns,
        );
    }
}

// ---------------------------------------------------------------------------
// Optional guest-CPU profiler (the x86-64 interpreter lives in aw2-game, which
// this crate cannot depend on; the loader registers these hooks).
// ---------------------------------------------------------------------------

/// Resolves a guest PC to the start of its `.pdata` function, if known.
pub type FuncStartFn<'a> = &'a dyn Fn(u64) -> Option<u64>;
/// Symbolizes a guest or host address.
pub type SymbolizeFn<'a> = &'a dyn Fn(u64) -> String;

#[derive(Clone, Copy)]
pub struct CpuProfilerHooks {
    /// Append a profile of the frame that just ended to a `[HITCH:FRAME]` report.
    pub frame_report: fn(&mut String, FuncStartFn<'_>, SymbolizeFn<'_>),
    /// Append a cumulative profile (written periodically to `emu-prof-<pid>.log`).
    pub cumulative_report: fn(&mut String, FuncStartFn<'_>, SymbolizeFn<'_>),
    /// Called once per `Present`, after any hitch report for the ending frame.
    pub on_frame: fn(),
}

static CPU_HOOKS: OnceLock<CpuProfilerHooks> = OnceLock::new();

pub fn set_cpu_profiler(h: CpuProfilerHooks) {
    let _ = CPU_HOOKS.set(h);
}

fn guest_func_start(pc: u64) -> Option<u64> {
    let (base, rf) = crate::try_loader()?.lookup_function_entry(pc)?;
    if rf.is_null() {
        return None;
    }
    // SAFETY: `rf` points into the mapped image's .pdata; unwind info lives in the same image.
    unsafe {
        let mut begin = (*rf).begin_rva;
        let mut unwind = (*rf).unwind_rva;
        // Chained fragments (UNW_FLAG_CHAININFO = 4) carry their parent RUNTIME_FUNCTION
        // after the (even-padded) unwind-code array.
        for _ in 0..8 {
            if unwind == 0 || unwind & 1 != 0 {
                break;
            }
            let ui = (base + unwind as u64) as *const u8;
            let flags = *ui >> 3;
            if flags & 4 == 0 {
                break;
            }
            let count = *ui.add(2) as usize;
            let parent = ui.add(4 + ((count + 1) & !1) * 2) as *const u32;
            begin = parent.read_unaligned();
            unwind = parent.add(2).read_unaligned();
        }
        Some(base + begin as u64)
    }
}

fn symbolize_u64(pc: u64) -> String {
    crate::symbolize(pc as usize)
}

static LAST_CPU_REPORT_NS: AtomicU64 = AtomicU64::new(0);

/// The per-frame interpreter profile (hot guest functions, native call tables) is
/// built synchronously on the presenting thread and costs 100+ ms, which itself
/// stutters the game. It is opt-in (`AW2_HITCH_CPU_REPORT=1`) and spaced at least
/// `AW2_HITCH_CPU_REPORT_S` seconds apart (default 5). The cumulative profile is
/// still written off-thread by [`ensure_prof_writer_started`].
fn append_cpu_frame_report(out: &mut String) {
    static ENABLED: OnceLock<(bool, u64)> = OnceLock::new();
    let (enabled, min_gap_ns) = *ENABLED.get_or_init(|| {
        (env_u64("AW2_HITCH_CPU_REPORT", 0) != 0, env_u64("AW2_HITCH_CPU_REPORT_S", 5).saturating_mul(1_000_000_000))
    });
    if !enabled {
        return;
    }
    let now = now_ns();
    let last = LAST_CPU_REPORT_NS.load(Ordering::Relaxed);
    if last != 0 && now.saturating_sub(last) < min_gap_ns {
        return;
    }
    LAST_CPU_REPORT_NS.store(now, Ordering::Relaxed);
    if let Some(h) = CPU_HOOKS.get() {
        (h.frame_report)(out, &guest_func_start, &symbolize_u64);
    }
}

static PROF_WRITER_STARTED: AtomicBool = AtomicBool::new(false);

/// Periodically (every `AW2_EMU_PROF_INTERVAL_S`, default 10 s) overwrite
/// `out/hitch/emu-prof-<pid>.log` with the cumulative interpreter profile.
fn ensure_prof_writer_started() {
    if CPU_HOOKS.get().is_none() {
        return;
    }
    let Some(log_path) = cfg().log_path.clone() else { return };
    if PROF_WRITER_STARTED.compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed).is_err() {
        return;
    }
    let interval = env_u64("AW2_EMU_PROF_INTERVAL_S", 10).max(1);
    let path = log_path.with_file_name(format!("emu-prof-{}.log", std::process::id()));
    let _ = std::thread::Builder::new().name("aw2-emu-prof".into()).spawn(move || loop {
        std::thread::sleep(Duration::from_secs(interval));
        let Some(h) = CPU_HOOKS.get() else { return };
        let mut out = String::new();
        let frame = CURRENT_FRAME.load(Ordering::Relaxed);
        let _ = writeln!(out, "# aw2 interpreter profile (cumulative) at frame #{frame}");
        let _ = writeln!(out, "# AW2_EMU_PROF=2 adds a mnemonic histogram; AW2_EMU_PROF=0 disables.");
        (h.cumulative_report)(&mut out, &guest_func_start, &symbolize_u64);
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let tmp = path.with_extension("log.tmp");
        if std::fs::write(&tmp, out.as_bytes()).is_ok() {
            let _ = std::fs::rename(&tmp, &path);
        }
    });
}

/// Called once per `IDXGISwapChain::Present` after `sc.present(sync)` completes.
#[inline(always)]
pub fn on_present(frame: u32, present_drawable_ns: u64) {
    on_present_inner(frame, present_drawable_ns);
    if let Some(h) = CPU_HOOKS.get() {
        (h.on_frame)();
    }
}

#[inline(always)]
fn on_present_inner(frame: u32, present_drawable_ns: u64) {
    let now = now_ns();
    let prev = LAST_PRESENT_NS.swap(now, Ordering::Relaxed);
    CURRENT_FRAME.store(frame, Ordering::Relaxed);

    let tid = if crate::teb::current_raw().is_null() { 0 } else { crate::teb::thread_id() };
    LAST_PRESENT_TID.store(tid, Ordering::Relaxed);
    LAST_PRESENT_PTHREAD.store(unsafe { libc::pthread_self() as usize }, Ordering::Relaxed);

    let snap = take_frame_snapshot();
    let (d_arch, d_mem, d_comp, d_comp_ns) = poll_pso_stats_delta();

    let c = cfg();
    if !c.enabled {
        return;
    }
    ensure_watchdog_started();
    ensure_prof_writer_started();

    if prev == 0 {
        return;
    }
    let dt_ns = now.saturating_sub(prev);
    let ema_ns = EMA_FRAME_NS.load(Ordering::Relaxed);

    // Warmup first 4 frames before triggering frame-spike alerts.
    // On iOS, sustained gameplay baselines can sit at 40-80 ms; raise the unconditional
    // absolute floor to 250 ms and rate-limit non-extreme reports to 1/s so synchronous
    // hitch file I/O never slows down the presenting thread every frame.
    let abs_floor_ns = if cfg!(target_os = "ios") { 250_000_000 } else { 100_000_000 };
    let is_hitch = frame > 4
        && dt_ns >= c.frame_threshold_ns
        && (dt_ns >= (ema_ns.saturating_mul(5)) / 2 || dt_ns >= abs_floor_ns);

    if is_hitch {
        static LAST_FRAME_HITCH_NS: AtomicU64 = AtomicU64::new(0);
        let allow_emit = if cfg!(target_os = "ios") && dt_ns < 500_000_000 {
            let last = LAST_FRAME_HITCH_NS.load(Ordering::Relaxed);
            last == 0 || now.saturating_sub(last) >= 1_000_000_000
        } else {
            true
        };
        if allow_emit {
            LAST_FRAME_HITCH_NS.store(now, Ordering::Relaxed);
            report_frame_hitch_cold(
                frame,
                dt_ns,
                ema_ns,
                present_drawable_ns,
                &snap,
                (d_arch, d_mem, d_comp, d_comp_ns),
            );
            // Reset the frame start timestamp after emitting the hitch report so
            // symbolization/logging time is not charged to the next frame's dt_ns.
            LAST_PRESENT_NS.store(now_ns(), Ordering::Relaxed);
        }
    }

    // Update EMA (clamped to 200 ms so a single loading pause doesn't blind subsequent hitch detection).
    let clamped_dt = dt_ns.min(200_000_000);
    let next_ema = (ema_ns.saturating_mul(7).saturating_add(clamped_dt)) / 8;
    EMA_FRAME_NS.store(next_ema.max(1_000_000), Ordering::Relaxed);
}

fn poll_pso_stats_delta() -> (u64, u64, u64, u64) {
    let Ok(cache) = d3d12_metal::metal::pipeline::pso_cache() else {
        return (0, 0, 0, 0);
    };
    let st = cache.stats();
    let prev_a = PREV_PSO_ARCHIVE_HITS.swap(st.archive_hits, Ordering::Relaxed);
    let prev_m = PREV_PSO_MEMORY_HITS.swap(st.memory_hits, Ordering::Relaxed);
    let prev_c = PREV_PSO_COMPILES.swap(st.compiles, Ordering::Relaxed);
    let prev_n = PREV_PSO_COMPILE_NS.swap(st.compile_nanos, Ordering::Relaxed);
    (
        st.archive_hits.saturating_sub(prev_a),
        st.memory_hits.saturating_sub(prev_m),
        st.compiles.saturating_sub(prev_c),
        st.compile_nanos.saturating_sub(prev_n),
    )
}

// ---------------------------------------------------------------------------
// Cold reporting paths (only executed when a hitch or stall actually happens)
// ---------------------------------------------------------------------------

static LOG_LOCK: Mutex<()> = Mutex::new(());

fn emit_report(text: &str) {
    for line in text.lines() {
        log::warn!(target: "hitch", "{line}");
    }
    let Some(path) = &cfg().log_path else { return };
    let _g = LOG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(mut f) = OpenOptions::new().create(true).append(true).open(path) {
        let _ = f.write_all(text.as_bytes());
        if !text.ends_with('\n') {
            let _ = f.write_all(b"\n");
        }
    }
}

fn current_thread_summary() -> String {
    let tid = if crate::teb::current_raw().is_null() { 0 } else { crate::teb::thread_id() };
    let name = crate::thread::current_thread_object()
        .and_then(|t| t.name.try_lock().ok().map(|n| n.clone()))
        .unwrap_or_default();
    if name.is_empty() {
        format!("tid 0x{tid:x}")
    } else {
        format!("tid 0x{tid:x} {name:?}")
    }
}

fn format_inline_backtrace(max_frames: usize) -> String {
    let pcs = crate::crash::walk_current(max_frames + 2);
    let mut out = String::new();
    // Skip the top hitch/crash frames so the caller's Win32/guest frames come first.
    for (idx, pc) in pcs.into_iter().skip(2).take(max_frames).enumerate() {
        let _ = writeln!(out, "    #{idx:<2} {}", crate::symbolize(pc as usize));
    }
    out
}

#[cold]
#[inline(never)]
fn report_slow_pso_cold(stage: &str, name: &str, elapsed_ns: u64) {
    TOTAL_OP_HITCHES.fetch_add(1, Ordering::Relaxed);
    let frame = CURRENT_FRAME.load(Ordering::Relaxed);
    let ms = elapsed_ns as f64 / 1_000_000.0;
    let thr = current_thread_summary();
    let bt = format_inline_backtrace(10);
    let msg = format!(
        "[HITCH:OP] Frame #{frame} slow PSO compile ({stage} {name:?}) took {ms:.2} ms on {thr}\n{bt}"
    );
    emit_report(&msg);
}

#[cold]
#[inline(never)]
fn report_slow_op_cold(op: &str, detail: &str, elapsed_ns: u64) {
    TOTAL_OP_HITCHES.fetch_add(1, Ordering::Relaxed);
    let frame = CURRENT_FRAME.load(Ordering::Relaxed);
    let ms = elapsed_ns as f64 / 1_000_000.0;
    let thr = current_thread_summary();
    let bt = format_inline_backtrace(10);
    let msg = format!(
        "[HITCH:OP] Frame #{frame} slow {op} ({detail}) took {ms:.2} ms on {thr}\n{bt}"
    );
    emit_report(&msg);
}

#[cold]
#[inline(never)]
fn report_slow_scope_cold(kind: ActKind, arg: u64, elapsed_ns: u64) {
    let render_tid = LAST_PRESENT_TID.load(Ordering::Relaxed);
    let my_tid = if crate::teb::current_raw().is_null() { 0 } else { crate::teb::thread_id() };
    // Background worker threads legitimately park on locks/fences; only report individual
    // scope waits on the render/main thread (or before the first present).
    if render_tid != 0 && my_tid != render_tid {
        return;
    }
    TOTAL_OP_HITCHES.fetch_add(1, Ordering::Relaxed);
    let frame = CURRENT_FRAME.load(Ordering::Relaxed);
    let ms = elapsed_ns as f64 / 1_000_000.0;
    let thr = current_thread_summary();
    let detail = describe_activity_arg(kind, arg);
    let bt = format_inline_backtrace(10);
    let msg = format!(
        "[HITCH:OP] Frame #{frame} slow {} ({detail}) took {ms:.2} ms on {thr}\n{bt}",
        kind.label()
    );
    emit_report(&msg);
}

fn describe_activity_arg(kind: ActKind, arg: u64) -> String {
    match kind {
        ActKind::FenceWait => format!("fence_val={arg}"),
        ActKind::LockWait => {
            // If `arg` points to a CriticalSection, check its `owning_thread` (+0x10).
            if let Some(owner) = crate::crash::read_u64(arg.wrapping_add(0x10)) {
                if owner != 0 && owner < 0x1000_0000 {
                    let owner_name = crate::thread::find_by_tid(owner as u32)
                        .and_then(|t| t.name.try_lock().ok().map(|n| n.clone()))
                        .unwrap_or_default();
                    return format!("lock=0x{arg:x} owner_tid=0x{owner:x} {owner_name:?}");
                }
            }
            format!("lock=0x{arg:x}")
        }
        ActKind::HandleWait => format!("handles={arg}"),
        ActKind::FileIo => format!("bytes={arg}"),
        ActKind::ResourceAlloc => format!("bytes={arg}"),
        ActKind::CmdSubmit => format!("cbs={arg}"),
        _ => format!("arg=0x{arg:x}"),
    }
}

#[cold]
#[inline(never)]
fn report_frame_hitch_cold(
    frame: u32,
    dt_ns: u64,
    ema_ns: u64,
    present_drawable_ns: u64,
    s: &FrameSnapshot,
    (d_arch, d_mem, d_comp, d_comp_ns): (u64, u64, u64, u64),
) {
    TOTAL_FRAME_HITCHES.fetch_add(1, Ordering::Relaxed);
    let dt_ms = dt_ns as f64 / 1_000_000.0;
    let ema_ms = ema_ns as f64 / 1_000_000.0;
    let spike_ms = dt_ms - ema_ms;
    let present_ms = present_drawable_ns as f64 / 1_000_000.0;
    let cmd_ms = s.cmd_submit_ns as f64 / 1_000_000.0;
    let fence_ms = s.fence_wait_ns as f64 / 1_000_000.0;
    let pso_ms = s.pso_ns.max(d_comp_ns) as f64 / 1_000_000.0;
    let res_ms = s.res_alloc_ns as f64 / 1_000_000.0;
    let io_ms = s.file_io_ns as f64 / 1_000_000.0;
    let lock_ms = s.lock_wait_ns as f64 / 1_000_000.0;
    let handle_ms = s.handle_wait_ns as f64 / 1_000_000.0;
    let cause = dominant_cause(dt_ns, present_drawable_ns, s, d_comp_ns);
    let thr = current_thread_summary();

    let mut out = String::new();
    let _ = writeln!(
        out,
        "[HITCH:FRAME] Frame #{frame} took {dt_ms:.1} ms (baseline {ema_ms:.1} ms, +{spike_ms:.1} ms) cause={cause} on {thr}:"
    );
    let _ = writeln!(
        out,
        "  present={present_ms:.1}ms | cmd_submit={cmd_ms:.1}ms ({} cbs) | fence_wait={fence_ms:.1}ms ({} waits) | pso={pso_ms:.1}ms ({} calls: {d_comp} cold, {d_arch} archive, {d_mem} mem) | res_alloc={res_ms:.1}ms ({} res, {:.2} MiB) | file_io={io_ms:.1}ms ({} ops, {:.2} MiB) | lock_wait={lock_ms:.1}ms ({} waits) | handle_wait={handle_ms:.1}ms ({} waits)",
        s.cmd_submit_cbs,
        s.fence_wait_calls,
        s.pso_calls,
        s.res_alloc_count,
        s.res_alloc_bytes as f64 / (1024.0 * 1024.0),
        s.file_io_calls,
        s.file_io_bytes as f64 / (1024.0 * 1024.0),
        s.lock_wait_calls,
        s.handle_wait_calls,
    );

    // Also list any concurrent threads that are mid-activity right now.
    let now = now_ns();
    let active = active_threads_summary(now);
    if !active.is_empty() {
        let _ = writeln!(out, "  concurrent thread activity: {active}");
    }
    append_cpu_frame_report(&mut out);
    emit_report(&out);
}

fn dominant_cause(dt_ns: u64, present_ns: u64, s: &FrameSnapshot, pso_compile_ns: u64) -> &'static str {
    let pso = s.pso_ns.max(pso_compile_ns);
    let candidates = [
        (pso, "PSO_COMPILE"),
        (s.file_io_ns, "FILE_IO"),
        (s.fence_wait_ns, "GPU_FENCE_WAIT"),
        (s.lock_wait_ns, "LOCK_CONTENTION"),
        (s.res_alloc_ns, "RESOURCE_ALLOC"),
        (s.cmd_submit_ns, "CMD_SUBMIT"),
        (present_ns, "PRESENT_DRAWABLE"),
    ];
    let (best_ns, label) = candidates.into_iter().max_by_key(|c| c.0).unwrap_or((0, "CPU_GAME_LOGIC"));
    if best_ns * 4 >= dt_ns {
        label
    } else {
        "CPU_GAME_LOGIC"
    }
}

fn active_threads_summary(now_ns: u64) -> String {
    let mut parts = Vec::new();
    for t in crate::thread::all_threads() {
        let k = ActKind::from_u8(t.act_kind.load(Ordering::Acquire));
        if k == ActKind::Idle {
            continue;
        }
        let start = t.act_start_ns.load(Ordering::Relaxed);
        let arg = t.act_arg.load(Ordering::Relaxed);
        let dur_ms = now_ns.saturating_sub(start) as f64 / 1_000_000.0;
        let tid = t.tid.load(Ordering::Relaxed);
        let name = t.name.try_lock().map(|n| n.clone()).unwrap_or_default();
        parts.push(format!(
            "0x{tid:x} {name:?} in {}({}, {dur_ms:.1}ms)",
            k.label(),
            describe_activity_arg(k, arg)
        ));
    }
    parts.join(", ")
}

// ---------------------------------------------------------------------------
// Live Multi-Thread Stall Watchdog (zero game-thread overhead)
// ---------------------------------------------------------------------------

static WATCHDOG_STARTED: AtomicBool = AtomicBool::new(false);

fn ensure_watchdog_started() {
    let c = cfg();
    if !c.enabled || c.stall_threshold_ns == 0 {
        return;
    }
    if WATCHDOG_STARTED.compare_exchange(false, true, Ordering::AcqRel, Ordering::Relaxed).is_ok() {
        let _ = std::thread::Builder::new().name("aw2-hitch-watchdog".into()).spawn(watchdog_loop);
    }
}

fn watchdog_loop() {
    let base_stall_ns = cfg().stall_threshold_ns;
    let mut watched_frame = u32::MAX;
    let mut next_threshold_ns = base_stall_ns;
    let idle_sleep = Duration::from_nanos((base_stall_ns / 2).clamp(100_000_000, 500_000_000));

    loop {
        let last_ns = LAST_PRESENT_NS.load(Ordering::Relaxed);
        if last_ns == 0 {
            std::thread::sleep(idle_sleep);
            continue;
        }
        let frame = CURRENT_FRAME.load(Ordering::Relaxed);
        if frame != watched_frame {
            watched_frame = frame;
            next_threshold_ns = base_stall_ns;
        }
        let now = now_ns();
        let elapsed_ns = now.saturating_sub(last_ns);
        if elapsed_ns < next_threshold_ns {
            let remaining = next_threshold_ns - elapsed_ns;
            // Sleep until the current threshold could fire, capped at `base_stall_ns`
            // so a new frame stalling right after wake is still detected promptly.
            let sleep_ns = remaining.min(base_stall_ns).max(50_000_000);
            std::thread::sleep(Duration::from_nanos(sleep_ns));
            continue;
        }
        report_live_stall(frame, elapsed_ns, now);
        // Exponential backoff (e.g. 250ms -> 1s -> 4s -> 16s) for the same frame so
        // long loading screens don't spam the log; afterwards report every 30 s.
        next_threshold_ns = next_threshold_ns
            .saturating_mul(4)
            .min(elapsed_ns.saturating_add(30_000_000_000))
            .max(elapsed_ns.saturating_add(base_stall_ns));
    }
}

#[cold]
#[inline(never)]
fn report_live_stall(frame: u32, stall_ns: u64, now: u64) {
    TOTAL_STALL_REPORTS.fetch_add(1, Ordering::Relaxed);
    let stall_ms = stall_ns as f64 / 1_000_000.0;
    let render_tid = LAST_PRESENT_TID.load(Ordering::Relaxed);
    let render_pt = LAST_PRESENT_PTHREAD.load(Ordering::Relaxed);

    let mut out = String::new();
    let _ = writeln!(
        out,
        "==================== [HITCH:STALL] No Present for {stall_ms:.1} ms (after Frame #{frame}, render tid 0x{render_tid:x}) ===================="
    );

    let threads = crate::thread::all_threads();
    let mut to_snapshot: Vec<(String, usize)> = Vec::new();
    let mut lock_owner_tid: u32 = 0;

    for t in &threads {
        let tid = t.tid.load(Ordering::Relaxed) as u32;
        let pt = t.pthread.load(Ordering::Relaxed);
        let name = t.name.try_lock().map(|n| n.clone()).unwrap_or_default();
        let kind = ActKind::from_u8(t.act_kind.load(Ordering::Acquire));
        let start = t.act_start_ns.load(Ordering::Relaxed);
        let arg = t.act_arg.load(Ordering::Relaxed);

        if kind != ActKind::Idle {
            let dur_ms = now.saturating_sub(start) as f64 / 1_000_000.0;
            let detail = describe_activity_arg(kind, arg);
            let _ = writeln!(
                out,
                "  thread 0x{tid:x} {name:?}: active in {} ({detail}) for {dur_ms:.1} ms",
                kind.label()
            );
            if kind == ActKind::LockWait && tid == render_tid {
                if let Some(owner) = crate::crash::read_u64(arg.wrapping_add(0x10)) {
                    lock_owner_tid = owner as u32;
                }
            }
        }

        // Snapshot stack if this is the render thread, a lock owner blocking the render thread,
        // or a thread actively executing a tracked non-wait operation (PSO, FileIo, CmdSubmit, ResourceAlloc).
        let interesting = tid == render_tid
            || (lock_owner_tid != 0 && tid == lock_owner_tid)
            || matches!(
                kind,
                ActKind::Present | ActKind::CmdSubmit | ActKind::PsoCompile | ActKind::ResourceAlloc | ActKind::FileIo
            );
        if interesting && pt != 0 && to_snapshot.len() < 6 {
            to_snapshot.push((format!("thread 0x{tid:x} {name:?} ({})", kind.label()), pt));
        }
    }

    // If the render thread wasn't in `threads` (or if nothing was queued), ensure we snapshot `render_pt` and `main_pthread`.
    if render_pt != 0 && !to_snapshot.iter().any(|(_, pt)| *pt == render_pt) {
        to_snapshot.insert(0, (format!("render thread 0x{render_tid:x}"), render_pt));
    }
    let main_pt = crate::crash::main_pthread();
    if main_pt != 0 && !to_snapshot.iter().any(|(_, pt)| *pt == main_pt) && to_snapshot.len() < 6 {
        to_snapshot.push(("main thread (AppKit)".into(), main_pt));
    }
    // The guest main thread (lowest Windows tid; runs the exe entry point / game loop).
    if let Some(t) = threads
        .iter()
        .filter(|t| t.pthread.load(Ordering::Relaxed) != 0 && t.tid.load(Ordering::Relaxed) != 0)
        .min_by_key(|t| t.tid.load(Ordering::Relaxed))
    {
        let pt = t.pthread.load(Ordering::Relaxed);
        if !to_snapshot.iter().any(|(_, p)| *p == pt) {
            let tid = t.tid.load(Ordering::Relaxed);
            to_snapshot.push((format!("guest main thread 0x{tid:x}"), pt));
        }
    }

    for (label, pt) in &to_snapshot {
        let _ = writeln!(out, "--- {label}{} ---", crate::crash::pthread_label(*pt));
        match unsafe { crate::crash::snapshot_thread(*pt, 20) } {
            Ok(frames) => {
                for (i, pc) in frames.iter().enumerate() {
                    let _ = writeln!(out, "  #{i:<2} {}", crate::symbolize(*pc as usize));
                }
            }
            Err(e) => {
                let _ = writeln!(out, "  (stack snapshot unavailable: {e})");
            }
        }
    }

    // Once per long stall: every other guest thread too (compact: guest frames + first host frames).
    static FULL_DUMP_FRAME: AtomicU32 = AtomicU32::new(u32::MAX);
    if stall_ms >= 20_000.0 && FULL_DUMP_FRAME.swap(frame, Ordering::Relaxed) != frame {
        let _ = writeln!(out, "--- all guest threads (once per stall) ---");
        for t in &threads {
            let pt = t.pthread.load(Ordering::Relaxed);
            if pt == 0 || to_snapshot.iter().any(|(_, p)| *p == pt) {
                continue;
            }
            let tid = t.tid.load(Ordering::Relaxed);
            let kind = ActKind::from_u8(t.act_kind.load(Ordering::Acquire));
            let _ = writeln!(out, "  thread 0x{tid:x} ({}):", kind.label());
            match unsafe { crate::crash::snapshot_thread(pt, 8) } {
                Ok(frames) => {
                    for pc in frames.iter().take(14) {
                        let _ = writeln!(out, "      {}", crate::symbolize(*pc as usize));
                    }
                }
                Err(e) => {
                    let _ = writeln!(out, "      (unavailable: {e})");
                }
            }
        }
    }

    let _ = writeln!(out, "--- interpreter profile since last Present ---");
    append_cpu_frame_report(&mut out);
    let _ = writeln!(out, "==================== end of stall report ====================");
    emit_report(&out);
}

/// Summary counters for tests and diagnostics.
#[derive(Debug, Clone, Copy)]
pub struct HitchStats {
    pub frame_hitches: u64,
    pub op_hitches: u64,
    pub stall_reports: u64,
}

pub fn stats() -> HitchStats {
    HitchStats {
        frame_hitches: TOTAL_FRAME_HITCHES.load(Ordering::Relaxed),
        op_hitches: TOTAL_OP_HITCHES.load(Ordering::Relaxed),
        stall_reports: TOTAL_STALL_REPORTS.load(Ordering::Relaxed),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_tracks_nested_thread_activity_and_accumulates() {
        let t0 = now_ns();
        assert!(t0 > 0);
        let before = ACCUM.fence_wait_calls.load(Ordering::Relaxed);
        {
            let _s1 = Scope::enter(ActKind::FenceWait, 42);
            std::thread::sleep(Duration::from_millis(2));
        }
        let after = ACCUM.fence_wait_calls.load(Ordering::Relaxed);
        assert!(after > before);
        assert!(ACCUM.fence_wait_ns.load(Ordering::Relaxed) >= 1_000_000);
    }

    #[test]
    fn dominant_cause_identifies_bottleneck() {
        let mut s = FrameSnapshot::default();
        s.pso_ns = 60_000_000;
        s.file_io_ns = 5_000_000;
        assert_eq!(dominant_cause(75_000_000, 2_000_000, &s, 0), "PSO_COMPILE");

        s.pso_ns = 0;
        s.file_io_ns = 45_000_000;
        assert_eq!(dominant_cause(60_000_000, 1_000_000, &s, 0), "FILE_IO");

        s.file_io_ns = 1_000_000;
        assert_eq!(dominant_cause(60_000_000, 1_000_000, &s, 0), "CPU_GAME_LOGIC");
    }

    #[test]
    fn fast_frames_stay_silent_and_spike_triggers_report() {
        let before = stats();
        for f in 1..=6 {
            on_present(f, 500_000);
        }
        assert_eq!(stats().frame_hitches, before.frame_hitches);
        // Simulate a 65 ms frame spike with a slow PSO compile
        record_pso("CS", "test_spike_cs", 55_000_000);
        std::thread::sleep(Duration::from_millis(60));
        on_present(7, 1_000_000);
        let after = stats();
        assert!(after.op_hitches > before.op_hitches);
        assert!(after.frame_hitches > before.frame_hitches);
    }
}
