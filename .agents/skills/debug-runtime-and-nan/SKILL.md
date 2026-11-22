---
name: debug-runtime-and-nan
description: Diagnose CPU translation divergences, floating-point NaN or MXCSR corruptions, AOT linking bugs, guest crashes, hangs, or gameplay regressions using the 5-stage Port Factory diagnostic toolkit (runtime-audit, verify-x86, live-probe, bisect-runtime, nan-trace, Rosetta 2 oracle), crash reports, and safe PID-attach LLDB. Use when hunting wrong math, visual/physics glitches, guest exceptions, or JIT vs interpreter mismatches.
---

# Debug Runtime, NaN, Crash & JIT/AOT Bugs

Follow the 5-stage diagnostic pipeline in `docs/DIAGNOSTICS.md` rather than guessing. Every tool
works on any Windows `.exe`/`.dll` with zero game-specific code.

## Stage 1 — Static Runtime-Hazard Audit (`runtime-audit`)

Scan the game's binaries offline in seconds for known Apple Silicon porting hazards (`ldmxcsr` /
`stmxcsr` FTZ/DAZ/RC assumptions, x87 FPU, AVX 256→128 narrowing, approximate `rcp`/`rsqrt`,
`loopcc`, `LOCK` atomics, `WaitOnAddress`, RWX sections):

```sh
cargo opsdb -- runtime-audit /path/to/GameDir
cargo opsdb -- runtime-audit /path/to/GameDir --only PhysX
```

## Stage 2 — Offline 4-Tier Verifier + Rosetta 2 Hardware Oracle (`verify-x86`)

Map any PE/DLL offline (no `DllMain`/`WinMain`, no GPU/window) and execute adversarial inputs
(normal, subnormal, `±0`, `±Inf`, `NaN`, boundary ints, unaligned pointers) across all four tiers
(`Generic` vs `FastOp` vs `Live JIT` vs `AOT` + linked multi-block CFG, including `MXCSR` parity):

```sh
# Exact-NaN mode (PORT_JIT_FAST_NAN=0 by default) across an entire install:
cargo opsdb -- verify-x86 /path/to/GameDir

# Focused DLL scan + bit-for-bit Rosetta 2 x86-64 hardware cross-check:
cargo opsdb -- verify-x86 /path/to/PhysX3_x64.dll --trials 4 --oracle --oracle-sample 64

# Direct alias for fast RVA / FP-only / fast-NaN sweeps:
cargo x86-verify /path/to/Game.exe --fp-only --rva 0x1000..0x200000 --strict-nan-payload
```

*(Note: `x86-verify` defaults to `PORT_JIT_LSE2_TRAP=0` and `PORT_JIT_FAST_NAN=0`; pass
`--fast-nan` to test fast-NaN mode.)*

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

`PORT_CONTROL_DIR` (`crates/port-loader/src/live_ctl.rs`) commands for `--step` or `--script`:
`shot <png>`, `key <name> <frames>`, `mouse <dx> <dy> <frames>`, `moveto <x> <y>`,
`click <left|right|middle> <frames>`, `wheel <delta> [frames]`,
`pad <lx> <ly> <rx> <ry> <lt> <rt> <buttons|none> <frames>`, `release`, `nantrace on|off`,
`wait <frames>`, `quit`.

## Stage 4 — Execution-Mode Knob Matrix & `O(log N)` VA Bisection (`bisect-runtime`)

Plug any `0/1/2` reproducer into `bisect-runtime` (`tools/debug/bisect_runtime.py`). It sweeps
`PORT_AOT=0`, `PORT_AOT_NO_LINK=1`, `PORT_AOT_NO_COND_PATCH=1`, `PORT_JIT_FAST_NAN=0`,
`PORT_JIT_LSE2_TRAP=0`, `PORT_JIT_STACK_TSO=1`, `PORT_JIT_XMM_PIN=0`, `AW2_JIT_XMMREG=0`,
`AW2_JIT_FLAGLAZY=0`, `PORT_JIT_TSO=0`, `PORT_JIT_LAST_BITS=0`, `AW2_JIT_RAS=0`,
`AW2_JIT_RO_PLAIN=0`, `PORT_JIT_HW_RANGE=0`, `AW2_JIT=0`, and `AW2_EMU_FAST=0` to isolate the
exact layer — then binary-searches `--bisect-range` via `PORT_AOT_SKIP` / `PORT_JIT_SKIP` down to
the single guilty basic block:

```sh
cargo opsdb -- bisect-runtime \
  --cmd "python3 tools/debug/live_probe.py --manifest games/<id>/game.toml --game-dir /path/to/Game --script repro.txt" \
  --bisect-range "Game.exe:0x1000..0x2000000" \
  --confirm-clean 3 \
  --out out/bisect_report.json
```

Quick manual isolation toggles:
- `PORT_AOT=0` → disables `.aot` cache (forces Live JIT)
- `PORT_AOT_VERIFY=1` → re-translates & compares `.aot` blocks word-for-word on load
- `AW2_JIT=0 PORT_AOT=0` → disables JIT (runs FastOp interpreter)
- `AW2_JIT=0 AW2_EMU_FAST=0 PORT_AOT=0` → pure Generic `iced-x86` interpreter
- `AW2_EMU_VERIFY=1` → shadow-checks every FastOp against Generic live
- `PORT_JIT_SKIP=mod.dll:0x1000..0x2000` / `PORT_AOT_SKIP=...` → isolate specific RVA spans

## Stage 5 — Live `NaN`-Birth Tracing + Rosetta 2 Hardware Oracle (`nan-trace` + `oracle`)

When a bug reproduces even in the Generic interpreter (e.g., `MXCSR` FTZ/DAZ mismatch, middleware
boundary NaN, or x86 semantic divergence):

```sh
# Record NaN births + guest backtraces + pre/post CPU/MXCSR snapshots (arm via `nantrace on` or --armed)
cargo opsdb -- nan-trace games/<id>/game.toml \
  --game-dir /path/to/Game \
  --module Game.exe \
  --dump-dir out/nan_snaps \
  --log out/nan_trace.jsonl \
  --control-dir out/ctl \
  --run-oracle

# Replay snapshots on real x86-64 Rosetta 2 hardware, or compare FTZ|DAZ vs IEEE-754 MXCSR:
cargo opsdb -- oracle out/nan_snaps/ --quiet-matches
cargo opsdb -- oracle out/nan_snaps/ --mxcsr 0x9fc0
cargo opsdb -- oracle out/nan_snaps/ --mxcsr 0x1f80
# Emit a regression test snippet from a diverging snapshot:
python3 tools/debug/x86_oracle.py out/nan_snaps/snap_00001_*.json --emit-rust
```

`PORT_NAN_TRACE` spec format (zero overhead when unset):
`module=a.exe,b.dll;range=mod:lo..hi;watch=lo..hi;host=sym;dump_dir=DIR;log=FILE;stores=1;max=N;armed=0|1`.
Pair with `PORT_NAN_BOUNDARY=1` to catch non-finite floats crossing `win32-rt` middleware boundaries.

## Crash Reports & Safe LLDB Debugging

- **Built-in crash reports (`crates/win32-rt/src/crash.rs`):** Unhandled guest exceptions, host
  faults, `__fastfail`, and unresolved-import traps automatically write
  `out/crash/crash-<pid>-<time>.txt` (override `AW2_CRASH_DIR`) with exception code, registers,
  disassembly at `RIP`, stack scan, and mixed PE (`.pdata`) + host (`rbp`) backtraces for all
  threads. Pass `PORT_SYMBOLS=<file.syms>` for full symbolication (`tools/debug/export_symbols.py`
  for AW2). Set `AW2_EXIT_HOLD=30` to keep windows visible on fatal fault.
- **CRITICAL LLDB + Rosetta rule:** **NEVER `process launch` an x86-64 Rosetta binary under
  `lldb`** (especially `--stop-at-entry`) — it can wedge Apple's `oahd` daemon machine-wide until
  `sudo killall -9 oahd`. Always attach by PID using `tools/lldb.sh`:
  ```sh
  tools/lldb.sh -b 0x1404aa1e0 -- --game-dir "$AW2_GAME_DIR"
  tools/lldb.sh -p <pid>
  ```
  Inside LLDB (`tools/debug/lldb_aw2.py`): `aw2sym <addr>`, `aw2b <va|rva|name>`, `aw2bt`,
  `aw2report [all]`, `aw2info`, `aw2go`.
