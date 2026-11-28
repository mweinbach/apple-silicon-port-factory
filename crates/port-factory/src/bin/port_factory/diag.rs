//! Diagnostic & verification toolkit CLI subcommands (`runtime-audit`, `verify-x86`,
//! `oracle`, `bisect-runtime`, `live-probe`, `boot-triage`, `nan-trace`).

use super::{emit_json, usage_err, workspace_root, ArgCursor};
use anyhow::{Context, Result};
use port_factory::boot_triage::{
    boot_triage_envelope, classify_boot_log, format_human_summary, run_boot_triage,
    split_arg_string, BootTriageInput, BootTriageOptions, BootTriageTarget,
};
use port_factory::bundle_common::{find_or_build_host_port_runner, TempDirGuard};
use port_factory::diag_toolkit::{
    audit_pe_runtime_hazards, find_or_build_x86_verify, format_hazard_reports,
    resolve_debug_script, NanTraceSpec,
};
use port_factory::json_output::{
    exit_codes, parse_json, runtime_audit_data_json, JsonEnvelope, JsonObj, JsonValue,
};
use port_factory::scanner::collect_pe_files;
use std::path::PathBuf;

pub fn cmd_runtime_audit(args: &[String]) -> Result<i32> {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        return Err(usage_err(
            "usage: port-factory runtime-audit <PE_FILE_OR_DIR...> [--only SUBSTR] [--json]",
        ));
    }
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut only_filter: Option<String> = None;
    let mut json = false;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--only" => {
                only_filter = Some(
                    cur.next_or_err("--only requires a substring")?
                        .to_ascii_lowercase(),
                );
            }
            "--json" => json = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        return Err(usage_err(
            "runtime-audit requires at least one PE file or directory",
        ));
    }
    let mut files = Vec::new();
    for p in &paths {
        files.extend(collect_pe_files(p)?);
    }
    if let Some(ref sub) = only_filter {
        files.retain(|p| {
            p.file_name()
                .map(|n| n.to_string_lossy().to_ascii_lowercase().contains(sub.as_str()))
                .unwrap_or(false)
        });
    }
    if files.is_empty() {
        return Err(usage_err(format!(
            "no valid x86-64 PE32+ binaries (.exe/.dll/.pyd/.node/.ocx) found in {paths:?}"
        )));
    }
    let mut reports = Vec::new();
    let mut skipped = Vec::new();
    for f in &files {
        match audit_pe_runtime_hazards(f) {
            Ok(r) => reports.push(r),
            Err(e) => {
                if !json {
                    eprintln!("[SKIP] {}: {e}", f.display());
                }
                skipped.push((f.display().to_string(), format!("{e:#}")));
            }
        }
    }
    if json {
        let data = runtime_audit_data_json(&reports, &skipped);
        return Ok(emit_json(&JsonEnvelope::ok("runtime-audit", data)));
    }
    print!("{}", format_hazard_reports(&reports));
    Ok(exit_codes::OK)
}

pub fn cmd_verify_x86(args: &[String]) -> Result<i32> {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!(
            "usage: port-factory verify-x86 <PE_FILE_OR_DIR...> [options]\n\
             Toolkit options:\n\
             \x20 --json                Emit stable schema v1 JSON envelope on stdout\n\
             \x20 --oracle              Export FP/SIMD samples and verify on Rosetta 2 x86-64 hardware\n\
             \x20 --oracle-dir DIR      Directory for Rosetta 2 oracle snapshots\n\
             \x20 --oracle-sample N     Samples per binary (default 32 when --oracle is set)\n\
             \x20 --rebuild             Force release rebuild of x86-verify before running\n\
             \x20 (All other flags pass straight through to x86-verify: --trials, --max-blocks,\n\
             \x20  --rva LO..HI, --fp-only, --only SUBSTR, --fast-nan, --strict-nan-payload,\n\
             \x20  --no-fast, --no-jit, --no-aot, --no-linked, --threads N)"
        );
        return Ok(exit_codes::OK);
    }
    let root = workspace_root();
    let mut run_oracle = false;
    let mut force_build = false;
    let mut json = false;
    let mut oracle_dir: Option<PathBuf> = None;
    let mut oracle_sample: Option<usize> = None;
    let mut pass_args: Vec<String> = Vec::new();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--oracle" => run_oracle = true,
            "--rebuild" => force_build = true,
            "--json" => json = true,
            "--oracle-dir" => {
                oracle_dir = Some(cur.next_path_or_err("--oracle-dir requires DIR")?);
            }
            "--oracle-sample" => {
                let raw = cur.next_or_err("--oracle-sample requires N")?;
                oracle_sample = Some(
                    raw.parse()
                        .map_err(|_| usage_err(format!("invalid --oracle-sample {raw:?}")))?,
                );
            }
            other => pass_args.push(other.to_string()),
        }
    }

    let auto_oracle_guard = if run_oracle && oracle_dir.is_none() {
        let g = TempDirGuard::create_unique("port_factory_oracle")?;
        oracle_dir = Some(g.path().to_path_buf());
        Some(g)
    } else {
        None
    };

    if let Some(ref od) = oracle_dir {
        pass_args.push("--oracle-dir".to_string());
        pass_args.push(od.to_string_lossy().into_owned());
        let n = oracle_sample.unwrap_or(if run_oracle { 32 } else { 0 });
        if n > 0 {
            pass_args.push("--oracle-sample".to_string());
            pass_args.push(n.to_string());
        }
    }
    if json {
        pass_args.push("--json".to_string());
    }

    let bin = find_or_build_x86_verify(&root, force_build)?;

    if json && run_oracle {
        let out = std::process::Command::new(&bin)
            .args(&pass_args)
            .output()
            .with_context(|| format!("running {}", bin.display()))?;
        let verify_code = out.status.code().unwrap_or(exit_codes::RUNTIME_ERROR);
        let verify_stdout = String::from_utf8_lossy(&out.stdout);
        let verify_val = parse_json(verify_stdout.trim()).unwrap_or_else(|_| {
            JsonObj::new()
                .str("raw_stdout", verify_stdout.trim().to_string())
                .build()
        });
        let verify_data = verify_val
            .get("data")
            .cloned()
            .unwrap_or_else(|| verify_val.clone());

        if !out.status.success() {
            drop(auto_oracle_guard);
            print!("{verify_stdout}");
            return Ok(verify_code);
        }

        let mut oracle_val = JsonValue::Null;
        let mut oracle_code = exit_codes::OK;
        if let Some(ref od) = oracle_dir {
            let script = resolve_debug_script(&root, "x86_oracle.py")?;
            let max_n = oracle_sample.unwrap_or(32).saturating_mul(16).max(256);
            let oout = std::process::Command::new("python3")
                .arg(&script)
                .arg(od)
                .arg("--max")
                .arg(max_n.to_string())
                .arg("--quiet-matches")
                .arg("--json")
                .output()
                .with_context(|| format!("running {}", script.display()))?;
            oracle_code = oout.status.code().unwrap_or(exit_codes::RUNTIME_ERROR);
            let ostdout = String::from_utf8_lossy(&oout.stdout);
            oracle_val = parse_json(ostdout.trim())
                .ok()
                .and_then(|v| v.get("data").cloned())
                .unwrap_or_else(|| {
                    JsonObj::new()
                        .str("raw_stdout", ostdout.trim().to_string())
                        .build()
                });
        }
        drop(auto_oracle_guard);

        let merged = match verify_data {
            JsonValue::Object(mut entries) => {
                entries.push(("oracle".to_string(), oracle_val));
                JsonValue::Object(entries)
            }
            other => JsonObj::new()
                .val("verify", other)
                .val("oracle", oracle_val)
                .build(),
        };
        let final_code = if oracle_code != exit_codes::OK {
            oracle_code
        } else {
            verify_code
        };
        let err_msg = (final_code != exit_codes::OK)
            .then(|| format!("verify-x86 exited with code {final_code}"));
        return Ok(emit_json(&JsonEnvelope::with_status(
            "verify-x86",
            final_code,
            merged,
            err_msg,
        )));
    }

    let status = std::process::Command::new(&bin)
        .args(&pass_args)
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    if !status.success() {
        drop(auto_oracle_guard);
        return Ok(status.code().unwrap_or(exit_codes::RUNTIME_ERROR));
    }

    if run_oracle {
        if let Some(ref od) = oracle_dir {
            println!("\n=== Phase 2: Rosetta 2 x86-64 Hardware Oracle ===");
            let script = resolve_debug_script(&root, "x86_oracle.py")?;
            let max_n = oracle_sample.unwrap_or(32).saturating_mul(16).max(256);
            let ostatus = std::process::Command::new("python3")
                .arg(&script)
                .arg(od)
                .arg("--max")
                .arg(max_n.to_string())
                .arg("--quiet-matches")
                .status()
                .with_context(|| format!("running {}", script.display()))?;
            drop(auto_oracle_guard);
            if !ostatus.success() {
                return Ok(ostatus.code().unwrap_or(exit_codes::RUNTIME_ERROR));
            }
        }
    }
    Ok(exit_codes::OK)
}

fn run_debug_python(script_name: &str, args: &[String]) -> Result<i32> {
    let root = workspace_root();
    let script = resolve_debug_script(&root, script_name)?;
    let status = std::process::Command::new("python3")
        .arg(&script)
        .args(args)
        .status()
        .with_context(|| format!("running {}", script.display()))?;
    Ok(status.code().unwrap_or(exit_codes::RUNTIME_ERROR))
}

pub fn cmd_oracle(args: &[String]) -> Result<i32> {
    run_debug_python("x86_oracle.py", args)
}

pub fn cmd_bisect_runtime(args: &[String]) -> Result<i32> {
    run_debug_python("bisect_runtime.py", args)
}

pub fn cmd_live_probe(args: &[String]) -> Result<i32> {
    run_debug_python("live_probe.py", args)
}

pub fn cmd_nan_trace(args: &[String]) -> Result<i32> {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!(
            "usage: port-factory nan-trace <PE_OR_MANIFEST_TOML> [options] [-- <game-args...>]\n\
             Options:\n\
             \x20 --game-dir DIR        Game install directory\n\
             \x20 --module MOD          Filter PORT_NAN_TRACE to module name (repeatable)\n\
             \x20 --range SPEC          Filter PORT_NAN_TRACE to VA/RVA range (repeatable)\n\
             \x20 --watch LO..HI        Watch memory writes overlapping guest range [LO, HI)\n\
             \x20 --host SUBSTR         Log host calls matching symbol substring\n\
             \x20 --dump-dir DIR        Dump Rosetta 2 oracle snap_*.json files to DIR\n\
             \x20 --log FILE            Write JSONL NaN trace events to FILE\n\
             \x20 --stores              Also trace FP NaN memory stores\n\
             \x20 --max N               Stop recording after N trace events\n\
             \x20 --armed               Start tracing immediately at boot (default: armed=0; arm via PORT_CONTROL_DIR `nantrace on`)\n\
             \x20 --no-boundary         Disable PORT_NAN_BOUNDARY=1 middleware boundary logging\n\
             \x20 --control-dir DIR     Enable PORT_CONTROL_DIR=DIR\n\
             \x20 --run-oracle          Automatically run Rosetta 2 x86_oracle.py on --dump-dir after exit"
        );
        return Ok(exit_codes::OK);
    }
    let root = workspace_root();
    let mut target: Option<PathBuf> = None;
    let mut game_dir: Option<PathBuf> = None;
    let mut control_dir: Option<PathBuf> = None;
    let mut boundary = true;
    let mut run_oracle = false;
    let mut spec = NanTraceSpec::default();
    let mut game_args: Vec<String> = Vec::new();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--" => {
                game_args.extend(cur.take_rest().iter().cloned());
                break;
            }
            "--game-dir" => {
                game_dir = Some(cur.next_path_or_err("--game-dir requires DIR")?);
            }
            "--module" => {
                spec.modules
                    .push(cur.next_or_err("--module requires MOD")?.to_string());
            }
            "--range" => {
                spec.ranges
                    .push(cur.next_or_err("--range requires SPEC")?.to_string());
            }
            "--watch" => {
                spec.watch_mem = Some(cur.next_or_err("--watch requires LO..HI")?.to_string());
            }
            "--host" => {
                spec.host_filter = Some(cur.next_or_err("--host requires SUBSTR")?.to_string());
            }
            "--dump-dir" => {
                spec.dump_dir = Some(cur.next_path_or_err("--dump-dir requires DIR")?);
            }
            "--log" => {
                spec.log_file = Some(cur.next_path_or_err("--log requires FILE")?);
            }
            "--stores" => spec.trace_stores = true,
            "--max" => {
                let raw = cur.next_or_err("--max requires N")?;
                spec.max_events = Some(
                    raw.parse()
                        .map_err(|_| usage_err(format!("invalid --max {raw:?}")))?,
                );
            }
            "--armed" => spec.start_armed = true,
            "--no-boundary" => boundary = false,
            "--control-dir" => {
                control_dir = Some(cur.next_path_or_err("--control-dir requires DIR")?);
            }
            "--run-oracle" => run_oracle = true,
            s if s.starts_with('-') => return Err(usage_err(format!("unknown flag {s:?}"))),
            p if target.is_none() => target = Some(PathBuf::from(p)),
            other => return Err(usage_err(format!("unexpected argument {other:?}"))),
        }
    }

    let Some(target) = target else {
        return Err(usage_err("nan-trace requires <PE_OR_MANIFEST_TOML>"));
    };
    if let Some(ref dd) = spec.dump_dir {
        std::fs::create_dir_all(dd)?;
    }
    let trace_env = spec.to_env_string();
    let runner = find_or_build_host_port_runner(&root)?;
    let mut cmd = std::process::Command::new(&runner);
    let is_toml = target
        .extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("toml"));
    if is_toml {
        cmd.arg("--manifest").arg(&target);
    } else {
        cmd.arg("--pe").arg(&target);
    }
    if let Some(ref gd) = game_dir {
        cmd.arg("--game-dir").arg(gd);
    }
    if !game_args.is_empty() {
        cmd.arg("--").args(&game_args);
    }
    cmd.env("PORT_NAN_TRACE", &trace_env);
    if boundary {
        cmd.env("PORT_NAN_BOUNDARY", "1");
    }
    if let Some(ref cd) = control_dir {
        std::fs::create_dir_all(cd)?;
        cmd.env("PORT_CONTROL_DIR", cd);
    }
    eprintln!("port-factory nan-trace: PORT_NAN_TRACE={trace_env}");
    let status = cmd
        .status()
        .with_context(|| format!("running {}", runner.display()))?;

    if let Some(ref dd) = spec.dump_dir {
        if run_oracle {
            eprintln!(
                "\n=== Running Rosetta 2 x86-64 oracle on {} ===",
                dd.display()
            );
            let ocode = run_debug_python(
                "x86_oracle.py",
                &[dd.to_string_lossy().into_owned(), "--quiet-matches".into()],
            )?;
            if ocode != exit_codes::OK {
                return Ok(ocode);
            }
        } else {
            eprintln!(
                "Tip: replay captured snapshots on Rosetta 2 via: cargo opsdb -- oracle {}",
                dd.display()
            );
        }
    }
    Ok(status.code().unwrap_or(exit_codes::RUNTIME_ERROR))
}

fn print_boot_triage_help() {
    eprintln!(
        "usage: port-factory boot-triage [options] [-- <game-args...>]\n\
         \n\
         Runs port-runner headless, captures the runtime log + boot/crash reports,\n\
         classifies the first blocker, emits Schema v1 JSON on stdout, and prints a\n\
         short human summary pointing at the likely owning crate.\n\
         \n\
         Target options (one of --manifest, --pe, --exe, or --log-file is required):\n\
         \x20 --manifest FILE       Declarative games/<id>/game.toml manifest\n\
         \x20 --game-dir DIR        Game install directory\n\
         \x20 --pe FILE             Direct path to a Windows x86-64 .exe\n\
         \x20 --exe NAME            Executable filename inside --game-dir\n\
         \x20 --args STR            Guest CLI argument(s) (repeatable; quotes supported, e.g. --args -nolauncher)\n\
         \x20 --timeout SECS        Wall-clock timeout in seconds (default: 30; alias: --timeout-secs)\n\
         \x20 --max-frames N        Stop cleanly after N presented frames (default: 5; 0 disables)\n\
         \x20 --runner PATH         Explicit path to port-runner binary\n\
         \x20 --aot-dir DIR         Override AOT cache directory\n\
         \x20 --drive-c DIR         Override Windows C:\\ prefix directory\n\
         \x20 --env KEY=VAL         Extra env var passed to port-runner (repeatable)\n\
         \x20 --metal-validation    Enable Apple Metal debug layer (MTL_DEBUG_LAYER=1)\n\
         \n\
         Offline log-triage mode (skip running port-runner):\n\
         \x20 --log-file FILE       Classify an already-captured log file\n\
         \x20 --boot-report FILE    Optional port-runner --report-json sidecar for --log-file\n\
         \x20 --crash-json FILE     Optional crash-*.json sidecar for --log-file\n\
         \x20 --exit-code N         Optional exit code for --log-file\n\
         \x20 --signal N            Optional POSIX termination signal for --log-file\n\
         \x20 --timed-out           Mark --log-file run as timed out\n\
         \n\
         Output options:\n\
         \x20 --json                Emit only the Schema v1 JSON envelope on stdout (quiet stderr)\n\
         \x20 --summary-only        Print only the short human summary on stdout\n\
         \x20 --json-out FILE       Also write the Schema v1 JSON envelope to FILE\n\
         \x20 --log-out FILE        Save the full captured log to FILE"
    );
}

pub fn cmd_boot_triage(args: &[String]) -> Result<i32> {
    if args.is_empty() {
        print_boot_triage_help();
        return Ok(exit_codes::OK);
    }

    let mut opts = BootTriageOptions {
        workspace_root: workspace_root(),
        ..BootTriageOptions::default()
    };
    let mut json_only = false;
    let mut summary_only = false;
    let mut json_out: Option<PathBuf> = None;

    // Offline log-triage options
    let mut log_file: Option<PathBuf> = None;
    let mut boot_report_file: Option<PathBuf> = None;
    let mut crash_json_file: Option<PathBuf> = None;
    let mut offline_exit_code: Option<i32> = None;
    let mut offline_signal: Option<i32> = None;
    let mut offline_timed_out = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        if let Some(val) = a.strip_prefix("--args=") {
            opts.game_args.extend(split_arg_string(val));
            continue;
        }
        match a {
            "-h" | "--help" => {
                print_boot_triage_help();
                return Ok(exit_codes::OK);
            }
            "--" => {
                opts.game_args.extend(cur.take_rest().iter().cloned());
                break;
            }
            "--manifest" => {
                opts.manifest = Some(cur.next_path_or_err("--manifest requires FILE")?);
            }
            "--game-dir" => {
                opts.game_dir = Some(cur.next_path_or_err("--game-dir requires DIR")?);
            }
            "--pe" => {
                opts.pe = Some(cur.next_path_or_err("--pe requires FILE")?);
            }
            "--exe" => {
                opts.exe = Some(cur.next_or_err("--exe requires NAME")?.to_string());
            }
            "--args" => {
                let raw = cur.next_or_err("--args requires a value")?;
                opts.game_args.extend(split_arg_string(raw));
            }
            "--timeout" | "--timeout-secs" => {
                let raw = cur.next_or_err("--timeout requires SECS")?;
                let secs: f64 = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --timeout value {raw:?}")))?;
                if !secs.is_finite() || secs <= 0.0 {
                    return Err(usage_err(format!(
                        "--timeout must be a positive finite number, got {raw:?}"
                    )));
                }
                opts.timeout_secs = secs;
            }
            "--max-frames" => {
                let raw = cur.next_or_err("--max-frames requires N")?;
                let n: u64 = raw
                    .parse()
                    .map_err(|_| usage_err(format!("invalid --max-frames value {raw:?}")))?;
                opts.max_frames = (n > 0).then_some(n);
            }
            "--runner" => {
                opts.runner = Some(cur.next_path_or_err("--runner requires PATH")?);
            }
            "--aot-dir" => {
                opts.aot_dir = Some(cur.next_path_or_err("--aot-dir requires DIR")?);
            }
            "--drive-c" => {
                opts.drive_c = Some(cur.next_path_or_err("--drive-c requires DIR")?);
            }
            "--env" => {
                let raw = cur.next_or_err("--env requires KEY=VAL")?;
                let Some((k, v)) = raw.split_once('=') else {
                    return Err(usage_err(format!("--env expects KEY=VAL, got {raw:?}")));
                };
                if k.trim().is_empty() {
                    return Err(usage_err(format!("--env key cannot be empty: {raw:?}")));
                }
                opts.extra_env.push((k.trim().to_string(), v.to_string()));
            }
            "--metal-validation" => opts.metal_validation = true,
            "--log-out" => {
                opts.log_out = Some(cur.next_path_or_err("--log-out requires FILE")?);
            }
            "--json-out" => {
                json_out = Some(cur.next_path_or_err("--json-out requires FILE")?);
            }
            "--json" => json_only = true,
            "--summary-only" => summary_only = true,
            "--log-file" => {
                log_file = Some(cur.next_path_or_err("--log-file requires FILE")?);
            }
            "--boot-report" => {
                boot_report_file = Some(cur.next_path_or_err("--boot-report requires FILE")?);
            }
            "--crash-json" => {
                crash_json_file = Some(cur.next_path_or_err("--crash-json requires FILE")?);
            }
            "--exit-code" => {
                let raw = cur.next_or_err("--exit-code requires N")?;
                offline_exit_code = Some(
                    raw.parse()
                        .map_err(|_| usage_err(format!("invalid --exit-code {raw:?}")))?,
                );
            }
            "--signal" => {
                let raw = cur.next_or_err("--signal requires N")?;
                offline_signal = Some(
                    raw.parse()
                        .map_err(|_| usage_err(format!("invalid --signal {raw:?}")))?,
                );
            }
            "--timed-out" => offline_timed_out = true,
            other => return Err(usage_err(format!("unknown boot-triage argument {other:?}"))),
        }
    }

    if json_only && summary_only {
        return Err(usage_err("--json and --summary-only cannot be combined"));
    }

    let report = if let Some(ref lf) = log_file {
        if !lf.is_file() {
            return Err(usage_err(format!("log file not found: {}", lf.display())));
        }
        let log_text = std::fs::read_to_string(lf)
            .with_context(|| format!("reading log file {}", lf.display()))?;
        let boot_report_json = match boot_report_file {
            Some(ref p) => Some(
                std::fs::read_to_string(p)
                    .with_context(|| format!("reading boot report {}", p.display()))?,
            ),
            None => None,
        };
        let crash_report_json = match crash_json_file {
            Some(ref p) => Some(
                std::fs::read_to_string(p)
                    .with_context(|| format!("reading crash json {}", p.display()))?,
            ),
            None => None,
        };
        let input = BootTriageInput {
            target: BootTriageTarget {
                manifest: opts.manifest.as_ref().map(|p| p.display().to_string()),
                game_dir: opts.game_dir.as_ref().map(|p| p.display().to_string()),
                pe: opts.pe.as_ref().map(|p| p.display().to_string()),
                exe: opts.exe.clone(),
                args: opts.game_args.clone(),
                timeout_secs: Some(opts.timeout_secs),
                max_frames: opts.max_frames,
            },
            mode: "offline".to_string(),
            log_text,
            boot_report_json,
            crash_report_json,
            exit_code: offline_exit_code,
            signal: offline_signal,
            timed_out: offline_timed_out,
            elapsed_ms: 0,
            log_path: Some(lf.display().to_string()),
        };
        classify_boot_log(&input)
    } else {
        if opts.manifest.is_none() && opts.pe.is_none() && opts.exe.is_none() {
            return Err(usage_err(
                "boot-triage requires --manifest <TOML>, --pe <EXE>, --exe <NAME>, or --log-file <FILE>",
            ));
        }
        if let Some(ref m) = opts.manifest {
            if !m.is_file() {
                return Err(usage_err(format!("manifest not found: {}", m.display())));
            }
        }
        if let Some(ref gd) = opts.game_dir {
            if !gd.is_dir() {
                return Err(usage_err(format!(
                    "game directory not found: {}",
                    gd.display()
                )));
            }
        }
        if let Some(ref pe) = opts.pe {
            if !pe.is_file() {
                return Err(usage_err(format!("PE file not found: {}", pe.display())));
            }
        }
        run_boot_triage(&opts)?
    };

    let env = boot_triage_envelope(&report);
    let json_pretty = env.to_json_pretty();
    let human = format_human_summary(&report);

    if let Some(ref out_p) = json_out {
        if let Some(parent) = out_p.parent().filter(|p| !p.as_os_str().is_empty()) {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(out_p, format!("{json_pretty}\n"))
            .with_context(|| format!("writing JSON report to {}", out_p.display()))?;
    }

    if summary_only {
        print!("{human}");
    } else if json_only {
        println!("{json_pretty}");
    } else {
        println!("{json_pretty}");
        let _ = std::io::Write::flush(&mut std::io::stdout());
        eprint!("\n{human}");
    }

    Ok(env.exit_code)
}
