# Physical iOS Device Deployment, `ios/PortHost` FFI & Storage Optimization Reference

## 1. Physical iPhone / iPad Build & Deploy Pipeline (`devicectl`)

Use this exact 4-step workflow when building and deploying updated Rust/AOT changes to a connected
iPhone or iPad (zero `MAP_JIT` pages):

```sh
# 1. Rebuild host universal runner (emits the static Mach-O .aot.o objects)
cargo runner-build

# 2. Build release aarch64-apple-ios static library (target-cpu=apple-a14 in .cargo/config.toml)
IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-lib
# Output: target/aarch64-apple-ios/release/libport_loader.a

# 3. Regenerate signed static ARM64 .aot.o objects for the game PE + mapped DLLs
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game \
  --emit-static-o /path/to/out_aot --static-platform ios
# (Or: cargo opsdb -- aot-static-obj /path/to/Game.exe --out /path/to/out_aot --platform ios)

# 4. Package + sign via `ios-bundle` (or XcodeGen + `xcodebuild`), then verify & install:
cargo opsdb -- ios-bundle /path/to/Game \
  --manifest games/<id>/game.toml \
  --target device --release \
  --bundle-shader-cache \
  --optimize-storage high \
  --sign "Apple Development: ..." \
  --provisioning-profile /path/to/profile.mobileprovision \
  --out out/ios/MyGame.ipa \
  --xcode-project

cargo opsdb -- ios-verify out/ios/MyGame.ipa

# Discover attached device UDID and install signed .app:
xcrun devicectl list devices
xcrun devicectl device install app --device <DEVICE_UDID> /path/to/DerivedData/Build/Products/Release-iphoneos/MyGame.app
```

### Critical Self-Contained Bundling Rules (macOS & iOS)
- **Bundle everything inside the `.app`:** Stage game `.exe`, DLLs, assets, `game.toml`, and
  portable Tier-1 `ShaderCache/` inside `<App>.app/` (flat `<App>.app/{game.toml,Game/,ShaderCache/}`
  on iOS; `<App>.app/Contents/Resources/{game.toml,Game/,AotCache/,ShaderCache/}` on macOS) — do
  not depend on ad-hoc `~/Documents/Game` mapping workarounds for packaged apps.
- **Keep bundle contents strictly read-only at runtime:** Route all writable saves/configs/Tier-2
  `*.mtl4a` caches into the user sandbox/home container (`~/Library/Application Support/Translator` on iOS,
  `~/Translator` on macOS) while passing bundled caches via `*_FALLBACK*`.
- **Never bundle host `*.mtl4a` archives** (GPU/OS-specific bloat; flagged by `bundle-verify` and
  `ios-verify`).
- **Multi-bank static `.aot.o` linking:** Modules > 112 MiB emit per-bank `x16` GOT veneers with
  zero `ARM64_RELOC_BRANCH26` relocations so `ld64` links 300–500+ MiB `__TEXT,__text` objects
  cleanly. If archiving into `libgame_aot.a`, link with `-Wl,-force_load,$(SRCROOT)/libgame_aot.a`
  so `__DATA,__mod_init_func` constructors (`port_static_aot_register_desc`) are never dead-stripped.

## 2. `ios/PortHost` Swift ↔ Rust C FFI Contract

Keep `ios/PortHost/PortHost-Bridging-Header.h` and `crates/port-loader/src/ios_host.rs` in strict
lockstep whenever changing signatures:

| Function | Direction / Purpose |
|---|---|
| `port_ios_start_background(const PortIosConfig *cfg)` | Starts Windows PE loader + runtime on background thread (`NULL` fields auto-discover bundled `game.toml` & `Game/`) |
| `port_ios_attach_surface` / `resize_surface` / `detach_surface` | Binds/updates UIKit `CAMetalLayer` + backing pixel size & scale |
| `port_ios_touch_event` / `mouse_button` / `scroll_wheel` | Direct touch-as-mouse (incl. 2-finger right-click) & scroll |
| `port_ios_key_event` / `text_input` / `keyboard_state` | Hardware `GCKeyboard` + `UIPress` (deduplicated) + `UIKeyInput` software keyboard |
| `port_ios_gamepad_state` / `port_ios_set_rumble_callback` | Multi-touch XInput overlay (slot 0) & game-to-Taptic-Engine CoreHaptics passthrough |
| `port_ios_tick` / `port_ios_memory_warning` / `port_ios_request_exit` | `CADisplayLink` tick, Jetsam memory warning hook, clean exit |
| `port_static_aot_register_desc` / `port_ios_register_static_aot` | Registers signed static ARM64 `.aot.o` descriptors before `main()` |

Required iOS frameworks: `Metal`, `QuartzCore`, `UIKit`, `Foundation`, `CoreFoundation`,
`AudioToolbox`, `AVFAudio`, `GameController`, `CoreHaptics`, `Security`, `-lz` (`MetalFX` is loaded
via `dlopen` at runtime so iOS Simulator builds never fail linking it).

## 3. Storage Optimizer & `KB2v` Hardware-HEVC Container (`storage_opt.rs`, `bink2-codec`)

- **Profiles (`--optimize-storage` / `optimize-assets --profile`):**
  - `lossless`: strip unused PC DLLs (always keeps static `.aot.o` DLLs, negative `builtin_overrides`,
    and `[storage].keep_dlls`), filter non-selected languages (`--keep-lang`), repack archive slack.
  - `high`: `lossless` + 1080p `KB2v` VideoToolbox HEVC + 2K lossless DDS mip cap (`--max-tex-dim`).
  - `compact`: `lossless` + 720p `KB2v` VideoToolbox HEVC + 1K lossless DDS mip cap (reduces iOS
    Jetsam VRAM & bundle size).
- **`KB2v` container (`crates/bink2-codec/src/hevc_container.rs`):** Preserves the exact 44-byte
  `Bk2FileHeader` (`magic = "KB2v"`, matching `"KB2*"` engine sniffers) followed by a 24-byte
  `Kb2vMeta` (`"HEVC"`, VPS/SPS/PPS NAL lengths), standard Bink2 32-bit frame offset table, and
  4-byte-aligned HVCC access units decoded at ~0% CPU via Apple `VideoToolbox`.
