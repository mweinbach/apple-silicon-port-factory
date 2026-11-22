# ADVICE_M3 — D3D12 + DXGI on Metal (first frame → main menu)

Advisor memo, 2026-10-05. Scope: implement the D3D12/DXGI surface used by `AlanWake2.exe` v1.2.10 plus
middleware inside the hybrid loader (x86_64 under Rosetta), so the main menu renders with the
game's own shaders. Everything not marked **[A]** was checked in this repo, the game binary or the
middleware DLLs. **[A]** means I'm going from memory and you should check it before relying on it.

Evidence tools: `tools/ghidra_decompile.sh`, `tools/fndb.sh`, full `llvm-objdump` of the exe, a
GUID scan of every PE in the game dir against `third_party/directx-headers`, and a DXIL container
scan of all `.binrfx`. The scratch scripts are in the advisor's brain dir (`peguid.py`, `scan.py`,
`dxilstats.py`, `imps.py`). They're worth moving into `tools/` (see §5.4).

---

## 0. TL;DR: the eight decisions

1. **Replace `sl.interposer.dll` with a builtin Rust module. Don't keep it in the chain.** The exe
   imports exactly 16 functions from it, and it's the only way the exe reaches D3D12/DXGI. The real
   interposer patches *our* vtables and *writes into our function bodies*
   (`hook.cpp:206-239`, `kCodePatchSize = 16`). The game passes `PreferenceFlags = 0x49`, which
   doesn't include `eUseManualHooking`, so that hooking would be active. AGENTS.md §6 already says
   "Streamline: stub". Still give **all** builtins a real export directory (cheap, and generic).
2. **Report a minimal, honest feature set that passes the game's gates:** RB tier 3,
   TypedUAVLoadAdditionalFormats, SM ≥ 6.5, ≥ ~5.5 GiB "dedicated" memory. Report
   **MeshShaderTier = 0, RaytracingTier = 0, VRS tier 0**. The game has verified fallbacks: every
   mesh-shader program in the bundles has a VS twin, and RT/mesh/VRS are just flags read from
   `CheckFeatureSupport` (§4).
3. **VendorId = 0x106B (Apple).** The adapter picker takes the "first adapter" path, and the vendor
   enum stays 0, so the NVIDIA/AMD/Intel-specific code (NVAPI, AMD paths) never runs.
4. **Split the work in two.** A new arch-neutral crate `aw2-d3d12` holds D3D12 semantics and the
   Metal backend. A thin COM/ABI layer goes in `aw2-win32` (`extern "win64"` vtables). That split
   is what makes a native arm64 trace replayer possible (§5.4).
5. **Use Metal 4 for submission** (`MTL4CommandAllocator`/`MTL4CommandBuffer`/`MTL4CommandQueue`,
   argument tables, residency sets, explicit barriers). It maps almost 1:1 onto D3D12 allocators,
   lists, queues and barriers. `MTL4*` symbols are exported for `x86_64-macos` in the SDK's
   `Metal.tbd`. Rosetta *runtime* behaviour still needs a ~50-line spike in M3.0. If that fails,
   use the same architecture on Metal 3 (§3.4).
6. **Encode directly at record time.** No IR replay. Keep a small amount of lazy pass state so
   Clear-then-draw folds into load actions.
7. **Descriptor heaps are Metal Shader Converter (MSC) descriptor tables.** A shader-visible heap
   is an `MTLBuffer` of 24-byte `IRDescriptorTableEntry`. The GPU descriptor handle *is* the Metal
   GPU address, so `SetGraphicsRootDescriptorTable` is a single u64 store into the top-level
   argument buffer. The game uses no SM6.6 `ResourceDescriptorHeap` at all (no program sets the
   flag); its bindless is through huge v1.0 descriptor tables (§3.6).
8. **Run shader conversion out of process in an arm64 helper**, with a disk cache keyed by
   (DXIL hash, root-sig hash, MSC version, options). MSC's LLVM running under Rosetta inside the
   game process will cause hitches.

---

## 1. Inventory: what the game and middleware actually use

### 1.1 How the exe reaches D3D12/DXGI

The exe imports **no** `d3d12.dll`/`dxgi.dll` directly. Everything goes through `sl.interposer.dll`
(fndb `imports`, IAT `0x1426472f8–0x142647370`):

| Import | Call sites (fndb `import_calls` / disasm) |
|---|---|
| `CreateDXGIFactory1` | `0x14208d281` (in `0x14208d1f0`), `0x14207c846` |
| `D3D12CreateDevice` | `0x14208d6d1` in `d3d12_init_and_check` `0x14208d490` |
| `D3D12GetDebugInterface` | `0x14208d4c2/508/543` (only with debug flags `param_2 & 0x19`) |
| `D3D12SerializeRootSignature` | `0x142079de5`, `0x142094a83` |
| `D3D12SerializeVersionedRootSignature` | `0x1420942aa`, `0x1420944ec`, `0x1420968e0` |
| `slInit` | `0x141857f39` (in WinMain `0x141856b20`). Result **ignored** |
| `slSetD3DDevice` | `0x14208d6fa` right after device creation |
| `slSetFeatureLoaded(1000 /*DLSS_G*/, false)` | `0x14207b11c` (FFX frame-gen swapchain path only) |
| `slGetFeatureFunction` | 8 sites, e.g. `0x1411e2a8c`, `0x14207a8e2`, `0x14208e306` |
| `slEvaluateFeature`, `slFreeResources`, `slSetTag`, `slSetConstants`, `slGetNewFrameToken`, `slIsFeatureSupported`, `slShutdown` | per-frame/feature paths (`0x14208e990`, `0x14208f3d0`, …) |

`slInit` arguments, from the disasm at `0x141857e12–0x141857f39`: Preferences GUID
`1CA10965-…`, features `{0 DLSS, 3 Reflex, 1000 DLSS_G, 1001 DLSS_RR}`, flags
`0x49 = eDisableCLStateTracking | eAllowOTA | eLoadDownloadedPlugins`
(`third_party/streamline/include/sl_core_types.h:478-503`), SDK version `0x200070000fedc`
(SL 2.7.0).

Middleware that imports D3D12/DXGI itself (from PE import tables):

| DLL | Imports | Notes |
|---|---|---|
| `NRI.dll` | `d3d12` **ordinal 101, 102** and `D3D12SerializeVersionedRootSignature`; `dxgi!CreateDXGIFactory2`, `DXGIGetDebugInterface1`; `d3d11!D3D11CreateDevice` | **Verified:** #102 = `D3D12GetDebugInterface` (2 args, IID in rcx, call at `NRI+0x3d25f`); #101 = `D3D12CreateDevice` (`edx=0xc000` FL12_0, call at `NRI+0x3d328`). The game only calls `nriCreateDeviceFromD3D12Device` (wrapping): `0x142058540`, `0x142263520` (a `rend::RendererInterface` ctor job lambda), `0x14229f740`. |
| `sl.common.dll` | `dxgi!CreateDXGIFactory`, `d3d12!D3D12SerializeRootSignature`, `d3d12 #101` | Never loaded if `sl.interposer` is builtin |
| `amd_fidelityfx_upscaler_dx12.dll` | `dxgi!CreateDXGIFactory1/2` | Loaded lazily by the FFX loader |
| `amd_fidelityfx_framegeneration_dx12.dll` | `dxgi!CreateDXGIFactory2` | Only for FFX FG swapchain |
| `RenoirCore`, `cohtml`, `bink2w64`, `NRD`, `sl.dlss*`, `sl.reflex`, `sl.pcl` | none | No D3D12 IIDs in these DLLs. The cohtml/Renoir D3D12 backend is **in the exe** (shaders `CoherentGT.binrfx`, `font.binrfx`). Bink GPU decode uses the exe's `bink.binrfx` through `BinkRegisterGPUDataBuffers`. |

The exe exports `D3D12SDKVersion = 619` (`0x1426df2a0`) and `D3D12SDKPath` (`0x143823058`), so
it was built against Agility SDK headers 1.619. Never load `D3D12Core.dll`. Use
`third_party/directx-headers` structure layouts, which have `OPTIONS22` and `D3D_SHADER_MODEL_6_9`
(`include/directx/d3d12.h:2646, 2868`).

### 1.2 Interfaces the exe references (IIDs found in `.rdata`)

`ID3D12Device`, **`Device2`** (the device is created with this IID), `Device5`, `Device6`,
`Device7`, `CommandQueue`, `CommandAllocator`, `GraphicsCommandList`, `GraphicsCommandList4`,
`GraphicsCommandList5`, `Fence`, `Heap`, `Resource`, `DescriptorHeap`, `RootSignature`,
`PipelineState`, `QueryHeap`, `CommandSignature`, `StateObject`, `StateObjectProperties`, `Debug`,
`Debug1`, `DeviceRemovedExtendedDataSettings(1)`; `IDXGIFactory4` (the factory IID),
`IDXGIFactory5`, `IDXGIAdapter3`, `IDXGIOutput6`, `IDXGISwapChain4`.

What's **absent** matters as much: no `Device1` (no pipeline libraries), no `Device3/4/8/9/10+`
(no enhanced barriers, no `CreateCommittedResource2/3`), no `GraphicsCommandList1/2/3/6/7+`.
`DispatchMesh` lives in `List6`, so if mesh shaders are ever enabled the game calls slots past
List5 on a pointer obtained as List4/5. **One superset vtable per object is mandatory** (§3.1).

NRI adds `List1`, `List6`, `InfoQueue`, `IDXGIDevice`, `IDXGIAdapter`, `IDXGIFactory2`,
`IDXGISwapChain1-3`, `IDXGIDebug1`. FFX FG adds `Device4`, `Resource1/2`, `IDXGIFactory2`.

Device vtable slots, from IID-adjacent call sites in the disasm (slot = offset/8): `CreateCommandQueue` (8),
`CreateCommandAllocator` (9), `CreateComputePipelineState` (11, 15 sites), `CreateCommandList` (12),
`CheckFeatureSupport` (13), `CreateDescriptorHeap` (14), `CreateRootSignature` (16),
`CreateCommittedResource` (27, 22 sites), `CreateHeap` (28), `CreatePlacedResource` (29),
**`CreateReservedResource` (30, one site in `0x142080120`)**, `CreateFence` (36), `CreateQueryHeap`
(39), `CreateCommandSignature` (41, `0x14206df07`), `Device2::CreatePipelineState` (47, the stream
PSO path), `Device5::CreateStateObject` (62, RT only). Calls are plain `call reg` / `call [rax+N]`,
**no CFG guard dispatch**, so direct `extern "win64"` function pointers in the vtables are fine.

### 1.3 Device bring-up and every gate (verified)

**Adapter pick** (`0x14208ce80`): walks `EnumAdapters` (slot 7) until `DXGI_ERROR_NOT_FOUND`
(0x887A0002) and reads `GetDesc` (slot 8) `VendorId`. It prefers 0x1002/0x10DE, then 0x8086, and
otherwise takes the **first adapter**. If the first adapter is **0x1414** it calls
`TerminateProcess(0xbadf00d)` ("Display driver is not installed"). A re-pick path (`0x14208d1f0`,
second branch) matches adapters by **AdapterLuid**, so the LUID must be stable and non-zero.
`IDXGIFactory1::IsCurrent` (slot 13) is called.

**`d3d12_init_and_check` (`0x14208d490`)**, field offsets on `DAT_14397fc58`:

* Vendor → `+0xa8` (1 NV, 2 AMD, 3 Intel, else 0). `[0x11]/[0x12]` = Dedicated video/system memory
  >> 10 (`DXGI_ADAPTER_DESC` +0x110/+0x118).
* `D3D12CreateDevice(adapter, 0xB000 /*FL11_0*/, IID_ID3D12Device2)`, then `slSetD3DDevice`.
* QI `Device5` → `CheckFeatureSupport(OPTIONS5=27, size 0xC)`: `RaytracingTier > 10` (≥ 1.1) →
  `+0xac` "RT available". `param_2 & 2` forces it off. A second OPTIONS5 query: tier ≥ 12 → `+0xad`.
* The NVAPI block runs only when vendor == 1 (`FUN_140001940/430/15e0/16d0/1500` are NVAPI
  thunks; driver version > 545.83 check). With 0x106B it is skipped entirely.
* `SHADER_MODEL` with HighestShaderModel = **0x69**, plus feature **65 = OPTIONS22**
  (`ShaderExecutionReorderingActuallyReorders`) → `+0xae` (SER). Only meaningful when `+0xac`.
* QI `Device6` → `OPTIONS6 (30)`: `VariableShadingRateTier > 1` → `+0x9f`, tile size → `+0xa4`.
* QI `Device7` → `OPTIONS7 (32)`: `MeshShaderTier > 0` → **`+0x9e` (mesh shaders)** unless
  `param_2 & 4`. `DAT+0xa0` = "query succeeded".
* `gpu_feature_validator(0x142070370)`. A false result → `TerminateProcess(0xb501e7f)`.

**`gpu_feature_validator` (`0x142070370`)** runs 4 checks (loop `iVar23 = 0..3`):

| # | Query | Requirement | Message string |
|---|---|---|---|
| 0 | `OPTIONS (0)`, size 0x3C, field +0x18 | `TypedUAVLoadAdditionalFormats == TRUE` | (GPU_FEATURE_TYPEDUAVLOAD) |
| 1 | same, field +0x10 | `ResourceBindingTier > 2` (Tier 3) | "Resource binding tier 3" `0x1433dc578` |
| 2 | `SHADER_MODEL (7)`, request 0x65 | highest > 0x64 (≥ 6.5) | "Shader Model 6.5" `0x1433dc550` |
| 3 | adapter memory `[0x11] << 10` | > 0x157BFFFFF bytes (~5.37 GiB) | VRAM warning/error (severity from a table; whether it's fatal is unverified) |

**HRESULT policy of the game.** After almost every call it runs `FUN_14205d410` only for
`E_OUTOFMEMORY` or `DXGI_ERROR_DEVICE_REMOVED/HUNG/RESET` (0x887A0005–7). **Any other failure is
silently ignored** and the out-params stay zero/null. A wrong `E_NOTIMPL` therefore turns into a
null dereference far from its cause. Log every failing HRESULT with the method name.

**Global root signatures** (`0x142071a1a…`, `0x142094680(type)`, `0x142096630`): `+0x20` graphics
RS, `+0x28` mesh RS (only built if `+0x9e`), `+0x30` compute RS ("ComputeRootSignature",
`D3D12SerializeVersionedRootSignature` **v1.0**). Decoded compute layout (Ghidra locals; register
bases partly inferred):

| # | Param | Contents |
|---|---|---|
| 0 | table | CBV×10 (b4…), SRV×51 (t0…), UAV×16 (u0…) |
| 1 | table | Sampler×4 |
| 2–5 | root CBV | b0–b3 |
| 6 | table | Sampler×64, space1 |
| 7 | 32-bit constants | 4 values, b0 space1 |
| 8 | table | **SRV×263,168, space1** (bindless) |
| 9 | table | **SRV×65,536, space2** (bindless) |
| 10 | table (only if `+0xaf` NVAPI extn) | UAV×1, space5 |

This is SM6.5-era bindless through RB-tier-3 descriptor tables, not SM6.6 heaps.

**Swapchain** (`0x14207aec0`):

1. `IDXGIFactory5::CheckFeatureSupport(PRESENT_ALLOW_TEARING)` (slot 28).
2. **`IDXGIFactory::CreateSwapChain` (slot 10, legacy desc)** with the direct queue
   (`DAT_14397fc60+0x10`). `DXGI_SWAP_CHAIN_DESC`: `R10G10B10A2_UNORM (0x18)`, or
   `B8G8R8A8_UNORM (0x57)` when `+0x66`; usage `RENDER_TARGET_OUTPUT`; **2 buffers**; windowed;
   **FLIP_DISCARD (4)**; flags `ALLOW_MODE_SWITCH (2)`, plus `ALLOW_TEARING (0x800)` if supported.
   No frame-latency waitable flag.
3. On the returned pointer, **without QI**: slot 38 `IDXGISwapChain3::SetColorSpace1(G22_P709)`.
4. `IDXGIFactory::MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER)`.
5. HDR branch: `FUN_14207c4b0/14207c410` (colour-space checks) and `GetFullscreenState` (slot 11).
6. `CreateFence`, then the buffers.

`0x14207b430` QIs `IDXGISwapChain4` for `SetHDRMetaData`. If frame generation is enabled
(`param_9` and `DAT_143857fb8`), the FFX FG swapchain is created through `ffxCreateContext(0x30005)`
instead. **Keep FG off for M3.**

### 1.4 Shader corpus (DXIL scan of all 153 `.binrfx`)

* 2,284 programs: PS 567, CS 566, **LIB 930** (DXR), VS 173, **MS 48**, AS 0, **HS/DS/GS 0**.
  No tessellation or GS emulation is needed.
* SM: 6.2 (740), 6.5 (902), **6.9 (642, only `rt_pathtracing`, `rt_transparent_pt`)**.
* Feature flags: Wave 113, TypedUAVLoad 322, Int64 46, NativeLowPrec 2, VPRTIndex 3, RT1.1 1047.
  **Zero** programs use ResourceDescriptorHeap, SamplerDescriptorHeap, VRS, SamplerFeedback,
  Barycentrics, ROV, AtomicInt64, WriteableMSAA or AdvancedTextureOps (flag bit names **[A]**,
  from DXIL SFI0 bit assignments).
* Every mesh-shader bundle has a matching VS count (`gpupipe_material`: VS 11 / MS 11,
  `gpupipe_visibility_buffer` 1/1, `hair` 5/5, `skydome` 1/1, …). That's the VS fallback path.
* 7 bundles (`standardmaterial`, `character`, `decal`, `billboard`, `cloth`, `gfxgraph_decal`,
  `deprecated_blendedmaterial`) contain **no DXBC**, so material shaders come from elsewhere
  (data packs). It doesn't matter for us: all DXIL arrives at PSO creation.

### 1.5 Deferred until after the first frame

DXR (state objects, AS builds, DispatchRays, RayQuery), mesh/amplification shaders, VRS, sampler
feedback, true tiled resources (emulate, §3.7), pipeline statistics, accurate mid-pass timestamps,
HDR/EDR output, tearing, the FFX FG swapchain, FFX upscalers (use native or the game's built-in
FSR2: `ffx_fsr2_*.binrfx` + `fsr2glue.binrfx`), NRI/NRD (RT denoising), DLSS/Reflex,
DRED/debug-layer interfaces, multi-adapter.

---

## 2. Streamline, FFX and NRI strategy

### 2.1 Why the crash happens

`sl.interposer` resolves real exports by parsing the target module's EAT
(`third_party/streamline/source/core/sl.interposer/hook.cpp:137-164 enumerateModuleExports`, used
by `dxgi/dxgi.cpp:417-450 loadDXGIModule` and `d3d12/d3d12.cpp:186-232`). Our builtin page has
`NumberOfRvaAndSizes = 16` but an empty export directory (`aw2-loader/src/loader.rs:680
write_fake_pe_header`). So `hookCreateDXGIFactory1.target == null`, and
`sl::interposer::call(...)` jumps to 0 at `sl.interposer+0x26726`.

### 2.2 Recommendation: builtin `sl.interposer.dll`

Keeping the real interposer means more than fixing the EAT:

* With flags 0x49 (no manual hooking, no factory proxy) it **patches the vtables of the base DXGI
  factory** and wraps device/queue/lists/swapchain in proxies. It then QIs every `Device1–10`,
  `List1–8`, `Factory1–7`, `SwapChain1–4` (all those IIDs are present in `sl.interposer.dll`).
* `registerHookForClassInstance` / `restoreOriginalCode` `VirtualProtect` and **`memcpy` 16 bytes
  into the hooked target function**, which would be our Rust code in the loader's `__TEXT`
  (`hook.cpp:166-239`, `hook.h:39`). That code is signed, max-prot r-x, and Rosetta-translated:
  a guaranteed fault, or worse, a stale translation (AGENTS.md §10).
* It loads `sl.common` → NGX/NVAPI probing, OTA, `WinVerifyTrust` (we already return
  `TRUST_E_NOSIGNATURE` three times in `out/m2_verify.log`).

None of that buys anything on a Mac. Implement `sl.interposer.dll` as a builtin and add a loader
"**builtin overrides game-dir PE**" list (initially just `sl.interposer.dll`; log it at INFO).

| Export | Behaviour |
|---|---|
| `D3D12CreateDevice`, `D3D12GetDebugInterface`, `CreateDXGIFactory1`, `D3D12SerializeRootSignature`, `D3D12SerializeVersionedRootSignature` | Tail-call the `d3d12`/`dxgi` builtins (same Rust functions) |
| `slInit` | `stub!`, return `eOk`. Store nothing. Make it a real "no features" runtime, not fake success: every feature query fails. |
| `slIsFeatureSupported` | `eErrorFeatureNotSupported` (codes from `include/sl_result.h`; eOk = 0) |
| `slGetFeatureFunction` | write null, return `eErrorFeatureNotSupported` |
| `slSetD3DDevice`, `slSetFeatureLoaded`, `slSetTag`, `slSetConstants`, `slEvaluateFeature`, `slFreeResources`, `slShutdown` | Return `eErrorFeatureNotSupported` / `eOk` as appropriate, rate-limited `stub!` logging (some are per-frame) |
| `slGetNewFrameToken` | Return a **real** `sl::FrameToken` object, a C++ object whose vtable slot 0 (`operator uint32_t`) returns the frame index (`sl_core_types.h:556+`), plus `eOk`. The game may dereference it **[A: verify at a call site]**. |

That matches AGENTS.md §6 ("stub (feature unsupported)") and removes a whole class of hazards.

### 2.3 Still do: real export directories for all builtins (generic loader fix)

This is cheap, and NRI/FFX/delay-load helpers and anything else that walks an EAT will then work.
Do it in `add_builtin`:

* Map `PAGE + edata + thunks` contiguously: headers / `.edata` / `.text` (thunks, RX).
* **Thunks are required.** EAT entries are 32-bit RVAs, and our Rust functions are far away. Each
  export gets a 16-byte stub `FF 25 00 00 00 00 <abs64>` (`jmp [rip+0]`) inside the image.
* `IMAGE_EXPORT_DIRECTORY`: `Name`, `Base = min ordinal`, `AddressOfFunctions` indexed by
  `ordinal − Base`, `AddressOfNames` **sorted by byte-wise strcmp** (Windows and `GetProcAddress`
  implementations binary-search), plus `AddressOfNameOrdinals`. Fill
  `DataDirectory[0] = {rva, size}`, two section headers, `SizeOfImage`, and `register_image(base,
  size, sections)`.
* `GetProcAddress` should return **the same thunk address** as the EAT, so pointer identity holds.
* `d3d12.dll` ordinals: **101 → `D3D12CreateDevice`, 102 → `D3D12GetDebugInterface`** (verified
  from `NRI.dll`; see §1.1) via the existing `BuiltinDll::ordinals`. Without these, NRI's IAT
  traps.
* Tests: (a) unit test in aw2-loader that parses a builtin's EAT with `aw2-pe` and checks every
  name resolves to the same address as `GetProcAddress`; (b) feed the builtin image to a Rust port
  of `enumerateModuleExports`' logic.

### 2.4 FFX loader (`amd_fidelityfx_loader_dx12.dll`, 2.2)

Keep it as a PE. It loads provider DLLs with `LoadLibrary`/`GetProcAddress` and calls standard
D3D12/DXGI through import tables, which works once `dxgi` exports
`CreateDXGIFactory1/2`. For M3:

* Make sure the upscaler setting is "off/native" (or the game's built-in FSR2) and frame
  generation is off. Then `ffxCreateContext` is never reached (upscaler `0x14205c4b0`, FG
  swapchain `0x14207aec0`).
* `ffxQuery` at `0x1422986e0` (from `0x14226c9b0`) runs at startup. It should only enumerate
  providers or versions; let it fail cleanly.
* Later, FSR3/4 provider DLLs embed DXIL and go through our normal PSO path. FSR4 (ML) needs AMD
  driver components and will decline by itself **[A]**.

### 2.5 NRI (1.99) / NRD

Keep them as PEs. The game wraps its device through `nriCreateDeviceFromD3D12Device`. Requirements:

* `d3d12` ordinals 101/102.
* `dxgi!CreateDXGIFactory2`, `DXGIGetDebugInterface1` (return `E_NOINTERFACE`).
* `d3d11!D3D11CreateDevice` (fail with `E_NOTIMPL` / `DXGI_ERROR_UNSUPPORTED`, never trap).
* NRI's `CheckFeatureSupport`/QI pattern (OPTIONS*, Device5, List1/4/6, `IDXGIFactory4::EnumAdapterByLuid`).
* Report `OPTIONS12.EnhancedBarriersSupported = FALSE` so NRI stays on legacy barriers
  (`third_party/nri/Source/D3D12/DeviceD3D12.cpp`).

NRD only dispatches with RT on, so after M3.

---

## 3. Core architecture

### 3.1 Crates and the COM object model

```
crates/aw2-d3d12/        (new; builds for x86_64 AND arm64; no win64 ABI)
  core/   device, resources, heaps, descriptors, rootsig (RTS0 parse+serialize), pso,
          cmdlist recorder, queue, fence, swapchain core, formats, footprints, queries
  metal/  objc2-metal backend (Metal 4 submission, Metal 3 objects), MSC glue (aw2-shaders)
  trace/  call trace format + replayer library
crates/aw2-win32/src/{dxgi,d3d12,d3d11stub,slstub}/
  generated vtables (extern "win64"), IID tables, HRESULTs, struct marshalling, private data
tools/d3d12-replay/      (arm64 native) replays traces, dumps PNG per Present
```

**Generate the vtables** from `third_party/directx-headers/include/directx/d3d12.h` and the DXGI
IDL with a script (`tools/gen_com_vtables.py`). That gives one `#[repr(C)]` struct per *newest*
interface (`ID3D12Device14`, `ID3D12GraphicsCommandList10`, `IDXGIFactory7`, `IDXGISwapChain4`,
`IDXGIAdapter4`, `IDXGIOutput6`) with every slot filled. Unimplemented slots point at generated
per-slot traps that log `UNIMPLEMENTED ID3D12GraphicsCommandList::DispatchMesh (slot N)` with a
guest backtrace (reuse the loader's trap machinery) and return `E_NOTIMPL`, or abort for `void`
methods. This turns "which methods does the game use?" into a runtime question with exact answers.

Object layout:

```rust
#[repr(C)] struct ComObj<T> {          // pointer handed to the guest
    vtbl: &'static VTable,              // superset vtable; QI returns this same pointer
    refs: AtomicU32,                    // COM (external) refcount
    private: PrivateData,               // Mutex<SmallVec<(GUID, Blob | IUnknown*)>>; SetName -> WKPDID_D3DDebugObjectNameW
    core: Arc<T>,                       // internal ref; GPU work and children hold clones
}
```

* **QI:** a static per-class IID list (whole inheritance chain, e.g. `IUnknown, ID3D12Object,
  ID3D12DeviceChild, ID3D12Pageable, ID3D12Resource, ID3D12Resource1, ID3D12Resource2`). A match
  does AddRef and returns `self`; otherwise null out-ptr + `E_NOINTERFACE` (and log the IID at
  debug level, which is how the version ladder is discovered).
* **Superset vtable:** required because the game calls slot 38 on a plain `IDXGISwapChain*`
  (§1.3), and `DispatchMesh` would be called through List4/5 pointers.
* **Device children** hold `Arc<DeviceCore>`. `GetDevice(riid)` QIs the device.
* **Two-level lifetime.** When the COM count reaches 0, the guest handle is destroyed. The
  `Arc<Core>` (and its Metal objects) is pushed to a **deferred-destruction list** tagged with the
  per-queue timeline values current at release, and freed when every queue has passed them.
  D3D12 makes this the app's job, but our side objects (views, TLAB pages, MSC artifacts) must
  never be freed under the GPU, and the game's timing assumptions were made against a different
  driver.
* **Thread safety:** device methods are free-threaded (job lambdas such as `0x142263520` create
  objects concurrently). Use per-subsystem locks (PSO cache, sampler cache, residency set). Make
  descriptor writes lock-free plain stores into heap memory, as in D3D12. Command lists are
  single-threaded, so the encoder state is unsynchronized. Queues get a mutex.
* **MSVC ABI trap:** methods that "return a struct" use a **hidden return pointer after `this`**
  and return it in RAX: `GetCPU/GPUDescriptorHandleForHeapStart`, `ID3D12Resource::GetDesc`,
  `ID3D12Heap::GetDesc`, `ID3D12DescriptorHeap::GetDesc`, `ID3D12CommandQueue::GetDesc`,
  `GetResourceAllocationInfo(1)`, `GetCustomHeapProperties`, `GetAdapterLuid`. d3d12.h shows both
  forms (e.g. `d3d12.h:6303` vs `:6307`, `RetVal`). Generate from the `RetVal` variant.

### 3.2 DXGI

* **Factory** (single object, chain up to Factory7): `EnumAdapters/EnumAdapters1` → one adapter,
  `NOT_FOUND` at index ≥ 1. `EnumAdapterByLuid`, `EnumAdapterByGpuPreference`, `IsCurrent` → TRUE,
  `MakeWindowAssociation` → record. `CheckFeatureSupport(ALLOW_TEARING)` → **FALSE for M3**.
  `CreateSwapChain` / `CreateSwapChainForHwnd` share one implementation and take
  `ID3D12CommandQueue*` as the "device".
* **Adapter** (chain up to Adapter4): `Description = MTLDevice.name`, **VendorId 0x106B**,
  `DeviceId` = a stable hash, `DedicatedVideoMemory = recommendedMaxWorkingSetSize` (must exceed
  ~5.37 GiB for gate #3), `SharedSystemMemory` = RAM − that, a stable non-zero LUID, Flags 0.
  `QueryVideoMemoryInfo`: Budget = `recommendedMaxWorkingSetSize`, CurrentUsage =
  `currentAllocatedSize`. The budget-change event is never signalled (log that).
* **Output** (chain up to Output6): one per `NSScreen`. `Monitor` must equal user32's `HMONITOR`
  for that screen (WINDOWING.md). `GetDesc1` reports `G22_NONE_P709` SDR for M3. Implement
  `GetDisplayModeList(1)`, `FindClosestMatchingMode`, and `WaitForVBlank` (CVDisplayLink).

### 3.3 Swapchain → CAMetalLayer

* The swapchain owns `BufferCount` **private MTLTextures**: the D3D12 back buffers, wrapped as
  `ID3D12Resource`s with stable identity for `GetBuffer(i)`. **Don't render into drawables.**
  Drawables rotate and D3D12 buffers are persistent.
* `Present(sync, flags)` on the swapchain's queue, in order after prior `ExecuteCommandLists`:
  acquire `nextDrawable` (layer `maximumDrawableCount = 3`), blit `backbuffer[cur]` →
  `drawable.texture`, then present. Metal 4: `queue.waitForDrawable` → commit blit →
  `signalDrawable` → `[drawable present]`. Advance `cur = (cur+1) % N` (FLIP_DISCARD). Throttle
  so ≤ `MaximumFrameLatency` (default 3) presents are in flight, using a shared event per present.
  `sync == 0` → `displaySyncEnabled = NO` only if tearing was reported.
* Layer config: `pixelFormat` = RGB10A2Unorm or BGRA8Unorm from the desc,
  `framebufferOnly = YES` (we only blit into it), `drawableSize` = client size in **pixels**
  (WINDOWING.md Retina rule). Use `metal_layer_for_hwnd`
  (`crates/aw2-win32/src/macos/mod.rs:225`). Set geometry-affecting layer properties through the
  AppKit thread (`run_sync`). `nextDrawable` / `present` are fine off-main **[A]**.
* `ResizeBuffers`: fail with `DXGI_ERROR_INVALID_CALL` if any buffer still has external refs (as
  Windows does), else recreate. `SetFullscreenState` → treat as borderless and log;
  `GetFullscreenState` → FALSE. The window is already `WS_POPUP` borderless (`style=0x90000000`
  in the M2 log).
* HDR (post-M3): `SetColorSpace1(G2084_P2020)` → `layer.colorspace = kCGColorSpaceITUR_2100_PQ`,
  `wantsExtendedDynamicRangeContent = YES`; `SetHDRMetaData` → `CAEDRMetadata HDR10`.

### 3.4 Device, queues, allocators, lists: Metal 4 mapping

| D3D12 | Metal 4 |
|---|---|
| `ID3D12CommandQueue` (DIRECT/COMPUTE/COPY) | `MTL4CommandQueue` each, all with the device-wide `MTLResidencySet` attached |
| `ID3D12CommandAllocator` | `MTL4CommandAllocator` + our **linear upload arena** (shared `MTLBuffer` chunks) for TLAB copies, root constants and scratch |
| `ID3D12GraphicsCommandList::Reset(alloc, pso)` | `MTL4CommandBuffer beginCommandBufferWithAllocator:` |
| `Close` | `endCommandBuffer` |
| `ExecuteCommandLists` | `commit:count:` in order (one call → one commit batch) |
| `Signal`/`Wait` | `signalEvent:value:` / `waitForEvent:value:` on `MTLSharedEvent` |

Lists are encoded **directly** on the recording thread. That's legal in Metal 4: command buffers
aren't tied to a queue until commit, exactly like D3D12. If the game re-executes a closed list,
detect it. If Metal 4 can't re-commit **[A]**, keep an optional lightweight IR per list so it can
be re-encoded, and log the first occurrence.

**Fallback if the Rosetta spike fails:** Metal 3 with the same structure. `MTLCommandBuffer` is
created from the target queue at `Reset` (the list type maps to the queue), ordered by
`enqueue`/`commit` at ECL. Use untracked heap resources + `MTLFence`/`memoryBarrier`, residency
sets (macOS 15+, they work with `MTLCommandQueue` too), and `useResource` is never needed. The
backend trait is "encoder ops", so both can coexist.

### 3.5 Fences and events

* `ID3D12Fence` → `MTLSharedEvent`. `GetCompletedValue` = `signaledValue`. CPU `Signal(v)` sets
  `signaledValue`.
* `SetEventOnCompletion(v, h)`: if already reached, `SetEvent(h)` immediately. If `h == NULL`,
  block (`waitUntilSignaledValue:timeoutMS:`). Otherwise register with one process-wide
  `MTLSharedEventListener` (its own dispatch queue) whose block calls aw2-win32's
  `SetEvent` (`kernel32/threads.rs:438`) on the Win32 event. Never run guest code in the block.
* `Queue::Wait` before the matching `Signal` was submitted is legal. Shared events handle it.
* Device lost (a command buffer with error status) → make every fence report `UINT64_MAX`,
  `GetDeviceRemovedReason` → `DXGI_ERROR_DEVICE_HUNG`. The game then shows its own error through
  `FUN_14205d410`, which is the honest outcome.

### 3.6 Resources, heaps, descriptors, root signatures

**Heaps and resources**

| D3D12 | Metal |
|---|---|
| DEFAULT buffers and textures | `MTLStorageModePrivate` (keep lossless compression) |
| UPLOAD | Shared + `WriteCombined` |
| READBACK | Shared, default cache |
| `CUSTOM` WRITE_BACK / L0 | Shared |
| `ID3D12Heap` | `MTLHeap` type **placement**, `hazardTrackingMode = untracked`; storage mode from heap type |
| `CreatePlacedResource` | `heap.newBuffer/TextureWithLength/Descriptor:offset:`. Overlapping offsets = aliasing (legal on placement heaps); aliasing barriers become barriers (§3.8) |
| `CreateCommittedResource` | Standalone resource (or a private heap per resource if it must alias) |

* `GetGPUVirtualAddress` = `MTLBuffer.gpuAddress`. `Map` returns `contents` (persistent);
  `Unmap` is a no-op. Unified memory means no flush, but keep the D3D12 ranges for tracing.
* Report **ResourceHeapTier 2**. `GetResourceAllocationInfo` uses
  `heapBuffer/TextureSizeAndAlign`, rounded up to D3D12's 64 KiB (4 MiB MSAA) placement alignment.
  `GetCopyableFootprints` is **pure D3D12 math** (pitch 256, placement 512, BC blocks, planes).
  Write a unit test per format class against d3dx12's formulas (`d3dx12_resource_helpers.h`,
  `d3dx12_property_format_table.h`).
* Every resource goes into the device-wide **residency set** at creation and leaves it at deferred
  destruction. Argument-buffer-referenced resources are otherwise invisible to Metal.
  `MakeResident`/`Evict` → `S_OK` no-ops.

**Descriptor heaps and MSC tables**

* CBV_SRV_UAV and SAMPLER, shader-visible → one shared `MTLBuffer` of `IRDescriptorTableEntry`
  (`{gpuVA, textureViewID, metadata}`, 24 B; `metal_irconverter_runtime.h:106`).
  `GetDescriptorHandleIncrementSize = 24`. CPU handle = `contents + i*24`, GPU handle =
  `gpuAddress + i*24`.
* Non-shader-visible CBV/SRV/UAV/SAMPLER heaps use host memory with the **same 24-byte encoding**,
  so `CopyDescriptors(Simple)` is a `memcpy`.
* RTV/DSV heaps: host memory, increment 32. Each entry is a POD `{view_id: u32, kind, fmt, mip,
  slice range, plane}` into a view table owned by the resource.
* **View objects are owned by the resource** (a cache keyed by view desc: `MTLTexture` views for
  format/mip/slice subsets, texture-buffer views for typed buffers), not by descriptors.
  Descriptors are raw IDs, as in D3D12. Debug builds keep a side table that validates
  `textureViewID`s at submit time.
* Encoders: textures `IRDescriptorTableSetTexture(entry, tex, minLODClamp, 0)`, raw/structured
  buffers `IRDescriptorTableSetBuffer(gpuVA+off, metadata)`, **typed buffers**
  `IRDescriptorTableSetBufferView` (`runtime.h:344`; texture-buffer view + element offset, which
  handles the linear-alignment limit), samplers `IRDescriptorTableSetSampler(entry, s, lodBias)`
  with **`supportArgumentBuffers = YES`**. Dedupe samplers in a global cache; Metal caps unique AB
  samplers (~2048 on Apple GPUs) **[A]**.

**Root signatures**

* Parse RTS0 v1.0/1.1 (DXBC container) → `IRVersionedRootSignatureDescriptor` →
  `IRRootSignatureCreateFromDescriptor` (`metal_irconverter.h:404`).
* Implement `D3D12SerializeRootSignature/Versioned` to produce **real RTS0**. One parser then
  handles game-serialized blobs, binrfx-embedded blobs (`FORMATS.md §2.2`) and DXIL-embedded
  ones. Round-trip tests: serialize ↔ parse on the three global RS (§1.3) and on every embedded
  RS in the binrfx corpus. Also implement `D3D12CreateRootSignatureDeserializer` (cheap) and
  `ID3DBlob`.
* **Top-level argument buffer (TLAB):** take the per-root-parameter offsets **from MSC reflection
  of a compiled shader**, not from assumptions, and assert they equal our computed layout in a test
  (expected: in order, u64 per table/root descriptor, 4·N bytes per constants block **[A]**).
* Each list keeps a CPU shadow TLAB per bind point (graphics/compute). `SetGraphicsRoot*` writes
  the shadow and sets a dirty flag. At the next draw/dispatch, if dirty, bump-allocate a copy from
  the allocator arena and bind its address at `kIRArgumentBufferBindPoint = 2` for all stages (MTL4
  argument table `setAddress:atIndex:`; `runtime.h:732`). Root CBV = raw `gpuVA` store; table =
  GPU handle store; no translation.
* SM6.6 heap bind points 0/1 are unused by this game (verified: no program uses heap indexing).
  Bind them anyway when a heap is set; it's cheap and future-proof.
* Draw params: every draw sets `IRRuntimeDrawParams` / uniforms at bind points 4/5 (as
  `IRRuntimeDraw*` do, `runtime.h:900+`), or `SV_VertexID`/`SV_InstanceID` base semantics break.
  Use `IRRuntime4*` for Metal 4 encoders (`runtime.h:606+`).

### 3.7 Reserved (tiled) resources

There's one `CreateReservedResource` site (`0x142080120`). For M3, report `TiledResourcesTier =
NOT_SUPPORTED`, then check in a trace whether the game still calls it:

* If it does, back the reserved resource with a **fully committed** allocation.
  `UpdateTileMappings`/`CopyTileMappings` become no-ops. Results are identical, it only costs
  memory. Log it with `stub!`.
* Later: placement sparse heaps (`MTLHeapTypeSparse` / Metal 4 placement sparse) **[A]**.

### 3.8 Barriers and hazards

Metal 4 has no automatic hazard tracking. Start conservative, then optimize with traces:

* **At the start of every command list:** a queue barrier, all stages → all stages. It's cheap and
  kills a class of cross-list bugs that real drivers hide.
* `ResourceBarrier` batch: compute the union of src→dst **stages** from D3D12 states
  (`RENDER_TARGET`/`DEPTH_WRITE` → fragment/raster, `UNORDERED_ACCESS` → dispatch/vertex/fragment,
  `COPY_*` → blit, `*_SHADER_RESOURCE` → the consuming stages, …).
  * Inside a compute encoder, a UAV barrier becomes
    `barrierAfterEncoderStages:Dispatch beforeEncoderStages:Dispatch`.
  * Anything inside a render encoder (or any transition involving a bound attachment) ends the
    encoder; the next encoder starts with `barrierAfterQueueStages:src beforeStages:dst`.
  * Aliasing barrier = full barrier.
  * Split barriers (`BEGIN_ONLY`/`END_ONLY`): act at END.
* Cross-queue: only through fences (D3D12 already requires that). With untracked resources nothing
  else is needed.

### 3.9 Render passes

Two paths:

* `List4::BeginRenderPass` (the game holds List4; whether it uses render passes is to be confirmed
  in a trace) maps 1:1 to `MTL4RenderPassDescriptor`.
* `OMSetRenderTargets` path: keep "pending pass" state (RT/DSV set, per-attachment pending
  load = load | clear(color) | dontCare from `DiscardResource`). Open the encoder lazily at the
  first draw.
  * `ClearRenderTargetView`/`ClearDepthStencilView` on a bound attachment before any draw → fold
    into the load action. Otherwise end the encoder and reopen with clear.
  * A clear of an unbound view → a draw-less pass with loadAction clear.
  * Clear rects → pass with load + scissored clear via a tiny utility pipeline (see below).
  * Store action is always `store`, unless the next op on that attachment is a known Discard.

On Apple TBDR every pass break costs bandwidth. It's fine for the menu; optimize later with trace
data.

**Utility kernels and pipelines** (ClearUAV, query resolve, ExecuteIndirect translation, clear
rects, MSAA resolve with non-trivial formats): small, ours, unit-tested. They're infrastructure,
not substitutes for game shaders. I read AGENTS.md §8 as allowing this; **confirm with the owner**
and list them in docs.

### 3.10 PSOs and the shader pipeline

* Entry points: `CreateGraphicsPipelineState` (10), `CreateComputePipelineState` (11),
  `Device2::CreatePipelineState` (47). The stream parser must respect **pointer-size alignment
  of subobjects** and unknown-subobject errors. Use `d3dx12_pipeline_state_stream.h` as the spec.
* Per stage: DXIL → MSC with `IRCompilerSetGlobalRootSignature(rootsig)`,
  `SetMinimumGPUFamily(Apple7)`, `SetMinimumDeploymentTarget(macOS, "26.0")`, compatibility flags
  as needed (`SamplerLODBias`, `TextureMinLODClamp`, `BoundsCheck` in debug;
  `metal_irconverter.h:190-203`). **Cache key** = BLAKE3(DXIL bytes) ⊕ RS hash ⊕ MSC version ⊕
  options. Disk: `~/Library/Caches/aw2-port/msc/<key>.metallib` + reflection JSON (AGENTS.md §8).
* Run conversion in an **arm64 helper** (`aw2-shaderd`, spawned at loader start, talking over a
  socketpair or shared memory). That gives native-speed LLVM, crash isolation and a parallel
  pool. The x86 in-process path stays as a fallback; the MSC dylib is universal (`lipo`: x86_64
  arm64).
* **Warm the cache offline:** a tool converts every binrfx program with its bundle's embedded RS.
  The keys match if the runtime RS bytes match.
* `MTLRenderPipelineState` / `MTLComputePipelineState` are built from the MTL4/Metal function
  plus fixed-function state:
  * blend (replicate RT0 when `IndependentBlendEnable` is false), alphaToCoverage, sample count,
    attachment formats, `inputPrimitiveTopology` class.
  * Depth/stencil goes into a separate cached `MTLDepthStencilState`.
  * Rasterizer state (cull, fill, front face, depth bias/clamp, depth clip → `setDepthClipMode`)
    is stored on the PSO object and applied at `SetPipelineState`.
  * Unsupported state (logic ops, depth-bounds, conservative raster, ForcedSampleCount) →
    report unsupported caps, log if used.
* Input layout → `MTLVertexDescriptor`. Attributes start at `kIRStageInAttributeStartIndex = 11`;
  vertex buffers at `kIRVertexBufferBindPoint = 6 + slot`. Use **`MTLBufferLayoutStrideDynamic`**
  and `setVertexBuffer:offset:attributeStride:`, because D3D12 strides come from
  `IASetVertexBuffers`, not the PSO. Otherwise use MSC stage-in synthesis
  (`IRMetalLibSynthesizeStageInFunction`, `metal_irconverter.h:540`) for formats a vertex
  descriptor can't express.
* `D3D12_PIPELINE_STATE_FLAG`, `CachedPSO`: `GetCachedBlob` returns our cache key. Accept and
  validate on input.
* PSO creation is synchronous in D3D12, and the game calls it from job threads. Make the cache
  thread-safe with single-flight per key. **Later:** return the PSO immediately with a compile
  future and block at first `SetPipelineState`; combine with Metal binary archives / MTL4
  pipeline datasets.

### 3.11 Indirect, queries, copies, formats

* **ExecuteIndirect.** The game builds command signatures generically (`0x14206da30`), with root
  argument changes (an arg type translated to 6 = CBV root descriptor) against the global RS
  (`+0x20/+0x28/+0x30`).
  * Plan: a utility compute kernel reads the D3D12 arg buffer + count buffer and **encodes an
    `MTLIndirectCommandBuffer`**: per command, per-draw TLAB copy with patched constants/VAs,
    draw params, and the indirect draw/dispatch args. Then
    `executeCommandsInBuffer:indirectBuffer:` takes the GPU-written count.
  * Interim, only if `MaxCommandCount` is small and there's no count buffer: N CPU-encoded
    indirect draws.
  * This is probably needed only once the 3D scene renders. Schedule it by trace evidence.
* **Queries.**
  * Occlusion → visibility result buffer (counting mode, per-query offset).
  * Timestamps → `MTLCounterSampleBuffer` at **encoder/stage boundaries** (Apple GPUs can't
    sample at draw boundaries); snap mid-pass `EndQuery` to the next boundary and log once.
    `GetTimestampFrequency` = 1e9, converting GPU ticks with `sampleTimestamps` correlation.
  * Pipeline statistics → zeros + `stub!`.
  * `ResolveQueryData` → GPU copy/convert kernel, ordered on the queue.
* **Copies.** `CopyBufferRegion`, `CopyTextureRegion` (footprint ↔ texture: `bytesPerRow` =
  `RowPitch`, BC block rules; depth/stencil planes need
  `MTLBlitOption{Depth,Stencil}FromDepthStencil`), `CopyResource`, `ResolveSubresource` (render
  pass resolve or compute), `ClearUnorderedAccessView{Uint,Float}` (`fillBuffer` when
  byte-uniform, else a kernel). COPY queue → blit encoders on its own `MTL4CommandQueue`.
* **Formats:** a table-driven `DXGI_FORMAT → MTLPixelFormat` plus capability bits (from the Apple
  GPU family tables) feeding `FORMAT_SUPPORT`/`FORMAT_INFO`/`MSAA_QUALITY_LEVELS`.
  * **D24_UNORM_S8_UINT, R24G8_TYPELESS, R24_UNORM_X8 / X24_G8 → Depth32Float_Stencil8 /
    X32_Stencil8** (no D24S8 on Apple Silicon).
  * D32_S8X24 → Depth32Float_Stencil8.
  * Typeless or view-cast resources get `MTLTextureUsagePixelFormatView`, **only** when needed,
    because it can disable lossless compression.
  * BC1–7 are fine on Apple Silicon macOS. 3-channel 32-bit (R32G32B32_*) textures aren't
    supported, buffers only.
  * Report `TypedUAVLoadAdditionalFormats = TRUE` (the gate needs it), but check Metal's
    read-write texture tier list per format. Formats Metal can't `read_write` (e.g.
    `R11G11B10_FLOAT`, `R10G10B10A2` UAV loads) need a plan **[A]**; log them at view creation.

---

## 4. Mesh shaders, DXR, and what to report for the first frame

**Verified fallbacks.** Mesh-shader use is a flag (`+0x9e` = `OPTIONS7.MeshShaderTier > 0`, also
disabled by `param_2 & 4`). With it off, the mesh root signature (`+0x28`) and mesh command
signatures aren't built (`FUN_1420719f0` builds RS type 3 only if `+0x9e`; `0x14206de51` checks `+0x9e`), and every MS bundle has a
VS twin. RT is `+0xac` (tier ≥ 1.1); gate checks don't require RT, mesh or VRS. The RT shaders are
the LIB/RayQuery programs (`rt_*`, 1,047 RT1.1-flagged); SM6.9 SER only for path tracing.

**`CheckFeatureSupport` table for M3:**

| Feature | Report |
|---|---|
| `FEATURE_LEVELS` | max **12_1** (12_2 needs DXR 1.1 + mesh + VRS2 + SF) |
| `OPTIONS` | DoublePrecision **FALSE**, LogicOp FALSE, MinPrecision 16-bit, TiledResourcesTier **0**, **ResourceBindingTier 3**, PSSpecifiedStencilRef FALSE (MSC support unverified), **TypedUAVLoadAdditionalFormats TRUE**, ROVs FALSE, ConservativeRaster 0, MaxGPUVABitsPerResource 40, StandardSwizzle FALSE, CrossNode 0, VPAndRTArrayIndexFromAnyShader TRUE (3 programs use it; Metal supports it), **ResourceHeapTier 2** |
| `SHADER_MODEL` | **6.6** (gate needs ≥ 6.5; no game program needs > 6.5 off the RT path) |
| `OPTIONS1` | WaveOps TRUE, WaveLaneCountMin = Max = **32**, Int64ShaderOps TRUE, ExpandedComputeResourceStates TRUE |
| `OPTIONS2` | DepthBoundsTest FALSE, ProgrammableSamplePositions 0 |
| `OPTIONS3` | CopyQueueTimestamp FALSE, WriteBufferImmediate FALSE, ViewInstancing 0, Barycentrics FALSE |
| `OPTIONS4` | Native16BitShaderOps TRUE |
| `OPTIONS5` | RenderPassesTier 0, **RaytracingTier NOT_SUPPORTED** |
| `OPTIONS6` | **VRS tier 0** |
| `OPTIONS7` | **MeshShaderTier NOT_SUPPORTED**, SamplerFeedbackTier 0 |
| `OPTIONS9` | AtomicInt64 FALSE, WaveMMA 0 |
| `OPTIONS12` | **EnhancedBarriers FALSE** |
| `OPTIONS13` | UnrestrictedBufferTextureCopyPitch FALSE |
| `OPTIONS16` | GPUUploadHeap FALSE (can flip to TRUE later; unified memory) |
| `OPTIONS21` | WorkGraphs 0 |
| `OPTIONS22` | all zero |
| `ROOT_SIGNATURE` | 1.1 |
| `ARCHITECTURE(1)` | UMA TRUE, CacheCoherentUMA TRUE, TileBasedRenderer TRUE (honest; watch for TBDR-specific paths **[A]**) |
| `GPU_VIRTUAL_ADDRESS_SUPPORT` | 40/40 |

* Check `DataSize` exactly (the game passes 0x3C for OPTIONS and 0xC for OPTIONS5). A mismatch
  returns `E_INVALIDARG`, like the runtime. Unknown feature → `E_INVALIDARG` + log.
* **M4 (later):** mesh = MSC MS → Metal mesh pipelines (MS without AS; `DispatchMesh` →
  `drawMeshThreadgroups`); flip `MeshShaderTier` to 1 only after the VS path is golden.
* DXR = MSC `ir_raytracing.h` (shader identifiers / SBT emulation, intersection function tables,
  AS builds → `MTLAccelerationStructure`, instance desc conversion kernel), RayQuery → MSL
  intersection queries. Report tier 1.1 only when both the LIB and inline paths work. SM6.9 SER
  isn't needed for tier 1.1: set `OPTIONS22.SERActuallyReorders = FALSE`.

---

## 5. Rosetta, memory, tooling

### 5.1 Rosetta / x86_64

* The process is x86_64, so **Metal.framework and the AGX user-mode driver run as Rosetta-
  translated x86_64 slices** **[A, very likely]**, as do objc_msgSend and our Rust D3D12 layer.
  CPU cost per API call is the risk, not the GPU. Mitigations:
  * Metal 4 (no implicit retains, argument tables, fewer calls per draw).
  * Dirty-state dedupe in our encoder (skip redundant PSO/TLAB/vertex-buffer binds).
  * ICBs for indirect.
  * Never call into AppKit synchronously from render threads.
  * D3DMetal (GPTK) runs this way inside x86_64 Wine, so it's viable **[A]**.
* `objc2-metal`: check that its Metal 4 bindings cover `MTL4*`. If not, generate them with
  `header-translator` against the 26/27 SDK, or hand-write the dozen classes needed **[A]**.
* Threads: Metal objects may be used from guest threads, but autorelease pools are needed on
  non-Cocoa threads. Wrap each COM entry that touches ObjC in `autoreleasepool` (or use only
  `new*`/owned APIs).
* W^X: our EAT thunk pages are created RW then RX before first use, so Rosetta translates them
  once. Never patch them later.

### 5.2 Memory and residency

Unified memory. Report "dedicated" = `recommendedMaxWorkingSetSize`, which passes gate #3 on
≥ 16 GB Macs **[A]**. One residency set holds everything, so commit and remove allocations in
batches (commit once per ECL if dirty). Use upload arenas per allocator (reset with it).
Deferred destruction is per queue timeline.

### 5.3 Debugging

* **Debug layer:** `MTL_DEBUG_LAYER=1`, `MTL_SHADER_VALIDATION=1` (slow; use with
  `AW2_D3D12_TRACE` replays).
* **GPU capture:** an `AW2_MTL_CAPTURE_FRAME=N` env var drives `MTLCaptureManager` → `out/*.gputrace`.
  Verify Xcode opens captures from x86_64 processes **[A]**. Instruments Metal System Trace.
* **Logging:** generated per-method trace thunks (`RUST_LOG=d3d12=trace`) including HRESULTs and
  object names (`SetName` → private data). This complements the loader's `--trace`.
* **Shaders:** keep `<key>.dxil` next to `<key>.metallib`. Disassemble with `dxc -dumpbin` (brew
  `directx-shader-compiler` **[A]**). MSC reflection JSON on disk.

### 5.4 D3D12 call trace and replay (build this early)

* Recorder at the COM layer, `AW2_D3D12_TRACE=out/traces/x.d3dtrace`. It writes a binary stream
  of `(thread, object id, iface, slot, args)`: deep-copied descs, shader bytecode by hash, initial
  data, upload-heap bytes **diffed at each `ExecuteCommandLists`**, window size at Present.
* `tools/d3d12-replay` (native arm64, links `aw2-d3d12`) replays into an offscreen swapchain and
  writes a PNG per Present.
* Golden-image tests then run without the game, in seconds. You can bisect barrier/format bugs,
  capture a replay under Metal debugging at native speed, and diff behaviour across backends
  (Metal 4 vs 3).
* Optional oracle: the same trace replayed on a real Windows D3D12 by a 300-line Windows replayer
  gives ground truth images.
* Also promote the scratch analyzers to `tools/`: `dxil_stats.py` (stage/SM/flag census) and
  `pe_iids.py` (which IIDs each PE references). Re-run them when the game updates.

---

## 6. Milestones (each with an objective exit test)

| Step | Work | Exit test |
|---|---|---|
| **M3.0 Plumbing** | EAT synthesis for builtins; d3d12 ordinals 101/102; builtin-override list; builtin `sl.interposer`; stub `d3d11`; Metal 4-under-Rosetta spike (create MTL4 queue, clear a texture, read back) | Loader unit test: EAT == GetProcAddress for every builtin export. `--no-run` shows sl.interposer as builtin. Boot passes `0x14208d281` with no fault. Spike prints the readback value. |
| **M3.1 DXGI** | factory/adapter/output; LUID; memory | Log shows the adapter loop ending with NOT_FOUND at index 1. No `TerminateProcess(0xbadf00d)`. Unit tests for desc fields. |
| **M3.2 Device + caps** | device object, superset vtables, QI, private data, the §4 table | `gpu_feature_validator` returns 1 (no `0xb501e7f`). Debug dump of `DAT_14397fc58+0x9e..0xaf` = mesh 0, VRS 0, RT 0. Golden test of every `(feature, size) → bytes`. |
| **M3.3 Objects without draws** | heaps/resources/views, descriptor heaps, RTS0 parse/serialize, root sigs, fences/events, allocators/lists (record-only), queues, swapchain + Present blit | No UNIMPLEMENTED traps for 60 s. Present count rising in the log. Window black. The first trace captured and replayed. RTS0 round-trip tests on all binrfx RS + the 3 global RS. |
| **M3.4 Copies/clears** | Map/upload, copies, footprints, clears, lazy passes, barriers v0 | `AW2_SCREENSHOT` shows the game's clear colour / loading background. Footprint unit tests. Replay golden PNG stable. |
| **M3.5 Shaders/PSOs** | MSC helper + disk cache, TLAB, compute + graphics draws, vertex descriptors, samplers, queries v0 | Startup Bink logos / loading screen render with game shaders (`bink.binrfx`, `copy`, `tonemap`); screenshot matches a reference within a perceptual threshold. Zero Metal validation errors with `MTL_DEBUG_LAYER=1`. |
| **M3.6 Main menu** | cohtml/Renoir path (`CoherentGT`, `font`), ExecuteIndirect/ICB if the menu scene is GPU-driven, timestamp snapping | Main menu visible; screenshot vs golden; 5-minute idle with stable memory (`currentAllocatedSize` flat), no validation errors. Trace replay reproduces the menu image on arm64. |

---

## 7. Top-15 pitfalls

1. **MSVC struct-return ABI** for COM methods: a hidden pointer after `this` (§3.1). It corrupts
   silently.
2. **Superset vtables:** the game calls newer slots through older interface pointers (slot 38 on a
   bare `IDXGISwapChain*`; `DispatchMesh` would come through List4/5). Fill every slot, with named
   traps.
3. **Silent HRESULT handling** in the game (`FUN_14205d410` only reacts to OOM/removed). Every
   failing return must be logged with method + args.
4. **`SV_VertexID`/`SV_InstanceID` bases:** MSC draw params at bind points 4/5 on *every* draw,
   indirect included.
5. **Dynamic vertex strides:** D3D12 strides are command-list state. Use
   `MTLBufferLayoutStrideDynamic`.
6. **Residency:** anything reachable only through descriptor heaps is invisible to Metal unless it
   is in the residency set. The symptom is garbage, not errors.
7. **Missing barriers with Metal 4's untracked model:** start with a full barrier per command list
   and per barrier batch; relax only with replay tests.
8. **Lifetime:** free nothing (views, TLAB arenas, PSOs, resources) before the queue timeline
   passes the release point.
9. **D24S8 and depth/stencil copies:** format substitution plus plane-specific blit options;
   `R24G8_TYPELESS` SRV/stencil views.
10. **Sampler limits and semantics:** dedupe; `supportArgumentBuffers`; only 3 border colours; LOD
    bias via MSC compat flag; min/max reduction support unverified **[A]**.
11. **Fence edge cases:** event set immediately when the value is already reached, `NULL` event =
    blocking wait, wait-before-signal, `UINT64_MAX` on device loss.
12. **`GetCopyableFootprints`/row-pitch math** must match D3D12 exactly (256/512 alignment, BC
    blocks, planes, mips). The game computes upload offsets from it.
13. **Typed UAV loads and typed buffer alignment:** the gate forces `TypedUAVLoadAdditionalFormats
    = TRUE`, but Metal's read-write tier doesn't cover every DXGI format; typed buffer views need
    `IRBufferView` offsets.
14. **Concurrency:** object creation from job threads (`0x142263520`), PSO creation storms,
    descriptor writes racing with copies (legal in D3D12). Make caches single-flight; keep
    descriptor writes lock-free.
15. **Rosetta CPU cost and PSO hitching:** MSC + Metal driver translated. Use the arm64 shader
    helper, the offline cache warm-up, and dirty-state dedupe from day one.

Honourable mentions:

* Clear-before-draw folding versus TBDR pass breaks.
* Primitive restart is always on in Metal for strips (D3D12 `IBStripCutValue` disabled differs).
* `ResizeBuffers` with outstanding `GetBuffer` refs must fail.
* `CreatePipelineState` stream alignment.
* NRI needs ordinal imports and `D3D11CreateDevice` to fail, not trap.
* FFX FG swapchain stays off.
