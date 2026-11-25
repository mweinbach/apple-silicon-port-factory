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
