# Third-Party Notices

This repository contains original code dual-licensed under MIT and Apache-2.0 (`LICENSE-MIT`,
`LICENSE-APACHE`), along with the following embedded third-party component and optional external
tool integrations:

## Embedded Third-Party Code

### `lewton` 0.10.2 (forked under `crates/wwise-audio/src/codec/lewton/`)
- **Upstream:** https://github.com/RustAudio/lewton
- **Copyright:** Copyright (c) 2016 est31 `<MTest31@outlook.com>` and contributors
- **License:** Dual-licensed under the MIT License or Apache License, Version 2.0 (see
  `LICENSE-MIT` and `LICENSE-APACHE` in this repository).
- **Modifications:** Ported to Rust 2021 edition, removed `byteorder`/`tinyvec` dependencies, and
  exposed configurable Vorbis floor1 synthesis (`FloorSynthesis`) for Wwise `.wem` audio
  interoperability.

## Optional Host-Installed Tools (Not Redistributed)

The runtime dynamically loads or invokes the following user-installed tools on macOS when present;
none of their binaries or source code are redistributed in this repository:

- **Apple Metal Shader Converter** (`libmetalirconverter.dylib`) — Apple proprietary developer
  tool installed separately by the user from Apple Developer.
- **Microsoft DirectXShaderCompiler / `dxilconv`** (`libdxilconv.dylib`) — University of Illinois /
  NCSA Open Source License; optional build script and macOS patch live under
  `crates/dxil-metal/tools/dxilconv/`.
- **MoltenVK** (`libMoltenVK.dylib`) — Apache-2.0; optionally installed via Homebrew or LunarG
  Vulkan SDK.
- **FFmpeg** (`libavcodec` / `libavformat` / `libavutil`) — LGPL / GPL depending on host build;
  optionally loaded via `dlopen` from the user's Homebrew installation.

## Optional Reference Checkouts (`./scripts/third_party.sh`)

No git submodules are included in this repository. Optional upstream reference repositories can be
fetched locally into the gitignored `third_party/` directory via `./scripts/third_party.sh`; each
upstream repository remains governed by its own license.

## Trademarks & User-Owned Games

This project distributes **no copyrighted game binaries, executables, DLLs, shaders, audio banks,
or game assets**. All product names, logos, and brands (including Apple, macOS, iOS, Metal,
Microsoft, Windows, DirectX, Remedy Entertainment, Alan Wake, Blizzard Entertainment, Overwatch,
Valve, Steam, NVIDIA, AMD, Audiokinetic, Wwise, and RAD Game Tools) are the property of their
respective owners and are used strictly for interoperability identification.
