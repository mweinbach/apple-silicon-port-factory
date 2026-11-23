---
name: profile-and-verify
description: Profile boot latency, CPU/AOT/interpreter hotspots, hitches/stalls, and Metal 4 GPU pass/encoder timing, and run the targeted pre-commit and pre-PR verification matrix (architectural boundaries, cargo opsdb -- doctor catalog drift check, and fast universal test suites). Use when optimizing performance, diagnosing stutters/hitches, or validating changes before committing or opening a PR.
---

# Profile Performance & Run Pre-PR Verification

See [verification_matrix.md](./references/verification_matrix.md) for the complete per-subsystem test
commands (including `aw2-core`, `aarch64-apple-ios`, `x86_64-apple-darwin`, and Rust 2024 lint
checks) and full profiler env-var reference.

## 1. Profiling Toolkit (Pick the Right Profiler)

| Bottleneck | Built-in Tool / Env Var | Output & Analysis |
|---|---|---|
| **Hitches, frame spikes, multi-thread freezes** | Always on (`win32-rt/src/hitch.rs`): `AW2_HITCH_OP_MS=20`, `AW2_HITCH_MS=50`, `AW2_STALL_MS=250` (`=0` disables) | Emits `[HITCH:FRAME]` breakdowns & live `SIGUSR2` mixed PE+host backtraces to `out/hitch/hitch-<pid>.log`. |
| **Native ARM64 AOT vs host CPU hotspots** | `AW2_PC_SAMPLE=500` (`x86-runtime/src/pc_sample.rs`, zero codegen change so `.aot` caches stay valid!) | Mach `ARM_THREAD_STATE64` sampler writing per-module AOT block RVAs & `dladdr` host symbols every 30 s (`pc-sample-<pid>.txt`). |
| **Interpreter / `GENERIC_INTERP` hotspots** | `AW2_EMU_PROF=1` (funcs + IP + host calls), `=2` (+ generic mnemonics), `=3` (+ fast ops) (`emu_prof.rs`) | Writes `out/hitch/emu-prof-<pid>.log` and annotates `[HITCH:FRAME]` reports. |
| **GPU encoder & barrier overhead** | `AW2_ENC_STATS=1` (`d3d12-metal/src/metal/encstats.rs`) | Per-frame draw/clear/util/compute encoders, queue/app barriers, ICB expands, folded/deferred clears. |
| **GPU pass duration & idle bubbles** | `AW2_PASS_TIMING=1` (`d3d12-metal/src/metal/passtiming.rs`) | Relaxed encoder boundary timestamps via 8 pooled 4096-entry `MTL4CounterHeap`s; reports GPU busy time, bubbles, and costliest encoders. |
| **MetalFX quality / perf A/B** | Compare `PORT_METALFX=1` (`PORT_METALFX_TEMPORAL=1\|0`) vs `PORT_METALFX=0` | Paired frame-time & `live-probe` PNG sharpness comparison. |
| **ECS / job scheduler timeline** | `AW2_JOB_PROF=<start_s>[,<dur_s>]` (`x86-runtime/src/jobprof.rs`) | Writes `out/hitch/jobprof-<pid>.bin`; analyze with `tools/jobprof{,_frame,_gpu,_passes,_sys}.py`. |
| **macOS thread sampling & boot time** | Invoke `/usr/bin/sample <pid> <secs> -file out/perf/sample.txt` explicitly (a Python `sample` package can shadow it!). Legacy Rosetta boot timer: `tools/perf/boot_time.sh -n 3 -m CreateDXGIFactory -s 5:10` | Summarize any macOS `sample` dump with `tools/perf/busy.py out/perf/sample.txt [--top 8] [--paths 6]`. |

### Reading Sample Profiles
- Busy `com.apple.rosetta.exceptionserver` → writes hitting executable pages (W^X / SMC trap). Keep
  mutable counters, tables, and vtables strictly off RX pages.
- High `jit_host_xfer` or `QueryPerformanceCounter` CPU → check whether hot leaf Win32/CRT calls
  belong in `plain_builtin_fns()` / `pure_float_math_fns()` (`win32-rt/src/lib.rs`) for tagged
  `JEnt::fast` / inline QPC dispatch.
- High `d3d12-metal-cs` `realloc` / lock time → check `TrackedSet<T>` reuse, `with_resource_registry`
  coalescing, and `win32-rt/src/d3d12/cmdlist.rs` inline stack captures.
- Sustained JIT codegen → repeated `va_gen` / `VirtualProtect` churn or missing offline
  `--prewarm-aot` coverage (`PORT_JIT_PROFILE=1`).

## 2. Pre-Commit & Pre-PR Fast Verification Gate

Avoid rebuilding full `aw2-loader` shards (~8+ min) unless you actually edited generated shards.
Use `port-runner` (~15 s) and `aw2-core` (~15 s):

```sh
# 1. Universal runner builds cleanly (~15 s)
cargo runner-build

# 2. Universal factory unit tests (always pass --test-threads=1)
cargo factory-test -- --test-threads=1

# 3. Repo hygiene, boundaries, LFS & canonical db/*.tsv drift gate
.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh --staged --doctor
```

Then run the subsystem-specific tests from [verification_matrix.md](./references/verification_matrix.md)
(e.g., `MTL_DEBUG_LAYER=1` for GPU edits, `cargo check -p win32-rt --target aarch64-apple-ios` for
Win32 edits, `cargo check -p aw2-game --no-default-features` + `cargo check -p port-loader
--features aw2-core` for shared-trait edits, and `-W rust-2024-compatibility` for raw-pointer edits).

Commit format: `<area>: <imperative summary>` (`factory`, `x86-runtime`, `win32`, `loader`, `d3d12`,
`d3d11`, `vulkan`, `shaders`, `audio`, `pe`, `ios`, `games`, `game/<ns>`, `tools`, `docs`, `build`).
Update `docs/TODOS.md` / `docs/STATUS.md` when closing tracked items.
