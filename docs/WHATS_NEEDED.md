# What's Needed

Two things stand between where the repo is today and the [End Goal](END_GOAL.md):
1. **Host tools & libraries** needed on your Mac to build and exercise every backend.
2. **Codebase gaps** that still need real engineering work.

Related docs: [End Goal](END_GOAL.md) · [What's Next](WHATS_NEXT.md) · [ToDos](TODOS.md) ·
[Status](STATUS.md) · [Third-Party Dependencies](THIRD_PARTY.md)

---

## 1. Host Machine Prerequisites

### Required for the universal factory
- **Apple Silicon Mac** running macOS with **Metal 4** support and Xcode Command Line Tools.
- **Rust toolchain** with `aarch64-apple-darwin` target (`rustup target add aarch64-apple-darwin`).
- **Apple Metal Shader Converter 4.0.1** (`libmetalirconverter.dylib`):
  ```sh
  ./scripts/setup.sh
  # or directly:
  ./scripts/install_metal_shader_converter.sh
  ```
  Installs to `~/.local/opt/metal-shaderconverter` (override with `METAL_IRCONVERTER_DYLIB`).
- **Git LFS** (required to pull `db/*.tsv` catalogs):
  ```sh
  brew install git-lfs && git lfs install && git lfs pull
  ```
- **Optional `third_party/` reference repos** (no git submodules; `vgmstream` is used at build time
  by `wwise-audio` for Wwise Vorbis codebook tables, and `directx-headers` is used when regenerating
  D3D12 vtables):
  ```sh
  ./scripts/third_party.sh        # minimal: directx-headers + vgmstream (~35 MB)
  ```
- **Your own legally owned Windows x86-64 game(s)** on disk (Steam, Epic/Heroic, Battle.net, GOG, or standalone).

### Optional host libraries (needed for specific backends)
| Backend / Feature | What you need | How to get it |
|---|---|---|
| **Direct3D 11 & SM 4/5 shaders** | `libdxilconv.dylib` | `crates/dxil-metal/tools/dxilconv/build.sh` (installs to `~/.local/lib`, override `AW2_DXILCONV`) |
| **Live Vulkan games** | `libMoltenVK.dylib` | `brew install molten-vk` (override `PORT_VULKAN_DYLIB`) |
| **Offline SPIR-V / WGSL → `.metallib`** | Xcode Metal Toolchain (`xcrun -sdk macosx metal`) | `xcodebuild -downloadComponent MetalToolchain` |
| **Out-of-process shader worker** | `dxil-metald` binary | `cargo build --release -p dxil-metal --bin dxil-metald` |
| **Alan Wake 2 reverse engineering** | Ghidra + OpenJDK 21, optional Rosetta `x86_64-apple-darwin` target | `brew install ghidra` and `rustup target add x86_64-apple-darwin` |

---

## 2. Engineering Gaps Still Needed in the Codebase

Everything below is real technical work still missing or unhardened. Individual work items are
tracked in [docs/TODOS.md](TODOS.md).

### Real multi-game runtime validation (biggest gap right now)
- Almost all end-to-end game hours so far have been on Alan Wake 2 and standalone DLLs (`zlib1.dll`).
- We need real bring-up runs on **non-Northlight games** (D3D12, D3D11, and Vulkan) using pure
  `port-runner` + `game.toml` to flush out real-world Win32, CRT, COM, JIT, and GPU assumptions
  that unit tests cannot catch.

### Direct3D 11 completeness (`crates/win32-rt/src/d3d11/`)
Our new D3D11-on-Metal 4 runtime passes 10 unit/GPU tests (including SM 5.0 compute + VS/PS draw),
but several methods used by real D3D11 games are still honest stubs (`E_NOTIMPL` / `stub!`):
- `ID3D11DeviceContext::GenerateMips`
- Indirect draws & dispatches (`DrawInstancedIndirect`, `DrawIndexedInstancedIndirect`,
  `DispatchIndirect`, `CopyStructureCount`, `DrawAuto`)
- `IDXGISurface::Map` / `IDXGISurface1::GetDC` and `OpenSharedResource*` (used by video/UI
  interop layers)
- Geometry / Hull / Domain shader stages (if target D3D11 games require them; Metal has no native
  GS, so any GS support would require mesh/compute lowering or an explicit compatibility boundary)
- Real-game edge cases: cubemaps, 2D texture arrays, depth-stencil readback, dynamic constant
  buffer ring pressure, and pipeline statistics queries.

### Vulkan live verification (`crates/win32-rt/src/misc/vulkan_1.rs`)
- Unit tests and mock-dylib E2E tests pass, but `vulkan_1.rs` still needs **live validation against
  real `libMoltenVK.dylib`** running a real Windows Vulkan binary (surface creation, swapchain
  recreation on window resize, dynamic device proc dispatch through `x86-runtime`, and LunarG
  validation layers).

### D3D12 & MetalFX (`crates/d3d12-metal/`)
- `MTL4FXSpatialScaler` is live-verified on Metal 4 GPU tests, and needs live in-game verification
  across FSR / DLSS / XeSS upscale calls (frame pacing, HDR tonemap handoff, resize stability).
- **Temporal MetalFX (`MTL4FXTemporalScaler`):** Not implemented yet; worth evaluating once
  spatial scaling is proven in gameplay and we have clean depth + motion vector + jitter plumbing.
- Multi-engine D3D12 barrier/residency validation on non-Northlight engines (UE5, Unity, Godot,
  proprietary engines).

### CPU execution & `.aot` precompilation (`crates/x86-runtime/`)
- **Multi-threaded offline `.aot` prewarming:** Large `.exe` files with tens of thousands of
  `.pdata` functions should prewarm quickly across Apple Silicon cores.
- **Remaining interpreter-only instructions:** x87 floating-point and rare legacy/AVX2 ops still run
  through `GENERIC_INTERP`; hot ones encountered in real games should move into `JIT_ARM64`.
- **Switch/indirect jump-table discovery** during offline `.aot` prewarm so CFG closures cover
  more blocks before first run without waiting for a runtime profile.
- **Dynamic codegen / JIT-ed guest runtimes** (Mono, CoreCLR, V8, LuaJIT): need clean detection and
  explicit policy (`v8_jitless`, `.NET` ReadyToRun stripping, or honest W^X handling).

### Win32, Input & Middleware (`crates/win32-rt/`)
- **Missing exports uncovered by new games:** Networking (`winhttp`, `wininet`, `ws2_32` edge
  cases), media (`mfplat` / `mfreadwrite`), `dinput8` real gamepad enumeration (currently returns
  zero devices; fine for XInput games, needed for older DirectInput-only titles), Steamworks
  (`steam_api64.dll` runtime gaps), and COM/OLE interfaces used by launchers.
- **Input & display polish:** Non-US keyboard layouts in `ToUnicode`, mixed-DPI multi-monitor
  scaling, clipboard payload transfer, and DualSense USB audio-haptics container pairing.
- **Audio capture endpoints:** WASAPI currently implements render endpoints only; games with voice
  chat / mic probing may expect a capture endpoint (even if silent or CoreAudio-backed).

### Packaging & distribution workflow (`crates/port-factory/`)
- Bundle `libdxilconv.dylib` (when built) and `libmetalirconverter.dylib` automatically into
  converted `.app` bundles under `Contents/Frameworks/` so double-clicked apps don't rely on
  developer paths.
- Run `port-factory steam-convert --format app` end-to-end on a real installed game and verify cold
  launch from Finder.

### Alan Wake 2 specific (`games/alan-wake-2/`)
- Root-cause and fix the Cauldron Lake terrain/camera drop seen in Return 0: The Cult (logging
  camera position/direction alongside player height and PhysX ground hits across movement input).
- Finish the remaining small `wwise-audio` A3 parity items if needed for `AW2_AUDIO=native`
  (bus `1134869537` gain offset, fade-in→fade-out handoff, Convolution Reverb IR media format).
