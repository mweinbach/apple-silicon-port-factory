# Stub Backlog & Zero-Stub Tracker

This index is generated from the checked-in static inventory [`db/stub_inventory.tsv`](../db/stub_inventory.tsv)
and aggregates every per-thread backlog file placed in [`docs/stub-backlog.d/`](stub-backlog.d/).

> **Zero-stub policy:** Do not add new `stub!()`, `failing!()`, or `win64_stubs!()` sites anywhere in `crates/`.
> Both `cargo opsdb -- doctor` and `.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh` fail when a new site appears.
> Regenerate this file and `db/stub_inventory.tsv` after eliminating stubs with `cargo opsdb -- stub-sync`.

## Summary

| Metric | Count |
|---|---:|
| Total remaining stub sites | **239** |
| `stub` (partial Win32/OS/D3D/UCRT branches) | 144 |
| `failing` (honest-failing macro exports) | 40 |
| `middleware_stub` (third-party/engine middleware fallbacks) | 55 |
| DLLs with remaining stubs | 43 |

## Per-Thread Backlog Notes (`docs/stub-backlog.d/`)

| File | Summary |
|---|---|
| [`graphics.md`](stub-backlog.d/graphics.md) | Stub Backlog — D3D11, D3D12 and DXGI (`stubs/graphics`) |
| [`guest-corpus-bench.md`](stub-backlog.d/guest-corpus-bench.md) | Stub & API Gap Backlog — `toolkit/guest-corpus-bench` |

### From `docs/stub-backlog.d/graphics.md`

# Stub Backlog — D3D11, D3D12 and DXGI (`stubs/graphics`)

Scope: 26 stub sites across `d3d12/mod.rs` (2), `dxgi/mod.rs` (7), `dxgi/adapter.rs` (3),
`d3d11/device.rs` (8), `d3d11/context.rs` (5), and `d3d11/resources.rs` (1).

- **Before:** 26 sites
- **Made real:** 17 sites
- **Native-unsupported (matches Windows without the hardware/OS feature):** 6 sites
- **Honest leftovers remaining in `db/stub_inventory.tsv`:** 3 sites

## Remaining Leftovers in Scope (3 sites)

| DLL!API | Location | Why Not Implemented Yet | Next Step |
|---|---|---|---|
| `d3d11.dll!ID3D11Device::CreateGeometryShaderWithStreamOutput` | `crates/win32-rt/src/d3d11/device.rs` | Apple Metal Shader Converter does not expose stream-out synthesis in its public C header (`metal_irconverter.h`; `IRCompilerSetStreamOutEnabled` / `IRCreateStreamOutFunction` are private undocumented C++ symbols that must not be called). Lowering arbitrary DXBC/DXIL stream-out shaders cleanly requires a DXIL LLVM 3.7 bitcode rewriter that translates `dx.op.storeOutput` / `dx.op.emitStream` into `RWByteAddressBuffer` UAV writes with per-stream atomic vertex counters according to `D3D11_SO_DECLARATION_ENTRY` strides. | Implement a clean-room DXIL stream-out bitcode lowering pass in `crates/dxil-metal` (rewriting stream-out outputs to internal UAV slots + atomic counter buffers), bind `so_targets` UAVs in `d3d11/context.rs`, and verify with a Metal 4 GPU E2E readback test. |
| `d3d11.dll!ID3D11DeviceContext::DrawAuto` | `crates/win32-rt/src/d3d11/context.rs` | Depends on `CreateGeometryShaderWithStreamOutput` populating per-stream GPU vertex-count atomic buffers for bound `SOSetTargets` buffers. | Once stream-out writes atomic vertex counts per SO target buffer, dispatch a tiny compute helper (or copy directly into a 16-byte `DrawInstancedIndirect` argument buffer `{vertexCount, 1, 0, 0}`) and execute via `CommandList::execute_indirect`. |
| `d3d11.dll!ID3D11DeviceContext::GetData` (`D3D11_QUERY_PIPELINE_STATISTICS`) | `crates/win32-rt/src/d3d11/context.rs` | CPU-submitted counts (`IAVertices`, `IAPrimitives`, `VSInvocations`, `GSInvocations`, `HSInvocations`, `CSInvocations`) are now accurately accumulated between `Begin` and `End`, but GPU-side counters (`CInvocations`, `CPrimitives`, `PSInvocations`, `GSPrimitives`, `DSInvocations`, and indirect draw counts) cannot be sampled on Apple Silicon because `MTLDevice.counterSets` only exposes `timestamp` (`GPUTimestamp`) and `MTL4CounterHeapType` only defines `Invalid` and `Timestamp` (no `MTLCommonCounterSetStatistic`). The `stub!` stays in place so GPU-side zeroes are never reported silently. | When Apple Metal exposes stage-invocation/clipper hardware counter sets on Apple Silicon, wire them through `d3d12-metal::metal::query` (`MTL4CounterHeap`), or add an opt-in shader-instrumentation pass for `PSInvocations`/`GSPrimitives`/`DSInvocations`. |

## Native-Unsupported (Matches Windows Without the Feature — 6 sites)

| DLL!API | Location | Return Code & Behavior | Microsoft Learn Reference |
|---|---|---|---|
| `dxgi.dll!DXGIGetDebugInterface1` | `crates/win32-rt/src/dxgi/mod.rs` | Validates `ppDebug` (`E_INVALIDARG` if null; zeroes `*ppDebug`) and returns `E_NOINTERFACE` because Windows Graphics Tools (`dxgidebug.dll` / `IDXGIDebug` / `IDXGIInfoQueue`) is not installed. | [DXGIGetDebugInterface1 function](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_3/nf-dxgi1_3-dxgigetdebuginterface1) ("If the Graphics Tools optional feature is not installed, `DXGIGetDebugInterface1` returns `E_NOINTERFACE`") |
| `dxgi.dll!IDXGIFactory4::EnumWarpAdapter` | `crates/win32-rt/src/dxgi/mod.rs` | Validates `ppvAdapter` (`E_INVALIDARG` if null; zeroes `*ppvAdapter`) and returns `DXGI_ERROR_NOT_FOUND` because Microsoft's WARP (`d3d10warp.dll`) software rasterizer is not installed. | [IDXGIFactory4::EnumWarpAdapter method](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_4/nf-dxgi1_4-idxgifactory4-enumwarpadapter) & [DXGI_ERROR](https://learn.microsoft.com/en-us/windows/win32/direct3ddxgi/dxgi-error) |
| `dxgi.dll!IDXGIFactory::CreateSoftwareAdapter` | `crates/win32-rt/src/dxgi/mod.rs` | Validates `Module` and `ppAdapter` (`E_INVALIDARG` if null; zeroes `*ppAdapter`) and returns `DXGI_ERROR_UNSUPPORTED` because Windows user-mode software display driver DLLs cannot execute as native Metal adapters. | [IDXGIFactory::CreateSoftwareAdapter method](https://learn.microsoft.com/en-us/windows/win32/api/dxgi/nf-dxgi-idxgifactory-createsoftwareadapter) |
| `dxgi.dll!IDXGIFactory2::CreateSwapChainForCoreWindow` & `CreateSwapChainForComposition` | `crates/win32-rt/src/dxgi/mod.rs` | Split into distinct Win64 methods validating `pDevice`/`pWindow`/`pDesc`/`ppSwapChain` (`E_INVALIDARG` on null, zeroes `*ppSwapChain`) and returning `DXGI_ERROR_UNSUPPORTED` because UWP `CoreWindow` and DirectComposition swap chains do not exist outside WinRT/DComp. | [IDXGIFactory2::CreateSwapChainForCoreWindow](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-createswapchainforcorewindow) & [CreateSwapChainForComposition](https://learn.microsoft.com/en-us/windows/win32/api/dxgi1_2/nf-dxgi1_2-idxgifactory2-createswapchainforcomposition) |
| `d3d11.dll!ID3D11Device::CheckCounter` | `crates/win32-rt/src/d3d11/device.rs` | Validates `pDesc` (`E_INVALIDARG` if null) and returns `DXGI_ERROR_UNSUPPORTED`, matching Windows D3D11 drivers whose `CheckCounterInfo` reports `LastDeviceDependentCounter == 0` (Apple Silicon exposes no device-dependent performance counters). | [ID3D11Device::CheckCounter method](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11device-checkcounter) & [ID3D11Device::CheckCounterInfo](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11device-checkcounterinfo) |
| `d3d11.dll!ID3D11Device::CreateCounter` | `crates/win32-rt/src/d3d11/device.rs` | Zeroes `*ppCounter`, validates `pCounterDesc` (`E_INVALIDARG` if null), and returns `DXGI_ERROR_UNSUPPORTED`, matching Windows D3D11 drivers that expose zero device-dependent counters. | [ID3D11Device::CreateCounter method](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11device-createcounter) |

## Resolved Sites — Made Real (17 sites)

| DLL!API | Location | Implementation Summary |
|---|---|---|
| `d3d12.dll!D3D12CreateRootSignatureDeserializer` | `crates/win32-rt/src/d3d12/mod.rs` | Parses DXBC/`RTS0` blobs via `dxil_metal::root_signature::RootSignatureDesc::from_blob` and returns a heap-stable `ID3D12RootSignatureDeserializer` COM object exposing `GetRootSignatureDesc` in exact Windows x64 ABI layout. |
| `d3d12.dll!D3D12CreateVersionedRootSignatureDeserializer` | `crates/win32-rt/src/d3d12/mod.rs` | Returns a real `ID3D12VersionedRootSignatureDeserializer` supporting `GetUnconvertedRootSignatureDesc` and `GetRootSignatureDescAtVersion` with lossless v1.0 ↔ v1.1 ↔ v1.2 conversion (rejecting lossy v1.2 static-sampler-flag down-conversions with `E_INVALIDARG`). |
| `dxgi.dll!IDXGIFactory2::GetSharedResourceAdapterLuid` | `crates/win32-rt/src/dxgi/mod.rs` | Resolves live D3D11 NT shared handles and legacy KMT handles in `d3d11::resources` and writes the active adapter LUID (`DXGI_ERROR_INVALID_CALL` / `E_INVALIDARG` on invalid/null inputs). |
| `dxgi.dll!IDXGIFactory2::Register{Occlusion,Stereo}StatusWindow` | `crates/win32-rt/src/dxgi/mod.rs` | Real cookie-keyed window notification registries wired to AppKit `windowDidChangeOcclusionState:` (`PostMessageW(hwnd, wMsg, 0, 0)`) with matching `Unregister*Status`. |
| `dxgi.dll!IDXGIFactory2::Register{Occlusion,Stereo}StatusEvent` & `IDXGIFactory7::RegisterAdaptersChangedEvent` | `crates/win32-rt/src/dxgi/mod.rs` | Real cookie-keyed Win32 `HANDLE` event notification registries wired to AppKit `windowDidChangeOcclusionState:` and `applicationDidChangeScreenParameters:` (`SetEvent`) with matching `Unregister*`. |
| `dxgi.dll!IDXGIAdapter3::RegisterHardwareContentProtectionTeardownStatusEvent` | `crates/win32-rt/src/dxgi/adapter.rs` | Real cookie-keyed HCP teardown event registry with `UnregisterHardwareContentProtectionTeardownStatusEvent` and `notify_hcp_teardown()`. |
| `dxgi.dll!IDXGIOutput::WaitForVBlank` | `crates/win32-rt/src/dxgi/adapter.rs` | Synchronizes to the macOS display refresh via CoreVideo `CVDisplayLink` (`CVDisplayLinkCreateWithCGDisplay` + callback condvar wake + headless/timeout monotonic fallback; iOS-safe cfg gating). |
| `dxgi.dll!IDXGIOutput::TakeOwnership` / `ReleaseOwnership` | `crates/win32-rt/src/dxgi/adapter.rs` | Per-output exclusive ownership tracking with `DXGI_ERROR_NOT_CURRENTLY_AVAILABLE` conflict reporting, `E_INVALIDARG` on null device, and automatic cleanup on `Drop`. |
| `d3d11.dll!IDXGIDevice::CreateSurface` | `crates/win32-rt/src/d3d11/device.rs` | Creates `NumSurfaces` real Metal-backed `IDXGISurface2` objects translating `DXGI_USAGE_*` flags into `D3D11_BIND_*` / `D3D11_CPU_ACCESS_*` flags, with rollback on failure. |
| `d3d11.dll!IDXGISurface1::GetDC` / `ReleaseDC` | `crates/win32-rt/src/d3d11/resources.rs` | Validates `D3D11_RESOURCE_MISC_GDI_COMPATIBLE`, non-MSAA, and `B8G8R8A8_UNORM(_SRGB)`; optionally reads back GPU texture when `Discard == FALSE`; creates a real `gdi32` top-down 32bpp BGRA `HDC` backed by staging memory; uploads dirty rect or full surface to Metal on `ReleaseDC`. |
| `d3d11.dll!ID3D11Device::CreateClassLinkage` | `crates/win32-rt/src/d3d11/device.rs` | Returns a real `ID3D11ClassLinkage` COM object implementing `GetClassInstance` and `CreateClassInstance`, returning real `ID3D11ClassInstance` objects (`GetClassLinkage`, `GetDesc`, `GetInstanceName`, `GetTypeName`) tracked on shader slots. |
| `d3d11.dll!ID3D11Device1::CreateDeviceContextState` | `crates/win32-rt/src/d3d11/device.rs` | Validates flags, feature-level array, SDK version, and emulated interface IID (`ID3D10Device` / `ID3D10Device1` / `ID3D11Device` / `ID3D11Device1`), selects the highest supported feature level, and creates a real `ID3DDeviceContextState` holding full `ContextState`. |
| `d3d11.dll!ID3D11DeviceContext1::SwapDeviceContextState` | `crates/win32-rt/src/d3d11/context.rs` | Lazily materializes the initial context state, swaps the entire immediate `ContextState` atomically with the target `ID3DDeviceContextState`, and returns the previous state in `ppPreviousState`. |
| `d3d11.dll!ID3D11Device::CreateHullShader` | `crates/win32-rt/src/d3d11/device.rs` | Validates DXBC container bytecode (`E_INVALIDARG` on invalid input, `S_FALSE` when `ppHullShader == NULL`) and creates a real `ID3D11HullShader` COM object. |
| `d3d11.dll!ID3D11Device::CreateDomainShader` | `crates/win32-rt/src/d3d11/device.rs` | Validates DXBC container bytecode (`E_INVALIDARG` on invalid input, `S_FALSE` when `ppDomainShader == NULL`) and creates a real `ID3D11DomainShader` COM object. |
| `d3d11.dll!ID3D11DeviceContext::HSSetShader` | `crates/win32-rt/src/d3d11/context.rs` | Binds `ID3D11HullShader` + class instances onto context state (round-tripped via `HSGetShader`), populates HS CBV/SRV/Sampler root tables (params 6 & 7), and compiles Metal 4 object+mesh tessellation emulation pipelines via `GraphicsPipeline::new_with_tessellation` (with MSC Apache-2.0 tessellator lookup tables at bind point 7). |
| `d3d11.dll!ID3D11DeviceContext::DSSetShader` | `crates/win32-rt/src/d3d11/context.rs` | Binds `ID3D11DomainShader` + class instances onto context state (round-tripped via `DSGetShader`), populates DS CBV/SRV/Sampler root tables (params 8 & 9), and executes full VS+HS+DS(+optional GS)+PS tessellation draws on Metal 4. |

### From `docs/stub-backlog.d/guest-corpus-bench.md`

# Stub & API Gap Backlog — `toolkit/guest-corpus-bench`

Exposed while building and running the headless Windows x86-64 guest corpus (`crates/port-loader/tests/os_guest_corpus.rs` and `crates/port-loader/tests/d3d12_e2e.rs`).

| DLL!API | What the test showed | Next step |
|---|---|---|
| `user32.dll!wsprintfW` | Missing export in `crates/win32-rt/src/user32/misc.rs`: guest binaries importing `wsprintfW` hit an unresolved-import trap thunk instead of formatting UTF-16 output. | Implement variadic `wsprintfW` via `win64_varargs_thunk!` backed by `ucrt::stdio` wide formatter (1024-char Windows cap) and run `cargo opsdb -- win32-sync`. |
| `user32.dll!wsprintfA` | Existing implementation in `crates/win32-rt/src/user32/misc.rs` copies the raw format string and ignores variadic arguments (`let _ = (a1, a2);`). | Replace with a real `win64_varargs_thunk!` formatter backed by `ucrt::stdio` narrow printf formatting (1024-char Windows cap). |

## Proprietary & Vendor Middleware (Legal / Architectural Limits)

These modules wrap proprietary third-party SDKs or GPU-vendor-specific extensions. Where a game ships its own Windows x86-64 PE DLL (e.g., Oodle `oo2core_*_win64.dll`, PhysX 4/5 `PhysX_64.dll`, `zstd.dll`), `port-runner` executes the real shipping PE through the ARM64 JIT rather than overriding it.

| DLL | Remaining Sites | Reason | Next Step |
|---|---:|---|---|
| `amd_ags_x64.dll` | 3 | AMD GPU Services vendor extension on non-D3D12 command list | Forward recognized D3D12 debug markers to Metal command-buffer labels; return failure on foreign pointers. |
| `amd_fidelityfx_loader_dx12.dll` | 6 | AMD FidelityFX Frame Generation / unsupported FFX context variant | Keep upscaler contexts routed to MetalFX; report FFX_API_RETURN_NO_PROVIDER honestly for frame-gen until Metal 4 frame interpolation is wired. |
| `libxess.dll` | 1 | Intel XeSS debug dump / Intel GPU crash-dump vendor extension (INTC_D3D12_*) | Keep XeSS execute routed through MetalFX; leave proprietary Intel GPA/crash-dump callbacks as documented unsupported hooks. |
| `libxess_dx11.dll` | 9 | Intel XeSS debug dump / Intel GPU crash-dump vendor extension (INTC_D3D12_*) | Keep XeSS execute routed through MetalFX; leave proprietary Intel GPA/crash-dump callbacks as documented unsupported hooks. |
| `nvngx_dlss.dll` | 5 | Proprietary NVIDIA NGX DLSS / Vulkan NGX extension | Route D3D12 NGX features through MetalFX temporal/spatial scalers; implement Vulkan NGX -> MetalFX or MoltenVK bridge when needed. |
| `oo2core_9_win64.dll` | 2 | Proprietary RAD Oodle compression (cannot be clean-room reimplemented legally without a license) | Always run the game's shipping oo2core_*_win64.dll PE via x86-runtime ARM64 JIT; keep this honest fallback returning 0 only when no shipping DLL exists. |
| `physx_64.dll` | 4 | NVIDIA PhysX 4.x/5.x C++ ABI (vtable layout varies by build; shipping DLL runs natively under ARM64 JIT) | Run shipping PhysX_64.dll / PhysXCooking_64.dll under ARM64 JIT via [middleware.physx] mode = "guest". |
| `physxcooking_64.dll` | 1 | NVIDIA PhysX 4.x/5.x C++ ABI (vtable layout varies by build; shipping DLL runs natively under ARM64 JIT) | Run shipping PhysX_64.dll / PhysXCooking_64.dll under ARM64 JIT via [middleware.physx] mode = "guest". |
| `sl.interposer.dll` | 2 | Proprietary NVIDIA Streamline plugin host (requires NVIDIA GPU + signed NGX plugins) | Keep honest eErrorFeatureNotSupported reporting; route DLSS/upscaling requests to MetalFX via nvngx_dlss/libxess/FFX bridges. |

## Remaining Stubs by DLL

| DLL | Count | Reason | Next Step |
|---|---:|---|---|
| `kernel32.dll` | 34 | Partial Windows kernel/file/memory/locale/job/threadpool sub-feature | Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives. |
| `bcrypt.dll` | 20 | Unimplemented CNG algorithm, key-wrap blob, or padding mode | Add remaining CNG algorithms/padding modes (OAEP/PSS/ECDH/3DES) backed by CommonCrypto/ring as games import them. |
| `crypt32.dll` | 19 | Unimplemented CryptoAPI ASN.1 decode, PKCS#7/PFX, or certificate store property | Implement remaining DER/ASN.1 object decoders, PFX import, and CryptMsg verification on top of rustls/ring. |
| `user32.dll` | 13 | Uncommon Win32 window/metric/SPI/hotkey/cursor/display-config sub-feature | Fill in remaining GetSystemMetrics / SystemParametersInfo / DisplayConfig cases and custom bitmap NSCursor creation. |
| `ole32.dll` | 12 | Unimplemented WinRT IPropertyValue scalar/array type or VARIANT/SAFEARRAY coercion | Implement remaining IPropertyValue* types and VariantChangeType/SafeArrayDestroy coercions. |
| `physx3common_x64.dll` | 12 | PhysX 3.4 geometry/mesh query path without cooked mesh payload | Prefer shipping PhysX3*.dll under ARM64 JIT, or decode cooked BVH/triangle mesh payloads in physx3common_x64. |
| `vulkan_1.dll` | 10 | Proprietary NVIDIA Reflex Low Latency Vulkan extension (NvLL_VK_*) | Return NVLL_VK_NO_IMPLEMENTATION honestly on Apple Silicon, or map sleep/marker semantics to Metal frame pacing. |
| `ws2_32.dll` | 10 | Uncommon Winsock2 async/QOS/control-message API or cross-process socket duplication | Implement WSARecvMsg/WSASendMsg via POSIX recvmsg/sendmsg and remaining WSAIoctl codes as games use them. |
| `libxess_dx11.dll` | 9 | Intel XeSS debug dump / Intel GPU crash-dump vendor extension (INTC_D3D12_*) | Keep XeSS execute routed through MetalFX; leave proprietary Intel GPA/crash-dump callbacks as documented unsupported hooks. |
| `ntdll.dll` | 9 | Unimplemented NT info class, TxF transaction API, or direct kernel-to-user dispatcher export | Add missing Process/Thread/System information classes on demand; TxF transactions and direct KiUser* calls remain honestly unsupported. |
| `advapi32.dll` | 8 | Legacy CryptoAPI (CAPI) key/sign/encrypt or uncommon token information class | Back remaining CAPI key/cipher calls with CommonCrypto/ring and implement missing TOKEN_INFORMATION_CLASS variants. |
| `amd_fidelityfx_loader_dx12.dll` | 6 | AMD FidelityFX Frame Generation / unsupported FFX context variant | Keep upscaler contexts routed to MetalFX; report FFX_API_RETURN_NO_PROVIDER honestly for frame-gen until Metal 4 frame interpolation is wired. |
| `mmdevapi.dll` | 6 | Optional WASAPI mode/service (exclusive mode, loopback capture, hardware offload, unknown COM IID) | Add CoreAudio loopback capture / tap support when a game requests AUDCLNT_STREAMFLAGS_LOOPBACK; keep exclusive/offload honestly rejected. |
| `ucrtbase.dll` | 6 | Uncommon CRT locale, multibyte codepage, UTF-16 stream encoding, or interactive console/shell call | Add UTF-16 FILE stream translation and locale/codepage tables; keep system() safely sandboxed/refused. |
| `dinput8.dll` | 5 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `nvngx_dlss.dll` | 5 | Proprietary NVIDIA NGX DLSS / Vulkan NGX extension | Route D3D12 NGX features through MetalFX temporal/spatial scalers; implement Vulkan NGX -> MetalFX or MoltenVK bridge when needed. |
| `dbghelp.dll` | 4 | Unimplemented DbgHelp stack walk or minidump writer | Wire StackWalk64 to win32-rt's .pdata unwinder and write valid MDMP headers for MiniDumpWriteDump. |
| `dwrite.dll` | 4 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `physx_64.dll` | 4 | NVIDIA PhysX 4.x/5.x C++ ABI (vtable layout varies by build; shipping DLL runs natively under ARM64 JIT) | Run shipping PhysX_64.dll / PhysXCooking_64.dll under ARM64 JIT via [middleware.physx] mode = "guest". |
| `secur32.dll` | 4 | Server-side Schannel (INBOUND) or uncommon context/credential attribute | Add server-side TLS or missing SECPKG_ATTR_* queries if a game hosts local TLS sockets. |
| `wintrust.dll` | 4 | Unimplemented WinVerifyTrust action/choice or TPM Base Services (TBS) | Keep TPM TBS returning TBS_E_TPM_NOT_FOUND honestly; extend WinVerifyTrust for additional catalog/blob choices as needed. |
| `amd_ags_x64.dll` | 3 | AMD GPU Services vendor extension on non-D3D12 command list | Forward recognized D3D12 debug markers to Metal command-buffer labels; return failure on foreign pointers. |
| `d3d11.dll` | 3 | Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop) | Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them. |
| `powrprof.dll` | 3 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `libcurl.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `nri.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `oo2core_9_win64.dll` | 2 | Proprietary RAD Oodle compression (cannot be clean-room reimplemented legally without a license) | Always run the game's shipping oo2core_*_win64.dll PE via x86-runtime ARM64 JIT; keep this honest fallback returning 0 only when no shipping DLL exists. |
| `sl.interposer.dll` | 2 | Proprietary NVIDIA Streamline plugin host (requires NVIDIA GPU + signed NGX plugins) | Keep honest eErrorFeatureNotSupported reporting; route DLSS/upscaling requests to MetalFX via nvngx_dlss/libxess/FFX bridges. |
| `uxtheme.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `xaudio2_9redist.dll` | 2 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `zstd.dll` | 2 | Open-source Zstandard compression fallback (no native zstd crate linked yet) | Run shipping zstd.dll via ARM64 JIT, or add a clean-room / zstd-safe native backend with unit round-trip tests. |
| `d2d1.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `directml.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `dsound.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `dstorage.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `gameinput.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `gdi32.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `hid.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `libxess.dll` | 1 | Intel XeSS debug dump / Intel GPU crash-dump vendor extension (INTC_D3D12_*) | Keep XeSS execute routed through MetalFX; leave proprietary Intel GPA/crash-dump callbacks as documented unsupported hooks. |
| `msctf.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `physxcooking_64.dll` | 1 | NVIDIA PhysX 4.x/5.x C++ ABI (vtable layout varies by build; shipping DLL runs natively under ARM64 JIT) | Run shipping PhysX_64.dll / PhysXCooking_64.dll under ARM64 JIT via [middleware.physx] mode = "guest". |
| `shell32.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |
| `xinput1_4.dll` | 1 | Partial or unimplemented Windows/middleware sub-feature | Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test. |

### `kernel32.dll` (34)

- **Reason:** Partial Windows kernel/file/memory/locale/job/threadpool sub-feature
- **Next step:** Implement missing info classes, PAGE_GUARD/MEM_WRITE_WATCH fault tracking, and threadpool cleanup-group tracking on macOS primitives.

| API | Kind | Location | Detail |
|---|---|---|---|
| `AllocConsole` | `stub` | `crates/win32-rt/src/kernel32/file.rs:475` | `"no separate console window; stdout/stderr of the loader act as the console"` |
| `AssignProcessToJobObject` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1655` | `"job objects have no effect"` |
| `CloseThreadpoolCleanupGroupMembers` | `stub` | `crates/win32-rt/src/kernel32/threadpool.rs:808` | `"cleanup groups do not track members"` |
| `CreateProcessW` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1283` | `"CreateProcessW unavailable on iOS: app={:?} cmd={:?}", from_wide_opt(app), from_wide_opt(cmd)` |
| `CreateRemoteThread` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:60` | `"CreateRemoteThread into another process"` |
| `DeviceIoControl` | `stub` | `crates/win32-rt/src/kernel32/file.rs:7057` | `"IOCTL_STORAGE_QUERY_PROPERTY property {prop} unsupported"` |
| `DeviceIoControl` | `stub` | `crates/win32-rt/src/kernel32/file.rs:7085` | `"DeviceIoControl code {code:#x} unsupported"` |
| `FlushProcessWriteBuffers` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:1028` | `"FlushProcessWriteBuffers approximated by a local fence"` |
| `GetCalendarInfoEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1354` | `"GetCalendarInfoEx type {ty:#x}"` |
| `GetFileInformationByHandleEx` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2111` | `"GetFileInformationByHandleEx class {class} not supported"` |
| `GetGeoInfoW` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1125` | `"GetGeoInfo for geo id {geo} answered with US data"` |
| `GetLocaleInfoW` | `stub` | `crates/win32-rt/src/kernel32/string.rs:918` | `"locale {lcid:#x} not available; answering with en-US data"` |
| `GetNumberFormatEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:1323` | `"custom NUMBERFMT ignored"` |
| `InitOnceBeginInitialize` | `stub` | `crates/win32-rt/src/sync.rs:744` | `"INIT_ONCE_ASYNC treated as synchronous"` |
| `LCMapStringEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:552` | `"LCMAP_HASH unsupported"` |
| `LCMapStringEx` | `stub` | `crates/win32-rt/src/kernel32/string.rs:578` | `"LCMapString flags {flags:#x} partially honoured"` |
| `MapViewOfFile3` | `stub` | `crates/win32-rt/src/kernel32/file.rs:6848` | `"MapViewOfFile3 on another process"` |
| `MoveFileExA` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2526` | `"MoveFileExA delete-on-reboot ignored"` |
| `MoveFileExW` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2519` | `"MoveFileExW delete-on-reboot ignored"` |
| `SetFileInformationByHandle` | `stub` | `crates/win32-rt/src/kernel32/file.rs:2198` | `"SetFileInformationByHandle class {class} not supported"` |
| `SetInformationJobObject` | `stub` | `crates/win32-rt/src/kernel32/process.rs:1659` | `"SetInformationJobObject class {class} ignored"` |
| `SetThreadAffinityMask` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:244` | `"thread affinity not applied"` |
| `SetThreadGroupAffinity` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:250` | `"thread group affinity not applied"` |
| `TerminateThread` | `stub` | `crates/win32-rt/src/kernel32/threads.rs:173` | `"TerminateThread unsupported"` |
| `VirtualAlloc2` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:575` | `"VirtualAlloc2 on another process"` |
| `VirtualAllocEx` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:566` | `"VirtualAllocEx on another process"` |
| `VirtualProtect` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:678` | `"PAGE_GUARD not emulated"` |
| `format_message` | `stub` | `crates/win32-rt/src/kernel32/process.rs:2180` | `"message tables in modules are not supported (id {id:#x})"` |
| `format_message` | `stub` | `crates/win32-rt/src/kernel32/process.rs:2189` | `"FormatMessage from module message table (id {id:#x})"` |
| `get_locale_info` | `stub` | `crates/win32-rt/src/kernel32/string.rs:865` | `"GetLocaleInfo LCTYPE {lctype:#x} unknown"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:362` | `"MEM_LARGE_PAGES ignored"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:365` | `"MEM_WRITE_WATCH not tracked"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:368` | `"PAGE_GUARD not emulated (treated as normal pages)"` |
| `virtual_alloc_aligned_impl` | `stub` | `crates/win32-rt/src/kernel32/memory.rs:375` | `"MEM_RESET_UNDO: contents may have been discarded"` |

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

### `user32.dll` (13)

- **Reason:** Uncommon Win32 window/metric/SPI/hotkey/cursor/display-config sub-feature
- **Next step:** Fill in remaining GetSystemMetrics / SystemParametersInfo / DisplayConfig cases and custom bitmap NSCursor creation.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CreateCursor` | `stub` | `crates/win32-rt/src/user32/cursor.rs:987` | `"CreateCursor {w}x{h}: custom bitmap cursors are shown as the arrow"` |
| `DisplayConfigGetDeviceInfo` | `stub` | `crates/win32-rt/src/user32/monitor.rs:684` | `"DisplayConfigGetDeviceInfo(type {ty}) -> ERROR_NOT_SUPPORTED"` |
| `DisplayConfigSetDeviceInfo` | `stub` | `crates/win32-rt/src/user32/monitor.rs:692` | `"DisplayConfigSetDeviceInfo(type {ty}) -> ERROR_NOT_SUPPORTED"` |
| `GetSystemMetrics` | `stub` | `crates/win32-rt/src/user32/misc.rs:234` | `"GetSystemMetrics({index}) -> 0"` |
| `RegisterHotKey` | `stub` | `crates/win32-rt/src/user32/misc.rs:759` | `"RegisterHotKey(hwnd {:#x}, id {id}, mods {mods:#x}, vk {vk:#x}): never fires", hwnd as usize` |
| `RegisterRawInputDevices` | `stub` | `crates/win32-rt/src/user32/input.rs:1363` | `"RegisterRawInputDevices: RIDEV_NOLEGACY ignored (legacy messages still sent)"` |
| `ReplyMessage` | `stub` | `crates/win32-rt/src/user32/message.rs:883` | `"ReplyMessage: early reply not supported"` |
| `def_window_proc` | `stub` | `crates/win32-rt/src/user32/defproc.rs:154` | `"DefWindowProc(WM_SYSCOMMAND {other:#x}) ignored"` |
| `load_cursor` | `stub` | `crates/win32-rt/src/user32/cursor.rs:87` | `"LoadCursor({inst:#x}, {n:?}): cursor resources not supported -> NULL"` |
| `message_box` | `stub` | `crates/win32-rt/src/user32/misc.rs:1876` | `"MessageBox type {ty:#x}: headless, cannot ask the user -> 0 (failure)"` |
| `spi` | `stub` | `crates/win32-rt/src/user32/misc.rs:565` | `"SystemParametersInfo{}({action:#x}, {uparam}) not supported -> FALSE", if wide { "W" } else { "A" }` |
| `user_object_info` | `stub` | `crates/win32-rt/src/user32/misc.rs:600` | `"GetUserObjectInformation on handle {h:#x} (only the window station is known)"` |
| `user_object_info` | `stub` | `crates/win32-rt/src/user32/misc.rs:622` | `"GetUserObjectInformation index {index} not supported"` |

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

### `vulkan_1.dll` (10)

- **Reason:** Proprietary NVIDIA Reflex Low Latency Vulkan extension (NvLL_VK_*)
- **Next step:** Return NVLL_VK_NO_IMPLEMENTATION honestly on Apple Silicon, or map sleep/marker semantics to Metal frame pacing.

| API | Kind | Location | Detail |
|---|---|---|---|
| `NvLL_VK_DestroyLowLatencyDevice` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |
| `NvLL_VK_GetLatency` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |
| `NvLL_VK_GetSleepStatus` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |
| `NvLL_VK_InitLowLatencyDevice` | `stub` | `crates/win32-rt/src/misc/vulkan_1.rs:4500` | `"NvLL_VK_InitLowLatencyDevice -> NVLL_VK_NO_IMPLEMENTATION (-3)"` |
| `NvLL_VK_Initialize` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |
| `NvLL_VK_NotifyOutOfBandQueue` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |
| `NvLL_VK_SetLatencyMarker` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |
| `NvLL_VK_SetSleepMode` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |
| `NvLL_VK_Sleep` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |
| `NvLL_VK_Unload` | `failing` | `crates/win32-rt/src/misc/vulkan_1.rs:4504` | `i32 = NVLL_VK_NO_IMPLEMENTATION, err 0` |

### `ws2_32.dll` (10)

- **Reason:** Uncommon Winsock2 async/QOS/control-message API or cross-process socket duplication
- **Next step:** Implement WSARecvMsg/WSASendMsg via POSIX recvmsg/sendmsg and remaining WSAIoctl codes as games use them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `WSAAsyncSelect` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSACancelAsyncRequest` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAConnectByNameA` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAConnectByNameW` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAGetQOSByName` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSAIoctl` | `stub` | `crates/win32-rt/src/misc/ws2_32.rs:1782` | `"WSAIoctl(code={io_control_code:#x}) -> WSAEOPNOTSUPP"` |
| `WSAJoinLeaf` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSARecvMsg` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `WSASendMsg` | `failing` | `crates/win32-rt/src/misc/ws2_32.rs:3357` | `i32 = SOCKET_ERROR, err WSAEOPNOTSUPP` |
| `duplicate_socket_into_info` | `stub` | `crates/win32-rt/src/misc/ws2_32.rs:1319` | `"WSADuplicateSocket cross-process target pid={pid} unsupported"` |

### `libxess_dx11.dll` (9)

- **Reason:** Intel XeSS debug dump / Intel GPU crash-dump vendor extension (INTC_D3D12_*)
- **Next step:** Keep XeSS execute routed through MetalFX; leave proprietary Intel GPA/crash-dump callbacks as documented unsupported hooks.

| API | Kind | Location | Detail |
|---|---|---|---|
| `INTC_D3D12_GetApplicationInfo` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:1186` | `"INTC_D3D12_GetApplicationInfo -> E_NOINTERFACE"` |
| `INTC_D3D12_GetGpuCrashDumpBufferInfo` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:1194` | `"INTC_D3D12_GetGpuCrashDumpBufferInfo (no-op)"` |
| `INTC_D3D12_GpuCrashDumpOnCommandListCreate` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:1201` | `"INTC_D3D12_GpuCrashDumpOnCommandListCreate (no-op)"` |
| `INTC_D3D12_GpuCrashDumpOnCommandListDestroy` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:1208` | `"INTC_D3D12_GpuCrashDumpOnCommandListDestroy (no-op)"` |
| `INTC_D3D12_GpuCrashDumpOnDeviceDestroy` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:1212` | `"INTC_D3D12_GpuCrashDumpOnDeviceDestroy (no-op)"` |
| `INTC_D3D12_RegisterDriverCallbacks` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:1219` | `"INTC_D3D12_RegisterDriverCallbacks (no-op)"` |
| `xessD3D11Execute` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:657` | `"xessD3D11Execute: non-zero subrect base offsets color=({},{}) mv=({},{}) depth=({},{}) reactive=({},{}) out=({},{}) not applied by MetalFX"...` |
| `xessStartDump` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:951` | `"xessStartDump -> XESS_RESULT_ERROR_NOT_IMPLEMENTED"` |
| `xess_set_context_parameter_f_impl` | `middleware_stub` | `crates/win32-rt/src/misc/libxess_dx11.rs:879` | `"xessSetContextParameterF(param={param:#x}, value={value}) -> NOT_IMPLEMENTED"` |

### `ntdll.dll` (9)

- **Reason:** Unimplemented NT info class, TxF transaction API, or direct kernel-to-user dispatcher export
- **Next step:** Add missing Process/Thread/System information classes on demand; TxF transactions and direct KiUser* calls remain honestly unsupported.

| API | Kind | Location | Detail |
|---|---|---|---|
| `KiUserApcDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1662` | `"KiUserApcDispatcher called directly"` |
| `KiUserCallbackDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1666` | `"KiUserCallbackDispatcher called directly"` |
| `KiUserExceptionDispatcher` | `stub` | `crates/win32-rt/src/ntdll.rs:1658` | `"KiUserExceptionDispatcher called directly"` |
| `LdrInitializeThunk` | `stub` | `crates/win32-rt/src/ntdll.rs:2404` | `"LdrInitializeThunk called directly"` |
| `NtCommitTransaction` | `stub` | `crates/win32-rt/src/ntdll.rs:731` | `"NtCommitTransaction -> STATUS_NOT_IMPLEMENTED"` |
| `NtCreateTransaction` | `stub` | `crates/win32-rt/src/ntdll.rs:726` | `"NtCreateTransaction -> STATUS_NOT_IMPLEMENTED"` |
| `NtQueryInformationProcess` | `stub` | `crates/win32-rt/src/ntdll.rs:239` | `"NtQueryInformationProcess class {class}"` |
| `NtQueryInformationThread` | `stub` | `crates/win32-rt/src/ntdll.rs:303` | `"NtQueryInformationThread class {class}"` |
| `NtQuerySystemInformation` | `stub` | `crates/win32-rt/src/ntdll.rs:422` | `"NtQuerySystemInformation class {class}"` |

### `advapi32.dll` (8)

- **Reason:** Legacy CryptoAPI (CAPI) key/sign/encrypt or uncommon token information class
- **Next step:** Back remaining CAPI key/cipher calls with CommonCrypto/ring and implement missing TOKEN_INFORMATION_CLASS variants.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CryptDecrypt` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptDestroyKey` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptEncrypt` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptExportKey` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptGetUserKey` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptImportKey` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `CryptSignHashW` | `failing` | `crates/win32-rt/src/advapi32.rs:2995` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `GetTokenInformation` | `stub` | `crates/win32-rt/src/advapi32.rs:1361` | `"GetTokenInformation(class={info_class}) -> ERROR_CALL_NOT_IMPLEMENTED"` |

### `amd_fidelityfx_loader_dx12.dll` (6)

- **Reason:** AMD FidelityFX Frame Generation / unsupported FFX context variant
- **Next step:** Keep upscaler contexts routed to MetalFX; report FFX_API_RETURN_NO_PROVIDER honestly for frame-gen until Metal 4 frame interpolation is wired.

| API | Kind | Location | Detail |
|---|---|---|---|
| `ffxConfigure` | `middleware_stub` | `crates/win32-rt/src/misc/amd_fidelityfx_loader_dx12.rs:394` | `"ffxConfigure(ty={ty:#x}): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"` |
| `ffxCreateContext` | `middleware_stub` | `crates/win32-rt/src/misc/amd_fidelityfx_loader_dx12.rs:266` | `"ffxCreateContext(FRAMEGENERATION): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"` |
| `ffxDispatch` | `middleware_stub` | `crates/win32-rt/src/misc/amd_fidelityfx_loader_dx12.rs:540` | `"ffxDispatch(GENERATEREACTIVEMASK): cleared outReactive to 0.0 (cleared={cleared}, render={}x{}, scale={:.3}, cutoff={:.3}, binary={:.3}, fl...` |
| `ffxDispatch` | `middleware_stub` | `crates/win32-rt/src/misc/amd_fidelityfx_loader_dx12.rs:558` | `"ffxDispatch(ty={ty:#x}): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"` |
| `ffxQuery` | `middleware_stub` | `crates/win32-rt/src/misc/amd_fidelityfx_loader_dx12.rs:620` | `"ffxQuery(INTERPOLATIONCOMMANDLIST_DX12): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"` |
| `ffxQuery` | `middleware_stub` | `crates/win32-rt/src/misc/amd_fidelityfx_loader_dx12.rs:630` | `"ffxQuery(INTERPOLATIONTEXTURE_DX12): frame generation is not supported -> FFX_API_RETURN_NO_PROVIDER"` |

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

### `dinput8.dll` (5)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `configure_devices` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1862` | `"IDirectInput8::ConfigureDevices -> DIERR_UNSUPPORTED"` |
| `dev_get_property` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1239` | `"IDirectInputDevice8::GetProperty({other:#x}) -> DIERR_UNSUPPORTED"` |
| `dev_run_control_panel` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1557` | `"IDirectInputDevice8::RunControlPanel -> DIERR_UNSUPPORTED"` |
| `dev_set_property` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1331` | `"IDirectInputDevice8::SetProperty({other:#x}) -> DIERR_UNSUPPORTED"` |
| `run_control_panel` | `stub` | `crates/win32-rt/src/misc/dinput8.rs:1822` | `"IDirectInput8::RunControlPanel -> DIERR_UNSUPPORTED"` |

### `nvngx_dlss.dll` (5)

- **Reason:** Proprietary NVIDIA NGX DLSS / Vulkan NGX extension
- **Next step:** Route D3D12 NGX features through MetalFX temporal/spatial scalers; implement Vulkan NGX -> MetalFX or MoltenVK bridge when needed.

| API | Kind | Location | Detail |
|---|---|---|---|
| `NVSDK_NGX_D3D12_CreateFeature` | `middleware_stub` | `crates/win32-rt/src/misc/nvngx_dlss.rs:630` | `"NVSDK_NGX_D3D12_CreateFeature: unsupported feature_id={other} ({}) -> FAIL_FeatureNotSupported", ngx_feature_name(other)` |
| `NVSDK_NGX_VULKAN_CreateFeature` | `middleware_stub` | `crates/win32-rt/src/misc/nvngx_dlss.rs:731` | `"NVSDK_NGX_VULKAN_CreateFeature(feature_id={feature_id} {}): Vulkan NGX is not implemented -> FAIL_FeatureNotSupported", ngx_feature_name(fe...` |
| `NVSDK_NGX_VULKAN_EvaluateFeature` | `middleware_stub` | `crates/win32-rt/src/misc/nvngx_dlss.rs:744` | `"NVSDK_NGX_VULKAN_EvaluateFeature: Vulkan NGX is not implemented -> FAIL_FeatureNotSupported"` |
| `NVSDK_NGX_VULKAN_Init` | `middleware_stub` | `crates/win32-rt/src/misc/nvngx_dlss.rs:702` | `"NVSDK_NGX_VULKAN_Init: Vulkan NGX upscaling is not implemented -> FAIL_FeatureNotSupported"` |
| `NVSDK_NGX_VULKAN_ReleaseFeature` | `middleware_stub` | `crates/win32-rt/src/misc/nvngx_dlss.rs:751` | `"NVSDK_NGX_VULKAN_ReleaseFeature: Vulkan NGX is not implemented -> FAIL_FeatureNotSupported"` |

### `dbghelp.dll` (4)

- **Reason:** Unimplemented DbgHelp stack walk or minidump writer
- **Next step:** Wire StackWalk64 to win32-rt's .pdata unwinder and write valid MDMP headers for MiniDumpWriteDump.

| API | Kind | Location | Detail |
|---|---|---|---|
| `MiniDumpWriteDump` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `StackWalk64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `SymGetLineFromAddr64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |
| `SymGetLineFromAddrW64` | `failing` | `crates/win32-rt/src/misc/dbghelp.rs:503` | `BOOL = FALSE, err ERROR_CALL_NOT_IMPLEMENTED` |

### `dwrite.dll` (4)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `fac_create_custom_font_collection` | `stub` | `crates/win32-rt/src/misc/dwrite.rs:76` | `"IDWriteFactory::CreateCustomFontCollection -> E_NOTIMPL"` |
| `fac_create_custom_font_file_reference` | `stub` | `crates/win32-rt/src/misc/dwrite.rs:119` | `"IDWriteFactory::CreateCustomFontFileReference -> E_NOTIMPL"` |
| `fac_create_glyph_run_analysis` | `stub` | `crates/win32-rt/src/misc/dwrite.rs:365` | `"IDWriteFactory::CreateGlyphRunAnalysis -> E_NOTIMPL"` |
| `fac_create_text_analyzer` | `stub` | `crates/win32-rt/src/misc/dwrite.rs:336` | `"IDWriteFactory::CreateTextAnalyzer -> E_NOTIMPL"` |

### `physx_64.dll` (4)

- **Reason:** NVIDIA PhysX 4.x/5.x C++ ABI (vtable layout varies by build; shipping DLL runs natively under ARM64 JIT)
- **Next step:** Run shipping PhysX_64.dll / PhysXCooking_64.dll under ARM64 JIT via [middleware.physx] mode = "guest".

| API | Kind | Location | Detail |
|---|---|---|---|
| `PxCreateBasePhysics` | `middleware_stub` | `crates/win32-rt/src/misc/physx_64.rs:26` | `"PhysX_64!PxCreateBasePhysics(version={formatted}, detected={detected}, configured={:?}): PhysX 4.x/5.x core simulation requires shipping PE...` |
| `PxCreatePhysics` | `middleware_stub` | `crates/win32-rt/src/misc/physx_64.rs:44` | `"PhysX_64!PxCreatePhysics(version={formatted}, detected={detected}, configured={:?}): PhysX 4.x/5.x core simulation requires shipping PE ([m...` |
| `PxRegisterPhysicsSerializers` | `middleware_stub` | `crates/win32-rt/src/misc/physx_64.rs:56` | `"PhysX_64!PxRegisterPhysicsSerializers: PhysX 4.x/5.x serializers require shipping PE ([middleware.physx] version = \"4.1\" \| \"5.x\", mode ...` |
| `PxUnregisterPhysicsSerializers` | `middleware_stub` | `crates/win32-rt/src/misc/physx_64.rs:62` | `"PhysX_64!PxUnregisterPhysicsSerializers: PhysX 4.x/5.x serializers require shipping PE ([middleware.physx] version = \"4.1\" \| \"5.x\", mod...` |

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

### `amd_ags_x64.dll` (3)

- **Reason:** AMD GPU Services vendor extension on non-D3D12 command list
- **Next step:** Forward recognized D3D12 debug markers to Metal command-buffer labels; return failure on foreign pointers.

| API | Kind | Location | Detail |
|---|---|---|---|
| `agsDriverExtensionsDX12_PopMarker` | `middleware_stub` | `crates/win32-rt/src/misc/amd_ags_x64.rs:406` | `"agsDriverExtensionsDX12_PopMarker: unsupported command list {cmd_list:#x}"` |
| `agsDriverExtensionsDX12_PushMarker` | `middleware_stub` | `crates/win32-rt/src/misc/amd_ags_x64.rs:384` | `"agsDriverExtensionsDX12_PushMarker: unsupported command list {cmd_list:#x}"` |
| `agsDriverExtensionsDX12_SetMarker` | `middleware_stub` | `crates/win32-rt/src/misc/amd_ags_x64.rs:428` | `"agsDriverExtensionsDX12_SetMarker: unsupported command list {cmd_list:#x}"` |

### `d3d11.dll` (3)

- **Reason:** Unimplemented D3D11 pipeline stage or optional diagnostic interface (tessellation, stream-out, counters, GDI interop)
- **Next step:** Implement hull/domain tessellation and stream-out lowering in dxil-metal + d3d12-metal; wire real MTL4CounterHeap pipeline stats where Metal exposes them.

| API | Kind | Location | Detail |
|---|---|---|---|
| `create_geometry_shader_so` | `stub` | `crates/win32-rt/src/d3d11/device.rs:727` | `"ID3D11Device::CreateGeometryShaderWithStreamOutput -> E_NOTIMPL"` |
| `draw_auto` | `stub` | `crates/win32-rt/src/d3d11/context.rs:2915` | `"ID3D11DeviceContext::DrawAuto not implemented"` |
| `get_data` | `stub` | `crates/win32-rt/src/d3d11/context.rs:3096` | `"GetData(PIPELINE_STATISTICS): GPU-side counters (PS/Clipper/GSPrimitives/DS/indirect) unavailable on Apple Silicon"` |

### `powrprof.dll` (3)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `CallNtPowerInformation` | `stub` | `crates/win32-rt/src/misc/powrprof.rs:468` | `"CallNtPowerInformation(level={info_level}) -> STATUS_NOT_IMPLEMENTED"` |
| `PowerRegisterSuspendResumeNotification` | `stub` | `crates/win32-rt/src/misc/powrprof.rs:533` | `"PowerRegisterSuspendResumeNotification(flags={flags:#x}) -> {handle:#x} (handle tracked; OS wake callback not armed)"` |
| `PowerSettingRegisterNotification` | `stub` | `crates/win32-rt/src/misc/powrprof.rs:604` | `"PowerSettingRegisterNotification(flags={flags:#x}) -> {handle:#x} (handle tracked; OS setting callback not armed)"` |

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

### `oo2core_9_win64.dll` (2)

- **Reason:** Proprietary RAD Oodle compression (cannot be clean-room reimplemented legally without a license)
- **Next step:** Always run the game's shipping oo2core_*_win64.dll PE via x86-runtime ARM64 JIT; keep this honest fallback returning 0 only when no shipping DLL exists.

| API | Kind | Location | Detail |
|---|---|---|---|
| `OodleLZ_Compress` | `middleware_stub` | `crates/win32-rt/src/misc/oo2core_9_win64.rs:86` | `"oo2core!OodleLZ_Compress(compressor={compressor}, raw_len={raw_len})"` |
| `OodleLZ_Decompress` | `middleware_stub` | `crates/win32-rt/src/misc/oo2core_9_win64.rs:106` | `"oo2core!OodleLZ_Decompress(comp={comp_buf_size}, raw={raw_len})"` |

### `sl.interposer.dll` (2)

- **Reason:** Proprietary NVIDIA Streamline plugin host (requires NVIDIA GPU + signed NGX plugins)
- **Next step:** Keep honest eErrorFeatureNotSupported reporting; route DLSS/upscaling requests to MetalFX via nvngx_dlss/libxess/FFX bridges.

| API | Kind | Location | Detail |
|---|---|---|---|
| `slSetConstants` | `middleware_stub` | `crates/win32-rt/src/slstub.rs:125` | `"slSetConstants accepted and dropped (no Streamline features)"` |
| `slSetTag` | `middleware_stub` | `crates/win32-rt/src/slstub.rs:120` | `"slSetTag({n} tags) accepted and dropped (no Streamline features)"` |

### `uxtheme.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `DrawThemeBackground` | `stub` | `crates/win32-rt/src/misc/uxtheme.rs:111` | `"DrawThemeBackground(htheme={htheme:#x}) -> E_NOTIMPL"` |
| `TaskDialogIndirect` | `stub` | `crates/win32-rt/src/misc/uxtheme.rs:203` | `"TaskDialogIndirect -> E_NOTIMPL (native task dialogs not implemented)"` |

### `xaudio2_9redist.dll` (2)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `parse_wave_format` | `middleware_stub` | `crates/win32-rt/src/misc/xaudio2_9redist/mixer.rs:216` | `"XAudio2 CreateSourceVoice: unsupported WAVE_FORMAT_EXTENSIBLE SubFormat {sub:?}"` |
| `parse_wave_format` | `middleware_stub` | `crates/win32-rt/src/misc/xaudio2_9redist/mixer.rs:233` | `"XAudio2 CreateSourceVoice: unsupported format tag {tag:#x} bits {bits}"` |

### `zstd.dll` (2)

- **Reason:** Open-source Zstandard compression fallback (no native zstd crate linked yet)
- **Next step:** Run shipping zstd.dll via ARM64 JIT, or add a clean-room / zstd-safe native backend with unit round-trip tests.

| API | Kind | Location | Detail |
|---|---|---|---|
| `ZSTD_compress` | `middleware_stub` | `crates/win32-rt/src/misc/zstd.rs:33` | `"zstd!ZSTD_compress(src_size={src_size})"` |
| `ZSTD_decompress` | `middleware_stub` | `crates/win32-rt/src/misc/zstd.rs:43` | `"zstd!ZSTD_decompress(compressed_size={compressed_size})"` |

### `d2d1.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `geom_tessellate` | `stub` | `crates/win32-rt/src/misc/d2d1/geometry.rs:1038` | `"ID2D1Geometry::Tessellate -> E_NOTIMPL"` |

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

### `dstorage.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `DStorageCreateCompressionCodec` | `middleware_stub` | `crates/win32-rt/src/misc/dstorage.rs:94` | `"DStorageCreateCompressionCodec: unsupported format {format:#x} -> E_NOTIMPL"` |

### `gameinput.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `gi_get_temporal_reading` | `stub` | `crates/win32-rt/src/misc/gameinput.rs:1968` | `"IGameInput::GetTemporalReading -> E_NOTIMPL"` |

### `gdi32.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `GetDeviceCaps` | `stub` | `crates/win32-rt/src/misc/gdi32.rs:904` | `"GetDeviceCaps({index}) -> 0"` |

### `hid.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `HidD_SetNumInputBuffers` | `stub` | `crates/win32-rt/src/misc/hid.rs:887` | `"ring size stays {INPUT_BUFFERS} (asked {n})"` |

### `libxess.dll` (1)

- **Reason:** Intel XeSS debug dump / Intel GPU crash-dump vendor extension (INTC_D3D12_*)
- **Next step:** Keep XeSS execute routed through MetalFX; leave proprietary Intel GPA/crash-dump callbacks as documented unsupported hooks.

| API | Kind | Location | Detail |
|---|---|---|---|
| `xessD3D12Execute` | `middleware_stub` | `crates/win32-rt/src/misc/libxess.rs:594` | `"xessD3D12Execute: non-zero subrect base offsets color=({},{}) mv=({},{}) depth=({},{}) reactive=({},{}) out=({},{}) not applied by MetalFX"...` |

### `msctf.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `ctx_track_properties` | `stub` | `crates/win32-rt/src/misc/msctf/context.rs:809` | `"ITfContext::TrackProperties -> E_NOTIMPL"` |

### `physxcooking_64.dll` (1)

- **Reason:** NVIDIA PhysX 4.x/5.x C++ ABI (vtable layout varies by build; shipping DLL runs natively under ARM64 JIT)
- **Next step:** Run shipping PhysX_64.dll / PhysXCooking_64.dll under ARM64 JIT via [middleware.physx] mode = "guest".

| API | Kind | Location | Detail |
|---|---|---|---|
| `PxCreateCooking` | `middleware_stub` | `crates/win32-rt/src/misc/physxcooking_64.rs:24` | `"PhysXCooking_64!PxCreateCooking(version={formatted}, detected={detected}, configured={:?}): PhysX 4.x/5.x cooking requires shipping PE ([mi...` |

### `shell32.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `SHCreateItemFromParsingName` | `stub` | `crates/win32-rt/src/misc/shell32.rs:1733` | `"SHCreateItemFromParsingName: unsupported shell namespace {raw:?}"` |

### `xinput1_4.dll` (1)

- **Reason:** Partial or unimplemented Windows/middleware sub-feature
- **Next step:** Replace with real macOS/Metal/CoreAudio implementation plus unit or zig-cc guest test.

| API | Kind | Location | Detail |
|---|---|---|---|
| `XInputGetKeystroke` | `stub` | `crates/win32-rt/src/misc/xinput1_4.rs:608` | `"XInputGetKeystroke: keystroke events not generated (ERROR_EMPTY)"` |

