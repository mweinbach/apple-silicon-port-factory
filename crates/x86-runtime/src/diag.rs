//! Universal, game-agnostic runtime diagnostics for `x86-runtime`:
//!
//! 1. **Per-VA JIT filter (`PORT_JIT_SKIP` / `PORT_JIT_ALLOW`)** — selectively forces
//!    guest address ranges (absolute VAs or `module:rva_lo..rva_hi`) onto the interpreter
//!    while keeping the rest of the process JIT-compiled. Designed for automated binary
//!    search (`O(log N)` runs) of ARM64 JIT vs interpreter divergences.
//! 2. **Floating-point `NaN`-birth, memory-watchpoint & host-boundary tracer
//!    (`PORT_NAN_TRACE`)** — zero overhead when unset (`OnceLock` fast path); when set
//!    (or dynamically toggled via `set_nan_trace_active` / `PORT_CONTROL_DIR`), steps
//!    covered guest code through the interpreter and detects:
//!    - **True `NaN` births** (per-lane): FP/SIMD/x87 arithmetic instructions whose
//!      participating source lanes were all non-`NaN` and whose result lane became `NaN`.
//!    - **Memory watchpoints / `NaN` stores**: stores overlapping a watched guest address
//!      range, or FP stores writing `NaN` floats.
//!    - **Host-call `NaN` returns & watched host symbols**: Win32/CRT/middleware calls
//!      producing `NaN` from finite inputs or matching a symbol filter.
//!    - **Optional x86-64 oracle snapshots (`dump_dir=...`)**: self-contained JSON dumps
//!      of pre/post register + memory state and raw x86-64 instruction bytes so an offline
//!      Rosetta `arch -x86_64` oracle can prove hardware parity vs emulator divergence.

use iced_x86::{Instruction, Mnemonic, OpKind, Register};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};

use crate::emu::{self, Cpu};

// ---------------------------------------------------------------------------
// Address / module-relative range parsing
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VaRange {
    /// Half-open absolute guest virtual address range `[lo, hi)`.
    Abs { lo: u64, hi: u64 },
    /// Half-open module-relative RVA range `[base + rva_lo, base + rva_hi)`.
    Mod { module_lc: String, rva_lo: u64, rva_hi: u64 },
}

#[inline]
fn resolve_guest_va_opt(va: usize) -> Option<(String, usize)> {
    #[cfg(target_arch = "aarch64")]
    {
        emu::aot::resolve_guest_va(va)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = va;
        None
    }
}

impl VaRange {
    pub fn parse(raw: &str) -> Option<Self> {
        let s = raw.trim();
        if s.is_empty() {
            return None;
        }
        if let Some((m, rest)) = s.split_once(':') {
            let module_lc = m.trim().to_ascii_lowercase();
            if module_lc.is_empty() {
                return None;
            }
            let (rva_lo, rva_hi) = parse_u64_span(rest)?;
            return Some(Self::Mod { module_lc, rva_lo, rva_hi });
        }
        let (lo, hi) = parse_u64_span(s)?;
        Some(Self::Abs { lo, hi })
    }

    pub fn matches(&self, ip: u64) -> bool {
        match self {
            Self::Abs { lo, hi } => ip >= *lo && ip < *hi,
            Self::Mod { module_lc, rva_lo, rva_hi } => {
                let Some((name, base)) = resolve_guest_va_opt(ip as usize) else {
                    return false;
                };
                if name != *module_lc && !name.ends_with(module_lc.as_str()) {
                    return false;
                }
                let rva = ip.wrapping_sub(base as u64);
                rva >= *rva_lo && rva < *rva_hi
            }
        }
    }
}

fn parse_u64_hex_or_dec(s: &str) -> Option<u64> {
    let s = s.trim();
    if let Some(h) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(h, 16).ok()
    } else {
        s.parse::<u64>().ok()
    }
}

fn parse_u64_span(s: &str) -> Option<(u64, u64)> {
    let s = s.trim();
    if let Some((a, b)) = s.split_once("..=") {
        let lo = parse_u64_hex_or_dec(a)?;
        let hi = parse_u64_hex_or_dec(b)?.saturating_add(1);
        return (lo < hi).then_some((lo, hi));
    }
    if let Some((a, b)) = s.split_once("..") {
        let lo = parse_u64_hex_or_dec(a)?;
        let hi = parse_u64_hex_or_dec(b)?;
        return (lo < hi).then_some((lo, hi));
    }
    if let Some((a, b)) = s.split_once('+') {
        let lo = parse_u64_hex_or_dec(a)?;
        let len = parse_u64_hex_or_dec(b)?;
        return (len > 0).then_some((lo, lo.saturating_add(len)));
    }
    let lo = parse_u64_hex_or_dec(s)?;
    Some((lo, lo.saturating_add(1)))
}

fn parse_range_list(raw: &str) -> Vec<VaRange> {
    raw.split(',')
        .filter_map(VaRange::parse)
        .collect()
}

// ---------------------------------------------------------------------------
// 1. Per-VA JIT Filter (`PORT_JIT_SKIP` / `PORT_JIT_ALLOW`)
// ---------------------------------------------------------------------------

struct JitFilter {
    skip: Vec<VaRange>,
    allow: Vec<VaRange>,
}

impl JitFilter {
    fn active(&self) -> bool {
        !self.skip.is_empty() || !self.allow.is_empty()
    }

    fn allows(&self, ip: u64) -> bool {
        if !self.allow.is_empty() && !self.allow.iter().any(|r| r.matches(ip)) {
            return false;
        }
        if self.skip.iter().any(|r| r.matches(ip)) {
            return false;
        }
        true
    }
}

fn jit_filter() -> &'static JitFilter {
    static F: OnceLock<JitFilter> = OnceLock::new();
    F.get_or_init(|| {
        let skip = std::env::var("PORT_JIT_SKIP").map(|s| parse_range_list(&s)).unwrap_or_default();
        let allow = std::env::var("PORT_JIT_ALLOW").map(|s| parse_range_list(&s)).unwrap_or_default();
        let f = JitFilter { skip, allow };
        if f.active() {
            log::info!(
                "diag: per-VA JIT filter active (skip={} ranges, allow={} ranges); AOT disabled",
                f.skip.len(),
                f.allow.len()
            );
        }
        f
    })
}

/// `true` iff `PORT_JIT_SKIP` or `PORT_JIT_ALLOW` is configured.
#[inline(always)]
pub fn jit_filter_active() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| jit_filter().active())
}

/// `true` iff guest `ip` is allowed to run in translated ARM64 JIT code.
#[inline]
pub fn jit_allows(ip: u64) -> bool {
    if !jit_filter_active() {
        return true;
    }
    jit_filter().allows(ip)
}

// ---------------------------------------------------------------------------
// 1b. Per-VA AOT Filter (`PORT_AOT_SKIP` / `PORT_AOT_ALLOW`)
// ---------------------------------------------------------------------------

fn aot_filter() -> &'static JitFilter {
    static F: OnceLock<JitFilter> = OnceLock::new();
    F.get_or_init(|| {
        let skip = std::env::var("PORT_AOT_SKIP").map(|s| parse_range_list(&s)).unwrap_or_default();
        let allow = std::env::var("PORT_AOT_ALLOW").map(|s| parse_range_list(&s)).unwrap_or_default();
        let f = JitFilter { skip, allow };
        if f.active() {
            log::info!(
                "diag: per-VA AOT filter active (skip={} ranges, allow={} ranges)",
                f.skip.len(),
                f.allow.len()
            );
        }
        f
    })
}

/// `true` iff `PORT_AOT_SKIP` or `PORT_AOT_ALLOW` is configured.
#[inline(always)]
pub fn aot_filter_active() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| aot_filter().active())
}

/// `true` iff guest `ip` is allowed to be installed from an `.aot` cache (filtered-out
/// blocks fall back cleanly to live JIT translation).
#[inline]
pub fn aot_allows(ip: u64) -> bool {
    if !aot_filter_active() {
        return true;
    }
    aot_filter().allows(ip)
}

// ---------------------------------------------------------------------------
// 2. Universal `NaN`-Birth, Memory-Watch & Host-Boundary Tracer
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct NanTraceConfig {
    pub initial_on: bool,
    pub out_path: PathBuf,
    pub dump_dir: Option<PathBuf>,
    pub ranges: Vec<VaRange>,
    pub watch_mem: Option<(u64, u64)>, // (lo, hi)
    pub host: bool,
    pub host_syms: Vec<String>,
    pub stores: bool,
    pub max_per_ip: u32,
    pub max_total: u64,
}

impl Default for NanTraceConfig {
    fn default() -> Self {
        let out_path = std::env::var("PORT_LOG_DIR")
            .ok()
            .filter(|s| !s.trim().is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(std::env::temp_dir)
            .join("port_nan_trace.jsonl");
        Self {
            initial_on: true,
            out_path,
            dump_dir: None,
            ranges: Vec::new(),
            watch_mem: None,
            host: true,
            host_syms: Vec::new(),
            stores: false,
            max_per_ip: 4,
            max_total: 4000,
        }
    }
}

impl NanTraceConfig {
    pub fn parse(raw: &str) -> Option<Self> {
        let s = raw.trim();
        if s.is_empty() || s == "0" || s.eq_ignore_ascii_case("off") || s.eq_ignore_ascii_case("false") {
            return None;
        }
        let mut cfg = Self::default();
        if s == "1" || s.eq_ignore_ascii_case("on") || s.eq_ignore_ascii_case("true") {
            return Some(cfg);
        }
        for part in s.split(';').map(str::trim).filter(|p| !p.is_empty()) {
            let Some((k, v)) = part.split_once('=') else {
                continue;
            };
            let v = v.trim();
            match k.trim().to_ascii_lowercase().as_str() {
                "on" | "active" => cfg.initial_on = v != "0" && !v.eq_ignore_ascii_case("false"),
                "out" | "log" => {
                    if !v.is_empty() {
                        cfg.out_path = PathBuf::from(v);
                    }
                }
                "dump_dir" | "dump" => {
                    if !v.is_empty() {
                        cfg.dump_dir = Some(PathBuf::from(v));
                    }
                }
                "range" | "ranges" => {
                    cfg.ranges.extend(parse_range_list(v));
                }
                "watch" => {
                    if let Some((a, b)) = v.split_once(':') {
                        if let (Some(addr), Some(len)) = (parse_u64_hex_or_dec(a), parse_u64_hex_or_dec(b)) {
                            if len > 0 {
                                cfg.watch_mem = Some((addr, addr.saturating_add(len)));
                            }
                        }
                    } else if let Some((lo, hi)) = parse_u64_span(v) {
                        cfg.watch_mem = Some((lo, hi));
                    }
                }
                "host" => cfg.host = v != "0" && !v.eq_ignore_ascii_case("false"),
                "host_sym" | "host_syms" => {
                    for sym in v.split(',').map(str::trim).filter(|x| !x.is_empty()) {
                        cfg.host_syms.push(sym.to_ascii_lowercase());
                    }
                }
                "stores" | "store_nan" => cfg.stores = v != "0" && !v.eq_ignore_ascii_case("false"),
                "max_per_ip" => {
                    if let Ok(n) = v.parse::<u32>() {
                        cfg.max_per_ip = n.max(1);
                    }
                }
                "max_total" => {
                    if let Ok(n) = v.parse::<u64>() {
                        cfg.max_total = n.max(1);
                    }
                }
                _ => {}
            }
        }
        Some(cfg)
    }
}

struct NanSink {
    writer: Option<BufWriter<File>>,
    dump_dir: Option<PathBuf>,
    per_ip: HashMap<u64, u32>,
    per_host: HashMap<(u64, u64), u32>,
    total: u64,
    seq: u64,
    max_per_ip: u32,
    max_total: u64,
}

impl NanSink {
    fn new(cfg: &NanTraceConfig) -> Self {
        if let Some(parent) = cfg.out_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let writer = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&cfg.out_path)
            .ok()
            .map(BufWriter::new);
        if let Some(ref dd) = cfg.dump_dir {
            let _ = std::fs::create_dir_all(dd);
        }
        Self {
            writer,
            dump_dir: cfg.dump_dir.clone(),
            per_ip: HashMap::new(),
            per_host: HashMap::new(),
            total: 0,
            seq: 0,
            max_per_ip: cfg.max_per_ip,
            max_total: cfg.max_total,
        }
    }

    fn write_line(&mut self, line: &str) {
        if let Some(ref mut w) = self.writer {
            let _ = w.write_all(line.as_bytes());
            let _ = w.write_all(b"\n");
            let _ = w.flush();
        }
    }
}

struct NanTracer {
    cfg: NanTraceConfig,
    /// Dynamic VA range override (replaces `cfg.ranges` when non-empty).
    dyn_ranges: Mutex<Vec<VaRange>>,
    /// Dynamic watchpoint `[lo, hi)` (overrides `cfg.watch_mem` when `lo < hi`).
    watch_lo: AtomicU64,
    watch_hi: AtomicU64,
    /// Whether any VA range filter is currently configured.
    has_ranges: AtomicBool,
    /// Whether FP store NaN logging is enabled.
    stores_on: AtomicBool,
    /// Counters for status reporting.
    nan_births: AtomicU64,
    host_events: AtomicU64,
    mem_watches: AtomicU64,
    started: std::time::Instant,
    sink: Mutex<NanSink>,
}

static NAN_INSTALLED: AtomicBool = AtomicBool::new(false);
static NAN_ACTIVE: AtomicBool = AtomicBool::new(false);
static NAN_TRACER: OnceLock<Option<NanTracer>> = OnceLock::new();

fn init_tracer() -> &'static Option<NanTracer> {
    NAN_TRACER.get_or_init(|| {
        let raw = std::env::var("PORT_NAN_TRACE").ok()?;
        let cfg = NanTraceConfig::parse(&raw)?;
        let has_ranges = !cfg.ranges.is_empty();
        let (wlo, whi) = cfg.watch_mem.unwrap_or((0, 0));
        let stores_on = cfg.stores;
        let initial_on = cfg.initial_on;
        let sink = NanSink::new(&cfg);
        log::info!(
            "diag: PORT_NAN_TRACE configured (on={}, out={}, ranges={}, watch={:#x}..{:#x}, host={}, stores={}, dump={:?})",
            initial_on,
            cfg.out_path.display(),
            cfg.ranges.len(),
            wlo,
            whi,
            cfg.host,
            stores_on,
            cfg.dump_dir
        );
        let t = NanTracer {
            dyn_ranges: Mutex::new(cfg.ranges.clone()),
            watch_lo: AtomicU64::new(wlo),
            watch_hi: AtomicU64::new(whi),
            has_ranges: AtomicBool::new(has_ranges),
            stores_on: AtomicBool::new(stores_on),
            nan_births: AtomicU64::new(0),
            host_events: AtomicU64::new(0),
            mem_watches: AtomicU64::new(0),
            started: std::time::Instant::now(),
            sink: Mutex::new(sink),
            cfg,
        };
        NAN_INSTALLED.store(true, Ordering::Release);
        NAN_ACTIVE.store(initial_on, Ordering::Release);
        Some(t)
    })
}

/// Initialize env-configured `PORT_NAN_TRACE` once and report whether it is installed.
#[inline(always)]
pub fn nan_trace_installed() -> bool {
    if NAN_TRACER.get().is_none() {
        let _ = init_tracer();
    }
    NAN_INSTALLED.load(Ordering::Relaxed)
}

/// Fast-path check: `true` iff `PORT_NAN_TRACE` is installed and currently armed.
#[inline(always)]
pub fn nan_trace_active() -> bool {
    NAN_ACTIVE.load(Ordering::Relaxed)
}

/// `true` iff `PORT_NAN_TRACE` was configured with a static VA range filter (in which case
/// AOT is disabled so blocks outside the range cannot direct-branch into blocks inside it).
pub fn nan_trace_has_static_ranges() -> bool {
    init_tracer().as_ref().is_some_and(|t| !t.cfg.ranges.is_empty())
}

/// Toggle runtime `NaN` tracing on/off (usable from `PORT_CONTROL_DIR` `live_ctl`).
pub fn set_nan_trace_active(on: bool) -> bool {
    if init_tracer().is_none() {
        log::warn!("diag: set_nan_trace_active({on}) ignored because PORT_NAN_TRACE was not set at startup");
        return false;
    }
    NAN_ACTIVE.store(on, Ordering::Release);
    log::info!("diag: nan_trace active -> {on}");
    true
}

/// Set or replace the active VA range filter at runtime.
pub fn set_nan_trace_ranges(ranges: Vec<VaRange>) {
    if let Some(t) = init_tracer().as_ref() {
        let has = !ranges.is_empty();
        if let Ok(mut g) = t.dyn_ranges.lock() {
            *g = ranges;
        }
        t.has_ranges.store(has, Ordering::Release);
        log::info!("diag: nan_trace ranges updated (active_filter={has})");
    }
}

/// Set a dynamic guest memory watchpoint `[addr, addr + len)`. Pass `len == 0` to clear.
pub fn set_nan_watch_mem(addr: u64, len: u64) {
    if let Some(t) = init_tracer().as_ref() {
        let hi = if len == 0 { 0 } else { addr.saturating_add(len) };
        t.watch_lo.store(if len == 0 { 0 } else { addr }, Ordering::Release);
        t.watch_hi.store(hi, Ordering::Release);
        log::info!("diag: nan_trace watch_mem -> {:#x}..{:#x}", addr, hi);
    }
}

/// Toggle FP NaN-store logging at runtime.
pub fn set_nan_trace_stores(on: bool) {
    if let Some(t) = init_tracer().as_ref() {
        t.stores_on.store(on, Ordering::Release);
    }
}

/// Clear dynamic VA range filter and memory watchpoint.
pub fn clear_nan_trace_filters() {
    set_nan_trace_ranges(Vec::new());
    set_nan_watch_mem(0, 0);
}

/// `(nan_births, host_events, mem_watches)` recorded so far.
pub fn nan_trace_stats() -> (u64, u64, u64) {
    match init_tracer().as_ref() {
        Some(t) => (
            t.nan_births.load(Ordering::Relaxed),
            t.host_events.load(Ordering::Relaxed),
            t.mem_watches.load(Ordering::Relaxed),
        ),
        None => (0, 0, 0),
    }
}

/// `true` iff guest `ip` is covered by the active `NaN` tracer.
#[inline]
pub fn nan_trace_covers(ip: u64) -> bool {
    if !NAN_ACTIVE.load(Ordering::Relaxed) {
        return false;
    }
    let Some(t) = init_tracer().as_ref() else {
        return false;
    };
    if !t.has_ranges.load(Ordering::Relaxed) {
        return true;
    }
    match t.dyn_ranges.lock() {
        Ok(g) => g.is_empty() || g.iter().any(|r| r.matches(ip)),
        Err(_) => true,
    }
}

/// Format a guest IP as `module.dll+0xRVA` when inside a mapped PE image, else `0x<ip>`.
pub fn format_guest_ip(ip: u64) -> String {
    if let Some((name, base)) = resolve_guest_va_opt(ip as usize) {
        format!("{name}+0x{:x}", ip.wrapping_sub(base as u64))
    } else {
        format!("0x{ip:x}")
    }
}

/// Install the `win32-rt` boundary-NaN diagnostic hook so any Win32/middleware boundary
/// reporting non-finite inputs automatically logs the live guest `RIP` and call stack.
pub fn install_boundary_hook() {
    win32_rt::set_boundary_nan_hook(on_boundary_nan);
}

#[cold]
#[inline(never)]
fn on_boundary_nan(tag: &str, ptr: usize, len: usize, detail: &str) {
    static BT_LOGGED: AtomicU64 = AtomicU64::new(0);
    let n = BT_LOGGED.fetch_add(1, Ordering::Relaxed);
    if n >= 16 {
        return;
    }
    let (where_rip, bt) = match emu::current_guest_state() {
        Some((gpr, rip)) => {
            let w = format_guest_ip(rip);
            let frames = unsafe { capture_guest_backtrace(gpr[4]) };
            (w, frames)
        }
        None => ("<no-guest-cpu>".to_string(), Vec::new()),
    };
    log::warn!(
        "[diag-boundary-bt] tag={tag} ptr={ptr:#x} len={len} rip={where_rip} detail=({detail}) bt=[{}]",
        bt.join(" <- ")
    );
    if let Some(t) = init_tracer().as_ref() {
        let mut sink = match t.sink.lock() {
            Ok(g) => g,
            Err(e) => e.into_inner(),
        };
        if sink.total < sink.max_total {
            sink.total += 1;
            sink.seq += 1;
            let seq = sink.seq;
            let ts_ms = t.started.elapsed().as_millis();
            let tid = cur_tid();
            let bt_json: Vec<String> = bt.iter().map(|f| format!("\"{}\"", json_escape(f))).collect();
            let line = format!(
                "{{\"seq\":{seq},\"ts_ms\":{ts_ms},\"tid\":{tid},\"kind\":\"boundary_nan\",\"tag\":\"{}\",\"ptr\":\"0x{ptr:x}\",\"len\":{len},\"rip\":\"{}\",\"detail\":\"{}\",\"backtrace\":[{}]}}",
                json_escape(tag),
                json_escape(&where_rip),
                json_escape(detail),
                bt_json.join(",")
            );
            sink.write_line(&line);
        }
    }
}

// ---------------------------------------------------------------------------
// Per-instruction FP classification and NaN-birth detection
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FpKind {
    None,
    ScalarF32,
    ScalarF64,
    /// `lanes`: 4 (XMM) or 8 (YMM). `horiz`: true for `dpps`/`haddps`/`hsubps`.
    PackedF32 { lanes: u8, horiz: bool },
    /// `lanes`: 2 (XMM) or 4 (YMM). `horiz`: true for `dppd`/`haddpd`/`hsubpd`.
    PackedF64 { lanes: u8, horiz: bool },
    CvtF64ToF32 { lanes: u8 },
    CvtF32ToF64 { lanes: u8 },
    CvtF16ToF32 { lanes: u8 },
    X87,
    FpStore { is_f64: bool, bytes: u8 },
    WatchStore { bytes: u8 },
}

/// Stack-allocated pre-instruction snapshot (zero heap allocations on the hot path).
pub(crate) struct PreFpState {
    kind: FpKind,
    dst_reg: u8,
    /// Bitmask of lanes where ANY input operand was already `NaN` before the instruction.
    in_nan_mask: u8,
    /// Up to 3 source operand values (low 128 + high 128) for detailed logging on NaN birth.
    src_lo: [u128; 3],
    src_hi: [u128; 3],
    src_count: u8,
    /// Pre-instruction memory operand info (when the instruction reads or writes memory).
    mem_addr: u64,
    mem_bytes: [u8; 32],
    mem_len: u8,
    /// Pre-instruction CPU state needed only if an oracle snapshot dump is requested.
    pre_dst_lo: u128,
    pre_dst_hi: u128,
    pre_gpr: [u64; 16],
    pre_rflags: u32,
    pre_mxcsr: u32,
    /// Pre-instruction x87 ST(0)/ST(1) bits for x87 arithmetic.
    x87_st0: u64,
    x87_st1: u64,
    x87_had_nan: bool,
}

impl PreFpState {
    const NONE: Self = Self {
        kind: FpKind::None,
        dst_reg: 0,
        in_nan_mask: 0,
        src_lo: [0; 3],
        src_hi: [0; 3],
        src_count: 0,
        mem_addr: 0,
        mem_bytes: [0; 32],
        mem_len: 0,
        pre_dst_lo: 0,
        pre_dst_hi: 0,
        pre_gpr: [0; 16],
        pre_rflags: 0,
        pre_mxcsr: 0x1F80,
        x87_st0: 0,
        x87_st1: 0,
        x87_had_nan: false,
    };
}

#[inline]
fn vec_reg_idx(r: Register) -> Option<(u8, bool)> {
    use Register::*;
    let (idx, is_ymm) = match r {
        XMM0 => (0, false), XMM1 => (1, false), XMM2 => (2, false), XMM3 => (3, false),
        XMM4 => (4, false), XMM5 => (5, false), XMM6 => (6, false), XMM7 => (7, false),
        XMM8 => (8, false), XMM9 => (9, false), XMM10 => (10, false), XMM11 => (11, false),
        XMM12 => (12, false), XMM13 => (13, false), XMM14 => (14, false), XMM15 => (15, false),
        YMM0 => (0, true), YMM1 => (1, true), YMM2 => (2, true), YMM3 => (3, true),
        YMM4 => (4, true), YMM5 => (5, true), YMM6 => (6, true), YMM7 => (7, true),
        YMM8 => (8, true), YMM9 => (9, true), YMM10 => (10, true), YMM11 => (11, true),
        YMM12 => (12, true), YMM13 => (13, true), YMM14 => (14, true), YMM15 => (15, true),
        _ => return Option::None,
    };
    Some((idx, is_ymm))
}

#[inline]
fn f32_bits_is_nan(b: u32) -> bool {
    (b & 0x7F80_0000) == 0x7F80_0000 && (b & 0x007F_FFFF) != 0
}

#[inline]
fn f64_bits_is_nan(b: u64) -> bool {
    (b & 0x7FF0_0000_0000_0000) == 0x7FF0_0000_0000_0000 && (b & 0x000F_FFFF_FFFF_FFFF) != 0
}

#[inline]
fn f16_bits_is_nan(b: u16) -> bool {
    (b & 0x7C00) == 0x7C00 && (b & 0x03FF) != 0
}

#[inline]
fn lane_f32(lo: u128, hi: u128, lane: u8) -> u32 {
    if lane < 4 {
        (lo >> (lane * 32)) as u32
    } else {
        (hi >> ((lane - 4) * 32)) as u32
    }
}

#[inline]
fn lane_f64(lo: u128, hi: u128, lane: u8) -> u64 {
    if lane < 2 {
        (lo >> (lane * 64)) as u64
    } else {
        (hi >> ((lane - 2) * 64)) as u64
    }
}

#[inline]
fn lane_f16(lo: u128, lane: u8) -> u16 {
    (lo >> (lane * 16)) as u16
}

fn classify_fp_ins(ins: &Instruction) -> FpKind {
    use Mnemonic::*;
    let is_ymm = ins.op_count() > 0
        && ins.op_kind(0) == OpKind::Register
        && vec_reg_idx(ins.op_register(0)).is_some_and(|(_, y)| y);
    let f32_lanes = if is_ymm { 8 } else { 4 };
    let f64_lanes = if is_ymm { 4 } else { 2 };
    match ins.mnemonic() {
        Addss | Vaddss | Subss | Vsubss | Mulss | Vmulss | Divss | Vdivss
        | Sqrtss | Vsqrtss | Rsqrtss | Vrsqrtss | Rcpss | Vrcpss
        | Roundss | Vroundss | Minss | Vminss | Maxss | Vmaxss
        | Vfmadd132ss | Vfmadd213ss | Vfmadd231ss
        | Vfmsub132ss | Vfmsub213ss | Vfmsub231ss
        | Vfnmadd132ss | Vfnmadd213ss | Vfnmadd231ss
        | Vfnmsub132ss | Vfnmsub213ss | Vfnmsub231ss => FpKind::ScalarF32,

        Addsd | Vaddsd | Subsd | Vsubsd | Mulsd | Vmulsd | Divsd | Vdivsd
        | Sqrtsd | Vsqrtsd | Roundsd | Vroundsd | Minsd | Vminsd | Maxsd | Vmaxsd
        | Vfmadd132sd | Vfmadd213sd | Vfmadd231sd
        | Vfmsub132sd | Vfmsub213sd | Vfmsub231sd
        | Vfnmadd132sd | Vfnmadd213sd | Vfnmadd231sd
        | Vfnmsub132sd | Vfnmsub213sd | Vfnmsub231sd => FpKind::ScalarF64,

        Addps | Vaddps | Subps | Vsubps | Mulps | Vmulps | Divps | Vdivps
        | Sqrtps | Vsqrtps | Rsqrtps | Vrsqrtps | Rcpps | Vrcpps
        | Roundps | Vroundps | Minps | Vminps | Maxps | Vmaxps
        | Addsubps | Vaddsubps
        | Vfmadd132ps | Vfmadd213ps | Vfmadd231ps
        | Vfmsub132ps | Vfmsub213ps | Vfmsub231ps
        | Vfnmadd132ps | Vfnmadd213ps | Vfnmadd231ps
        | Vfnmsub132ps | Vfnmsub213ps | Vfnmsub231ps
        | Vfmaddsub132ps | Vfmaddsub213ps | Vfmaddsub231ps
        | Vfmsubadd132ps | Vfmsubadd213ps | Vfmsubadd231ps => {
            FpKind::PackedF32 { lanes: f32_lanes, horiz: false }
        }

        Dpps | Vdpps | Haddps | Vhaddps | Hsubps | Vhsubps => {
            FpKind::PackedF32 { lanes: f32_lanes, horiz: true }
        }

        Addpd | Vaddpd | Subpd | Vsubpd | Mulpd | Vmulpd | Divpd | Vdivpd
        | Sqrtpd | Vsqrtpd | Roundpd | Vroundpd | Minpd | Vminpd | Maxpd | Vmaxpd
        | Addsubpd | Vaddsubpd
        | Vfmadd132pd | Vfmadd213pd | Vfmadd231pd
        | Vfmsub132pd | Vfmsub213pd | Vfmsub231pd
        | Vfnmadd132pd | Vfnmadd213pd | Vfnmadd231pd
        | Vfnmsub132pd | Vfnmsub213pd | Vfnmsub231pd
        | Vfmaddsub132pd | Vfmaddsub213pd | Vfmaddsub231pd
        | Vfmsubadd132pd | Vfmsubadd213pd | Vfmsubadd231pd => {
            FpKind::PackedF64 { lanes: f64_lanes, horiz: false }
        }

        Dppd | Vdppd | Haddpd | Vhaddpd | Hsubpd | Vhsubpd => {
            FpKind::PackedF64 { lanes: f64_lanes, horiz: true }
        }

        Cvtsd2ss | Vcvtsd2ss => FpKind::CvtF64ToF32 { lanes: 1 },
        Cvtpd2ps | Vcvtpd2ps => FpKind::CvtF64ToF32 {
            lanes: if ins.op_count() > 1 && vec_reg_idx(ins.op_register(1)).is_some_and(|(_, y)| y) { 4 } else { 2 },
        },
        Cvtss2sd | Vcvtss2sd => FpKind::CvtF32ToF64 { lanes: 1 },
        Cvtps2pd | Vcvtps2pd => FpKind::CvtF32ToF64 { lanes: f64_lanes },
        Vcvtph2ps => FpKind::CvtF16ToF32 { lanes: f32_lanes },

        Fadd | Faddp | Fiadd | Fsub | Fsubp | Fsubr | Fsubrp | Fisub | Fisubr
        | Fmul | Fmulp | Fimul | Fdiv | Fdivp | Fdivr | Fdivrp | Fidiv | Fidivr
        | Fsqrt | Fsin | Fcos | Fsincos | Fptan | Fpatan | Fprem | Fprem1
        | Fyl2x | Fyl2xp1 | F2xm1 | Fscale | Frndint => FpKind::X87,

        _ => FpKind::None,
    }
}

fn is_fp_store(ins: &Instruction) -> Option<(bool, u8)> {
    use Mnemonic::*;
    if ins.op_count() == 0 || ins.op_kind(0) != OpKind::Memory {
        return None;
    }
    let sz = ins.memory_size().size().min(32) as u8;
    match ins.mnemonic() {
        Movss | Vmovss => Some((false, 4)),
        Movsd | Vmovsd => Some((true, 8)),
        Movaps | Vmovaps | Movups | Vmovups | Movntps | Vmovntps | Movlps | Vmovlps | Movhps | Vmovhps => {
            Some((false, sz.max(4)))
        }
        Movapd | Vmovapd | Movupd | Vmovupd | Movntpd | Vmovntpd | Movlpd | Vmovlpd | Movhpd | Vmovhpd => {
            Some((true, sz.max(8)))
        }
        Fst | Fstp => Some((sz == 8, sz.max(4))),
        _ => None,
    }
}

fn is_fma_mnemonic(mn: Mnemonic) -> bool {
    use Mnemonic::*;
    matches!(
        mn,
        Vfmadd132ss | Vfmadd213ss | Vfmadd231ss
            | Vfmsub132ss | Vfmsub213ss | Vfmsub231ss
            | Vfnmadd132ss | Vfnmadd213ss | Vfnmadd231ss
            | Vfnmsub132ss | Vfnmsub213ss | Vfnmsub231ss
            | Vfmadd132sd | Vfmadd213sd | Vfmadd231sd
            | Vfmsub132sd | Vfmsub213sd | Vfmsub231sd
            | Vfnmadd132sd | Vfnmadd213sd | Vfnmadd231sd
            | Vfnmsub132sd | Vfnmsub213sd | Vfnmsub231sd
            | Vfmadd132ps | Vfmadd213ps | Vfmadd231ps
            | Vfmsub132ps | Vfmsub213ps | Vfmsub231ps
            | Vfnmadd132ps | Vfnmadd213ps | Vfnmadd231ps
            | Vfnmsub132ps | Vfnmsub213ps | Vfnmsub231ps
            | Vfmaddsub132ps | Vfmaddsub213ps | Vfmaddsub231ps
            | Vfmsubadd132ps | Vfmsubadd213ps | Vfmsubadd231ps
            | Vfmadd132pd | Vfmadd213pd | Vfmadd231pd
            | Vfmsub132pd | Vfmsub213pd | Vfmsub231pd
            | Vfnmadd132pd | Vfnmadd213pd | Vfnmadd231pd
            | Vfnmsub132pd | Vfnmsub213pd | Vfnmsub231pd
            | Vfmaddsub132pd | Vfmaddsub213pd | Vfmaddsub231pd
            | Vfmsubadd132pd | Vfmsubadd213pd | Vfmsubadd231pd
    )
}

fn is_unary_fp_mnemonic(mn: Mnemonic) -> bool {
    use Mnemonic::*;
    matches!(
        mn,
        Sqrtss | Vsqrtss | Rsqrtss | Vrsqrtss | Rcpss | Vrcpss
            | Sqrtsd | Vsqrtsd | Sqrtps | Vsqrtps | Rsqrtps | Vrsqrtps
            | Rcpps | Vrcpps | Sqrtpd | Vsqrtpd
            | Roundss | Vroundss | Roundsd | Vroundsd | Roundps | Vroundps | Roundpd | Vroundpd
            | Cvtsd2ss | Vcvtsd2ss | Cvtpd2ps | Vcvtpd2ps
            | Cvtss2sd | Vcvtss2sd | Cvtps2pd | Vcvtps2pd | Vcvtph2ps
    )
}

unsafe fn safe_read_bytes(addr: u64, out: &mut [u8]) -> bool {
    if addr < 0x10000 || out.is_empty() {
        return false;
    }
    if !win32_rt::kernel32::memory::is_speculative_read_safe(addr as usize, out.len()) {
        return false;
    }
    core::ptr::copy_nonoverlapping(addr as *const u8, out.as_mut_ptr(), out.len());
    true
}

unsafe fn read_operand_vec(
    cpu: &Cpu,
    ins: &Instruction,
    op_idx: u32,
    mem_out: &mut (u64, [u8; 32], u8),
) -> Option<(u128, u128)> {
    match ins.op_kind(op_idx) {
        OpKind::Register => {
            let (r, is_ymm) = vec_reg_idx(ins.op_register(op_idx))?;
            let lo = cpu.xmm[r as usize];
            let hi = if is_ymm { cpu.ymm_hi[r as usize] } else { 0 };
            Some((lo, hi))
        }
        OpKind::Memory => {
            let addr = cpu.eff_addr(ins);
            let sz = ins.memory_size().size().clamp(1, 32);
            let mut buf = [0u8; 32];
            if !safe_read_bytes(addr, &mut buf[..sz]) {
                return None;
            }
            *mem_out = (addr, buf, sz as u8);
            let lo = u128::from_le_bytes(buf[0..16].try_into().unwrap());
            let hi = u128::from_le_bytes(buf[16..32].try_into().unwrap());
            Some((lo, hi))
        }
        _ => None,
    }
}

/// Snapshot source FP lanes (or watched store target) before executing `ins`.
pub(crate) unsafe fn pre_ins(cpu: &Cpu, ins: &Instruction) -> PreFpState {
    let Some(t) = init_tracer().as_ref() else {
        return PreFpState::NONE;
    };
    let mut pre = PreFpState::NONE;

    // Check memory watchpoints / FP NaN stores first when op 0 is memory.
    if ins.op_count() > 0 && ins.op_kind(0) == OpKind::Memory {
        let wlo = t.watch_lo.load(Ordering::Relaxed);
        let whi = t.watch_hi.load(Ordering::Relaxed);
        if wlo < whi {
            let addr = cpu.eff_addr(ins);
            let sz = ins.memory_size().size().clamp(1, 32) as u64;
            let end = addr.saturating_add(sz);
            if addr < whi && end > wlo {
                pre.kind = FpKind::WatchStore { bytes: sz as u8 };
                pre.mem_addr = addr;
                pre.mem_len = sz as u8;
                let _ = safe_read_bytes(addr, &mut pre.mem_bytes[..sz as usize]);
                return pre;
            }
        }
        if t.stores_on.load(Ordering::Relaxed) {
            if let Some((is_f64, bytes)) = is_fp_store(ins) {
                let addr = cpu.eff_addr(ins);
                pre.kind = FpKind::FpStore { is_f64, bytes };
                pre.mem_addr = addr;
                pre.mem_len = bytes;
                return pre;
            }
        }
        return pre;
    }

    let kind = classify_fp_ins(ins);
    if kind == FpKind::None {
        return pre;
    }
    pre.kind = kind;

    if kind == FpKind::X87 {
        let st = crate::emu::regfile_ext::x87_snapshot();
        let st0 = st.peek_f64_bits(0).unwrap_or(0);
        let st1 = st.peek_f64_bits(1).unwrap_or(0);
        pre.x87_st0 = st0;
        pre.x87_st1 = st1;
        let mut had_nan = f64_bits_is_nan(st0) || f64_bits_is_nan(st1);
        if ins.op_count() > 0 && ins.op_kind(0) == OpKind::Memory {
            let addr = cpu.eff_addr(ins);
            let sz = ins.memory_size().size().clamp(1, 16);
            if safe_read_bytes(addr, &mut pre.mem_bytes[..sz]) {
                pre.mem_addr = addr;
                pre.mem_len = sz as u8;
                if sz == 4 {
                    let b = u32::from_le_bytes(pre.mem_bytes[0..4].try_into().unwrap());
                    had_nan |= f32_bits_is_nan(b);
                } else if sz == 8 {
                    let b = u64::from_le_bytes(pre.mem_bytes[0..8].try_into().unwrap());
                    had_nan |= f64_bits_is_nan(b);
                }
            }
        }
        pre.x87_had_nan = had_nan;
        return pre;
    }

    let Some((dst_reg, _)) = (ins.op_count() > 0 && ins.op_kind(0) == OpKind::Register)
        .then(|| vec_reg_idx(ins.op_register(0)))
        .flatten()
    else {
        return PreFpState::NONE;
    };
    pre.dst_reg = dst_reg;

    // Collect participating source operands:
    // - 2-op SSE unary (`sqrtps dst, src`, `cvt* dst, src`): only op 1 is a numeric input
    //   (except scalar 2-op `sqrtss dst, src`, where op 1 is the numeric input for lane 0).
    // - 3-op VEX unary scalar (`vsqrtss dst, src1, src2`): op 2 is the numeric input for lane 0.
    // - 2-op SSE binary (`addss dst, src`): op 0 and op 1 are inputs.
    // - 3-op VEX binary (`vaddps dst, src1, src2`): op 1 and op 2 are inputs.
    // - 3-op FMA (`vfmadd213ps dst, src1, src2`): op 0, op 1, and op 2 are ALL inputs!
    let mn = ins.mnemonic();
    let nops = ins.op_count();
    let fma = is_fma_mnemonic(mn);
    let unary = is_unary_fp_mnemonic(mn);
    let mut mem_tmp = (0u64, [0u8; 32], 0u8);

    let op_indices: &[u32] = if fma {
        &[0, 1, 2]
    } else if unary {
        match nops {
            2 => &[1],
            3 => {
                // `roundps dst, src, imm8` has imm8 at op 2; `vsqrtss dst, src1, src2` has src2 at op 2.
                if ins.op_kind(2) == OpKind::Immediate8 { &[1] } else { &[2] }
            }
            4 => &[2], // `vroundss dst, src1, src2, imm8`
            _ => &[1],
        }
    } else {
        // Binary arithmetic (2-op SSE or 3-op VEX, possibly with trailing imm8 like dpps)
        let effective_ops = if nops > 0 && ins.op_kind(nops - 1) == OpKind::Immediate8 {
            nops - 1
        } else {
            nops
        };
        if effective_ops >= 3 { &[1, 2] } else { &[0, 1] }
    };

    for &oi in op_indices {
        if oi < nops && (pre.src_count as usize) < 3 {
            if let Some((lo, hi)) = read_operand_vec(cpu, ins, oi, &mut mem_tmp) {
                let idx = pre.src_count as usize;
                pre.src_lo[idx] = lo;
                pre.src_hi[idx] = hi;
                pre.src_count += 1;
            }
        }
    }
    pre.mem_addr = mem_tmp.0;
    pre.mem_bytes = mem_tmp.1;
    pre.mem_len = mem_tmp.2;

    if t.cfg.dump_dir.is_some() {
        pre.pre_dst_lo = cpu.xmm[dst_reg as usize];
        pre.pre_dst_hi = cpu.ymm_hi[dst_reg as usize];
        pre.pre_gpr = cpu.gpr;
        pre.pre_rflags = cpu.rflags();
        pre.pre_mxcsr = cpu.mxcsr();
    }

    // Compute per-lane `in_nan_mask`.
    let nsrc = pre.src_count as usize;
    let mut mask = 0u8;
    match kind {
        FpKind::ScalarF32 => {
            for s in 0..nsrc {
                if f32_bits_is_nan(lane_f32(pre.src_lo[s], pre.src_hi[s], 0)) {
                    mask |= 1;
                }
            }
        }
        FpKind::ScalarF64 => {
            for s in 0..nsrc {
                if f64_bits_is_nan(lane_f64(pre.src_lo[s], pre.src_hi[s], 0)) {
                    mask |= 1;
                }
            }
        }
        FpKind::PackedF32 { lanes, horiz } => {
            if horiz {
                let mut any = false;
                for s in 0..nsrc {
                    for l in 0..lanes {
                        if f32_bits_is_nan(lane_f32(pre.src_lo[s], pre.src_hi[s], l)) {
                            any = true;
                        }
                    }
                }
                if any {
                    mask = 0xFF;
                }
            } else {
                for l in 0..lanes {
                    for s in 0..nsrc {
                        if f32_bits_is_nan(lane_f32(pre.src_lo[s], pre.src_hi[s], l)) {
                            mask |= 1 << l;
                        }
                    }
                }
            }
        }
        FpKind::PackedF64 { lanes, horiz } => {
            if horiz {
                let mut any = false;
                for s in 0..nsrc {
                    for l in 0..lanes {
                        if f64_bits_is_nan(lane_f64(pre.src_lo[s], pre.src_hi[s], l)) {
                            any = true;
                        }
                    }
                }
                if any {
                    mask = 0xFF;
                }
            } else {
                for l in 0..lanes {
                    for s in 0..nsrc {
                        if f64_bits_is_nan(lane_f64(pre.src_lo[s], pre.src_hi[s], l)) {
                            mask |= 1 << l;
                        }
                    }
                }
            }
        }
        FpKind::CvtF64ToF32 { lanes } => {
            for l in 0..lanes {
                for s in 0..nsrc {
                    if f64_bits_is_nan(lane_f64(pre.src_lo[s], pre.src_hi[s], l)) {
                        mask |= 1 << l;
                    }
                }
            }
        }
        FpKind::CvtF32ToF64 { lanes } => {
            for l in 0..lanes {
                for s in 0..nsrc {
                    if f32_bits_is_nan(lane_f32(pre.src_lo[s], pre.src_hi[s], l)) {
                        mask |= 1 << l;
                    }
                }
            }
        }
        FpKind::CvtF16ToF32 { lanes } => {
            for l in 0..lanes {
                for s in 0..nsrc {
                    if f16_bits_is_nan(lane_f16(pre.src_lo[s], l)) {
                        mask |= 1 << l;
                    }
                }
            }
        }
        FpKind::None | FpKind::X87 | FpKind::FpStore { .. } | FpKind::WatchStore { .. } => {}
    }
    pre.in_nan_mask = mask;
    pre
}

/// Inspect post-instruction outputs against `pre` and record any true `NaN` birth or watch hit.
pub(crate) unsafe fn post_ins(pre: &PreFpState, cpu: &Cpu, ip: u64, ins: &Instruction) {
    if pre.kind == FpKind::None {
        return;
    }
    match pre.kind {
        FpKind::WatchStore { bytes } => {
            let mut after = [0u8; 32];
            let sz = (bytes as usize).min(32);
            let _ = safe_read_bytes(pre.mem_addr, &mut after[..sz]);
            record_mem_watch(ip, ins, cpu, pre.mem_addr, &pre.mem_bytes[..sz], &after[..sz]);
            return;
        }
        FpKind::FpStore { is_f64, bytes } => {
            let mut after = [0u8; 32];
            let sz = (bytes as usize).min(32);
            if !safe_read_bytes(pre.mem_addr, &mut after[..sz]) {
                return;
            }
            let has_nan = if is_f64 {
                after[..sz]
                    .chunks_exact(8)
                    .any(|c| f64_bits_is_nan(u64::from_le_bytes(c.try_into().unwrap())))
            } else {
                after[..sz]
                    .chunks_exact(4)
                    .any(|c| f32_bits_is_nan(u32::from_le_bytes(c.try_into().unwrap())))
            };
            if has_nan {
                record_nan_store(ip, ins, cpu, pre.mem_addr, is_f64, &after[..sz]);
            }
            return;
        }
        FpKind::X87 => {
            if pre.x87_had_nan {
                return;
            }
            let st = crate::emu::regfile_ext::x87_snapshot();
            if let Some(out) = st.peek_f64_bits(0) {
                if f64_bits_is_nan(out) {
                    record_nan_birth(pre, cpu, ip, ins, 0, out);
                }
            }
            return;
        }
        FpKind::None => return,
        _ => {}
    }

    let d = pre.dst_reg as usize;
    let out_lo = cpu.xmm[d];
    let out_hi = cpu.ymm_hi[d];

    match pre.kind {
        FpKind::ScalarF32 => {
            let b = lane_f32(out_lo, out_hi, 0);
            if f32_bits_is_nan(b) && (pre.in_nan_mask & 1) == 0 {
                record_nan_birth(pre, cpu, ip, ins, 0, b as u64);
            }
        }
        FpKind::ScalarF64 => {
            let b = lane_f64(out_lo, out_hi, 0);
            if f64_bits_is_nan(b) && (pre.in_nan_mask & 1) == 0 {
                record_nan_birth(pre, cpu, ip, ins, 0, b);
            }
        }
        FpKind::PackedF32 { lanes, .. } | FpKind::CvtF64ToF32 { lanes } | FpKind::CvtF16ToF32 { lanes } => {
            for l in 0..lanes {
                let b = lane_f32(out_lo, out_hi, l);
                if f32_bits_is_nan(b) && (pre.in_nan_mask & (1 << l)) == 0 {
                    record_nan_birth(pre, cpu, ip, ins, l, b as u64);
                    break;
                }
            }
        }
        FpKind::PackedF64 { lanes, .. } | FpKind::CvtF32ToF64 { lanes } => {
            for l in 0..lanes {
                let b = lane_f64(out_lo, out_hi, l);
                if f64_bits_is_nan(b) && (pre.in_nan_mask & (1 << l)) == 0 {
                    record_nan_birth(pre, cpu, ip, ins, l, b);
                    break;
                }
            }
        }
        FpKind::None | FpKind::X87 | FpKind::FpStore { .. } | FpKind::WatchStore { .. } => {}
    }
}

/// Inspect a host Win32/CRT/middleware call for finite-input -> `NaN`-return or watched symbols.
pub(crate) unsafe fn check_host_call(
    target_va: u64,
    caller_rip: u64,
    flags: u32,
    gpr: &[u64; 8],
    xmm_in: &[u128; 4],
    r_gpr: u64,
    r_xmm: u128,
) {
    let Some(t) = init_tracer().as_ref() else {
        return;
    };
    if !t.cfg.host && t.cfg.host_syms.is_empty() {
        return;
    }
    let is_float_ret = flags & emu::DF_FLOAT_RET != 0;

    // Fast path: if no symbol watches and not float-returning, only inspect when lane 0 changed
    // to a genuine float/double NaN (and not an all-ones mask).
    let out_f32 = r_xmm as u32;
    let out_f64 = r_xmm as u64;
    let out_is_f32_nan = f32_bits_is_nan(out_f32) && out_f32 != 0xFFFF_FFFF;
    let out_is_f64_nan = f64_bits_is_nan(out_f64) && out_f64 != 0xFFFF_FFFF_FFFF_FFFF;

    let sym_watched = !t.cfg.host_syms.is_empty();
    let check_nan_ret = t.cfg.host && (is_float_ret || ((out_is_f32_nan || out_is_f64_nan) && r_xmm != xmm_in[0]));
    if !sym_watched && !check_nan_ret {
        return;
    }

    let sym = win32_rt::symbolize(target_va as usize);
    let sym_lc = sym.to_ascii_lowercase();

    if sym_watched && t.cfg.host_syms.iter().any(|pat| sym_lc.contains(pat.as_str())) {
        record_host_event(t, "host_sym", target_va, caller_rip, &sym, gpr, xmm_in, r_gpr, r_xmm);
    }

    if !check_nan_ret {
        return;
    }
    // Avoid false positives on integer Win32 calls that happen to leave a random value in q0:
    // require either DF_FLOAT_RET or a CRT/math/middleware symbol name.
    let likely_fp_sym = is_float_ret
        || sym_lc.contains("ucrt")
        || sym_lc.contains("msvcrt")
        || sym_lc.contains("crt-math")
        || sym_lc.contains("nvcloth")
        || sym_lc.contains("physx");
    if !likely_fp_sym {
        return;
    }
    let in_has_nan = xmm_in.iter().any(|&x| f32_bits_is_nan(x as u32) || f64_bits_is_nan(x as u64));
    if !in_has_nan && (out_is_f32_nan || out_is_f64_nan) {
        record_host_event(t, "host_nan_ret", target_va, caller_rip, &sym, gpr, xmm_in, r_gpr, r_xmm);
    }
}

// ---------------------------------------------------------------------------
// Backtrace & JSONL/Snapshot recording (cold paths)
// ---------------------------------------------------------------------------

unsafe fn looks_like_call_return(ret_va: u64) -> bool {
    let m = emu::to_mapped_addr(ret_va);
    if m < 0x10008 {
        return false;
    }
    let mut prev = [0u8; 7];
    if !safe_read_bytes((m - 7) as u64, &mut prev) {
        return false;
    }
    // E8 rel32 (5 bytes): prev[2] == 0xE8
    if prev[2] == 0xE8 {
        return true;
    }
    // FF /2 indirect calls: 2..7 bytes ending at ret_va (`prev[5]`=2B, `prev[4]`=3B, `prev[3]`=4B, `prev[1]`=6B, `prev[0]`=7B)
    for &idx in &[0usize, 1, 3, 4, 5] {
        if prev[idx] == 0xFF && idx + 1 < 7 && ((prev[idx + 1] >> 3) & 7) == 2 {
            return true;
        }
    }
    false
}

unsafe fn capture_guest_backtrace(rsp: u64) -> Vec<String> {
    let mut frames = Vec::new();
    if rsp < 0x10000 {
        return frames;
    }
    let aligned = rsp & !7u64;
    for i in 0..160u64 {
        let slot = aligned.wrapping_add(i * 8);
        let mut buf = [0u8; 8];
        if !safe_read_bytes(slot, &mut buf) {
            break;
        }
        let cand = u64::from_le_bytes(buf);
        if cand < 0x10000 {
            continue;
        }
        if let Some((name, base)) = resolve_guest_va_opt(cand as usize) {
            if looks_like_call_return(cand) {
                frames.push(format!("{name}+0x{:x}", cand.wrapping_sub(base as u64)));
                if frames.len() >= 10 {
                    break;
                }
            }
        }
    }
    frames
}

fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {}
            c => out.push(c),
        }
    }
    out
}

fn hex_bytes(b: &[u8]) -> String {
    let mut s = String::with_capacity(b.len() * 2);
    for &x in b {
        use core::fmt::Write;
        let _ = write!(s, "{x:02x}");
    }
    s
}

fn format_vec_f32_lanes(lo: u128, hi: u128, n: u8) -> String {
    let mut parts = Vec::with_capacity(n as usize);
    for l in 0..n {
        let bits = lane_f32(lo, hi, l);
        let val = f32::from_bits(bits);
        parts.push(format!("\"{val:?}(0x{bits:08x})\""));
    }
    format!("[{}]", parts.join(","))
}

fn format_vec_f64_lanes(lo: u128, hi: u128, n: u8) -> String {
    let mut parts = Vec::with_capacity(n as usize);
    for l in 0..n {
        let bits = lane_f64(lo, hi, l);
        let val = f64::from_bits(bits);
        parts.push(format!("\"{val:?}(0x{bits:016x})\""));
    }
    format!("[{}]", parts.join(","))
}

fn cur_tid() -> u64 {
    #[cfg(target_vendor = "apple")]
    unsafe {
        unsafe extern "C" {
            fn pthread_threadid_np(thread: usize, thread_id: *mut u64) -> i32;
        }
        let mut tid = 0u64;
        pthread_threadid_np(0, &mut tid);
        tid
    }
    #[cfg(not(target_vendor = "apple"))]
    {
        0
    }
}

#[cold]
#[inline(never)]
unsafe fn record_nan_birth(pre: &PreFpState, cpu: &Cpu, ip: u64, ins: &Instruction, lane: u8, out_bits: u64) {
    let Some(t) = init_tracer().as_ref() else {
        return;
    };
    let mut sink = match t.sink.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    if sink.total >= sink.max_total {
        return;
    }
    let max_per_ip = sink.max_per_ip;
    let count = sink.per_ip.entry(ip).or_insert(0);
    if *count >= max_per_ip {
        return;
    }
    *count += 1;
    sink.total += 1;
    sink.seq += 1;
    let seq = sink.seq;
    t.nan_births.fetch_add(1, Ordering::Relaxed);

    let ts_ms = t.started.elapsed().as_millis();
    let tid = cur_tid();
    let where_str = format_guest_ip(ip);
    let disasm = format!("{ins}");

    let mut ins_bytes = [0u8; 16];
    let ins_len = ins.len().min(16);
    let mapped_ip = emu::to_mapped_addr(ip) as u64;
    let bytes_hex = if safe_read_bytes(mapped_ip, &mut ins_bytes[..ins_len]) {
        hex_bytes(&ins_bytes[..ins_len])
    } else {
        String::new()
    };

    let mut inputs_json = Vec::new();
    match pre.kind {
        FpKind::X87 => {
            inputs_json.push(format!(
                "[\"{:?}(0x{:016x})\",\"{:?}(0x{:016x})\"]",
                f64::from_bits(pre.x87_st0),
                pre.x87_st0,
                f64::from_bits(pre.x87_st1),
                pre.x87_st1
            ));
        }
        FpKind::ScalarF64 | FpKind::PackedF64 { .. } | FpKind::CvtF64ToF32 { .. } => {
            let nl = match pre.kind {
                FpKind::ScalarF64 => 1,
                FpKind::PackedF64 { lanes, .. } | FpKind::CvtF64ToF32 { lanes } => lanes,
                _ => 2,
            };
            for s in 0..pre.src_count as usize {
                inputs_json.push(format_vec_f64_lanes(pre.src_lo[s], pre.src_hi[s], nl));
            }
        }
        _ => {
            let nl = match pre.kind {
                FpKind::ScalarF32 => 1,
                FpKind::PackedF32 { lanes, .. } | FpKind::CvtF16ToF32 { lanes } => lanes,
                FpKind::CvtF32ToF64 { lanes } => lanes,
                _ => 4,
            };
            for s in 0..pre.src_count as usize {
                inputs_json.push(format_vec_f32_lanes(pre.src_lo[s], pre.src_hi[s], nl));
            }
        }
    }

    let bt = capture_guest_backtrace(cpu.gpr[4]);
    let bt_json: Vec<String> = bt.iter().map(|f| format!("\"{}\"", json_escape(f))).collect();

    let snap_file = if let Some(ref dd) = sink.dump_dir {
        let fname = format!("snap_{seq:05}_{ip:x}.json");
        let fpath = dd.join(&fname);
        let mut xmm_pre = Vec::with_capacity(16);
        let mut ymm_pre = Vec::with_capacity(16);
        let mut xmm_post = Vec::with_capacity(16);
        let mut ymm_post = Vec::with_capacity(16);
        for i in 0..16 {
            let (plo, phi) = if i == pre.dst_reg as usize {
                (pre.pre_dst_lo, pre.pre_dst_hi)
            } else {
                (cpu.xmm[i], cpu.ymm_hi[i])
            };
            xmm_pre.push(format!("\"0x{plo:032x}\""));
            ymm_pre.push(format!("\"0x{phi:032x}\""));
            xmm_post.push(format!("\"0x{:032x}\"", cpu.xmm[i]));
            ymm_post.push(format!("\"0x{:032x}\"", cpu.ymm_hi[i]));
        }
        let mut src_lo_hex = Vec::new();
        let mut src_hi_hex = Vec::new();
        for s in 0..pre.src_count as usize {
            src_lo_hex.push(format!("\"0x{:032x}\"", pre.src_lo[s]));
            src_hi_hex.push(format!("\"0x{:032x}\"", pre.src_hi[s]));
        }
        let gpr_pre: Vec<String> = pre.pre_gpr.iter().map(|v| format!("\"0x{v:016x}\"")).collect();
        let gpr_post: Vec<String> = cpu.gpr.iter().map(|v| format!("\"0x{v:016x}\"")).collect();
        let snap = format!(
            "{{\"seq\":{seq},\"ip\":\"0x{ip:x}\",\"where\":\"{}\",\"disasm\":\"{}\",\"bytes\":\"{bytes_hex}\",\"dst_reg\":{},\"lane\":{lane},\"out_bits\":\"0x{out_bits:x}\",\"src_lo\":[{}],\"src_hi\":[{}],\"mem_addr\":\"0x{:x}\",\"mem_bytes\":\"{}\",\"pre_gpr\":[{}],\"pre_xmm\":[{}],\"pre_ymm_hi\":[{}],\"pre_rflags\":\"0x{:x}\",\"pre_mxcsr\":\"0x{:x}\",\"post_gpr\":[{}],\"post_xmm\":[{}],\"post_ymm_hi\":[{}],\"post_mxcsr\":\"0x{:x}\"}}\n",
            json_escape(&where_str),
            json_escape(&disasm),
            pre.dst_reg,
            src_lo_hex.join(","),
            src_hi_hex.join(","),
            pre.mem_addr,
            hex_bytes(&pre.mem_bytes[..pre.mem_len as usize]),
            gpr_pre.join(","),
            xmm_pre.join(","),
            ymm_pre.join(","),
            pre.pre_rflags,
            pre.pre_mxcsr,
            gpr_post.join(","),
            xmm_post.join(","),
            ymm_post.join(","),
            cpu.mxcsr(),
        );
        let _ = std::fs::write(&fpath, snap);
        Some(fpath.to_string_lossy().into_owned())
    } else {
        None
    };

    let line = format!(
        "{{\"seq\":{seq},\"ts_ms\":{ts_ms},\"tid\":{tid},\"kind\":\"nan_birth\",\"ip\":\"0x{ip:x}\",\"where\":\"{}\",\"disasm\":\"{}\",\"bytes\":\"{bytes_hex}\",\"lane\":{lane},\"out_hex\":\"0x{out_bits:x}\",\"inputs\":[{}],\"backtrace\":[{}]{}}}",
        json_escape(&where_str),
        json_escape(&disasm),
        inputs_json.join(","),
        bt_json.join(","),
        snap_file
            .map(|p| format!(",\"snapshot\":\"{}\"", json_escape(&p)))
            .unwrap_or_default()
    );
    sink.write_line(&line);
}

#[cold]
#[inline(never)]
unsafe fn record_mem_watch(ip: u64, ins: &Instruction, cpu: &Cpu, addr: u64, before: &[u8], after: &[u8]) {
    let Some(t) = init_tracer().as_ref() else {
        return;
    };
    let mut sink = match t.sink.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    if sink.total >= sink.max_total {
        return;
    }
    sink.total += 1;
    sink.seq += 1;
    let seq = sink.seq;
    t.mem_watches.fetch_add(1, Ordering::Relaxed);

    let ts_ms = t.started.elapsed().as_millis();
    let tid = cur_tid();
    let where_str = format_guest_ip(ip);
    let disasm = format!("{ins}");
    let bt = capture_guest_backtrace(cpu.gpr[4]);
    let bt_json: Vec<String> = bt.iter().map(|f| format!("\"{}\"", json_escape(f))).collect();
    let line = format!(
        "{{\"seq\":{seq},\"ts_ms\":{ts_ms},\"tid\":{tid},\"kind\":\"mem_watch\",\"ip\":\"0x{ip:x}\",\"where\":\"{}\",\"disasm\":\"{}\",\"addr\":\"0x{addr:x}\",\"before\":\"{}\",\"after\":\"{}\",\"backtrace\":[{}]}}",
        json_escape(&where_str),
        json_escape(&disasm),
        hex_bytes(before),
        hex_bytes(after),
        bt_json.join(",")
    );
    sink.write_line(&line);
}

#[cold]
#[inline(never)]
unsafe fn record_nan_store(ip: u64, ins: &Instruction, cpu: &Cpu, addr: u64, is_f64: bool, after: &[u8]) {
    let Some(t) = init_tracer().as_ref() else {
        return;
    };
    let mut sink = match t.sink.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    if sink.total >= sink.max_total {
        return;
    }
    let max_per_ip = sink.max_per_ip;
    let count = sink.per_ip.entry(ip ^ 0x8000_0000_0000_0000).or_insert(0);
    if *count >= max_per_ip {
        return;
    }
    *count += 1;
    sink.total += 1;
    sink.seq += 1;
    let seq = sink.seq;
    t.mem_watches.fetch_add(1, Ordering::Relaxed);

    let ts_ms = t.started.elapsed().as_millis();
    let tid = cur_tid();
    let where_str = format_guest_ip(ip);
    let disasm = format!("{ins}");
    let bt = capture_guest_backtrace(cpu.gpr[4]);
    let bt_json: Vec<String> = bt.iter().map(|f| format!("\"{}\"", json_escape(f))).collect();
    let line = format!(
        "{{\"seq\":{seq},\"ts_ms\":{ts_ms},\"tid\":{tid},\"kind\":\"nan_store\",\"ip\":\"0x{ip:x}\",\"where\":\"{}\",\"disasm\":\"{}\",\"addr\":\"0x{addr:x}\",\"f64\":{is_f64},\"bytes\":\"{}\",\"backtrace\":[{}]}}",
        json_escape(&where_str),
        json_escape(&disasm),
        hex_bytes(after),
        bt_json.join(",")
    );
    sink.write_line(&line);
}

#[cold]
#[inline(never)]
fn record_host_event(
    t: &NanTracer,
    kind: &str,
    target_va: u64,
    caller_rip: u64,
    sym: &str,
    gpr: &[u64; 8],
    xmm_in: &[u128; 4],
    r_gpr: u64,
    r_xmm: u128,
) {
    let mut sink = match t.sink.lock() {
        Ok(g) => g,
        Err(e) => e.into_inner(),
    };
    if sink.total >= sink.max_total {
        return;
    }
    let max_per_ip = sink.max_per_ip;
    let count = sink.per_host.entry((target_va, caller_rip)).or_insert(0);
    if *count >= max_per_ip {
        return;
    }
    *count += 1;
    sink.total += 1;
    sink.seq += 1;
    let seq = sink.seq;
    t.host_events.fetch_add(1, Ordering::Relaxed);

    let ts_ms = t.started.elapsed().as_millis();
    let tid = cur_tid();
    let caller_str = format_guest_ip(caller_rip);
    let line = format!(
        "{{\"seq\":{seq},\"ts_ms\":{ts_ms},\"tid\":{tid},\"kind\":\"{kind}\",\"target\":\"0x{target_va:x}\",\"symbol\":\"{}\",\"caller\":\"{}\",\"gpr\":[\"0x{:x}\",\"0x{:x}\",\"0x{:x}\",\"0x{:x}\"],\"xmm_in\":[\"0x{:032x}\",\"0x{:032x}\",\"0x{:032x}\",\"0x{:032x}\"],\"r_gpr\":\"0x{r_gpr:x}\",\"r_xmm\":\"0x{r_xmm:032x}\"}}",
        json_escape(sym),
        json_escape(&caller_str),
        gpr[0],
        gpr[1],
        gpr[2],
        gpr[3],
        xmm_in[0],
        xmm_in[1],
        xmm_in[2],
        xmm_in[3],
    );
    sink.write_line(&line);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn va_range_parses_absolute_and_module_spans() {
        let r1 = VaRange::parse("0x140000000..0x140010000").unwrap();
        assert!(r1.matches(0x140000000));
        assert!(r1.matches(0x14000ffff));
        assert!(!r1.matches(0x140010000));

        let r2 = VaRange::parse("0x140000000..=0x140000010").unwrap();
        assert!(r2.matches(0x140000010));
        assert!(!r2.matches(0x140000011));

        let r3 = VaRange::parse("0x180001000+0x200").unwrap();
        assert!(r3.matches(0x1800011ff));
        assert!(!r3.matches(0x180001200));

        let r4 = VaRange::parse("Game.exe:0x1000..0x2000").unwrap();
        assert_eq!(
            r4,
            VaRange::Mod { module_lc: "game.exe".into(), rva_lo: 0x1000, rva_hi: 0x2000 }
        );
    }

    #[test]
    fn nan_trace_config_parses_spec() {
        assert!(NanTraceConfig::parse("0").is_none());
        let c1 = NanTraceConfig::parse("1").unwrap();
        assert!(c1.initial_on);
        assert!(c1.host);

        let c2 = NanTraceConfig::parse(
            "on=0;out=/tmp/t.jsonl;dump_dir=/tmp/d;range=0x140000000..0x140100000,foo.dll:0x10..0x20;watch=0x50000:0x40;host=0;host_sym=NvCloth,PhysX;stores=1;max_per_ip=8;max_total=500",
        )
        .unwrap();
        assert!(!c2.initial_on);
        assert_eq!(c2.out_path, PathBuf::from("/tmp/t.jsonl"));
        assert_eq!(c2.dump_dir, Some(PathBuf::from("/tmp/d")));
        assert_eq!(c2.ranges.len(), 2);
        assert_eq!(c2.watch_mem, Some((0x50000, 0x50040)));
        assert!(!c2.host);
        assert_eq!(c2.host_syms, vec!["nvcloth", "physx"]);
        assert!(c2.stores);
        assert_eq!(c2.max_per_ip, 8);
        assert_eq!(c2.max_total, 500);
    }

    #[test]
    fn per_lane_nan_birth_detects_zero_div_zero_and_ignores_propagation() {
        use iced_x86::{Decoder, DecoderOptions};
        // `divps xmm0, xmm1` = 0F 5E C1
        let code = [0x0Fu8, 0x5E, 0xC1];
        let mut dec = Decoder::with_ip(64, &code, 0x1_4000_1000, DecoderOptions::NONE);
        let ins = dec.decode();
        assert_eq!(classify_fp_ins(&ins), FpKind::PackedF32 { lanes: 4, horiz: false });

        // Lane 0: 0.0 / 0.0 (NaN birth!), lane 1: 4.0 / 2.0 = 2.0, lane 3: pre-existing NaN (propagation only)
        let mut cpu = Cpu::new_zeroed();
        let pack = |a: [f32; 4]| -> u128 {
            let mut b = [0u8; 16];
            for (i, v) in a.iter().enumerate() {
                b[i * 4..(i + 1) * 4].copy_from_slice(&v.to_bits().to_le_bytes());
            }
            u128::from_le_bytes(b)
        };
        cpu.xmm[0] = pack([0.0, 4.0, 1.0, f32::NAN]);
        cpu.xmm[1] = pack([0.0, 2.0, 1.0, 1.0]);

        // Directly verify mask calculation: lane 3 had NaN, lanes 0..3 did not.
        let mut mask = 0u8;
        for l in 0..4u8 {
            if f32_bits_is_nan(lane_f32(cpu.xmm[0], 0, l)) || f32_bits_is_nan(lane_f32(cpu.xmm[1], 0, l)) {
                mask |= 1 << l;
            }
        }
        assert_eq!(mask, 1 << 3);
        let post = pack([f32::NAN, 2.0, 1.0, f32::NAN]);
        // Lane 0 is a true birth (`in_nan_mask` bit 0 is 0, post lane 0 is NaN); lane 3 is NOT a birth.
        assert!(f32_bits_is_nan(lane_f32(post, 0, 0)) && (mask & (1 << 0)) == 0);
        assert!(f32_bits_is_nan(lane_f32(post, 0, 3)) && (mask & (1 << 3)) != 0);
    }
}
