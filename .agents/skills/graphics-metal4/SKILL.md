---
name: graphics-metal4
description: Develop, extend, or debug Direct3D 12, Direct3D 11, Vulkan-on-MoltenVK, Metal 4 pipelines/command lists/DXR/mesh/geometry shaders, MetalFX Spatial/Temporal upscaling, and offline D3D12 trace replay across crates/d3d12-metal, crates/win32-rt/src/{d3d12,dxgi,d3d11,misc/vulkan_1.rs}, and crates/d3d12-replay. Use when fixing GPU rendering bugs, Metal validation errors, swapchain/resize defects, missing D3D12/D3D11/Vulkan APIs, or upscaler issues.
---

# Graphics Stack: Metal 4, D3D12, D3D11, Vulkan & MetalFX

## Architectural Boundaries (Non-Negotiable)

1. **NEVER import `objc2-metal` inside `crates/win32-rt`.**
   All Metal 4 and MetalFX calls live inside `crates/d3d12-metal` (plus `MTL4Archive` PSO caching
   in `crates/dxil-metal`). Both `win32-rt/src/d3d12/` and `win32-rt/src/d3d11/` call safe Rust
   types in `d3d12-metal`; `win32-rt/src/misc/vulkan_1.rs` dynamically `dlopen`s `libMoltenVK.dylib`.
2. **Keep barriers conservative by default.** Engine-specific barrier shortcuts (such as
   `AW2_IB_BINDLESS_WAR`) must stay **OFF by default** so hazard ordering stays sound across engines.
3. **Never substitute game shaders or fake GPU passes.** Unsupported stages/APIs must log via
   `stub!` and return documented Windows/Vulkan failure codes (`E_NOTIMPL`, `DXGI_ERROR_UNSUPPORTED`,
   `VK_ERROR_INCOMPATIBLE_DRIVER`).
4. **Apple AGX counter-heap ceiling:** Apple Silicon enforces ~31–32 `MTL4CounterHeap` instances
   per process — always pool timestamp queries (as in D3D11's 4096-slot heap and `passtiming.rs`).

## Subsystem Map

| Layer | Key Files | Key Notes |
|---|---|---|
| **D3D12 → Metal 4** | `crates/d3d12-metal/src/{metal/*,mesh.rs,rt/*}` & `crates/win32-rt/src/{d3d12,dxgi}/` | `MTL4CommandQueue`, `MTLResidencySet`, 24-byte `IRDescriptorTableEntry`, planar `D32_FLOAT_S8X24_UINT` companion `R8Uint` stencil, DXR 1.1 BLAS/TLAS/RayQuery/`DispatchRays`, Mesh & Geometry Shader emulation (`MTL4MeshRenderPipelineDescriptor`), GPU ICB `ExecuteIndirect`. Regenerate D3D12 COM vtable slot lists via `python3 crates/win32-rt/src/d3d12/gen_vtables.py > crates/win32-rt/src/d3d12/vtables.rs`. |
| **MetalFX Upscaling** | `crates/d3d12-metal/src/metal/metalfx.rs` | `dlopen`s `MetalFX.framework` (no hard link; works on iOS Sim without it). Fallback chain: `MTL4FXTemporalScaler` → `MTL4FXSpatialScaler` → bilinear compute (`port_upscale_2d_f`). Uses `Linear` color processing on float HDR formats (`RGBA16F`, `RG11B10F`, `RGBA32F`). Knobs: `PORT_METALFX=0` (bilinear), `PORT_METALFX_TEMPORAL=0` (spatial). |
| **D3D11 → Metal 4** | `crates/win32-rt/src/d3d11/{device,context,resources,state,swapchain,com,types,mod}.rs` | Real D3D11.0/11.1 COM runtime (immediate + deferred contexts, `IDXGIDevice3`, `IDXGISurface2` `Map`/`Unmap`, `OpenSharedResource*`, `GenerateMips`, real `ID3D11GeometryShader` on Metal 4, pooled `MTL4CounterHeap` timestamps, indirect draw/dispatch + UAV counters). Hull, domain, and stream-out GS honestly return `E_NOTIMPL`. Opt out via `PORT_D3D11=0`. |
| **Vulkan → MoltenVK** | `crates/win32-rt/src/misc/vulkan_1.rs` | Dynamically loads `libMoltenVK.dylib` (`PORT_VULKAN_DYLIB`, bundled `Contents/Frameworks/`, Homebrew, `$VULKAN_SDK`). Rewrites `VK_KHR_win32_surface` → `VK_EXT_metal_surface` (`CAMetalLayer`, `_dispatch_main_q` resize sync) + `VK_KHR_portability_enumeration`, injects `VK_KHR_portability_subset`, adapts Win64→AAPCS64 float args, sanitizes `VkAllocationCallbacks`, and registers dynamic procs via `register_native_va`. |
| **Swapchain / Present** | `d3d12-metal/src/metal/{swapchain,present_thread,capture}.rs` & `win32-rt/src/macos/` | Attaches via `macos::metal_layer_for_hwnd(hwnd)` (headless `MTLTexture` ring when `PORT_HEADLESS=1` or null `HWND`). Compute-swizzled `RGBA8 → BGRA8` presentation. |

## Offline D3D12 Trace Capture & Replay

Record a binary D3D12 stream (`AW2D3D12\0`) from any game run and inspect/validate/replay offline:

```sh
# 1. Capture during a run:
AW2_D3D12_TRACE=out/d3d12.trace cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game

# 2. Inspect stats, dump records, validate shaders, or replay on NullBackend:
cargo run -p d3d12-replay -- out/d3d12.trace --stats
cargo run -p d3d12-replay -- out/d3d12.trace --dump --until 500
AW2_SHADER_BACKEND=helper cargo run -p d3d12-replay -- out/d3d12.trace --shaders
cargo run -p d3d12-replay -- out/d3d12.trace --replay
```

For GPU telemetry on live runs, see `AW2_ENC_STATS=1` (`encstats.rs`) and `AW2_PASS_TIMING=1`
(`passtiming.rs`) in the `profile-and-verify` skill.

## Verification Checklist

Always enable Metal API validation (`MTL_DEBUG_LAYER=1`) and `-- --test-threads=1` on GPU tests:

```sh
# 1. Core Metal 4 + MetalFX + PSO manifest unit/GPU tests
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- --test-threads=1

# 2. Metal 4 GPU integration suites (run those relevant to your change)
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_clear_resolve --target aarch64-apple-darwin -- --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_mesh --target aarch64-apple-darwin -- --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_rayquery --target aarch64-apple-darwin -- --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_dispatch_rays --target aarch64-apple-darwin -- --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_rfx_psos --target aarch64-apple-darwin -- --test-threads=1

# 3. D3D11 + Vulkan unit & live Windows x86-64 PE E2E tests
MTL_DEBUG_LAYER=1 cargo test -p win32-rt --lib --target aarch64-apple-darwin -- d3d11 misc::vulkan_1 --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p port-loader --test d3d11_e2e --target aarch64-apple-darwin -- --test-threads=1
PORT_TEST_MOLTENVK=1 cargo test -p port-loader --test vulkan_e2e --target aarch64-apple-darwin -- --test-threads=1

# 4. Sync Win32 DB if you touched COM/DLL exports, verify runner build
cargo opsdb -- win32-sync
cargo opsdb -- doctor
cargo runner-build
```

Confirm **0 Metal validation errors** in stderr when running under `MTL_DEBUG_LAYER=1`. See
`docs/D3D12.md` for deeper design reference.
