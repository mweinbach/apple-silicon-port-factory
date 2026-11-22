# What's Next

This is the ordered action plan from right now forward. Each step builds directly on the one
before it so we discover real blockers early instead of polishing things in a vacuum.

Related docs: [End Goal](END_GOAL.md) · [What's Needed](WHATS_NEEDED.md) · [ToDos](TODOS.md) ·
[Status](STATUS.md) · [Roadmap](ROADMAP.md)

---

## Step 1 — Bring up our first non-AW2 Windows game on `port-runner` (First Shipping-Game Frames Done)

**Milestone reached:** **Overwatch 2** (`games/overwatch/game.toml`, Steam AppID `2357570`) now
prewarms all 4 PE modules (339,480 ARM64 blocks in 0.91s) and renders **420+ real 1080p Metal 4
frames** on `port-runner` (and inside `out/apps/Overwatch.app`, `15/15` `bundle-verify` PASS)
through the Blizzard splash screen all the way to the interactive **Overwatch 2 Login Screen**
(`2.25.1.0`) with zero game-specific Rust code in `crates/`, live 48 kHz CoreAudio, D3D11 Geometry
Shader emulation, pooled `MTL4CounterHeap` timestamps, live Schannel TLS 1.2 HTTPS, real CCD
display config (`1920x1080 @ 60/1`), `CryptQueryObject`, `GetSystemTimes`, `WSADuplicateSocket*`,
`IAudioSessionManager2`, and dynamic native arm64 `steamclient.dylib` IPC bridging.

*(Honest caveat: Blizzard's `Overwatch_loader.dll` Byfron anti-tamper watchdog still raises its
periodic `0xc00000e5` termination at ~30–35s, which we intentionally do not bypass.)*

**Next up in Step 1:** Pick a clean single-player / offline Steam title without anti-tamper or
kernel anti-cheat and take it from first frame to interactive gameplay using the same one-command
workflow:
```sh
cargo opsdb -- steam-port <AppID>
```
Work through any real blockers one at a time:
1. Missing Win32 API → implement honestly in `crates/win32-rt`, add a unit test, run `cargo opsdb
   -- win32-sync`.
2. Missing x86 instruction or JIT edge case → implement in `crates/x86-runtime` with a unit test.
3. Graphics/shader bug → fix in `d3d12-metal`, `win32-rt/src/d3d11`, `vulkan_1.rs`, or
   `dxil-metal` with a GPU test.

**Done when:** A non-Alan Wake 2 Windows game is playable past menus into gameplay on `port-runner`
and its `.app` bundle with zero game-specific Rust code in `crates/`.

---

## Step 2 — Live-test D3D11-on-Metal 4 and Vulkan→MoltenVK on real workloads (Baseline E2E Done)

Both backends now render real Windows x86-64 PE `.exe` binaries cleanly into a live `NSWindow` via
`port-runner` with verified swapchain presentation, live resize/recreation, and exact GPU pixel
readbacks:

- **Vulkan (`crates/port-loader/tests/vulkan_e2e.rs`):** 2/2 live E2E tests pass (headless + live
  `NSWindow`) against real `libMoltenVK.dylib` (`Apple M3 Ultra`, `Vulkan 1.3.323`), verifying
  `VK_KHR_win32_surface` → `CAMetalLayer`, `vkGetDeviceProcAddr`, SPIR-V draw/present, and
  `256x192` → `320x200` resize + `oldSwapchain` recreation.
- **D3D11 (`crates/port-loader/tests/d3d11_e2e.rs`):** 2/2 live E2E tests pass (headless + live
  `NSWindow`) under `MTL_DEBUG_LAYER=1` (0 validation errors), verifying SM 5.0 VS/PS + depth +
  compute, `RGBA8 → BGRA8` swizzled presentation, `ResizeBuffers`, and staging pixel readbacks.

Next: stress-test both backends on full shipping third-party D3D11 and Vulkan games as titles are
installed.

---

## Step 3 — Verify cold Finder launch of self-contained `.app` bundles (Automated E2E Done)

`cargo opsdb -- steam-convert <game> --format app` now copies `libmetalirconverter.dylib` (and
optional `libdxilconv.dylib` / `libMoltenVK.dylib`) into `Contents/Frameworks/` and resolves
frameworks, `Contents/Resources/AotCache`, `Contents/Resources/game.toml`, and
`Contents/MacOS/dxil-metald` from a native launcher under `env_clear()` (verified end-to-end in
`crates/port-factory/tests/bundle_cold_launch.rs` with `plutil -lint`, `codesign --verify --deep --strict`,
and real `dxil-metald` + `port-runner` execution under `env_clear()`). Next:
- Confirm double-click launch from Finder (`open out/apps/<Game>.app`) on a full shipping game
  bundle once a playable non-AW2 title is installed.

---

## Step 4 — Close the Alan Wake 2 Return 0 / Chapter 1 gameplay loop

Both the Cauldron Lake terrain-drop / cyan-view bug and live `MTL4FXSpatialScaler` verification
are **done**:
- Fixed `unknown_14212c3e0` (`cauldron_hot_za13.rs`, `0x58`-byte AABB+Transform) + defaulted
  shipping PhysX 3.4 DLLs; live `AW2_CAMERA_TRACE=1` walked Nightingale 44m uphill along the
  Cauldron Lake trail with `on_g=1` and zero cyan void.
- Live `PORT_METALFX=1` vs `PORT_METALFX=0` paired Cauldron Lake verification confirmed
  `MTL4FXSpatialScaler` (`ColorProcessingMode::Hdr`, `RG11B10Float`, `1440x810→1920x1080` and
  `960x540→1280x720`) delivers **3.83×–4.65×** center-3D sharpness vs bilinear, clean HDR tonemap
  handoff, and `10.67–11.31 ms/f` (`88.4–93.7 FPS`, `0.12–0.40 ms/f` faster than bilinear).

Next:
- Play Return 0: The Cult and Chapter 1 start-to-finish interactively (including windowed resize,
  native audio, gamepad/mouse input, and `~/Translator` save/load).

**Done when:** Return 0 and Chapter 1 are playable start-to-finish on native ARM64 without
regression in `cargo factory-test`.

---

## Step 5 — Scale JIT, `.aot`, and multi-engine coverage (Core Milestones Done)

Parallel offline `.aot` prewarming, RIP-relative jump-table CFG discovery, hardware ARMv8 SHA-NI
JIT, hot `GENERIC_INTERP` promotions (`0` non-trap interpreter ops in 577M-instr Overwatch trace),
and `MTL4FXTemporalScaler` prototyping are now in place. Next as more games are brought up:
- Validate Metal 4 barrier/residency behavior on a second D3D12 engine (non-Northlight) with
  `MTL_DEBUG_LAYER=1`.
- Verify offline SPIR-V / WGSL → `.metallib` once Xcode's Metal Toolchain is installed.
- Add declarative `games/<id>/game.toml` manifests as each new title reaches playable status.

---

Want to jump straight into a concrete task? Pick the top unchecked item from [docs/TODOS.md](TODOS.md).
