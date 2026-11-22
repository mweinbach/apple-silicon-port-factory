---
name: shader-pipeline
description: Inspect, convert, debug, or extend the crates/dxil-metal shader compiler pipeline (DXIL SM 6.x, FXC SM 4/5 via libdxilconv, SPIR-V/WGSL via naga), out-of-process dxil-metald worker, Tier-1 LC_UUID .metallib disk cache, and Tier-2 MTL4Archive + PsoManifest cold-device prewarming. Use when fixing shader conversion errors, PSO cache misses/stutters, converter crashes, or shader format support.
---

# Shader Pipeline & 2-Tier PSO Caching (`crates/dxil-metal`)

Hard rules: **never substitute game shaders**, **never emit fake `MTLB` bytes** when a toolchain is
missing (return honest `Error::Load`), and keep default `dxil-metal` builds (`features = ["metal"]`)
100% free of `aw2-data` (`tools` feature is opt-in for Alan Wake 2 `.binrfx` prewarming).

## Architecture Overview

| Input | Pipeline | Host Dependency |
|---|---|---|
| **DXIL (SM 6.0–6.6 + SM 6.8/6.9 lowering)** | `DXBC(DXIL)` → `libmetalirconverter.dylib` 4.0.1 → `.metallib` | `~/.local/opt/metal-shaderconverter` (`./scripts/install_metal_shader_converter.sh`, override `METAL_IRCONVERTER_DYLIB`) |
| **FXC DXBC (SM 4.0 / 5.x)** | `DXBC(SHEX/SHDR)` → `libdxilconv.dylib` → signed DXIL 1.0 → `.metallib` | `crates/dxil-metal/tools/dxilconv/build.sh` (override `AW2_DXILCONV`). Cache keys hash the original DXBC. |
| **SPIR-V (`.spv`) / WGSL (`.wgsl`)** | `naga` → MSL → `xcrun -sdk macosx metal` + `metallib` | Built-in `naga`; offline `.metallib` needs Xcode Metal Toolchain (`xcodebuild -downloadComponent MetalToolchain`). Live Vulkan forwards SPIR-V straight to MoltenVK. |

### 2-Tier Cache + Cold-Device Prewarming
- **Tier 1 (`crates/dxil-metal/src/cache.rs`, `.aw2s` `.metallib` AIR):** Portable across Apple
  Silicon Macs. Fingerprinted by immutable `codesign`-invariant Mach-O `LC_UUID`
  (`irX.Y.Z-uuid-<LC_UUID>`, with legacy prefix fallback). Reads optional read-only bundled seeds
  from `PORT_SHADER_CACHE_FALLBACK` (`Contents/Resources/ShaderCache`) and writes new entries to
  `PORT_SHADER_CACHE` (default `~/Translator/ShaderCache`). On iOS, runs as `BackendKind::CacheOnly`.
- **Tier 2 (`crates/dxil-metal/src/pso_cache.rs`, `.mtl4a` `MTL4Archive`):** AGX GPU machine code
  strictly isolated per `(MTLDevice.name, macOS build)` under `PORT_PSO_CACHE` (default
  `<shader cache>/pso/<gpu>-<hash>/`). Flushed periodically on D3D12/D3D11 `Present` and guaranteed
  on `ExitProcess`. **Never copy host `*.mtl4a` files into distributable `.app`/`.ipa` bundles.**
- **PSO Recipe Manifest (`crates/d3d12-metal/src/metal/pso_manifest.rs`, `records.p2m`):** Records
  GPU/OS-independent Compute/Graphics/Mesh PSO recipes (`PORT_PSO_MANIFEST`, read-only bundled
  `PORT_PSO_MANIFEST_FALLBACK`). On a new Mac or after a macOS update (`is_cold_device() == true`),
  `port-runner` precompiles recorded recipes in parallel (`PORT_PREWARM_SHADERS=auto|force|0`, or
  explicitly via `port-runner --prewarm-shaders`) with a native AppKit progress window when
  interactive.
- **Out-of-process worker (`dxil-metald`, `src/bin/dxil-metald.rs`, `src/helper.rs`):** Isolates
  converter crashes and enables x86-64 Rosetta runs (`PORT_SHADERD` / `AW2_SHADER_BACKEND=helper`).
  Uses a macOS `kqueue(EVFILT_PROC | NOTE_EXIT)` parent-death watchdog + `atexit`/`Drop` cleanup
  so zero orphan workers survive even after parent `SIGKILL`.
- **Function constants (`d3d12-metal/src/metal/pipeline.rs`):** Whenever
  `reflection.needs_function_constants` is set (e.g., `dual_source_blending` or `depth_feedback`
  at runtime), wrap in `MTL4SpecializedFunctionDescriptor` across compute, graphics, and mesh PSOs.

## Everyday Commands

```sh
# Inspect, convert, or offline-precompile shaders from any file or game dir
cargo opsdb -- shader-formats
cargo opsdb -- shader-inspect path/to/shader.dxil
cargo opsdb -- shader-convert path/to/shader.spv --msl-out out.metal --out out.metallib
cargo opsdb -- precompile-shaders /path/to/Game/

# Rebuild out-of-process arm64 worker daemon
cargo build --release -p dxil-metal --bin dxil-metald

# Cold-device parallel Tier-1 + Tier-2 Metal 4 PSO prewarm
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game --prewarm-shaders

# Validate every shader & PSO inside a recorded D3D12 trace (crash-isolated)
AW2_SHADER_BACKEND=helper cargo run -p d3d12-replay -- out/d3d12.trace --shaders

# Alan Wake 2 .binrfx corpus prewarm (opt-in tools feature)
cargo run --release -p dxil-metal --features tools --bin dxil-metal -- prewarm --manifest
```

## Debugging Shader Failures

1. **Isolate the failing stage:** Extract or inspect the container with `shader-inspect` (checks
   `DXIL`, `RTS0`, `SFI0`, `PSV0`, `RDAT`, `ISG1`/`ISGN`, `OSG1`/`OSGN`, `SHEX`/`SHDR`).
2. **Isolate converter crashes:** Run with `AW2_SHADER_BACKEND=helper` so a `libmetalirconverter`
   segfault surfaces cleanly as `Error::Helper` instead of crashing the game process.
3. **Check root signatures & stage-in layouts:** Verify `src/root_signature.rs` lowering and
   `ISG1`/`ISGN` vertex input signature matching (`d3d12-metal/src/metal/pipeline.rs`).
4. **Check SM 6.8/6.9 / Wave64 caps:** `d3d12-metal/src/caps.rs` caps `HighestShaderModel` at
   `0x66`; bitcode lowering in `src/bitcode.rs` handles SM 6.8/6.9 compatibility transforms.

## Verification Checklist

```sh
# 1. Full dxil-metal unit, LC_UUID cache, and worker SIGKILL lifecycle tests
cargo test -p dxil-metal --target aarch64-apple-darwin -- --test-threads=1

# 2. Metal 4 PSO manifest & cold-device prewarm tests
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- pso_manifest --test-threads=1

# 3. Sync shader format catalog if formats.rs changed & check drift
cargo opsdb -- shader-formats --sync
cargo opsdb -- doctor
cargo runner-build
```

See `docs/SHADERS.md` and `crates/dxil-metal/tools/README.md` for reference.
