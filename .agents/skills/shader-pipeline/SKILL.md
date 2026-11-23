---
name: shader-pipeline
description: Inspect, convert, debug, or extend the crates/dxil-metal shader compiler pipeline (DXIL SM 6.x, FXC SM 4/5 via libdxilconv, SPIR-V/WGSL via naga), runtime D3DCompile/D3DReflect in d3dcompiler_47.rs, out-of-process dxil-metald worker, Tier-1 LC_UUID .metallib disk cache, and Tier-2 MTL4Archive + PsoManifest cold-device prewarming. Use when fixing shader conversion errors, PSO cache misses/stutters, converter crashes, or shader format support.
---

# Shader Pipeline & 2-Tier PSO Caching (`crates/dxil-metal`)

Hard rules: **never substitute game shaders**, **never emit fake `MTLB` bytes** when a toolchain is
missing (return honest `Error::Load`), and keep default `dxil-metal` builds (`features = ["metal"]`)
100% free of `aw2-data` (`tools` feature is opt-in for Alan Wake 2 `.binrfx` prewarming).

See [cache_and_formats.md](./references/cache_and_formats.md) for the complete 2-tier cache
directory layout, `d3dcompiler_47.rs` runtime HLSL compiler/reflector details, `dxil-metald`
watchdog design, and host toolchain paths.

## Architecture Overview

| Input | Pipeline | Host Dependency |
|---|---|---|
| **DXIL (SM 6.0–6.6 + SM 6.8/6.9 lowering)** | `DXBC(DXIL)` → `libmetalirconverter.dylib` 4.0.1 → `.metallib` | `~/.local/opt/metal-shaderconverter` (`./scripts/install_metal_shader_converter.sh`, override `METAL_IRCONVERTER_DYLIB`) |
| **FXC DXBC (SM 4.0 / 5.x)** | `DXBC(SHEX/SHDR)` → `libdxilconv.dylib` → signed DXIL 1.0 → `.metallib` | `crates/dxil-metal/tools/dxilconv/build.sh` (`AW2_DXILCONV`). Cache keys hash original DXBC. |
| **Runtime HLSL (`d3dcompiler_47.rs`)** | Pure-Rust `D3DPreprocess` + `D3DReflect` (macOS & iOS); `D3DCompile[2]` via `libdxcompiler.dylib` (SM 4/5 auto-upgraded to SM 6.0 + retail MD5 re-sign) | Optional `libdxcompiler.dylib` (`DXC_DYLIB`, `~/.local/lib`, Homebrew) or `dxc` CLI |
| **SPIR-V (`.spv`) / WGSL (`.wgsl`)** | `naga` → MSL → `xcrun -sdk macosx metal` + `metallib` | Built-in `naga`; offline `.metallib` needs Xcode Metal Toolchain. Live Vulkan forwards SPIR-V straight to MoltenVK. |

### Key Cache & Pipeline Invariants
- **Tier 1 (`.aw2s` `.metallib` AIR):** Portable across Apple Silicon Macs & iOS. Fingerprinted by
  immutable `codesign`-invariant Mach-O `LC_UUID` (`irX.Y.Z-uuid-<LC_UUID>`). Read-only bundled
  seeds use `PORT_SHADER_CACHE_FALLBACK`; writes go to `PORT_SHADER_CACHE` (`BackendKind::CacheOnly`
  on iOS).
- **Tier 2 (`MTL4Archive` `.mtl4a`):** Isolated per `(MTLDevice.name, macOS build)` under
  `PORT_PSO_CACHE`, flushed on `Present` and `ExitProcess`. **Never copy host `*.mtl4a` files into
  distributable `.app`/`.ipa` bundles.**
- **`PsoManifest` (`records.p2m`):** Portable GPU/OS-independent Compute/Graphics/Mesh PSO recipes
  (`d3d12-metal/src/metal/pso_manifest.rs`) driving parallel cold-device prewarming on new Macs or
  macOS updates (`PORT_PREWARM_SHADERS=auto|force|0`, `--prewarm-shaders`).
- **Out-of-process worker (`dxil-metald`):** Crash-isolated arm64 worker (`AW2_SHADER_BACKEND=helper`,
  `PORT_SHADERD`) with `kqueue(EVFILT_PROC | NOTE_EXIT)` parent-death watchdog + `atexit`/`Drop`
  cleanup (0 stray workers even on parent `SIGKILL`).
- **Function constants (`d3d12-metal/src/metal/pipeline.rs`):** Wrap in
  `MTL4SpecializedFunctionDescriptor` whenever `reflection.needs_function_constants` is set.

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

# Crash-isolated validation of every shader & PSO inside a recorded D3D12 trace
AW2_SHADER_BACKEND=helper cargo run -p d3d12-replay -- out/d3d12.trace --shaders

# Alan Wake 2 .binrfx corpus prewarm (opt-in tools feature)
cargo run --release -p dxil-metal --features tools --bin dxil-metal -- prewarm --manifest
```

## Debugging Shader Failures

1. **Inspect the container:** Run `cargo opsdb -- shader-inspect <file>` (verifies `DXIL`, `RTS0`,
   `SFI0`, `PSV0`, `RDAT`, `ISG1`/`ISGN`, `OSG1`/`OSGN`, `SHEX`/`SHDR`, and MD5 checksum).
2. **Isolate converter segfaults:** Run with `AW2_SHADER_BACKEND=helper` so converter crashes
   surface cleanly as `Error::Helper`.
3. **Check root signatures, stage-in layouts & caps:** Check `src/root_signature.rs`, `ISG1`/`ISGN`
   vertex input matching (`d3d12-metal/src/metal/pipeline.rs`), and SM 6.8/6.9 lowering
   (`src/bitcode.rs`; `caps.rs` caps `HighestShaderModel` at `0x66`).

## Verification Checklist

```sh
cargo test -p dxil-metal --target aarch64-apple-darwin -- --test-threads=1
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- misc::d3dcompiler_47 --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- pso_manifest --test-threads=1
cargo opsdb -- shader-formats --sync && cargo opsdb -- doctor
cargo runner-build
```

See `docs/SHADERS.md` and `crates/dxil-metal/tools/README.md` for background.
