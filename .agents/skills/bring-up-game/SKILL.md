---
name: bring-up-game
description: Bring up a new Windows x86-64 game on the universal port-runner using offline PE audits, a declarative games/<id>/game.toml manifest, ARM64 .aot prewarming, and iterative blocker triage with zero game-specific Rust code in crates/. Use when porting or adding a new Windows game, auditing a game install, or triaging why a title fails to boot or render.
---

# Bring Up a New Windows Game (`port-runner` + `game.toml`)

Universal rule: **normal games require zero custom Rust code in `crates/`.** Configure per-game
metadata and override policies declaratively in `games/<id>/game.toml`, and fix genuinely missing
Win32, x86-64, shader, or graphics behavior in the universal crates so every game benefits.

**Hard policy:** Never commit copyrighted game binaries/assets or personal machine paths, and never
bypass kernel anti-cheat or DRM. Flag anti-cheat/DRM honestly.

## Step 1 — Audit the Game Install Offline

Run the bundled 7-step offline audit script ([audit_game.sh](./scripts/audit_game.sh)) on the
user's legally owned Windows game directory before booting:

```sh
.agents/skills/bring-up-game/scripts/audit_game.sh /path/to/Game [MainGame.exe] [--record]
```

What to inspect in the output:
- **Anti-cheat / DRM (`anticheat-audit`):** Kernel drivers (Vanguard, kernel EAC/BattlEye,
  Ricochet) or anti-tamper watchdogs must be reported honestly as blockers/caveats.
- **Unresolved imports (`win32-audit`):** Identify missing startup DLLs/exports. Check
  [game_toml_schema.md](./references/game_toml_schema.md) for which DLLs are overridden by native
  ARM64 builtins (`BUILTIN_OVERRIDES`) vs run as retail PEs via ARM64 JIT (`PhysX3*`, `nrd.dll`,
  compression DLLs `oo2core_*`/`zstd`).
- **Graphics API (`app-audit`):** D3D12, D3D11 (needs `libdxilconv.dylib` for SM 4/5), or Vulkan
  (needs `libMoltenVK.dylib`).
- **.NET / Unity (`dotnet-audit`):** `win32-rt` includes a pure-Rust ECMA-335 CLR/CIL fallback
  (`hostfxr`/`coreclr`/`mscoree`/`mono-2.0-bdwgc`) in addition to host `dlopen`; check for
  ReadyToRun images (`--strip-r2r`).
- **Runtime hazards (`runtime-audit`):** Note `ldmxcsr` FTZ/DAZ, x87, AVX 256→128 narrowing,
  approx FP, `LOCK` atomics, or RWX/JIT code (V8/Cohtml → set `v8_jitless = true`).
- **Steam installs:** Run `cargo opsdb -- steam-postinstall /path/to/Game` to extract
  `installscript.vdf` registry keys and check `_CommonRedist`.

## Step 2 — Create `games/<id>/game.toml`

Copy `games/_template/game.toml` to `games/<id>/game.toml` (see `games/overwatch/game.toml`,
`games/alan-wake-2/game.toml`, and the complete schema + `BUILTIN_OVERRIDES` table in
[game_toml_schema.md](./references/game_toml_schema.md)).

Minimal template:

```toml
[app]
id = "my-game"
name = "My Game"
exe = "MyGame.exe"
win_dir = "C:\\Games\\MyGame"
log_name = "MyGame.log"
computer_name = "WIN64PORT"
# steam_app_id = "123456"

[runtime]
preferred_base_retry = false
v8_jitless = false
legacy_remedy_migration = false
# Negate specific builtins with `-foo.dll` (or `"0"` for all) to run the game's retail PE via JIT:
# builtin_overrides = "-physx3_x64.dll"
# args = "-windowed"
```

## Step 3 — Dry-Run Imports & Prewarm ARM64 `.aot` Cache

Build the thin universal runner (~15 s, zero AW2 shards) and precompile ARM64 `.aot` blocks:

```sh
cargo runner-build
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game --ops-report --no-run
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game --prewarm-aot
```

- Control thread count with `PORT_AOT_THREADS=<N>` or cache directory with `--aot-dir` /
  `PORT_AOT_DIR`. Test VFS mappings without booting via `cargo opsdb -- vfs-resolve "<WIN_PATH>"`.

## Step 4 — Boot & Triage Blockers One at a Time

Start headless first, then windowed:

```sh
PORT_HEADLESS=1 cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game
cargo runner -- --manifest games/<id>/game.toml --game-dir /path/to/Game
```

Add `--trace` to log resolved imports (`RUST_LOG=trace=info`). Check `out/crash/crash-*.txt` and
`out/hitch/hitch-*.log`. Route each blocker to the appropriate skill:

| Symptom | Skill & Action |
|---|---|
| `UNIMPLEMENTED foo.dll!Bar`, wrong ABI/COM/middleware behavior | `implement-win32-api` (`crates/win32-rt` + `cargo opsdb -- win32-sync`) |
| Unhandled or hot `GENERIC_INTERP` x86 opcode | `implement-x86-op` (`crates/x86-runtime` + `cargo opsdb -- sync`) |
| Wrong math, NaN physics/geometry, AOT/JIT crash, hang | `debug-runtime-and-nan` (`verify-x86`, `live-probe`, `bisect-runtime`, `nan-trace`) |
| Shader conversion / `D3DCompile` / PSO stutter | `shader-pipeline` (`crates/dxil-metal`, `dxil-metald`, `d3dcompiler_47`) |
| Metal validation error (`MTL_DEBUG_LAYER=1`), black frame, D3D11/D3D12/Vulkan/upscaler bug | `graphics-metal4` (`crates/d3d12-metal`, `win32-rt/src/{d3d11,d3d12,misc/vulkan_1.rs}`) |
| Window freeze, broken mouse/gamepad/IME, silent/glitchy audio | `audio-and-windowing` (`win32-rt/src/{user32,macos,audio,misc/{fmod,xaudio2_9redist}}`) |

Once frames render, capture repeatable smoke verification with `live-probe`:

```sh
cargo opsdb -- live-probe \
  --manifest games/<id>/game.toml \
  --game-dir /path/to/Game \
  --ready-regex "Present #10" \
  --settle-frames 60 \
  --step "shot out/bringup_f0.png" \
  --fail-regex 'UNIMPLEMENTED|\[diag-boundary\]'
```

## Step 5 — Verify & Land Cleanly

```sh
cargo runner-build
cargo factory-test -- --test-threads=1
.agents/skills/audit-repo-hygiene/scripts/audit_repo_hygiene.sh --staged --doctor
```

- Package as a self-contained macOS `.app` or iOS `.ipa` using `package-macos-and-ios` once playable.
- Commit cleanly (`games: add manifest for <title>`) and record honest caveats in `docs/STATUS.md`.
