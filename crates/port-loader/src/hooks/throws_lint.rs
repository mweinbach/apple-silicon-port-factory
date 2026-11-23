//! Hook-safety lint (`docs/design/ADVICE_SEH.md` §5 step 1).
//!
//! Every registry entry declares `throws: Throws::{No, Discard, Caught}`. `tools/fndb.sh sync`
//! writes `db/throws.tsv` with the original's can-throw masks (`fndb can-throw`) and its own EH
//! shape (FH4 try map / unwind map). The table is embedded at compile time so the lint needs no
//! files at run time. At hook time findings are logged as warnings; `--diff-test` (and the
//! `registry_difftests_against_exe` test) count errors as failures.

use std::collections::HashMap;

/// Fence frames (hook shim with unwind info + unwinder skip over Rust frames) are implemented:
/// replacements annotated `Discard`/`Caught` are entered through a fence shim (`shim::build_shim`).
pub(super) const FENCES: bool = true;
/// `exe::try_call` (catch an MSVC exception from an original callee in Rust) is implemented
/// (`shim::install_try_thunk`).
pub(super) const TRY_CALL: bool = true;

#[cfg(feature = "aw2-core")]
pub(super) const THROWS_TSV: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../db/throws.tsv"));
#[cfg(not(feature = "aw2-core"))]
pub(super) const THROWS_TSV: &str = "";

// Mask bits from tools/fndb/canthrow.py.
pub(super) const T_CXX: u32 = 1;
pub(super) const T_CONTRACT: u32 = 2;
pub(super) const T_SEH: u32 = 4;
pub(super) const T_UNKNOWN: u32 = 8;
pub(super) const T_STD: u32 = 16;
/// Exceptions a game `catch` may handle (contract throws are treated as abort-equivalent).
const T_CATCHABLE: u32 = T_CXX | T_SEH | T_STD;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct ThrowFacts {
    /// Bits reachable through direct calls only.
    pub(super) direct: u32,
    /// Bits reachable when vcalls are followed (over-approximated by vftable slot).
    pub(super) all: u32,
    /// The original's own EH shape: `none`, `cleanup`, `catch`, `noexcept`.
    pub(super) eh: &'static str,
    pub(super) witness: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum LintLevel {
    Debug,
    Warn,
    Error,
}

pub(super) fn parse_throws_tsv(text: &'static str) -> HashMap<u64, ThrowFacts> {
    let mut out = HashMap::new();
    for line in text.lines() {
        if line.starts_with('#') || line.starts_with("va\t") || line.trim().is_empty() {
            continue;
        }
        let f: Vec<&'static str> = line.split('\t').collect();
        let (Some(va), Some(direct), Some(all)) = (
            f.first().and_then(|s| u64::from_str_radix(s.trim_start_matches("0x"), 16).ok()),
            f.get(1).and_then(|s| s.parse().ok()),
            f.get(2).and_then(|s| s.parse().ok()),
        ) else {
            continue;
        };
        out.insert(
            va,
            ThrowFacts {
                direct,
                all,
                eh: f.get(3).copied().unwrap_or("none"),
                witness: f.get(4).copied().unwrap_or(""),
            },
        );
    }
    out
}

/// Findings for one entry (pure; see the module comment for the policy).
pub(super) fn lint_rule(
    throws: x86_runtime::registry_types::Throws,
    f: &ThrowFacts,
) -> Vec<(LintLevel, String)> {
    use x86_runtime::registry_types::Throws;
    use LintLevel::*;
    let mut out = Vec::new();
    let catchable = f.direct & T_CATCHABLE != 0;
    let contract = f.direct & T_CONTRACT != 0;
    let wit = if f.witness.is_empty() { String::new() } else { format!(" [via {}]", f.witness) };
    // What the original requires.
    if f.eh == "catch" && catchable && throws != Throws::Caught {
        out.push((
            Error,
            format!(
                "original has a try map and a throwing callee: needs Throws::Caught (exe::try_call){wit}"
            ),
        ));
    } else if f.eh == "catch" && !catchable && throws != Throws::Caught {
        out.push((
            Warn,
            "original has a try map but no direct throwing callee: check what it catches (vcall/indirect callee, /EHa fault?)".into(),
        ));
    } else if f.eh == "cleanup" && catchable && throws != Throws::Caught {
        out.push((
            Error,
            format!(
                "original runs destructors on unwind and a callee can throw: needs Throws::Caught (cleanup on Err, rethrow){wit}"
            ),
        ));
    } else if (catchable || contract) && throws == Throws::No {
        out.push((
            Error,
            format!(
                "a callee can throw ({}): annotate Throws::Discard (no live Drop values across it) or Caught{wit}",
                mask_names(f.direct)
            ),
        ));
    }
    // What the annotation needs from the runtime.
    if throws == Throws::Discard && catchable && !FENCES {
        out.push((
            Error,
            "Throws::Discard with catchable throws needs fence frames (disabled): an exception would hit the Rust frame and be fatal".into(),
        ));
    }
    if throws == Throws::Caught && !TRY_CALL {
        out.push((Error, "Throws::Caught needs exe::try_call (not implemented yet)".into()));
    }
    // Over-annotation and weak evidence.
    if throws != Throws::No && f.direct & (T_CATCHABLE | T_CONTRACT) == 0 {
        let lvl = if f.all & T_CATCHABLE != 0 { Debug } else { Warn };
        out.push((
            lvl,
            format!(
                "annotated {throws:?} but no direct callee can throw (with vcalls: {})",
                mask_names(f.all)
            ),
        ));
    }
    if throws == Throws::No && f.direct & (T_CATCHABLE | T_CONTRACT) == 0 && f.all != 0 {
        out.push((
            Debug,
            format!(
                "may throw only via vcalls/indirect calls (over-approximation): {}",
                mask_names(f.all)
            ),
        ));
    }
    out
}

fn mask_names(m: u32) -> String {
    let names: Vec<&str> = [
        (T_CXX, "cxx"),
        (T_CONTRACT, "contract"),
        (T_SEH, "seh"),
        (T_UNKNOWN, "unknown-indirect"),
        (T_STD, "std"),
    ]
    .iter()
    .filter(|(b, _)| m & b != 0)
    .map(|(_, n)| *n)
    .collect();
    if names.is_empty() {
        "none".into()
    } else {
        names.join(",")
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct LintSummary {
    pub(super) errors: usize,
    pub(super) warnings: usize,
}

pub(super) fn hook_lint_with_tsv(
    set: &x86_runtime::game_plugin::ReplacementSet,
    tsv: &'static str,
) -> LintSummary {
    if tsv.is_empty() {
        log::info!("hook-lint: no throws table embedded; skipping");
        return LintSummary::default();
    }
    let facts = parse_throws_tsv(tsv);
    let mut s = LintSummary::default();
    for &r in &set.all {
        let Some(f) = facts.get(&r.va) else {
            log::warn!(
                "hook-lint {} @0x{:x}: no row in db/throws.tsv (run tools/fndb.sh sync)",
                r.name,
                r.va
            );
            s.warnings += 1;
            continue;
        };
        for (lvl, msg) in lint_rule(r.throws, f) {
            match lvl {
                LintLevel::Error => {
                    log::warn!(
                        "hook-lint ERROR {} @0x{:x}: {msg} (fails --diff-test)",
                        r.name,
                        r.va
                    );
                    s.errors += 1;
                }
                LintLevel::Warn => {
                    if r.diff.is_some() {
                        log::warn!("hook-lint {} @0x{:x}: {msg}", r.name, r.va);
                    } else {
                        log::debug!("hook-lint {} @0x{:x}: {msg}", r.name, r.va);
                    }
                    s.warnings += 1;
                }
                LintLevel::Debug => log::debug!("hook-lint {} @0x{:x}: {msg}", r.name, r.va),
            }
        }
    }
    log::info!(
        "hook-lint: {} replacements, {} error(s), {} warning(s) (fences: {FENCES}, try_call: {TRY_CALL})",
        set.total_count,
        s.errors,
        s.warnings
    );
    s
}

/// Lint the registry once (cached) and log the findings. Errors are logged as warnings here
/// (dev boots keep going); `run_difftests` turns them into failures.
pub(super) fn hook_lint(set: &x86_runtime::game_plugin::ReplacementSet) -> LintSummary {
    static ONCE: std::sync::OnceLock<LintSummary> = std::sync::OnceLock::new();
    *ONCE.get_or_init(|| hook_lint_with_tsv(set, THROWS_TSV))
}
