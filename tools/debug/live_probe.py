#!/usr/bin/env python3
"""Universal, game-agnostic frame-paced automation & diagnostic probe (`live_probe.py`).

Launches any Windows game under `port-runner` with `PORT_CONTROL_DIR` enabled, waits for
a configurable ready marker in the runtime log, executes a declarative sequence of
frame-paced keyboard/mouse/gamepad/screenshot/`nantrace` actions, and evaluates pass/fail
log regexes.

Exit-code contract (matches `tools/debug/bisect_runtime.py` directly):
  0 = CLEAN (reached ready state, completed all steps, 0 fail-regex hits, pass-regex met)
  1 = BUG   (fail-regex matched during run, or process crashed unexpectedly)
  2 = ERROR (configuration error or timed out before reaching ready state)

Examples:
  # Smoke-test any game with a 5-frame screenshot after Present #10
  python3 tools/debug/live_probe.py \
      --manifest games/overwatch/game.toml --game-dir /path/to/Overwatch \
      --ready-regex "Present #10" --settle-frames 60 \
      --step "shot out/probe_f0.png" --step "wait 30"

  # Drive a repeatable gameplay stress repro & plug into bisect_runtime.py
  python3 tools/debug/bisect_runtime.py \
      --cmd "python3 tools/debug/live_probe.py --manifest games/<id>/game.toml \
             --game-dir /path/to/Game --script repro_steps.txt" \
      --bisect-range "Game.exe:0x1000..0x2000000"
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path


EXIT_UNIMPL_OP = 86


def repo_root() -> Path:
    return Path(__file__).resolve().parent.parent.parent


def find_port_runner(explicit: str | None) -> Path:
    if explicit:
        p = Path(explicit)
        if not p.is_file():
            raise FileNotFoundError(f"port-runner not found: {p}")
        return p
    env_p = os.environ.get("PORT_RUNNER")
    if env_p and Path(env_p).is_file():
        return Path(env_p)
    root = repo_root()
    candidates = [
        root / "target/aarch64-apple-darwin/release/port-runner",
        root / "target/release/port-runner",
        root / "target/aarch64-apple-darwin/debug/port-runner",
        root / "target/debug/port-runner",
    ]
    existing = [p for p in candidates if p.is_file()]
    if existing:
        existing.sort(key=lambda p: p.stat().st_mtime, reverse=True)
        return existing[0]
    raise FileNotFoundError(
        "port-runner binary not found; build it first with `cargo runner-build` or pass --runner PATH"
    )


def send_ctl_cmd(ctl_dir: Path, cmd: str, wait_s: float = 15.0) -> None:
    cmd_file = ctl_dir / "cmd.txt"
    tmp_file = ctl_dir / "cmd.tmp"
    tmp_file.write_text(cmd.strip() + "\n", encoding="utf-8")
    tmp_file.replace(cmd_file)
    deadline = time.time() + wait_s
    while time.time() < deadline:
        if not cmd_file.exists():
            return
        time.sleep(0.05)


def read_status(ctl_dir: Path) -> dict:
    sf = ctl_dir / "status.json"
    try:
        return json.loads(sf.read_text(encoding="utf-8"))
    except Exception:
        return {}


def read_text_lossy(path: Path) -> str:
    try:
        return path.read_text(encoding="utf-8", errors="ignore")
    except Exception:
        return ""


def load_steps(inline_steps: list[str], script_path: str | None) -> list[str]:
    steps: list[str] = []
    if script_path:
        for raw in Path(script_path).read_text(encoding="utf-8").splitlines():
            line = raw.strip()
            if line and not line.startswith("#"):
                steps.append(line)
    for raw in inline_steps:
        line = raw.strip()
        if line and not line.startswith("#"):
            steps.append(line)
    return steps


def parse_wait_frames_from_step(step: str) -> int:
    """Infer how many frames to wait after sending a `PORT_CONTROL_DIR` command."""
    parts = step.split()
    if not parts:
        return 0
    verb = parts[0].lower()
    if verb == "wait" and len(parts) >= 2:
        return max(1, int(parts[1]))
    if verb == "sleep":
        return 0
    if verb in ("key", "click") and len(parts) >= 3:
        return max(1, int(parts[2])) + 2
    if verb == "mouse" and len(parts) >= 4:
        return max(1, int(parts[3])) + 2
    if verb == "pad" and len(parts) >= 9:
        return max(1, int(parts[8])) + 2
    if verb == "shot":
        return 4
    if verb in ("nantrace", "release", "moveto", "wheel"):
        return 2
    return 4


def wait_frames(
    proc: subprocess.Popen,
    ctl_dir: Path,
    log_path: Path,
    delta_frames: int,
    max_s: float,
    fail_re: re.Pattern[str] | None,
    stop_early: bool,
) -> tuple[int, list[str]]:
    f0 = int(read_status(ctl_dir).get("frame", 0))
    target = f0 + max(1, delta_frames)
    deadline = time.time() + max_s
    while time.time() < deadline and proc.poll() is None:
        if stop_early and fail_re is not None:
            hits = [l for l in read_text_lossy(log_path).splitlines() if fail_re.search(l)]
            if hits:
                cur = int(read_status(ctl_dir).get("frame", f0))
                return (max(0, cur - f0), hits)
        cur = int(read_status(ctl_dir).get("frame", f0))
        if cur >= target:
            hits = (
                [l for l in read_text_lossy(log_path).splitlines() if fail_re.search(l)]
                if fail_re is not None
                else []
            )
            return (max(0, cur - f0), hits)
        time.sleep(0.1)
    cur = int(read_status(ctl_dir).get("frame", f0))
    hits = (
        [l for l in read_text_lossy(log_path).splitlines() if fail_re.search(l)]
        if fail_re is not None
        else []
    )
    return (max(0, cur - f0), hits)


def run_one_trial(args: argparse.Namespace, runner: Path, steps: list[str], trial_idx: int) -> dict:
    label = os.environ.get("BISECT_LABEL", args.label)
    if args.repeat > 1:
        label = f"{label}_r{trial_idx}"

    log_path = Path(args.log_file).expanduser()
    ctl_dir = Path(tempfile.mkdtemp(prefix=f"port_live_probe_{label}_"))
    stdout_path = ctl_dir / "runner_stdout.log"

    fail_re = re.compile(args.fail_regex) if args.fail_regex else None
    ready_re = re.compile(args.ready_regex) if args.ready_regex else None
    pass_re = re.compile(args.pass_regex) if args.pass_regex else None

    if log_path.exists():
        try:
            log_path.unlink()
        except OSError:
            pass

    cmd = [str(runner)]
    if args.manifest:
        cmd += ["--manifest", args.manifest]
    if args.pe:
        cmd += ["--pe", args.pe]
    if args.game_dir:
        cmd += ["--game-dir", args.game_dir]
    if args.fail_on_unimpl_op:
        cmd += ["--fail-on-unimpl-op"]
    else:
        cmd += ["--no-fail-on-unimpl-op"]
    if args.game_args:
        cmd += ["--"] + args.game_args

    env = os.environ.copy()
    env["PORT_CONTROL_DIR"] = str(ctl_dir)
    env["PORT_FAIL_ON_UNIMPL_OP"] = "1" if args.fail_on_unimpl_op else "0"
    if args.nan_boundary:
        env["PORT_NAN_BOUNDARY"] = "1"
    if args.audio_null:
        env.setdefault("AW2_AUDIO_BACKEND", "null")
    for item in args.extra_env:
        if "=" not in item:
            raise ValueError(f"Expected KEY=VAL in --extra-env, got {item!r}")
        k, v = item.split("=", 1)
        env[k] = v

    shots_dir = Path(args.shots_dir) if args.shots_dir else None
    if shots_dir is not None:
        shots_dir.mkdir(parents=True, exist_ok=True)

    reached_ready = False
    frames_run = 0
    fail_hits: list[str] = []
    rc = 2
    t_start = time.time()

    try:
        with stdout_path.open("w", encoding="utf-8") as lf:
            proc = subprocess.Popen(cmd, env=env, stdout=lf, stderr=subprocess.STDOUT)
            try:
                # Phase 1: wait for ready regex (optionally pulsing boot-cmds after --boot-cmd-delay)
                t0 = time.time()
                boot_step = 0
                last_pulse = 0.0
                while time.time() - t0 < args.timeout_boot and proc.poll() is None:
                    log_txt = read_text_lossy(log_path)
                    if ready_re is None or ready_re.search(log_txt):
                        reached_ready = True
                        print(f"{label}: ready after +{time.time() - t0:.1f}s", flush=True)
                        break
                    elapsed = time.time() - t0
                    if (
                        args.boot_cmd
                        and elapsed >= args.boot_cmd_delay
                        and time.time() - last_pulse >= args.boot_cmd_interval
                    ):
                        send_ctl_cmd(ctl_dir, args.boot_cmd[boot_step % len(args.boot_cmd)])
                        boot_step += 1
                        last_pulse = time.time()
                    time.sleep(0.3)

                if reached_ready and proc.poll() is None:
                    if args.settle_frames > 0:
                        df, hits = wait_frames(
                            proc, ctl_dir, log_path, args.settle_frames, args.timeout_step, fail_re, False
                        )
                        frames_run += df
                        fail_hits = hits

                    if args.nantrace_arm:
                        send_ctl_cmd(ctl_dir, "nantrace on")
                        df, _ = wait_frames(proc, ctl_dir, log_path, 2, args.timeout_step, fail_re, False)
                        frames_run += df

                    # Phase 2: execute declarative steps
                    shot_idx = 0
                    for step in steps:
                        if proc.poll() is not None:
                            break
                        if args.stop_early and fail_hits:
                            break
                        lower = step.lower()
                        if lower.startswith("sleep "):
                            time.sleep(float(step.split()[1]))
                            continue
                        if lower.startswith("wait "):
                            n = int(step.split()[1])
                            df, hits = wait_frames(
                                proc, ctl_dir, log_path, n, args.timeout_step, fail_re, args.stop_early
                            )
                            frames_run += df
                            fail_hits = hits
                            continue
                        actual_cmd = step
                        if lower == "shot" and shots_dir is not None:
                            actual_cmd = f"shot {shots_dir / f'{label}_f{shot_idx}.png'}"
                            shot_idx += 1
                        elif lower.startswith("shot "):
                            sp = step.split(None, 1)[1].strip()
                            sp = sp.replace("{label}", label).replace("{trial}", str(trial_idx))
                            actual_cmd = f"shot {sp}"
                        send_ctl_cmd(ctl_dir, actual_cmd)
                        df, hits = wait_frames(
                            proc,
                            ctl_dir,
                            log_path,
                            parse_wait_frames_from_step(actual_cmd),
                            args.timeout_step,
                            fail_re,
                            args.stop_early,
                        )
                        frames_run += df
                        fail_hits = hits

                    if args.nantrace_arm and proc.poll() is None:
                        send_ctl_cmd(ctl_dir, "nantrace off")
                        time.sleep(0.2)

                # Final log check before shutting down cleanly
                final_log = read_text_lossy(log_path)
                if fail_re is not None:
                    fail_hits = [l for l in final_log.splitlines() if fail_re.search(l)]
                pass_ok = True if pass_re is None else bool(pass_re.search(final_log))

                if proc.poll() is None:
                    send_ctl_cmd(ctl_dir, "quit", wait_s=3.0)
                    try:
                        proc.wait(timeout=8)
                    except subprocess.TimeoutExpired:
                        proc.terminate()
                        try:
                            proc.wait(timeout=4)
                        except subprocess.TimeoutExpired:
                            proc.kill()
                            proc.wait(timeout=4)

                crashed = proc.returncode not in (0, None, -15)
                hit_unimpl_op = proc.returncode == EXIT_UNIMPL_OP
                if hit_unimpl_op:
                    rc = 1
                    print(
                        f"{label}: BUG after +{frames_run} frames (unimplemented x86 instruction, exit code {EXIT_UNIMPL_OP})",
                        flush=True,
                    )
                elif not reached_ready:
                    rc = 2
                    print(f"{label}: ERROR (did not reach ready state within {args.timeout_boot}s)", flush=True)
                elif fail_hits or crashed or not pass_ok:
                    rc = 1
                    reason = (
                        f"{len(fail_hits)} fail-regex hit(s)"
                        if fail_hits
                        else (f"exit code {proc.returncode}" if crashed else "pass-regex not matched")
                    )
                    print(f"{label}: BUG after +{frames_run} frames ({reason})", flush=True)
                    for h in fail_hits[:5]:
                        print(f"  {h}", flush=True)
                else:
                    rc = 0
                    print(f"{label}: CLEAN after +{frames_run} frames (0 fail hits)", flush=True)
            finally:
                if proc.poll() is None:
                    proc.kill()
                    proc.wait(timeout=5)
    finally:
        if not args.keep_ctl:
            shutil.rmtree(ctl_dir, ignore_errors=True)

    return {
        "label": label,
        "trial": trial_idx,
        "rc": rc,
        "status": "CLEAN" if rc == 0 else ("BUG" if rc == 1 else "ERROR"),
        "reached_ready": reached_ready,
        "frames_run": frames_run,
        "elapsed_s": round(time.time() - t_start, 2),
        "fail_hits": fail_hits[:20],
    }


def main() -> int:
    ap = argparse.ArgumentParser(
        description="Universal game-agnostic frame-paced PORT_CONTROL_DIR probe (exit 0=CLEAN, 1=BUG, 2=ERROR)"
    )
    ap.add_argument("--manifest", help="Path to games/<id>/game.toml")
    ap.add_argument("--pe", help="Path to target Windows .exe (if not using --manifest)")
    ap.add_argument("--game-dir", help="Game install directory")
    ap.add_argument("--runner", help="Explicit path to port-runner binary")
    ap.add_argument("--label", default="probe", help="Label prefix for logs and screenshots")
    ap.add_argument("--log-file", default="~/Library/Logs/Translator/port-runner.log", help="Runtime log file to monitor")
    ap.add_argument("--ready-regex", default="Present #10", help="Regex in log indicating game is ready for steps (empty to start immediately)")
    ap.add_argument("--fail-regex", default=r"\[diag-boundary\]", help="Regex in log marking a BUG (exit 1)")
    ap.add_argument("--pass-regex", default="", help="Optional regex that must appear in log for CLEAN")
    ap.add_argument("--settle-frames", type=int, default=60, help="Frames to wait after --ready-regex before steps")
    ap.add_argument("--step", action="append", default=[], help="Action step (repeatable): e.g. 'key w 60', 'shot out/f0.png', 'wait 30', 'pad ...'")
    ap.add_argument("--script", help="File containing one action step per line (# comments allowed)")
    ap.add_argument("--boot-cmd", action="append", default=[], help="Command(s) pulsed cyclically while waiting for --ready-regex (e.g. 'key return 5')")
    ap.add_argument("--boot-cmd-delay", type=float, default=10.0, help="Seconds before pulsing --boot-cmd")
    ap.add_argument("--boot-cmd-interval", type=float, default=2.5, help="Seconds between --boot-cmd pulses")
    ap.add_argument("--shots-dir", help="Directory for bare 'shot' steps")
    ap.add_argument("--nantrace-arm", action="store_true", help="Send 'nantrace on' before steps and 'nantrace off' after")
    ap.add_argument("--nan-boundary", action="store_true", default=True, help="Set PORT_NAN_BOUNDARY=1 (default on)")
    ap.add_argument("--no-nan-boundary", dest="nan_boundary", action="store_false")
    ap.add_argument("--fail-on-unimpl-op", action="store_true", default=True, help="Pass --fail-on-unimpl-op / PORT_FAIL_ON_UNIMPL_OP=1 (exit 86 on unimplemented x86 op; default on)")
    ap.add_argument("--no-fail-on-unimpl-op", dest="fail_on_unimpl_op", action="store_false")
    ap.add_argument("--audio-null", action="store_true", default=True, help="Default AW2_AUDIO_BACKEND=null for headless/CI reliability")
    ap.add_argument("--keep-audio", dest="audio_null", action="store_false")
    ap.add_argument("--stop-early", action="store_true", help="Stop executing steps on first --fail-regex hit")
    ap.add_argument("--repeat", type=int, default=1, help="Run N consecutive trials (fails immediately on BUG/ERROR)")
    ap.add_argument("--timeout-boot", type=int, default=140, help="Max seconds to wait for --ready-regex")
    ap.add_argument("--timeout-step", type=float, default=60.0, help="Max seconds per step wait")
    ap.add_argument("--extra-env", action="append", default=[], help="Extra KEY=VAL env var(s) passed to port-runner")
    ap.add_argument("--keep-ctl", action="store_true", help="Keep temporary PORT_CONTROL_DIR for debugging")
    ap.add_argument("--json-out", help="Write structured JSON trial report to this path")
    ap.add_argument("game_args", nargs=argparse.REMAINDER, help="Extra arguments passed after `--` to the Windows game")
    args = ap.parse_args()

    if args.game_args and args.game_args[0] == "--":
        args.game_args = args.game_args[1:]
    if not args.manifest and not args.pe:
        ap.error("Provide --manifest <game.toml> or --pe <Game.exe>")

    runner = find_port_runner(args.runner)
    steps = load_steps(args.step, args.script)
    trials = []
    worst_rc = 0
    for idx in range(1, max(1, args.repeat) + 1):
        rec = run_one_trial(args, runner, steps, idx)
        trials.append(rec)
        if rec["rc"] != 0:
            worst_rc = rec["rc"]
            break

    if args.json_out:
        out_p = Path(args.json_out)
        out_p.parent.mkdir(parents=True, exist_ok=True)
        out_p.write_text(json.dumps({"trials": trials, "rc": worst_rc}, indent=2) + "\n", encoding="utf-8")

    return worst_rc


if __name__ == "__main__":
    sys.exit(main())
