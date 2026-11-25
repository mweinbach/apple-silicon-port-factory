# Stub Backlog & Zero-Stub Tracker

This index is generated from the checked-in static inventory [`db/stub_inventory.tsv`](../db/stub_inventory.tsv)
and aggregates every per-thread backlog file placed in [`docs/stub-backlog.d/`](stub-backlog.d/).

> **Zero-stub policy:** Do not add new `stub!()`, `failing!()`, or `win64_stubs!()` sites anywhere in `crates/`.
> Both `cargo opsdb -- doctor` and `.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh` fail when a new site appears.
> Regenerate this file and `db/stub_inventory.tsv` after eliminating stubs with `cargo opsdb -- stub-sync`.

## Summary

| Metric | Count |
|---|---:|
| Total remaining stub sites | **177** |
| `stub` (partial Win32/OS/D3D/UCRT branches) | 129 |
| `failing` (honest-failing macro exports) | 29 |
| `middleware_stub` (third-party/engine middleware fallbacks) | 19 |
| DLLs with remaining stubs | 22 |

## Per-Thread Backlog Notes (`docs/stub-backlog.d/`)

| File | Summary |
|---|---|
| [`guest-corpus-bench.md`](stub-backlog.d/guest-corpus-bench.md) | Stub & API Gap Backlog — `toolkit/guest-corpus-bench` |
| [`middleware.md`](stub-backlog.d/middleware.md) | Stub & API Gap Backlog — `stubs/middleware` (Vendor Middleware & Vulkan Extensions) |
| [`window-input-ui.md`](stub-backlog.d/window-input-ui.md) | Stub Backlog — Window, Input, UI & Text DLLs (`stubs/window-input-ui`) |

### From `docs/stub-backlog.d/guest-corpus-bench.md`

# Stub & API Gap Backlog — `toolkit/guest-corpus-bench`

Exposed while building and running the headless Windows x86-64 guest corpus (`crates/port-loader/tests/os_guest_corpus.rs` and `crates/port-loader/tests/d3d12_e2e.rs`).

| DLL!API | What the test showed | Next step |
|---|---|---|
| `user32.dll!wsprintfW` | Missing export in `crates/win32-rt/src/user32/misc.rs`: guest binaries importing `wsprintfW` hit an unresolved-import trap thunk instead of formatting UTF-16 output. | Implement variadic `wsprintfW` via `win64_varargs_thunk!` backed by `ucrt::stdio` wide formatter (1024-char Windows cap) and run `cargo opsdb -- win32-sync`. |
| `user32.dll!wsprintfA` | Existing implementation in `crates/win32-rt/src/user32/misc.rs` copies the raw format string and ignores variadic arguments (`let _ = (a1, a2);`). | Replace with a real `win64_varargs_thunk!` formatter backed by `ucrt::stdio` narrow printf formatting (1024-char Windows cap). |

### From `docs/stub-backlog.d/middleware.md`

# Stub & API Gap Backlog — `stubs/middleware` (Vendor Middleware & Vulkan Extensions)

Scope: 61 static stub sites across vendor middleware DLLs (`amd_ags_x64`, `amd_fidelityfx_loader_dx12`, `libxess`, `libxess_dx11`, `nvngx_dlss`, `sl.interposer`, `oo2core_9_win64`, `physx_64`, `physxcooking_64`, `physx3common_x64`, `zstd`, `directml`, `dstorage`, `nri`) and `vulkan_1.dll` (`nvlowlatencyvk.dll`).

- **Before:** 61 sites in scope
- **After:** 15 sites remaining in scope (46 sites eliminated)
  - **Made real:** 6 sites
  - **Removed fake proprietary overrides (Rule 4 — shipping PE runs via ARM64 JIT; absent DLL returns `ERROR_MOD_NOT_FOUND`):** 7 sites
  - **Native-unsupported (matches Windows without vendor hardware/feature):** 33 sites
  - **Leftovers (honest partial implementations kept in place):** 15 sites

## Removed Sites — Made Real (6)

| DLL!API | Disposition | Implementation |
|---|---|---|
| `zstd.dll!ZSTD_compress` | Made real | Backed by vendored BSD-3-Clause `zstd-sys` (Meta Zstandard 1.6.0) with null-pointer safety and full unit round-trip/error tests in `crates/win32-rt/src/misc/zstd.rs`. |
| `zstd.dll!ZSTD_decompress` | Made real | Backed by vendored BSD-3-Clause `zstd-sys` (Meta Zstandard 1.6.0) with null-pointer safety and corrupt/small-buffer error tests in `crates/win32-rt/src/misc/zstd.rs`. |
| `dstorage.dll!DStorageCreateCompressionCodec` | Made real | Complete DirectStorage 1.2 built-in codec dispatch (`DSTORAGE_COMPRESSION_FORMAT_GDEFLATE` -> real GDeflate v1 codec; `NONE` -> `E_INVALIDARG`; custom `>= 0x80` & reserved `2..=0x7F` -> `E_NOTIMPL` per Microsoft Learn `DStorageCreateCompressionCodec`). |
| `amd_fidelityfx_loader_dx12.dll!ffxDispatch` (`GENERATEREACTIVEMASK`) | Made real | Implemented real Metal GPU compute shader `port_ffx_gen_reactive_2d` in `crates/d3d12-metal` (`clear.rs`, `metalfx_rt.rs`) matching FidelityFX FSR3 autogen reactive mask (`APPLY_TONEMAP`, `APPLY_INVERSETONEMAP`, `APPLY_THRESHOLD`, `USE_COMPONENTS_MAX`, `scale`, `cutoff_threshold`, `binary_value`), verified via GPU readback test. |
| `libxess.dll!xessD3D12Execute` | Made real | Implemented non-zero input/output subrect base offsets (`inputColorBase`, `inputDepthBase`, `inputMotionVectorBase`, `inputResponsiveMaskBase`, `outputColorBase`) in `crates/d3d12-metal` via dedicated-slot Metal staging blits (zero overhead at `[0, 0]`), verified via GPU readback test. |
| `libxess_dx11.dll!xessD3D11Execute` | Made real | Wired non-zero subrect base offsets through `d3d12-metal` `TemporalUpscaleParams` and `upscale_resource_subrect`, verified via D3D11 GPU readback test. |

## Removed Sites — Fake Proprietary Middleware Overrides Removed (7)

Per Rule 4, games shipping these proprietary DLLs execute their real Windows x86-64 PE via the ARM64 JIT (`DefaultDllMode::Guest`), and when no shipping DLL is present `LoadLibrary` fails with `ERROR_MOD_NOT_FOUND` (126), matching Windows.

| DLL!API | Disposition | Reason |
|---|---|---|
| `oo2core_9_win64.dll!OodleLZ_Compress` | Removed fake override | Proprietary RAD Oodle codec; deleted `crates/win32-rt/src/misc/oo2core_9_win64.rs` so shipping `oo2core_*_win64.dll` runs via ARM64 JIT and absent DLL fails with `ERROR_MOD_NOT_FOUND`. |
| `oo2core_9_win64.dll!OodleLZ_Decompress` | Removed fake override | Proprietary RAD Oodle codec; deleted `crates/win32-rt/src/misc/oo2core_9_win64.rs`. |
| `physx_64.dll!PxCreateBasePhysics` | Removed fake override | NVIDIA PhysX 4.x/5.x core simulation; deleted `crates/win32-rt/src/misc/physx_64.rs` so shipping `PhysX_64.dll` runs via ARM64 JIT (`[middleware.physx] mode = "guest"`). |
| `physx_64.dll!PxCreatePhysics` | Removed fake override | NVIDIA PhysX 4.x/5.x core simulation; deleted `crates/win32-rt/src/misc/physx_64.rs`. |
| `physx_64.dll!PxRegisterPhysicsSerializers` | Removed fake override | NVIDIA PhysX 4.x/5.x serializers; deleted `crates/win32-rt/src/misc/physx_64.rs`. |
| `physx_64.dll!PxUnregisterPhysicsSerializers` | Removed fake override | NVIDIA PhysX 4.x/5.x serializers; deleted `crates/win32-rt/src/misc/physx_64.rs`. |
| `physxcooking_64.dll!PxCreateCooking` | Removed fake override | NVIDIA PhysX 4.x/5.x cooking; deleted `crates/win32-rt/src/misc/physxcooking_64.rs` so shipping `PhysXCooking_64.dll` runs via ARM64 JIT. |

## Native-unsupported (matches Windows without the feature) (33)

| DLL!API | Behavior on Apple Silicon | Windows / Vendor SDK Citation |
|---|---|---|
| `amd_ags_x64.dll!agsDriverExtensionsDX12_PushMarker` | Forwards recognized D3D12 command lists to Metal debug markers (`AGS_SUCCESS`); returns `AGS_EXTENSION_NOT_SUPPORTED` (`7`) on foreign/non-AMD command lists. | AMD GPUOpen AGS SDK `amd_ags.h`: on non-AMD GPUs `extensionsSupported == 0` and DX12 driver-extension calls return `AGS_EXTENSION_NOT_SUPPORTED` (`7`). |
| `amd_ags_x64.dll!agsDriverExtensionsDX12_PopMarker` | Forwards recognized D3D12 command lists to Metal debug markers (`AGS_SUCCESS`); returns `AGS_EXTENSION_NOT_SUPPORTED` (`7`) on foreign/non-AMD command lists. | AMD GPUOpen AGS SDK `amd_ags.h`: `AGS_EXTENSION_NOT_SUPPORTED` (`7`) without AMD driver extension context. |
| `amd_ags_x64.dll!agsDriverExtensionsDX12_SetMarker` | Forwards recognized D3D12 command lists to Metal debug markers (`AGS_SUCCESS`); returns `AGS_EXTENSION_NOT_SUPPORTED` (`7`) on foreign/non-AMD command lists. | AMD GPUOpen AGS SDK `amd_ags.h`: `AGS_EXTENSION_NOT_SUPPORTED` (`7`) without AMD driver extension context. |
| `amd_fidelityfx_loader_dx12.dll!ffxCreateContext` (`FRAMEGENERATION`) | Zeroes `*context = 0` and returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp` / `ffx_api_types.h`: returns `FFX_API_RETURN_NO_PROVIDER` (`4`) when no frame-generation provider is installed (upscaler contexts remain routed to MetalFX; future Metal 4 `MTL4FXFrameInterpolator` optional). |
| `amd_fidelityfx_loader_dx12.dll!ffxConfigure` (`FRAMEGENERATION`) | Returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp`: `FFX_API_RETURN_NO_PROVIDER` (`4`) with no frame-generation provider. |
| `amd_fidelityfx_loader_dx12.dll!ffxDispatch` (`FRAMEGENERATION`) | Returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp`: `FFX_API_RETURN_NO_PROVIDER` (`4`) with no frame-generation provider. |
| `amd_fidelityfx_loader_dx12.dll!ffxQuery` (`INTERPOLATIONCOMMANDLIST_DX12`) | Zeroes `*out_cmd = 0` and returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp`: `FFX_API_RETURN_NO_PROVIDER` (`4`) with no frame-generation swapchain provider. |
| `amd_fidelityfx_loader_dx12.dll!ffxQuery` (`INTERPOLATIONTEXTURE_DX12`) | Zeroes `*out_tex = [0; 6]` and returns `FFX_API_RETURN_NO_PROVIDER` (`4`). | AMD FidelityFX SDK `ffx_api.cpp`: `FFX_API_RETURN_NO_PROVIDER` (`4`) with no frame-generation swapchain provider. |
| `libxess_dx11.dll!xessSetContextParameterF` | Validates context and finite value, returns `XESS_RESULT_ERROR_NOT_IMPLEMENTED` (`-7`). | Intel XeSS SDK `xess.h`: retail non-Intel XeSS runtimes expose no Intel-driver-internal float context parameters and return `XESS_RESULT_ERROR_NOT_IMPLEMENTED` (`-7`). |
| `libxess_dx11.dll!xessStartDump` | Validates context and non-null params/path, returns `XESS_RESULT_ERROR_NOT_IMPLEMENTED` (`-7`). | Intel XeSS SDK `xess_debug.h`: retail `libxess_dx11.dll` builds omit Intel debug frame-dump capture and return `XESS_RESULT_ERROR_NOT_IMPLEMENTED` (`-7`). |
| `libxess_dx11.dll!INTC_D3D12_GetApplicationInfo` | Returns `E_NOINTERFACE` (`0x80004002`). | Intel Graphics Extensions (`igdext64.dll` / `IntelExtensionsFramework`): no Intel `INTCExtensionContext` exists on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_GetGpuCrashDumpBufferInfo` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD crash-dump hook has no Intel GPU driver on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_GpuCrashDumpOnCommandListCreate` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD crash-dump hook has no Intel GPU driver on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_GpuCrashDumpOnCommandListDestroy` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD crash-dump hook has no Intel GPU driver on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_GpuCrashDumpOnDeviceDestroy` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD crash-dump hook has no Intel GPU driver on non-Intel GPUs. |
| `libxess_dx11.dll!INTC_D3D12_RegisterDriverCallbacks` | Void no-op. | Intel Graphics Extensions (`igdext64.dll`): Intel UMD callback hook has no Intel GPU driver on non-Intel GPUs. |
| `nvngx_dlss.dll!NVSDK_NGX_D3D12_CreateFeature` (non-SR/ISP features) | Zeroes `*out_handle = 0` and returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). (`SuperSampling` and `ImageSignalProcessing` run on MetalFX.) | NVIDIA NGX SDK `nvsdk_ngx_defs.h`: GPUs lacking Tensor Core / Optical Flow features (`FrameGeneration`, `DeepDVC`, `RayReconstruction`) return `NVSDK_NGX_Result_FAIL_FeatureNotSupported`. |
| `nvngx_dlss.dll!NVSDK_NGX_VULKAN_Init` | Returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). | NVIDIA NGX SDK `nvsdk_ngx_vk.h` / `nvsdk_ngx_defs.h`: requires NVIDIA Vulkan driver (`VK_NVX_binary_import`); non-NVIDIA GPUs return `NVSDK_NGX_Result_FAIL_FeatureNotSupported`. |
| `nvngx_dlss.dll!NVSDK_NGX_VULKAN_CreateFeature` | Zeroes `*out_handle = 0` and returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). | NVIDIA NGX SDK `nvsdk_ngx_vk.h`: returns `NVSDK_NGX_Result_FAIL_FeatureNotSupported` on non-NVIDIA GPUs. |
| `nvngx_dlss.dll!NVSDK_NGX_VULKAN_EvaluateFeature` | Returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). | NVIDIA NGX SDK `nvsdk_ngx_vk.h`: returns `NVSDK_NGX_Result_FAIL_FeatureNotSupported` on non-NVIDIA GPUs. |
| `nvngx_dlss.dll!NVSDK_NGX_VULKAN_ReleaseFeature` | Returns `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`). | NVIDIA NGX SDK `nvsdk_ngx_vk.h`: returns `NVSDK_NGX_Result_FAIL_FeatureNotSupported` on non-NVIDIA GPUs. |
| `sl.interposer.dll!slSetTag` | Returns `eErrorInvalidParameter` (`25`) if `tags == NULL \|\| n == 0`, else `eErrorFeatureNotSupported` (`32`). | NVIDIA Streamline SDK `source/core/sl.api/sl.cpp` (`slSetTag` -> `slValidateFeatureContext(kFeatureCommon, ctx)`): returns `eErrorFeatureNotSupported` when no Streamline plugins are supported on non-NVIDIA GPUs. |
| `sl.interposer.dll!slSetConstants` | Returns `eErrorInvalidParameter` (`25`) if `values == NULL \|\| frame == NULL`, else `eErrorFeatureNotSupported` (`32`). | NVIDIA Streamline SDK `source/core/sl.api/sl.cpp` (`slSetConstants` -> `slValidateFeatureContext(kFeatureCommon, ctx)`): returns `eErrorFeatureNotSupported` when no Streamline plugins are supported on non-NVIDIA GPUs. |
| `vulkan_1.dll!NvLL_VK_Initialize` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h` (`nvlowlatencyvk.dll`): returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) on non-NVIDIA GPUs without `VK_NV_low_latency`. |
| `vulkan_1.dll!NvLL_VK_Unload` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_InitLowLatencyDevice` | Zeroes `*signal_sem = 0` when non-null and returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_DestroyLowLatencyDevice` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_SetSleepMode` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_Sleep` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_GetSleepStatus` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_SetLatencyMarker` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_GetLatency` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |
| `vulkan_1.dll!NvLL_VK_NotifyOutOfBandQueue` | Returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`). | NVIDIA Reflex Vulkan SDK `NvLowLatencyVk.h`: returns `NVLL_VK_NO_IMPLEMENTATION` (`-3`) without NVIDIA Reflex driver. |

## Remaining Leftovers in Scope (15)

| DLL!API | Kind | Location | What Is Missing | Concrete Next Step |
|---|---|---|---|---|
| `physx3common_x64.dll!geom_get_world_bounds` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry (`ConvexMesh`, `TriangleMesh`, `HeightField`) bounds not parsed (analytic Sphere/Plane/Capsule/Box implemented). | Prefer shipping `PhysX3Common_x64.dll` via ARM64 JIT (`DefaultDllMode::Guest`), or parse PhysX 3.4 cooked `Gu::ConvexMeshData` / `Gu::TriangleMeshData` / `Gu::HeightFieldData` AABBs. |
| `physx3common_x64.dll!geom_point_distance` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry point-distance query not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement GJK/BVH closest-point over cooked PhysX 3.4 mesh payloads. |
| `physx3common_x64.dll!geom_compute_penetration` (2 sites: geom0, geom1) | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry penetration depth/direction (MTD) not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement EPA/SAT penetration over cooked convex/triangle/heightfield meshes. |
| `physx3common_x64.dll!geom_overlap` (2 sites: geom0, geom1) | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry overlap test not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement BVH/SAT overlap against cooked meshes. |
| `physx3common_x64.dll!geom_raycast` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry raycast not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement BVH33/BVH34 ray-triangle and ray-convex-hull intersection. |
| `physx3common_x64.dll!geom_sweep` (2 sites: geom0, geom1) | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | Cooked geometry shape sweep not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement GJK-raycast/TOI sweep over cooked meshes. |
| `physx3common_x64.dll!mesh_find_overlap_triangle_mesh` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | `PxMeshQuery::findOverlapTriangleMesh` requires cooked triangle mesh BVH/index payload decoding. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or decode `Gu::TriangleMesh` vertex/index/BVH buffers. |
| `physx3common_x64.dll!mesh_get_triangle` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | `PxMeshQuery::getTriangle` requires cooked triangle mesh vertex/index/adjacency decoding. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or decode `Gu::TriangleMesh` vertex/index/adjacency tables. |
| `physx3common_x64.dll!mesh_sweep` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs` | `PxMeshQuery::sweep` over triangle array not implemented. | Run shipping `PhysX3Common_x64.dll` via ARM64 JIT, or implement analytic shape-vs-triangle sweep loop in `physx3common_x64.rs`. |
| `directml.dll!parse` | `middleware_stub` | `crates/win32-rt/src/misc/directml/ops.rs` | Only ~35 core DirectML operators are implemented; remaining DML 1.x / FL 5.0 operators (`Gather`, `Scatter`, `Resample`, `Resize`, `TopK`, `Pad`, `Tile`, `Cast`, `RNN`/`LSTM`/`GRU`, `QuantizeLinear`/`DequantizeLinear`, `MultiheadAttention`, etc.) return `E_NOTIMPL`. | Implement missing `DML_OPERATOR_*` descriptors and CPU/Metal compute kernels as games request them, with per-operator unit tests. |
| `nri.dll!core_cmd_set_sample_positions` | `middleware_stub` | `crates/win32-rt/src/misc/nri/cmdlist.rs` | `ID3D12GraphicsCommandList1::SetSamplePositions` is not yet wired to Metal's `MTLRenderPassDescriptor::setSamplePositions:count:` (`D3D12_OPTIONS2.ProgrammableSamplePositionsTier` currently reports `0`). | Wire `SetSamplePositions` in `d3d12-metal` (`MTL4RenderPassDescriptor` / `MTLRenderPassDescriptor` programmable sample positions on Apple GPU family 7+) and `win32-rt/src/d3d12`, then forward `CmdSetSamplePositions` to it. |
| `nri.dll!core_create_query_pool` | `middleware_stub` | `crates/win32-rt/src/misc/nri/device.rs` | `QueryType::PIPELINE_STATISTICS` (`2`) and `QueryType::ACCELERATION_STRUCTURE_COMPACTED_SIZE` (`3`) return `NRI_RESULT_UNSUPPORTED` (`TIMESTAMP` and `OCCLUSION` are implemented). | Back `ACCELERATION_STRUCTURE_COMPACTED_SIZE` with a readback buffer + `EmitRaytracingAccelerationStructurePostbuildInfo` (`COMPACTED_SIZE`), and wire `PIPELINE_STATISTICS` once `d3d12-metal` exposes `MTL4CounterHeap` pipeline-statistics queries. |

### From `docs/stub-backlog.d/window-input-ui.md`

# Stub Backlog — Window, Input, UI & Text DLLs (`stubs/window-input-ui`)

Scope: 34 `stub!` sites in `db/stub_inventory.tsv` across `user32`, `dinput8`, `xinput1_4`, `gameinput`, `hid`, `uxtheme`, `msctf`, `dwrite`, `d2d1`, `gdi32`, `shell32`, and `powrprof`, plus 2 `user32` guest-corpus gaps (`wsprintfA` varargs and `wsprintfW` export, plus `NONCLIENTMETRICS` layout verification).

## Summary

| Category | Count |
|---|---|
| `stub!` sites in scope (before) | 34 |
| `stub!` sites in scope (after) | 1 |
| Removed — made real (incl. 2 `wsprintf` guest-corpus gaps) | 30 (28 `stub!` sites + `wsprintfA`/`wsprintfW`) |
| Removed — native-unsupported (matches Windows without feature) | 5 |
| Remaining honest leftovers | 1 |

---

## Removed Sites (Made Real)

| DLL!API | File | What was implemented |
|---|---|---|
| `user32.dll!LoadCursorA/W` | `crates/win32-rt/src/user32/cursor.rs`, `macos/appkit.rs` | Parses PE `RT_GROUP_CURSOR` (12) + `RT_CURSOR` (1) resources with 4-byte `(xHotSpot, yHotSpot)` prefix and DIB/PNG decoding; renders custom `Shape::Custom` cursors natively via CoreGraphics + `NSImage` + `NSCursor`. |
| `user32.dll!CreateCursor` | `crates/win32-rt/src/user32/cursor.rs`, `macos/appkit.rs` | Decodes 1bpp AND/XOR monochrome cursor masks with hotspot into RGBA/BGRA `IconData` and custom `NSCursor`. |
| `user32.dll!DefWindowProcA/W` (`WM_SYSCOMMAND`) | `crates/win32-rt/src/user32/defproc.rs` | Handles all documented `SC_*` codes (`SC_HOTKEY`, `SC_NEXTWINDOW`/`SC_PREVWINDOW`, standard WindowServer-owned no-ops) and ignores unknown codes as Windows does. |
| `user32.dll!RegisterRawInputDevices` (`RIDEV_NOLEGACY`) | `crates/win32-rt/src/user32/input.rs` | Enforces `RIDEV_NOLEGACY` for mouse `(1,2)` and keyboard `(1,6)` (suppressing legacy `WM_MOUSE*`/`WM_KEY*` while continuing to deliver `WM_INPUT` and update async key state); rejects `RIDEV_NOLEGACY` on non-legacy usages with `ERROR_INVALID_PARAMETER`. |
| `user32.dll!ReplyMessage` | `crates/win32-rt/src/user32/message.rs` | Unblocks waiting cross-thread `SendMessage` callers early via thread-local reply stack and RAII `Drop` guard. |
| `user32.dll!GetSystemMetrics` / `GetSystemMetricsForDpi` | `crates/win32-rt/src/user32/misc.rs` | Complete `SM_*` table with DPI scaling; returns `0` for undefined indices matching Windows. |
| `user32.dll!SystemParametersInfoA/W/ForDpi` | `crates/win32-rt/src/user32/misc.rs` | Persistent `SpiState` across all standard SPI getters/setters, DPI-scaled `NONCLIENTMETRICS` with compile-time layout assertions (`344B`/`504B`), and `FALSE` + `ERROR_INVALID_PARAMETER` for unknown actions. |
| `user32.dll!GetUserObjectInformationA/W` (handles & indices) | `crates/win32-rt/src/user32/misc.rs` | Added desktop handle APIs (`GetThreadDesktop`, `OpenInputDesktop`, `OpenDesktopA/W`, `CloseDesktop`, `CloseWindowStation`) and all 6 `UOI_*` queries (`UOI_FLAGS`, `UOI_NAME`, `UOI_TYPE`, `UOI_USER_SID`, `UOI_HEAPSIZE`, `UOI_IO`). |
| `user32.dll!RegisterHotKey` / `UnregisterHotKey` | `crates/win32-rt/src/user32/misc.rs`, `input.rs`, `window.rs` | Per-thread/per-HWND global hotkey table with duplicate detection (`ERROR_HOTKEY_ALREADY_REGISTERED`), `WM_HOTKEY` dispatch on keydown, and automatic cleanup on window destruction. |
| `user32.dll!wsprintfA` / `wsprintfW` | `crates/win32-rt/src/user32/misc.rs`, `ucrt/printf.rs` | Column-0 `win64_varargs_thunk!` varargs formatting backed by `ucrt::printf` with Windows 1024-char cap (1023 + NUL) and `LEGACY_WIDE_SPECIFIERS` for `wsprintfW`; un-ignored `guest_user32_wsprintf_backlog`. |
| `user32.dll!DisplayConfigGetDeviceInfo` | `crates/win32-rt/src/user32/monitor.rs` | Implemented remaining CCD GET packets (`GET_TARGET_BASE_TYPE`, `GET_SUPPORT_VIRTUAL_RESOLUTION`, `GET_MONITOR_SPECIALIZATION`, `GET_ADVANCED_COLOR_INFO_2`, `GET_DPI_SCALE`); unknown types return `ERROR_INVALID_PARAMETER`. |
| `dinput8.dll!IDirectInputDevice8::GetProperty` / `SetProperty` | `crates/win32-rt/src/misc/dinput8.rs` | Implemented `DIPROP_FFGAIN`, `DIPROP_FFLOAD`, `DIPROP_AUTOCENTER`, `DIPROP_CALIBRATIONMODE`, `DIPROP_CALIBRATION`, `DIPROP_GUIDANDPATH` (Xbox `IG_` path), `DIPROP_GETPORTDISPLAYNAME`, `DIPROP_KEYNAME`, `DIPROP_APPDATA` (wired to `DIDEVICEOBJECTDATA.uAppData`), `DIPROP_TYPENAME`, `DIPROP_USERNAME`. |
| `xinput1_4.dll!XInputGetKeystroke` | `crates/win32-rt/src/misc/xinput1_4.rs` | Real per-slot and `XUSER_INDEX_ANY` (`0xFF`) `XINPUT_KEYSTROKE` ring queue populated on button, trigger, and thumbstick edge transitions. |
| `gameinput.dll!IGameInput::GetTemporalReading` | `crates/win32-rt/src/misc/gameinput.rs` | Searches device reading history for the newest snapshot with `timestamp_us <= timestamp`, returning `GAMEINPUT_E_READING_NOT_FOUND` when none precedes it. |
| `hid.dll!HidD_SetNumInputBuffers` | `crates/win32-rt/src/misc/hid.rs` | Per-handle input report ring-buffer capacity (`2..=512`), enforced on report delivery, trimmed on shrink, and returned by `HidD_GetNumInputBuffers`. |
| `uxtheme.dll!DrawThemeBackground` | `crates/win32-rt/src/misc/uxtheme.rs` | Matches Windows Classic / unthemed desktop (`IsThemeActive() == FALSE`, zero open `HTHEME` handles): `ValidateHandle` rejects any unopened handle with `E_HANDLE`. |
| `uxtheme.dll!TaskDialogIndirect` | `crates/win32-rt/src/misc/uxtheme.rs` | Parses 1-byte-packed x64 `TASKDIALOGCONFIG` (160B) and `TASKDIALOG_BUTTON` (12B), resolves `IS_INTRESOURCE` strings via `LoadStringW`, supports custom/common buttons, radio defaults, verification flag, and callback notifications, and displays via native `NSAlert`/`UIAlertController` (with honest headless handling). |
| `msctf.dll!ITfContext::TrackProperties` | `crates/win32-rt/src/misc/msctf/{context,range,abi}.rs` | Returns a composite `ITfReadOnlyProperty` whose `GetValue` yields `VT_UNKNOWN` -> `IEnumTfPropertyValue` enumerating `TF_PROPERTYVAL` entries for all tracked properties. |
| `dwrite.dll!IDWriteFactory::CreateCustomFontFileReference` | `crates/win32-rt/src/misc/dwrite{,.rs,/font.rs,/sfnt.rs}` | Invokes `IDWriteFontFileLoader::CreateStreamFromKey` + `IDWriteFontFileStream::{GetFileSize, ReadFileFragment, ReleaseFileFragment}` to build a memory-backed SFNT `IDWriteFontFile` usable with `Analyze` and `CreateFontFace`. |
| `dwrite.dll!IDWriteFactory::CreateCustomFontCollection` | `crates/win32-rt/src/misc/dwrite{,.rs,/font.rs}` | Invokes `IDWriteFontCollectionLoader::CreateEnumeratorFromKey` + `IDWriteFontFileEnumerator::{MoveNext, GetCurrentFontFile}` to assemble a real `IDWriteFontCollection`. |
| `dwrite.dll!IDWriteFactory::CreateTextAnalyzer` | `crates/win32-rt/src/misc/dwrite/analysis.rs` | Full 10-slot `IDWriteTextAnalyzer` (`AnalyzeScript`, `AnalyzeBidi`, `AnalyzeNumberSubstitution`, `AnalyzeLineBreakpoints`, `GetGlyphs`, `GetGlyphPlacements`, `GetGdiCompatibleGlyphPlacements`). |
| `dwrite.dll!IDWriteFactory::CreateGlyphRunAnalysis` | `crates/win32-rt/src/misc/dwrite/analysis.rs` | Full 6-slot `IDWriteGlyphRunAnalysis` (`GetAlphaTextureBounds`, `CreateAlphaTexture` for `ALIASED_1x1` and `CLEARTYPE_3x1`, `GetAlphaBlendParams`). |
| `d2d1.dll!ID2D1Geometry::Tessellate` | `crates/win32-rt/src/misc/d2d1/{geometry,abi}.rs` | Flattens geometry with transform/tolerance, triangulates convex and concave filled subpaths via ear-clipping, and emits `D2D1_TRIANGLE` arrays to `ID2D1TessellationSink::AddTriangles`. |
| `gdi32.dll!GetDeviceCaps` | `crates/win32-rt/src/misc/gdi32.rs` | Covers all standard `wingdi.h` capability indices, honoring selected bitmap dimensions/depth on memory DCs and returning `0` for unknown indices. |
| `powrprof.dll!CallNtPowerInformation` | `crates/win32-rt/src/misc/powrprof.rs` | Implements standard read-only `POWER_INFORMATION_LEVEL`s (including `LastWakeTime`/`LastSleepTime` via macOS `sysctlbyname`), returns `STATUS_ACCESS_DENIED` for privileged admin levels and `STATUS_INVALID_PARAMETER` for unknown levels. |
| `powrprof.dll!PowerRegisterSuspendResumeNotification` | `crates/win32-rt/src/misc/powrprof.rs` | Arms a dedicated background macOS IOKit `IORegisterForSystemPower` `CFRunLoop` delivering `PBT_APMSUSPEND`/`PBT_APMRESUME*` to `DEVICE_NOTIFY_CALLBACK` and `WM_POWERBROADCAST` recipients. |
| `powrprof.dll!PowerSettingRegisterNotification` | `crates/win32-rt/src/misc/powrprof.rs` | Arms macOS IOKit `IOPSNotificationCreateRunLoopSource`, delivers the immediate initial `PBT_POWERSETTINGCHANGE` value upon registration, and dispatches `POWERBROADCAST_SETTING` packets on power-source/battery changes. |

---

## Native-unsupported (matches Windows without the feature)

| DLL!API | File | Behavior Returned | Windows Documentation Citation |
|---|---|---|---|
| `user32.dll!MessageBoxA/W` (headless multi-button branch) | `crates/win32-rt/src/user32/misc.rs` | Returns `0` with `SetLastError(ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION)` (`1459`) when no interactive desktop session is active (interactive macOS/iOS displays a real `NSAlert`/`UIAlertController`). | Microsoft Learn — [MessageBox function](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messagebox) & Windows Services / non-interactive window station behavior (`ERROR_REQUIRES_INTERACTIVE_WINDOWSTATION`, `1459`). |
| `user32.dll!DisplayConfigSetDeviceInfo` | `crates/win32-rt/src/user32/monitor.rs` | Validates packet pointer/size/type (`ERROR_INVALID_PARAMETER`) and returns `ERROR_NOT_SUPPORTED` (`50`) for valid WDDM/CCD `SET_*` packets (`5, 8, 10, 13, 16`), because Apple Silicon macOS has no Windows WDDM/CCD display driver. | Microsoft Learn — [DisplayConfigSetDeviceInfo function](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-displayconfigsetdeviceinfo) (`ERROR_NOT_SUPPORTED` when the display driver/system does not support the requested configuration change). |
| `dinput8.dll!IDirectInputDevice8::RunControlPanel` | `crates/win32-rt/src/misc/dinput8.rs` | Validates `dwFlags == 0` (`DIERR_INVALIDPARAM`) and returns `DIERR_UNSUPPORTED` (`E_NOTIMPL`), because Windows `joy.cpl` does not exist on macOS/iOS. | Microsoft Learn — [IDirectInputDevice8::RunControlPanel](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee417909(v=vs.85)) (optional control-panel UI; returns `DIERR_UNSUPPORTED` when unsupported). |
| `dinput8.dll!IDirectInput8::RunControlPanel` | `crates/win32-rt/src/misc/dinput8.rs` | Validates `dwFlags == 0` (`DIERR_INVALIDPARAM`) and returns `DIERR_UNSUPPORTED` (`E_NOTIMPL`), because Windows `joy.cpl` does not exist on macOS/iOS. | Microsoft Learn — [IDirectInput8::RunControlPanel](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee417816(v=vs.85)) (returns `DIERR_UNSUPPORTED` when control panel launch is unsupported). |
| `dinput8.dll!IDirectInput8::ConfigureDevices` | `crates/win32-rt/src/misc/dinput8.rs` | Returns `DIERR_UNSUPPORTED` (`E_NOTIMPL`), matching Windows Vista and all later Windows releases where the DirectInput action-mapper UI was removed. | Microsoft Learn — [IDirectInput8::ConfigureDevices](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee417799(v=vs.85)) (deprecated and unsupported starting with Windows Vista). |

---

## Remaining Honest Leftovers

| DLL!API | File | What works now | What is still missing & next step |
|---|---|---|---|
| `shell32.dll!SHCreateItemFromParsingName` | `crates/win32-rt/src/misc/shell32.rs` | Local VFS paths (including `\\?\` extended paths), `file:///` URLs (percent-decoded), `shell:<KnownFolderName>`, and filesystem-backed `::{<FOLDERID>}` / `shell:::{<FOLDERID>}` parsing names now resolve to real `IShellItem` COM objects. | Non-filesystem virtual Explorer shell namespaces (e.g. `shell:::{20D04FE0-...}` This PC, `::{645FF040-...}` Recycle Bin), `ms-settings:` URIs, `http://` URLs, and SMB/UNC network shares (`\\server\share\...`) still hit `stub!` and return `E_NOTIMPL`. Next step: implement a virtual `IShellFolder`/`IShellItem` namespace tree for `CLSID_MyComputer` / `CLSID_RecycleBin` (backed by mounted drives / `~/.Trash`) and an SMB/UNC resolver if a title requests them. |

## Proprietary & Vendor Middleware (Legal / Architectural Limits)

These modules wrap proprietary third-party SDKs or GPU-vendor-specific extensions. Where a game ships its own Windows x86-64 PE DLL (e.g., Oodle `oo2core_*_win64.dll`, PhysX 4/5 `PhysX_64.dll`, `zstd.dll`), `port-runner` executes the real shipping PE through the ARM64 JIT rather than overriding it.

| DLL | Remaining Sites | Reason | Next Step |
|---|---:|---|---|

## Remaining Stubs by DLL

| DLL | Count | Reason | Next Step |
|---|---:|---|---|
| `kernel32.dll` | 30 | Partial Windows kernel/file/memory/locale/job/threadpool sub-feature | Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives. |
| `bcrypt.dll` | 20 | Unimplemented CNG algorithm, key-wrap blob, or padding mode | Add remaining CNG algorithms/padding modes (OAEP/PSS/ECDH/3DES) backed by CommonCrypto/ring as games import them. |
| `crypt32.dll` | 19 | Unimplemented CryptoAPI ASN.1 decode, PKCS#7/PFX, or certificate store property | Implement remaining DER/ASN.1 object decoders, PFX import, and CryptMsg verification on top of rustls/ring. |
| `d3d11.dll` | 14 | Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop) | Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them. |
| `ole32.dll` | 12 | Unimplemented WinRT IPropertyValue scalar/array type or VARIANT/SAFEARRAY coercion | Implement remaining IPropertyValue* types and VariantChangeType/SafeArrayDestroy coercions. |
| `physx3common_x64.dll` | 12 | PhysX 3.4 geometry/mesh query path without cooked mesh payload | Prefer shipping PhysX3*.dll under ARM64 JIT, or decode cooked BVH/triangle mesh payloads in physx3common_x64. |
| `dxgi.dll` | 10 | Unsupported Windows-only DXGI path (WARP software rasterizer, UWP CoreWindow, exclusive fullscreen ownership, event notifications) | Implement VSync-synchronized WaitForVBlank via CVDisplayLink/CADisplayLink and shared-handle LUID lookup; keep WARP/CoreWindow honestly unsupported. |
| `ntdll.dll` | 9 | Unimplemented NT info class, TxF transaction API, or direct kernel-to-user dispatcher export | Add missing Process/Thread/System information classes on demand; TxF transactions and direct KiUser* calls remain honestly unsupported. |
| `advapi32.dll` | 8 | Legacy CryptoAPI (CAPI) key/sign/encrypt or uncommon token information class | Back remaining CAPI key/cipher calls with CommonCrypto/ring and implement missing TOKEN_INFORMATION_CLASS variants. |
| `ws2_32.dll` | 8 | Uncommon Winsock2 async/QOS/control-message API or cross-process socket duplication | Implement WSARecvMsg/WSASendMsg via POSIX recvmsg/sendmsg and remaining WSAIoctl codes as games use them. |
| `mmdevapi.dll` | 6 | Optional WASAPI mode/service (exclusive mode, loopback capture, hardware offload, unknown COM IID) | Add CoreAudio loopback capture / tap support when a game requests AUDCLNT_STREAMFLAGS_LOOPBACK; keep exclusive/offload honestly rejected. |
| `ucrtbase.dll` | 6 | Uncommon CRT locale, multibyte codepage, UTF-16 stream encoding, or interactive console/shell call | Add UTF-16 FILE stream translation and locale/codepage tables; keep system() safely sandboxed/refused. |
| `dbghelp.dll` | 4 | Unimplemented DbgHelp stack walk or minidump writer | Wire StackWalk64 to win32-rt's .pdata unwinder and write valid MDMP headers for MiniDumpWriteDump. |
| `secur32.dll` | 4 | Server-side Schannel (INBOUND) or uncommon context/credential attribute | Add server-side TLS or missing SECPKG_ATTR_* queries if a game hosts local TLS sockets. |
| `wintrust.dll` | 4 | Unimplemented WinVerifyTrust action/choice or TPM Base Services (TBS) | Keep TPM TBS returning TBS_E_TPM_NOT_FOUND honestly; extend WinVerifyTrust for additional catalog/blob choices as needed. |
| `d3d12.dll` | 2 | D3D12 root-signature deserializer COM object not yet implemented | Parse DXBC/RTS0 root-signature blobs in win32-rt/src/d3d12 and expose ID3D12RootSignatureDeserializer / VersionedDeserializer. |
| `libcurl.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `nri.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `xaudio2_9redist.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `directml.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `dsound.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `shell32.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |

### `kernel32.dll` (30)

- **Reason:** Partial Windows kernel/file/memory/locale/job/threadpool sub-feature
- **Next step:** Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives.

| API | Kind | Location | Detail |
|---|---|---|---|
| `AllocConsole` | `stub` | `crates/win32-rt/src/kernel32/file.rs:477` | `"no separate console window; stdout/stderr of the loader act as the console"` |
| `AssignProcessToJobObject` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1655` | `"job objects have no effect"` |
| `CreateProcessW` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1283` | `"CreateProcessW unavailable on iOS: app={:?} cmd={:?}", from_wide_opt(app), from_wide_opt(cmd)` |
| `CreateRemoteThread` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:60` | `"CreateRemoteThread into another process"` |
| `DeviceIoControl` | `stub` | `crates/win32-rt/src/kernel32/file.rs:7127` | `"IOCTL_STORAGE_QUERY_PROPERTY property {prop} unsupported"` |
| `DeviceIoControl` | `stub` | `crates/win32-rt/src/kernel32/file.rs:7155` | `"DeviceIoControl code {code:#x} unsupported"` |
| `FlushProcessWriteBuffers` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:1045` | `"FlushProcessWriteBuffers approximated by a local fence"` |
| `GetCalendarInfoEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1354` | `"GetCalendarInfoEx type {ty:#x}"` |
| `GetFileInformationByHandleEx` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2137` | `"GetFileInformationByHandleEx class {class} not supported"` |
| `GetGeoInfoW` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1125` | `"GetGeoInfo for geo id {geo} answered with US data"` |
| `GetLocaleInfoW` | `stub` | `crates/win32-rt/src/kernel32/string.rs:918` | `"locale {lcid:#x} not available; answering with en-US data"` |
| `GetNumberFormatEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1323` | `"custom NUMBERFMT ignored"` |
| `LCMapStringEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:552` | `"LCMAP_HASH unsupported"` |
| `LCMapStringEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:578` | `"LCMapString flags {flags:#x} partially honoured"` |
| `MapViewOfFile3` | `stub` | `crates/win32-rt/src/kernel32/file.rs:6918` | `"MapViewOfFile3 on another process"` |
| `MoveFileExA` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2596` | `"MoveFileExA delete-on-reboot ignored"` |
| `MoveFileExW` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2589` | `"MoveFileExW delete-on-reboot ignored"` |
| `SetFileInformationByHandle` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2268` | `"SetFileInformationByHandle class {class} not supported"` |
| `SetInformationJobObject` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1659` | `"SetInformationJobObject class {class} ignored"` |
| `TerminateThread` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:173` | `"TerminateThread unsupported"` |
| `VirtualAlloc2` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:577` | `"VirtualAlloc2 on another process"` |
| `VirtualAllocEx` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:568` | `"VirtualAllocEx on another process"` |
| `VirtualProtect` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:684` | `"PAGE_GUARD not emulated"` |
| `format_message` | `stub` | `crates/win32-rt/src/kernel32/process.rs:2180` | `"message tables in modules are not supported (id {id:#x})"` |
| `format_message` | `stub` | `crates/win32-rt/src/kernel32/process.rs:2189` | `"FormatMessage from module message table (id {id:#x})"` |
| `get_locale_info` | `stub` | `crates/win32-rt/src/kernel32/string.rs:865` | `"GetLocaleInfo LCTYPE {lctype:#x} unknown"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:362` | `"MEM_LARGE_PAGES ignored"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:365` | `"MEM_WRITE_WATCH not tracked"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:369` | `"PAGE_GUARD not emulated"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:377` | `"MEM_RESET_UNDO: contents may have been discarded"` |

### `bcrypt.dll` (20)

- **Reason:** Unimplemented CNG algorithm, key-wrap blob, or padding mode
- **Next step:** Add remaining CNG algorithms/padding modes (OAEP/PSS/ECDH/3DES) backed by CommonCrypto/ring as games import them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `BCryptExportKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2447` | `"BCryptExportKey(wrapped h_export_key) -> STATUS_NOT_SUPPORTED"` |
| `BCryptExportKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2505` | `"BCryptExportKey({t:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptGenerateKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2614` | `"BCryptGenerateKeyPair({}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptGenerateSymmetricKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2528` | `"BCryptGenerateSymmetricKey({}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptGetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1810` | `"BCryptGetProperty(alg={}, {p:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptGetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1824` | `"BCryptGetProperty(hash, {p:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptGetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1859` | `"BCryptGetProperty(key, {p:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptImportKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2372` | `"BCryptImportKey({}, wrapped h_import_key) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKey` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2429` | `"BCryptImportKey({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1941` | `"BCryptImportKeyPair({}, wrapped import_key) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1949` | `"BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1964` | `"BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptImportKeyPair` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1979` | `"BCryptImportKeyPair({}, {t:?}) -> STATUS_NOT_SUPPORTED", a.name` |
| `BCryptOpenAlgorithmProvider` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:1720` | `"BCryptOpenAlgorithmProvider({name:?}, flags 0x{flags:x}) -> STATUS_NOT_FOUND (algorithm not implemented)"` |
| `BCryptSetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2670` | `"BCryptSetProperty(ChainingMode={mode_str:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptSetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2718` | `"BCryptSetProperty(ECCCurveName={curve_str:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptSetProperty` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2756` | `"BCryptSetProperty({p:?}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptSignHash` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2057` | `"BCryptSignHash(RSA, flags 0x{flags:x}) -> STATUS_NOT_SUPPORTED"` |
| `BCryptVerifySignature` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2122` | `"BCryptVerifySignature(RSA, flags 0x{flags:x}) -> STATUS_NOT_SUPPORTED (padding)"` |
| `bcrypt_aes_crypt` | `stub` | `crates/win32-rt/src/misc/bcrypt.rs:2173` | `"BCrypt{} on non-AES key -> STATUS_NOT_SUPPORTED", if encrypt { "Encrypt" } else { "Decrypt" }` |

### `crypt32.dll` (19)

- **Reason:** Unimplemented CryptoAPI ASN.1 decode, PKCS#7/PFX, or certificate store property
- **Next step:** Implement remaining DER/ASN.1 object decoders, PFX import, and CryptMsg verification on top of rustls/ring.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CertFreeCRLContext` | `stub` | `crates/win32-rt/src/misc/crypt32.rs:355` | `"CertFreeCRLContext({ctx:p}): foreign CRL context ignored"` |
| `CertFreeCTLContext` | `stub` | `crates/win32-rt/src/misc/crypt32.rs:362` | `"CertFreeCTLContext({ctx:p}): foreign CTL context ignored"` |
| `CertGetCertificateContextProperty` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CertStrToNameA` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CertStrToNameW` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CertVerifySubjectCertificateContext` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptDecodeObject` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptDecodeObjectEx` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptFindOIDInfo` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:42` | `*mut c_void = std::ptr::null_mut(), err CRYPT_E_NOT_FOUND` |
| `CryptImportPublicKeyInfo` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptMsgOpenToDecode` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:42` | `*mut c_void = std::ptr::null_mut(), err CRYPT_E_NOT_FOUND` |
| `CryptMsgUpdate` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptVerifyMessageSignature` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:47` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `PFXImportCertStore` | `failing` | `crates/win32-rt/src/misc/crypt32.rs:42` | `*mut c_void = std::ptr::null_mut(), err CRYPT_E_NOT_FOUND` |
| `cert_free_certificate_chain_engine_impl` | `stub` | `crates/win32-rt/src/misc/crypt32/chain.rs:593` | `"CertFreeCertificateChainEngine({engine:p}): foreign engine ignored"` |
| `cert_free_certificate_chain_impl` | `stub` | `crates/win32-rt/src/misc/crypt32/chain.rs:787` | `"CertFreeCertificateChain({chain:p}): foreign chain ignored"` |
| `cert_verify_certificate_chain_policy_impl` | `stub` | `crates/win32-rt/src/misc/crypt32/chain.rs:807` | `"CertVerifyCertificateChainPolicy(oid={policy_id:#x}) -> FALSE"` |
| `crypt_string_to_binary_impl` | `stub` | `crates/win32-rt/src/misc/crypt32/asn1.rs:1276` | `"CryptStringToBinary unsupported flags {flags:#x}"` |
| `encode_x509_object` | `stub` | `crates/win32-rt/src/misc/crypt32/asn1.rs:941` | `"CryptEncodeObjectEx(struct_type={raw_stype:#x}) -> FALSE"` |

### `d3d11.dll` (14)

- **Reason:** Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop)
- **Next step:** Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `check_counter` | `stub` | `crates/win32-rt/src/d3d11/device.rs:1074` | `"ID3D11Device::CheckCounter -> DXGI_ERROR_UNSUPPORTED"` |
| `create_class_linkage` | `stub` | `crates/win32-rt/src/d3d11/device.rs:749` | `"ID3D11Device::CreateClassLinkage -> E_NOTIMPL"` |
| `create_counter` | `stub` | `crates/win32-rt/src/d3d11/device.rs:958` | `"ID3D11Device::CreateCounter -> DXGI_ERROR_UNSUPPORTED"` |
| `create_device_context_state` | `stub` | `crates/win32-rt/src/d3d11/device.rs:1038` | `"ID3D11Device1::CreateDeviceContextState -> E_NOTIMPL"` |
| `create_domain_shader` | `stub` | `crates/win32-rt/src/d3d11/device.rs:743` | `"ID3D11Device::CreateDomainShader -> E_NOTIMPL"` |
| `create_geometry_shader_so` | `stub` | `crates/win32-rt/src/d3d11/device.rs:727` | `"ID3D11Device::CreateGeometryShaderWithStreamOutput -> E_NOTIMPL"` |
| `create_hull_shader` | `stub` | `crates/win32-rt/src/d3d11/device.rs:735` | `"ID3D11Device::CreateHullShader -> E_NOTIMPL"` |
| `draw_auto` | `stub` | `crates/win32-rt/src/d3d11/context.rs:2732` | `"ID3D11DeviceContext::DrawAuto not implemented"` |
| `ds_set_shader` | `stub` | `crates/win32-rt/src/d3d11/context.rs:1972` | `"DSSetShader(non-null) ignored"` |
| `dxgi_dev_create_surface` | `stub` | `crates/win32-rt/src/d3d11/device.rs:1308` | `"IDXGIDevice::CreateSurface -> E_NOTIMPL"` |
| `get_data` | `stub` | `crates/win32-rt/src/d3d11/context.rs:2908` | `"GetData(PIPELINE_STATISTICS) returning zeroes"` |
| `hs_set_shader` | `stub` | `crates/win32-rt/src/d3d11/context.rs:1969` | `"HSSetShader(non-null) ignored"` |
| `surf_get_dc` | `stub` | `crates/win32-rt/src/d3d11/resources.rs:1308` | `"IDXGISurface1::GetDC -> E_NOTIMPL"` |
| `swap_device_context_state` | `stub` | `crates/win32-rt/src/d3d11/context.rs:3013` | `"ID3D11DeviceContext1::SwapDeviceContextState not implemented"` |

### `ole32.dll` (12)

- **Reason:** Unimplemented WinRT IPropertyValue scalar/array type or VARIANT/SAFEARRAY coercion
- **Next step:** Implement remaining IPropertyValue* types and VariantChangeType/SafeArrayDestroy coercions.

| API | Kind | Location | Detail |
|---|---|---|---|
| `pv_get_date_time` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2167` | `"IPropertyValue::GetDateTime -> E_NOTIMPL"` |
| `pv_get_point` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2185` | `"IPropertyValue::GetPoint -> E_NOTIMPL"` |
| `pv_get_rect` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2203` | `"IPropertyValue::GetRect -> E_NOTIMPL"` |
| `pv_get_rect` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2223` | `concat!("IPropertyValue::", $method, " -> E_NOTIMPL")` |
| `pv_get_size` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2194` | `"IPropertyValue::GetSize -> E_NOTIMPL"` |
| `pv_get_time_span` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2176` | `"IPropertyValue::GetTimeSpan -> E_NOTIMPL"` |
| `pvs_create_date_time` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2582` | `"IPropertyValueStatics::CreateDateTime -> E_NOTIMPL"` |
| `pvs_create_point` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2609` | `"IPropertyValueStatics::CreatePoint -> E_NOTIMPL"` |
| `pvs_create_rect` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2640` | `"IPropertyValueStatics::CreateRect -> E_NOTIMPL"` |
| `pvs_create_rect` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2659` | `concat!("IPropertyValueStatics::", $method, " -> E_NOTIMPL")` |
| `pvs_create_size` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2623` | `"IPropertyValueStatics::CreateSize -> E_NOTIMPL"` |
| `pvs_create_time_span` | `stub` | `crates/win32-rt/src/misc/ole32.rs:2595` | `"IPropertyValueStatics::CreateTimeSpan -> E_NOTIMPL"` |

### `physx3common_x64.dll` (12)

- **Reason:** PhysX 3.4 geometry/mesh query path without cooked mesh payload
- **Next step:** Prefer shipping PhysX3*.dll under ARM64 JIT, or decode cooked BVH/triangle mesh payloads in physx3common_x64.

| API | Kind | Location | Detail |
|---|---|---|---|
| `geom_compute_penetration` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2336` | `"PxGeometryQuery::computePenetration unsupported geometry0 {}", geom_type_name(t)` |
| `geom_compute_penetration` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2346` | `"PxGeometryQuery::computePenetration unsupported geometry1 {}", geom_type_name(t)` |
| `geom_get_world_bounds` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2243` | `"PxGeometryQuery::getWorldBounds unsupported cooked geometry {}", geom_type_name(gtype)` |
| `geom_overlap` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2378` | `"PxGeometryQuery::overlap unsupported geometry0 {}", geom_type_name(t)` |
| `geom_overlap` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2388` | `"PxGeometryQuery::overlap unsupported geometry1 {}", geom_type_name(t)` |
| `geom_point_distance` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2316` | `"PxGeometryQuery::pointDistance unsupported cooked geometry {}", geom_type_name(gtype)` |
| `geom_raycast` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2426` | `"PxGeometryQuery::raycast unsupported cooked geometry {}", geom_type_name(t)` |
| `geom_sweep` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2566` | `"PxGeometryQuery::sweep unsupported geometry0 {}", geom_type_name(t)` |
| `geom_sweep` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2576` | `"PxGeometryQuery::sweep unsupported geometry1 {}", geom_type_name(t)` |
| `mesh_find_overlap_triangle_mesh` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2614` | `"PxMeshQuery::findOverlapTriangleMesh without cooked triangle mesh payload"` |
| `mesh_get_triangle` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2629` | `"PxMeshQuery::getTriangle without cooked triangle mesh payload"` |
| `mesh_sweep` | `middleware_stub` | `crates/win32-rt/src/misc/physx3common_x64.rs:2642` | `"PxMeshQuery::sweep without cooked triangle mesh payload"` |

### `dxgi.dll` (10)

- **Reason:** Unsupported Windows-only DXGI path (WARP software rasterizer, UWP CoreWindow, exclusive fullscreen ownership, event notifications)
- **Next step:** Implement VSync-synchronized WaitForVBlank via CVDisplayLink/CADisplayLink and shared-handle LUID lookup; keep WARP/CoreWindow honestly unsupported.

| API | Kind | Location | Detail |
|---|---|---|---|
| `DXGIGetDebugInterface1` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:114` | `"DXGIGetDebugInterface1({:?}) -> E_NOINTERFACE (no debug layer)", Guid::read(riid).unwrap_or_default()` |
| `create_software_adapter` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:248` | `"CreateSoftwareAdapter -> DXGI_ERROR_UNSUPPORTED"` |
| `create_swap_chain_unsupported` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:275` | `"CreateSwapChainForCoreWindow/Composition -> DXGI_ERROR_UNSUPPORTED"` |
| `enum_warp_adapter` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:240` | `"EnumWarpAdapter: no WARP software rasterizer -> DXGI_ERROR_NOT_FOUND"` |
| `get_shared_resource_adapter_luid` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:283` | `"GetSharedResourceAdapterLuid: no shared handles -> E_INVALIDARG"` |
| `register_event_stub` | `stub` | `crates/win32-rt/src/dxgi/adapter.rs:298` | `"IDXGIAdapter3 notification event registered; it is never signaled"` |
| `register_status_event` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:298` | `"Register*Event (stereo/occlusion/adapters-changed): the event is never signaled"` |
| `register_status_window` | `stub` | `crates/win32-rt/src/dxgi/mod.rs:290` | `"Register{{Stereo,Occlusion}}StatusWindow: status changes are never posted"` |
| `take_ownership` | `stub` | `crates/win32-rt/src/dxgi/adapter.rs:669` | `"TakeOwnership: exclusive fullscreen is not supported"` |
| `wait_for_vblank` | `stub` | `crates/win32-rt/src/dxgi/adapter.rs:663` | `"WaitForVBlank: sleeps one refresh period ({hz} Hz), not synchronized to the display"` |

### `ntdll.dll` (9)

- **Reason:** Unimplemented NT info class, TxF transaction API, or direct kernel-to-user dispatcher export
- **Next step:** Add missing Process/Thread/System information classes on demand; TxF transactions and direct KiUser* calls remain honestly unsupported.

| API | Kind | Location | Detail |
|---|---|---|---|
| `KiUserApcDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1678` | `"KiUserApcDispatcher called directly"` |
| `KiUserCallbackDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1682` | `"KiUserCallbackDispatcher called directly"` |
| `KiUserExceptionDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1674` | `"KiUserExceptionDispatcher called directly"` |
| `LdrInitializeThunk` | `stub` | `crates/win32-rt/src/ntdll.rs:2420` | `"LdrInitializeThunk called directly"` |
| `NtCommitTransaction` | `stub` | `crates/win32-rt/src/ntdll.rs:747` | `"NtCommitTransaction -> STATUS_NOT_IMPLEMENTED"` |
| `NtCreateTransaction` | `stub` | `crates/win32-rt/src/ntdll.rs:742` | `"NtCreateTransaction -> STATUS_NOT_IMPLEMENTED"` |
| `NtQueryInformationProcess` | `stub` | `crates/win32-rt/src/ntdll.rs:241` | `"NtQueryInformationProcess class {class}"` |
| `NtQueryInformationThread` | `stub` | `crates/win32-rt/src/ntdll.rs:313` | `"NtQueryInformationThread class {class}"` |
| `NtQuerySystemInformation` | `stub` | `crates/win32-rt/src/ntdll.rs:438` | `"NtQuerySystemInformation class {class}"` |

### `advapi32.dll` (8)

- **Reason:** Legacy CryptoAPI (CAPI) key/sign/encrypt or uncommon token information class
- **Next step:** Back remaining CAPI key/cipher calls with CommonCrypto/ring and implement missing TOKEN_INFORMATION_CLASS variants.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CryptDecrypt` | `failing` | `crates/win32-rt/src/advapi32.rs:3010` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptDestroyKey` | `failing` | `crates/win32-rt/src/advapi32.rs:3010` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptEncrypt` | `failing` | `crates/win32-rt/src/advapi32.rs:3010` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptExportKey` | `failing` | `crates/win32-rt/src/advapi32.rs:3010` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptGetUserKey` | `failing` | `crates/win32-rt/src/advapi32.rs:3010` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptImportKey` | `failing` | `crates/win32-rt/src/advapi32.rs:3010` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptSignHashW` | `failing` | `crates/win32-rt/src/advapi32.rs:3010` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `GetTokenInformation` | `stub` | `crates/win32-rt/src/advapi32.rs:1380` | `"GetTokenInformation(class={info_class}) -> ERROR_CALL_NOT_IMPLEMENTED"` |

### `ws2_32.dll` (8)

- **Reason:** Uncommon Winsock2 async/QOS/control-message API or cross-process socket duplication
- **Next step:** Implement WSARecvMsg/WSASendMsg via POSIX recvmsg/sendmsg and remaining WSAIoctl codes as games use them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `WSAAsyncSelect` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSACancelAsyncRequest` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAConnectByNameA` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAConnectByNameW` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAGetQOSByName` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAIoctl` | `stub` | `crates/win32-rt/src/misc/ws2_32.rs:1802` | `"WSAIoctl(code={io_control_code:#x}) -> WSAEOPNOTSUPP"` |
| `WSAJoinLeaf` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `duplicate_socket_into_info` | `stub` | `crates/win32-rt/src/misc/ws2_32.rs:1322` | `"WSADuplicateSocket cross-process target pid={pid} unsupported"` |

### `mmdevapi.dll` (6)

- **Reason:** Optional WASAPI mode/service (exclusive mode, loopback capture, hardware offload, unknown COM IID)
- **Next step:** Add CoreAudio loopback capture / tap support when a game requests AUDCLNT_STREAMFLAGS_LOOPBACK; keep exclusive/offload honestly rejected.

| API | Kind | Location | Detail |
|---|---|---|---|
| `activate` | `stub` | `crates/win32-rt/src/audio/mmdevapi.rs:592` | `"IMMDevice::Activate({iid:?} = {what}) -> E_NOINTERFACE"` |
| `get_buffer_size_limits` | `stub` | `crates/win32-rt/src/audio/audioclient.rs:919` | `"IAudioClient2::GetBufferSizeLimits -> E_NOTIMPL (no offload)"` |
| `get_service` | `stub` | `crates/win32-rt/src/audio/audioclient.rs:885` | `"IAudioClient::GetService({iid:?} = {what}) -> E_NOINTERFACE"` |
| `get_service` | `stub` | `crates/win32-rt/src/audio/spatial.rs:846` | `"ISpatialAudioObjectRenderStream::GetService({iid:?}) -> E_NOINTERFACE"` |
| `initialize` | `stub` | `crates/win32-rt/src/audio/audioclient.rs:427` | `"IAudioClient::Initialize: exclusive mode is not supported -> AUDCLNT_E_EXCLUSIVE_MODE_NOT_ALLOWED"` |
| `initialize` | `stub` | `crates/win32-rt/src/audio/audioclient.rs:440` | `"IAudioClient::Initialize: loopback capture is not supported -> E_NOTIMPL"` |

### `ucrtbase.dll` (6)

- **Reason:** Uncommon CRT locale, multibyte codepage, UTF-16 stream encoding, or interactive console/shell call
- **Next step:** Add UTF-16 FILE stream translation and locale/codepage tables; keep system() safely sandboxed/refused.

| API | Kind | Location | Detail |
|---|---|---|---|
| `_getch` | `stub` | `crates/win32-rt/src/ucrt/runtime.rs:940` | `"_getch returns EOF"` |
| `_setmbcp` | `stub` | `crates/win32-rt/src/ucrt/locale.rs:274` | `"_setmbcp({cp}) unsupported"` |
| `open_host_path` | `stub` | `crates/win32-rt/src/ucrt/stdio.rs:210` | `"Unicode text modes (0x{oflag:x}) treated as plain text mode"` |
| `parse_mode` | `stub` | `crates/win32-rt/src/ucrt/stdio.rs:1062` | `"fopen ccs={c}: UTF-16 stream encoding not implemented; byte stream instead"` |
| `setlocale` | `stub` | `crates/win32-rt/src/ucrt/locale.rs:64` | `"setlocale({cat}, {n:?}): locale not available"` |
| `system` | `stub` | `crates/win32-rt/src/ucrt/runtime.rs:952` | `"system({:?}) refused", from_ansi(cmd)` |

### `dbghelp.dll` (4)

- **Reason:** Unimplemented DbgHelp stack walk or minidump writer
- **Next step:** Wire StackWalk64 to win32-rt's .pdata unwinder and write valid MDMP headers for MiniDumpWriteDump.

| API | Kind | Location | Detail |
|---|---|---|---|
| `MiniDumpWriteDump` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `StackWalk64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `SymGetLineFromAddr64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `SymGetLineFromAddrW64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |

### `secur32.dll` (4)

- **Reason:** Server-side Schannel (INBOUND) or uncommon context/credential attribute
- **Next step:** Add server-side TLS or missing SECPKG_ATTR_* queries if a game hosts local TLS sockets.

| API | Kind | Location | Detail |
|---|---|---|---|
| `acquire_credentials_impl` | `stub` | `crates/win32-rt/src/misc/secur32/schannel.rs:594` | `"AcquireCredentialsHandle(pkg={package_name:?}, use={f_credential_use:#x}): inbound unsupported"` |
| `apply_control_token_impl` | `stub` | `crates/win32-rt/src/misc/secur32/schannel.rs:1025` | `"ApplyControlToken(ctx={id}, token={token_val:?}) -> SEC_E_UNSUPPORTED_FUNCTION"` |
| `query_context_attributes_impl` | `stub` | `crates/win32-rt/src/misc/secur32/schannel.rs:1787` | `"QueryContextAttributes(ctx={ctx_id}, attr={other:#x}) -> SEC_E_UNSUPPORTED_FUNCTION"` |
| `query_credentials_attributes_impl` | `stub` | `crates/win32-rt/src/misc/secur32/schannel.rs:1849` | `"QueryCredentialsAttributes{}(attr={ul_attribute:#x}) -> SEC_E_UNSUPPORTED_FUNCTION", if wide { "W" } else { "A" }` |

### `wintrust.dll` (4)

- **Reason:** Unimplemented WinVerifyTrust action/choice or TPM Base Services (TBS)
- **Next step:** Keep TPM TBS returning TBS_E_TPM_NOT_FOUND honestly; extend WinVerifyTrust for additional catalog/blob choices as needed.

| API | Kind | Location | Detail |
|---|---|---|---|
| `Tbsip_Context_Close` | `stub` | `crates/win32-rt/src/misc/wintrust.rs:1644` | `"Tbsip_Context_Close({h_context:p}) -> TBS_E_INVALID_CONTEXT"` |
| `Tbsip_Submit_Command` | `stub` | `crates/win32-rt/src/misc/wintrust.rs:1661` | `"Tbsip_Submit_Command -> TBS_E_TPM_NOT_FOUND"` |
| `WinVerifyTrust` | `stub` | `crates/win32-rt/src/misc/wintrust.rs:1415` | `"WinVerifyTrust(action={a}) -> TRUST_E_ACTION_UNKNOWN"` |
| `WinVerifyTrust` | `stub` | `crates/win32-rt/src/misc/wintrust.rs:1439` | `"WinVerifyTrust(choice={union_choice}) -> TRUST_E_NOSIGNATURE"` |

### `d3d12.dll` (2)

- **Reason:** D3D12 root-signature deserializer COM object not yet implemented
- **Next step:** Parse DXBC/RTS0 root-signature blobs in win32-rt/src/d3d12 and expose ID3D12RootSignatureDeserializer / VersionedDeserializer.

| API | Kind | Location | Detail |
|---|---|---|---|
| `D3D12CreateRootSignatureDeserializer` | `stub` | `crates/win32-rt/src/d3d12/mod.rs:191` | `"D3D12CreateRootSignatureDeserializer({size} bytes, {:?}) -> E_NOTIMPL", Guid::read(riid).unwrap_or_default()` |
| `D3D12CreateVersionedRootSignatureDeserializer` | `stub` | `crates/win32-rt/src/d3d12/mod.rs:199` | `"D3D12CreateVersionedRootSignatureDeserializer({size} bytes, {:?}) -> E_NOTIMPL", Guid::read(riid).unwrap_or_default()` |

### `libcurl.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `easy_getinfo` | `middleware_stub` | `crates/win32-rt/src/misc/libcurl/easy.rs:1344` | `"libcurl: unsupported curl_easy_getinfo(info={other:#x})"` |
| `easy_setopt` | `middleware_stub` | `crates/win32-rt/src/misc/libcurl/easy.rs:1158` | `"libcurl: unsupported curl_easy_setopt(option={other}, param={param:#x})"` |

### `nri.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `core_cmd_set_sample_positions` | `middleware_stub` | `crates/win32-rt/src/misc/nri/cmdlist.rs:965` | `"CmdSetSamplePositions: programmable sample positions not supported on Metal"` |
| `core_create_query_pool` | `middleware_stub` | `crates/win32-rt/src/misc/nri/device.rs:1383` | `"CreateQueryPool: queryType {} is not supported on Metal", qd.query_type` |

### `xaudio2_9redist.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `parse_wave_format` | `middleware_stub` | `crates/win32-rt/src/misc/xaudio2_9redist/mixer.rs:216` | `"XAudio2 CreateSourceVoice: unsupported WAVE_FORMAT_EXTENSIBLE SubFormat {sub:?}"` |
| `parse_wave_format` | `middleware_stub` | `crates/win32-rt/src/misc/xaudio2_9redist/mixer.rs:233` | `"XAudio2 CreateSourceVoice: unsupported format tag {tag:#x} bits {bits}"` |

### `directml.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `parse` | `middleware_stub` | `crates/win32-rt/src/misc/directml/ops.rs:1048` | `"DirectML CreateOperator: unsupported operator type {other}"` |

### `dsound.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `dsb_set_fx` | `stub` | `crates/win32-rt/src/misc/dsound/com_vtbl.rs:1458` | `"IDirectSoundBuffer8::SetFX({dw_effects_count} effects) -> DSERR_FXUNAVAILABLE"` |

### `shell32.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `SHCreateItemFromParsingName` | `stub` | `crates/win32-rt/src/misc/shell32.rs:1848` | `"SHCreateItemFromParsingName: unsupported shell namespace {raw:?}"` |

