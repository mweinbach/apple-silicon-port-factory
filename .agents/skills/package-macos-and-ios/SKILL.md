---
name: package-macos-and-ios
description: Package, optimize storage for, sign, and verify self-contained macOS .app bundles, iOS/iPadOS .app or .ipa bundles (zero MAP_JIT static .aot.o), Steam game ports, external SSD exports, and ios/PortHost changes via port-factory. Use when running steam-port, steam-convert, bundle-verify, ios-bundle, ios-verify, aot-static-obj, optimize-assets, or editing ios/PortHost or bink2-codec.
---

# Package Portable macOS `.app` & iOS `.app`/`.ipa` Bundles

Never commit `.app/`, `.ipa`, game binaries/assets, signing certificates, or personal `/Users/`
or `/Volumes/` paths.

## 1. macOS Portable `.app` & Steam Workflow

End-to-end Steam download → `.app` → verify → external SSD export:

```sh
# One-time setup: install native ARM64 SteamRE/DepotDownloader
cargo opsdb -- steam-setup

# Scan local libraries or download/repair a Win64 Steam depot (Steam Mobile QR supported)
cargo opsdb -- steam-scan
cargo opsdb -- steam-download <AppID_or_Title> --qr --validate

# One-command full pipeline (or convert an existing game directory)
cargo opsdb -- steam-port <AppID_or_Title> --qr --copy-to /Volumes/MySSD
cargo opsdb -- steam-convert /path/to/Game --format app --copy-game --max-fns 0 \
  --bundle-shader-cache --out out/apps/MyGame.app

# Verify portability & export to SSD (also available via interactive UI: `cargo opsdb -- launcher`)
cargo opsdb -- bundles
cargo opsdb -- bundle-verify out/apps/MyGame.app
cargo opsdb -- bundle-export out/apps/MyGame.app /Volumes/MySSD
```

### Critical macOS Portability Invariants
- **`Contents/Resources/` MUST stay strictly read-only** so `codesign --verify --deep --strict`
  never breaks after launch. Bundles pass seed paths via `PORT_AOT_FALLBACK_DIR`,
  `PORT_SHADER_CACHE_FALLBACK`, and `PORT_PSO_MANIFEST_FALLBACK`, routing all writes to `$HOME`.
- **Never bundle host-specific `*.mtl4a` files** — ship portable Tier-1 `.aw2s` (`LC_UUID`-keyed)
  + `pso_manifest/records.p2m` so a new Mac prewarms its own GPU's `MTL4Archive` automatically.
- **Self-contained `Contents/Frameworks/`:** Stage `libmetalirconverter.dylib` (plus optional
  `libdxilconv.dylib` / `libMoltenVK.dylib`) with normalized `minos <= 15.x` and `@rpath` install
  names, alongside release `port-runner` and `dxil-metald` under `Contents/MacOS/`.
- **Cold Finder launch:** The native launcher must work under `env_clear()` with zero developer
  environment variables set.

*(For the bespoke Alan Wake 2 `.app` bundle, use `./scripts/package_app.sh [--copy-to ...]`.)*

## 2. iOS / iPadOS Zero-`MAP_JIT` `.app` / `.ipa` Workflow

Non-jailbroken iOS apps cannot map writable+executable `MAP_JIT` pages. All guest `.exe`/`.dll`
code compiles offline on macOS into Apple-signed ARM64 Mach-O relocatable `.aot.o` objects
(`crates/x86-runtime/src/emu/static_macho.rs`) linked straight into the signed `__TEXT` segment:

```sh
# Build universal static library (target-cpu=apple-a14 / apple-m1 in .cargo/config.toml)
IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-lib       # target/aarch64-apple-ios/release/libport_loader.a
IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-sim-lib   # target/aarch64-apple-ios-sim/release/libport_loader.a

# Emit standalone signed static ARM64 .aot.o objects from a PE or v5 .aot cache
cargo opsdb -- aot-static-obj /path/to/Game.exe --out out/ios-aot --platform ios
# (or: cargo runner -- --pe /path/to/Game.exe --emit-static-o out/ios-aot --static-platform ios)

# One-command iOS device .ipa (+ optional XcodeGen project.yml) with storage optimization
cargo opsdb -- ios-bundle /path/to/Game \
  --target device \
  --out out/ios/MyGame.ipa \
  --manifest games/<id>/game.toml \
  --release \
  --bundle-shader-cache \
  --optimize-storage high \
  --sign "Apple Development: ..." \
  --provisioning-profile /path/to/profile.mobileprovision \
  --xcode-project

# Apple Silicon iOS Simulator .app (fast local iteration)
cargo opsdb -- ios-bundle /path/to/Game --target sim --out out/ios/MyGame.app

# Verify iOS bundle (ARM64 iOS Mach-O platform tag, static AOT count, zero macOS leaks / .mtl4a bloat)
cargo opsdb -- ios-verify out/ios/MyGame.ipa
```

### iOS Host & Flat Bundle Layout (`ios/PortHost/`)
- Flat layout: `<App>.app/{game.toml, Game/, ShaderCache/, Info.plist, PkgInfo}` (first-launch
  config seeding via `[ios] seed_file = "<bundle_rel> -> <drive_c_rel>"` in `game.toml`).
- `ios/PortHost/` Swift files: `PortHostApp.swift`, `MetalSurfaceView.swift` (`CAMetalLayer` via
  `port_ios_attach_surface`), `TouchOverlayView.swift` (virtual XInput slot 0 + touch-as-mouse),
  `KeyboardBridge.swift` (`GCKeyboard` + `UIKeyInput` software keyboard), `PhoneHaptics.swift`
  (`CHHapticEngine` Taptic Engine passthrough yielding to physical `GCDeviceHaptics`).
- `crates/port-loader/src/ios_host.rs` and `ios/PortHost/PortHost-Bridging-Header.h` define the C
  FFI boundary — keep signatures in exact lockstep when changing either side.
- On iOS, `dxil-metal` runs in `BackendKind::CacheOnly` (all shaders must be precompiled on macOS
  into `ShaderCache/`).

## 3. Universal Storage Optimization (`optimize-assets` & `bink2-codec`)

Reduce footprint & iOS Jetsam VRAM on any Windows game directory (`crates/port-factory/src/storage_opt.rs`):

```sh
cargo opsdb -- optimize-assets /path/to/Game --out /path/to/Game_Opt \
  --manifest games/<id>/game.toml --profile compact --dry-run
```

- **Profiles:** `none`, `lossless` (strip unused PC DLLs + language filter + archive slack),
  `high` (+ 1080p `KB2v` hardware-HEVC VideoToolbox Bink2 + 2K lossless DDS mip cap), `compact`
  (+ 720p `KB2v` HEVC + 1K DDS mip cap).
- **Safety rules:** Unused PC DLL stripping always preserves DLLs compiled into static `.aot.o`,
  negative `[runtime].builtin_overrides` entries (`-foo.dll`), and `[storage].keep_dlls`.
- **Archive delegation:** Game-specific packed archives delegate via `[storage].archive_repacker`
  + `archive_dir` in `game.toml` — never add game archive parsers into `crates/port-factory`.

## Verification Checklist

```sh
cargo opsdb -- bundle-verify out/apps/MyGame.app
cargo opsdb -- ios-verify out/ios/MyGame.ipa
cargo test -p port-factory --test bundle_cold_launch --target aarch64-apple-darwin -- --test-threads=1
cargo test -p bink2-codec --target aarch64-apple-darwin
IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-sim-lib
cargo factory-test -- --test-threads=1
```

See `ios/PortHost/README.md` and `docs/OPS_DB.md` for details.
