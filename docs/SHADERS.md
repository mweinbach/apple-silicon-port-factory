# Shader Pipeline (`crates/dxil-metal`) & Metal 4 PSO Cache

`crates/dxil-metal` translates Windows GPU shaders into Apple Metal `.metallib` libraries and
caches compiled Metal 4 pipeline archives (`MTL4Archive`) on disk. Its default build
(`features = ["metal"]`) is 100% game-agnostic and has no dependency on `aw2-data`; the optional
`tools` feature adds the Northlight `.binrfx` prewarm CLI for Alan Wake 2.

---

## 1. Supported Shader Inputs

| Input format | Path to Metal | Runtime dependency |
|---|---|---|
| **DXIL (SM 6.0–6.6, plus SM 6.8/6.9 lowering)** | `DXBC(DXIL)` → Apple Metal Shader Converter (`libmetalirconverter.dylib` 4.0.1) → `.metallib` | `~/.local/opt/metal-shaderconverter` (install via `./scripts/install_metal_shader_converter.sh`, override `METAL_IRCONVERTER_DYLIB`) |
| **Legacy FXC DXBC (SM 4.0 / 5.x)** | `DXBC(SHEX/SHDR)` → `libdxilconv.dylib` → signed DXIL 1.0 → Metal Shader Converter → `.metallib` | Optional `libdxilconv.dylib` (build via `crates/dxil-metal/tools/dxilconv/build.sh`, override `AW2_DXILCONV`). Cache keys use the original DXBC digest. |
| **Vulkan SPIR-V (`.spv`) & WebGPU WGSL (`.wgsl`)** | `naga` (`spv-in`/`wgsl-in` → IR → `msl-out`) → `.metal` → `xcrun -sdk macosx metal` + `metallib` → `.metallib` | `naga` is built in. Offline `.metallib` emission requires Xcode's Metal Toolchain; when absent, returns honest `Error::Load` (never fake `MTLB` bytes). Live Vulkan apps forward SPIR-V directly to MoltenVK. |

---

## 2. How the Pipeline Works

1. **Container & bitcode inspection (`src/dxbc.rs`, `src/bitcode.rs`, `src/dxilconv.rs`)**
   - Parses DXBC chunks (`DXIL`, `RTS0`, `SFI0`, `PSV0`, `RDAT`, `HASH`, `ISG1`/`ISGN`,
     `OSG1`/`OSGN`, `SHEX`/`SHDR`).
   - Extracts entry points from `PSV0` (validator 1.8+) or LLVM bitcode function definitions, DXR
     library exports from `RDAT`, and wave-size hints (`MinimumExpectedWaveLaneCount` /
     `MaximumExpectedWaveLaneCount`) from `PSV0`.
   - Converts SM 4/5 containers transparently via `libdxilconv.dylib` when present.

2. **Root signatures (`src/root_signature.rs`)**
   - Parses and serializes D3D12 Root Signatures (`1.0` and `1.1`) and lowers them to
     `IRVersionedRootSignatureDescriptor` for top-level argument buffers.
   - Can synthesize covering root signatures (`RootSignatureDesc::covering_dxil` /
     `covering_bindings`) from `PSV0`/`RDAT` metadata for offline prewarming and standalone tests.

3. **Compilation, out-of-process worker & portable Tier-1 disk cache (`src/compiler.rs`, `src/cache.rs`, `src/ffi.rs`, `src/bin/dxil-metald.rs`)**
   - Calls `IRCompilerAllocCompileAndLink` (or `IRCompilerAllocCombineCompileAndLink` for fused DXR
     hit groups).
   - For crash isolation or x86-64 Rosetta runs (where `libmetalirconverter.dylib` is arm64-only),
     `dxil-metal` spawns the native arm64 worker binary `dxil-metald`
     (`cargo build --release -p dxil-metal --bin dxil-metald`, override `PORT_SHADERD` /
     `AW2_SHADERD`).
   - Fingerprints `libmetalirconverter.dylib` by its immutable Mach-O `LC_UUID` (with automatic
     prefix fallback to legacy `mtime`-suffixed directories), so prewarmed `.aw2s` `.metallib`
     entries stay valid when copied into `.app/Contents/Frameworks/` or moved to a new Mac.
   - Reads optional read-only bundled seed caches (`PORT_SHADER_CACHE_FALLBACK` /
     `Contents/Resources/ShaderCache`) while writing new entries to the writable user cache
     (`PORT_SHADER_CACHE` / `AW2_SHADER_CACHE`, default `~/Translator/ShaderCache`).

4. **Metal 4 PSO Archive Cache & Cold-Device Prewarming (`src/pso_cache.rs`, `crates/d3d12-metal/src/metal/pso_manifest.rs`)**
   - Wraps `MTL4Compiler` and `MTL4Archive` (`PORT_PSO_CACHE` / `AW2_PSO_CACHE`, default
     `<shader cache>/pso/<gpu>-<hash(MTLDevice.name | macOS build)>/`) so compiled
     `MTLComputePipelineState` and `MTLRenderPipelineState` binaries persist per GPU + macOS build
     with automatic compaction, corrupt-archive recovery, periodic `Present` flushes (D3D12 &
     D3D11), and guaranteed flush on `ExitProcess`.
   - Records GPU- and OS-independent Compute/Graphics/Mesh PSO recipes into `pso_manifest/records.p2m`
     (with optional read-only bundled fallback `PORT_PSO_MANIFEST_FALLBACK`).
   - When launched on a **new Mac or after a macOS update** (`is_cold_device() == true`, 0 loaded
     `.mtl4a` archives for the current GPU+OS), `port-runner` automatically precompiles recorded
     Tier-1 shaders and Metal 4 PSOs in parallel into the new device's `MTL4Archive` before
     entering the game (`PORT_PREWARM_SHADERS=auto|force|0`, or run explicitly via
     `port-runner --prewarm-shaders`).

5. **Function-constant specialization (`crates/d3d12-metal/src/metal/pipeline.rs`)**
   - When the converter emits Metal function constants (`reflection.needs_function_constants`, e.g.
     from `dual_source_blending = DecideAtRuntime` or `depth_feedback = DecideAtRuntime`),
     `d3d12-metal` automatically wraps `MTL4LibraryFunctionDescriptor` in
     `MTL4SpecializedFunctionDescriptor` across compute, graphics, and mesh pipelines.

6. **Opt-in Metal 4.1 / MSL 4.1 compiler controls (`crates/dxil-metal/src/{pso_cache,spirv_metal}.rs`)**
   - `PORT_METAL41_RTZ_FP` (follows `PORT_METAL41`): enables MSL 4.1
     `MTLFloatingPointConversionRoundingModeTowardZero` / `-fmetal-rtz-fp-conversion` on runtime
     MSL source compiles and `spirv_metal` compilations so FP32→FP16 narrowing saturates to
     `±65504` instead of overflowing to `±Inf`.
   - `PORT_METAL41_SPIRV` (follows `PORT_METAL41`): upgrades `spirv_metal` (`naga` SPIR-V/WGSL →
     MSL → `xcrun metal`) from legacy `-std=metal3.1` to `naga` `(4, 0)` + `-std=metal4.1`
     (falling back to `metal4.0` → `metal3.1` on older toolchains) with separated disk cache keys
     so default caches remain unaffected.

---

## 3. CLI & Prewarming

- **Universal shader inspection & conversion (`port-factory`):**
  ```sh
  cargo opsdb -- shader-inspect path/to/shader.dxil
  cargo opsdb -- shader-convert path/to/shader.spv --msl-out out.metal --out out.metallib
  cargo opsdb -- precompile-shaders /path/to/game/
  ```
- **Alan Wake 2 `.binrfx` corpus prewarming (`dxil-metal --features tools`):**
  ```sh
  cargo run --release -p dxil-metal --features tools --bin dxil-metal -- prewarm [--manifest]
  ```
- **Build `libdxilconv.dylib` & test fixtures:** see [crates/dxil-metal/tools/README.md](../crates/dxil-metal/tools/README.md).

---

## 4. Benchmark Validation (Alan Wake 2 Corpus)

Alan Wake 2's shader corpus is the main stress test for `dxil-metal` and `d3d12-metal`:

- **Conversion:** **5,052 / 5,052 (100%)** shaders across all 153 `.binrfx` bundles and PE
  binaries convert cleanly (including SM 6.8/6.9 raygen lowering and SM 5.0 BinkGPU shaders via
  `libdxilconv`).
- **Active Apple Silicon GPU PSO compilation (`crates/d3d12-metal/tests/gpu_rfx_psos.rs`):**
  **3,311 / 3,311 (100%)** active shader exports (**3,216** unique compiled keys; Wave64-only
  permutations and SM 6.9 SER raygens are intentionally skipped because `caps.rs` caps
  `HighestShaderModel` at `0x66`) compile on the Metal 4 GPU compiler with **0 failures**:
  - 1,895 Compute PSOs
  - 152 Vertex PSOs
  - 41 Mesh PSOs
  - 576 Fragment PSOs (with `ISG1`/`ISGN`-matched vertex stubs and function-constant specialization)
  - 552 DXR Library exports
- **DXR StateObjects (`crates/d3d12-metal/tests/gpu_dispatch_rays.rs`):** 19 DXR `StateObject`
  pipelines (61 shader identifiers) and 28 `RayQuery` compute PSOs across all 24 `rt_*.binrfx`
  bundles compile and run on Metal 4.
