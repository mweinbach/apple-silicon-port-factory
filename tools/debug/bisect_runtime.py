#!/usr/bin/env python3
"""Universal automated execution-mode matrix & PORT_JIT_SKIP binary-search bisector.

Scales to ANY Windows game and ANY reproducible runtime/visual/NaN bug:
  1. Runs a caller-provided reproducer command across key `x86-runtime` execution
     modes (default JIT, `AW2_JIT=0`, `PORT_JIT_FAST_NAN=0`, `PORT_JIT_XMM_PIN=0`,
     `AW2_JIT_XMMREG=0`, `AW2_JIT_FLAGLAZY=0`, `PORT_JIT_TSO=0`, `AW2_EMU_FAST=0`,
     `AW2_EMU_VERIFY=1`).
  2. Reproducer exit code contract:
       0 = CLEAN (bug did NOT reproduce)
       1 = BUG   (bug reproduced)
       other = ERROR / timeout (skipped / reported)
  3. If `default` reproduces the bug (exit 1) and `AW2_JIT=0` is clean (exit 0),
     optionally runs binary search over `--bisect-range <lo>..<hi>` (or
     `module.exe:0x0..0x2000000`) using `PORT_JIT_SKIP` to narrow the bug to the
     exact guilty guest virtual address range / basic block in O(log N) runs!

Usage:
  python3 tools/debug/bisect_runtime.py \
      --cmd "python3 tools/debug/live_probe.py --manifest games/<id>/game.toml --game-dir /path/to/Game --script repro.txt" \
      --bisect-range "game.exe:0x1000..0x2000000" \
      --out out/bisect_report.json
"""

import argparse
import json
import os
import shlex
import subprocess
import sys
import time


DEFAULT_MATRIX = [
    ("default", {}),
    ("no_aot", {"PORT_AOT": "0"}),
    ("aot_no_link", {"PORT_AOT_NO_LINK": "1"}),
    ("aot_no_cond_patch", {"PORT_AOT_NO_COND_PATCH": "1"}),
    ("no_fast_nan", {"PORT_JIT_FAST_NAN": "0", "PORT_AOT": "0"}),
    ("no_lse2_trap", {"PORT_JIT_LSE2_TRAP": "0", "PORT_AOT": "0"}),
    ("stack_tso", {"PORT_JIT_STACK_TSO": "1", "PORT_AOT": "0"}),
    ("no_xmm_pin", {"PORT_JIT_XMM_PIN": "0", "PORT_AOT": "0"}),
    ("no_xmmreg", {"AW2_JIT_XMMREG": "0", "PORT_AOT": "0"}),
    ("no_flaglazy", {"AW2_JIT_FLAGLAZY": "0", "PORT_AOT": "0"}),
    ("no_tso", {"PORT_JIT_TSO": "0", "PORT_AOT": "0"}),
    ("no_last_bits", {"PORT_JIT_LAST_BITS": "0", "PORT_AOT": "0"}),
    ("no_ras", {"AW2_JIT_RAS": "0", "PORT_AOT": "0"}),
    ("no_ro_plain", {"AW2_JIT_RO_PLAIN": "0", "PORT_AOT": "0"}),
    ("no_hw_range", {"PORT_JIT_HW_RANGE": "0", "PORT_AOT": "0"}),
    ("no_jit", {"AW2_JIT": "0", "PORT_AOT": "0"}),
    ("no_fast_ops", {"AW2_JIT": "0", "AW2_EMU_FAST": "0", "PORT_AOT": "0"}),
]


def run_probe(cmd: list[str], extra_env: dict[str, str], label: str, timeout_s: int, confirm_clean: int = 1) -> dict:
    attempts = []
    final = None
    for attempt in range(1, max(1, confirm_clean) + 1):
        env = os.environ.copy()
        env.update(extra_env)
        alabel = label if confirm_clean == 1 else f"{label}_t{attempt}"
        env["BISECT_LABEL"] = alabel
        t0 = time.time()
        try:
            p = subprocess.run(cmd, env=env, capture_output=True, text=True, timeout=timeout_s)
            rc = p.returncode
            out_tail = (p.stdout + "\n" + p.stderr).strip().splitlines()[-12:]
        except subprocess.TimeoutExpired:
            rc = 124
            out_tail = ["TIMEOUT"]
        dt = round(time.time() - t0, 2)
        status = "CLEAN" if rc == 0 else ("BUG" if rc == 1 else f"ERR({rc})")
        tag = f"{label}" if confirm_clean == 1 else f"{label}[{attempt}/{confirm_clean}]"
        print(f"  [{status:7s}] {tag:22s} ({dt:5.1f}s) env={extra_env}", flush=True)
        rec = {"label": alabel, "env": extra_env, "rc": rc, "status": status, "elapsed_s": dt, "tail": out_tail}
        attempts.append(rec)
        final = rec
        # A single BUG (exit 1) is definitive; only retry on CLEAN (exit 0) to rule out flakiness.
        if rc != 0:
            break
    assert final is not None
    final["attempts"] = len(attempts)
    return final


def parse_bisect_range(spec: str) -> tuple[str | None, int, int]:
    mod = None
    span = spec.strip()
    if ":" in span:
        mod, span = span.split(":", 1)
        mod = mod.strip()
    if ".." not in span:
        raise ValueError(f"Expected lo..hi in --bisect-range, got {spec!r}")
    a, b = span.split("..", 1)
    lo = int(a.strip(), 0)
    hi = int(b.strip(), 0)
    if lo >= hi:
        raise ValueError(f"Invalid range {lo:#x}..{hi:#x}")
    return (mod, lo, hi)


def fmt_range(mod: str | None, lo: int, hi: int) -> str:
    s = f"0x{lo:x}..0x{hi:x}"
    return f"{mod}:{s}" if mod else s


def main() -> int:
    ap = argparse.ArgumentParser(description="Automated x86-runtime knob matrix and PORT_JIT_SKIP bisector")
    ap.add_argument("--cmd", required=True, help="Reproducer command (exit 0=CLEAN, 1=BUG)")
    ap.add_argument("--bisect-range", help="Optional VA/RVA span for PORT_JIT_SKIP binary search (e.g. game.exe:0x1000..0x2000000)")
    ap.add_argument("--min-span", type=lambda s: int(s, 0), default=64, help="Stop binary search when span <= bytes (default 64)")
    ap.add_argument("--timeout", type=int, default=180, help="Per-run timeout in seconds")
    ap.add_argument("--matrix-only", action="store_true", help="Only run the knob matrix, skip binary search")
    ap.add_argument("--bisect-only", action="store_true", help="Skip the knob matrix and run binary search directly")
    ap.add_argument("--skip-var", choices=["PORT_AOT_SKIP", "PORT_JIT_SKIP"], default="PORT_AOT_SKIP", help="Skip filter env var for --bisect-only (default PORT_AOT_SKIP)")
    ap.add_argument("--confirm-clean", type=int, default=1, help="Require N consecutive CLEAN runs before accepting a CLEAN verdict (guards against probabilistic repros)")
    ap.add_argument("--extra-env", action="append", default=[], help="Extra KEY=VAL env var(s) passed to every probe")
    ap.add_argument("--fast-only", action="store_true", help="Skip slow full-boot AW2_JIT=0 / AW2_EMU_FAST=0 probes")
    ap.add_argument("--out", help="Write structured JSON report to this path")
    args = ap.parse_args()

    base_env: dict[str, str] = {}
    for item in args.extra_env:
        if "=" not in item:
            raise ValueError(f"Expected KEY=VAL in --extra-env, got {item!r}")
        k, v = item.split("=", 1)
        base_env[k] = v

    cmd = shlex.split(args.cmd)
    report: dict = {"cmd": cmd, "base_env": base_env, "matrix": [], "bisect": []}

    results = {}
    first_clean_knob = None
    if not args.bisect_only:
        print("=== Phase 1: Execution-mode knob matrix ===", flush=True)
        for label, extra in DEFAULT_MATRIX:
            if args.fast_only and label in ("no_jit", "no_fast_ops"):
                continue
            merged = dict(base_env)
            merged.update(extra)
            r = run_probe(cmd, merged, label, args.timeout, confirm_clean=args.confirm_clean)
            report["matrix"].append(r)
            results[label] = r
            if label == "default" and r["rc"] != 1:
                print("Default configuration did not reproduce BUG (exit 1); stopping.", flush=True)
                break
            if label == "no_aot" and r["rc"] == 0:
                first_clean_knob = label
                print("AOT vs live JIT divergence detected! Checking AOT link/cond_patch knobs next.", flush=True)
                continue
            if first_clean_knob == "no_aot" and label not in ("aot_no_link", "aot_no_cond_patch"):
                break
            if label not in ("default", "no_aot", "aot_no_link", "aot_no_cond_patch", "no_jit", "no_fast_ops") and r["rc"] == 0:
                first_clean_knob = label
                print(f"Found fixing JIT knob: {label} ({extra})! Stopping matrix early.", flush=True)
                break

    aot_only = (args.bisect_only and args.skip_var == "PORT_AOT_SKIP") or (
        results.get("default", {}).get("rc") == 1 and results.get("no_aot", {}).get("rc") == 0
    )
    jit_only = (args.bisect_only and args.skip_var == "PORT_JIT_SKIP") or (
        (first_clean_knob is not None and not aot_only)
        or (results.get("default", {}).get("rc") == 1 and results.get("no_jit", {}).get("rc") == 0)
    )
    if args.bisect_only:
        report["classification"] = f"DIRECT_BISECT:{args.skip_var}"
    elif aot_only:
        report["classification"] = "AOT_BUG"
        print("\nClassification: AOT_BUG (reproduces with AOT, CLEAN with PORT_AOT=0).", flush=True)
    elif first_clean_knob is not None:
        report["classification"] = f"JIT_OPT_BUG:{first_clean_knob}"
        print(f"\nClassification: JIT_OPT_BUG ({first_clean_knob} makes repro CLEAN).", flush=True)
    elif jit_only:
        report["classification"] = "JIT_MISCOMPILATION"
        print("\nClassification: JIT_MISCOMPILATION (reproduces with JIT, CLEAN with AW2_JIT=0).", flush=True)
    elif results.get("no_jit", {}).get("rc") == 1 and results.get("no_fast_ops", {}).get("rc") == 0:
        report["classification"] = "FAST_OP_INTERP_BUG"
        print("\nClassification: FAST_OP_INTERP_BUG (CLEAN with AW2_EMU_FAST=0; run AW2_EMU_VERIFY=1!).", flush=True)
    elif results.get("no_fast_ops", {}).get("rc") == 1:
        report["classification"] = "GENERIC_INTERP_OR_UPSTREAM_OR_WIN32"
        print("\nClassification: GENERIC_INTERP_OR_UPSTREAM_OR_WIN32 (use PORT_NAN_TRACE + x86_oracle.py).", flush=True)
    else:
        report["classification"] = "INCONCLUSIVE"

    if (aot_only or jit_only) and args.bisect_range and not args.matrix_only:
        skip_var = "PORT_AOT_SKIP" if aot_only else "PORT_JIT_SKIP"
        mod, lo, hi = parse_bisect_range(args.bisect_range)
        print(f"\n=== Phase 2: {skip_var} binary search on {fmt_range(mod, lo, hi)} (confirm_clean={args.confirm_clean}) ===", flush=True)
        step = 0
        while hi - lo > args.min_span:
            step += 1
            mid = lo + (hi - lo) // 2
            spec = fmt_range(mod, lo, mid)
            extra = dict(base_env)
            extra[skip_var] = spec
            if aot_only:
                extra["PORT_AOT"] = "1"
            r = run_probe(cmd, extra, f"bisect_{step:02d}", args.timeout, confirm_clean=args.confirm_clean)
            report["bisect"].append({
                "step": step,
                "var": skip_var,
                "skip": spec,
                "rc": r["rc"],
                "status": r["status"],
                "attempts": r.get("attempts", 1),
            })
            if args.out:
                with open(args.out, "w") as fh:
                    json.dump(report, fh, indent=2)
            if r["rc"] == 0:
                hi = mid
            elif r["rc"] == 1:
                lo = mid
            else:
                print(f"Bisection hit error rc={r['rc']} on {spec}; stopping.", flush=True)
                break
        report["bisect_result"] = fmt_range(mod, lo, hi)
        print(f"\nIsolated culprit {skip_var} range: {report['bisect_result']}", flush=True)

    if args.out:
        with open(args.out, "w") as fh:
            json.dump(report, fh, indent=2)
        print(f"Wrote report to {args.out}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
