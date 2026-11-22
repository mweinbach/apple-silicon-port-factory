# iOS Reference Host (`ios/PortHost`)

Reference UIKit / `CAMetalLayer` wrapper that embeds the universal Apple runtime stack
(`libport_loader.a`) into an iOS / iPadOS app bundle.

## Architecture

- **Main thread (`UIApplicationMain` / `UIScene`)**: owned by UIKit (`PortHostApp.swift`,
  `MetalSurfaceView.swift`, `TouchOverlayView.swift`, `KeyboardBridge.swift`, `PhoneHaptics.swift`).
  Attaches its fullscreen `CAMetalLayer` via `port_ios_attach_surface`, forwards resizes/rotations,
  multi-touch virtual XInput gamepad controls, direct touch-as-mouse input (including 2-finger
  right-click), native hardware & software keyboard detection/input, game-to-phone CoreHaptics
  rumble passthrough, and `CADisplayLink` ticks for `GameController.framework`.
- **Game-to-Phone Haptics Passthrough (`PhoneHaptics.swift`)**:
  - Routes Windows game rumble (`XInputSetState` left low-frequency + right high-frequency motors,
    including Wwise `AkXInputRumbleSink`) directly to the iPhone's built-in Taptic Engine via
    `CoreHaptics` (`CHHapticEngine` with `playsHapticsOnly = true` and a looping continuous
    `CHHapticAdvancedPatternPlayer` updated in real time with dynamic intensity & sharpness).
  - Automatically activates when playing with the touch overlay or a non-rumble MFi/Backbone
    controller, yields to a physical DualSense/Xbox controller's native `GCDeviceHaptics` motors
    when connected, stops cleanly on backgrounding/`XInputEnable(0)`, and exposes a live HUD mute
    toggle on iPhones with hardware Taptic Engine support.
- **Touch-Overlay Controls (`TouchOverlayView.swift`)**:
  - Full multi-touch translucent XInput overlay (dual analog sticks, 4-way D-pad, ABXY diamond,
    LB/LT/RB/RT shoulders & triggers, L3/R3, View/Menu) pushing live state directly into
    `win32-rt`'s `xinput1_4` virtual slot 0 via `port_ios_gamepad_state`.
  - Compact top-center HUD pill (safe-area aware around Dynamic Island) with mode cycle
    (`Pad` → `Touch` → `Hide`), `⌨` native keyboard toggle, and live peripheral badge
    (`XINPUT`, `CTRL`, `HW KB`, `SOFT KB`, `MOUSE`).
  - Automatically fades out and releases virtual inputs whenever the native iOS software keyboard
    opens or a real physical `GCController` connects (while remaining one tap away to override).
- **Native Hardware & Software Keyboard Detection (`KeyboardBridge.swift`)**:
  - Detects physical keyboards via `GCKeyboard.coalesced` (`GCKeyboardDidConnect`/`Disconnect`),
    `UIPress` fallback (deduplicated so keys never double-fire), and iOS shortcut-bar frame height
    detection (`< 100 pt`), mapping 95+ USB HID usages to macOS/Win32 scan codes & VKs.
  - Summons the native iOS software keyboard via a `UIKeyInput` proxy (`Esc`, `Tab`, `Ctrl`, `Alt`,
    `Done` accessory toolbar), forwarding Unicode text via `port_ios_text_input` and reporting
    `port_ios_keyboard_state(hardware_connected, software_visible, software_height_px)`.
- **Background runtime thread (`port_ios_start_background`)**: boots the Windows PE loader,
  `win32-rt` (UIKit host surface, `RemoteIO` + `AVAudioSession`, `GameController` + `CoreHaptics`,
  `~/Documents/Translator` sandboxed `C:\`), `d3d12-metal` (Metal 4 + MetalFX), `dxil-metal`
  (`BackendKind::CacheOnly`), and `x86-runtime`.
- **Flat iOS `.app` bundle layout**:
  - `<App>.app/game.toml` — declarative game manifest
  - `<App>.app/Game/` — Windows game binaries & assets
  - `<App>.app/ShaderCache/ir4.0.1-.../` — Tier-1 Metal shaders precompiled on macOS via `dxil-metal prewarm`
  - `<App>.app/AotCache/` — optional `.aot` cache (used only if `MAP_JIT` is granted; see Milestone #2 below)

## Building `libport_loader.a`

```sh
# iOS device (ARM64)
IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-lib

# iOS Simulator (Apple Silicon Mac)
IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-sim-lib
```

Static library output:
- Device: `target/aarch64-apple-ios/release/libport_loader.a`
- Simulator: `target/aarch64-apple-ios-sim/release/libport_loader.a`

Link against these system libraries & frameworks in Xcode:
- Frameworks: `Metal`, `QuartzCore`, `UIKit`, `Foundation`, `CoreFoundation`,
  `AudioToolbox`, `AVFAudio`, `GameController`, `CoreHaptics`, `Security`
  (`MetalFX` is loaded dynamically via `dlopen` at runtime when present on device; not required for iOS Simulator)
- Libraries: `libz` (`-lz`)

Configure Xcode build settings:
- Objective-C Bridging Header: `ios/PortHost/PortHost-Bridging-Header.h`
- Code Signing Entitlements: `ios/PortHost/PortHost.entitlements`
- Library Search Paths: `target/aarch64-apple-ios/release` (or simulator path)
- Other Linker Flags: `-lport_loader`

## Static ARM64 AOT Objects (Milestone #2 — Zero `MAP_JIT`)

Non-jailbroken iOS apps cannot map writable+executable `MAP_JIT` pages. Compile each Windows
`.exe` and `.dll` offline on macOS into an Apple-signed ARM64 Mach-O relocatable object (`.aot.o`):

```sh
# 1. Precompile all mapped PEs directly to iOS ARM64 Mach-O `.aot.o` objects
cargo runner -- --pe /path/to/Game.exe --emit-static-o out/ios-aot --static-platform ios

# Or via port-factory (also accepts an existing v5 `.aot` file directly)
cargo opsdb -- aot-static-obj /path/to/Game.exe --out out/ios-aot --platform ios
```

Each emitted `<module>.aot.o` contains:
- Position-independent shared stubs + hot/cold translated ARM64 blocks in `(__TEXT,__text)` with
  all intra-module direct branches patched offline and host calls linked via `bl _port_static_aot_host_xfer`
- Sorted RVA-to-code entry table in `(__TEXT,__const)` and `StaticAotModuleDesc` in `(__DATA,__const)`
- An automatic `__DATA,__mod_init_func` initializer calling `port_static_aot_register_desc` before `main()`

Link the `.aot.o` files directly into the iOS executable (or archive them into `libgame_aot.a`
and pass `-Wl,-force_load,$(SRCROOT)/libgame_aot.a` in Xcode *Other Linker Flags*). When the PE
loader maps each PE image at runtime, it prefers the module's compiled base address and binds
its signed `__TEXT` code automatically with zero `MAP_JIT` pages.

## One-Command iOS `.app` / `.ipa` Bundler (Milestone #3)

Package any Windows x86-64 `.exe` or game directory into a self-contained iOS device `.ipa`
or iOS Simulator `.app` (with `libport_loader.a`, statically linked signed `.aot.o` code,
portable Tier-1 `ShaderCache/`, `game.toml`, `Info.plist`, and code signature) in one command:

```sh
# Build a signed iOS device .ipa archive (+ optional XcodeGen project.yml)
cargo opsdb -- ios-bundle /path/to/Game.exe \
  --target device \
  --out out/ios/MyGame.ipa \
  --bundle-shader-cache \
  --release \
  --xcode-project

# Build an Apple Silicon iOS Simulator .app bundle
cargo opsdb -- ios-bundle /path/to/Game.exe \
  --target sim \
  --out out/ios/MyGame.app

# Verify any .app or .ipa bundle (ARM64 iOS Mach-O, static AOT count, zero macOS leaks / .mtl4a bloat)
cargo opsdb -- ios-verify out/ios/MyGame.ipa
```

For real iOS device sideloading or TestFlight, pass `--sign "Apple Development: ..."` and
`--provisioning-profile /path/to/profile.mobileprovision` (or open the generated `.project.yml`
with XcodeGen / Xcode).

## Current Execution Status (Milestones #1–#3)

- **Milestone #1 (Complete)**: Every universal runtime crate (`win64-abi`, `pe-image`, `wwise-audio`,
  `dxil-metal`, `d3d12-metal`, `win32-rt`, `x86-runtime`, `port-loader`) cross-compiles cleanly to
  `aarch64-apple-ios` and `aarch64-apple-ios-sim` with zero `MAP_JIT` crashes, cache-only precompiled
  shaders, UIKit surface/touch/pasteboard, RemoteIO audio, and GameController+CoreHaptics support.
- **Milestone #2 (Complete)**: Offline v5 `.aot` -> ARM64 Mach-O `.o` static compiler
  (`x86_runtime::emu::static_macho`, `port-runner --emit-static-o` / `--aot-to-o`,
  `port-factory aot-static-obj`) plus zero-`MAP_JIT` `JitCache` static-AOT execution
  (`PORT_EXEC_MODE=static-aot` / locked-down iOS) running natively out of Apple-signed `__TEXT`.
- **Milestone #3 (Complete)**: One-command iOS `.app` / `.ipa` packager, verifier, and XcodeGen
  emitter (`port-factory ios-bundle` / `ios-verify`) bundling static `.aot.o` modules, flat
  `game.toml` + `Game/` + portable Tier-1 `ShaderCache/`, and code signing for both physical
  iOS devices and Apple Silicon iOS Simulators.
