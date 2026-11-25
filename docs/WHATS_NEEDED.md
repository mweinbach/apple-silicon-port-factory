# What's Needed

Two things stand between where the repo is today and the [End Goal](END_GOAL.md):
1. **Host tools & libraries** needed on your Mac to build and exercise every backend.
2. **Codebase gaps** that still need real engineering work.

Related docs: [End Goal](END_GOAL.md) · [What's Next](WHATS_NEXT.md) · [ToDos](TODOS.md) ·
[Status](STATUS.md) · [Third-Party Dependencies](THIRD_PARTY.md)

---

## 1. Host Machine Prerequisites

### Required for the universal factory
- **Apple Silicon Mac** running macOS with **Metal 4** support and Xcode Command Line Tools.
- **Rust toolchain** with `aarch64-apple-darwin` target (`rustup target add aarch64-apple-darwin`).
- **Apple Metal Shader Converter 4.0.1** (`libmetalirconverter.dylib`):
  ```sh
  ./scripts/setup.sh
  # or directly:
  ./scripts/install_metal_shader_converter.sh
  ```
  Installs to `~/.local/opt/metal-shaderconverter` (override with `METAL_IRCONVERTER_DYLIB`).
- **Git LFS** (optional; only the large Alan Wake 2 analysis tables such as `db/progress.tsv` and
  `db/throws.tsv` live there — the catalogs that `cargo opsdb -- doctor` checks are plain text):
  ```sh
  brew install git-lfs && git lfs install && git lfs pull
  ```
- **Optional `third_party/` reference repos** (no git submodules; `vgmstream` is used at build time
  by `wwise-audio` for Wwise Vorbis codebook tables, and `directx-headers` is used when regenerating
  D3D12 vtables):
  ```sh
  ./scripts/third_party.sh        # minimal: directx-headers + vgmstream (~35 MB)
  ```
- **Your own legally owned Windows x86-64 game(s)** on disk (Steam, Epic/Heroic, Battle.net, GOG, or standalone).

### Optional host libraries (needed for specific backends)
| Backend / Feature | What you need | How to get it |
|---|---|---|
| **Direct3D 11 & SM 4/5 shaders** | `libdxilconv.dylib` | `crates/dxil-metal/tools/dxilconv/build.sh` (installs to `~/.local/lib`, override `AW2_DXILCONV`) |
| **Live Vulkan games** | `libMoltenVK.dylib` | `brew install molten-vk` (override `PORT_VULKAN_DYLIB`) |
| **Offline SPIR-V / WGSL → `.metallib`** | Xcode Metal Toolchain (`xcrun -sdk macosx metal`) | `xcodebuild -downloadComponent MetalToolchain` |
| **Out-of-process shader worker** | `dxil-metald` binary | `cargo build --release -p dxil-metal --bin dxil-metald` |
| **Alan Wake 2 reverse engineering** | Ghidra + OpenJDK 21, optional Rosetta `x86_64-apple-darwin` target | `brew install ghidra` and `rustup target add x86_64-apple-darwin` |

---

## 2. Engineering Gaps Still Needed in the Codebase

Everything below is real technical work still missing or unhardened. Individual work items are
tracked in [docs/TODOS.md](TODOS.md).

### Real multi-game runtime validation (biggest gap right now)
- Almost all end-to-end game hours so far have been on Alan Wake 2 and standalone DLLs (`zlib1.dll`).
- We need real bring-up runs on **non-Northlight games** (D3D12, D3D11, and Vulkan) using pure
  `port-runner` + `game.toml` to flush out real-world Win32, CRT, COM, JIT, and GPU assumptions
  that unit tests cannot catch.

### Graphics on shipping games

- Validate D3D11 mip generation, indirect draws, geometry shaders, depth readback and shared
  resources on another engine. The fixture and unit coverage is recorded in [Status](STATUS.md).
- The Vulkan guest fixture presents and resizes on live MoltenVK. Exercise the same path with
  a shipping Vulkan title.
- Check D3D12 barrier and residency behavior on a non-Northlight engine. Spatial MetalFX has
  run in gameplay. Temporal MetalFX needs validation with a shipping game's depth, motion
  vectors and jitter.

### CPU and Win32 coverage

- Use profiles from the next offline game to find gaps in parallel AOT prewarming, indirect
  control flow and interpreter fallbacks. A catalog family can contain an unsupported form.
- Exercise the APIs the game actually imports, including COM calls resolved at runtime.
  Static implementation kinds and stub sites are starting points for guest conformance tests.

### Packaging and gameplay

- Launch a shipping offline title from its bundle in Finder and verify gameplay and save/load.
  The packager already stages the runner, worker and available converter libraries.
- Complete Return 0 and Chapter 1 in Alan Wake 2 with the terrain fix and audio updates enabled.
  [Ordered milestones](WHATS_NEXT.md)
