//! Offline whole-binary & DLL differential verifier CLI (`x86-verify`).
//!
//! Scans one or more Windows x86-64 `.exe`/`.dll` files (or an entire game directory),
//! maps each PE offline (zero `DllMain`/`WinMain`, zero GPU/window), discovers all reachable
//! basic blocks, and executes every block + linked multi-block CFG traces across:
//!   1. Generic `iced-x86` interpreter (reference)
//!   2. FastOp interpreter (`fast.rs`)
//!   3. ARM64 Live JIT (`jit.rs`)
//!   4. ARM64 AOT (`aot.rs`: hot/cold split + rebase + `link_pass` fallthrough-nop & cond-patch)
//!
//! Usage:
//!   cargo run --release -p x86-runtime --bin x86-verify -- /path/to/GameDir
//!   cargo run --release -p x86-runtime --bin x86-verify -- /path/to/PhysX3_x64.dll --trials 4
//!   cargo run --release -p x86-runtime --bin x86-verify -- /path/to/Game.exe --fp-only --oracle-dir /tmp/oracle --oracle-sample 64

#[cfg(target_arch = "aarch64")]
fn json_escape(s: &str) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

#[cfg(target_arch = "aarch64")]
fn emit_json_error(code: u8, msg: &str) {
    println!(
        "{{\n  \"schema_version\": 1,\n  \"command\": \"verify-x86\",\n  \"ok\": false,\n  \"exit_code\": {},\n  \"data\": null,\n  \"error\": {}\n}}",
        code,
        json_escape(msg)
    );
}

#[cfg(target_arch = "aarch64")]
fn main() -> std::process::ExitCode {
    use std::fmt::Write as _;
    use std::path::PathBuf;
    use std::process::ExitCode;
    use x86_runtime::emu::verify::{
        collect_pe_targets, verify_pe_file, BinaryVerifyReport, VerifyConfig,
    };

    const EXIT_OK: u8 = 0;
    const EXIT_USAGE: u8 = 2;
    const EXIT_VERIFICATION_FAILED: u8 = 4;

    // Offline CLI has no main process exe mapped at 0x140000000; clear the default so
    // `dll_native::in_ro_section` and `to_mapped_addr` never probe unmapped addresses.
    x86_runtime::exe::set_base(0);
    x86_runtime::exe::set_preferred_base(0, 0);
    // Offline CLI does not install the Win32 SIGBUS VEH handler; disable LSE2 trap patching
    // by default so adversarial unaligned-pointer trials use inline TSO barriers cleanly.
    if std::env::var_os("PORT_JIT_LSE2_TRAP").is_none() {
        std::env::set_var("PORT_JIT_LSE2_TRAP", "0");
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    let want_json = args.iter().any(|a| a == "--json");
    // Default to exact IEEE-754 NaN propagation unless `--fast-nan` or `PORT_JIT_FAST_NAN` is set.
    let want_fast_nan = args.iter().any(|a| a == "--fast-nan");
    if want_fast_nan {
        std::env::set_var("PORT_JIT_FAST_NAN", "1");
    } else if std::env::var_os("PORT_JIT_FAST_NAN").is_none() {
        std::env::set_var("PORT_JIT_FAST_NAN", "0");
    }
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        let usage_msg = "Usage: x86-verify <exe|dll|game-dir> [...] [options]\n\
             Options:\n\
             \x20 --json                  Emit stable schema v1 JSON envelope on stdout\n\
             \x20 --max-blocks N          Cap blocks verified per binary (default 0 = all)\n\
             \x20 --trials N              Adversarial input trials per block (default 3)\n\
             \x20 --rva LO..HI            Only verify blocks in RVA range [LO, HI)\n\
             \x20 --fp-only               Only verify blocks containing FP/SIMD instructions\n\
             \x20 --no-fast               Skip FastOp vs Generic check\n\
             \x20 --no-jit                Skip Live JIT vs Generic check\n\
             \x20 --no-aot                Skip Unlinked AOT vs Generic check\n\
             \x20 --no-linked             Skip Linked multi-block AOT check\n\
             \x20 --fast-nan              Test with PORT_JIT_FAST_NAN=1 (default: 0, exact NaN parity)\n\
             \x20 --strict-nan-payload    Treat NaN-payload-only diffs as critical mismatches\n\
             \x20 --threads N             Threads for discovery & AOT builds\n\
             \x20 --max-mismatches N      Max mismatch records printed per binary (default 64)\n\
             \x20 --oracle-dir DIR        Export Rosetta x86-64 oracle snapshots to DIR\n\
             \x20 --oracle-sample N       Export up to N passing FP/SIMD instructions per binary\n\
             \x20 --only SUBSTR           Only scan binaries whose filename contains SUBSTR (case-insensitive)";
        if want_json {
            emit_json_error(EXIT_USAGE, "missing input target(s) for x86-verify");
        } else {
            eprintln!("{usage_msg}");
        }
        return ExitCode::from(EXIT_USAGE);
    }

    let mut cfg = VerifyConfig::default();
    let mut inputs: Vec<PathBuf> = Vec::new();
    let mut only_filter: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => {}
            "--max-blocks" => {
                i += 1;
                cfg.max_blocks = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(0);
            }
            "--trials" => {
                i += 1;
                cfg.trials = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(3).max(1);
            }
            "--threads" => {
                i += 1;
                cfg.threads = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(4).max(1);
            }
            "--max-mismatches" => {
                i += 1;
                cfg.max_mismatches = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(64);
            }
            "--oracle-dir" => {
                i += 1;
                if let Some(d) = args.get(i) {
                    cfg.oracle_dir = Some(PathBuf::from(d));
                }
            }
            "--oracle-sample" => {
                i += 1;
                cfg.oracle_sample = args.get(i).and_then(|s| s.parse().ok()).unwrap_or(0);
            }
            "--only" => {
                i += 1;
                only_filter = args.get(i).map(|s| s.to_ascii_lowercase());
            }
            "--rva" => {
                i += 1;
                if let Some(span) = args.get(i) {
                    if let Some((a, b)) = span.split_once("..") {
                        let parse_hex = |s: &str| {
                            let t = s.trim();
                            if let Some(h) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
                                u32::from_str_radix(h, 16).ok()
                            } else {
                                t.parse::<u32>().ok()
                            }
                        };
                        if let (Some(lo), Some(hi)) = (parse_hex(a), parse_hex(b)) {
                            cfg.rva_filter = Some((lo, hi));
                        }
                    }
                }
            }
            "--fp-only" => cfg.fp_only = true,
            "--no-fast" => cfg.check_fast = false,
            "--no-jit" => cfg.check_jit = false,
            "--no-aot" => cfg.check_aot = false,
            "--no-linked" => cfg.check_linked_aot = false,
            "--fast-nan" => {}
            "--strict-nan-payload" => cfg.strict_nan_payload = true,
            other if other.starts_with('-') => {
                let msg = format!("Unknown option: {other}");
                if want_json {
                    emit_json_error(EXIT_USAGE, &msg);
                } else {
                    eprintln!("{msg}");
                }
                return ExitCode::from(EXIT_USAGE);
            }
            path => inputs.push(PathBuf::from(path)),
        }
        i += 1;
    }

    let mut targets = Vec::new();
    for inp in &inputs {
        match collect_pe_targets(inp) {
            Ok(mut v) => targets.append(&mut v),
            Err(e) => {
                if want_json {
                    emit_json_error(EXIT_USAGE, &e);
                } else {
                    eprintln!("error: {e}");
                }
                return ExitCode::from(EXIT_USAGE);
            }
        }
    }
    if let Some(ref sub) = only_filter {
        targets.retain(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().to_ascii_lowercase().contains(sub.as_str()))
                .unwrap_or(false)
        });
    }
    if targets.is_empty() {
        let msg = "No PE32+ .exe/.dll targets found.";
        if want_json {
            emit_json_error(EXIT_USAGE, msg);
        } else {
            eprintln!("{msg}");
        }
        return ExitCode::from(EXIT_USAGE);
    }

    if !want_json {
        println!(
            "x86-verify: scanning {} binary target(s) (trials={}, fp_only={}, fast={}, jit={}, aot={}, linked_aot={})",
            targets.len(),
            cfg.trials,
            cfg.fp_only,
            cfg.check_fast,
            cfg.check_jit,
            cfg.check_aot,
            cfg.check_linked_aot
        );
    }

    let mut grand_blocks = 0usize;
    let mut grand_trials = 0usize;
    let mut grand_linked = 0usize;
    let mut grand_mismatches = 0usize;
    let mut reports: Vec<BinaryVerifyReport> = Vec::new();
    let mut skipped: Vec<(String, String)> = Vec::new();

    for target in &targets {
        match verify_pe_file(target, &cfg) {
            Ok(rep) => {
                let crit = rep.total_critical_mismatches();
                grand_blocks += rep.blocks_tested;
                grand_trials += rep.trials_executed;
                grand_linked += rep.linked_traces_tested;
                grand_mismatches += crit;
                if !want_json {
                    let tag = if crit == 0 { "PASS" } else { "FAIL" };
                    println!(
                        "[{tag}] {:30} roots={:6} disc={:6} tested={:6} trials={:7} linked={:6} (links={} nop={} cond={}) | diff(fast={} jit={} aot={} link={} nan_pay={}) ({:.2}s)",
                        rep.module,
                        rep.roots,
                        rep.discovered_blocks,
                        rep.blocks_tested,
                        rep.trials_executed,
                        rep.linked_traces_tested,
                        rep.aot_links_patched,
                        rep.aot_fallthrough_nops,
                        rep.aot_cond_patched,
                        rep.fast_mismatches,
                        rep.jit_mismatches,
                        rep.aot_mismatches,
                        rep.linked_aot_mismatches,
                        rep.nan_payload_diffs,
                        rep.elapsed_ms as f64 / 1000.0
                    );
                    if !rep.unhandled_mnemonics.is_empty() {
                        let top: Vec<String> = rep
                            .unhandled_mnemonics
                            .iter()
                            .take(8)
                            .map(|(k, v)| format!("{k}:{v}"))
                            .collect();
                        println!("       unhandled generic mnemonics: {}", top.join(", "));
                    }
                    for m in &rep.mismatches {
                        println!(
                            "  [MISMATCH][{:10}] {}+0x{:x} trial={} kind={} {} | {}",
                            m.engine, m.module, m.rva, m.trial, m.kind, m.detail, m.disasm
                        );
                    }
                }
                reports.push(rep);
            }
            Err(e) => {
                if !want_json {
                    println!("[SKIP] {}: {e}", target.display());
                }
                skipped.push((target.display().to_string(), e));
            }
        }
    }

    let exit_code = if grand_mismatches > 0 {
        EXIT_VERIFICATION_FAILED
    } else {
        EXIT_OK
    };

    if want_json {
        let mut out = String::new();
        let ok = exit_code == EXIT_OK;
        let _ = writeln!(out, "{{");
        let _ = writeln!(out, "  \"schema_version\": 1,");
        let _ = writeln!(out, "  \"command\": \"verify-x86\",");
        let _ = writeln!(out, "  \"ok\": {ok},");
        let _ = writeln!(out, "  \"exit_code\": {exit_code},");
        let _ = writeln!(out, "  \"data\": {{");
        let _ = writeln!(out, "    \"binaries_verified\": {},", reports.len());
        let _ = writeln!(out, "    \"blocks_tested\": {grand_blocks},");
        let _ = writeln!(out, "    \"trials_executed\": {grand_trials},");
        let _ = writeln!(out, "    \"linked_traces_tested\": {grand_linked},");
        let _ = writeln!(out, "    \"critical_mismatches\": {grand_mismatches},");
        let _ = writeln!(out, "    \"config\": {{");
        let _ = writeln!(out, "      \"trials\": {},", cfg.trials);
        let _ = writeln!(out, "      \"fp_only\": {},", cfg.fp_only);
        let _ = writeln!(out, "      \"check_fast\": {},", cfg.check_fast);
        let _ = writeln!(out, "      \"check_jit\": {},", cfg.check_jit);
        let _ = writeln!(out, "      \"check_aot\": {},", cfg.check_aot);
        let _ = writeln!(out, "      \"check_linked_aot\": {},", cfg.check_linked_aot);
        let _ = writeln!(
            out,
            "      \"strict_nan_payload\": {}",
            cfg.strict_nan_payload
        );
        let _ = writeln!(out, "    }},");
        let _ = writeln!(out, "    \"binaries\": [");
        for (bi, rep) in reports.iter().enumerate() {
            let crit = rep.total_critical_mismatches();
            let comma = if bi + 1 < reports.len() { "," } else { "" };
            let _ = writeln!(out, "      {{");
            let _ = writeln!(out, "        \"module\": {},", json_escape(&rep.module));
            let _ = writeln!(out, "        \"passed\": {},", crit == 0);
            let _ = writeln!(out, "        \"roots\": {},", rep.roots);
            let _ = writeln!(
                out,
                "        \"discovered_blocks\": {},",
                rep.discovered_blocks
            );
            let _ = writeln!(
                out,
                "        \"translatable_blocks\": {},",
                rep.translatable_blocks
            );
            let _ = writeln!(out, "        \"blocks_tested\": {},", rep.blocks_tested);
            let _ = writeln!(out, "        \"trials_executed\": {},", rep.trials_executed);
            let _ = writeln!(
                out,
                "        \"linked_traces_tested\": {},",
                rep.linked_traces_tested
            );
            let _ = writeln!(
                out,
                "        \"aot_links_patched\": {},",
                rep.aot_links_patched
            );
            let _ = writeln!(
                out,
                "        \"aot_fallthrough_nops\": {},",
                rep.aot_fallthrough_nops
            );
            let _ = writeln!(
                out,
                "        \"aot_cond_patched\": {},",
                rep.aot_cond_patched
            );
            let _ = writeln!(out, "        \"fast_mismatches\": {},", rep.fast_mismatches);
            let _ = writeln!(out, "        \"jit_mismatches\": {},", rep.jit_mismatches);
            let _ = writeln!(out, "        \"aot_mismatches\": {},", rep.aot_mismatches);
            let _ = writeln!(
                out,
                "        \"linked_aot_mismatches\": {},",
                rep.linked_aot_mismatches
            );
            let _ = writeln!(out, "        \"critical_mismatches\": {crit},");
            let _ = writeln!(
                out,
                "        \"nan_payload_diffs\": {},",
                rep.nan_payload_diffs
            );
            let _ = writeln!(out, "        \"oracle_exported\": {},", rep.oracle_exported);
            let _ = writeln!(out, "        \"elapsed_ms\": {},", rep.elapsed_ms);
            let _ = write!(out, "        \"mismatches\": [");
            for (mi, m) in rep.mismatches.iter().enumerate() {
                if mi > 0 {
                    out.push(',');
                }
                let _ = write!(
                    out,
                    "{{\"module\":{},\"rva\":\"0x{:x}\",\"engine\":{},\"trial\":{},\"kind\":{},\"detail\":{},\"disasm\":{}}}",
                    json_escape(&m.module),
                    m.rva,
                    json_escape(m.engine),
                    m.trial,
                    json_escape(m.kind),
                    json_escape(&m.detail),
                    json_escape(&m.disasm)
                );
            }
            let _ = writeln!(out, "]");
            let _ = writeln!(out, "      }}{comma}");
        }
        let _ = writeln!(out, "    ],");
        let _ = write!(out, "    \"skipped\": [");
        for (si, (path, reason)) in skipped.iter().enumerate() {
            if si > 0 {
                out.push(',');
            }
            let _ = write!(
                out,
                "{{\"path\":{},\"reason\":{}}}",
                json_escape(path),
                json_escape(reason)
            );
        }
        let _ = writeln!(out, "]");
        let _ = writeln!(out, "  }},");
        if grand_mismatches > 0 {
            let err = format!("{grand_mismatches} critical differential mismatch(es) detected");
            let _ = writeln!(out, "  \"error\": {}", json_escape(&err));
        } else {
            let _ = writeln!(out, "  \"error\": null");
        }
        let _ = writeln!(out, "}}");
        print!("{out}");
        return ExitCode::from(exit_code);
    }

    println!(
        "\nSummary: {} blocks tested, {} trials, {} linked CFG traces, {} critical mismatches.",
        grand_blocks, grand_trials, grand_linked, grand_mismatches
    );
    ExitCode::from(exit_code)
}

#[cfg(not(target_arch = "aarch64"))]
fn main() -> std::process::ExitCode {
    eprintln!("x86-verify requires aarch64-apple-darwin");
    std::process::ExitCode::from(2)
}
