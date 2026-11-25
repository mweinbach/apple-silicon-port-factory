//! Structured diff between two `port-runner --report-json` boot reports
//! (`cargo opsdb -- report-diff <BEFORE.json> <AFTER.json>`).

use crate::stub_inventory::{
    format_runtime_stub_diff, parse_json, parse_runtime_stub_snapshot, JsonValue,
    RuntimeStubSnapshot,
};
use anyhow::{Context, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Default)]
pub struct AotFallbackDiffData {
    pub present: bool,
    pub strict_mode: String,
    pub aot_insns: u64,
    pub runtime_jit_insns: u64,
    pub runtime_jit_compiles: u64,
    pub interp_fast_steps: u64,
    pub interp_generic_steps: u64,
    pub interp_mmx_steps: u64,
    pub interp_trap_steps: u64,
    pub soft_fp_steps: u64,
    pub total_fallbacks: u64,
    pub top_mnemonics: BTreeMap<String, u64>,
    pub top_sites: BTreeMap<String, u64>,
    pub top_jit_compiles: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Default)]
pub struct BootReport {
    pub exe: String,
    pub exit_kind: String,
    pub exit_code: Option<u64>,
    pub exit_detail: String,
    pub elapsed_ms: u64,
    pub frames_presented: u64,
    pub graphics_api: String,
    pub blockers: Vec<(String, String, String, u64)>, // (severity, category, symbol, count)
    pub stubs: RuntimeStubSnapshot,
    pub exec_mode: String,
    pub decoded_interp_blocks: u64,
    pub jit_blocks_compiled: u64,
    pub aot_active: bool,
    pub aot_kills: u64,
    pub aot_revived: u64,
    pub static_aot_blocks: u64,
    pub aot_fallbacks: AotFallbackDiffData,
    pub tier1_memory_hits: u64,
    pub tier1_disk_hits: u64,
    pub tier1_compiles: u64,
    pub tier1_failures: u64,
    pub tier1_compile_ms: u64,
    pub pso_memory_hits: u64,
    pub pso_archive_hits: u64,
    pub pso_compiles: u64,
    pub pso_failures: u64,
    pub pso_compile_ms: u64,
    pub unresolved_imports: BTreeSet<String>,
}

fn get_u64(obj: &BTreeMap<String, JsonValue>, key: &str) -> u64 {
    obj.get(key).and_then(|v| v.as_u64()).unwrap_or(0)
}

fn get_str<'a>(obj: &'a BTreeMap<String, JsonValue>, key: &str) -> &'a str {
    obj.get(key).and_then(|v| v.as_str()).unwrap_or("")
}

fn get_bool(obj: &BTreeMap<String, JsonValue>, key: &str) -> bool {
    obj.get(key).and_then(|v| v.as_bool()).unwrap_or(false)
}

pub fn parse_boot_report(raw: &str) -> Result<BootReport> {
    let root = parse_json(raw)?;
    let obj = root
        .as_obj()
        .ok_or_else(|| anyhow::anyhow!("boot report root must be a JSON object"))?;

    let mut r = BootReport::default();
    if let Some(target) = obj.get("target").and_then(|v| v.as_obj()) {
        r.exe = get_str(target, "exe").to_string();
    }
    if let Some(exit) = obj.get("exit").and_then(|v| v.as_obj()) {
        r.exit_kind = get_str(exit, "kind").to_string();
        r.exit_code = exit.get("code").and_then(|v| v.as_u64());
        r.exit_detail = get_str(exit, "detail").to_string();
        r.elapsed_ms = get_u64(exit, "elapsed_ms");
        r.frames_presented = get_u64(exit, "frames_presented");
        r.graphics_api = get_str(exit, "graphics_api").to_string();
    }
    if let Some(arr) = obj.get("ranked_blockers").and_then(|v| v.as_arr()) {
        for item in arr {
            if let Some(b) = item.as_obj() {
                r.blockers.push((
                    get_str(b, "severity").to_string(),
                    get_str(b, "category").to_string(),
                    get_str(b, "symbol").to_string(),
                    get_u64(b, "count"),
                ));
            }
        }
    }
    if obj.contains_key("stubs") {
        r.stubs = parse_runtime_stub_snapshot(raw)?;
    }
    if let Some(t) = obj.get("tiers").and_then(|v| v.as_obj()) {
        r.exec_mode = get_str(t, "exec_mode").to_string();
        r.decoded_interp_blocks = get_u64(t, "decoded_interp_blocks");
        r.jit_blocks_compiled = get_u64(t, "jit_blocks_compiled");
        r.aot_active = get_bool(t, "aot_active");
        r.aot_kills = get_u64(t, "aot_kills");
        r.aot_revived = get_u64(t, "aot_revived");
        r.static_aot_blocks = get_u64(t, "static_aot_blocks");
    }
    if let Some(af) = obj.get("aot_fallbacks").and_then(|v| v.as_obj()) {
        let d = &mut r.aot_fallbacks;
        d.present = true;
        d.strict_mode = get_str(af, "strict_mode").to_string();
        d.aot_insns = get_u64(af, "aot_insns");
        d.runtime_jit_insns = get_u64(af, "runtime_jit_insns");
        d.runtime_jit_compiles = get_u64(af, "runtime_jit_compiles");
        d.interp_fast_steps = get_u64(af, "interp_fast_steps");
        d.interp_generic_steps = get_u64(af, "interp_generic_steps");
        d.interp_mmx_steps = get_u64(af, "interp_mmx_steps");
        d.interp_trap_steps = get_u64(af, "interp_trap_steps");
        d.soft_fp_steps = get_u64(af, "soft_fp_steps");
        d.total_fallbacks = get_u64(af, "total_fallbacks");
        if let Some(arr) = af.get("top_mnemonics").and_then(|v| v.as_arr()) {
            for item in arr {
                if let Some(m) = item.as_obj() {
                    let name = get_str(m, "mnemonic");
                    if !name.is_empty() {
                        d.top_mnemonics.insert(name.to_string(), get_u64(m, "count"));
                    }
                }
            }
        }
        if let Some(arr) = af.get("top_sites").and_then(|v| v.as_arr()) {
            for item in arr {
                if let Some(s) = item.as_obj() {
                    let loc = get_str(s, "location");
                    let kind = get_str(s, "kind");
                    let mn = get_str(s, "mnemonic");
                    if !loc.is_empty() {
                        d.top_sites
                            .insert(format!("[{kind}:{mn}] {loc}"), get_u64(s, "count"));
                    }
                }
            }
        }
        if let Some(arr) = af.get("top_jit_compiles").and_then(|v| v.as_arr()) {
            for item in arr {
                if let Some(s) = item.as_obj() {
                    let loc = get_str(s, "location");
                    let reason = get_str(s, "reason");
                    if !loc.is_empty() {
                        d.top_jit_compiles
                            .insert(format!("[{reason}] {loc}"), get_u64(s, "count"));
                    }
                }
            }
        }
    }
    if let Some(s) = obj.get("shaders").and_then(|v| v.as_obj()) {
        r.tier1_memory_hits = get_u64(s, "tier1_memory_hits");
        r.tier1_disk_hits = get_u64(s, "tier1_disk_hits");
        r.tier1_compiles = get_u64(s, "tier1_compiles");
        r.tier1_failures = get_u64(s, "tier1_failures");
        r.tier1_compile_ms = get_u64(s, "tier1_compile_ms");
        r.pso_memory_hits = get_u64(s, "pso_memory_hits");
        r.pso_archive_hits = get_u64(s, "pso_archive_hits");
        r.pso_compiles = get_u64(s, "pso_compiles");
        r.pso_failures = get_u64(s, "pso_failures");
        r.pso_compile_ms = get_u64(s, "pso_compile_ms");
    }
    if let Some(arr) = obj.get("unresolved_imports").and_then(|v| v.as_arr()) {
        for item in arr {
            if let Some(sym) = item.as_str() {
                r.unresolved_imports.insert(sym.to_string());
            }
        }
    }
    Ok(r)
}

fn fmt_delta(before: u64, after: u64) -> String {
    let d = after as i128 - before as i128;
    format!("{before} -> {after} ({d:+})")
}

pub fn format_boot_report_diff(before: &BootReport, after: &BootReport) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "=== Boot Report Diff ===");
    let _ = writeln!(
        out,
        "  Target           : {} -> {}",
        before.exe, after.exe
    );
    let _ = writeln!(
        out,
        "  Exit Kind        : {} (code {:?}) -> {} (code {:?})",
        before.exit_kind, before.exit_code, after.exit_kind, after.exit_code
    );
    if before.exit_detail != after.exit_detail && (!before.exit_detail.is_empty() || !after.exit_detail.is_empty()) {
        let _ = writeln!(
            out,
            "  Exit Detail      : {:?} -> {:?}",
            before.exit_detail, after.exit_detail
        );
    }
    let _ = writeln!(
        out,
        "  Frames Presented : {}  (API: {} -> {})",
        fmt_delta(before.frames_presented, after.frames_presented),
        before.graphics_api,
        after.graphics_api
    );
    let _ = writeln!(
        out,
        "  Elapsed (ms)     : {}",
        fmt_delta(before.elapsed_ms, after.elapsed_ms)
    );

    // Blockers diff (fatal + error)
    let fatal_set = |r: &BootReport| -> BTreeSet<String> {
        r.blockers
            .iter()
            .filter(|(sev, _, _, _)| sev == "fatal" || sev == "error")
            .map(|(sev, cat, sym, _)| format!("[{sev}:{cat}] {sym}"))
            .collect()
    };
    let bf = fatal_set(before);
    let af = fatal_set(after);
    let new_fatal: Vec<_> = af.difference(&bf).collect();
    let fixed_fatal: Vec<_> = bf.difference(&af).collect();
    if !new_fatal.is_empty() || !fixed_fatal.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(out, "Fatal/Error Blockers:");
        for b in new_fatal {
            let _ = writeln!(out, "  + {b}");
        }
        for b in fixed_fatal {
            let _ = writeln!(out, "  - {b}");
        }
    }

    // Unresolved static imports diff
    let new_unres: Vec<_> = after.unresolved_imports.difference(&before.unresolved_imports).collect();
    let fixed_unres: Vec<_> = before.unresolved_imports.difference(&after.unresolved_imports).collect();
    if !new_unres.is_empty() || !fixed_unres.is_empty() {
        let _ = writeln!(out);
        let _ = writeln!(
            out,
            "Unresolved Imports: {}",
            fmt_delta(before.unresolved_imports.len() as u64, after.unresolved_imports.len() as u64)
        );
        for s in new_unres {
            let _ = writeln!(out, "  + {s}");
        }
        for s in fixed_unres {
            let _ = writeln!(out, "  - {s}");
        }
    }

    // Execution tiers diff
    let _ = writeln!(out);
    let _ = writeln!(out, "Execution Tiers:");
    let _ = writeln!(
        out,
        "  exec_mode             : {} -> {}",
        before.exec_mode, after.exec_mode
    );
    let _ = writeln!(
        out,
        "  decoded_interp_blocks : {}",
        fmt_delta(before.decoded_interp_blocks, after.decoded_interp_blocks)
    );
    let _ = writeln!(
        out,
        "  jit_blocks_compiled   : {}",
        fmt_delta(before.jit_blocks_compiled, after.jit_blocks_compiled)
    );
    let _ = writeln!(
        out,
        "  aot_active            : {} -> {} (kills: {}, revived: {}, static_blocks: {})",
        before.aot_active,
        after.aot_active,
        fmt_delta(before.aot_kills, after.aot_kills),
        fmt_delta(before.aot_revived, after.aot_revived),
        fmt_delta(before.static_aot_blocks, after.static_aot_blocks)
    );

    if before.aot_fallbacks.present || after.aot_fallbacks.present {
        let bf = &before.aot_fallbacks;
        let af = &after.aot_fallbacks;
        let _ = writeln!(out);
        let _ = writeln!(out, "AOT Fallbacks:");
        let _ = writeln!(
            out,
            "  strict_mode           : {} -> {}",
            bf.strict_mode, af.strict_mode
        );
        let _ = writeln!(
            out,
            "  aot_insns             : {}",
            fmt_delta(bf.aot_insns, af.aot_insns)
        );
        let _ = writeln!(
            out,
            "  runtime_jit_insns     : {}",
            fmt_delta(bf.runtime_jit_insns, af.runtime_jit_insns)
        );
        let _ = writeln!(
            out,
            "  runtime_jit_compiles  : {}",
            fmt_delta(bf.runtime_jit_compiles, af.runtime_jit_compiles)
        );
        let _ = writeln!(
            out,
            "  interp_fast_steps     : {}",
            fmt_delta(bf.interp_fast_steps, af.interp_fast_steps)
        );
        let _ = writeln!(
            out,
            "  interp_generic_steps  : {}",
            fmt_delta(bf.interp_generic_steps, af.interp_generic_steps)
        );
        let _ = writeln!(
            out,
            "  interp_mmx_steps      : {}",
            fmt_delta(bf.interp_mmx_steps, af.interp_mmx_steps)
        );
        let _ = writeln!(
            out,
            "  interp_trap_steps     : {}",
            fmt_delta(bf.interp_trap_steps, af.interp_trap_steps)
        );
        let _ = writeln!(
            out,
            "  soft_fp_steps         : {}",
            fmt_delta(bf.soft_fp_steps, af.soft_fp_steps)
        );
        let _ = writeln!(
            out,
            "  total_fallbacks       : {}",
            fmt_delta(bf.total_fallbacks, af.total_fallbacks)
        );
        let new_sites: Vec<_> = af
            .top_sites
            .iter()
            .filter(|(k, _)| !bf.top_sites.contains_key(*k))
            .collect();
        let gone_sites: Vec<_> = bf
            .top_sites
            .iter()
            .filter(|(k, _)| !af.top_sites.contains_key(*k))
            .collect();
        for (site, c) in new_sites.iter().take(15) {
            let _ = writeln!(out, "  + fallback site {site} ({c})");
        }
        for (site, c) in gone_sites.iter().take(15) {
            let _ = writeln!(out, "  - fallback site {site} ({c})");
        }
        let new_jit: Vec<_> = af
            .top_jit_compiles
            .iter()
            .filter(|(k, _)| !bf.top_jit_compiles.contains_key(*k))
            .collect();
        let gone_jit: Vec<_> = bf
            .top_jit_compiles
            .iter()
            .filter(|(k, _)| !af.top_jit_compiles.contains_key(*k))
            .collect();
        for (site, c) in new_jit.iter().take(15) {
            let _ = writeln!(out, "  + runtime-jit {site} ({c})");
        }
        for (site, c) in gone_jit.iter().take(15) {
            let _ = writeln!(out, "  - runtime-jit {site} ({c})");
        }
    }

    // Shaders & PSOs diff
    let _ = writeln!(out);
    let _ = writeln!(out, "Shaders & PSOs:");
    let _ = writeln!(
        out,
        "  Tier-1 compiles       : {} (failures: {}, disk_hits: {}, mem_hits: {}, ms: {})",
        fmt_delta(before.tier1_compiles, after.tier1_compiles),
        fmt_delta(before.tier1_failures, after.tier1_failures),
        fmt_delta(before.tier1_disk_hits, after.tier1_disk_hits),
        fmt_delta(before.tier1_memory_hits, after.tier1_memory_hits),
        fmt_delta(before.tier1_compile_ms, after.tier1_compile_ms)
    );
    let _ = writeln!(
        out,
        "  Tier-2 PSO compiles   : {} (failures: {}, archive_hits: {}, mem_hits: {}, ms: {})",
        fmt_delta(before.pso_compiles, after.pso_compiles),
        fmt_delta(before.pso_failures, after.pso_failures),
        fmt_delta(before.pso_archive_hits, after.pso_archive_hits),
        fmt_delta(before.pso_memory_hits, after.pso_memory_hits),
        fmt_delta(before.pso_compile_ms, after.pso_compile_ms)
    );

    // Embedded stub snapshot diff
    let _ = writeln!(out);
    out.push_str(&format_runtime_stub_diff(&before.stubs, &after.stubs));
    out
}

pub fn diff_boot_report_files(before_path: &Path, after_path: &Path) -> Result<String> {
    let b_raw = fs::read_to_string(before_path)
        .with_context(|| format!("reading {}", before_path.display()))?;
    let a_raw = fs::read_to_string(after_path)
        .with_context(|| format!("reading {}", after_path.display()))?;
    let before = parse_boot_report(&b_raw)
        .with_context(|| format!("parsing {}", before_path.display()))?;
    let after = parse_boot_report(&a_raw)
        .with_context(|| format!("parsing {}", after_path.display()))?;
    Ok(format_boot_report_diff(&before, &after))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_diff_highlights_exit_tiers_shaders_and_stub_progress() {
        let before_json = r#"{
          "schema_version": 1,
          "target": { "exe": "game.exe", "game_dir": "/game", "manifest": "" },
          "exit": { "kind": "unimplemented_import", "code": 3221225785, "detail": "advapi32.dll!CredWriteW", "elapsed_ms": 400, "frames_presented": 0, "graphics_api": "unknown" },
          "ranked_blockers": [
            { "rank": 1, "severity": "fatal", "category": "unimplemented_import", "symbol": "advapi32.dll!CredWriteW", "count": 1 }
          ],
          "stubs": {
            "total_hits": 4,
            "unique_sites": 1,
            "entries": [
              { "dll": "ws2_32.dll", "api": "WSAJoinLeaf", "kind": "failing", "hits": 4, "first_frame": 0, "first_guest_caller_va": 4096, "first_guest_caller": "game.exe+0x1000", "file": "crates/win32-rt/src/misc/ws2_32.rs", "line": 3350, "first_detail": "WSAJoinLeaf" }
            ]
          },
          "tiers": { "exec_mode": "dynamic_jit", "decoded_interp_blocks": 50, "jit_blocks_compiled": 12, "aot_active": false, "aot_kills": 0, "aot_revived": 0, "static_aot_modules": 0, "static_aot_blocks": 0, "static_aot_bytes": 0 },
          "shaders": { "tier1_initialized": true, "tier1_memory_hits": 0, "tier1_disk_hits": 2, "tier1_compiles": 5, "tier1_failures": 1, "tier1_compile_ms": 120, "pso_initialized": true, "pso_memory_hits": 0, "pso_archive_hits": 0, "pso_compiles": 3, "pso_failures": 0, "pso_compile_ms": 80 },
          "unresolved_imports": ["advapi32.dll!CredWriteW"]
        }"#;
        let after_json = r#"{
          "schema_version": 1,
          "target": { "exe": "game.exe", "game_dir": "/game", "manifest": "" },
          "exit": { "kind": "clean_exit", "code": 0, "detail": "", "elapsed_ms": 250, "frames_presented": 60, "graphics_api": "d3d11" },
          "ranked_blockers": [],
          "stubs": { "total_hits": 0, "unique_sites": 0, "entries": [] },
          "tiers": { "exec_mode": "dynamic_jit", "decoded_interp_blocks": 80, "jit_blocks_compiled": 45, "aot_active": true, "aot_kills": 0, "aot_revived": 0, "static_aot_modules": 0, "static_aot_blocks": 0, "static_aot_bytes": 0 },
          "shaders": { "tier1_initialized": true, "tier1_memory_hits": 4, "tier1_disk_hits": 7, "tier1_compiles": 0, "tier1_failures": 0, "tier1_compile_ms": 0, "pso_initialized": true, "pso_memory_hits": 2, "pso_archive_hits": 3, "pso_compiles": 0, "pso_failures": 0, "pso_compile_ms": 0 },
          "unresolved_imports": []
        }"#;

        let b = parse_boot_report(before_json).unwrap();
        let a = parse_boot_report(after_json).unwrap();
        let diff = format_boot_report_diff(&b, &a);
        assert!(diff.contains("unimplemented_import (code Some(3221225785)) -> clean_exit (code Some(0))"));
        assert!(diff.contains("0 -> 60 (+60)"));
        assert!(diff.contains("- [fatal:unimplemented_import] advapi32.dll!CredWriteW"));
        assert!(diff.contains("- advapi32.dll!CredWriteW"));
        assert!(diff.contains("RESOLVED stubs (-1)"));
        assert!(diff.contains("ws2_32.dll!WSAJoinLeaf"));
        assert!(diff.contains("12 -> 45 (+33)"));
        assert!(diff.contains("5 -> 0 (-5)"));
    }

    #[test]
    fn report_diff_surfaces_aot_fallback_regressions_and_fixes() {
        let before_json = r#"{
          "schema_version": 1,
          "target": { "exe": "app.exe", "game_dir": "/app", "manifest": "" },
          "exit": { "kind": "clean_exit", "code": 0, "detail": "", "elapsed_ms": 100, "frames_presented": 10, "graphics_api": "d3d12" },
          "ranked_blockers": [],
          "stubs": { "total_hits": 0, "unique_sites": 0, "entries": [] },
          "tiers": { "exec_mode": "jit_writable", "decoded_interp_blocks": 10, "jit_blocks_compiled": 5, "aot_active": true, "aot_kills": 0, "aot_revived": 0, "static_aot_modules": 0, "static_aot_blocks": 0, "static_aot_bytes": 0 },
          "aot_fallbacks": {
            "strict_mode": "report",
            "aot_insns": 100000,
            "runtime_jit_insns": 500,
            "runtime_jit_compiles": 5,
            "interp_fast_steps": 12,
            "interp_generic_steps": 3,
            "interp_mmx_steps": 0,
            "interp_trap_steps": 0,
            "soft_fp_steps": 1,
            "total_fallbacks": 21,
            "runtime_jit_reasons": [{ "reason": "aot_miss", "count": 5 }],
            "top_mnemonics": [{ "mnemonic": "xgetbv", "count": 3 }],
            "top_sites": [{ "location": "app.exe+0x1200", "ip": 4608, "kind": "interp_generic", "mnemonic": "xgetbv", "count": 3 }],
            "top_jit_compiles": [{ "location": "app.exe+0x1204", "ip": 4612, "reason": "aot_miss", "count": 1 }]
          },
          "shaders": { "tier1_initialized": false, "tier1_memory_hits": 0, "tier1_disk_hits": 0, "tier1_compiles": 0, "tier1_failures": 0, "tier1_compile_ms": 0, "pso_initialized": false, "pso_memory_hits": 0, "pso_archive_hits": 0, "pso_compiles": 0, "pso_failures": 0, "pso_compile_ms": 0 },
          "unresolved_imports": []
        }"#;
        let after_json = r#"{
          "schema_version": 1,
          "target": { "exe": "app.exe", "game_dir": "/app", "manifest": "" },
          "exit": { "kind": "clean_exit", "code": 0, "detail": "", "elapsed_ms": 95, "frames_presented": 10, "graphics_api": "d3d12" },
          "ranked_blockers": [],
          "stubs": { "total_hits": 0, "unique_sites": 0, "entries": [] },
          "tiers": { "exec_mode": "jit_writable", "decoded_interp_blocks": 2, "jit_blocks_compiled": 1, "aot_active": true, "aot_kills": 0, "aot_revived": 0, "static_aot_modules": 0, "static_aot_blocks": 0, "static_aot_bytes": 0 },
          "aot_fallbacks": {
            "strict_mode": "report",
            "aot_insns": 100500,
            "runtime_jit_insns": 0,
            "runtime_jit_compiles": 1,
            "interp_fast_steps": 0,
            "interp_generic_steps": 3,
            "interp_mmx_steps": 0,
            "interp_trap_steps": 0,
            "soft_fp_steps": 0,
            "total_fallbacks": 4,
            "runtime_jit_reasons": [{ "reason": "aot_miss", "count": 1 }],
            "top_mnemonics": [{ "mnemonic": "xgetbv", "count": 3 }],
            "top_sites": [{ "location": "app.exe+0x1200", "ip": 4608, "kind": "interp_generic", "mnemonic": "xgetbv", "count": 3 }],
            "top_jit_compiles": [{ "location": "app.exe+0x9000", "ip": 36864, "reason": "aot_miss", "count": 1 }]
          },
          "shaders": { "tier1_initialized": false, "tier1_memory_hits": 0, "tier1_disk_hits": 0, "tier1_compiles": 0, "tier1_failures": 0, "tier1_compile_ms": 0, "pso_initialized": false, "pso_memory_hits": 0, "pso_archive_hits": 0, "pso_compiles": 0, "pso_failures": 0, "pso_compile_ms": 0 },
          "unresolved_imports": []
        }"#;

        let b = parse_boot_report(before_json).unwrap();
        let a = parse_boot_report(after_json).unwrap();
        let diff = format_boot_report_diff(&b, &a);
        assert!(diff.contains("AOT Fallbacks:"));
        assert!(diff.contains("runtime_jit_compiles  : 5 -> 1 (-4)"));
        assert!(diff.contains("interp_fast_steps     : 12 -> 0 (-12)"));
        assert!(diff.contains("soft_fp_steps         : 1 -> 0 (-1)"));
        assert!(diff.contains("- runtime-jit [aot_miss] app.exe+0x1204 (1)"));
        assert!(diff.contains("+ runtime-jit [aot_miss] app.exe+0x9000 (1)"));
    }
}
