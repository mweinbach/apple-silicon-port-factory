//! Strict AOT fallback accounting (`PORT_AOT_STRICT=off|report|fail`).
//!
//! Counts and logs every execution event that leaves precompiled AOT machine code:
//! - runtime JIT block compiles (with reason: `aot_miss`, `smc_or_dyn_code`, `revalidation_dead`, `aot_disabled`)
//! - interpreter fast-op steps while the translator is enabled
//! - interpreter generic-op steps
//! - interpreter MMX-dispatch steps
//! - guest trap / syscall boundary steps
//! - software-FP denormal fallback steps (`denorm.rs`)
//!
//! Each fallback is attributed to `module+0xRVA` (falling back to symbolization / raw VA for
//! unmapped dynamic code). In `PORT_AOT_STRICT=fail` mode, the first fallback immediately logs
//! full context and terminates the process with [`EXIT_AOT_FALLBACK`] (`87`), making it easy for
//! CI fixtures to assert zero runtime fallback.

use iced_x86::Mnemonic;
use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::{Mutex, Once, OnceLock};

/// Documented process exit code when `PORT_AOT_STRICT=fail` observes a runtime JIT compile,
/// interpreter step, or software-FP fallback.
pub const EXIT_AOT_FALLBACK: u32 = 87;

/// Operating mode of `PORT_AOT_STRICT`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AotStrictMode {
    Off = 0,
    Report = 1,
    Fail = 2,
}

impl AotStrictMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Off => "off",
            Self::Report => "report",
            Self::Fail => "fail",
        }
    }

    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "report" | "warn" | "log" | "1" => Self::Report,
            "fail" | "abort" | "strict" | "error" | "2" => Self::Fail,
            _ => Self::Off,
        }
    }
}

/// Classification of one AOT fallback event.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FallbackKind {
    RuntimeJit = 0,
    InterpFast = 1,
    InterpGeneric = 2,
    InterpMmx = 3,
    InterpTrap = 4,
    SoftFp = 5,
}

impl FallbackKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RuntimeJit => "runtime_jit",
            Self::InterpFast => "interp_fast",
            Self::InterpGeneric => "interp_generic",
            Self::InterpMmx => "interp_mmx",
            Self::InterpTrap => "interp_trap",
            Self::SoftFp => "soft_fp",
        }
    }
}

/// One aggregated fallback site (`module+RVA`, kind, mnemonic/reason, execution count).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AotFallbackSite {
    pub location: String,
    pub ip: u64,
    pub kind: &'static str,
    pub mnemonic: String,
    pub reason: String,
    pub count: u64,
}

/// Snapshot of strict AOT fallback accounting exported to `--report-json` and diagnostics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AotFallbackReport {
    pub strict_mode: &'static str,
    pub aot_insns: u64,
    pub runtime_jit_insns: u64,
    pub runtime_jit_compiles: u64,
    pub runtime_jit_reasons: Vec<(String, u64)>,
    pub interp_fast_steps: u64,
    pub interp_generic_steps: u64,
    pub interp_mmx_steps: u64,
    pub interp_trap_steps: u64,
    pub soft_fp_steps: u64,
    pub total_fallbacks: u64,
    pub top_mnemonics: Vec<(String, u64)>,
    pub top_sites: Vec<AotFallbackSite>,
    pub top_jit_compiles: Vec<AotFallbackSite>,
}

/// Programmatic override (`0` = env, `1` = Off, `2` = Report, `3` = Fail).
static MODE_OVERRIDE: AtomicU8 = AtomicU8::new(0);

/// Explicitly override `PORT_AOT_STRICT` for the current process (primarily for tests/harness).
pub fn set_aot_strict_mode(mode: AotStrictMode) {
    MODE_OVERRIDE.store((mode as u8) + 1, Ordering::Release);
    if mode == AotStrictMode::Report {
        register_report_exit_hook();
    }
}

/// Clear any programmatic override so `PORT_AOT_STRICT` env var controls behavior.
pub fn clear_aot_strict_mode_override() {
    MODE_OVERRIDE.store(0, Ordering::Release);
}

fn env_mode() -> AotStrictMode {
    static V: OnceLock<AotStrictMode> = OnceLock::new();
    *V.get_or_init(|| {
        let m = std::env::var("PORT_AOT_STRICT")
            .ok()
            .map(|v| AotStrictMode::parse(&v))
            .unwrap_or(AotStrictMode::Off);
        if m != AotStrictMode::Off {
            log::info!("emu aot-strict: enabled (PORT_AOT_STRICT={})", m.as_str());
            if m == AotStrictMode::Report {
                register_report_exit_hook();
            }
        }
        m
    })
}

/// Active `PORT_AOT_STRICT` mode.
#[inline]
pub fn aot_strict_mode() -> AotStrictMode {
    match MODE_OVERRIDE.load(Ordering::Acquire) {
        1 => AotStrictMode::Off,
        2 => AotStrictMode::Report,
        3 => AotStrictMode::Fail,
        _ => env_mode(),
    }
}

/// Fast check: true whenever `PORT_AOT_STRICT` is `report` or `fail`.
#[inline]
pub(super) fn is_active() -> bool {
    aot_strict_mode() != AotStrictMode::Off
}

// Fast atomic totals (always updated on cold fallback paths so `--report-json` always has counts).
static AOT_INSNS: AtomicU64 = AtomicU64::new(0);
static RUNTIME_JIT_INSNS: AtomicU64 = AtomicU64::new(0);
static RUNTIME_JIT_COMPILES: AtomicU64 = AtomicU64::new(0);
static INTERP_FAST_STEPS: AtomicU64 = AtomicU64::new(0);
static INTERP_GENERIC_STEPS: AtomicU64 = AtomicU64::new(0);
static INTERP_MMX_STEPS: AtomicU64 = AtomicU64::new(0);
static INTERP_TRAP_STEPS: AtomicU64 = AtomicU64::new(0);
static SOFT_FP_STEPS: AtomicU64 = AtomicU64::new(0);

#[derive(Default)]
struct FallbackDetails {
    /// `(ip, kind, mnemonic_u16, reason)` -> count
    sites: HashMap<(u64, FallbackKind, u16, &'static str), u64>,
    /// `mnemonic_u16` -> count (across interpreter & soft-fp fallbacks)
    mnemonics: HashMap<u16, u64>,
    /// `reason` -> count (for runtime JIT compiles)
    jit_reasons: HashMap<&'static str, u64>,
    /// Deduplication set for `report`-mode per-site logging.
    logged: HashSet<(u64, FallbackKind)>,
}

static DETAILS: Mutex<Option<FallbackDetails>> = Mutex::new(None);

/// Clear all accumulated counters and detail maps (for test isolation).
pub fn reset_aot_fallback_counters() {
    AOT_INSNS.store(0, Ordering::Relaxed);
    RUNTIME_JIT_INSNS.store(0, Ordering::Relaxed);
    RUNTIME_JIT_COMPILES.store(0, Ordering::Relaxed);
    INTERP_FAST_STEPS.store(0, Ordering::Relaxed);
    INTERP_GENERIC_STEPS.store(0, Ordering::Relaxed);
    INTERP_MMX_STEPS.store(0, Ordering::Relaxed);
    INTERP_TRAP_STEPS.store(0, Ordering::Relaxed);
    SOFT_FP_STEPS.store(0, Ordering::Relaxed);
    if let Ok(mut g) = DETAILS.lock() {
        *g = None;
    }
}

fn register_report_exit_hook() {
    static HOOK: Once = Once::new();
    HOOK.call_once(|| {
        win32_rt::process::register_exit_hook(log_exit_summary);
    });
}

/// Format a guest IP as `module+0xRVA` when mapped, else fall back to symbolizer or raw VA.
pub(super) fn format_guest_location(ip: u64) -> String {
    #[cfg(target_arch = "aarch64")]
    if let Some((module, base)) = super::aot::resolve_guest_va(ip as usize) {
        let rva = ip.saturating_sub(base as u64);
        return format!("{module}+{rva:#x}");
    }
    let sym = win32_rt::symbolize(ip as usize);
    if !sym.is_empty() && sym != "unknown" {
        sym
    } else {
        format!("unmapped+{ip:#x}")
    }
}

fn mnemonic_name(id: u16) -> String {
    if id == u16::MAX {
        return String::new();
    }
    Mnemonic::try_from(id as usize)
        .map(|m| format!("{m:?}").to_ascii_lowercase())
        .unwrap_or_else(|_| format!("#{id}"))
}

/// Record translated guest instructions split between AOT code and runtime-JIT code.
#[inline]
pub(super) fn record_translated_insns(executed: i64, from_aot: bool) {
    let n = executed.max(0) as u64;
    if n == 0 {
        return;
    }
    if from_aot {
        AOT_INSNS.fetch_add(n, Ordering::Relaxed);
    } else {
        RUNTIME_JIT_INSNS.fetch_add(n, Ordering::Relaxed);
    }
}

#[cold]
#[inline(never)]
fn handle_strict_event(
    ip: u64,
    kind: FallbackKind,
    mn_id: u16,
    reason: &'static str,
    count: u64,
    always_record_detail: bool,
) {
    let mode = aot_strict_mode();
    if mode == AotStrictMode::Off && !always_record_detail {
        return;
    }

    let mut first_log = false;
    if let Ok(mut g) = DETAILS.lock() {
        let d = g.get_or_insert_with(FallbackDetails::default);
        *d.sites.entry((ip, kind, mn_id, reason)).or_insert(0) += count;
        if kind == FallbackKind::RuntimeJit {
            *d.jit_reasons.entry(reason).or_insert(0) += count;
        } else if mn_id != u16::MAX {
            *d.mnemonics.entry(mn_id).or_insert(0) += count;
        }
        if mode != AotStrictMode::Off {
            first_log = d.logged.insert((ip, kind));
        }
    }

    if mode == AotStrictMode::Off {
        return;
    }

    let loc = format_guest_location(ip);
    let mn = mnemonic_name(mn_id);
    match mode {
        AotStrictMode::Report => {
            if first_log {
                if kind == FallbackKind::RuntimeJit {
                    log::warn!(
                        "emu aot-strict: runtime_jit_compile reason={reason} at {ip:#x} ({loc})"
                    );
                } else {
                    log::warn!(
                        "emu aot-strict: fallback kind={} mnemonic={mn} at {ip:#x} ({loc})",
                        kind.as_str()
                    );
                }
            }
        }
        AotStrictMode::Fail => {
            let msg = if kind == FallbackKind::RuntimeJit {
                format!(
                    "emu aot-strict: FAIL runtime_jit_compile reason={reason} at {ip:#x} ({loc}) — aborting (exit {EXIT_AOT_FALLBACK})"
                )
            } else {
                format!(
                    "emu aot-strict: FAIL fallback kind={} mnemonic={mn} at {ip:#x} ({loc}) — aborting (exit {EXIT_AOT_FALLBACK})",
                    kind.as_str()
                )
            };
            log::error!("{msg}");
            eprintln!("{msg}");
            win32_rt::process::exit_process(EXIT_AOT_FALLBACK);
        }
        AotStrictMode::Off => {}
    }
}

/// Record a runtime JIT block compilation (when AOT could not supply the block).
#[cold]
#[cfg_attr(not(target_arch = "aarch64"), allow(dead_code))]
pub(super) fn record_jit_compile(ip: u64, reason: &'static str) {
    RUNTIME_JIT_COMPILES.fetch_add(1, Ordering::Relaxed);
    handle_strict_event(ip, FallbackKind::RuntimeJit, u16::MAX, reason, 1, true);
}

/// Record interpreter fast-op execution while the translator is active.
#[inline]
pub(super) fn record_interp_fast(ip: u64, mn: Mnemonic, count: u64) {
    INTERP_FAST_STEPS.fetch_add(count, Ordering::Relaxed);
    if is_active() {
        handle_strict_event(ip, FallbackKind::InterpFast, mn as u16, "", count, false);
    }
}

/// Record a generic interpreter instruction step.
#[inline]
pub(super) fn record_interp_generic(ip: u64, mn: Mnemonic) {
    INTERP_GENERIC_STEPS.fetch_add(1, Ordering::Relaxed);
    if is_active() {
        handle_strict_event(ip, FallbackKind::InterpGeneric, mn as u16, "", 1, false);
    }
}

/// Record an MMX interpreter dispatch step.
#[inline]
pub(super) fn record_interp_mmx(ip: u64, mn: Mnemonic) {
    INTERP_MMX_STEPS.fetch_add(1, Ordering::Relaxed);
    if is_active() {
        handle_strict_event(ip, FallbackKind::InterpMmx, mn as u16, "", 1, false);
    }
}

/// Record a guest trap / syscall boundary step (`int3`, `ud2`, `int`, `syscall`, `sysenter`).
#[inline]
pub(super) fn record_interp_trap(ip: u64, mn: Mnemonic) {
    INTERP_TRAP_STEPS.fetch_add(1, Ordering::Relaxed);
    if is_active() {
        handle_strict_event(ip, FallbackKind::InterpTrap, mn as u16, "", 1, false);
    }
}

/// Record a software-FP denormal fallback step (`denorm.rs`).
#[inline]
pub(super) fn record_soft_fp(ip: u64, mn: Mnemonic) {
    SOFT_FP_STEPS.fetch_add(1, Ordering::Relaxed);
    if is_active() {
        handle_strict_event(ip, FallbackKind::SoftFp, mn as u16, "", 1, false);
    }
}

/// Snapshot process-wide strict AOT fallback metrics for `--report-json` and logging.
pub fn aot_fallback_report() -> AotFallbackReport {
    let aot_insns = AOT_INSNS.load(Ordering::Relaxed);
    let runtime_jit_insns = RUNTIME_JIT_INSNS.load(Ordering::Relaxed);
    let runtime_jit_compiles = RUNTIME_JIT_COMPILES.load(Ordering::Relaxed);
    let interp_fast_steps = INTERP_FAST_STEPS.load(Ordering::Relaxed);
    let interp_generic_steps = INTERP_GENERIC_STEPS.load(Ordering::Relaxed);
    let interp_mmx_steps = INTERP_MMX_STEPS.load(Ordering::Relaxed);
    let interp_trap_steps = INTERP_TRAP_STEPS.load(Ordering::Relaxed);
    let soft_fp_steps = SOFT_FP_STEPS.load(Ordering::Relaxed);
    let total_fallbacks = runtime_jit_compiles
        + interp_fast_steps
        + interp_generic_steps
        + interp_mmx_steps
        + interp_trap_steps
        + soft_fp_steps;

    let mut runtime_jit_reasons: Vec<(String, u64)> = Vec::new();
    let mut top_mnemonics: Vec<(String, u64)> = Vec::new();
    let mut top_sites: Vec<AotFallbackSite> = Vec::new();
    let mut top_jit_compiles: Vec<AotFallbackSite> = Vec::new();

    if let Ok(g) = DETAILS.try_lock() {
        if let Some(d) = g.as_ref() {
            runtime_jit_reasons = d
                .jit_reasons
                .iter()
                .map(|(&r, &c)| (r.to_string(), c))
                .collect();
            runtime_jit_reasons.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

            let mut mns: Vec<(u16, u64)> = d.mnemonics.iter().map(|(&m, &c)| (m, c)).collect();
            mns.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            top_mnemonics = mns
                .into_iter()
                .take(30)
                .map(|(m, c)| (mnemonic_name(m), c))
                .collect();

            let mut exec_sites = Vec::new();
            let mut jit_sites = Vec::new();
            for (&k, &c) in &d.sites {
                if k.1 == FallbackKind::RuntimeJit {
                    jit_sites.push((k, c));
                } else {
                    exec_sites.push((k, c));
                }
            }
            exec_sites.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0 .0.cmp(&b.0 .0)));
            jit_sites.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0 .0.cmp(&b.0 .0)));

            top_sites = exec_sites
                .into_iter()
                .take(30)
                .map(|((ip, kind, mn, reason), count)| AotFallbackSite {
                    location: format_guest_location(ip),
                    ip,
                    kind: kind.as_str(),
                    mnemonic: mnemonic_name(mn),
                    reason: reason.to_string(),
                    count,
                })
                .collect();

            top_jit_compiles = jit_sites
                .into_iter()
                .take(30)
                .map(|((ip, kind, mn, reason), count)| AotFallbackSite {
                    location: format_guest_location(ip),
                    ip,
                    kind: kind.as_str(),
                    mnemonic: mnemonic_name(mn),
                    reason: reason.to_string(),
                    count,
                })
                .collect();
        }
    }

    AotFallbackReport {
        strict_mode: aot_strict_mode().as_str(),
        aot_insns,
        runtime_jit_insns,
        runtime_jit_compiles,
        runtime_jit_reasons,
        interp_fast_steps,
        interp_generic_steps,
        interp_mmx_steps,
        interp_trap_steps,
        soft_fp_steps,
        total_fallbacks,
        top_mnemonics,
        top_sites,
        top_jit_compiles,
    }
}

fn pct(a: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (a as f64) * 100.0 / (total as f64)
    }
}

/// Format a human-readable summary of strict AOT fallback counters.
pub fn format_aot_fallback_summary(r: &AotFallbackReport) -> String {
    let total_insns = r.aot_insns
        + r.runtime_jit_insns
        + r.interp_fast_steps
        + r.interp_generic_steps
        + r.interp_mmx_steps
        + r.interp_trap_steps
        + r.soft_fp_steps;
    let mut out = String::new();
    let _ = writeln!(out, "emu aot-strict summary (mode={}):", r.strict_mode);
    let _ = writeln!(
        out,
        "  guest instrs {total_insns}: aot {} ({:.4}%), runtime_jit {} ({:.4}%), interp_fast {} ({:.4}%), interp_generic {} ({:.4}%), interp_mmx {} ({:.4}%), soft_fp {} ({:.4}%), traps {} ({:.4}%)",
        r.aot_insns,
        pct(r.aot_insns, total_insns),
        r.runtime_jit_insns,
        pct(r.runtime_jit_insns, total_insns),
        r.interp_fast_steps,
        pct(r.interp_fast_steps, total_insns),
        r.interp_generic_steps,
        pct(r.interp_generic_steps, total_insns),
        r.interp_mmx_steps,
        pct(r.interp_mmx_steps, total_insns),
        r.soft_fp_steps,
        pct(r.soft_fp_steps, total_insns),
        r.interp_trap_steps,
        pct(r.interp_trap_steps, total_insns),
    );
    let _ = writeln!(
        out,
        "  runtime_jit_compiles: {} (reasons: {:?})",
        r.runtime_jit_compiles, r.runtime_jit_reasons
    );
    if !r.top_mnemonics.is_empty() {
        let _ = writeln!(out, "  top fallback mnemonics:");
        for (mn, c) in &r.top_mnemonics {
            let _ = writeln!(out, "    {:>12}  {}", c, mn);
        }
    }
    if !r.top_sites.is_empty() {
        let _ = writeln!(out, "  top fallback IPs:");
        for s in &r.top_sites {
            let _ = writeln!(
                out,
                "    {:>12}  {:14}  {:14}  {} ({:#x})",
                s.count, s.kind, s.mnemonic, s.location, s.ip
            );
        }
    }
    if !r.top_jit_compiles.is_empty() {
        let _ = writeln!(out, "  top runtime-JIT compile IPs:");
        for s in &r.top_jit_compiles {
            let _ = writeln!(
                out,
                "    {:>12}  {:18}  {} ({:#x})",
                s.count, s.reason, s.location, s.ip
            );
        }
    }
    out
}

/// Exit hook for `PORT_AOT_STRICT=report`.
pub fn log_exit_summary() {
    if aot_strict_mode() != AotStrictMode::Report {
        return;
    }
    let rep = aot_fallback_report();
    log::info!("{}", format_aot_fallback_summary(&rep));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_strict_modes() {
        assert_eq!(AotStrictMode::parse(""), AotStrictMode::Off);
        assert_eq!(AotStrictMode::parse("0"), AotStrictMode::Off);
        assert_eq!(AotStrictMode::parse("off"), AotStrictMode::Off);
        assert_eq!(AotStrictMode::parse("report"), AotStrictMode::Report);
        assert_eq!(AotStrictMode::parse("REPORT"), AotStrictMode::Report);
        assert_eq!(AotStrictMode::parse("fail"), AotStrictMode::Fail);
        assert_eq!(AotStrictMode::parse("STRICT"), AotStrictMode::Fail);
    }

    #[test]
    fn report_mode_records_jit_interp_and_soft_fp_fallbacks() {
        reset_aot_fallback_counters();
        set_aot_strict_mode(AotStrictMode::Report);
        record_translated_insns(1000, true);
        record_translated_insns(50, false);
        record_jit_compile(0x140001234, "aot_miss");
        record_interp_fast(0x140002000, Mnemonic::Cpuid, 2);
        record_interp_generic(0x140003000, Mnemonic::Xgetbv);
        record_interp_mmx(0x140004000, Mnemonic::Paddw);
        record_soft_fp(0x140005000, Mnemonic::Addss);
        record_interp_trap(0x140006000, Mnemonic::Int3);

        let rep = aot_fallback_report();
        assert_eq!(rep.strict_mode, "report");
        assert_eq!(rep.aot_insns, 1000);
        assert_eq!(rep.runtime_jit_insns, 50);
        assert_eq!(rep.runtime_jit_compiles, 1);
        assert_eq!(rep.interp_fast_steps, 2);
        assert_eq!(rep.interp_generic_steps, 1);
        assert_eq!(rep.interp_mmx_steps, 1);
        assert_eq!(rep.soft_fp_steps, 1);
        assert_eq!(rep.interp_trap_steps, 1);
        assert_eq!(rep.total_fallbacks, 7);
        assert_eq!(rep.runtime_jit_reasons, vec![("aot_miss".to_string(), 1)]);
        assert_eq!(rep.top_jit_compiles.len(), 1);
        assert_eq!(rep.top_sites.len(), 5);
        assert!(format_aot_fallback_summary(&rep).contains("aot 1000"));

        clear_aot_strict_mode_override();
        reset_aot_fallback_counters();
    }
}
