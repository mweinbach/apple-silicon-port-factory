# Port Factory Diagnostic & Verification Toolkit

Every diagnostic tool we use to hunt down CPU translation divergences, floating-point `NaN` / `MXCSR`
issues, AOT linking bugs, and reproducible gameplay regressions lives in the universal **Port
Factory** toolkit (`crates/port-factory`, `crates/x86-runtime`, `crates/port-loader`, and
`tools/debug/`). All commands work on **any** Windows x86-64 `.exe`, `.dll`, or game install with
zero game-specific code.

Run every tool through the unified factory CLI (`cargo opsdb -- <cmd>` or `./tools/opsdb.sh <cmd>`):

---

## 1. The 5-Stage Bug-Hunting Workflow

When a game exhibits corrupted geometry, `NaN` physics/animation glitches, wrong math, hangs, or
execution-tier mismatches, run these stages in order:

### Stage 1 — Static Runtime-Hazard Audit (`runtime-audit`)
Scan the game's `.exe` and `.dll` files offline in seconds to spot known Apple Silicon porting
hazards (`ldmxcsr`/`stmxcsr` FTZ/DAZ/RC assumptions, x87 FPU ops, AVX 256→128 narrowing ops,
approximate `rcp`/`rsqrt`, `loopcc` branches, `LOCK` atomics, `WaitOnAddress` imports, and RWX
sections):

```sh
cargo opsdb -- runtime-audit /path/to/GameDir
cargo opsdb -- runtime-audit /path/to/GameDir --only PhysX
```

### Stage 2 — Whole-Binary 4-Tier Differential Verification + Hardware Oracle (`verify-x86`)
Map every `.exe`/`.dll` offline (no `WinMain`/`DllMain`, no window/GPU), discover all reachable
basic blocks, and execute adversarial trials (normal, subnormal, `±0`, `±Inf`, `NaN`, boundary ints,
unaligned pointers) across all four execution tiers (`Generic` vs `FastOp` vs `Live JIT` vs `AOT`
and linked multi-block CFG traces, including `MXCSR` parity).

Pass `--oracle` to also export passing FP/SIMD snapshots and validate them bit-for-bit on real
x86-64 hardware semantics via Rosetta 2:

```sh
# Scan an entire game install (defaults to exact-NaN mode PORT_JIT_FAST_NAN=0)
cargo opsdb -- verify-x86 /path/to/GameDir

# Scan one DLL + cross-check samples on real Rosetta 2 x86-64 hardware in one command
cargo opsdb -- verify-x86 /path/to/PhysX3_x64.dll --trials 4 --oracle --oracle-sample 64

# Direct Cargo alias
cargo x86-verify /path/to/Game.exe --fp-only --rva 0x1000..0x200000
```

#### Check the A64 words emitted by the JIT

[The A64 catalog](../db/a64_encodings.tsv) contains all 4,429 instruction encodings and
314 alias forms in Arm's September 2026 open-source A-profile release. [The emitter
matches](../db/a64_emitter_matches.tsv) record each emitted test word, its assembly, its
intended Arm operation page and the IDs matching both the bits and the assembly mnemonic.
Each row keeps its case number and word position so repeated instructions in a patched
sequence are traceable.
For example, the following commands show both register widths of `LSLV` and the cases that
emit them.

```sh
awk -F '\t' '$1 == "lslv"' db/a64_encodings.tsv
awk -F '\t' '$5 == "lslv"' db/a64_emitter_matches.tsv
cargo test -p x86-runtime --lib --target aarch64-apple-darwin -- a64_emitter_matches
```

The test re-emits the corpus and checks the committed matches offline. Clang independently
assembled every reference word when the matches were generated, including its register,
immediate and displacement operands. To refresh the matches after editing the emitters or cases,
run from the repository root on Apple Silicon with the Xcode command-line tools installed.

```sh
out_dir="$(mktemp -d)"
PORT_A64_MATCHES_OUT="$out_dir/matches.tsv" \
  cargo test -p x86-runtime --lib --target aarch64-apple-darwin -- a64_emitter_matches --nocapture &&
  mv "$out_dir/matches.tsv" db/a64_emitter_matches.tsv
```

For a catalog lookup, read `pattern` from bit 31 to bit 0. A word must match every `0`
and `1` and may take either value at an `x`. Reject a word matching any comma-separated
pattern in `exclude`. The `mnemonics` column lists the spellings in Arm's assembly grammar,
including suffixed forms such as `sxtl2`. The `fields` column gives the diagram's variable
fields as `name[high:low]`. An ID such as `CSINC_64_condsel/CSET` identifies an alias and its base
encoding. `condition` retains Arm's feature and operand requirements. The word patterns
assume all architectural features are available. `preferred` records Arm's disassembly
preference for an alias. A matched row establishes an encoding for the supplied assembly.
Use the feature condition when selecting an instruction for a particular CPU.

To reproduce the full catalog, download the pinned BSD-licensed archive and pass it to the
generator. It checks the archive SHA-256 and can compare the result byte for byte with the
tracked TSV. Omit `--check` to write the catalog to stdout.
[Arm archive and license](../THIRD_PARTY_NOTICES.md#arm-a64-encoding-data)

```sh
python3 tools/debug/a64_catalog.py /path/to/AARCHMRS_OPENSOURCE_A_profile_FAT-2026-09.tar.gz --check
```

For an additional independent check, set `PORT_A64_ENCODINGS` to an external TSV of Arm
instruction patterns. This test also assembles the operands with Clang. To build the
external TSV, extract an Arm A64 ISA XML release outside the checkout and
pass the directory containing its instruction XML files to the converter.
Its TSV has a `page` column for the instruction's XML filename stem and a `pattern`
column with 32 bits in most-significant-first order. A variable bit is written as
`x`. [Arm A64 ISA XML, September 2026](https://developer.arm.com/-/cdn-downloads/permalink/Exploration-Tools-A64-ISA/ISA_A64/ISA_A64_xml_A_profile-2026-09.tar.gz)

```sh
ref_dir="$HOME/Library/Caches/port-factory/arm-a64/2026-09"
mkdir -p "$ref_dir"
python3 tools/debug/arm_encoding_table.py /path/to/ISA_A64_xml_A_profile-2026-09 > "$ref_dir/encodings.tsv.tmp" &&
  mv "$ref_dir/encodings.tsv.tmp" "$ref_dir/encodings.tsv"
PORT_A64_ENCODINGS="$ref_dir/encodings.tsv" \
  cargo test -p x86-runtime --lib --target aarch64-apple-darwin -- a64_opcode_encodings --nocapture
```

### Stage 3 — Automated Reproducer & Frame Capture (`live-probe`)
Turn any manual bug into a repeatable, frame-paced reproducer using `PORT_CONTROL_DIR`.
`live-probe` launches `port-runner`, waits for a log readiness marker, pulses optional boot/menu
commands, executes a declarative step script (keyboard, mouse, XInput gamepad, PNG back-buffer
captures, `nantrace on/off`, frame waits), and returns a clean exit code (`0 = CLEAN`, `1 = BUG`,
`2 = ERROR/TIMEOUT`):

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
  --repeat 5
```

Or put the steps in a text file (`--script repro.txt`, one command per line, `#` comments allowed).

### Stage 4 — Execution-Mode Knob Matrix & `O(log N)` VA Bisection (`bisect-runtime`)
Plug any exit-code `0/1/2` reproducer (including `live-probe`) into `bisect-runtime`. It sweeps the
`x86-runtime` execution-mode matrix (`PORT_AOT=0`, `PORT_AOT_NO_LINK=1`, `PORT_AOT_NO_COND_PATCH=1`,
`PORT_JIT_FAST_NAN=0`, `PORT_JIT_LSE2_TRAP=0`, `PORT_JIT_STACK_TSO=1`, `PORT_JIT_XMM_PIN=0`,
`AW2_JIT_XMMREG=0`, `AW2_JIT_FLAGLAZY=0`, `PORT_JIT_TSO=0`, `AW2_JIT=0`, `AW2_EMU_FAST=0`) to
classify whether the bug is AOT-only, a JIT optimization, FastOp, or Generic/Win32 — and if a JIT/AOT
knob flips it clean, binary-searches `--bisect-range` via `PORT_AOT_SKIP` / `PORT_JIT_SKIP` down to
the exact guilty basic block:

```sh
cargo opsdb -- bisect-runtime \
  --cmd "python3 tools/debug/live_probe.py --manifest games/<id>/game.toml --game-dir /path/to/Game --script repro.txt" \
  --bisect-range "Game.exe:0x1000..0x2000000" \
  --confirm-clean 3 \
  --out out/bisect_report.json
```

### Stage 5 — Live `NaN`-Birth Tracing + Rosetta 2 Hardware Oracle (`nan-trace` + `oracle`)
When a bug reproduces even in the Generic interpreter (e.g., an `MXCSR` / FP-environment mismatch,
middleware boundary NaN, or an x86 instruction semantic difference), record exact `NaN` births,
guest backtraces, and full pre/post CPU + `MXCSR` snapshots, then replay them on Rosetta 2:

```sh
# Launch under PORT_NAN_TRACE + PORT_NAN_BOUNDARY (arm mid-run via PORT_CONTROL_DIR `nantrace on`, or pass --armed)
cargo opsdb -- nan-trace games/<id>/game.toml \
  --game-dir /path/to/Game \
  --module Game.exe \
  --dump-dir out/nan_snaps \
  --log out/nan_trace.jsonl \
  --control-dir out/ctl \
  --run-oracle

# Or replay existing snapshots under their captured MXCSR (or override --mxcsr to compare FTZ|DAZ vs IEEE-754)
cargo opsdb -- oracle out/nan_snaps/ --quiet-matches
cargo opsdb -- oracle out/nan_snaps/ --mxcsr 0x9fc0
cargo opsdb -- oracle out/nan_snaps/ --mxcsr 0x1f80
```

---

## 2. Runtime Environment Variable Reference

| Env Var | Component | Purpose |
|---|---|---|
| `PORT_NAN_TRACE` | `x86-runtime::diag` | Semicolon-separated spec: `module=a.exe,b.dll;range=mod:lo..hi;watch=lo..hi;host=sym;dump_dir=DIR;log=FILE;stores=1;max=N;armed=0\|1`. Zero overhead when unset. |
| `PORT_NAN_BOUNDARY=1` | `win32-rt` | Logs `[diag-boundary]` whenever non-finite floats cross middleware boundaries (e.g., `NvCloth::setSpheres`). |
| `PORT_JIT_SKIP` / `PORT_JIT_ALLOW` | `x86-runtime::diag` | Comma-separated `module:rva_lo..rva_hi` or absolute VA spans forced onto the interpreter. |
| `PORT_AOT_SKIP` | `x86-runtime::diag` | Comma-separated `module:rva_lo..rva_hi` spans excluded from AOT lookup (falls back to Live JIT). |
| `PORT_CONTROL_DIR=DIR` | `port-loader::live_ctl` | Enables frame-paced `<DIR>/cmd.txt` & `<DIR>/status.json` controller (`shot`, `key`, `mouse`, `moveto`, `click`, `wheel`, `pad`, `release`, `nantrace on\|off`, `quit`). |
| `PORT_AOT=0` | `x86-runtime` | Disables `.aot` cache loading (forces 100% Live JIT). |
| `PORT_AOT_NO_LINK=1` / `PORT_AOT_NO_COND_PATCH=1` | `x86-runtime` | Disables AOT direct branch patching / conditional branch patching. |
| `PORT_JIT_FAST_NAN=0` | `x86-runtime` | Enforces exact x86-64 NaN propagation & sign-bit parity in JIT/AOT. |
| `PORT_JIT_LSE2_TRAP=0` / `PORT_JIT_STACK_TSO=1` | `x86-runtime` | Controls LSE2 alignment-fault patching and stack-slot TSO ordering. |
| `AW2_JIT=0` / `AW2_EMU_FAST=0` | `x86-runtime` | Disables ARM64 JIT (`AW2_JIT=0`) or FastOp interpreter (`AW2_EMU_FAST=0`, pure Generic `iced-x86` interpreter). |

---

## 3. Source Files

| Tool | Source |
|---|---|
| Static hazard auditor & CLI integration | `crates/port-factory/src/diag_toolkit.rs`, `crates/port-factory/src/bin/port-factory.rs` |
| Offline 4-tier PE/DLL verifier (`x86-verify`) | `crates/x86-runtime/src/emu/verify.rs`, `crates/x86-runtime/src/bin/x86-verify.rs` |
| Live `NaN`-birth / watchpoint / host tracer | `crates/x86-runtime/src/diag.rs` |
| Live frame-paced controller (`PORT_CONTROL_DIR`) | `crates/port-loader/src/live_ctl.rs` |
| Universal game-agnostic probe harness | `tools/debug/live_probe.py` |
| Execution-mode matrix & VA bisector | `tools/debug/bisect_runtime.py` |
| Rosetta 2 x86-64 hardware snapshot oracle | `tools/debug/x86_oracle.py` |
