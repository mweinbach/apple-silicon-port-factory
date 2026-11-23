# Shader Caches, `d3dcompiler_47` & Worker Daemon Reference

## 1. Two-Tier Cache + `PsoManifest` Directory Layout

| Layer | Env Var (Writable / Read-Only Seed) | Default Location | Portability & Contents |
|---|---|---|---|
| **Tier 1 (`.metallib` AIR)** | `PORT_SHADER_CACHE` (`AW2_SHADER_CACHE`) / `PORT_SHADER_CACHE_FALLBACK` | `~/Translator/ShaderCache/irX.Y.Z-uuid-<LC_UUID>/` | **Portable across Apple Silicon Macs & iOS.** Keyed by content digest + immutable `codesign`-invariant Mach-O `LC_UUID` of `libmetalirconverter.dylib` (with automatic legacy `irX.Y.Z-*` fallback). On iOS runs as `BackendKind::CacheOnly`. |
| **Tier 2 (`MTL4Archive` AGX)** | `PORT_PSO_CACHE` (`AW2_PSO_CACHE`) | `<shader cache>/pso/<gpu>-<hash(MTLDevice.name \| macOS build)>/` | **Host-specific AGX machine code — NEVER bundle `*.mtl4a` into `.app`/`.ipa`!** Flushed periodically on D3D12/D3D11 `Present` and guaranteed on `ExitProcess`. Corrupt archives auto-recover. |
| **PSO Recipes (`records.p2m`)** | `PORT_PSO_MANIFEST` / `PORT_PSO_MANIFEST_FALLBACK` | `<pso cache>/pso_manifest/records.p2m` | **Portable GPU/OS-independent Compute/Graphics/Mesh PSO recipes** (`d3d12-metal/src/metal/pso_manifest.rs`). Drives parallel cold-device prewarming (`PORT_PREWARM_SHADERS=auto\|force\|0` or `--prewarm-shaders`) with a native AppKit progress window on interactive cold launches. |

## 2. Runtime HLSL Compilation & Reflection (`crates/win32-rt/src/misc/d3dcompiler_47.rs`)

Games that compile or reflect shaders at runtime call `d3dcompiler_47.dll` (also aliased for `_43`
and `_46`), which is now a full native implementation (zero `stub!` calls, works on macOS & iOS):

- **`D3DReflect` (pure Rust, macOS & iOS):** Parses `RDEF` (SM 4.x, 5.0 `RD11`, 5.1 `0x25441313`
  with `Space`/`uID`), `ISGN`/`ISG1`, `OSGN`/`OSG5`/`OSG1`, `PCSG`/`PSG1`, FXC `STAT`, `SHEX`/`SHDR`,
  `SFI0`, and `DXIL`/`PSV0`/`RDAT`. Implements ABI-accurate `ID3D12ShaderReflection` &
  `ID3D11ShaderReflection` (including `d3dcompiler_43` vs `47` struct sizing) and Windows-compatible
  invalid sentinel objects (`GetDesc` → `E_FAIL`).
- **`D3DPreprocess` (pure Rust):** Line splicing, comments, `#define`/`#undef`, `#if`/`#elif`
  expressions, `#pragma once`, `#error`, guest `ID3DInclude::Open`/`Close` callbacks (via
  `call_guest8`), and `D3D_COMPILE_STANDARD_FILE_INCLUDE`.
- **`D3DCompile` / `D3DCompile2`:** Preprocesses HLSL, auto-upgrades SM 4/5 targets (`vs_5_0`, etc.)
  to SM 6.0 for DXC, and on macOS dynamically loads `libdxcompiler.dylib` (`DXC_DYLIB`,
  `~/.local/lib/libdxcompiler.dylib`, Homebrew) via `IDxcCompiler3` (with `dxc` CLI fallback), then
  re-signs DXBC+DXIL containers with retail MD5 and returns real diagnostic `ID3DBlob`s.
- **`D3DGetBlobPart` / `D3DSetBlobPart` / `D3DStripShader` / `D3DDisassemble`:** Full part
  extraction/replacement with retail MD5 re-signing and structured disassembly text.

## 3. Out-of-Process `dxil-metald` Worker (`crates/dxil-metal/src/{bin/dxil-metald.rs,helper.rs}`)

- Build: `cargo build --release -p dxil-metal --bin dxil-metald`
- Enable explicitly: `AW2_SHADER_BACKEND=helper` (override path with `PORT_SHADERD` / `AW2_SHADERD`).
- Parent-death safety: spawns a macOS `kqueue(EVFILT_PROC | NOTE_EXIT)` watchdog thread + signal-safe
  `SIGTERM`/`SIGINT` exit + global `atexit`/`Drop` worker reaping so **0 stray workers** survive
  even after parent `SIGKILL` (verified by `crates/dxil-metal/tests/worker_lifecycle.rs`).

## 4. Converter & Toolchain Setup

| Tool | Install / Build Command | Override Env Var |
|---|---|---|
| Metal Shader Converter 4.0.1 (`libmetalirconverter.dylib`) | `./scripts/install_metal_shader_converter.sh` (`~/.local/opt/metal-shaderconverter`) | `METAL_IRCONVERTER_DYLIB` |
| FXC SM 4/5 → DXIL (`libdxilconv.dylib`) | `crates/dxil-metal/tools/dxilconv/build.sh` (`~/.local/lib`) | `AW2_DXILCONV` |
| DXC (`libdxcompiler.dylib`, runtime `D3DCompile`) | Built alongside DirectXShaderCompiler or Homebrew | `DXC_DYLIB` |
| Xcode Metal Toolchain (offline SPIR-V/WGSL → `.metallib`) | `xcodebuild -downloadComponent MetalToolchain` | — |
