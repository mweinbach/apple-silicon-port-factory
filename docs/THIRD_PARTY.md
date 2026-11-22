# Third-Party Tools, Optional References & Middleware

No Wine, CrossOver, or D3DMetal code runs at runtime. External components fall into three groups:
universal runtime/build libraries, optional shallow clones in the gitignored `third_party/` folder
used as API/format references (no git submodules), and proprietary middleware shipped by games.

Fetch or refresh optional references with `./scripts/third_party.sh` (default fetches `directx-headers`
and `vgmstream`; pass `--all` for all 14 reference repos).

---

## 1. Universal Runtime & Build Tools (Host-Installed)

| Component | Version | How it's used | Install |
|---|---|---|---|
| **Apple Metal Shader Converter** (`libmetalirconverter.dylib`) | 4.0.1 | Core DXIL → Metal AIR translator used by `crates/dxil-metal` (required for D3D12/D3D11 shaders). | `./scripts/install_metal_shader_converter.sh` → `~/.local/opt/metal-shaderconverter` (override `METAL_IRCONVERTER_DYLIB`) |
| **dxilconv** (`libdxilconv.dylib`) | Pinned DXC commit (NCSA) | Optional SM 4/5 FXC DXBC → DXIL converter used by `crates/dxil-metal` and D3D11. Built from open-source Microsoft DXC with a small macOS build patch. | `crates/dxil-metal/tools/dxilconv/build.sh` → `~/.local/lib` (override `AW2_DXILCONV`) |
| **MoltenVK** (`libMoltenVK.dylib`) | Homebrew / Vulkan SDK | Optional runtime backend dynamically `dlopen`ed by `crates/win32-rt/src/misc/vulkan_1.rs` for Windows Vulkan games. | `brew install molten-vk` (override `PORT_VULKAN_DYLIB`) |
| **Xcode Metal Toolchain** (`xcrun -sdk macosx metal`) | Matching Xcode | Optional offline compiler for turning `naga`-generated MSL (from SPIR-V/WGSL) into `.metallib` files. | Xcode / `xcodebuild -downloadComponent MetalToolchain` |

---

## 2. Optional Reference Checkouts (`third_party/`, gitignored)

Fetched via `./scripts/third_party.sh` (plain `git clone --depth 1`, no git submodules):

### Universal API & format references
| Reference | Upstream | Role |
|---|---|---|
| `directx-headers` | `microsoft/DirectX-Headers` | Authoritative D3D12/DXGI COM vtable layouts, structs, and GUIDs for `win32-rt` and `d3d12-metal`. |
| `faudio` | `FNA-XNA/FAudio` | Reference for XAudio2 semantics (today `xaudio2_9redist.dll` runs on our CoreAudio WASAPI shim). |
| `vgmstream` | `vgmstream/vgmstream` | Wwise `.wem` Vorbis codebook tables (consumed at build time by `crates/wwise-audio/build.rs`) and decoder reference. |
| `wwiser` | `bnnm/wwiser` | Wwise 2022.1 (v145) `.bnk` HIRC layout reference for `crates/wwise-audio`. |

### Alan Wake 2 / engine middleware references
Pinned to match the versions shipped by Alan Wake 2 v1.2.10:

| Reference | Upstream | Pin | Role |
|---|---|---|---|
| `fidelityfx-sdk` | `GPUOpen-LibrariesAndSDKs/FidelityFX-SDK` | `main` | ABI layout reference for `amd_fidelityfx_loader_dx12.rs` (which routes upscaling into native MetalFX). |
| `streamline` | `NVIDIA-RTX/Streamline` | `v2.7.2` | ABI reference for stubbing `sl.interposer.dll` cleanly. |
| `physx-3.4` | `NVIDIAGameWorks/PhysX-3.4` | 3.4.2 + APEX 1.4.1 | Struct/ABI reference for PhysX middleware. |
| `nvcloth` | `NVIDIAGameWorks/NvCloth` | 1.1.6 | Cloth simulation ABI reference. |
| `nrd` / `nri` | `NVIDIA-RTX/NRD`, `NRI` | v4.3.6 / v1.99 | RT denoiser shader & render interface reference. |
| `nv_cluster_builder` | `nvpro-samples/nv_cluster_builder` | `main` | Meshlet cluster builder reference. |
| `luau` | `luau-lang/luau` | `master` | Game scripting VM reference (`Luau::` in AW2). |
| `mimalloc` | `microsoft/mimalloc` | v2.1.2 | Engine allocator struct reference (`mi_*` in AW2). |
| `recastnavigation` | `recastnavigation/recastnavigation` | `main` | Reference for navigation mesh replacement. |

---

## 3. Proprietary Middleware Handled at Runtime

| Middleware | How we handle it |
|---|---|
| **Audiokinetic Wwise 2022.1** | Runs as original guest code over our CoreAudio WASAPI shim for any game, or natively via `crates/wwise-audio` (`AW2_AUDIO=native` for Alan Wake 2). |
| **Compression DLLs (`oo2core_*`, `zstd`, `lz4`)** | Let shipping PE DLLs execute natively through `x86-runtime`'s ARM64 block JIT unless a real, tested native implementation exists — never stub them with `memcpy`. |
| **Coherent Gameface (`cohtml`, `RenoirCore`, `v8`)** | Runs through `x86-runtime` + `d3d12-metal` (V8 with `--jitless` when configured via `game.toml` to avoid guest RWX codegen). |
| **Bink 2 (`bink2w64.dll`)** | Runs through `x86-runtime`; BinkGPU SM 5.0 shaders convert via `libdxilconv` → Metal 4, and Bink audio runs via `xaudio2_9redist.dll` on our WASAPI shim. |
| **Vendor upscalers & anti-tamper/SDKs (`sl.interposer`, `libxess`, EOS, Razer Chroma)** | Honest stubs or native redirects (`amd_fidelityfx_loader_dx12` routes upscaling to Apple `MTL4FXSpatialScaler` MetalFX). |
