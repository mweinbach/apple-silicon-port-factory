---
name: package-macos-and-ios
description: Package, optimize storage for, sign, deploy, and verify self-contained macOS .app bundles, iOS/iPadOS .app or .ipa bundles (zero MAP_JIT static .aot.o), physical iPhone/iPad devicectl installs, Steam game ports, external SSD exports, and ios/PortHost changes via port-factory. Use when running steam-port, steam-convert, bundle-verify, ios-bundle, ios-verify, aot-static-obj, optimize-assets, deploying to an iPhone/iPad, or editing ios/PortHost or bink2-codec.
---

# Package Portable macOS `.app` & iOS `.app`/`.ipa` Bundles

Never commit `.app/`, `.ipa`, game binaries/assets, signing profiles, or personal `/Users/<name>`
or `/Volumes/` paths. Read [ios_device_and_storage.md](./references/ios_device_and_storage.md) for
the full physical iPhone/iPad `xcodebuild` + `devicectl` deploy recipe, `ios/PortHost` C FFI
table, multi-bank `.aot.o` linker rules, and `KB2v` HEVC container layout.

## 1. Self-Contained Bundling Invariants (macOS & iOS)

1. **Bundle all game binaries, DLLs, assets, manifests, and portable shaders inside the `.app`:**
   - macOS: `<App>.app/Contents/Resources/{game.toml,Game/,AotCache/,ShaderCache/}`
   - iOS (flat bundle): `<App>.app/{game.toml,Game/,ShaderCache/,Info.plist,PkgInfo}`
   - Do **not** depend on ad-hoc `~/Documents/Game` mapping workarounds for packaged apps.
2. **Keep bundle directories strictly read-only at runtime** so `codesign --verify --deep --strict`
   never breaks after launch. Seed caches pass via `PORT_AOT_FALLBACK_DIR`,
   `PORT_SHADER_CACHE_FALLBACK`, and `PORT_PSO_MANIFEST_FALLBACK`; all mutable saves/caches write
   into the user container (`~/Translator` on macOS, `~/Library/Application Support/Translator` on iOS).
3. **Never bundle host-specific `*.mtl4a` files** — ship portable `LC_UUID`-keyed Tier-1 `.aw2s` +
   `pso_manifest/records.p2m` so cold devices prewarm their own GPU's `MTL4Archive` automatically.

## 2. macOS Portable `.app` & Steam Workflow

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

# Verify portability & export to SSD (or launch interactive UI: `cargo opsdb -- launcher`)
cargo opsdb -- bundles
cargo opsdb -- bundle-verify out/apps/MyGame.app
cargo opsdb -- bundle-export out/apps/MyGame.app /Volumes/MySSD
```

- `Contents/Frameworks/` must stage `libmetalirconverter.dylib` (plus optional `libdxilconv.dylib`
  / `libMoltenVK.dylib`) with normalized `minos <= 15.x` and `@rpath` install names alongside
  release `port-runner` and `dxil-metald` in `Contents/MacOS/`, booting cleanly under `env_clear()`.
- *(For the bespoke Alan Wake 2 macOS bundle, use `./scripts/package_app.sh [--copy-to ...]`.)*

## 3. iOS / iPadOS Zero-`MAP_JIT` `.app` / `.ipa` & Physical Device Deploy

Non-jailbroken iOS apps cannot map `MAP_JIT` pages. Compile every guest `.exe`/`.dll` offline on
macOS into Apple-signed ARM64 Mach-O `.aot.o` objects (`x86-runtime/src/emu/static_macho.rs`)
linked directly into signed `__TEXT`:

```sh
# 1. Build universal runner + release iOS static library
cargo runner-build
IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-lib       # target/aarch64-apple-ios/release/libport_loader.a
# (Or `cargo ios-sim-lib` for Apple Silicon iOS Simulator)

# 2. Emit signed static ARM64 .aot.o objects (multi-bank GOT veneers, zero BRANCH26)
cargo opsdb -- aot-static-obj /path/to/Game.exe --out out/ios-aot --platform ios
# (or: cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game --emit-static-o out/ios-aot --static-platform ios)

# 3. One-command iOS device .ipa (+ optional XcodeGen project.yml) or Simulator .app
cargo opsdb -- ios-bundle /path/to/Game \
  --target device --release \
  --manifest games/<id>/game.toml \
  --bundle-shader-cache \
  --optimize-storage high \
  --sign "Apple Development: ..." \
  --provisioning-profile /path/to/profile.mobileprovision \
  --out out/ios/MyGame.ipa --xcode-project

cargo opsdb -- ios-verify out/ios/MyGame.ipa

# 4. Install onto connected physical iPhone / iPad via devicectl
xcrun devicectl list devices
xcrun devicectl device install app --device <DEVICE_UDID> /path/to/MyGame.app
```

Keep `crates/port-loader/src/ios_host.rs` and `ios/PortHost/PortHost-Bridging-Header.h` in exact
lockstep when editing the Swift host. On iOS, `dxil-metal` runs in `BackendKind::CacheOnly` (all
shaders must be precompiled on macOS into `ShaderCache/`).

## 4. Universal Storage Optimization (`optimize-assets` & `bink2-codec`)

```sh
cargo opsdb -- optimize-assets /path/to/Game --out /path/to/Game_Opt \
  --manifest games/<id>/game.toml --profile compact --dry-run
```

- **Profiles:** `none`, `lossless`, `high` (1080p `KB2v` VideoToolbox HEVC + 2K DDS mip cap),
  `compact` (720p `KB2v` HEVC + 1K DDS mip cap).
- Always preserves static `.aot.o` DLLs, negative `[runtime].builtin_overrides`, and
  `[storage].keep_dlls`; delegates game-specific archive repacking via `[storage].archive_repacker`.

## Verification Checklist

```sh
cargo opsdb -- bundle-verify out/apps/MyGame.app
cargo opsdb -- ios-verify out/ios/MyGame.ipa
cargo test -p port-factory --test bundle_cold_launch --target aarch64-apple-darwin -- --test-threads=1
cargo test -p bink2-codec --target aarch64-apple-darwin
IPHONEOS_DEPLOYMENT_TARGET=17.0 cargo ios-sim-lib
cargo factory-test -- --test-threads=1
```
