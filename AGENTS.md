# AGENTS.md

This repo (**Apple Silicon Port Factory**) is an **experimental** native Apple Silicon
(`aarch64-apple-darwin` / `aarch64-apple-ios`) porting runtime and research toolkit for running
**legitimate Windows x86-64 games that you already legally own**. It loads user-supplied Windows
`.exe` and `.dll` files, translates x86-64 machine code into ARM64, handles Win32/CRT/COM calls in
Rust, converts shaders to Metal, and renders on Metal 4 with **no Wine, CrossOver, or D3DMetal at
runtime**.

It started as an Alan Wake 2 port and grew into a general-purpose, single-repo runtime (zero git
submodules; `db/*.tsv` tracked in Git LFS). The universal engine lives under `crates/`; everything
specific to a single game lives under `games/<id>/` (including `games/overwatch/`,
`games/alan-wake-2/`, and starter template `games/_template/game.toml`). Nothing in this repo
distributes game binaries or assets.

---

## How a Windows game runs here

When you run `port-runner` on a Windows `.exe`, five layers do the work:

1. **PE loading (`crates/pe-image`, `crates/port-loader`)** — Maps the `.exe` and its non-builtin
   DLLs into memory, applies relocations, sets up static TLS, and synthesizes minimal fake PE
   headers/export tables for our builtin DLLs (some middleware walks PE export directories by hand
   instead of calling `GetProcAddress`).
2. **CPU execution (`crates/x86-runtime`)** — Translates guest x86-64 code into ARM64/NEON blocks
   via an `iced-x86` decoder, block JIT, and persistent `.aot` cache on disk (`--prewarm-aot`
   compiles blocks ahead of time). Handles Win64 ↔ AAPCS64 calls, `VA_DISPATCH`, and MSVC
   FH4/SEH exceptions.
3. **Win32 & OS layer (`crates/win32-rt`)** — Implements 86 Windows and middleware DLLs (3,241
   exports) directly on macOS primitives: Mach/POSIX threads and waits, AppKit windows and HID/
   GameController input, CoreAudio for WASAPI, a `~/Translator` virtual `C:\` filesystem, plus
   Direct3D 12, Direct3D 11, and Vulkan entry points.
4. **Shaders (`crates/dxil-metal`)** — Converts DXIL SM 6.x through Apple's Metal Shader Converter
   (`libmetalirconverter.dylib`), legacy FXC SM 4/5 through `libdxilconv.dylib`, and SPIR-V/WGSL
   through `naga`. Caches both `.metallib` output and compiled `MTL4Archive` pipelines.
5. **GPU execution (`crates/d3d12-metal`)** — Drives Metal 4 directly: command queues/lists,
   residency sets, descriptor tables, mesh shaders, DXR 1.1 ray tracing, GPU indirect command
   buffers, and `MTL4FXSpatialScaler` MetalFX upscaling.

Per-game overrides sit on top of this stack. Most games only need a declarative `games/<id>/game.toml`
manifest (see `games/overwatch/game.toml`). Alan Wake 2 also has an optional bespoke decompilation
target (`games/alan-wake-2/`, enabled via `port-loader --features aw2 --bin aw2-loader`).

---

## Hard rules (and why they exist)

Read these before changing code. Every rule here comes from something that actually broke.

1. **Keep `crates/` completely game-agnostic.**
   Never put game-specific virtual addresses, hardcoded game names, autoplay bots, or engine-specific
   asset parsers in `crates/`. Configure per-game behavior through `TargetManifest` (`game.toml`),
   `win32_rt::config::AppConfig`, `DllRegistry`/`ComRegistry`, `x86_runtime::hooks`, or middleware
   hook traits (`CohtmlViewHook`, `CompletionTaskHook`, `PresentHook`). All Alan Wake 2 code lives
   under `games/alan-wake-2/`.

2. **Never import `objc2-metal` in `win32-rt`.**
   All Metal 4 calls belong inside `d3d12-metal` (and `dxil-metal` for PSO archives). Even D3D11
   runs through `d3d12-metal`'s safe Rust types, and Vulkan loads `libMoltenVK.dylib` dynamically.
   Keeping Metal behind `d3d12-metal` stops COM shims from turning into a tangle of raw Objective-C
   calls.

3. **Be honest about stubs, errors, and test results.**
   Never fake success, silently `memcpy` past an unimplemented API, fabricate telemetry, or inflate
   coverage numbers. If an API isn't implemented yet, log it through `stub!` / `failing!` and return
   the documented Windows failure code. Never claim a fix works without running the relevant build
   or test and reporting what actually happened.

4. **Never write mutable data to executable pages.**
   ARM64 enforces W^X and kills the process; Rosetta treats writes to code pages as self-modifying
   code and stalls in `com.apple.rosetta.exceptionserver` (that once cost 40 seconds on every boot).
   Keep JIT/AOT code pages strictly separate from RW data, counters, and vtables.

5. **Assume all guest memory is unaligned.**
   x86-64 doesn't care if a `u64` sits at a 4-byte offset or a `u16` sits at an odd address. Rust
   does, and debug builds abort on misaligned typed derefs. Always read and write guest memory via
   `exe::rd::<T>`, `exe::wr::<T>`, `ptr::read_unaligned` / `write_unaligned`, or `#[repr(C, packed)]`.

6. **Never stub out shipping decompressors or swap game shaders.**
   Let real compression DLLs (`oo2core_*`, `zstd`, `lz4` when shipped) run through the ARM64 JIT
   unless you have a real, tested native implementation. Never substitute game shaders, and keep
   engine-specific GPU barrier shortcuts (like `AW2_IB_BINDLESS_WAR`) **OFF by default**.

7. **Every line of decompiled game code must trace to the binary.**
   In `games/alan-wake-2/crates/aw2-game`, never guess behavior or copy from old prototypes without
   re-verifying. Check the original disassembly/Ghidra output and prove parity with a differential
   test against the x86-64 oracle.

8. **Never commit game files, `.app` bundles, or personal machine paths.**
   No `.exe`, `.dll`, `.rmdblob`, `.rmdtoc`, `.binrfx`, `.bnk`, `.wem`, `.bk2`, `*.app/`, Steam HTML
   dumps, or `/Users/<name>` paths anywhere in code or docs.

---

## Where code belongs

Use this lookup table before you create or edit a file:

| Problem or task | Where it goes |
|---|---|
| Missing or broken Win32/CRT/COM/middleware export (`UNIMPLEMENTED foo.dll!Bar`) | `crates/win32-rt/src/` (then run `cargo opsdb -- win32-sync`) |
| Windowing, AppKit, mouse/keyboard/raw input, XInput, DualSense HID | `crates/win32-rt/src/user32/`, `macos/`, `misc/{xinput1_4,hid,setupapi}.rs` |
| WASAPI / CoreAudio / `mmdevapi` / `avrt` | `crates/win32-rt/src/audio/`, `misc/{mmdevapi,avrt}.rs` |
| Direct3D 12 COM shim vs Metal 4 implementation | COM vtables in `crates/win32-rt/src/d3d12/`; Metal 4 + MetalFX in `crates/d3d12-metal/` |
| Direct3D 11.0/11.1 runtime | `crates/win32-rt/src/d3d11/` (backed by `d3d12-metal` + `dxil-metal`) |
| Vulkan loader & `VK_KHR_win32_surface` bridge | `crates/win32-rt/src/misc/vulkan_1.rs` (loads `libMoltenVK.dylib`) |
| Shader translation (DXIL, FXC SM 4/5, SPIR-V, WGSL) or PSO cache | `crates/dxil-metal/` |
| x86-64 decoder, ARM64/NEON block JIT, `.aot` cache, FH4/SEH unwinding | `crates/x86-runtime/` |
| PE32+ / .NET metadata parsing | `crates/pe-image/` |
| Win64 ↔ AAPCS64 proc-macro ABI shims | `crates/win64-abi/` |
| Universal Wwise 2022.1 `.bnk`/`.wem` parser, Vorbis decoder, DSP engine | `crates/wwise-audio/` |
| Universal runner CLI (`port-runner`), `game.toml` loading, PE mapping | `crates/port-loader/` |
| Ops DB, Win32 DB, PE scanner, `aot-prewarm`, Steam `.app` bundler | `crates/port-factory/` (`cargo opsdb`) |
| Offline D3D12 trace inspector & Metal 4 replay | `crates/d3d12-replay/` |
| Adding a new game target | `games/<id>/game.toml` (start declarative; don't add Rust crates unless truly needed) |
| Alan Wake 2 decomp, lifted shards, Northlight VFS, AW2 hooks/perf logs | `games/alan-wake-2/` |

---

## Fast build, run, and test commands

The default universal runner (`port-runner`) is ~17 MB and builds in ~15 seconds because it does not
compile the 4.47M LOC of Alan Wake 2 shards. Stay on `port-runner` and `cargo factory-test` unless
you are explicitly working on Alan Wake 2.

```sh
# Build & run the universal runner
cargo runner-build
cargo runner -- --manifest games/overwatch/game.toml --game-dir /path/to/Overwatch
cargo runner -- --pe /path/to/any.exe --prewarm-aot          # offline ARM64 .aot block precompile
cargo runner -- --pe /path/to/any.exe --ops-report --no-run  # audit instructions + unresolved imports

# Fast test suites (always pass --test-threads=1 for win32-rt / factory-test)
cargo factory-test -- --test-threads=1
cargo test -p d3d12-metal --lib --target aarch64-apple-darwin -- --test-threads=1
cargo test -p win32-rt --lib --target aarch64-apple-darwin -- d3d11 misc::vulkan_1 --test-threads=1

# Factory CLI (crates/port-factory) & Diagnostic Toolkit
cargo opsdb -- stats
cargo opsdb -- win32-sync
cargo opsdb -- win32-audit /path/to/game/
cargo opsdb -- runtime-audit /path/to/game/        # static MXCSR/x87/AVX-narrow/sync/RWX hazard scan
cargo opsdb -- verify-x86 /path/to/game/ --oracle  # offline 4-tier diff verify + Rosetta 2 oracle
cargo opsdb -- aot-prewarm /path/to/game.exe
cargo opsdb -- scan /path/to/game/ --record

# Alan Wake 2 full target (links aw2-game shards + Wwise native bridge)
cargo loader-build
cargo loader -- --game-dir "$AW2_GAME_DIR"
cargo check -p aw2-game --no-default-features --target aarch64-apple-darwin  # fast check of ns/ (~12s)
cargo data status
tools/fndb.sh report

# Legacy Rosetta 2 x86_64 build (kept for differential testing)
cargo rosetta-build
cargo hybrid-test
```

Key host paths and env vars:
- **Metal Shader Converter 4.0.1**: `~/.local/opt/metal-shaderconverter` (override with `METAL_IRCONVERTER_DYLIB`). Out-of-process worker: `cargo build --release -p dxil-metal --bin dxil-metald`.
- **Virtual Windows root (`C:\`)**: `~/Translator` (`TRANSLATOR_HOME` or `AW2_DRIVE_C`).
- **AOT block cache**: `~/Library/Caches/port-factory/aot` (`PORT_AOT_DIR`, fallback `AW2_DLL_AOT_DIR`).
- **Primary env prefix**: use `PORT_*` for new knobs (`PORT_METALFX=0`, `PORT_D3D11=0`, `PORT_AOT_DIR`, `PORT_HEADLESS=1`, `PORT_MANIFEST`, `PORT_GAME_DIR`). Legacy `AW2_*` aliases still work where kept for compatibility.

---

## Repo gotchas (save yourself an hour)

- **`win32-rt/build.rs` stops at the first `#[cfg(test)]`.**
  The export scanner splits every source file on the first `#[cfg(test)]` token. If you put a
  `#[cfg(test)]` helper above an `extern "win64"` export, every export below it silently disappears
  from the DLL table. Keep `#[cfg(test)]` modules at the bottom of the file.
- **Always run `win32-rt` audio tests with `-- --test-threads=1`.**
  CoreAudio / `akrec` tests share process-global audio state and deadlock under Rust's default
  parallel test runner.
- **Sync `db/win32_apis.tsv` whenever you add or move `win32-rt` exports.**
  Run `cargo opsdb -- win32-sync` and commit the updated TSV alongside your Rust change.
- **Avoid building `aw2-loader` unless you touched AW2.**
  `aw2-game` links hundreds of generated shard files. For universal changes, build `port-runner`
  (`cargo runner-build`) and run `cargo check -p aw2-game --no-default-features` if you changed a
  shared trait or signature in `x86-runtime` or `win32-rt`.
- **Zsh `$path` trap.**
  The shell is zsh, where lowercase `path` is tied to `$PATH`. Never write `path=...` in shell
  commands.
- **Never `process launch` Rosetta binaries under `lldb`.**
  Launching an x86-64 binary directly under lldb can wedge Apple's `oahd` daemon machine-wide. Use
  `tools/lldb.sh`, which attaches by PID after the image is mapped.
- **Read the actual code before trusting old comments or memos.**
  `docs/design/` contains historical design notes from earlier phases.
  Treat current source files and tests as the source of truth.

---

## Standard workflows & repo skills

Detailed step-by-step runbooks live in `.agents/skills/`:

| Skill | When to use it |
|---|---|
| `bring-up-game` | Auditing a new Windows game, writing `games/<id>/game.toml`, prewarming `.aot`, and triaging boot blockers |
| `implement-win32-api` | Implementing or fixing Win32, CRT, COM, or middleware exports in `crates/win32-rt` |
| `implement-x86-op` | Adding, promoting, or fixing x86-64 → ARM64/NEON instructions in `crates/x86-runtime` and `ops_db` |
| `debug-runtime-and-nan` | Hunting CPU/NaN/MXCSR/AOT/crash bugs with `runtime-audit`, `verify-x86`, `live-probe`, `bisect-runtime`, `nan-trace`, Rosetta `oracle`, and `tools/lldb.sh` |
| `graphics-metal4` | Working on D3D12, D3D11, Vulkan→MoltenVK, Metal 4, MetalFX, or `d3d12-replay` |
| `shader-pipeline` | Working on `crates/dxil-metal`, `dxil-metald`, `libdxilconv`, `naga`, Tier-1/Tier-2 PSO caches, and `PsoManifest` |
| `package-macos-and-ios` | Building & verifying portable macOS `.app` bundles, iOS `.app`/`.ipa` (static `.aot.o`), Steam ports, `optimize-assets`, and `ios/PortHost` |
| `audio-and-windowing` | Working on AppKit/UIKit windowing & deadlock rules, raw input, gamepads/DualSense HID, WASAPI→CoreAudio, and `wwise-audio` |
| `profile-and-verify` | Profiling boot/CPU/hitches/GPU passes and running pre-PR boundary, `cargo opsdb -- doctor`, and test gates |
| `decompile-function` | Bespoke Alan Wake 2 function decompilation under `games/alan-wake-2/crates/aw2-game` |

### Bringing up a new Windows game

See `.agents/skills/bring-up-game/SKILL.md`. In short:
1. Audit the install first:
   ```sh
   cargo opsdb -- scan /path/to/game/
   cargo opsdb -- win32-audit /path/to/game/
   cargo opsdb -- anticheat-audit /path/to/game/Game.exe
   cargo opsdb -- app-audit /path/to/game/Game.exe
   cargo opsdb -- runtime-audit /path/to/game/
   ```
2. Create `games/<id>/game.toml` (copy `games/_template/game.toml` as a template). Set `name`,
   `exe`, `win_dir`, optional `steam_app_id`, and runtime flags.
3. Prewarm the ARM64 `.aot` cache and boot with `port-runner`:
   ```sh
   cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/game --prewarm-aot
   cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/game
   ```
4. Fix the real blockers you hit, one at a time. Missing Win32 APIs go in `crates/win32-rt` with a
   unit test + `cargo opsdb -- win32-sync`. Missing x86 instructions go in `crates/x86-runtime`. Do
   not hardcode game workarounds in universal crates.

### Adding or fixing a Win32 API

See `.agents/skills/implement-win32-api/SKILL.md` for the step-by-step checklist. In short:
reproduce the missing/failing call, verify the exact Windows ABI and types, implement real macOS
behavior (or an honest `stub!` failure), add a unit test, run `cargo opsdb -- win32-sync`, and
verify with `cargo test -p win32-rt --lib --target aarch64-apple-darwin -- <module> --test-threads=1`.

### Decompiling an Alan Wake 2 function

See `.agents/skills/decompile-function/SKILL.md`. In short: inspect via `tools/fndb.sh show` and
`tools/ghidra_decompile.sh`, implement under `games/alan-wake-2/crates/aw2-game/src/ns/` using
`exe::rd`/`exe::wr`, register with accurate `Throws::{No, Discard, Caught}`, prove parity with
`--diff-test`, run `tools/fndb.sh sync`, and commit code + `db/progress.tsv` + `db/throws.tsv`.

---

## Definition of done

Before you finish a change or commit:

1. **Right crate, clean boundary** — Universal code stays in `crates/`; `win32-rt` has zero
   `objc2-metal` and zero `aw2-data`; `port-runner` has zero `aw2-game` or `aw2-data`.
2. **Real verification** — Run the narrowest tests that cover your change, plus `cargo factory-test
   -- --test-threads=1` and `cargo runner-build` for cross-cutting edits. If you touched `win32-rt`
   exports, run `cargo opsdb -- win32-sync`.
3. **Zero warnings in code you touched** — Fix unused helpers, dead code, and compiler warnings
   before committing.
4. **Small, clear commits** — Format: `<area>: <imperative summary>` (`factory`, `x86-runtime`,
   `win32`, `loader`, `d3d12`, `d3d11`, `vulkan`, `shaders`, `audio`, `pe`, `game/<ns>`, `tools`,
   `docs`, `build`). Keep `main` green on every commit.

---

## Doc map

Check `docs/WHATS_NEXT.md` and `docs/TODOS.md` when looking for active priorities or picking up the
next task, and update `docs/TODOS.md` / `docs/STATUS.md` when you finish a tracked item.

| Doc | What it covers |
|---|---|
| `README.md` | High-level project overview and quick start |
| `docs/END_GOAL.md` | What "done" looks like, finish-line criteria, and explicit non-goals |
| `docs/WHATS_NEEDED.md` | Host machine prerequisites plus remaining engineering gaps across crates |
| `docs/WHATS_NEXT.md` | Ordered immediate execution plan from right now forward |
| `docs/TODOS.md` | Live checkboxed engineering backlog grouped by crate and priority |
| `docs/STATUS.md` | Honest current state of every subsystem |
| `docs/ROADMAP.md` | Completed milestones and active medium-term tracks |
| `docs/D3D12.md` | Graphics stack: Metal 4 D3D12 + MetalFX, D3D11-on-Metal 4, Vulkan→MoltenVK, trace/replay |
| `docs/SHADERS.md` | `dxil-metal` compiler pipeline (DXIL, FXC SM 4/5, SPIR-V, WGSL), `dxil-metald`, PSO cache |
| `docs/WINDOWING.md` | AppKit threading/deadlock rules, Retina/DPI, raw input, XInput, DualSense HID |
| `docs/AUDIO.md` | WASAPI→CoreAudio shim, device hotplug, RT scheduling, `wwise-audio`, AW2 native Wwise |
| `docs/OPS_DB.md` | `port-factory` CLI: Ops DB, Win32 DB, PE scanner, `.aot` prewarm, Steam packager, `~/Translator` VFS |
| `docs/DIAGNOSTICS.md` | Port Factory Diagnostic & Verification Toolkit: `runtime-audit`, `verify-x86`, `oracle`, `bisect-runtime`, `live-probe`, `nan-trace` |
| `docs/THIRD_PARTY.md` | External tools, optional `third_party/` references (`./scripts/third_party.sh`), and proprietary middleware status |
| `crates/port-loader/README.md` | `port-runner` vs `aw2-loader`, `game.toml`, CLI flags |
| `tools/README.md` | Factory scripts, profiling, LLDB attach workflow, and AW2 `fndb`/Ghidra tools |
| `docs/FNDB.md` · `docs/FORMATS.md` | Alan Wake 2–specific function ownership classifier and Northlight `.rmdtoc`/`.binrfx` specs |
