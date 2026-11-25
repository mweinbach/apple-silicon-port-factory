# ToDos

Live engineering backlog for the port factory. Check items off as commits land, and link the
relevant crate/test when you add new ones.

Related docs: [What's Next](WHATS_NEXT.md) · [What's Needed](WHATS_NEEDED.md) ·
[End Goal](END_GOAL.md) · [Status](STATUS.md) · [Roadmap](ROADMAP.md)

---

## P0 — Immediate Priorities

- [ ] Take a second offline single-player game past menus on both `port-runner` and the packaged
      app. Keep game-specific settings in its manifest.
- [x] Report hangs before the first frame. The launch watchdog records heartbeats, thread
      activity and guest stacks. `pre_present_hang_e2e` checks a blocked guest and a healthy
      message pump. Fatal crash files include structured JSON and guest `.pdata` frames.

- [x] **First non-AW2 shipping game to rendered frame on `port-runner`:** **Overwatch 2**
      (`games/overwatch/game.toml`, Steam AppID `2357570`) now runs end-to-end on `port-runner` and
      inside its self-contained bundle `out/apps/Overwatch.app` (`15/15` `bundle-verify` PASS),
      rendering **420+ real 1080p Metal 4 frames** through the Blizzard Entertainment splash screen
      to the interactive **Overwatch 2 Login Screen** (`2.25.1.0`) with live 48 kHz CoreAudio,
      Metal 4 Geometry Shader emulation, pooled 4096-slot `MTL4CounterHeap` timestamp queries,
      `PORT_AOT=2` live AOT build/install (0 faults), live Schannel TLS 1.2 HTTPS, real CCD display
      config (`1920x1080 @ 60/1`), `CryptQueryObject`/`CryptMsgGetParam`/`CertFindCertificateInStore`,
      `GetSystemTimes`, `WSADuplicateSocket*`, `IAudioSessionManager2`, and dynamic arm64
      `steamclient.dylib` IPC bridging. *(Note: Blizzard's `Overwatch_loader.dll` Byfron anti-tamper
      watchdog still raises its periodic `0xc00000e5` termination at ~30–35s, which we do not
      bypass; next target is a playable single-player / offline Steam title for full end-to-end
      interactive gameplay.)*
- [x] **Live Windows Vulkan executable through `port-runner` (`crates/win32-rt/src/misc/vulkan_1.rs`, `crates/port-loader/tests/vulkan_e2e.rs`):**
      Real Windows x86-64 Vulkan PE `.exe` compiled via `zig cc` runs end-to-end through
      `port-runner` against real `libMoltenVK.dylib` (`PORT_TEST_MOLTENVK=1`, Apple M3 Ultra,
      `Vulkan 1.3.323`), passing both headless and live `NSWindow` modes (2/2): static + dynamic
      `vkGetDeviceProcAddr` dispatch, Win64→AAPCS64 float-arg adapters, allocator sanitization,
      SPIR-V graphics pipeline draw + swapchain present, `SetWindowPos` `256x192` → `320x200` resize
      with `_dispatch_main_q` sync + `oldSwapchain` recreation, and exact GPU pixel readback.
- [x] **Live D3D11 game/sample rendering through `port-runner` (`crates/win32-rt/src/d3d11/`, `crates/port-loader/tests/d3d11_e2e.rs`):**
      Real Windows x86-64 D3D11 PE `.exe` compiled via `zig cc` runs end-to-end through
      `port-runner` on Metal 4 with `libdxilconv.dylib` + `libmetalirconverter.dylib` (2/2 passing
      headless + live `NSWindow` under `MTL_DEBUG_LAYER=1` with 0 validation errors):
      `D3D11CreateDeviceAndSwapChain`, `IDXGIDevice3`/`IDXGISurface2` QI chains, standalone
      `CreateDXGIFactory*` D3D11 swapchain routing, SM 5.0 VS/PS + depth-stencil (`Less`), compute
      dispatch, `RGBA8 → BGRA8` compute-swizzled `CAMetalLayer` presentation, `ResizeBuffers`
      (explicit + `0x0` auto-size), and exact staging pixel/buffer readbacks.
- [x] **Alan Wake 2 Cauldron Lake movement bug (`games/alan-wake-2/`):** Root-caused via live
      `AW2_CAMERA_TRACE=1` telemetry and binary disassembly to two bugs: (1) `unknown_14212c3e0` in
      `cauldron_hot_za13.rs` allocated a 0x30-byte `buf_1b0` instead of 0x58 bytes and passed a
      zero-initialized `buf_1e0` (`Transform`) to `CollisionCache::fillCache`, yielding 0 triangles
      (`t=0`); and (2) default `aw2-loader` routed PhysX to the no-op builtin stub instead of the
      shipping PhysX 3.4 DLLs. Fixed both (10/10 `--diff-test` pass with strengthened 0x58-byte
      transform check); live `Return 0: The Cult` test holding `W` across frames `20800..23000`
      extracts `28..677` triangles and walks Nightingale 44m uphill along the forest trail
      (`y: -7.571 -> -5.670`, `on_g=1`) with visually verified frames and zero cyan void.

---

## CPU & Runtime (`crates/x86-runtime`, `crates/port-loader`)

- [x] Check finished JIT instruction words against Arm's encoding diagrams and independently
      assembled operands. [Run the A64 emitter test](DIAGNOSTICS.md#check-the-a64-words-emitted-by-the-jit)
- [x] Check in the complete A64 encoding catalog and precompute the encoding form for each
      emitted test word. [A64 catalog](../db/a64_encodings.tsv) · [Emitter matches](../db/a64_emitter_matches.tsv)
- [x] Give `RDTSC` and `RDTSCP` a live 1 GHz guest clock across the interpreter, JIT, AOT,
      and lifted Rust. Freeze the clock for differential comparisons. JIT and AOT reads
      skip division on 1 GHz and 24 MHz hosts. JIT, AOT and generated Rust select the
      host's ordered-counter instruction at runtime.
      [Guest timestamp semantics](DIAGNOSTICS.md#guest-timestamp-counter)
- [x] **Parallel offline `.aot` prewarming (`crates/x86-runtime/src/emu/aot.rs`):** Parallelized
      block translation in `aot_prewarm_all_mapped` across `PORT_AOT_THREADS` workers with reused
      `WorkerCaches` and deterministic ascending-IP output (bit-for-bit identical across 1/2/4/8
      threads; 4.63x on 6,400-block benchmark; prewarms all 4 Overwatch PE modules — 339,480 ARM64
      blocks / 57.4 MiB — in 0.91s on 32 threads).
- [x] **Jump-table CFG discovery in `.aot` prewarm:** Added sound abstract 16-GPR jump-table
      analysis in `aot.rs` covering MSVC `rva32`, Clang/GCC PIC signed-32, direct 64-bit pointer
      tables, and compact 8/16-bit indirection tables with `cmp`/branch bounds propagation
      (discovers 48 tables / 517 targets across Overwatch DLLs and 8 tables / 100 targets in
      `CrashMailer_64.exe`).
- [x] **Hot interpreter fallback audit:** Audited real non-AW2 workloads (`Overwatch.exe`,
      `CrashMailer_64.exe`, `bink2w64.dll`, `vivoxsdk.dll`, `steamclient64.dll`, `SDL3.dll`, etc.);
      implemented all 7 Intel SHA-NI instructions lowered directly to Apple Silicon ARMv8
      `FEAT_SHA1`/`FEAT_SHA256` hardware ops (NIST FIPS 180-4 verified, CPUID leaf 7 bit 29
      re-enabled), promoted `0x67` 32-bit addr-size override (`lea` + mem), `movnti`, `cwd`,
      `wait`/`rdsspq`, and AVX2 256-bit reg/mem-count shifts (`vpsll*/vpsrl*/vpsra*`) to
      `JIT_ARM64` (`0` non-trap `GENERIC_INTERP` instructions in 577M-instr live Overwatch run),
      expanded x87 FPU (`fdivr`/`ffreep`/`fistp`/`fstp`/`fld m80`/`fxam`/`fnclex`), and synced
      the [ops catalog](STATUS.md#catalog-totals).
- [x] **`.aot` verification in CI/local checks:** Added standalone multi-module `.aot` prewarm +
      shifted-base reload verification integration test under `PORT_AOT_VERIFY=1`
      (`crates/x86-runtime/tests/aot_multi_module_verify.rs`, 0 verify mismatches).

---

## Graphics & Shaders (`crates/d3d12-metal`, `crates/win32-rt`, `crates/dxil-metal`)

### Direct3D 11 (`crates/win32-rt/src/d3d11/`)
- [x] Implement `ID3D11DeviceContext::GenerateMips` (backed by Metal 4 `generateMipmapsForTexture:`
      via safe `d3d12-metal` `CommandList::generate_mips`, supporting immediate + deferred contexts,
      2D / 2D-array / cubemap textures, and mip/slice subranges).
- [x] Implement indirect draw/dispatch (`DrawInstancedIndirect`, `DrawIndexedInstancedIndirect`,
      `DispatchIndirect`, `CopyStructureCount` with GPU-timeline UAV append/counter buffers and
      `pUAVInitialCounts` across immediate + deferred contexts).
- [x] Implement `IDXGISurface::Map`/`Unmap` & `OpenSharedResource*` (`IDXGIResource::GetSharedHandle`,
      `IDXGIResource1::CreateSharedHandle`, `ID3D11Device::OpenSharedResource`,
      `ID3D11Device1::OpenSharedResource1`, `OpenSharedResourceByName` with cross-device GPU
      coherence; verified under `MTL_DEBUG_LAYER=1`).
- [x] Add GPU tests for D3D11 cubemaps, 2D texture arrays, depth-stencil testing, and multi-target
      MRT rendering (22/22 `d3d11` GPU tests pass under `MTL_DEBUG_LAYER=1` with 0 validation
      errors).

### Direct3D 12 & MetalFX (`crates/d3d12-metal/`)
- [x] Verify `MTL4FXSpatialScaler` live inside Alan Wake 2 (`PORT_METALFX=1` vs `PORT_METALFX=0`)
      for visual quality, HDR tonemap output, multi-resolution (`1440x810→1920x1080` and
      `960x540→1280x720`), and frame time: dispatches 1.0×/frame directly in `ColorProcessingMode::Hdr`
      on `RG11B10Float` (0 scratch copies, `-1.0` compute encoder/frame), **3.83×–4.65×** center-3D
      Laplacian sharpness vs bilinear with clean HDR tonemap (`<0.12%` median luma diff, 0 artifacts),
      and `10.67–11.31 ms/f` (`88.4–93.7 FPS`, `0.12–0.40 ms/f` faster than bilinear; interactive
      mid-gameplay windowed drag-resize remains to be checked during the full playthrough).
- [x] Evaluate and prototype `MTL4FXTemporalScaler` (`crates/d3d12-metal/src/metal/metalfx.rs`,
      `cmdlist.rs`): dynamically probes and drives Metal 4 `MTL4FXTemporalScaler` with full color,
      depth, motion vector, jitter, reset, reactive mask, exposure, and DRS inputs, scratch-output
      fallback, and graceful `TemporalMetalFx -> SpatialMetalFx -> BilinearCompute` fallback chain
      (4/4 MetalFX GPU tests pass under `MTL_DEBUG_LAYER=1` on Apple M3 Ultra, 0 validation errors).
- [ ] Validate Metal 4 barrier/residency behavior on a second D3D12 engine (non-Northlight) with
      `MTL_DEBUG_LAYER=1`.

### Vulkan (`crates/win32-rt/src/misc/vulkan_1.rs`)
- [x] Add an optional live-MoltenVK integration test (`PORT_TEST_MOLTENVK=1`) exercising instance,
      physical device, Metal surface (`CAMetalLayer`), logical device, dynamic proc forwarding, and
      SPIR-V shader module creation against real `libMoltenVK.dylib` (live-verified on Apple M3
      Ultra, Vulkan 1.3.323, 10/10 unit/driver tests + 2/2 live Windows x86-64 E2E tests).
- [x] Audit Vulkan 1.2/1.3 dynamic-rendering and synchronization2 extension forwarding (added core ↔
      KHR/EXT aliases, static exports for dynamic rendering / sync2 / timeline semaphores / buffer
      device address / copy_commands2 / maintenance4, and `VK_KHR_portability_subset` device
      injection).

### Shaders (`crates/dxil-metal/`)
- [ ] Verify offline SPIR-V / WGSL → `.metallib` end-to-end once Xcode Metal Toolchain is
      installed (`xcodebuild -downloadComponent MetalToolchain`).
- [x] Keep `dxil-metald` worker cleanly managed on process exit (`crates/dxil-metal/src/{bin/dxil-metald.rs,helper.rs}`):
      added macOS `kqueue(EVFILT_PROC | NOTE_EXIT)` parent-death watchdog, async-signal-safe
      `SIGTERM`/`SIGINT` exit, clean stdin EOF / `BrokenPipe` handling, and global `atexit` +
      `Drop` worker reaping (6/6 `tests/worker_lifecycle.rs` pass including abrupt parent `SIGKILL`,
      0 stray `dxil-metald` processes).
- [x] **Cross-device shader/PSO caching, cold-device progress UI & SSD `.app` portability (`crates/dxil-metal`, `crates/d3d12-metal/src/metal/pso_manifest.rs`, `crates/win32-rt/src/macos/appkit.rs`, `crates/port-loader`, `crates/x86-runtime`, `crates/port-factory`, `scripts/package_app.sh`):**
      Replaced fragile `mtime`/length converter fingerprinting with immutable `codesign`-invariant
      Mach-O `LC_UUID` (`irX.Y.Z-uuid-<LC_UUID>`, with legacy `irX.Y.Z-*` fallback matching +
      `PORT_SHADER_CACHE_FALLBACK`), added GPU/OS-independent `PsoManifest` (`records.p2m`)
      recording + parallel cold-device `MTL4Archive` prewarming (`port-runner --prewarm-shaders`,
      `PORT_PREWARM_SHADERS`) with a native macOS AppKit progress window on interactive cold
      launches, wired guaranteed `ExitProcess` + D3D11/D3D12 `Present` PSO archive flushing, kept
      `.app/Contents/Resources/` read-only via `*_FALLBACK*` env vars, normalized staged framework
      Mach-O `minos` (`<= 15.x`) + `@rpath` install names, added curated `games/<id>/game.toml`
      adoption + `--max-fns 0` + `--bundle-shader-cache` in `steam-convert`, and added
      `cargo opsdb -- bundle-verify <PATH.app>` (15/15 PASS on packaged `out/apps/Overwatch.app`
      and signed `ColdTest.app` under `env_clear()` + read-only `Contents/Resources`).

---

## Win32, Input & Audio (`crates/win32-rt`, `crates/wwise-audio`)

- [x] **DirectInput gamepads (`crates/win32-rt/src/misc/dinput8.rs`):** Exposed `GameController`
      pads through `IDirectInput8W`/`A::EnumDevices` & `CreateDevice` with full 32-slot
      `IDirectInputDevice8W`/`A` (`DIJOYSTATE`/`DIJOYSTATE2`/custom formats, `DIPROP_*`, 8-way POV,
      deadzone/range, preserving Wwise rumble fallback; 3/3 tests).
- [x] **Keyboard & display edge cases (`crates/win32-rt/src/{user32,macos}/`):** Added macOS
      `HIToolbox` `'uchr'` / `UCKeyTranslate` non-US keyboard layout support in `ToUnicode`/`Ex`
      (`macos/keyboard.rs`), per-monitor DPI scaling on mixed-scale displays (`macos/display.rs`,
      `user32/{monitor,window}.rs`), and real `NSPasteboard` clipboard text transfer with
      test-isolated fallback (`macos/appkit.rs`, `user32/misc.rs`).
- [x] **WASAPI capture endpoint (`crates/win32-rt/src/audio/`):** Added honest `eCapture`/`eAll`
      endpoint enumeration (read-only CoreAudio input metadata query with zero TCC mic prompt),
      `IAudioClient` capture stream init/start/stop, and `IAudioCaptureClient` delivering paced
      silent PCM packets (`AUDCLNT_BUFFERFLAGS_SILENT` / `AUDCLNT_S_BUFFER_EMPTY`).
- [x] **DualSense USB audio-haptics pairing:** Wired deterministic `dualsense_usb_container_id`
      between `setupapi.rs` (`SPDRP_BASE_CONTAINERID` + `SetupDiGetDevicePropertyW(DEVPKEY_Device_ContainerId)`)
      and `audio/{mmdevapi,propsys}.rs` (`PKEY_Device_ContainerId` `VT_CLSID`, verified end-to-end
      through `StringFromGUID2`).
- [x] **`wwise-audio` A3 parity polish:** Implemented exact decompiled `CAkBus::IsMixingBus`
      (`sub_1415ca5a0`) pass-through folding in `params.rs` + `render.rs` (eliminating the static
      `1 / FastPow10(0)^2` offset on filtered bus `1134869537`), fixed sample-continuous fade-in →
      fade-out envelope handoff, and reverse-engineered Wwise's `00 04 02 00` / `00 04 00 00`
      preprocessed Convolution Reverb IR format (`uBlockLength=1024`, BFloat16 / Float32 packed
      real-FFT partitioned spectra) with a real overlap-add FFT `PartitionedConvolver`
      (`crates/wwise-audio/src/engine/convolver.rs`, **88/88** lib + **4/4** integration tests).
- [x] **Eliminate parallel audio test deadlock:** Fixed CoreAudio/HAL lock-order inversions in
      `audio/{audioclient,backend,akrec,devnotify}.rs`, avoided `NorthlightVfs` scan in unit-test
      `native_engine()`, and added RAII `audio_test_lock()` (verified 3 consecutive default-parallel
      runs without `--test-threads=1`: 28/28 default + 36/36 `--features wwise-bridge`).

---

## Factory CLI & Packaging (`crates/port-factory/`)

- [x] **Interactive Steam Launcher & One-Command CLI (`crates/port-factory/src/{launcher,steam_library}.rs`):**
      Added `steam-setup` (auto-installs official native Apple Silicon `SteamRE/DepotDownloader`),
      enhanced `steam-download` (`--qr` Steam Mobile QR sign-in, `--validate` in-place repair,
      title/AppID resolution, `~/Games/PortFactory/Steam` discovery), added one-command
      `steam-port` (download → `.app` → `bundle-verify` → optional SSD export), `bundles` +
      `bundle-export`, and the zero-dependency local interactive Launcher UI (`cargo opsdb --
      launcher` + double-clickable `out/apps/Port Factory.app` via `cargo opsdb -- launcher-app`).
- [x] **Self-contained `.app` frameworks (`steam_library.rs`):** `steam-convert --format app` now
      copies `libmetalirconverter.dylib` (and optional `libdxilconv.dylib` / `libMoltenVK.dylib`)
      into `Contents/Frameworks/` and generates a native launcher that resolves them with an empty
      environment (verified by compiling and executing the generated launcher under `env_clear()`;
      cold Finder launch of a full converted game remains tracked in [the cold-launch milestone](WHATS_NEXT.md#package-and-relaunch)).
- [x] **Verify `cargo opsdb -- doctor`:** Checks five generated TSVs, catalog source references, the zero-new-stub policy and the
      [published totals](STATUS.md#catalog-totals). Exits nonzero on drift (`crates/port-factory/src/doctor.rs`
      with unit tests).
- [x] **Overwatch 2 bring-up audit (`games/overwatch/game.toml`):** Ran `win32-audit`
      (1,155/1,155 = 100%), `anticheat-audit` (no kernel anti-cheat; runtime unpacker),
      `--prewarm-aot` (4/4 modules, 339,480 blocks in 0.91s), and live `port-runner` boot (now
      boots without `--trace`, unpacks all 681 imports, passes Authenticode/Steamworks/Winsock2/
      MMDevAPI/Prism GPU + Dx11/Dx12 device init, and reads `data/.build.config` + 106 MB
      `data.000.000` BLTE chunks via async IOCP when `.build.config` matches the installed
      `data/data.*` archives).

---

## Alan Wake 2 Target (`games/alan-wake-2/`)

- [x] Fix the Cauldron Lake camera/terrain issue in Return 0: The Cult (`cauldron_hot_za13.rs`
      `unknown_14212c3e0` `0x58`-byte AABB+Transform layout fix + default shipping PhysX DLLs in
      `games/alan-wake-2/game.toml` & `aw2_hooks.rs`; verified with `--diff-test` and live
      `AW2_CAMERA_TRACE=1` forest walk).
- [ ] Complete a clean native ARM64 playthrough of Return 0 and Chapter 1 (verifying MetalFX,
      audio, gamepad/mouse input, and `~/Translator` save/load).
- [ ] Keep `cargo check -p aw2-game --no-default-features` and `cargo check -p port-loader
      --features aw2 --bin aw2-loader` green whenever shared signatures in `x86-runtime` or
      `win32-rt` change.
- [ ] Update `db/progress.tsv` and `db/throws.tsv` via `tools/fndb.sh sync` whenever hand-decompiled
      functions in `aw2-game` change.
