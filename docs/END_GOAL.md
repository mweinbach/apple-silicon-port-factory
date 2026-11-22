# End Goal

This document defines what "done" looks like so we don't drift into endless refactoring or chase
the wrong work.

Related docs: [What's Needed](WHATS_NEEDED.md) · [What's Next](WHATS_NEXT.md) ·
[ToDos](TODOS.md) · [Status](STATUS.md) · [Roadmap](ROADMAP.md)

---

## The Target Experience

You point the port factory at a legitimate Windows x86-64 game you already own on disk, run one
conversion pipeline, and get a **native Apple Silicon macOS `.app` bundle** that launches cleanly
from Finder:

1. **Zero Wine, zero CrossOver, zero D3DMetal, zero Rosetta at runtime.**
   Every frame runs as native `aarch64-apple-darwin` code on Metal 4, MetalFX, CoreAudio, AppKit,
   GameController, and IOHIDManager.
2. **Pre-translated before first launch.**
   Offline `.aot` block-cache precompilation (`port-runner --prewarm-aot` / `port-factory aot-prewarm`)
   and offline shader precompilation (`dxil-metal`) compile the bulk of the game's x86-64 code and
   shaders ahead of time, so launch and gameplay don't hitch on translation storms.
3. **One universal runtime, declarative per-game config.**
   Normal games require zero Rust code in `crates/`. Bringing up a new title means adding a small
   `games/<id>/game.toml` manifest and fixing any genuinely missing Win32, graphics, or x86 JIT
   behavior inside the universal crates so every future game benefits too.
4. **Honest behavior everywhere.**
   Unsupported APIs fail clearly with documented Windows error codes and visible logs rather than
   silently faking success, skipping work, or hiding crashes.

---

## Finish-Line Criteria

We consider the **Universal Port Factory** complete when all six bars below hold on real Apple
Silicon hardware:

1. **Multi-API graphics proven on real shipping games:**
   - At least one **Direct3D 12** game (beyond Alan Wake 2), one **Direct3D 11** game, and one
     **Vulkan** game (through MoltenVK) boot, render cleanly, accept input, play audio, and save
     files via `port-runner` with zero game-specific Rust code in `crates/`.
   - Native **MetalFX** (`MTL4FXSpatialScaler`, plus temporal scaling where motion/depth inputs are
     available) upscales real game frames cleanly with zero Metal validation errors.
2. **Fast, stable ARM64 execution:**
   - `x86-runtime`'s ARM64/NEON block JIT and `.aot` cache run multi-threaded 64-bit Windows games
     with low translation overhead, reliable MSVC FH4/SEH exception handling, and crash-loop safe
     cache invalidation.
3. **Self-contained one-click `.app` bundles:**
   - `cargo opsdb -- steam-convert <game> --format app` outputs a working `.app` containing
     `port-runner`, `dxil-metald`, `game.toml`, prewarmed `AotCache`, and required converter
     dylibs (`libmetalirconverter.dylib`, optional `libdxilconv.dylib`) that boots without manual
     env-var wrangling.
4. **Solid OS, input, and audio layer:**
   - `win32-rt` handles normal game startup, `~/Translator` saves/configs, Retina & window resize,
     keyboard/mouse/raw FPS look, XInput + CoreHaptics rumble, DualSense HID, and glitch-free
     CoreAudio WASAPI across device switches.
5. **Clean, fast developer loop:**
   - Universal builds (`cargo runner-build`) stay fast (~15–20 s) and completely decoupled from
     game-specific shards; `cargo factory-test` and GPU integration suites stay green on `main`.
6. **Alan Wake 2 playable slice finished (`games/alan-wake-2/`):**
   - Return 0: The Cult and Chapter 1 play start-to-finish on native ARM64 with Metal 4 + MetalFX,
     audio, input, and saves working cleanly.

---

## Explicit Non-Goals

Knowing what we are *not* building matters just as much:

- **No shipping game assets or binaries.** Users always supply their own legally purchased Windows
  game installs.
- **No kernel anti-cheat bypassing.** Games that require Windows kernel drivers (Vanguard, EAC
  kernel mode, BattlEye kernel mode, Ricochet) are flagged honestly by `cargo opsdb --
  anticheat-audit`; we do not build anti-cheat bypasses.
- **No scaling bespoke hand-decompilation to every game.** `games/alan-wake-2/crates/aw2-game`
  stays maintained as an opt-in target, but the universal factory scales through `x86-runtime`'s
  ARM64 block JIT + `.aot` cache, not by hand-lifting millions of lines of Rust per title.
- **No silent shader swaps or fake-pass stubs.** Game shaders translate honestly into Metal, and
  missing features report real errors until implemented properly.
