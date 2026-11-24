//! CLI handler for `port-factory bench` (`cargo opsdb -- bench`).

use super::{usage_err, workspace_root, ArgCursor};
use anyhow::{bail, Context, Result};
use port_factory::bench::{
    format_report_json, format_summary_table, run_benchmarks, BenchMode, BenchOptions,
    DEFAULT_BASELINE_PATH, DEFAULT_OUT_PATH, DEFAULT_TOLERANCE, WORKLOADS,
};
use port_factory::json_output::exit_codes;
use std::path::PathBuf;

pub fn cmd_bench(args: &[String]) -> Result<i32> {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        let names = WORKLOADS
            .iter()
            .map(|w| w.id)
            .collect::<Vec<_>>()
            .join(", ");
        eprintln!(
            "usage: port-factory bench [options]\n\
             \n\
             Compiles and runs self-checking Windows x86-64 guest workloads through port-runner,\n\
             writes structured JSON results, and compares against a chip-keyed baseline.\n\
             \n\
             Workloads: {names}\n\
             \n\
             Options:\n\
               --mode aot|jit          Execution mode (default: aot)\n\
               --only NAME[,NAME...]   Run only matching workload(s)\n\
               --tolerance FRAC        Max allowed slowdown fraction (default: {DEFAULT_TOLERANCE})\n\
               --baseline PATH         Baseline JSON path (default: {DEFAULT_BASELINE_PATH})\n\
               --out PATH              Output JSON report path (default: {DEFAULT_OUT_PATH})\n\
               --no-out                Skip writing output JSON file\n\
               --runner PATH           Override port-runner binary path\n\
               --update-baseline       Record current chip's results into the baseline JSON\n\
               --no-compare            Do not fail if current chip has no baseline yet\n\
               --json                  Print full JSON report to stdout\n"
        );
        return Ok(exit_codes::OK);
    }

    let root = workspace_root();
    let mut mode = BenchMode::Aot;
    let mut only: Vec<String> = Vec::new();
    let mut tolerance = DEFAULT_TOLERANCE;
    let mut baseline_path = root.join(DEFAULT_BASELINE_PATH);
    let mut out_path: Option<PathBuf> = Some(root.join(DEFAULT_OUT_PATH));
    let mut runner_override: Option<PathBuf> = None;
    let mut update_baseline = false;
    let mut no_compare = false;
    let mut print_json = false;

    let mut cur = ArgCursor::new(args);
    while let Some(a) = cur.next() {
        match a {
            "--mode" => {
                let raw = cur.next_or_err("--mode requires aot or jit")?;
                mode = BenchMode::parse(raw)?;
            }
            "--only" => {
                let raw = cur.next_or_err("--only requires workload name(s)")?;
                for part in raw.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                    only.push(part.to_string());
                }
            }
            "--tolerance" => {
                let raw = cur.next_or_err("--tolerance requires a float (e.g. 0.25)")?;
                tolerance = raw
                    .parse::<f64>()
                    .with_context(|| format!("invalid --tolerance {raw:?}"))?;
                if !tolerance.is_finite() || tolerance < 0.0 {
                    bail!("--tolerance must be a finite non-negative number (got {tolerance})");
                }
            }
            "--baseline" => {
                baseline_path = PathBuf::from(cur.next_or_err("--baseline requires PATH")?);
            }
            "--out" => {
                out_path = Some(PathBuf::from(cur.next_or_err("--out requires PATH")?));
            }
            "--no-out" => out_path = None,
            "--runner" => {
                runner_override = Some(PathBuf::from(cur.next_or_err("--runner requires PATH")?));
            }
            "--update-baseline" => update_baseline = true,
            "--no-compare" => no_compare = true,
            "--json" => print_json = true,
            other => return Err(usage_err(format!("unknown bench argument {other:?}"))),
        }
    }

    let opts = BenchOptions {
        workspace_root: root,
        mode,
        only,
        tolerance,
        baseline_path,
        out_path: out_path.clone(),
        runner_override,
        update_baseline,
        no_compare,
        print_json,
    };

    let report = run_benchmarks(&opts)?;
    if print_json {
        print!("{}", format_report_json(&report));
        return Ok(if report.overall_pass {
            exit_codes::OK
        } else {
            exit_codes::VERIFICATION_FAILED
        });
    } else {
        print!("{}", format_summary_table(&report));
        if let Some(ref p) = out_path {
            println!("JSON report written to {}", p.display());
        }
        if update_baseline {
            println!("Updated baseline at {}", opts.baseline_path.display());
        }
    }

    if !report.overall_pass {
        bail!(
            "benchmark baseline check FAILED for chip {:?} (mode={}, tolerance=+{:.0}%)",
            report.chip,
            report.mode.as_str(),
            report.tolerance * 100.0
        );
    }
    Ok(exit_codes::OK)
}
