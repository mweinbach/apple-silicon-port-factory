---
name: graphics-metal4
description: Develop, extend, or debug Direct3D 12, Direct3D 11, Vulkan-on-MoltenVK, Metal 4 pipelines/command lists/DXR/mesh/geometry shaders, MetalFX Spatial/Temporal upscaling, GPU vendor bridges (XeSS/DLSS/FFX/AGS/NRD/NRI/DirectStorage/DirectML), and offline D3D12 trace replay across crates/d3d12-metal, crates/win32-rt/src/{d3d12,dxgi,d3d11,misc/vulkan_1.rs}, and crates/d3d12-replay. Use when fixing GPU rendering bugs, Metal validation errors, swapchain/resize/fullscreen defects, missing D3D12/D3D11/Vulkan APIs, or upscaler issues.
---

# Graphics Stack: Metal 4, D3D12, D3D11, Vulkan & MetalFX

Read [gpu_bridges_and_hotpaths.md](./references/gpu_bridges_and_hotpaths.md) for the complete
upscaler/vendor bridge matrix (XeSS D3D12 & D3D11, DLSS, FFX, AGS, NRI/NRD, DirectStorage,
DirectML), DXGI driver version & fullscreen state rules, and CS-thread zero-allocation patterns.

## Architectural Boundaries (Non-Negotiable)

1. **NEVER import `objc2-metal` inside `crates/win32-rt`.**
   All Metal 4 and MetalFX calls live inside `crates/d3d12-metal` (plus `MTL4Archive` PSO caching
   in `crates/dxil-metal`). Both `win32-rt/src/d3d12/`, `d3d11/`, and GPU middleware (`libxess*`,
   `dstorage`, `directml`, `nri`) call safe Rust types in `d3d12-metal`; `vulkan_1.rs` dynamically
   `dlopen`s `libMoltenVK.dylib`.
2. **Keep barriers conservative by default.** Engine-specific barrier shortcuts (such as
   `AW2_IB_BINDLESS_WAR`) must stay **OFF by default** so hazard ordering stays sound across engines.
3. **Never substitute game shaders or fake GPU passes.** Unsupported stages/features (e.g., D3D11
   hull/domain/stream-out GS, DLSS/FFX Frame Generation, Ray Reconstruction) must log via `stub!`
   and return documented failure codes (`E_NOTIMPL`, `DXGI_ERROR_UNSUPPORTED`,
   `FFX_API_RETURN_NO_PROVIDER`, `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED`,
   `VK_ERROR_INCOMPATIBLE_DRIVER`).
4. **Apple AGX counter-heap ceiling (~31–32 per process):** Always pool timestamp queries into
   shared 4096-slot heaps (`d3d11` timestamps, `d3d12-metal/src/metal/passtiming.rs`).
5. **Zero-allocation hot paths:** Keep `d3d12-metal` CS-thread tracking on `TrackedSet<T>` (reuse
   `Vec` capacity), coalesce `with_resource_registry` locks in `dispatch()`, and keep
   `win32-rt/src/d3d12/cmdlist.rs` on size-tiered inline stack arrays with `ptr::read_unaligned`.

## Subsystem Map

| Layer | Key Files | Key Notes |
|---|---|---|
| **D3D12 → Metal 4** | `crates/d3d12-metal/src/{metal/*,mesh.rs,rt/*}` & `crates/win32-rt/src/{d3d12,dxgi}/` | `MTL4CommandQueue`, `MTLResidencySet`, 24-byte `IRDescriptorTableEntry`, planar `D32_FLOAT_S8X24_UINT` companion `R8Uint` stencil, DXR 1.1 BLAS/TLAS/RayQuery/`DispatchRays`, Mesh & Geometry Shader emulation (`MTL4MeshRenderPipelineDescriptor`), GPU ICB `ExecuteIndirect`. Regenerate D3D12 COM vtables via `python3 crates/win32-rt/src/d3d12/gen_vtables.py > crates/win32-rt/src/d3d12/vtables.rs`. |
| **MetalFX & Upscalers** | `d3d12-metal/src/metal/metalfx.rs` & `win32-rt/src/misc/{amd_fidelityfx_loader_dx12,nvngx_dlss,libxess,libxess_dx11,amd_ags_x64}.rs` | `dlopen`s `MetalFX.framework`. Fallback chain: `MTL4FXTemporalScaler` → `MTL4FXSpatialScaler` → bilinear compute (`port_upscale_2d_f`). Uses `Linear` color mode on float HDR formats (`RGBA16F`, `RG11B10F`, `RGBA32F`). Knobs: `PORT_METALFX=0`, `PORT_METALFX_TEMPORAL=0`. |
| **D3D11 → Metal 4** | `crates/win32-rt/src/d3d11/{device,context,resources,state,swapchain,com,types,mod}.rs` | Full D3D11.0/11.1 COM runtime (immediate + deferred contexts, `IDXGIDevice3`, `IDXGISurface2` `Map`/`Unmap`, `OpenSharedResource*`, `GenerateMips`, real `ID3D11GeometryShader` on Metal 4, pooled `MTL4CounterHeap` timestamps, indirect draw/dispatch + UAV counters, `SetFullscreenState`). Opt out via `PORT_D3D11=0`. |
| **Vulkan → MoltenVK** | `crates/win32-rt/src/misc/vulkan_1.rs` | Loads `libMoltenVK.dylib` (`PORT_VULKAN_DYLIB`, `Contents/Frameworks/`, Homebrew, `$VULKAN_SDK`). Rewrites `VK_KHR_win32_surface` → `VK_EXT_metal_surface` (`_dispatch_main_q` resize sync) + `VK_KHR_portability_enumeration`, injects `VK_KHR_portability_subset`, adapts Win64→AAPCS64 floats, sanitizes `VkAllocationCallbacks`, registers dynamic procs via `register_native_va`. |
| **DXGI / Swapchain** | `win32-rt/src/dxgi/adapter.rs`, `d3d11/swapchain.rs`, `d3d12/swapchain.rs`, `d3d12-metal/src/metal/{swapchain,present_thread,capture}.rs` | WDDM 3.2 UMD driver version (`32.0.15.4000`), seeded Display Adapter registry keys (`advapi32.rs`), native macOS borderless fullscreen toggle (`SetFullscreenState`), compute-swizzled `RGBA8 → BGRA8` present over `macos::metal_layer_for_hwnd`. |

## Offline D3D12 Trace Capture & Replay

```sh
AW2_D3D12_TRACE=out/d3d12.trace cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game
cargo run -p d3d12-replay -- out/d3d12.trace --stats
cargo run -p d3d12-replay -- out/d3d12.trace --dump --until 500
AW2_SHADER_BACKEND=helper cargo run -p d3d12-replay -- out/d3d12.trace --shaders
cargo run -p d3d12-replay -- out/d3d12.trace --replay
```

## Verification Checklist

Always enable Metal validation (`MTL_DEBUG_LAYER=1`) and `-- --test-threads=1` on GPU tests:

```sh
# 1. Core Metal 4 + MetalFX + PSO manifest + ResRegistry unit/GPU tests
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- --test-threads=1

# 2. Metal 4 GPU integration suites (run those covering your change)
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_clear_resolve --target aarch64-apple-darwin -- --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_mesh --target aarch64-apple-darwin -- --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_rayquery --target aarch64-apple-darwin -- --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_dispatch_rays --target aarch64-apple-darwin -- --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --test gpu_rfx_psos --target aarch64-apple-darwin -- --test-threads=1

# 3. D3D11, DXGI, Vulkan, upscalers & live Windows x86-64 PE E2E tests
MTL_DEBUG_LAYER=1 cargo test -p win32-rt --lib --target aarch64-apple-darwin -- \
  d3d11 dxgi d3d12::swapchain misc::vulkan_1 misc::libxess --test-threads=1
MTL_DEBUG_LAYER=1 cargo test -p port-loader --test d3d11_e2e --target aarch64-apple-darwin -- --test-threads=1
PORT_TEST_MOLTENVK=1 cargo test -p port-loader --test vulkan_e2e --target aarch64-apple-darwin -- --test-threads=1

# 4. Sync Win32 DB if COM/DLL exports changed & check runner + iOS build
cargo opsdb -- win32-sync && cargo opsdb -- doctor
cargo check -p win32-rt --lib --target aarch64-apple-ios
cargo runner-build
```

Confirm **0 Metal validation errors** in stderr. See `docs/D3D12.md` for design details.
