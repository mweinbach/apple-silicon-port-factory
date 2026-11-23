---
name: debug-runtime-and-nan
description: Diagnose CPU translation divergences, floating-point NaN or MXCSR corruptions, AOT linking bugs, guest crashes, hangs, or gameplay regressions using the 5-stage Port Factory diagnostic toolkit (runtime-audit, verify-x86, live-probe, bisect-runtime, nan-trace, Rosetta 2 oracle), crash reports, and safe PID-attach LLDB. Use when hunting wrong math, visual/physics glitches, guest exceptions, or JIT vs interpreter mismatches.
---

# Debug Runtime, NaN, Crash & JIT/AOT Bugs

Follow the 5-stage diagnostic pipeline below (and see [env_knobs.md](./references/env_knobs.md) for
the full execution-knob table, `PORT_CONTROL_DIR` command grammar, and `x86-verify` blind-spot
decision tree). Every tool works on any Windows `.exe`/`.dll` with zero game-specific code.

## Stage 1 — Static Runtime-Hazard Audit (`runtime-audit`)

Scan the game's binaries offline in seconds for known Apple Silicon porting hazards (`ldmxcsr` /
`stmxcsr` FTZ/DAZ/RC assumptions, x87 FPU, AVX 256→128 narrowing, approximate `rcp`/`rsqrt`,
`loopcc`, `LOCK` atomics, `WaitOnAddress`, RWX sections):

```sh
cargo opsdb -- runtime-audit /path/to/GameDir
cargo opsdb -- runtime-audit /path/to/GameDir --only PhysX
```

## Stage 2 — Offline 4-Tier Verifier + Rosetta 2 Hardware Oracle (`verify-x86`)

Map any PE/DLL offline (no `DllMain`/`WinMain`, no GPU/window) and execute adversarial trials
(normal, subnormal, `±0`, `±Inf`, `NaN`, boundary ints, unaligned pointers) across all four tiers
(`Generic` vs `FastOp` vs `Live JIT` vs `AOT` + linked multi-block CFG + `MXCSR` parity):

```sh
# Exact-NaN mode (PORT_JIT_FAST_NAN=0 by default) across an entire install:
cargo opsdb -- verify-x86 /path/to/GameDir

# Focused DLL scan + bit-for-bit Rosetta 2 x86-64 hardware cross-check:
cargo opsdb -- verify-x86 /path/to/PhysX3_x64.dll --trials 4 --oracle --oracle-sample 64

# Direct alias for fast RVA / FP-only / strict-NaN sweeps:
cargo x86-verify /path/to/Game.exe --fp-only --rva 0x1000..0x200000 --strict-nan-payload
```

**Critical blind-spot rule:** If `verify-x86` passes with 0 mismatches AND the bug still reproduces
under 100% Live JIT (`PORT_AOT=0`) and `PORT_JIT_FAST_NAN=0`, then **JIT and the Rust reference
interpreter agree with each other while diverging from Windows x86-64 hardware** (e.g., `MXCSR`/
`FPCR.FZ`, host-call ABI, or multi-thread sync). Immediately run:
1. `AW2_JIT_PLAINCALL=0 AW2_JIT_HOSTCALL=0` (zero-rebuild test eliminating all inline host bridges),
2. Stage 5 (`PORT_NAN_TRACE` armed right before the failing moment + `oracle --mxcsr 0x9fc0` vs `0x1f80`).

## Stage 3 — Automated Reproducer (`live-probe` + `PORT_CONTROL_DIR`)

Turn any manual visual/physics/crash bug into a deterministic exit-code `0/1/2` reproducer
(`0 = CLEAN`, `1 = BUG`, `2 = ERROR/TIMEOUT`):

```sh
cargo opsdb -- live-probe \
  --manifest games/<id>/game.toml \
  --game-dir /path/to/Game \
  --ready-regex "Present #10" \
  --settle-frames 120 \
  --step "shot out/probe_f0.png" \
  --step "pad 0 32000 18000 0 0 0 lthumb 120" \
  --step "shot out/probe_f1.png" \
  --fail-regex '\[diag-boundary\]|UNIMPLEMENTED' \
  --repeat 3
```

See [env_knobs.md](./references/env_knobs.md) for all `PORT_CONTROL_DIR` commands (`shot`, `key`,
`mouse`, `moveto`, `click`, `wheel`, `pad`, `nantrace on|off`, `release`, `quit`).

## Stage 4 — Execution-Mode Knob Matrix & `O(log N)` VA Bisection (`bisect-runtime`)

Plug any `0/1/2` reproducer into `bisect-runtime` (`tools/debug/bisect_runtime.py`). It sweeps
17 execution-mode configurations to classify AOT vs JIT vs FastOp vs Generic/Win32 — then
binary-searches `--bisect-range` via `PORT_AOT_SKIP` / `PORT_JIT_SKIP` down to the exact guilty
basic block:

```sh
cargo opsdb -- bisect-runtime \
  --cmd "python3 tools/debug/live_probe.py --manifest games/<id>/game.toml --game-dir /path/to/Game --script repro.txt" \
  --bisect-range "Game.exe:0x1000..0x2000000" \
  --confirm-clean 3 \
  --out out/bisect_report.json
```

**Watch out for AOT cache misses masking A/B sweeps:** Always verify in logs whether `.aot` caches
actually loaded — `no cache for this build/knobs` means the run executed 100% Live JIT, not AOT!

## Stage 5 — Live `NaN`-Birth Tracing + Rosetta 2 Hardware Oracle (`nan-trace` + `oracle`)

Capture the exact first guest `RIP`, instruction, finite/subnormal source operands, `NaN` output,
guest backtrace, and `MXCSR` snapshot (start disarmed so boot runs at full JIT speed, then arm via
`PORT_CONTROL_DIR` `nantrace on` right before the failing action):

```sh
cargo opsdb -- nan-trace games/<id>/game.toml \
  --game-dir /path/to/Game \
  --module Game.exe \
  --dump-dir out/nan_snaps \
  --log out/nan_trace.jsonl \
  --control-dir out/ctl \
  --run-oracle

# Replay snapshots on real Rosetta 2 x86-64 hardware (or compare FTZ|DAZ vs IEEE-754):
cargo opsdb -- oracle out/nan_snaps/ --quiet-matches
cargo opsdb -- oracle out/nan_snaps/ --mxcsr 0x9fc0
cargo opsdb -- oracle out/nan_snaps/ --mxcsr 0x1f80

# Generate a Rust regression test snippet from a diverging snapshot:
python3 tools/debug/x86_oracle.py out/nan_snaps/snap_00001_*.json --emit-rust
```

Pair with `PORT_NAN_BOUNDARY=1` to log `[diag-boundary]` whenever non-finite floats cross
`win32-rt` middleware boundaries.

## Crash Reports & Safe LLDB Debugging

- **Built-in crash reports (`crates/win32-rt/src/crash.rs`):** Writes `out/crash/crash-<pid>-<time>.txt`
  (override `AW2_CRASH_DIR`) with exception code, registers, `RIP` disassembly, stack scan, and
  mixed PE (`.pdata`) + host (`rbp`) backtraces for all threads. Pass `PORT_SYMBOLS=<file.syms>`
  for full symbolication (`tools/debug/export_symbols.py` for AW2) and `AW2_EXIT_HOLD=30` to hold
  the window open on fatal fault.
- **CRITICAL LLDB + Rosetta rule:** **NEVER `process launch` an x86-64 Rosetta binary under
  `lldb`** (especially `--stop-at-entry`) — it can wedge Apple's `oahd` daemon machine-wide until
  `sudo killall -9 oahd`. Always attach by PID via `tools/lldb.sh`:
  ```sh
  tools/lldb.sh -b 0x1404aa1e0 -- --game-dir "$AW2_GAME_DIR"
  tools/lldb.sh -p <pid>
  ```
  Inside LLDB (`tools/debug/lldb_aw2.py`): `aw2sym`, `aw2b`, `aw2bt`, `aw2report [all]`, `aw2info`, `aw2go`.
