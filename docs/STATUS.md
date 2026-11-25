# Project Status

This is an honest snapshot of what works today, what is tested on real Apple Silicon hardware, and
what still has gaps.

Related docs: [End Goal](END_GOAL.md) · [What's Needed](WHATS_NEEDED.md) ·
[What's Next](WHATS_NEXT.md) · [ToDos](TODOS.md) · [Roadmap](ROADMAP.md)

---

## 1. Universal Port Factory (`crates/`)

### Catalog totals

<!-- opsdb:catalog -->

- x86-64 operations — 54 families (1 FAST_INTERP, 5 GENERIC_INTERP, 47 JIT_ARM64, 1 TRAP_BOUNDARY), 1017 distinct mnemonics, 4979 cataloged vectors.
- Win32 — 4340 exports across 90 DLLs (4250 real, 40 partial, 21 stub, 29 failing).
- Stub inventory — 155 sites across 20 DLLs.
- A64 emitter — 1100 test words checked against 4743 encoding forms.

<!-- /opsdb:catalog -->

`cargo opsdb -- doctor` verifies these counts. The tiers and vector counts are catalog
metadata. For example, the AVX2 gather row still says `JIT_ARM64`, but its eight gather
mnemonics have no lowering or generic handler. Its source link points to the interpreter's
unimplemented-instruction path. Win32 `real` means the source scanner found no stub marker
in the implementation. Individual API behavior still needs a guest test.

### Diagnostics delivered in #11 and #12

- The manifest knob registry feeds `[knobs]`, `--knobs` and the AOT cache key. Presentation
  hooks across graphics APIs drive frame limits and heartbeats. The watchdog can capture a
  blocked thread before its first present.
- `audit-game` combines import, shader, managed-runtime and instruction audits. Bounded
  guest runs, schema-v1 JSON, boot reports and `report-diff` make bring-up runs comparable.
  The Zig guest corpus exercises the JIT and prewarmed AOT paths. `bench` records chip-specific
  baselines. The runtime also handles unaligned LOCK operations, reports unsupported instructions
  and syscalls, identifies unknown COM calls and records stub usage. Doctor checks new stub
  sites against the inventory.
- Fatal reports write both `crash-<pid>-<time>.txt` and `.json` under `out/crash`
  (`PORT_CRASH_DIR` overrides it). Guest crash and stall backtraces unwind PE `.pdata`.
  An incomplete walk is labeled and can fall back to a stack scan. The Zig fixtures in
  `pre_present_hang_e2e` cover a blocked guest, a pumping message loop and crash reporting.

### Subsystems

The universal runner builds independently of the optional Alan Wake 2 target. The game and
hardware observations below describe earlier runs. The current local verification is recorded
under [Baseline test health](#3-baseline-test-health).

| Area | Status | What works today | Known gaps / caveats |
|---|---|---|---|
| **Universal runner (`port-loader` / `port-runner`)** | Working | Loads any PE32+ `.exe`/`.dll`, reads declarative `game.toml` manifests (`games/overwatch/game.toml`, `games/alan-wake-2/game.toml`), supports `--prewarm-aot`, `--ops-report`, `--list-exports`, and `--call-export`. Emulates `KUSER_SHARED_DATA` reads, handles isolated guest page-fault/C-SEH traps (guarded against non-guest threads with null TEB), emits canonical `ntdll.dll` syscall stubs, materializes builtin PEs in `C:\Windows\System32\`, and fails dynamic `LoadLibrary` cleanly with `ERROR_MOD_NOT_FOUND` on missing dependency DLLs. Live-verified on `zlib1.dll`, `CrashMailer_64.exe` (`rc=0`), real Windows x86-64 D3D11 and Vulkan `.exe` rendering/resize tests (`d3d11_e2e`, `vulkan_e2e`), and a shipping Steam **Overwatch 2** bundle (`out/apps/Overwatch.app`, 15/15 `bundle-verify` PASS) rendering **420+ real 1080p Metal 4 frames** through the Blizzard Entertainment intro splash all the way to the interactive **Overwatch 2 Login Screen** (`2.25.1.0`) with live 48 kHz CoreAudio, Geometry Shader emulation, `PORT_AOT=2` live AOT build/install (221k blocks, 0 faults), and live Schannel TLS 1.2 HTTPS (`telemetry-in.battle.net`, `overwatch.ingest.gdp.blizzard.com`). | Overwatch 2's `Overwatch_loader.dll` (Byfron anti-tamper watchdog thread) still raises its periodic `RaiseException(0xc00000e5, [0xe1d04f02, 0x602])` integrity termination at ~34s; online Battle.net login also requires an authenticated Steam/Battle.net session ticket. |
| **x86-64 → ARM64 execution (`x86-runtime`)** | Working | `iced-x86` interpreter + ARM64/NEON block JIT + relocatable `.aot` disk cache (safely skips runtime-modified / `VirtualProtect`-reprotected PE images during background builds and loads) + **parallel multi-threaded offline prewarming** (`--prewarm-aot`, `PORT_AOT_THREADS`, deterministic bit-for-bit output across 1–8 threads, 4.63x on 6,400-block benchmark; prewarms all 4 Overwatch PE modules — 339,480 blocks / 57.4 MiB — in 0.91s) + **RIP-relative switch/jump-table CFG discovery** + multi-module `PORT_AOT_VERIFY=1` shifted-base integration test + **hardware ARMv8 `FEAT_SHA1`/`FEAT_SHA256` JIT for all 7 Intel SHA-NI ops** (NIST FIPS 180-4 verified, CPUID leaf 7 bit 29 enabled) + `0x67` 32-bit addr-size override, `movnti`, `cwd`, ring-3 `sidt`/`sgdt`/`sldt`/`str`/`smsw`, AVX2 256-bit reg/mem shifts, expanded x87 FPU (`fdivr`/`ffreep`/`fistp`/`fstp`/`fld m80`), and MSVC FH4/SEH unwinding (`eh_rt`). `0` unimplemented opcodes in live 420-frame Overwatch 2 runs. | Selected x87 instructions run in the interpreter. Unsupported legacy and vector instructions can reach the unimplemented-instruction handler. Some games use anti-cheat/DRM or self-modifying JIT code that needs case-by-case handling. |
| **Win32 OS & middleware (`win32-rt`)** | Working | See the [export counts](#catalog-totals). Zero `objc2-metal` and zero `aw2-data` in default builds. Dynamic `AppConfig`, `DllRegistry`, `ComRegistry`, `~/Translator` virtual `C:\` filesystem with NT device-path round-tripping (`NtQueryVirtualMemory(MemoryMappedFilenameInformation)`, `QueryDosDevice*`, `GetLogicalDriveStrings*`, `K32GetMappedFileName*`), Toolhelp32 (`Thread32First`/`Next`, `Module32First`/`Next`), `K32EnumProcessModules`, DbgHelp (`EnumerateLoadedModules64`/`W64`), real Mach `HOST_CPU_LOAD_INFO` `GetSystemTimes`, seeded `HKLM` Windows 10 22H2 OS/BIOS/CPU/MachineGuid registry keys, `GetThreadContext`/`SetThreadContext`, real CCD display configuration (`GetDisplayConfigBufferSizes`/`QueryDisplayConfig`/`DisplayConfigGetDeviceInfo`), real POSIX-backed Winsock2 (`ws2_32`, including `WSAEventSelect`/`WSAEnumNetworkEvents`, `WSADuplicateSocket*` + `WSASocket*` dup-token roundtrip, and `WSAIoctl(SIO_IDEAL_SEND_BACKLOG_QUERY)`), real hardware-accelerated CNG AES (ECB/CBC/CFB/CTR/GCM via CommonCrypto) + `BCryptDeriveKeyPBKDF2` (`bcrypt.dll`), real client-side **Schannel TLS 1.2 / 1.3** (`secur32.dll` / `sspicli.dll` / `security.dll` backed by statically linked `rustls` + embedded `webpki-roots`, live-verified against `telemetry-in.battle.net`), CryptoAPI peer certificate chains / memory stores / chain engines / SSL policy verification / `CryptEncodeObjectEx(X509_PUBLIC_KEY_INFO)` / `CryptQueryObject` + `CryptMsgGetParam` + `CertFindCertificateInStore` (`crypt32.dll`), cryptographic PE Authenticode verification (`WinVerifyTrust` / `bcrypt`), cooperative Win32 fibers, async IOCP `ReadFile`/`WriteFile`, C-SEH + `Wow64PrepareForException` trap dispatch, serialized 16 KiB / 4 KiB deferred `VirtualProtect` + cross-core unlock race recovery, DirectInput 8 gamepads, and **dynamic native Apple Silicon Steamworks IPC** (`steam_api64.dll` dynamically `dlopen`s Valve's arm64 `steamclient.dylib` when macOS Steam is running for live `ISteamUser`/`ISteamApps`/`ISteamFriends`/`ISteamUtils` + Win64↔ARM64 `CSteamID` & callback/CallResult bridging, with clean offline fallback). The Oodle and PhysX binaries are supplied by the game. The zstd builtin uses a native decoder. See the stub inventory for the remaining conditional and failing APIs. | New games will inevitably import Win32/COM/CRT APIs we haven't hit yet; check with `cargo opsdb -- win32-audit`. |
| **Direct3D 12 + MetalFX (`d3d12-metal`, `d3d12-replay`)** | Working | Pure Metal 4 backend: `ID3D12Device`..`Device10`, `GraphicsCommandList`..`List7`, DXR 1.1 (BLAS/TLAS build/refit/compact, inline `RayQuery`, `StateObject`/`DispatchRays`), Mesh Shaders, **Metal 4 Geometry Shader emulation** (`MTL4MeshRenderPipelineDescriptor` object+mesh+fragment pipeline + `draw_geometry_emulated`), GPU ICB `ExecuteIndirect`, UAV clear/counter tracking, `generate_mips` / `write_buffer_u32` primitives, compute-swizzled `RGBA8 → BGRA8` swapchain presentation, safe swapchain teardown, binary trace & offline replay. Native **`MTL4FXSpatialScaler`** (live-verified inside Alan Wake 2: `1440x810→1920x1080` & `960x540→1280x720`, HDR `RG11B10Float`, 3.83×–4.65× sharper center-3D than bilinear, `10.67–11.31 ms/f` / `88.4–93.7 FPS`) AND **`MTL4FXTemporalScaler`** (multi-frame jitter + depth + motion vectors + reactive mask + exposure + DRS, `PORT_METALFX=0` fallback), verified with `MTL_DEBUG_LAYER=1` (0 validation errors). | Barrier/clear heuristics remain conservative by default so they stay sound across engines. |
| **Direct3D 11 (`win32-rt/src/d3d11`)** | Working | Real D3D11.0 / 11.1 COM runtime (`ID3D11Device1`, `ID3D11DeviceContext1`, immediate + deferred contexts, `IDXGIDevice3`, `IDXGISurface2` including `Map`/`Unmap`, `IDXGIResource1` + `OpenSharedResource*` with cross-device coherence, `IDXGISwapChain4`, standalone `CreateDXGIFactory*` D3D11 swapchain routing, buffers/textures/views, `Map`/`Unmap`/`UpdateSubresource`/`CopyResource`, `GenerateMips`, **real `ID3D11GeometryShader` on Metal 4**, **pooled 4096-slot `MTL4CounterHeap` timestamp queries**, `DrawInstancedIndirect`, `DrawIndexedInstancedIndirect`, `DispatchIndirect`, `CopyStructureCount` with GPU UAV counters), backed by `d3d12-metal` + `dxil-metal`. Live-verified rendering **Overwatch 2** at 1080p through splash to the Login screen (172+ compiled Metal 4 PSOs, 0 shader errors, 0 `MTLCounterHeap` errors), plus unit/GPU coverage and two live Windows x86-64 PE `.exe` E2E tests (`d3d11_e2e`: headless + live `NSWindow`, SM 5.0 VS/PS + depth + compute + `ResizeBuffers` + staging readback) passing under `MTL_DEBUG_LAYER=1` (0 validation errors). Opt out with `PORT_D3D11=0`. | Hull/domain tessellation and stream-out geometry shaders return `E_NOTIMPL` honestly. |
| **Vulkan (`win32-rt/src/misc/vulkan_1.rs`)** | Working | Dynamic `libMoltenVK.dylib` / `libvulkan.1.dylib` bridge (checks `PORT_VULKAN_DYLIB`, bundled `Contents/Frameworks/`, Homebrew, and `$VULKAN_SDK`). Rewrites `VK_KHR_win32_surface` → `VK_EXT_metal_surface` (`CAMetalLayer`, including headless fallback and `_dispatch_main_q` resize sync) + `VK_KHR_portability_enumeration`, injects `VK_KHR_portability_subset` on `vkCreateDevice`, forwards Vulkan 1.0–1.3 core & KHR/EXT entry points (including dynamic rendering, synchronization2, timeline semaphores, buffer device address, copy_commands2, maintenance4), adapts Win64→AAPCS64 by-value float args, and sanitizes `VkAllocationCallbacks` across dynamic procs. **Live-verified** on Apple M3 Ultra (`Vulkan 1.3.323`): unit/driver coverage and two real Windows x86-64 PE `.exe` E2E tests (`vulkan_e2e`: headless + live `NSWindow`, SPIR-V draw/present, `256x192`→`320x200` resize + `oldSwapchain` recreation, exact pixel readback). Returns honest `VK_ERROR_INCOMPATIBLE_DRIVER` if MoltenVK is absent. | Needs stress testing on a large shipping Windows Vulkan game title. |
| **Shaders (`dxil-metal`, `d3d12-metal/pso_manifest`)** | Working | DXIL SM 6.0–6.6 (plus SM 6.8/6.9 bitcode lowering) via Metal Shader Converter 4.0.1, FXC SM 4/5 via `libdxilconv.dylib`, SPIR-V & WGSL via `naga` → MSL. Out-of-process `dxil-metald` worker with macOS `kqueue` parent-exit watchdog + `atexit`/`Drop` cleanup (parent-death and worker-reaping fixtures). Tier-1 `.metallib` disk cache uses immutable `codesign`-invariant Mach-O `LC_UUID` converter fingerprinting (`irX.Y.Z-uuid-<LC_UUID>`, portable across re-signing, copies, and Macs, with legacy `irX.Y.Z-*` prefix fallback + read-only `PORT_SHADER_CACHE_FALLBACK`). Tier-2 `MTL4Archive` PSO cache isolates per `(MTLDevice.name, macOS build)`, flushes periodically on D3D12 & D3D11 `Present` AND on `ExitProcess`, records GPU/OS-independent `pso_manifest/records.p2m` recipes, and auto-precompiles in parallel on cold devices/macOS updates (`--prewarm-shaders`, `PORT_PREWARM_SHADERS`) with a native macOS AppKit progress window when launched interactively. | Compiling `naga`-generated MSL offline to `.metallib` requires Xcode's Metal Toolchain (`xcrun -sdk macosx metal`); returns honest `Error::Load` when absent. |
| **Windowing & input (`win32-rt/{user32,macos}`)** | Working | Cocoa main-thread architecture (`NSWindow` + `CAMetalLayer`), Win32 message queues, keyboard/mouse/raw input (`GCMouse` + `NSEvent` fallback), non-US macOS `'uchr'`/`UCKeyTranslate` `ToUnicode`, per-monitor mixed-DPI scaling (`GetDpiForMonitor`/`Window`, `WM_DPICHANGED`), `NSPasteboard` clipboard text transfer, relative cursor mode, `GameController` XInput + CoreHaptics rumble + DirectInput 8 gamepads, `IOHIDManager` DualSense/DS4 + SetupDi overlapped HID with `SPDRP_BASE_CONTAINERID` pairing. Headless mode via `PORT_HEADLESS=1` / `AW2_HEADLESS=1`. | Live interactive mixed-DPI drag testing across physical external displays stays worth validating as hardware setups vary. |
| **Audio (`win32-rt/src/audio`, `wwise-audio`)** | Working | Shared-mode WASAPI render + capture (`eCapture`/`eAll` + `IAudioCaptureClient`) on CoreAudio (48 kHz float32, device hotplug/invalidation, Mach RT scheduling, `mmdevapi` `ActivateAudioInterfaceAsync` + `IAudioSessionManager`/`IAudioSessionManager2`/`IAudioSessionControl2`/`ISimpleAudioVolume` + `avrt`, DualSense `PKEY_Device_ContainerId` pairing, deadlock-free parallel tests). `wwise-audio` is decoupled via `AudioDataSource` (`northlight` feature optional), implements Wwise 2022.1 `.bnk`/`.wem` parsing, Vorbis decoding, 3D spatialization, music engine, 21 source/effect DSP plugins, exact `CAkBus::IsMixingBus` pass-through bus folding (resolving bus `1134869537`), continuous fade-in→fade-out handoff, and a real partitioned overlap-add FFT convolver decoding Wwise `00 04 02 00` BFloat16 & `00 04 00 00` Float32 Convolution Reverb IRs (**88/88** lib + **4/4** integration tests). | Real-world virtual-voice threshold edge cases and live multi-game Wwise bank coverage remain ongoing validation targets. |
| **Factory CLI (`port-factory`)** | Working | `cargo opsdb --` provides ops DB sync/query, Win32 DB sync/audit, `doctor` checks five generated TSVs, source references and the published catalog counts, PE scanner, `aot-prewarm`, anti-cheat/.NET/shader/app audits, `scaffold-op`, `scaffold-dll`, prototype Rust `aot`/`aot-crate`, VFS resolver, native ARM64 `DepotDownloader` management (`steam-setup`, `steam-scan`, `steam-download` with `--qr`/`--validate`, one-command `steam-port`), interactive local Launcher UI (`launcher` + double-clickable `Port Factory.app` via `launcher-app`), SSD-portable `.app` bundler (`steam-convert`: release `port-runner` + `dxil-metald` + curated `games/<id>/game.toml` + prewarmed `AotCache` + `minos`-normalized `Contents/Frameworks/` dylibs + portable Tier-1 `ShaderCache` excluding host `*.mtl4a`; keeps `Contents/Resources/` strictly read-only via `*_FALLBACK*` while routing mutable caches to `$HOME`), and `bundles` / `bundle-verify` / `bundle-export` (15/15 PASS on `out/apps/Overwatch.app`). | Prototype Rust source transpiler (`aot`/`aot-crate`) targets small C-leaf DLLs (like `zlib1.dll`); large games use `x86-runtime`'s ARM64 block JIT + `.aot` cache (`--max-fns 0`). |

---

## 2. Alan Wake 2 Target (`games/alan-wake-2/`)

Alan Wake 2 is still the most thoroughly tested real-world game on this stack and is our main
regression benchmark for the universal crates.

- **Build**: `cargo loader-build` (`port-loader --features aw2 --bin aw2-loader`), linking
  `aw2-game`, `aw2-data`, `win32-rt/wwise-bridge`, and AW2-only hooks (`aw2_hooks.rs`).
- **Shaders**: 5,052 / 5,052 shaders across all 153 `.binrfx` bundles and PE binaries convert with
  0 failures; 3,311 / 3,311 active Apple Silicon shader exports compile to Metal 4 PSOs with 0
  failures (including all 24 `rt_*.binrfx` DXR bundles).
- **Lifted coverage**: 244,796 / 244,796 `.pdata` functions in `AlanWake2.exe` plus 8 guest DLLs
  registered in `aw2-game`, with 39,155 differential tests passing on ARM64.
- **Rendering & upscaling**: Main Menu and in-game `cauldronlake` 3D rendering run on Metal 4
  with `ExecuteIndirect` embedded index buffers and FSR/upscaler dispatch routed into native
  MetalFX (`MTL4FXSpatialScaler`) with bilinear compute fallback.
- **Audio**: Both original Wwise over CoreAudio WASAPI (verified sample-exact against Wwise
  `StartOutputCapture`) and `AW2_AUDIO=native` (155 hooked `AK::` exports driving `wwise-audio`
  directly into CoreAudio) are implemented.
- **Cauldron Lake movement & physics**: Fixed `unknown_14212c3e0` (`cauldron_hot_za13.rs`, `0x58`-byte AABB+Transform layout passed to `CollisionCache::fillCache`, verified 10/10 with `--diff-test`) and defaulted AW2 to real shipping PhysX 3.4 DLLs (`games/alan-wake-2/game.toml` & `aw2_hooks.rs`); verified live in Return 0: The Cult holding `W` across frames `20800..23000` (extracts `28..677` terrain triangles, walks Nightingale 44m uphill along the trail with `on_g=1` and zero cyan void).
- **Open gameplay work**: Full end-to-end campaign verification of Return 0: The Cult and Chapter 1 progression, plus long-run stability under native audio + MetalFX (`PORT_METALFX=1`).

---

## 3. Baseline Test Health

`RDTSC` and `RDTSCP` expose a shared 1 GHz guest clock on Apple Silicon. Translated blocks
read 1 GHz counters directly and convert 24 MHz counters without division. JIT, cached and
static AOT, and generated Rust use the self-synchronizing counter for ordered reads on
ECV-capable hosts. Differential verification can replay a fixed timestamp in all tiers.
[Guest timestamp semantics](DIAGNOSTICS.md#guest-timestamp-counter)

The A64 emitter-match test re-emits the registered cases and checks each word against the
committed matches and Arm form. The optional assembler check also compares operands with
Clang. [Catalog and reproduction commands](DIAGNOSTICS.md#check-the-a64-words-emitted-by-the-jit)

Local baseline on 2026-11-24, Apple M5 Ultra, macOS 27.3. This run includes the middleware changes from #14. Cargo commands below used
`--target aarch64-apple-darwin` and `-- --test-threads=1` for tests. The factory alias
already selects that target. Counts are harness results. Some fixtures return early when
local inputs are missing.

| Suite | Result on this host |
|---|---|
| `cargo factory-test` with `PORT_A64_ENCODINGS` set to the checked-in catalog | 523 passed, 3 ignored. dxil-metal 20, pe-image 5, port-factory 64, port-loader 41, port-manifest 17, wwise-audio 88, x86-runtime 288. Both A64 emitter checks ran. |
| `cargo test -p x86-runtime --test aot_multi_module_verify` | 1 passed, including the shifted-base reload. |
| `cargo test -p port-loader --test pre_present_hang_e2e` | 4 passed on repeat with Zig, including crash JSON and a multi-frame guest stack. The first run had 3 passes and one false stall during builtin loading in the message-pump test. |
| `cargo test -p port-factory --test bundle_cold_launch` | 2 passed with the local runner built. |
| `cargo test -p dxil-metal` | 58 harness passes on repeat. Game/converter-dependent cases self-skipped. The first run failed the daemon SIGTERM/SIGINT test. |
| `MTL_DEBUG_LAYER=1 cargo test -p d3d12-metal --lib` | 194 passed, 6 failed loading the native Metal Shader Converter. |
| `cargo test -p win32-rt --lib` filtered to `d3d11 misc::dinput8 user32 macos misc::vulkan_1` | 93 harness passes with Metal validation and live MoltenVK enabled. |
| Same library, `misc::secur32 misc::crypt32 misc::wintrust` | 17 passed. |
| Same library, `audio misc::setupapi` | Default 59 passed, 1 ignored. With `--features wwise-bridge`, 67 passed, 1 ignored. The physical hotplug test self-skipped in each run. |
| `cargo test -p port-loader --test vulkan_e2e` with `PORT_TEST_MOLTENVK=1` | 2 passed with Zig and an arm64 MoltenVK. Both headless and windowed guests presented, resized and returned the expected pixels. |
| `cargo test -p port-loader --test d3d11_e2e` | Both guests self-skipped at the libdxilconv prerequisite. This host also lacks the native Metal Shader Converter. |
| `cargo test -p wwise-audio` | 88 library and 4 integration harness passes, 1 ignored. The optional Vorbis codebook header was absent. |
| `cargo test -p wwise-audio --features northlight --test real_data` | All 8 selected cases self-skipped because the game install was unavailable. The full-bank scan remains ignored. |

`cargo runner-build`, `cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin`,
`cargo opsdb -- doctor` and the repository hygiene audit passed. No commercial game was booted for
this baseline.
