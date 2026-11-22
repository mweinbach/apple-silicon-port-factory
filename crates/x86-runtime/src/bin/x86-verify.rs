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
fn main() -> std::process::ExitCode {
    use std::path::PathBuf;
    use std::process::ExitCode;
    use x86_runtime::emu::verify::{collect_pe_targets, verify_pe_file, VerifyConfig};

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
    // Default to exact IEEE-754 NaN propagation unless `--fast-nan` or `PORT_JIT_FAST_NAN` is set.
    let want_fast_nan = args.iter().any(|a| a == "--fast-nan");
    if want_fast_nan {
        std::env::set_var("PORT_JIT_FAST_NAN", "1");
    } else if std::env::var_os("PORT_JIT_FAST_NAN").is_none() {
        std::env::set_var("PORT_JIT_FAST_NAN", "0");
    }
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!(
            "Usage: x86-verify <exe|dll|game-dir> [...] [options]\n\
             Options:\n\
             \x20 --max-blocks N        Cap blocks verified per binary (default 0 = all)\n\
             \x20 --trials N            Adversarial input trials per block (default 3)\n\
             \x20 --rva LO..HI          Only verify blocks in RVA range [LO, HI)\n\
             \x20 --fp-only             Only verify blocks containing FP/SIMD instructions\n\
             \x20 --no-fast             Skip FastOp vs Generic check\n\
             \x20 --no-jit              Skip Live JIT vs Generic check\n\
             \x20 --no-aot              Skip Unlinked AOT vs Generic check\n\
             \x20 --no-linked           Skip Linked multi-block AOT check\n\
             \x20 --fast-nan            Test with PORT_JIT_FAST_NAN=1 (default: 0, exact NaN parity)\n\
             \x20 --strict-nan-payload  Treat NaN-payload-only diffs as critical mismatches\n\
             \x20 --threads N           Threads for discovery & AOT builds\n\
             \x20 --max-mismatches N    Max mismatch records printed per binary (default 64)\n\
             \x20 --oracle-dir DIR      Export Rosetta x86-64 oracle snapshots to DIR\n\
             \x20 --oracle-sample N     Export up to N passing FP/SIMD instructions per binary\n\
             \x20 --only SUBSTR         Only scan binaries whose filename contains SUBSTR (case-insensitive)"
        );
        return ExitCode::from(2);
    }

    let mut cfg = VerifyConfig::default();
    let mut inputs: Vec<PathBuf> = Vec::new();
    let mut only_filter: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
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
                eprintln!("Unknown option: {other}");
                return ExitCode::from(2);
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
                eprintln!("error: {e}");
                return ExitCode::from(2);
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
        eprintln!("No PE32+ .exe/.dll targets found.");
        return ExitCode::from(2);
    }

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

    let mut grand_blocks = 0usize;
    let mut grand_trials = 0usize;
    let mut grand_linked = 0usize;
    let mut grand_mismatches = 0usize;

    for target in &targets {
        match verify_pe_file(target, &cfg) {
            Ok(rep) => {
                let crit = rep.total_critical_mismatches();
                grand_blocks += rep.blocks_tested;
                grand_trials += rep.trials_executed;
                grand_linked += rep.linked_traces_tested;
                grand_mismatches += crit;
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
            Err(e) => {
                println!("[SKIP] {}: {e}", target.display());
            }
        }
    }

    println!(
        "\nSummary: {} blocks tested, {} trials, {} linked CFG traces, {} critical mismatches.",
        grand_blocks, grand_trials, grand_linked, grand_mismatches
    );
    if grand_mismatches > 0 {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

#[cfg(not(target_arch = "aarch64"))]
fn main() -> std::process::ExitCode {
    eprintln!("x86-verify requires aarch64-apple-darwin");
    std::process::ExitCode::from(2)
}
