# Roadmap

This project moved through two eras: first proving we could run Alan Wake 2 natively on Apple
Silicon without Wine or D3DMetal, and then pulling the reusable machinery out into a universal
Windows-to-macOS port factory.

Every active milestone below has a concrete exit check.

Related docs: [End Goal](END_GOAL.md) · [What's Needed](WHATS_NEEDED.md) ·
[What's Next](WHATS_NEXT.md) · [ToDos](TODOS.md) · [Status](STATUS.md)

---

## Completed Foundation

### Phase 1 — Alan Wake 2 Bootstrap (M0–M3) ✅
- **M0 (Tooling):** Cargo workspace, `fndb` function database (`.pdata`, RTTI vftables,
  `__FUNCSIG__`), Ghidra headless scripts, pinned `third_party/` references.
- **M1 (PE Loader):** PE32+ mapping, relocations, imports & delay-loads, static TLS, TEB/PEB,
  reaching `WinMain` with all CRT constructors and worker threads alive.
- **M2 (Windowing & Input):** Cocoa main-thread `NSWindow` + `CAMetalLayer`, Win32 message pump,
  raw mouse/keyboard, XInput via GameController + CoreHaptics, DualSense via `IOHIDManager` +
  SetupDi.
- **M3 (Metal 4 First Frame):** Real fake-PE export directories for builtin DLLs, Metal 4 D3D12
  device/queues/fences/command lists/PSOs, DXIL → Metal Shader Converter + `MTL4Archive` cache,
  booting Alan Wake 2 through setup and rendering the 3D Main Menu.
- **DXR, Mesh & Audio:** DXR 1.1 BLAS/TLAS/RayQuery/`DispatchRays` and mesh shaders verified on
  GPU; WASAPI → CoreAudio shim verified sample-exact; `wwise-audio` native Wwise 2022.1 runtime
  implemented across 21 DSP plugins.

### Phase 2 — Universal Port Factory Extraction ✅
- **De-branded universal core (`crates/`):** Ten game-agnostic crates (`win64-abi`, `pe-image`,
  `x86-runtime`, `win32-rt`, `dxil-metal`, `d3d12-metal`, `d3d12-replay`, `wwise-audio`,
  `port-factory`, `port-loader`) with zero Alan Wake 2 code or `aw2-data` in default builds.
- **Thin universal runner (`port-runner`):** ~17 MB ARM64 binary (~15s build) with declarative
  `games/<id>/game.toml` manifests and offline `.aot` block-cache prewarming (`--prewarm-aot`).
- **Clean boundaries:** `objc2-metal` completely removed from `win32-rt`; native
  `MTL4FXSpatialScaler` MetalFX upscaling wired in `d3d12-metal`; Alan Wake 2 isolated under
  `games/alan-wake-2/`.
- **Multi-API graphics:** Real Direct3D 11.0/11.1 runtime in `win32-rt/src/d3d11` (backed by
  `d3d12-metal` + `dxil-metal`) and dynamic Vulkan → `libMoltenVK.dylib` bridge in
  `win32-rt/src/misc/vulkan_1.rs`.

---

## Active Tracks

### Track A — Real Multi-Game Bring-Up (Universal `port-runner`)
Proving the universal runtime on real Windows games beyond Alan Wake 2 using only `game.toml`,
offline `.aot` prewarming, and general Win32/graphics fixes.

- Bring up at least one D3D11 title, one D3D12 title (non-Northlight), and one Vulkan title
  (with MoltenVK installed) through `port-runner --manifest games/<id>/game.toml`.
- Close real Win32 / UCRT / COM / DirectInput / Steam API gaps discovered during bring-up, adding
  unit tests and syncing `db/win32_apis.tsv` as we go.
- Harden `x86-runtime`'s ARM64 block JIT and `.aot` cache for common Windows idioms (AVX2/FMA
  patterns, indirect jump tables, TLS/SEH edge cases, dynamic codegen hooks).
- **Exit:** Three distinct engines (D3D12, D3D11, Vulkan) boot to rendered gameplay frames on
  `port-runner` with zero game-specific Rust code in `crates/`.

### Track B — Graphics & Upscaling Hardening (`d3d12-metal`, `d3d11`, `vulkan_1`, `dxil-metal`)
- **D3D11:** Exercise deferred contexts, dynamic constant buffers, mip generation, cubemaps,
  depth/stencil edge cases, and SM 4/5 shader variants on real D3D11 games; add GPU tests for each
  bug fixed.
- **Vulkan:** Test `vulkan_1.rs` end-to-end against a real `libMoltenVK.dylib` install with a
  shipping Vulkan app (swapchain resize, validation-clean surface creation, dynamic proc lookup).
- **D3D12 & MetalFX:** Verify `MTL4FXSpatialScaler` on live upscaling workloads (including FSR /
  DLSS / XeSS input/output resource patterns), expand temporal upscaling evaluation if MetalFX
  temporal inputs (depth + motion vectors + jitter) are available cleanly, and keep barrier
  coalescing sound across all engines.
- **Shaders:** Smooth out `dxil-metald` worker lifecycle, `libdxilconv` distribution inside
  packaged `.app` bundles, and SPIR-V/WGSL compilation when Xcode Metal Toolchain is present.
- **Exit:** Zero Metal validation layer errors across all D3D12, D3D11, and MetalFX GPU tests,
  plus verified live Vulkan rendering through MoltenVK.

### Track C — Packaging & Developer Experience (`port-factory`)
- Make `cargo opsdb -- steam-convert <game> --format app` produce a reliable self-contained macOS
  `.app` bundle (with `port-runner`, `dxil-metald`, `libmetalirconverter.dylib`, optional
  `libdxilconv.dylib`, `game.toml`, and prewarmed `AotCache`).
- Keep `cargo opsdb -- doctor` catching stale `db/*.tsv` files or broken assumptions early.
- **Exit:** One command converts an installed Windows game directory into a double-clickable `.app`
  that boots straight into prewarmed ARM64 code.

### Track D — Alan Wake 2 Playable Slice (`games/alan-wake-2/`)
- Diagnose and fix the Cauldron Lake movement/camera drop issue observed in Return 0: The Cult.
- Play through Return 0 and Chapter 1 start-to-finish on native ARM64 with MetalFX upscaling and
  either CoreAudio WASAPI or `AW2_AUDIO=native`, with save/load working cleanly under `~/Translator`.
- Retire remaining A3/A4 audio parity rough edges documented in [docs/AUDIO.md](AUDIO.md) as needed
  for clean gameplay audio.
- **Exit:** First chapter playable start-to-finish on native ARM64 with audio, input, and saves
  intact.
