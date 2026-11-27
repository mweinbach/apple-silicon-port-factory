# Tools (`tools/`)

Helper scripts fall into two groups: **universal factory & debugging tools** (usable on any
Windows game) and **Alan Wake 2 reverse-engineering tools** (`fndb`, Ghidra, Wwise audio oracle).

Generated databases, Ghidra projects, and logs live in gitignored directories (`db/functions.sqlite`,
`re/ghidra/`, `out/`). Never commit game binaries or personal machine paths.

---

## 1. Universal Factory, Profiling & Debugging Tools

| Tool | What it does |
|---|---|
| `tools/port-acquire` | Pluggable CLI game acquisition wrapper (`epic` via Heroic's `legendary`, plus `tools/port_acquire/backend.py` interface for `steam` and other stores) with `--json` output, fuzzy search, DLC/redistributable listing, and `port-acquire-manifest.json` export. See [docs/ACQUISITION.md](../docs/ACQUISITION.md). |
| `tools/opsdb.sh` | Shell wrapper around `cargo opsdb --` (`crates/port-factory`). See [docs/OPS_DB.md](../docs/OPS_DB.md). |
| `tools/debug/live_probe.py` | Universal frame-paced `PORT_CONTROL_DIR` automation, input scripting, PNG capture, and pass/fail reproducer (`cargo opsdb -- live-probe`). See [docs/DIAGNOSTICS.md](../docs/DIAGNOSTICS.md). |
| `tools/debug/bisect_runtime.py` | Execution-mode knob matrix (`PORT_AOT`, `FAST_NAN`, `LSE2_TRAP`, `STACK_TSO`, `XMM_PIN`, `AW2_JIT`, `AW2_EMU_FAST`) + `O(log N)` `PORT_AOT_SKIP`/`PORT_JIT_SKIP` bisector (`cargo opsdb -- bisect-runtime`). |
| `tools/debug/x86_oracle.py` | Replays `PORT_NAN_TRACE` / `x86-verify` JSON snapshots (including exact GPR/XMM/YMM/`MXCSR`) on real Rosetta 2 x86-64 hardware (`cargo opsdb -- oracle`). |
| `tools/perf/boot_time.sh` | Launches the loader (headless by default) and times boot until the first log line matching `-m REGEX` (default `CreateDXGIFactory`). Supports `-n N` runs and `-s AT[:DUR]` `/usr/bin/sample` captures into `out/perf/`. Build first (`cargo runner-build` or `cargo loader-build`). |
| `tools/perf/busy.py` | Summarizes a macOS `/usr/bin/sample` file into per-thread busy percentages (filtering idle waits) and hottest leaf paths. |
| `tools/lldb.sh` | Safe LLDB wrapper that **attaches by PID** after `port-loader` maps guest images (`AW2_WAIT_DEBUGGER`), installs `-b <va\|rva\|name>` breakpoints, and resumes. Includes a timeout watchdog and exit cleanup trap. |
| `tools/debug/lldb_aw2.py` | LLDB helper commands: `aw2sym <addr>`, `aw2b <va\|rva\|name>`, `aw2bt`, `aw2report [all]`, `aw2info`, `aw2attach`, `aw2go`. |
| `tools/rosetta_run.sh` | Cargo runner for legacy `x86_64-apple-darwin` Rosetta differential tests (`cargo hybrid-test`). |
| `tools/jobprof*.py` | Offline analyzers for `x86-runtime` job/frame/GPU/pass profiler traces. |

### Built-in crash reports (`crates/win32-rt/src/crash.rs`)
On an unhandled guest exception, host fault, `__fastfail`, unresolved-import trap, or CRT abort,
the runtime logs a structured crash report and writes both `out/crash/crash-<pid>-<time>.txt` and a
schema v1 `out/crash/crash-<pid>-<time>.json` sidecar (override with `PORT_CRASH_DIR` /
`AW2_CRASH_DIR`). Each report contains the exception code, watchdog/heartbeat state, annotated
registers, bytes + disassembly at `RIP`, and mixed guest/host backtraces for all threads (guest PE
and dynamic table frames unwind via `.pdata`, including on ARM64 fatal crashes and `SIGUSR2`
thread snapshots, with stack scanning only as a fallback when unwinding stops early; host
Rust/system frames walk frame pointers; peer threads are paused briefly via `SIGUSR2`).

### Critical LLDB + Rosetta warning
**Never `process launch` an x86-64 Rosetta binary under `lldb`** (especially with
`--stop-at-entry`). A stopped Rosetta process can leave an orphaned
`/Library/Apple/usr/libexec/oah/debugserver` that wedges `oahd` machine-wide until
`sudo killall -9 oahd`. Always use `tools/lldb.sh`, which only attaches by PID and cleans up on
exit. Use Xcode's `xcrun -f lldb`, and call `/usr/bin/sample` explicitly so a Python `sample`
package on `PATH` doesn't shadow it.

### Reading Rosetta profiles
- A busy `com.apple.rosetta.exceptionserver` thread means writes are hitting executable code pages
  (self-modifying code trap). Keep mutable data strictly off RX/RWX pages.
- Guest threads busy in `Rosetta Runtime Routines` should happen once per code page; sustained
  activity there indicates code invalidation or repeated translation.

---

## 2. Alan Wake 2 Reverse-Engineering Tools

These scripts read your local Alan Wake 2 install via `$AW2_GAME_DIR` (or override the exe path
with `AW2_EXE=/path/to/AlanWake2.exe`).

| Tool | What it does |
|---|---|
| `tools/fndb.sh` (`tools/fndb/fndb.py`) | Function database + ownership classifier + decomp progress tracker (Python 3 stdlib + `tools/fndb/fnsig` Rust feature extractor). |
| `tools/ghidra_analyze.sh` | Builds/updates the headless Ghidra project in `re/ghidra/AW2` (`brew install ghidra`). |
| `tools/ghidra_decompile.sh` | Prints cached Ghidra pseudo-C for a VA/RVA/name (`--server-start` keeps Ghidra open for concurrent requests). |
| `tools/debug/export_symbols.py` | Exports `db/functions.sqlite` → `out/symbols/AlanWake2.exe.syms` so crash reports and logs print full function names (`PORT_SYMBOLS` / `AW2_SYMBOLS`). |
| `tools/audio_harness.sh`, `tools/audio_oracle.sh`, `tools/audio_dc_*.py` | Wwise realtime/offline capture & A/B oracle drivers (see [docs/AUDIO.md](../docs/AUDIO.md)). |
| `tools/decomp_scale/` | Python lifter scripts for `games/alan-wake-2/crates/aw2-game` shards. |

### Daily `fndb` & Ghidra commands
```sh
tools/fndb.sh build                     # ~20s -> db/functions.sqlite (~185 MB, gitignored)
tools/fndb.sh report                    # progress by owner / namespace
tools/fndb.sh next --ns rend            # small todo Remedy/unknown functions, leaves first
tools/fndb.sh show 0x1404aa1e0          # names, ownership evidence, vftables, callers/callees
tools/fndb.sh disasm 0x1404aa1e0        # annotated Intel disassembly (incl. chained fragments)
tools/fndb.sh can-throw 0x1404aa1e0     # check whether a VA can reach C++/SEH throw
tools/fndb.sh sync                      # sync games/alan-wake-2/crates/aw2-game -> db/{progress,throws}.tsv

tools/ghidra_decompile.sh --server-start
tools/ghidra_decompile.sh 0x1404aa1e0   # cached in out/decomp/<va>.c
tools/ghidra_decompile.sh --server-stop
```

### Key `fndb` subcommands
- `build [--exe PATH] [--no-fnsig] [--no-overrides] [--no-ghidra]` — Parses `.pdata`, RTTI
  (`re/symbol_map.json`), `fnsig` features, and optional `db/ghidra_functions.jsonl`, then classifies
  ownership ([docs/FNDB.md](../docs/FNDB.md)). Writes atomically via `*.tmp.sqlite`.
- `owner-eval` — Evaluates classifier precision against `db/owner_overrides.tsv` (with overrides
  temporarily disabled) and a 10% block name hold-out.
- `libcorpus <fetch|strings|refbuild|all>` — Optional; fetches pinned library sources and builds
  `clang-cl` reference objects for value-bag ownership matching.
- `set <va> <todo|draft|verified|skip> [--rust PATH] [--notes TEXT]` — Updates `db/progress.tsv`.
- `sync [--dry-run] [--keep-missing] [--src DIR]` — Scans `aw2-game` registries and `// @0x...`
  annotations to update both `db/progress.tsv` and `db/throws.tsv`.
- `funcsig <substr>` / `strings <substr>` / `sql "<query>"` — Searches `__FUNCSIG__` strings,
  referenced string literals, or runs ad-hoc SQLite queries (VAs formatted in hex).
- `ghidra-names` / `ingest-ghidra` — Round-trips function names and call graphs with
  `tools/ghidra_analyze.sh` (`--names`, `--export`, `--status`). Note that Ghidra locks its project
  to one process at a time, so stop `--server-stop` before running `--names` or `--export`.

### Database notes (`db/functions.sqlite`)
- `functions` tracks every function's `source` (`pdata`, `gap_ref`, `gap_unref`, `import_thunk`,
  `ghidra`), `owner` (`remedy`, `thirdparty`, `crt`, `unknown`), `owner_lib`, `owner_confidence`,
  and `owner_src`. All supporting evidence rows live in `owner_evidence`.
- Be skeptical of heuristic edges: `is_leaf` is conservative, `gap_*` boundaries can merge adjacent
  frameless functions, `vfunc_<slot>` names may be ICF-folded across unrelated classes, and
  low-confidence `neighbor`/`weak_neighbor` labels are layout-only guesses.
