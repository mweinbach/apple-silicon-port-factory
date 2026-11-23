//! Diagnostic & verification toolkit CLI subcommands (`runtime-audit`, `verify-x86`,
//! `oracle`, `bisect-runtime`, `live-probe`, `nan-trace`).

use super::{workspace_root, ArgCursor};
use anyhow::{bail, Context, Result};
use port_factory::bundle_common::{find_or_build_host_port_runner, TempDirGuard};
use port_factory::diag_toolkit::{
    audit_pe_runtime_hazards, find_or_build_x86_verify, format_hazard_reports,
    resolve_debug_script, NanTraceSpec,
};
use port_factory::scanner::collect_pe_files;
use std::path::PathBuf;

pub fn cmd_runtime_audit(args: &[String]) -> Result<()> {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        bail!("usage: port-factory runtime-audit <PE_FILE_OR_DIR...> [--only SUBSTR]");
    }
    let mut paths: Vec<PathBuf> = Vec::new();
    let mut only_filter: Option<String> = None;
    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--only" => {
                only_filter = Some(
                    cur.next_or_err("--only requires a substring")?
                        .to_ascii_lowercase(),
                );
            }
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p => paths.push(PathBuf::from(p)),
        }
    }
    if paths.is_empty() {
        bail!("runtime-audit requires at least one PE file or directory");
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
        bail!("no PE32+ .exe/.dll files found in {:?}", paths);
    }
    let mut reports = Vec::new();
    for f in &files {
        match audit_pe_runtime_hazards(f) {
            Ok(r) => reports.push(r),
            Err(e) => eprintln!("[SKIP] {}: {e}", f.display()),
        }
    }
    print!("{}", format_hazard_reports(&reports));
    Ok(())
}

pub fn cmd_verify_x86(args: &[String]) -> Result<()> {
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        eprintln!(
            "usage: port-factory verify-x86 <PE_FILE_OR_DIR...> [options]\n\
             Toolkit options:\n\
             \x20 --oracle              Export FP/SIMD samples and verify on Rosetta 2 x86-64 hardware\n\
             \x20 --oracle-dir DIR      Directory for Rosetta 2 oracle snapshots\n\
             \x20 --oracle-sample N     Samples per binary (default 32 when --oracle is set)\n\
             \x20 --rebuild             Force release rebuild of x86-verify before running\n\
             \x20 (All other flags pass straight through to x86-verify: --trials, --max-blocks,\n\
             \x20  --rva LO..HI, --fp-only, --only SUBSTR, --fast-nan, --strict-nan-payload,\n\
             \x20  --no-fast, --no-jit, --no-aot, --no-linked, --threads N)"
        );
        return Ok(());
    }
    let root = workspace_root();
    let mut run_oracle = false;
    let mut force_build = false;
    let mut oracle_dir: Option<PathBuf> = None;
    let mut oracle_sample: Option<usize> = None;
    let mut pass_args: Vec<String> = Vec::new();

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--oracle" => run_oracle = true,
            "--rebuild" => force_build = true,
            "--oracle-dir" => {
                oracle_dir = Some(PathBuf::from(cur.next_or_err("--oracle-dir requires DIR")?));
            }
            "--oracle-sample" => {
                let raw = cur.next_or_err("--oracle-sample requires N")?;
                oracle_sample = Some(raw.parse().context("invalid --oracle-sample")?);
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

    let bin = find_or_build_x86_verify(&root, force_build)?;
    let status = std::process::Command::new(&bin)
        .args(&pass_args)
        .status()
        .with_context(|| format!("running {}", bin.display()))?;
    if !status.success() {
        drop(auto_oracle_guard);
        std::process::exit(status.code().unwrap_or(1));
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
                std::process::exit(ostatus.code().unwrap_or(1));
            }
        }
    }
    Ok(())
}

fn run_debug_python(script_name: &str, args: &[String]) -> Result<()> {
    let root = workspace_root();
    let script = resolve_debug_script(&root, script_name)?;
    let status = std::process::Command::new("python3")
        .arg(&script)
        .args(args)
        .status()
        .with_context(|| format!("running {}", script.display()))?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

pub fn cmd_oracle(args: &[String]) -> Result<()> {
    run_debug_python("x86_oracle.py", args)
}

pub fn cmd_bisect_runtime(args: &[String]) -> Result<()> {
    run_debug_python("bisect_runtime.py", args)
}

pub fn cmd_live_probe(args: &[String]) -> Result<()> {
    run_debug_python("live_probe.py", args)
}

pub fn cmd_nan_trace(args: &[String]) -> Result<()> {
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
        return Ok(());
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
                game_dir = Some(PathBuf::from(cur.next_or_err("--game-dir requires DIR")?));
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
                spec.dump_dir = Some(PathBuf::from(cur.next_or_err("--dump-dir requires DIR")?));
            }
            "--log" => {
                spec.log_file = Some(PathBuf::from(cur.next_or_err("--log requires FILE")?));
            }
            "--stores" => spec.trace_stores = true,
            "--max" => {
                let raw = cur.next_or_err("--max requires N")?;
                spec.max_events = Some(raw.parse().context("invalid --max")?);
            }
            "--armed" => spec.start_armed = true,
            "--no-boundary" => boundary = false,
            "--control-dir" => {
                control_dir = Some(PathBuf::from(
                    cur.next_or_err("--control-dir requires DIR")?,
                ));
            }
            "--run-oracle" => run_oracle = true,
            s if s.starts_with('-') => bail!("unknown flag {s:?}"),
            p if target.is_none() => target = Some(PathBuf::from(p)),
            other => bail!("unexpected argument {other:?}"),
        }
    }

    let target = target.context("nan-trace requires <PE_OR_MANIFEST_TOML>")?;
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
            run_debug_python(
                "x86_oracle.py",
                &[dd.to_string_lossy().into_owned(), "--quiet-matches".into()],
            )?;
        } else {
            eprintln!(
                "Tip: replay captured snapshots on Rosetta 2 via: cargo opsdb -- oracle {}",
                dd.display()
            );
        }
    }
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}
