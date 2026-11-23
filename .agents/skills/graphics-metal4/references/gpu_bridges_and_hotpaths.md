# GPU Vendor Bridges, MetalFX Upscalers, DXGI & Hot-Path Rules

## 1. Upscaler & GPU Vendor Middleware Bridges (Zero `objc2-metal` in `win32-rt`)

All GPU upscaler and middleware bridges live in `crates/win32-rt/src/misc/` and route strictly
through `crate::d3d12` / `crate::d3d11` and safe Rust types in `crates/d3d12-metal`:

| Module | Real Behavior | Honest Refusals / Caveats |
|---|---|---|
| `amd_fidelityfx_loader_dx12.rs` | MetalFX Temporal (`upscale_temporal`) + Spatial/bilinear fallback; exact FSR3 Halton `(2,3)` jitter; `FRAMEGENERATIONSWAPCHAIN_{WRAP,NEW,FOR_HWND}_DX12`; clears `outReactive` via `ClearUnorderedAccessViewFloat` | Refuses Frame Generation (`0x0002_xxxx`, `INTERPOLATION*`) with `FFX_API_RETURN_NO_PROVIDER` (`4`) + `stub!` |
| `nvngx_dlss.rs` | D3D12 `SuperSampling` (`1`) & `ImageSignalProcessing` (`9`) → MetalFX Temporal (`NVSDK_NGX_D3D12_EvaluateFeature`), `win64_bridge!` for float/double `Set` slots | `Available = 0` & `NVSDK_NGX_RESULT_FAIL_FEATURE_NOT_SUPPORTED` (`0xBAD0_0001`) for FrameGen (`11`), DeepDVC (`12`), RayRecon (`13`), Vulkan NGX. Missing keys return `FAIL_INVALID_PARAMETER` (`0xBAD0_0005`), never `1` |
| `libxess.rs` (D3D12) & `libxess_dx11.rs` (D3D11, 51 exports + `INTC_D3D11_Xess*`) | XeSS 1.3 scaling factors (`3.0`/`2.3`/`2.0`/`1.7`/`1.5`/`1.3`/`1.0`), `win64_bridge!` for all float-taking exports, real MetalFX Temporal/Spatial/bilinear (D3D11 extracts backing `d3d12_metal` resources, flushes `imm_exec`, uses 4-slot `CommandAllocator` ring + `queue.defer_release`) | Rejects `XESS_INIT_FLAG_EXTERNAL_DESCRIPTOR_HEAP`; logs non-zero subrect base offsets once via `stub!` |
| `amd_ags_x64.rs` | Exact 32-byte `AGSGPUInfo` & 120-byte `AGSDeviceInfo`, real Apple Silicon Metal adapter info (`vendorId = 0x106B`, `AGSAsicFamily_Unknown`), real `IUnknown::Release`, command-list `BeginEvent`/`EndEvent`/`SetMarker` | `extensionsSupported = 0` |
| `nri.rs` + `nrd.rs` | D3D12-backed NRI + native NRD v4.3.6 with 7 DXC-compiled signed `cs_6_0` DXIL compute shaders across 21 denoiser modes | `nrd.dll` is intentionally NOT in `BUILTIN_OVERRIDES` (retail PE runs via ARM64 JIT; builtin serves as fallback) |
| `dstorage.rs` | Real Microsoft GDeflate v1 (`libz` raw DEFLATE `windowBits = -15`, 8B `TileStreamHeader` + `tile_offsets`), FIFO queue, `MEMORY`/`BUFFER`/`TEXTURE_REGION`/`MULTIPLE_SUBRESOURCES*` D3D12 CopyQueue uploads, `ID3D12Fence` & Win32 `SetEvent` sync | Corrupt streams fail honestly with `E_DSTORAGE_DECOMPRESSION_ERROR` (never `memcpy`); unknown compression fails `E_NOTIMPL` |
| `directml.rs` | Real `IDMLDevice1`/`Operator`/`CompiledOperator`/`BindingTable`/`CommandRecorder` executing CPU-backed D3D12 buffer tensors (Gemm, Conv1D/2D/3D, Pooling, ElementWise, Activations, Reduce, Norm, Cast, Join, Split, Graph) | Runs via `crate::d3d12` + `d3d12_metal` safe Rust wrappers |

## 2. DXGI Adapter Reporting, Display Registry & Fullscreen Transitions

- **Adapter & WDDM version (`win32-rt/src/dxgi/adapter.rs`, `d3d11/device.rs`):**
  - Reports non-zero `SubSysId = 0x106B_0001`, `Revision = 1`, and packed WDDM 3.2 Metal 4 UMD
    driver version `32.0.15.4000` (`(32 << 48) | (0 << 32) | (15 << 16) | 4000`) via
    `CheckInterfaceSupport` for `IDXGIDevice[1]` / `ID3D10Device` / `ID3D11Device`.
  - `advapi32.rs` seeds `HKLM\SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}\0000`
    (`DriverVersion`, `DriverDate`, `DriverDesc`, `ProviderName`, `MatchingDeviceId`, VRAM sizes)
    so games checking driver age/version via registry pass cleanly.
- **`IDXGISwapChain::SetFullscreenState` (`d3d11/swapchain.rs`, `d3d12/swapchain.rs`, `macos/appkit.rs`):**
  - Toggles native macOS borderless fullscreen on the target/containing screen (saving & restoring
    window frame, `styleMask`, `level`, `collectionBehavior`, and title; auto-hiding menu bar & Dock
    while key). Guarded against `window_set_frame`/`window_set_style` clobbering while active;
    cleanly tracks state in headless or iOS `single_surface_mode()`.

## 3. CPU Hot-Path Rules for D3D12 Command Recording & CS Thread

Measured via `/usr/bin/sample` on `d3d12-metal-cs` and real 60+ FPS gameplay:

1. **Zero-reallocation `TrackedSet<T>` (`crates/d3d12-metal/src/metal/cmdlist.rs`):**
   - Never replace `Option<Vec<...>>` with `Some(Vec::new())` or `.clone()` inside per-encoder or
     per-barrier paths (`ib_reset_tight`, `ib_clear_satisfied`, `note_compute_barriers_tight`).
   - Always reuse capacity via `TrackedSet<T>` (`clear_exact()`, `invalidate()`, `copy_from()`) or
     in-place `clear()` + `extend_from_slice()`.
2. **Coalesced `ResRegistry` lookups (`crates/d3d12-metal/src/metal/resource.rs`, `cmdlist.rs`):**
   - Acquire `with_resource_registry` **once** per `dispatch()` (or skip when all slots are empty).
   - Keep `ResRegistry::tex_ids` on `U64MixHasher` (not SipHash) and preserve the single-buffer
     fast path in `push_buffer_span` (skipping the second `partition_point` when non-aliasing).
3. **Zero-alloc D3D12 COM recording (`crates/win32-rt/src/d3d12/{cmdlist,descheap}.rs`):**
   - Use size-tiered inline stack arrays for `ResourceBarrier` (`1`, `2..=8`), `RSSetViewports`
     (`0`, `1..=2`), `SetDescriptorHeaps` (`1..=2`), `Set*Root32BitConstants` (`0`, `1`, `2..=4`,
     `5..=16`, `17..=64`), `IASetVertexBuffers` (`0`, `1`, `2..=4`, `5..=24`), and `Clear*` rects
     (`0`, `1..=4`), falling back to `Vec` only above the inline tier.
   - Read all guest descriptors/arrays via `ptr::read_unaligned`.
   - Keep `descriptor_size(ty)` `#[inline(always)]` and coalesce contiguous `CopyDescriptors` runs.
4. **Apple AGX `MTL4CounterHeap` ceiling (~31–32 per process):**
   - Never allocate one `MTL4CounterHeap` per draw or per D3D11 query object! Pool queries into
     shared 4096-slot heaps (`d3d11` timestamps, `d3d12-metal/src/metal/passtiming.rs`).
