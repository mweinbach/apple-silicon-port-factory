# Contributing to Apple Silicon Port Factory

Thank you for your interest in contributing! This is an **experimental** project dedicated to
running legitimate Windows x86-64 games that users already legally own on Apple Silicon (macOS and
iOS) via Metal 4, MetalFX, and native ARM64 translation.

Please read [AGENTS.md](AGENTS.md) alongside this guide for detailed crate boundaries, build
commands, and repo conventions.

---

## Ground Rules

1. **Never commit copyrighted game files, binaries, assets, or captures.**
   No `.exe`, `.dll`, `.rmdblob`, `.rmdtoc`, `.binrfx`, `.bnk`, `.wem`, `.bk2`, `.pak`, `.ucas`,
   `.utoc`, `.app/`, `.ipa`, game screenshots, or Steam library HTML dumps.
2. **Never hardcode personal machine paths.**
   No `/Users/<name>`, personal `/Volumes/<drive>`, or launcher-specific personal directories in
   code, tests, scripts, or docs. Always read game locations from CLI flags (`--game-dir`, `--pe`)
   or environment variables (`PORT_GAME_DIR`, `AW2_GAME_DIR`), and make tests that depend on a local
   game install skip cleanly when the env var is unset.
3. **Keep `crates/` completely game-agnostic.**
   Do not add game-specific virtual addresses, hardcoded game names, or engine-specific hacks to
   `crates/`. Configure per-game behavior declaratively in `games/<id>/game.toml` or via universal
   traits and configuration (`AppConfig`, `DllRegistry`, `ComRegistry`, `x86_runtime::hooks`).
4. **No anti-cheat or DRM bypasses.**
   Anti-cheat and DRM requirements must be detected and reported honestly. PRs that attempt to
   circumvent kernel anti-cheat or DRM protections will not be accepted.
5. **Be honest about stubs and errors.**
   Never fake success or silently `memcpy` past an unimplemented API. Unimplemented exports should
   log via `stub!` / `failing!` and return the documented Windows failure code.
6. **Keep Metal calls inside `d3d12-metal` / `dxil-metal`.**
   Never import `objc2-metal` inside `crates/win32-rt`.

---

## Adding a New Game Target

Most games need **zero custom Rust code** — just a declarative manifest plus any genuinely missing
Win32, x86-64, or graphics fixes in the universal crates:

1. **Audit your legally owned Windows install:**
   ```sh
   cargo opsdb -- scan /path/to/Game
   cargo opsdb -- win32-audit /path/to/Game
   cargo opsdb -- anticheat-audit /path/to/Game/Game.exe
   cargo opsdb -- app-audit /path/to/Game/Game.exe
   cargo opsdb -- runtime-audit /path/to/Game
   ```
2. **Create a manifest from the starter template:**
   ```sh
   mkdir -p games/<game-id>
   cp games/_template/game.toml games/<game-id>/game.toml
   ```
   Edit `games/<game-id>/game.toml` with the game's `id`, `name`, `exe`, virtual `win_dir`,
   optional `steam_app_id`, and runtime flags (`builtin_overrides`, `args`, etc.).
3. **Prewarm the ARM64 `.aot` block cache and test booting:**
   ```sh
   cargo runner-build
   cargo runner -- --manifest games/<game-id>/game.toml --game-dir /path/to/Game --prewarm-aot
   cargo runner -- --manifest games/<game-id>/game.toml --game-dir /path/to/Game
   ```
4. **Fix universal blockers (when encountered):**
   - **Missing Win32/CRT/COM export (`UNIMPLEMENTED foo.dll!Bar`):**
     Implement it in `crates/win32-rt/src/` (or scaffold a new DLL module with
     `cargo opsdb -- scaffold-dll`), add a unit test, and run:
     ```sh
     cargo opsdb -- win32-sync
     ```
     Commit the updated `db/win32_apis.tsv` (tracked via Git LFS) with your change.
   - **Missing or buggy x86-64 instruction:**
     Implement it in `crates/x86-runtime/`, add unit/differential tests, and sync `db/ops_x86_to_arm64.tsv`
     if you added a new catalog entry (`cargo opsdb -- sync`).
   - **D3D12 / D3D11 / Vulkan / shader bug:**
     Fix it in `crates/d3d12-metal/`, `crates/win32-rt/src/d3d11/`, `crates/win32-rt/src/misc/vulkan_1.rs`,
     or `crates/dxil-metal/` with a regression test.
5. **Document test results honestly in your PR** (what boots/renders, what still fails, macOS
   version, Apple Silicon chip, and what tests you ran).

---

## Before Opening a Pull Request

Run the relevant checks and confirm zero compiler warnings in files you touched:

```sh
# 1. Universal runner builds cleanly (~15s)
cargo runner-build

# 2. Universal test suite passes
cargo factory-test -- --test-threads=1

# 3. Catalog TSVs are in sync with source code
cargo opsdb -- doctor

# 4. If you edited graphics, Win32, or shared traits:
cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- --test-threads=1
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- d3d11 misc::vulkan_1 --test-threads=1
cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin
```

Use small, focused commits following `<area>: <imperative summary>` (for example:
`win32: implement GetFileInformationByHandleEx`, `x86-runtime: lower vpermq in ARM64 JIT`,
`games: add manifest for <title>`).

## Licensing of Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
this project shall be dual-licensed under the MIT License and Apache License 2.0 (see
[LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE)), without any additional terms or
conditions.
