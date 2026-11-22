# Apple Silicon Port Factory (Experimental)

> [!WARNING]
> **Experimental Research Project — Bring Your Own Legally Owned Games Only**
>
> This repository is an **experimental** open-source runtime, binary translator, and porting
> research framework aimed at running **legitimate Windows x86-64 games that you already legally
> own** natively on Apple Silicon (`aarch64-apple-darwin` macOS and `aarch64-apple-ios` iOS/iPadOS)
> using Metal 4 and MetalFX — **with no Wine, CrossOver, or D3DMetal at runtime**.
>
> - **No game files are distributed here.** No copyrighted executables, DLLs, shaders, audio banks,
>   textures, or game assets are included. You must supply your own legally purchased Windows game
>   installs.
> - **Experimental status.** Expect rough edges, missing Win32/COM/graphics APIs, and per-game
>   compatibility gaps. Pull requests adding new game targets, Win32 APIs, x86-64 instructions, or
>   graphics/audio fixes are welcome (see [CONTRIBUTING.md](CONTRIBUTING.md)).
> - **No anti-cheat or DRM circumvention.** Kernel anti-cheat requirements are detected and
>   reported honestly (`cargo opsdb -- anticheat-audit`); bypasses are explicitly out of scope.
> - **No affiliation.** This project is independent and is not affiliated with, endorsed by, or
>   supported by Apple, Microsoft, Remedy Entertainment, Blizzard Entertainment, Valve, NVIDIA, AMD,
>   Audiokinetic, or RAD Game Tools. All trademarks belong to their respective owners.

---

## Architecture

This is a single, self-contained monorepo (zero git submodules; large `db/*.tsv` analysis tables
are stored via Git LFS). Universal runtime code lives in `crates/`; per-game manifests and
opt-in game targets live in `games/<id>/` (currently including **Overwatch** and **Alan Wake 2**,
plus a starter template at [`games/_template/game.toml`](games/_template/game.toml)).

- **Universal ARM64 execution (`crates/x86-runtime`, `crates/port-loader`, `crates/pe-image`)** —
  Loads Windows PE32+ executables and DLLs, translates x86-64 to ARM64/NEON through an `iced-x86`
  interpreter, block JIT, relocatable `.aot` disk cache (`--prewarm-aot`), and static iOS Mach-O
  `.aot.o` emitter, with MSVC FH4/SEH unwinding. The default `port-runner` binary is ~17 MB, builds
  in ~15 seconds, and contains zero game-specific code.
- **Native Win32 & OS layer (`crates/win32-rt`)** — Clean-room Windows and middleware DLL exports
  built directly on macOS/iOS primitives: Mach/POSIX threads & sync, `~/Translator` virtual `C:\`
  filesystem, AppKit/UIKit surfaces, raw input, `GameController` XInput + CoreHaptics rumble,
  `IOHIDManager` DualSense/SetupDi, Winsock2, `rustls`-backed Schannel TLS 1.2/1.3, and WASAPI on
  CoreAudio.
- **Multi-API Metal 4 graphics (`crates/d3d12-metal`, `crates/dxil-metal`, `crates/win32-rt`)** —
  - **Direct3D 12**: Metal 4 backend with DXR 1.1 ray tracing, mesh shaders, geometry shader
    emulation, GPU indirect command buffers, and native `MTL4FXSpatialScaler` /
    `MTL4FXTemporalScaler` MetalFX upscaling.
  - **Direct3D 11.0 / 11.1**: COM device, immediate/deferred contexts, and SM 4/5 shaders routed
    through `libdxilconv` → `dxil-metal` → Metal 4.
  - **Vulkan**: Dynamic `libMoltenVK.dylib` bridge translating `VK_KHR_win32_surface` to
    `VK_EXT_metal_surface` (`CAMetalLayer`) + `VK_KHR_portability_enumeration`.
  - **Shaders**: DXIL SM 6.x (Apple Metal Shader Converter 4.0.1), FXC SM 4/5 (`libdxilconv`), and
    SPIR-V/WGSL (`naga`), with portable `.metallib` and `MTL4Archive` PSO caches.
- **Audio (`crates/win32-rt/src/audio`, `crates/wwise-audio`)** — CoreAudio-backed WASAPI shim
  with device hotplug and real-time Mach scheduling for any game, plus an optional standalone Rust
  Wwise 2022.1 (v145) `.bnk`/`.wem` runtime.
- **iOS / iPadOS host (`ios/PortHost`)** — UIKit + `CAMetalLayer` reference host with touch
  gamepad overlay, hardware/software keyboard bridge, Taptic Engine rumble passthrough, and
  zero-`MAP_JIT` static `.aot.o` execution.
- **Factory CLI & Diagnostic Toolkit (`crates/port-factory`)** — PE scanner, x86-64 → ARM64 ops
  catalog, Win32 import auditor, anti-cheat / .NET / shader / runtime-hazard auditors, offline
  `.aot` prewarming, scaffolding tools (`scaffold-op`, `scaffold-dll`), macOS `.app` and iOS
  `.app`/`.ipa` bundlers, and 4-tier x86 differential verification.
- **Per-game targets (`games/`)** — Declarative `game.toml` manifests
  ([`games/_template/game.toml`](games/_template/game.toml),
  [`games/overwatch/game.toml`](games/overwatch/game.toml),
  [`games/alan-wake-2/game.toml`](games/alan-wake-2/game.toml)), plus Alan Wake 2's opt-in
  bespoke decompilation/lifting target under `games/alan-wake-2/`.

---

## Prerequisites & Quick Start

1. **Clone with Git LFS** (required to materialize `db/*.tsv`):
   ```sh
   brew install git-lfs
   git lfs install
   git clone https://github.com/mweinbach/apple-silicon-port-factory.git
   cd apple-silicon-port-factory
   ```
2. **Install Apple Metal Shader Converter 4.0.1** (downloaded from Apple Developer) and verify
   host prerequisites:
   ```sh
   ./scripts/install_metal_shader_converter.sh /path/to/Metal_Shader_Converter.pkg
   ./scripts/setup.sh
   ```
   *(Optional)* Fetch upstream reference repositories (`directx-headers`, `vgmstream`, etc.) into
   the gitignored `third_party/` folder with plain shallow clones (no git submodules):
   ```sh
   ./scripts/third_party.sh            # minimal (~35 MB: directx-headers + vgmstream)
   ./scripts/third_party.sh --all      # all optional reference repos (~2.7 GB)
   ```
3. **Build the universal ARM64 runner (~17 MB, ~15s):**
   ```sh
   cargo runner-build
   ```
4. **Audit, prewarm, and launch a Windows game you already own:**
   ```sh
   # Audit Win32 imports, x86-64 instruction coverage, and anti-cheat compatibility
   cargo opsdb -- win32-audit /path/to/YourGame
   cargo opsdb -- scan /path/to/YourGame
   cargo opsdb -- anticheat-audit /path/to/YourGame/Game.exe

   # Prewarm ARM64 .aot block cache and run with a game.toml manifest
   cargo runner -- --manifest games/overwatch/game.toml --game-dir /path/to/Overwatch --prewarm-aot
   cargo runner -- --manifest games/overwatch/game.toml --game-dir /path/to/Overwatch

   # Or run directly against any Windows x86-64 .exe / .dll without a manifest
   cargo runner -- --pe /path/to/app.exe --ops-report --no-run
   cargo runner -- --pe /path/to/library.dll --call-export ExportName
   ```
5. **Run the universal factory test suite:**
   ```sh
   cargo factory-test -- --test-threads=1
   cargo opsdb -- doctor
   ```
6. **Alan Wake 2 opt-in target (`games/alan-wake-2/`):**
   ```sh
   export AW2_GAME_DIR=/path/to/your/AlanWake2
   cargo loader -- --game-dir "$AW2_GAME_DIR"
   cargo data status
   ./tools/fndb.sh report
   ```

---

## Adding a New Game & Contributing

We want this framework to be easy to expand — adding support for a new game usually requires
**zero game-specific Rust code**:

1. Copy [`games/_template/game.toml`](games/_template/game.toml) to `games/<game-id>/game.toml`
   and fill in `[app]` (`id`, `name`, `exe`, `win_dir`, optional `steam_app_id`) and `[runtime]`.
2. Audit your install (`cargo opsdb -- win32-audit`, `scan`, `anticheat-audit`, `runtime-audit`).
3. Prewarm and boot with `cargo runner -- --manifest games/<game-id>/game.toml --game-dir /path/to/Game`.
4. Fix any real universal blockers you hit in `crates/win32-rt`, `crates/x86-runtime`,
   `crates/d3d12-metal`, or `crates/dxil-metal` so every future game benefits too.
5. Open a PR! See [CONTRIBUTING.md](CONTRIBUTING.md) and [AGENTS.md](AGENTS.md) for architecture
   rules, Win32 API sync steps (`cargo opsdb -- win32-sync`), and the PR checklist.

---

## Documentation

- **Contributing & agent guide:** [CONTRIBUTING.md](CONTRIBUTING.md) · [AGENTS.md](AGENTS.md)
- **Direction & planning:** [End Goal](docs/END_GOAL.md) · [What's Needed](docs/WHATS_NEEDED.md) · [What's Next](docs/WHATS_NEXT.md) · [ToDos](docs/TODOS.md)
- **Current status & roadmap:** [docs/STATUS.md](docs/STATUS.md) · [docs/ROADMAP.md](docs/ROADMAP.md)
- **Graphics & shaders:** [docs/D3D12.md](docs/D3D12.md) · [docs/SHADERS.md](docs/SHADERS.md)
- **Windowing, input & audio:** [docs/WINDOWING.md](docs/WINDOWING.md) · [docs/AUDIO.md](docs/AUDIO.md)
- **Factory CLI, Ops DB & Diagnostics:** [docs/OPS_DB.md](docs/OPS_DB.md) · [docs/DIAGNOSTICS.md](docs/DIAGNOSTICS.md) · [crates/port-loader/README.md](crates/port-loader/README.md)
- **iOS host & packaging:** [ios/PortHost/README.md](ios/PortHost/README.md)
- **Tools & third-party deps:** [tools/README.md](tools/README.md) · [docs/THIRD_PARTY.md](docs/THIRD_PARTY.md) · [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md)
- **Alan Wake 2 specs:** [docs/FNDB.md](docs/FNDB.md) · [docs/FORMATS.md](docs/FORMATS.md)

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option. See
[THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for third-party attributions.
