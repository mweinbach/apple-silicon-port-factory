---
name: profile-and-verify
description: Profile boot latency, CPU/AOT/interpreter hotspots, hitches/stalls, and Metal 4 GPU pass/encoder timing, and run the targeted pre-commit and pre-PR verification matrix (architectural boundaries, cargo opsdb -- doctor catalog drift check, and fast universal test suites). Use when optimizing performance, diagnosing stutters/hitches, or validating changes before committing or opening a PR.
---

# Profile Performance & Run Pre-PR Verification

## 1. Profiling Toolkit (Pick the Right Profiler)

| Bottleneck | Built-in Tool / Env Var | Output & Analysis |
|---|---|---|
| **Hitches, frame spikes, multi-thread freezes** | Always on (`crates/win32-rt/src/hitch.rs`): `AW2_HITCH_OP_MS=20`, `AW2_HITCH_MS=50`, `AW2_STALL_MS=250` (`AW2_HITCH_MS=0` disables) | Appends `[HITCH:FRAME]` breakdowns & live `SIGUSR2` mixed PE+host backtraces to `out/hitch/hitch-<pid>.log` (`AW2_HITCH_LOG`). |
| **Native ARM64 AOT vs host CPU hotspots** | `AW2_PC_SAMPLE=500` (`crates/x86-runtime/src/pc_sample.rs`, zero codegen change so AOT caches stay valid) | Mach `ARM_THREAD_STATE64` sampler writing per-module AOT block RVAs & `dladdr` host symbols every `AW2_PC_SAMPLE_REPORT_S` (30 s) to `pc-sample-<pid>.txt`. |
| **Interpreter / `GENERIC_INTERP` hotspots** | `AW2_EMU_PROF=1` (entry funcs + IP + host calls), `=2` (+ generic mnemonics), `=3` (+ fast mnemonics) (`emu_prof.rs`) | Writes `out/hitch/emu-prof-<pid>.log` and annotates `[HITCH:FRAME]` reports. |
| **GPU encoder & barrier overhead** | `AW2_ENC_STATS=1` (`d3d12-metal/src/metal/encstats.rs`) | Per-frame averages for draw/clear/util/compute encoders, queue/app barriers, ICB expands, cleared/deferred passes. |
| **GPU pass duration & bubbles** | `AW2_PASS_TIMING=1` (`d3d12-metal/src/metal/passtiming.rs`) | Relaxed encoder boundary timestamps via 8 pooled 4096-entry `MTL4CounterHeap`s; reports GPU busy time, idle bubbles, and costliest encoders. |
| **MetalFX quality / perf A/B** | Compare `PORT_METALFX=1` (and `PORT_METALFX_TEMPORAL=1|0`) vs `PORT_METALFX=0` | Paired frame-time & `live-probe` PNG sharpness comparison. |
| **ECS / job scheduler timeline** | `AW2_JOB_PROF=<start_s>[,<dur_s>]` (`x86-runtime/src/jobprof.rs`) | Writes `out/hitch/jobprof-<pid>.bin`; analyze with `tools/jobprof.py`, `jobprof_frame.py`, `jobprof_gpu.py`, `jobprof_passes.py`, `jobprof_sys.py`. |
| **macOS thread sampling & boot time** | Always invoke `/usr/bin/sample` explicitly (a Python `sample` package can shadow it!). For legacy Rosetta `aw2-loader`: `tools/perf/boot_time.sh -n 3 -m CreateDXGIFactory -s 5:10` | Summarize any macOS `sample` dump with `tools/perf/busy.py out/perf/sample_1.txt [--top 8] [--paths 6]`. |

### Reading Sample Profiles
- Busy `com.apple.rosetta.exceptionserver` → writes are hitting executable pages (W^X / SMC trap).
  Move mutable counters/tables/vtables strictly off RX pages.
- Sustained `Rosetta Runtime Routines` or per-thread JIT codegen → repeated code invalidation
  (`va_gen` / `VirtualProtect` churn) or missing offline `--prewarm-aot` coverage (`PORT_JIT_PROFILE=1`
  to inspect translated blocks).

---

## 2. Pre-Commit & Pre-PR Verification Matrix

Avoid rebuilding `aw2-loader` (~8+ min for 4.47M LOC shards) unless you actually edited
`games/alan-wake-2/`. Stay on `port-runner` (~15 s) and run the exact gates matching your diff:

### Always Run
```sh
# 1. Universal runner builds with zero warnings in files you touched
cargo runner-build

# 2. Universal factory unit tests (always pass --test-threads=1)
cargo factory-test -- --test-threads=1

# 3. Canonical db/*.tsv drift gate (exits non-zero on any drift; pass --fix to sync)
cargo opsdb -- doctor
```

### Run Based on Touched Subsystems

| Touched Area | Required Verification Command(s) |
|---|---|
| `crates/win32-rt` exports | `cargo opsdb -- win32-sync && cargo opsdb -- doctor` + `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- <module> --test-threads=1` |
| `crates/x86-runtime` or `ops_db` | `cargo opsdb -- sync && cargo opsdb -- doctor` + `cargo test -p x86-runtime --lib --target aarch64-apple-darwin` + `cargo test -p x86-runtime --test aot_multi_module_verify --target aarch64-apple-darwin -- --test-threads=1` |
| `crates/d3d12-metal` / `d3d11` / `vulkan_1` | `MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- --test-threads=1` + `MTL_DEBUG_LAYER=1 cargo test -p win32-rt --lib --target aarch64-apple-darwin -- d3d11 misc::vulkan_1 --test-threads=1` + `MTL_DEBUG_LAYER=1 cargo test -p port-loader --test d3d11_e2e --target aarch64-apple-darwin -- --test-threads=1` (+ `PORT_TEST_MOLTENVK=1 ... vulkan_e2e` if Vulkan changed) |
| `crates/dxil-metal` | `cargo test -p dxil-metal --target aarch64-apple-darwin -- --test-threads=1` (+ `cargo opsdb -- shader-formats --sync` if `formats.rs` changed) |
| `crates/wwise-audio` / `win32-rt/src/audio` | `cargo test -p wwise-audio --lib --target aarch64-apple-darwin` + `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- audio misc::setupapi` |
| `crates/port-factory` bundling / `ios/` | `cargo test -p port-factory --test bundle_cold_launch --target aarch64-apple-darwin -- --test-threads=1` + `IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-sim-lib` |
| Shared traits/signatures in `x86-runtime` or `win32-rt` | `cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin` (fast ~12 s check without shards) |
| `games/alan-wake-2/` | `cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin` + `tools/fndb.sh sync` + `--diff-test` |

### Final Hygiene Checks Before Committing
1. **Boundary check:** Zero `objc2-metal` or `aw2-data` in `crates/win32-rt`; zero `aw2-game` or
   `aw2-data` in `port-runner` default builds; zero game-specific VAs/hacks in `crates/`.
2. **Privacy & copyright check:** `git diff --cached` contains zero `.exe`/`.dll`/game assets,
   zero `.app`/`.ipa`, and zero hardcoded `/Users/<name>` or personal `/Volumes/` paths.
3. **Commit format:** `<area>: <imperative summary>` (`factory`, `x86-runtime`, `win32`, `loader`,
   `d3d12`, `d3d11`, `vulkan`, `shaders`, `audio`, `pe`, `ios`, `games`, `game/<ns>`, `tools`,
   `docs`, `build`). Update `docs/TODOS.md` / `docs/STATUS.md` if closing a tracked item.
