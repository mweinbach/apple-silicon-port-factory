# Complete Verification Matrix & Profiling Reference

## 1. Fast Targeted Verification Commands (By Subsystem)

Stay on `port-runner` (~15 s) and `aw2-core` (~15 s) rather than rebuilding full `aw2` shards
(~8+ min) unless you touched generated shards!

### Baseline Universal Gate (Run Before Every Commit / PR)
```sh
cargo runner-build
cargo factory-test -- --test-threads=1
cargo opsdb -- doctor
.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh --staged
```

### Subsystem Gates

| Subsystem Edited | Verification Commands |
|---|---|
| **`crates/win32-rt` (any export)** | `python3 .agents/skills/implement-win32-api/scripts/audit_win32_module.py <file.rs>` + `cargo opsdb -- win32-sync && cargo opsdb -- doctor` + `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- <module> --test-threads=1` + `cargo check -p win32-rt --lib --target aarch64-apple-ios` |
| **`crates/win32-rt` (`plain_builtin_fns` / `handles` / `d3d12::cmdlist`)** | `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- plain_and_pure_math handles d3d12 --test-threads=1` |
| **`crates/win32-rt` (TLS / Crypto / Authenticode)** | `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- misc::secur32 misc::crypt32 misc::bcrypt misc::wintrust misc::winhttp misc::libcurl --test-threads=1` |
| **`crates/win32-rt` (CLR / .NET / Mono)** | `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- misc::clr misc::hostfxr misc::coreclr misc::mscoree misc::mono_2_0_bdwgc dotnet_and_mono_runtime_bridges --test-threads=1` |
| **`crates/win32-rt` (Middleware: Bink2/FMOD/XAudio2/DirectML/DirectStorage/D3DCompiler/XeSS)** | `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- misc::bink2w64 misc::fmod misc::xaudio2_9redist misc::dstorage misc::directml misc::mfplat misc::d3dcompiler_47 misc::libxess --test-threads=1` |
| **`crates/x86-runtime` / `ops_db`** | `cargo opsdb -- sync && cargo opsdb -- doctor` + `cargo test -p x86-runtime --lib --target aarch64-apple-darwin -- --test-threads=1` + `cargo test -p x86-runtime --test aot_multi_module_verify --target aarch64-apple-darwin -- --test-threads=1` + `cargo check -p x86-runtime --lib --target aarch64-apple-ios` |
| **`crates/d3d12-metal` / D3D11 / Vulkan / MetalFX** | `MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- --test-threads=1` + `MTL_DEBUG_LAYER=1 cargo test -p win32-rt --lib --target aarch64-apple-darwin -- d3d11 dxgi d3d12::swapchain misc::vulkan_1 --test-threads=1` + `MTL_DEBUG_LAYER=1 cargo test -p port-loader --test d3d11_e2e --target aarch64-apple-darwin -- --test-threads=1` (+ `PORT_TEST_MOLTENVK=1 ... vulkan_e2e`) |
| **`crates/dxil-metal`** | `cargo test -p dxil-metal --target aarch64-apple-darwin -- --test-threads=1` (+ `cargo opsdb -- shader-formats --sync` if `formats.rs` changed) |
| **`crates/wwise-audio` / `win32-rt/src/audio`** | `cargo test -p wwise-audio --lib --target aarch64-apple-darwin` + `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- audio misc::setupapi` + `cargo test -p win32-rt --lib --features wwise-bridge --target aarch64-apple-darwin -- audio` |
| **`crates/port-factory` / `ios/` / `bink2-codec`** | `cargo test -p port-factory --test bundle_cold_launch --target aarch64-apple-darwin -- --test-threads=1` + `cargo test -p bink2-codec --target aarch64-apple-darwin` + `IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-sim-lib` |
| **Shared traits / signatures (`x86-runtime`, `win32-rt`, `port-loader`)** | `cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin` + `cargo check -p port-loader --features aw2-core --target aarch64-apple-darwin` + `cargo check -p win32-rt --lib --target x86_64-apple-darwin` |
| **Rust 2024 lint check** | `RUSTFLAGS="-D warnings -W rust-2024-compatibility" cargo check -p <crate> --target aarch64-apple-darwin` (catches `dangerous_implicit_autorefs` and `static_mut_refs`) |

## 2. Profiling Environment Variable Quick Card

- **Always-on hitch/stall detector (`win32-rt/src/hitch.rs`):** `AW2_HITCH_OP_MS=20`, `AW2_HITCH_MS=50`,
  `AW2_STALL_MS=250`, `AW2_HITCH_LOG=<file>` (`AW2_HITCH_MS=0` disables).
- **Zero-perturbation ARM64 AOT + host PC sampler (`x86-runtime/src/pc_sample.rs`):**
  `AW2_PC_SAMPLE=500` (Hz), `AW2_PC_SAMPLE_REPORT_S=30`, `AW2_JIT_STATS=1`.
- **Interpreter / `GENERIC_INTERP` profiler (`x86-runtime/src/emu_prof.rs`):**
  `AW2_EMU_PROF=1` (entry funcs + IP + host calls), `=2` (+ generic mnemonics), `=3` (+ fast ops).
- **AOT block translation profile (`x86-runtime/src/emu/aot.rs`):** `PORT_JIT_PROFILE=1`.
- **Metal 4 GPU encoder/barrier counters & pass timing (`d3d12-metal/src/metal/{encstats,passtiming}.rs`):**
  `AW2_ENC_STATS=1`, `AW2_PASS_TIMING=1`.
- **ECS / scheduler job timeline (`x86-runtime/src/jobprof.rs`):** `AW2_JOB_PROF=<start_s>[,<dur_s>]`
  → `tools/jobprof{,_frame,_gpu,_passes,_sys}.py out/hitch/jobprof-<pid>.bin`.
- **macOS stack sampler:** Always call `/usr/bin/sample <pid> <secs> -file out/perf/sample.txt`
  explicitly (avoids Python `sample` shadowing) and analyze with `tools/perf/busy.py`.
