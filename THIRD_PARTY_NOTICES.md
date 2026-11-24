# Third-Party Notices

This repository contains original code dual-licensed under MIT and Apache-2.0 (`LICENSE-MIT`,
`LICENSE-APACHE`), embedded third-party code and data, and optional external tool integrations.

## Embedded Third-Party Code

### `lewton` 0.10.2 (forked under `crates/wwise-audio/src/codec/lewton/`)
- **Upstream:** https://github.com/RustAudio/lewton
- **Copyright:** Copyright (c) 2016 est31 `<MTest31@outlook.com>` and contributors
- **License:** Dual-licensed under the MIT License or Apache License, Version 2.0 (see
  `LICENSE-MIT` and `LICENSE-APACHE` in this repository).
- **Modifications:** Ported to Rust 2021 edition, removed `byteorder`/`tinyvec` dependencies, and
  exposed configurable Vorbis floor1 synthesis (`FloorSynthesis`) for Wwise `.wem` audio
  interoperability.

## Arm A64 encoding data

`db/a64_encodings.tsv` and the Arm encoding identifiers in `db/a64_emitter_matches.tsv`
derive from `Instructions.json` in Arm's BSD-licensed AARCHMRS A-profile release
`2026-09_rel` (build 868). The archive SHA-256 is
`d5bf3f538663fb18be081ba8e9f08b188ed06f81dd19d4aa2da5b2795103a4ec`.
[Arm open-source machine-readable data, September 2026](https://developer.arm.com/-/cdn-downloads/permalink/Exploration-Tools-OS-Machine-Readable-Data/AARCHMRS_BSD/AARCHMRS_OPENSOURCE_A_profile_FAT-2026-09.tar.gz)

Copyright (c) 2010-2026 Arm Limited or its affiliates. All rights reserved.

Redistribution and use in source and binary forms, with or without modification, are permitted
provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice, this list of conditions
   and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions
   and the following disclaimer in the documentation and/or other materials provided with the
   distribution.
3. Neither the name of the copyright holder nor the names of its contributors may be used to
   endorse or promote products derived from this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS “AS IS” AND ANY EXPRESS OR
IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR
CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER
IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

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
